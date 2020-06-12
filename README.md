# Book of Stars (WIP)
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

### Known Issues
  * Currently the bot only works with one server at a time. The data model needs tweaking to support more.

### TODO
  - [x] Write a basic data model
  - [x] Write code to add quoths and query them
  - [ ] Write bot to record quoths using reactions
  - [X] Add bot functionality to query and return quoths
  - [X] Write utility to load legacy quoths from slack
  - [X] ~~Write utility to retrieve pinned messages as quoths (Do them via message id)~~ (do this via message snowflake ids instead)
  - [ ] Functionality to pick your own emojis via `.env`
  - [X] Functionality to save quoth via message id
  - [X] Functionality to delete bad or buggy quoths

### History
This bot is a re-implementation of my original `quoth` bot for slack. That was implemented in Go and used Mongo (ick) for storage. Now we use Rust, a relational database, and connect to Discord. The world is better.