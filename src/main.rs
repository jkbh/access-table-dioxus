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
        Table { users }
    }
}
