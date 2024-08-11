use serenity::builder::{CreateInteractionResponse, CreateInteractionResponseMessage};
use serenity::model::application::Interaction;
use serenity::prelude::*;
use crate::commands;

pub async fn handle_interaction(ctx: Context, interaction: Interaction) {
    if let Interaction::Command(command) = interaction {
        println!("Received command interaction: {command:#?}");

        let content = match command.data.name.as_str() {
            "ping" => {
                if let Err(why) = commands::ping::run(&ctx, &command).await {
                    println!("Error handling join command: {why}");
                }
                None
            },
            "play" => {
                if let Err(why) = commands::play::run(&ctx, &command).await {
                    println!("Error handling play command: {why}");
                }
                None
            },
            "join" => {
                if let Err(why) = commands::join::run(&ctx, &command).await {
                    println!("Error handling join command: {why}");
                }
                None
            },
            "leave" => {
                if let Err(why) = commands::leave::run(&ctx, &command).await {
                    println!("Error handling leave command: {why}");
                }
                None
            },
            "skip" => {
                if let Err(why) = commands::skip::run(&ctx, &command).await {
                    println!("Error handling skip command: {why}");
                }
                None
            },
            "pause" => {
                if let Err(why) = commands::pause::run(&ctx, &command).await {
                    println!("Error handling pause command: {why}");
                }
                None
            },
            "resume" => {
                if let Err(why) = commands::resume::run(&ctx, &command).await {
                    println!("Error handling resume command: {why}");
                }
                None
            },
            _ => Some("not implemented :(".to_string()),
        };

        if let Some(content) = content {
            let data = CreateInteractionResponseMessage::new().content(content);
            let builder = CreateInteractionResponse::Message(data);
            if let Err(why) = command.create_response(&ctx.http, builder).await {
                println!("Cannot respond to slash command: {why}");
            }
        }
    }
}
