use core::fmt;

#[derive(PartialEq, Copy, Clone, Eq)]
pub enum Status {
    Configuration,
    InProgress,
    Won,
    Lost,
}

impl fmt::Debug for Status {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "test")
    }
}
