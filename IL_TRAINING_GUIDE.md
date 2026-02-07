# IL 训练数据收集 - 完整功能说明

> 本文档详细说明了为模仿学习（Imitation Learning）训练优化的鼠标事件追踪功能。

## 🎯 设计目标

收集完整的用户交互序列数据，用于训练 IL 模型学习用户的鼠标操作行为模式。

## 📊 新增功能

### 1. 扩展的事件类型

**原有事件：**
- `mousemove` - 鼠标移动
- `mousedown` - 鼠标按下
- `mouseup` - 鼠标释放
- `wheel` - 滚轮滚动

**新增事件：**
- `dragstart` - 开始拖拽 ✨
- `drag` - 拖拽中 ✨
- `dragend` - 结束拖拽 ✨
- `keydown` - 按键按下 ✨
- `keyup` - 按键释放 ✨

### 2. 丰富的数据字段

#### 基础信息
```rust
pub event_type: String,    // 事件类型
pub timestamp: u64,        // 时间戳（毫秒）
```

#### 位置信息
```rust
pub x: i32,                 // 客户端 X 坐标
pub y: i32,                 // 客户端 Y 坐标
pub screen_x: Option<i32>,  // 屏幕 X 坐标
pub screen_y: Option<i32>,  // 屏幕 Y 坐标
pub page_x: Option<i32>,    // 页面 X 坐标
pub page_y: Option<i32>,    // 页面 Y 坐标
```

#### 鼠标状态
```rust
pub button: Option<String>,    // 按下的按钮："left", "middle", "right"
pub buttons: Option<u16>,      // 按钮状态位掩码（1=左键, 2=右键, 4=中键）
```

#### 滚轮信息
```rust
pub scroll_y: Option<f64>,     // 垂直滚动量
pub scroll_x: Option<f64>,     // 水平滚动量
```

#### 目标元素信息
```rust
pub target_tag: Option<String>,     // 元素标签名（如 "div", "button"）
pub target_id: Option<String>,      // 元素 ID 属性
pub target_class: Option<String>,   // 元素类名
pub target_text: Option<String>,    // 元素文本内容（限制 50 字符）
```

#### 事件关联
```rust
pub session_id: String,             // 会话 ID，关联同一会话的所有事件
pub event_id: String,               // 事件唯一 ID
pub parent_event_id: Option<String>, // 父事件 ID（用于关联拖拽序列）
```

#### 运动分析
```rust
pub velocity_x: Option<f64>,  // X 轴速度（像素/毫秒）
pub velocity_y: Option<f64>,  // Y 轴速度（像素/毫秒）
pub distance: Option<f64>,    // 距离上一个事件的距离（像素）
```

#### 键盘事件
```rust
pub key: Option<String>,      // 按键名称（如 "Enter", "a"）
pub code: Option<String>,     // 按键代码（如 "KeyA", "Enter"）
pub ctrl_key: Option<bool>,   // Control 键状态
pub shift_key: Option<bool>,  // Shift 键状态
pub alt_key: Option<bool>,    // Alt 键状态
pub meta_key: Option<bool>,   // Meta/Win 键状态
```

#### 环境信息
```rust
pub viewport_width: Option<u32>,   // 视口宽度
pub viewport_height: Option<u32>,  // 视口高度
```

## 🏗️ 核心架构

### 会话管理

```rust
pub struct TrackingState {
    pub session_id: String,                              // 会话 ID
    pub event_counter: Rc<RefCell<u64>>,                  // 事件计数器
    pub last_event: Rc<RefCell<Option<(MouseEvent, f64)>>>, // 上一个事件
    pub drag_state: Rc<RefCell<Option<String>>>,         // 当前拖拽 ID
}
```

**功能：**
- 生成唯一的会话 ID（格式：`session_{timestamp}`）
- 为每个事件生成唯一 ID（格式：`event_{session_id}_{count}`）
- 追踪上一个事件，用于计算速度和距离
- 管理拖拽状态，关联拖拽序列事件

### 事件处理流程

```
用户交互
    ↓
捕获事件（mousemove, mousedown, etc.）
    ↓
提取上下文信息
    ├─ 目标元素（标签、ID、类名、文本）
    ├─ 按钮状态（哪个键被按下）
    ├─ 位置信息（x, y, screen, page）
    └─ 时间戳
    ↓
创建 MouseEvent 对象
    ├─ 生成事件 ID
    ├─ 计算速度和距离
    ├─ 关联父事件（拖拽）
    └─ 添加会话信息
    ↓
添加到缓冲区
    ↓
防抖触发（500ms 静止后）
    ↓
批量发送到服务器
    ↓
保存到 JSONL 文件
```

## 📝 数据示例

### 鼠标点击事件
```json
{
  "event_type": "mousedown",
  "timestamp": 1737985200123,
  "x": 450,
  "y": 230,
  "screen_x": 450,
  "screen_y": 230,
  "page_x": 450,
  "page_y": 230,
  "button": "left",
  "buttons": 1,
  "scroll_y": null,
  "scroll_x": null,
  "target": null,
  "target_tag": Some("div"),
  "target_id": Some("submit-button"),
  "target_class": Some("btn btn-primary"),
  "target_text": Some("Submit"),
  "session_id": "session_1737985200000",
  "event_id": "event_session_1737985200000_42",
  "parent_event_id": null,
  "velocity_x": 0.5,
  "velocity_y": -0.3,
  "distance": 15.2,
  "key": null,
  "code": null,
  "ctrl_key": null,
  "shift_key": null,
  "alt_key": null,
  "meta_key": null,
  "viewport_width": 1920,
  "viewport_height": 1080,
  "metadata": null
}
```

### 拖拽事件序列
```json
// 1. 拖拽开始
{
  "event_type": "dragstart",
  "event_id": "event_session_xxx_10",
  "parent_event_id": null,
  "x": 100,
  "y": 200,
  // ...
}

// 2. 拖拽中
{
  "event_type": "drag",
  "event_id": "event_session_xxx_11",
  "parent_event_id": "event_session_xxx_10",  // 关联到 dragstart
  "x": 120,
  "y": 210,
  "velocity_x": 2.5,
  "velocity_y": 1.8,
  // ...
}

// 3. 拖拽结束
{
  "event_type": "dragend",
  "event_id": "event_session_xxx_12",
  "parent_event_id": null,  // 拖拽结束，解除关联
  "x": 150,
  "y": 230,
  // ...
}
```

### 键盘事件
```json
{
  "event_type": "keydown",
  "x": 0,
  "y": 0,
  "key": "Enter",
  "code": "Enter",
  "ctrl_key": false,
  "shift_key": false,
  "alt_key": false,
  "meta_key": false,
  "timestamp": 1737985200456,
  "session_id": "session_1737985200000",
  "event_id": "event_session_1737985200000_50",
  // ...
}
```

## 🔍 数据分析用途

### 1. 行为模式识别

**点击模式：**
```python
# 分析用户倾向于点击哪些类型的元素
target_tags = df[df['event_type'] == 'mousedown']['target_tag'].value_counts()
# 结果：button: 45%, a: 30%, div: 15%, other: 10%
```

**移动速度：**
```python
# 分析鼠标移动速度分布
mousemove = df[df['event_type'] == 'mousemove']
mousemove['speed'] = np.sqrt(mousemove['velocity_x']**2 + mousemove['velocity_y']**2)
print(f"平均速度: {mousemove['speed'].mean():.2f} px/ms")
print(f"最大速度: {mousemove['speed'].max():.2f} px/ms")
```

### 2. 序列预测

**拖拽检测：**
```python
# 通过事件序列识别拖拽操作
drag_sequences = []
current_drag = None

for _, row in df.iterrows():
    if row['event_type'] == 'dragstart':
        current_drag = [row]
    elif row['event_type'] == 'drag' and current_drag:
        current_drag.append(row)
    elif row['event_type'] == 'dragend' and current_drag:
        current_drag.append(row)
        drag_sequences.append(current_drag)
        current_drag = None
```

**点击-拖拽-释放序列：**
```python
# 识别完整的交互序列
# mousedown (button=left) → mousemove (buttons=1) → mouseup
```

### 3. 用户画像

**操作习惯：**
- 点击速度（从 mousedown 到 mouseup 的时间间隔）
- 移动轨迹（是否平滑、有无抖动）
- 滚轮使用频率
- 键盘快捷键使用
- 拖拽操作频率

## 📈 IL 训练数据准备

### 特征工程建议

**时序特征：**
```python
# 滑动窗口统计
window = 10  # 10 个事件
features['velocity_mean'] = df['velocity_x'].rolling(window).mean()
features['velocity_std'] = df['velocity_x'].rolling(window).std()
features['acceleration'] = df['velocity_x'].diff()
```

**序列特征：**
```python
# 事件类型编码
event_types = ['mousemove', 'mousedown', 'mouseup', 'wheel',
               'dragstart', 'drag', 'dragend', 'keydown', 'keyup']
one_hot = pd.get_dummies(df['event_type'])
```

**上下文特征：**
```python
# 目标元素特征
features['is_button'] = df['target_tag'] == 'button'
features['is_link'] = df['target_tag'] == 'a'
features['has_text'] = df['target_text'].notna()
```

### 训练样本构建

**状态-动作对：**
```python
# 状态：当前时刻的上下文
state = {
    'position': (x, y),
    'velocity': (vx, vy),
    'buttons': buttons_state,
    'target_element': element_features,
    'recent_events': event_history,  # 过去 N 个事件
}

# 动作：下一个时刻的预测
action = {
    'next_position': (next_x, next_y),
    'next_event_type': next_event,
    'time_delta': dt,
}
```

## 🚀 使用示例

### 启动数据收集

1. **运行服务器：**
   ```bash
   cargo leptos watch
   ```

2. **访问页面：**
   ```
   http://127.0.0.1:3000
   ```

3. **执行操作：**
   - 在灰色区域内移动鼠标
   - 点击绿色方块并拖拽
   - 按下键盘按键（先点击区域获得焦点）
   - 滚动滚轮

4. **查看数据：**
   ```bash
   # 查看最新收集的事件
   tail -20 mouse_events.jsonl | python -m json.tool
   ```

### 数据统计

```python
import json

events = []
with open('mouse_events.jsonl', 'r') as f:
    for line in f:
        events.append(json.loads(line))

print(f"总事件数: {len(events)}")
print(f"会话数: {len(set(e['session_id'] for e in events))}")
print(f"事件类型分布:")
from collections import Counter
for event_type, count in Counter(e['event_type'] for e in events).items():
    print(f"  {event_type}: {count}")
```

## 🎯 IL 模型训练建议

### 数据量需求

| 模型复杂度 | 事件数量 | 会话数量 | 持续时间 |
|----------|---------|---------|----------|
| 简单轨迹预测 | 5万-10万 | 10-50 | 数小时 |
| 中等行为模仿 | 10万-50万 | 50-200 | 数天到数周 |
| 复杂行为学习 | 50万-200万 | 200-1000 | 数周到数月 |

### 数据质量要点

1. **多样性**：覆盖不同的操作类型和场景
2. **连续性**：保持会话的完整性，避免碎片化
3. **真实性**：在真实使用场景中收集，而非刻意操作
4. **标注**：考虑添加行为标签（如"快速移动"、"精准点击"等）

### 模型输入

```python
# 输入特征
input_features = [
    # 历史轨迹（过去 10 个事件）
    'past_positions': (10, 2),      # x, y
    'past_velocities': (10, 2),     # vx, vy
    'past_event_types': (10, 9),    # one-hot 编码

    # 当前上下文
    'current_position': (2,),       # x, y
    'current_velocity': (2,),       # vx, vy
    'button_state': (3,),           # left, middle, right
    'target_features': (50,),       # 元素特征 embedding

    # 环境信息
    'viewport_size': (2,),          # width, height
    'time_since_last': (1,),        # 毫秒
]

# 输出预测
output = [
    'next_position': (2,),          # 预测的 x, y
    'next_event_type': (9,),        # 事件类型概率
    'time_to_next': (1,),           # 预测时间间隔
]
```

## 🔧 配置和优化

### 防抖延迟调整

```rust
// 在 app.rs 中修改
const DEBOUNCE_MS: i32 = 500;  // 默认 500ms
```

**建议值：**
- 快速反馈：100-200ms（会增加网络请求）
- 平衡模式：500ms（推荐）
- 减少请求：1000ms（可能丢失细节）

### 事件采样率

```rust
// 为 mousemove 添加采样
if event_type == "mousemove" {
    static COUNTER: AtomicU32 = AtomicU32::new(0);
    if COUNTER.fetch_add(1, Ordering::SeqCst) % 5 != 0 {
        return;  // 只记录 1/5 的 mousemove 事件
    }
}
```

### 数据增强

```python
# 在训练前进行数据增强
augmented = []

# 1. 位置抖动
for event in events:
    for _ in range(5):
        new_event = event.copy()
        new_event['x'] += np.random.normal(0, 2)
        new_event['y'] += np.random.normal(0, 2)
        augmented.append(new_event)

# 2. 时间扰动
for event in events:
    new_event = event.copy()
    new_event['timestamp'] += np.random.normal(0, 10)
    augmented.append(new_event)

# 3. 速度缩放
# ...
```

## 📊 性能指标

### 数据收集效率

- **内存占用**：每个事件约 500-800 字节（JSON 格式）
- **网络传输**：批量发送，平均每个事件 100-200 字节（gzip 压缩后）
- **磁盘占用**：100 万事件约 100-200 MB

### 收集速率

- **轻度使用**：~10-50 事件/秒
- **中度使用**：~50-200 事件/秒
- **重度使用**：~200-500 事件/秒

## 🛡️ 隐私和安全

### 敏感信息过滤

```rust
// 不记录的元素
if target_tag == Some("input".to_string()) {
    let input_type = element.get_attribute("type");
    if input_type == Some("password".to_string()) {
        return;  // 跳过密码输入框
    }
}
```

### 数据脱敏

```rust
// 移除可能包含的敏感信息
target_text = target_text.map(|txt| {
    if txt.len() > 50 {
        format!("{}...", &txt[..50])  // 截断长文本
    } else {
        txt
    }
});
```

### 用户同意

建议在应用启动时添加明确的提示：

```rust
view! {
    <div class="consent-banner">
        <p>"此应用会收集您的鼠标和键盘交互数据用于研究目的。"
        <button on:click=accept_consent>"同意"</button>
        <button on:click=decline_consent>"拒绝"</button>
    </div>
}
```

## 📚 相关资源

- [Leptos 文档](https://leptos.dev/)
- [Web-sys API](https://docs.rs/web-sys/)
- [模仿学习综述](https://arxiv.org/abs/1911.07158)
- [行为克隆教程](https://www.youtube.com/watch?v=example)

## 📝 更新日志

**v0.2.0** (2025-02-07)
- ✨ 添加拖拽事件支持（dragstart, drag, dragend）
- ✨ 添加键盘事件支持（keydown, keyup）
- ✨ 扩展数据字段（30+ 个字段）
- ✨ 添加会话管理和事件关联
- ✨ 添加速度和距离计算
- ✨ 添加目标元素信息提取
- ✨ 添加视口信息记录
- 📝 创建完整的 IL 训练文档

---

**最后更新:** 2025-02-07
**维护者:** Your Name
**许可证:** MIT OR Apache-2.0
