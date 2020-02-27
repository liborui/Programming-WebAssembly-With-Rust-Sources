extern crate wasmi;

mod checkersgame;
mod imports;
mod runtime;

use checkersgame::CheckersGame;
use std::error::Error;

fn main() -> Result<(), Box<Error>> {
    let mut game = CheckersGame::new("../../Champ.2/wasmcheckers/checkers.wasm");
    game.init()?;

    let board_display = game.get_board_contents()?;
    println!("game board at start:\n{}\n", board_display);

    println!(
        "At game start, current turn is : {:?}",
        game.get_turn_owner()?
    );
    game.move_piece(&(0, 5), &(0, 4))?;
    println!(
        "After first move, current turn is : {:?}",
        game.get_turn_owner()?
    );

    let board_display = game.get_board_contents()?;
    println!("game board after 1 move:\n{}\n", board_display);

    Ok(())
}