# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## 项目概述

这是一个基于 Leptos 框架的鼠标事件追踪应用，使用 Rust 编写前后端代码。前端捕获鼠标事件并实时发送到后端 API，后端将事件保存到 JSONL 文件中。

## 开发命令

### 开发模式
```bash
cargo leptos watch
```
启动开发服务器，支持热重载。服务器运行在 http://127.0.0.1:3000

### 发布构建
```bash
cargo leptos build --release
```
生成服务器二进制文件（`target/server/release`）和静态站点（`target/site`）

### 端到端测试
```bash
cargo leptos end-to-end
```
使用 Playwright 运行 E2E 测试，测试位于 `end2end/tests` 目录

### 运行单个测试
```bash
cargo test --test <test_name>
```

### 手动编译 WASM
```bash
cargo build --lib --target wasm32-unknown-unknown
```

## 架构

### 编译目标特性
- **ssr**: 服务端渲染特性，用于编译服务器二进制
- **hydrate**: 客户端水合特性，用于编译 WASM

### 目录结构
```
src/
├── lib.rs          # 库入口，包含 WASM 水合函数
├── main.rs         # 服务器入口（仅在 ssr 特性下编译）
├── app.rs          # Leptos 应用组件和前端逻辑
├── types.rs        # 共享数据类型定义
└── mouse_handler.rs # 后端 API 处理器
```

### 前后端交互流程
1. **客户端**（`app.rs`）：在浏览器中捕获鼠标事件（mousemove、mousedown、mouseup、wheel）
2. **网络传输**：通过 `fetch` API 将 JSON 格式事件 POST 到 `/api/mouse`
3. **服务端**（`mouse_handler.rs`）：Axum 处理器接收事件，调用 `MouseLogger` 追加到 `mouse_events.jsonl`

### 共享类型
`MouseEvent` 结构体在 `types.rs` 中定义，使用 `serde` 序列化/反序列化，确保前后端数据格式一致。

### 样式
- 样式文件：`style/main.scss`
- 编译后的 CSS 输出到 `target/site/pkg/app.css`

### 静态资源
- 静态文件目录：`public/`
- 站点根目录：`target/site`

## 依赖说明

### 核心依赖
- **leptos**: Web 框架
- **leptos_router**: 路由
- **leptos_meta**: 元标签管理
- **axum**: HTTP 服务器（仅 ssr）
- **tokio**: 异步运行时（仅 ssr）
- **wasm-bindgen**: WASM 绑定（仅 hydrate）
- **web-sys**: Web API 绑定
- **serde/serde_json**: 序列化

### 自定义编译配置
`[profile.wasm-release]` 定义了 WASM 优化配置（`opt-level = 'z'`, `lto = true`）

## 开发注意事项

### 添加新的 API 端点
1. 在 `mouse_handler.rs` 中添加处理函数
2. 在 `main.rs` 的 Router 中添加路由：
   ```rust
   .route("/api/your-endpoint", axum::routing::post(handler_fn))
   ```

### 添加新的组件
在 `app.rs` 中定义组件，然后在路由中注册：
```rust
<Route path=StaticSegment("/your-path") view=YourComponent/>
```

### 特性标记
- 服务器专用代码使用 `#[cfg(feature = "ssr")]`
- 客户端专用代码使用 `#[cfg(feature = "hydrate")]`
