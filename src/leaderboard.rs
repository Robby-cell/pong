
use std::fs;

pub struct Leaderboard {
    top_ten: Box<[Top_Player; 10]>,
}

impl Leaderboard {

    pub fn new() -> Result<(), Box<dyn std::error::Error>> {
        let all_players = fs::read_to_string("src/top_players.json")?;

        println!("{}", all_players);

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
