use std::fs;
use pgn_parser::{GameRecord, parse_pgn_to_game_record};
use rusqlite::{params, Connection, Result};

// Ensure this is correct for your project structure
mod pgn_parser;

fn create_db_connection() -> Result<Connection> {
    let conn = Connection::open("chess_games.db")?;
    conn.execute(
        "CREATE TABLE IF NOT EXISTS games (
                  id INTEGER PRIMARY KEY,
                  white TEXT NOT NULL,
                  black TEXT NOT NULL,
                  result TEXT NOT NULL,
                  moves TEXT NOT NULL,
                  white_elo INTEGER,
                  black_elo INTEGER,
                  opening TEXT,
                  time_control TEXT,
                  termination TEXT
                  )",
        [],
    )?;
    Ok(conn)
}

fn insert_game_record(conn: &Connection, game_record: &GameRecord) -> Result<()> {
    conn.execute(
        "INSERT INTO games (white, black, result, moves, white_elo, black_elo, opening, time_control, termination)
                  VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9)",
        params![
            game_record.white,
            game_record.black,
            game_record.result,
            game_record.moves.join(" "),  // Convert Vec<String> to a single string
            game_record.white_elo,
            game_record.black_elo,
            game_record.opening,
            game_record.time_control,
            game_record.termination,
        ],
    )?;
    Ok(())
}

fn main() -> Result<()> {
    let conn = create_db_connection()?;

    // Read the file content
    let file_content = fs::read_to_string("D:/Chess_Bot/Data/lichess_db_standard_rated_2013-07.pgn")
        .expect("Failed to read PGN file");

    // Split the content by two newlines, which is the typical separator for PGN games
    let pgn_games: Vec<&str> = file_content.split("\n\n[Event").collect();

    for (index, pgn_data) in pgn_games.iter().enumerate() {
        // We need to re-add the '[Event' tag to each PGN, except the first one.
        let pgn_data = if index == 0 {
            pgn_data.to_string()
        } else {
            format!("[Event{}", pgn_data)
        };

        if let Some(record) = parse_pgn_to_game_record(&pgn_data) {
            insert_game_record(&conn, &record)?;
            //println!("Game record has been saved to the database.");
        } else {
            println!("Failed to parse PGN data");
        }
    }

    Ok(())
}
