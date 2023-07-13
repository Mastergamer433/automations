use serde_json::Value;
use std::process::Command;

pub fn switch_workspace(arguments: &Value) {
    println!("{}", arguments["index"]);
    let output = Command::new("herbstclient")
        .arg("use_index")
        .arg(arguments["index"].as_str().unwrap())
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
