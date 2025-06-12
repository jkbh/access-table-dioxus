use dioxus::desktop::{
    tao::event::{DeviceEvent, Event, RawKeyEvent},
    use_wry_event_handler,
};

pub fn use_key_event(mut handler: impl FnMut(&RawKeyEvent) + 'static) {
    use_wry_event_handler(move |evt, _| {
        if let Event::DeviceEvent {
            event: DeviceEvent::Key(raw),
            ..
        } = evt
        {
            handler(raw);
        }
    });
}
