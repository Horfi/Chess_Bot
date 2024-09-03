mod pgn_parser;

use rusqlite::{Connection, Result};



fn main() -> Result<()> {
    let conn = Connection::open("catSSs.db")?;

    // Create the cat_colors table
    conn.execute(
        "CREATE TABLE IF NOT EXISTS cat_colors (
            id INTEGER PRIMARY KEY,
            name TEXT NOT NULL UNIQUE
        )",
        [],
    )?;

    // Create the cats table
    conn.execute(
        "CREATE TABLE IF NOT EXISTS cats (
            id INTEGER PRIMARY KEY,
            name TEXT NOT NULL,
            color_id INTEGER NOT NULL REFERENCES cat_colors(id)
        )",
        [],
    )?;

    Ok(())
}
