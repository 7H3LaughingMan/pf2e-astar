use bytemuck::{Pod, Zeroable};

#[derive(Pod, Zeroable)]
#[derive(Copy, Clone)]
#[derive(PartialEq, Eq)]
#[derive(Debug)]
#[repr(C)]
pub struct Pixel {
    pub r: u8,
    pub g: u8,
    pub b: u8,
    pub a: u8,
}

impl Pixel {
    pub const RED: Pixel = Pixel { r: 255, g: 0, b: 0, a: 255 };
}
