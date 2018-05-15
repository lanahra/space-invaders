use game;
use game::entity::*;

const VELOCITY: f64 = 0.8333 * game::WIDTH;

const RIGHT_BOUND: f64 = 0.95 * game::WIDTH;
const LEFT_BOUND: f64 = 0.05 * game::WIDTH;

pub enum State {
    Idle,
    MovingRight,
    MovingLeft,
    Dead,
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
                    x: game::WIDTH / 2.0,
                    y: 0.875 * game::HEIGHT,
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
        match self.state {
            State::Dead => {}

            _ => {
                self.state = State::MovingRight;
            }
        }
    }

    pub fn move_left(&mut self) {
        match self.state {
            State::Dead => {}

            _ => {
                self.state = State::MovingLeft;
            }
        }
    }

    pub fn idle(&mut self) {
        match self.state {
            State::Dead => {}

            _ => {
                self.state = State::Idle;
            }
        }
    }

    pub fn update(&mut self, dt: f64) {
        match self.state {
            State::MovingRight => {
                if self.position.x + self.size.width / 2.0 < RIGHT_BOUND {
                    self.move_x(dt * VELOCITY);
                }
            }

            State::MovingLeft => {
                if self.position.x - self.size.width / 2.0 > LEFT_BOUND {
                    self.move_x(-dt * VELOCITY);
                }
            }

            _ => {}
        }
    }
}

impl Entity for Canon {
    fn position(&mut self) -> &mut Position {
        &mut self.position
    }

    fn size(&mut self) -> &mut Size {
        &mut self.size
    }
}

impl Solid for Canon {}
impl Kinetic for Canon {}
