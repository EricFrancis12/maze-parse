use std::str::FromStr;

use anyhow::{anyhow, Error};

pub const WALL_DASHES: usize = 3;
pub const CELL_CHAR_WIDTH: usize = WALL_DASHES + 2;

pub const CELL_WALL_PIPES: usize = 1;
pub const CELL_LINE_HEIGHT: usize = CELL_WALL_PIPES + 2;

#[derive(Clone, Debug, Default)]
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
        // Ensure the input string has the correct dimensions
        let lines: Vec<&str> = s.lines().collect();
        if lines.len() != CELL_LINE_HEIGHT {
            return Err(anyhow!(
                "Invalid cell: Expected {} lines, but got {}.",
                CELL_LINE_HEIGHT,
                lines.len()
            ));
        }

        // Parse walls and corners
        let wall_top = lines[0].chars().nth(1) == Some('-');
        let wall_bottom = lines[2].chars().nth(1) == Some('-');
        let wall_left = lines[1].chars().nth(0) == Some('|');
        let wall_right = lines[1].chars().nth(4) == Some('|');

        let corner_top_left = lines[0].chars().nth(0) == Some('+');
        let corner_top_right = lines[0].chars().nth(4) == Some('+');
        let corner_bottom_left = lines[2].chars().nth(0) == Some('+');
        let corner_bottom_right = lines[2].chars().nth(4) == Some('+');

        // Parse inner text (center of the cell)
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

impl PartialEq for Cell {
    fn eq(&self, other: &Self) -> bool {
        self.wall_top == other.wall_top
            && self.wall_bottom == other.wall_bottom
            && self.wall_left == other.wall_left
            && self.wall_right == other.wall_right
            && self.corner_top_left == other.corner_top_left
            && self.corner_top_right == other.corner_top_right
            && self.corner_bottom_left == other.corner_bottom_left
            && self.corner_bottom_right == other.corner_bottom_right
            && self.inner_text == other.inner_text
    }
}

impl Eq for Cell {}
