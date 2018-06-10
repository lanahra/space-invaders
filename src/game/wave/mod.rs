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

#[derive(Clone)]
pub enum State {
    MovingRight,
    MovingLeft,
    MovingDownLeft,
    MovingDownRight,
}

#[derive(Clone)]
pub struct Wave {
    pub aliens: Vec<Vec<Alien>>,
    pub step: f64,
    pub time: f64,
    pub state: State,
}

impl Wave {
    pub fn new() -> Wave {
        Wave {
            aliens: Wave::new_aliens(),
            step: 0.55,
            time: 0.0,
            state: State::MovingRight,
        }
    }

    fn new_aliens() -> Vec<Vec<Alien>> {
        let new_alien = |pos| {
            let (j, i): (u32, u32) = pos;
            let position =
                Position {
                    x: self::POSITION.x + (j as f64 * self::WIDTH_GAP),
                    y: self::POSITION.y + (i as f64 * self::HEIGHT_GAP),
                };

            match i {
                0 => {
                    Alien::new(position, self::Kind::Alpha)
                }

                1 | 2 => {
                    Alien::new(position, self::Kind::Beta)
                }

                _ => {
                    Alien::new(position, self::Kind::Gamma)
                }
            }
        };

        struct New<'a> {
            rows: &'a Fn(&New, u32, Vec<u32>, Vec<Alien>) -> Vec<Alien>,
            columns: &'a Fn(&New, Vec<u32>, Vec<Vec<Alien>>) -> Vec<Vec<Alien>>,
        }

        let new =
            New {
                rows: &|new, j, position, rows| {
                    if position.len() >= 1 {
                        let (head, tail) = position.split_at(1);

                        let pos = (j, head[0]);
                        let alien = new_alien(pos);
                        let mut rows = rows.to_vec();
                        rows.push(alien);

                        (new.rows)(new, j, tail.to_vec(), rows)
                    } else {
                        rows
                    }
                },

                columns: &|new, positions, columns| {
                    let rows = (0..ROWS).collect();
                    if positions.len() >= 1 {
                        let (head, tail) = positions.split_at(1);

                        let rows = (new.rows)(new, head[0], rows, Vec::new());
                        let mut columns = columns.to_vec();
                        columns.push(rows);

                        (new.columns)(new, tail.to_vec(), columns)
                    } else {
                        columns
                    }
                },
            };

        let columns = (0..COLUMNS).collect();
        (new.columns)(&new, columns, Vec::new())
    }

    pub fn len(wave: Wave) -> usize {
        struct Count<'a> {
            f: &'a Fn(&Count, Vec<Vec<Alien>>) -> usize,
        }

        let count =
            Count {
                f: &|count, wave| {
                    if wave.len() >= 1 {
                        let (head, tail) = wave.split_at(1);

                        head[0].len() + (count.f)(count, tail.to_vec())
                    } else {
                        0
                    }
                },
            };

        (count.f)(&count, wave.aliens)
    }

    fn update_step(wave: Wave) -> Wave {
        let ts = Wave::len(wave.clone()) as f64 / (2 * ROWS * COLUMNS) as f64;

        Wave {
            step: ts + 0.05,
            ..wave
        }
    }

    fn update_time(dt: f64, wave: Wave) -> Wave {
        Wave {
            time: wave.time + dt,
            ..wave
        }
    }

    fn move_alien(dx: f64, dy: f64) -> impl Fn(Alien) -> Alien {
        move |alien| {
            let alien = Alien::move_x(dx, alien);
            Alien::move_y(dy, alien)
        }
    }

    pub fn update(dt: f64, wave: Wave) -> Wave {
        let wave = Wave::update_step(wave);
        let wave = Wave::update_time(dt, wave);

        let move_right = Wave::move_alien(STEP_DX, 0.0);
        let move_left = Wave::move_alien(-STEP_DX, 0.0);
        let move_down = Wave::move_alien(0.0, STEP_DY);

        if wave.time >= wave.step {
            let wave = Wave::update_time(-wave.step, wave);

            match wave.state {
                State::MovingRight => {
                    let aliens: Vec<Vec<Alien>> =
                        wave.aliens.iter().map(|column| {
                            column.iter().map(|&alien| {
                                let alien = move_right(alien);
                                Alien::change_state(alien)
                            }).collect()
                        }).collect();

                    if let Some(column) = aliens.clone().last() {
                        if let Some(alien) = column.last() {
                            if alien.position.x > 865.0 {
                                return
                                    Wave {
                                        aliens,
                                        state: State::MovingDownLeft,
                                        ..wave
                                    };
                            } else {
                                return
                                    Wave {
                                        aliens,
                                        ..wave
                                    };
                            }
                        }
                    }

                    return wave;
                }

                State::MovingLeft => {
                    let aliens: Vec<Vec<Alien>> =
                        wave.aliens.iter().map(|column| {
                            column.iter().map(|&alien| {
                                let alien = move_left(alien);
                                Alien::change_state(alien)
                            }).collect()
                        }).collect();

                    if let Some(column) = aliens.clone().first() {
                        if let Some(alien) = column.first() {
                            if alien.position.x < 80.0 {
                                return
                                    Wave {
                                        aliens,
                                        state: State::MovingDownRight,
                                        ..wave
                                    };
                            } else {
                                return
                                    Wave {
                                        aliens,
                                        ..wave
                                    };
                            }
                        }
                    }

                    return wave;
                }

                State::MovingDownLeft => {
                    let aliens: Vec<Vec<Alien>> =
                        wave.aliens.iter().map(|column| {
                            column.iter().map(|&alien| {
                                let alien = move_down(alien);
                                Alien::change_state(alien)
                            }).collect()
                        }).collect();

                    Wave {
                        aliens,
                        state: State::MovingLeft,
                        ..wave
                    }

                }

                State::MovingDownRight => {
                    let aliens: Vec<Vec<Alien>> =
                        wave.aliens.iter().map(|column| {
                            column.iter().map(|&alien| {
                                let alien = Alien::move_y(STEP_DY, alien);
                                Alien::change_state(alien)
                            }).collect()
                        }).collect();

                    Wave {
                        aliens,
                        state: State::MovingRight,
                        ..wave
                    }
                }
            }
        } else {
            wave
        }
    }
}
