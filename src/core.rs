use core::fmt;
use std::collections::HashSet;

#[derive(Debug, PartialEq, Eq)]
enum GameError {
    IncorrectStatus(Status, Status),
}

impl fmt::Display for GameError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match *self {
            GameError::IncorrectStatus(given_status, corr_status) => write!(
                f,
                "game in status {:?}, but should be in {:?}",
                given_status, corr_status
            ),
        }
    }
}

#[derive(PartialEq, Copy, Clone, Eq)]
enum Status {
    Configuration,
    InProgress,
    Won,
    Lost,
}

impl fmt::Debug for Status {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
struct Position(usize, usize);

pub struct Game {
    width: usize,
    height: usize,
    mine_positions: HashSet<Position>,
    open_positions: HashSet<Position>,
    flag_positions: HashSet<Position>,
    status: Status,
}

impl Game {
    fn new(width: usize, height: usize) -> Game {
        Game {
            width,
            height,
            mine_positions: HashSet::new(),
            open_positions: HashSet::new(),
            flag_positions: HashSet::new(),
            status: Status::Configuration,
        }
    }

    fn mine(&mut self, position: Position) -> Result<(), GameError> {
        if self.status != Status::Configuration {
            return Err(GameError::IncorrectStatus(
                self.status,
                Status::Configuration,
            ));
        }

        self.mine_positions.insert(position);
        Ok(())
    }

    fn start(&mut self) -> Result<(), GameError> {
        if self.status != Status::Configuration {
            return Err(GameError::IncorrectStatus(
                self.status,
                Status::Configuration,
            ));
        }

        self.status = Status::InProgress;
        Ok(())
    }

    fn open(&mut self, position: Position) -> Result<(), GameError> {
        if self.status != Status::InProgress {
            return Err(GameError::IncorrectStatus(self.status, Status::InProgress));
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

    fn flag(&mut self, position: Position) -> Result<(), GameError> {
        if self.status != Status::InProgress {
            return Err(GameError::IncorrectStatus(self.status, Status::InProgress));
        }

        self.flag_positions.insert(position);

        if self.open_positions.len() + self.flag_positions.len() == self.width * self.height {
            self.status = Status::Won;
        }

        Ok(())
    }
}

#[cfg(test)]
mod game_new {
    use super::*;

    #[test]
    fn create_new_game() {
        let game = Game::new(100, 100);

        assert_eq!(game.width, 100);
        assert_eq!(game.height, 100);
        assert_eq!(game.mine_positions.len(), 0);
        assert_eq!(game.open_positions.len(), 0);
        assert_eq!(game.flag_positions.len(), 0);
        assert_eq!(game.status, Status::Configuration);
    }
}

#[cfg(test)]
mod game_mine {
    use super::*;

    #[test]
    fn set_mine_in_fresh_game() {
        let mut game = Game::new(100, 100);

        let mine_position = Position(1, 1);

        game.mine(mine_position).expect("Set mine");

        assert!(game.mine_positions.contains(&mine_position));
    }

    #[test]
    fn set_mine_in_progress_game() {
        let mut game = Game::new(10, 10);

        game.start().expect("Game started");

        assert_eq!(
            game.mine(Position(1, 1)),
            Err(GameError::IncorrectStatus(
                Status::InProgress,
                Status::Configuration
            ))
        );
    }
}

#[cfg(test)]
mod game_start {
    use super::*;

    #[test]
    fn start_fresh_game() {
        let mut game = Game::new(10, 10);

        game.start().expect("Game started");

        assert_eq!(game.status, Status::InProgress);
    }

    #[test]
    fn start_already_started_game() {
        let mut game = Game::new(1, 1);

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
        let mut game = Game::new(1, 1);

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
        let mut game = Game::new(10, 10);

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
        let mut game = Game::new(10, 10);

        let mine_position = Position(1, 1);

        game.mine(mine_position).expect("Set mine");
        game.start().expect("Game started");

        game.open(mine_position).expect("Position opened");

        assert_eq!(game.status, Status::Lost);
    }

    fn win_game() {
        let mut game = Game::new(1, 2);

        game.mine(Position(1, 2)).expect("Set mine");

        game.start().expect("Game started");

        game.flag(Position(1, 1)).expect("Position flagged");
        game.open(Position(1, 2)).expect("Position opened");

        assert!(matches!(game.status, Status::Won));
    }
}

#[cfg(test)]
mod game_flag {
    use super::*;

    #[test]
    fn flag_position() {
        let mut game = Game::new(10, 10);

        let flag_position = Position(1, 1);

        game.start().expect("Game started");

        game.flag(flag_position).expect("Position flagged");

        assert!(game.flag_positions.contains(&flag_position));
    }

    #[test]
    fn flag_before_start() {
        let mut game = Game::new(10, 10);

        assert_eq!(
            game.flag(Position(1, 1)),
            Err(GameError::IncorrectStatus(
                Status::Configuration,
                Status::InProgress
            ))
        );
    }

    #[test]
    fn win_game() {
        let mut game = Game::new(1, 2);

        game.mine(Position(1, 2)).expect("Set mine");

        game.start().expect("Game started");

        game.open(Position(1, 1)).expect("Position opened");
        game.flag(Position(1, 2)).expect("Position flagged");

        assert!(matches!(game.status, Status::Won));
    }
}
