mod config;
mod error;
mod structs;
mod handler;

use anyhow::Error;
use error::ConfigError;
use tracing_subscriber::util::TryInitError;
use core::time::Duration;
use std::env;
use std::process;
use std::process::Command;
use config::Config;
use std::fs::File;
use std::io::BufReader;
use paho_mqtt::Message;
use std::thread;
use handler::message_handler::message_handler;

const CONFIG_PATH: &str = "./config.json";
const DFLT_BROKER:&str = "tcp://10.1.10.37:1883";
const DFLT_CLIENT:&str = "912838120038283";
const DFLT_TOPICS:&[&str] = &["rust/mqtt"];
// Define the qos.
const QOS:i32 = 0;
const DFLT_QOS:&[i32] = &[0];
// Reconnect to the broker when connection is lost.
fn try_reconnect(cli: &paho_mqtt::Client) -> bool
{
    println!("Connection lost. Waiting to retry connection");
    for _ in 0..12 {
        thread::sleep(Duration::from_millis(500));
        if cli.reconnect().is_ok() {
            println!("Successfully reconnected");
            return true;
        }
    }
    println!("Unable to reconnect after several attempts.");
    false
}

fn subscribe_topics(cli: &paho_mqtt::Client) {
    if let Err(e) = cli.subscribe_many(DFLT_TOPICS, DFLT_QOS) {
        println!("Error subscribes topics: {:?}", e);
        process::exit(1);
    }
}

fn init_config() -> Result<Config, ConfigError> {
    let file = File::open(CONFIG_PATH)?;
    let reader = BufReader::new(file);
    let config: Config = serde_json::from_reader(reader)?;
    Ok(config)
}

fn main() -> Result<(), Error>{
    let config = init_config()?;


    let host = env::args().nth(1).unwrap_or_else(||
						 DFLT_BROKER.to_string()
    );

    // Define the set of options for the create.
    // Use an ID for a persistent session.
    let create_opts = paho_mqtt::CreateOptionsBuilder::new()
	.server_uri(host)
	.client_id(DFLT_CLIENT.to_string())
	.finalize();

    // Create a client.
    let mut cli = paho_mqtt::Client::new(create_opts).unwrap_or_else(|err| {
	println!("Error creating the client: {:?}", err);
	process::exit(1);
    });

    // Define the set of options for the connection.
    let conn_opts = paho_mqtt::ConnectOptionsBuilder::new()
        .user_name(&config.username)
        .password(&config.password)
	.keep_alive_interval(Duration::from_secs(20))
	.clean_session(true)
	.finalize();

    // Connect and wait for it to complete or fail.
    if let Err(e) = cli.connect(conn_opts) {
	println!("Unable to connect:\n\t{:?}", e);
	process::exit(1);
    }
    subscribe_topics(&cli);
    let rx = cli.start_consuming();
    for msg in rx.iter() {
        if let Some(msg) = msg {
	    println!("got message");
	    message_handler(msg);
        }
        else if !cli.is_connected() {
            if try_reconnect(&cli) {
                println!("Resubscribe topics...");
                subscribe_topics(&cli);
            } else {
                break;
            }
        }
    }
    Ok(())
}
