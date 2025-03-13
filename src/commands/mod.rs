use std::error::Error;
use serenity::all::{ButtonStyle, CommandInteraction, Context, CreateActionRow, CreateButton, CreateInteractionResponse, CreateInteractionResponseMessage};

pub mod play;
pub mod join;
pub mod stop;
pub mod skip;
pub mod pause;
pub mod resume;


pub async fn send_interaction_response(
    ctx: &Context,
    interaction: &CommandInteraction,
    message: impl Into<String>,
) -> Result<(), Box<dyn Error + Send + Sync>> {
    interaction
        .create_response(
            &ctx.http,
            CreateInteractionResponse::Message(
                CreateInteractionResponseMessage::new().content(message.into()),
            ),
        )
        .await?;
    Ok(())
}

#[macro_export]
macro_rules! try_or_respond {
    ($opt:expr, $ctx:expr, $interaction:expr, $msg:expr) => {
        match $opt {
            Some(val) => val,
            None => {
                send_interaction_response($ctx, $interaction, $msg).await?;
                return Ok(());
            }
        }
    };
}

fn create_control_buttons() -> Vec<CreateActionRow> {
    let buttons_top = vec![
        CreateButton::new("pause_button").label("⏸️Pause").style(ButtonStyle::Secondary),
        CreateButton::new("resume_button").label("▶️Resume").style(ButtonStyle::Secondary),
        CreateButton::new("back_button").label("⏮️Back").style(ButtonStyle::Secondary),
        CreateButton::new("skip_button").label("⏭️Skip").style(ButtonStyle::Secondary)
    ];

    let buttons_bottom = vec![
        CreateButton::new("down_button").label("🔉Down").style(ButtonStyle::Secondary),
        CreateButton::new("up_button").label("🔊Up").style(ButtonStyle::Secondary),
        CreateButton::new("loop_song_button").label("🔁Loop").style(ButtonStyle::Secondary),
        CreateButton::new("playlist_button").label("📜Playlist").style(ButtonStyle::Secondary),
        CreateButton::new("stop_button").label("⏹️Stop").style(ButtonStyle::Secondary)
    ];

    vec![
        CreateActionRow::Buttons(buttons_top),
        CreateActionRow::Buttons(buttons_bottom),
    ]
}