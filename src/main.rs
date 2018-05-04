extern crate glutin_window;
extern crate graphics;
extern crate opengl_graphics;
extern crate piston;

mod game;
mod position;
mod alien;

use position::Position;
use alien::Alien;
use game::Game;
use glutin_window::GlutinWindow as Window;
use graphics::*;
use opengl_graphics::{ GlGraphics, OpenGL };
use piston::event_loop::*;
use piston::input::*;
use piston::window::WindowSettings;

pub struct App {
    gl: GlGraphics,
    window: Window,
    game: Game,
}

impl App {
    fn new() -> App {
        // Change this to OpenGL::V2_1 if not working.
        let opengl = OpenGL::V3_2;

        // Create an Glutin window.
        let window = WindowSettings::new(
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
        while let Some(e) = events.next(&mut self.window) {
            if let Some(r) = e.render_args() {
                self.render(&r);
            }

            if let Some(u) = e.update_args() {
                self.update(&u);
            }
        }
    }

    fn render(&mut self, args: &RenderArgs) {
        draw(self, &args);
    }

    fn update(&mut self, args: &UpdateArgs) {
        self.game.update(args.dt);
    }
}

fn main() {
    let mut position = Position::new((0.0,0.0), 20.0, 30.0);
    position.generate_collision_box();
    let alien = Alien::new(position, alien::SPRITE_ALIEN_V1);
    println!("{}", (alien.position.coordinates_collision_box.0).0);
    println!("{}", (alien.position.coordinates_collision_box.0).1);
    println!("{}", (alien.position.coordinates_collision_box.1).0);
    println!("{}", (alien.position.coordinates_collision_box.1).1);
    let mut app = App::new();
    app.run();
}

fn draw(app: &mut App, args: &RenderArgs) {
    const WHITE: [f32; 4] = [1.0, 1.0, 1.0, 1.0];

    let scale = args.height as f64 / game::HEIGHT;
    let offset = (args.width / 2) as i32 - (game::WIDTH * scale / 2.0) as i32;

    let width = game::WIDTH * scale;
    let height = game::HEIGHT * scale;

    let viewport =
        Viewport {
            rect: [offset, 0, width as i32, height as i32],
            window_size: [width as u32, height as u32],
            draw_size: [width as u32, height as u32],
        };

    app.gl.draw(viewport, |c, g| {
        graphics::clear(WHITE, g);

        draw_field(height, &c, g);
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
