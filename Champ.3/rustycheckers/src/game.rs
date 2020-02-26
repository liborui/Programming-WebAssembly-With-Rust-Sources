use super::board::{Coordinate, GamePiece, Move, PieceColor};

pub struct GameEngine {
    board: [[Option<GamePiece>; 8]; 8],
    current_turn: PieceColor,
    move_count: u32,
}

pub struct MoveResult {
    pub mv: Move,
    pub crowned: bool,
}

impl GameEngine {
    pub fn new() -> GameEngine {
        let mut engine = GameEngine {
            board: [[None; 8]; 8],
            current_turn: PieceColor::Black,
            move_count: 0,
        };
        engine.initialize_pieces();
        engine
    }

    pub fn initialize_pieces(&mut self) {
        [1, 3, 5, 7, 0, 2, 4, 6, 1, 3, 5, 7]
            .iter()
            .zip([0, 0, 0, 0, 1, 1, 1, 1, 2, 2, 2, 2].iter())
            .map(|(a, b)| (*a as usize, *b as usize))
            .for_each(|(x, y)| {
                self.board[x][y] = Some(GamePiece::new(PieceColor::White));
            });

        [0, 2, 4, 6, 1, 3, 5, 7, 0, 2, 4, 6]
            .iter()
            .zip([5, 5, 5, 5, 6, 6, 6, 6, 7, 7, 7, 7].iter())
            .map(|(a, b)| (*a as usize, *b as usize))
            .for_each(|(x, y)| {
                self.board[x][y] = Some(GamePiece::new(PieceColor::Black));
            });
    }

    pub fn move_piece(&mut self, mv: &Move) -> Result<MoveResult, ()> {
        let legal_moves = self.legal_moves();

        if !legal_moves.contains(mv) {
            return Err(());
        }

        let Coordinate(fx, fy) = mv.from;
        let Coordinate(tx, ty) = mv.to;
        let piece = self.board[fx][fy].unwrap();
        let midpiece_coordinate = self.midpiece_coordinate(fx, fy, tx, ty);
        if let Some(Coordinate(x, y)) = midpiece_coordinate {
            self.board[x][y] = None; // remove the jumped piece
        }

        // Move piece from source to dest
        self.board[tx][ty] = Some(piece);
        self.board[fx][fy] = None;

        let crowned = if self.should_crown(piece, mv.to) {
            self.crown_piece(mv.to);
            true
        } else {
            false
        };
        self.advance_turn();

        Ok(MoveResult {
            mv: mv.clone(),
            crowned: crowned,
        })
    }

    pub fn get_piece(&self, coord: Coordinate) -> Result<Option<GamePiece>, ()> {
        let Coordinate(x, y) = coord;
        if x <= 7 && y <= 7 {
            Ok(self.board[x][y])
        } else {
            Err(())
        }
    }

    pub fn current_turn(&self) -> PieceColor {
        self.current_turn
    }

    fn advance_turn(&mut self) {
        if self.current_turn == PieceColor::Black {
            self.current_turn = PieceColor::White
        } else {
            self.current_turn = PieceColor::Black
        }
        self.move_count += 1;
    }

    // Black pieces in row 0 or White pieces in row 7 are crowned
    fn should_crown(&self, piece: GamePiece, coord: Coordinate) -> bool {
        let Coordinate(_x, y) = coord;

        (y == 0 && piece.color == PieceColor::Black) || (y == 7 && piece.color == PieceColor::White)
    }

    fn crown_piece(&mut self, coord: Coordinate) -> bool {
        let Coordinate(x, y) = coord;
        if let Some(piece) = self.board[x][y] {
            self.board[x][y] = Some(GamePiece::crowned(piece));
            true
        } else {
            false
        }
    }

    pub fn is_crowned(&self, coord: Coordinate) -> bool {
        let Coordinate(x, y) = coord;
        match self.board[x][y] {
            Some(piece) => piece.crowned,
            None => false,
        }
    }

    pub fn move_count(&self) -> u32 {
        self.move_count
    }

    fn legal_moves(&self) -> Vec<Move> {
        let mut moves: Vec<Move> = Vec::new();
        for col in 0..8 {
            for row in 0..8 {
                if let Some(piece) = self.board[col][row] {
                    if piece.color == self.current_turn {
                        let loc = Coordinate(col, row);
                        let mut vmoves = self.valid_moves_from(loc);
                        moves.append(&mut vmoves);
                    }
                }
            }
        }

        moves
    }

    fn valid_moves_from(&self, loc: Coordinate) -> Vec<Move> {
        let Coordinate(x, y) = loc;
        if let Some(p) = self.board[x][y] {
            let mut jumps = loc
                .jump_targets_from()
                .filter(|t| self.valid_jump(&p, &loc, &t))
                .map(|ref t| Move {
                    from: loc.clone(),
                    to: t.clone(),
                }).collect::<Vec<Move>>();
            let mut moves = loc
                .move_targets_from()
                .filter(|t| self.valid_move(&p, &loc, &t))
                .map(|ref t| Move {
                    from: loc.clone(),
                    to: t.clone(),
                }).collect::<Vec<Move>>();
            jumps.append(&mut moves);
            jumps
        } else {
            Vec::new()
        }
    }

    fn midpiece_coordinate(&self, x: usize, y: usize, tx: usize, ty: usize) -> Option<Coordinate> {
        if tx == x + 2 && ty == y + 2 {
            Some(Coordinate(x + 1, y + 1))
        } else if x >= 2 && y >= 2 && tx == x - 2 && ty == y - 2 {
            Some(Coordinate(x - 1, y - 1))
        } else if x >= 2 && tx == x - 2 && ty == y + 2 {
            Some(Coordinate(x - 1, y + 1))
        } else if y >= 2 && tx == x + 2 && ty == y - 2 {
            Some(Coordinate(x + 1, y - 1))
        } else {
            None
        }
    }

    fn midpiece(&self, x: usize, y: usize, tx: usize, ty: usize) -> Option<GamePiece> {
        match self.midpiece_coordinate(x, y, tx, ty) {
            Some(Coordinate(x, y)) => self.board[x][y],
            None => None,
        }
    }

    fn valid_jump(&self, p: &GamePiece, from: &Coordinate, to: &Coordinate) -> bool {
        if !to.on_board() || !from.on_board() {
            false
        } else {
            let Coordinate(x, y) = *from;
            let Coordinate(tx, ty) = *to;

            let midpiece = self.midpiece(x, y, tx, ty);
            match midpiece {
                Some(mp) if mp.color != p.color => true,
                _ => false,
            }
        }
    }

    fn valid_move(&self, p: &GamePiece, from: &Coordinate, to: &Coordinate) -> bool {
        if !to.on_board() || !from.on_board() {
            false
        } else {
            let Coordinate(tx, ty) = *to;
            if let Some(_piece) = self.board[tx][ty] {
                false
            } else {
                let Coordinate(_fx, fy) = *from;
                let mut valid = false;
                if ty > fy && p.color == PieceColor::White {
                    // white moves down
                    valid = true;
                }
                if ty < fy && p.color == PieceColor::Black {
                    // black moves up
                    valid = true;
                }
                if ty > fy && p.color == PieceColor::Black && p.crowned {
                    // crowned black mv down
                    valid = true;
                }
                if ty < fy && p.color == PieceColor::White && p.crowned {
                    // crowned white mv up
                    valid = true;
                }
                valid
            }
        }
    }
}

#[cfg(test)]
mod test {
    use super::super::board::{Coordinate, GamePiece, Move, PieceColor};
    use super::GameEngine;

    #[test]
    fn should_crown() {
        let engine = GameEngine::new();
        let black = GamePiece::new(PieceColor::Black);
        let res = engine.should_crown(black, Coordinate(3, 0));
        assert!(res);
        let res_nocrown = engine.should_crown(black, Coordinate(5, 2));
        assert_eq!(res_nocrown, false);
    }

    #[test]
    fn mut_crown() {
        let mut engine = GameEngine::new();
        engine.initialize_pieces();
        let crowned = engine.crown_piece(Coordinate(1, 0));
        assert!(crowned);
        assert!(engine.is_crowned(Coordinate(1, 0)));
    }

    #[test]
    fn advance_turn() {
        let mut engine = GameEngine::new();
        engine.advance_turn();
        assert_eq!(engine.current_turn(), PieceColor::White);
        engine.advance_turn();
        assert_eq!(engine.current_turn(), PieceColor::Black);
        assert_eq!(engine.move_count(), 2);
    }

    #[test]
    fn move_targets() {
        let c1 = Coordinate(0, 5);
        let targets = c1.move_targets_from().collect::<Vec<Coordinate>>();
        assert_eq!(targets, [Coordinate(1, 6), Coordinate(1, 4)]);

        let c2 = Coordinate(1, 6);
        let targets2 = c2.move_targets_from().collect::<Vec<Coordinate>>();
        assert_eq!(
            targets2,
            [
                Coordinate(0, 7),
                Coordinate(2, 7),
                Coordinate(2, 5),
                Coordinate(0, 5)
            ]
        );

        let c3 = Coordinate(2, 5);
        let targets3 = c3.move_targets_from().collect::<Vec<Coordinate>>();
        assert_eq!(
            targets3,
            [
                Coordinate(1, 6),
                Coordinate(3, 6),
                Coordinate(3, 4),
                Coordinate(1, 4)
            ]
        );
    }

    #[test]
    fn valid_from() {
        let c1 = Coordinate(0, 5);
        let c2 = Coordinate(2, 5);

        let mut engine = GameEngine::new();
        engine.initialize_pieces();
        let m1 = engine.valid_moves_from(c1);
        let m2 = engine.valid_moves_from(c2);
        assert_eq!(
            m1,
            [Move {
                from: Coordinate(0, 5),
                to: Coordinate(1, 4),
            }]
        );
        assert_eq!(
            m2,
            [
                Move {
                    from: Coordinate(2, 5),
                    to: Coordinate(3, 4),
                },
                Move {
                    from: Coordinate(2, 5),
                    to: Coordinate(1, 4),
                }
            ]
        );
    }

    #[test]
    fn legal_moves_black() {
        let mut engine = GameEngine::new();
        engine.initialize_pieces();
        let moves = engine.legal_moves();
        assert_eq!(
            moves,
            [
                Move {
                    from: Coordinate(0, 5),
                    to: Coordinate(1, 4),
                },
                Move {
                    from: Coordinate(2, 5),
                    to: Coordinate(3, 4),
                },
                Move {
                    from: Coordinate(2, 5),
                    to: Coordinate(1, 4),
                },
                Move {
                    from: Coordinate(4, 5),
                    to: Coordinate(5, 4),
                },
                Move {
                    from: Coordinate(4, 5),
                    to: Coordinate(3, 4),
                },
                Move {
                    from: Coordinate(6, 5),
                    to: Coordinate(7, 4),
                },
                Move {
                    from: Coordinate(6, 5),
                    to: Coordinate(5, 4),
                }
            ]
        );
    }

    #[test]
    fn legal_moves_white() {
        let mut engine = GameEngine::new();
        engine.initialize_pieces();
        engine.advance_turn();
        let moves = engine.legal_moves();
        assert_eq!(
            moves,
            [
                Move {
                    from: Coordinate(1, 2),
                    to: Coordinate(0, 3),
                },
                Move {
                    from: Coordinate(1, 2),
                    to: Coordinate(2, 3),
                },
                Move {
                    from: Coordinate(3, 2),
                    to: Coordinate(2, 3),
                },
                Move {
                    from: Coordinate(3, 2),
                    to: Coordinate(4, 3),
                },
                Move {
                    from: Coordinate(5, 2),
                    to: Coordinate(4, 3),
                },
                Move {
                    from: Coordinate(5, 2),
                    to: Coordinate(6, 3),
                },
                Move {
                    from: Coordinate(7, 2),
                    to: Coordinate(6, 3),
                }
            ]
        );
    }

    #[test]
    fn jump_targets() {
        let c1 = Coordinate(3, 3);
        let targets = c1.jump_targets_from().collect::<Vec<Coordinate>>();
        assert_eq!(
            targets,
            [
                Coordinate(5, 1),
                Coordinate(5, 5),
                Coordinate(1, 1),
                Coordinate(1, 5)
            ]
        );
    }

    #[test]
    fn jump_moves_validation() {
        let mut engine = GameEngine::new();
        engine.initialize_pieces();
        engine.board[1][4] = Some(GamePiece::new(PieceColor::White)); // this should be jumpable from 0,5 to 2,3
        let moves = engine.legal_moves();
        assert_eq!(
            moves,
            [
                Move {
                    from: Coordinate(0, 5),
                    to: Coordinate(2, 3),
                },
                Move {
                    from: Coordinate(2, 5),
                    to: Coordinate(0, 3)
                },
                Move {
                    from: Coordinate(2, 5),
                    to: Coordinate(3, 4)
                },
                Move {
                    from: Coordinate(4, 5),
                    to: Coordinate(5, 4)
                },
                Move {
                    from: Coordinate(4, 5),
                    to: Coordinate(3, 4)
                },
                Move {
                    from: Coordinate(6, 5),
                    to: Coordinate(7, 4)
                },
                Move {
                    from: Coordinate(6, 5),
                    to: Coordinate(5, 4)
                }
            ]
        );
    }

    #[test]
    fn test_basic_move() {
        let mut engine = GameEngine::new();
        engine.initialize_pieces();
        let res = engine.move_piece(&Move::new((0, 5), (1, 4)));
        assert!(res.is_ok());

        let old = engine.board[0][5];
        let new = engine.board[1][4];
        assert_eq!(old, None);
        assert_eq!(
            new,
            Some(GamePiece {
                color: PieceColor::Black,
                crowned: false
            })
        );

        // fail to perform illegal move
        let res = engine.move_piece(&Move::new((1, 4), (2, 4))); // can't move horiz
        assert!(!res.is_ok());
        assert_eq!(engine.board[2][4], None);
    }

}