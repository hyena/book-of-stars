# Book of Stars
A discord bot to save quotes from your favorite servers and play them back.

## Usage
  1. `cargo install diesel`
  2. `diesel migrations run` to make an empty sqlite database
  3. Copy `sample.env` to `.env`
  4. Fill in your discord token and guild id in `.env`
  5. Compile and run the bot
  6. Use the star emoji reaction to tag messages as quoths in your server
  7. Use `!quoth` in discord to get a random quoth and `!quoth <username>` to get one from a particular user (checks discord id then nicknames)

The star emoji is a constant in the src file and can be changed.

## Known Issues
  * Currently the bot only works with one server at a time. The data model needs tweaking to support more.

## History
This bot is a re-implementation of my original `quoth` bot for slack. That was implemented in Go and used Mongo (ick) for storage. Now we use Rust, a relational database, and connect to Discord. The world is better.