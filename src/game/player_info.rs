const LIFES: u32 = 3;

pub enum State {
    Started,
    GameOver
}

pub struct PlayerInfo {
    pub points: u32,
    pub lifes: u32,
    state: State,
}

impl PlayerInfo {
    pub fn new() -> PlayerInfo {
        PlayerInfo {
            points: 0,
            lifes: self::LIFES,
            state: State::Started
        }
    }

    pub fn die(&mut self) {
        self.lifes -= 1;
        if self.lifes == 0 {
            self.game_over();
        }
    }

    pub fn game_over(&mut self) {
        self.state = State::GameOver;
    }

    pub fn kill_alien(&mut self) {
        self.points += 50;
    }

    pub fn kill_red_alien(&mut self) {
        self.points += 250;
    }

    pub fn reset_wave(&mut self) {
        self.points += 500;
    }
}