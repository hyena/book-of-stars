extern crate diesel;
extern crate dotenv;
extern crate serenity;

use diesel::sqlite::SqliteConnection;
use dotenv::dotenv;
use serenity::client::Client;
use serenity::model::channel::Message;
use serenity::model::id::UserId;
use serenity::model::user::User;
use serenity::prelude::{EventHandler, Context, TypeMapKey};
use serenity::framework::standard::{
    StandardFramework,
    CommandError,
    CommandResult,
    macros::{
        command,
        group
    },
    
};
use serenity::utils::MessageBuilder;
use std::env;
use std::sync::Mutex;

struct Conn;
impl TypeMapKey for Conn {
    type Value = Mutex<SqliteConnection>;
}

#[group]
#[commands(quoth)]  // TODO: !erase, !pen
struct General;

struct Handler;
impl EventHandler for Handler {}

 fn main() {
    dotenv().ok();
    let conn = stars_lib::establish_connection();

    let mut client = Client::new(&env::var("DISCORD_TOKEN").expect("token"), Handler)
        .expect("Error creating client");
    client.with_framework(StandardFramework::new()
        .configure(|c| c.prefix("!"))
        .group(&GENERAL_GROUP));
    {
        let mut data = client.data.write();
        // Set-up our timeout system
        data.insert::<Conn>(Mutex::new(conn));
    }
    client.start().expect("Error connecting to discord");
}

#[command]
fn quoth(ctx: &mut Context, msg: &Message) -> CommandResult {
    let data = ctx.data.read();
    let conn = data.get::<Conn>().unwrap().lock().unwrap();

    if let Ok(quoth) = stars_lib::random_quoth(&conn, None) {
        // Inefficient String creation here till I learn how to make references live long enough.
        let name = if let Some(user_id) = quoth.author {
            match ctx.cache.read().user(user_id as u64) {
                Some(u) => u.read().name.clone(),
                None => String::from("UNKNOWN"),
            }
        } else if let Some(legacy_name) = &quoth.legacy_author_fallback {
            String::from(legacy_name)
        } else {
            String::from("UNKNOWN")
        };
        let response = MessageBuilder::new()
            .push_bold_safe(&name)
            .push(":\t\t (")
            .push(quoth.id)
            .push(")\n")
            .push(quoth.content)
            .build();
        msg.channel_id.say(&ctx.http, &response).and(Ok(())).or_else(|e| Err(CommandError::from(e)))
    } else {
        msg.reply(&ctx, "No quoths found. Consider better posting").and(Ok(())).or_else(|e| Err(CommandError::from(e)))
    }
}