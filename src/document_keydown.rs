use dioxus::prelude::*;
use serde::Deserialize;

#[derive(Deserialize)]
pub struct KeyDownEvent {
    pub key: String,
    pub code: String,
    pub ctrl_key: bool,
    pub shift_key: bool,
    pub alt_key: bool,
    pub meta_key: bool,
}

pub fn use_document_keydown<H: FnMut(KeyDownEvent) + Clone + 'static>(handler: H) {
    use_future(move || {
        let mut handler = handler.clone();
        async move {
            let mut eval = document::eval(
                "document.addEventListener('keydown', (e) => {
                dioxus.send({
                    key: e.key,
                    code: e.code,
                    ctrl_key: e.ctrlKey,
                    shift_key: e.shiftKey,
                    alt_key: e.altKey,
                    meta_key: e.metaKey,
                });
            });",
            );

            loop {
                let res: KeyDownEvent = eval.recv().await.unwrap();
                handler(res);
            }
        }
    });
}
