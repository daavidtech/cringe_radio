use serenity::model::prelude::Message;
use serenity::prelude::*;

pub async fn skip(
    ctx: &Context,
    msg: &Message,
) {
    let guild = msg.guild(&ctx.cache).unwrap();
    let guild_id = guild.id;

    let manager = songbird::get(ctx).await
        .expect("Songbird Voice client placed in at initialisation.").clone();

    if let Some(handler_lock) = manager.get(guild_id) {
        let handler = handler_lock.lock().await;

        let queue = handler.queue();

        if queue.len() > 0 {
            queue.skip().unwrap();
        }
    }
}