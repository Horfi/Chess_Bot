pub mod pgn_parser;

use pgn_parser::{GameRecord, parse_pgn_to_game_record};

use rusqlite::{params, Connection, Result};

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
            game_record.moves.join(" "),
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

    let pgn_data = r#"
    [Event "Rated Blitz game"]
    [Site "https://lichess.org/c1rfp1ck"]
    [White "dragansimic1946"]
    [Black "aicho"]
    [Result "0-1"]
    [UTCDate "2013.06.30"]
    [UTCTime "22:05:06"]
    [WhiteElo "1685"]
    [BlackElo "1577"]
    [WhiteRatingDiff "-21"]
    [BlackRatingDiff "+14"]
    [ECO "A43"]
    [Opening "Benoni Defense: Benoni Gambit Accepted"]
    [TimeControl "300+0"]
    [Termination "Time forfeit"]

    1. d4 c5 2. dxc5 Qa5+ 3. Nd2 Qxc5 4. Nb3 Qc7 5. Nf3 d6 6. e4 e5 7. Bd3 Bg4 8. h3 Bxf3 9. Qxf3 Nf6 10. Bg5 Be7 11. O-O a5 12. c3 a4 13. Nd2 h6 0-1
    "#;

    if let Some(record) = parse_pgn_to_game_record(pgn_data) {
        insert_game_record(&conn, &record)?;
        println!("Game record has been saved to the database.");
    } else {
        println!("Failed to parse PGN data");
    }

    Ok(())
}
