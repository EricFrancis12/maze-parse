use anyhow::Result;
use std::{fs, path::Path};

const WALL_DASHES: i32 = 3;
const CELL_CHAR_WIDTH: i32 = WALL_DASHES + 2;

#[derive(Clone, Default)]
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
    fn new_dense(height: usize, width: usize) -> Self {
        Maze {
            cells: vec![vec![Cell::default(); width]; height],
        }
    }

    fn new_from_file(path: impl AsRef<Path>) -> Result<Self> {
        let content = fs::read_to_string(path)?;

        parse_sm_maze(&content)
            .or_else(|_| parse_lg_maze(&content))
            .map_err(|_| anyhow::anyhow!("Failed to parse maze from file"))
    }
}

fn parse_sm_maze(s: impl Into<String>) -> Result<Maze> {
    todo!();
}

fn parse_lg_maze(s: impl Into<String>) -> Result<Maze> {
    todo!();
}

fn main() {
	let content = fs::read_to_string("my_maze_sm").unwrap();

    let sm_maze = parse_sm_maze(&content).unwrap();
    let lg_maze = parse_lg_maze(&content).unwrap();
}
