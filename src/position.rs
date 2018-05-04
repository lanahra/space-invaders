// Struct for manage position of elements in map
pub struct Position {
    pub coordinate: (f64, f64), // A pair of integers for coordinates x and y
    // height and width will be used for calculate collision box
    pub height: f64,
    pub width: f64,
}

impl Position {
    // Constructor define the fields
    pub fn new(coordinate: (f64, f64), height: f64, width: f64) -> Position {
        Position {
            coordinate: coordinate,
            height: height,
            width: width,
        }
    }
}
