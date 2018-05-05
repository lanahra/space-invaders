use position::Position;

// Constants for specify the sprites of alien
pub static SPRITE_ALIEN_V1: i32 = 0;
pub static WIDTH_SPRITE_ALIEN_V1: f64 = 30.0;
pub static HEIGHT_SPRITE_ALIEN_V1: f64 = 20.0;

pub static SPRITE_ALIEN_V2: i32 = 1;
pub static WIDTH_SPRITE_ALIEN_V2: f64 = 30.0;
pub static HEIGHT_SPRITE_ALIEN_V2: f64 = 20.0;

// Max step size to one side
pub static MAX_STEP_SIDE: f64 = 75.0;

// Step down value
pub static STEP_DOWN: f64 = 5.0;

// Constants for specify the direction of alien step
pub static LEFT_TO_RIGHT: i32 = 0;
pub static RIGHT_TO_LEFT: i32 = 1;

// Struct for manage alien enemys
pub struct Alien {
    pub position: Position, // Position in map
    pub direction: i32,
    pub sprite: i32, // Sprite identificator
}

impl Alien {
    // Constructor fill the fields
    pub fn new(position: Position, sprite: i32) -> Alien {
        Alien {
            position,
            direction: LEFT_TO_RIGHT,
            sprite,
        }
    }

    pub fn step(&mut self, dt: f64) { // Walks 150 in total (75 first time)
        if self.direction == LEFT_TO_RIGHT {
            if self.position.center.0 < self.position.initial_center.0+MAX_STEP_SIDE {
                self.position.center.0+=1.0*dt;
            }
            else {
                self.direction = RIGHT_TO_LEFT;
                self.position.center.1+=STEP_DOWN*dt;
            }
        }
        else {
            if self.position.center.0 > self.position.initial_center.0-MAX_STEP_SIDE {
                self.position.center.0-=1.0*dt;
            }
            else {
                self.direction = LEFT_TO_RIGHT;
                self.position.center.1+=STEP_DOWN*dt;
            }
        }
        self.position.generate_collision_box();
    }
}
