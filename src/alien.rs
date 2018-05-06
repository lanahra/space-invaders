use position::Position;

// Constants for specify the sprites of alien
pub static SPRITE_ALIEN_B1: i32 = 0;
pub static WIDTH_SPRITE_ALIEN_B1: f64 = 30.0;
pub static HEIGHT_SPRITE_ALIEN_B1: f64 = 20.0;

pub static SPRITE_ALIEN_B2: i32 = 1;
pub static WIDTH_SPRITE_ALIEN_B2: f64 = 30.0;
pub static HEIGHT_SPRITE_ALIEN_B2: f64 = 20.0;

pub static SPRITE_ALIEN_A1: i32 = 2;
pub static WIDTH_SPRITE_ALIEN_A1: f64 = 30.0;
pub static HEIGHT_SPRITE_ALIEN_A1: f64 = 20.0;

pub static SPRITE_ALIEN_A2: i32 = 3;
pub static WIDTH_SPRITE_ALIEN_A2: f64 = 30.0;
pub static HEIGHT_SPRITE_ALIEN_A2: f64 = 20.0;

pub static SPRITE_ALIEN_C1: i32 = 4;
pub static WIDTH_SPRITE_ALIEN_C1: f64 = 30.0;
pub static HEIGHT_SPRITE_ALIEN_C1: f64 = 20.0;

pub static SPRITE_ALIEN_C2: i32 = 5;
pub static WIDTH_SPRITE_ALIEN_C2: f64 = 30.0;
pub static HEIGHT_SPRITE_ALIEN_C2: f64 = 20.0;

// Max step size to one side
pub static MAX_STEP_SIDE: f64 = 75.0;

// Step down value
pub static STEP_DOWN: f64 = 15.0;

// Constants for specify the direction of alien step
pub static LEFT_TO_RIGHT: i32 = 0;
pub static RIGHT_TO_LEFT: i32 = 1;

// Struct for manage alien enemys
pub struct Alien {
    pub position: Position, // Position in map
    pub direction: i32,
    pub sprite: i32, // Sprite identificator
    pub timer: f64,
}

impl Alien {
    // Constructor fill the fields
    pub fn new(position: Position, sprite: i32) -> Alien {
        Alien {
            position,
            direction: LEFT_TO_RIGHT,
            sprite,
            timer: 0.0,
        }
    }

    pub fn step(&mut self, dt: f64, level: f64) { // Walks 150 in total (75 first time)
        self.animate(dt);

        if self.direction == LEFT_TO_RIGHT {
            if self.position.center.0 < self.position.initial_center.0+MAX_STEP_SIDE {
                self.position.center.0+=level*dt;
            }
            else {
                self.direction = RIGHT_TO_LEFT;
                self.position.center.1+=STEP_DOWN;
            }
        }
        else {
            if self.position.center.0 > self.position.initial_center.0-MAX_STEP_SIDE {
                self.position.center.0-=level*dt;
            }
            else {
                self.direction = LEFT_TO_RIGHT;
                self.position.center.1+=STEP_DOWN;
            }
        }
        self.position.generate_collision_box();
    }

    pub fn animate(&mut self, dt: f64) {
        self.timer+=dt;
        if self.timer > 0.3 {
            if self.sprite == SPRITE_ALIEN_A1 {
                self.sprite = SPRITE_ALIEN_A2;
            }
            else if self.sprite == SPRITE_ALIEN_A2 {
                self.sprite = SPRITE_ALIEN_A1;
            }
            else if self.sprite == SPRITE_ALIEN_B1 {
                self.sprite = SPRITE_ALIEN_B2;
            }
            else if self.sprite == SPRITE_ALIEN_B2 {
                self.sprite = SPRITE_ALIEN_B1;
            }
            else if self.sprite == SPRITE_ALIEN_C1 {
                self.sprite = SPRITE_ALIEN_C2;
            }
            else if self.sprite == SPRITE_ALIEN_C2 {
                self.sprite = SPRITE_ALIEN_C1;
            }
            self.timer = 0.0;
        }
    }
}
