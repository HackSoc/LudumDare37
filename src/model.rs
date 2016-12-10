pub const X: usize = 63;
pub const Y: usize = 31;

pub use self::Static::*;
#[derive(Clone, Copy, PartialEq)]
pub enum Static {
    Wall,
    Gate,
    Goal { health: u8, max_health: u8 },
    Turret {
        form: (),
        reload_counter: u8,
        health: u8,
        max_health: u8,
    },
    Obstacle { health: u8, max_health: u8 },
}

pub use self::Mobile::*;
#[derive(Clone, Copy, PartialEq)]
pub enum Mobile {
    Player,
    Fiend { info: FiendInfo },
    Arrow { dx: i8, dy: i8 },
}

impl Mobile {
    pub fn is_player(&self) -> bool {
        match *self {
            Player => true,
            _ => false,
        }
    }
}

#[derive(Clone, Copy, PartialEq)]
pub struct PlayerInfo {
    pub location: (usize, usize),
    pub health: u8,
    pub max_health: u8,
    pub damage_factor: u8,
    pub heal_factor: u8,
    pub armour_factor: u8,
}

#[derive(Clone, Copy, PartialEq)]
pub struct FiendInfo {
    pub ch: char,
    pub form: (),
    pub health: u8,
    pub damage_factor: u8,
    pub armour_factor: u8,
    pub player_target_distance: u8,
    pub goal_target_distance: u8,
    pub turret_target_distance: u8,
    pub obstacle_target_distance: u8,
}

pub struct WorldData {
    pub statics: [[Option<Static>; X]; Y],
    pub mobiles: [[Option<Mobile>; X]; Y],
    pub player_info: PlayerInfo,
}
