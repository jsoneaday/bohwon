use leptos::*;
use leptos::logging::log;
use web_sys::FileList;
use web_sys::File;
use crate::common::api::liteseed_api::LsApi;

#[component]
pub fn App() -> impl IntoView {    
    let file_element: NodeRef<html::Input> = create_node_ref();
    
    let upload_file = create_action(move |fl: &FileList| {
        let fl = fl.clone();
        let api = LsApi::new();
        async move {
            log!("getting price");
            let files = (0..fl.length())
                .map(|i| fl.item(i).unwrap())
                .collect::<Vec<File>>();

            let price = api.get_cost(files.get(0).unwrap().size()).await;
            if let Ok(price) = price { 
                log!("price: {price}");
            } else { 
                log!("error: {}", price.err().unwrap());
            }

            
            api.upload_file(files.get(0).unwrap()).await;
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