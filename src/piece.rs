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

impl std::fmt::Display for Piece {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} {}", self.color, self.bug)
    }
}

pub(crate) trait PieceTrait {
    fn color(&self) -> &Color;
    fn bug(&self) -> &Bug;
}

impl PieceTrait for Piece {
    fn color(&self) -> &Color {
        &self.color
    }

    fn bug(&self) -> &Bug {
        &self.bug
    }
}
