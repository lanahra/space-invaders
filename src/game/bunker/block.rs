use game::entity::*;

pub const WIDTH: f64 = 32.0;
pub const HEIGHT: f64 = 23.0;

#[derive(Copy, Clone)]
pub enum State {
    Full,
    Half,
    Weak,
    Dead,
}

#[derive(Copy, Clone)]
pub enum Kind {
    Normal,
    BottomLeft,
    BottomRight,
    TopLeft,
    TopRight,
}

#[derive(Copy, Clone)]
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

    pub fn change_state(block: Block) -> Block {
        match block.state {
            State::Full => {
                Block {
                    state: State::Half,
                    ..block
                }
            }

            State::Half => {
                Block {
                    state: State::Weak,
                    ..block
                }
            }

            State::Weak => {
                Block {
                    state: State::Dead,
                    ..block
                }
            }

            State::Dead => {
                block
            }
        }
    }
}

impl Entity for Block {
    fn position(entity: &Self) -> Position {
        entity.position
    }

    fn size(entity: &Self) -> Size {
        entity.size
    }
}

impl Solid for Block {}
