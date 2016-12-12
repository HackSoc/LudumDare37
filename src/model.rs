use std::collections::BTreeSet;
use std::fmt;

pub const X: usize = 63;
pub const Y: usize = 31;

pub use self::Static::*;
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Static {
    Wall,
    Gate,
    Goal { health: usize, max_health: usize },
    Turret { info: TurretInfo },
    Obstacle { health: usize, max_health: usize },
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct TurretInfo {
    pub form: (),
    pub cooldown: usize,
    pub max_cooldown: usize,
    pub range: usize,
    pub health: usize,
    pub max_health: usize,
    pub arrow_speed: usize,
    pub damage_factor: usize,
}

pub use self::Mobile::*;
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
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

#[derive(Clone, Copy, PartialEq, Eq)]
pub struct PlayerInfo {
    pub location: (usize, usize),
    pub health: usize,
    pub max_health: usize,
    pub damage_factor: usize,
    pub heal_factor: usize,
    pub armour_factor: usize,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct FiendName {
    pub prefix: Option<&'static str>,
    pub name: &'static str,
    pub suffix: Option<&'static str>,
}

impl fmt::Display for FiendName {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f,
               "{}{}{}",
               self.prefix.unwrap_or(""),
               self.name,
               self.suffix.unwrap_or(""))
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct FiendInfo {
    pub ch: char,
    pub name: FiendName,
    pub form: (),
    pub health: usize,
    pub max_health: usize,
    pub damage_factor: usize,
    pub armour_factor: usize,
    pub player_target_distance: usize,
    pub goal_target_distance: usize,
    pub turret_target_distance: usize,
    pub obstacle_target_distance: usize,
    pub value: usize,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct ArrowInfo {
    pub dx: usize,
    pub dy: usize,
    pub incx: bool,
    pub incy: bool,
    pub damage_factor: usize,
}

#[derive(PartialEq, Eq, Clone, Copy)]
pub enum Menu {
    Root,
    Build,
    Move,
    Upgrade,
    Continue,
    Place,
    Move2,
}

// pub enum RootItem {
// Build,
// Move,
// Upgrade,
// Continue
// }
//
// pub enum BuildItem {
// Turret,
// Obstacle
// }
//

pub struct WorldData {
    pub statics: [[Option<Static>; X]; Y],
    pub mobiles: [[Option<Mobile>; X]; Y],
    pub player_info: PlayerInfo,
    pub goal_location: (usize, usize),
    pub fiends: BTreeSet<(usize, usize)>,
    pub turrets: BTreeSet<(usize, usize)>,
    pub arrows: BTreeSet<(usize, usize)>,
    pub obstacles: BTreeSet<(usize, usize)>,
    pub gates: BTreeSet<(usize, usize)>,
    pub placement: Option<Static>,
    pub log: [String; 5],
    pub cash: usize,
    pub wave: usize,
}

impl WorldData {
    pub fn log_msg(&mut self, msg: String) {
        let len = self.log.len();
        for i in 1..len {
            self.log[len - i] = self.log[len - i - 1].clone();
        }
        self.log[0] = msg;
    }
}

pub use self::GameState::*;
#[derive(PartialEq, Eq)]
pub enum GameState {
    Startup,
    Construct { menu: Menu, menu_index: usize },
    Fight { to_spawn: Vec<FiendInfo> },
    GameOver { msg: String },
    End,
}
