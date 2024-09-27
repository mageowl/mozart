use std::error::Error;

use miniquad::{window, Backend, PassAction, RenderingBackend, ShaderMeta, ShaderSource};
use slotmap::{new_key_type, SlotMap};

use shader::Shader;

use crate::math::color::Color;

mod shader;

new_key_type! { pub struct ShaderId; }

pub struct GraphicsContext {
    ctx: Box<dyn RenderingBackend>,
    shaders: SlotMap<ShaderId, Shader>,
    default_shader: ShaderId,
}

impl GraphicsContext {
    pub(crate) fn new() -> Result<Self, Box<dyn Error>> {
        let mut ctx = window::new_rendering_backend();

        let mut shaders = SlotMap::with_key();
        let default_shader = ctx.new_shader(
            match ctx.info().backend {
                Backend::OpenGl => ShaderSource::Glsl {
                    vertex: shader::default::VERTEX,
                    fragment: shader::default::FRAGMENT,
                },
                Backend::Metal => panic!("metal is not supported yet."),
            },
            shader::default::meta(),
        )?;
        let default_shader = shaders.insert(Shader::new(&mut ctx, default_shader));

        Ok(Self {
            default_shader,
            shaders,
            ctx,
        })
    }

    pub(crate) fn create_shader(
        &mut self,
        vertex: &str,
        fragment: &str,
        metal: &str,
        meta: ShaderMeta,
    ) -> Result<ShaderId, miniquad::ShaderError> {
        let shader = self.ctx.new_shader(
            match self.ctx.info().backend {
                Backend::OpenGl => ShaderSource::Glsl { vertex, fragment },
                Backend::Metal => ShaderSource::Msl { program: metal },
            },
            meta,
        )?;
        Ok(self.shaders.insert(Shader::new(&mut self.ctx, shader)))
    }

    pub(crate) fn clear(&mut self, color: Color) {
        let pass_action = PassAction::clear_color(color.r, color.g, color.b, color.a);
    }
}
