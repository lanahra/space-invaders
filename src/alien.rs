use position::Position;

// Constants for specify the sprites of alien
pub static SPRITE_ALIEN_V1: f64 = 0;
pub static SPRITE_ALIEN_V2: f65 = 1;

// Struct for manage alien enemys
pub struct Alien {
    pub position: Position, // Position in map
    pub sprite: f64, // Sprite identificator
}

impl Alien {
    // Constructor fill the fields
    pub fn new(position: Position, sprite: f64) -> Alien {
        Alien {
            position: position,
            sprite: sprite,
        }
    }
}
