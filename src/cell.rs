use std::str::FromStr;

use anyhow::{anyhow, Error};

pub const WALL_DASHES: usize = 3;
pub const CELL_CHAR_WIDTH: usize = WALL_DASHES + 2;

pub const CELL_WALL_PIPES: usize = 1;
pub const CELL_LINE_HEIGHT: usize = CELL_WALL_PIPES + 2;

#[derive(Clone, Debug, Default)]
pub struct Cell {
    // Walls
    wall_top: bool,
    wall_bottom: bool,
    wall_left: bool,
    wall_right: bool,

    // Corners
    corner_top_left: bool,
    corner_top_right: bool,
    corner_bottom_left: bool,
    corner_bottom_right: bool,

    // Inner Text (parsed from chars in center of cell)
    inner_text: String,
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
