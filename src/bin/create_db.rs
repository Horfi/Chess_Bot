use std::fs;
use pgn_parser::{GameRecord, parse_pgn_to_game_record};
use rusqlite::{params, Connection, Result, Transaction};

// Ensure this is correct for your project structure
mod pgn_parser;

fn main() -> Result<()> {
    let mut conn = create_db_connection()?; // Declare conn as mutable

    // Read the file content
    let file_content = fs::read_to_string("D:/Chess_Bot/Data/lichess_db_standard_rated_2013-07.pgn")
        .expect("Failed to read PGN file");

    // Split the content by two newlines, which is the typical separator for PGN games
    let pgn_games: Vec<&str> = file_content.split("\n\n[Event").collect();

    println!("Number of PGN games found: {}", pgn_games.len());

    let mut total_inserted = 0; // Track the total number of inserted games
    let mut batch_inserted = 0; // Track how many games have been inserted in the current batch

    // Start the first transaction
    let mut transaction = conn.transaction()?;

    for (index, pgn_data) in pgn_games.iter().enumerate() {
        // We need to re-add the '[Event' tag to each PGN, except the first one.
        let pgn_data = if index == 0 {
            pgn_data.to_string()
        } else {
            format!("[Event{}", pgn_data)
        };

        if let Some(record) = parse_pgn_to_game_record(&pgn_data) {
            // Skip games where both Elo ratings are below 2000
            if record.white_elo >= 2000 || record.black_elo >= 2000 {
                if let Err(e) = insert_game_record(&transaction, &record) {
                    println!("Error inserting game record: {}", e);
                } else {
                    batch_inserted += 1;
                    total_inserted += 1;
                }

                // Commit the transaction after every 500 records
                if batch_inserted >= 1000 {
                    println!("Committing batch of 1000 records...");
                    if let Err(e) = transaction.commit() {
                        println!("Failed to commit transaction: {}", e);
                    } else {
                        println!("Batch of 1000 records committed successfully.");
                    }
                    batch_inserted = 0; // Reset the batch counter
                
                    // Start a new transaction
                    transaction = conn.transaction()?;
                }
                
            }
        } else {
            println!("Failed to parse PGN data at index: {}", index);
        }
    }

    // Commit any remaining records that haven't been committed yet
    if batch_inserted > 0 {
        println!("Committing final batch of {} records...", batch_inserted);
        transaction.commit()?;
    }

    println!("All game records have been saved to the database. Total inserted: {}", total_inserted);

    Ok(())
}

fn create_db_connection() -> Result<Connection> {
    let conn = Connection::open("chess_games.db")?;
    conn.execute(
        "CREATE TABLE IF NOT EXISTS games (
                  id INTEGER PRIMARY KEY,
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

fn insert_game_record(transaction: &Transaction, game_record: &GameRecord) -> Result<()> {
    transaction.execute(
        "INSERT INTO games (result, moves, white_elo, black_elo, opening, time_control, termination)
                  VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7)",
        params![
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
