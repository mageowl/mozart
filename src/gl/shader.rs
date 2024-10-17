use miniquad::{
    BufferLayout, Pipeline, PipelineParams, RenderingBackend, VertexAttribute, VertexFormat,
};

pub mod default {
    use miniquad::{ShaderMeta, UniformBlockLayout};

    pub const VERTEX: &str = include_str!("./vertex.glsl");
    pub const FRAGMENT: &str = include_str!("./frag.glsl");

    pub fn meta() -> ShaderMeta {
        ShaderMeta {
            images: vec!["tex".to_string()],
            uniforms: UniformBlockLayout { uniforms: vec![] },
        }
    }
}

pub struct Shader {
    pub(super) pipeline: Pipeline,
}

impl Shader {
    pub(crate) fn new(
        ctx: &mut Box<dyn RenderingBackend>,
        quad_shader: miniquad::ShaderId,
    ) -> Self {
        let pipeline = ctx.new_pipeline(
            &[BufferLayout::default()],
            &[
                VertexAttribute::new("in_pos", VertexFormat::Float2),
                VertexAttribute::new("in_uv", VertexFormat::Float2),
            ],
            quad_shader,
            PipelineParams::default(),
        );

        Self { pipeline }
    }
}
