use wasm_bindgen::prelude::*;
use wasm_bindgen::JsValue;
use serde::{Deserialize, Serialize};

#[allow(unused)]
pub const IRYS_DEVNET_URL: &str = "https://devnet.irys.xyz";
pub const SOLFLARE_URL: &str = "https://solflare.com/ul/v1/connect";
pub const SOLANA_WALLET: &str = "solana";

#[wasm_bindgen(module = "/wasm/main.mjs")]
extern "C" {
    pub type WebIrys;    

    // #[wasm_bindgen(constructor)]
    // pub fn new(params: &JsValue) -> JsValue;

    #[wasm_bindgen(method, js_name = getPrice)]
    pub async fn get_price(this: &WebIrys, bytes: u64) -> JsValue;

    #[wasm_bindgen(method, js_name = fund)]
    pub async fn fund(this: &WebIrys, atomicValue: JsValue) -> JsValue;

    #[wasm_bindgen(method, js_name = uploadFile)]
    pub async fn upload_file(this: &WebIrys, file: JsValue, configs: JsValue) -> JsValue;

    #[wasm_bindgen(js_name = toAtomic)]
    pub fn to_atomic(irys: &WebIrys, value: JsValue) -> JsValue;

    #[wasm_bindgen(js_name = fromAtomic)]
    pub fn from_atomic(irys: &WebIrys, value: JsValue) -> JsValue;

    #[wasm_bindgen(js_name = token)]
    pub fn token(irys: &WebIrys) -> JsValue;

    #[wasm_bindgen(js_name = getIrysInstance)]
    pub fn get_irys_instance(url: String, token: String, wallet: JsValue) -> JsValue;

    pub type Solflare;

    #[wasm_bindgen(js_name = getSolflareInstance)]
    pub fn get_solflare_instance(immediate_connect: bool) -> JsValue;

}


#[allow(unused)]
#[wasm_bindgen]
#[derive(Deserialize)]
pub struct FundTx {
    pub quantity: f64
}

impl FundTx {
    pub fn new(quantity: f64) -> Self {
        FundTx { quantity }
    }
}

#[allow(unused)]
#[wasm_bindgen]
pub struct UploadConfigs {
    tags: Vec<Tag>
}

impl UploadConfigs {
    pub fn new(tags: Vec<Tag>) -> Self {
        UploadConfigs {
            tags
        }
    }
}

#[allow(unused)]
#[wasm_bindgen]
pub struct Tag {
    name: String,
    value: String
}

impl Tag {
    pub fn new(name: String, value: String) -> Self {
        Tag {
            name,
            value
        }
    }
}

#[allow(unused)]
#[wasm_bindgen]
pub struct Wallet {
    rpc_url: String,
    name: String,
    provider: JsValue
}

#[wasm_bindgen]
#[derive(Serialize, Deserialize, Debug)]
pub struct IrysConstructorConfig {
    url: Option<String>,
    token: Option<String>
}

#[wasm_bindgen]
pub fn connect_to_irys(url: String, token: String) -> WebIrys {    
    let web_irys = get_irys_instance(url, token, JsValue::from(Wallet {
        rpc_url: SOLFLARE_URL.to_string(),
        name: SOLANA_WALLET.to_string(),
        provider: get_solflare_instance(true)
    }));

    web_irys.into()
}