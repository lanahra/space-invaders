mod bullet;
mod entity;
mod explosion;
mod spaceship;
pub mod bunker;
pub mod canon;
pub mod wave;

use game::entity::*;
use self::bullet::Bullet;
use self::explosion::Explosion;
use self::spaceship::Spaceship;
use self::bunker::Bunker;
use self::bunker::block;
use self::bunker::block::Block;
use self::canon::Canon;
use self::wave::Wave;
use self::wave::alien;
use self::wave::alien::Alien;
use rand::{Rng, thread_rng};
use std::vec::Vec;

pub const WIDTH: f64 = 945.0;
pub const HEIGHT: f64 = 1080.0;

const RAND_STEP: f64 = 0.3;
const PAUSED_STEP: f64 = 2.0;
const MYSTERY_STEP: f64 = 10.0;

#[derive(Clone)]
pub enum Selection {
    NewGame,
    Exit,
}

#[derive(Clone)]
pub enum State {
    Menu(Selection),
    Running,
    Paused,
    Over,
    Restart,
    Exit,
}

#[derive(Clone)]
pub struct Info {
    pub score: u32,
    pub canons: u32,
    pub state: State,
    rand_time: f64,
    paused_time: f64,
    mystery_time: f64,
}

#[derive(Clone)]
pub struct Game {
    pub wave: Wave,
    pub spaceship: Option<Spaceship>,
    pub canon: Canon,
    pub bullets: Vec<Bullet>,
    pub bunkers: Vec<Bunker>,
    pub explosions: Vec<Explosion>,
    pub info: Info,
}

impl Game {
    pub fn new() -> Game {
        Game {
            wave: Wave::new(),
            spaceship: None,
            canon: Canon::new(),
            bullets: Vec::new(),
            bunkers: Bunker::new_bunkers(),
            explosions: Vec::new(),
            info:
                Info {
                    score: 0,
                    canons: 3,
                    state: State::Menu(Selection::NewGame),
                    rand_time: 0.0,
                    paused_time: 0.0,
                    mystery_time: 0.0,
                },
        }
    }

    pub fn change_selection(game: Game) -> Game {
        match game.info.state {
            State::Menu(Selection::NewGame) => {
                Game {
                    info:
                        Info {
                            state: State::Menu(Selection::Exit),
                            ..game.info
                        },
                    ..game
                }
            }

            State::Menu(Selection::Exit) => {
                Game {
                    info:
                        Info {
                            state: State::Menu(Selection::NewGame),
                            ..game.info
                        },
                    ..game
                }
            }

            _ => {
                game
            }
        }
    }

    pub fn make_selection(game: Game) -> Game {
        match game.info.state {
            State::Menu(Selection::NewGame) => {
                Game {
                    info:
                        Info {
                            state: State::Running,
                            ..game.info
                        },
                    ..game
                }
            }

            State::Menu(Selection::Exit) => {
                Game {
                    info:
                        Info {
                            state: State::Exit,
                            ..game.info
                        },
                    ..game
                }
            }

            State::Over => {
                Game {
                    info:
                        Info {
                            state: State::Restart,
                            ..game.info
                        },
                    ..game
                }
            }

            _ => {
                game
            }
        }
    }

    pub fn canon_fire(game: Game) -> Game {
        let position =
            Position {
                y: game.canon.position.y - game.canon.size.height,
                ..game.canon.position
            };

        let bullet =
            Bullet::new(
                position,
                bullet::State::MovingUp
            );

        let mut bullets = game.bullets.to_vec();
        bullets.push(bullet);

        if game.bullets.iter().any(|b| {
            match b.state {
                bullet::State::MovingUp => {
                    true
                }

                _ => {
                    false
                }
            }
        }) {
            game
        } else {
            Game {
                bullets,
                ..game
            }
        }
    }

    fn update_rand_time(dt: f64, game: Game) -> Game {
        Game {
            info:
                Info {
                    rand_time: game.info.rand_time + dt,
                    ..game.info
                },
            ..game
        }
    }

    fn alien_fire(dt: f64, game: Game) -> Game {
        let ts =
            Wave::len(game.wave.clone()) as f64 / (2 * wave::ROWS * wave::COLUMNS) as f64;

        let mut rng = thread_rng();
        let x: f64 = rng.gen();

        let game = Game::update_rand_time(dt, game);
        if game.info.rand_time > RAND_STEP {
            let game = Game::update_rand_time(-RAND_STEP, game);

            if x > ts + 0.3 {
                let column: usize =
                    rng.gen_range(0, game.wave.aliens.len()) as usize;

                if let Some(alien) = game.clone().wave.aliens[column].last() {
                    let position =
                        Position {
                            y: alien.position.y + alien.size.height,
                            ..alien.position
                        };

                    let bullet =
                        Bullet::new(
                            position,
                            bullet::State::MovingDown
                        );

                    let mut bullets = game.bullets.to_vec();
                    bullets.push(bullet);

                    Game {
                        bullets,
                        ..game
                    }
                } else {
                    game
                }
            } else {
                game
            }
        } else {
            game
        }
    }

    fn update_bullets(
        dt: f64,
        bullets: Vec<Bullet>) -> Vec<Bullet> {

        bullets.iter().map(|&b| {
            Bullet::update(dt, b)
        }).collect()
    }

    fn update_mystery_time(dt: f64, game: Game) -> Game {
        Game {
            info:
                Info {
                    mystery_time: game.info.mystery_time + dt,
                    ..game.info
                },
            ..game
        }
    }

    fn handle_spaceship(dt: f64, game: Game) -> Game {
        if let Some(spaceship) = game.spaceship {
            Game {
                spaceship: Some(Spaceship::update(dt, spaceship)),
                ..game
            }
        } else {
            let game = Game::update_mystery_time(dt, game);

            if game.info.mystery_time > MYSTERY_STEP {
                let game = Game::update_mystery_time(-MYSTERY_STEP, game);

                let mut rng = thread_rng();
                let x: f64 = rng.gen();

                let position =
                    if x > 0.5 {
                        Position {
                            x: WIDTH,
                            y: 200.0,
                        }
                    } else {
                        Position {
                            x: 0.0,
                            y: 200.0,
                        }
                    };

                let state =
                    if x > 0.5 {
                        spaceship::State::MovingLeft
                    } else {
                        spaceship::State::MovingRight
                    };

                Game {
                    spaceship: Some(Spaceship::new(position, state)),
                    ..game
                }
            } else {
                game
            }
        }
    }

    fn handle_collisions(game: Game) -> Game {
        struct Overlaps<'a> {
            aliens: &'a Fn(&Overlaps, Bullet, Vec<Alien>) -> bool,
            wave: &'a Fn(&Overlaps, Bullet, Vec<Vec<Alien>>) -> bool,
            blocks: &'a Fn(&Overlaps, Bullet, Vec<Block>) -> bool,
            bunkers: &'a Fn(&Overlaps, Bullet, Vec<Bunker>) -> bool,
        }

        let overlaps =
            Overlaps {
                aliens: &|overlaps, bullet, aliens| {
                    if aliens.len() >= 1 {
                        let (head, tail) = aliens.split_at(1);
                        let alien = head[0];

                        if bullet.overlaps(alien) {
                            true
                        } else {
                            (overlaps.aliens)(overlaps, bullet, tail.to_vec())
                        }
                    } else {
                        false
                    }
                },

                wave: &|overlaps, bullet, wave| {
                    if wave.len() >= 1 {
                        let (head, tail) = wave.split_at(1);
                        let aliens = &head[0];

                        if (overlaps.aliens)(overlaps, bullet, aliens.to_vec()) {
                            true
                        } else {
                            (overlaps.wave)(overlaps, bullet, tail.to_vec())
                        }
                    } else {
                        false
                    }
                },

                blocks: &|overlaps, bullet, blocks| {
                    if blocks.len() >= 1 {
                        let (head, tail) = blocks.split_at(1);
                        let block = head[0];

                        if bullet.overlaps(block) {
                            true
                        } else {
                            (overlaps.blocks)(overlaps, bullet, tail.to_vec())
                        }
                    } else {
                        false
                    }
                },

                bunkers: &|overlaps, bullet, bunkers| {
                    if bunkers.len() >= 1 {
                        let (head, tail) = bunkers.split_at(1);
                        let blocks = &head[0].blocks;

                        if (overlaps.blocks)(overlaps, bullet, blocks.to_vec()) {
                            true
                        } else {
                            (overlaps.bunkers)(overlaps, bullet, tail.to_vec())
                        }
                    } else {
                        false
                    }
                },
            };

        let bullets =
            game.bullets.clone().into_iter().filter(|&bullet| {
                if bullet.position.y >= 1006.0 || bullet.position.y <= 0.0 {
                    return false;
                }

                if let Some(spaceship) = game.spaceship {
                    if bullet.overlaps(spaceship) {
                        return false;
                    }
                }

                if bullet.overlaps(game.canon) {
                    return false;
                }

                if (overlaps.wave)(&overlaps, bullet, game.wave.aliens.clone()) {
                    return false;
                }

                if (overlaps.bunkers)(&overlaps, bullet, game.bunkers.clone()) {
                    return false;
                }

                return true;
            }).collect();

        struct Hit<'a> {
            spaceship:
                &'a Fn(&Hit, Option<Spaceship>, Vec<Explosion>, u32, Vec<Bullet>)
                    -> (Option<Spaceship>, Vec<Explosion>, u32),
            alien:
                &'a Fn(&Hit, Alien, Vec<Explosion>, u32, Vec<Bullet>)
                    -> (Alien, Vec<Explosion>, u32),
            wave_aliens:
                &'a Fn(&Hit, Vec<Alien>, Vec<Explosion>, u32, Vec<Bullet>)
                    -> (Vec<Alien>, Vec<Explosion>, u32),
            wave:
                &'a Fn(&Hit, Vec<Vec<Alien>>, Vec<Explosion>, u32, Vec<Bullet>)
                    -> (Vec<Vec<Alien>>, Vec<Explosion>, u32),
            block_bullets: &'a Fn(&Hit, Block, Vec<Bullet>) -> Block,
            block_aliens: &'a Fn(&Hit, Block, Vec<Alien>) -> Block,
            block_wave: &'a Fn(&Hit, Block, Vec<Vec<Alien>>) -> Block,
            blocks:
                &'a Fn(&Hit, Vec<Block>, Vec<Bullet>, Vec<Vec<Alien>>)
                    -> Vec<Block>,
            bunkers:
                &'a Fn(&Hit, Vec<Bunker>, Vec<Bullet>, Vec<Vec<Alien>>)
                    -> Vec<Bunker>,
            canon_aliens:
                &'a Fn(&Hit, Canon, Vec<Alien>) -> Canon,
            canon_wave:
                &'a Fn(&Hit, Canon, Vec<Vec<Alien>>) -> Canon,
            canon_bullets:
                &'a Fn(&Hit, Canon, Vec<Bullet>) -> Canon,
            canon:
                &'a Fn(&Hit, Canon, Vec<Bullet>, Vec<Vec<Alien>>) -> Canon,
        }

        let hit =
            Hit {
                spaceship: &|hit, spaceship, explosions, score, bullets| {
                    if let Some(s) = spaceship {
                        if s.position.x > WIDTH || s.position.x < 0.0 {
                            (None, explosions, score)
                        } else if bullets.len() >= 1 {
                            let (head, tail) = bullets.split_at(1);
                            let bullet = head[0];

                            if bullet.overlaps(s) {
                                let explosion =
                                    Explosion::new(s.position);

                                let mut explosions = explosions.to_vec();
                                explosions.push(explosion);

                                (None, explosions, score + 100)
                            } else {
                                (hit.spaceship)(
                                    hit,
                                    spaceship,
                                    explosions,
                                    score,
                                    tail.to_vec()
                                )
                            }
                        } else {
                            (spaceship, explosions, score)
                        }
                    } else {
                        (None, explosions, score)
                    }
                },

                alien: &|hit, alien, explosions, score, bullets| {
                    if alien.position.y > 986.0 {
                        (alien, explosions, score)
                    } else if bullets.is_empty() {
                        (alien, explosions, score)
                    } else {
                        let (head, tail) = bullets.split_at(1);
                        let bullet = head[0];

                        if bullet.overlaps(alien) {
                            let explosion =
                                Explosion::new(alien.position);

                            let mut explosions = explosions.to_vec();
                            explosions.push(explosion);

                            let alien =
                                Alien {
                                    state: alien::State::Dead,
                                    ..alien
                                };

                            match alien.kind {
                                alien::Kind::Alpha => {
                                    (alien, explosions, score + 30)
                                }

                                alien::Kind::Beta => {
                                    (alien, explosions, score + 20)
                                }

                                alien::Kind::Gamma => {
                                    (alien, explosions, score + 10)
                                }
                            }
                        } else {
                            (hit.alien)(
                                hit,
                                alien,
                                explosions,
                                score,
                                tail.to_vec()
                            )
                        }
                    }
                },

                wave_aliens: &|hit, aliens, explosions, score, bullets| {
                    if aliens.is_empty() {
                        (Vec::new(), explosions, score)
                    } else {
                        let (head, tail) = aliens.split_at(1);
                        let alien = head[0];

                        let (alien, explosions, score) =
                            (hit.alien)(
                                hit,
                                alien,
                                explosions,
                                score,
                                bullets.clone()
                            );

                        let (aliens, explosions, score) =
                            (hit.wave_aliens)(
                                hit,
                                tail.to_vec(),
                                explosions,
                                score,
                                bullets
                            );

                        match alien.state {
                            alien::State::Dead => {
                                (aliens, explosions, score)
                            }

                            _ => {
                                if aliens.is_empty() {
                                    (vec![alien], explosions, score)
                                } else {
                                    let mut aliens = aliens.to_vec();
                                    aliens.insert(0, alien);

                                    (aliens, explosions, score)
                                }
                            }
                        }
                    }
                },

                wave: &|hit, wave, explosions, score, bullets| {
                    if wave.is_empty() {
                        (Vec::new(), explosions, score)
                    } else {
                        let (head, tail) = wave.split_at(1);
                        let aliens = &head[0];

                        let (aliens, explosions, score) =
                            (hit.wave_aliens)(
                                hit,
                                aliens.to_vec(),
                                explosions,
                                score,
                                bullets.clone()
                            );

                        let (columns, explosions, score) =
                            (hit.wave)(
                                hit,
                                tail.to_vec(),
                                explosions,
                                score,
                                bullets
                            );

                        if aliens.is_empty() {
                            (columns, explosions, score)
                        } else if columns.is_empty() {
                            (vec![aliens], explosions, score)
                        } else {
                            let mut columns = columns.to_vec();
                            columns.insert(0, aliens);

                            (columns, explosions, score)
                        }
                    }
                },

                block_bullets: &|hit, block, bullets| {
                    if bullets.is_empty() {
                        block
                    } else {
                        let (head, tail) = bullets.split_at(1);
                        let bullet = head[0];

                        if bullet.overlaps(block) {
                            Block::change_state(block)
                        } else {
                            (hit.block_bullets)(hit, block, tail.to_vec())
                        }
                    }
                },

                block_aliens: &|hit, block, aliens| {
                    if aliens.is_empty() {
                        block
                    } else {
                        let (head, tail) = aliens.split_at(1);
                        let alien = head[0];

                        if alien.overlaps(block) {
                            Block::change_state(block)
                        } else {
                            (hit.block_aliens)(hit, block, tail.to_vec())
                        }
                    }
                },

                block_wave: &|hit, block, wave| {
                    if wave.is_empty() {
                        block
                    } else {
                        let (head, tail) = wave.split_at(1);
                        let aliens = &head[0];

                        let block =
                            (hit.block_aliens)(hit, block, aliens.to_vec());
                        let block =
                            (hit.block_wave)(hit, block, tail.to_vec());

                        block
                    }
                },

                blocks: &|hit, blocks, bullets, wave| {
                    if blocks.is_empty() {
                        Vec::new()
                    } else {
                        let (head, tail) = blocks.split_at(1);
                        let block = head[0];

                        let block =
                            (hit.block_bullets)(hit, block, bullets.clone());
                        let block =
                            (hit.block_wave)(hit, block, wave.clone());

                        let blocks =
                            (hit.blocks)(hit, tail.to_vec(), bullets, wave);

                        match block.state {
                            block::State::Dead => {
                                blocks
                            }

                            _ => {
                                if blocks.is_empty() {
                                    vec![block]
                                } else {
                                    let mut blocks = blocks.to_vec();
                                    blocks.insert(0, block);

                                    blocks
                                }
                            }
                        }
                    }
                },

                bunkers: &|hit, bunkers, bullets, wave| {
                    if bunkers.is_empty() {
                        return Vec::new();
                    } else {
                        let (head, tail) = bunkers.split_at(1);
                        let blocks = &head[0].blocks;

                        let blocks =
                            (hit.blocks)(
                                hit,
                                blocks.to_vec(),
                                bullets.clone(),
                                wave.clone()
                            );
                        let bunker =
                            Bunker {
                                blocks
                            };

                        let bunkers =
                            (hit.bunkers)(hit, tail.to_vec(), bullets, wave);
                        if bunker.blocks.is_empty() {
                            return bunkers;
                        } if bunkers.len() >= 1 {
                            let mut bunkers = bunkers.to_vec();
                            bunkers.insert(0, bunker);

                            return bunkers;
                        } else {
                            return vec![bunker];
                        }
                    }
                },

                canon_aliens: &|hit, canon, aliens| {
                    if aliens.is_empty() {
                        canon
                    } else {
                        let (head, tail) = aliens.split_at(1);
                        let alien = head[0];

                        if alien.overlaps(canon) {
                            Canon {
                                state: canon::State::Dead,
                                ..canon
                            }
                        } else {
                            (hit.canon_aliens)(hit, canon, tail.to_vec())
                        }
                    }
                },

                canon_wave: &|hit, canon, wave| {
                    if wave.is_empty() {
                        canon
                    } else {
                        let (head, tail) = wave.split_at(1);
                        let aliens = &head[0];

                        let canon =
                            (hit.canon_aliens)(hit, canon, aliens.to_vec());

                        match canon.state {
                            canon::State::Dead => {
                                canon
                            }

                            _ => {
                                (hit.canon_wave)(hit, canon, tail.to_vec())
                            }
                        }
                    }
                },

                canon_bullets: &|hit, canon, bullets| {
                    if bullets.is_empty() {
                        canon
                    } else {
                        let (head, tail) = bullets.split_at(1);
                        let bullet = head[0];

                        if bullet.overlaps(canon) {
                            Canon {
                                state: canon::State::Dead,
                                ..canon
                            }
                        } else {
                            (hit.canon_bullets)(hit, canon, tail.to_vec())
                        }
                    }
                },

                canon: &|hit, canon, bullets, wave| {
                    let canon = (hit.canon_bullets)(hit, canon, bullets);

                    match canon.state {
                        canon::State::Dead => {
                            canon
                        }

                        _ => {
                            (hit.canon_wave)(hit, canon, wave)
                        }
                    }
                },
            };

        let (spaceship, explosions, score) =
            (hit.spaceship)(
                &hit,
                game.spaceship,
                game.explosions.clone(),
                game.info.score,
                game.bullets.clone()
            );

        let (aliens, explosions, score) =
            (hit.wave)(
                &hit,
                game.wave.aliens.clone(),
                explosions,
                score,
                game.bullets.clone()
            );

        let bunkers =
            (hit.bunkers)(
                &hit,
                game.bunkers.clone(),
                game.bullets.clone(),
                game.wave.aliens.clone()
            );

        let canon =
            (hit.canon)(
                &hit,
                game.canon,
                game.bullets.clone(),
                game.wave.aliens.clone(),
            );

        let (state, canons) =
            match canon.state {
                canon::State::Dead => {
                    if game.info.canons > 1 {
                        (State::Paused, game.info.canons - 1)
                    } else {
                        (State::Over, 0)
                    }
                }

                _ => {
                    (State::Running, game.info.canons)
                }
            };

        let (wave, canons, score) =
            if aliens.is_empty() {
                (Wave::new(), canons + 1, score + 100)
            } else {
                let wave =
                    Wave {
                        aliens,
                        ..game.wave
                    };

                (wave, canons, score)
            };

        struct Out<'a> {
            aliens: &'a Fn(&Out, Vec<Alien>) -> bool,
            wave: &'a Fn(&Out, Vec<Vec<Alien>>) -> bool,
        }

        let out =
            Out {
                aliens: &|out, aliens| {
                    if aliens.is_empty() {
                        false
                    } else {
                        let (head, tail) = aliens.split_at(1);
                        let alien = head[0];

                        if alien.position.y > 986.0 {
                            true
                        } else {
                            (out.aliens)(out, tail.to_vec())
                        }
                    }
                },

                wave: &|out, wave| {
                    if wave.is_empty() {
                        false
                    } else {
                        let (head, tail) = wave.split_at(1);
                        let aliens = &head[0];

                        (out.aliens)(out, aliens.to_vec())
                            || (out.wave)(out, tail.to_vec())
                    }
                },
            };

        let state =
            if (out.wave)(&out, game.wave.aliens) {
                State::Over
            } else {
                state
            };

        Game {
            wave,
            canon,
            bullets,
            explosions,
            spaceship,
            bunkers,
            info:
                Info {
                    state,
                    canons,
                    score,
                    ..game.info
                },
            ..game
        }
    }

    fn update_paused_time(dt: f64, game: Game) -> Game {
        Game {
            info:
                Info {
                    paused_time: game.info.paused_time + dt,
                    ..game.info
                },
            ..game
        }
    }

    pub fn update(dt: f64, game: Game) -> Game {
        match game.info.state {
            State::Running => {
                let game = Game::handle_spaceship(dt, game);
                let game = Game::alien_fire(dt, game);
                let game = Game::handle_collisions(game);

                Game {
                    wave: Wave::update(dt, game.wave),
                    canon: Canon::update(dt, game.canon),
                    bullets: Game::update_bullets(dt, game.bullets),
                    explosions: Explosion::update(dt, game.explosions),
                    ..game
                }
            }

            State::Paused => {
                let game = Game::update_paused_time(dt, game);

                if game.info.paused_time > PAUSED_STEP {
                    Game {
                        explosions: Explosion::update(dt, game.explosions),
                        canon: Canon::new(),
                        bullets: Vec::new(),
                        info:
                            Info {
                                state: State::Running,
                                paused_time: 0.0,
                                ..game.info
                            },
                        ..game
                    }
                } else {
                    Game {
                        explosions: Explosion::update(dt, game.explosions),
                        ..game
                    }
                }
            }

            _ => {
                game
            }
        }
    }
}
