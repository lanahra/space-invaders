use game::position::Position;
use game::size::Size;
use game::WIDTH;
use game::HEIGHT;
use game::collision::Collision;
use game::entity::Entity;
use game::entity::active::Active;

const VELOCITY: f64 = 0.15 * WIDTH;

const DEATH_STEP: f64 = 0.3;

pub enum State {
    Active,
    Dead,
    Inactive
}

pub enum Movement {
    MovingRight,
    MovingLeft
}

pub struct RedAlien {
    pub position: Position,
    pub state: State,
    pub movement: Movement,
    pub size: Size,
    pub timer: f64,
}

impl RedAlien {
    pub fn new(position: Position) -> RedAlien {
        RedAlien {
            position,
            state: State::Inactive,
            movement: Movement::MovingRight,
            size:
                Size {
                    width: 0.08 * WIDTH,
                    height: 0.04 * HEIGHT,
                },
            timer: 0.0
        }
    }

    pub fn move_right(&mut self) {
        self.movement = Movement::MovingRight;
        self.position.x = -0.3 * WIDTH;
    }

    pub fn move_left(&mut self) {
        self.movement = Movement::MovingLeft;
        self.position.x = 1.3 * WIDTH;
    }
}

impl Collision for RedAlien {
    fn position(&self) -> &Position {
        &self.position
    }

    fn size(&self) -> &Size {
        &self.size
    }
}

impl Entity for RedAlien {
    fn is_active(&self) -> bool {
        match self.state {
            State::Inactive => {
                return false;
            }

            _ => {
                return true;
            }
        }
    }

    fn shot_hit(&mut self) {
        self.state = State::Dead;
    }

    fn change_state(&mut self) {
        match self.state {
            State::Dead => {
                self.state = State::Inactive;
            }

            State::Inactive => {
                self.state = State::Active;
            }

            State::Active => {
                self.state = State::Inactive;
            }
        }
    }
}

impl Active for RedAlien {
    fn position(&mut self) -> &mut Position {
        &mut self.position
    }

    fn update(&mut self, dt: f64) {
        match self.state {
            State::Active => {
                match self.movement {
                    Movement::MovingRight => {
                        if self.position.x-self.size.width/2.0 > WIDTH {
                            self.change_state();
                        }
                            else {
                                self.move_x(dt * VELOCITY);
                            }
                    }

                    Movement::MovingLeft => {
                        if self.position.x+self.size.width/2.0 < 0.0 {
                            self.change_state();
                        }
                            else {
                                self.move_x(-dt * VELOCITY);
                            }
                    }

                    _ => {}
                }
            }

            State::Dead => {
                self.timer += dt;
                if self.timer >= DEATH_STEP {
                    self.timer -= DEATH_STEP;
                    self.change_state();
                }
            }

            _ => {}
        }

    }
}
