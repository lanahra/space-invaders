use game;
use game::Game;
use game::entity::Entity;
use game::entity::statical::bunker::block;
use game::wave::alien;
use assets::Assets;
use graphics::*;
use opengl_graphics::GlGraphics;
use opengl_graphics::*;
use std::path::Path;

pub struct Draw;

impl Draw {
    pub fn draw(
        game: &Game,
        assets: &Assets,
        c: &Context,
        gl: &mut GlGraphics) {

        Draw::draw_field(c, gl);
        Draw::draw_canon(game, assets, c, gl);
        Draw::draw_wave(game, assets, c, gl);
//        Draw::draw_spaceship(game, assets, c, gl);
        Draw::draw_bullets(game, assets, c, gl);
        Draw::draw_bunkers(game, assets, c, gl);
        Draw::draw_score(game, assets, c, gl);
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

            Image::new()
                .draw(
                    &assets.canon,
                    &c.draw_state,
                    transform,
                    gl
                );
    }

    fn draw_bunkers(
        game: &Game,
        assets: &Assets,
        c: &Context,
        gl: &mut GlGraphics) {

        let bunkers = &game.bunkers;

        for bunker in bunkers.iter() {
            for block in bunker.iter() {
                let transform =
                    c.transform
                        .trans(
                            block.position.x - (block.size.width / 2.0),
                            block.position.y - (block.size.height / 2.0)
                        );

                match block.kind {
                    block::Kind::Normal => {
                        match block.state {
                            block::State::Strong => {
                                Image::new()
                                    .draw(
                                        &assets.barrier_full,
                                        &c.draw_state,
                                        transform,
                                        gl
                                    );
                            }

                            block::State::HalfLife => {
                                Image::new()
                                    .draw(
                                        &assets.barrier_ok,
                                        &c.draw_state,
                                        transform,
                                        gl
                                    );
                            }

                            block::State::Weak => {
                                Image::new()
                                    .draw(
                                        &assets.barrier_weak,
                                        &c.draw_state,
                                        transform,
                                        gl
                                    );
                            }

                            _ => {}
                        }
                    }

                    block::Kind::TopLeftCorner => {
                        match block.state {
                            block::State::Strong => {
                                Image::new()
                                    .draw(
                                        &assets.barrier_full_tl,
                                        &c.draw_state,
                                        transform,
                                        gl
                                    );
                            }

                            block::State::HalfLife => {
                                Image::new()
                                    .draw(
                                        &assets.barrier_ok_tl,
                                        &c.draw_state,
                                        transform,
                                        gl
                                    );
                            }

                            block::State::Weak => {
                                Image::new()
                                    .draw(
                                        &assets.barrier_weak_tl,
                                        &c.draw_state,
                                        transform,
                                        gl
                                    );
                            }

                            _ => {}
                        }
                    }

                    block::Kind::TopRightCorner => {
                        match block.state {
                            block::State::Strong => {
                                Image::new()
                                    .draw(
                                        &assets.barrier_full_tr,
                                        &c.draw_state,
                                        transform,
                                        gl
                                    );
                            }

                            block::State::HalfLife => {
                                Image::new()
                                    .draw(
                                        &assets.barrier_ok_tr,
                                        &c.draw_state,
                                        transform,
                                        gl
                                    );
                            }

                            block::State::Weak => {
                                Image::new()
                                    .draw(
                                        &assets.barrier_weak_tr,
                                        &c.draw_state,
                                        transform,
                                        gl
                                    );
                            }

                            _ => {}
                        }
                    }

                    block::Kind::BottomLeftCorner => {
                        match block.state {
                            block::State::Strong => {
                                Image::new()
                                    .draw(
                                        &assets.barrier_full_bl,
                                        &c.draw_state,
                                        transform,
                                        gl
                                    );
                            }

                            block::State::HalfLife => {
                                Image::new()
                                    .draw(
                                        &assets.barrier_ok_bl,
                                        &c.draw_state,
                                        transform,
                                        gl
                                    );
                            }

                            block::State::Weak => {
                                Image::new()
                                    .draw(
                                        &assets.barrier_weak_bl,
                                        &c.draw_state,
                                        transform,
                                        gl
                                    );
                            }

                            _ => {}
                        }
                    }

                    block::Kind::BottomRightCorner => {
                        match block.state {
                            block::State::Strong => {
                                Image::new()
                                    .draw(
                                        &assets.barrier_full_br,
                                        &c.draw_state,
                                        transform,
                                        gl
                                    );
                            }

                            block::State::HalfLife => {
                                Image::new()
                                    .draw(
                                        &assets.barrier_ok_br,
                                        &c.draw_state,
                                        transform,
                                        gl
                                    );
                            }

                            block::State::Weak => {
                                Image::new()
                                    .draw(
                                        &assets.barrier_weak_br,
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

    fn draw_score(
        game: &Game,
        _assets: & Assets,
        c: &Context,
        gl: &mut GlGraphics) {

        let player_info = &game.player_info;

        let transform =
            c.transform
                .trans(0.05 * game::WIDTH, 0.05 * game::WIDTH);

        let score_text = String::from("Score");
        let score_text_print = &score_text[..];

        let path = Path::new("./assets/fonts/ca.ttf");
        let mut font =
            GlyphCache::new(path, (), TextureSettings::new())
                .unwrap();

        text::Text::new_color([255.0, 255.0, 255.0, 1.0], 14).draw(
            score_text_print,
            &mut font,
            &c.draw_state,
            transform,
            gl
        ).unwrap();

        let transform =
            c.transform
                .trans(0.21 * game::WIDTH, 0.05 * game::WIDTH);

        let score = String::from(player_info.points.to_string());
        let score_print = &score[..];

        text::Text::new_color([0.0, 255.0, 0.0, 1.0], 14).draw(
            score_print,
            &mut font,
            &c.draw_state,
            transform,
            gl
        ).unwrap();

        let transform =
            c.transform
                .trans(0.78 * game::WIDTH, 0.05 * game::WIDTH);

        let lifes_text = String::from("Lifes");
        let lifes_text_print = &lifes_text[..];

        text::Text::new_color([255.0, 255.0, 255.0, 1.0], 14).draw(
            lifes_text_print,
            &mut font,
            &c.draw_state,
            transform,
            gl
        ).unwrap();

        let transform =
            c.transform
                .trans(0.93 * game::WIDTH, 0.05 * game::WIDTH);


        let lifes = String::from(player_info.lifes.to_string());
        let lifes_print = &lifes[..];

        text::Text::new_color([0.0, 255.0, 0.0, 1.0], 14).draw(
            lifes_print,
            &mut font,
            &c.draw_state,
            transform,
            gl
        ).unwrap();
    }
}
