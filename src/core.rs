use std::collections::HashSet;

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
struct Position(usize, usize);

pub struct Game {
    width: usize,
    height: usize,
    mine_positions: HashSet<Position>,
    open_positions: HashSet<Position>,
    lost: bool,
}

impl Game {
    fn new(width: usize, height: usize) -> Game {
        Game {
            width,
            height,
            mine_positions: HashSet::new(),
            open_positions: HashSet::new(),
            lost: false,
        }
    }

    fn set_mine_position(&mut self, position: Position) -> () {
        self.mine_positions.insert(position);
    }

    fn open(&mut self, position: Position) -> () {
        if self.mine_positions.contains(&position) {
            self.lost = true;
        } else {
            self.open_positions.insert(position);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn create_new_game() {
        let game = Game::new(100, 100);

        assert_eq!(game.width, 100);
        assert_eq!(game.height, 100);
        assert_eq!(game.mine_positions.len(), 0);
        assert_eq!(game.open_positions.len(), 0);
        assert_eq!(game.lost, false);
    }

    #[test]
    fn set_mine_in_fresh_game() {
        let mut game = Game::new(100, 100);

        let mine_position = Position(1, 1);

        game.set_mine_position(mine_position);

        assert!(game.mine_positions.contains(&mine_position));
    }

    #[test]
    fn open_safe_position() {
        let mut game = Game::new(10, 10);

        let mine_position = Position(1, 1);
        let safe_position = Position(1, 2);

        game.set_mine_position(mine_position);
        game.open(safe_position);

        assert_eq!(game.lost, false);
        assert!(game.open_positions.contains(&safe_position));
    }

    #[test]
    fn open_mine_position() {
        let mut game = Game::new(10, 10);

        let mine_position = Position(1, 1);

        game.set_mine_position(mine_position);
        game.open(mine_position);

        assert!(game.lost);
    }
}
