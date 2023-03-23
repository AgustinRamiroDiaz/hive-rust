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
fn simple_game() {
    let mut board = Board{cells: HashMap::new()};
    let black_bee = Piece{};
    let white_bee = Piece{};
    board.put_piece(&black_bee, 0, 0);
    board.put_piece(&white_bee, 0, 1);

    assert_eq!(board.get_cell(0, 0), Some(vec![&black_bee]));
    assert_eq!(board.get_cell(0, 1), Some(vec![&white_bee]));
    assert_eq!(board.get_cell(10, 1), None);
}
