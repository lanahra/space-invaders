pub mod canon;
pub mod bullet;
mod position;
mod size;
mod collision;
pub mod wave;
pub mod bunker;

use self::bunker::Bunker;
use self::canon::Canon;
use self::position::Position;
use self::collision::Collision;
use self::wave::Wave;
use self::bullet::Shot;
use std::collections::LinkedList;
use std::collections::linked_list::Iter;


const POSITION_BUNKER: Position =
    Position {
        x: 0.103333 * WIDTH,
        y: 0.6875 * HEIGHT,
    };

pub const BUNKERS: u32 = 3;
pub const BUNKERS_GAP: f64 = 0.32 * WIDTH;
pub const WIDTH: f64 = 600.0;
pub const HEIGHT: f64 = 800.0;

pub struct Game {
    pub score: i32,
    pub wave: Wave,
    pub canon: Canon,
    pub player_shot: Shot,
    pub bunkers: LinkedList<Bunker>
}

impl Game {
    pub fn new() -> Game {
        Game {
            score: 0,
            wave: Wave::new(),
            canon: Canon::new(),
            player_shot: Shot::new(Position {x: 0.0, y: 0.0}),
            bunkers: Game::create_bunkers(),
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
        if self.player_shot.is_active() {
            for alien in self.wave.iter_mut() {
                if alien.is_active() {
                    if self.player_shot.overlaps(alien) {
                        alien.shot_hit();
                        self.player_shot.inactivate_shot();
                    }
                }
            }
            for mut bunker in self.bunkers.iter_mut() {
                for mut block in bunker.iter_mut() {
                    if block.is_active() {
                        if self.player_shot.overlaps(block) {
                            block.shot_hit();
                            self.player_shot.inactivate_shot();
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

    pub fn update(&mut self, dt: f64) {
        self.check_player_shot_collision();
        self.wave.update(dt);
        self.canon.update(dt);
        self.player_shot.update(dt);
    }
}
