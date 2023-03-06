use args::Args;
use clap::Parser;
use serenity::Client;
use serenity::framework::StandardFramework;
use serenity::model::prelude::Message;
use serenity::model::prelude::Ready;
use serenity::model::prelude::ResumedEvent;
use serenity::model::voice::VoiceState;
use serenity::prelude::Context;
use serenity::prelude::EventHandler;
use serenity::prelude::GatewayIntents;
use songbird::SerenityInit;

mod play;
mod stop;
mod args;

struct Handler;

#[async_trait::async_trait]
impl EventHandler for Handler {
    async fn ready(&self, ctx: Context, ready: Ready) {
        println!("Connected as {}", ready.user.name);
    }

    async fn message(&self, ctx: Context, msg: Message) {
        println!("{}: {}", msg.author.name, msg.content);

        if msg.content.starts_with("play") {
            println!("play command");
            play::play(&ctx, &msg).await;
        }

        if msg.content.starts_with("stop") {
            println!("stop command");
            stop::stop(&ctx, &msg).await;
        }
    }

    async fn resume(&self, _: Context, _: ResumedEvent) {
        println!("Resumed");
    }
    
    async fn voice_state_update(
        &self,
        ctx: Context,
        old_state: Option<VoiceState>,
        new_state: VoiceState,
    ) {
        let guild_id = match new_state.guild_id {
            Some(guild_id) => guild_id,
            None => return,
        };

        log::info!("guild_id: {:?}", guild_id);

        let manager = match songbird::get(&ctx).await {
            Some(manager) => manager,
            None => return,
        };

        log::info!("manager found");

        let call = match manager.get(guild_id) {
            Some(call) => call,
            None => return,
        };

        let mut call = call.lock().await;

        log::info!("call found ({:?})", call);

        let channel_id = match call.current_channel() {
            Some(channel_id) => channel_id,
            None => return,
        };

        log::info!("channel_id: {:?}", channel_id);

        let channel_id = serenity::model::id::ChannelId(channel_id.0);

        let channel = match ctx.cache.guild_channel(channel_id) {
            Some(channel) => channel,
            None => return,
        };

        let members = match channel.members(&ctx.cache).await {
            Ok(members) => members,
            Err(why) => {
                log::error!("Error getting members: {:?}", why);
                return;
            }
        };

        log::info!("members count: {}", members.len());

        if members.len() == 1 {
            log::info!("there is only one member in the channel leaving the call");

            match call.leave().await {
                Ok(_) => log::info!("left the call"),
                Err(why) => log::error!("Error leaving the call: {:?}", why),
            }
        }
    }
}

#[tokio::main]
async fn main() {
    env_logger::builder()
        .filter_level(log::LevelFilter::Info)
        .filter_module("tracing::span", log::LevelFilter::Error)
        .init();

    let args = Args::parse();

    let token = match std::env::var("CRINGE_RADIO_TOKEN") {
        Ok(t) => {
            t
        }
        Err(_) => {
            match args.token {
                Some(t) => {
                    t
                }
                None => {
                    log::error!("No token provided");
                    return;
                }
            }
        }
    };

    loop {
        log::info!("building client...");

        let framework = StandardFramework::new()
        .configure(|c| c.with_whitespace(true).prefix("cringe"));

        let intents = GatewayIntents::non_privileged() | GatewayIntents::MESSAGE_CONTENT;

        let mut client = match Client::builder(&token, intents)
            .framework(framework)
            .register_songbird()
            .event_handler(Handler)
            .await {
            Ok(client) => client,
            Err(why) => {
                log::error!("Error creating client: {:?}", why);

                log::info!("retrying in 5 seconds...");
            
                tokio::time::sleep(std::time::Duration::from_secs(5)).await;
                continue;
            }
        };

        match client.start().await {
            Ok(_) => {
                break;
            }
            Err(why) => {
                log::error!("client error: {:?}", why);

                log::info!("retrying in 5 seconds...");
                
                tokio::time::sleep(std::time::Duration::from_secs(5)).await;
            }
        }
    }
}
