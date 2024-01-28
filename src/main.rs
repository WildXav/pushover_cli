use std::fs;
use anyhow::Context;
use clap::{Arg, Command};
use crate::cmds::send_notif;
use crate::structs::Config;

mod structs;
mod cmds;

#[tokio::main]
async fn main() -> Result<(), anyhow::Error> {
    let http_client = reqwest::Client::new();

    let config_str =
        fs::read_to_string("config.toml").context("Failed to read from config.toml")?;
    let config: Config = toml::from_str(config_str.as_str())
        .context("Invalid configuration declared in config.toml")?;

    let args = Command::new("cmd")
        .subcommand_required(true)
        .subcommand(Command::new("send")
            .arg(Arg::new("title").required(true))
            .arg(Arg::new("msg").required(true)))
        .get_matches();

    match args.subcommand() {
        Some(("send", cmd)) => {
            let title = cmd
                .get_one::<String>("title")
                .context("Title is required")?;
            let msg = cmd
                .get_one::<String>("msg")
                .context("Message is required")?;

            println!("Sending the following: [{}] {}", title, msg);
            send_notif(&http_client, &config, title.clone(), msg.clone())
                .await
                .context("Failed to send notification")?;
        }
        _ => {}
    }

    Ok(())
}
