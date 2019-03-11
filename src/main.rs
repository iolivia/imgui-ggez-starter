extern crate ggez;

use ggez::conf;
use ggez::event::Keycode;
use ggez::event::Mod;
use ggez::event::{self, MouseButton, MouseState};
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
        let imgui_wrapper = ImGuiWrapper::new(&mut ctx);
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
        _xrel: i32,
        _yrel: i32,
    ) {
        self.imgui_wrapper.update_mouse_pos(x, y);
    }

    fn mouse_button_down_event(
        &mut self,
        _ctx: &mut Context,
        button: MouseButton,
        _x: i32,
        _y: i32,
    ) {
        self.imgui_wrapper.update_mouse_down((
            button == MouseButton::Left,
            button == MouseButton::Right,
            button == MouseButton::Middle,
        ));
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
    }

    fn key_down_event(&mut self, _ctx: &mut Context, keycode: Keycode, keymod: Mod, repeat: bool) {
        match keycode {
            Keycode::P => {
                self.imgui_wrapper.open_popup();
            }
            _ => (),
        }
    }
    fn key_up_event(&mut self, _ctx: &mut Context, keycode: Keycode, keymod: Mod, repeat: bool) {
        println!(
            "Key released: {:?}, modifier {:?}, repeat: {}",
            keycode, keymod, repeat
        );
    }
}

pub fn main() {
    let c = conf::Conf::new();
    let ctx = &mut Context::load_from_conf("super_simple with imgui", "ggez", c).unwrap();
    let state = &mut MainState::new(ctx).unwrap();
    event::run(ctx, state).unwrap();
}
