pub mod active;
pub mod statical;

pub trait Entity {
    fn is_active(&self) -> bool;
    fn shot_hit(&mut self);
    fn change_state(&mut self);
}
