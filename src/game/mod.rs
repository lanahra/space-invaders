mod bullet;
mod entity;
mod explosions;
mod spaceship;
pub mod bunkers;
pub mod canon;
pub mod wave;

use game::entity::*;
use rand::{Rng, thread_rng};
use std::vec::Vec;

pub const WIDTH: f64 = 945.0;
pub const HEIGHT: f64 = 1080.0;

const RAND_STEP: f64 = 0.3;
const PAUSED_STEP: f64 = 2.0;
const MYSTERY_STEP: f64 = 10.0;

pub enum Selection {
    NewGame,
    Records,
    Exit,
}

pub enum State {
    Menu(Selection),
    Records,
    Running,
    Paused,
    Over,
    Restart,
    Exit,
}

pub struct Info {
    pub score: u32,
    pub canons: u32,
    pub state: State,
    rand_time: f64,
    paused_time: f64,
    mystery_time: f64,
}

pub struct Game {
    pub wave: wave::Wave,
    pub spaceship: Option<spaceship::Spaceship>,
    pub canon: canon::Canon,
    pub bullets: Vec<bullet::Bullet>,
    pub bunkers: bunkers::Bunkers,
    pub explosions: explosions::Explosions,
    pub info: Info,
}

impl Game {
    pub fn new() -> Game {
        Game {
            wave: wave::Wave::new(),
            spaceship: None,
            canon: canon::Canon::new(),
            bullets: Vec::new(),
            bunkers: bunkers::Bunkers::new(),
            explosions: explosions::Explosions::new(),
            info:
                Info {
                    score: 0,
                    canons: 3,
                    state: State::Menu(Selection::NewGame),
                    rand_time: 0.0,
                    paused_time: 0.0,
                    mystery_time: 0.0,
                },
        }
    }

    pub fn change_selection_down(&mut self) {
        match self.info.state {
            State::Menu(Selection::NewGame) => {
                self.info.state = State::Menu(Selection::Records);
            }

            State::Menu(Selection::Records) => {
                self.info.state = State::Menu(Selection::Exit);
            }

            State::Menu(Selection::Exit) => {
                self.info.state = State::Menu(Selection::NewGame);
            }

            _ => {}
        }
    }

    pub fn change_selection_up(&mut self) {
        match self.info.state {
            State::Menu(Selection::NewGame) => {
                self.info.state = State::Menu(Selection::Exit);
            }

            State::Menu(Selection::Records) => {
                self.info.state = State::Menu(Selection::NewGame);
            }

            State::Menu(Selection::Exit) => {
                self.info.state = State::Menu(Selection::Records);
            }

            _ => {}
        }
    }

    pub fn make_selection(&mut self) {
        match self.info.state {
            State::Menu(Selection::NewGame) => {
                self.info.state = State::Running;
            }

            State::Menu(Selection::Records) => {
                self.info.state = State::Records;
            }

            State::Menu(Selection::Exit) => {
                self.info.state = State::Exit;
            }

            State::Over => {
                self.info.state = State::Restart;
            }

            State::Records => {
                self.info.state = State::Menu(Selection::NewGame);
            }

            _ => {}
        }
    }

    pub fn canon_fire(&mut self) {
        let mut position = self.canon.position.clone();
        position.y -= self.canon.size.height;

        let bullet =
            bullet::Bullet::new(
                position,
                bullet::State::MovingUp
            );

        for b in &self.bullets {
            match b.state {
                bullet::State::MovingUp => {
                    return;
                }

                _ => {}
            }
        }

        self.bullets.push(bullet);
    }

    fn alien_fire(&mut self, dt: f64) {
        let ts =
            self.wave.len() as f64 / (2 * wave::ROWS * wave::COLUMNS) as f64;

        let mut rng = thread_rng();
        let x: f64 = rng.gen();

        self.info.rand_time += dt;
        if self.info.rand_time > RAND_STEP {
            self.info.rand_time -= RAND_STEP;

            if x > ts + 0.3 {
                let column: usize =
                    rng.gen_range(0, self.wave.aliens.len()) as usize;

                if let Some(alien) = self.wave.aliens[column].last() {
                    let mut position = alien.position.clone();
                    position.y += alien.size.height;

                    let bullet =
                        bullet::Bullet::new(
                            position,
                            bullet::State::MovingDown
                        );

                    self.bullets.push(bullet);
                }
            }
        }

    }

    fn update_bullets(&mut self, dt: f64) {
        for bullet in &mut self.bullets {
            bullet.update(dt);
        }
    }

    fn handle_spaceship(&mut self, dt: f64) {
        if let Some(ref mut spaceship) = self.spaceship {
            spaceship.update(dt);
        } else {
            self.info.mystery_time += dt;

            if self.info.mystery_time > MYSTERY_STEP {
                self.info.mystery_time -= MYSTERY_STEP;

                let mut rng = thread_rng();
                let x: f64 = rng.gen();

                let position =
                    if x > 0.5 {
                        Position {
                            x: WIDTH,
                            y: 200.0,
                        }
                    } else {
                        Position {
                            x: 0.0,
                            y: 200.0,
                        }
                    };

                let state =
                    if x > 0.5 {
                        spaceship::State::MovingLeft
                    } else {
                        spaceship::State::MovingRight
                    };

                self.spaceship =
                    Some(spaceship::Spaceship::new(position, state));
            }
        }
    }

    fn handle_collisions(&mut self) {
        let mut bullets = self.bullets.clone();

        bullets.retain(|bullet| {
            let mut bullet = bullet.clone();

            if bullet.position.y >= 1006.0 || bullet.position.y <= 0.0 {
                return false;
            }

            for column in &mut self.wave.aliens {
                for alien in column {
                    if bullet.overlaps(alien) {
                        match alien.kind {
                            wave::alien::Kind::Alpha => {
                                self.info.score += 30;
                            }

                            wave::alien::Kind::Beta => {
                                self.info.score += 20;
                            }

                            wave::alien::Kind::Gamma => {
                                self.info.score += 10;
                            }
                        }

                        self.explosions.add(alien.position.clone());
                        alien.kill();
                        return false;
                    }
                }
            }

            for bunker in &mut self.bunkers.bunkers {
                for block in &mut bunker.blocks {
                    if bullet.overlaps(block) {
                        block.change_state();
                        return false;
                    }
                }
            }

            if let Some(ref mut spaceship) = self.spaceship.clone() {
                if bullet.overlaps(spaceship) {
                    self.info.score += 100;
                    self.explosions.add(spaceship.position.clone());
                    self.spaceship = None;
                    return false;
                }
            }

            if bullet.overlaps(&mut self.canon) {
                self.info.canons -= 1;

                if self.info.canons == 0 {
                    self.info.state = State::Over;
                } else {
                    self.info.state = State::Paused;
                }

                self.canon.state = canon::State::Dead;
                return false;
            }

            return true;
        });

        if let Some(ref spaceship) = self.spaceship.clone() {
            let position = &spaceship.position;
            if position.x < 0.0 || position.x > WIDTH {
                self.spaceship = None;
            }
        }

        for column in &mut self.wave.aliens {
            for alien in column {
                for bunker in &mut self.bunkers.bunkers {
                    for block in &mut bunker.blocks {
                        if alien.overlaps(block) {
                            block.change_state();
                        }
                    }
                }

                if alien.overlaps(&mut self.canon) {
                    self.info.canons -= 1;

                    if self.info.canons == 0 {
                        self.info.state = State::Over;
                    } else {
                        self.info.state = State::Paused;
                    }

                    self.canon.state = canon::State::Dead;
                }

                if alien.position.y > 986.0 {
                    self.info.state = State::Over;
                }
            }
        }

        self.bullets = bullets.to_vec();

        self.bunkers.clear();
        self.wave.clear();

        if self.wave.is_empty() {
            self.info.score += 100;
            self.info.canons += 1;
            self.bullets = Vec::new();
            self.wave = wave::Wave::new();
        }
    }

    pub fn update(&mut self, dt: f64) {
        match self.info.state {
            State::Running => {
                self.wave.update(dt);
                self.canon.update(dt);
                self.explosions.update(dt);
                self.update_bullets(dt);
                self.handle_spaceship(dt);
                self.handle_collisions();
                self.alien_fire(dt);
            }

            State::Paused => {
                self.explosions.update(dt);
                self.info.paused_time += dt;
                if self.info.paused_time > PAUSED_STEP {
                    self.info.paused_time = 0.0;
                    self.info.state = State::Running;
                    self.canon = canon::Canon::new();
                    self.bullets = Vec::new();
                }
            }

            _ => {}
        }
    }
}
