use args::Args;
use clap::Parser;
use serenity::Client;
use serenity::framework::StandardFramework;
use serenity::model::prelude::Message;
use serenity::model::prelude::Ready;
use serenity::model::prelude::ResumedEvent;
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
}

#[tokio::main]
async fn main() {
    env_logger::init();

    let args = Args::parse();

    let framework = StandardFramework::new()
        .configure(|c| c.with_whitespace(true).prefix("cringe"));

    let intents = GatewayIntents::non_privileged() | GatewayIntents::MESSAGE_CONTENT;

    let mut client = Client::builder(args.token, intents)
        .framework(framework)
        .register_songbird()
        .event_handler(Handler)
        .await.unwrap();

    client.start().await.unwrap();
}
