pub mod alien;

use self::alien::*;
use game;
use game::entity::*;

const POSITION: Position =
    Position {
        x: 0.08 * game::WIDTH,
        y: 0.25 * game::HEIGHT,
    };

pub const COLUMNS: u32 = 11;
pub const ROWS: u32 = 5;

const WIDTH_GAP: f64 = 0.06666 * game::WIDTH;
const HEIGHT_GAP: f64 = 0.0375 * game::HEIGHT;

const STEPS: u32 = 14;
const STEP_DX: f64 = 0.0116666 * game::WIDTH;
const STEP_DY: f64 = 0.0125 * game::HEIGHT;

enum State {
    MovingRight(u32),
    MovingLeft(u32),
}

pub struct Wave {
    pub aliens: Vec<Vec<Alien>>,
    pub step: f64,
    timer: f64,
    state: State,
}

impl Wave {
    pub fn new() -> Wave {
        Wave {
            aliens: Wave::create_aliens(),
            step: 0.55,
            timer: 0.0,
            state: State::MovingRight(0),
        }
    }

    fn create_aliens() -> Vec<Vec<Alien>> {
        let mut aliens = Vec::new();

        for j in 0..COLUMNS {
            let mut column = Vec::new();
            for i in 0..ROWS {
                let position =
                    Position {
                        x: self::POSITION.x + (j as f64 * self::WIDTH_GAP),
                        y: self::POSITION.y + (i as f64 * self::HEIGHT_GAP),
                    };

                let alien =
                    match i {
                        0 =>
                            Alien::new(position, self::Kind::Alpha),

                        1 | 2 =>
                            Alien::new(position, self::Kind::Beta),

                        _ =>
                            Alien::new(position, self::Kind::Gamma),
                    };

                column.push(alien);
            }

            aliens.push(column);
        }

        return aliens;
    }

    pub fn len(&self) -> usize {
        let mut len: usize = 0;

        for column in &self.aliens {
            len += column.len();
        }

        len
    }

    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    pub fn clear(&mut self) {
        for column in &mut self.aliens {
            column.retain(|alien| {
                match alien.state {
                    alien::State::Dead => {
                        false
                    }

                    _ => {
                        true
                    }
                }
            });
        }

        self.aliens.retain(|column| {
            !column.is_empty()
        });
    }

    pub fn update(&mut self, dt: f64) {
        let ts = self.len() as f64 / (2 * ROWS * COLUMNS) as f64;
        self.step = ts + 0.05;

        self.timer += dt;

        if self.timer >= self.step {
            self.timer -= self.step;

            match self.state {
                State::MovingRight(i) if i < self::STEPS  => {
                    for column in &mut self.aliens {
                        for mut alien in column {
                            alien.move_x(STEP_DX);
                            alien.change_state();
                        }
                    }

                    self.state = State::MovingRight(i + 1);
                }

                State::MovingRight(_i) => {
                    for column in &mut self.aliens {
                        for mut alien in column {
                            alien.move_y(STEP_DY);
                            alien.change_state();
                        }
                    }

                    self.state = State::MovingLeft(0);
                }

                State::MovingLeft(i) if i < self::STEPS  => {
                    for column in &mut self.aliens {
                        for mut alien in column {
                            alien.move_x(-STEP_DX);
                            alien.change_state();
                        }
                    }

                    self.state = State::MovingLeft(i + 1);
                }

                State::MovingLeft(_i) => {
                    for column in &mut self.aliens {
                        for mut alien in column {
                            alien.move_y(STEP_DY);
                            alien.change_state();
                        }
                    }

                    self.state = State::MovingRight(0);
                }
            }
        }
    }
}
