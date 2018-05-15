#[derive(Clone)]
pub struct Position {
    pub x: f64,
    pub y: f64,
}

#[derive(Clone)]
pub struct Size {
    pub width: f64,
    pub height: f64,
}

pub trait Entity {
    fn position(&mut self) -> &mut Position;
    fn size(&mut self) -> &mut Size;
}

pub trait Solid: Entity {
    fn overlaps<T: Solid>(&mut self, other: &mut T) -> bool {
        let s_position = self.position().clone();
        let o_position = other.position().clone();

        let s_size = self.size().clone();
        let o_size = other.size().clone();

        ((s_position.x - o_position.x).abs() * 2.0 <
         s_size.width + o_size.width) &&
            ((s_position.y - o_position.y).abs() * 2.0 <
             s_size.height + o_size.height)
    }
}

pub trait Kinetic: Entity {
    fn move_x(&mut self, dx: f64) {
        self.position().x += dx;
    }

    fn move_y(&mut self, dy: f64) {
        self.position().y += dy;
    }
}
