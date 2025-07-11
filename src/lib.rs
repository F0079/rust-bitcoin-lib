use serde_json::Value;

use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn foobar() -> i32 {
    return 1;
}

#[wasm_bindgen]
pub async fn fetch_ticker() -> String {
    let url: &str = "https://blockchain.info/ticker";
    let client = reqwest_wasm::Client::new();
    let res = match client.get(url).send().await {
        Ok(response) => response.text().await.unwrap(),
        Err(error) => panic!("Error: {}", error),
    };

    let ticker: Value = serde_json::from_str(&res).unwrap();
    return ticker["EUR"]["last"].to_string();
}
