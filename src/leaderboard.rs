extern crate serde_json;


use std::fs;
use serde_json::{Result, Value};


fn read_json(path: &str) -> std::result::Result<Value, Box<dyn std::error::Error>> {

    let all_players = fs::read_to_string(path)?;

    let val: Value = serde_json::json!(&all_players);

    Ok(val)
}

pub struct Leaderboard {
    top_ten: Box<[Top_Player; 10]>,
}

impl Leaderboard {

    pub fn new() -> std::result::Result<(), Box<dyn std::error::Error>> {

        let all_players = read_json("assets/top_players.json")?;

        println!("{:#?}", all_players);

        Ok(())
    }
}


pub struct Top_Player {
    name: String,
    score: u32,
}

impl Top_Player {

    pub fn add(&self) {
        todo!("add the player to top_players.json")
    }


}
