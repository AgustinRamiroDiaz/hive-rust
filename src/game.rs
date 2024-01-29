use std::collections::HashSet;
use std::vec;

use crate::board::StackableHexagonalBoard;
use crate::coordinate::{
    GenericCoordinateSystem, HexagonalCoordinateSystem, XYCoordinate, RELATIVE_NEIGHBORS_CLOCKWISE,
};
use crate::piece::{Board, Bug, BugTrait, Color, Piece};

#[derive(PartialEq, Clone)]
pub(crate) struct Game<B> {
    turn: Color,
    result: Option<GameResult>,
    board: StackableHexagonalBoard<
        Piece<B>,
        GenericCoordinateSystem<XYCoordinate, XYCoordinate>,
        XYCoordinate,
    >,
    turn_number: u8,
    pool: Vec<Piece<B>>,
}

#[derive(Debug, PartialEq, Clone)]
pub(crate) enum GameResult {
    Win(Color),
    Draw,
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
    GameFinished(GameResult),
    MustPlaceBeeBeforeMoving,
}

impl<'a, Bug> Game<Bug>
where
    Bug: BugTrait<
            &'a StackableHexagonalBoard<
                Piece<Bug>,
                GenericCoordinateSystem<XYCoordinate, XYCoordinate>,
                XYCoordinate,
            >,
            XYCoordinate,
            GenericCoordinateSystem<XYCoordinate, XYCoordinate>,
        > + 'a,
{
    pub(crate) fn new(pool: Vec<Piece<Bug>>) -> Self {
        Game {
            turn: Color::Black,
            result: None,
            board: StackableHexagonalBoard::new(GenericCoordinateSystem::new(
                RELATIVE_NEIGHBORS_CLOCKWISE,
            )),
            turn_number: 1,
            pool,
        }
    }
    pub(crate) fn put(
        &mut self,
        piece: Piece<Bug>,
        coordinate: XYCoordinate,
    ) -> Result<(), GameError> {
        if let Some(winner) = self.result.clone() {
            return Err(GameError::GameFinished(winner));
        }

        if piece.color != self.turn {
            return Err(GameError::NotYourTurn);
        }

        if self.board.get_top_piece(coordinate).is_some() {
            return Err(GameError::SpawnedOnTopOfAnotherPiece);
        }

        let neighbors = self.board.neighbor_pieces(coordinate);

        if self.board.occupied_amount() != 0 && neighbors.is_empty() {
            return Err(GameError::SpawnedOutOfHive);
        }

        if self.board.occupied_amount() > 1 && neighbors.iter().any(|p| p.color != piece.color) {
            return Err(GameError::SpawnedInOpponentsHive);
        }

        // TODO: test this
        let colored_queen_is_not_placed = self
            .pool
            .iter()
            .any(|p| p.bug.is_bee() && p.color == self.turn);

        let is_fourth_turn = self.turn_number == 7 || self.turn_number == 8;

        if is_fourth_turn && !piece.bug.is_bee() && colored_queen_is_not_placed {
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
        // TODO: we are supposing that there are only 2 players
        // Once we extend the game to support more players, this will have to change
        let color_enclosed = [Color::Black, Color::White].map(|color| {
            let bee = self.board.find(|p| p.color == color && p.bug.is_bee());

            let any_enclosed = bee
                .iter()
                .any(|&coordinate| self.board.neighbor_pieces(coordinate).len() == 6);

            (color, any_enclosed)
        });

        match color_enclosed {
            [(_, true), (_, true)] => self.result = Some(GameResult::Draw),
            [(_, true), (color, _)] => self.result = Some(GameResult::Win(color)),
            [(color, _), (_, true)] => self.result = Some(GameResult::Win(color)),
            _ => {}
        }

        self.turn = !self.turn.clone();
        self.turn_number += 1;
    }

    pub(crate) fn move_top(
        &'a mut self,
        from: XYCoordinate,
        to: XYCoordinate,
    ) -> Result<(), GameError> {
        if from == to {
            return Err(GameError::InvalidMove);
        }

        if let Some(winner) = &self.result {
            return Err(GameError::GameFinished(winner.clone()));
        }

        if self
            .pool
            .iter()
            .filter(|p| p.color == self.turn)
            .any(|p| p.bug.is_bee())
        {
            return Err(GameError::MustPlaceBeeBeforeMoving);
        }

        let piece = self
            .board
            .get_top_piece(from)
            .ok_or(GameError::NoPieceAtLocation)?;

        if piece.color != self.turn {
            return Err(GameError::NotYourTurn);
        }

        let Ok(true) = self.can_move(from, to) else {
            Err(GameError::InvalidMove)?
        };

        // TODO: remove repetitive errors
        self.board
            .move_top_piece(from, to)
            .or_else(|_| Err(GameError::NoPieceAtLocation))?;

        let hive = (&self.board).hive();

        // TODO: could this be done more efficiently?
        let mut reachable = HashSet::new();
        let mut to_visit = vec![to];
        while let Some(coordinate) = to_visit.pop() {
            reachable.insert(coordinate);
            for neighbor in self
                .board
                .coordinate_system
                .neighbor_coordinates(coordinate)
            {
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

    fn can_move(&'a self, from: XYCoordinate, to: XYCoordinate) -> Result<bool, ()> {
        Ok(self.possible_moves(from)?.contains(&to))
    }

    pub(crate) fn possible_moves(
        &'a self,
        from: XYCoordinate,
    ) -> Result<HashSet<XYCoordinate>, ()> {
        let piece = self.board.get_top_piece(from).ok_or(())?;

        piece.bug.possible_moves(&self.board, from)
    }

    pub(crate) fn get_pool(&self) -> &Vec<Piece<Bug>> {
        &self.pool
    }

    pub(crate) fn get_top_piece(&self, coordinate: XYCoordinate) -> Option<&Piece<Bug>> {
        self.board.get_top_piece(coordinate)
    }

    pub(crate) fn hive(&self) -> HashSet<XYCoordinate> {
        (&self.board).hive()
    }
}

impl Game<Bug> {
    pub(crate) fn default_pool() -> Vec<Piece<Bug>> {
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
}

#[cfg(test)]
mod tests {
    use crate::piece::Bug;

    use super::*;

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
            Err(GameError::GameFinished(GameResult::Win(Color::Black)))
        ); // white grasshopper cannot move to (0, 1) because the black bee is trapped
    }
}
