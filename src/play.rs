use serenity::model::prelude::*;
use serenity::prelude::Context;
use url::Url;


pub async fn play(ctx: &Context, msg: &Message) {
    let url = msg.content.replace("play", "").trim().to_string();

    let url = match Url::parse(&url) {
        Ok(url) => url,
        Err(why) => {
            log::error!("Error parsing url: {:?}", why);
            
            match msg.reply(ctx, "Nigga please write correct url").await {
                Ok(_) => {},
                Err(why) => log::error!("Error sending message: {:?}", why),
            }
            
            return;
        }
    };

    let guild = msg.guild(&ctx.cache).unwrap();
    let guild_id = guild.id;

    let channel_id = guild
        .voice_states.get(&msg.author.id)
        .and_then(|voice_state| voice_state.channel_id);

    let connect_to = match channel_id {
        Some(channel) => channel,
        None => {
            match msg.reply(ctx, "Not in a voice channel").await {
                Ok(_) => {},
                Err(why) => log::error!("Error sending message: {:?}", why),
            }

            return;
        }
    };

    let manager = songbird::get(ctx).await
        .expect("Songbird Voice client placed in at initialisation.").clone();

    let _handler = manager.join(guild_id, connect_to).await;

    if let Some(handler_lock) = manager.get(guild_id) {
        let mut handler = handler_lock.lock().await;

        let source = songbird::ytdl(url)
            .await
            .expect("Songbird ytdl failed");

        handler.play_source(source);
    }
}