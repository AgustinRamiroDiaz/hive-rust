use crate::piece;
use std::collections::HashMap;

#[derive(Eq, Hash, PartialEq)]
struct Coordinate {
    x: i8,
    y: i8,
}

struct Board<'a> {
    cells: HashMap<Coordinate, Cell<'a>>,
}

type Cell<'a> = Vec<&'a piece::Piece>;

impl<'a> Board<'a> {
    fn get_cell(&self, y: i8, x: i8) -> Option<Cell> {
        self.cells.get(&Coordinate { x, y }).cloned()
    }

    fn put_piece(&mut self, p: &'a piece::Piece, y: i8, x: i8) {
        match self.cells.get_mut(&Coordinate { x, y }) {
            None => {
                let cell = vec![p];
                self.cells.insert(Coordinate { x, y }, cell);
            }
            Some(cell) => cell.push(p),
        }
    }

    fn move_top_piece(&mut self, from_y: i8, from_x: i8, to_y: i8, to_x: i8) -> Result<(), ()> {
        let from_cell = self
            .cells
            .get_mut(&Coordinate {
                x: from_x,
                y: from_y,
            })
            .ok_or(())?;
        let piece = from_cell.pop().ok_or(())?;

        let to_cell = self
            .cells
            .get_mut(&Coordinate { x: to_x, y: to_y })
            .ok_or(())?;
        to_cell.push(piece);
        Ok(())
    }
}

#[test]
fn simple_board() {
    use piece::Bug::*;
    use piece::Color::*;
    let mut board = Board {
        cells: HashMap::new(),
    };
    let black_bee = piece::Piece {
        bug: Bee,
        color: Black,
    };
    let white_bee = piece::Piece {
        bug: Bee,
        color: White,
    };
    let black_beetle = piece::Piece {
        bug: Beetle,
        color: Black,
    };
    let white_beetle = piece::Piece {
        bug: Beetle,
        color: White,
    };

    board.put_piece(&black_bee, 0, 0);
    board.put_piece(&white_bee, 0, 1);
    board.put_piece(&black_beetle, 0, 1);
    board.put_piece(&white_beetle, 0, 1);

    board.move_top_piece(0, 1, 0, 0).unwrap();

    assert_eq!(board.get_cell(0, 0), Some(vec![&black_bee, &white_beetle]));
    assert_eq!(board.get_cell(0, 1), Some(vec![&white_bee, &black_beetle]));
    assert_eq!(board.get_cell(10, 1), None);

    assert_eq!(board.move_top_piece(0, 50, 0, 0), Err(()));
}
