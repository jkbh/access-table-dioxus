use dioxus::prelude::*;
use dioxus_test::{create_mock_users, table::Table};

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
