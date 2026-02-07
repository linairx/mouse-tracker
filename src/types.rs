use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MouseEvent {
    // 基础事件信息
    pub event_type: String,  // "mousemove", "mousedown", "mouseup", "wheel", "dragstart", "drag", "dragend", "keydown", "keyup", "click"
    pub timestamp: u64,

    // 位置信息
    pub x: i32,
    pub y: i32,
    pub screen_x: Option<i32>,    // 屏幕坐标
    pub screen_y: Option<i32>,
    pub page_x: Option<i32>,      // 页面坐标
    pub page_y: Option<i32>,

    // 鼠标按钮状态
    pub button: Option<String>,    // "left", "middle", "right"
    pub buttons: Option<u16>,      // 位掩码：1=左键, 2=右键, 4=中键

    // 滚轮信息
    pub scroll_y: Option<f64>,
    pub scroll_x: Option<f64>,

    // 目标元素信息
    pub target: Option<String>,
    pub target_tag: Option<String>,     // 元素标签名
    pub target_id: Option<String>,      // 元素 ID
    pub target_class: Option<String>,   // 元素类名
    pub target_text: Option<String>,    // 元素文本内容（截断）

    // 事件关联
    pub session_id: String,             // 会话 ID，用于关联同一会话的所有事件
    pub event_id: String,               // 事件唯一 ID
    pub parent_event_id: Option<String>, // 父事件 ID（用于拖拽等序列事件）

    // 速度和方向（用于 mousemove 和 drag）
    pub velocity_x: Option<f64>,        // X 轴速度（像素/毫秒）
    pub velocity_y: Option<f64>,        // Y 轴速度
    pub distance: Option<f64>,          // 距离上一个事件点的距离

    // 键盘事件
    pub key: Option<String>,            // 按键名称
    pub code: Option<String>,           // 按键代码
    pub ctrl_key: Option<bool>,         // 修饰键状态
    pub shift_key: Option<bool>,
    pub alt_key: Option<bool>,
    pub meta_key: Option<bool>,

    // 视口信息
    pub viewport_width: Option<u32>,
    pub viewport_height: Option<u32>,

    // 额外元数据
    pub metadata: Option<String>,       // JSON 字符串，存储额外信息
}
