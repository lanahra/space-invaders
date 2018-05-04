use position::Position;

// Constants for specify the sprites of alien
pub static SPRITE_ALIEN_V1: i32 = 0;
pub static SPRITE_ALIEN_V2: i32 = 1;

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
