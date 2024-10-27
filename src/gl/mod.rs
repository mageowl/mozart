use std::error::Error;

use miniquad::{
    window, Backend, Bindings, BufferId, BufferSource, BufferType, BufferUsage, PassAction,
    RenderingBackend, ShaderMeta, ShaderSource, TextureAccess, TextureId, TextureParams,
    TextureSource,
};
use slotmap::{new_key_type, SlotMap};
use vertex::Vertex;

use crate::{
    game::assets::texture::Image,
    math::{
        color::Color,
        matrix::Matrix,
        point::{pt2, Pt2},
        transform::Transform,
    },
};
use shader::Shader;

mod shader;
pub mod vertex;

new_key_type! { pub struct ShaderId; }

pub struct GraphicsContext {
    ctx: Box<dyn RenderingBackend>,
    shaders: SlotMap<ShaderId, Shader>,
    default_shader: ShaderId,

    indices_square: BufferId,
    viewport_transform: Transform,
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

        let indices_square = ctx.new_buffer(
            BufferType::IndexBuffer,
            BufferUsage::Immutable,
            BufferSource::slice(&[0, 1, 2, 0, 2, 3]),
        );

        let size = window::screen_size();
        let viewport_transform = Transform::new(
            Matrix::new([[1. / size.0, 0.], [0., -1. / size.1]]),
            pt2(-1.0, 1.0),
            Pt2::ZERO,
        );

        Ok(Self {
            default_shader,
            shaders,
            indices_square,
            ctx,
            viewport_transform,
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
    pub(crate) fn create_texture(&mut self, image: &Image) -> TextureId {
        self.ctx.new_texture(
            TextureAccess::Static,
            TextureSource::Bytes(&image.bytes),
            TextureParams {
                width: image.width,
                height: image.height,
                mag_filter: miniquad::FilterMode::Nearest,
                ..Default::default()
            },
        )
    }

    pub fn create_vertex_buffer(&mut self, size: usize) -> BufferId {
        self.ctx.new_buffer(
            BufferType::VertexBuffer,
            BufferUsage::Stream,
            BufferSource::empty::<Vertex>(size),
        )
    }
    pub fn create_index_buffer(&mut self, size: usize) -> BufferId {
        self.ctx.new_buffer(
            BufferType::IndexBuffer,
            BufferUsage::Stream,
            BufferSource::empty::<usize>(size),
        )
    }
    pub fn update_buffer(&mut self, buffer: BufferId, data: BufferSource) {
        self.ctx.buffer_update(buffer, data)
    }

    pub(crate) fn start_frame(&mut self, color: Color) {
        let color: [f32; 4] = color.into();
        self.ctx.begin_default_pass(PassAction::clear_color(
            color[0], color[1], color[2], color[3],
        ))
    }
    pub(crate) fn finish(&mut self) {
        self.ctx.end_render_pass();
        self.ctx.commit_frame();
    }

    pub fn draw(&mut self, bindings: &Bindings, num_indices: i32) {
        self.ctx
            .apply_pipeline(&self.shaders[self.default_shader].pipeline);
        self.ctx.apply_bindings(bindings);
        self.ctx.draw(0, num_indices, 1);
    }

    pub fn indices_square(&self) -> BufferId {
        self.indices_square
    }
    pub fn viewport_transform(&self) -> Transform {
        self.viewport_transform
    }
    pub(crate) fn update_viewport_transform(&mut self, size: Pt2) {
        self.viewport_transform = Transform::new(
            Matrix::new([[2. / size.x, 0.], [0., -2. / size.y]]),
            pt2(-1.0, 1.0),
            Pt2::ZERO,
        );
    }
}
