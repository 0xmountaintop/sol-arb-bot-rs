use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize)]
pub struct QuoteParams {
    #[serde(rename = "inputMint")]
    pub input_mint: String,
    #[serde(rename = "outputMint")]
    pub output_mint: String,
    pub amount: String,
    #[serde(rename = "onlyDirectRoutes")]
    pub only_direct_routes: bool,
    #[serde(rename = "slippageBps")]
    pub slippage_bps: u64,
    #[serde(rename = "maxAccounts")]
    pub max_accounts: u64,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct QuoteResponse {
    #[serde(rename = "inputMint")]
    pub input_mint: String,
    #[serde(rename = "inAmount")]
    pub in_amount: String,
    #[serde(rename = "outputMint")]
    pub output_mint: String,
    #[serde(rename = "outAmount")]
    pub out_amount: String,
    #[serde(rename = "otherAmountThreshold")]
    pub other_amount_threshold: String,
    #[serde(rename = "priceImpactPct")]
    pub price_impact_pct: String,
    #[serde(rename = "routePlan")]
    pub route_plan: Vec<serde_json::Value>,
    #[serde(rename = "swapMode")]
    pub swap_mode: String,
    #[serde(rename = "slippageBps")]
    pub slippage_bps: u64,
}

#[derive(Debug, Serialize)]
pub struct SwapData {
    #[serde(rename = "userPublicKey")]
    pub user_public_key: String,
    #[serde(rename = "wrapAndUnwrapSol")]
    pub wrap_and_unwrap_sol: bool,
    #[serde(rename = "useSharedAccounts")]
    pub use_shared_accounts: bool,
    #[serde(rename = "computeUnitPriceMicroLamports")]
    pub compute_unit_price_micro_lamports: u64,
    #[serde(rename = "dynamicComputeUnitLimit")]
    pub dynamic_compute_unit_limit: bool,
    #[serde(rename = "skipUserAccountsRpcCalls")]
    pub skip_user_accounts_rpc_calls: bool,
    #[serde(rename = "quoteResponse")]
    pub quote_response: QuoteResponse,
}

#[derive(Debug, Deserialize)]
pub struct SwapInstructionResponse {
    #[serde(rename = "computeUnitLimit")]
    pub compute_unit_limit: u32,
    #[serde(rename = "setupInstructions")]
    pub setup_instructions: Vec<InstructionData>,
    #[serde(rename = "swapInstruction")]
    pub swap_instruction: InstructionData,
    #[serde(rename = "addressLookupTableAddresses")]
    pub address_lookup_table_addresses: Vec<String>,
    #[serde(rename = "tokenLedgerInstruction")]
    pub token_ledger_instruction: Option<InstructionData>,
    #[serde(rename = "computeBudgetInstructions")]
    pub compute_budget_instructions: Vec<InstructionData>,
    #[serde(rename = "cleanupInstruction")]
    pub cleanup_instruction: Option<InstructionData>,
    #[serde(rename = "otherInstructions")]
    pub other_instructions: Vec<InstructionData>,
    #[serde(rename = "prioritizationFeeLamports")]
    pub prioritization_fee_lamports: u64,
}

#[derive(Debug, Deserialize)]
pub struct InstructionData {
    #[serde(rename = "programId")]
    pub program_id: String,
    pub accounts: Vec<AccountData>,
    pub data: String,
}

#[derive(Debug, Deserialize)]
pub struct AccountData {
    pub pubkey: String,
    #[serde(rename = "isSigner")]
    pub is_signer: bool,
    #[serde(rename = "isWritable")]
    pub is_writable: bool,
}
