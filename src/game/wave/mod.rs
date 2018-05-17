pub mod alien;

use self::alien::*;
use game::entity::*;

const POSITION: Position =
    Position {
        x: 75.0,
        y: 270.0,
    };

pub const COLUMNS: u32 = 11;
pub const ROWS: u32 = 5;

const WIDTH_GAP: f64 = 70.0;
const HEIGHT_GAP: f64 = 64.0;

const STEP_DX: f64 = 10.0;
const STEP_DY: f64 = 10.0;

enum State {
    MovingRight,
    MovingLeft,
    MovingDownLeft,
    MovingDownRight,
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
            state: State::MovingRight,
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
                State::MovingRight => {
                    for column in &mut self.aliens {
                        for mut alien in column {
                            alien.move_x(STEP_DX);
                            alien.change_state();
                        }
                    }

                    if let Some(column) = self.aliens.last() {
                        if let Some(alien) = column.last() {
                            if alien.position.x > 865.0 {
                                self.state = State::MovingDownLeft;
                            }
                        }
                    }
                }

                State::MovingLeft => {
                    for column in &mut self.aliens {
                        for mut alien in column {
                            alien.move_x(-STEP_DX);
                            alien.change_state();
                        }
                    }

                    if let Some(column) = self.aliens.first() {
                        if let Some(alien) = column.first() {
                            if alien.position.x < 80.0 {
                                self.state = State::MovingDownRight;
                            }
                        }
                    }
                }

                State::MovingDownLeft => {
                    for column in &mut self.aliens {
                        for mut alien in column {
                            alien.move_y(STEP_DY);
                            alien.change_state();
                        }
                    }

                    self.state = State::MovingLeft;
                }

                State::MovingDownRight => {
                    for column in &mut self.aliens {
                        for mut alien in column {
                            alien.move_y(STEP_DY);
                            alien.change_state();
                        }
                    }

                    self.state = State::MovingRight;
                }
            }
        }
    }
}
