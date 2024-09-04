use rusqlite::Connection;
use chess::{Board, ChessMove, Game};
use std::str::FromStr;

fn main() {
    let conn = Connection::open("chess_games.db").expect("Failed to open database");

    println!("Extracting chess data for neural network training...");

    let mut stmt = conn
        .prepare("SELECT result, moves FROM games")
        .expect("Failed to prepare statement");

    let game_iter = stmt
        .query_map([], |row| {
            let result: String = row.get(0)?;
            let moves: String = row.get(1)?;
            Ok((result, moves))
        })
        .expect("Failed to map query");

    for game in game_iter {
        let (result, moves) = game.expect("Failed to extract game data");

        let mut game = Game::new();
        for mov in moves.split_whitespace() {
            if let Ok(chess_move) = ChessMove::from_san(&game.current_position(), mov) {
                game.make_move(chess_move);
            } else {
                println!("Failed to parse move: {}", mov);
            }
        }

        // Here, you can convert the board state to a vector and the result to a label (0, 1 for win/loss)
        let board_vector = board_to_vector(game.current_position());
        let result_label = result_to_label(&result);

        // Example print of the vector and result
        println!("Board Vector: {:?}", board_vector);
        println!("Result: {}", result_label);
    }

    println!("Chess data extraction completed.");
}

// Convert board state to a vector of 1s and 0s
fn board_to_vector(board: &Board) -> Vec<f32> {
    let mut vector = vec![0.0; 64]; // Example: a 64-length vector for the board state
    for square in board.squares() {
        if let Some(piece) = board.piece_on(square) {
            // You can customize this representation
            vector[square.to_index()] = 1.0;
        }
    }
    vector
}

// Convert result to a label (0 for loss, 1 for win)
fn result_to_label(result: &str) -> u8 {
    match result {
        "1-0" => 1,
        "0-1" => 0,
        _ => 2, // Could also handle draws separately if desired
    }
}
