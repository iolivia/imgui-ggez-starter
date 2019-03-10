//! The simplest possible example that does something.


extern crate ggez;

use ggez::conf;
use ggez::event;
use ggez::graphics::{self, DrawMode, Point2};
use ggez::{Context, GameResult};

mod imgui_wrapper;

use crate::imgui_wrapper::ImGuiWrapper;

struct MainState {
    pos_x: f32,
    imgui_wrapper: ImGuiWrapper,
}

impl MainState {
    fn new(mut ctx: &mut Context) -> GameResult<MainState> {
        let mut imgui_wrapper = ImGuiWrapper::new(&mut ctx);
        let s = MainState { pos_x: 0.0, imgui_wrapper };
        Ok(s)
    }
}

impl event::EventHandler for MainState {
    fn update(&mut self, _ctx: &mut Context) -> GameResult<()> {
        self.pos_x = self.pos_x % 800.0 + 1.0;
        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult<()> {
        graphics::clear(ctx);
        graphics::circle(
            ctx,
            DrawMode::Fill,
            Point2::new(self.pos_x, 380.0),
            100.0,
            2.0,
        )?;
        graphics::present(ctx);
        Ok(())
    }
}

pub fn main() {
    let c = conf::Conf::new();
    let ctx = &mut Context::load_from_conf("super_simple", "ggez", c).unwrap();
    let state = &mut MainState::new(ctx).unwrap();
    event::run(ctx, state).unwrap();
}