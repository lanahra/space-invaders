use game::entity::*;

const VELOCITY: f64 = 700.0;

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
                    width: 5.0,
                    height: 18.0,
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
}

impl Entity for Bullet {
    fn position(&mut self) -> &mut Position {
        &mut self.position
    }

    fn size(&mut self) -> &mut Size {
        &mut self.size
    }
}

impl Solid for Bullet {}
impl Kinetic for Bullet {}
