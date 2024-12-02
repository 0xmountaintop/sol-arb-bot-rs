use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize)]
pub struct QuoteParams {
    input_mint: String,
    output_mint: String,
    amount: u64,
    only_direct_routes: bool,
    slippage_bps: u64,
    max_accounts: u64,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct QuoteResponse {
    out_amount: u64,
    route_plan: serde_json::Value,
    context_slot: u64,
}

#[derive(Debug, Serialize)]
pub struct SwapData {
    user_public_key: String,
    wrap_and_unwrap_sol: bool,
    use_shared_accounts: bool,
    compute_unit_price_micro_lamports: u64,
    dynamic_compute_unit_limit: bool,
    skip_user_accounts_rpc_calls: bool,
    quote_response: QuoteResponse,
}