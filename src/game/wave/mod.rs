pub mod alien;

use alien::*;
use game::position::Position;
use game::WIDTH;
use game::HEIGHT;
use std::collections::LinkedList;
use std::collections::linked_list::Iter;
use std::collections::linked_list::IterMut;

const POSITION: Position =
    Position {
        x: 0.083333 * WIDTH,
        y: 0.075 * HEIGHT,
    };

const COLUMNS: u32 = 11;
const ROWS: u32 = 5;

const WIDTH_GAP: f64 = 0.06666 * WIDTH;
const HEIGHT_GAP: f64 = 0.0375 * HEIGHT;

const STEPS: u32 = 14;
const STEP_DX: f64 = 0.0116666 * WIDTH;
const STEP_DY: f64 = 0.0125 * HEIGHT;

enum State {
    MovingRight(u32),
    MovingLeft(u32),
}

pub struct Wave {
    pub aliens: LinkedList<Alien>,
    pub step: f64,
    timer: f64,
    state: State,
}

impl Wave {
    pub fn new() -> Wave {
        Wave {
            aliens: Wave::create_aliens(),
            step: 1.0,
            timer: 0.0,
            state: State::MovingRight(0),
        }
    }

    fn create_aliens() -> LinkedList<Alien> {
        let mut aliens: LinkedList<Alien> = LinkedList::new();

        for i in 0..ROWS {
            for j in 0..COLUMNS {
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

                aliens.push_back(alien);
            }
        }

        return aliens;
    }

    pub fn update(&mut self, dt: f64) {
        self.timer += dt;

        if self.timer >= self.step {
            self.timer -= self.step;

            match self.state {
                State::MovingRight(i) if i < self::STEPS  => {
                    for alien in self.aliens.iter_mut() {
                        alien.change_state();
                        alien.move_x(STEP_DX );
                    }

                    self.state = State::MovingRight(i + 1);
                }

                State::MovingRight(i) => {
                    for alien in self.aliens.iter_mut() {
                        alien.change_state();
                        alien.move_y(STEP_DY);
                    }

                    self.state = State::MovingLeft(0);
                }

                State::MovingLeft(i) if i < self::STEPS  => {
                    for alien in self.aliens.iter_mut() {
                        alien.change_state();
                        alien.move_x(-STEP_DX);
                    }

                    self.state = State::MovingLeft(i + 1);
                }

                State::MovingLeft(i) => {
                    for alien in self.aliens.iter_mut() {
                        alien.change_state();
                        alien.move_y(STEP_DY);
                    }

                    self.state = State::MovingRight(0);
                }
            }
        }
    }

    pub fn iter(&self) -> Iter<Alien> {
        self.aliens.iter()
    }

    pub fn iter_mut(&mut self) -> IterMut<Alien> {
        self.aliens.iter_mut()
    }
}
