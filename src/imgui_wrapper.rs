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
  pub imgui: ImGui,
  pub renderer: Renderer<gfx_device_gl::Resources>,
  last_frame: Instant,
  mouse_state: MouseState,
  show_popup: bool,
}

impl ImGuiWrapper {
  pub fn new(ctx: &mut Context) -> Self {
    // Create the imgui object
    let mut imgui = ImGui::init();

    // Shaders
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

    // Renderer
    let render_target = graphics::get_screen_render_target(ctx);
    let factory = graphics::get_factory(ctx);

    let renderer = Renderer::init(
      &mut imgui,
      &mut *factory,
      shaders,
      RenderTargetView::new(render_target.clone()),
    )
    .unwrap();

    // Create instace
    Self {
      imgui,
      renderer,
      last_frame: Instant::now(),
      mouse_state: MouseState::default(),
      show_popup: false,
    }
  }

  pub fn render(&mut self, ctx: &mut Context) {
    // Update mouse
    self.update_mouse();

    // Create new frame
    let screen_size = graphics::get_size(ctx);
    let w = screen_size.0;
    let h = screen_size.1;

    let frame_size = FrameSize {
      logical_size: (w as f64, h as f64),
      hidpi_factor: 2.0,
    };

    let now = Instant::now();
    let delta = now - self.last_frame;
    let delta_s = delta.as_secs() as f32 + delta.subsec_nanos() as f32 / 1_000_000_000.0;
    self.last_frame = now;

    let ui = self.imgui.frame(frame_size, delta_s);

    // Various ui things
    {
      // Window
      ui.window(im_str!("Hello world"))
        .size((300.0, 600.0), ImGuiCond::FirstUseEver)
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

          if ui.small_button(im_str!("small button")) {
            println!("Small button clicked");
          }
        });

      // Popup
      ui.popup(im_str!("popup"), || {
        if ui.menu_item(im_str!("popup menu item 1")).build() {
          println!("popup menu item 1 clicked");
        }

        if ui.menu_item(im_str!("popup menu item 2")).build() {
          println!("popup menu item 2 clicked");
        }
      });

      // Menu bar
      ui.main_menu_bar(|| {
        ui.menu(im_str!("Menu 1")).build(|| {
          if ui.menu_item(im_str!("Item 1.1")).build() {
            println!("item 1.1 inside menu bar clicked");
          }

          ui.menu(im_str!("Item 1.2")).build(|| {
            if ui.menu_item(im_str!("Item 1.2.1")).build() {
              println!("item 1.2.1 inside menu bar clicked");
            }
            if ui.menu_item(im_str!("Item 1.2.2")).build() {
              println!("item 1.2.2 inside menu bar clicked");
            }
          });
        });

        ui.menu(im_str!("Menu 2")).build(|| {
          if ui.menu_item(im_str!("Item 2.1")).build() {
            println!("item 2.1 inside menu bar clicked");
          }
        });
      });
    }

    if self.show_popup {
      ui.open_popup(im_str!("popup"));
    }

    // Render
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

    if pressed.0 {
      self.show_popup = false;
    }
  }

  pub fn open_popup(&mut self) {
    self.show_popup = true;
  }
}
