use std::collections::HashSet;

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

pub(crate) struct CoordinateHandler {}

impl CoordinateHandler {
    pub(crate) fn neighbor_coordinates(from: Coordinate) -> [Coordinate; 6] {
        RELATIVE_NEIGHBORS_CLOCKWISE.map(|delta| from + delta)
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
            RELATIVE_NEIGHBORS_CLOCKWISE[(relative_neighbors_position + 5) % 6];

        let right_neighbor = hive.get(&(from + relative_right_neighbor));

        let left_neighbor = hive.get(&(from + relative_left_neighbor));

        return left_neighbor.is_none() || right_neighbor.is_none();
    }
}
