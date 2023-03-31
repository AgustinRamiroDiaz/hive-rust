use crate::piece;
use std::{
    collections::{HashMap, HashSet},
    ops::Sub,
};

// [(-1, 0), (-1, 1), (0, 1), (1, 0), (1, -1), (0, -1)]
pub(crate) const RELATIVE_NEIGHBORS_CLOCKWISE: [Coordinate; 6] = [
    Coordinate { x: -1, y: 0 },
    Coordinate { x: -1, y: 1 },
    Coordinate { x: 0, y: 1 },
    Coordinate { x: 1, y: 0 },
    Coordinate { x: 1, y: -1 },
    Coordinate { x: 0, y: -1 },
];

#[derive(Eq, Hash, PartialEq, Clone, Copy)]
pub(crate) struct Coordinate {
    x: i8,
    y: i8,
}

impl From<(i8, i8)> for Coordinate {
    fn from((x, y): (i8, i8)) -> Self {
        Self { x, y }
    }
}

impl std::ops::Add for Coordinate {
    type Output = Self;

    fn add(self, other: Self) -> Self::Output {
        Self {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}

impl std::ops::Sub for Coordinate {
    type Output = Self;

    fn sub(self, other: Self) -> Self::Output {
        Self {
            x: self.x - other.x,
            y: self.y - other.y,
        }
    }
}

pub(crate) struct Board<'a> {
    // TODO: Can we make cells private? could it be an implementation detail?
    pub(crate) cells: HashMap<Coordinate, Cell<'a>>,
}

type Cell<'a> = Vec<&'a piece::Piece>;

impl<'a> Board<'a> {
    pub(crate) fn new() -> Self {
        let x = (1, 2);
        Board {
            cells: HashMap::new(),
        }
    }

    pub(crate) fn get_cell(&self, coordinate: Coordinate) -> Option<&Cell> {
        self.cells.get(&coordinate)
    }

    pub(crate) fn get_top_piece(&self, coordinate: Coordinate) -> Option<&piece::Piece> {
        self.get_cell(coordinate)?.last().copied()
    }

    pub(crate) fn put_piece(&mut self, p: &'a piece::Piece, coordinate: Coordinate) {
        match self.cells.get_mut(&coordinate) {
            None => {
                let cell = vec![p];
                self.cells.insert(coordinate, cell);
            }
            Some(cell) => cell.push(p),
        }
    }

    pub(crate) fn move_top_piece(
        &mut self,
        from: Coordinate,
        to: Coordinate,
    ) -> Result<(), String> {
        let from_cell = self
            .cells
            .get_mut(&from)
            .ok_or("Could not get cell 'from'")?;
        let piece = from_cell.pop().ok_or("'from' cell is empty")?;

        self.put_piece(piece, to);
        Ok(())
    }

    fn neighbors(&self, coordinate: Coordinate) -> Vec<(Coordinate, &piece::Piece)> {
        Self::neighbor_coordinates(coordinate)
            .into_iter()
            .flat_map(|c| Some((c, self.get_top_piece(coordinate)?)))
            .collect()
    }

    pub(crate) fn neighbor_pieces(&self, coordinate: Coordinate) -> Vec<&piece::Piece> {
        self.neighbors(coordinate)
            .iter()
            .map(|(_, piece)| *piece)
            .collect()
    }

    pub(crate) fn neighbor_coordinates(from: Coordinate) -> HashSet<Coordinate> {
        RELATIVE_NEIGHBORS_CLOCKWISE
            .into_iter()
            .map(move |delta| from + delta)
            .collect()
    }

    pub(crate) fn is_neighboor(a: Coordinate, b: Coordinate) -> bool {
        RELATIVE_NEIGHBORS_CLOCKWISE.contains(&(a - b))
    }

    pub(crate) fn hive_without(&self, coordinate: Coordinate) -> HashSet<Coordinate> {
        HashSet::from_iter(self.cells.iter().flat_map(|(&c, _)| {
            if c == coordinate || self.get_top_piece(coordinate).is_none() {
                None
            } else {
                Some(c)
            }
        }))
    }

    // Returns the outline walkable cells without taking into account the top piece at the position given
    pub(crate) fn walkable_without(&self, coordinate: Coordinate) -> HashSet<Coordinate> {
        let hive_without = self.hive_without(coordinate);

        let neighbors: HashSet<Coordinate> = hive_without
            .iter()
            .flat_map(|&c| Self::neighbor_coordinates(c))
            .collect();

        neighbors.sub(&hive_without)
    }

    pub(crate) fn hive_and_walkable_without(&self, coordinate: Coordinate) -> HashSet<Coordinate> {
        let hive_without = self.hive_without(coordinate);

        let neighbors: HashSet<Coordinate> = hive_without
            .iter()
            .flat_map(|&c| Self::neighbor_coordinates(c))
            .collect();

        neighbors.union(&hive_without).copied().collect()
    }

    pub(crate) fn can_slide(from: Coordinate, to: Coordinate, hive: &HashSet<Coordinate>) -> bool {
        let relative_position = to - from;

        // TODO: remove unwrap
        let relative_neighbors_position = RELATIVE_NEIGHBORS_CLOCKWISE
            .iter()
            .position(|&p| p == relative_position)
            .unwrap();

        let relative_right_neighbor =
            RELATIVE_NEIGHBORS_CLOCKWISE[(relative_neighbors_position + 1) % 6];
        let relative_left_neighbor =
            RELATIVE_NEIGHBORS_CLOCKWISE[(relative_neighbors_position - 1) % 6];

        let right_neighbor = hive.get(&(from + relative_right_neighbor));

        let left_neighbor = hive.get(&(from + relative_left_neighbor));

        return left_neighbor.is_some() && right_neighbor.is_some();
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

    board.put_piece(&black_bee, Coordinate { x: 0, y: 0 });
    board.put_piece(&white_bee, Coordinate { x: 0, y: 1 });
    board.put_piece(&black_beetle, Coordinate { x: 0, y: 1 });
    board.put_piece(&white_beetle, Coordinate { x: 0, y: 1 });
    board.put_piece(&black_ant, Coordinate { x: 0, y: -1 });
    board
        .move_top_piece(Coordinate { x: 0, y: -1 }, Coordinate { x: 1, y: 1 })
        .unwrap();
    board
        .move_top_piece(Coordinate { x: 0, y: 1 }, Coordinate { x: 0, y: 0 })
        .unwrap();

    assert_eq!(
        board.get_cell(Coordinate { x: 0, y: 0 }),
        Some(&vec![&black_bee, &white_beetle])
    );
    assert_eq!(
        board.get_cell(Coordinate { x: 0, y: 1 }),
        Some(&vec![&white_bee, &black_beetle])
    );
    assert_eq!(
        board.get_cell(Coordinate { x: 1, y: 1 }),
        Some(&vec![&black_ant])
    );
    assert_eq!(board.get_cell((10, 1).into()), None);

    assert!(board.move_top_piece((0, 50).into(), (0, 0).into()).is_err());
}
