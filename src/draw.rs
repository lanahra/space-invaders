use super::App;
use game;
use game::Game;
use game::canon;
use game::wave::alien;
use game::bunkers::block;
use graphics;
use assets::Assets;
use graphics::*;
use opengl_graphics::GlGraphics;
use piston_window::*;

const BLACK: [f32; 4] = [0.0, 0.0, 0.0, 1.0];
const WHITE: [f32; 4] = [1.0, 1.0, 1.0, 1.0];
const GREEN: [f32; 4] = [0.0, 1.0, 0.0, 1.0];

pub struct Draw;

impl Draw {
    pub fn render(app: &mut App, args: &RenderArgs) {
        let scale = args.height as f64 / game::HEIGHT;
        let offset =
            (args.width / 2) as i32 - (game::WIDTH * scale / 2.0) as i32;

        let width = game::WIDTH * scale;
        let height = game::HEIGHT * scale;

        let viewport =
            Viewport {
                rect: [offset, 0, width as i32, height as i32],
                window_size: [width as u32, height as u32],
                draw_size: [width as u32, height as u32],
            };

        let highscores = &app.highscores; 
        let game = &app.game;
        let mut assets = &mut app.assets;

        app.gl.draw(viewport, |mut c, gl| {
            graphics::clear(WHITE, gl);

            let transform =
                c.transform
                    .zoom(scale);

            c.transform = transform;

            Draw::draw(highscores, &game, &mut assets, &c, gl);
        });
    }

    fn draw(
        highscores: &Vec<u32>,
        game: &Game,
        assets: &mut Assets,
        c: &Context,
        gl: &mut GlGraphics) {

        Draw::draw_field(c, gl);

        match game.info.state {
            game::State::Running
            | game::State::Paused => {
                Draw::draw_line(c, gl);
                Draw::draw_canon(game, assets, c, gl);
                Draw::draw_canons(game, assets, c, gl);
                Draw::draw_wave(game, assets, c, gl);
                Draw::draw_explosions(game, assets, c, gl);
                Draw::draw_spaceship(game, assets, c, gl);
                Draw::draw_bullets(game, assets, c, gl);
                Draw::draw_bunkers(game, assets, c, gl);
                Draw::draw_score(game, assets, c, gl);
            }

            game::State::Over => {
                Draw::draw_game_over(game, assets, c, gl);
            }

            game::State::Records => {
                Draw::draw_records(highscores, game, assets, c, gl);
            }            

            game::State::Menu(ref _s) => {
                Draw::draw_menu(game, assets, c, gl);
            }

            _ => {}
        }
    }

    fn draw_field(c: &Context, gl: &mut GlGraphics) {
        Rectangle::new(BLACK)
            .draw(
                [0.0, 0.0, game::WIDTH, game::HEIGHT],
                &c.draw_state,
                c.transform,
                gl
            );

    }

    fn draw_line(c: &Context, gl: &mut GlGraphics) {
        let transform =
            c.transform
                .trans(0.0, 1006.0);

        Rectangle::new(GREEN)
            .draw(
                [0.0, 0.0, game::WIDTH, 4.0],
                &c.draw_state,
                transform,
                gl
            );
    }

    fn draw_canons(
        game: &Game,
        assets: &mut Assets,
        c: &Context,
        gl: &mut GlGraphics) {

        let transform =
            c.transform
                .trans(25.0, 1045.0);

        text::Text::new_color(WHITE, 34).draw(
            &game.info.canons.to_string(),
            &mut assets.font,
            &c.draw_state,
            transform,
            gl
        ).unwrap();

        for i in 0..game.info.canons - 1 {
            let transform =
                c.transform
                    .trans((i * 70) as f64 + 105.0, 1010.0);

            Image::new()
                .draw(
                    &assets.canon,
                    &c.draw_state,
                    transform,
                    gl
                );
        }
    }

    fn draw_game_over(
        game: &Game,
        assets: &mut Assets,
        c: &Context,
        gl: &mut GlGraphics) {

        let transform =
            c.transform
                .trans(305.0, 450.0);

        text::Text::new_color(WHITE, 50).draw(
            "game over",
            &mut assets.font,
            &c.draw_state,
            transform,
            gl
        ).unwrap();

        let transform =
            c.transform
                .trans(340.0, 550.0);

        text::Text::new_color(WHITE, 34).draw(
            "final score",
            &mut assets.font,
            &c.draw_state,
            transform,
            gl
        ).unwrap();

        let transform =
            c.transform
                .trans(410.0, 600.0);

        text::Text::new_color(WHITE, 34).draw(
            &format!("{:04}", game.info.score),
            &mut assets.font,
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

    fn draw_spaceship(
        game: &Game,
        assets: &Assets,
        c: &Context,
        gl: &mut GlGraphics) {

        if let Some(ref spaceship) = game.spaceship {
            let transform =
                c.transform
                    .trans(
                        spaceship.position.x - (spaceship.size.width / 2.0),
                        spaceship.position.y - (spaceship.size.height / 2.0)
                    );

            Image::new()
                .draw(
                    &assets.spaceship,
                    &c.draw_state,
                    transform,
                    gl
                );
        }
    }

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

    fn draw_score(
        game: &Game,
        assets: &mut Assets,
        c: &Context,
        gl: &mut GlGraphics) {

        let info = &game.info;

        let transform =
            c.transform
                .trans(30.0, 38.0);

        text::Text::new_color(WHITE, 34).draw(
            "score",
            &mut assets.font,
            &c.draw_state,
            transform,
            gl
        ).unwrap();

        let transform =
            c.transform
                .trans(45.0, 100.0);

        text::Text::new_color(WHITE, 34).draw(
            &format!("{:04}", info.score),
            &mut assets.font,
            &c.draw_state,
            transform,
            gl
        ).unwrap();
    }

    fn draw_menu(
        game: &Game,
        assets: &mut Assets,
        c: &Context,
        gl: &mut GlGraphics) {

        let transform =
            c.transform
                .trans(150.0, 400.0);

        text::Text::new_color(WHITE, 50).draw(
            "space invaders",
            &mut assets.font,
            &c.draw_state,
            transform,
            gl
        ).unwrap();

        let transform =
            c.transform
                .trans(150.0, 600.0);

        text::Text::new_color(WHITE, 34).draw(
            "new game",
            &mut assets.font,
            &c.draw_state,
            transform,
            gl
        ).unwrap();


        let transform =
            c.transform
                .trans(150.0, 650.0);

        text::Text::new_color(WHITE, 34).draw(
            "records",
            &mut assets.font,
            &c.draw_state,
            transform,
            gl
        ).unwrap();

        let transform =
            c.transform
                .trans(150.0, 700.0);

        text::Text::new_color(WHITE, 34).draw(
            "exit",
            &mut assets.font,
            &c.draw_state,
            transform,
            gl
        ).unwrap();

        let transform = match game.info.state {
            game::State::Menu(game::Selection::NewGame) => {
                c.transform
                    .trans(120.0, 600.0)
            }

            game::State::Menu(game::Selection::Records) => {
                c.transform
                    .trans(120.0, 650.0)
            }

            game::State::Menu(game::Selection::Exit) => {
                c.transform
                    .trans(120.0, 700.0)
            }

            _ => {
                c.transform
            }
        };

        text::Text::new_color(WHITE, 34).draw(
            ">",
            &mut assets.font,
            &c.draw_state,
            transform,
            gl
        ).unwrap();
    }

    fn draw_records(
        highscores: &Vec<u32>,
        game: &Game,
        assets: &mut Assets,
        c: &Context,
        gl: &mut GlGraphics) {

        let mut position = 300.0;

        let transform =
            c.transform
                .trans(150.0, 200.0);

        text::Text::new_color(WHITE, 50).draw(
            "records",
            &mut assets.font,
            &c.draw_state,
            transform,
            gl
        ).unwrap();

        for i in 0..(*highscores).len() as isize {
            let transform =
            c.transform
                .trans(150.0, position);

            text::Text::new_color(WHITE, 34).draw(
                &format!("{:04}", (*highscores)[(i as usize)]),
                &mut assets.font,
                &c.draw_state,
                transform,
                gl
            ).unwrap();

            position += 55.0;
        }

        let transform =
            c.transform
                .trans(150.0, 900.0);

        text::Text::new_color(WHITE, 34).draw(
            "exit",
            &mut assets.font,
            &c.draw_state,
            transform,
            gl
        ).unwrap();

        let transform =
            c.transform
                .trans(120.0, 900.0);

        text::Text::new_color(WHITE, 34).draw(
            ">",
            &mut assets.font,
            &c.draw_state,
            transform,
            gl
        ).unwrap();

    }
}
