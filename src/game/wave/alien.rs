use game::entity::*;

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
                    width: 30.0,
                    height: 20.0,
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
}

impl Entity for Alien {
    fn position(&mut self) -> &mut Position {
        &mut self.position
    }

    fn size(&mut self) -> &mut Size {
        &mut self.size
    }
}

impl Solid for Alien {}
impl Kinetic for Alien {}
