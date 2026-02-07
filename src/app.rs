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
use web_sys::{Request, RequestInit, RequestMode, Headers, Response, EventTarget, HtmlElement, KeyboardEvent};

#[cfg(feature = "hydrate")]
use std::cell::RefCell;
#[cfg(feature = "hydrate")]
use std::rc::Rc;
#[cfg(feature = "hydrate")]
use js_sys::Function;
#[cfg(feature = "hydrate")]
use std::collections;

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
    let (event_buffer, debounced_send, tracking_state) = create_tracking_system();

    // 为每个事件处理器创建克隆的引用
    #[cfg(feature = "hydrate")]
    let buffer_mv = event_buffer.clone();
    #[cfg(feature = "hydrate")]
    let sender_mv = debounced_send.clone();
    #[cfg(feature = "hydrate")]
    let state_mv = tracking_state.clone();

    #[cfg(feature = "hydrate")]
    let buffer_md = event_buffer.clone();
    #[cfg(feature = "hydrate")]
    let sender_md = debounced_send.clone();
    #[cfg(feature = "hydrate")]
    let state_md = tracking_state.clone();

    #[cfg(feature = "hydrate")]
    let buffer_mu = event_buffer.clone();
    #[cfg(feature = "hydrate")]
    let sender_mu = debounced_send.clone();
    #[cfg(feature = "hydrate")]
    let state_mu = tracking_state.clone();
    #[cfg(feature = "hydrate")]
    let state_wh = tracking_state.clone();

    #[cfg(feature = "hydrate")]
    let buffer_wh = event_buffer.clone();
    #[cfg(feature = "hydrate")]
    let sender_wh = debounced_send.clone();

    #[cfg(feature = "hydrate")]
    let buffer_ds = event_buffer.clone();
    #[cfg(feature = "hydrate")]
    let sender_ds = debounced_send.clone();
    #[cfg(feature = "hydrate")]
    let state_ds = tracking_state.clone();

    #[cfg(feature = "hydrate")]
    let buffer_d = event_buffer.clone();
    #[cfg(feature = "hydrate")]
    let sender_d = debounced_send.clone();
    #[cfg(feature = "hydrate")]
    let state_d = tracking_state.clone();

    #[cfg(feature = "hydrate")]
    let buffer_de = event_buffer.clone();
    #[cfg(feature = "hydrate")]
    let sender_de = debounced_send.clone();
    #[cfg(feature = "hydrate")]
    let state_de = tracking_state.clone();

    #[cfg(feature = "hydrate")]
    let buffer_kd = event_buffer.clone();
    #[cfg(feature = "hydrate")]
    let sender_kd = debounced_send.clone();
    #[cfg(feature = "hydrate")]
    let state_kd = tracking_state.clone();

    #[cfg(feature = "hydrate")]
    let buffer_ku = event_buffer.clone();
    #[cfg(feature = "hydrate")]
    let sender_ku = debounced_send.clone();
    #[cfg(feature = "hydrate")]
    let state_ku = tracking_state.clone();

    view! {
        <div style="padding: 20px;">
            <h1>"Mouse Tracker Demo - IL Training Data"</h1>
            <p>"移动鼠标、点击、拖拽、按键以记录交互事件"</p>
            <p>"已记录事件数: " {event_count}</p>
            <p>"会话 ID: " {move || {
                #[cfg(feature = "hydrate")]
                return tracking_state.session_id.clone();
                #[cfg(not(feature = "hydrate"))]
                return "N/A".to_string();
            }}</p>

            // 全局键盘事件监听
            <div
                style="width: 100%; height: 500px; background: #f0f0f0; border: 2px solid #ccc; position: relative; user-select: none;"
                tabindex="0"
                on:mousemove=move |e| {
                    event_count.update(|n| *n += 1);
                    #[cfg(feature = "hydrate")]
                    {
                        let mut extra = std::collections::HashMap::new();
                        extra.insert("buttons".to_string(), e.buttons().to_string());

                        let mouse_event = create_mouse_event("mousemove", e.client_x(), e.client_y(), &state_mv, Some(extra), &e);
                        queue_enriched_event(mouse_event, &buffer_mv, &sender_mv);
                    }
                }
                on:mousedown=move |e| {
                    event_count.update(|n| *n += 1);
                    #[cfg(feature = "hydrate")]
                    {
                        let button = match e.button() {
                            0 => "left",
                            1 => "middle",
                            2 => "right",
                            _ => "unknown",
                        };

                        let mut extra = std::collections::HashMap::new();
                        extra.insert("button".to_string(), button.to_string());
                        extra.insert("buttons".to_string(), e.buttons().to_string());

                        let mouse_event = create_mouse_event("mousedown", e.client_x(), e.client_y(), &state_md, Some(extra), &e);
                        queue_enriched_event(mouse_event, &buffer_md, &sender_md);
                    }
                }
                on:mouseup=move |e| {
                    event_count.update(|n| *n += 1);
                    #[cfg(feature = "hydrate")]
                    {
                        let button = match e.button() {
                            0 => "left",
                            1 => "middle",
                            2 => "right",
                            _ => "unknown",
                        };

                        let mut extra = std::collections::HashMap::new();
                        extra.insert("button".to_string(), button.to_string());
                        extra.insert("buttons".to_string(), e.buttons().to_string());

                        let mouse_event = create_mouse_event("mouseup", e.client_x(), e.client_y(), &state_mu, Some(extra), &e);
                        queue_enriched_event(mouse_event, &buffer_mu, &sender_mu);
                    }
                }
                on:wheel=move |e| {
                    event_count.update(|n| *n += 1);
                    #[cfg(feature = "hydrate")]
                    {
                        let mut extra = std::collections::HashMap::new();
                        extra.insert("scroll_y".to_string(), e.delta_y().to_string());
                        extra.insert("scroll_x".to_string(), e.delta_x().to_string());

                        let mouse_event = create_mouse_event("wheel", e.client_x(), e.client_y(), &state_wh, Some(extra), &e);
                        queue_enriched_event(mouse_event, &buffer_wh, &sender_wh);
                    }
                }
                on:dragstart=move |e| {
                    event_count.update(|n| *n += 1);
                    #[cfg(feature = "hydrate")]
                    {
                        let mouse_event = create_mouse_event("dragstart", e.client_x(), e.client_y(), &state_ds, None, &e);

                        // 记录拖拽开始事件 ID
                        *state_ds.drag_state.borrow_mut() = Some(mouse_event.event_id.clone());

                        queue_enriched_event(mouse_event, &buffer_ds, &sender_ds);
                    }
                }
                on:drag=move |e| {
                    event_count.update(|n| *n += 1);
                    #[cfg(feature = "hydrate")]
                    {
                        let mouse_event = create_mouse_event("drag", e.client_x(), e.client_y(), &state_d, None, &e);
                        queue_enriched_event(mouse_event, &buffer_d, &sender_d);
                    }
                }
                on:dragend=move |e| {
                    event_count.update(|n| *n += 1);
                    #[cfg(feature = "hydrate")]
                    {
                        let mouse_event = create_mouse_event("dragend", e.client_x(), e.client_y(), &state_de, None, &e);

                        // 清除拖拽状态
                        *state_de.drag_state.borrow_mut() = None;

                        queue_enriched_event(mouse_event, &buffer_de, &sender_de);
                    }
                }
                on:keydown=move |e| {
                    event_count.update(|n| *n += 1);
                    #[cfg(feature = "hydrate")]
                    {
                        let keyboard_evt = e.dyn_ref::<web_sys::KeyboardEvent>().unwrap();
                        let mut extra = std::collections::HashMap::new();
                        extra.insert("key".to_string(), keyboard_evt.key());
                        extra.insert("code".to_string(), keyboard_evt.code());
                        extra.insert("ctrl_key".to_string(), keyboard_evt.ctrl_key().to_string());
                        extra.insert("shift_key".to_string(), keyboard_evt.shift_key().to_string());
                        extra.insert("alt_key".to_string(), keyboard_evt.alt_key().to_string());
                        extra.insert("meta_key".to_string(), keyboard_evt.meta_key().to_string());

                        let mouse_event = create_mouse_event("keydown", 0, 0, &state_kd, Some(extra), &e);
                        queue_enriched_event(mouse_event, &buffer_kd, &sender_kd);
                    }
                }
                on:keyup=move |e| {
                    event_count.update(|n| *n += 1);
                    #[cfg(feature = "hydrate")]
                    {
                        let keyboard_evt = e.dyn_ref::<web_sys::KeyboardEvent>().unwrap();
                        let mut extra = std::collections::HashMap::new();
                        extra.insert("key".to_string(), keyboard_evt.key());
                        extra.insert("code".to_string(), keyboard_evt.code());
                        extra.insert("ctrl_key".to_string(), keyboard_evt.ctrl_key().to_string());
                        extra.insert("shift_key".to_string(), keyboard_evt.shift_key().to_string());
                        extra.insert("alt_key".to_string(), keyboard_evt.alt_key().to_string());
                        extra.insert("meta_key".to_string(), keyboard_evt.meta_key().to_string());

                        let mouse_event = create_mouse_event("keyup", 0, 0, &state_ku, Some(extra), &e);
                        queue_enriched_event(mouse_event, &buffer_ku, &sender_ku);
                    }
                }
            >
                <p style="padding: 20px; text-align: center; color: #666;">
                    "在此区域内移动鼠标、点击、拖拽或按键"
                </p>
                <div
                    style="width: 100px; height: 100px; background: #4CAF50; color: white; display: flex; align-items: center; justify-content: center; position: absolute; top: 50%; left: 50%; transform: translate(-50%, -50%); cursor: move;"
                    draggable="true"
                >
                    "拖拽我!"
                </div>
            </div>

            <p style="margin-top: 20px; font-size: 14px; color: #666;">
                "事件将保存到服务器上的 mouse_events.jsonl 文件"
            </p>
            <p style="font-size: 12px; color: #999;">
                "支持的事件: mousemove, mousedown, mouseup, wheel, dragstart, drag, dragend, keydown, keyup"
            </p>
        </div>
    }
}

// ============================================================================
// IL 训练数据收集 - 会话和事件管理
// ============================================================================

#[cfg(feature = "hydrate")]
#[derive(Clone)]
pub struct TrackingState {
    pub session_id: String,
    pub event_counter: Rc<RefCell<u64>>,
    pub last_event: Rc<RefCell<Option<(MouseEvent, f64)>>>, // (上一次事件, 上一次时间戳)
    pub drag_state: Rc<RefCell<Option<String>>>, // 当前拖拽的事件 ID
}

#[cfg(feature = "hydrate")]
impl TrackingState {
    pub fn new() -> Self {
        let session_id = format!("session_{}", js_sys::Date::now());
        Self {
            session_id,
            event_counter: Rc::new(RefCell::new(0)),
            last_event: Rc::new(RefCell::new(None)),
            drag_state: Rc::new(RefCell::new(None)),
        }
    }

    pub fn generate_event_id(&self) -> String {
        let count = *self.event_counter.borrow();
        *self.event_counter.borrow_mut() += 1;
        format!("event_{}_{}", self.session_id, count)
    }
}

/// 创建防抖发送器和追踪状态
/// 返回：(事件缓冲区, 防抖发送函数, 追踪状态)
#[cfg(feature = "hydrate")]
fn create_tracking_system() -> (Rc<RefCell<Vec<MouseEvent>>>, Rc<impl Fn()>, TrackingState) {
    let event_buffer = Rc::new(RefCell::new(Vec::new()));
    let timeout_handle = Rc::new(RefCell::new(None::<i32>));
    let tracking_state = TrackingState::new();

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

        let callback_ptr = callback.as_ref().dyn_ref::<Function>().unwrap();
        let timeout_id = window.set_timeout_with_callback_and_timeout_and_arguments_0(
            callback_ptr,
            500,
        ).expect("Failed to set timeout");

        callback.forget();
        *handle.borrow_mut() = Some(timeout_id);
    };

    (event_buffer, Rc::new(debounced_send), tracking_state)
}

/// 获取目标元素信息
#[cfg(feature = "hydrate")]
fn get_target_info(event: &web_sys::Event) -> (Option<String>, Option<String>, Option<String>, Option<String>) {
    let target = event.target();
    let mut tag_name = None;
    let mut id = None;
    let mut class_name = None;
    let mut text = None;

    if let Some(target) = target {
        if let Some(element) = target.dyn_ref::<web_sys::Element>() {
            tag_name = Some(element.tag_name().to_lowercase());
            id = element.get_attribute("id");
            class_name = element.get_attribute("class");

            // 获取文本内容（限制长度）
            if let Some(html_element) = element.dyn_ref::<web_sys::HtmlElement>() {
                let txt = html_element.inner_text();
                // 按字符而非字节截断，避免在中文字符中间切分
                let truncated: String = txt.chars().take(50).collect();
                if txt.chars().count() > 50 {
                    text = Some(format!("{}...", truncated));
                } else if !txt.is_empty() {
                    text = Some(txt);
                }
            }
        }
    }

    (tag_name, id, class_name, text)
}

/// 计算速度和距离
#[cfg(feature = "hydrate")]
fn calculate_velocity_and_distance(
    last_event: &Option<(MouseEvent, f64)>,
    current_x: i32,
    current_y: i32,
    current_time: f64,
) -> (Option<f64>, Option<f64>, Option<f64>) {
    if let Some((last, last_time)) = last_event {
        let dx = (current_x - last.x) as f64;
        let dy = (current_y - last.y) as f64;
        let dt = current_time - last_time;

        if dt > 0.0 {
            let distance = (dx * dx + dy * dy).sqrt();
            let velocity_x = dx / dt;
            let velocity_y = dy / dt;

            return (Some(velocity_x), Some(velocity_y), Some(distance));
        }
    }

    (None, None, None)
}

/// 创建完整的事件对象
#[cfg(feature = "hydrate")]
fn create_mouse_event(
    event_type: &str,
    x: i32,
    y: i32,
    tracking_state: &TrackingState,
    extra: Option<std::collections::HashMap<String, String>>,
    event: &web_sys::Event,
) -> MouseEvent {
    let timestamp = js_sys::Date::now();
    let current_time = timestamp as f64;

    // 生成事件 ID
    let event_id = tracking_state.generate_event_id();

    // 获取目标元素信息
    let (target_tag, target_id, target_class, target_text) = get_target_info(event);

    // 安全地获取上一个事件数据（使用 try_borrow 避免冲突）
    let last_event_data = tracking_state.last_event.try_borrow().ok().and_then(|b| b.as_ref().cloned());

    // 计算速度和距离
    let (vel_x, vel_y, dist) = calculate_velocity_and_distance(
        &last_event_data,
        x,
        y,
        current_time,
    );

    // 获取视口信息
    let window = web_sys::window().expect("Window not available");
    let viewport_width = Some(window.inner_width().unwrap().as_f64().unwrap() as u32);
    let viewport_height = Some(window.inner_height().unwrap().as_f64().unwrap() as u32);

    // 安全地获取拖拽状态
    let parent_event_id = tracking_state.drag_state.try_borrow().ok().and_then(|b| b.as_ref().cloned());

    // 处理额外信息
    let mut button = None;
    let mut buttons = None;
    let mut scroll_y = None;
    let mut scroll_x = None;
    let mut key = None;
    let mut code = None;
    let mut ctrl_key = None;
    let mut shift_key = None;
    let mut alt_key = None;
    let mut meta_key = None;

    if let Some(extra) = extra {
        if let Some(v) = extra.get("button") { button = Some(v.clone()); }
        if let Some(v) = extra.get("buttons") { buttons = v.parse::<u16>().ok(); }
        if let Some(v) = extra.get("scroll_y") { scroll_y = v.parse::<f64>().ok(); }
        if let Some(v) = extra.get("scroll_x") { scroll_x = v.parse::<f64>().ok(); }
        if let Some(v) = extra.get("key") { key = Some(v.clone()); }
        if let Some(v) = extra.get("code") { code = Some(v.clone()); }
        if let Some(v) = extra.get("ctrl_key") { ctrl_key = v.parse::<bool>().ok(); }
        if let Some(v) = extra.get("shift_key") { shift_key = v.parse::<bool>().ok(); }
        if let Some(v) = extra.get("alt_key") { alt_key = v.parse::<bool>().ok(); }
        if let Some(v) = extra.get("meta_key") { meta_key = v.parse::<bool>().ok(); }
    }

    let mouse_event = MouseEvent {
        event_type: event_type.to_string(),
        timestamp: timestamp as u64,
        x,
        y,
        screen_x: Some(x),  // 简化处理，使用 client_x/y
        screen_y: Some(y),
        page_x: Some(x),
        page_y: Some(y),
        button,
        buttons,
        scroll_y,
        scroll_x,
        target: None,
        target_tag,
        target_id,
        target_class,
        target_text,
        session_id: tracking_state.session_id.clone(),
        event_id: event_id.clone(),
        parent_event_id,
        velocity_x: vel_x,
        velocity_y: vel_y,
        distance: dist,
        key,
        code,
        ctrl_key,
        shift_key,
        alt_key,
        meta_key,
        viewport_width,
        viewport_height,
        metadata: None,
    };

    // 尝试更新上一个事件（如果借用失败则跳过）
    if let Ok(mut last) = tracking_state.last_event.try_borrow_mut() {
        *last = Some((mouse_event.clone(), current_time));
    }

    mouse_event
}

/// 将事件添加到缓冲区并触发防抖
#[cfg(feature = "hydrate")]
fn queue_enriched_event(
    mouse_event: MouseEvent,
    buffer: &Rc<RefCell<Vec<MouseEvent>>>,
    debounced_send: &Rc<impl Fn()>,
) {
    buffer.borrow_mut().push(mouse_event);
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
