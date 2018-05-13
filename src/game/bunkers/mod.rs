pub mod block;

use game;
use game::position::Position;
use std::vec::Vec;
use self::block::Block;

const BUNKERS: u32 = 4;

const POSITION: Position =
    Position {
        x: 0.1 * game::WIDTH,
        y: 0.75 * game::HEIGHT,
    };

const ROWS: u32 = 3;
const COLUMNS: u32 = 4;
const WIDTH_GAP: f64 = 0.25 * game::WIDTH;

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

                let pos = (i, j);
                match pos {
                    (2, 1) | (2, 2) => {
                        continue;
                    }

                    (0, 0) => {
                        blocks.push(
                            Block::new(position, block::Kind::TopLeft)
                        );
                    }

                    (0, 3) => {
                        blocks.push(
                            Block::new(position, block::Kind::TopRight)
                        );
                    }

                    (1, 1) => {
                        blocks.push(
                            Block::new(position, block::Kind::BottomRight)
                        );
                    }

                    (1, 2) => {
                        blocks.push(
                            Block::new(position, block::Kind::BottomLeft)
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
