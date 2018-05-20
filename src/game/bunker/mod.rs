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

const WIDTH_GAP: f64 = 208.0;

#[derive(Clone)]
pub struct Bunker {
    pub blocks: Vec<Block>,
}

impl Bunker {
    fn new(position: Position) -> Bunker {
        Bunker {
            blocks: Bunker::new_blocks(position),
        }
    }

    fn new_blocks(position: Position) -> Vec<Block> {
        let positions =
            vec![ (0, 0)
                , (0, 1)
                , (0, 2)
                , (1, 0)
                , (1, 1)
                , (2, 0)
                , (2, 1)
                , (2, 2)
                ];

        let new_block = |pos| {
            let (j, i) = pos;
            let position =
                Position {
                    x: position.x + (j as f64 * block::WIDTH),
                    y: position.y + (i as f64 * block::HEIGHT),
                };

            match pos {
                (0, 0) => {
                    Block::new(position, block::Kind::TopLeft)
                }

                (2, 0) => {
                    Block::new(position, block::Kind::TopRight)
                }

                (0, 2) => {
                    Block::new(position, block::Kind::BottomLeft)
                }

                (2, 2) => {
                    Block::new(position, block::Kind::BottomRight)
                }

                _ => {
                    Block::new(position, block::Kind::Normal)
                }
            }
        };

        struct Add<'a> {
            f: &'a Fn(&Add, Vec<(u32, u32)>, Vec<Block>) -> Vec<Block>,
        }

        let add_blocks =
            Add {
                f: &|add, positions, blocks| {
                    if positions.len() >= 1 {
                        let (head, tail) = positions.split_at(1);

                        let block = new_block(head[0]);
                        let mut blocks = blocks.to_vec();
                        blocks.push(block);

                        (add.f)(add, tail.to_vec(), blocks)
                    } else {
                        blocks
                    }
                },
            };

        (add_blocks.f)(&add_blocks, positions, Vec::new())
    }


    pub fn new_bunkers() -> Vec<Bunker> {
        let positions = (0..BUNKERS).collect();

        let new_bunker = |i| {
            let position =
                Position {
                    x: POSITION.x + (i as f64 * WIDTH_GAP),
                    y: POSITION.y,
                };

            Bunker::new(position)
        };

        struct Add<'a> {
            f: &'a Fn(&Add, Vec<u32>, Vec<Bunker>) -> Vec<Bunker>,
        }

        let add_bunkers =
            Add {
                f: &|add, positions, bunkers| {
                    if positions.len() >= 1 {
                        let (head, tail) = positions.split_at(1);

                        let bunker = new_bunker(head[0]);
                        let mut bunkers = bunkers.to_vec();
                        bunkers.push(bunker);

                        (add.f)(add, tail.to_vec(), bunkers)
                    } else {
                        bunkers
                    }
                },
            };

        (add_bunkers.f)(&add_bunkers, positions, Vec::new())
    }
}
