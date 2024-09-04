use chess::{Board, ChessMove, Game};

fn main() {
    // Create a new chess game
    let mut game = Game::new();

    // Sample list of moves (in SAN format)
    let moves = vec!["d4", "c5", "dxc5", "Qa5+"];

    // Loop over the moves and display the board
    for mov in moves {
        if let Ok(chess_move) = ChessMove::from_san(&game.current_position(), mov) {
            game.make_move(chess_move);
            println!("Move made: {}", mov);
            println!("{}", game.current_position());
        } else {
            println!("Failed to parse move: {}", mov);
        }
    }
}
