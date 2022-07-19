use core::fmt;

use crate::status::Status;

#[derive(Debug, PartialEq, Eq)]
pub enum GameError {
    IncorrectStatus(Status, Status),
    ZeroFieldArea,
    OutOfBounds,
    AlreadyMined,
    AlreadyOpened,
    AlreadyFlagged,
}

impl fmt::Display for GameError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match *self {
            GameError::IncorrectStatus(given_status, corr_status) => write!(
                f,
                "game in status {:?}, but should be in {:?}",
                given_status, corr_status
            ),
            GameError::OutOfBounds => write!(f, "position out of bounds"),
            GameError::AlreadyMined => write!(f, "position already have mine"),
            GameError::AlreadyOpened => write!(f, "position already opened"),
            GameError::AlreadyFlagged => write!(f, "position already have flag"),
            GameError::ZeroFieldArea => write!(f, "field area is zero"),
        }
    }
}
