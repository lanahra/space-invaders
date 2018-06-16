extern crate glutin_window;
extern crate graphics;
extern crate opengl_graphics;
extern crate piston;
extern crate piston_window;
extern crate rand;

mod assets;
mod draw;
mod game;

use draw::Draw;
use game::Game;
use opengl_graphics::{ GlGraphics, OpenGL };
use piston::event_loop::*;
use piston_window::*;
use piston::window::WindowSettings;

pub struct App {
    gl: GlGraphics,
    window: PistonWindow ,
    game: Game,
    assets: assets::Assets,
    pub highscores: Vec<u32>,
}

impl App {
    fn new() -> App {
        let opengl = OpenGL::V3_2;

        let window = WindowSettings::new(
                "Space Invaders",
                [1920, 1080]
            )
            .fullscreen(true)
            .opengl(opengl)
            .build()
            .unwrap();

        let mut highscores: Vec<u32> = Vec::new();
        highscores = Vec::with_capacity(10);
        highscores = vec![0; 10];



        return App {
            window,
            gl: GlGraphics::new(opengl),
            game: Game::new(),
            assets: assets::Assets::new(),
            highscores,
        };
    }

    fn run(&mut self) {
        let mut events = Events::new(EventSettings::new());
        while let Some(e) = events.next(&mut self.window) {
            match self.game.info.state {
                game::State::Exit => {
                    break;
                }

                _ => {}
            }

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
        Draw::render(self, &args);
    }

    fn update(&mut self, args: &UpdateArgs) {
        match self.game.info.state {
            game::State::Restart => {
                self.game = Game::new();
            }

            _ => {
                self.game.update(&mut self.highscores, args.dt);
            }
        }
    }

    fn input(&mut self, args: &ButtonArgs) {
        let game = &mut self.game;


        match args.state {
            ButtonState::Press => {
                match game.info.state {
                    game::State::Running => {
                        match args.button {
                            Button::Keyboard(Key::A)
                            | Button::Keyboard(Key::Left) => {
                                game.canon.move_left();
                            }

                            Button::Keyboard(Key::D)
                            | Button::Keyboard(Key::Right) => {
                                game.canon.move_right();
                            }

                            Button::Keyboard(Key::Space) => {
                                game.canon_fire();
                            }

                            _ => {}
                        }
                    }

                    game::State::Records => {
                        match args.button {
                            Button::Keyboard(Key::Return) => {
                                game.make_selection();
                            }

                            _ => {}
                        }
                    }

                    _ => {
                        match args.button {
                            | Button::Keyboard(Key::S)
                            | Button::Keyboard(Key::Down) => {
                                game.change_selection_down();
                            }

                            Button::Keyboard(Key::W)
                            | Button::Keyboard(Key::Up) => {
                                game.change_selection_up();
                            }

                            Button::Keyboard(Key::Return) => {
                                game.make_selection();
                            }

                            _ => {}
                        }
                    }
                }
            }

            ButtonState::Release => {
                match game.info.state {
                    game::State::Running => {
                        match args.button {
                            Button::Keyboard(Key::A)
                            | Button::Keyboard(Key::Left) => {
                                game.canon.idle();
                            }

                            Button::Keyboard(Key::D)
                            | Button::Keyboard(Key::Right) => {
                                game.canon.idle();
                            }

                            _ => {}
                        }
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
