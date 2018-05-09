use game::position::Position;
use game::size::Size;
use game::WIDTH;
use game::HEIGHT;
use game::collision::Collision;

pub enum State {
    Strong,
    HalfLife,
    Weak,
    Inactive
}

pub enum Kind {
    TopLeftCorner,
    TopRightCorner,
    BottomLeftCorner,
    BottomRightCorner,
    Normal
}

pub struct Block {
    pub position: Position,
    pub kind: Kind,
    pub state: State,
    pub size: Size,
}

impl Block {
    pub fn new(position: Position) -> Block {
        Block {
            position,
            kind: Kind::Normal,
            state: State::Strong,
            size:
            Size {
                width: 0.026666 * WIDTH,
                height: 0.01875 * HEIGHT,
            },
        }
    }

    pub fn change_to_top_left(&mut self) {
        self.kind = Kind::TopLeftCorner;
    }

    pub fn change_to_top_right(&mut self) {
        self.kind = Kind::TopRightCorner;
    }

    pub fn change_to_bottom_left(&mut self) {
        self.kind = Kind::BottomLeftCorner;
    }

    pub fn change_to_bottom_right(&mut self) {
        self.kind = Kind::BottomRightCorner;
    }

    pub fn shot_hit(&mut self) {
        match self.state {
            State::Strong => {
                self.state = State::HalfLife;
            }

            State::HalfLife => {
                self.state = State::Weak;
            }

            State::Weak => {
                self.state = State::Inactive;
            }

            State::Inactive => {}
        }
    }

    pub fn is_active(&self) -> bool {
        match self.state {
            State::Inactive => {
                return false;
            }

            _ => {
                return true;
            }
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
