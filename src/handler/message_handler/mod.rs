mod switch_workspace;
mod send_notification;

use anyhow::Error;
use crate::structs::Command;
use paho_mqtt::Message;
use send_notification::*;
use serde_json::Value;
use switch_workspace::*;

pub fn message_handler(msg: Message) -> Result<(), Error> {
    println!("in message handler");
    let data: Command = serde_json::from_str(msg.payload_str().as_ref())?;
    println!("{}", data.command);
    // Extract command and arguments
    let command = &data.command;
    let arguments = &data.arguments;
    // Process the command
    match command.as_str() {
        "switch_workspace" => switch_workspace(arguments),
        "send_notification" => send_notification(arguments),
        // Handle other commands here...
        _ => println!("Unknown command"),
    }
    Ok(())
}
