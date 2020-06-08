/// This tool is rather particular to the author's usage case.
/// Specifically, an earlier version of this program used Slack and stored quoths in mongodb. Since much of that content is still funny to me, I wanted
/// a program to copy over the data. This works by writing up a mapping in toml that maps all slack users to either a discord userid or to a legacy name
/// for people without a discord account.
extern crate regex;
#[macro_use]
extern crate serde;
extern crate serde_json;

use regex::Regex;
use serde::Deserialize;
use std::collections::HashMap;
use std::env;
use std::fs::read_to_string;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

#[derive(Debug, Deserialize)]
#[serde(untagged)]
/// We use this to read in a map of users from toml.
enum UserMapping {
    DiscordUser(u64),
    LegacyName(String)
}

#[derive(Deserialize)]
pub struct LegacyQuoth {
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
    if let Ok(lines) = read_lines(quoth_file_path) {
        // Consumes the iterator, returns an (Optional) String
        for line in lines {
            if let Ok(quote) = line {
                let q: LegacyQuoth = serde_json::from_str(&quote).unwrap();
                println!("{:?}: {}", user_map.get(&q.user).unwrap(), bracket_remover_re.replace_all(&q.text, "$url"));
            }
        }
    }

    // Load the translated quoths into the database.
    let conn = stars_lib::establish_connection();

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