use std::{fs, path::Path};

const WALL_DASHES: i32 = 3;
const CELL_CHAR_WIDTH: i32 = WALL_DASHES + 2;

struct Cell {
    wall_top: bool,
    wall_bottom: bool,
    wall_left: bool,
    wall_right: bool,
}

struct Maze {
    cells: Vec<Vec<Cell>>,
}

impl Maze {
    fn new_from_file(path: impl AsRef<Path>) -> Self {
        let content = fs::read_to_string(path).unwrap();
        todo!()
    }
}

fn main() {
    let lg_maze = Maze::new_from_file("my_maze_lg");
    let sm_maze = Maze::new_from_file("my_maze_sm");
}
