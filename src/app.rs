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

    #[cfg(feature = "hydrate")]
    let (event_buffer, debounced_send) = create_debounced_sender();

    #[cfg(feature = "hydrate")]
    let buffer1 = event_buffer.clone();
    #[cfg(feature = "hydrate")]
    let sender1 = debounced_send.clone();
    #[cfg(feature = "hydrate")]
    let buffer2 = event_buffer.clone();
    #[cfg(feature = "hydrate")]
    let sender2 = debounced_send.clone();
    #[cfg(feature = "hydrate")]
    let buffer3 = event_buffer.clone();
    #[cfg(feature = "hydrate")]
    let sender3 = debounced_send.clone();
    #[cfg(feature = "hydrate")]
    let buffer4 = event_buffer.clone();
    #[cfg(feature = "hydrate")]
    let sender4 = debounced_send.clone();

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
                    queue_event("mousemove", _e.client_x(), _e.client_y(), None, &buffer1, &sender1);
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
                    queue_event("mousedown", e.client_x(), e.client_y(), Some(("button".to_string(), _button.to_string())), &buffer2, &sender2);
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
                    queue_event("mouseup", e.client_x(), e.client_y(), Some(("button".to_string(), _button.to_string())), &buffer3, &sender3);
                }
                on:wheel=move |e| {
                    event_count.update(|n| *n += 1);
                    let _scroll_y = e.delta_y().to_string();
                    #[cfg(feature = "hydrate")]
                    queue_event("wheel", e.client_x(), e.client_y(), Some(("scroll_y".to_string(), _scroll_y)), &buffer4, &sender4);
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
use std::cell::RefCell;
#[cfg(feature = "hydrate")]
use std::rc::Rc;
#[cfg(feature = "hydrate")]
use js_sys::Function;

/// 创建防抖发送器
/// 返回：(事件缓冲区, 防抖发送函数)
#[cfg(feature = "hydrate")]
fn create_debounced_sender() -> (Rc<RefCell<Vec<MouseEvent>>>, Rc<impl Fn()>) {
    let event_buffer = Rc::new(RefCell::new(Vec::new()));
    let timeout_handle = Rc::new(RefCell::new(None::<i32>));

    let buffer_clone = event_buffer.clone();
    let handle_clone = timeout_handle.clone();

    let debounced_send = move || {
        // 清除之前的定时器
        if let Some(handle) = *handle_clone.borrow() {
            let window = web_sys::window().expect("Window not available");
            window.clear_timeout_with_handle(handle);
        }

        // 设置新的定时器（500ms）
        let buffer = buffer_clone.clone();
        let handle = handle_clone.clone();
        let window = web_sys::window().expect("Window not available");

        let callback = Closure::wrap(Box::new(move || {
            // 发送缓冲区中的所有事件
            let events = buffer.borrow_mut().drain(..).collect::<Vec<_>>();

            if !events.is_empty() {
                web_sys::console::log_1(&format!("Sending {} events after debounce", events.len()).into());

                wasm_bindgen_futures::spawn_local(async move {
                    if let Err(e) = send_events_batch(&events).await {
                        web_sys::console::log_1(&format!("Failed to send events: {:?}", e).into());
                    }
                });
            }
        }) as Box<dyn FnMut()>);

        // 将 Closure 转换为 Function
        let callback_ptr = callback.as_ref().dyn_ref::<Function>().unwrap();
        let timeout_id = window.set_timeout_with_callback_and_timeout_and_arguments_0(
            callback_ptr,
            500,
        ).expect("Failed to set timeout");

        // 忘记 Closure 以保持其存活（直到定时器触发）
        callback.forget();

        *handle.borrow_mut() = Some(timeout_id);
    };

    (event_buffer, Rc::new(debounced_send))
}

/// 将事件添加到缓冲区并触发防抖
#[cfg(feature = "hydrate")]
fn queue_event(
    event_type: &str,
    x: i32,
    y: i32,
    extra: Option<(String, String)>,
    buffer: &Rc<RefCell<Vec<MouseEvent>>>,
    debounced_send: &Rc<impl Fn()>,
) {
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

    // 添加到缓冲区
    buffer.borrow_mut().push(mouse_event);

    // 触发防抖
    debounced_send();
}

/// 批量发送事件到服务器
#[cfg(feature = "hydrate")]
async fn send_events_batch(events: &[MouseEvent]) -> Result<(), JsValue> {
    let events_json = serde_json::to_string(events).unwrap();
    send_to_server(&events_json).await
}

/// 发送 JSON 数据到服务器
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
