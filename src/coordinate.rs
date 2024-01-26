use std::{collections::HashSet, marker::PhantomData};

// Useful guide for understanding hexagonal coordinates: https://www.redblobgames.com/grids/hexagons/#neighbors-axial

// [(-1, 0), (-1, 1), (0, 1), (1, 0), (1, -1), (0, -1)]
// starts from the left and goes clockwise
const RELATIVE_NEIGHBORS_CLOCKWISE: [Coordinate; 6] = [
    Coordinate { x: -1, y: 0 },
    Coordinate { x: -1, y: 1 },
    Coordinate { x: 0, y: 1 },
    Coordinate { x: 1, y: 0 },
    Coordinate { x: 1, y: -1 },
    Coordinate { x: 0, y: -1 },
];

#[derive(Eq, Hash, PartialEq, Clone, Copy)]
pub(crate) struct Coordinate {
    pub(crate) x: i8,
    pub(crate) y: i8,
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

pub(crate) trait NewHexagonalCoordinateSystem {
    type Coord;
    type Direction;

    fn neighbor_coordinates(&self, from: Self::Coord) -> [Self::Coord; 6];

    // Determine if a piece can slide from one cell to another
    // Behavior is undefined if the cells are not neighbors
    fn can_slide(&self, from: Self::Coord, to: Self::Coord, hive: &HashSet<Self::Coord>) -> bool;

    fn relative_neighbors_clockwise(&self) -> [Self::Direction; 6];
}

struct GenericCoordinateSystem<Coord, Direction> {
    neighbors: [Direction; 6],
    _phantom: PhantomData<Coord>,
}

impl<Coord, Direction> NewHexagonalCoordinateSystem for GenericCoordinateSystem<Coord, Direction>
where
    Coord: std::ops::Add
        + std::ops::Add<Direction, Output = Coord>
        + PartialEq
        + Eq
        + std::hash::Hash
        + std::ops::Sub<Output = Coord>
        + Copy
        + std::convert::TryInto<Direction>,
    <Coord as TryInto<Direction>>::Error: std::fmt::Debug,
    Direction: Copy + PartialEq,
{
    type Coord = Coord;
    type Direction = Direction;
    fn neighbor_coordinates(&self, from: Self::Coord) -> [Self::Coord; 6] {
        self.neighbors.map(|delta| from + delta)
    }

    fn can_slide(
        &self,
        from: Self::Coord,
        to: Self::Coord,
        occupied: &HashSet<Self::Coord>,
    ) -> bool {
        // TODO: remove unwrap
        let relative_position = (to - from).try_into().unwrap();

        // TODO: remove unwrap
        let relative_neighbors_position = self
            .neighbors
            .iter()
            .position(|&p| p == relative_position)
            .unwrap();

        let relative_right_neighbor = self.neighbors[(relative_neighbors_position + 1) % 6];
        let relative_left_neighbor = self.neighbors[(relative_neighbors_position + 5) % 6];

        let right_neighbor = occupied.get(&(from + relative_right_neighbor));

        let left_neighbor = occupied.get(&(from + relative_left_neighbor));

        return left_neighbor.is_none() || right_neighbor.is_none();
    }

    fn relative_neighbors_clockwise(&self) -> [Self::Direction; 6] {
        self.neighbors
    }
}

pub(crate) trait HexagonalCoordinateSystem {
    type Coord;

    fn neighbor_coordinates(from: Self::Coord) -> [Self::Coord; 6];

    // Determine if a piece can slide from one cell to another
    // Behavior is undefined if the cells are not neighbors
    fn can_slide(from: Self::Coord, to: Self::Coord, hive: &HashSet<Self::Coord>) -> bool;

    fn relative_neighbors_clockwise() -> [Self::Coord; 6];
}

// 2 axis aligned with the hive, flat top
// https://www.redblobgames.com/grids/hexagons/#neighbors-axial
//
//                 / y axis
//                /
//    -1,1      0,1      1,1
//              /
//             /
//-1,0      0,0      1,0   ------> x axis
//
//
//    0,-1     1,-1     2,-1
//
#[derive(PartialEq, Clone)]
pub(crate) struct AxialCoordinateSystem {}

impl HexagonalCoordinateSystem for AxialCoordinateSystem {
    type Coord = Coordinate;

    fn neighbor_coordinates(from: Self::Coord) -> [Self::Coord; 6] {
        RELATIVE_NEIGHBORS_CLOCKWISE.map(|delta| from + delta)
    }

    fn can_slide(from: Self::Coord, to: Self::Coord, occupied: &HashSet<Self::Coord>) -> bool {
        let relative_position = to - from;

        // TODO: remove unwrap
        let relative_neighbors_position = RELATIVE_NEIGHBORS_CLOCKWISE
            .iter()
            .position(|&p| p == relative_position)
            .unwrap();

        let relative_right_neighbor =
            RELATIVE_NEIGHBORS_CLOCKWISE[(relative_neighbors_position + 1) % 6];
        let relative_left_neighbor =
            RELATIVE_NEIGHBORS_CLOCKWISE[(relative_neighbors_position + 5) % 6];

        let right_neighbor = occupied.get(&(from + relative_right_neighbor));

        let left_neighbor = occupied.get(&(from + relative_left_neighbor));

        return left_neighbor.is_none() || right_neighbor.is_none();
    }

    fn relative_neighbors_clockwise() -> [Self::Coord; 6] {
        RELATIVE_NEIGHBORS_CLOCKWISE
    }
}
