use std::collections::HashMap;

fn main() {
    println!("Hello, world!");
}

#[derive(Eq, Hash, PartialEq)]
struct Coordinate {
    x: i8,
    y: i8,
}

struct Board<'a> {
    cells: HashMap<Coordinate,  Cell<'a>>,
} 

#[derive(PartialEq, Debug)]
struct Piece {
    bug: Bug,
    color: Color,
}

#[derive(PartialEq, Debug)]
enum Bug {
    Bee,
    Beetle,
    Grasshopper,
    Spider,
    Ant,
}

#[derive(PartialEq, Debug)]
enum Color {
    Black,
    White,
}

type Cell<'a> = Vec<&'a Piece>;

impl<'a> Board<'a> {
    fn put_piece(&mut self, p:&'a Piece, y: i8, x: i8) {
       match self.cells.get_mut(&Coordinate{x, y}) {
        None => {
            let cell = vec![p];
            self.cells.insert(Coordinate{x, y}, cell);
        },
        Some(cell) => cell.push(p),
       }
    }

    fn get_cell(&self, y: i8, x: i8) -> Option<Cell> {
        self.cells.get(&Coordinate{x, y}).cloned()
    }
}

#[test]
fn simple_board() {
    use Color::*;
    use Bug::*;
    let mut board = Board{cells: HashMap::new()};
    let black_bee = Piece{bug: Bee, color: Black};
    let white_bee = Piece{bug: Bee, color: White};
    let black_beetle = Piece{bug: Beetle, color: Black};
    let white_beetle = Piece{bug: Beetle, color: White};

    board.put_piece(&black_bee, 0, 0);
    board.put_piece(&white_bee, 0, 1);
    board.put_piece(&black_beetle, 0, 1);
    board.put_piece(&white_beetle, 0, 1);

    assert_eq!(board.get_cell(0, 0), Some(vec![&black_bee]));
    assert_eq!(board.get_cell(0, 1), Some(vec![&white_bee, &black_beetle, &white_beetle]));
    assert_eq!(board.get_cell(10, 1), None);
}
