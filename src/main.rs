extern crate diesel;
extern crate dotenv;
extern crate serenity;

use diesel::result::{DatabaseErrorKind::UniqueViolation, Error::NotFound, Error::DatabaseError};
use diesel::sqlite::SqliteConnection;
use dotenv::dotenv;
use serenity::client::Client;
use serenity::model::channel::{Channel, Message};
use serenity::model::id::{MessageId, UserId};
use serenity::model::user::User;
use serenity::prelude::{EventHandler, Context, TypeMapKey};
use serenity::framework::standard::{
    Args,
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
#[commands(quoth, pen)]  // TODO: !erase, !pen
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

/// Add a new quoth into the book of stars. This is a temporary command that should be replaced with a reaction-based approach.
#[command]
fn pen(ctx: &mut Context, msg: &Message, mut args: Args) -> CommandResult {
    if let Ok(msg_id) = args.single::<u64>() {
        match msg.channel(&ctx.cache) {
            Some(Channel::Guild(guild_channel)) => {
                if let Ok(pen_message) = guild_channel.read().message(&ctx, MessageId(msg_id)) {
                    let data = &ctx.data.read();
                    let conn = data.get::<Conn>().unwrap().lock().unwrap();
                    match stars_lib::add_quoth(
                        &conn, 
                        pen_message.author.id.0 as i64, 
                        msg.guild_id.unwrap().0 as i64, 
                        msg.author.id.0 as i64, 
                        pen_message.id.0 as i64,
                        &pen_message.content
                    ) {
                        Ok(_) => msg.reply(&ctx, format!("Penned \"{}\" by {} into the book of stars....", pen_message.content, pen_message.author.name)),
                        Err(DatabaseError(UniqueViolation, _)) => msg.reply(&ctx, "Already penned into the book of stars."),
                        Err(e) => msg.reply(&ctx, format!("Unknown error while penning into the book of stars: {:?}", e)),
                    }
                } else {
                    msg.reply(&ctx, "Couldn't find that message in this channel.")
                }
            },
            Some(_) => msg.reply(&ctx, "Wrong channel type."),
            None => msg.reply(&ctx, "Couldn't find channel in cache.")
        }
    } else {
        msg.reply(&ctx, "Invalid argument. Usage: !pen <message id>")
    }
    .and(Ok(())).or_else(|e| Err(CommandError::from(e)))
}