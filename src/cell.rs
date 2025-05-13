use std::str::FromStr;

use anyhow::Error;

// x (cols)
pub const WALL_DASHES: usize = 3;
pub const CELL_CHAR_WIDTH: usize = WALL_DASHES + 2;

// y (lines)
pub const CELL_WALL_PIPES: usize = 1;
pub const CELL_LINE_HEIGHT: usize = CELL_WALL_PIPES + 2;

#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub struct Cell {
    // Walls
    pub wall_top: bool,
    pub wall_bottom: bool,
    pub wall_left: bool,
    pub wall_right: bool,

    // Corners
    pub corner_top_left: bool,
    pub corner_top_right: bool,
    pub corner_bottom_left: bool,
    pub corner_bottom_right: bool,

    // Inner Text (parsed from chars in center of cell)
    pub inner_text: String,
}

impl FromStr for Cell {
    type Err = Error;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        let lines: Vec<&str> = s.lines().collect();

        let wall_top = lines[0].chars().nth(1) == Some('-');
        let wall_bottom = lines[2].chars().nth(1) == Some('-');
        let wall_left = lines[1].chars().nth(0) == Some('|');
        let wall_right = lines[1].chars().nth(4) == Some('|');

        let corner_top_left = lines[0].chars().nth(0) == Some('+');
        let corner_top_right = lines[0].chars().nth(4) == Some('+');
        let corner_bottom_left = lines[2].chars().nth(0) == Some('+');
        let corner_bottom_right = lines[2].chars().nth(4) == Some('+');

        let inner_text: String = lines[1].chars().skip(1).take(WALL_DASHES).collect();

        Ok(Cell {
            wall_top,
            wall_bottom,
            wall_left,
            wall_right,
            corner_top_left,
            corner_top_right,
            corner_bottom_left,
            corner_bottom_right,
            inner_text,
        })
    }
}
