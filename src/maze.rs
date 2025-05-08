use std::{fs, path::Path, str::FromStr};

use anyhow::{anyhow, Error, Result};

use crate::{
    cell::{Cell, CELL_CHAR_WIDTH, CELL_LINE_HEIGHT},
    errors::ParseMazeError,
};

#[derive(Debug)]
pub struct Maze {
    pub cells: Vec<Vec<Cell>>,
}

impl Maze {
    pub fn new_from_file(path: impl AsRef<Path>) -> Result<Self> {
        Self::from_str(&fs::read_to_string(path)?).map_err(|err| Error::new(err))
    }

    pub fn parse_sm(s: impl Into<String>) -> Result<Self> {
        let content = s.into();
        let lines: Vec<&str> = content.lines().collect();

        if lines.len() < CELL_LINE_HEIGHT {
            return Err(anyhow!(
                "Invalid maze: Expected number of lines to be greater than {}, but got: {}",
                CELL_LINE_HEIGHT,
                lines.len(),
            ));
        }

        let line_length = lines[0].len();

        // Calculate the number of rows and columns of cells
        let rows = (lines.len() - 1) / CELL_LINE_HEIGHT;
        let cols = (line_length - 1) / CELL_CHAR_WIDTH;

        // Parse each cell and assemble the maze
        let mut cells = Vec::new();
        for row in 0..rows {
            let mut cell_row = Vec::new();
            for col in 0..cols {
                // Extract the "square" for the current cell, including overlapping walls
                let cell_str = (0..CELL_LINE_HEIGHT)
                    .map(|i| {
                        &lines[row * CELL_LINE_HEIGHT + i]
                            [col * CELL_CHAR_WIDTH..=(col + 1) * CELL_CHAR_WIDTH]
                    })
                    .collect::<Vec<&str>>()
                    .join("\n");

                // Parse the cell
                let cell = cell_str.parse::<Cell>()?;
                cell_row.push(cell);
            }
            cells.push(cell_row);
        }

        Ok(Maze { cells })
    }

    pub fn parse_lg(s: impl Into<String>) -> Result<Self> {
        let content = s.into();
        let lines: Vec<&str> = content.lines().collect();

        if lines.len() < CELL_LINE_HEIGHT {
            return Err(anyhow!(
                "Invalid maze: Expected number of lines to be greater than {}, but got: {}",
                CELL_LINE_HEIGHT,
                lines.len(),
            ));
        }

        let line_length = lines[0].len();

        // Calculate the number of rows and columns of cells
        let rows = (lines.len() - 1) / CELL_LINE_HEIGHT;
        let cols = (line_length - 1) / CELL_CHAR_WIDTH;

        // Parse each cell and assemble the maze
        let mut cells = Vec::new();
        for row in 0..rows {
            let mut cell_row = Vec::new();
            for col in 0..cols {
                // Extract the "square" for the current cell
                let cell_str = (0..CELL_LINE_HEIGHT)
                    .map(|i| {
                        &lines[row * CELL_LINE_HEIGHT + i]
                            [col * CELL_CHAR_WIDTH..(col + 1) * CELL_CHAR_WIDTH]
                    })
                    .collect::<Vec<&str>>()
                    .join("\n");

                // Parse the cell
                let cell = cell_str.parse::<Cell>()?;
                cell_row.push(cell);
            }
            cells.push(cell_row);
        }

        Ok(Maze { cells })
    }
}

impl FromStr for Maze {
    type Err = ParseMazeError;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        let mut sm_err: Option<Error> = None;

        Self::parse_sm(s)
            .or_else(|err| {
                sm_err = Some(err);
                Self::parse_lg(s)
            })
            .map_err(|err| ParseMazeError {
                sm_err: sm_err,
                lg_err: Some(err),
            })
    }
}

impl PartialEq for Maze {
    fn eq(&self, other: &Self) -> bool {
        self.cells == other.cells
    }
}

impl Eq for Maze {}
