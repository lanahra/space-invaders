use game::WIDTH;
use game::HEIGHT;

// Struct for manage position of elements in map
pub struct Position {
    pub center: (f64, f64), // A pair of integers for coordinates x and y
    // height and width will be used for calculate collision box
    pub height: f64,
    pub width: f64,
    pub coordinates_collision_box: ((f64, f64), (f64, f64)),
}

impl Position {
    // Constructor define the fields
    pub fn new(center: (f64, f64), height: f64, width: f64) -> Position {
        let mut position = Position {
            center: center,
            height: height,
            width: width,
            coordinates_collision_box: ((0.0, 0.0), (0.0, 0.0)),
        };
        position.generate_collision_box();
        position.check_center();
        return position;
    }

    pub fn generate_collision_box(&mut self) {
        //First point wiil be bottom left
        (self.coordinates_collision_box.0).0 = self.center.0-(self.width/2.0);
        (self.coordinates_collision_box.0).1 = self.center.1-(self.height/2.0);
        //Second point wiil be top right
        (self.coordinates_collision_box.1).0 = self.center.0+(self.width/2.0);
        (self.coordinates_collision_box.1).1 = self.center.1+(self.height/2.0);
    }

    pub fn check_center(&self) {
        if (self.coordinates_collision_box.0).0 < 0.0 || (self.coordinates_collision_box.0).1 < 0.0 {
            panic!("Position out of bounds!");
        }
        if (self.coordinates_collision_box.1).0 > game::WIDTH || (self.coordinates_collision_box.1).1 > game::HEIGHT {
            panic!("Position out of bounds!");
        }
    }
}
