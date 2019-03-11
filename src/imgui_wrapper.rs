use gfx_core::{handle::RenderTargetView, memory::Typed};
use gfx_device_gl;
use ggez::graphics;
use ggez::Context;

use imgui::*;
use imgui::{FrameSize, ImGui, ImGuiCond, Window};
use imgui_gfx_renderer::{Renderer, Shaders};
use std::time::Instant;

const IMGUI_TAB: u8 = 0;
const IMGUI_LEFT_ARROW: u8 = 1;
const IMGUI_RIGHT_ARROW: u8 = 2;
const IMGUI_UP_ARROW: u8 = 3;
const IMGUI_DOWN_ARROW: u8 = 3;
const IMGUI_PAGE_UP: u8 = 5;
const IMGUI_PAGE_DOWN: u8 = 6;
const IMGUI_HOME: u8 = 7;
const IMGUI_END: u8 = 8;
const IMGUI_DELETE: u8 = 9;
const IMGUI_BACKSPACE: u8 = 10;
const IMGUI_ENTER: u8 = 11;
const IMGUI_ESCAPE: u8 = 12;
const IMGUI_A: u8 = 13;
const IMGUI_C: u8 = 14;
const IMGUI_V: u8 = 15;
const IMGUI_X: u8 = 16;
const IMGUI_Y: u8 = 17;
const IMGUI_Z: u8 = 18;

#[derive(Copy, Clone, PartialEq, Debug, Default)]
struct MouseState {
  pos: (i32, i32),
  pressed: (bool, bool, bool),
  wheel: f32,
}

pub struct ImGuiWrapper {
  pub imgui: ImGui,
  pub renderer: Renderer<gfx_device_gl::Resources>,
  last_frame: Instant,
  mouse_state: MouseState,
}

impl ImGuiWrapper {
  pub fn new(ctx: &mut Context) -> Self {
    let mut imgui = ImGui::init();

    // Fix incorrect colors with sRGB framebuffer
    fn imgui_gamma_to_linear(col: ImVec4) -> ImVec4 {
      let x = col.x.powf(2.2);
      let y = col.y.powf(2.2);
      let z = col.z.powf(2.2);
      let w = 1.0 - (1.0 - col.w).powf(2.2);
      ImVec4::new(x, y, z, w)
    }

    let style = imgui.style_mut();
    style.window_rounding = 10.;
    style.child_rounding = 10.;
    style.frame_rounding = 10.;

    for col in 0..style.colors.len() {
      style.colors[col] = imgui_gamma_to_linear(style.colors[col]);
    }

    let shaders = {
      let version = graphics::get_device(ctx).get_info().shading_language;
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

    Self::configure_keys(&mut imgui);

    let render_target = graphics::get_screen_render_target(ctx);
    let factory = graphics::get_factory(ctx);

    let renderer = Renderer::init(
      &mut imgui,
      &mut *factory,
      shaders,
      RenderTargetView::new(render_target.clone()),
    )
    .unwrap();

    Self {
      imgui,
      renderer,
      last_frame: Instant::now(),
      mouse_state: MouseState::default(),
    }
  }

  fn configure_keys(imgui: &mut ImGui) {
    use imgui::ImGuiKey;

    imgui.set_imgui_key(ImGuiKey::Tab, IMGUI_TAB);
    imgui.set_imgui_key(ImGuiKey::LeftArrow, IMGUI_LEFT_ARROW);
    imgui.set_imgui_key(ImGuiKey::RightArrow, IMGUI_RIGHT_ARROW);
    imgui.set_imgui_key(ImGuiKey::UpArrow, IMGUI_UP_ARROW);
    imgui.set_imgui_key(ImGuiKey::DownArrow, IMGUI_DOWN_ARROW);
    imgui.set_imgui_key(ImGuiKey::PageUp, IMGUI_PAGE_UP);
    imgui.set_imgui_key(ImGuiKey::PageDown, IMGUI_PAGE_DOWN);
    imgui.set_imgui_key(ImGuiKey::Home, IMGUI_HOME);
    imgui.set_imgui_key(ImGuiKey::End, IMGUI_END);
    imgui.set_imgui_key(ImGuiKey::Delete, IMGUI_DELETE);
    imgui.set_imgui_key(ImGuiKey::Backspace, IMGUI_BACKSPACE);
    imgui.set_imgui_key(ImGuiKey::Enter, IMGUI_ENTER);
    imgui.set_imgui_key(ImGuiKey::Escape, IMGUI_ESCAPE);
    imgui.set_imgui_key(ImGuiKey::A, IMGUI_A);
    imgui.set_imgui_key(ImGuiKey::C, IMGUI_C);
    imgui.set_imgui_key(ImGuiKey::V, IMGUI_V);
    imgui.set_imgui_key(ImGuiKey::X, IMGUI_X);
    imgui.set_imgui_key(ImGuiKey::Y, IMGUI_Y);
    imgui.set_imgui_key(ImGuiKey::Z, IMGUI_Z);
  }

  pub fn render_scene_ui(&mut self, ctx: &mut Context) {
    self.update_mouse();

    let w = ctx.conf.window_mode.width;
    let h = ctx.conf.window_mode.height;

    let frame_size = FrameSize {
      logical_size: (w as f64, h as f64),
      hidpi_factor: 1.0,
    };

    let now = Instant::now();
    let delta = now - self.last_frame;
    let delta_s = delta.as_secs() as f32 + delta.subsec_nanos() as f32 / 1_000_000_000.0;
    self.last_frame = now;

    let ui = self.imgui.frame(frame_size, delta_s);

    ui.window(im_str!("Hello world"))
      .size((300.0, 100.0), ImGuiCond::FirstUseEver)
      .position((100.0, 100.0), ImGuiCond::FirstUseEver)
      .build(|| {
        ui.text(im_str!("Hello world!"));
        ui.text(im_str!("こんにちは世界！"));
        ui.text(im_str!("This...is...imgui-rs!"));
        ui.separator();
        let mouse_pos = ui.imgui().mouse_pos();
        ui.text(im_str!(
          "Mouse Position: ({:.1},{:.1})",
          mouse_pos.0,
          mouse_pos.1
        ));
      });

    let (factory, _, encoder, _, _) = graphics::get_gfx_objects(ctx);
    self.renderer.render(ui, &mut *factory, encoder).unwrap();
  }

  fn update_mouse(&mut self) {
    self
      .imgui
      .set_mouse_pos(self.mouse_state.pos.0 as f32, self.mouse_state.pos.1 as f32);
    self.imgui.set_mouse_down([
      self.mouse_state.pressed.0,
      self.mouse_state.pressed.1,
      self.mouse_state.pressed.2,
      false,
      false,
    ]);
    self.imgui.set_mouse_wheel(self.mouse_state.wheel);
    self.mouse_state.wheel = 0.0;
  }

  pub fn update_mouse_pos(&mut self, x: i32, y: i32) {
    self.mouse_state.pos = (x, y);
  }

  pub fn update_mouse_down(&mut self, pressed: (bool, bool, bool)) {
    self.mouse_state.pressed = pressed;
  }
}
