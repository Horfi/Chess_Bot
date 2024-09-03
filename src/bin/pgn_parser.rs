use chess::{ChessMove, Game, Action};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct GameRecord {
    pub white: String,
    pub black: String,
    pub result: String,
    pub moves: Vec<String>,
    pub white_elo: i32,
    pub black_elo: i32,
    pub opening: String,
    pub time_control: String,
    pub termination: String,
}

pub fn parse_pgn_to_game_record(pgn: &str) -> Option<GameRecord> {
    let mut game = Game::new();
    let mut white = String::new();
    let mut black = String::new();
    let mut result = String::new();
    let mut white_elo = 0;
    let mut black_elo = 0;
    let mut opening = String::new();
    let mut time_control = String::new();
    let mut termination = String::new();
    let mut moves = Vec::new();
    let mut parsing_moves = false;

    for line in pgn.lines() {
        let line = line.trim();  // Trim whitespace around the line

        println!("Processing line: {}", line); // Debugging statement

        if line.starts_with("[") {
            // Parsing PGN headers (metadata)
            if line.contains("White ") {
                white = line.split('"').nth(1).unwrap_or("").to_string();
                println!("Parsed White: {}", white);
            } else if line.contains("Black ") {
                black = line.split('"').nth(1).unwrap_or("").to_string();
                println!("Parsed Black: {}", black);
            } else if line.contains("Result ") {
                result = line.split('"').nth(1).unwrap_or("").to_string();
                println!("Parsed Result: {}", result);
            } else if line.contains("WhiteElo ") {
                white_elo = line.split('"').nth(1).unwrap_or("1600").parse().unwrap_or(1600);
                println!("Parsed White Elo: {}", white_elo);
            } else if line.contains("BlackElo ") {
                black_elo = line.split('"').nth(1).unwrap_or("1600").parse().unwrap_or(1600);
                println!("Parsed Black Elo: {}", black_elo);
            } else if line.contains("Opening ") {
                opening = line.split('"').nth(1).unwrap_or("").to_string();
                println!("Parsed Opening: {}", opening);
            } else if line.contains("TimeControl ") {
                time_control = line.split('"').nth(1).unwrap_or("").to_string();
                println!("Parsed Time Control: {}", time_control);
            } else if line.contains("Termination ") {
                termination = line.split('"').nth(1).unwrap_or("").to_string();
                println!("Parsed Termination: {}", termination);
            }
        } else if line.starts_with("1.") {
            // We have reached the moves section when the line starts with a number (like "1.")
            parsing_moves = true;
            println!("Starting to parse moves...");
        }

        if parsing_moves {
            // Parsing the actual moves, skipping move numbers and checking for actual SAN (Standard Algebraic Notation) moves
            for mov in line.split_whitespace() {
                if mov.contains(".") {
                    // Skip move numbers like "1.", "2.", etc.
                    continue;
                }
                if let Ok(chess_move) = ChessMove::from_san(&game.current_position(), mov) {
                    game.make_move(chess_move);
                    moves.push(mov.to_string());
                    println!("Parsed Move: {}", mov);
                } else {
                    println!("Failed to parse move: {}", mov);
                }
            }
        }
    }

    let result_str = match result.as_str() {
        "1-0" => "White wins".to_string(),
        "0-1" => "Black wins".to_string(),
        "1/2-1/2" => "Draw".to_string(),
        _ => "Unknown".to_string(),
    };
    println!("Final Result: {}", result_str);

    Some(GameRecord {
        white,
        black,
        result: result_str,
        moves,
        white_elo,
        black_elo,
        opening,
        time_control,
        termination,
    })
}
