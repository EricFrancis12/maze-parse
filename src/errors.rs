use std::fmt;

use anyhow::Error;

pub struct ParseMazeError {
    pub sm_err: Option<Error>,
    pub lg_err: Option<Error>,
}

impl fmt::Display for ParseMazeError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Failed to parse maze.")?;
        if let Some(sm_err) = &self.sm_err {
            write!(f, " Small maze error: {}", sm_err)?;
        }
        if let Some(lg_err) = &self.lg_err {
            write!(f, " Large maze error: {}", lg_err)?;
        }
        Ok(())
    }
}

impl fmt::Debug for ParseMazeError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("ParseMazeError")
            .field("sm_err", &self.sm_err)
            .field("lg_err", &self.lg_err)
            .finish()
    }
}

impl std::error::Error for ParseMazeError {}
