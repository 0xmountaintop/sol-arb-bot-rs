use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize)]
pub struct QuoteParams {
    pub input_mint: String,
    pub output_mint: String,
    pub amount: u64,
    pub only_direct_routes: bool,
    pub slippage_bps: u64,
    pub max_accounts: u64,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct QuoteResponse {
    pub output_mint: String,
    pub out_amount: u64,
    pub other_amount_threshold: u64,
    pub price_impact_pct: f64,
    pub route_plan: Vec<serde_json::Value>,
}

#[derive(Debug, Serialize)]
pub struct SwapData {
    pub user_public_key: String,
    pub wrap_and_unwrap_sol: bool,
    pub use_shared_accounts: bool,
    pub compute_unit_price_micro_lamports: u64,
    pub dynamic_compute_unit_limit: bool,
    pub skip_user_accounts_rpc_calls: bool,
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
