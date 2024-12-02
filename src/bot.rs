use crate::consts::*;
use crate::types::*;
use anyhow::Result;
use serde::{Deserialize, Serialize};
use solana_client::rpc_client::RpcClient;
use solana_sdk::{
    bs58,
    commitment_config::CommitmentConfig,
    compute_budget::ComputeBudgetInstruction,
    instruction::{AccountMeta, Instruction},
    pubkey::Pubkey,
    signature::{read_keypair_file, Keypair},
    signer::Signer,
    system_instruction,
    transaction::VersionedTransaction,
};
use std::{env, str::FromStr, time::Instant};

pub struct ArbitrageBot {
    client: RpcClient,
    http_client: reqwest::Client,
    payer: Keypair,
}

impl ArbitrageBot {
    pub fn new() -> Result<Self> {
        let keypair_path = env::var("KEYPAIR_PATH").expect("KEYPAIR_PATH must be set");
        let payer = read_keypair_file(&keypair_path).expect("Failed to read keypair file");
        println!("payer: {:?}", bs58::encode(payer.pubkey()).into_string());

        Ok(Self {
            client: RpcClient::new_with_commitment(
                RPC_URL.to_string(),
                CommitmentConfig::processed(),
            ),
            http_client: reqwest::Client::new(),
            payer,
        })
    }

    async fn get_quote(&self, params: &QuoteParams) -> Result<QuoteResponse> {
        let response = self
            .http_client
            .get(JUP_V6_API_BASE_URL.to_string() + "/quote")
            .query(&params)
            .send()
            .await?
            .json()
            .await?;
        Ok(response)
    }

    async fn get_swap_instructions(&self, params: &SwapData) -> Result<SwapInstructionResponse> {
        let response = self
            .http_client
            .post(JUP_V6_API_BASE_URL.to_string() + "/swap-instructions")
            .json(&params)
            .send()
            .await?
            .json()
            .await?;
        Ok(response)
    }

    pub async fn run(&self) -> Result<()> {
        let start = Instant::now();

        // Quote 0: WSOL -> USDC
        let quote0_params = QuoteParams {
            input_mint: WSOL_MINT.to_string(),
            output_mint: USDC_MINT.to_string(),
            amount: 10_000_000, // 0.01 WSOL
            only_direct_routes: false,
            slippage_bps: 0,
            max_accounts: 20,
        };
        let quote0_resp = self.get_quote(&quote0_params).await?;

        // Quote 1: USDC -> WSOL
        let quote1_params = QuoteParams {
            input_mint: USDC_MINT.to_string(),
            output_mint: WSOL_MINT.to_string(),
            amount: quote0_resp.out_amount,
            only_direct_routes: false,
            slippage_bps: 0,
            max_accounts: 20,
        };
        let quote1_resp = self.get_quote(&quote1_params).await?;

        // Calculate potential profit
        let diff_lamports = quote1_resp.out_amount.saturating_sub(quote0_params.amount);
        println!("diffLamports: {}", diff_lamports);

        let jito_tip = diff_lamports / 2;

        const THRESHOLD: u64 = 3000;
        if diff_lamports > THRESHOLD {
            // Build and send transaction
            self.execute_arbitrage(quote0_resp, quote1_resp, jito_tip)
                .await?;

            let duration = start.elapsed();
            println!("Total duration: {}ms", duration.as_millis());
        }

        Ok(())
    }

    async fn execute_arbitrage(
        &self,
        quote0: QuoteResponse,
        quote1: QuoteResponse,
        jito_tip: u64,
    ) -> Result<()> {
        // Merge quote responses for Jupiter API
        let merged_quote = QuoteResponse {
            out_amount: quote0.out_amount,
            route_plan: [quote0.route_plan, quote1.route_plan].concat(),
            context_slot: quote0.context_slot,
        };

        // Prepare swap data for Jupiter API
        let swap_data = SwapData {
            user_public_key: self.payer.pubkey().to_string(),
            wrap_and_unwrap_sol: false,
            use_shared_accounts: false,
            compute_unit_price_micro_lamports: 1,
            dynamic_compute_unit_limit: true,
            skip_user_accounts_rpc_calls: true,
            quote_response: merged_quote,
        };

        // Get swap instructions from Jupiter
        let instructions_resp: SwapInstructionResponse =
            self.get_swap_instructions(&swap_data).await?;

        // Build transaction instructions
        let mut instructions = Vec::new();

        // 1. Add compute budget instruction
        let compute_budget_ix =
            ComputeBudgetInstruction::set_compute_unit_limit(instructions_resp.compute_unit_limit);
        instructions.push(compute_budget_ix);

        // 2. Add setup instructions
        for setup_ix in instructions_resp.setup_instructions {
            instructions.push(self.convert_instruction_data(setup_ix)?);
        }

        // 3. Add swap instruction
        instructions.push(self.convert_instruction_data(instructions_resp.swap_instruction)?);

        // 4. Add tip instruction
        let tip_ix = system_instruction::transfer(
            &self.payer.pubkey(),
            &Pubkey::from_str(JITO_TIP_ACCOUNT)?,
            jito_tip,
        );
        instructions.push(tip_ix);

        // Get latest blockhash
        let blockhash = self.client.get_latest_blockhash()?;

        // Convert address lookup tables
        let address_lookup_tables = self
            .get_address_lookup_tables(&instructions_resp.address_lookup_table_addresses)
            .await?;

        // Create versioned transaction
        let message = solana_sdk::message::v0::Message::try_compile(
            &self.payer.pubkey(),
            &instructions,
            &address_lookup_tables,
            blockhash,
        )?;

        let transaction = VersionedTransaction::try_new(
            solana_sdk::message::VersionedMessage::V0(message),
            &[&self.payer],
        )?;

        // Serialize transaction for Jito bundle
        let serialized_tx = transaction.serialize()?;
        let base58_tx = bs58::encode(&serialized_tx).into_string();

        // Send bundle to Jito
        let bundle_request = serde_json::json!({
            "jsonrpc": "2.0",
            "id": 1,
            "method": "sendBundle",
            "params": [[base58_tx]]
        });

        let bundle_resp = self
            .http_client
            .post("https://frankfurt.mainnet.block-engine.jito.wtf/api/v1/bundles")
            .json(&bundle_request)
            .send()
            .await?;

        let bundle_result: serde_json::Value = bundle_resp.json().await?;
        println!(
            "Sent to frankfurt, bundle id: {}",
            bundle_result["result"].as_str().unwrap_or("unknown")
        );

        Ok(())
    }

    fn convert_instruction_data(&self, ix_data: InstructionData) -> Result<Instruction> {
        let program_id = Pubkey::from_str(&ix_data.program_id)?;

        let accounts: Vec<AccountMeta> = ix_data
            .accounts
            .into_iter()
            .map(|acc| {
                let pubkey = Pubkey::from_str(&acc.pubkey).expect("Failed to parse pubkey");
                if acc.is_writable {
                    AccountMeta::new(pubkey, acc.is_signer)
                } else {
                    AccountMeta::new_readonly(pubkey, acc.is_signer)
                }
            })
            .collect();

        let data = bs58::decode(&ix_data.data).into_vec()?;

        Ok(Instruction {
            program_id,
            accounts,
            data,
        })
    }

    async fn get_address_lookup_tables(
        &self,
        addresses: &[String],
    ) -> Result<Vec<solana_sdk::address_lookup_table_account::AddressLookupTableAccount>> {
        let futures = addresses.iter().map(|address| async {
            let pubkey = Pubkey::from_str(address)?;
            let account = self
                .client
                .get_account_with_commitment(&pubkey, CommitmentConfig::processed())?
                .value
                .ok_or_else(|| anyhow::anyhow!("Address lookup table not found"))?;

            Ok(
                solana_sdk::address_lookup_table_account::AddressLookupTableAccount::new(
                    pubkey,
                    account.lamports,
                    account.data,
                ),
            )
        });

        futures::future::join_all(futures)
            .await
            .into_iter()
            .collect::<Result<Vec<_>>>()
    }
}
