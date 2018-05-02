pub static WIDTH: f64 = 600.0;
pub static HEIGHT: f64 = 800.0;

pub struct Game {
    pub score: i32,
}

impl Game {
    pub fn new() -> Game {
        Game {
            score: 0,
        }
    }

    pub fn update(&mut self, dt: f64) {
    }
}
