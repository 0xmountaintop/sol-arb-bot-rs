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
    pub compute_unit_limit: u32,
    pub setup_instructions: Vec<InstructionData>,
    pub swap_instruction: InstructionData,
    pub address_lookup_table_addresses: Vec<String>,
}

#[derive(Debug, Deserialize)]
pub struct InstructionData {
    pub program_id: String,
    pub accounts: Vec<AccountData>,
    pub data: String,
}

#[derive(Debug, Deserialize)]
pub struct AccountData {
    pub pubkey: String,
    pub is_signer: bool,
    pub is_writable: bool,
}
