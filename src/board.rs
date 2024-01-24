use std::{
    collections::{HashMap, HashSet},
    ops::Sub,
};

use crate::coordinate::{Coordinate, CoordinateHandler};

#[derive(PartialEq, Clone)]
pub(crate) struct StackableHexagonalBoard<P> {
    cells: HashMap<Coordinate, Cell<P>>,
}

type Cell<T> = Vec<T>;

impl<P> StackableHexagonalBoard<P> {
    pub(crate) fn new() -> Self {
        StackableHexagonalBoard {
            cells: HashMap::new(),
        }
    }

    pub(crate) fn get_cell(&self, coordinate: Coordinate) -> Option<&Cell<P>> {
        self.cells.get(&coordinate)
    }

    pub(crate) fn get_top_piece(&self, coordinate: Coordinate) -> Option<&P> {
        self.get_cell(coordinate)?.last()
    }

    pub(crate) fn put_piece(&mut self, p: P, coordinate: Coordinate) {
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

    fn neighbors(&self, from: Coordinate) -> Vec<(Coordinate, &P)> {
        CoordinateHandler::neighbor_coordinates(from)
            .into_iter()
            .flat_map(|neighbor_coordinate| {
                Some((
                    neighbor_coordinate,
                    self.get_top_piece(neighbor_coordinate)?,
                ))
            })
            .collect()
    }

    pub(crate) fn neighbor_pieces(&self, coordinate: Coordinate) -> Vec<&P> {
        self.neighbors(coordinate)
            .iter()
            .map(|(_, piece)| *piece)
            .collect()
    }

    pub(crate) fn hive(&self) -> HashSet<Coordinate> {
        HashSet::from_iter(
            self.cells
                .iter()
                .flat_map(|(&c, _)| self.get_top_piece(c).map(|_| c)),
        )
    }

    pub(crate) fn hive_without(&self, coordinate: Coordinate) -> HashSet<Coordinate> {
        self.hive().sub(&[coordinate].into())
    }

    pub(crate) fn occupied_amount(&self) -> usize {
        self.cells.len()
    }

    // Returns the outline walkable cells without taking into account the top piece at the position given
    pub(crate) fn walkable_without(&self, coordinate: Coordinate) -> HashSet<Coordinate> {
        let hive_without = self.hive_without(coordinate);

        let neighbors: HashSet<Coordinate> = hive_without
            .iter()
            .flat_map(|&c| CoordinateHandler::neighbor_coordinates(c))
            .collect();

        neighbors.sub(&hive_without)
    }

    pub(crate) fn hive_and_walkable_without(&self, coordinate: Coordinate) -> HashSet<Coordinate> {
        let hive_without = self.hive_without(coordinate);

        let neighbors: HashSet<Coordinate> = hive_without
            .iter()
            .flat_map(|&c| CoordinateHandler::neighbor_coordinates(c))
            .collect();

        neighbors.union(&hive_without).copied().collect()
    }

    pub(crate) fn find<F>(&self, filter: F) -> Vec<Coordinate>
    where
        F: Fn(&P) -> bool,
    {
        self.cells
            .iter()
            .flat_map(|(&c, cell)| Some((c, cell.last()?)))
            .filter(|(_, p)| filter(p))
            .map(|(c, _)| c)
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::piece;

    #[test]
    fn simple_board() {
        use piece::Bug::*;
        use piece::Color::*;
        let mut board = StackableHexagonalBoard::new();
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

        board.put_piece(black_bee.clone(), Coordinate { x: 0, y: 0 });
        board.put_piece(white_bee.clone(), Coordinate { x: 0, y: 1 });
        board.put_piece(black_beetle.clone(), Coordinate { x: 0, y: 1 });
        board.put_piece(white_beetle.clone(), Coordinate { x: 0, y: 1 });
        board.put_piece(black_ant.clone(), Coordinate { x: 0, y: -1 });
        board
            .move_top_piece(Coordinate { x: 0, y: -1 }, Coordinate { x: 1, y: 1 })
            .unwrap();
        board
            .move_top_piece(Coordinate { x: 0, y: 1 }, Coordinate { x: 0, y: 0 })
            .unwrap();

        assert_eq!(
            board.get_cell(Coordinate { x: 0, y: 0 }),
            Some(&vec![black_bee, white_beetle])
        );
        assert_eq!(
            board.get_cell(Coordinate { x: 0, y: 1 }),
            Some(&vec![white_bee, black_beetle])
        );
        assert_eq!(
            board.get_cell(Coordinate { x: 1, y: 1 }),
            Some(&vec![black_ant])
        );
        assert_eq!(board.get_cell((10, 1).into()), None);

        assert!(board.move_top_piece((0, 50).into(), (0, 0).into()).is_err());
    }
}
