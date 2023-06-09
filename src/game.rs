use std::collections::{HashMap, HashSet};
use std::fmt::Error;
use std::vec;

use crate::board::{self, Board, Coordinate};
use crate::piece::{Bug, Color, Piece};

#[derive(PartialEq)]
pub(crate) struct Game {
    turn: Color,
    won: Option<Color>,
    board: Board,
    turn_number: u8,
    pool: Vec<Piece>,
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
    PieceNotInPool,
    PlayerWon(Color),
}

// TODO: handle block conditions
impl Game {
    pub(crate) fn new(pool: Vec<Piece>) -> Self {
        Game {
            turn: Color::Black,
            won: None,
            board: Board::new(),
            turn_number: 1,
            pool,
        }
    }
    pub(crate) fn put(&mut self, piece: Piece, coordinate: Coordinate) -> Result<(), GameError> {
        if let Some(winner) = self.won.clone() {
            return Err(GameError::PlayerWon(winner));
        }

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
        let colored_queen_is_not_placed = !self
            .board
            .cells
            .values()
            .flatten()
            .any(|p| p.bug == Bug::Bee && p.color == self.turn);

        let is_fourth_turn = self.turn_number == 7 || self.turn_number == 8;

        if is_fourth_turn && piece.bug != Bug::Bee && colored_queen_is_not_placed {
            return Err(GameError::QueenMustBePlacedBeforeFifthTurn);
        }

        if let Some(index) = self.pool.iter().position(|p| p == &piece) {
            self.pool.swap_remove(index);
        } else {
            return Err(GameError::PieceNotInPool);
        }

        self.board.put_piece(piece, coordinate);
        self.end_turn();
        Ok(())
    }

    fn end_turn(&mut self) {
        for color in [Color::Black, Color::White] {
            let bee = self
                .board
                .find(|p| p.color == Color::Black && p.bug == Bug::Bee);

            if let Some(&coordinate) = bee.first() {
                // TODO: what happens if there are multiple bees?
                if self.board.neighbor_pieces(coordinate).len() == 6 {
                    self.won = Some(!color);
                }
            }
        }

        self.turn = !self.turn.clone();
        self.turn_number += 1;
    }

    // TODO: move_top is not handling the bee placement condition
    pub(crate) fn move_top(&mut self, from: Coordinate, to: Coordinate) -> Result<(), GameError> {
        if from == to {
            return Err(GameError::InvalidMove);
        }

        if let Some(winner) = self.won.clone() {
            return Err(GameError::PlayerWon(winner));
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

        // TODO: could this be done more efficiently?
        let mut reachable = HashSet::new();
        let mut to_visit = vec![to];
        while let Some(coordinate) = to_visit.pop() {
            reachable.insert(coordinate);
            for neighbor in Board::neighbor_coordinates(coordinate) {
                if !reachable.contains(&neighbor) && hive.contains(&neighbor) {
                    to_visit.push(neighbor);
                }
            }
        }

        if hive != reachable {
            self.board.move_top_piece(to, from).unwrap(); // TODO: remove unwrap
            return Err(GameError::HiveDisconnected);
        }

        self.end_turn();
        Ok(())
    }

    fn can_move(&self, from: Coordinate, to: Coordinate) -> Result<bool, ()> {
        Ok(self.possible_moves(from)?.contains(&to))
    }

    pub(crate) fn possible_moves(&self, from: Coordinate) -> Result<HashSet<Coordinate>, ()> {
        let piece = self.board.get_top_piece(from).ok_or(())?;

        Ok(match piece.bug {
            Bug::Bee => {
                let walkable = self.board.walkable_without(from);

                let hive = self.board.hive_without(from);

                let neighbor_coordinates = Board::neighbor_coordinates(from).into();
                let slidable_neighbors = walkable
                    .intersection(&neighbor_coordinates)
                    .filter(|&c| Board::can_slide(from, *c, &hive));

                slidable_neighbors.cloned().collect()
            }
            Bug::Beetle => self
                .board
                .hive_and_walkable_without(from)
                .intersection(&Board::neighbor_coordinates(from).into())
                .cloned()
                .collect(),
            Bug::Grasshopper => {
                let hive = self.board.hive_without(from);

                let possible_destinies =
                    board::RELATIVE_NEIGHBORS_CLOCKWISE
                        .iter()
                        .flat_map(|&direction| {
                            let position = from + direction;

                            if !hive.contains(&position) {
                                return None;
                            }

                            let mut last = position;
                            while hive.contains(&last) {
                                last = last + direction;
                            }
                            Some(last)
                        });

                possible_destinies.collect()
            }
            Bug::Spider => {
                let walkable = self.board.walkable_without(from);

                let mut paths = vec![vec![from]];

                for _ in 0..3 {
                    let mut new_paths = vec![];

                    for path in paths {
                        let last = *path.last().ok_or(())?; // TODO: this should never fail

                        let neighbor_coordinates = Board::neighbor_coordinates(last).into();
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

                paths.iter().flat_map(|p| p.last()).cloned().collect()
            }
            Bug::Ant => {
                let walkable = self.board.walkable_without(from);

                // Traverse the tree
                let mut reachable: HashSet<Coordinate> = HashSet::new();
                let mut to_check = vec![from];

                while let Some(current) = to_check.pop() {
                    let neighbor_coordinates = Board::neighbor_coordinates(current).into();
                    let slidable_neighbors = walkable
                        .intersection(&neighbor_coordinates)
                        .filter(|&&c| Board::can_slide(current, c, &self.board.hive()));

                    for &neighbor in slidable_neighbors {
                        if !reachable.contains(&neighbor) {
                            to_check.push(neighbor);
                        }
                    }

                    reachable.insert(current);
                }

                reachable
            }
        })
    }

    pub(crate) fn get_pool(&self) -> &Vec<Piece> {
        &self.pool
    }

    pub(crate) fn default_pool() -> Vec<Piece> {
        [Color::Black, Color::White]
            .iter()
            .flat_map(|color| {
                vec![
                    (
                        1,
                        Piece {
                            bug: Bug::Bee,
                            color: color.clone(),
                        },
                    ),
                    (
                        2,
                        Piece {
                            bug: Bug::Beetle,
                            color: color.clone(),
                        },
                    ),
                    (
                        2,
                        Piece {
                            bug: Bug::Spider,
                            color: color.clone(),
                        },
                    ),
                    (
                        3,
                        Piece {
                            bug: Bug::Ant,
                            color: color.clone(),
                        },
                    ),
                    (
                        3,
                        Piece {
                            bug: Bug::Grasshopper,
                            color: color.clone(),
                        },
                    ),
                ]
            })
            .flat_map(|(count, piece)| (0..count).map(move |_| piece.clone()))
            .collect()
    }

    pub(crate) fn get_top_piece(&self, coordinate: Coordinate) -> Option<&Piece> {
        self.board.get_top_piece(coordinate)
    }

    pub(crate) fn hive(&self) -> HashSet<Coordinate> {
        self.board.hive()
    }
}

#[test]
fn simple_game() {
    let mut game = Game::new(Game::default_pool());

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

    game.put(black_bee.clone(), (0, 0).into()).unwrap(); // black bee is placed at (0, 0)

    assert_eq!(
        game.put(black_bee.clone(), (0, 0).into()),
        Err(GameError::NotYourTurn)
    ); // it's not black's turn

    game.put(white_bee.clone(), (1, 0).into()).unwrap(); // white bee is placed at (1, 0)

    assert_eq!(
        game.put(black_beetle.clone(), (2, 0).into()),
        Err(GameError::SpawnedInOpponentsHive)
    ); // black beetle cannot spawn in white's hive

    assert_eq!(
        game.put(black_beetle.clone(), (0, 0).into()),
        Err(GameError::SpawnedOnTopOfAnotherPiece)
    ); // pieces cannot spawn on top of another piece

    game.put(black_beetle.clone(), (-1, 0).into()).unwrap(); // black beetle is placed at (-1, 0)

    game.put(white_beetle.clone(), (2, 0).into()).unwrap(); // white beetle is placed at (2, 0)
    game.put(black_ant.clone(), (-1, 1).into()).unwrap(); // black ant is placed at (-1, 1)

    game.move_top((2, 0).into(), (1, 0).into()).unwrap(); // white beetle moves to (1, 0), stacking on top of the white bee

    game.move_top((-1, 1).into(), (1, 1).into()).unwrap(); // black ant moves to (1, 1)

    assert_eq!(
        game.move_top((1, 0).into(), (1, 0).into()),
        Err(GameError::InvalidMove)
    ); // cannot move to the same location

    game.move_top((1, 0).into(), (0, 0).into()).unwrap(); // white beetle moves to (0, 0), stacking on top of the black beetle

    assert_eq!(
        game.move_top((-10, -10).into(), (5, 0).into()),
        Err(GameError::NoPieceAtLocation)
    );
}

#[test]
fn bee_gets_trapped() {
    let mut game = Game::new(Game::default_pool());

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
    let white_ant = Piece {
        bug: Bug::Ant,
        color: Color::White,
    };
    let white_grasshopper = Piece {
        bug: Bug::Grasshopper,
        color: Color::White,
    };

    game.put(black_bee.clone(), (0, 0).into()).unwrap(); // black bee is placed at (0, 0)
    game.put(white_bee.clone(), (1, 0).into()).unwrap(); // white bee is placed at (1, 0)
    game.put(black_beetle.clone(), (-1, 1).into()).unwrap(); // black beetle is placed at (-1, 1)
    game.put(white_beetle.clone(), (2, 0).into()).unwrap(); // white beetle is placed at (2, 0)
    game.put(black_ant.clone(), (0, -1).into()).unwrap(); // black ant is placed at (0, -1)
    game.put(white_grasshopper.clone(), (1, 1).into()).unwrap(); // white grasshopper is placed at (1, 1)

    game.move_top((-1, 1).into(), (0, 1).into()).unwrap(); // black beetle moves to (0, 1)

    game.put(white_ant.clone(), (2, -1).into()).unwrap(); // white ant is placed at (2, -1)

    game.move_top((0, -1).into(), (1, -1).into()).unwrap(); // black ant moves to (1, -1)

    assert_eq!(
        game.move_top((1, 1).into(), (0, 1).into()),
        Err(GameError::PlayerWon(Color::Black))
    ); // white grasshopper cannot move to (0, 1) because the black bee is trapped
}
