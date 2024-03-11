use leptos::{logging::log, *};
use crate::common::irys::{ connect_to_irys, WebIrys, IRYS_DEVNET_URL, SOLANA_WALLET};
use gloo_utils::format::JsValueSerdeExt;
use std::fs::File;
use web_sys::FileList;

#[component]
pub fn App() -> impl IntoView {
    let file_element: NodeRef<html::Input> = create_node_ref();

    let irys = create_action(|_: &()| async move {
        let irys: WebIrys = connect_to_irys(IRYS_DEVNET_URL.to_string(), SOLANA_WALLET.to_string()).await;
        
        // let payload = File::open("/wasm/typescript.svg").unwrap();
        // let price = irys.get_price(payload.metadata().unwrap().len()).await;
        // let final_val = price.into_serde::<String>().unwrap().parse::<u64>().unwrap();
        // log!("irys payload, cost {:?}", final_val);
    });
    irys.dispatch(());
    
    view! {
        <div>
            <input 
                type="file" 
                node_ref=file_element 
                on:click=move |_ev| {
                    log!("clicked");
                } 
                on:change=move |_ev| {
                    log!("file changed");
                    let _result = match file_element() {
                        Some(f) => f.files(),
                        None => None
                    };
                }
            />
        </div>
    }
}