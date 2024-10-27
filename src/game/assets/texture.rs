use crate::{
    gl::GraphicsContext,
    math::{
        color::Color,
        point::{pt2i, Pt2i},
    },
};

use super::{Asset, GlAsset};

pub struct Image {
    pub width: u32,
    pub height: u32,
    pub bytes: Vec<u8>,
}

impl Image {
    pub fn from_color(width: u32, height: u32, color: Color) -> Self {
        Self {
            width,
            height,
            bytes: (0..width * height)
                .flat_map(|_| Into::<[u8; 4]>::into(color))
                .collect(),
        }
    }

    /// Size in pixels of texture
    pub fn size(&self) -> Pt2i {
        pt2i(self.width as i32, self.height as i32)
    }
}

impl Asset for Image {
    fn load(data: &[u8]) -> Result<Self, Box<dyn std::error::Error>> {
        let img = image::load_from_memory(data)?.into_rgba8();

        Ok(Self {
            width: img.width(),
            height: img.height(),
            bytes: img.into_raw(),
        })
    }
}

pub struct Texture {
    pub image: Image,
    pub gl_texture: miniquad::TextureId,
}

impl GlAsset for Texture {
    fn load(data: &[u8], gl: &mut GraphicsContext) -> Result<Self, Box<dyn std::error::Error>> {
        let image = Image::load(data)?;
        Ok(Self {
            gl_texture: gl.create_texture(&image),
            image,
        })
    }
}
