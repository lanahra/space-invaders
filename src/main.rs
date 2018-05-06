extern crate glutin_window;
extern crate graphics;
extern crate opengl_graphics;
extern crate piston;
extern crate piston_window;
extern crate find_folder;

mod game;
mod position;
mod alien;
mod shot;
mod wave;
mod sprites;

use sprites::Sprites;
use wave::Wave;
use position::Position;
use alien::Alien;
use shot::Shot;
use game::Game;
use glutin_window::GlutinWindow as Window;
use graphics::*;
use opengl_graphics::{ GlGraphics, OpenGL };
use piston::event_loop::*;
use piston_window::*;
use piston::input::*;
use piston::window::WindowSettings;
use graphics::rectangle::square;
use std::path::Path;

pub struct App {
    gl: GlGraphics,
    window: PistonWindow ,
    game: Game,
}

impl App {
    fn new() -> App {
        // Change this to OpenGL::V2_1 if not working.
        let opengl = OpenGL::V3_2;

        // Create an Glutin window.
        let window: PistonWindow  = WindowSettings::new(
                "Space Invaders",
                [game::WIDTH as u32, game::HEIGHT as u32]
            )
            .opengl(opengl)
            .exit_on_esc(true)
            .build()
            .unwrap();

        return App {
            window,
            gl: GlGraphics::new(opengl),
            game: Game::new(),
        };
    }

    fn run(&mut self) {
        let mut events = Events::new(EventSettings::new());
        while let Some(e) = self.window.next() {
            if let Some(r) = e.render_args() {
                self.render(&r, &e);
            }

            if let Some(u) = e.update_args() {
                self.update(&u);
            }
        }
    }

    fn render(&mut self, args: &RenderArgs, e: &piston_window::Event) {
        draw(self, &args, e);
    }

    fn update(&mut self, args: &UpdateArgs) {
        self.game.update(args.dt);
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

    let game = &mut app.game;


    /// Load assets
    let assets = find_folder::Search::ParentsThenKids(3, 3)
        .for_folder("resources").unwrap();
    let alien_sprite_a1 = assets.join("InvaderA1.png");
    let alien_sprite_a1: G2dTexture = Texture::from_path(
        &mut app.window.factory,
        &alien_sprite_a1,
        Flip::None,
        &TextureSettings::new()
    ).unwrap();

    let alien_sprite_a2 = assets.join("InvaderA2.png");
    let alien_sprite_a2: G2dTexture = Texture::from_path(
        &mut app.window.factory,
        &alien_sprite_a2,
        Flip::None,
        &TextureSettings::new()
    ).unwrap();

    let assets = find_folder::Search::ParentsThenKids(3, 3)
        .for_folder("resources").unwrap();
    let alien_sprite_b1 = assets.join("InvaderB1.png");
    let alien_sprite_b1: G2dTexture = Texture::from_path(
        &mut app.window.factory,
        &alien_sprite_b1,
        Flip::None,
        &TextureSettings::new()
    ).unwrap();

    let alien_sprite_b2 = assets.join("InvaderB2.png");
    let alien_sprite_b2: G2dTexture = Texture::from_path(
        &mut app.window.factory,
        &alien_sprite_b2,
        Flip::None,
        &TextureSettings::new()
    ).unwrap();

    let assets = find_folder::Search::ParentsThenKids(3, 3)
        .for_folder("resources").unwrap();
    let alien_sprite_c1 = assets.join("InvaderC1.png");
    let alien_sprite_c1: G2dTexture = Texture::from_path(
        &mut app.window.factory,
        &alien_sprite_c1,
        Flip::None,
        &TextureSettings::new()
    ).unwrap();

    let alien_sprite_c2 = assets.join("InvaderC2.png");
    let alien_sprite_c2: G2dTexture = Texture::from_path(
        &mut app.window.factory,
        &alien_sprite_c2,
        Flip::None,
        &TextureSettings::new()
    ).unwrap();

    game.sprites.alien_a1 = &alien_sprite_a1;
    game.sprites.alien_a2 = &alien_sprite_a2;
    game.sprites.alien_b1 = &alien_sprite_b1;
    game.sprites.alien_b2 = &alien_sprite_b2;
    game.sprites.alien_c1 = &alien_sprite_c1;
    game.sprites.alien_c2 = &alien_sprite_c2;
    /// Finish Load


    app.window.draw_2d(e, |c, g| {
        graphics::clear(WHITE, g);
        draw_field(height, &c, g);
        draw_alien(height, &c, g, game);
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
    c: &Context,
    g: &mut G,
    game: &mut Game) where G: Graphics<Texture = G2dTexture>{

    const BLACK: [f32; 4] = [255.0, 0.0, 0.0, 1.0];

    let scale = height as f64 / game::HEIGHT;

    for alien in &game.wave.aliens {
        let transform =
            c.transform
                .zoom(scale).trans((alien.position.coordinates_collision_box.0).0, (alien.position.coordinates_collision_box.0).1);
        if alien.sprite == alien::SPRITE_ALIEN_A1 {
            Image::new().draw(unsafe{&*game.sprites.alien_a1}, &c.draw_state, transform, g);
        }
        else if alien.sprite == alien::SPRITE_ALIEN_A2 {
            Image::new().draw(unsafe{&*game.sprites.alien_a2}, &c.draw_state, transform, g);
        }
        else if alien.sprite == alien::SPRITE_ALIEN_B1 {
            Image::new().draw(unsafe{&*game.sprites.alien_b1}, &c.draw_state, transform, g);
        }
        else if alien.sprite == alien::SPRITE_ALIEN_B2 {
            Image::new().draw(unsafe{&*game.sprites.alien_b2}, &c.draw_state, transform, g);
        }
        else if alien.sprite == alien::SPRITE_ALIEN_C1 {
            Image::new().draw(unsafe{&*game.sprites.alien_c1}, &c.draw_state, transform, g);
        }
        else if alien.sprite == alien::SPRITE_ALIEN_C2 {
            Image::new().draw(unsafe{&*game.sprites.alien_c2}, &c.draw_state, transform, g);
        }
    }

}
