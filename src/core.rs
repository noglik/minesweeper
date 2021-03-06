use std::collections::HashSet;

use crate::error::GameError;
use crate::position::Position;
use crate::status::Status;

pub struct Game {
    pub width: usize,
    pub height: usize,
    pub mine_positions: HashSet<Position>,
    pub open_positions: HashSet<Position>,
    pub flag_positions: HashSet<Position>,
    pub status: Status,
}

impl Game {
    pub fn new(width: usize, height: usize) -> Result<Game, GameError> {
        if width == 0 || height == 0 {
            return Err(GameError::ZeroFieldArea);
        }

        Ok(Game {
            width,
            height,
            mine_positions: HashSet::new(),
            open_positions: HashSet::new(),
            flag_positions: HashSet::new(),
            status: Status::Configuration,
        })
    }

    fn is_in_bounds(&self, position: &Position) -> bool {
        if position.0 > self.width - 1 {
            return false;
        }
        if position.1 > self.height - 1 {
            return false;
        }

        true
    }

    pub fn mine(&mut self, position: Position) -> Result<(), GameError> {
        if self.status != Status::Configuration {
            return Err(GameError::IncorrectStatus(
                self.status,
                Status::Configuration,
            ));
        }

        if !self.is_in_bounds(&position) {
            return Err(GameError::OutOfBounds);
        }

        if self.mine_positions.contains(&position) {
            return Err(GameError::AlreadyMined);
        }

        self.mine_positions.insert(position);
        Ok(())
    }

    pub fn start(&mut self) -> Result<(), GameError> {
        if self.status != Status::Configuration {
            return Err(GameError::IncorrectStatus(
                self.status,
                Status::Configuration,
            ));
        }

        self.status = Status::InProgress;
        Ok(())
    }

    pub fn open(&mut self, position: Position) -> Result<(), GameError> {
        if self.status != Status::InProgress {
            return Err(GameError::IncorrectStatus(self.status, Status::InProgress));
        }

        if !self.is_in_bounds(&position) {
            return Err(GameError::OutOfBounds);
        }

        if self.open_positions.contains(&position) {
            return Err(GameError::AlreadyOpened);
        }

        if self.flag_positions.contains(&position) {
            self.flag_positions.remove(&position);
        }

        if self.mine_positions.contains(&position) {
            self.status = Status::Lost;
            return Ok(());
        }

        self.open_positions.insert(position);

        if self.open_positions.len() + self.flag_positions.len() == self.width * self.height {
            self.status = Status::Won;
        }

        Ok(())
    }

    pub fn flag(&mut self, position: Position) -> Result<(), GameError> {
        if self.status != Status::InProgress {
            return Err(GameError::IncorrectStatus(self.status, Status::InProgress));
        }

        if !self.is_in_bounds(&position) {
            return Err(GameError::OutOfBounds);
        }

        if self.open_positions.contains(&position) {
            return Err(GameError::AlreadyOpened);
        }

        if self.flag_positions.contains(&position) {
            return Err(GameError::AlreadyFlagged);
        }

        self.flag_positions.insert(position);

        if self.open_positions.len() + self.flag_positions.len() == self.width * self.height {
            self.status = Status::Won;
        }

        Ok(())
    }

    fn check_proximity(&self, position: Position) -> Result<u8, GameError> {
        if self.status != Status::InProgress {
            return Err(GameError::IncorrectStatus(self.status, Status::InProgress));
        }

        if !self.is_in_bounds(&position) {
            return Err(GameError::OutOfBounds);
        }

        let mut mine_proximity_counter: u8 = 0;

        // TODO: think of better way of getting neigbour relative coordinates
        let relative_coordinates: Vec<(i8, i8)> = vec![
            (-1, -1),
            (0, -1),
            (1, -1),
            (-1, 0),
            (1, 0),
            (-1, 1),
            (0, 1),
            (1, 1),
        ];

        for (x_dif, y_dif) in relative_coordinates.iter() {
            match position.get_relative((*x_dif).into(), (*y_dif).into()) {
                Ok(neighbour) => {
                    if self.is_in_bounds(&neighbour) && self.mine_positions.contains(&neighbour) {
                        mine_proximity_counter += 1;
                    }
                }
                // don't expect to happen, dif values are too small
                Err(_) => (),
            }
        }

        Ok(mine_proximity_counter)
    }
}

#[cfg(test)]
mod game_new {
    use super::*;

    #[test]
    fn create_new_game() {
        let game = Game::new(100, 100).expect("game created");

        assert_eq!(game.width, 100);
        assert_eq!(game.height, 100);
        assert_eq!(game.mine_positions.len(), 0);
        assert_eq!(game.open_positions.len(), 0);
        assert_eq!(game.flag_positions.len(), 0);
        assert_eq!(game.status, Status::Configuration);
    }

    #[test]
    fn zero_area() {
        assert!(matches!(Game::new(0, 1), Err(GameError::ZeroFieldArea)));
    }
}

#[cfg(test)]
mod game_is_in_bounds {
    use super::*;

    #[test]
    fn in_bounds() {
        let game = Game::new(10, 10).expect("game created");

        assert!(game.is_in_bounds(&Position(1, 1)));
    }

    #[test]
    fn out_of_bounds() {
        let game = Game::new(10, 10).expect("game created");

        assert_eq!(game.is_in_bounds(&Position(100, 1)), false);
    }
}

#[cfg(test)]
mod game_mine {
    use super::*;

    #[test]
    fn set_mine_in_fresh_game() {
        let mut game = Game::new(100, 100).expect("game created");

        let mine_position = Position(1, 1);

        game.mine(mine_position).expect("Set mine");

        assert!(game.mine_positions.contains(&mine_position));
    }

    #[test]
    fn set_mine_in_progress_game() {
        let mut game = Game::new(10, 10).expect("game created");

        game.start().expect("Game started");

        assert_eq!(
            game.mine(Position(1, 1)),
            Err(GameError::IncorrectStatus(
                Status::InProgress,
                Status::Configuration
            ))
        );
    }

    #[test]
    fn set_mine_twice() {
        let mut game = Game::new(10, 10).expect("game created");

        let mine_position = Position(1, 1);

        game.mine(mine_position).expect("Set mine");
        assert_eq!(game.mine(mine_position), Err(GameError::AlreadyMined));
    }

    #[test]
    fn set_mine_out_of_bounds() {
        let mut game = Game::new(10, 10).expect("game created");

        assert_eq!(game.mine(Position(20, 5)), Err(GameError::OutOfBounds));
    }
}

#[cfg(test)]
mod game_start {
    use super::*;

    #[test]
    fn start_fresh_game() {
        let mut game = Game::new(10, 10).expect("game created");

        game.start().expect("Game started");

        assert_eq!(game.status, Status::InProgress);
    }

    #[test]
    fn start_already_started_game() {
        let mut game = Game::new(1, 1).expect("game created");

        game.start().expect("Game started");

        assert_eq!(
            game.start(),
            Err(GameError::IncorrectStatus(
                Status::InProgress,
                Status::Configuration
            ))
        );
    }
}

#[cfg(test)]
mod game_open {
    use super::*;

    #[test]
    fn open_in_config_game() {
        let mut game = Game::new(1, 1).expect("game created");

        assert_eq!(
            game.open(Position(1, 1)),
            Err(GameError::IncorrectStatus(
                Status::Configuration,
                Status::InProgress
            ))
        );
    }

    #[test]
    fn open_safe_position() {
        let mut game = Game::new(10, 10).expect("game created");

        let mine_position = Position(1, 1);
        let safe_position = Position(1, 2);

        game.mine(mine_position).expect("Set mine");
        game.start().expect("Game started");

        game.open(safe_position).expect("Position opened");

        assert_eq!(game.status, Status::InProgress);
        assert!(game.open_positions.contains(&safe_position));
    }

    #[test]
    fn open_mine_position() {
        let mut game = Game::new(10, 10).expect("game created");

        let mine_position = Position(1, 1);

        game.mine(mine_position).expect("Set mine");
        game.start().expect("Game started");

        game.open(mine_position).expect("Position opened");

        assert_eq!(game.status, Status::Lost);
    }

    #[test]
    fn open_safe_position_twice() {
        let mut game = Game::new(10, 10).expect("game created");

        let open = Position(1, 2);

        game.start().expect("Game started");

        game.open(open).expect("Position opened");

        assert_eq!(game.open(open), Err(GameError::AlreadyOpened));
    }

    #[test]
    fn open_flagged_position() {
        let mut game = Game::new(10, 10).expect("game created");

        let flag = Position(1, 2);

        game.start().expect("Game started");

        game.flag(flag).expect("Position flagged");
        game.open(flag).expect("Position opened");

        assert_eq!(game.flag_positions.contains(&flag), false);
        assert!(game.open_positions.contains(&flag));
    }

    #[test]
    fn out_of_bounds() {
        let mut game = Game::new(10, 10).expect("game created");
        game.start().expect("Game started");

        assert_eq!(game.open(Position(11, 10)), Err(GameError::OutOfBounds));
    }

    #[test]
    fn win_game() {
        let mut game = Game::new(1, 2).expect("game created");

        game.start().expect("Game started");

        game.flag(Position(0, 0)).expect("Position flagged");
        game.open(Position(0, 1)).expect("Position opened");

        assert!(matches!(game.status, Status::Won));
    }
}

#[cfg(test)]
mod game_flag {
    use super::*;

    #[test]
    fn flag_position() {
        let mut game = Game::new(10, 10).expect("game created");

        let flag_position = Position(1, 1);

        game.start().expect("Game started");

        game.flag(flag_position).expect("Position flagged");

        assert!(game.flag_positions.contains(&flag_position));
    }

    #[test]
    fn flag_before_start() {
        let mut game = Game::new(10, 10).expect("game created");

        assert_eq!(
            game.flag(Position(1, 1)),
            Err(GameError::IncorrectStatus(
                Status::Configuration,
                Status::InProgress
            ))
        );
    }

    #[test]
    fn flag_position_twice() {
        let mut game = Game::new(10, 10).expect("game created");

        let flag_position = Position(1, 1);

        game.start().expect("Game started");

        game.flag(flag_position).expect("Position flagged");

        assert_eq!(game.flag(flag_position), Err(GameError::AlreadyFlagged));
    }

    #[test]
    fn flag_open_position() {
        let mut game = Game::new(10, 10).expect("game created");

        let open = Position(1, 1);

        game.start().expect("Game started");

        game.open(open).expect("Position opened");

        assert_eq!(game.flag(open), Err(GameError::AlreadyOpened));
    }

    #[test]
    fn out_of_bounds() {
        let mut game = Game::new(10, 10).expect("game created");

        game.start().expect("Game started");

        assert_eq!(game.flag(Position(12, 8)), Err(GameError::OutOfBounds));
    }

    #[test]
    fn win_game() {
        let mut game = Game::new(1, 2).expect("game created");

        let mine = Position(0, 1);

        game.mine(mine).expect("Set mine");

        game.start().expect("Game started");

        game.open(Position(0, 0)).expect("Position opened");
        game.flag(mine).expect("Position flagged");

        assert!(matches!(game.status, Status::Won));
    }
}

#[cfg(test)]
mod game_check_proximity {
    use super::*;

    #[test]
    fn zero_mines() {
        let mut game = Game::new(5, 5).expect("game created");

        game.start().expect("game started");

        assert_eq!(game.check_proximity(Position(3, 3)), Ok(0));
    }

    #[test]
    fn some_mines() {
        let mut game = Game::new(5, 5).expect("game created");

        game.mine(Position(2, 3)).expect("Set mine");
        game.mine(Position(4, 4)).expect("Set mine");

        game.start().expect("game started");

        assert_eq!(game.check_proximity(Position(3, 3)), Ok(2));
    }

    #[test]
    fn border_position() {
        let mut game = Game::new(5, 5).expect("game created");

        game.mine(Position(0, 2)).expect("Set mine");
        game.mine(Position(1, 3)).expect("Set mine");
        game.mine(Position(0, 4)).expect("Set mine");

        game.start().expect("game started");

        assert_eq!(game.check_proximity(Position(0, 3)), Ok(3));
    }
}
