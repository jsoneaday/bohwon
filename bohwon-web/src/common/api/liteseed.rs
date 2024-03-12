use leptos::logging::log;
use reqwest::{Client, StatusCode};

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

    pub async fn get_cost(&self, size: f64) {
        let cost_result = self.client.get(format!("{LITESEED_URL}cost/{size}"))
            .send()
            .await;

        match cost_result {
            Ok(resp) => {
                match resp.status() {
                    StatusCode::OK => {
                        let result = resp.json::<String>().await;
                        if result.is_err() {
                            log!("Err: {:?}", result.err().unwrap());
                        } else {
                            log!("Ok: {:?}", result.ok().unwrap());
                        }
                        ()
                    },
                    _ => log!("{}", resp.error_for_status().err().unwrap())
                }
            },
            Err(e) => log!("{e}")
        }
    }
}