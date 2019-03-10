//! The simplest possible example that does something.

extern crate ggez;

use ggez::conf;
use ggez::event::{self, Axis, Button, Keycode, Mod, MouseButton, MouseState};
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
        let s = MainState {
            pos_x: 0.0,
            imgui_wrapper,
        };
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

        self.imgui_wrapper.render_scene_ui(ctx);

        graphics::present(ctx);
        Ok(())
    }

    fn mouse_motion_event(
        &mut self,
        _ctx: &mut Context,
        _state: MouseState,
        x: i32,
        y: i32,
        xrel: i32,
        yrel: i32,
    ) {
        self.imgui_wrapper.update_mouse_pos(x, y);
        // println!("Mouse motion, x: {}, y: {}", x, y);
    }

    fn mouse_button_down_event(&mut self, _ctx: &mut Context, button: MouseButton, x: i32, y: i32) {
        self.imgui_wrapper.update_mouse_down((
            button == MouseButton::Left,
            button == MouseButton::Right,
            button == MouseButton::Middle,
        ));
        // println!("Mouse button pressed: {:?}, x: {}, y: {}", button, x, y);
    }

    fn mouse_button_up_event(&mut self, _ctx: &mut Context, button: MouseButton, x: i32, y: i32) {
        self.imgui_wrapper.update_mouse_down((
            match button {
                MouseButton::Left => false,
                _ => true,
            },
            match button {
                MouseButton::Right => false,
                _ => true,
            },
            match button {
                MouseButton::Middle => false,
                _ => true,
            },
        ));
        println!("Mouse button released: {:?}, x: {}, y: {}", button, x, y);
    }
}

pub fn main() {
    let c = conf::Conf::new();
    let ctx = &mut Context::load_from_conf("super_simple", "ggez", c).unwrap();
    let state = &mut MainState::new(ctx).unwrap();
    event::run(ctx, state).unwrap();
}
