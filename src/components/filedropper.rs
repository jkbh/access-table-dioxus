use dioxus::{document::eval, html::HasFileData, prelude::*};

#[component]
pub fn FileDropzone(ondrop: Callback<DragEvent>) -> Element {
    let mut is_over = use_signal(|| false);
    rsx! {
        div {
            class: "w-full h-full flex justify-center items-center border border-2 border-dashed rounded",
            class: if is_over() { "bg-blue-200" } else { "bg-white" },
            onclick: |_| {
                eval("document.getElementById('file_input').click()");
            },
            ondrop: |evt| {
                evt.prevent_default();
                dbg!(evt.files().unwrap().files());
            },
            ondragenter: move |_| { is_over.set(true) },
            ondragexit: move |_| { is_over.set(false) },
            "Drop file or click to open dialog"
            input {
                r#type: "file",
                class: "hidden",
                id: "file_input",
                onclick: |evt| evt.stop_propagation(),
            }
        }
    }
}
