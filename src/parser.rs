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
                self.col,
            ));
        }

        Ok(lines[self.line - 1].chars().nth(self.col - 1).unwrap())
    }

    /// Returns the parser's current position (line, col).
    pub fn curr_pos(&self) -> (usize, usize) {
        (self.line, self.col)
    }

    /// Moves the parser to the specified line and column, returning the character at the new position.
    pub fn go_to(&mut self, line: usize, col: usize) -> Result<char> {
        let lines: Vec<&str> = self.content.lines().collect();

        if line == 0 || col == 0 {
            return Err(anyhow::anyhow!(
                "Invalid position: line and column must be greater than 0.",
            ));
        }

        if line > lines.len() || col > lines[line - 1].len() {
            return Err(anyhow::anyhow!(
                "Invalid position: Out of bounds (line {}, col {})",
                line,
                col,
            ));
        }

        self.line = line;
        self.col = col;

        Ok(lines[line - 1].chars().nth(col - 1).unwrap())
    }

    /// Moves the parser to the specified position, returning the character at the new position.
    pub fn go_to_pos(&mut self, (line, col): (usize, usize)) -> Result<char> {
        self.go_to(line, col)
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

    /// Moves the parser `n` columns forward or backward.
    /// Positive `n` moves forward, negative `n` moves backward.
    pub fn move_cols(&mut self, n: isize) -> Result<()> {
        let lines: Vec<&str> = self.content.lines().collect();
        if self.line == 0 || self.line > lines.len() {
            return Err(anyhow::anyhow!(
                "Invalid move: Current line {} is out of bounds.",
                self.line,
            ));
        }

        let line_length = lines[self.line - 1].len();
        let new_col = (self.col as isize + n) as usize;
        if new_col == 0 || new_col > line_length {
            return Err(anyhow::anyhow!(
                "Invalid move: Column {} is out of bounds.",
                new_col,
            ));
        }
        self.col = new_col;
        Ok(())
    }

    /// Moves the parser `n` lines forward or backward.
    /// Positive `n` moves forward, negative `n` moves backward.
    pub fn move_lines(&mut self, n: isize) -> Result<()> {
        let new_line = (self.line as isize + n) as usize;
        if new_line == 0 || new_line > self.content.lines().count() {
            return Err(anyhow::anyhow!(
                "Invalid move: Line {} is out of bounds.",
                new_line,
            ));
        }
        self.line = new_line;
        Ok(())
    }

    /// Moves the parser `n` lines and `m` columns.
    /// Positive `n` moves lines forward, negative `n` moves lines backward.
    /// Positive `m` moves columns forward, negative `m` moves columns backward.
    pub fn move_pos(&mut self, n_lines: isize, n_cols: isize) -> Result<()> {
        // Move lines first
        let new_line = (self.line as isize + n_lines) as usize;
        if new_line == 0 || new_line > self.content.lines().count() {
            return Err(anyhow::anyhow!(
                "Invalid move: Line {} is out of bounds.",
                new_line,
            ));
        }

        // Move columns
        let lines: Vec<&str> = self.content.lines().collect();
        let line_length = lines[new_line - 1].len();
        let new_col = (self.col as isize + n_cols) as usize;
        if new_col == 0 || new_col > line_length {
            return Err(anyhow::anyhow!(
                "Invalid move: Column {} is out of bounds.",
                new_col,
            ));
        }

        // Update position
        self.line = new_line;
        self.col = new_col;
        Ok(())
    }

    /// Returns a slice of the content starting from the current position
    /// with the specified width (number of columns).
    pub fn slice(&self, width: usize) -> Result<String> {
        let lines: Vec<&str> = self.content.lines().collect();

        // Ensure the current position is valid
        if self.line == 0 || self.line > lines.len() {
            return Err(anyhow::anyhow!(
                "Invalid position: Line {} is out of bounds.",
                self.line,
            ));
        }

        let line = lines[self.line - 1];
        if self.col == 0 || self.col > line.len() {
            return Err(anyhow::anyhow!(
                "Invalid position: Column {} is out of bounds.",
                self.col,
            ));
        }

        // Ensure the slice does not exceed the line's length
        let end_col = self.col + width - 1;
        if end_col > line.len() {
            return Err(anyhow::anyhow!(
                "Invalid slice: End column {} exceeds line length {}.",
                end_col,
                line.len(),
            ));
        }

        // Extract and return the slice
        Ok(line[self.col - 1..end_col].to_owned())
    }
}
