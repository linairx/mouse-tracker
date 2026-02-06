use leptos::prelude::*;
use leptos_meta::{provide_meta_context, MetaTags, Stylesheet, Title};
use leptos_router::{
    components::{Route, Router, Routes},
    StaticSegment,
};

#[cfg(feature = "hydrate")]
use crate::types::MouseEvent;

#[cfg(feature = "hydrate")]
use wasm_bindgen::prelude::*;

#[cfg(feature = "hydrate")]
use wasm_bindgen_futures::JsFuture;

#[cfg(feature = "hydrate")]
use web_sys::{Request, RequestInit, RequestMode, Headers, Response};

pub fn shell(options: LeptosOptions) -> impl IntoView {
    view! {
        <!DOCTYPE html>
        <html lang="en">
            <head>
                <meta charset="utf-8"/>
                <meta name="viewport" content="width=device-width, initial-scale=1"/>
                <AutoReload options=options.clone() />
                <HydrationScripts options/>
                <MetaTags/>
            </head>
            <body>
                <App/>
            </body>
        </html>
    }
}

#[component]
pub fn App() -> impl IntoView {
    // Provides context that manages stylesheets, titles, meta tags, etc.
    provide_meta_context();

    view! {
        // injects a stylesheet into the document <head>
        // id=leptos means cargo-leptos will hot-reload this stylesheet
        <Stylesheet id="leptos" href="/pkg/mouse-tracker.css"/>

        // sets the document title
        <Title text="Mouse Tracker"/>

        // content for this welcome page
        <Router>
            <main>
                <Routes fallback=|| "Page not found.".into_view()>
                    <Route path=StaticSegment("") view=HomePage/>
                </Routes>
            </main>
        </Router>
    }
}

/// Renders the home page of your application.
#[component]
fn HomePage() -> impl IntoView {
    let event_count = RwSignal::new(0u32);

    view! {
        <div style="padding: 20px;">
            <h1>"Mouse Tracker Demo"</h1>
            <p>"移动鼠标或点击以记录事件"</p>
            <p>"已记录事件数: " {event_count}</p>

            <div
                style="width: 100%; height: 400px; background: #f0f0f0; border: 2px solid #ccc;"
                on:mousemove=move |_e| {
                    event_count.update(|n| *n += 1);
                    #[cfg(feature = "hydrate")]
                    send_event("mousemove", _e.client_x(), _e.client_y(), None);
                }
                on:mousedown=move |e| {
                    event_count.update(|n| *n += 1);
                    let _button = match e.button() {
                        0 => "left",
                        1 => "middle",
                        2 => "right",
                        _ => "unknown",
                    };
                    #[cfg(feature = "hydrate")]
                    send_event("mousedown", e.client_x(), e.client_y(), Some(("button".to_string(), _button.to_string())));
                }
                on:mouseup=move |e| {
                    event_count.update(|n| *n += 1);
                    let _button = match e.button() {
                        0 => "left",
                        1 => "middle",
                        2 => "right",
                        _ => "unknown",
                    };
                    #[cfg(feature = "hydrate")]
                    send_event("mouseup", e.client_x(), e.client_y(), Some(("button".to_string(), _button.to_string())));
                }
                on:wheel=move |e| {
                    event_count.update(|n| *n += 1);
                    let _scroll_y = e.delta_y().to_string();
                    #[cfg(feature = "hydrate")]
                    send_event("wheel", e.client_x(), e.client_y(), Some(("scroll_y".to_string(), _scroll_y)));
                }
            >
                <p style="padding: 20px; text-align: center; color: #666;">
                    "在此区域内移动鼠标、点击或滚动"
                </p>
            </div>

            <p style="margin-top: 20px; font-size: 14px; color: #666;">
                "事件将保存到服务器上的 mouse_events.jsonl 文件"
            </p>
        </div>
    }
}

#[cfg(feature = "hydrate")]
fn send_event(event_type: &str, x: i32, y: i32, extra: Option<(String, String)>) {
    let timestamp = js_sys::Date::now() as u64;
    let mut mouse_event = MouseEvent {
        event_type: event_type.to_string(),
        x,
        y,
        timestamp,
        target: None,
        button: None,
        scroll_y: None,
    };

    if let Some((key, value)) = extra {
        match key.as_str() {
            "button" => mouse_event.button = Some(value),
            "scroll_y" => mouse_event.scroll_y = value.parse().ok(),
            _ => {}
        }
    }

    let event_json = serde_json::to_string(&mouse_event).unwrap();
    wasm_bindgen_futures::spawn_local(async move {
        if let Err(e) = send_to_server(&event_json).await {
            web_sys::console::log_1(&format!("Failed to send event: {:?}", e).into());
        }
    });
}

#[cfg(feature = "hydrate")]
async fn send_to_server(event_json: &str) -> Result<(), JsValue> {
    let opts = RequestInit::new();
    opts.set_method("POST");
    opts.set_mode(RequestMode::Cors);

    let headers = Headers::new()?;
    headers.append("Content-Type", "application/json")?;
    opts.set_headers(&headers);

    opts.set_body(&JsValue::from_str(event_json));

    let request = Request::new_with_str_and_init("/api/mouse", &opts)?;
    let window = web_sys::window().ok_or(JsValue::from_str("Window not available"))?;

    let promise = window.fetch_with_request(&request);
    let resp_value: JsValue = JsFuture::from(promise).await?;
    let resp: Response = resp_value.dyn_into()?;

    if resp.ok() {
        Ok(())
    } else {
        Err(JsValue::from_str("Request failed"))
    }
}
