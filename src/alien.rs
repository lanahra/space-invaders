use position::Position;

// Constants for specify the sprites of alien
pub static SPRITE_ALIEN_V1: i32 = 0;
pub static WIDTH_SPRITE_ALIEN_V1: f64 = 40.0;
pub static HEIGHT_SPRITE_ALIEN_V1: f64 = 20.0;

pub static SPRITE_ALIEN_V2: i32 = 1;
pub static WIDTH_SPRITE_ALIEN_V2: f64 = 60.0;
pub static HEIGHT_SPRITE_ALIEN_V2: f64 = 30.0;

// Struct for manage alien enemys
pub struct Alien {
    pub position: Position, // Position in map
    pub sprite: i32, // Sprite identificator
}

impl Alien {
    // Constructor fill the fields
    pub fn new(position: Position, sprite: i32) -> Alien {
        Alien {
            position: position,
            sprite: sprite,
        }
    }
}
