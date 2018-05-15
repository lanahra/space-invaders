use game::entity::*;
use std::vec::Vec;

const LIFETIME: f64 = 0.1;

enum State {
    Active,
    Inactive,
}

pub struct Explosion {
    pub position: Position,
    pub size: Size,
    state: State,
    time: f64,
}

pub struct Explosions {
    pub explosions: Vec<Explosion>,
}

impl Explosions {
    pub fn new() -> Explosions {
        Explosions {
            explosions: Vec::new(),
        }
    }

    fn clear(&mut self) {
        self.explosions.retain(|e| {
            match e.state {
                State::Active => {
                    true
                }

                State::Inactive => {
                    false
                }
            }
        });
    }

    pub fn add(&mut self, position: Position) {
        let e =
            Explosion {
                position,
                size:
                    Size {
                        width: 30.0,
                        height: 20.0,
                    },
                state: State::Active,
                time: 0.0,
            };

        self.explosions.push(e);
    }

    pub fn update(&mut self, dt: f64) {
        for explosions in &mut self.explosions {
            explosions.time += dt;

            if explosions.time > LIFETIME {
                explosions.state = State::Inactive;
            }
        }

        self.clear();
    }
}
