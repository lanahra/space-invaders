use game;
use game::position::Position;
use game::size::Size;
use game::collision::Collision;
use game::entity::Entity;
use game::entity::active::Active;

pub enum Kind {
    Alpha,
    Beta,
    Gamma,
}

pub enum State {
    ArmsUp,
    ArmsDown,
    Dead,
}

pub struct Alien {
    pub position: Position,
    pub size: Size,
    pub kind: Kind,
    pub state: State,
}

impl Alien {
    pub fn new(position: Position, kind: Kind) -> Alien {
        Alien {
            position,
            kind,
            state: State::ArmsUp,
            size:
                Size {
                    width: 0.05 * game::WIDTH,
                    height: 0.025 * game::HEIGHT,
                }
        }
    }

    pub fn change_state(&mut self) {
        match self.state {
            State::ArmsUp => {
                self.state = State::ArmsDown;
            }

            State::ArmsDown => {
                self.state = State::ArmsUp;
            }

            _ => {}
        }
    }

    pub fn kill(&mut self) {
        self.state = State::Dead;
    }

    pub fn move_x(&mut self, dx: f64) {
        self.position.x += dx;
    }

    pub fn move_y(&mut self, dy: f64) {
        self.position.y += dy;
    }
}

impl Collision for Alien {
    fn position(&self) -> &Position {
        &self.position
    }

    fn size(&self) -> &Size {
        &self.size
    }
}
