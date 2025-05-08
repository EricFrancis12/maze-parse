use std::usize;

use anyhow::Result;

pub struct ContentParser {
    content: String,
    line: usize,
    col: usize,
}

impl ContentParser {
    pub fn new(content: impl Into<String>) -> Self {
        return Self {
            content: content.into(),
            line: 1,
            col: 1,
        };
    }

    /// Returns the current character at the parser's position.
    pub fn curr_char(&self) -> Result<char> {
        let lines: Vec<&str> = self.content.lines().collect();

        if self.line == 0
            || self.col == 0
            || self.line > lines.len()
            || self.col > lines[self.line - 1].len()
        {
            return Err(anyhow::anyhow!(
                "Invalid position: Out of bounds (line {}, col {}).",
                self.line,
                self.col
            ));
        }

        Ok(lines[self.line - 1].chars().nth(self.col - 1).unwrap())
    }

    /// Moves the parser to the specified line and column, returning the character at the new position.
    pub fn go_to(&mut self, line: usize, col: usize) -> Result<char> {
        let lines: Vec<&str> = self.content.lines().collect();

        if line == 0 || col == 0 {
            return Err(anyhow::anyhow!(
                "Invalid position: line and column must be greater than 0."
            ));
        }

        if line > lines.len() || col > lines[line - 1].len() {
            return Err(anyhow::anyhow!(
                "Invalid position: Out of bounds (line {}, col {}).",
                line,
                col
            ));
        }

        self.line = line;
        self.col = col;

        Ok(lines[line - 1].chars().nth(col - 1).unwrap())
    }

    /// Move to the next column
    pub fn next_col(&mut self) -> Result<char> {
        self.go_to(self.line, self.col + 1)
    }

    /// Move to the previous column
    pub fn prev_col(&mut self) -> Result<char> {
        if self.col == 1 {
            return Err(anyhow::anyhow!("Cannot move to a column less than 1."));
        }
        self.go_to(self.line, self.col - 1)
    }

    /// Move to the next line
    pub fn next_line(&mut self) -> Result<char> {
        self.go_to(self.line + 1, self.col)
    }

    /// Move to the previous line
    pub fn prev_line(&mut self) -> Result<char> {
        if self.line == 1 {
            return Err(anyhow::anyhow!("Cannot move to a line less than 1."));
        }
        self.go_to(self.line - 1, self.col)
    }

    /// Move to the start of the current line
    pub fn start_of_line(&mut self) -> Result<char> {
        self.go_to(self.line, 1)
    }

    /// Move to the end of the current line
    pub fn end_of_line(&mut self) -> Result<char> {
        let lines: Vec<&str> = self.content.lines().collect();
        let line_length = lines[self.line - 1].len();
        self.go_to(self.line, line_length)
    }
}
