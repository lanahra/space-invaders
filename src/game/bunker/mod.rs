pub mod block;

use block::*;
use game::position::Position;
use game::WIDTH;
use game::HEIGHT;
use std::collections::LinkedList;
use std::collections::linked_list::Iter;
use std::collections::linked_list::IterMut;

const ROWS: u32 = 3;
const COLUMNS: u32 = 4;
const WIDTH_GAP: f64 = 0.025 * WIDTH;
const HEIGHT_GAP: f64 = 0.025 * WIDTH;

pub struct Bunker {
    pub blocks: LinkedList<Block>,
}

impl Bunker {
    pub fn new(position: Position) -> Bunker {
        Bunker {
            blocks: Bunker::create_blocks(position),
        }
    }

    fn create_blocks(position_ref: Position) -> LinkedList<Block> {
        let mut blocks: LinkedList<Block> = LinkedList::new();

        for i in 0..ROWS {
            for j in 0..COLUMNS {
                let position =
                    Position {
                        x: position_ref.x + (j as f64 * self::WIDTH_GAP),
                        y: position_ref.y + (i as f64 * self::HEIGHT_GAP),
                    };

                if !(i == ROWS-1 && (j >= 1 && j <= 2)) {
                    let block = Block::new(position);

                    blocks.push_back(block);
                }
            }
        }

        return blocks;
    }

    pub fn iter(&self) -> Iter<Block> {
        self.blocks.iter()
    }

    pub fn iter_mut(&mut self) -> IterMut<Block> {
        self.blocks.iter_mut()
    }
}
