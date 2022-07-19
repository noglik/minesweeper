use crate::error::GameError;

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
pub struct Position(pub(crate) usize, pub(crate) usize);

impl Position {
    pub fn get_relative(&self, x_dif: isize, y_dif: isize) -> Result<Position, GameError> {
        let x: Option<usize>;
        let y: Option<usize>;

        if x_dif.is_negative() {
            x = self.0.checked_sub(
                x_dif
                    .checked_neg()
                    .unwrap_or(0isize)
                    .try_into()
                    .unwrap_or(usize::MIN),
            );
        } else {
            x = self.0.checked_add(x_dif.try_into().unwrap_or(usize::MAX));
        }

        if x.is_none() {
            return Err(GameError::OutOfBounds);
        }

        if y_dif.is_negative() {
            y = self.1.checked_sub(
                y_dif
                    .checked_neg()
                    .unwrap_or(0isize)
                    .try_into()
                    .unwrap_or(usize::MIN),
            );
        } else {
            y = self.1.checked_add(y_dif.try_into().unwrap_or(usize::MAX));
        }

        if y.is_none() {
            return Err(GameError::OutOfBounds);
        }

        Ok(Position(x.unwrap(), y.unwrap()))
    }
}

#[cfg(test)]
mod position_get_relative {
    use super::*;

    #[test]
    fn get_relative_with_negative() {
        assert_eq!(Position(2, 2).get_relative(-1, -1), Ok(Position(1, 1)));
    }

    #[test]
    fn get_relative_with_positive() {
        assert_eq!(Position(2, 2).get_relative(1, 1), Ok(Position(3, 3)));
    }

    #[test]
    fn get_relative_with_oob_negative() {
        assert_eq!(
            Position(2, 2).get_relative(-100, -100),
            Err(GameError::OutOfBounds)
        );
    }
}
