use game::WIDTH;
use game::position::Position;
use game::size::Size;

static VELOCITY: f64 = 500.0;

pub enum State {
    Idle,
    MovingRight,
    MovingLeft,
}

pub struct Canon {
    pub position: Position,
    pub size: Size,
    pub state: State,
}

impl Canon {
    pub fn new() -> Canon {
        Canon {
            position:
                Position {
                    x: WIDTH / 2.0,
                    y: 700.0
                },
            size:
                Size {
                    width: 60.0,
                    height: 32.0,
                },
            state: State::Idle,
        }
    }

    pub fn move_right(&mut self) {
            self.state = State::MovingRight;
    }

    pub fn move_left(&mut self) {
            self.state = State::MovingLeft;
    }

    pub fn idle(&mut self) {
        self.state = State::Idle;
    }

    pub fn update(&mut self, dt: f64) {
        match self.state {
            State::MovingRight => {
                if self.position.x+self.size.width/2.0 < WIDTH {
                    self.position.x += dt * VELOCITY;
                }
            }

            State::MovingLeft => {
                if self.position.x-self.size.width/2.0 > 0.0 {
                    self.position.x -= dt * VELOCITY;
                }
            }

            _ => {}
        }
    }
}
