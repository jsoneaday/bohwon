mod app;

use leptos::*;
use crate::app::App;

pub fn run() {
    mount_to_body(|| {
        view! {
            <App />
        }
    })
}