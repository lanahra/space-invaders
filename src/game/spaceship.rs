use game::entity::*;

const VELOCITY: f64 = 250.0;

#[derive(Copy, Clone)]
pub enum State {
    MovingLeft,
    MovingRight,
}

#[derive(Copy, Clone)]
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

    pub fn update(dt: f64, spaceship: Spaceship) -> Spaceship {
        match spaceship.state {
            State::MovingLeft => {
                Spaceship::move_x(dt * -VELOCITY, spaceship)
            }

            State::MovingRight => {
                Spaceship::move_x(dt * VELOCITY, spaceship)
            }
        }
    }
}

impl Entity for Spaceship {
    fn position(entity: &Self) -> Position {
        entity.position
    }

    fn size(entity: &Self) -> Size {
        entity.size
    }
}

impl Solid for Spaceship {}

impl Kinetic for Spaceship {
    fn move_x(dx: f64, entity: Self) -> Self {
        Spaceship {
            position:
                Position {
                    x: entity.position.x + dx,
                    y: entity.position.y,
                },
            ..entity
        }
    }

    fn move_y(dy: f64, entity: Self) -> Self {
        Spaceship {
            position:
                Position {
                    x: entity.position.x,
                    y: entity.position.y + dy,
                },
            ..entity
        }
    }
}
