use game::position::Position;
use game::size::Size;
use game::WIDTH;
use game::HEIGHT;
use game::collision::Collision;

pub enum Kind {
    Alpha,
    Beta,
    Gamma,
}

pub enum State {
    ArmsUp,
    ArmsDown,
    Dead,
    Inactive
}

pub struct Alien {
    pub position: Position,
    pub kind: Kind,
    pub state: State,
    pub size: Size,
}

impl Alien {
    pub fn new(position: Position, kind: Kind) -> Alien {
        Alien {
            position,
            kind,
            state: State::ArmsUp,
            size:
                Size {
                    width: 0.05 * WIDTH,
                    height: 0.025 * HEIGHT,
                },
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

            State::Dead => {
                self.state = State::Inactive;
            }

            _ => {}
        }
    }

    pub fn is_active(&self) -> bool {
        match self.state {
            State::Inactive => {
                return false;
            }

            _ => {
                return true;
            }
        }
    }

    pub fn shot_hit(&mut self) {
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
