use game;
use game::Game;
use game::canon;
use game::wave::alien;
use game::bunkers::block;
use assets::Assets;
use graphics::*;
use opengl_graphics::{GlGraphics, GlyphCache, TextureSettings};
use std::path::Path;

pub struct Draw;

impl Draw {
    pub fn draw(
        game: &Game,
        assets: &Assets,
        c: &Context,
        gl: &mut GlGraphics) {

        Draw::draw_field(c, gl);

        match game.info.state {
            game::State::Over => {
                Draw::draw_game_over(game, assets, c, gl);
            }

            _ => {
                Draw::draw_canon(game, assets, c, gl);
                Draw::draw_wave(game, assets, c, gl);
                Draw::draw_explosions(game, assets, c, gl);
                //Draw::draw_spaceship(game, assets, c, gl);
                Draw::draw_bullets(game, assets, c, gl);
                Draw::draw_bunkers(game, assets, c, gl);
                Draw::draw_info(game, assets, c, gl);
            }
        }
    }

    fn draw_field(c: &Context, gl: &mut GlGraphics) {
        const BLACK: [f32; 4] = [0.0, 0.0, 0.0, 1.0];

        Rectangle::new(BLACK)
            .draw(
                [0.0, 0.0, game::WIDTH, game::HEIGHT],
                &c.draw_state,
                c.transform,
                gl
            );

    }

    fn draw_game_over(
        _game: &Game,
        _assets: &Assets,
        c: &Context,
        gl: &mut GlGraphics) {

        let transform =
            c.transform
                .trans(0.3 * game::WIDTH, 0.5 * game::WIDTH);

        let path = Path::new("./assets/fonts/ca.ttf");
        let mut font =
            GlyphCache::new(path, (), TextureSettings::new())
                .unwrap();

        text::Text::new_color([1.0, 1.0, 1.0, 1.0], 24).draw(
            "game over ",
            &mut font,
            &c.draw_state,
            transform,
            gl
        ).unwrap();
    }

    fn draw_wave(
        game: &Game,
        assets: &Assets,
        c: &Context,
        gl: &mut GlGraphics) {

        for column in &game.wave.aliens {
            for alien in column.iter() {
                let transform =
                    c.transform
                        .trans(
                            alien.position.x - (alien.size.width / 2.0),
                            alien.position.y - (alien.size.height / 2.0)
                        );

                match alien.state {
                    alien::State::ArmsUp => {
                        match alien.kind {
                            alien::Kind::Alpha => {
                                Image::new()
                                    .draw(
                                        &assets.alpha_up,
                                        &c.draw_state,
                                        transform,
                                        gl
                                    );
                            }

                            alien::Kind::Beta => {
                                Image::new()
                                    .draw(
                                        &assets.beta_up,
                                        &c.draw_state,
                                        transform,
                                        gl
                                    );
                            }

                            alien::Kind::Gamma => {
                                Image::new()
                                    .draw(
                                        &assets.gamma_up,
                                        &c.draw_state,
                                        transform,
                                        gl
                                    );
                            }
                        }
                    }

                    alien::State::ArmsDown => {
                        match alien.kind {
                            alien::Kind::Alpha => {
                                Image::new()
                                    .draw(
                                        &assets.alpha_down,
                                        &c.draw_state,
                                        transform,
                                        gl
                                    );
                            }

                            alien::Kind::Beta => {
                                Image::new()
                                    .draw(
                                        &assets.beta_down,
                                        &c.draw_state,
                                        transform,
                                        gl
                                    );
                            }

                            alien::Kind::Gamma => {
                                Image::new()
                                    .draw(
                                        &assets.gamma_down,
                                        &c.draw_state,
                                        transform,
                                        gl
                                    );
                            }
                        }
                    }

                    _ => {}
                }
            }
        }
    }

    fn draw_explosions(
        game: &Game,
        assets: &Assets,
        c: &Context,
        gl: &mut GlGraphics) {

        let explosions = &game.explosions;

        for e in &explosions.explosions {
            let transform =
                c.transform
                    .trans(
                        e.position.x - (e.size.width / 2.0),
                        e.position.y - (e.size.height / 2.0)
                    );

            Image::new()
                .draw(
                    &assets.explosion,
                    &c.draw_state,
                    transform,
                    gl
                );
        }
    }

//    fn draw_spaceship(
//        game: &Game,
//        assets: &Assets,
//        c: &Context,
//        gl: &mut GlGraphics) {
//
//        let spaceship = &game.wave.red_alien;
//
//        let transform =
//            c.transform
//                .trans(
//                    spaceship.position.x - (spaceship.size.width / 2.0),
//                    spaceship.position.y - (spaceship.size.height / 2.0)
//                );
//
//        match spaceship.state {
//            red_alien::State::Active => {
//                Image::new()
//                    .draw(
//                        &assets.spaceship,
//                        &c.draw_state,
//                        transform,
//                        gl
//                    );
//            }
//
//            red_alien::State::Dead => {
//                Image::new()
//                    .draw(
//                        &assets.dead,
//                        &c.draw_state,
//                        transform,
//                        gl
//                    );
//            }
//
//            _ => {}
//        }
//    }

    fn draw_bullets(
        game: &Game,
        assets: &Assets,
        c: &Context,
        gl: &mut GlGraphics) {

        let bullets = &game.bullets;

        for bullet in bullets {
            let transform =
                c.transform
                    .trans(
                        bullet.position.x - (bullet.size.width / 2.0),
                        bullet.position.y - (bullet.size.height / 2.0)
                    );

            Image::new()
                .draw(
                    &assets.bullet,
                    &c.draw_state,
                    transform,
                    gl
                );
        }
    }

    fn draw_canon(
        game: &Game,
        assets: &Assets,
        c: &Context,
        gl: &mut GlGraphics) {

        let canon = &game.canon;

        let transform =
            c.transform
                .trans(
                    canon.position.x - (canon.size.width / 2.0),
                    canon.position.y - (canon.size.height / 2.0)
                );

        match canon.state {
            canon::State::Idle
            | canon::State::MovingLeft
            | canon::State::MovingRight => {

                Image::new()
                    .draw(
                        &assets.canon,
                        &c.draw_state,
                        transform,
                        gl
                    );
            }

            _ => {}
        }
    }

    fn draw_bunkers(
        game: &Game,
        assets: &Assets,
        c: &Context,
        gl: &mut GlGraphics) {

        let bunkers = &game.bunkers;

        for bunker in &bunkers.bunkers {
            for block in &bunker.blocks {
                let transform =
                    c.transform
                        .trans(
                            block.position.x - (block.size.width / 2.0),
                            block.position.y - (block.size.height / 2.0)
                        );

                match block.kind {
                    block::Kind::Normal => {
                        match block.state {
                            block::State::Full => {
                                Image::new()
                                    .draw(
                                        &assets.block_full,
                                        &c.draw_state,
                                        transform,
                                        gl
                                    );
                            }

                            block::State::Half => {
                                Image::new()
                                    .draw(
                                        &assets.block_half,
                                        &c.draw_state,
                                        transform,
                                        gl
                                    );
                            }

                            block::State::Weak => {
                                Image::new()
                                    .draw(
                                        &assets.block_weak,
                                        &c.draw_state,
                                        transform,
                                        gl
                                    );
                            }

                            _ => {}
                        }
                    }

                    block::Kind::TopLeft => {
                        match block.state {
                            block::State::Full => {
                                Image::new()
                                    .draw(
                                        &assets.block_full_tl,
                                        &c.draw_state,
                                        transform,
                                        gl
                                    );
                            }

                            block::State::Half => {
                                Image::new()
                                    .draw(
                                        &assets.block_half_tl,
                                        &c.draw_state,
                                        transform,
                                        gl
                                    );
                            }

                            block::State::Weak => {
                                Image::new()
                                    .draw(
                                        &assets.block_weak_tl,
                                        &c.draw_state,
                                        transform,
                                        gl
                                    );
                            }

                            _ => {}
                        }
                    }

                    block::Kind::TopRight => {
                        match block.state {
                            block::State::Full => {
                                Image::new()
                                    .draw(
                                        &assets.block_full_tr,
                                        &c.draw_state,
                                        transform,
                                        gl
                                    );
                            }

                            block::State::Half => {
                                Image::new()
                                    .draw(
                                        &assets.block_half_tr,
                                        &c.draw_state,
                                        transform,
                                        gl
                                    );
                            }

                            block::State::Weak => {
                                Image::new()
                                    .draw(
                                        &assets.block_weak_tr,
                                        &c.draw_state,
                                        transform,
                                        gl
                                    );
                            }

                            _ => {}
                        }
                    }

                    block::Kind::BottomLeft => {
                        match block.state {
                            block::State::Full => {
                                Image::new()
                                    .draw(
                                        &assets.block_full_bl,
                                        &c.draw_state,
                                        transform,
                                        gl
                                    );
                            }

                            block::State::Half => {
                                Image::new()
                                    .draw(
                                        &assets.block_half_bl,
                                        &c.draw_state,
                                        transform,
                                        gl
                                    );
                            }

                            block::State::Weak => {
                                Image::new()
                                    .draw(
                                        &assets.block_weak_bl,
                                        &c.draw_state,
                                        transform,
                                        gl
                                    );
                            }

                            _ => {}
                        }
                    }

                    block::Kind::BottomRight => {
                        match block.state {
                            block::State::Full => {
                                Image::new()
                                    .draw(
                                        &assets.block_full_br,
                                        &c.draw_state,
                                        transform,
                                        gl
                                    );
                            }

                            block::State::Half => {
                                Image::new()
                                    .draw(
                                        &assets.block_half_br,
                                        &c.draw_state,
                                        transform,
                                        gl
                                    );
                            }

                            block::State::Weak => {
                                Image::new()
                                    .draw(
                                        &assets.block_weak_br,
                                        &c.draw_state,
                                        transform,
                                        gl
                                    );
                            }

                            _ => {}
                        }
                    }
                }
            }
        }
    }

    fn draw_info(
        game: &Game,
        _assets: & Assets,
        c: &Context,
        gl: &mut GlGraphics) {

        let info = &game.info;

        let transform =
            c.transform
                .trans(0.05 * game::WIDTH, 0.05 * game::WIDTH);

        let path = Path::new("./assets/fonts/ca.ttf");
        let mut font =
            GlyphCache::new(path, (), TextureSettings::new())
                .unwrap();

        text::Text::new_color([1.0, 1.0, 1.0, 1.0], 14).draw(
            "score",
            &mut font,
            &c.draw_state,
            transform,
            gl
        ).unwrap();

        let transform =
            c.transform
                .trans(0.21 * game::WIDTH, 0.05 * game::WIDTH);

        text::Text::new_color([0.0, 1.0, 0.0, 1.0], 14).draw(
            &info.score.to_string(),
            &mut font,
            &c.draw_state,
            transform,
            gl
        ).unwrap();

        let transform =
            c.transform
                .trans(0.745 * game::WIDTH, 0.05 * game::WIDTH);

        text::Text::new_color([1.0, 1.0, 1.0, 1.0], 14).draw(
            "canons",
            &mut font,
            &c.draw_state,
            transform,
            gl
        ).unwrap();

        let transform =
            c.transform
                .trans(0.93 * game::WIDTH, 0.05 * game::WIDTH);

        text::Text::new_color([0.0, 1.0, 0.0, 1.0], 14).draw(
            &format!("{}{}", &info.canons.to_string(), " "),
            &mut font,
            &c.draw_state,
            transform,
            gl
        ).unwrap();
    }
}