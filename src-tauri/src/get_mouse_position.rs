use chrono::Local;
use rdev::{listen, Event, EventType, Button};

// static LAST_POSITION: Arc<Mutex<(f32, f32)>> = Arc::new(Mutex::new((0.0, 0.0)));
// static CLICK_RECORDS: Arc<Mutex<Vec<ClickRecord>>> = Arc::new(Mutex::new(Vec::new()));

/// マウスクリックのイベントをリッスンして、位置と時間を取得します。
pub fn listen_mouse_click<F>(callback: F) -> Result<(), Box<dyn std::error::Error>>
where
    F: Fn((i32, i32), String) + Send + 'static,
{
    // let position_clone = LAST_POSITION.clone();
    // let records_clone = CLICK_RECORDS.clone();

    // let start_time = Instant::now();


    // マウスイベントを非同期でリッスン
    std::thread::spawn(move || {
        if let Err(error) = listen(move |event| {
            match event.event_type {
                EventType::MouseMove { x, y } => {
                    // マウスの移動位置を取得
                    let timestamp = Local::now().format("%Y-%m-%d %H:%M:%S").to_string();
                    println!("Mouse moved to ({}, {}) at {}", x, y, timestamp);
                    // 必要であればコールバックを呼び出す
                    // callback((x as i32, y as i32), timestamp);
                }
                EventType::ButtonPress(button) => {
                    if button == Button::Left {
                        // マウスの左ボタンが押されたときの処理
                        let timestamp = Local::now().format("%Y-%m-%d %H:%M:%S").to_string();
                        if let EventType::MouseMove { x, y } = event.event_type {
                            let timestamp = Local::now().format("%Y-%m-%d %H:%M:%S").to_string();
                            callback((x as i32, y as i32), timestamp);
                        } else {
                            eprintln!("Mouse position not available for button press.");
                        }
                    }
                }
                _ => {
                    // 他のイベントは無視
                }
            }
        }) {
            eprintln!("Error listening to mouse events: {:?}", error);
        }
    });
    Ok(())
}
