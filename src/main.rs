extern crate glutin_window;
extern crate graphics;
extern crate opengl_graphics;
extern crate piston;
extern crate piston_window;
extern crate find_folder;

mod assets;
mod draw;
mod game;

use draw::Draw;
use game::entity::statical::bunker::block;
use game::player_info::PlayerInfo;
use game::entity::statical::bunker::Bunker;
use game::entity::active::wave::alien;
use game::entity::active::wave::red_alien;
use game::Game;
use game::entity::active::canon::Canon;
use game::entity::active::bullet::Shot;
use game::entity::active::bullet::Kind;
use game::entity::active::wave::Wave;
use game::entity::Entity;
use graphics::*;
use opengl_graphics::{ GlGraphics, OpenGL };
use piston::event_loop::*;
use piston_window::*;
use piston::input::*;
use piston::window::WindowSettings;
use graphics::rectangle::square;

pub struct App {
    gl: GlGraphics,
    window: PistonWindow ,
    game: Game,
    assets: assets::Assets,
}

impl App {
    fn new() -> App {
        let opengl = OpenGL::V3_2;

        let window = WindowSettings::new(
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
            assets: assets::Assets::new(),
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

            if let Some(b) = e.button_args() {
                self.input(&b);
            }
        }
    }

    fn render(&mut self, args: &RenderArgs) {
        draw(self, &args);
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

    let game = &app.game;
    let assets = &app.assets;

    app.gl.draw(viewport, |mut c, gl| {
        graphics::clear(WHITE, gl);

        let scale = height as f64 / game::HEIGHT;

        let transform =
            c.transform
                .zoom(scale);

        c.transform = transform;

        Draw::draw(&game, &assets, &c, gl);
    });
}
