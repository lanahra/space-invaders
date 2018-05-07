pub mod canon;
pub mod bullet;
mod position;
mod size;
pub mod wave;

use self::canon::Canon;
use self::position::Position;
use self::wave::Wave;
use self::bullet::Shot;
use std::collections::LinkedList;
use std::collections::linked_list::Iter;


pub static WIDTH: f64 = 600.0;
pub static HEIGHT: f64 = 800.0;

pub struct Game {
    pub score: i32,
    pub wave: Wave,
    pub canon: Canon,
    pub player_shots: LinkedList<Shot>,
}

impl Game {
    pub fn new() -> Game {
        Game {
            score: 0,
            wave: Wave::new(),
            canon: Canon::new(),
            player_shots: LinkedList::new(),
        }
    }

    pub fn check_collision(&self, box_one: ((f64,f64),(f64,f64)), box_two: ((f64,f64),(f64,f64))) -> bool {
        if (box_one.0).0 >= (box_two.0).0 && (box_one.0).0 <= (box_two.1).0 {
            if (box_one.0).1 >= (box_two.0).1 && (box_one.0).1 <= (box_two.1).1 {
                return true;
            }
        }
        if (box_one.1).0 >= (box_two.0).0 && (box_one.1).0 <= (box_two.1).0 {
            if (box_one.1).1 >= (box_two.0).1 && (box_one.1).1 <= (box_two.1).1 {
                return true;
            }
        }
        return false;
    }

    pub fn create_player_shot(&mut self) {
        let mut position = Position {
            x: self.canon.position.x,
            y: self.canon.position.y,
        };
        let mut player_shot = Shot::new(position);
        self.player_shots.push_back(player_shot);
    }

    pub fn update(&mut self, dt: f64) {
        self.wave.update(dt);
        self.canon.update(dt);
        for shot in self.player_shots.iter_mut() {
            shot.update(dt);
        }
    }

    pub fn iterate_player_shots(&self) -> Iter<Shot> {
        self.player_shots.iter()
    }
}
