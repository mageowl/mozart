use std::sync::Arc;

use miniquad::{Bindings, BufferSource};
use mozart_macro::Obj2d;

use super::{Draw, Make, Obj};
use crate::{
    self as mozart,
    game::assets::texture::Texture,
    gl::{vertex::Vertex, GraphicsContext},
    math::{point::pt2, transform::Transform},
};

#[derive(Obj, Obj2d)]
pub struct Sprite {
    transform: Transform,

    texture: Arc<Texture>,
    bindings: Bindings,
}

pub struct SpriteConf {
    texture_path: &'static str,
}

impl Sprite {
    pub fn cfg_from_texture(path: &'static str) -> SpriteConf {
        SpriteConf { texture_path: path }
    }
}

impl Make for Sprite {
    type Config = SpriteConf;

    fn make(game: &mut crate::game::Game, config: Self::Config) -> Self {
        let texture: Arc<Texture> = game.load_gl_asset(config.texture_path);
        Self {
            transform: Transform::IDENTITY,

            bindings: Bindings {
                index_buffer: game.gl.indices_square,
                vertex_buffers: vec![game.gl.create_vertex_buffer(4)],
                images: vec![texture.gl_texture],
            },
            texture,
        }
    }
}

impl Draw for Sprite {
    fn draw(&self, ctx: &mut GraphicsContext) {
        let w = self.texture.image.width as f32;
        let h = self.texture.image.height as f32;

        #[rustfmt::skip]
        let mut vertices = vec![
            Vertex { pos: pt2(0., 0.), uv: pt2(0., 0.) },
            Vertex { pos: pt2(w, 0.), uv: pt2(1., 0.) },
            Vertex { pos: pt2(w, h), uv: pt2(1., 1.) },
            Vertex { pos: pt2(0., h), uv: pt2(0., 1.) },
        ];

        for i in 0..vertices.len() {
            vertices[i].pos *= self.transform;
            vertices[i].pos *= ctx.viewport_transform();
        }

        ctx.update_buffer(
            self.bindings.vertex_buffers[0],
            BufferSource::slice(&vertices),
        );
        ctx.draw(&self.bindings, 6);
    }
}
