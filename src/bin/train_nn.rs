use rusqlite::Connection;

fn main() {
    let conn = Connection::open("chess_games.db").expect("Failed to open database");

    // Implement your neural network training logic here
    // Example: Load data from the DB and train a model

    println!("Neural network training started...");

    // For example, read the data and print it
    let mut stmt = conn
        .prepare("SELECT white, black, result, moves FROM games")
        .expect("Failed to prepare statement");

    let game_iter = stmt
        .query_map([], |row| {
            Ok(GameData {
                white: row.get(0)?,
                black: row.get(1)?,
                result: row.get(2)?,
                moves: row.get(3)?,
            })
        })
        .expect("Failed to map query");

    for game in game_iter {
        println!("{:?}", game);
    }

    println!("Neural network training completed.");
}

#[derive(Debug)]
struct GameData {
    white: String,
    black: String,
    result: String,
    moves: String,
}