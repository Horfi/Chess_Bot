use rusqlite::{Connection, Result};
use chess::{Game, ChessMove, Piece, Color};
use std::vec::Vec;

fn main() -> Result<()> {
    // Sample move history (replace this with the move history you retrieve from your database)
    let moves_history = "d3 d5 Nd2 c5 e3 Nc6 Qe2 e5 f3 e4 f4 exd3 cxd3 Nf6 Nh3 Bxh3 gxh3 Bd6 Bg2 O-O Nf3 Re8 Kf2 d4 e4 c4 e5 cxd3 Qxd3 Nb4 Qxd4 b6 Be3 Bc5 Qxd8 Raxd8 Bxc5 bxc5 Rad1 Nfd5 Ng5 Nxa2 Bxd5 Nb4 Bxf7+ Kh8 Bxe8 Rxe8 Rd6 c4 Rhd1 Nd3+ Kg3 h6 Nf7+ Kh7 Rd8 Re7 Nd6 Rc7 Rf8 Nxb2 Rd5 c3 e6 c2 e7 h5+ Rg5 Nd3";

    // Split the moves into a vector
    let moves_vec: Vec<&str> = moves_history.split_whitespace().collect();

    // Initialize a new chess game
    let mut game = Game::new();

    // Process each move and generate board tensors
    for i in 0..(moves_vec.len() - 1) {
        // Get the current move
        let current_move = moves_vec[i];

        // Apply the current move to the game
        if let Ok(chess_move) = ChessMove::from_san(&game.current_position(), current_move) {
            game.make_move(chess_move);
        } else {
            println!("Failed to parse move: {}", current_move);
        }

        // Generate the tensor for the current board state after the move
        let board_state = game.current_position();
        let tensor = generate_board_tensor(board_state);

        // Print the tensor in a more readable format, with all layers side by side
        println!("Board state after move {}: ", current_move);
        print_tensor_row_format(&tensor);
    }

    Ok(())
}

// Function to generate the 8x8x6 tensor from the current board position
fn generate_board_tensor(board: chess::Board) -> Vec<Vec<Vec<i8>>> {
    // Initialize a 3D tensor: 8 rows, 8 columns, and 6 layers for different pieces
    let mut tensor = vec![vec![vec![0; 8]; 8]; 6];

    for sq in chess::ALL_SQUARES {
        if let Some(piece) = board.piece_on(sq) {
            let piece_color = board.color_on(sq).expect("No piece color found");

            let (piece_layer, piece_value) = match piece {
                Piece::Pawn => (0, get_piece_value(piece_color)),  // Layer 0 for pawns
                Piece::Rook => (1, get_piece_value(piece_color)),  // Layer 1 for rooks
                Piece::Knight => (2, get_piece_value(piece_color)),// Layer 2 for knights
                Piece::Bishop => (3, get_piece_value(piece_color)),// Layer 3 for bishops
                Piece::Queen => (4, get_piece_value(piece_color)), // Layer 4 for queens
                Piece::King => (5, get_piece_value(piece_color)),  // Layer 5 for kings
            };

            // Convert square index into 2D board coordinates
            let (rank, file) = square_to_rank_file(sq);

            // Set the value in the tensor
            tensor[piece_layer][rank][file] = piece_value;
        }
    }

    tensor
}

// Helper function to convert piece color to tensor value
fn get_piece_value(color: Color) -> i8 {
    match color {
        Color::White => 1,
        Color::Black => -1,
    }
}

// Helper function to convert a chess::Square to 2D board coordinates
fn square_to_rank_file(sq: chess::Square) -> (usize, usize) {
    let rank = sq.get_rank().to_index();
    let file = sq.get_file().to_index();
    (rank, file)
}

// Function to print all the 2D tensors side by side, one for each piece type
fn print_tensor_row_format(tensor: &Vec<Vec<Vec<i8>>>) {
    let layers = ["Pawns", "Rooks", "Knights", "Bishops", "Queens", "Kings"];

    // Print header with piece type names
    println!("Pieces:    {}                    {}                    {}                    {}                    {}                    {}", layers[0], layers[1], layers[2], layers[3], layers[4], layers[5]);

    // Print the 8x8 grids for each piece type side by side
    for rank in 0..8 {
        for layer in 0..6 {
            for file in 0..8 {
                print!("{:2} ", tensor[layer][rank][file]);
            }
            print!("   "); // Add space between layers
        }
        println!(); // Newline after each rank
    }
    println!(); // Extra newline after the full tensor row for better separation
}
