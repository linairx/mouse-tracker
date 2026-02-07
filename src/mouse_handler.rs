use crate::types::MouseEvent;
use std::fs::OpenOptions;
use std::io::Write;
use std::path::PathBuf;
use std::sync::Arc;
use tokio::sync::Mutex;

pub struct MouseLogger {
    file_path: PathBuf,
    // 使用 Mutex 保护并发写入
    _guard: Arc<Mutex<()>>,
}

impl MouseLogger {
    pub fn new(file_path: PathBuf) -> Self {
        Self {
            file_path,
            _guard: Arc::new(Mutex::new(())),
        }
    }

    pub async fn log_event(&self, event: &MouseEvent) -> Result<(), Box<dyn std::error::Error>> {
        self.log_events(&[event.clone()]).await
    }

    pub async fn log_events(&self, events: &[MouseEvent]) -> Result<(), Box<dyn std::error::Error>> {
        if events.is_empty() {
            return Ok(());
        }

        // 获取锁以确保同一时间只有一个写入操作
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
