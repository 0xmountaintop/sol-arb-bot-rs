use anyhow::Result;
use solana_client::rpc_client::RpcClient;
use solana_sdk::{
    commitment_config::CommitmentConfig,
    signature::{Keypair, read_keypair_file},
};
use std::{env, time::Instant};
use crate::types::*;
use crate::consts::*;

pub struct ArbitrageBot {
    client: RpcClient,
    http_client: reqwest::Client,
    payer: Keypair,
}

impl ArbitrageBot {
    pub fn new() -> Result<Self> {
        dotenv::dotenv().ok();
        let keypair_path = env::var("KEYPAIR_PATH").expect("KEYPAIR_PATH must be set");
        let payer = read_keypair_file(&keypair_path).expect("Failed to read keypair file");
        println!("payer: {:?}", payer);
        
        Ok(Self {
            client: RpcClient::new_with_commitment(RPC_URL.to_string(), CommitmentConfig::processed()),
            http_client: reqwest::Client::new(),
            payer,
        })
    }

    async fn get_quote(&self, params: &QuoteParams) -> Result<QuoteResponse> {
        unimplemented!()
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
            self.execute_arbitrage(quote0_resp, quote1_resp, jito_tip).await?;
            
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
        unimplemented!()
    }
}