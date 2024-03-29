use std::{
    collections::{HashMap, HashSet},
    ops::Sub,
};

use crate::coordinate::HexagonalCoordinateSystem;

#[derive(PartialEq, Clone)]
pub(crate) struct StackableHexagonalBoard<P, CS, C>
where
    CS: HexagonalCoordinateSystem<Coordinate = C>,
    C: std::hash::Hash + std::cmp::Eq,
{
    cells: HashMap<C, Cell<P>>,
    pub(crate) coordinate_system: CS,
}

struct PieceGuard<'a, P, CS, C>
where
    CS: HexagonalCoordinateSystem<Coordinate = C>,
    C: PartialEq + std::hash::Hash + std::cmp::Eq + Clone + Copy,
    P: Clone,
{
    board: &'a mut StackableHexagonalBoard<P, CS, C>,
    piece: P,
    coordinate: C,
}

impl<'a, P, CS, C> Drop for PieceGuard<'a, P, CS, C>
where
    CS: HexagonalCoordinateSystem<Coordinate = C>,
    C: PartialEq + std::hash::Hash + std::cmp::Eq + Clone + Copy,
    P: Clone, // TODO: is it possible to remove this? we want to move the ownership back to the board
{
    fn drop(&mut self) {
        self.board.put_piece(self.piece.clone(), self.coordinate);
    }
}

type Cell<T> = Vec<T>;

impl<P, CS, C> StackableHexagonalBoard<P, CS, C>
where
    CS: HexagonalCoordinateSystem<Coordinate = C>,
    C: PartialEq + std::hash::Hash + std::cmp::Eq + Clone + Copy,
    P: Clone,
{
    pub(crate) fn new(cs: CS) -> Self {
        StackableHexagonalBoard {
            cells: HashMap::new(),
            coordinate_system: cs,
        }
    }

    // examine takes a piece from the board and returns a guard that will put the piece back on drop
    fn examine(&mut self, coordinate: C) -> Option<PieceGuard<P, CS, C>> {
        let from_cell = self.cells.get_mut(&coordinate)?;
        let piece = from_cell.pop()?;

        Some(PieceGuard {
            board: self,
            piece,
            coordinate,
        })
    }

    pub(crate) fn get_cell(&self, coordinate: C) -> Option<&Cell<P>> {
        self.cells.get(&coordinate)
    }

    pub(crate) fn get_top_piece(&self, coordinate: C) -> Option<&P> {
        self.get_cell(coordinate)?.last()
    }

    pub(crate) fn put_piece(&mut self, p: P, coordinate: C) {
        match self.cells.get_mut(&coordinate) {
            None => {
                let cell = vec![p];
                self.cells.insert(coordinate, cell);
            }
            Some(cell) => cell.push(p),
        }
    }

    pub(crate) fn move_top_piece(&mut self, from: C, to: C) -> Result<(), String> {
        let from_cell = self
            .cells
            .get_mut(&from)
            .ok_or("Could not get cell 'from'")?;
        let piece = from_cell.pop().ok_or("'from' cell is empty")?;

        self.put_piece(piece, to);
        Ok(())
    }

    fn neighbors(&self, from: C) -> Vec<(C, &P)> {
        self.coordinate_system
            .neighbor_coordinates(from)
            .into_iter()
            .flat_map(|neighbor_coordinate| {
                Some((
                    neighbor_coordinate,
                    self.get_top_piece(neighbor_coordinate)?,
                ))
            })
            .collect()
    }

    pub(crate) fn neighbor_pieces(&self, coordinate: C) -> Vec<&P> {
        self.neighbors(coordinate)
            .iter()
            .map(|(_, piece)| *piece)
            .collect()
    }

    pub(crate) fn hive(&self) -> HashSet<C> {
        HashSet::from_iter(
            self.cells
                .iter()
                .flat_map(|(&c, _)| self.get_top_piece(c).map(|_| c)),
        )
    }

    pub(crate) fn hive_without(&mut self, coordinate: C) -> HashSet<C> {
        let piece_guard = self.examine(coordinate).unwrap();
        piece_guard.board.hive()
    }

    pub(crate) fn occupied_amount(&self) -> usize {
        self.cells.len()
    }

    // Returns the outline walkable cells without taking into account the top piece at the position given
    pub(crate) fn walkable_without(&mut self, coordinate: C) -> HashSet<C> {
        let hive_without = self.hive_without(coordinate);

        let neighbors: HashSet<C> = hive_without
            .iter()
            .flat_map(|&c| self.coordinate_system.neighbor_coordinates(c))
            .collect();

        neighbors.sub(&hive_without)
    }

    pub(crate) fn hive_and_walkable_without(&mut self, coordinate: C) -> HashSet<C> {
        let hive_without = self.hive_without(coordinate);

        let neighbors: HashSet<C> = hive_without
            .iter()
            .flat_map(|&c| self.coordinate_system.neighbor_coordinates(c))
            .collect();

        neighbors.union(&hive_without).copied().collect()
    }

    pub(crate) fn find<F>(&self, filter: F) -> Vec<C>
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
    use crate::{
        coordinate::{GenericCoordinateSystem, XYCoordinate, RELATIVE_NEIGHBORS_CLOCKWISE},
        piece,
    };

    #[test]
    fn simple_board() {
        use piece::Bug::*;
        use piece::Color::*;
        let mut board = StackableHexagonalBoard::new(GenericCoordinateSystem::new(
            RELATIVE_NEIGHBORS_CLOCKWISE,
        ));
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

        board.put_piece(black_bee.clone(), XYCoordinate { x: 0, y: 0 });
        board.put_piece(white_bee.clone(), XYCoordinate { x: 0, y: 1 });
        board.put_piece(black_beetle.clone(), XYCoordinate { x: 0, y: 1 });
        board.put_piece(white_beetle.clone(), XYCoordinate { x: 0, y: 1 });
        board.put_piece(black_ant.clone(), XYCoordinate { x: 0, y: -1 });
        board
            .move_top_piece(XYCoordinate { x: 0, y: -1 }, XYCoordinate { x: 1, y: 1 })
            .unwrap();
        board
            .move_top_piece(XYCoordinate { x: 0, y: 1 }, XYCoordinate { x: 0, y: 0 })
            .unwrap();

        assert_eq!(
            board.get_cell(XYCoordinate { x: 0, y: 0 }),
            Some(&vec![black_bee, white_beetle])
        );
        assert_eq!(
            board.get_cell(XYCoordinate { x: 0, y: 1 }),
            Some(&vec![white_bee, black_beetle])
        );
        assert_eq!(
            board.get_cell(XYCoordinate { x: 1, y: 1 }),
            Some(&vec![black_ant])
        );
        assert_eq!(board.get_cell((10, 1).into()), None);

        assert!(board.move_top_piece((0, 50).into(), (0, 0).into()).is_err());
    }
}
