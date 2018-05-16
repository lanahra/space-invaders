use game::entity::*;

pub const WIDTH: f64 = 32.0;
pub const HEIGHT: f64 = 23.0;

pub enum State {
    Full,
    Half,
    Weak,
    Dead,
}

pub enum Kind {
    Normal,
    BottomLeft,
    BottomRight,
    TopLeft,
    TopRight,
}

pub struct Block {
    pub position: Position,
    pub kind: Kind,
    pub state: State,
    pub size: Size,
}

impl Block {
    pub fn new(position: Position, kind: Kind) -> Block {
        Block {
            position,
            kind,
            state: State::Full,
            size:
            Size {
                width: WIDTH,
                height: HEIGHT,
            },
        }
    }

    pub fn change_state(&mut self) {
        match self.state {
            State::Full => {
                self.state = State::Half;
            }

            State::Half => {
                self.state = State::Weak;
            }

            State::Weak => {
                self.state = State::Dead;
            }

            State::Dead => {}
        }
    }
}

impl Entity for Block {
    fn position(&mut self) -> &mut Position {
        &mut self.position
    }

    fn size(&mut self) -> &mut Size {
        &mut self.size
    }
}

impl Solid for Block {}
