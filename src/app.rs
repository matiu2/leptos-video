use html::Video;
use leptos::*;
use leptos_meta::*;
use leptos_router::*;
use leptos_use::{use_user_media_with_options, UseUserMediaOptions, UseUserMediaReturn};
use web_sys::HtmlMediaElement;

#[component]
pub fn App() -> impl IntoView {
    // Provides context that manages stylesheets, titles, meta tags, etc.
    provide_meta_context();

    view! {
        // injects a stylesheet into the document <head>
        // id=leptos means cargo-leptos will hot-reload this stylesheet
        <Stylesheet id="leptos" href="/pkg/qr-storage-app.css"/>

        // sets the document title
        <Title text="QR Storage app"/>

        // content for this welcome page
        <Router>
            <main>
                <Routes>
                    <Route path="" view=HomePage/>
                    <Route path="/*any" view=NotFound/>
                </Routes>
            </main>
        </Router>
    }
}

/// Renders the home page of your application.
#[component]
fn HomePage() -> impl IntoView {
    // Creates a reactive value to update the button
    let options = UseUserMediaOptions::default().video(true);
    let UseUserMediaReturn {
        stream,
        start,
        stop,
        enabled,
        set_enabled,
    } = use_user_media_with_options(options);
    let stream_debug = move || stream.with(|stream| format!("Here's the stream: {stream:#?}"));
    let stream_id = move || {
        stream.with(|stream| match stream {
            Some(Ok(stream)) => stream.id(),
            _ => "No ID yet".to_string(),
        })
    };
    let enabled = move || enabled.with(|enabled| if *enabled { "Enabled" } else { "Disabled" });
    let video = create_node_ref::<Video>();
    create_effect(move |_| {
        if let Some(video) = video.get() {
            if let Some(Ok(stream)) = stream() {
                (&video as &HtmlMediaElement).set_src_object(Some(&stream));
                let _ = video.play();
                return true;
            }
        }
        false
    });

    view! {
        <h1>"Welcome to Leptos!"</h1>
        <p>{stream_debug}</p>
        <button on:click=move |_| start()>"start"</button>
        <button on:click=move |_| stop()>"stop"</button>
        <button on:click=move |_| set_enabled(true)>"Enable"</button>
        <button on:click=move |_| set_enabled(false)>"Disable"</button>
        <p>{enabled}</p>
        <p>Stream Id: {stream_id}</p>
        <video _ref=video width=640 height=480 ></video>
    }
}

/// 404 - Not Found
#[component]
fn NotFound() -> impl IntoView {
    // set an HTTP status code 404
    // this is feature gated because it can only be done during
    // initial server-side rendering
    // if you navigate to the 404 page subsequently, the status
    // code will not be set because there is not a new HTTP request
    // to the server
    #[cfg(feature = "ssr")]
    {
        // this can be done inline because it's synchronous
        // if it were async, we'd use a server function
        let resp = expect_context::<leptos_actix::ResponseOptions>();
        resp.set_status(actix_web::http::StatusCode::NOT_FOUND);
    }

    view! {
        <h1>"Not Found"</h1>
    }
}
