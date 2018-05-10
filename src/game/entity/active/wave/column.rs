use alien::Kind;
use alien::Alien;
use game::position::Position;
use game::WIDTH;
use game::HEIGHT;
use std::collections::LinkedList;
use std::collections::linked_list::Iter;
use std::collections::linked_list::IterMut;

pub const ROWS: u32 = 5;

const HEIGHT_GAP: f64 = 0.0375 * HEIGHT;

pub struct Column {
    aliens: LinkedList<Alien>,
}

impl Column {
    pub fn new(position: Position, kind: Kind) -> Column {
        Column {
            aliens: Column::create_aliens(position, kind),
        }
    }

    pub fn create_aliens(position_aux: Position, kind: Kind) -> LinkedList<Alien> {
        let mut aliens: LinkedList<Alien> = LinkedList::new();

        for i in 0..ROWS {
            let position =
                Position {
                    x: position_aux.x,
                    y: position_aux.y - (i as f64 * self::HEIGHT_GAP),
                };

            let alien = 
                match kind {
                    Kind::Alpha => 
                        Alien::new(position, Kind::Alpha),
                    
                    
                    Kind::Beta => 
                        Alien::new(position, Kind::Beta),
                    

                    Kind::Gamma => 
                        Alien::new(position, Kind::Gamma),
                };

            aliens.push_back(alien);
        }

        return aliens;
    }

    pub fn iter(&self) -> Iter<Alien> {
        self.aliens.iter()
    }

    pub fn iter_mut(&mut self) -> IterMut<Alien> {
        self.aliens.iter_mut()
    }
}