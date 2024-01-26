#[derive(PartialEq, Debug, Clone)]
pub(crate) struct Piece<B>
where
    B: BugTrait,
{
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

pub(crate) trait BugTrait: PartialEq + std::fmt::Debug + Clone {}

impl BugTrait for Bug {}

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

impl std::fmt::Display for Piece<Bug> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} {}", self.color, self.bug)
    }
}
