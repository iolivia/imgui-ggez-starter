use ggez::Context;
use nalgebra::Vector2;

pub trait CtxExtension {
    fn screen_size(&self) -> Vector2<u32>;
}

impl CtxExtension for Context {
    fn screen_size(&self) -> Vector2<u32> {
        let size = self.gfx_context.window.drawable_size();
        Vector2::new(size.0, size.1)
    }
}