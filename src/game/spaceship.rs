use game::entity::*;

const VELOCITY: f64 = 250.0;

#[derive(Clone)]
pub enum State {
    MovingLeft,
    MovingRight,
}

#[derive(Clone)]
pub struct Spaceship {
    pub position: Position,
    pub size: Size,
    pub state: State,
}

impl Spaceship {
    pub fn new(position: Position, state: State) -> Spaceship {
        Spaceship {
            position,
            size:
                Size {
                    width: 62.0,
                    height: 28.0,
                },
            state,
        }
    }

    pub fn update(&mut self, dt: f64) {
        match self.state {
            State::MovingLeft => {
                self.move_x(dt * -VELOCITY);
            }

            State::MovingRight => {
                self.move_x(dt * VELOCITY);
            }
        }
    }
}

impl Entity for Spaceship {
    fn position(&mut self) -> &mut Position {
        &mut self.position
    }

    fn size(&mut self) -> &mut Size {
        &mut self.size
    }
}

impl Solid for Spaceship {}
impl Kinetic for Spaceship {}
