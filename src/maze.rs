use std::{fs, path::Path, str::FromStr};

use anyhow::{Error, Result};

use crate::{
    cell::{Cell, CELL_CHAR_WIDTH, CELL_LINE_HEIGHT},
    errors::ParseMazeError,
    parser::ContentParser,
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
        Self::do_parse(s, 0)
    }

    pub fn parse_lg(s: impl Into<String>) -> Result<Self> {
        Self::do_parse(s, 1)
    }

    fn do_parse(s: impl Into<String>, offset: isize) -> Result<Self> {
        let mut parser = ContentParser::new(s.into());
        let mut cells = Vec::new();

        while parser.curr_char().is_ok() {
            let starting_pos = parser.curr_pos();
            let mut row = Vec::new();

            loop {
                let mut square = vec![parser.slice(CELL_CHAR_WIDTH)?];

                for _ in 0..CELL_LINE_HEIGHT - 1 {
                    parser.next_line()?;
                    square.push(parser.slice(CELL_CHAR_WIDTH)?);
                }

                row.push(square.join("\n").parse::<Cell>()?);

                parser
                    .move_lines(-(CELL_LINE_HEIGHT as isize) + 1)
                    .expect("to be at the starting line");

                if parser.move_cols(CELL_CHAR_WIDTH as isize + offset).is_err() {
                    break;
                }
                parser
                    .move_cols(-1)
                    .expect("to be on the right edge of previous cell");
            }

            cells.push(row);

            parser
                .go_to_pos(starting_pos)
                .expect("to be at the starting position");

            if parser
                .move_lines(CELL_LINE_HEIGHT as isize + offset)
                .is_err()
            {
                break;
            }
            parser
                .move_lines(-1)
                .expect("to be on the bottom edge of previous row");
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
