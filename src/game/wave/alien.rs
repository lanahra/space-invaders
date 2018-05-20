use game::entity::*;

#[derive(Copy, Clone)]
pub enum Kind {
    Alpha,
    Beta,
    Gamma,
}

#[derive(Copy, Clone)]
pub enum State {
    ArmsUp,
    ArmsDown,
    Dead,
}

#[derive(Copy, Clone)]
pub struct Alien {
    pub position: Position,
    pub size: Size,
    pub kind: Kind,
    pub state: State,
}

impl Alien {
    pub fn new(position: Position, kind: Kind) -> Alien {
        let size =
            match kind {
                Kind::Alpha => {
                    Size {
                        width: 35.0,
                        height: 35.0,
                    }
                }

                Kind::Beta => {
                    Size {
                        width: 48.0,
                        height: 35.0,
                    }
                }

                Kind::Gamma => {
                    Size {
                        width: 52.0,
                        height: 35.0,
                    }
                }
            };

        Alien {
            position,
            kind,
            state: State::ArmsUp,
            size,
        }
    }

    pub fn change_state(alien: Alien) -> Alien{
        match alien.state {
            State::ArmsUp => {
                Alien {
                    state: State::ArmsDown,
                    ..alien
                }
            }

            State::ArmsDown => {
                Alien {
                    state: State::ArmsUp,
                    ..alien
                }
            }

            _ => {
                alien
            }
        }
    }
}

impl Entity for Alien {
    fn position(entity: &Self) -> Position {
        entity.position
    }

    fn size(entity: &Self) -> Size {
        entity.size
    }
}

impl Solid for Alien {}

impl Kinetic for Alien {
    fn move_x(dx: f64, entity: Self) -> Self {
        Alien {
            position:
                Position {
                    x: entity.position.x + dx,
                    y: entity.position.y,
                },
            ..entity
        }
    }

    fn move_y(dy: f64, entity: Self) -> Self {
        Alien {
            position:
                Position {
                    x: entity.position.x,
                    y: entity.position.y + dy,
                },
            ..entity
        }
    }
}
