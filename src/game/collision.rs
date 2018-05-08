use game::position::Position;
use game::size::Size;

pub trait Collision {
    fn position(&self) -> &Position;
    fn size(&self) -> &Size;

    fn overlaps<T: Collision>(&self, other: &T) -> bool {
        let s_position = self.position();
        let o_position = other.position();

        let s_size = self.size();
        let o_size = other.size();

        ((s_position.x - o_position.x).abs() * 2.0 <
         s_size.width + o_size.width) &&
            ((s_position.y - o_position.y).abs() * 2.0 <
             s_size.height + o_size.height)
    }
}
