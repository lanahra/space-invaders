#[derive(Copy, Clone)]
pub struct Position {
    pub x: f64,
    pub y: f64,
}

#[derive(Copy, Clone)]
pub struct Size {
    pub width: f64,
    pub height: f64,
}

pub trait Entity {
    fn position(entity: &Self) -> Position;
    fn size(entity: &Self) -> Size;
}

pub trait Solid: Entity {
    fn overlaps<T: Solid>(&self, other: T) -> bool {
        let e_position = Entity::position(self);
        let o_position = Entity::position(&other);

        let e_size = Entity::size(self);
        let o_size = Entity::size(&other);

        ((e_position.x - o_position.x).abs() * 2.0 <
         e_size.width + o_size.width) &&
            ((e_position.y - o_position.y).abs() * 2.0 <
             e_size.height + o_size.height)
    }
}

pub trait Kinetic: Entity {
    fn move_x(dx: f64, entity: Self) -> Self;
    fn move_y(dy: f64, entity: Self) -> Self;
}
