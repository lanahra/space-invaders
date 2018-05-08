use game::position::Position;
use game::size::Size;
use game::WIDTH;
use game::HEIGHT;


const VELOCITY: f64 = 1.3 * WIDTH;

pub enum ShotType {
    PlayerShot,
    EnemyShot,
}

pub enum ShotActive {
    Active,
    Inactive,
}

// Struct for manage shots
pub struct Shot {
    pub position: Position,
    pub shot_type: ShotType,
    pub size: Size,
    pub active: ShotActive,
}

impl Shot {
    // Constructor fill fields
    pub fn new(position: Position) -> Shot {
        Shot {
            position,
            shot_type: ShotType::PlayerShot,
            size:
                Size {
                    width: 0.004*WIDTH,
                    height: 0.02125*HEIGHT,
                },
            active : ShotActive::Inactive,
        }
    }
    pub fn activate_shot(&mut self) {
        self.active = ShotActive::Active;
    }

    pub fn inactivate_shot(&mut self) {
        self.active = ShotActive::Inactive;
    }

    pub fn is_active(&self) -> bool {
        match self.active {
            ShotActive::Active => {
                return true;
            }
            ShotActive::Inactive => {
                return false;
            }
        }
    }

    pub fn change_to_player_type(&mut self) {
        self.shot_type = ShotType::PlayerShot;
    }

    pub fn change_to_enemy_type(&mut self) {
        self.shot_type = ShotType::EnemyShot;
    }

    pub fn update(&mut self, dt: f64) {
        if self.is_active() {
            match self.shot_type {
                ShotType::PlayerShot if self.position.y < 0.0 => {
                    self.inactivate_shot();
                }

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


}