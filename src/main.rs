use crate::{components::Table, user::create_mock_users};
use dioxus::prelude::*;

mod components;
mod user;
mod utils;

fn main() {
    dioxus::launch(App);
}

#[component]
fn App() -> Element {
    let users = create_mock_users(50, 200);

    rsx! {
        document::Stylesheet { href: asset!("/assets/tailwind.css") }
        div { class: "w-screen h-screen p-1 overflow-none",
            Table { users }
                // FileDropzone { ondrop: |_| {} }
        }
    }
}
