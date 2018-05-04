// Struct for manage position of elements in map
pub struct Position {
    pub coordinate: (i32, i32), // A pair of integers for coordinates x and y
    // height and width will be used for calculate collision box
    pub height: i32,
    pub width: i32,
}

impl Position {
    // Constructor define the fields
    pub fn new(coordinate: (i32, i32), height: i32, width: i32) -> Position {
        Position {
            coordinate: coordinate,
            height: height,
            width: width,
        }
    }
}
