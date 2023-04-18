use tauri::Window;

#[derive(Clone, serde::Serialize)]
struct Payload {
    message: String,
}

pub fn test_emit(window: &Window, event_listener_name: &str) {
    let event_message = format!("Called from backend");
    // window.center();
    window
        .emit(&event_listener_name, event_message.clone())
        .unwrap();
    println!(
        "Sent to event listener: {},  message: {}",
        event_listener_name, event_message
    );
    return {};
}
