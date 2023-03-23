use crate::{piece::Bug, piece::Color, piece::Piece};

struct Game {
    turn: Color,
}

#[derive(Debug, PartialEq)]
enum GameError {
    NotYourTurn,
    NoPieceAtLocation,
    InvalidMove,
    QueenMustBePlacedBeforeFifthTurn,
    SpawnedInOpponentsHive,
    SpawnedOnTopOfAnotherPiece,
}

impl Game {
    fn put(&self, piece: &Piece, x: i8, y: i8) -> Result<(), GameError> {
        Ok(())
    }

    fn move_top(&self, from_x: i8, from_y: i8, to_x: i8, to_y: i8) -> Result<(), GameError> {
        Ok(())
    }
}

#[test]
fn simple_game() {
    let game = Game { turn: Color::Black };

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

    assert_eq!(game.move_top(0, 0, 0, 0), Err(GameError::InvalidMove));
    assert_eq!(game.move_top(0, 0, 0, 1), Err(GameError::InvalidMove));

    game.put(&white_grasshopper, 0, 0).unwrap(); // white grasshopper is placed at (0, 0)
}
