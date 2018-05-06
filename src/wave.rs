use alien::Alien;
use alien::HEIGHT_SPRITE_ALIEN_V1;
use alien::WIDTH_SPRITE_ALIEN_V1;
use alien::SPRITE_ALIEN_V1;
use position::Position;
use std::collections::LinkedList;

// Constants to define first alien position of wave in screen
pub static FIRST_ALIEN_POSITION_X: f64 = 75.0;
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
        for i in 0..NUMBER_OF_ROWS {
            for j in 0..NUMBER_OF_ALIENS_PER_ROW {
                // x_position is the sum of all previous aliens and spaces width plus initial position
                let x_position = FIRST_ALIEN_POSITION_X+j as f64 *(WIDTH_SPRITE_ALIEN_V1+HORIZONTAL_SPACES_BETWEEN_ALIENS);

                // y_position is the sum of all previous aliens and spaces width plus initial position
                let y_position = FIRST_ALIEN_POSITION_Y+i as f64 *(VERTICAL_SPACES_BETWEEN_ALIENS + HEIGHT_SPRITE_ALIEN_V1);

                let position = Position::new((x_position, y_position),
                                             HEIGHT_SPRITE_ALIEN_V1,
                                             WIDTH_SPRITE_ALIEN_V1);
                let alien = Alien::new(position, SPRITE_ALIEN_V1);
                self.aliens.push_back(alien);
            }
        }
    }
}