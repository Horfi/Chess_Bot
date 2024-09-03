// src/pgn_parser.rs

use crate::db_manager::{GameRecord, MoveRecord};
use chess::{Board, ChessMove, Color, Game, GameResult, Square};
use chess_pgn::{Pgn, SanPlus};
use std::fs;
use std::path::Path;

pub struct PgnParser;

impl PgnParser {
    /// Parses a PGN file and returns a vector of game records along with their moves.
    pub fn parse_pgn_file(pgn_path: &Path) -> Result<Vec<(GameRecord, Vec<MoveRecord>)>, String> {
        let pgn_content = fs::read_to_string(pgn_path)
            .map_err(|e| format!("Failed to read PGN file: {}", e))?;

        let parsed_pgn = Pgn::parse(&pgn_content)
            .map_err(|e| format!("Failed to parse PGN content: {}", e))?;

        let mut games_data = Vec::new();

        for pgn_game in parsed_pgn.iter() {
            let game_record = Self::extract_game_record(pgn_game)?;
            let moves_record = Self::extract_moves_record(pgn_game)?;

            games_data.push((game_record, moves_record));
        }

        Ok(games_data)
    }

    /// Extracts game metadata from PGN tags.
    fn extract_game_record(pgn_game: &Game) -> Result<GameRecord, String> {
        let header = &pgn_game.tags;

        Ok(GameRecord {
            event: header.get("Event").unwrap_or(&"Unknown Event".to_string()).clone(),
            site: header.get("Site").unwrap_or(&"Unknown Site".to_string()).clone(),
            date: header.get("Date").unwrap_or(&"????.??.??".to_string()).clone(),
            round: header.get("Round").unwrap_or(&"-".to_string()).clone(),
            white_player: header.get("White").unwrap_or(&"Unknown Player".to_string()).clone(),
            black_player: header.get("Black").unwrap_or(&"Unknown Player".to_string()).clone(),
            result: header.get("Result").unwrap_or(&"*".to_string()).clone(),
            white_elo: header.get("WhiteElo").and_then(|elo| elo.parse().ok()),
            black_elo: header.get("BlackElo").and_then(|elo| elo.parse().ok()),
            eco: header.get("ECO").unwrap_or(&"Unknown ECO".to_string()).clone(),
            time_control: header.get("TimeControl").unwrap_or(&"-".to_string()).clone(),
            termination: header.get("Termination").unwrap_or(&"-".to_string()).clone(),
        })
    }

    /// Extracts move records from a PGN game.
    fn extract_moves_record(pgn_game: &Game) -> Result<Vec<MoveRecord>, String> {
        let mut moves = Vec::new();
        let mut board = Board::default();

        for (index, san_plus) in pgn_game.moves.iter().enumerate() {
            let san = match san_plus {
                SanPlus::Move(mv) => mv.to_string(),
                SanPlus::Comment(_) => continue, // Skip comments
                SanPlus::NAG(_) => continue,     // Skip NAGs
            };

            let chess_move = san_plus.to_move(&board)
                .map_err(|e| format!("Failed to convert SAN to move: {}", e))?;

            board = board.make_move_new(chess_move);

            moves.push(MoveRecord {
                move_number: (index as u32) + 1,
                san_move: san,
                fen: board.to_string(),
            });
        }

        Ok(moves)
    }
}
