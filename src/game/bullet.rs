use game::entity::*;

const VELOCITY: f64 = 700.0;

#[derive(Copy, Clone)]
pub enum State {
    MovingUp,
    MovingDown,
}

#[derive(Copy, Clone)]
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

    pub fn update(dt: f64, bullet: Bullet) -> Bullet {
        match bullet.state {
            State::MovingUp => {
                Bullet::move_y(dt * -VELOCITY, bullet)
            }

            State::MovingDown => {
                Bullet::move_y(dt * VELOCITY, bullet)
            }
        }
    }

    fn move_y(dy: f64, bullet: Bullet) -> Bullet {
        Bullet {
            position:
                Position {
                    x: bullet.position.x,
                    y: bullet.position.y + dy,
                },
            ..bullet
        }
    }
}

impl Entity for Bullet {
    fn position(entity: &Self) -> Position {
        entity.position
    }

    fn size(entity: &Self) -> Size {
        entity.size
    }
}

impl Solid for Bullet {}

impl Kinetic for Bullet {
    fn move_x(dx: f64, entity: Self) -> Self {
        Bullet {
            position:
                Position {
                    x: entity.position.x + dx,
                    y: entity.position.y,
                },
            ..entity
        }
    }

    fn move_y(dy: f64, entity: Self) -> Self {
        Bullet {
            position:
                Position {
                    x: entity.position.x,
                    y: entity.position.y + dy,
                },
            ..entity
        }
    }
}
