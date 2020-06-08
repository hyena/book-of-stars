/// This tool is rather particular to the author's usage case.
/// Specifically, an earlier version of this program used Slack and stored quoths in mongodb. Since much of that content is still funny to me, I wanted
/// a program to copy over the data. This works by writing up a mapping in toml that maps all slack users to either a discord userid or to a legacy name
/// for people without a discord account.
extern crate diesel;
extern crate regex;
#[macro_use]
extern crate serde;
extern crate serde_json;

use diesel::prelude::*;
use diesel::result::Error;
use regex::Regex;
use serde::Deserialize;
use std::collections::HashMap;
use std::env;
use std::fs::read_to_string;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

use stars_lib::models::NewLegacyQuoth;
use stars_lib::schema::quoths;

#[derive(Debug, Deserialize)]
#[serde(untagged)]
/// We use this to read in a map of users from toml.
enum UserMapping {
    DiscordUser(u64),
    LegacyName(String)
}

#[derive(Deserialize)]
pub struct SlackQuoth {
    text: String,
    user: String,
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 3 {
        panic!("Usage: {} <toml map path> <quoth file path>")
    }
    let toml_map_path = &args[1];
    let quoth_file_path = &args[2];

    let mut user_map = parse_toml_map(toml_map_path);
    user_map.insert("".to_string(), UserMapping::LegacyName("UNKNOWN".to_string()));


    let bracket_remover_re = Regex::new(r"<(?P<url>.*?)>").unwrap();
    let legacy_quoths: Vec<NewLegacyQuoth> = read_lines(quoth_file_path).unwrap().map(|l| {
        let sq: SlackQuoth = serde_json::from_str(&l.expect("Error reading quote lines")).unwrap();
        let authorship = user_map.get(&sq.user).unwrap();
        NewLegacyQuoth {
            author: match authorship {
                UserMapping::DiscordUser(id) => Some(*id as i64),
                UserMapping::LegacyName(_) => None,
            },
            legacy_author_fallback: match authorship {
                UserMapping::DiscordUser(_) => None,
                UserMapping::LegacyName(name) => Some(name.to_string()),
            },
            content: bracket_remover_re.replace_all(&sq.text, "$url").to_string(),
        }
    }).collect();

    // Load the translated quoths into the database.
    let conn = stars_lib::establish_connection();
    conn.transaction::<(), Error, _>(|| {
        for lq in legacy_quoths {
            diesel::insert_into(quoths::table)
                .values(&lq)
                .execute(&conn)?;
        }
        Ok(())
    }).expect("Error inserting legacy quoths. Transaction rolled back.");
}

fn parse_toml_map<P>(filename: P) -> HashMap<String, UserMapping> where P: AsRef<Path> {
    toml::from_str(&read_to_string(filename).unwrap()).unwrap()
}

// The output is wrapped in a Result to allow matching on errors
// Returns an Iterator to the Reader of the lines of the file.
fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>> 
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}