extern crate rand;

pub mod alien;
pub mod column;
pub mod red_alien;

use self::column::*;
use alien::*;
use red_alien::*;
use game::position::Position;
use game::WIDTH;
use game::HEIGHT;
use std::collections::LinkedList;
use std::collections::linked_list::Iter;
use std::collections::linked_list::IterMut;
use game::entity::Entity;
use game::entity::active::Active;
use self::rand::Rng;

const POSITION: Position =
    Position {
        x: 0.083333 * WIDTH,
        y: 0.2875 * HEIGHT,
    };

const RED_ALIEN_POSITION: Position =
    Position {
        x: 1.3 * WIDTH,
        y: 0.11 * HEIGHT,
    };

const STEP_DECREASE: f64 = 0.1;

pub const COLUMNS: u32 = 11;

const WIDTH_GAP: f64 = 0.06666 * WIDTH;

const STEPS: u32 = 14;
const STEP_DX: f64 = 0.0116666 * WIDTH;
const STEP_DY: f64 = 0.0125 * HEIGHT;

const RED_ALIEN_PROBABILITY: i64 = 3000;
pub const ANSWER_TO_THE_ULTIMATE_QUESTION_OF_LIFE_THE_UNIVERSE_AND_EVERYTHING: i64 = 42; //Aww, it has 85 characters :(

enum State {
    MovingRight(u32),
    MovingLeft(u32),
}

pub struct Wave {
    pub columns: LinkedList<Column>,
    pub red_alien: RedAlien,
    pub step: f64,
    pub deadly_counter: u32,
    timer: f64,
    state: State,
    pub level: i64,
}

impl Wave {
    pub fn new() -> Wave {
        Wave {
            columns: Wave::create_columns(),
            red_alien: RedAlien::new(self::RED_ALIEN_POSITION),
            step: 0.8,
            deadly_counter: 0,
            timer: 0.0,
            state: State::MovingRight(0),
            level: 1,
        }
    }

    fn create_columns() -> LinkedList<Column> {
        let mut columns: LinkedList<Column> = LinkedList::new();
        for i in 0..COLUMNS {
            let position =
                Position {
                    x: self::POSITION.x + (i as f64 * self::WIDTH_GAP),
                    y: self::POSITION.y,
                };
            let column =
                match i {
                    0 =>
                        Column::new(position, self::Kind::Alpha),

                    1 | 2 =>
                        Column::new(position, self::Kind::Beta),

                    _ =>
                        Column::new(position, self::Kind::Gamma),
                };

            columns.push_back(column);
        }

        return columns;
    }

    pub fn kill_alien(&mut self) {
        self.deadly_counter+=1;
    }

    pub fn reset_wave(&mut self) {
        self.columns = Wave::create_columns();
        self.deadly_counter = 0;
        self.step -= STEP_DECREASE;
        self.timer = 0.0;
        self.state = State::MovingRight(0);
        self.level += 1;
    }

    fn generate_red_alien(&mut self, dt: f64) {
        if !self.red_alien.is_active() {
            let mut rng = rand::thread_rng();
            let num: i64 = rng.gen_range(0, RED_ALIEN_PROBABILITY);

            if num == ANSWER_TO_THE_ULTIMATE_QUESTION_OF_LIFE_THE_UNIVERSE_AND_EVERYTHING {
                self.red_alien.move_right();
                self.red_alien.change_state();
            }
            else if num == RED_ALIEN_PROBABILITY-ANSWER_TO_THE_ULTIMATE_QUESTION_OF_LIFE_THE_UNIVERSE_AND_EVERYTHING {
                self.red_alien.move_left();
                self.red_alien.change_state();
            }
        }
        else {
            self.red_alien.update(dt);
        }
    }


    pub fn update(&mut self, dt: f64) {
        self.timer += dt;

        self.generate_red_alien(dt);

        if self.timer >= self.step {
            self.timer -= self.step;

            match self.state {
                State::MovingRight(i) if i < self::STEPS  => {
                    for column in self.iter_mut() {
                        for alien in column.iter_mut() {
                            alien.change_state();
                            alien.move_x(STEP_DX );
                        }
                    }

                    self.state = State::MovingRight(i + 1);
                }

                State::MovingRight(i) => {
                    for column in self.iter_mut() {
                        for alien in column.iter_mut() {
                            alien.change_state();
                            alien.move_y(STEP_DY);
                        }
                    }

                    self.state = State::MovingLeft(0);
                }

                State::MovingLeft(i) if i < self::STEPS  => {
                    for column in self.iter_mut() {
                        for alien in column.iter_mut() {
                            alien.change_state();
                            alien.move_x(-STEP_DX);
                        }
                    }

                    self.state = State::MovingLeft(i + 1);
                }

                State::MovingLeft(i) => {
                    for column in self.iter_mut() {
                        for alien in column.iter_mut() {
                            alien.change_state();
                            alien.move_y(STEP_DY);
                        }
                    }

                    self.state = State::MovingRight(0);
                }
            }
        }
    }

    pub fn iter(&self) -> Iter<Column> {
        self.columns.iter()
    }

    pub fn iter_mut(&mut self) -> IterMut<Column> {
        self.columns.iter_mut()
    }
}
