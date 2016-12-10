pub const X: usize = 63;
pub const Y: usize = 31;

pub use self::Static::*;
#[derive(Clone, Copy, PartialEq)]
pub enum Static {
    Wall,
    Gate,
    Goal,
    Turret,
    Obstacle,
}

pub use self::Mobile::*;
#[derive(Clone, Copy, PartialEq)]
pub enum Mobile {
    Player,
    Fiend,
    Arrow{dx: i8, dy: i8},
}

pub struct WorldData {
    pub statics: [[Option<Static>; X]; Y],
    pub mobiles: [[Option<Mobile>; X]; Y],
}
