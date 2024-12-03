pub const WSOL_MINT: &str = "So11111111111111111111111111111111111111112";
pub const USDC_MINT: &str = "EPjFWdd5AufqSSqeM2qN1xzybapC8G4wEGGkZwyTDt1v";
pub const JITO_TIP_ACCOUNT: &str = "Cw8CFyM9FkoMi7K7Crf6HNQqf4uEMzpKw6QNghXLvLkY";

use lazy_static::lazy_static;
use std::env;

lazy_static! {
    pub static ref RPC_URL: String = {
        env::var("RPC_URL").unwrap_or_else(|_| "https://mainnet-ams.chainbuff.com".to_string())
    };
    pub static ref JUP_V6_API_BASE_URL: String = {
        env::var("JUP_V6_API_BASE_URL")
            .unwrap_or_else(|_| "http://127.0.0.1:8080".to_string())
    };
    pub static ref JITO_RPC_URL: String = {
        env::var("JITO_RPC_URL")
            .unwrap_or_else(|_| "https://frankfurt.mainnet.block-engine.jito.wtf/api/v1/bundles".to_string())
    };
}
