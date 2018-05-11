extern crate rand;

mod position;
mod size;
mod collision;
pub mod entity;
pub mod player_info;

use self::player_info::PlayerInfo;
use self::entity::statical::bunker::Bunker;
use self::entity::active::canon::Canon;
use self::position::Position;
use self::collision::Collision;
use self::entity::active::wave::Wave;
use self::entity::active::wave::COLUMNS;
use self::entity::active::wave::column::ROWS;
use self::entity::active::bullet::Shot;
use self::entity::active::bullet::Kind;
use self::entity::Entity;
use self::entity::active::Active;
use self::entity::active::wave::ANSWER_TO_THE_ULTIMATE_QUESTION_OF_LIFE_THE_UNIVERSE_AND_EVERYTHING;
use std::collections::LinkedList;
use std::collections
::linked_list::Iter;
use self::rand::Rng;


const POSITION_BUNKER: Position =
    Position {
        x: 0.103333 * WIDTH,
        y: 0.6875 * HEIGHT,
    };

pub const BUNKERS: u32 = 4;
pub const BUNKERS_GAP: f64 = 0.23 * WIDTH;
pub const WIDTH: f64 = 600.0;
pub const HEIGHT: f64 = 800.0;

const SHOT_PROBABILITY: i64 = 500;

pub struct Game {
    pub wave: Wave,
    pub canon: Canon,
    pub player_shot: Shot,
    pub enemy_shots: LinkedList<Shot>,
    pub bunkers: LinkedList<Bunker>,
    pub player_info: PlayerInfo,
    timer: f64,
}

impl Game {
    pub fn new() -> Game {
        Game {
            wave: Wave::new(),
            canon: Canon::new(),
            player_shot: Shot::new(Position {x: 0.0, y: 0.0}, Kind::PlayerShot),
            enemy_shots: LinkedList::new(),
            bunkers: Game::create_bunkers(),
            player_info: PlayerInfo::new(),
            timer: 0.0,
        }
    }

    pub fn create_player_shot(&mut self) {
        if !self.player_shot.is_active() {
            self.player_shot.position.x = self.canon.position.x;
            self.player_shot.position.y = self.canon.position.y;
            self.player_shot.change_state();
        }
    }

    pub fn check_player_shot_collision(&mut self) {
        if self.player_shot.is_active() {
            if self.wave.red_alien.is_active() {
                if self.player_shot.overlaps(&self.wave.red_alien) {
                    self.player_shot.change_state();
                    self.player_info.kill_red_alien();
                    self.wave.red_alien.shot_hit();
                }
            }

            let mut hit_alien: u32 = 0;
            for column in self.wave.iter_mut() {
                for alien in column.iter_mut() {
                    if alien.is_active() {
                        if self.player_shot.overlaps(alien) {
                            hit_alien += 1;
                            self.player_shot.change_state();
                            alien.shot_hit();
                        }
                    }
                }
            }
            for i in 0..hit_alien {
                self.wave.kill_alien();
                self.player_info.kill_alien();
            }

            for mut bunker in self.bunkers.iter_mut() {
                for mut block in bunker.iter_mut() {
                    if block.is_active() {
                        if self.player_shot.overlaps(block) {
                            self.player_shot.change_state();
                            block.shot_hit();
                        }
                    }
                }
            }
        }
    }

    pub fn check_enemy_shot_collision(&mut self) {
        for shot in self.enemy_shots.iter_mut() {
            if shot.is_active() {
                if self.canon.is_active() {
                    if shot.overlaps(&self.canon) {
                        shot.change_state();
                        self.player_info.die();
                        self.canon.shot_hit();
                    }
                }

                for mut bunker in self.bunkers.iter_mut() {
                    for mut block in bunker.iter_mut() {
                        if block.is_active() {
                            if shot.overlaps(block) {
                                shot.change_state();
                                block.shot_hit();
                            }
                        }
                    }
                }
            }
        }
    }

    fn create_bunkers() -> LinkedList<Bunker> {
        let mut bunkers: LinkedList<Bunker> = LinkedList::new();

        for i in 0..BUNKERS {
            let position =
                Position {
                    x: self::POSITION_BUNKER.x + (i as f64 * self::BUNKERS_GAP),
                    y: self::POSITION_BUNKER.y,
                };

            let bunker = Bunker::new(position);
            bunkers.push_back(bunker);
        }

        return bunkers;
    }

    fn check_reset_wave(&mut self) {
        if self.wave.deadly_counter == ROWS * COLUMNS {
            self.wave.reset_wave();
            self.player_info.reset_wave();
        }
    }

    fn create_alien_shots(&mut self) {
        let mut rng = rand::thread_rng();
        let num: i64 = rng.gen_range(0, SHOT_PROBABILITY / self.wave.level);
        if num == ANSWER_TO_THE_ULTIMATE_QUESTION_OF_LIFE_THE_UNIVERSE_AND_EVERYTHING {
            let alien_index: i64 = rng.gen_range(0, COLUMNS as i64);
            for (index, column) in self.wave.iter_mut().enumerate() {
                if index as i64 == alien_index {
                    let mut stop = false;
                    for alien in column.iter_mut() {
                        if alien.is_active() && !stop {
                            if self.enemy_shots.is_empty() {
                                let mut shot = Shot::new(
                                    Position { 
                                        x: alien.position.x,
                                        y: alien.position.y,
                                    },
                                    Kind::EnemyShot,
                                );
                                shot.change_state();
                                self.enemy_shots.push_back(shot);
                            }
                            else {
                                let mut changed = false;
                                for existing_shot in self.enemy_shots.iter_mut() {
                                    if !existing_shot.is_active() {
                                        existing_shot.position.x = alien.position.x;
                                        existing_shot.position.y = alien.position.y;
                                        existing_shot.kind = Kind::EnemyShot;
                                        existing_shot.change_state();
                                        changed = true;
                                    }
                                }
                                if !changed {
                                    let mut shot = Shot::new(
                                        Position { 
                                            x: alien.position.x,
                                            y: alien.position.y,
                                        },
                                        Kind::EnemyShot,
                                    );
                                    shot.change_state();
                                    self.enemy_shots.push_back(shot);
                                }
                            }
                            stop = true;
                        }
                    }
                }
            }
        }
    }

    

    pub fn update(&mut self, dt: f64) {
        self.check_player_shot_collision();
        self.check_enemy_shot_collision();
        self.check_reset_wave();
        self.create_alien_shots();
        self.wave.update(dt);
        self.canon.update(dt);
        self.player_shot.update(dt);
        for shot in self.enemy_shots.iter_mut() {
            shot.update(dt);
        }
    }
}
