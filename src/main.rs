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
use game::canon::Canon;
use opengl_graphics::{ GlGraphics, OpenGL };
use piston::event_loop::*;
use piston_window::*;
use piston::window::WindowSettings;

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
                [1920, 1080]
            )
            .fullscreen(true)
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
                self.game = Game::update(args.dt, self.game.clone());
            }
        }
    }

    fn input(&mut self, args: &ButtonArgs) {
        match args.state {
            ButtonState::Press => {
                match self.game.info.state {
                    game::State::Running => {
                        match args.button {
                            Button::Keyboard(Key::A)
                            | Button::Keyboard(Key::Left) => {
                                self.game =
                                    Game {
                                        canon: Canon::move_left(self.game.canon),
                                        ..self.game.clone()
                                    };
                            }

                            Button::Keyboard(Key::D)
                            | Button::Keyboard(Key::Right) => {
                                self.game =
                                    Game {
                                        canon: Canon::move_right(self.game.canon),
                                        ..self.game.clone()
                                    };
                            }

                            Button::Keyboard(Key::Space) => {
                                self.game = Game::canon_fire(self.game.clone());
                            }

                            _ => {}
                        }
                    }

                    _ => {
                        match args.button {
                            Button::Keyboard(Key::W)
                            | Button::Keyboard(Key::Up)
                            | Button::Keyboard(Key::S)
                            | Button::Keyboard(Key::Down) => {
                                self.game =
                                    Game::change_selection(self.game.clone());
                            }

                            Button::Keyboard(Key::Return) => {
                                self.game =
                                    Game::make_selection(self.game.clone());
                            }

                            _ => {}
                        }
                    }
                }
            }

            ButtonState::Release => {
                match self.game.info.state {
                    game::State::Running => {
                        match args.button {
                            Button::Keyboard(Key::A)
                            | Button::Keyboard(Key::Left) => {
                                self.game =
                                    Game {
                                        canon: Canon::idle(self.game.canon),
                                        ..self.game.clone()
                                    };
                            }

                            Button::Keyboard(Key::D)
                            | Button::Keyboard(Key::Right) => {
                                self.game =
                                    Game {
                                        canon: Canon::idle(self.game.canon),
                                        ..self.game.clone()
                                    };
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
