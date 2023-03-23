#[derive(PartialEq, Debug)]
pub(crate) struct Piece {
    pub(crate) bug: Bug,
    pub(crate) color: Color,
}

#[derive(PartialEq, Debug)]
pub(crate) enum Bug {
    Bee,
    Beetle,
    Grasshopper,
    Spider,
    Ant,
}

#[derive(PartialEq, Debug)]
pub(crate) enum Color {
    Black,
    White,
}
