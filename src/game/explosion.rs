use game::entity::*;
use std::vec::Vec;

const LIFETIME: f64 = 0.1;

#[derive(Copy, Clone)]
enum State {
    Active,
    Inactive,
}

#[derive(Copy, Clone)]
pub struct Explosion {
    pub position: Position,
    pub size: Size,
    state: State,
    time: f64,
}

impl Explosion {
    pub fn new(position: Position) -> Explosion {
        Explosion {
            position,
            size:
                Size {
                    width: 30.0,
                    height: 20.0,
                },
            state: State::Active,
            time: 0.0,
        }
    }

    fn update_time(dt: f64, explosion: Explosion) -> Explosion {
        Explosion {
            time: explosion.time + dt,
            ..explosion
        }
    }

    pub fn update(
        dt: f64,
        explosions: Vec<Explosion>) -> Vec<Explosion> {

        let e = explosions.into_iter().map(|e| {
            let e = Explosion::update_time(dt, e);

            if e.time > LIFETIME {
                Explosion {
                    state: State::Inactive,
                    ..e
                }
            } else {
                e
            }
        }).collect();

        Explosion::clear(e)
    }

    fn clear(explosions: Vec<Explosion>) -> Vec<Explosion> {
        explosions.into_iter().filter(|e| {
            match e.state {
                State::Active => {
                    true
                }

                State::Inactive => {
                    false
                }
            }
        }).collect()
    }
}
