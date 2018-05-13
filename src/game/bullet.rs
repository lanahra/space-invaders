use game;
use game::position::Position;
use game::size::Size;
use game::collision::Collision;

const VELOCITY: f64 = 1.3 * game::WIDTH;

#[derive(Clone)]
pub enum State {
    MovingUp,
    MovingDown,
}

#[derive(Clone)]
pub struct Bullet {
    pub position: Position,
    pub size: Size,
    pub state: State,
}

impl Bullet {
    pub fn new(position: Position, state: State) -> Bullet {
        Bullet {
            position,
            size:
                Size {
                    width: 2.0,
                    height: 17.0,
                },
            state,
        }
    }

    pub fn update(&mut self, dt: f64) {
        match self.state {
            State::MovingUp => {
                self.move_y(dt * -VELOCITY);
            }

            State::MovingDown => {
                self.move_y(dt * VELOCITY);
            }
        }
    }

    fn move_y(&mut self, dy: f64) {
        self.position.y += dy;
    }
}

impl Collision for Bullet {
    fn position(&self) -> &Position {
        &self.position
    }

    fn size(&self) -> &Size {
        &self.size
    }
}
