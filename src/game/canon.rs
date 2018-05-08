use game::WIDTH;
use game::HEIGHT;
use game::position::Position;
use game::size::Size;
use game::collision::Collision;

const VELOCITY: f64 = 0.8333 * WIDTH;

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
                    y: 0.875*HEIGHT
                },
            size:
                Size {
                    width: 0.1*WIDTH,
                    height: 0.04375*HEIGHT,
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

impl Collision for Canon {
    fn position(&self) -> &Position {
        &self.position
    }

    fn size(&self) -> &Size {
        &self.size
    }
}
