use bevy::prelude::*;

pub fn color_u32(value: u32) -> Color {
    let r = ((value >> 24) & 0xFF) as f32 / 255.0;
    let g = ((value >> 16) & 0xFF) as f32 / 255.0;
    let b = ((value >> 8) & 0xFF) as f32 / 255.0;
    let a = (value & 0xFF) as f32 / 255.0;
    Color::srgba(r, g, b, a)
}
