mod app;
mod common {
    pub mod irys;
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