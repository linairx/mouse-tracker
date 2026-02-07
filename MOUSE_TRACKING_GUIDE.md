# 鼠标事件追踪 - 实现指南

> 这是一个基于 Leptos 框架的鼠标事件追踪功能，支持防抖优化和批量数据发送。

## 📋 功能特性

- ✅ 实时捕获多种鼠标事件（mousemove, mousedown, mouseup, wheel）
- ✅ 500ms 防抖机制，减少网络请求
- ✅ 批量事件发送优化
- ✅ 前后端类型安全（共享 Rust 类型）
- ✅ 异步数据处理，不阻塞 UI

## 🏗️ 架构设计

```
┌─────────────┐      防抖缓冲区      ┌──────────┐
│  浏览器端   │ ──────→ 事件队列 ───→ │  批量发送 │
│ (Leptos)    │      500ms 静止后    │  API     │
└─────────────┘                     └──────────┘
                                          │
                                          ↓
                                    ┌──────────┐
                                    │ Axum API │
                                    └──────────┘
                                          │
                                          ↓
                                    ┌──────────┐
                                    │ JSONL    │
                                    │ 日志文件 │
                                    └──────────┘
```

## 📦 核心文件

### 1. 类型定义 (`src/types.rs`)
```rust
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MouseEvent {
    pub event_type: String,  // "mousemove", "mousedown", "mouseup", "wheel"
    pub x: i32,
    pub y: i32,
    pub timestamp: u64,
    pub target: Option<String>,
    pub button: Option<String>,    // "left", "middle", "right"
    pub scroll_y: Option<f64>,
}
```

### 2. 前端实现 (`src/app.rs`)

#### 依赖项
```rust
// Cargo.toml
[dependencies]
leptos = { version = "0.6", features = ["serde"] }
serde = { version = "1", features = ["derive"] }
serde_json = "1"
wasm-bindgen = "0.2"
wasm-bindgen-futures = "0.4"
web-sys = "0.3"
js-sys = "0.3"

[target.'cfg(target_arch = "wasm32")'.dependencies]
console_error_panic_hook = "0.1"
```

#### 核心代码模块

**防抖发送器创建：**
```rust
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
            let events = buffer.borrow_mut().drain(..).collect::<Vec<_>>();

            if !events.is_empty() {
                wasm_bindgen_futures::spawn_local(async move {
                    if let Err(e) = send_events_batch(&events).await {
                        web_sys::console::log_1(&format!("Failed to send events: {:?}", e).into());
                    }
                });
            }
        }) as Box<dyn FnMut()>);

        let callback_ptr = callback.as_ref().dyn_ref::<js_sys::Function>().unwrap();
        let timeout_id = window.set_timeout_with_callback_and_timeout_and_arguments_0(
            callback_ptr,
            500,
        ).expect("Failed to set timeout");

        callback.forget();
        *handle.borrow_mut() = Some(timeout_id);
    };

    (event_buffer, Rc::new(debounced_send))
}
```

**事件队列函数：**
```rust
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

    buffer.borrow_mut().push(mouse_event);
    debounced_send();
}
```

**批量发送函数：**
```rust
#[cfg(feature = "hydrate")]
async fn send_events_batch(events: &[MouseEvent]) -> Result<(), JsValue> {
    let events_json = serde_json::to_string(events).unwrap();
    send_to_server(&events_json).await
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
```

**组件中使用：**
```rust
#[component]
fn MyComponent() -> impl IntoView {
    #[cfg(feature = "hydrate")]
    let (event_buffer, debounced_send) = create_debounced_sender();

    #[cfg(feature = "hydrate")]
    let buffer1 = event_buffer.clone();
    #[cfg(feature = "hydrate")]
    let sender1 = debounced_send.clone();

    view! {
        <div
            on:mousemove=move |e| {
                #[cfg(feature = "hydrate")]
                queue_event("mousemove", e.client_x(), e.client_y(), None, &buffer1, &sender1);
            }
        >
            // 内容
        </div>
    }
}
```

### 3. 后端实现 (`src/mouse_handler.rs`)

#### 依赖项
```toml
[dependencies]
axum = "0.7"
tokio = { version = "1", features = ["full"] }
serde = { version = "1", features = ["derive"] }
serde_json = "1"
```

**日志记录器：**
```rust
use std::fs::OpenOptions;
use std::io::Write;
use std::path::PathBuf;
use std::sync::Arc;
use tokio::sync::Mutex;

pub struct MouseLogger {
    file_path: PathBuf,
    _guard: Arc<Mutex<()>>,
}

impl MouseLogger {
    pub fn new(file_path: PathBuf) -> Self {
        Self {
            file_path,
            _guard: Arc::new(Mutex::new(())),
        }
    }

    pub async fn log_events(&self, events: &[MouseEvent]) -> Result<(), Box<dyn std::error::Error>> {
        if events.is_empty() {
            return Ok(());
        }

        let _guard = self._guard.lock().await;

        let mut file = OpenOptions::new()
            .create(true)
            .append(true)
            .open(&self.file_path)?;

        for event in events {
            let json_line = serde_json::to_string(event)?;
            writeln!(file, "{}", json_line)?;
        }
        file.flush()?;

        Ok(())
    }
}
```

**API 处理器：**
```rust
pub type AppState = Arc<MouseLogger>;

pub async fn handle_mouse_event(
    axum::extract::State(mouse_logger): axum::extract::State<AppState>,
    axum::Json(events): axum::Json<Vec<MouseEvent>>,
) -> Result<String, axum::http::StatusCode> {
    mouse_logger
        .log_events(&events)
        .await
        .map_err(|_| axum::http::StatusCode::INTERNAL_SERVER_ERROR)?;

    Ok(format!("{} events logged", events.len()))
}
```

**路由配置（main.rs）：**
```rust
let mouse_logger = Arc::new(MouseLogger::new("mouse_events.jsonl".into()));

let app = Router::new()
    .route("/api/mouse", axum::routing::post(handle_mouse_event))
    .with_state(mouse_logger);
```

## 🚀 迁移到新项目的步骤

### 方案 1：直接复制代码文件

1. **复制类型定义**
   - 复制 `src/types.rs` 的 `MouseEvent` 结构体

2. **复制前端代码**
   - 从 `src/app.rs` 复制：
     - `create_debounced_sender()`
     - `queue_event()`
     - `send_events_batch()`
     - `send_to_server()`

3. **复制后端代码**
   - 从 `src/mouse_handler.rs` 复制：
     - `MouseLogger` 结构体
     - `handle_mouse_event` 处理器

4. **添加依赖**
   - 将上述依赖项添加到新项目的 `Cargo.toml`

### 方案 2：创建独立的 crate（推荐）

```bash
# 创建独立的库项目
cargo new --lib mouse-tracker-core
cd mouse-tracker-core
```

**库结构：**
```
mouse-tracker-core/
├── Cargo.toml
├── src/
│   ├── lib.rs          # 导出公共 API
│   ├── types.rs        # 共享类型
│   ├── client.rs       # 前端功能
│   └── server.rs       # 后端功能
└── README.md
```

**在新项目中使用：**
```toml
# 新项目的 Cargo.toml
[dependencies]
mouse-tracker-core = { path = "../mouse-tracker-core" }
```

```rust
// 新项目中使用
use mouse_tracker_core::{MouseEvent, create_tracker};

#[component]
fn App() -> impl IntoView {
    let tracker = create_tracker();

    view! {
        <div on:mousemove=move |e| {
            tracker.track("mousemove", e.client_x(), e.client_y());
        }>
            // ...
        </div>
    }
}
```

### 方案 3：使用 GitHub 作为模块源

```toml
# 发布到 crates.io 或使用 git 依赖
[dependencies]
mouse-tracker = { git = "https://github.com/yourusername/mouse-tracker", package = "mouse-tracker-core" }
```

## ⚙️ 配置选项

### 可调整的参数

```rust
// 防抖延迟（毫秒）
const DEBOUNCE_MS: i32 = 500;

// 批量发送的最大事件数
const MAX_BATCH_SIZE: usize = 1000;

// 日志文件路径
const LOG_FILE: &str = "mouse_events.jsonl";

// API 端点
const API_ENDPOINT: &str = "/api/mouse";
```

### 扩展功能建议

1. **添加事件过滤**
   ```rust
   fn should_track(event: &MouseEvent) -> bool {
       // 只记录特定区域的事件
       event.x > 0 && event.x < 1920 && event.y > 0 && event.y < 1080
   }
   ```

2. **添加数据采样**
   ```rust
   // 对于 mousemove，只记录每 N 个事件
   static MOUSEMOVE_SAMPLE_RATE: u32 = 5;
   ```

3. **添加会话 ID**
   ```rust
   pub struct MouseEvent {
       pub session_id: String,  // 新增
       pub event_type: String,
       // ...
   }
   ```

4. **添加错误重试**
   ```rust
   async fn send_with_retry(events: &[MouseEvent], max_retries: u32) -> Result<(), JsValue> {
       for i in 0..max_retries {
           match send_events_batch(events).await {
               Ok(()) => return Ok(()),
               Err(e) if i < max_retries - 1 => {
                   web_sys::console::log_1(&format!("Retry {}/{}", i + 1, max_retries).into());
                   continue;
               }
               Err(e) => return Err(e),
           }
       }
       unreachable!()
   }
   ```

## 🧪 测试建议

### 单元测试
```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_mouse_event_serialization() {
        let event = MouseEvent {
            event_type: "mousemove".to_string(),
            x: 100,
            y: 200,
            timestamp: 1234567890,
            target: None,
            button: None,
            scroll_y: None,
        };

        let json = serde_json::to_string(&event).unwrap();
        let deserialized: MouseEvent = serde_json::from_str(&json).unwrap();

        assert_eq!(event.event_type, deserialized.event_type);
    }
}
```

### 集成测试
```rust
#[tokio::test]
async fn test_batch_logging() {
    let logger = MouseLogger::new("/tmp/test_events.jsonl".into());
    let events = vec![
        MouseEvent { /* ... */ },
        MouseEvent { /* ... */ },
    ];

    logger.log_events(&events).await.unwrap();

    // 验证文件写入
    let content = std::fs::read_to_string("/tmp/test_events.jsonl").unwrap();
    assert_eq!(content.lines().count(), 2);
}
```

## 📊 数据分析示例

### 读取 JSONL 文件
```python
import json

events = []
with open('mouse_events.jsonl', 'r') as f:
    for line in f:
        events.append(json.loads(line))

print(f"Total events: {len(events)}")
print(f"Event types: {set(e['event_type'] for e in events)}")
```

### 统计分析
```python
import pandas as pd

df = pd.DataFrame(events)

# 事件类型分布
print(df['event_type'].value_counts())

# 时间跨度
df['timestamp'] = pd.to_datetime(df['timestamp'], unit='ms')
print(f"Time span: {df['timestamp'].max() - df['timestamp'].min()}")

# 鼠标移动速度
mousemove_df = df[df['event_type'] == 'mousemove'].sort_values('timestamp')
mousemove_df['speed'] = mousemove_df.diff().pow(2).sum(axis=1)**0.5
print(f"Average speed: {mousemove_df['speed'].mean()}")
```

## 🔒 安全和隐私考虑

1. **数据脱敏**
   - 不要记录敏感信息（密码输入等）
   - 考虑添加隐私区域过滤

2. **用户同意**
   - 添加明确的追踪提示
   - 提供禁用追踪的选项

3. **数据存储**
   - 考虑数据加密
   - 定期清理旧数据

4. **传输安全**
   - 使用 HTTPS
   - 添加认证机制

## 📚 相关资源

- [Leptos 官方文档](https://leptos.dev/)
- [Axum 文档](https://docs.rs/axum/)
- [serde 文档](https://serde.rs/)
- [Web-sys API](https://docs.rs/web-sys/)

## 📝 版本历史

- v0.1.0 (2025-02-07): 初始实现
  - 基础鼠标事件捕获
  - 防抖优化
  - 批量发送

## 🤝 贡献

欢迎提交问题和改进建议！

---

**最后更新:** 2025-02-07
**维护者:** Your Name
**许可证:** MIT OR Apache-2.0
