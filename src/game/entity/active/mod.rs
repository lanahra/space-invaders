pub mod wave;
pub mod bullet;
pub mod canon;

use game::position::Position;

pub trait Active {
    fn position(&mut self) -> &mut Position;
    fn update(&mut self, dt: f64);

    fn move_x(&mut self, dx: f64) {
        self.position().x += dx;
    }

    fn move_y(&mut self, dy: f64) {
        self.position().y += dy;
    }
}