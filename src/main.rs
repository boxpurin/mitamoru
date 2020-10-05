use std::env;
use serenity::{
  client::Client
};

mod handler;

#[tokio::main]
async fn main() {
  pretty_env_logger::init();
  //dotenv().ok();

  let token = env::var("DISCORD_TOKEN")
    .expect("Expected a token in the enviroment");
  
  println!("DISCODE_TOKEN {}",token);

    // Create a new instance of the Client, logging in as a bot. This will
    // automatically prepend your bot token with "Bot ", which is a requirement
    // by Discord for bot users.
    let mut client = Client::new(token)
        .event_handler(handler::Handler)
        .await
        .expect("Err creating client");

    // Finally, start a single shard, and start listening to events.
    //
    // Shards will automatically attempt to reconnect, and will perform
    // exponential backoff until it reconnects.
    if let Err(why) = client.start().await {
        println!("Client error: {:?}", why);
    }
}

