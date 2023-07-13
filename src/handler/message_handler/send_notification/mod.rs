use std::process::Command;
use serde_json::Value;

pub fn send_notification(arguments: &Value) {
    let output = Command::new("notify-send")
        .arg(arguments["summary"].as_str().unwrap())
        .arg(arguments["body"].as_str().unwrap())
        .output()
        .expect("Failed to execute command");

    if output.status.success() {
        let stdout = String::from_utf8_lossy(&output.stdout);
        println!("Command executed successfully. Output:\n{}", stdout);
    } else {
        let stderr = String::from_utf8_lossy(&output.stderr);
        println!("Command failed. Error:\n{}", stderr);
    }
}
