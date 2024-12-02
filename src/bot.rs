use anyhow::Result;
use solana_client::rpc_client::RpcClient;
use solana_sdk::signature::Keypair;

use crate::types::*;

pub struct ArbitrageBot {
    client: RpcClient,
    http_client: reqwest::Client,
    payer: Keypair,
}

impl ArbitrageBot {
    pub fn new() -> Result<Self> {
        unimplemented!()
    }

    async fn get_quote(&self, params: &QuoteParams) -> Result<QuoteResponse> {
        unimplemented!()
    }

    pub async fn run(&self) -> Result<()> {
        unimplemented!()
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