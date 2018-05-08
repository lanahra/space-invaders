pub mod canon;
pub mod bullet;
mod position;
mod size;
mod collision;
pub mod wave;

use self::canon::Canon;
use self::position::Position;
use self::collision::Collision;
use self::wave::Wave;
use self::bullet::Shot;
use std::collections::LinkedList;
use std::collections::linked_list::Iter;

pub const WIDTH: f64 = 600.0;
pub const HEIGHT: f64 = 800.0;

pub struct Game {
    pub score: i32,
    pub wave: Wave,
    pub canon: Canon,
    pub player_shot: Shot,
}

impl Game {
    pub fn new() -> Game {
        Game {
            score: 0,
            wave: Wave::new(),
            canon: Canon::new(),
            player_shot: Shot::new(Position {x: 0.0, y: 0.0}),
        }
    }

    pub fn create_player_shot(&mut self) {
        if !self.player_shot.is_active() {
            self.player_shot.position.x = self.canon.position.x;
            self.player_shot.position.y = self.canon.position.y;
            self.player_shot.activate_shot();
        }
    }

    pub fn check_player_shot_collision(&mut self) {
        for alien in self.wave.iter() {
            if self.player_shot.overlaps(alien) {
                self.player_shot.inactivate_shot();
            }
        }
    }

    pub fn update(&mut self, dt: f64) {
        self.check_player_shot_collision();
        self.wave.update(dt);
        self.canon.update(dt);
        self.player_shot.update(dt);
    }
}
