use crate::piece;
use std::collections::HashMap;

#[derive(Eq, Hash, PartialEq)]
pub(crate) struct Coordinate {
    x: i8,
    y: i8,
}

pub(crate) struct Board<'a> {
    pub(crate) cells: HashMap<Coordinate, Cell<'a>>,
}

type Cell<'a> = Vec<&'a piece::Piece>;

impl<'a> Board<'a> {
    pub(crate) fn new() -> Self {
        Board {
            cells: HashMap::new(),
        }
    }

    pub(crate) fn get_cell(&self, x: i8, y: i8) -> Option<&Cell> {
        self.cells.get(&Coordinate { x, y })
    }

    pub(crate) fn put_piece(&mut self, p: &'a piece::Piece, x: i8, y: i8) {
        match self.cells.get_mut(&Coordinate { x, y }) {
            None => {
                let cell = vec![p];
                self.cells.insert(Coordinate { x, y }, cell);
            }
            Some(cell) => cell.push(p),
        }
    }

    pub(crate) fn move_top_piece(
        &mut self,
        from_x: i8,
        from_y: i8,
        to_x: i8,
        to_y: i8,
    ) -> Result<(), String> {
        let from_cell = self
            .cells
            .get_mut(&Coordinate {
                x: from_x,
                y: from_y,
            })
            .ok_or("Could not get cell 'from'")?;
        let piece = from_cell.pop().ok_or("'from' cell is empty")?;

        self.put_piece(piece, to_x, to_y);
        Ok(())
    }

    // TODO: can we make this return a reference to the piece?
    pub(crate) fn neighbors(&self, x: i8, y: i8) -> Vec<&&piece::Piece> {
        let relative_positions = [(1, 0), (-1, 0), (-1, 1), (0, 1), (-1, -1), (0, -1)];

        relative_positions
            .into_iter()
            .filter_map(move |(dx, dy)| {
                let x = x + dx;
                let y = y + dy;
                self.get_cell(x, y)
            })
            .filter_map(|cell| cell.last())
            .collect()
    }
}

#[test]
fn simple_board() {
    use piece::Bug::*;
    use piece::Color::*;
    let mut board = Board::new();
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
    let black_ant = piece::Piece {
        bug: Ant,
        color: Black,
    };

    board.put_piece(&black_bee, 0, 0);
    board.put_piece(&white_bee, 0, 1);
    board.put_piece(&black_beetle, 0, 1);
    board.put_piece(&white_beetle, 0, 1);
    board.put_piece(&black_ant, 0, -1);
    board.move_top_piece(0, -1, 1, 1).unwrap();
    board.move_top_piece(0, 1, 0, 0).unwrap();

    assert_eq!(board.get_cell(0, 0), Some(&vec![&black_bee, &white_beetle]));
    assert_eq!(board.get_cell(0, 1), Some(&vec![&white_bee, &black_beetle]));
    assert_eq!(board.get_cell(1, 1), Some(&vec![&black_ant]));
    assert_eq!(board.get_cell(10, 1), None);

    assert!(board.move_top_piece(0, 50, 0, 0).is_err());
}
