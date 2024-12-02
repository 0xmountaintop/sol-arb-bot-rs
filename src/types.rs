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

#[derive(Debug, Deserialize, Serialize)]
pub struct QuoteResponse {
    pub out_amount: u64,
    pub route_plan: serde_json::Value,
    pub context_slot: u64,
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

#[derive(Debug, Deserialize, Serialize)]
pub struct SwapResponse {
    pub swap_instructions: serde_json::Value, // TODO: FIXME
}
