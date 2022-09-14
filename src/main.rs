use anyhow::Result;
use chess::*;

fn main() -> Result<()> {
    let mut board = GameBoard::new();
    println!("{}", board);

    board.move_piece(Position::new(0, 1)?, Position::new(0, 3)?)?;
    board.move_piece(Position::new(1, 0)?, Position::new(2, 2)?)?;
    
    // invalid move for pawn
    // board.move_piece(Position::new(1, 1)?, Position::new(0, 2)?)?;

    println!("{}", board);

    Ok(())
}
