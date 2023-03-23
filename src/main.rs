use args::Args;
use clap::Parser;
use openai::Openai;
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
use youtube::Youtube;

use crate::openai::ChatMessage;
use crate::parser::parse_songs_from_string;

mod play;
mod stop;
mod args;
mod youtube;
mod openai;
mod parser;
mod skip;

struct Handler {
    youtube: Youtube,
    openai: Openai
}

#[async_trait::async_trait]
impl EventHandler for Handler {
    async fn ready(&self, ctx: Context, ready: Ready) {
        println!("Connected as {}", ready.user.name);
    }

    async fn message(&self, ctx: Context, msg: Message) {
        let msg_content = msg.content.clone();

        println!("{}: {}", msg.author.name, msg_content);

        if msg.mentions_user_id(ctx.cache.current_user_id()) {
            // Handle the message
            println!("Bot mentioned in message: {}", msg_content);

            let channel_id = msg.channel_id;

            let typing = channel_id.start_typing(&ctx.http).unwrap();

            let mut chat_messages: Vec<ChatMessage> = Vec::new();

            // let messages = msg.channel_id
            //     .messages(&ctx, |ret| ret.limit(4)).await.unwrap();

            // for msg in messages.iter() {
            //     let role = match msg.author.bot {
            //         true => "assistant",
            //         false => "user",
            //     };

            //     // Filter username <@1071367438098247700> feeling cute today
            //     let filttered_content = msg.content.clone().replace("<@1071367438098247700>", "");

            //     let chat_message = ChatMessage {
            //         role: role.to_string(),
            //         content: msg.content.clone().replace("", to),
            //     };

            //     chat_messages.push(chat_message);
            // }

            // chat_messages.reverse();

            chat_messages.push(ChatMessage {
                role: "user".to_string(),
                content: msg_content
            });

            let choice = self.openai.create_chat_completion(&chat_messages).await.unwrap();

            log::info!("choice: {}", choice);

            let (songs, cleaned_text) = parse_songs_from_string(&choice);

            msg.channel_id.say(&ctx, cleaned_text).await.unwrap();

            log::info!("songs: {:?}", songs);
            
            if songs.len() > 0 {
                play::play(&ctx, &self.youtube, &self.openai, &msg, &songs[0]).await;
            }

            typing.stop();
            
            return;
        }

        if msg.content.starts_with("play") {
            log::info!("play command");

            play::play(&ctx, &self.youtube, &self.openai, &msg, &msg.content).await;
        }

        if msg.content.starts_with("stop") {
            log::info!("stop command");

            stop::stop(&ctx, &msg).await;
        }

        if msg_content.starts_with("skip") {
            log::info!("skip command");

            skip::skip(&ctx, &msg).await;
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
            match args.discord_apikey {
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

    println!("discord token: {}", token);

    let openai_apikey = match std::env::var("OPENAI_APIKEY") {
        Ok(t) => {
            t
        }
        Err(_) => {
            match args.openai_apikey {
                Some(t) => {
                    t
                }
                None => {
                    "".to_string()
                }
            }
        }
    };

    let youtube_apikey = match std::env::var("YOUTUBE_APIKEY") {
        Ok(t) => {
            t
        }
        Err(_) => {
            match args.youtube_apikey {
                Some(t) => {
                    t
                }
                None => {
                    "".to_string()
                }
            }
        }
    };

    loop {
        log::info!("building client...");

        let framework = StandardFramework::new()
        .configure(|c| c.with_whitespace(true).prefix("cringe"));

        let intents = GatewayIntents::non_privileged() | GatewayIntents::MESSAGE_CONTENT;

        let youtube = Youtube::new(&youtube_apikey);
        let openai = Openai::new(&openai_apikey);

        let mut client = match Client::builder(&token, intents)
            .framework(framework)
            .register_songbird()
            .event_handler(Handler {
                youtube: youtube,
                openai: openai
            })
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
