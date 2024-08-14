use leptos::{window, WriteSignal};
use wasm_bindgen::prelude::*;
use web_sys::{MediaDevices, MediaStreamConstraints};

pub fn request_camera_access(w: WriteSignal<String>) -> Result<(), JsValue> {
    // let window = web_sys::window().unwrap();
    // let navigator = window.navigator();
    // let media_devices = navigator.media_devices();
    // let mut constraints = MediaStreamConstraints::new();
    // constraints.video(true);
    // media_devices.get_user_media_with_constraints(&constraints)
    let mut constraints = MediaStreamConstraints::new();
    let constraints = constraints.video(&JsValue::TRUE);
    let promise = window()
        .navigator()
        .media_devices()?
        .get_user_media_with_constraints(constraints)?;
    let good_callback = move |value: JsValue| match value.dyn_into::<MediaDevices>() {
        Ok(_video) => {
            w("Got media devices".to_string());
        }
        Err(err) => {
            w(format!("Media devices failed: {err:?}"));
        }
    };
    let bad_callback = move |value: JsValue| {
        w(format!("Bad {value:?}"));
    };
    let _promise = promise.then2(&Closure::new(good_callback), &Closure::new(bad_callback));
    Ok(())
}

pub fn create_video_stream(stream: JsValue) -> Result<JsValue, JsValue> {
    todo!()
}
