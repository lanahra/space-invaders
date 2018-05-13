extern crate rand;

mod bullet;
mod canon;
mod position;
mod size;
mod collision;
pub mod bunkers;
pub mod wave;
pub mod player_info;

use self::player_info::PlayerInfo;
use self::position::Position;
use self::collision::Collision;
use std::collections::LinkedList;
use std::vec::Vec;
use self::rand::Rng;

pub const WIDTH: f64 = 600.0;
pub const HEIGHT: f64 = 800.0;

pub struct Game {
    pub wave: wave::Wave,
    pub canon: canon::Canon,
    pub bullets: Vec<bullet::Bullet>,
    pub bunkers: bunkers::Bunkers,
    pub player_info: PlayerInfo,
}

impl Game {
    pub fn new() -> Game {
        Game {
            wave: wave::Wave::new(),
            canon: canon::Canon::new(),
            bullets: Vec::new(),
            bunkers: bunkers::Bunkers::new(),
            player_info: PlayerInfo::new(),
        }
    }

    pub fn canon_fire(&mut self) {
        let position = self.canon.position.clone();

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

    fn update_bullets(&mut self, dt: f64) {
        for bullet in &mut self.bullets {
            bullet.update(dt);
        }
    }

    fn handle_collisions(&mut self) {
        let mut bullets = self.bullets.clone();

        bullets.retain(|bullet| {
            if bullet.position.y >= HEIGHT || bullet.position.y <= 0.0 {
                return false;
            }

            for column in &mut self.wave.aliens {
                for alien in column {
                    if bullet.overlaps(alien) {
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

            return true;
        });

        self.bunkers.clear();
        self.wave.clear();

        self.bullets = bullets.to_vec();
    }

    pub fn update(&mut self, dt: f64) {
        self.wave.update(dt);
        self.canon.update(dt);
        self.update_bullets(dt);
        self.handle_collisions();
    }
}
