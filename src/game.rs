use std::collections::{HashMap, HashSet};
use std::fmt::Error;
use std::vec;

use crate::board::{self, Board, Coordinate};
use crate::piece::{Bug, Color, Piece};

pub(crate) struct Game<'a> {
    turn: Color,
    board: Board<'a>,
    turn_number: u8,
}

#[derive(Debug, PartialEq)]
pub(crate) enum GameError {
    NotYourTurn,
    NoPieceAtLocation,
    InvalidMove,
    QueenMustBePlacedBeforeFifthTurn,
    SpawnedInOpponentsHive,
    SpawnedOnTopOfAnotherPiece,
    SpawnedOutOfHive,
    HiveDisconnected,
}

// TODO: handle win and block conditions
// TODO: handle piece pool
impl<'a> Game<'a> {
    pub(crate) fn new() -> Self {
        Game {
            turn: Color::Black,
            board: Board::new(),
            turn_number: 1,
        }
    }
    pub(crate) fn put(
        &mut self,
        piece: &'a Piece,
        coordinate: Coordinate,
    ) -> Result<(), GameError> {
        if piece.color != self.turn {
            return Err(GameError::NotYourTurn);
        }

        if self.board.get_top_piece(coordinate).is_some() {
            return Err(GameError::SpawnedOnTopOfAnotherPiece);
        }

        let neighbors = self.board.neighbor_pieces(coordinate);

        if self.board.cells.values().len() != 0 && neighbors.is_empty() {
            return Err(GameError::SpawnedOutOfHive);
        }

        if self.board.cells.values().len() > 1 && neighbors.iter().any(|p| p.color != piece.color) {
            return Err(GameError::SpawnedInOpponentsHive);
        }

        // TODO: test this
        // TODO: remove the knowledge of the internal board cells
        let colored_queen_is_not_placed = self
            .board
            .cells
            .values()
            .flatten()
            .any(|p| p.bug != Bug::Bee && p.color == self.turn);

        let is_fourth_turn = self.turn_number == 7 || self.turn_number == 8;

        if is_fourth_turn && piece.bug != Bug::Bee && colored_queen_is_not_placed {
            return Err(GameError::QueenMustBePlacedBeforeFifthTurn);
        }

        self.board.put_piece(piece, coordinate);
        self.end_turn();
        Ok(())
    }

    fn end_turn(&mut self) {
        self.turn = !self.turn.clone();
        self.turn_number += 1;
    }

    pub(crate) fn move_top(&mut self, from: Coordinate, to: Coordinate) -> Result<(), GameError> {
        if self.board.cells.values().len() == 0 {
            todo!()
        }

        let piece = self
            .board
            .get_top_piece(from)
            .ok_or(GameError::NoPieceAtLocation)?;

        if piece.color != self.turn {
            return Err(GameError::NotYourTurn);
        }

        if !self
            .can_move(from, to)
            .or_else(|_| Err(GameError::InvalidMove))?
        {
            return Err(GameError::InvalidMove);
        }

        // TODO: remove repetitive errors
        self.board
            .move_top_piece(from, to)
            .or_else(|_| Err(GameError::NoPieceAtLocation))?;

        let hive = self.board.hive();
        if hive
            .iter()
            .any(|&c| self.board.neighbor_pieces(c).len() == 0)
        {
            self.board.move_top_piece(to, from).unwrap(); // TODO: remove unwrap
            return Err(GameError::HiveDisconnected);
        }

        self.end_turn();
        Ok(())
    }

    fn can_move(&self, from: Coordinate, to: Coordinate) -> Result<bool, ()> {
        let piece = self.board.get_top_piece(from).ok_or(())?;

        Ok(match piece.bug {
            Bug::Bee => {
                let walkable = self.board.walkable_without(from);

                let hive = self.board.hive_without(from);

                let neighbor_coordinates = Board::neighbor_coordinates(from);
                let slidable_neighbors = walkable
                    .intersection(&neighbor_coordinates)
                    .filter(|&c| Board::can_slide(from, *c, &hive));

                slidable_neighbors.into_iter().any(|&c| c == to)
            }
            Bug::Beetle => self.board.hive_and_walkable_without(from).contains(&to),
            Bug::Grasshopper => {
                let walkable = self.board.walkable_without(from);
                let hive = self.board.hive_without(from);

                let possible_destinies =
                    board::RELATIVE_NEIGHBORS_CLOCKWISE
                        .iter()
                        .flat_map(|&direction| {
                            let position = from + direction;

                            if !walkable.contains(&position) {
                                return None;
                            }

                            let mut last = position;
                            while hive.contains(&last) {
                                last = last + direction;
                            }
                            Some(last)
                        });

                possible_destinies.into_iter().any(|c| c == to)
            }
            Bug::Spider => {
                let walkable = self.board.walkable_without(from);

                let mut paths = vec![vec![from]];

                for _ in 0..3 {
                    let mut new_paths = vec![];

                    for path in paths {
                        let last = *path.last().ok_or(())?; // TODO: this should never fail

                        let neighbor_coordinates = Board::neighbor_coordinates(last);
                        let walkable_neighbors = walkable.intersection(&neighbor_coordinates);
                        let slidable_neighbors = walkable_neighbors
                            .filter(|&c| !path.contains(c))
                            .filter(|&c| Board::can_slide(last, *c, &walkable));

                        for &neighbor in slidable_neighbors {
                            let mut new_path = path.clone();
                            new_path.push(neighbor);
                            new_paths.push(new_path);
                        }
                    }

                    paths = new_paths;
                }

                paths.iter().flat_map(|p| p.last()).any(|&c| c == to)
            }
            Bug::Ant => {
                let walkable = self.board.walkable_without(from);

                // Traverse the tree
                let mut checked: HashSet<Coordinate> = HashSet::new();
                let mut to_check = vec![from];

                while let Some(current) = to_check.pop() {
                    let neighbor_coordinates = Board::neighbor_coordinates(current);
                    let slidable_neighbors = walkable
                        .intersection(&neighbor_coordinates)
                        .filter(|&c| Board::can_slide(current, *c, &walkable));

                    for &neighbor in slidable_neighbors {
                        if neighbor == to {
                            return Ok(true);
                        }

                        if !checked.contains(&neighbor) {
                            to_check.push(neighbor);
                        }
                    }

                    checked.insert(current);
                }
                false
            }
        })
    }
}

#[test]
fn simple_game() {
    let mut game = Game::new();

    let black_bee = Piece {
        bug: Bug::Bee,
        color: Color::Black,
    };
    let white_bee = Piece {
        bug: Bug::Bee,
        color: Color::White,
    };
    let black_beetle = Piece {
        bug: Bug::Beetle,
        color: Color::Black,
    };
    let white_beetle = Piece {
        bug: Bug::Beetle,
        color: Color::White,
    };
    let black_ant = Piece {
        bug: Bug::Ant,
        color: Color::Black,
    };
    let white_grasshopper = Piece {
        bug: Bug::Grasshopper,
        color: Color::White,
    };

    game.put(&black_bee, (0, 0).into()).unwrap(); // black bee is placed at (0, 0)

    assert_eq!(
        game.put(&black_bee, (0, 0).into()),
        Err(GameError::NotYourTurn)
    ); // it's not black's turn

    game.put(&white_bee, (1, 0).into()).unwrap(); // white bee is placed at (1, 0)

    assert_eq!(
        game.put(&black_beetle, (2, 0).into()),
        Err(GameError::SpawnedInOpponentsHive)
    ); // black beetle cannot spawn in white's hive

    assert_eq!(
        game.put(&black_beetle, (0, 0).into()),
        Err(GameError::SpawnedOnTopOfAnotherPiece)
    ); // pieces cannot spawn on top of another piece

    game.put(&black_beetle, (-1, 0).into()).unwrap(); // black beetle is placed at (-1, 0)

    game.put(&white_beetle, (2, 0).into()).unwrap(); // white beetle is placed at (2, 0)
    game.put(&black_ant, (-1, 1).into()).unwrap(); // black ant is placed at (-1, 1)

    game.move_top((2, 0).into(), (1, 0).into()).unwrap(); // white beetle moves to (1, 0), stacking on top of the white bee

    game.move_top((-1, 1).into(), (1, 1).into()).unwrap(); // black ant moves to (1, 1)

    assert_eq!(
        game.move_top((1, 1).into(), (1, 1).into()),
        Err(GameError::InvalidMove)
    ); // cannot move to the same location

    game.move_top((2, 0).into(), (1, 0).into()).unwrap(); // black beetle moves to (0, 0), stacking on top of the black beetle

    game.move_top((1, 0).into(), (0, 0).into()).unwrap(); // white beetle moves to (0, 0), stacking on top of the black beetle

    assert_eq!(
        game.move_top((-10, -10).into(), (5, 0).into()),
        Err(GameError::NoPieceAtLocation)
    );

    game.put(&white_grasshopper, (-1, 0).into()).unwrap(); // white grasshopper is placed at (-1, 0)
}
