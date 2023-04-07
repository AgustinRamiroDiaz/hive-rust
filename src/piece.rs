#[derive(PartialEq, Debug, Clone)]
pub(crate) struct Piece {
    pub(crate) bug: Bug,
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
