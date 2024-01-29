use std::collections::HashSet;

use crate::coordinate::HexagonalCoordinateSystem;

#[derive(PartialEq, Debug, Clone)]
pub(crate) struct Piece<B> {
    pub(crate) bug: B,
    pub(crate) color: Color,
}

#[derive(PartialEq, Debug, Clone)]
pub(crate) enum Bug {
    Bee,
    Beetle,
    Grasshopper,
    Spider,
    Ant,
}

pub(crate) trait Board<Coordinate, CoordinateSystem> {
    fn walkable_without(&self, _: Coordinate) -> HashSet<Coordinate>;
    fn hive_and_walkable_without(&self, _: Coordinate) -> HashSet<Coordinate>;
    fn coordinate_system(&self) -> CoordinateSystem;
    fn hive_without(&self, _: Coordinate) -> HashSet<Coordinate>;
    fn hive(&self) -> HashSet<Coordinate>;
}

pub(crate) trait BugTrait<Board, Coordinate, CoordinateSystem>:
    PartialEq + std::fmt::Debug + Clone
{
    fn is_bee(&self) -> bool;
    fn possible_moves(&self, _: Board, from: Coordinate) -> Result<HashSet<Coordinate>, ()>;
}

impl<B, Coordinate, CS> BugTrait<B, Coordinate, CS> for Bug
where
    B: Board<Coordinate, CS>,
    CS: HexagonalCoordinateSystem<Coordinate = Coordinate, Direction = Coordinate>,
    Coordinate: std::hash::Hash + Eq + Clone + std::ops::Add<Output = Coordinate>,
{
    fn is_bee(&self) -> bool {
        match self {
            Bug::Bee => true,
            _ => false,
        }
    }

    fn possible_moves(&self, board: B, from: Coordinate) -> Result<HashSet<Coordinate>, ()> {
        Ok(match self {
            Bug::Bee => {
                let walkable = board.walkable_without(from);

                let hive = board.hive_without(from);

                let neighbor_coordinates =
                    board.coordinate_system().neighbor_coordinates(from).into();
                let slidable_neighbors =
                    walkable.intersection(&neighbor_coordinates).filter(|&c| {
                        board
                            .coordinate_system()
                            .can_slide(from, *c, &hive)
                            .unwrap()
                        // TODO: dont unwrap
                    });

                slidable_neighbors.cloned().collect()
            }
            Bug::Beetle => board
                .hive_and_walkable_without(from)
                .intersection(&board.coordinate_system().neighbor_coordinates(from).into())
                .cloned()
                .collect(),
            Bug::Grasshopper => {
                let hive = board.hive_without(from);

                let possible_destinies = board
                    .coordinate_system()
                    .relative_neighbors_clockwise()
                    .into_iter()
                    .flat_map(|direction| {
                        let position = from + direction;

                        if !hive.contains(&position) {
                            return None;
                        }

                        let mut last = position;
                        while hive.contains(&last) {
                            last = last + direction;
                        }
                        Some(last)
                    });

                possible_destinies.collect()
            }
            Bug::Spider => {
                let walkable = board.walkable_without(from);

                let mut paths = vec![vec![from]];

                for _ in 0..3 {
                    let mut new_paths = vec![];

                    for path in paths {
                        let last = *path.last().ok_or(())?; // TODO: this should never fail

                        let neighbor_coordinates =
                            board.coordinate_system().neighbor_coordinates(last).into();
                        let walkable_neighbors = walkable.intersection(&neighbor_coordinates);
                        let slidable_neighbors = walkable_neighbors
                            .filter(|&c| !path.contains(c))
                            .filter(|&c| {
                                board
                                    .coordinate_system()
                                    .can_slide(last, *c, &walkable)
                                    .unwrap() // TODO: dont unwrap
                            });

                        for &neighbor in slidable_neighbors {
                            let mut new_path = path.clone();
                            new_path.push(neighbor);
                            new_paths.push(new_path);
                        }
                    }

                    paths = new_paths;
                }

                paths.iter().flat_map(|p| p.last()).cloned().collect()
            }
            Bug::Ant => {
                let walkable = board.walkable_without(from);

                // Traverse the tree
                let mut reachable: HashSet<Coordinate> = HashSet::new();
                let mut to_check = vec![from];

                while let Some(current) = to_check.pop() {
                    let neighbor_coordinates = board
                        .coordinate_system()
                        .neighbor_coordinates(current)
                        .into();
                    let slidable_neighbors =
                        walkable.intersection(&neighbor_coordinates).filter(|&&c| {
                            board
                                .coordinate_system()
                                .can_slide(current, c, &board.hive())
                                .unwrap() // TODO: dont unwrap
                        });

                    for &neighbor in slidable_neighbors {
                        if !reachable.contains(&neighbor) {
                            to_check.push(neighbor);
                        }
                    }

                    reachable.insert(current);
                }

                reachable
            }
        })
    }
}

impl std::fmt::Display for Bug {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Bug::Bee => "🐝",
                Bug::Beetle => "🪲",
                Bug::Grasshopper => "🦗",
                Bug::Spider => "🕷",
                Bug::Ant => "🐜",
            }
        )
    }
}

#[derive(PartialEq, Debug, Clone)]
pub(crate) enum Color {
    Black,
    White,
}

impl std::ops::Not for Color {
    type Output = Color;

    fn not(self) -> Self::Output {
        match self {
            Color::Black => Color::White,
            Color::White => Color::Black,
        }
    }
}

impl std::fmt::Display for Color {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Color::Black => "⚫",
                Color::White => "⚪",
            }
        )
    }
}

impl<B> std::fmt::Display for Piece<B>
where
    B: std::fmt::Display,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} {}", self.color, self.bug)
    }
}
