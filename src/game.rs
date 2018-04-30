pub struct Game {
    pub score: i32,
}

impl Game {
    pub fn new(score: i32) -> Game {
        Game {
            score
        }
    }
}
