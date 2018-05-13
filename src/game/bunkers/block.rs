use game::position::Position;
use game::size::Size;
use game::collision::Collision;

pub const WIDTH: f64 = 15.0;
pub const HEIGHT: f64 = 15.0;

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

impl Collision for Block {
    fn position(&self) -> &Position {
        &self.position
    }

    fn size(&self) -> &Size {
        &self.size
    }
}
