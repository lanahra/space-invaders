use game::position::Position;
use game::size::Size;

pub enum Kind {
    Alpha,
    Beta,
    Gamma,
}

pub enum State {
    ArmsUp,
    ArmsDown,
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
                    width: 30.0,
                    height: 20.0,
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
        }
    }

    pub fn move_x(&mut self, dx: f64) {
        self.position.x += dx;
    }

    pub fn move_y(&mut self, dy: f64) {
        self.position.y += dy;
    }
}
