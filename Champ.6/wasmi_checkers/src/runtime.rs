use super::imports::{PIECECROWNED_INDEX, PIECEMOVED_INDEX};

use wasmi::{Externals, RuntimeArgs, RuntimeValue, Trap};

pub struct Runtime {} // (1) Great place to put the module-specific state

impl Runtime {
    pub fn new() -> Runtime {
        Runtime {}
    }

    fn handle_piece_moved( // (2) Dispatcher calls this function when appropriate
        &self,
        from: (i32, i32),
        to: (i32, i32),
    ) -> Result<Option<RuntimeValue>, Trap> {
        println!(
            "A piece was moved from ({},{}) to ({},{})",
            from.0, from.1, to.0, to.1
        );
        Ok(None)
    }

    fn handle_piece_crowned( // (3) Dispatcher calls this function when appropriate
        &self,
        loc: (i32, i32)) -> Result<Option<RuntimeValue>, Trap> {
        println!("A piece was crowned at ({},{})", loc.0, loc.1);
        Ok(None)
    }
}

impl Externals for Runtime {
    fn invoke_index( // (4) Central dispatcher, converts function index to call result
        &mut self,
        index: usize,
        args: RuntimeArgs,
    ) -> Result<Option<RuntimeValue>, Trap> {
        match index {
            PIECECROWNED_INDEX => {
                let piece_x: i32 = args.nth(0);
                let piece_y: i32 = args.nth(1);
                self.handle_piece_crowned((piece_x, piece_y))
            }
            PIECEMOVED_INDEX => {
                let from_x: i32 = args.nth(0);
                let from_y: i32 = args.nth(1);
                let to_x: i32 = args.nth(2);
                let to_y: i32 = args.nth(3);
                self.handle_piece_moved((from_x, from_y), (to_x, to_y))
            }
            _ => panic!("unknown function index"),
        }
    }
}