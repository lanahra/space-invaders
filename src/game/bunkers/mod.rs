pub mod block;

use game::entity::Position;
use std::vec::Vec;
use self::block::Block;

const BUNKERS: u32 = 4;

const POSITION: Position =
    Position {
        x: 130.0,
        y: 801.0,
    };

const ROWS: u32 = 3;
const COLUMNS: u32 = 3;
const WIDTH_GAP: f64 = 208.0;

pub struct Bunker {
    pub blocks: Vec<Block>,
}

impl Bunker {
    fn new(position: Position) -> Bunker {
        Bunker {
            blocks: Bunker::create_blocks(position),
        }
    }

    fn create_blocks(position: Position) -> Vec<Block> {
        let mut blocks: Vec<Block> = Vec::new();

        for i in 0..ROWS {
            for j in 0..COLUMNS {
                let position =
                    Position {
                        x: position.x + (j as f64 * block::WIDTH),
                        y: position.y + (i as f64 * block::HEIGHT),
                    };

                let pos = (j, i);
                match pos {
                    (1, 2) => {
                        continue;
                    }

                    (0, 0) => {
                        blocks.push(
                            Block::new(position, block::Kind::TopLeft)
                        );
                    }

                    (2, 0) => {
                        blocks.push(
                            Block::new(position, block::Kind::TopRight)
                        );
                    }

                    (0, 2) => {
                        blocks.push(
                            Block::new(position, block::Kind::BottomLeft)
                        );
                    }

                    (2, 2) => {
                        blocks.push(
                            Block::new(position, block::Kind::BottomRight)
                        );
                    }

                    _ => {
                        blocks.push(
                            Block::new(position, block::Kind::Normal)
                        );
                    }
                }
            }
        }

        return blocks;
    }
}

pub struct Bunkers {
    pub bunkers: Vec<Bunker>
}

impl Bunkers {
    pub fn new() -> Bunkers {
        let mut bunkers = Vec::new();

        for i in 0..BUNKERS {
            let position =
                Position {
                    x: POSITION.x + (i as f64 * WIDTH_GAP),
                    y: POSITION.y,
                };

            bunkers.push(Bunker::new(position));
        }

        Bunkers {
            bunkers,
        }
    }

    pub fn clear(&mut self) {
        for bunker in &mut self.bunkers {
            bunker.blocks.retain(|block| {
                match block.state {
                    block::State::Dead => {
                        false
                    }

                    _ => {
                        true
                    }
                }
            });
        }

        self.bunkers.retain(|bunker| {
            !bunker.blocks.is_empty()
        });
    }
}
