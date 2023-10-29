#[allow(unused)]
pub fn pack_rgba8_to_u32(rgba: &[u8]) -> u32 {
    ((rgba[0] as u32) << 24) | ((rgba[1] as u32) << 16) | ((rgba[2] as u32) << 8) | rgba[3] as u32
}

pub fn unpack_u32_to_rgba8(value: u32) -> [u8; 4] {
    [
        ((value >> 24) & 0xff) as u8,
        ((value >> 16) & 0xff) as u8,
        ((value >> 8) & 0xff) as u8,
        (value & 0xff) as u8,
    ]
}

pub fn unpack_u32_to_rgba_f32(value: u32) -> [f32; 4] {
    let rgba8 = unpack_u32_to_rgba8(value);
    [
        rgba8[0] as f32 / 255.0,
        rgba8[1] as f32 / 255.0,
        rgba8[2] as f32 / 255.0,
        rgba8[3] as f32 / 255.0,
    ]
}

#[allow(unused)]
pub fn unpack_u32_to_color(value: u32) -> wgpu::Color {
    let rgba8 = unpack_u32_to_rgba8(value);
    wgpu::Color {
        r: rgba8[0] as f64 / 255.0,
        g: rgba8[1] as f64 / 255.0,
        b: rgba8[2] as f64 / 255.0,
        a: rgba8[3] as f64 / 255.0,
    }
}

/// 几个 iOS 颜色
pub static VISION_COLORS: [u32; 29] = [
    0xE2CF95ff, 0xC9E1A0ff, 0xEBC6B2ff, 0x8cacdaff, 0xbed35eff, 0xcad8bbff, 0xbdb8a7, 0xd8c19fff,
    0xf2e175ff, 0xafcdf3ff, 0xfdf2bdff, 0xfffec2ff, 0xff4962ff, 0x36C2D8ff, 0x21FEFEff, 0x565656ff,
    0x42F2FFff, 0x1597F5ff, 0x48D7FFff, 0x9AF47Aff, 0x8DDFFEff, 0xFFA94Eff, 0xF3D82Dff, 0xFEC859ff,
    0x8ADECFff, 0xFD9CC0ff, 0xC773D6ff, 0x71C3FDff, 0x8961F6ff,
];
