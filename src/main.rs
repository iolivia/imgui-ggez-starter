extern crate ggez;

mod imgui_wrapper;

use ggez::{Context, GameResult};
use ggez::event::{self, EventHandler, MouseButton, KeyCode, KeyMods};
use ggez::graphics;
use ggez::nalgebra as na;

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

impl EventHandler for MainState {
    fn update(&mut self, _ctx: &mut Context) -> GameResult<()> {
        self.pos_x = self.pos_x % 800.0 + 1.0;
        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult<()> {
        graphics::clear(ctx, graphics::BLACK);

        // Render game stuff
        {
            // graphics::circle(
            //     ctx,
            //     DrawMode::Fill,
            //     Point2::new(self.pos_x, 380.0),
            //     100.0,
            //     2.0,
            // )?;

            let circle = graphics::Mesh::new_circle(
                ctx,
                graphics::DrawMode::fill(),
                na::Point2::new(self.pos_x, 380.0),
                100.0,
                2.0,
                graphics::WHITE,
            )?;
            graphics::draw(ctx, &circle, (na::Point2::new(0.0, 0.0),))?;
        }

        // Render game ui
        {
            self.imgui_wrapper.render(ctx);
        }

        graphics::present(ctx);
        Ok(())
    }

    fn mouse_motion_event(
        &mut self,
        _ctx: &mut Context,
        x: f32,
        y: f32,
        _dx: f32,
        _dy: f32,
    ) {
        self.imgui_wrapper.update_mouse_pos(x, y);
    }

    fn mouse_button_down_event(
        &mut self,
        _ctx: &mut Context,
        button: MouseButton,
        _x: f32,
        _y: f32,
    ) {
        self.imgui_wrapper.update_mouse_down((
            button == MouseButton::Left,
            button == MouseButton::Right,
            button == MouseButton::Middle,
        ));
    }

    fn mouse_button_up_event(&mut self, _ctx: &mut Context, button: MouseButton, _x: f32, _y: f32) {
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

    fn key_down_event(
        &mut self,
        _ctx: &mut Context,
        keycode: KeyCode,
        _keymods: KeyMods,
        _repeat: bool,
    ) {
        match keycode {
            KeyCode::P => {
                self.imgui_wrapper.open_popup();
            }
            _ => (),
        }
    }
}

pub fn main() -> ggez::GameResult {
    // let c = conf::Conf::new();
    // let ctx = &mut Context::load_from_conf("super_simple with imgui", "ggez", c).unwrap();
    // let state = &mut MainState::new(ctx).unwrap();
    // event::run(ctx, state).unwrap();

    let cb = ggez::ContextBuilder::new("super_simple with imgui", "ggez");
    let (ref mut ctx, event_loop) = &mut cb.build()?;
    let state = &mut MainState::new(ctx)?;
    event::run(ctx, event_loop, state)

    // let (mut ctx, mut event_loop) = ContextBuilder::new("super_simple with imgui", "oliviaifrim")
    //     .build()
    //     .unwrap();

    // let mut game = MainState::new(&mut ctx);

    // match event::run(&mut ctx, &mut event_loop, &mut game) {
    //     Ok(_) => println!("Exited cleanly."),
    //     Err(e) => println!("Error occured: {}", e)
    // }
}
