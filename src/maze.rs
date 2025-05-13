use std::{fs, path::Path, str::FromStr};

use anyhow::{Error, Result};

use crate::{
    cell::{Cell, CELL_CHAR_WIDTH, CELL_LINE_HEIGHT},
    errors::ParseMazeError,
    parser::ContentParser,
};

#[derive(Debug, Eq, PartialEq)]
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
            let row_start = parser.curr_pos();
            let mut row = Vec::new();

            loop {
                let cell_lines = (0..CELL_LINE_HEIGHT)
                    .map(|i| {
                        if i > 0 {
                            parser.next_line()?;
                        }
                        parser.slice(CELL_CHAR_WIDTH)
                    })
                    .collect::<Result<Vec<String>>>()?;

                row.push(cell_lines.join("\n").parse::<Cell>()?);

                parser
                    .move_lines(-(CELL_LINE_HEIGHT as isize) + 1)
                    .expect("should be at the start of cell row");

                if parser.move_cols(CELL_CHAR_WIDTH as isize + offset).is_err() {
                    break;
                }

                parser
                    .move_cols(-1)
                    .expect("should be on the right edge of previous cell");
            }

            cells.push(row);

            parser
                .go_to_pos(row_start)
                .expect("should return to start of current row");

            if parser
                .move_lines(CELL_LINE_HEIGHT as isize + offset)
                .is_err()
            {
                break;
            }

            parser
                .move_lines(-1)
                .expect("should be at the bottom edge of the row");
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
