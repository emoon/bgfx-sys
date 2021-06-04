#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

include!("./ffi.rs");

pub const BGFX_RESET_VSYNC: u32 = 0x00000080;

pub const BGFX_CLEAR_NONE: u16 = 0x0000;
pub const BGFX_CLEAR_COLOR: u16 = 0x0001;
pub const BGFX_CLEAR_DEPTH: u16 = 0x0002;
pub const BGFX_CLEAR_STENCIL: u16 = 0x0004;
pub const BGFX_CLEAR_DISCARD_COLOR_0: u16 = 0x0008;
pub const BGFX_CLEAR_DISCARD_COLOR_1: u16 = 0x0010;
pub const BGFX_CLEAR_DISCARD_COLOR_2: u16 = 0x0020;
pub const BGFX_CLEAR_DISCARD_COLOR_3: u16 = 0x0040;
pub const BGFX_CLEAR_DISCARD_COLOR_4: u16 = 0x0080;
pub const BGFX_CLEAR_DISCARD_COLOR_5: u16 = 0x0100;
pub const BGFX_CLEAR_DISCARD_COLOR_6: u16 = 0x0200;
pub const BGFX_CLEAR_DISCARD_COLOR_7: u16 = 0x0400;
pub const BGFX_CLEAR_DISCARD_DEPTH: u16 = 0x0800;
pub const BGFX_CLEAR_DISCARD_STENCIL: u16 = 0x1000;

pub const BGFX_DEBUG_NONE: u32 = 0x00000000;
pub const BGFX_DEBUG_WIREFRAME: u32 = 0x00000001;
pub const BGFX_DEBUG_IFH: u32 = 0x00000002;
pub const BGFX_DEBUG_STATS: u32 = 0x00000004;
pub const BGFX_DEBUG_TEXT: u32 = 0x00000008;
