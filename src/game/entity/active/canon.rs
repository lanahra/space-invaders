use game::WIDTH;
use game::HEIGHT;
use game::position::Position;
use game::size::Size;
use game::collision::Collision;
use game::entity::Entity;
use game::entity::active::Active;

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
}

impl Collision for Canon {
    fn position(&self) -> &Position {
        &self.position
    }

    fn size(&self) -> &Size {
        &self.size
    }
}

impl Entity for Canon {
    fn is_active(&self) -> bool {
        return true;
    }

    fn shot_hit(&mut self) {}

    fn change_state(&mut self) {}
}

impl Active for Canon {
    fn position(&mut self) -> &mut Position {
        &mut self.position
    }

    fn update(&mut self, dt: f64) {
        match self.state {
            State::MovingRight => {
                if self.position.x+self.size.width/2.0 < WIDTH {
                    self.move_x(dt * VELOCITY);
                }
            }

            State::MovingLeft => {
                if self.position.x-self.size.width/2.0 > 0.0 {
                    self.move_x(-dt * VELOCITY);
                }
            }

            _ => {}
        }
    }
}
