use serenity::prelude::*;
use serenity::model::application::Command;
use crate::commands;

pub async fn register_global_commands(ctx: &Context) {
    
    let commands = vec![
        commands::ping::register(),
        commands::play::register(),
        commands::join::register(),
        commands::leave::register(),
        commands::pause::register(),
        commands::resume::register(),
        commands::skip::register(),

    ];

    for command in commands {
        let result = Command::create_global_command(&ctx.http, command).await;
        match result {
            Ok(cmd) => println!("Registered global command: {:?}", cmd.name),
            Err(why) => eprintln!("Failed to register global command: {:?}", why),
        }
    }
}