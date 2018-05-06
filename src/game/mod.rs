mod position;
pub mod wave;

use self::wave::Wave;
use super::sprites::Sprites;

pub static WIDTH: f64 = 600.0;
pub static HEIGHT: f64 = 800.0;



pub struct Game {
    pub score: i32,
    pub wave: Wave,
    pub sprites: Sprites,
}

impl Game {
    pub fn new() -> Game {
        Game {
            score: 0,
            wave: Wave::new(),
            sprites: Sprites::new(),
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

    pub fn update(&mut self, dt: f64) {
        for mut alien in &mut self.wave.aliens {
            alien.step(dt, 20.0);
        }
    }
}
