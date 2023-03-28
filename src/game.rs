use std::collections::{HashMap, HashSet};
use std::fmt::Error;

use crate::board::{Board, Coordinate};
use crate::piece::{Bug, Color, Piece};

struct Game<'a> {
    turn: Color,
    board: Board<'a>,
    turn_number: u8,
}

#[derive(Debug, PartialEq)]
enum GameError {
    NotYourTurn,
    NoPieceAtLocation,
    InvalidMove,
    QueenMustBePlacedBeforeFifthTurn,
    SpawnedInOpponentsHive,
    SpawnedOnTopOfAnotherPiece,
    SpawnedOutOfHive,
}

impl<'a> Game<'a> {
    fn new() -> Self {
        Game {
            turn: Color::Black,
            board: Board::new(),
            turn_number: 1,
        }
    }
    fn put(&mut self, piece: &'a Piece, x: i8, y: i8) -> Result<(), GameError> {
        let coordinate = Coordinate::from((x, y));
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
        self.turn = !self.turn.clone();
        self.turn_number += 1;
        Ok(())
    }

    fn move_top(&self, from_x: i8, from_y: i8, to_x: i8, to_y: i8) -> Result<(), GameError> {
        todo!()
    }

    fn can_move(&self, from: Coordinate, to: Coordinate) -> Result<bool, ()> {
        let piece = self.board.get_top_piece(from).ok_or(())?;

        match piece.bug {
            Bug::Bee => {
                let walkable = self.board.walkable_without(from);

                let neighbor_coordinates = Board::neighbor_coordinates(from);
                let walkable_neighbors = walkable.intersection(&neighbor_coordinates);

                Ok(walkable_neighbors.into_iter().any(|&c| c == to))
            }
            Bug::Beetle => {
                let hive_and_walkable = self.board.hive_and_walkable_without(from);
                Ok(hive_and_walkable.contains(&to))
            }
            Bug::Grasshopper => todo!(),
            Bug::Spider => {
                let walkable = self.board.walkable_without(from);

                let mut paths = vec![vec![from]];

                for _ in 0..3 {
                    let mut new_paths = vec![];

                    for path in paths {
                        let last = path.last().ok_or(())?;

                        let neighbor_coordinates = Board::neighbor_coordinates(*last);
                        let walkable_neighbors = walkable.intersection(&neighbor_coordinates);
                        let possible_neighbors = walkable_neighbors.filter(|&c| !path.contains(c));

                        for &neighbor in possible_neighbors {
                            let mut new_path = path.clone();
                            new_path.push(neighbor);
                            new_paths.push(new_path);
                        }
                    }

                    paths = new_paths;
                }

                Ok(paths.iter().flat_map(|p| p.last()).any(|&c| c == to))
            }
            Bug::Ant => {
                let walkable = self.board.walkable_without(from);

                // Traverse the tree
                let mut checked: HashSet<Coordinate> = HashSet::new();
                let mut to_check = vec![from];

                while let Some(current) = to_check.pop() {
                    let neighbor_coordinates = Board::neighbor_coordinates(current);
                    let possible_neighbors = walkable.intersection(&neighbor_coordinates);

                    for &neighbor in possible_neighbors {
                        if neighbor == to {
                            return Ok(true);
                        }

                        if !checked.contains(&neighbor) {
                            to_check.push(neighbor);
                        }
                    }

                    checked.insert(current);
                }
                Ok(false)
            }
        }
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

    game.put(&black_bee, 0, 0).unwrap(); // black bee is placed at (0, 0)

    assert_eq!(game.put(&black_bee, 0, 0), Err(GameError::NotYourTurn)); // it's not black's turn

    game.put(&white_bee, 1, 0).unwrap(); // white bee is placed at (1, 0)

    assert_eq!(
        game.put(&black_beetle, 2, 0),
        Err(GameError::SpawnedInOpponentsHive)
    ); // black beetle cannot spawn in white's hive

    assert_eq!(
        game.put(&black_beetle, 0, 0),
        Err(GameError::SpawnedOnTopOfAnotherPiece)
    ); // pieces cannot spawn on top of another piece

    game.put(&black_beetle, -1, 0).unwrap(); // black beetle is placed at (-1, 0)

    game.put(&white_beetle, 2, 0).unwrap(); // white beetle is placed at (2, 0)
    game.put(&black_ant, -1, 1).unwrap(); // black ant is placed at (-1, 1)
    game.move_top(-1, 1, 1, 1).unwrap(); // black ant moves to (1, 1)

    assert_eq!(game.move_top(1, 1, 1, 1), Err(GameError::InvalidMove)); // cannot move to the same location

    game.move_top(2, 0, 1, 0).unwrap(); // white beetle moves to (1, 0), stacking on top of the white bee

    game.move_top(2, 0, 1, 0).unwrap(); // black beetle moves to (0, 0), stacking on top of the black beetle

    game.move_top(1, 0, 0, 0).unwrap(); // white beetle moves to (0, 0), stacking on top of the black beetle

    assert_eq!(
        game.move_top(-100, -100, 5, 0),
        Err(GameError::NoPieceAtLocation)
    );

    game.put(&white_grasshopper, -1, 0).unwrap(); // white grasshopper is placed at (-1, 0)
}
