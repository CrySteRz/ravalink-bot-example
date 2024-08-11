use serenity::all::CreateCommand;
use serenity::all::Context;
use serenity::all::CommandInteraction;
use std::error::Error;

pub async fn run(_ctx: &Context, _interaction: &CommandInteraction) -> Result<(), Box<dyn Error + Send + Sync>> {


    Ok(())
}


pub fn register() -> CreateCommand {
    CreateCommand::new("skip")
        .description("Skip current track")
}