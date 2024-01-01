use serde::{Serialize, Deserialize};
use wasm_bindgen::prelude::*;

pub const PROGRAM_ID: &str = "HpPMAWFuUrpXgxTBVAwse227jLqerVd8iTQ1oS17Zn2o";
pub const RPC_URL: &str = "http://localhost:8899";

#[derive(Serialize, Deserialize)]
pub struct Image {
    url: String,
    title: String
}

#[wasm_bindgen(module = "/src/scripts/wallet.js")]
extern "C" {
    pub async fn connect() -> JsValue;
    pub async fn trustedConnect() -> JsValue;
}
