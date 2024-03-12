use gloo_utils::format::JsValueSerdeExt;
use leptos::*;
use leptos::logging::log;
use wasm_bindgen::JsValue;
use crate::common::irys::{ connect_to_irys, from_atomic, to_atomic, token, FundTx, Tag, UploadConfigs, IRYS_DEVNET_URL, SOLANA_WALLET};
use web_sys::FileList;
use gloo_file::futures::read_as_bytes;
use futures::future::join_all;

#[component]
pub fn App() -> impl IntoView {    
    let file_element: NodeRef<html::Input> = create_node_ref();
    
    let upload_file = create_action(move |fl: &FileList| {
        let fl = fl.clone();
        async move {
            log!("getting price");
            let irys = connect_to_irys(IRYS_DEVNET_URL.to_string(), SOLANA_WALLET.to_string());
            let files = (0..fl.length())
                .map(|i| fl.item(i).unwrap())
                .map(move |file| async move {
                    let bytes = read_as_bytes(&file.into()).await.unwrap();
                    bytes
                })
                .collect::<Vec<_>>();
            let files = join_all(files).await;
            let price = irys.get_price(files.len() as u64).await;
            let cost = price.into_serde::<String>().unwrap().parse::<u64>().unwrap();
            log!("cost {}", cost);

            let atomic_cost = to_atomic(&irys, price);
            let fund_tx = irys.fund(atomic_cost).await;
            log!("Funding status {:?} {}", from_atomic(&irys, JsValue::from(fund_tx.into_serde::<FundTx>().unwrap().quantity)), token(&irys).into_serde::<String>().unwrap());

            let configs: UploadConfigs = UploadConfigs::new(vec![Tag::new("test".to_string(), "test".to_string())]);
            let upload_receipt = irys.upload_file(fl.item(0).unwrap().into(), JsValue::from(configs)).await;
            log!("upload_receipt {:?}", upload_receipt);
        }
    });
    
    view! {
        <div>
            <input 
                type="file" 
                node_ref=file_element 
                on:input=move |_ev| {
                    let file_list = match file_element() {
                        Some(f) => f.files(),
                        None => None
                    };
                    
                    log!("file input {:?}", file_list);

                    upload_file.dispatch(file_list.unwrap())
                } 
            />
        </div>
    }
}