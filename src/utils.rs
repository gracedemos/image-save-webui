use serde::{Serialize, Deserialize};
use wasm_bindgen::prelude::*;
use solana_client_wasm::solana_sdk::transaction::Transaction;

pub const PROGRAM_ID: &str = "BQBREaqSAAXTkAiQSF75naFQBGUrZzkv3jkBjxopZRgs";
pub const RPC_URL: &str = "http://localhost:8899";

#[derive(Serialize, Deserialize)]
pub struct Image {
    pub url: String,
    pub title: String
}

#[wasm_bindgen(module = "/src/scripts/wallet.js")]
extern "C" {
    pub async fn connect() -> JsValue;
    pub async fn trustedConnect() -> JsValue;
    pub async fn sendTransaction(transaction: Transaction) -> JsValue;
}
