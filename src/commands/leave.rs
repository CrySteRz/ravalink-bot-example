
use serenity::all::CreateCommand;
use serenity::all::Context;
use serenity::all::CommandInteraction;
use std::error::Error;

pub async fn run(ctx: &Context, interaction: &CommandInteraction) -> Result<(), Box<dyn Error + Send + Sync>> {

    

Ok(())
}

pub fn register() -> CreateCommand {
    CreateCommand::new("leave")
        .description("Leave command")
}