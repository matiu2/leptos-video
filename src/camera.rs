use leptos_use::{use_user_media_with_options, UseUserMediaOptions};

pub fn request_camera_access() {
    // let window = web_sys::window().unwrap();
    // let navigator = window.navigator();
    // let media_devices = navigator.media_devices();
    // let mut constraints = MediaStreamConstraints::new();
    // constraints.video(true);
    // media_devices.get_user_media_with_constraints(&constraints)
    let options = UseUserMediaOptions::default().video(true);
    let leptos_use::UseUserMediaReturn {
        stream,
        start,
        stop,
        enabled,
        set_enabled,
    } = use_user_media_with_options(options);
}
