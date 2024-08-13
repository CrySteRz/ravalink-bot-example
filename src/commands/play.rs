
type Error = Box<dyn std::error::Error + Send + Sync>;
use serenity::all::{ButtonStyle, CreateActionRow, CreateButton, CreateEmbed};
use serenity::builder::{CreateCommand, CreateCommandOption, CreateInteractionResponse, CreateInteractionResponseMessage};
use serenity::model::application::CommandOptionType;
use::serenity::client::Context;
use crate::utils::*;

use charcoal_client::{
    get_handler_from_interaction_mutable, PlayerObject,
};
use charcoal_client::serenity::CharcoalKey;
use serenity::model::application::CommandInteraction;
use serenity::model::application::CommandDataOptionValue;
use charcoal_client::actions::player::Player;

pub async fn run(ctx: &Context, interaction: &CommandInteraction) -> Result<(), Error> {
    let url = interaction.data.options
        .iter()
        .find(|opt| opt.name == "url")
        .and_then(|opt| match &opt.value {
            CommandDataOptionValue::String(val) => Some(val),
            _ => None,
        });

    if let Some(url) = url {
        if !validate_url(url) {
            interaction.create_response(&ctx.http, CreateInteractionResponse::Message(CreateInteractionResponseMessage::new().content("Invalid URL"))).await?;
            return Ok(());
        }

        let mut _handler: Option<&mut PlayerObject> = None;

        get_handler_from_interaction_mutable!(ctx, interaction, _handler);

        match _handler {
            Some(_handler) => {
                _handler.play_from_youtube(url.to_string()).await.unwrap();
                let user = interaction.user.clone();
                let embed = CreateEmbed::default()
                    .description(format!("Playing: ({})", url)) 
                    .author(user.into());

                let pause_button = CreateButton::new("pause_button")
                    .label("Pause")
                    .style(ButtonStyle::Primary);

                let ping_button = CreateButton::new("ping_button")
                    .label("PING")
                    .style(ButtonStyle::Secondary);
                    
                let stop_button = CreateButton::new("stop_button")
                    .label("Stop")
                    .style(ButtonStyle::Danger);

                let action_row = CreateActionRow::Buttons(vec![pause_button, ping_button, stop_button]);

                interaction.create_response(&ctx.http, CreateInteractionResponse::Message(CreateInteractionResponseMessage::new().embed(embed).components(vec![action_row]))).await?;
            }
            None => {
                interaction.create_response(&ctx.http, CreateInteractionResponse::Message(CreateInteractionResponseMessage::new().content("Failed to get manager"))).await?;
                return Ok(());
            }
        }

        Ok(())

    } else {
        Err(Box::from("URL option not found"))
    }
}

pub fn register() -> CreateCommand {
    CreateCommand::new("play")
        .description("A play command")
        .add_option(
            CreateCommandOption::new(
                CommandOptionType::String,
                "url",
                "URL to play (Youtube | Soundcloud | Spotify)",
            ).required(true)
        )
}