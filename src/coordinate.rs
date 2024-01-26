use std::{collections::HashSet, error::Error, marker::PhantomData};

// Useful guide for understanding hexagonal coordinates: https://www.redblobgames.com/grids/hexagons/#neighbors-axial

// [(-1, 0), (-1, 1), (0, 1), (1, 0), (1, -1), (0, -1)]
// starts from the left and goes clockwise
pub(crate) const RELATIVE_NEIGHBORS_CLOCKWISE: [XYCoordinate; 6] = [
    XYCoordinate { x: -1, y: 0 },
    XYCoordinate { x: -1, y: 1 },
    XYCoordinate { x: 0, y: 1 },
    XYCoordinate { x: 1, y: 0 },
    XYCoordinate { x: 1, y: -1 },
    XYCoordinate { x: 0, y: -1 },
];

#[derive(Eq, Hash, PartialEq, Clone, Copy)]
pub(crate) struct XYCoordinate {
    pub(crate) x: i8,
    pub(crate) y: i8,
}

impl From<(i8, i8)> for XYCoordinate {
    fn from((x, y): (i8, i8)) -> Self {
        Self { x, y }
    }
}

impl std::ops::Add for XYCoordinate {
    type Output = Self;

    fn add(self, other: Self) -> Self::Output {
        Self {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}

impl std::ops::Sub for XYCoordinate {
    type Output = Self;

    fn sub(self, other: Self) -> Self::Output {
        Self {
            x: self.x - other.x,
            y: self.y - other.y,
        }
    }
}

pub(crate) trait NewHexagonalCoordinateSystem {
    type Coordinate;
    type Direction;

    fn neighbor_coordinates(&self, from: Self::Coordinate) -> [Self::Coordinate; 6];

    // Determine if a piece can slide from one cell to another
    // Behavior is undefined if the cells are not neighbors
    fn can_slide(
        &self,
        from: Self::Coordinate,
        to: Self::Coordinate,
        hive: &HashSet<Self::Coordinate>,
    ) -> Result<bool, Box<dyn Error>>;

    fn relative_neighbors_clockwise(&self) -> [Self::Direction; 6];
}

#[derive(PartialEq, Clone)]
pub(crate) struct GenericCoordinateSystem<Coordinate, Direction> {
    neighbors: [Direction; 6],
    _phantom: PhantomData<Coordinate>,
}

impl<Coordinate, Direction> GenericCoordinateSystem<Coordinate, Direction> {
    pub(crate) fn new(neighbors: [Direction; 6]) -> Self {
        Self {
            neighbors,
            _phantom: PhantomData,
        }
    }
}

impl<Coordinate, Direction> NewHexagonalCoordinateSystem
    for GenericCoordinateSystem<Coordinate, Direction>
where
    Coordinate: std::ops::Add
        + std::ops::Add<Direction, Output = Coordinate>
        + PartialEq
        + Eq
        + std::hash::Hash
        + std::ops::Sub<Output = Coordinate>
        + Copy
        + std::convert::TryInto<Direction>,
    <Coordinate as TryInto<Direction>>::Error: Error + 'static,
    Direction: Copy + PartialEq,
{
    type Coordinate = Coordinate;
    type Direction = Direction;
    fn neighbor_coordinates(&self, from: Self::Coordinate) -> [Self::Coordinate; 6] {
        self.neighbors.map(|delta| from + delta)
    }

    fn can_slide(
        &self,
        from: Self::Coordinate,
        to: Self::Coordinate,
        occupied: &HashSet<Self::Coordinate>,
    ) -> Result<bool, Box<dyn Error>> {
        let relative_position = (to - from).try_into()?;

        let relative_neighbors_position = self
            .neighbors
            .iter()
            .position(|&p| p == relative_position)
            .ok_or("Direction not found in the possible existent directions")?; // TODO: is there a way to do this so that the type system does the check for us and we don't need to do it at runtime?

        let relative_right_neighbor = self.neighbors[(relative_neighbors_position + 1) % 6];
        let relative_left_neighbor = self.neighbors[(relative_neighbors_position + 5) % 6];

        let right_neighbor = occupied.get(&(from + relative_right_neighbor));

        let left_neighbor = occupied.get(&(from + relative_left_neighbor));

        Ok(left_neighbor.is_none() || right_neighbor.is_none())
    }

    fn relative_neighbors_clockwise(&self) -> [Self::Direction; 6] {
        self.neighbors
    }
}
