use model::*;

use pancurses::Input;
use pancurses::Input::*;

use std::cmp::{min, max};

#[derive(PartialEq, Eq)]
pub enum GameState{
    Startup,
    Build,
    Fight,
    GameOver,
    End
}

pub use self::GameState::*;

enum Dir {N,E,S,W}

impl Static {
    fn player_interact(&mut self, player_info: &PlayerInfo) {
        match *self {
            Wall | Gate => {},
            Obstacle{mut health, max_health} |
            Goal{mut health, max_health} |
            Turret{mut health, max_health, ..} =>
                health = min(health + player_info.heal_factor, max_health),
        };
    }
}

impl Mobile {
    fn player_interact(&mut self, player_info: &PlayerInfo) {
        match *self {
            Arrow{..} => {},
            Fiend{mut health, ..} =>
                health = health.saturating_sub(player_info.damage_factor),
            Player => panic!("Player walked into themself")
        };
    }
}

impl GameState {
    pub fn handle(&mut self, world_data: &mut WorldData, i: Input) {
        match *self {
            Startup => *self = Build,
            Build => unimplemented!(),
            Fight => self.fight_handler(world_data, i),
            GameOver => unimplemented!(),
            End => panic!("Should have ended and didn't!")
        };
    }

    fn fight_handler(&mut self, world_data: &mut WorldData, i: Input) {
        match i {
            KeyDown  | Character('s') => world_data.move_player(Dir::S),
            KeyUp    | Character('w') => world_data.move_player(Dir::N),
            KeyLeft  | Character('a') => world_data.move_player(Dir::W),
            KeyRight | Character('d') => world_data.move_player(Dir::E),
            Character('q') => *self = End,
            _ => {}
        };
    }
}

impl WorldData {
    fn move_player(&mut self, dir: Dir) {
        let old_x = self.player_info.location.0;
        let old_y = self.player_info.location.1;
        assert!(self.mobiles[old_y][old_x].map_or(false, |p| p.is_player()));
        let mut new_x = old_x;
        let mut new_y = old_y;
        match dir {
            Dir::N => new_y = old_y - 1,
            Dir::E => new_x = old_x + 1,
            Dir::S => new_y = old_y + 1,
            Dir::W => new_x = old_x - 1
        };
        match self.statics[new_y][new_x] {
            Some(mut sta) => {
                sta.player_interact(&self.player_info);
                return;
            },
            None => {} // we can move into an empty space
        };
        match self.mobiles[new_y][new_x] {
            Some(mut mob) => {
                mob.player_interact(&self.player_info);
                return;
            },
            None => {} // we can move into an empty space
        }
        self.player_info.location = (new_x, new_y);
        self.mobiles[old_y][old_x] = None;
        self.mobiles[new_y][new_x] = Some(Player);
        assert!(self.mobiles[new_y][new_x].map_or(false, |p| p.is_player()));
    }
}
