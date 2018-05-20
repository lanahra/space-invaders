use game;
use game::entity::*;

const VELOCITY: f64 = 800.0;

const RIGHT_BOUND: f64 = 900.0;
const LEFT_BOUND: f64 = 45.0;

#[derive(Copy, Clone)]
pub enum State {
    Idle,
    MovingRight,
    MovingLeft,
    Dead,
}

#[derive(Copy, Clone)]
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
                    y: 906.0,
                },
            size:
                Size {
                    width: 57.0,
                    height: 35.0,
                },
            state: State::Idle,
        }
    }

    pub fn move_right(canon: Canon) -> Canon {
        match canon.state {
            State::Dead => {
                canon
            }

            _ => {
                Canon {
                    state: State::MovingRight,
                    ..canon
                }
            }
        }
    }

    pub fn move_left(canon: Canon) -> Canon {
        match canon.state {
            State::Dead => {
                canon
            }

            _ => {
                Canon {
                    state: State::MovingLeft,
                    ..canon
                }
            }
        }
    }

    pub fn idle(canon: Canon) -> Canon {
        match canon.state {
            State::Dead => {
                canon
            }

            _ => {
                Canon {
                    state: State::Idle,
                    ..canon
                }
            }
        }
    }

    pub fn update(dt: f64, canon: Canon) -> Canon {
        match canon.state {
            State::MovingRight => {
                if canon.position.x + canon.size.width / 2.0 < RIGHT_BOUND {
                    Canon::move_x(dt * VELOCITY, canon)
                } else {
                    canon
                }
            }

            State::MovingLeft => {
                if canon.position.x - canon.size.width / 2.0 > LEFT_BOUND {
                    Canon::move_x(dt * -VELOCITY, canon)
                } else {
                    canon
                }
            }

            _ => {
                canon
            }
        }
    }
}

impl Entity for Canon {
    fn position(entity: &Self) -> Position {
        entity.position
    }

    fn size(entity: &Self) -> Size {
        entity.size
    }
}

impl Solid for Canon {}

impl Kinetic for Canon {
    fn move_x(dx: f64, entity: Self) -> Self {
        Canon {
            position:
                Position {
                    x: entity.position.x + dx,
                    y: entity.position.y,
                },
            ..entity
        }
    }

    fn move_y(dy: f64, entity: Self) -> Self {
        Canon {
            position:
                Position {
                    x: entity.position.x,
                    y: entity.position.y + dy,
                },
            ..entity
        }
    }
}
