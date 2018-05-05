use position::Position;

pub static PLAYER_SHOT: i32 = 0;
pub static ENEMY_SHOT: i32 = 1;

// Struct for manage shots
pub struct Shot {
    pub position: Position,
    pub shot_type: i32,
}

impl Shot {
    // Constructor fill fields
    pub fn new(position: Position, shot_type: i32) -> Shot {
        Shot {
            position,
            shot_type,
        }
    }
}