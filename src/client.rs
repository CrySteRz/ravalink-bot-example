use serenity::prelude::*;
use serenity::Client as SerenityClient;
use serenity::model::id::ApplicationId;
use std::sync::Arc;
use crate::handlers::Handler;
use crate::caches::guild::GuildCacheKey;
use std::{collections::HashMap, env, error::Error};
use serenity::all::GatewayIntents;
use ravalink_lib::{RavalinkConfig, SASLConfig};
use ravalink_lib::serenity::SerenityInit;


pub struct Client {
    pub rusty_client: SerenityClient,
}

impl Client {
    pub async fn default() -> Result<Client, Box<dyn Error>> {
        let token = env::var("DISCORD_TOKEN").expect("Fatality! DISCORD_TOKEN not set!");
        let app_id = ApplicationId::new(
            env::var("DISCORD_APP_ID")
                .expect("Fatality! DISCORD_APP_ID not set!")
                .parse()
                .expect("Expected a valid application ID")
        );
        Client::new(token, app_id).await
    }

    pub async fn new(token: String, app_id: ApplicationId) -> Result<Client, Box<dyn Error>> {
        
        let gateway_intents = GatewayIntents::non_privileged();
       
        let guild_cache = Arc::new(RwLock::new(HashMap::new()));
        let handler = Handler::new(guild_cache.clone());

        let client = SerenityClient::builder(token, gateway_intents)
            .event_handler(handler)
            .application_id(app_id)
            .register_ravalink(
                env::var("KAFKA_BROKER").expect("Fatality! KAFKA_BROKER not set!"),
                RavalinkConfig {
                    ssl: None,
                    sasl: Some(SASLConfig {
                        kafka_username: env::var("KAFKA_USERNAME").expect("Fatality! KAFKA_USERNAME not set!"),
                        kafka_password: env::var("KAFKA_PASSWORD").expect("Fatality! KAFKA_PASSWORD not set!"),
                    }),
                    kafka_topic: env::var("KAFKA_TOPIC").expect("Fatality! KAFKA_TOPIC not set!"),
                },
            )
            .await?;

        {
            let mut data = client.data.write().await;
            data.insert::<GuildCacheKey>(guild_cache);
        }

        Ok(Client { rusty_client: client })
    }

    pub async fn start(&mut self) -> Result<(), serenity::Error> {
        self.rusty_client.start().await
    }
}
