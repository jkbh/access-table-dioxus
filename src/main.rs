#![windows_subsystem = "windows"]

use crate::{components::table::Table, user::create_mock_users};
use dioxus::prelude::*;

mod components;
mod user;
mod utils;

static TAILWIND: Asset = asset!("/assets/tailwind.css");
fn main() {
    dioxus::launch(App);
}

#[component]
fn App() -> Element {
    let users = create_mock_users(50, 200);

    rsx! {
        document::Stylesheet { href: TAILWIND }
        div { class: "w-screen h-screen p-1 overflow-none",
            Table { users }
                // FileDropzone { ondrop: |_| {} }
        }
    }
}
