extern crate glutin_window;
extern crate graphics;
extern crate opengl_graphics;
extern crate piston;
extern crate piston_window;
extern crate find_folder;

mod game;
mod sprites;

use game::bunker::block;
use game::bunker::Bunker;
use game::wave::alien;
use game::wave::red_alien;
use game::Game;
use game::canon::Canon;
use game::bullet::Shot;
use game::bullet::ShotType;
use game::wave::Wave;
use sprites::Sprites;
use glutin_window::GlutinWindow as Window;
use graphics::*;
use opengl_graphics::{ GlGraphics, OpenGL };
use piston::event_loop::*;
use piston_window::*;
use piston::input::*;
use piston::window::WindowSettings;
use graphics::rectangle::square;
use std::path::Path;
use std::collections::LinkedList;

pub struct App {
    gl: GlGraphics,
    window: PistonWindow ,
    game: Game,
    sprites: Sprites,
}

impl App {
    fn new() -> App {
        let opengl = OpenGL::V3_2;

        // Create an Glutin window.
        let window: PistonWindow  = WindowSettings::new(
                "Space Invaders",
                [game::WIDTH as u32, game::HEIGHT as u32]
            )
            .opengl(opengl)
            .build()
            .unwrap();

        return App {
            window,
            gl: GlGraphics::new(opengl),
            game: Game::new(),
            sprites: Sprites::new(),
        };
    }

    fn run(&mut self) {
        let assets = find_folder::Search::ParentsThenKids(3, 3)
            .for_folder("resources").unwrap();
        let alien_sprite_a1 = assets.join("InvaderA1.png");
        let alien_sprite_a1: G2dTexture = Texture::from_path(
            &mut self.window.factory,
            &alien_sprite_a1,
            Flip::None,
            &TextureSettings::new()
        ).unwrap();

        let alien_sprite_a2 = assets.join("InvaderA2.png");
        let alien_sprite_a2: G2dTexture = Texture::from_path(
            &mut self.window.factory,
            &alien_sprite_a2,
            Flip::None,
            &TextureSettings::new()
        ).unwrap();

        let assets = find_folder::Search::ParentsThenKids(3, 3)
            .for_folder("resources").unwrap();
        let alien_sprite_b1 = assets.join("InvaderB1.png");
        let alien_sprite_b1: G2dTexture = Texture::from_path(
            &mut self.window.factory,
            &alien_sprite_b1,
            Flip::None,
            &TextureSettings::new()
        ).unwrap();

        let alien_sprite_b2 = assets.join("InvaderB2.png");
        let alien_sprite_b2: G2dTexture = Texture::from_path(
            &mut self.window.factory,
            &alien_sprite_b2,
            Flip::None,
            &TextureSettings::new()
        ).unwrap();

        let assets = find_folder::Search::ParentsThenKids(3, 3)
            .for_folder("resources").unwrap();
        let alien_sprite_c1 = assets.join("InvaderC1.png");
        let alien_sprite_c1: G2dTexture = Texture::from_path(
            &mut self.window.factory,
            &alien_sprite_c1,
            Flip::None,
            &TextureSettings::new()
        ).unwrap();

        let alien_sprite_c2 = assets.join("InvaderC2.png");
        let alien_sprite_c2: G2dTexture = Texture::from_path(
            &mut self.window.factory,
            &alien_sprite_c2,
            Flip::None,
            &TextureSettings::new()
        ).unwrap();

        let canon_sprite = assets.join("Ship.png");
        let canon_sprite: G2dTexture = Texture::from_path(
            &mut self.window.factory,
            &canon_sprite,
            Flip::None,
            &TextureSettings::new()
        ).unwrap();

        let bullet_sprite = assets.join("Bullet.png");
        let bullet_sprite: G2dTexture = Texture::from_path(
            &mut self.window.factory,
            &bullet_sprite,
            Flip::None,
            &TextureSettings::new()
        ).unwrap();

        let full_block_sprite = assets.join("FullBlock.png");
        let full_block_sprite: G2dTexture = Texture::from_path(
            &mut self.window.factory,
            &full_block_sprite,
            Flip::None,
            &TextureSettings::new()
        ).unwrap();

        let full_tl_block_sprite = assets.join("FullTLCornerBlock.png");
        let full_tl_block_sprite: G2dTexture = Texture::from_path(
            &mut self.window.factory,
            &full_tl_block_sprite,
            Flip::None,
            &TextureSettings::new()
        ).unwrap();

        let full_tr_block_sprite = assets.join("FullTRCornerBlock.png");
        let full_tr_block_sprite: G2dTexture = Texture::from_path(
            &mut self.window.factory,
            &full_tr_block_sprite,
            Flip::None,
            &TextureSettings::new()
        ).unwrap();

        let full_bl_block_sprite = assets.join("FullBLCornerBlock.png");
        let full_bl_block_sprite: G2dTexture = Texture::from_path(
            &mut self.window.factory,
            &full_bl_block_sprite,
            Flip::None,
            &TextureSettings::new()
        ).unwrap();

        let full_br_block_sprite = assets.join("FullBRCornerBlock.png");
        let full_br_block_sprite: G2dTexture = Texture::from_path(
            &mut self.window.factory,
            &full_br_block_sprite,
            Flip::None,
            &TextureSettings::new()
        ).unwrap();

        let ok_block_sprite = assets.join("OkBlock.png");
        let ok_block_sprite: G2dTexture = Texture::from_path(
            &mut self.window.factory,
            &ok_block_sprite,
            Flip::None,
            &TextureSettings::new()
        ).unwrap();

        let ok_tl_block_sprite = assets.join("OkTLCornerBlock.png");
        let ok_tl_block_sprite: G2dTexture = Texture::from_path(
            &mut self.window.factory,
            &ok_tl_block_sprite,
            Flip::None,
            &TextureSettings::new()
        ).unwrap();

        let ok_tr_block_sprite = assets.join("OkTRCornerBlock.png");
        let ok_tr_block_sprite: G2dTexture = Texture::from_path(
            &mut self.window.factory,
            &ok_tr_block_sprite,
            Flip::None,
            &TextureSettings::new()
        ).unwrap();

        let ok_bl_block_sprite = assets.join("OkBLCornerBlock.png");
        let ok_bl_block_sprite: G2dTexture = Texture::from_path(
            &mut self.window.factory,
            &ok_bl_block_sprite,
            Flip::None,
            &TextureSettings::new()
        ).unwrap();

        let ok_br_block_sprite = assets.join("OkBRCornerBlock.png");
        let ok_br_block_sprite: G2dTexture = Texture::from_path(
            &mut self.window.factory,
            &ok_br_block_sprite,
            Flip::None,
            &TextureSettings::new()
        ).unwrap();

        let weak_block_sprite = assets.join("WeakBlock.png");
        let weak_block_sprite: G2dTexture = Texture::from_path(
            &mut self.window.factory,
            &weak_block_sprite,
            Flip::None,
            &TextureSettings::new()
        ).unwrap();

        let weak_tl_block_sprite = assets.join("WeakTLCornerBlock.png");
        let weak_tl_block_sprite: G2dTexture = Texture::from_path(
            &mut self.window.factory,
            &weak_tl_block_sprite,
            Flip::None,
            &TextureSettings::new()
        ).unwrap();

        let weak_tr_block_sprite = assets.join("WeakTRCornerBlock.png");
        let weak_tr_block_sprite: G2dTexture = Texture::from_path(
            &mut self.window.factory,
            &weak_tr_block_sprite,
            Flip::None,
            &TextureSettings::new()
        ).unwrap();

        let weak_bl_block_sprite = assets.join("WeakBLCornerBlock.png");
        let weak_bl_block_sprite: G2dTexture = Texture::from_path(
            &mut self.window.factory,
            &weak_bl_block_sprite,
            Flip::None,
            &TextureSettings::new()
        ).unwrap();

        let weak_br_block_sprite = assets.join("WeakBRCornerBlock.png");
        let weak_br_block_sprite: G2dTexture = Texture::from_path(
            &mut self.window.factory,
            &weak_br_block_sprite,
            Flip::None,
            &TextureSettings::new()
        ).unwrap();

        let dead_sprite = assets.join("Dead.png");
        let dead_sprite: G2dTexture = Texture::from_path(
            &mut self.window.factory,
            &dead_sprite,
            Flip::None,
            &TextureSettings::new()
        ).unwrap();

        let red_alien_sprite = assets.join("RedInvader.png");
        let red_alien_sprite: G2dTexture = Texture::from_path(
            &mut self.window.factory,
            &red_alien_sprite,
            Flip::None,
            &TextureSettings::new()
        ).unwrap();

        self.sprites.alien_a1 = &alien_sprite_a1;
        self.sprites.alien_a2 = &alien_sprite_a2;
        self.sprites.alien_b1 = &alien_sprite_b1;
        self.sprites.alien_b2 = &alien_sprite_b2;
        self.sprites.alien_c1 = &alien_sprite_c1;
        self.sprites.alien_c2 = &alien_sprite_c2;
        self.sprites.canon = &canon_sprite;
        self.sprites.bullet = &bullet_sprite;
        self.sprites.full_block = &full_block_sprite;
        self.sprites.full_tl_block = &full_tl_block_sprite;
        self.sprites.full_tr_block = &full_tr_block_sprite;
        self.sprites.full_bl_block = &full_bl_block_sprite;
        self.sprites.full_br_block = &full_br_block_sprite;
        self.sprites.ok_block = &ok_block_sprite;
        self.sprites.ok_tl_block = &ok_tl_block_sprite;
        self.sprites.ok_tr_block = &ok_tr_block_sprite;
        self.sprites.ok_bl_block = &ok_bl_block_sprite;
        self.sprites.ok_br_block = &ok_br_block_sprite;
        self.sprites.weak_block = &weak_block_sprite;
        self.sprites.weak_tl_block = &weak_tl_block_sprite;
        self.sprites.weak_tr_block = &weak_tr_block_sprite;
        self.sprites.weak_bl_block = &weak_bl_block_sprite;
        self.sprites.weak_br_block = &weak_br_block_sprite;
        self.sprites.dead = &dead_sprite;
        self.sprites.red_alien = &red_alien_sprite;

        let mut events = Events::new(EventSettings::new());
        while let Some(e) = self.window.next() {
            if let Some(r) = e.render_args() {
                self.render(&r, &e);
            }

            if let Some(u) = e.update_args() {
                self.update(&u);
            }

            if let Some(b) = e.button_args() {
                self.input(&b);
            }
        }
    }

    fn render(&mut self, args: &RenderArgs, e: &piston_window::Event) {
        draw(self, &args, e);
    }

    fn update(&mut self, args: &UpdateArgs) {
        self.game.update(args.dt);
    }

    fn input(&mut self, args: &ButtonArgs) {
        let game = &mut self.game;

        match args.state {
            ButtonState::Press => {
                match args.button {
                    Button::Keyboard(Key::A) => {
                        game.canon.move_left();
                    }

                    Button::Keyboard(Key::Left) => {
                        game.canon.move_left();
                    }

                    Button::Keyboard(Key::D) => {
                        game.canon.move_right();
                    }

                    Button::Keyboard(Key::Right) => {
                        game.canon.move_right();
                    }

                    Button::Keyboard(Key::Space) => {
                        game.create_player_shot();
                    }

                    _ => {}
                }
            }

            ButtonState::Release => {
                match args.button {
                    Button::Keyboard(Key::A) => {
                        game.canon.idle();
                    }

                    Button::Keyboard(Key::Left) => {
                        game.canon.idle();
                    }

                    Button::Keyboard(Key::D) => {
                        game.canon.idle();
                    }

                    Button::Keyboard(Key::Right) => {
                        game.canon.idle();
                    }

                    _ => {}
                }
            }
        }
    }
}

fn main() {
    let mut app = App::new();
    app.run();
}

fn draw(app: &mut App, args: &RenderArgs, e: &piston_window::Event) {
    const WHITE: [f32; 4] = [1.0, 1.0, 1.0, 1.0];

    let scale = args.height as f64 / game::HEIGHT;
    let offset = (args.width / 2) as i32 - (game::WIDTH * scale / 2.0) as i32;

    let width = game::WIDTH * scale;
    let height = game::HEIGHT * scale;

    let game = &app.game;
    let sprites = &app.sprites;

    app.window.draw_2d(e, |c, g| {
        graphics::clear(WHITE, g);
        draw_field(height, &c, g);
        draw_alien(height, &game.wave, sprites, &c, g);
        draw_red_alien(height, &game.wave, sprites, &c, g);
        draw_canon(height, &game.canon, sprites, &c, g);
        draw_shots(height, &game, sprites, &c, g);
        draw_bunkers(height, &game.bunkers, sprites, &c, g);
    });
}

fn draw_field<G: Graphics>(
    height: f64,
    c: &Context,
    g: &mut G) {

    const BLACK: [f32; 4] = [0.0, 0.0, 0.0, 1.0];

    let scale = height as f64 / game::HEIGHT;

    let transform =
        c.transform
            .zoom(scale);

    let rect = graphics::Rectangle::new(BLACK);
    rect.draw(
        [0.0, 0.0, game::WIDTH, game::HEIGHT],
        &c.draw_state,
        transform,
        g
    );

}

fn draw_alien<G>(
    height: f64,
    wave: &Wave,
    sprites: &Sprites,
    c: &Context,
    g: &mut G
) where G: Graphics<Texture = G2dTexture>{

    let scale = height as f64 / game::HEIGHT;

    for alien in wave.iter() {
        let transform =
            c.transform
                .zoom(scale)
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
                                unsafe{&*sprites.alien_a1},
                                &c.draw_state,
                                transform,
                                g
                            );
                    }

                    alien::Kind::Beta => {
                        Image::new()
                            .draw(
                                unsafe{&*sprites.alien_b1},
                                &c.draw_state,
                                transform,
                                g
                            );
                    }

                    alien::Kind::Gamma => {
                        Image::new()
                            .draw(
                                unsafe{&*sprites.alien_c1},
                                &c.draw_state,
                                transform,
                                g
                            );
                    }
                }
            }

            alien::State::ArmsDown => {
                match alien.kind {
                    alien::Kind::Alpha => {
                        Image::new()
                            .draw(
                                unsafe{&*sprites.alien_a2},
                                &c.draw_state,
                                transform,
                                g
                            );
                    }

                    alien::Kind::Beta => {
                        Image::new()
                            .draw(
                                unsafe{&*sprites.alien_b2},
                                &c.draw_state,
                                transform,
                                g
                            );
                    }

                    alien::Kind::Gamma => {
                        Image::new()
                            .draw(
                                unsafe{&*sprites.alien_c2},
                                &c.draw_state,
                                transform,
                                g
                            );
                    }
                }
            }

            alien::State::Dead => {
                Image::new()
                    .draw(
                        unsafe{&*sprites.dead},
                        &c.draw_state,
                        transform,
                        g
                    );
            }

            _ => {}
        }
    }

}

fn draw_red_alien<G>(
    height: f64,
    wave: &Wave,
    sprites: &Sprites,
    c: &Context,
    g: &mut G
) where G: Graphics<Texture = G2dTexture>{

    let scale = height as f64 / game::HEIGHT;

    match wave.red_alien.state {
        red_alien::State::Active => {
            let transform =
                c.transform
                    .zoom(scale)
                    .trans(
                        wave.red_alien.position.x - (wave.red_alien.size.width / 2.0),
                        wave.red_alien.position.y - (wave.red_alien.size.height / 2.0)
                    );

            Image::new()
                .draw(
                    unsafe { &*sprites.red_alien },
                    &c.draw_state,
                    transform,
                    g
                );
        }

        red_alien::State::Dead => {
            let transform =
                c.transform
                    .zoom(scale)
                    .trans(
                        wave.red_alien.position.x - (wave.red_alien.size.width / 2.0),
                        wave.red_alien.position.y - (wave.red_alien.size.height / 2.0)
                    );

            Image::new()
                .draw(
                    unsafe { &*sprites.dead },
                    &c.draw_state,
                    transform,
                    g
                );
        }

        _ => {}
    }
}

fn draw_canon<G>(
    height: f64,
    canon: &Canon,
    sprites: &Sprites,
    c: &Context,
    g: &mut G
) where G: Graphics<Texture = G2dTexture>{
    let scale = height as f64 / game::HEIGHT;

    let transform =
        c.transform
            .zoom(scale)
            .trans(
                canon.position.x - (canon.size.width / 2.0),
                canon.position.y - (canon.size.height / 2.0)
            );

        Image::new()
            .draw(
                unsafe{&*sprites.canon},
                &c.draw_state,
                transform,
                g
            );
}

fn draw_shots<G>(
    height: f64,
    game: &Game,
    sprites: &Sprites,
    c: &Context,
    g: &mut G
) where G: Graphics<Texture = G2dTexture>{

    let scale = height as f64 / game::HEIGHT;

    let transform =
        c.transform
            .zoom(scale)
            .trans(
                game.player_shot.position.x - (game.player_shot.size.width / 2.0),
                game.player_shot.position.y - (game.player_shot.size.height / 2.0)
            );

    if game.player_shot.is_active() {
        Image::new()
            .draw(
                unsafe{&*sprites.bullet},
                &c.draw_state,
                transform,
                g
            );
    }

}

fn draw_bunkers<G>(
    height: f64,
    bunkers: &LinkedList<Bunker>,
    sprites: &Sprites,
    c: &Context,
    g: &mut G
) where G: Graphics<Texture = G2dTexture>{

    let scale = height as f64 / game::HEIGHT;

    for bunker in bunkers.iter() {
        for block in bunker.iter() {
            let transform =
                c.transform
                    .zoom(scale)
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
                                    unsafe { &*sprites.full_block },
                                    &c.draw_state,
                                    transform,
                                    g
                                );
                        }

                        block::State::HalfLife => {
                            Image::new()
                                .draw(
                                    unsafe { &*sprites.ok_block },
                                    &c.draw_state,
                                    transform,
                                    g
                                );
                        }

                        block::State::Weak => {
                            Image::new()
                                .draw(
                                    unsafe { &*sprites.weak_block },
                                    &c.draw_state,
                                    transform,
                                    g
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
                                    unsafe { &*sprites.full_tl_block },
                                    &c.draw_state,
                                    transform,
                                    g
                                );
                        }

                        block::State::HalfLife => {
                            Image::new()
                                .draw(
                                    unsafe { &*sprites.ok_tl_block },
                                    &c.draw_state,
                                    transform,
                                    g
                                );
                        }

                        block::State::Weak => {
                            Image::new()
                                .draw(
                                    unsafe { &*sprites.weak_tl_block },
                                    &c.draw_state,
                                    transform,
                                    g
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
                                    unsafe { &*sprites.full_tr_block },
                                    &c.draw_state,
                                    transform,
                                    g
                                );
                        }

                        block::State::HalfLife => {
                            Image::new()
                                .draw(
                                    unsafe { &*sprites.ok_tr_block },
                                    &c.draw_state,
                                    transform,
                                    g
                                );
                        }

                        block::State::Weak => {
                            Image::new()
                                .draw(
                                    unsafe { &*sprites.weak_tr_block },
                                    &c.draw_state,
                                    transform,
                                    g
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
                                    unsafe { &*sprites.full_bl_block },
                                    &c.draw_state,
                                    transform,
                                    g
                                );
                        }

                        block::State::HalfLife => {
                            Image::new()
                                .draw(
                                    unsafe { &*sprites.ok_bl_block },
                                    &c.draw_state,
                                    transform,
                                    g
                                );
                        }

                        block::State::Weak => {
                            Image::new()
                                .draw(
                                    unsafe { &*sprites.weak_bl_block },
                                    &c.draw_state,
                                    transform,
                                    g
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
                                    unsafe { &*sprites.full_br_block },
                                    &c.draw_state,
                                    transform,
                                    g
                                );
                        }

                        block::State::HalfLife => {
                            Image::new()
                                .draw(
                                    unsafe { &*sprites.ok_br_block },
                                    &c.draw_state,
                                    transform,
                                    g
                                );
                        }

                        block::State::Weak => {
                            Image::new()
                                .draw(
                                    unsafe { &*sprites.weak_br_block },
                                    &c.draw_state,
                                    transform,
                                    g
                                );
                        }

                        _ => {}
                    }
                }
            }
        }
    }

}