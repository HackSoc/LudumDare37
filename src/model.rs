use std::collections::BTreeSet;

pub const X: usize = 63;
pub const Y: usize = 31;

pub use self::Static::*;
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Static {
    Wall,
    Gate,
    Goal { health: usize, max_health: usize },
    Turret { info: TurretInfo },
    Obstacle { health: usize, max_health: usize },
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct TurretInfo {
    pub form: (),
    pub cooldown: usize,
    pub max_cooldown: usize,
    pub range: usize,
    pub health: usize,
    pub max_health: usize,
    pub arrow_speed: usize,
}

pub use self::Mobile::*;
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Mobile {
    Player,
    Fiend { info: FiendInfo },
    Arrow { info: ArrowInfo },
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
    pub health: usize,
    pub max_health: usize,
    pub damage_factor: usize,
    pub heal_factor: usize,
    pub armour_factor: usize,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct FiendInfo {
    pub ch: char,
    pub form: (),
    pub health: usize,
    pub max_health: usize,
    pub damage_factor: usize,
    pub armour_factor: usize,
    pub player_target_distance: usize,
    pub goal_target_distance: usize,
    pub turret_target_distance: usize,
    pub obstacle_target_distance: usize,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct ArrowInfo {
    pub dx: usize,
    pub dy: usize,
    pub incx: bool,
    pub incy: bool,
    pub damage_factor: usize,
}

pub struct WorldData {
    pub statics: [[Option<Static>; X]; Y],
    pub mobiles: [[Option<Mobile>; X]; Y],
    pub player_info: PlayerInfo,
    pub fiends: BTreeSet<(usize, usize)>,
    pub turrets: BTreeSet<(usize, usize)>,
    pub arrows: BTreeSet<(usize, usize)>,
    pub obstacles: BTreeSet<(usize, usize)>,
}
