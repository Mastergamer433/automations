mod subcommands;

use crate::structs::Context;
use crate::structs::Command;
use crate::structs::CommandResult;
use subcommands::address::address;

#[poise::command(prefix_command, track_edits, slash_command, subcommands("address"))]
pub async fn email(
    ctx: Context<'_>,
    #[autocomplete = "poise::builtins::autocomplete_command"]
    command: Option<i32>,
) -> CommandResult {
    
    Ok(())
}

pub fn commands() -> [Command; 1] {
    [email()]
}
