use game::position::Position;
use game::size::Size;
use game::WIDTH;
use game::HEIGHT;
use game::collision::Collision;
use game::entity::Entity;
use game::entity::active::Active;

const VELOCITY: f64 = 1.3 * WIDTH;

pub enum Kind {
    PlayerShot,
    EnemyShot,
}

pub enum State {
    Active,
    Inactive,
}

// Struct for manage shots
pub struct Shot {
    pub position: Position,
    pub kind: Kind,
    pub size: Size,
    pub state: State,
}

impl Shot {
    // Constructor fill fields
    pub fn new(position: Position, kind: Kind) -> Shot {
        Shot {
            position,
            kind,
            size:
                Size {
                    width: 0.004*WIDTH,
                    height: 0.02125*HEIGHT,
                },
            state : State::Inactive,
        }
    }
}

impl Collision for Shot {
    fn position(&self) -> &Position {
        &self.position
    }

    fn size(&self) -> &Size {
        &self.size
    }
}

impl Entity for Shot {
    fn is_active(&self) -> bool {
        match self.state {
            State::Inactive => {
                return false;
            }

            _ => {
                return true;
            }
        }
    }

    fn shot_hit(&mut self) {}

    fn change_state(&mut self) {
        match self.state {
            State::Active => {
                self.state = State::Inactive;
            }

            State::Inactive => {
                self.state = State::Active;
            }
        }
    }
}

impl Active for Shot {
    fn position(&mut self) -> &mut Position {
        &mut self.position
    }

    fn update(&mut self, dt: f64) {
        if self.is_active() {
            match self.kind {
                Kind::PlayerShot if self.position.y < 0.0 => {
                    self.change_state();
                }

                Kind::PlayerShot => {
                    self.move_y(-dt * VELOCITY);
                }

                Kind::EnemyShot if self.position.y > HEIGHT => {
                    self.change_state();
                }

                Kind::EnemyShot => {
                    self.move_y(dt * VELOCITY/2.0);
                }

                _ => {}
            }
        }
    }
}