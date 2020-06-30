use ggez::event::{KeyCode, KeyMods};
use ggez::graphics;
use ggez::Context;

use gfx_core::{handle::RenderTargetView, memory::Typed};
use gfx_device_gl;

use imgui::*;
use imgui_gfx_renderer::*;

use std::time::Instant;

#[derive(Copy, Clone, PartialEq, Debug, Default)]
struct MouseState {
  pos: (i32, i32),
  pressed: (bool, bool, bool),
  wheel: f32,
}

pub struct ImGuiWrapper {
  pub imgui: imgui::Context,
  pub renderer: Renderer<gfx_core::format::Rgba8, gfx_device_gl::Resources>,
  last_frame: Instant,
  mouse_state: MouseState,
}

impl ImGuiWrapper {
  pub fn new(ctx: &mut Context) -> Self {
    // Create the imgui object
    let mut imgui = imgui::Context::create();
    let (factory, gfx_device, _, _, _) = graphics::gfx_objects(ctx);

    // Shaders
    let shaders = {
      let version = gfx_device.get_info().shading_language;
      if version.is_embedded {
        if version.major >= 3 {
          Shaders::GlSlEs300
        } else {
          Shaders::GlSlEs100
        }
      } else if version.major >= 4 {
        Shaders::GlSl400
      } else if version.major >= 3 {
        Shaders::GlSl130
      } else {
        Shaders::GlSl110
      }
    };

    // Renderer
    let mut renderer = Renderer::init(&mut imgui, &mut *factory, shaders).unwrap();

    {
      let mut io = imgui.io_mut();
      io[Key::Tab] = KeyCode::Tab as _;
      io[Key::LeftArrow] = KeyCode::Left as _;
      io[Key::RightArrow] = KeyCode::Right as _;
      io[Key::UpArrow] = KeyCode::Up as _;
      io[Key::DownArrow] = KeyCode::Down as _;
      io[Key::PageUp] = KeyCode::PageUp as _;
      io[Key::PageDown] = KeyCode::PageDown as _;
      io[Key::Home] = KeyCode::Home as _;
      io[Key::End] = KeyCode::End as _;
      io[Key::Insert] = KeyCode::Insert as _;
      io[Key::Delete] = KeyCode::Delete as _;
      io[Key::Backspace] = KeyCode::Back as _;
      io[Key::Space] = KeyCode::Space as _;
      io[Key::Enter] = KeyCode::Return as _;
      io[Key::Escape] = KeyCode::Escape as _;
      io[Key::KeyPadEnter] = KeyCode::NumpadEnter as _;
      io[Key::A] = KeyCode::A as _;
      io[Key::C] = KeyCode::C as _;
      io[Key::V] = KeyCode::V as _;
      io[Key::X] = KeyCode::X as _;
      io[Key::Y] = KeyCode::Y as _;
      io[Key::Z] = KeyCode::Z as _;
    }

    // Create instance
    Self {
      imgui,
      renderer,
      last_frame: Instant::now(),
      mouse_state: MouseState::default(),
    }
  }

  pub fn render(&mut self, ctx: &mut Context, hidpi_factor: f32) {
    // Update mouse
    self.update_mouse();

    // Create new frame
    let now = Instant::now();
    let delta = now - self.last_frame;
    let delta_s = delta.as_secs() as f32 + delta.subsec_nanos() as f32 / 1_000_000_000.0;
    self.last_frame = now;

    let (draw_width, draw_height) = graphics::drawable_size(ctx);
    self.imgui.io_mut().display_size = [draw_width, draw_height];
    self.imgui.io_mut().display_framebuffer_scale = [hidpi_factor, hidpi_factor];
    self.imgui.io_mut().delta_time = delta_s;

    let ui = self.imgui.frame();
    let mut show = true;

    // Various ui things
    {
      // Window
      Window::new(im_str!("Hello world"))
        .size([300.0, 600.0], imgui::Condition::FirstUseEver)
        .position([50.0, 50.0], imgui::Condition::FirstUseEver)
        .build(&ui, || {
          // Your window stuff here!
          ui.text(im_str!("Hi from this label!"));
        });
      ui.show_demo_window(&mut show);
    }

    // Render
    let (factory, _, encoder, _, render_target) = graphics::gfx_objects(ctx);
    let draw_data = ui.render();
    self
      .renderer
      .render(
        &mut *factory,
        encoder,
        &mut RenderTargetView::new(render_target.clone()),
        draw_data,
      )
      .unwrap();
  }

  fn update_mouse(&mut self) {
    self.imgui.io_mut().mouse_pos = [self.mouse_state.pos.0 as f32, self.mouse_state.pos.1 as f32];

    self.imgui.io_mut().mouse_down = [
      self.mouse_state.pressed.0,
      self.mouse_state.pressed.1,
      self.mouse_state.pressed.2,
      false,
      false,
    ];

    self.imgui.io_mut().mouse_wheel = self.mouse_state.wheel;
    self.mouse_state.wheel = 0.0;
  }

  pub fn update_mouse_pos(&mut self, x: f32, y: f32) {
    self.mouse_state.pos = (x as i32, y as i32);
  }

  pub fn update_mouse_down(&mut self, pressed: (bool, bool, bool)) {
    self.mouse_state.pressed = pressed;
  }

  pub fn update_key_down(&mut self, key: KeyCode, mods: KeyMods) {
    self.imgui.io_mut().key_shift = mods.contains(KeyMods::SHIFT);
    self.imgui.io_mut().key_ctrl = mods.contains(KeyMods::CTRL);
    self.imgui.io_mut().key_alt = mods.contains(KeyMods::ALT);
    self.imgui.io_mut().keys_down[key as usize] = true;
  }

  pub fn update_key_up(&mut self, key: KeyCode, mods: KeyMods) {
    if mods.contains(KeyMods::SHIFT) {
      self.imgui.io_mut().key_shift = false;
    }
    if mods.contains(KeyMods::CTRL) {
      self.imgui.io_mut().key_ctrl = false;
    }
    if mods.contains(KeyMods::ALT) {
      self.imgui.io_mut().key_alt = false;
    }
    self.imgui.io_mut().keys_down[key as usize] = false;
  }

  pub fn update_text(&mut self, val: char) {
    self.imgui.io_mut().add_input_character(val);
  }

  pub fn update_scroll(&mut self, x: f32, y: f32) {
    self.imgui.io_mut().mouse_wheel += y;
    self.imgui.io_mut().mouse_wheel_h += x;
    println!("{}", self.imgui.io().mouse_wheel);
  }
}
