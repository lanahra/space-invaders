use game::position::Position;
use game::size::Size;

static VELOCITY: f64 = 500.0;

pub enum ShotType {
    PlayerShot,
    EnemyShot,
}

// Struct for manage shots
pub struct Shot {
    pub position: Position,
    pub shot_type: ShotType,
    pub size: Size,
}

impl Shot {
    // Constructor fill fields
    pub fn new(position: Position) -> Shot {
        Shot {
            position,
            shot_type: ShotType::PlayerShot,
            size:
                Size {
                    width: 6.0,
                    height: 17.0,
                },
        }
    }

    pub fn change_to_player_type(&mut self) {
        self.shot_type = ShotType::PlayerShot;
    }

    pub fn change_to_enemy_type(&mut self) {
        self.shot_type = ShotType::EnemyShot;
    }

    pub fn update(&mut self, dt: f64) {
        match self.shot_type {
            ShotType::PlayerShot => {
                self.position.y -= dt * VELOCITY;
            }

            ShotType::EnemyShot => {
                self.position.y += dt * VELOCITY;
            }

            _ => {}
        }
    }


}