use serenity::{
  async_trait,
  model::{channel::Message, gateway::Ready},
  prelude::*
};
pub struct Handler;

#[async_trait]
impl EventHandler for Handler {
  async fn message(&self, ctx: Context, msg: Message){
    if msg.content == "!ping"{
      if let Err(why) = msg.channel_id.say(&ctx.http, "Pong!").await {
          println!("Error sending message: {:?}", why);
        }
    }
  }
  
  // Set a handler to be called on the `ready` event. This is called when a
  // shard is booted, and a READY payload is sent by Discord. This payload
  // contains data like the current user's guild Ids, current user data,
  // private channels, and more.
  //
  // In this case, just print what the current user's username is.
  async fn ready(&self, _: Context, ready: Ready) {
      println!("{} is connected!", ready.user.name);
  }
}