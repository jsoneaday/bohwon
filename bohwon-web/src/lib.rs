mod app;
mod common {
    pub mod irys;
    pub mod api {
        pub mod liteseed_api;
        pub mod liteseed_types;
    }
}

use leptos::*;
use crate::app::App;

pub fn run() {
    mount_to_body(|| {
        view! {
            <App />
        }
    })
}