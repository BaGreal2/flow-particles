mod colors;
mod consts;
mod effect;
mod particle;

use consts::{HEIGHT, PARTICLES_AMOUNT, WIDTH};
use effect::Effect;
use ggez::event::{self, EventHandler};
use ggez::graphics::{self, Color};
use ggez::input::keyboard::{KeyCode, KeyInput};
use ggez::{conf, Context, ContextBuilder, GameResult};

fn main() {
    let window_setup = conf::WindowSetup::default().title("Flow Particles");
    let window_mode = conf::WindowMode::default()
        .dimensions(WIDTH, HEIGHT)
        .fullscreen_type(conf::FullscreenType::Windowed)
        .resizable(true);
    let (mut ctx, event_loop) = ContextBuilder::new("particles", "xanin")
        .window_setup(window_setup)
        .window_mode(window_mode)
        .build()
        .expect("aieee, could not create ggez context!");

    let my_game = MyGame::new(&mut ctx);
    event::run(ctx, event_loop, my_game);
}

struct MyGame {
    effect: Effect,
}

impl MyGame {
    pub fn new(_ctx: &mut Context) -> MyGame {
        let mut effect = Effect::new();
        effect.init(PARTICLES_AMOUNT);
        MyGame { effect }
    }
}

impl EventHandler for MyGame {
    fn update(&mut self, _ctx: &mut Context) -> GameResult {
        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult {
        let mut canvas = graphics::Canvas::from_frame(ctx, Color::BLACK);
        self.effect.render(&mut canvas, ctx);
        canvas.finish(ctx)
    }

    fn key_down_event(
        &mut self,
        _ctx: &mut Context,
        input: KeyInput,
        _repeated: bool,
    ) -> Result<(), ggez::GameError> {
        if let Some(keycode) = input.keycode {
            if keycode == KeyCode::Equals {
                self.effect.zoom -= self.effect.zoom * 0.1;
                self.effect.reset();
                self.effect.init(PARTICLES_AMOUNT);
            }
            if keycode == KeyCode::Minus {
                self.effect.zoom += self.effect.zoom * 0.1;
                self.effect.reset();
                self.effect.init(PARTICLES_AMOUNT);
            }
            if keycode == KeyCode::P {
                self.effect.curve += self.effect.curve * 0.1;
                self.effect.reset();
                self.effect.init(PARTICLES_AMOUNT);
            }
            if keycode == KeyCode::O {
                self.effect.curve -= self.effect.curve * 0.1;
                self.effect.reset();
                self.effect.init(PARTICLES_AMOUNT);
            }
            if keycode == KeyCode::I {
                if self.effect.color_index < self.effect.colors.len() - 1 {
                    self.effect.color_index += 1;
                    self.effect.reset();
                    self.effect.init(PARTICLES_AMOUNT);
                }
            }
            if keycode == KeyCode::U {
                if self.effect.color_index > 0 {
                    self.effect.color_index -= 1;
                    self.effect.reset();
                    self.effect.init(PARTICLES_AMOUNT);
                }
            }
        }
        Ok(())
    }
}
