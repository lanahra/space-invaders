extern crate rand;

mod bullet;
mod position;
mod size;
mod collision;
pub mod wave;
pub mod entity;
pub mod player_info;

use self::player_info::PlayerInfo;
use self::entity::statical::bunker::Bunker;
use self::entity::active::canon::Canon;
use self::position::Position;
use self::collision::Collision;
use self::entity::Entity;
use self::entity::active::Active;
use std::collections::LinkedList;
use std::collections
::linked_list::Iter;
use std::vec::Vec;
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

pub struct Game {
    pub wave: wave::Wave,
    pub canon: Canon,
    pub bullets: Vec<bullet::Bullet>,
    pub bunkers: LinkedList<Bunker>,
    pub player_info: PlayerInfo,
}

impl Game {
    pub fn new() -> Game {
        Game {
            wave: wave::Wave::new(),
            canon: Canon::new(),
            bullets: Vec::new(),
            bunkers: Game::create_bunkers(),
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

            return true;
        });

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
