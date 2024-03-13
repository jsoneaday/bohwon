use wasm_bindgen::closure::Closure;
use wasm_bindgen::JsCast;
use web_sys::Event;
use web_sys::File;
use web_sys::FileReader;
use leptos::logging::log;
use reqwest::{Client, StatusCode};
use gloo_utils::format::JsValueSerdeExt;
use super::liteseed_types::LiteseedError;

const LITESEED_URL: &str = "https://api.liteseed.xyz/";

pub struct LsApi {
    client: Client
}

impl LsApi {
    pub fn new() -> Self {
        LsApi {
            client: Client::new()
        }
    }

    pub async fn get_cost(&self, size: f64) -> Result<f64, LiteseedError> {
        let cost_result = self.client.get(format!("{LITESEED_URL}cost/{size}"))
            .send()
            .await;

        match cost_result {
            Ok(resp) => {
                match resp.status() {
                    StatusCode::OK => {
                        let result = resp.json::<String>().await;
                        match result {
                            Ok(price_str) => {
                                match price_str.parse::<f64>() {
                                    Ok(price) => Ok(price),
                                    Err(_) => Err(LiteseedError::ParseFloatError)
                                }
                            },
                            Err(e) => {
                                log!("{:?}", e);
                                Err(LiteseedError::InternalServer)
                            }
                        }
                    },
                    _ => {
                        log!("{:?}", resp.error_for_status().err().unwrap());
                        Err(LiteseedError::InternalServer)
                    }
                }
            },
            Err(e) => {
                log!("{:?}", e);
                Err(LiteseedError::InternalServer)
            }
        }
    }

    pub async fn upload_file(&self, file: &File) {
        let fr = FileReader::new().unwrap();
        fr.read_as_array_buffer(&file.slice().unwrap()).unwrap();
        
        let client = self.client.clone();
        let on_load = Closure::wrap(Box::new(move |ev: Event| {
            let file_reader: FileReader = ev.target().unwrap().dyn_into().unwrap();
            let jsvalue = file_reader.result().unwrap();
            let bytes = js_sys::Uint8Array::new(&jsvalue).to_vec();
            let client = client.clone();
            
            wasm_bindgen_futures::spawn_local(async move {
                let upload_result = client.post(format!("{LITESEED_URL}data"))
                .body(bytes)
                .send()
                .await;

                match upload_result {
                    Ok(res) => log!("res {:?}", res.json::<String>().await),
                    Err(e) => log!("{e}")
                };
            });
        }) as Box<dyn FnMut(_)>);
        
        fr.set_onload(Some(on_load.as_ref().unchecked_ref()));
        on_load.forget();
    }
}