use alien::Alien;
use alien::HEIGHT_SPRITE_ALIEN_A1;
use alien::WIDTH_SPRITE_ALIEN_A1;
use alien::SPRITE_ALIEN_A1;
use alien::HEIGHT_SPRITE_ALIEN_B1;
use alien::WIDTH_SPRITE_ALIEN_B1;
use alien::SPRITE_ALIEN_B1;
use alien::HEIGHT_SPRITE_ALIEN_C1;
use alien::WIDTH_SPRITE_ALIEN_C1;
use alien::SPRITE_ALIEN_C1;
use position::Position;
use std::collections::LinkedList;

// Constants to define first alien position of wave in screen
pub static FIRST_ALIEN_POSITION_X: f64 = 90.0;
pub static FIRST_ALIEN_POSITION_Y: f64 = 200.0;

// Constants with number of aliens, spaces between them and rows
pub static NUMBER_OF_ALIENS_PER_ROW: i32 = 11;
pub static NUMBER_OF_ROWS: i32 = 5;
pub static HORIZONTAL_SPACES_BETWEEN_ALIENS: f64 = 12.0;
pub static VERTICAL_SPACES_BETWEEN_ALIENS: f64 = 10.0;

// Struct to manage waves in game
pub struct Wave {
    pub aliens: LinkedList<Alien>
}

impl Wave {
    // Fills the fields and generates aliens
    pub fn new() -> Wave {
        let mut wave = Wave {
            aliens: LinkedList::new()
        };
        wave.generate_aliens();
        return wave;
    }

    // Put aliens in their must be places
    pub fn generate_aliens(&mut self) {
        let mut width: f64;
        let mut height: f64;
        let mut sprite: i32;
        for i in 0..NUMBER_OF_ROWS {
            for j in 0..NUMBER_OF_ALIENS_PER_ROW {
                if i == 0 {
                    width = WIDTH_SPRITE_ALIEN_A1;
                    height = HEIGHT_SPRITE_ALIEN_A1;
                    sprite = SPRITE_ALIEN_A1;
                }
                else if i == 1 || i == 2 {
                    width = WIDTH_SPRITE_ALIEN_B1;
                    height = HEIGHT_SPRITE_ALIEN_B1;
                    sprite = SPRITE_ALIEN_B1;
                }
                else {
                    width = WIDTH_SPRITE_ALIEN_C1;
                    height = HEIGHT_SPRITE_ALIEN_C1;
                    sprite = SPRITE_ALIEN_C1;
                }
                // x_position is the sum of all previous aliens and spaces width plus initial position
                let x_position = FIRST_ALIEN_POSITION_X+j as f64 *(width+HORIZONTAL_SPACES_BETWEEN_ALIENS);

                // y_position is the sum of all previous aliens and spaces width plus initial position
                let y_position = FIRST_ALIEN_POSITION_Y+i as f64 *(VERTICAL_SPACES_BETWEEN_ALIENS + height);

                let position = Position::new((x_position, y_position),
                                             height,
                                             width);
                let alien = Alien::new(position, sprite);
                self.aliens.push_back(alien);
            }
        }
    }
}