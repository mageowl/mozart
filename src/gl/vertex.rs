use crate::math::point::Pt2;

#[repr(C)]
#[derive(Debug)]
pub struct Vertex {
    pub pos: Pt2,
    pub uv: Pt2,
}

impl Vertex {
    pub fn new(pos: Pt2, uv: Pt2) -> Self {
        Self { pos, uv }
    }
}
