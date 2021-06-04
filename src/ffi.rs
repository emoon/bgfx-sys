pub const BGFX_FATAL_DEBUG_CHECK: bgfx_fatal = 0;
#[doc = " ( 0)"]
pub const BGFX_FATAL_INVALID_SHADER: bgfx_fatal = 1;
#[doc = " ( 1)"]
pub const BGFX_FATAL_UNABLE_TO_INITIALIZE: bgfx_fatal = 2;
#[doc = " ( 2)"]
pub const BGFX_FATAL_UNABLE_TO_CREATE_TEXTURE: bgfx_fatal = 3;
#[doc = " ( 3)"]
pub const BGFX_FATAL_DEVICE_LOST: bgfx_fatal = 4;
#[doc = " ( 4)"]
pub const BGFX_FATAL_COUNT: bgfx_fatal = 5;
#[doc = " Fatal error enum."]
#[doc = ""]
pub type bgfx_fatal = u32;
#[doc = " Fatal error enum."]
#[doc = ""]
pub use self::bgfx_fatal as bgfx_fatal_t;
pub const BGFX_RENDERER_TYPE_NOOP: bgfx_renderer_type = 0;
#[doc = " ( 0) No rendering."]
pub const BGFX_RENDERER_TYPE_DIRECT3D9: bgfx_renderer_type = 1;
#[doc = " ( 1) Direct3D 9.0"]
pub const BGFX_RENDERER_TYPE_DIRECT3D11: bgfx_renderer_type = 2;
#[doc = " ( 2) Direct3D 11.0"]
pub const BGFX_RENDERER_TYPE_DIRECT3D12: bgfx_renderer_type = 3;
#[doc = " ( 3) Direct3D 12.0"]
pub const BGFX_RENDERER_TYPE_GNM: bgfx_renderer_type = 4;
#[doc = " ( 4) GNM"]
pub const BGFX_RENDERER_TYPE_METAL: bgfx_renderer_type = 5;
#[doc = " ( 5) Metal"]
pub const BGFX_RENDERER_TYPE_NVN: bgfx_renderer_type = 6;
#[doc = " ( 6) NVN"]
pub const BGFX_RENDERER_TYPE_OPENGLES: bgfx_renderer_type = 7;
#[doc = " ( 7) OpenGL ES 2.0+"]
pub const BGFX_RENDERER_TYPE_OPENGL: bgfx_renderer_type = 8;
#[doc = " ( 8) OpenGL 2.1+"]
pub const BGFX_RENDERER_TYPE_VULKAN: bgfx_renderer_type = 9;
#[doc = " ( 9) Vulkan"]
pub const BGFX_RENDERER_TYPE_WEBGPU: bgfx_renderer_type = 10;
#[doc = " (10) WebGPU"]
pub const BGFX_RENDERER_TYPE_COUNT: bgfx_renderer_type = 11;
#[doc = " Renderer backend type enum."]
#[doc = ""]
pub type bgfx_renderer_type = u32;
#[doc = " Renderer backend type enum."]
#[doc = ""]
pub use self::bgfx_renderer_type as bgfx_renderer_type_t;
pub const BGFX_ACCESS_READ: bgfx_access = 0;
#[doc = " ( 0) Read."]
pub const BGFX_ACCESS_WRITE: bgfx_access = 1;
#[doc = " ( 1) Write."]
pub const BGFX_ACCESS_READWRITE: bgfx_access = 2;
#[doc = " ( 2) Read and write."]
pub const BGFX_ACCESS_COUNT: bgfx_access = 3;
#[doc = " Access mode enum."]
#[doc = ""]
pub type bgfx_access = u32;
#[doc = " Access mode enum."]
#[doc = ""]
pub use self::bgfx_access as bgfx_access_t;
pub const BGFX_ATTRIB_POSITION: bgfx_attrib = 0;
#[doc = " ( 0) a_position"]
pub const BGFX_ATTRIB_NORMAL: bgfx_attrib = 1;
#[doc = " ( 1) a_normal"]
pub const BGFX_ATTRIB_TANGENT: bgfx_attrib = 2;
#[doc = " ( 2) a_tangent"]
pub const BGFX_ATTRIB_BITANGENT: bgfx_attrib = 3;
#[doc = " ( 3) a_bitangent"]
pub const BGFX_ATTRIB_COLOR0: bgfx_attrib = 4;
#[doc = " ( 4) a_color0"]
pub const BGFX_ATTRIB_COLOR1: bgfx_attrib = 5;
#[doc = " ( 5) a_color1"]
pub const BGFX_ATTRIB_COLOR2: bgfx_attrib = 6;
#[doc = " ( 6) a_color2"]
pub const BGFX_ATTRIB_COLOR3: bgfx_attrib = 7;
#[doc = " ( 7) a_color3"]
pub const BGFX_ATTRIB_INDICES: bgfx_attrib = 8;
#[doc = " ( 8) a_indices"]
pub const BGFX_ATTRIB_WEIGHT: bgfx_attrib = 9;
#[doc = " ( 9) a_weight"]
pub const BGFX_ATTRIB_TEXCOORD0: bgfx_attrib = 10;
#[doc = " (10) a_texcoord0"]
pub const BGFX_ATTRIB_TEXCOORD1: bgfx_attrib = 11;
#[doc = " (11) a_texcoord1"]
pub const BGFX_ATTRIB_TEXCOORD2: bgfx_attrib = 12;
#[doc = " (12) a_texcoord2"]
pub const BGFX_ATTRIB_TEXCOORD3: bgfx_attrib = 13;
#[doc = " (13) a_texcoord3"]
pub const BGFX_ATTRIB_TEXCOORD4: bgfx_attrib = 14;
#[doc = " (14) a_texcoord4"]
pub const BGFX_ATTRIB_TEXCOORD5: bgfx_attrib = 15;
#[doc = " (15) a_texcoord5"]
pub const BGFX_ATTRIB_TEXCOORD6: bgfx_attrib = 16;
#[doc = " (16) a_texcoord6"]
pub const BGFX_ATTRIB_TEXCOORD7: bgfx_attrib = 17;
#[doc = " (17) a_texcoord7"]
pub const BGFX_ATTRIB_COUNT: bgfx_attrib = 18;
#[doc = " Vertex attribute enum."]
#[doc = ""]
pub type bgfx_attrib = u32;
#[doc = " Vertex attribute enum."]
#[doc = ""]
pub use self::bgfx_attrib as bgfx_attrib_t;
pub const BGFX_ATTRIB_TYPE_UINT8: bgfx_attrib_type = 0;
#[doc = " ( 0) Uint8"]
pub const BGFX_ATTRIB_TYPE_UINT10: bgfx_attrib_type = 1;
#[doc = " ( 1) Uint10, availability depends on: `BGFX_CAPS_VERTEX_ATTRIB_UINT10`."]
pub const BGFX_ATTRIB_TYPE_INT16: bgfx_attrib_type = 2;
#[doc = " ( 2) Int16"]
pub const BGFX_ATTRIB_TYPE_HALF: bgfx_attrib_type = 3;
#[doc = " ( 3) Half, availability depends on: `BGFX_CAPS_VERTEX_ATTRIB_HALF`."]
pub const BGFX_ATTRIB_TYPE_FLOAT: bgfx_attrib_type = 4;
#[doc = " ( 4) Float"]
pub const BGFX_ATTRIB_TYPE_COUNT: bgfx_attrib_type = 5;
#[doc = " Vertex attribute type enum."]
#[doc = ""]
pub type bgfx_attrib_type = u32;
#[doc = " Vertex attribute type enum."]
#[doc = ""]
pub use self::bgfx_attrib_type as bgfx_attrib_type_t;
pub const BGFX_TEXTURE_FORMAT_BC1: bgfx_texture_format = 0;
#[doc = " ( 0) DXT1 R5G6B5A1"]
pub const BGFX_TEXTURE_FORMAT_BC2: bgfx_texture_format = 1;
#[doc = " ( 1) DXT3 R5G6B5A4"]
pub const BGFX_TEXTURE_FORMAT_BC3: bgfx_texture_format = 2;
#[doc = " ( 2) DXT5 R5G6B5A8"]
pub const BGFX_TEXTURE_FORMAT_BC4: bgfx_texture_format = 3;
#[doc = " ( 3) LATC1/ATI1 R8"]
pub const BGFX_TEXTURE_FORMAT_BC5: bgfx_texture_format = 4;
#[doc = " ( 4) LATC2/ATI2 RG8"]
pub const BGFX_TEXTURE_FORMAT_BC6H: bgfx_texture_format = 5;
#[doc = " ( 5) BC6H RGB16F"]
pub const BGFX_TEXTURE_FORMAT_BC7: bgfx_texture_format = 6;
#[doc = " ( 6) BC7 RGB 4-7 bits per color channel, 0-8 bits alpha"]
pub const BGFX_TEXTURE_FORMAT_ETC1: bgfx_texture_format = 7;
#[doc = " ( 7) ETC1 RGB8"]
pub const BGFX_TEXTURE_FORMAT_ETC2: bgfx_texture_format = 8;
#[doc = " ( 8) ETC2 RGB8"]
pub const BGFX_TEXTURE_FORMAT_ETC2A: bgfx_texture_format = 9;
#[doc = " ( 9) ETC2 RGBA8"]
pub const BGFX_TEXTURE_FORMAT_ETC2A1: bgfx_texture_format = 10;
#[doc = " (10) ETC2 RGB8A1"]
pub const BGFX_TEXTURE_FORMAT_PTC12: bgfx_texture_format = 11;
#[doc = " (11) PVRTC1 RGB 2BPP"]
pub const BGFX_TEXTURE_FORMAT_PTC14: bgfx_texture_format = 12;
#[doc = " (12) PVRTC1 RGB 4BPP"]
pub const BGFX_TEXTURE_FORMAT_PTC12A: bgfx_texture_format = 13;
#[doc = " (13) PVRTC1 RGBA 2BPP"]
pub const BGFX_TEXTURE_FORMAT_PTC14A: bgfx_texture_format = 14;
#[doc = " (14) PVRTC1 RGBA 4BPP"]
pub const BGFX_TEXTURE_FORMAT_PTC22: bgfx_texture_format = 15;
#[doc = " (15) PVRTC2 RGBA 2BPP"]
pub const BGFX_TEXTURE_FORMAT_PTC24: bgfx_texture_format = 16;
#[doc = " (16) PVRTC2 RGBA 4BPP"]
pub const BGFX_TEXTURE_FORMAT_ATC: bgfx_texture_format = 17;
#[doc = " (17) ATC RGB 4BPP"]
pub const BGFX_TEXTURE_FORMAT_ATCE: bgfx_texture_format = 18;
#[doc = " (18) ATCE RGBA 8 BPP explicit alpha"]
pub const BGFX_TEXTURE_FORMAT_ATCI: bgfx_texture_format = 19;
#[doc = " (19) ATCI RGBA 8 BPP interpolated alpha"]
pub const BGFX_TEXTURE_FORMAT_ASTC4X4: bgfx_texture_format = 20;
#[doc = " (20) ASTC 4x4 8.0 BPP"]
pub const BGFX_TEXTURE_FORMAT_ASTC5X5: bgfx_texture_format = 21;
#[doc = " (21) ASTC 5x5 5.12 BPP"]
pub const BGFX_TEXTURE_FORMAT_ASTC6X6: bgfx_texture_format = 22;
#[doc = " (22) ASTC 6x6 3.56 BPP"]
pub const BGFX_TEXTURE_FORMAT_ASTC8X5: bgfx_texture_format = 23;
#[doc = " (23) ASTC 8x5 3.20 BPP"]
pub const BGFX_TEXTURE_FORMAT_ASTC8X6: bgfx_texture_format = 24;
#[doc = " (24) ASTC 8x6 2.67 BPP"]
pub const BGFX_TEXTURE_FORMAT_ASTC10X5: bgfx_texture_format = 25;
#[doc = " (25) ASTC 10x5 2.56 BPP"]
pub const BGFX_TEXTURE_FORMAT_UNKNOWN: bgfx_texture_format = 26;
#[doc = " (26) Compressed formats above."]
pub const BGFX_TEXTURE_FORMAT_R1: bgfx_texture_format = 27;
#[doc = " (27)"]
pub const BGFX_TEXTURE_FORMAT_A8: bgfx_texture_format = 28;
#[doc = " (28)"]
pub const BGFX_TEXTURE_FORMAT_R8: bgfx_texture_format = 29;
#[doc = " (29)"]
pub const BGFX_TEXTURE_FORMAT_R8I: bgfx_texture_format = 30;
#[doc = " (30)"]
pub const BGFX_TEXTURE_FORMAT_R8U: bgfx_texture_format = 31;
#[doc = " (31)"]
pub const BGFX_TEXTURE_FORMAT_R8S: bgfx_texture_format = 32;
#[doc = " (32)"]
pub const BGFX_TEXTURE_FORMAT_R16: bgfx_texture_format = 33;
#[doc = " (33)"]
pub const BGFX_TEXTURE_FORMAT_R16I: bgfx_texture_format = 34;
#[doc = " (34)"]
pub const BGFX_TEXTURE_FORMAT_R16U: bgfx_texture_format = 35;
#[doc = " (35)"]
pub const BGFX_TEXTURE_FORMAT_R16F: bgfx_texture_format = 36;
#[doc = " (36)"]
pub const BGFX_TEXTURE_FORMAT_R16S: bgfx_texture_format = 37;
#[doc = " (37)"]
pub const BGFX_TEXTURE_FORMAT_R32I: bgfx_texture_format = 38;
#[doc = " (38)"]
pub const BGFX_TEXTURE_FORMAT_R32U: bgfx_texture_format = 39;
#[doc = " (39)"]
pub const BGFX_TEXTURE_FORMAT_R32F: bgfx_texture_format = 40;
#[doc = " (40)"]
pub const BGFX_TEXTURE_FORMAT_RG8: bgfx_texture_format = 41;
#[doc = " (41)"]
pub const BGFX_TEXTURE_FORMAT_RG8I: bgfx_texture_format = 42;
#[doc = " (42)"]
pub const BGFX_TEXTURE_FORMAT_RG8U: bgfx_texture_format = 43;
#[doc = " (43)"]
pub const BGFX_TEXTURE_FORMAT_RG8S: bgfx_texture_format = 44;
#[doc = " (44)"]
pub const BGFX_TEXTURE_FORMAT_RG16: bgfx_texture_format = 45;
#[doc = " (45)"]
pub const BGFX_TEXTURE_FORMAT_RG16I: bgfx_texture_format = 46;
#[doc = " (46)"]
pub const BGFX_TEXTURE_FORMAT_RG16U: bgfx_texture_format = 47;
#[doc = " (47)"]
pub const BGFX_TEXTURE_FORMAT_RG16F: bgfx_texture_format = 48;
#[doc = " (48)"]
pub const BGFX_TEXTURE_FORMAT_RG16S: bgfx_texture_format = 49;
#[doc = " (49)"]
pub const BGFX_TEXTURE_FORMAT_RG32I: bgfx_texture_format = 50;
#[doc = " (50)"]
pub const BGFX_TEXTURE_FORMAT_RG32U: bgfx_texture_format = 51;
#[doc = " (51)"]
pub const BGFX_TEXTURE_FORMAT_RG32F: bgfx_texture_format = 52;
#[doc = " (52)"]
pub const BGFX_TEXTURE_FORMAT_RGB8: bgfx_texture_format = 53;
#[doc = " (53)"]
pub const BGFX_TEXTURE_FORMAT_RGB8I: bgfx_texture_format = 54;
#[doc = " (54)"]
pub const BGFX_TEXTURE_FORMAT_RGB8U: bgfx_texture_format = 55;
#[doc = " (55)"]
pub const BGFX_TEXTURE_FORMAT_RGB8S: bgfx_texture_format = 56;
#[doc = " (56)"]
pub const BGFX_TEXTURE_FORMAT_RGB9E5F: bgfx_texture_format = 57;
#[doc = " (57)"]
pub const BGFX_TEXTURE_FORMAT_BGRA8: bgfx_texture_format = 58;
#[doc = " (58)"]
pub const BGFX_TEXTURE_FORMAT_RGBA8: bgfx_texture_format = 59;
#[doc = " (59)"]
pub const BGFX_TEXTURE_FORMAT_RGBA8I: bgfx_texture_format = 60;
#[doc = " (60)"]
pub const BGFX_TEXTURE_FORMAT_RGBA8U: bgfx_texture_format = 61;
#[doc = " (61)"]
pub const BGFX_TEXTURE_FORMAT_RGBA8S: bgfx_texture_format = 62;
#[doc = " (62)"]
pub const BGFX_TEXTURE_FORMAT_RGBA16: bgfx_texture_format = 63;
#[doc = " (63)"]
pub const BGFX_TEXTURE_FORMAT_RGBA16I: bgfx_texture_format = 64;
#[doc = " (64)"]
pub const BGFX_TEXTURE_FORMAT_RGBA16U: bgfx_texture_format = 65;
#[doc = " (65)"]
pub const BGFX_TEXTURE_FORMAT_RGBA16F: bgfx_texture_format = 66;
#[doc = " (66)"]
pub const BGFX_TEXTURE_FORMAT_RGBA16S: bgfx_texture_format = 67;
#[doc = " (67)"]
pub const BGFX_TEXTURE_FORMAT_RGBA32I: bgfx_texture_format = 68;
#[doc = " (68)"]
pub const BGFX_TEXTURE_FORMAT_RGBA32U: bgfx_texture_format = 69;
#[doc = " (69)"]
pub const BGFX_TEXTURE_FORMAT_RGBA32F: bgfx_texture_format = 70;
#[doc = " (70)"]
pub const BGFX_TEXTURE_FORMAT_R5G6B5: bgfx_texture_format = 71;
#[doc = " (71)"]
pub const BGFX_TEXTURE_FORMAT_RGBA4: bgfx_texture_format = 72;
#[doc = " (72)"]
pub const BGFX_TEXTURE_FORMAT_RGB5A1: bgfx_texture_format = 73;
#[doc = " (73)"]
pub const BGFX_TEXTURE_FORMAT_RGB10A2: bgfx_texture_format = 74;
#[doc = " (74)"]
pub const BGFX_TEXTURE_FORMAT_RG11B10F: bgfx_texture_format = 75;
#[doc = " (75)"]
pub const BGFX_TEXTURE_FORMAT_UNKNOWNDEPTH: bgfx_texture_format = 76;
#[doc = " (76) Depth formats below."]
pub const BGFX_TEXTURE_FORMAT_D16: bgfx_texture_format = 77;
#[doc = " (77)"]
pub const BGFX_TEXTURE_FORMAT_D24: bgfx_texture_format = 78;
#[doc = " (78)"]
pub const BGFX_TEXTURE_FORMAT_D24S8: bgfx_texture_format = 79;
#[doc = " (79)"]
pub const BGFX_TEXTURE_FORMAT_D32: bgfx_texture_format = 80;
#[doc = " (80)"]
pub const BGFX_TEXTURE_FORMAT_D16F: bgfx_texture_format = 81;
#[doc = " (81)"]
pub const BGFX_TEXTURE_FORMAT_D24F: bgfx_texture_format = 82;
#[doc = " (82)"]
pub const BGFX_TEXTURE_FORMAT_D32F: bgfx_texture_format = 83;
#[doc = " (83)"]
pub const BGFX_TEXTURE_FORMAT_D0S8: bgfx_texture_format = 84;
#[doc = " (84)"]
pub const BGFX_TEXTURE_FORMAT_COUNT: bgfx_texture_format = 85;
#[doc = " Texture format enum."]
#[doc = " Notation:"]
#[doc = "       RGBA16S"]
#[doc = "       ^   ^ ^"]
#[doc = "       |   | +-- [ ]Unorm"]
#[doc = "       |   |     [F]loat"]
#[doc = "       |   |     [S]norm"]
#[doc = "       |   |     [I]nt"]
#[doc = "       |   |     [U]int"]
#[doc = "       |   +---- Number of bits per component"]
#[doc = "       +-------- Components"]
#[doc = " @attention Availability depends on Caps (see: formats)."]
#[doc = ""]
pub type bgfx_texture_format = u32;
#[doc = " Texture format enum."]
#[doc = " Notation:"]
#[doc = "       RGBA16S"]
#[doc = "       ^   ^ ^"]
#[doc = "       |   | +-- [ ]Unorm"]
#[doc = "       |   |     [F]loat"]
#[doc = "       |   |     [S]norm"]
#[doc = "       |   |     [I]nt"]
#[doc = "       |   |     [U]int"]
#[doc = "       |   +---- Number of bits per component"]
#[doc = "       +-------- Components"]
#[doc = " @attention Availability depends on Caps (see: formats)."]
#[doc = ""]
pub use self::bgfx_texture_format as bgfx_texture_format_t;
pub const BGFX_UNIFORM_TYPE_SAMPLER: bgfx_uniform_type = 0;
#[doc = " ( 0) Sampler."]
pub const BGFX_UNIFORM_TYPE_END: bgfx_uniform_type = 1;
#[doc = " ( 1) Reserved, do not use."]
pub const BGFX_UNIFORM_TYPE_VEC4: bgfx_uniform_type = 2;
#[doc = " ( 2) 4 floats vector."]
pub const BGFX_UNIFORM_TYPE_MAT3: bgfx_uniform_type = 3;
#[doc = " ( 3) 3x3 matrix."]
pub const BGFX_UNIFORM_TYPE_MAT4: bgfx_uniform_type = 4;
#[doc = " ( 4) 4x4 matrix."]
pub const BGFX_UNIFORM_TYPE_COUNT: bgfx_uniform_type = 5;
#[doc = " Uniform type enum."]
#[doc = ""]
pub type bgfx_uniform_type = u32;
#[doc = " Uniform type enum."]
#[doc = ""]
pub use self::bgfx_uniform_type as bgfx_uniform_type_t;
pub const BGFX_BACKBUFFER_RATIO_EQUAL: bgfx_backbuffer_ratio = 0;
#[doc = " ( 0) Equal to backbuffer."]
pub const BGFX_BACKBUFFER_RATIO_HALF: bgfx_backbuffer_ratio = 1;
#[doc = " ( 1) One half size of backbuffer."]
pub const BGFX_BACKBUFFER_RATIO_QUARTER: bgfx_backbuffer_ratio = 2;
#[doc = " ( 2) One quarter size of backbuffer."]
pub const BGFX_BACKBUFFER_RATIO_EIGHTH: bgfx_backbuffer_ratio = 3;
#[doc = " ( 3) One eighth size of backbuffer."]
pub const BGFX_BACKBUFFER_RATIO_SIXTEENTH: bgfx_backbuffer_ratio = 4;
#[doc = " ( 4) One sixteenth size of backbuffer."]
pub const BGFX_BACKBUFFER_RATIO_DOUBLE: bgfx_backbuffer_ratio = 5;
#[doc = " ( 5) Double size of backbuffer."]
pub const BGFX_BACKBUFFER_RATIO_COUNT: bgfx_backbuffer_ratio = 6;
#[doc = " Backbuffer ratio enum."]
#[doc = ""]
pub type bgfx_backbuffer_ratio = u32;
#[doc = " Backbuffer ratio enum."]
#[doc = ""]
pub use self::bgfx_backbuffer_ratio as bgfx_backbuffer_ratio_t;
pub const BGFX_OCCLUSION_QUERY_RESULT_INVISIBLE: bgfx_occlusion_query_result = 0;
#[doc = " ( 0) Query failed test."]
pub const BGFX_OCCLUSION_QUERY_RESULT_VISIBLE: bgfx_occlusion_query_result = 1;
#[doc = " ( 1) Query passed test."]
pub const BGFX_OCCLUSION_QUERY_RESULT_NORESULT: bgfx_occlusion_query_result = 2;
#[doc = " ( 2) Query result is not available yet."]
pub const BGFX_OCCLUSION_QUERY_RESULT_COUNT: bgfx_occlusion_query_result = 3;
#[doc = " Occlusion query result."]
#[doc = ""]
pub type bgfx_occlusion_query_result = u32;
#[doc = " Occlusion query result."]
#[doc = ""]
pub use self::bgfx_occlusion_query_result as bgfx_occlusion_query_result_t;
pub const BGFX_TOPOLOGY_TRI_LIST: bgfx_topology = 0;
#[doc = " ( 0) Triangle list."]
pub const BGFX_TOPOLOGY_TRI_STRIP: bgfx_topology = 1;
#[doc = " ( 1) Triangle strip."]
pub const BGFX_TOPOLOGY_LINE_LIST: bgfx_topology = 2;
#[doc = " ( 2) Line list."]
pub const BGFX_TOPOLOGY_LINE_STRIP: bgfx_topology = 3;
#[doc = " ( 3) Line strip."]
pub const BGFX_TOPOLOGY_POINT_LIST: bgfx_topology = 4;
#[doc = " ( 4) Point list."]
pub const BGFX_TOPOLOGY_COUNT: bgfx_topology = 5;
#[doc = " Primitive topology."]
#[doc = ""]
pub type bgfx_topology = u32;
#[doc = " Primitive topology."]
#[doc = ""]
pub use self::bgfx_topology as bgfx_topology_t;
pub const BGFX_TOPOLOGY_CONVERT_TRI_LIST_FLIP_WINDING: bgfx_topology_convert = 0;
#[doc = " ( 0) Flip winding order of triangle list."]
pub const BGFX_TOPOLOGY_CONVERT_TRI_STRIP_FLIP_WINDING: bgfx_topology_convert = 1;
#[doc = " ( 1) Flip winding order of triangle strip."]
pub const BGFX_TOPOLOGY_CONVERT_TRI_LIST_TO_LINE_LIST: bgfx_topology_convert = 2;
#[doc = " ( 2) Convert triangle list to line list."]
pub const BGFX_TOPOLOGY_CONVERT_TRI_STRIP_TO_TRI_LIST: bgfx_topology_convert = 3;
#[doc = " ( 3) Convert triangle strip to triangle list."]
pub const BGFX_TOPOLOGY_CONVERT_LINE_STRIP_TO_LINE_LIST: bgfx_topology_convert = 4;
#[doc = " ( 4) Convert line strip to line list."]
pub const BGFX_TOPOLOGY_CONVERT_COUNT: bgfx_topology_convert = 5;
#[doc = " Topology conversion function."]
#[doc = ""]
pub type bgfx_topology_convert = u32;
#[doc = " Topology conversion function."]
#[doc = ""]
pub use self::bgfx_topology_convert as bgfx_topology_convert_t;
pub const BGFX_TOPOLOGY_SORT_DIRECTION_FRONT_TO_BACK_MIN: bgfx_topology_sort = 0;
#[doc = " ( 0)"]
pub const BGFX_TOPOLOGY_SORT_DIRECTION_FRONT_TO_BACK_AVG: bgfx_topology_sort = 1;
#[doc = " ( 1)"]
pub const BGFX_TOPOLOGY_SORT_DIRECTION_FRONT_TO_BACK_MAX: bgfx_topology_sort = 2;
#[doc = " ( 2)"]
pub const BGFX_TOPOLOGY_SORT_DIRECTION_BACK_TO_FRONT_MIN: bgfx_topology_sort = 3;
#[doc = " ( 3)"]
pub const BGFX_TOPOLOGY_SORT_DIRECTION_BACK_TO_FRONT_AVG: bgfx_topology_sort = 4;
#[doc = " ( 4)"]
pub const BGFX_TOPOLOGY_SORT_DIRECTION_BACK_TO_FRONT_MAX: bgfx_topology_sort = 5;
#[doc = " ( 5)"]
pub const BGFX_TOPOLOGY_SORT_DISTANCE_FRONT_TO_BACK_MIN: bgfx_topology_sort = 6;
#[doc = " ( 6)"]
pub const BGFX_TOPOLOGY_SORT_DISTANCE_FRONT_TO_BACK_AVG: bgfx_topology_sort = 7;
#[doc = " ( 7)"]
pub const BGFX_TOPOLOGY_SORT_DISTANCE_FRONT_TO_BACK_MAX: bgfx_topology_sort = 8;
#[doc = " ( 8)"]
pub const BGFX_TOPOLOGY_SORT_DISTANCE_BACK_TO_FRONT_MIN: bgfx_topology_sort = 9;
#[doc = " ( 9)"]
pub const BGFX_TOPOLOGY_SORT_DISTANCE_BACK_TO_FRONT_AVG: bgfx_topology_sort = 10;
#[doc = " (10)"]
pub const BGFX_TOPOLOGY_SORT_DISTANCE_BACK_TO_FRONT_MAX: bgfx_topology_sort = 11;
#[doc = " (11)"]
pub const BGFX_TOPOLOGY_SORT_COUNT: bgfx_topology_sort = 12;
#[doc = " Topology sort order."]
#[doc = ""]
pub type bgfx_topology_sort = u32;
#[doc = " Topology sort order."]
#[doc = ""]
pub use self::bgfx_topology_sort as bgfx_topology_sort_t;
pub const BGFX_VIEW_MODE_DEFAULT: bgfx_view_mode = 0;
#[doc = " ( 0) Default sort order."]
pub const BGFX_VIEW_MODE_SEQUENTIAL: bgfx_view_mode = 1;
#[doc = " ( 1) Sort in the same order in which submit calls were called."]
pub const BGFX_VIEW_MODE_DEPTH_ASCENDING: bgfx_view_mode = 2;
#[doc = " ( 2) Sort draw call depth in ascending order."]
pub const BGFX_VIEW_MODE_DEPTH_DESCENDING: bgfx_view_mode = 3;
#[doc = " ( 3) Sort draw call depth in descending order."]
pub const BGFX_VIEW_MODE_COUNT: bgfx_view_mode = 4;
#[doc = " View mode sets draw call sort order."]
#[doc = ""]
pub type bgfx_view_mode = u32;
#[doc = " View mode sets draw call sort order."]
#[doc = ""]
pub use self::bgfx_view_mode as bgfx_view_mode_t;
pub const BGFX_RENDER_FRAME_NO_CONTEXT: bgfx_render_frame = 0;
#[doc = " ( 0) Renderer context is not created yet."]
pub const BGFX_RENDER_FRAME_RENDER: bgfx_render_frame = 1;
#[doc = " ( 1) Renderer context is created and rendering."]
pub const BGFX_RENDER_FRAME_TIMEOUT: bgfx_render_frame = 2;
#[doc = " ( 2) Renderer context wait for main thread signal timed out without rendering."]
pub const BGFX_RENDER_FRAME_EXITING: bgfx_render_frame = 3;
#[doc = " ( 3) Renderer context is getting destroyed."]
pub const BGFX_RENDER_FRAME_COUNT: bgfx_render_frame = 4;
#[doc = " Render frame enum."]
#[doc = ""]
pub type bgfx_render_frame = u32;
#[doc = " Render frame enum."]
#[doc = ""]
pub use self::bgfx_render_frame as bgfx_render_frame_t;
pub type bgfx_view_id_t = u16;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct bgfx_allocator_interface_s {
    pub vtbl: *const bgfx_allocator_vtbl_s,
}
pub type bgfx_allocator_interface_t = bgfx_allocator_interface_s;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct bgfx_allocator_vtbl_s {
    pub realloc: ::std::option::Option<
        unsafe extern "C" fn(
            _this: *mut bgfx_allocator_interface_t,
            _ptr: *mut ::std::os::raw::c_void,
            _size: size_t,
            _align: size_t,
            _file: *const ::std::os::raw::c_char,
            _line: u32,
        ) -> *mut ::std::os::raw::c_void,
    >,
}
pub type bgfx_allocator_vtbl_t = bgfx_allocator_vtbl_s;
pub type bgfx_interface_vtbl_t = bgfx_interface_vtbl;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct bgfx_callback_interface_s {
    pub vtbl: *const bgfx_callback_vtbl_s,
}
pub type bgfx_callback_interface_t = bgfx_callback_interface_s;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct bgfx_callback_vtbl_s {
    pub fatal: ::std::option::Option<
        unsafe extern "C" fn(
            _this: *mut bgfx_callback_interface_t,
            _filePath: *const ::std::os::raw::c_char,
            _line: u16,
            _code: bgfx_fatal_t,
            _str: *const ::std::os::raw::c_char,
        ),
    >,
    pub trace_vargs: ::std::option::Option<
        unsafe extern "C" fn(
            _this: *mut bgfx_callback_interface_t,
            _filePath: *const ::std::os::raw::c_char,
            _line: u16,
            _format: *const ::std::os::raw::c_char,
            _argList: *mut __va_list_tag,
        ),
    >,
    pub profiler_begin: ::std::option::Option<
        unsafe extern "C" fn(
            _this: *mut bgfx_callback_interface_t,
            _name: *const ::std::os::raw::c_char,
            _abgr: u32,
            _filePath: *const ::std::os::raw::c_char,
            _line: u16,
        ),
    >,
    pub profiler_begin_literal: ::std::option::Option<
        unsafe extern "C" fn(
            _this: *mut bgfx_callback_interface_t,
            _name: *const ::std::os::raw::c_char,
            _abgr: u32,
            _filePath: *const ::std::os::raw::c_char,
            _line: u16,
        ),
    >,
    pub profiler_end:
        ::std::option::Option<unsafe extern "C" fn(_this: *mut bgfx_callback_interface_t)>,
    pub cache_read_size: ::std::option::Option<
        unsafe extern "C" fn(_this: *mut bgfx_callback_interface_t, _id: u64) -> u32,
    >,
    pub cache_read: ::std::option::Option<
        unsafe extern "C" fn(
            _this: *mut bgfx_callback_interface_t,
            _id: u64,
            _data: *mut ::std::os::raw::c_void,
            _size: u32,
        ) -> bool,
    >,
    pub cache_write: ::std::option::Option<
        unsafe extern "C" fn(
            _this: *mut bgfx_callback_interface_t,
            _id: u64,
            _data: *const ::std::os::raw::c_void,
            _size: u32,
        ),
    >,
    pub screen_shot: ::std::option::Option<
        unsafe extern "C" fn(
            _this: *mut bgfx_callback_interface_t,
            _filePath: *const ::std::os::raw::c_char,
            _width: u32,
            _height: u32,
            _pitch: u32,
            _data: *const ::std::os::raw::c_void,
            _size: u32,
            _yflip: bool,
        ),
    >,
    pub capture_begin: ::std::option::Option<
        unsafe extern "C" fn(
            _this: *mut bgfx_callback_interface_t,
            _width: u32,
            _height: u32,
            _pitch: u32,
            _format: bgfx_texture_format_t,
            _yflip: bool,
        ),
    >,
    pub capture_end:
        ::std::option::Option<unsafe extern "C" fn(_this: *mut bgfx_callback_interface_t)>,
    pub capture_frame: ::std::option::Option<
        unsafe extern "C" fn(
            _this: *mut bgfx_callback_interface_t,
            _data: *const ::std::os::raw::c_void,
            _size: u32,
        ),
    >,
}
pub type bgfx_callback_vtbl_t = bgfx_callback_vtbl_s;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct bgfx_dynamic_index_buffer_handle_s {
    pub idx: u16,
}
pub type bgfx_dynamic_index_buffer_handle_t = bgfx_dynamic_index_buffer_handle_s;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct bgfx_dynamic_vertex_buffer_handle_s {
    pub idx: u16,
}
pub type bgfx_dynamic_vertex_buffer_handle_t = bgfx_dynamic_vertex_buffer_handle_s;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct bgfx_frame_buffer_handle_s {
    pub idx: u16,
}
pub type bgfx_frame_buffer_handle_t = bgfx_frame_buffer_handle_s;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct bgfx_index_buffer_handle_s {
    pub idx: u16,
}
pub type bgfx_index_buffer_handle_t = bgfx_index_buffer_handle_s;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct bgfx_indirect_buffer_handle_s {
    pub idx: u16,
}
pub type bgfx_indirect_buffer_handle_t = bgfx_indirect_buffer_handle_s;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct bgfx_occlusion_query_handle_s {
    pub idx: u16,
}
pub type bgfx_occlusion_query_handle_t = bgfx_occlusion_query_handle_s;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct bgfx_program_handle_s {
    pub idx: u16,
}
pub type bgfx_program_handle_t = bgfx_program_handle_s;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct bgfx_shader_handle_s {
    pub idx: u16,
}
pub type bgfx_shader_handle_t = bgfx_shader_handle_s;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct bgfx_texture_handle_s {
    pub idx: u16,
}
pub type bgfx_texture_handle_t = bgfx_texture_handle_s;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct bgfx_uniform_handle_s {
    pub idx: u16,
}
pub type bgfx_uniform_handle_t = bgfx_uniform_handle_s;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct bgfx_vertex_buffer_handle_s {
    pub idx: u16,
}
pub type bgfx_vertex_buffer_handle_t = bgfx_vertex_buffer_handle_s;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct bgfx_vertex_layout_handle_s {
    pub idx: u16,
}
pub type bgfx_vertex_layout_handle_t = bgfx_vertex_layout_handle_s;
#[doc = " Memory release callback."]
#[doc = ""]
#[doc = " @param[in] _ptr Pointer to allocated data."]
#[doc = " @param[in] _userData User defined data if needed."]
#[doc = ""]
pub type bgfx_release_fn_t = ::std::option::Option<
    unsafe extern "C" fn(_ptr: *mut ::std::os::raw::c_void, _userData: *mut ::std::os::raw::c_void),
>;
#[doc = " GPU info."]
#[doc = ""]
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct bgfx_caps_gpu_s {
    pub vendorId: u16,
    #[doc = " Vendor PCI id. See `BGFX_PCI_ID_*`."]
    pub deviceId: u16,
}
#[doc = " GPU info."]
#[doc = ""]
pub type bgfx_caps_gpu_t = bgfx_caps_gpu_s;
#[doc = " Renderer runtime limits."]
#[doc = ""]
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct bgfx_caps_limits_s {
    pub maxDrawCalls: u32,
    #[doc = " Maximum number of draw calls."]
    pub maxBlits: u32,
    #[doc = " Maximum number of blit calls."]
    pub maxTextureSize: u32,
    #[doc = " Maximum texture size."]
    pub maxTextureLayers: u32,
    #[doc = " Maximum texture layers."]
    pub maxViews: u32,
    #[doc = " Maximum number of views."]
    pub maxFrameBuffers: u32,
    #[doc = " Maximum number of frame buffer handles."]
    pub maxFBAttachments: u32,
    #[doc = " Maximum number of frame buffer attachments."]
    pub maxPrograms: u32,
    #[doc = " Maximum number of program handles."]
    pub maxShaders: u32,
    #[doc = " Maximum number of shader handles."]
    pub maxTextures: u32,
    #[doc = " Maximum number of texture handles."]
    pub maxTextureSamplers: u32,
    #[doc = " Maximum number of texture samplers."]
    pub maxComputeBindings: u32,
    #[doc = " Maximum number of compute bindings."]
    pub maxVertexLayouts: u32,
    #[doc = " Maximum number of vertex format layouts."]
    pub maxVertexStreams: u32,
    #[doc = " Maximum number of vertex streams."]
    pub maxIndexBuffers: u32,
    #[doc = " Maximum number of index buffer handles."]
    pub maxVertexBuffers: u32,
    #[doc = " Maximum number of vertex buffer handles."]
    pub maxDynamicIndexBuffers: u32,
    #[doc = " Maximum number of dynamic index buffer handles."]
    pub maxDynamicVertexBuffers: u32,
    #[doc = " Maximum number of dynamic vertex buffer handles."]
    pub maxUniforms: u32,
    #[doc = " Maximum number of uniform handles."]
    pub maxOcclusionQueries: u32,
    #[doc = " Maximum number of occlusion query handles."]
    pub maxEncoders: u32,
    #[doc = " Maximum number of encoder threads."]
    pub minResourceCbSize: u32,
    #[doc = " Minimum resource command buffer size."]
    pub transientVbSize: u32,
    #[doc = " Maximum transient vertex buffer size."]
    pub transientIbSize: u32,
}
#[doc = " Renderer runtime limits."]
#[doc = ""]
pub type bgfx_caps_limits_t = bgfx_caps_limits_s;
#[doc = " Renderer capabilities."]
#[doc = ""]
#[repr(C)]
#[derive(Copy, Clone)]
pub struct bgfx_caps_s {
    pub rendererType: bgfx_renderer_type_t,
    #[doc = " Supported functionality."]
    #[doc = "   @attention See `BGFX_CAPS_*` flags at https://bkaradzic.github.io/bgfx/bgfx.html#available-caps"]
    pub supported: u64,
    pub vendorId: u16,
    #[doc = " Selected GPU vendor PCI id."]
    pub deviceId: u16,
    #[doc = " Selected GPU device id."]
    pub homogeneousDepth: bool,
    #[doc = " True when NDC depth is in [-1, 1] range, otherwise its [0, 1]."]
    pub originBottomLeft: bool,
    #[doc = " True when NDC origin is at bottom left."]
    pub numGPUs: u8,
    #[doc = " Number of enumerated GPUs."]
    pub gpu: [bgfx_caps_gpu_t; 4usize],
    #[doc = " Enumerated GPUs."]
    pub limits: bgfx_caps_limits_t,
    #[doc = " Supported texture format capabilities flags:"]
    #[doc = "   - `BGFX_CAPS_FORMAT_TEXTURE_NONE` - Texture format is not supported."]
    #[doc = "   - `BGFX_CAPS_FORMAT_TEXTURE_2D` - Texture format is supported."]
    #[doc = "   - `BGFX_CAPS_FORMAT_TEXTURE_2D_SRGB` - Texture as sRGB format is supported."]
    #[doc = "   - `BGFX_CAPS_FORMAT_TEXTURE_2D_EMULATED` - Texture format is emulated."]
    #[doc = "   - `BGFX_CAPS_FORMAT_TEXTURE_3D` - Texture format is supported."]
    #[doc = "   - `BGFX_CAPS_FORMAT_TEXTURE_3D_SRGB` - Texture as sRGB format is supported."]
    #[doc = "   - `BGFX_CAPS_FORMAT_TEXTURE_3D_EMULATED` - Texture format is emulated."]
    #[doc = "   - `BGFX_CAPS_FORMAT_TEXTURE_CUBE` - Texture format is supported."]
    #[doc = "   - `BGFX_CAPS_FORMAT_TEXTURE_CUBE_SRGB` - Texture as sRGB format is supported."]
    #[doc = "   - `BGFX_CAPS_FORMAT_TEXTURE_CUBE_EMULATED` - Texture format is emulated."]
    #[doc = "   - `BGFX_CAPS_FORMAT_TEXTURE_VERTEX` - Texture format can be used from vertex shader."]
    #[doc = "   - `BGFX_CAPS_FORMAT_TEXTURE_IMAGE_READ` - Texture format can be used as image"]
    #[doc = "     and read from."]
    #[doc = "   - `BGFX_CAPS_FORMAT_TEXTURE_IMAGE_WRITE` - Texture format can be used as image"]
    #[doc = "     and written to."]
    #[doc = "   - `BGFX_CAPS_FORMAT_TEXTURE_FRAMEBUFFER` - Texture format can be used as frame"]
    #[doc = "     buffer."]
    #[doc = "   - `BGFX_CAPS_FORMAT_TEXTURE_FRAMEBUFFER_MSAA` - Texture format can be used as MSAA"]
    #[doc = "     frame buffer."]
    #[doc = "   - `BGFX_CAPS_FORMAT_TEXTURE_MSAA` - Texture can be sampled as MSAA."]
    #[doc = "   - `BGFX_CAPS_FORMAT_TEXTURE_MIP_AUTOGEN` - Texture format supports auto-generated"]
    #[doc = "     mips."]
    pub formats: [u16; 85usize],
}
#[doc = " Renderer capabilities."]
#[doc = ""]
pub type bgfx_caps_t = bgfx_caps_s;
#[doc = " Internal data."]
#[doc = ""]
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct bgfx_internal_data_s {
    pub caps: *const bgfx_caps_t,
    #[doc = " Renderer capabilities."]
    pub context: *mut ::std::os::raw::c_void,
}
#[doc = " Internal data."]
#[doc = ""]
pub type bgfx_internal_data_t = bgfx_internal_data_s;
#[doc = " Platform data."]
#[doc = ""]
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct bgfx_platform_data_s {
    pub ndt: *mut ::std::os::raw::c_void,
    #[doc = " Native window handle. If `NULL` bgfx will create headless"]
    #[doc = " context/device if renderer API supports it."]
    pub nwh: *mut ::std::os::raw::c_void,
    pub context: *mut ::std::os::raw::c_void,
    #[doc = " GL back-buffer, or D3D render target view. If `NULL` bgfx will"]
    #[doc = " create back-buffer color surface."]
    pub backBuffer: *mut ::std::os::raw::c_void,
    #[doc = " Backbuffer depth/stencil. If `NULL` bgfx will create back-buffer"]
    #[doc = " depth/stencil surface."]
    pub backBufferDS: *mut ::std::os::raw::c_void,
}
#[doc = " Platform data."]
#[doc = ""]
pub type bgfx_platform_data_t = bgfx_platform_data_s;
#[doc = " Backbuffer resolution and reset parameters."]
#[doc = ""]
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct bgfx_resolution_s {
    pub format: bgfx_texture_format_t,
    #[doc = " Backbuffer format."]
    pub width: u32,
    #[doc = " Backbuffer width."]
    pub height: u32,
    #[doc = " Backbuffer height."]
    pub reset: u32,
    #[doc = " Reset parameters."]
    pub numBackBuffers: u8,
    #[doc = " Number of back buffers."]
    pub maxFrameLatency: u8,
}
#[doc = " Backbuffer resolution and reset parameters."]
#[doc = ""]
pub type bgfx_resolution_t = bgfx_resolution_s;
#[doc = " Configurable runtime limits parameters."]
#[doc = ""]
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct bgfx_init_limits_s {
    pub maxEncoders: u16,
    #[doc = " Maximum number of encoder threads."]
    pub minResourceCbSize: u32,
    #[doc = " Minimum resource command buffer size."]
    pub transientVbSize: u32,
    #[doc = " Maximum transient vertex buffer size."]
    pub transientIbSize: u32,
}
#[doc = " Configurable runtime limits parameters."]
#[doc = ""]
pub type bgfx_init_limits_t = bgfx_init_limits_s;
#[doc = " Initialization parameters used by `bgfx::init`."]
#[doc = ""]
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct bgfx_init_s {
    #[doc = " Select rendering backend. When set to RendererType::Count"]
    #[doc = " a default rendering backend will be selected appropriate to the platform."]
    #[doc = " See: `bgfx::RendererType`"]
    pub type_: bgfx_renderer_type_t,
    #[doc = " Vendor PCI id. If set to `BGFX_PCI_ID_NONE` it will select the first"]
    #[doc = " device."]
    #[doc = "   - `BGFX_PCI_ID_NONE` - Autoselect adapter."]
    #[doc = "   - `BGFX_PCI_ID_SOFTWARE_RASTERIZER` - Software rasterizer."]
    #[doc = "   - `BGFX_PCI_ID_AMD` - AMD adapter."]
    #[doc = "   - `BGFX_PCI_ID_INTEL` - Intel adapter."]
    #[doc = "   - `BGFX_PCI_ID_NVIDIA` - nVidia adapter."]
    pub vendorId: u16,
    #[doc = " Device id. If set to 0 it will select first device, or device with"]
    #[doc = " matching id."]
    pub deviceId: u16,
    pub capabilities: u64,
    #[doc = " Capabilities initialization mask (default: UINT64_MAX)."]
    pub debug: bool,
    #[doc = " Enable device for debuging."]
    pub profile: bool,
    #[doc = " Enable device for profiling."]
    pub platformData: bgfx_platform_data_t,
    #[doc = " Platform data."]
    pub resolution: bgfx_resolution_t,
    #[doc = " Backbuffer resolution and reset parameters. See: `bgfx::Resolution`."]
    pub limits: bgfx_init_limits_t,
    #[doc = " Provide application specific callback interface."]
    #[doc = " See: `bgfx::CallbackI`"]
    pub callback: *mut bgfx_callback_interface_t,
    #[doc = " Custom allocator. When a custom allocator is not"]
    #[doc = " specified, bgfx uses the CRT allocator. Bgfx assumes"]
    #[doc = " custom allocator is thread safe."]
    pub allocator: *mut bgfx_allocator_interface_t,
}
#[doc = " Initialization parameters used by `bgfx::init`."]
#[doc = ""]
pub type bgfx_init_t = bgfx_init_s;
#[doc = " Memory must be obtained by calling `bgfx::alloc`, `bgfx::copy`, or `bgfx::makeRef`."]
#[doc = " @attention It is illegal to create this structure on stack and pass it to any bgfx API."]
#[doc = ""]
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct bgfx_memory_s {
    pub data: *mut u8,
    #[doc = " Pointer to data."]
    pub size: u32,
}
#[doc = " Memory must be obtained by calling `bgfx::alloc`, `bgfx::copy`, or `bgfx::makeRef`."]
#[doc = " @attention It is illegal to create this structure on stack and pass it to any bgfx API."]
#[doc = ""]
pub type bgfx_memory_t = bgfx_memory_s;
#[doc = " Transient index buffer."]
#[doc = ""]
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct bgfx_transient_index_buffer_s {
    pub data: *mut u8,
    #[doc = " Pointer to data."]
    pub size: u32,
    #[doc = " Data size."]
    pub startIndex: u32,
    #[doc = " First index."]
    pub handle: bgfx_index_buffer_handle_t,
    #[doc = " Index buffer handle."]
    pub isIndex16: bool,
}
#[doc = " Transient index buffer."]
#[doc = ""]
pub type bgfx_transient_index_buffer_t = bgfx_transient_index_buffer_s;
#[doc = " Transient vertex buffer."]
#[doc = ""]
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct bgfx_transient_vertex_buffer_s {
    pub data: *mut u8,
    #[doc = " Pointer to data."]
    pub size: u32,
    #[doc = " Data size."]
    pub startVertex: u32,
    #[doc = " First vertex."]
    pub stride: u16,
    #[doc = " Vertex stride."]
    pub handle: bgfx_vertex_buffer_handle_t,
    #[doc = " Vertex buffer handle."]
    pub layoutHandle: bgfx_vertex_layout_handle_t,
}
#[doc = " Transient vertex buffer."]
#[doc = ""]
pub type bgfx_transient_vertex_buffer_t = bgfx_transient_vertex_buffer_s;
#[doc = " Instance data buffer info."]
#[doc = ""]
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct bgfx_instance_data_buffer_s {
    pub data: *mut u8,
    #[doc = " Pointer to data."]
    pub size: u32,
    #[doc = " Data size."]
    pub offset: u32,
    #[doc = " Offset in vertex buffer."]
    pub num: u32,
    #[doc = " Number of instances."]
    pub stride: u16,
    #[doc = " Vertex buffer stride."]
    pub handle: bgfx_vertex_buffer_handle_t,
}
#[doc = " Instance data buffer info."]
#[doc = ""]
pub type bgfx_instance_data_buffer_t = bgfx_instance_data_buffer_s;
#[doc = " Texture info."]
#[doc = ""]
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct bgfx_texture_info_s {
    pub format: bgfx_texture_format_t,
    #[doc = " Texture format."]
    pub storageSize: u32,
    #[doc = " Total amount of bytes required to store texture."]
    pub width: u16,
    #[doc = " Texture width."]
    pub height: u16,
    #[doc = " Texture height."]
    pub depth: u16,
    #[doc = " Texture depth."]
    pub numLayers: u16,
    #[doc = " Number of layers in texture array."]
    pub numMips: u8,
    #[doc = " Number of MIP maps."]
    pub bitsPerPixel: u8,
    #[doc = " Format bits per pixel."]
    pub cubeMap: bool,
}
#[doc = " Texture info."]
#[doc = ""]
pub type bgfx_texture_info_t = bgfx_texture_info_s;
#[doc = " Uniform info."]
#[doc = ""]
#[repr(C)]
#[derive(Copy, Clone)]
pub struct bgfx_uniform_info_s {
    pub name: [::std::os::raw::c_char; 256usize],
    #[doc = " Uniform name."]
    pub type_: bgfx_uniform_type_t,
    #[doc = " Uniform type."]
    pub num: u16,
}
#[doc = " Uniform info."]
#[doc = ""]
pub type bgfx_uniform_info_t = bgfx_uniform_info_s;
#[doc = " Frame buffer texture attachment info."]
#[doc = ""]
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct bgfx_attachment_s {
    pub access: bgfx_access_t,
    #[doc = " Attachment access. See `Access::Enum`."]
    pub handle: bgfx_texture_handle_t,
    #[doc = " Render target texture handle."]
    pub mip: u16,
    #[doc = " Mip level."]
    pub layer: u16,
    #[doc = " Cubemap side or depth layer/slice to use."]
    pub numLayers: u16,
    #[doc = " Number of texture layer/slice(s) in array to use."]
    pub resolve: u8,
}
#[doc = " Frame buffer texture attachment info."]
#[doc = ""]
pub type bgfx_attachment_t = bgfx_attachment_s;
#[doc = " Transform data."]
#[doc = ""]
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct bgfx_transform_s {
    pub data: *mut f32,
    #[doc = " Pointer to first 4x4 matrix."]
    pub num: u16,
}
#[doc = " Transform data."]
#[doc = ""]
pub type bgfx_transform_t = bgfx_transform_s;
#[doc = " View stats."]
#[doc = ""]
#[repr(C)]
#[derive(Copy, Clone)]
pub struct bgfx_view_stats_s {
    pub name: [::std::os::raw::c_char; 256usize],
    #[doc = " View name."]
    pub view: bgfx_view_id_t,
    #[doc = " View id."]
    pub cpuTimeBegin: i64,
    #[doc = " CPU (submit) begin time."]
    pub cpuTimeEnd: i64,
    #[doc = " CPU (submit) end time."]
    pub gpuTimeBegin: i64,
    #[doc = " GPU begin time."]
    pub gpuTimeEnd: i64,
}
#[doc = " View stats."]
#[doc = ""]
pub type bgfx_view_stats_t = bgfx_view_stats_s;
#[doc = " Encoder stats."]
#[doc = ""]
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct bgfx_encoder_stats_s {
    pub cpuTimeBegin: i64,
    #[doc = " Encoder thread CPU submit begin time."]
    pub cpuTimeEnd: i64,
}
#[doc = " Encoder stats."]
#[doc = ""]
pub type bgfx_encoder_stats_t = bgfx_encoder_stats_s;
#[doc = " Renderer statistics data."]
#[doc = " @remarks All time values are high-resolution timestamps, while"]
#[doc = " time frequencies define timestamps-per-second for that hardware."]
#[doc = ""]
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct bgfx_stats_s {
    pub cpuTimeFrame: i64,
    #[doc = " CPU time between two `bgfx::frame` calls."]
    pub cpuTimeBegin: i64,
    #[doc = " Render thread CPU submit begin time."]
    pub cpuTimeEnd: i64,
    #[doc = " Render thread CPU submit end time."]
    pub cpuTimerFreq: i64,
    #[doc = " CPU timer frequency. Timestamps-per-second"]
    pub gpuTimeBegin: i64,
    #[doc = " GPU frame begin time."]
    pub gpuTimeEnd: i64,
    #[doc = " GPU frame end time."]
    pub gpuTimerFreq: i64,
    #[doc = " GPU timer frequency."]
    pub waitRender: i64,
    #[doc = " Time spent waiting for render backend thread to finish issuing draw commands to underlying graphics API."]
    pub waitSubmit: i64,
    #[doc = " Time spent waiting for submit thread to advance to next frame."]
    pub numDraw: u32,
    #[doc = " Number of draw calls submitted."]
    pub numCompute: u32,
    #[doc = " Number of compute calls submitted."]
    pub numBlit: u32,
    #[doc = " Number of blit calls submitted."]
    pub maxGpuLatency: u32,
    #[doc = " GPU driver latency."]
    pub numDynamicIndexBuffers: u16,
    #[doc = " Number of used dynamic index buffers."]
    pub numDynamicVertexBuffers: u16,
    #[doc = " Number of used dynamic vertex buffers."]
    pub numFrameBuffers: u16,
    #[doc = " Number of used frame buffers."]
    pub numIndexBuffers: u16,
    #[doc = " Number of used index buffers."]
    pub numOcclusionQueries: u16,
    #[doc = " Number of used occlusion queries."]
    pub numPrograms: u16,
    #[doc = " Number of used programs."]
    pub numShaders: u16,
    #[doc = " Number of used shaders."]
    pub numTextures: u16,
    #[doc = " Number of used textures."]
    pub numUniforms: u16,
    #[doc = " Number of used uniforms."]
    pub numVertexBuffers: u16,
    #[doc = " Number of used vertex buffers."]
    pub numVertexLayouts: u16,
    #[doc = " Number of used vertex layouts."]
    pub textureMemoryUsed: i64,
    #[doc = " Estimate of texture memory used."]
    pub rtMemoryUsed: i64,
    #[doc = " Estimate of render target memory used."]
    pub transientVbUsed: i32,
    #[doc = " Amount of transient vertex buffer used."]
    pub transientIbUsed: i32,
    #[doc = " Amount of transient index buffer used."]
    pub numPrims: [u32; 5usize],
    #[doc = " Number of primitives rendered."]
    pub gpuMemoryMax: i64,
    #[doc = " Maximum available GPU memory for application."]
    pub gpuMemoryUsed: i64,
    #[doc = " Amount of GPU memory used by the application."]
    pub width: u16,
    #[doc = " Backbuffer width in pixels."]
    pub height: u16,
    #[doc = " Backbuffer height in pixels."]
    pub textWidth: u16,
    #[doc = " Debug text width in characters."]
    pub textHeight: u16,
    #[doc = " Debug text height in characters."]
    pub numViews: u16,
    #[doc = " Number of view stats."]
    pub viewStats: *mut bgfx_view_stats_t,
    #[doc = " Array of View stats."]
    pub numEncoders: u8,
    #[doc = " Number of encoders used during frame."]
    pub encoderStats: *mut bgfx_encoder_stats_t,
}
#[doc = " Renderer statistics data."]
#[doc = " @remarks All time values are high-resolution timestamps, while"]
#[doc = " time frequencies define timestamps-per-second for that hardware."]
#[doc = ""]
pub type bgfx_stats_t = bgfx_stats_s;
#[doc = " Vertex layout."]
#[doc = ""]
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct bgfx_vertex_layout_s {
    pub hash: u32,
    #[doc = " Hash."]
    pub stride: u16,
    #[doc = " Stride."]
    pub offset: [u16; 18usize],
    #[doc = " Attribute offsets."]
    pub attributes: [u16; 18usize],
}
#[doc = " Vertex layout."]
#[doc = ""]
pub type bgfx_vertex_layout_t = bgfx_vertex_layout_s;
#[doc = " Encoders are used for submitting draw calls from multiple threads. Only one encoder"]
#[doc = " per thread should be used. Use `bgfx::begin()` to obtain an encoder for a thread."]
#[doc = ""]
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct bgfx_encoder_s {
    _unused: [u8; 0],
}
pub type bgfx_encoder_t = bgfx_encoder_s;
extern "C" {
    #[doc = " Init attachment."]
    #[doc = ""]
    #[doc = " @param[in] _handle Render target texture handle."]
    #[doc = " @param[in] _access Access. See `Access::Enum`."]
    #[doc = " @param[in] _layer Cubemap side or depth layer/slice to use."]
    #[doc = " @param[in] _numLayers Number of texture layer/slice(s) in array to use."]
    #[doc = " @param[in] _mip Mip level."]
    #[doc = " @param[in] _resolve Resolve flags. See: `BGFX_RESOLVE_*`"]
    #[doc = ""]
    pub fn bgfx_attachment_init(
        _this: *mut bgfx_attachment_t,
        _handle: bgfx_texture_handle_t,
        _access: bgfx_access_t,
        _layer: u16,
        _numLayers: u16,
        _mip: u16,
        _resolve: u8,
    );
}
extern "C" {
    #[doc = " Start VertexLayout."]
    #[doc = ""]
    #[doc = " @param[in] _rendererType Renderer backend type. See: `bgfx::RendererType`"]
    #[doc = ""]
    #[doc = " @returns Returns itself."]
    #[doc = ""]
    pub fn bgfx_vertex_layout_begin(
        _this: *mut bgfx_vertex_layout_t,
        _rendererType: bgfx_renderer_type_t,
    ) -> *mut bgfx_vertex_layout_t;
}
extern "C" {
    #[doc = " Add attribute to VertexLayout."]
    #[doc = " @remarks Must be called between begin/end."]
    #[doc = ""]
    #[doc = " @param[in] _attrib Attribute semantics. See: `bgfx::Attrib`"]
    #[doc = " @param[in] _num Number of elements 1, 2, 3 or 4."]
    #[doc = " @param[in] _type Element type."]
    #[doc = " @param[in] _normalized When using fixed point AttribType (f.e. Uint8)"]
    #[doc = "  value will be normalized for vertex shader usage. When normalized"]
    #[doc = "  is set to true, AttribType::Uint8 value in range 0-255 will be"]
    #[doc = "  in range 0.0-1.0 in vertex shader."]
    #[doc = " @param[in] _asInt Packaging rule for vertexPack, vertexUnpack, and"]
    #[doc = "  vertexConvert for AttribType::Uint8 and AttribType::Int16."]
    #[doc = "  Unpacking code must be implemented inside vertex shader."]
    #[doc = ""]
    #[doc = " @returns Returns itself."]
    #[doc = ""]
    pub fn bgfx_vertex_layout_add(
        _this: *mut bgfx_vertex_layout_t,
        _attrib: bgfx_attrib_t,
        _num: u8,
        _type: bgfx_attrib_type_t,
        _normalized: bool,
        _asInt: bool,
    ) -> *mut bgfx_vertex_layout_t;
}
extern "C" {
    #[doc = " Decode attribute."]
    #[doc = ""]
    #[doc = " @param[in] _attrib Attribute semantics. See: `bgfx::Attrib`"]
    #[doc = " @param[out] _num Number of elements."]
    #[doc = " @param[out] _type Element type."]
    #[doc = " @param[out] _normalized Attribute is normalized."]
    #[doc = " @param[out] _asInt Attribute is packed as int."]
    #[doc = ""]
    pub fn bgfx_vertex_layout_decode(
        _this: *const bgfx_vertex_layout_t,
        _attrib: bgfx_attrib_t,
        _num: *mut u8,
        _type: *mut bgfx_attrib_type_t,
        _normalized: *mut bool,
        _asInt: *mut bool,
    );
}
extern "C" {
    #[doc = " Returns `true` if VertexLayout contains attribute."]
    #[doc = ""]
    #[doc = " @param[in] _attrib Attribute semantics. See: `bgfx::Attrib`"]
    #[doc = ""]
    #[doc = " @returns True if VertexLayout contains attribute."]
    #[doc = ""]
    pub fn bgfx_vertex_layout_has(
        _this: *const bgfx_vertex_layout_t,
        _attrib: bgfx_attrib_t,
    ) -> bool;
}
extern "C" {
    #[doc = " Skip `_num` bytes in vertex stream."]
    #[doc = ""]
    #[doc = " @param[in] _num Number of bytes to skip."]
    #[doc = ""]
    #[doc = " @returns Returns itself."]
    #[doc = ""]
    pub fn bgfx_vertex_layout_skip(
        _this: *mut bgfx_vertex_layout_t,
        _num: u8,
    ) -> *mut bgfx_vertex_layout_t;
}
extern "C" {
    #[doc = " End VertexLayout."]
    #[doc = ""]
    pub fn bgfx_vertex_layout_end(_this: *mut bgfx_vertex_layout_t);
}
extern "C" {
    #[doc = " Pack vertex attribute into vertex stream format."]
    #[doc = ""]
    #[doc = " @param[in] _input Value to be packed into vertex stream."]
    #[doc = " @param[in] _inputNormalized `true` if input value is already normalized."]
    #[doc = " @param[in] _attr Attribute to pack."]
    #[doc = " @param[in] _layout Vertex stream layout."]
    #[doc = " @param[in] _data Destination vertex stream where data will be packed."]
    #[doc = " @param[in] _index Vertex index that will be modified."]
    #[doc = ""]
    pub fn bgfx_vertex_pack(
        _input: *const f32,
        _inputNormalized: bool,
        _attr: bgfx_attrib_t,
        _layout: *const bgfx_vertex_layout_t,
        _data: *mut ::std::os::raw::c_void,
        _index: u32,
    );
}
extern "C" {
    #[doc = " Unpack vertex attribute from vertex stream format."]
    #[doc = ""]
    #[doc = " @param[out] _output Result of unpacking."]
    #[doc = " @param[in] _attr Attribute to unpack."]
    #[doc = " @param[in] _layout Vertex stream layout."]
    #[doc = " @param[in] _data Source vertex stream from where data will be unpacked."]
    #[doc = " @param[in] _index Vertex index that will be unpacked."]
    #[doc = ""]
    pub fn bgfx_vertex_unpack(
        _output: *mut f32,
        _attr: bgfx_attrib_t,
        _layout: *const bgfx_vertex_layout_t,
        _data: *const ::std::os::raw::c_void,
        _index: u32,
    );
}
extern "C" {
    #[doc = " Converts vertex stream data from one vertex stream format to another."]
    #[doc = ""]
    #[doc = " @param[in] _dstLayout Destination vertex stream layout."]
    #[doc = " @param[in] _dstData Destination vertex stream."]
    #[doc = " @param[in] _srcLayout Source vertex stream layout."]
    #[doc = " @param[in] _srcData Source vertex stream data."]
    #[doc = " @param[in] _num Number of vertices to convert from source to destination."]
    #[doc = ""]
    pub fn bgfx_vertex_convert(
        _dstLayout: *const bgfx_vertex_layout_t,
        _dstData: *mut ::std::os::raw::c_void,
        _srcLayout: *const bgfx_vertex_layout_t,
        _srcData: *const ::std::os::raw::c_void,
        _num: u32,
    );
}
extern "C" {
    #[doc = " Weld vertices."]
    #[doc = ""]
    #[doc = " @param[in] _output Welded vertices remapping table. The size of buffer"]
    #[doc = "  must be the same as number of vertices."]
    #[doc = " @param[in] _layout Vertex stream layout."]
    #[doc = " @param[in] _data Vertex stream."]
    #[doc = " @param[in] _num Number of vertices in vertex stream."]
    #[doc = " @param[in] _index32 Set to `true` if input indices are 32-bit."]
    #[doc = " @param[in] _epsilon Error tolerance for vertex position comparison."]
    #[doc = ""]
    #[doc = " @returns Number of unique vertices after vertex welding."]
    #[doc = ""]
    pub fn bgfx_weld_vertices(
        _output: *mut ::std::os::raw::c_void,
        _layout: *const bgfx_vertex_layout_t,
        _data: *const ::std::os::raw::c_void,
        _num: u32,
        _index32: bool,
        _epsilon: f32,
    ) -> u32;
}
extern "C" {
    #[doc = " Convert index buffer for use with different primitive topologies."]
    #[doc = ""]
    #[doc = " @param[in] _conversion Conversion type, see `TopologyConvert::Enum`."]
    #[doc = " @param[out] _dst Destination index buffer. If this argument is NULL"]
    #[doc = "  function will return number of indices after conversion."]
    #[doc = " @param[in] _dstSize Destination index buffer in bytes. It must be"]
    #[doc = "  large enough to contain output indices. If destination size is"]
    #[doc = "  insufficient index buffer will be truncated."]
    #[doc = " @param[in] _indices Source indices."]
    #[doc = " @param[in] _numIndices Number of input indices."]
    #[doc = " @param[in] _index32 Set to `true` if input indices are 32-bit."]
    #[doc = ""]
    #[doc = " @returns Number of output indices after conversion."]
    #[doc = ""]
    pub fn bgfx_topology_convert(
        _conversion: bgfx_topology_convert_t,
        _dst: *mut ::std::os::raw::c_void,
        _dstSize: u32,
        _indices: *const ::std::os::raw::c_void,
        _numIndices: u32,
        _index32: bool,
    ) -> u32;
}
extern "C" {
    #[doc = " Sort indices."]
    #[doc = ""]
    #[doc = " @param[in] _sort Sort order, see `TopologySort::Enum`."]
    #[doc = " @param[out] _dst Destination index buffer."]
    #[doc = " @param[in] _dstSize Destination index buffer in bytes. It must be"]
    #[doc = "  large enough to contain output indices. If destination size is"]
    #[doc = "  insufficient index buffer will be truncated."]
    #[doc = " @param[in] _dir Direction (vector must be normalized)."]
    #[doc = " @param[in] _pos Position."]
    #[doc = " @param[in] _vertices Pointer to first vertex represented as"]
    #[doc = "  float x, y, z. Must contain at least number of vertices"]
    #[doc = "  referencende by index buffer."]
    #[doc = " @param[in] _stride Vertex stride."]
    #[doc = " @param[in] _indices Source indices."]
    #[doc = " @param[in] _numIndices Number of input indices."]
    #[doc = " @param[in] _index32 Set to `true` if input indices are 32-bit."]
    #[doc = ""]
    pub fn bgfx_topology_sort_tri_list(
        _sort: bgfx_topology_sort_t,
        _dst: *mut ::std::os::raw::c_void,
        _dstSize: u32,
        _dir: *const f32,
        _pos: *const f32,
        _vertices: *const ::std::os::raw::c_void,
        _stride: u32,
        _indices: *const ::std::os::raw::c_void,
        _numIndices: u32,
        _index32: bool,
    );
}
extern "C" {
    #[doc = " Returns supported backend API renderers."]
    #[doc = ""]
    #[doc = " @param[in] _max Maximum number of elements in _enum array."]
    #[doc = " @param[inout] _enum Array where supported renderers will be written."]
    #[doc = ""]
    #[doc = " @returns Number of supported renderers."]
    #[doc = ""]
    pub fn bgfx_get_supported_renderers(_max: u8, _enum: *mut bgfx_renderer_type_t) -> u8;
}
extern "C" {
    #[doc = " Returns name of renderer."]
    #[doc = ""]
    #[doc = " @param[in] _type Renderer backend type. See: `bgfx::RendererType`"]
    #[doc = ""]
    #[doc = " @returns Name of renderer."]
    #[doc = ""]
    pub fn bgfx_get_renderer_name(_type: bgfx_renderer_type_t) -> *const ::std::os::raw::c_char;
}
extern "C" {
    pub fn bgfx_init_ctor(_init: *mut bgfx_init_t);
}
extern "C" {
    #[doc = " Initialize bgfx library."]
    #[doc = ""]
    #[doc = " @param[in] _init Initialization parameters. See: `bgfx::Init` for more info."]
    #[doc = ""]
    #[doc = " @returns `true` if initialization was successful."]
    #[doc = ""]
    pub fn bgfx_init(_init: *const bgfx_init_t) -> bool;
}
extern "C" {
    #[doc = " Shutdown bgfx library."]
    #[doc = ""]
    pub fn bgfx_shutdown();
}
extern "C" {
    #[doc = " Reset graphic settings and back-buffer size."]
    #[doc = " @attention This call doesn't actually change window size, it just"]
    #[doc = "   resizes back-buffer. Windowing code has to change window size."]
    #[doc = ""]
    #[doc = " @param[in] _width Back-buffer width."]
    #[doc = " @param[in] _height Back-buffer height."]
    #[doc = " @param[in] _flags See: `BGFX_RESET_*` for more info."]
    #[doc = "    - `BGFX_RESET_NONE` - No reset flags."]
    #[doc = "    - `BGFX_RESET_FULLSCREEN` - Not supported yet."]
    #[doc = "    - `BGFX_RESET_MSAA_X[2/4/8/16]` - Enable 2, 4, 8 or 16 x MSAA."]
    #[doc = "    - `BGFX_RESET_VSYNC` - Enable V-Sync."]
    #[doc = "    - `BGFX_RESET_MAXANISOTROPY` - Turn on/off max anisotropy."]
    #[doc = "    - `BGFX_RESET_CAPTURE` - Begin screen capture."]
    #[doc = "    - `BGFX_RESET_FLUSH_AFTER_RENDER` - Flush rendering after submitting to GPU."]
    #[doc = "    - `BGFX_RESET_FLIP_AFTER_RENDER` - This flag  specifies where flip"]
    #[doc = "      occurs. Default behaviour is that flip occurs before rendering new"]
    #[doc = "      frame. This flag only has effect when `BGFX_CONFIG_MULTITHREADED=0`."]
    #[doc = "    - `BGFX_RESET_SRGB_BACKBUFFER` - Enable sRGB backbuffer."]
    #[doc = " @param[in] _format Texture format. See: `TextureFormat::Enum`."]
    #[doc = ""]
    pub fn bgfx_reset(_width: u32, _height: u32, _flags: u32, _format: bgfx_texture_format_t);
}
extern "C" {
    #[doc = " Advance to next frame. When using multithreaded renderer, this call"]
    #[doc = " just swaps internal buffers, kicks render thread, and returns. In"]
    #[doc = " singlethreaded renderer this call does frame rendering."]
    #[doc = ""]
    #[doc = " @param[in] _capture Capture frame with graphics debugger."]
    #[doc = ""]
    #[doc = " @returns Current frame number. This might be used in conjunction with"]
    #[doc = "  double/multi buffering data outside the library and passing it to"]
    #[doc = "  library via `bgfx::makeRef` calls."]
    #[doc = ""]
    pub fn bgfx_frame(_capture: bool) -> u32;
}
extern "C" {
    #[doc = " Returns current renderer backend API type."]
    #[doc = " @remarks"]
    #[doc = "   Library must be initialized."]
    #[doc = ""]
    pub fn bgfx_get_renderer_type() -> bgfx_renderer_type_t;
}
extern "C" {
    #[doc = " Returns renderer capabilities."]
    #[doc = " @remarks"]
    #[doc = "   Library must be initialized."]
    #[doc = ""]
    pub fn bgfx_get_caps() -> *const bgfx_caps_t;
}
extern "C" {
    #[doc = " Returns performance counters."]
    #[doc = " @attention Pointer returned is valid until `bgfx::frame` is called."]
    #[doc = ""]
    pub fn bgfx_get_stats() -> *const bgfx_stats_t;
}
extern "C" {
    #[doc = " Allocate buffer to pass to bgfx calls. Data will be freed inside bgfx."]
    #[doc = ""]
    #[doc = " @param[in] _size Size to allocate."]
    #[doc = ""]
    #[doc = " @returns Allocated memory."]
    #[doc = ""]
    pub fn bgfx_alloc(_size: u32) -> *const bgfx_memory_t;
}
extern "C" {
    #[doc = " Allocate buffer and copy data into it. Data will be freed inside bgfx."]
    #[doc = ""]
    #[doc = " @param[in] _data Pointer to data to be copied."]
    #[doc = " @param[in] _size Size of data to be copied."]
    #[doc = ""]
    #[doc = " @returns Allocated memory."]
    #[doc = ""]
    pub fn bgfx_copy(_data: *const ::std::os::raw::c_void, _size: u32) -> *const bgfx_memory_t;
}
extern "C" {
    #[doc = " Make reference to data to pass to bgfx. Unlike `bgfx::alloc`, this call"]
    #[doc = " doesn't allocate memory for data. It just copies the _data pointer. You"]
    #[doc = " can pass `ReleaseFn` function pointer to release this memory after it's"]
    #[doc = " consumed, otherwise you must make sure _data is available for at least 2"]
    #[doc = " `bgfx::frame` calls. `ReleaseFn` function must be able to be called"]
    #[doc = " from any thread."]
    #[doc = " @attention Data passed must be available for at least 2 `bgfx::frame` calls."]
    #[doc = ""]
    #[doc = " @param[in] _data Pointer to data."]
    #[doc = " @param[in] _size Size of data."]
    #[doc = ""]
    #[doc = " @returns Referenced memory."]
    #[doc = ""]
    pub fn bgfx_make_ref(_data: *const ::std::os::raw::c_void, _size: u32) -> *const bgfx_memory_t;
}
extern "C" {
    #[doc = " Make reference to data to pass to bgfx. Unlike `bgfx::alloc`, this call"]
    #[doc = " doesn't allocate memory for data. It just copies the _data pointer. You"]
    #[doc = " can pass `ReleaseFn` function pointer to release this memory after it's"]
    #[doc = " consumed, otherwise you must make sure _data is available for at least 2"]
    #[doc = " `bgfx::frame` calls. `ReleaseFn` function must be able to be called"]
    #[doc = " from any thread."]
    #[doc = " @attention Data passed must be available for at least 2 `bgfx::frame` calls."]
    #[doc = ""]
    #[doc = " @param[in] _data Pointer to data."]
    #[doc = " @param[in] _size Size of data."]
    #[doc = " @param[in] _releaseFn Callback function to release memory after use."]
    #[doc = " @param[in] _userData User data to be passed to callback function."]
    #[doc = ""]
    #[doc = " @returns Referenced memory."]
    #[doc = ""]
    pub fn bgfx_make_ref_release(
        _data: *const ::std::os::raw::c_void,
        _size: u32,
        _releaseFn: bgfx_release_fn_t,
        _userData: *mut ::std::os::raw::c_void,
    ) -> *const bgfx_memory_t;
}
extern "C" {
    #[doc = " Set debug flags."]
    #[doc = ""]
    #[doc = " @param[in] _debug Available flags:"]
    #[doc = "    - `BGFX_DEBUG_IFH` - Infinitely fast hardware. When this flag is set"]
    #[doc = "      all rendering calls will be skipped. This is useful when profiling"]
    #[doc = "      to quickly assess potential bottlenecks between CPU and GPU."]
    #[doc = "    - `BGFX_DEBUG_PROFILER` - Enable profiler."]
    #[doc = "    - `BGFX_DEBUG_STATS` - Display internal statistics."]
    #[doc = "    - `BGFX_DEBUG_TEXT` - Display debug text."]
    #[doc = "    - `BGFX_DEBUG_WIREFRAME` - Wireframe rendering. All rendering"]
    #[doc = "      primitives will be rendered as lines."]
    #[doc = ""]
    pub fn bgfx_set_debug(_debug: u32);
}
extern "C" {
    #[doc = " Clear internal debug text buffer."]
    #[doc = ""]
    #[doc = " @param[in] _attr Background color."]
    #[doc = " @param[in] _small Default 8x16 or 8x8 font."]
    #[doc = ""]
    pub fn bgfx_dbg_text_clear(_attr: u8, _small: bool);
}
extern "C" {
    #[doc = " Print formatted data to internal debug text character-buffer (VGA-compatible text mode)."]
    #[doc = ""]
    #[doc = " @param[in] _x Position x from the left corner of the window."]
    #[doc = " @param[in] _y Position y from the top corner of the window."]
    #[doc = " @param[in] _attr Color palette. Where top 4-bits represent index of background, and bottom"]
    #[doc = "  4-bits represent foreground color from standard VGA text palette (ANSI escape codes)."]
    #[doc = " @param[in] _format `printf` style format."]
    #[doc = " @param[in]"]
    #[doc = ""]
    pub fn bgfx_dbg_text_printf(
        _x: u16,
        _y: u16,
        _attr: u8,
        _format: *const ::std::os::raw::c_char,
        ...
    );
}
extern "C" {
    #[doc = " Print formatted data from variable argument list to internal debug text character-buffer (VGA-compatible text mode)."]
    #[doc = ""]
    #[doc = " @param[in] _x Position x from the left corner of the window."]
    #[doc = " @param[in] _y Position y from the top corner of the window."]
    #[doc = " @param[in] _attr Color palette. Where top 4-bits represent index of background, and bottom"]
    #[doc = "  4-bits represent foreground color from standard VGA text palette (ANSI escape codes)."]
    #[doc = " @param[in] _format `printf` style format."]
    #[doc = " @param[in] _argList Variable arguments list for format string."]
    #[doc = ""]
    pub fn bgfx_dbg_text_vprintf(
        _x: u16,
        _y: u16,
        _attr: u8,
        _format: *const ::std::os::raw::c_char,
        _argList: *mut __va_list_tag,
    );
}
extern "C" {
    #[doc = " Draw image into internal debug text buffer."]
    #[doc = ""]
    #[doc = " @param[in] _x Position x from the left corner of the window."]
    #[doc = " @param[in] _y Position y from the top corner of the window."]
    #[doc = " @param[in] _width Image width."]
    #[doc = " @param[in] _height Image height."]
    #[doc = " @param[in] _data Raw image data (character/attribute raw encoding)."]
    #[doc = " @param[in] _pitch Image pitch in bytes."]
    #[doc = ""]
    pub fn bgfx_dbg_text_image(
        _x: u16,
        _y: u16,
        _width: u16,
        _height: u16,
        _data: *const ::std::os::raw::c_void,
        _pitch: u16,
    );
}
extern "C" {
    #[doc = " Create static index buffer."]
    #[doc = ""]
    #[doc = " @param[in] _mem Index buffer data."]
    #[doc = " @param[in] _flags Buffer creation flags."]
    #[doc = "    - `BGFX_BUFFER_NONE` - No flags."]
    #[doc = "    - `BGFX_BUFFER_COMPUTE_READ` - Buffer will be read from by compute shader."]
    #[doc = "    - `BGFX_BUFFER_COMPUTE_WRITE` - Buffer will be written into by compute shader. When buffer"]
    #[doc = "        is created with `BGFX_BUFFER_COMPUTE_WRITE` flag it cannot be updated from CPU."]
    #[doc = "    - `BGFX_BUFFER_COMPUTE_READ_WRITE` - Buffer will be used for read/write by compute shader."]
    #[doc = "    - `BGFX_BUFFER_ALLOW_RESIZE` - Buffer will resize on buffer update if a different amount of"]
    #[doc = "        data is passed. If this flag is not specified, and more data is passed on update, the buffer"]
    #[doc = "        will be trimmed to fit the existing buffer size. This flag has effect only on dynamic"]
    #[doc = "        buffers."]
    #[doc = "    - `BGFX_BUFFER_INDEX32` - Buffer is using 32-bit indices. This flag has effect only on"]
    #[doc = "        index buffers."]
    #[doc = ""]
    pub fn bgfx_create_index_buffer(
        _mem: *const bgfx_memory_t,
        _flags: u16,
    ) -> bgfx_index_buffer_handle_t;
}
extern "C" {
    #[doc = " Set static index buffer debug name."]
    #[doc = ""]
    #[doc = " @param[in] _handle Static index buffer handle."]
    #[doc = " @param[in] _name Static index buffer name."]
    #[doc = " @param[in] _len Static index buffer name length (if length is INT32_MAX, it's expected"]
    #[doc = "  that _name is zero terminated string."]
    #[doc = ""]
    pub fn bgfx_set_index_buffer_name(
        _handle: bgfx_index_buffer_handle_t,
        _name: *const ::std::os::raw::c_char,
        _len: i32,
    );
}
extern "C" {
    #[doc = " Destroy static index buffer."]
    #[doc = ""]
    #[doc = " @param[in] _handle Static index buffer handle."]
    #[doc = ""]
    pub fn bgfx_destroy_index_buffer(_handle: bgfx_index_buffer_handle_t);
}
extern "C" {
    #[doc = " Create vertex layout."]
    #[doc = ""]
    #[doc = " @param[in] _layout Vertex layout."]
    #[doc = ""]
    pub fn bgfx_create_vertex_layout(
        _layout: *const bgfx_vertex_layout_t,
    ) -> bgfx_vertex_layout_handle_t;
}
extern "C" {
    #[doc = " Destroy vertex layout."]
    #[doc = ""]
    #[doc = " @param[in] _layoutHandle Vertex layout handle."]
    #[doc = ""]
    pub fn bgfx_destroy_vertex_layout(_layoutHandle: bgfx_vertex_layout_handle_t);
}
extern "C" {
    #[doc = " Create static vertex buffer."]
    #[doc = ""]
    #[doc = " @param[in] _mem Vertex buffer data."]
    #[doc = " @param[in] _layout Vertex layout."]
    #[doc = " @param[in] _flags Buffer creation flags."]
    #[doc = "   - `BGFX_BUFFER_NONE` - No flags."]
    #[doc = "   - `BGFX_BUFFER_COMPUTE_READ` - Buffer will be read from by compute shader."]
    #[doc = "   - `BGFX_BUFFER_COMPUTE_WRITE` - Buffer will be written into by compute shader. When buffer"]
    #[doc = "       is created with `BGFX_BUFFER_COMPUTE_WRITE` flag it cannot be updated from CPU."]
    #[doc = "   - `BGFX_BUFFER_COMPUTE_READ_WRITE` - Buffer will be used for read/write by compute shader."]
    #[doc = "   - `BGFX_BUFFER_ALLOW_RESIZE` - Buffer will resize on buffer update if a different amount of"]
    #[doc = "       data is passed. If this flag is not specified, and more data is passed on update, the buffer"]
    #[doc = "       will be trimmed to fit the existing buffer size. This flag has effect only on dynamic buffers."]
    #[doc = "   - `BGFX_BUFFER_INDEX32` - Buffer is using 32-bit indices. This flag has effect only on index buffers."]
    #[doc = ""]
    #[doc = " @returns Static vertex buffer handle."]
    #[doc = ""]
    pub fn bgfx_create_vertex_buffer(
        _mem: *const bgfx_memory_t,
        _layout: *const bgfx_vertex_layout_t,
        _flags: u16,
    ) -> bgfx_vertex_buffer_handle_t;
}
extern "C" {
    #[doc = " Set static vertex buffer debug name."]
    #[doc = ""]
    #[doc = " @param[in] _handle Static vertex buffer handle."]
    #[doc = " @param[in] _name Static vertex buffer name."]
    #[doc = " @param[in] _len Static vertex buffer name length (if length is INT32_MAX, it's expected"]
    #[doc = "  that _name is zero terminated string."]
    #[doc = ""]
    pub fn bgfx_set_vertex_buffer_name(
        _handle: bgfx_vertex_buffer_handle_t,
        _name: *const ::std::os::raw::c_char,
        _len: i32,
    );
}
extern "C" {
    #[doc = " Destroy static vertex buffer."]
    #[doc = ""]
    #[doc = " @param[in] _handle Static vertex buffer handle."]
    #[doc = ""]
    pub fn bgfx_destroy_vertex_buffer(_handle: bgfx_vertex_buffer_handle_t);
}
extern "C" {
    #[doc = " Create empty dynamic index buffer."]
    #[doc = ""]
    #[doc = " @param[in] _num Number of indices."]
    #[doc = " @param[in] _flags Buffer creation flags."]
    #[doc = "    - `BGFX_BUFFER_NONE` - No flags."]
    #[doc = "    - `BGFX_BUFFER_COMPUTE_READ` - Buffer will be read from by compute shader."]
    #[doc = "    - `BGFX_BUFFER_COMPUTE_WRITE` - Buffer will be written into by compute shader. When buffer"]
    #[doc = "        is created with `BGFX_BUFFER_COMPUTE_WRITE` flag it cannot be updated from CPU."]
    #[doc = "    - `BGFX_BUFFER_COMPUTE_READ_WRITE` - Buffer will be used for read/write by compute shader."]
    #[doc = "    - `BGFX_BUFFER_ALLOW_RESIZE` - Buffer will resize on buffer update if a different amount of"]
    #[doc = "        data is passed. If this flag is not specified, and more data is passed on update, the buffer"]
    #[doc = "        will be trimmed to fit the existing buffer size. This flag has effect only on dynamic"]
    #[doc = "        buffers."]
    #[doc = "    - `BGFX_BUFFER_INDEX32` - Buffer is using 32-bit indices. This flag has effect only on"]
    #[doc = "        index buffers."]
    #[doc = ""]
    #[doc = " @returns Dynamic index buffer handle."]
    #[doc = ""]
    pub fn bgfx_create_dynamic_index_buffer(
        _num: u32,
        _flags: u16,
    ) -> bgfx_dynamic_index_buffer_handle_t;
}
extern "C" {
    #[doc = " Create dynamic index buffer and initialized it."]
    #[doc = ""]
    #[doc = " @param[in] _mem Index buffer data."]
    #[doc = " @param[in] _flags Buffer creation flags."]
    #[doc = "    - `BGFX_BUFFER_NONE` - No flags."]
    #[doc = "    - `BGFX_BUFFER_COMPUTE_READ` - Buffer will be read from by compute shader."]
    #[doc = "    - `BGFX_BUFFER_COMPUTE_WRITE` - Buffer will be written into by compute shader. When buffer"]
    #[doc = "        is created with `BGFX_BUFFER_COMPUTE_WRITE` flag it cannot be updated from CPU."]
    #[doc = "    - `BGFX_BUFFER_COMPUTE_READ_WRITE` - Buffer will be used for read/write by compute shader."]
    #[doc = "    - `BGFX_BUFFER_ALLOW_RESIZE` - Buffer will resize on buffer update if a different amount of"]
    #[doc = "        data is passed. If this flag is not specified, and more data is passed on update, the buffer"]
    #[doc = "        will be trimmed to fit the existing buffer size. This flag has effect only on dynamic"]
    #[doc = "        buffers."]
    #[doc = "    - `BGFX_BUFFER_INDEX32` - Buffer is using 32-bit indices. This flag has effect only on"]
    #[doc = "        index buffers."]
    #[doc = ""]
    #[doc = " @returns Dynamic index buffer handle."]
    #[doc = ""]
    pub fn bgfx_create_dynamic_index_buffer_mem(
        _mem: *const bgfx_memory_t,
        _flags: u16,
    ) -> bgfx_dynamic_index_buffer_handle_t;
}
extern "C" {
    #[doc = " Update dynamic index buffer."]
    #[doc = ""]
    #[doc = " @param[in] _handle Dynamic index buffer handle."]
    #[doc = " @param[in] _startIndex Start index."]
    #[doc = " @param[in] _mem Index buffer data."]
    #[doc = ""]
    pub fn bgfx_update_dynamic_index_buffer(
        _handle: bgfx_dynamic_index_buffer_handle_t,
        _startIndex: u32,
        _mem: *const bgfx_memory_t,
    );
}
extern "C" {
    #[doc = " Destroy dynamic index buffer."]
    #[doc = ""]
    #[doc = " @param[in] _handle Dynamic index buffer handle."]
    #[doc = ""]
    pub fn bgfx_destroy_dynamic_index_buffer(_handle: bgfx_dynamic_index_buffer_handle_t);
}
extern "C" {
    #[doc = " Create empty dynamic vertex buffer."]
    #[doc = ""]
    #[doc = " @param[in] _num Number of vertices."]
    #[doc = " @param[in] _layout Vertex layout."]
    #[doc = " @param[in] _flags Buffer creation flags."]
    #[doc = "    - `BGFX_BUFFER_NONE` - No flags."]
    #[doc = "    - `BGFX_BUFFER_COMPUTE_READ` - Buffer will be read from by compute shader."]
    #[doc = "    - `BGFX_BUFFER_COMPUTE_WRITE` - Buffer will be written into by compute shader. When buffer"]
    #[doc = "        is created with `BGFX_BUFFER_COMPUTE_WRITE` flag it cannot be updated from CPU."]
    #[doc = "    - `BGFX_BUFFER_COMPUTE_READ_WRITE` - Buffer will be used for read/write by compute shader."]
    #[doc = "    - `BGFX_BUFFER_ALLOW_RESIZE` - Buffer will resize on buffer update if a different amount of"]
    #[doc = "        data is passed. If this flag is not specified, and more data is passed on update, the buffer"]
    #[doc = "        will be trimmed to fit the existing buffer size. This flag has effect only on dynamic"]
    #[doc = "        buffers."]
    #[doc = "    - `BGFX_BUFFER_INDEX32` - Buffer is using 32-bit indices. This flag has effect only on"]
    #[doc = "        index buffers."]
    #[doc = ""]
    #[doc = " @returns Dynamic vertex buffer handle."]
    #[doc = ""]
    pub fn bgfx_create_dynamic_vertex_buffer(
        _num: u32,
        _layout: *const bgfx_vertex_layout_t,
        _flags: u16,
    ) -> bgfx_dynamic_vertex_buffer_handle_t;
}
extern "C" {
    #[doc = " Create dynamic vertex buffer and initialize it."]
    #[doc = ""]
    #[doc = " @param[in] _mem Vertex buffer data."]
    #[doc = " @param[in] _layout Vertex layout."]
    #[doc = " @param[in] _flags Buffer creation flags."]
    #[doc = "    - `BGFX_BUFFER_NONE` - No flags."]
    #[doc = "    - `BGFX_BUFFER_COMPUTE_READ` - Buffer will be read from by compute shader."]
    #[doc = "    - `BGFX_BUFFER_COMPUTE_WRITE` - Buffer will be written into by compute shader. When buffer"]
    #[doc = "        is created with `BGFX_BUFFER_COMPUTE_WRITE` flag it cannot be updated from CPU."]
    #[doc = "    - `BGFX_BUFFER_COMPUTE_READ_WRITE` - Buffer will be used for read/write by compute shader."]
    #[doc = "    - `BGFX_BUFFER_ALLOW_RESIZE` - Buffer will resize on buffer update if a different amount of"]
    #[doc = "        data is passed. If this flag is not specified, and more data is passed on update, the buffer"]
    #[doc = "        will be trimmed to fit the existing buffer size. This flag has effect only on dynamic"]
    #[doc = "        buffers."]
    #[doc = "    - `BGFX_BUFFER_INDEX32` - Buffer is using 32-bit indices. This flag has effect only on"]
    #[doc = "        index buffers."]
    #[doc = ""]
    #[doc = " @returns Dynamic vertex buffer handle."]
    #[doc = ""]
    pub fn bgfx_create_dynamic_vertex_buffer_mem(
        _mem: *const bgfx_memory_t,
        _layout: *const bgfx_vertex_layout_t,
        _flags: u16,
    ) -> bgfx_dynamic_vertex_buffer_handle_t;
}
extern "C" {
    #[doc = " Update dynamic vertex buffer."]
    #[doc = ""]
    #[doc = " @param[in] _handle Dynamic vertex buffer handle."]
    #[doc = " @param[in] _startVertex Start vertex."]
    #[doc = " @param[in] _mem Vertex buffer data."]
    #[doc = ""]
    pub fn bgfx_update_dynamic_vertex_buffer(
        _handle: bgfx_dynamic_vertex_buffer_handle_t,
        _startVertex: u32,
        _mem: *const bgfx_memory_t,
    );
}
extern "C" {
    #[doc = " Destroy dynamic vertex buffer."]
    #[doc = ""]
    #[doc = " @param[in] _handle Dynamic vertex buffer handle."]
    #[doc = ""]
    pub fn bgfx_destroy_dynamic_vertex_buffer(_handle: bgfx_dynamic_vertex_buffer_handle_t);
}
extern "C" {
    #[doc = " Returns number of requested or maximum available indices."]
    #[doc = ""]
    #[doc = " @param[in] _num Number of required indices."]
    #[doc = " @param[in] _index32 Set to `true` if input indices will be 32-bit."]
    #[doc = ""]
    #[doc = " @returns Number of requested or maximum available indices."]
    #[doc = ""]
    pub fn bgfx_get_avail_transient_index_buffer(_num: u32, _index32: bool) -> u32;
}
extern "C" {
    #[doc = " Returns number of requested or maximum available vertices."]
    #[doc = ""]
    #[doc = " @param[in] _num Number of required vertices."]
    #[doc = " @param[in] _layout Vertex layout."]
    #[doc = ""]
    #[doc = " @returns Number of requested or maximum available vertices."]
    #[doc = ""]
    pub fn bgfx_get_avail_transient_vertex_buffer(
        _num: u32,
        _layout: *const bgfx_vertex_layout_t,
    ) -> u32;
}
extern "C" {
    #[doc = " Returns number of requested or maximum available instance buffer slots."]
    #[doc = ""]
    #[doc = " @param[in] _num Number of required instances."]
    #[doc = " @param[in] _stride Stride per instance."]
    #[doc = ""]
    #[doc = " @returns Number of requested or maximum available instance buffer slots."]
    #[doc = ""]
    pub fn bgfx_get_avail_instance_data_buffer(_num: u32, _stride: u16) -> u32;
}
extern "C" {
    #[doc = " Allocate transient index buffer."]
    #[doc = ""]
    #[doc = " @param[out] _tib TransientIndexBuffer structure is filled and is valid"]
    #[doc = "  for the duration of frame, and it can be reused for multiple draw"]
    #[doc = "  calls."]
    #[doc = " @param[in] _num Number of indices to allocate."]
    #[doc = " @param[in] _index32 Set to `true` if input indices will be 32-bit."]
    #[doc = ""]
    pub fn bgfx_alloc_transient_index_buffer(
        _tib: *mut bgfx_transient_index_buffer_t,
        _num: u32,
        _index32: bool,
    );
}
extern "C" {
    #[doc = " Allocate transient vertex buffer."]
    #[doc = ""]
    #[doc = " @param[out] _tvb TransientVertexBuffer structure is filled and is valid"]
    #[doc = "  for the duration of frame, and it can be reused for multiple draw"]
    #[doc = "  calls."]
    #[doc = " @param[in] _num Number of vertices to allocate."]
    #[doc = " @param[in] _layout Vertex layout."]
    #[doc = ""]
    pub fn bgfx_alloc_transient_vertex_buffer(
        _tvb: *mut bgfx_transient_vertex_buffer_t,
        _num: u32,
        _layout: *const bgfx_vertex_layout_t,
    );
}
extern "C" {
    #[doc = " Check for required space and allocate transient vertex and index"]
    #[doc = " buffers. If both space requirements are satisfied function returns"]
    #[doc = " true."]
    #[doc = ""]
    #[doc = " @param[out] _tvb TransientVertexBuffer structure is filled and is valid"]
    #[doc = "  for the duration of frame, and it can be reused for multiple draw"]
    #[doc = "  calls."]
    #[doc = " @param[in] _layout Vertex layout."]
    #[doc = " @param[in] _numVertices Number of vertices to allocate."]
    #[doc = " @param[out] _tib TransientIndexBuffer structure is filled and is valid"]
    #[doc = "  for the duration of frame, and it can be reused for multiple draw"]
    #[doc = "  calls."]
    #[doc = " @param[in] _numIndices Number of indices to allocate."]
    #[doc = " @param[in] _index32 Set to `true` if input indices will be 32-bit."]
    #[doc = ""]
    pub fn bgfx_alloc_transient_buffers(
        _tvb: *mut bgfx_transient_vertex_buffer_t,
        _layout: *const bgfx_vertex_layout_t,
        _numVertices: u32,
        _tib: *mut bgfx_transient_index_buffer_t,
        _numIndices: u32,
        _index32: bool,
    ) -> bool;
}
extern "C" {
    #[doc = " Allocate instance data buffer."]
    #[doc = ""]
    #[doc = " @param[out] _idb InstanceDataBuffer structure is filled and is valid"]
    #[doc = "  for duration of frame, and it can be reused for multiple draw"]
    #[doc = "  calls."]
    #[doc = " @param[in] _num Number of instances."]
    #[doc = " @param[in] _stride Instance stride. Must be multiple of 16."]
    #[doc = ""]
    pub fn bgfx_alloc_instance_data_buffer(
        _idb: *mut bgfx_instance_data_buffer_t,
        _num: u32,
        _stride: u16,
    );
}
extern "C" {
    #[doc = " Create draw indirect buffer."]
    #[doc = ""]
    #[doc = " @param[in] _num Number of indirect calls."]
    #[doc = ""]
    #[doc = " @returns Indirect buffer handle."]
    #[doc = ""]
    pub fn bgfx_create_indirect_buffer(_num: u32) -> bgfx_indirect_buffer_handle_t;
}
extern "C" {
    #[doc = " Destroy draw indirect buffer."]
    #[doc = ""]
    #[doc = " @param[in] _handle Indirect buffer handle."]
    #[doc = ""]
    pub fn bgfx_destroy_indirect_buffer(_handle: bgfx_indirect_buffer_handle_t);
}
extern "C" {
    #[doc = " Create shader from memory buffer."]
    #[doc = ""]
    #[doc = " @param[in] _mem Shader binary."]
    #[doc = ""]
    #[doc = " @returns Shader handle."]
    #[doc = ""]
    pub fn bgfx_create_shader(_mem: *const bgfx_memory_t) -> bgfx_shader_handle_t;
}
extern "C" {
    #[doc = " Returns the number of uniforms and uniform handles used inside a shader."]
    #[doc = " @remarks"]
    #[doc = "   Only non-predefined uniforms are returned."]
    #[doc = ""]
    #[doc = " @param[in] _handle Shader handle."]
    #[doc = " @param[out] _uniforms UniformHandle array where data will be stored."]
    #[doc = " @param[in] _max Maximum capacity of array."]
    #[doc = ""]
    #[doc = " @returns Number of uniforms used by shader."]
    #[doc = ""]
    pub fn bgfx_get_shader_uniforms(
        _handle: bgfx_shader_handle_t,
        _uniforms: *mut bgfx_uniform_handle_t,
        _max: u16,
    ) -> u16;
}
extern "C" {
    #[doc = " Set shader debug name."]
    #[doc = ""]
    #[doc = " @param[in] _handle Shader handle."]
    #[doc = " @param[in] _name Shader name."]
    #[doc = " @param[in] _len Shader name length (if length is INT32_MAX, it's expected"]
    #[doc = "  that _name is zero terminated string)."]
    #[doc = ""]
    pub fn bgfx_set_shader_name(
        _handle: bgfx_shader_handle_t,
        _name: *const ::std::os::raw::c_char,
        _len: i32,
    );
}
extern "C" {
    #[doc = " Destroy shader."]
    #[doc = " @remark Once a shader program is created with _handle,"]
    #[doc = "   it is safe to destroy that shader."]
    #[doc = ""]
    #[doc = " @param[in] _handle Shader handle."]
    #[doc = ""]
    pub fn bgfx_destroy_shader(_handle: bgfx_shader_handle_t);
}
extern "C" {
    #[doc = " Create program with vertex and fragment shaders."]
    #[doc = ""]
    #[doc = " @param[in] _vsh Vertex shader."]
    #[doc = " @param[in] _fsh Fragment shader."]
    #[doc = " @param[in] _destroyShaders If true, shaders will be destroyed when program is destroyed."]
    #[doc = ""]
    #[doc = " @returns Program handle if vertex shader output and fragment shader"]
    #[doc = "  input are matching, otherwise returns invalid program handle."]
    #[doc = ""]
    pub fn bgfx_create_program(
        _vsh: bgfx_shader_handle_t,
        _fsh: bgfx_shader_handle_t,
        _destroyShaders: bool,
    ) -> bgfx_program_handle_t;
}
extern "C" {
    #[doc = " Create program with compute shader."]
    #[doc = ""]
    #[doc = " @param[in] _csh Compute shader."]
    #[doc = " @param[in] _destroyShaders If true, shaders will be destroyed when program is destroyed."]
    #[doc = ""]
    #[doc = " @returns Program handle."]
    #[doc = ""]
    pub fn bgfx_create_compute_program(
        _csh: bgfx_shader_handle_t,
        _destroyShaders: bool,
    ) -> bgfx_program_handle_t;
}
extern "C" {
    #[doc = " Destroy program."]
    #[doc = ""]
    #[doc = " @param[in] _handle Program handle."]
    #[doc = ""]
    pub fn bgfx_destroy_program(_handle: bgfx_program_handle_t);
}
extern "C" {
    #[doc = " Validate texture parameters."]
    #[doc = ""]
    #[doc = " @param[in] _depth Depth dimension of volume texture."]
    #[doc = " @param[in] _cubeMap Indicates that texture contains cubemap."]
    #[doc = " @param[in] _numLayers Number of layers in texture array."]
    #[doc = " @param[in] _format Texture format. See: `TextureFormat::Enum`."]
    #[doc = " @param[in] _flags Texture flags. See `BGFX_TEXTURE_*`."]
    #[doc = ""]
    #[doc = " @returns True if texture can be successfully created."]
    #[doc = ""]
    pub fn bgfx_is_texture_valid(
        _depth: u16,
        _cubeMap: bool,
        _numLayers: u16,
        _format: bgfx_texture_format_t,
        _flags: u64,
    ) -> bool;
}
extern "C" {
    #[doc = " Validate frame buffer parameters."]
    #[doc = ""]
    #[doc = " @param[in] _num Number of attachments."]
    #[doc = " @param[in] _attachment Attachment texture info. See: `bgfx::Attachment`."]
    #[doc = ""]
    #[doc = " @returns True if frame buffer can be successfully created."]
    #[doc = ""]
    pub fn bgfx_is_frame_buffer_valid(_num: u8, _attachment: *const bgfx_attachment_t) -> bool;
}
extern "C" {
    #[doc = " Calculate amount of memory required for texture."]
    #[doc = ""]
    #[doc = " @param[out] _info Resulting texture info structure. See: `TextureInfo`."]
    #[doc = " @param[in] _width Width."]
    #[doc = " @param[in] _height Height."]
    #[doc = " @param[in] _depth Depth dimension of volume texture."]
    #[doc = " @param[in] _cubeMap Indicates that texture contains cubemap."]
    #[doc = " @param[in] _hasMips Indicates that texture contains full mip-map chain."]
    #[doc = " @param[in] _numLayers Number of layers in texture array."]
    #[doc = " @param[in] _format Texture format. See: `TextureFormat::Enum`."]
    #[doc = ""]
    pub fn bgfx_calc_texture_size(
        _info: *mut bgfx_texture_info_t,
        _width: u16,
        _height: u16,
        _depth: u16,
        _cubeMap: bool,
        _hasMips: bool,
        _numLayers: u16,
        _format: bgfx_texture_format_t,
    );
}
extern "C" {
    #[doc = " Create texture from memory buffer."]
    #[doc = ""]
    #[doc = " @param[in] _mem DDS, KTX or PVR texture binary data."]
    #[doc = " @param[in] _flags Texture creation (see `BGFX_TEXTURE_*`.), and sampler (see `BGFX_SAMPLER_*`)"]
    #[doc = "  flags. Default texture sampling mode is linear, and wrap mode is repeat."]
    #[doc = "  - `BGFX_SAMPLER_[U/V/W]_[MIRROR/CLAMP]` - Mirror or clamp to edge wrap"]
    #[doc = "    mode."]
    #[doc = "  - `BGFX_SAMPLER_[MIN/MAG/MIP]_[POINT/ANISOTROPIC]` - Point or anisotropic"]
    #[doc = "    sampling."]
    #[doc = " @param[in] _skip Skip top level mips when parsing texture."]
    #[doc = " @param[out] _info When non-`NULL` is specified it returns parsed texture information."]
    #[doc = ""]
    #[doc = " @returns Texture handle."]
    #[doc = ""]
    pub fn bgfx_create_texture(
        _mem: *const bgfx_memory_t,
        _flags: u64,
        _skip: u8,
        _info: *mut bgfx_texture_info_t,
    ) -> bgfx_texture_handle_t;
}
extern "C" {
    #[doc = " Create 2D texture."]
    #[doc = ""]
    #[doc = " @param[in] _width Width."]
    #[doc = " @param[in] _height Height."]
    #[doc = " @param[in] _hasMips Indicates that texture contains full mip-map chain."]
    #[doc = " @param[in] _numLayers Number of layers in texture array. Must be 1 if caps"]
    #[doc = "  `BGFX_CAPS_TEXTURE_2D_ARRAY` flag is not set."]
    #[doc = " @param[in] _format Texture format. See: `TextureFormat::Enum`."]
    #[doc = " @param[in] _flags Texture creation (see `BGFX_TEXTURE_*`.), and sampler (see `BGFX_SAMPLER_*`)"]
    #[doc = "  flags. Default texture sampling mode is linear, and wrap mode is repeat."]
    #[doc = "  - `BGFX_SAMPLER_[U/V/W]_[MIRROR/CLAMP]` - Mirror or clamp to edge wrap"]
    #[doc = "    mode."]
    #[doc = "  - `BGFX_SAMPLER_[MIN/MAG/MIP]_[POINT/ANISOTROPIC]` - Point or anisotropic"]
    #[doc = "    sampling."]
    #[doc = " @param[in] _mem Texture data. If `_mem` is non-NULL, created texture will be immutable. If"]
    #[doc = "  `_mem` is NULL content of the texture is uninitialized. When `_numLayers` is more than"]
    #[doc = "  1, expected memory layout is texture and all mips together for each array element."]
    #[doc = ""]
    #[doc = " @returns Texture handle."]
    #[doc = ""]
    pub fn bgfx_create_texture_2d(
        _width: u16,
        _height: u16,
        _hasMips: bool,
        _numLayers: u16,
        _format: bgfx_texture_format_t,
        _flags: u64,
        _mem: *const bgfx_memory_t,
    ) -> bgfx_texture_handle_t;
}
extern "C" {
    #[doc = " Create texture with size based on backbuffer ratio. Texture will maintain ratio"]
    #[doc = " if back buffer resolution changes."]
    #[doc = ""]
    #[doc = " @param[in] _ratio Texture size in respect to back-buffer size. See: `BackbufferRatio::Enum`."]
    #[doc = " @param[in] _hasMips Indicates that texture contains full mip-map chain."]
    #[doc = " @param[in] _numLayers Number of layers in texture array. Must be 1 if caps"]
    #[doc = "  `BGFX_CAPS_TEXTURE_2D_ARRAY` flag is not set."]
    #[doc = " @param[in] _format Texture format. See: `TextureFormat::Enum`."]
    #[doc = " @param[in] _flags Texture creation (see `BGFX_TEXTURE_*`.), and sampler (see `BGFX_SAMPLER_*`)"]
    #[doc = "  flags. Default texture sampling mode is linear, and wrap mode is repeat."]
    #[doc = "  - `BGFX_SAMPLER_[U/V/W]_[MIRROR/CLAMP]` - Mirror or clamp to edge wrap"]
    #[doc = "    mode."]
    #[doc = "  - `BGFX_SAMPLER_[MIN/MAG/MIP]_[POINT/ANISOTROPIC]` - Point or anisotropic"]
    #[doc = "    sampling."]
    #[doc = ""]
    #[doc = " @returns Texture handle."]
    #[doc = ""]
    pub fn bgfx_create_texture_2d_scaled(
        _ratio: bgfx_backbuffer_ratio_t,
        _hasMips: bool,
        _numLayers: u16,
        _format: bgfx_texture_format_t,
        _flags: u64,
    ) -> bgfx_texture_handle_t;
}
extern "C" {
    #[doc = " Create 3D texture."]
    #[doc = ""]
    #[doc = " @param[in] _width Width."]
    #[doc = " @param[in] _height Height."]
    #[doc = " @param[in] _depth Depth."]
    #[doc = " @param[in] _hasMips Indicates that texture contains full mip-map chain."]
    #[doc = " @param[in] _format Texture format. See: `TextureFormat::Enum`."]
    #[doc = " @param[in] _flags Texture creation (see `BGFX_TEXTURE_*`.), and sampler (see `BGFX_SAMPLER_*`)"]
    #[doc = "  flags. Default texture sampling mode is linear, and wrap mode is repeat."]
    #[doc = "  - `BGFX_SAMPLER_[U/V/W]_[MIRROR/CLAMP]` - Mirror or clamp to edge wrap"]
    #[doc = "    mode."]
    #[doc = "  - `BGFX_SAMPLER_[MIN/MAG/MIP]_[POINT/ANISOTROPIC]` - Point or anisotropic"]
    #[doc = "    sampling."]
    #[doc = " @param[in] _mem Texture data. If `_mem` is non-NULL, created texture will be immutable. If"]
    #[doc = "  `_mem` is NULL content of the texture is uninitialized. When `_numLayers` is more than"]
    #[doc = "  1, expected memory layout is texture and all mips together for each array element."]
    #[doc = ""]
    #[doc = " @returns Texture handle."]
    #[doc = ""]
    pub fn bgfx_create_texture_3d(
        _width: u16,
        _height: u16,
        _depth: u16,
        _hasMips: bool,
        _format: bgfx_texture_format_t,
        _flags: u64,
        _mem: *const bgfx_memory_t,
    ) -> bgfx_texture_handle_t;
}
extern "C" {
    #[doc = " Create Cube texture."]
    #[doc = ""]
    #[doc = " @param[in] _size Cube side size."]
    #[doc = " @param[in] _hasMips Indicates that texture contains full mip-map chain."]
    #[doc = " @param[in] _numLayers Number of layers in texture array. Must be 1 if caps"]
    #[doc = "  `BGFX_CAPS_TEXTURE_2D_ARRAY` flag is not set."]
    #[doc = " @param[in] _format Texture format. See: `TextureFormat::Enum`."]
    #[doc = " @param[in] _flags Texture creation (see `BGFX_TEXTURE_*`.), and sampler (see `BGFX_SAMPLER_*`)"]
    #[doc = "  flags. Default texture sampling mode is linear, and wrap mode is repeat."]
    #[doc = "  - `BGFX_SAMPLER_[U/V/W]_[MIRROR/CLAMP]` - Mirror or clamp to edge wrap"]
    #[doc = "    mode."]
    #[doc = "  - `BGFX_SAMPLER_[MIN/MAG/MIP]_[POINT/ANISOTROPIC]` - Point or anisotropic"]
    #[doc = "    sampling."]
    #[doc = " @param[in] _mem Texture data. If `_mem` is non-NULL, created texture will be immutable. If"]
    #[doc = "  `_mem` is NULL content of the texture is uninitialized. When `_numLayers` is more than"]
    #[doc = "  1, expected memory layout is texture and all mips together for each array element."]
    #[doc = ""]
    #[doc = " @returns Texture handle."]
    #[doc = ""]
    pub fn bgfx_create_texture_cube(
        _size: u16,
        _hasMips: bool,
        _numLayers: u16,
        _format: bgfx_texture_format_t,
        _flags: u64,
        _mem: *const bgfx_memory_t,
    ) -> bgfx_texture_handle_t;
}
extern "C" {
    #[doc = " Update 2D texture."]
    #[doc = " @attention It's valid to update only mutable texture. See `bgfx::createTexture2D` for more info."]
    #[doc = ""]
    #[doc = " @param[in] _handle Texture handle."]
    #[doc = " @param[in] _layer Layer in texture array."]
    #[doc = " @param[in] _mip Mip level."]
    #[doc = " @param[in] _x X offset in texture."]
    #[doc = " @param[in] _y Y offset in texture."]
    #[doc = " @param[in] _width Width of texture block."]
    #[doc = " @param[in] _height Height of texture block."]
    #[doc = " @param[in] _mem Texture update data."]
    #[doc = " @param[in] _pitch Pitch of input image (bytes). When _pitch is set to"]
    #[doc = "  UINT16_MAX, it will be calculated internally based on _width."]
    #[doc = ""]
    pub fn bgfx_update_texture_2d(
        _handle: bgfx_texture_handle_t,
        _layer: u16,
        _mip: u8,
        _x: u16,
        _y: u16,
        _width: u16,
        _height: u16,
        _mem: *const bgfx_memory_t,
        _pitch: u16,
    );
}
extern "C" {
    #[doc = " Update 3D texture."]
    #[doc = " @attention It's valid to update only mutable texture. See `bgfx::createTexture3D` for more info."]
    #[doc = ""]
    #[doc = " @param[in] _handle Texture handle."]
    #[doc = " @param[in] _mip Mip level."]
    #[doc = " @param[in] _x X offset in texture."]
    #[doc = " @param[in] _y Y offset in texture."]
    #[doc = " @param[in] _z Z offset in texture."]
    #[doc = " @param[in] _width Width of texture block."]
    #[doc = " @param[in] _height Height of texture block."]
    #[doc = " @param[in] _depth Depth of texture block."]
    #[doc = " @param[in] _mem Texture update data."]
    #[doc = ""]
    pub fn bgfx_update_texture_3d(
        _handle: bgfx_texture_handle_t,
        _mip: u8,
        _x: u16,
        _y: u16,
        _z: u16,
        _width: u16,
        _height: u16,
        _depth: u16,
        _mem: *const bgfx_memory_t,
    );
}
extern "C" {
    #[doc = " Update Cube texture."]
    #[doc = " @attention It's valid to update only mutable texture. See `bgfx::createTextureCube` for more info."]
    #[doc = ""]
    #[doc = " @param[in] _handle Texture handle."]
    #[doc = " @param[in] _layer Layer in texture array."]
    #[doc = " @param[in] _side Cubemap side `BGFX_CUBE_MAP_<POSITIVE or NEGATIVE>_<X, Y or Z>`,"]
    #[doc = "    where 0 is +X, 1 is -X, 2 is +Y, 3 is -Y, 4 is +Z, and 5 is -Z."]
    #[doc = "                   +----------+"]
    #[doc = "                   |-z       2|"]
    #[doc = "                   | ^  +y    |"]
    #[doc = "                   | |        |    Unfolded cube:"]
    #[doc = "                   | +---->+x |"]
    #[doc = "        +----------+----------+----------+----------+"]
    #[doc = "        |+y       1|+y       4|+y       0|+y       5|"]
    #[doc = "        | ^  -x    | ^  +z    | ^  +x    | ^  -z    |"]
    #[doc = "        | |        | |        | |        | |        |"]
    #[doc = "        | +---->+z | +---->+x | +---->-z | +---->-x |"]
    #[doc = "        +----------+----------+----------+----------+"]
    #[doc = "                   |+z       3|"]
    #[doc = "                   | ^  -y    |"]
    #[doc = "                   | |        |"]
    #[doc = "                   | +---->+x |"]
    #[doc = "                   +----------+"]
    #[doc = " @param[in] _mip Mip level."]
    #[doc = " @param[in] _x X offset in texture."]
    #[doc = " @param[in] _y Y offset in texture."]
    #[doc = " @param[in] _width Width of texture block."]
    #[doc = " @param[in] _height Height of texture block."]
    #[doc = " @param[in] _mem Texture update data."]
    #[doc = " @param[in] _pitch Pitch of input image (bytes). When _pitch is set to"]
    #[doc = "  UINT16_MAX, it will be calculated internally based on _width."]
    #[doc = ""]
    pub fn bgfx_update_texture_cube(
        _handle: bgfx_texture_handle_t,
        _layer: u16,
        _side: u8,
        _mip: u8,
        _x: u16,
        _y: u16,
        _width: u16,
        _height: u16,
        _mem: *const bgfx_memory_t,
        _pitch: u16,
    );
}
extern "C" {
    #[doc = " Read back texture content."]
    #[doc = " @attention Texture must be created with `BGFX_TEXTURE_READ_BACK` flag."]
    #[doc = " @attention Availability depends on: `BGFX_CAPS_TEXTURE_READ_BACK`."]
    #[doc = ""]
    #[doc = " @param[in] _handle Texture handle."]
    #[doc = " @param[in] _data Destination buffer."]
    #[doc = " @param[in] _mip Mip level."]
    #[doc = ""]
    #[doc = " @returns Frame number when the result will be available. See: `bgfx::frame`."]
    #[doc = ""]
    pub fn bgfx_read_texture(
        _handle: bgfx_texture_handle_t,
        _data: *mut ::std::os::raw::c_void,
        _mip: u8,
    ) -> u32;
}
extern "C" {
    #[doc = " Set texture debug name."]
    #[doc = ""]
    #[doc = " @param[in] _handle Texture handle."]
    #[doc = " @param[in] _name Texture name."]
    #[doc = " @param[in] _len Texture name length (if length is INT32_MAX, it's expected"]
    #[doc = "  that _name is zero terminated string."]
    #[doc = ""]
    pub fn bgfx_set_texture_name(
        _handle: bgfx_texture_handle_t,
        _name: *const ::std::os::raw::c_char,
        _len: i32,
    );
}
extern "C" {
    #[doc = " Returns texture direct access pointer."]
    #[doc = " @attention Availability depends on: `BGFX_CAPS_TEXTURE_DIRECT_ACCESS`. This feature"]
    #[doc = "   is available on GPUs that have unified memory architecture (UMA) support."]
    #[doc = ""]
    #[doc = " @param[in] _handle Texture handle."]
    #[doc = ""]
    #[doc = " @returns Pointer to texture memory. If returned pointer is `NULL` direct access"]
    #[doc = "  is not available for this texture. If pointer is `UINTPTR_MAX` sentinel value"]
    #[doc = "  it means texture is pending creation. Pointer returned can be cached and it"]
    #[doc = "  will be valid until texture is destroyed."]
    #[doc = ""]
    pub fn bgfx_get_direct_access_ptr(
        _handle: bgfx_texture_handle_t,
    ) -> *mut ::std::os::raw::c_void;
}
extern "C" {
    #[doc = " Destroy texture."]
    #[doc = ""]
    #[doc = " @param[in] _handle Texture handle."]
    #[doc = ""]
    pub fn bgfx_destroy_texture(_handle: bgfx_texture_handle_t);
}
extern "C" {
    #[doc = " Create frame buffer (simple)."]
    #[doc = ""]
    #[doc = " @param[in] _width Texture width."]
    #[doc = " @param[in] _height Texture height."]
    #[doc = " @param[in] _format Texture format. See: `TextureFormat::Enum`."]
    #[doc = " @param[in] _textureFlags Texture creation (see `BGFX_TEXTURE_*`.), and sampler (see `BGFX_SAMPLER_*`)"]
    #[doc = "  flags. Default texture sampling mode is linear, and wrap mode is repeat."]
    #[doc = "  - `BGFX_SAMPLER_[U/V/W]_[MIRROR/CLAMP]` - Mirror or clamp to edge wrap"]
    #[doc = "    mode."]
    #[doc = "  - `BGFX_SAMPLER_[MIN/MAG/MIP]_[POINT/ANISOTROPIC]` - Point or anisotropic"]
    #[doc = "    sampling."]
    #[doc = ""]
    #[doc = " @returns Frame buffer handle."]
    #[doc = ""]
    pub fn bgfx_create_frame_buffer(
        _width: u16,
        _height: u16,
        _format: bgfx_texture_format_t,
        _textureFlags: u64,
    ) -> bgfx_frame_buffer_handle_t;
}
extern "C" {
    #[doc = " Create frame buffer with size based on backbuffer ratio. Frame buffer will maintain ratio"]
    #[doc = " if back buffer resolution changes."]
    #[doc = ""]
    #[doc = " @param[in] _ratio Frame buffer size in respect to back-buffer size. See:"]
    #[doc = "  `BackbufferRatio::Enum`."]
    #[doc = " @param[in] _format Texture format. See: `TextureFormat::Enum`."]
    #[doc = " @param[in] _textureFlags Texture creation (see `BGFX_TEXTURE_*`.), and sampler (see `BGFX_SAMPLER_*`)"]
    #[doc = "  flags. Default texture sampling mode is linear, and wrap mode is repeat."]
    #[doc = "  - `BGFX_SAMPLER_[U/V/W]_[MIRROR/CLAMP]` - Mirror or clamp to edge wrap"]
    #[doc = "    mode."]
    #[doc = "  - `BGFX_SAMPLER_[MIN/MAG/MIP]_[POINT/ANISOTROPIC]` - Point or anisotropic"]
    #[doc = "    sampling."]
    #[doc = ""]
    #[doc = " @returns Frame buffer handle."]
    #[doc = ""]
    pub fn bgfx_create_frame_buffer_scaled(
        _ratio: bgfx_backbuffer_ratio_t,
        _format: bgfx_texture_format_t,
        _textureFlags: u64,
    ) -> bgfx_frame_buffer_handle_t;
}
extern "C" {
    #[doc = " Create MRT frame buffer from texture handles (simple)."]
    #[doc = ""]
    #[doc = " @param[in] _num Number of texture handles."]
    #[doc = " @param[in] _handles Texture attachments."]
    #[doc = " @param[in] _destroyTexture If true, textures will be destroyed when"]
    #[doc = "  frame buffer is destroyed."]
    #[doc = ""]
    #[doc = " @returns Frame buffer handle."]
    #[doc = ""]
    pub fn bgfx_create_frame_buffer_from_handles(
        _num: u8,
        _handles: *const bgfx_texture_handle_t,
        _destroyTexture: bool,
    ) -> bgfx_frame_buffer_handle_t;
}
extern "C" {
    #[doc = " Create MRT frame buffer from texture handles with specific layer and"]
    #[doc = " mip level."]
    #[doc = ""]
    #[doc = " @param[in] _num Number of attachments."]
    #[doc = " @param[in] _attachment Attachment texture info. See: `bgfx::Attachment`."]
    #[doc = " @param[in] _destroyTexture If true, textures will be destroyed when"]
    #[doc = "  frame buffer is destroyed."]
    #[doc = ""]
    #[doc = " @returns Frame buffer handle."]
    #[doc = ""]
    pub fn bgfx_create_frame_buffer_from_attachment(
        _num: u8,
        _attachment: *const bgfx_attachment_t,
        _destroyTexture: bool,
    ) -> bgfx_frame_buffer_handle_t;
}
extern "C" {
    #[doc = " Create frame buffer for multiple window rendering."]
    #[doc = " @remarks"]
    #[doc = "   Frame buffer cannot be used for sampling."]
    #[doc = " @attention Availability depends on: `BGFX_CAPS_SWAP_CHAIN`."]
    #[doc = ""]
    #[doc = " @param[in] _nwh OS' target native window handle."]
    #[doc = " @param[in] _width Window back buffer width."]
    #[doc = " @param[in] _height Window back buffer height."]
    #[doc = " @param[in] _format Window back buffer color format."]
    #[doc = " @param[in] _depthFormat Window back buffer depth format."]
    #[doc = ""]
    #[doc = " @returns Frame buffer handle."]
    #[doc = ""]
    pub fn bgfx_create_frame_buffer_from_nwh(
        _nwh: *mut ::std::os::raw::c_void,
        _width: u16,
        _height: u16,
        _format: bgfx_texture_format_t,
        _depthFormat: bgfx_texture_format_t,
    ) -> bgfx_frame_buffer_handle_t;
}
extern "C" {
    #[doc = " Set frame buffer debug name."]
    #[doc = ""]
    #[doc = " @param[in] _handle Frame buffer handle."]
    #[doc = " @param[in] _name Frame buffer name."]
    #[doc = " @param[in] _len Frame buffer name length (if length is INT32_MAX, it's expected"]
    #[doc = "  that _name is zero terminated string."]
    #[doc = ""]
    pub fn bgfx_set_frame_buffer_name(
        _handle: bgfx_frame_buffer_handle_t,
        _name: *const ::std::os::raw::c_char,
        _len: i32,
    );
}
extern "C" {
    #[doc = " Obtain texture handle of frame buffer attachment."]
    #[doc = ""]
    #[doc = " @param[in] _handle Frame buffer handle."]
    #[doc = " @param[in] _attachment"]
    #[doc = ""]
    pub fn bgfx_get_texture(
        _handle: bgfx_frame_buffer_handle_t,
        _attachment: u8,
    ) -> bgfx_texture_handle_t;
}
extern "C" {
    #[doc = " Destroy frame buffer."]
    #[doc = ""]
    #[doc = " @param[in] _handle Frame buffer handle."]
    #[doc = ""]
    pub fn bgfx_destroy_frame_buffer(_handle: bgfx_frame_buffer_handle_t);
}
extern "C" {
    #[doc = " Create shader uniform parameter."]
    #[doc = " @remarks"]
    #[doc = "   1. Uniform names are unique. It's valid to call `bgfx::createUniform`"]
    #[doc = "      multiple times with the same uniform name. The library will always"]
    #[doc = "      return the same handle, but the handle reference count will be"]
    #[doc = "      incremented. This means that the same number of `bgfx::destroyUniform`"]
    #[doc = "      must be called to properly destroy the uniform."]
    #[doc = "   2. Predefined uniforms (declared in `bgfx_shader.sh`):"]
    #[doc = "      - `u_viewRect vec4(x, y, width, height)` - view rectangle for current"]
    #[doc = "        view, in pixels."]
    #[doc = "      - `u_viewTexel vec4(1.0/width, 1.0/height, undef, undef)` - inverse"]
    #[doc = "        width and height"]
    #[doc = "      - `u_view mat4` - view matrix"]
    #[doc = "      - `u_invView mat4` - inverted view matrix"]
    #[doc = "      - `u_proj mat4` - projection matrix"]
    #[doc = "      - `u_invProj mat4` - inverted projection matrix"]
    #[doc = "      - `u_viewProj mat4` - concatenated view projection matrix"]
    #[doc = "      - `u_invViewProj mat4` - concatenated inverted view projection matrix"]
    #[doc = "      - `u_model mat4[BGFX_CONFIG_MAX_BONES]` - array of model matrices."]
    #[doc = "      - `u_modelView mat4` - concatenated model view matrix, only first"]
    #[doc = "        model matrix from array is used."]
    #[doc = "      - `u_modelViewProj mat4` - concatenated model view projection matrix."]
    #[doc = "      - `u_alphaRef float` - alpha reference value for alpha test."]
    #[doc = ""]
    #[doc = " @param[in] _name Uniform name in shader."]
    #[doc = " @param[in] _type Type of uniform (See: `bgfx::UniformType`)."]
    #[doc = " @param[in] _num Number of elements in array."]
    #[doc = ""]
    #[doc = " @returns Handle to uniform object."]
    #[doc = ""]
    pub fn bgfx_create_uniform(
        _name: *const ::std::os::raw::c_char,
        _type: bgfx_uniform_type_t,
        _num: u16,
    ) -> bgfx_uniform_handle_t;
}
extern "C" {
    #[doc = " Retrieve uniform info."]
    #[doc = ""]
    #[doc = " @param[in] _handle Handle to uniform object."]
    #[doc = " @param[out] _info Uniform info."]
    #[doc = ""]
    pub fn bgfx_get_uniform_info(_handle: bgfx_uniform_handle_t, _info: *mut bgfx_uniform_info_t);
}
extern "C" {
    #[doc = " Destroy shader uniform parameter."]
    #[doc = ""]
    #[doc = " @param[in] _handle Handle to uniform object."]
    #[doc = ""]
    pub fn bgfx_destroy_uniform(_handle: bgfx_uniform_handle_t);
}
extern "C" {
    #[doc = " Create occlusion query."]
    #[doc = ""]
    pub fn bgfx_create_occlusion_query() -> bgfx_occlusion_query_handle_t;
}
extern "C" {
    #[doc = " Retrieve occlusion query result from previous frame."]
    #[doc = ""]
    #[doc = " @param[in] _handle Handle to occlusion query object."]
    #[doc = " @param[out] _result Number of pixels that passed test. This argument"]
    #[doc = "  can be `NULL` if result of occlusion query is not needed."]
    #[doc = ""]
    #[doc = " @returns Occlusion query result."]
    #[doc = ""]
    pub fn bgfx_get_result(
        _handle: bgfx_occlusion_query_handle_t,
        _result: *mut i32,
    ) -> bgfx_occlusion_query_result_t;
}
extern "C" {
    #[doc = " Destroy occlusion query."]
    #[doc = ""]
    #[doc = " @param[in] _handle Handle to occlusion query object."]
    #[doc = ""]
    pub fn bgfx_destroy_occlusion_query(_handle: bgfx_occlusion_query_handle_t);
}
extern "C" {
    #[doc = " Set palette color value."]
    #[doc = ""]
    #[doc = " @param[in] _index Index into palette."]
    #[doc = " @param[in] _rgba RGBA floating point values."]
    #[doc = ""]
    pub fn bgfx_set_palette_color(_index: u8, _rgba: *const f32);
}
extern "C" {
    #[doc = " Set palette color value."]
    #[doc = ""]
    #[doc = " @param[in] _index Index into palette."]
    #[doc = " @param[in] _rgba Packed 32-bit RGBA value."]
    #[doc = ""]
    pub fn bgfx_set_palette_color_rgba8(_index: u8, _rgba: u32);
}
extern "C" {
    #[doc = " Set view name."]
    #[doc = " @remarks"]
    #[doc = "   This is debug only feature."]
    #[doc = "   In graphics debugger view name will appear as:"]
    #[doc = "       \"nnnc <view name>\""]
    #[doc = "        ^  ^ ^"]
    #[doc = "        |  +--- compute (C)"]
    #[doc = "        +------ view id"]
    #[doc = ""]
    #[doc = " @param[in] _id View id."]
    #[doc = " @param[in] _name View name."]
    #[doc = ""]
    pub fn bgfx_set_view_name(_id: bgfx_view_id_t, _name: *const ::std::os::raw::c_char);
}
extern "C" {
    #[doc = " Set view rectangle. Draw primitive outside view will be clipped."]
    #[doc = ""]
    #[doc = " @param[in] _id View id."]
    #[doc = " @param[in] _x Position x from the left corner of the window."]
    #[doc = " @param[in] _y Position y from the top corner of the window."]
    #[doc = " @param[in] _width Width of view port region."]
    #[doc = " @param[in] _height Height of view port region."]
    #[doc = ""]
    pub fn bgfx_set_view_rect(_id: bgfx_view_id_t, _x: u16, _y: u16, _width: u16, _height: u16);
}
extern "C" {
    #[doc = " Set view rectangle. Draw primitive outside view will be clipped."]
    #[doc = ""]
    #[doc = " @param[in] _id View id."]
    #[doc = " @param[in] _x Position x from the left corner of the window."]
    #[doc = " @param[in] _y Position y from the top corner of the window."]
    #[doc = " @param[in] _ratio Width and height will be set in respect to back-buffer size."]
    #[doc = "  See: `BackbufferRatio::Enum`."]
    #[doc = ""]
    pub fn bgfx_set_view_rect_ratio(
        _id: bgfx_view_id_t,
        _x: u16,
        _y: u16,
        _ratio: bgfx_backbuffer_ratio_t,
    );
}
extern "C" {
    #[doc = " Set view scissor. Draw primitive outside view will be clipped. When"]
    #[doc = " _x, _y, _width and _height are set to 0, scissor will be disabled."]
    #[doc = ""]
    #[doc = " @param[in] _id View id."]
    #[doc = " @param[in] _x Position x from the left corner of the window."]
    #[doc = " @param[in] _y Position y from the top corner of the window."]
    #[doc = " @param[in] _width Width of view scissor region."]
    #[doc = " @param[in] _height Height of view scissor region."]
    #[doc = ""]
    pub fn bgfx_set_view_scissor(_id: bgfx_view_id_t, _x: u16, _y: u16, _width: u16, _height: u16);
}
extern "C" {
    #[doc = " Set view clear flags."]
    #[doc = ""]
    #[doc = " @param[in] _id View id."]
    #[doc = " @param[in] _flags Clear flags. Use `BGFX_CLEAR_NONE` to remove any clear"]
    #[doc = "  operation. See: `BGFX_CLEAR_*`."]
    #[doc = " @param[in] _rgba Color clear value."]
    #[doc = " @param[in] _depth Depth clear value."]
    #[doc = " @param[in] _stencil Stencil clear value."]
    #[doc = ""]
    pub fn bgfx_set_view_clear(
        _id: bgfx_view_id_t,
        _flags: u16,
        _rgba: u32,
        _depth: f32,
        _stencil: u8,
    );
}
extern "C" {
    #[doc = " Set view clear flags with different clear color for each"]
    #[doc = " frame buffer texture. Must use `bgfx::setPaletteColor` to setup clear color"]
    #[doc = " palette."]
    #[doc = ""]
    #[doc = " @param[in] _id View id."]
    #[doc = " @param[in] _flags Clear flags. Use `BGFX_CLEAR_NONE` to remove any clear"]
    #[doc = "  operation. See: `BGFX_CLEAR_*`."]
    #[doc = " @param[in] _depth Depth clear value."]
    #[doc = " @param[in] _stencil Stencil clear value."]
    #[doc = " @param[in] _c0 Palette index for frame buffer attachment 0."]
    #[doc = " @param[in] _c1 Palette index for frame buffer attachment 1."]
    #[doc = " @param[in] _c2 Palette index for frame buffer attachment 2."]
    #[doc = " @param[in] _c3 Palette index for frame buffer attachment 3."]
    #[doc = " @param[in] _c4 Palette index for frame buffer attachment 4."]
    #[doc = " @param[in] _c5 Palette index for frame buffer attachment 5."]
    #[doc = " @param[in] _c6 Palette index for frame buffer attachment 6."]
    #[doc = " @param[in] _c7 Palette index for frame buffer attachment 7."]
    #[doc = ""]
    pub fn bgfx_set_view_clear_mrt(
        _id: bgfx_view_id_t,
        _flags: u16,
        _depth: f32,
        _stencil: u8,
        _c0: u8,
        _c1: u8,
        _c2: u8,
        _c3: u8,
        _c4: u8,
        _c5: u8,
        _c6: u8,
        _c7: u8,
    );
}
extern "C" {
    #[doc = " Set view sorting mode."]
    #[doc = " @remarks"]
    #[doc = "   View mode must be set prior calling `bgfx::submit` for the view."]
    #[doc = ""]
    #[doc = " @param[in] _id View id."]
    #[doc = " @param[in] _mode View sort mode. See `ViewMode::Enum`."]
    #[doc = ""]
    pub fn bgfx_set_view_mode(_id: bgfx_view_id_t, _mode: bgfx_view_mode_t);
}
extern "C" {
    #[doc = " Set view frame buffer."]
    #[doc = " @remarks"]
    #[doc = "   Not persistent after `bgfx::reset` call."]
    #[doc = ""]
    #[doc = " @param[in] _id View id."]
    #[doc = " @param[in] _handle Frame buffer handle. Passing `BGFX_INVALID_HANDLE` as"]
    #[doc = "  frame buffer handle will draw primitives from this view into"]
    #[doc = "  default back buffer."]
    #[doc = ""]
    pub fn bgfx_set_view_frame_buffer(_id: bgfx_view_id_t, _handle: bgfx_frame_buffer_handle_t);
}
extern "C" {
    #[doc = " Set view view and projection matrices, all draw primitives in this"]
    #[doc = " view will use these matrices."]
    #[doc = ""]
    #[doc = " @param[in] _id View id."]
    #[doc = " @param[in] _view View matrix."]
    #[doc = " @param[in] _proj Projection matrix."]
    #[doc = ""]
    pub fn bgfx_set_view_transform(
        _id: bgfx_view_id_t,
        _view: *const ::std::os::raw::c_void,
        _proj: *const ::std::os::raw::c_void,
    );
}
extern "C" {
    #[doc = " Post submit view reordering."]
    #[doc = ""]
    #[doc = " @param[in] _id First view id."]
    #[doc = " @param[in] _num Number of views to remap."]
    #[doc = " @param[in] _order View remap id table. Passing `NULL` will reset view ids"]
    #[doc = "  to default state."]
    #[doc = ""]
    pub fn bgfx_set_view_order(_id: bgfx_view_id_t, _num: u16, _order: *const bgfx_view_id_t);
}
extern "C" {
    #[doc = " Reset all view settings to default."]
    #[doc = ""]
    #[doc = " @param[in] _id"]
    #[doc = ""]
    pub fn bgfx_reset_view(_id: bgfx_view_id_t);
}
extern "C" {
    #[doc = " Begin submitting draw calls from thread."]
    #[doc = ""]
    #[doc = " @param[in] _forThread Explicitly request an encoder for a worker thread."]
    #[doc = ""]
    #[doc = " @returns Encoder."]
    #[doc = ""]
    pub fn bgfx_encoder_begin(_forThread: bool) -> *mut bgfx_encoder_t;
}
extern "C" {
    #[doc = " End submitting draw calls from thread."]
    #[doc = ""]
    #[doc = " @param[in] _encoder Encoder."]
    #[doc = ""]
    pub fn bgfx_encoder_end(_encoder: *mut bgfx_encoder_t);
}
extern "C" {
    #[doc = " Sets a debug marker. This allows you to group graphics calls together for easy browsing in"]
    #[doc = " graphics debugging tools."]
    #[doc = ""]
    #[doc = " @param[in] _marker Marker string."]
    #[doc = ""]
    pub fn bgfx_encoder_set_marker(
        _this: *mut bgfx_encoder_t,
        _marker: *const ::std::os::raw::c_char,
    );
}
extern "C" {
    #[doc = " Set render states for draw primitive."]
    #[doc = " @remarks"]
    #[doc = "   1. To setup more complex states use:"]
    #[doc = "      `BGFX_STATE_ALPHA_REF(_ref)`,"]
    #[doc = "      `BGFX_STATE_POINT_SIZE(_size)`,"]
    #[doc = "      `BGFX_STATE_BLEND_FUNC(_src, _dst)`,"]
    #[doc = "      `BGFX_STATE_BLEND_FUNC_SEPARATE(_srcRGB, _dstRGB, _srcA, _dstA)`,"]
    #[doc = "      `BGFX_STATE_BLEND_EQUATION(_equation)`,"]
    #[doc = "      `BGFX_STATE_BLEND_EQUATION_SEPARATE(_equationRGB, _equationA)`"]
    #[doc = "   2. `BGFX_STATE_BLEND_EQUATION_ADD` is set when no other blend"]
    #[doc = "      equation is specified."]
    #[doc = ""]
    #[doc = " @param[in] _state State flags. Default state for primitive type is"]
    #[doc = "    triangles. See: `BGFX_STATE_DEFAULT`."]
    #[doc = "    - `BGFX_STATE_DEPTH_TEST_*` - Depth test function."]
    #[doc = "    - `BGFX_STATE_BLEND_*` - See remark 1 about BGFX_STATE_BLEND_FUNC."]
    #[doc = "    - `BGFX_STATE_BLEND_EQUATION_*` - See remark 2."]
    #[doc = "    - `BGFX_STATE_CULL_*` - Backface culling mode."]
    #[doc = "    - `BGFX_STATE_WRITE_*` - Enable R, G, B, A or Z write."]
    #[doc = "    - `BGFX_STATE_MSAA` - Enable hardware multisample antialiasing."]
    #[doc = "    - `BGFX_STATE_PT_[TRISTRIP/LINES/POINTS]` - Primitive type."]
    #[doc = " @param[in] _rgba Sets blend factor used by `BGFX_STATE_BLEND_FACTOR` and"]
    #[doc = "    `BGFX_STATE_BLEND_INV_FACTOR` blend modes."]
    #[doc = ""]
    pub fn bgfx_encoder_set_state(_this: *mut bgfx_encoder_t, _state: u64, _rgba: u32);
}
extern "C" {
    #[doc = " Set condition for rendering."]
    #[doc = ""]
    #[doc = " @param[in] _handle Occlusion query handle."]
    #[doc = " @param[in] _visible Render if occlusion query is visible."]
    #[doc = ""]
    pub fn bgfx_encoder_set_condition(
        _this: *mut bgfx_encoder_t,
        _handle: bgfx_occlusion_query_handle_t,
        _visible: bool,
    );
}
extern "C" {
    #[doc = " Set stencil test state."]
    #[doc = ""]
    #[doc = " @param[in] _fstencil Front stencil state."]
    #[doc = " @param[in] _bstencil Back stencil state. If back is set to `BGFX_STENCIL_NONE`"]
    #[doc = "  _fstencil is applied to both front and back facing primitives."]
    #[doc = ""]
    pub fn bgfx_encoder_set_stencil(_this: *mut bgfx_encoder_t, _fstencil: u32, _bstencil: u32);
}
extern "C" {
    #[doc = " Set scissor for draw primitive."]
    #[doc = " @remark"]
    #[doc = "   To scissor for all primitives in view see `bgfx::setViewScissor`."]
    #[doc = ""]
    #[doc = " @param[in] _x Position x from the left corner of the window."]
    #[doc = " @param[in] _y Position y from the top corner of the window."]
    #[doc = " @param[in] _width Width of view scissor region."]
    #[doc = " @param[in] _height Height of view scissor region."]
    #[doc = ""]
    #[doc = " @returns Scissor cache index."]
    #[doc = ""]
    pub fn bgfx_encoder_set_scissor(
        _this: *mut bgfx_encoder_t,
        _x: u16,
        _y: u16,
        _width: u16,
        _height: u16,
    ) -> u16;
}
extern "C" {
    #[doc = " Set scissor from cache for draw primitive."]
    #[doc = " @remark"]
    #[doc = "   To scissor for all primitives in view see `bgfx::setViewScissor`."]
    #[doc = ""]
    #[doc = " @param[in] _cache Index in scissor cache."]
    #[doc = ""]
    pub fn bgfx_encoder_set_scissor_cached(_this: *mut bgfx_encoder_t, _cache: u16);
}
extern "C" {
    #[doc = " Set model matrix for draw primitive. If it is not called,"]
    #[doc = " the model will be rendered with an identity model matrix."]
    #[doc = ""]
    #[doc = " @param[in] _mtx Pointer to first matrix in array."]
    #[doc = " @param[in] _num Number of matrices in array."]
    #[doc = ""]
    #[doc = " @returns Index into matrix cache in case the same model matrix has"]
    #[doc = "  to be used for other draw primitive call."]
    #[doc = ""]
    pub fn bgfx_encoder_set_transform(
        _this: *mut bgfx_encoder_t,
        _mtx: *const ::std::os::raw::c_void,
        _num: u16,
    ) -> u32;
}
extern "C" {
    #[doc = "  Set model matrix from matrix cache for draw primitive."]
    #[doc = ""]
    #[doc = " @param[in] _cache Index in matrix cache."]
    #[doc = " @param[in] _num Number of matrices from cache."]
    #[doc = ""]
    pub fn bgfx_encoder_set_transform_cached(_this: *mut bgfx_encoder_t, _cache: u32, _num: u16);
}
extern "C" {
    #[doc = " Reserve matrices in internal matrix cache."]
    #[doc = " @attention Pointer returned can be modifed until `bgfx::frame` is called."]
    #[doc = ""]
    #[doc = " @param[out] _transform Pointer to `Transform` structure."]
    #[doc = " @param[in] _num Number of matrices."]
    #[doc = ""]
    #[doc = " @returns Index in matrix cache."]
    #[doc = ""]
    pub fn bgfx_encoder_alloc_transform(
        _this: *mut bgfx_encoder_t,
        _transform: *mut bgfx_transform_t,
        _num: u16,
    ) -> u32;
}
extern "C" {
    #[doc = " Set shader uniform parameter for draw primitive."]
    #[doc = ""]
    #[doc = " @param[in] _handle Uniform."]
    #[doc = " @param[in] _value Pointer to uniform data."]
    #[doc = " @param[in] _num Number of elements. Passing `UINT16_MAX` will"]
    #[doc = "  use the _num passed on uniform creation."]
    #[doc = ""]
    pub fn bgfx_encoder_set_uniform(
        _this: *mut bgfx_encoder_t,
        _handle: bgfx_uniform_handle_t,
        _value: *const ::std::os::raw::c_void,
        _num: u16,
    );
}
extern "C" {
    #[doc = " Set index buffer for draw primitive."]
    #[doc = ""]
    #[doc = " @param[in] _handle Index buffer."]
    #[doc = " @param[in] _firstIndex First index to render."]
    #[doc = " @param[in] _numIndices Number of indices to render."]
    #[doc = ""]
    pub fn bgfx_encoder_set_index_buffer(
        _this: *mut bgfx_encoder_t,
        _handle: bgfx_index_buffer_handle_t,
        _firstIndex: u32,
        _numIndices: u32,
    );
}
extern "C" {
    #[doc = " Set index buffer for draw primitive."]
    #[doc = ""]
    #[doc = " @param[in] _handle Dynamic index buffer."]
    #[doc = " @param[in] _firstIndex First index to render."]
    #[doc = " @param[in] _numIndices Number of indices to render."]
    #[doc = ""]
    pub fn bgfx_encoder_set_dynamic_index_buffer(
        _this: *mut bgfx_encoder_t,
        _handle: bgfx_dynamic_index_buffer_handle_t,
        _firstIndex: u32,
        _numIndices: u32,
    );
}
extern "C" {
    #[doc = " Set index buffer for draw primitive."]
    #[doc = ""]
    #[doc = " @param[in] _tib Transient index buffer."]
    #[doc = " @param[in] _firstIndex First index to render."]
    #[doc = " @param[in] _numIndices Number of indices to render."]
    #[doc = ""]
    pub fn bgfx_encoder_set_transient_index_buffer(
        _this: *mut bgfx_encoder_t,
        _tib: *const bgfx_transient_index_buffer_t,
        _firstIndex: u32,
        _numIndices: u32,
    );
}
extern "C" {
    #[doc = " Set vertex buffer for draw primitive."]
    #[doc = ""]
    #[doc = " @param[in] _stream Vertex stream."]
    #[doc = " @param[in] _handle Vertex buffer."]
    #[doc = " @param[in] _startVertex First vertex to render."]
    #[doc = " @param[in] _numVertices Number of vertices to render."]
    #[doc = ""]
    pub fn bgfx_encoder_set_vertex_buffer(
        _this: *mut bgfx_encoder_t,
        _stream: u8,
        _handle: bgfx_vertex_buffer_handle_t,
        _startVertex: u32,
        _numVertices: u32,
    );
}
extern "C" {
    #[doc = " Set vertex buffer for draw primitive."]
    #[doc = ""]
    #[doc = " @param[in] _stream Vertex stream."]
    #[doc = " @param[in] _handle Vertex buffer."]
    #[doc = " @param[in] _startVertex First vertex to render."]
    #[doc = " @param[in] _numVertices Number of vertices to render."]
    #[doc = " @param[in] _layoutHandle Vertex layout for aliasing vertex buffer. If invalid"]
    #[doc = "  handle is used, vertex layout used for creation"]
    #[doc = "  of vertex buffer will be used."]
    #[doc = ""]
    pub fn bgfx_encoder_set_vertex_buffer_with_layout(
        _this: *mut bgfx_encoder_t,
        _stream: u8,
        _handle: bgfx_vertex_buffer_handle_t,
        _startVertex: u32,
        _numVertices: u32,
        _layoutHandle: bgfx_vertex_layout_handle_t,
    );
}
extern "C" {
    #[doc = " Set vertex buffer for draw primitive."]
    #[doc = ""]
    #[doc = " @param[in] _stream Vertex stream."]
    #[doc = " @param[in] _handle Dynamic vertex buffer."]
    #[doc = " @param[in] _startVertex First vertex to render."]
    #[doc = " @param[in] _numVertices Number of vertices to render."]
    #[doc = ""]
    pub fn bgfx_encoder_set_dynamic_vertex_buffer(
        _this: *mut bgfx_encoder_t,
        _stream: u8,
        _handle: bgfx_dynamic_vertex_buffer_handle_t,
        _startVertex: u32,
        _numVertices: u32,
    );
}
extern "C" {
    pub fn bgfx_encoder_set_dynamic_vertex_buffer_with_layout(
        _this: *mut bgfx_encoder_t,
        _stream: u8,
        _handle: bgfx_dynamic_vertex_buffer_handle_t,
        _startVertex: u32,
        _numVertices: u32,
        _layoutHandle: bgfx_vertex_layout_handle_t,
    );
}
extern "C" {
    #[doc = " Set vertex buffer for draw primitive."]
    #[doc = ""]
    #[doc = " @param[in] _stream Vertex stream."]
    #[doc = " @param[in] _tvb Transient vertex buffer."]
    #[doc = " @param[in] _startVertex First vertex to render."]
    #[doc = " @param[in] _numVertices Number of vertices to render."]
    #[doc = ""]
    pub fn bgfx_encoder_set_transient_vertex_buffer(
        _this: *mut bgfx_encoder_t,
        _stream: u8,
        _tvb: *const bgfx_transient_vertex_buffer_t,
        _startVertex: u32,
        _numVertices: u32,
    );
}
extern "C" {
    #[doc = " Set vertex buffer for draw primitive."]
    #[doc = ""]
    #[doc = " @param[in] _stream Vertex stream."]
    #[doc = " @param[in] _tvb Transient vertex buffer."]
    #[doc = " @param[in] _startVertex First vertex to render."]
    #[doc = " @param[in] _numVertices Number of vertices to render."]
    #[doc = " @param[in] _layoutHandle Vertex layout for aliasing vertex buffer. If invalid"]
    #[doc = "  handle is used, vertex layout used for creation"]
    #[doc = "  of vertex buffer will be used."]
    #[doc = ""]
    pub fn bgfx_encoder_set_transient_vertex_buffer_with_layout(
        _this: *mut bgfx_encoder_t,
        _stream: u8,
        _tvb: *const bgfx_transient_vertex_buffer_t,
        _startVertex: u32,
        _numVertices: u32,
        _layoutHandle: bgfx_vertex_layout_handle_t,
    );
}
extern "C" {
    #[doc = " Set number of vertices for auto generated vertices use in conjuction"]
    #[doc = " with gl_VertexID."]
    #[doc = " @attention Availability depends on: `BGFX_CAPS_VERTEX_ID`."]
    #[doc = ""]
    #[doc = " @param[in] _numVertices Number of vertices."]
    #[doc = ""]
    pub fn bgfx_encoder_set_vertex_count(_this: *mut bgfx_encoder_t, _numVertices: u32);
}
extern "C" {
    #[doc = " Set instance data buffer for draw primitive."]
    #[doc = ""]
    #[doc = " @param[in] _idb Transient instance data buffer."]
    #[doc = " @param[in] _start First instance data."]
    #[doc = " @param[in] _num Number of data instances."]
    #[doc = ""]
    pub fn bgfx_encoder_set_instance_data_buffer(
        _this: *mut bgfx_encoder_t,
        _idb: *const bgfx_instance_data_buffer_t,
        _start: u32,
        _num: u32,
    );
}
extern "C" {
    #[doc = " Set instance data buffer for draw primitive."]
    #[doc = ""]
    #[doc = " @param[in] _handle Vertex buffer."]
    #[doc = " @param[in] _startVertex First instance data."]
    #[doc = " @param[in] _num Number of data instances."]
    #[doc = "  Set instance data buffer for draw primitive."]
    #[doc = ""]
    pub fn bgfx_encoder_set_instance_data_from_vertex_buffer(
        _this: *mut bgfx_encoder_t,
        _handle: bgfx_vertex_buffer_handle_t,
        _startVertex: u32,
        _num: u32,
    );
}
extern "C" {
    #[doc = " Set instance data buffer for draw primitive."]
    #[doc = ""]
    #[doc = " @param[in] _handle Dynamic vertex buffer."]
    #[doc = " @param[in] _startVertex First instance data."]
    #[doc = " @param[in] _num Number of data instances."]
    #[doc = ""]
    pub fn bgfx_encoder_set_instance_data_from_dynamic_vertex_buffer(
        _this: *mut bgfx_encoder_t,
        _handle: bgfx_dynamic_vertex_buffer_handle_t,
        _startVertex: u32,
        _num: u32,
    );
}
extern "C" {
    #[doc = " Set number of instances for auto generated instances use in conjuction"]
    #[doc = " with gl_InstanceID."]
    #[doc = " @attention Availability depends on: `BGFX_CAPS_VERTEX_ID`."]
    #[doc = ""]
    #[doc = " @param[in] _numInstances"]
    #[doc = ""]
    pub fn bgfx_encoder_set_instance_count(_this: *mut bgfx_encoder_t, _numInstances: u32);
}
extern "C" {
    #[doc = " Set texture stage for draw primitive."]
    #[doc = ""]
    #[doc = " @param[in] _stage Texture unit."]
    #[doc = " @param[in] _sampler Program sampler."]
    #[doc = " @param[in] _handle Texture handle."]
    #[doc = " @param[in] _flags Texture sampling mode. Default value UINT32_MAX uses"]
    #[doc = "    texture sampling settings from the texture."]
    #[doc = "    - `BGFX_SAMPLER_[U/V/W]_[MIRROR/CLAMP]` - Mirror or clamp to edge wrap"]
    #[doc = "      mode."]
    #[doc = "    - `BGFX_SAMPLER_[MIN/MAG/MIP]_[POINT/ANISOTROPIC]` - Point or anisotropic"]
    #[doc = "      sampling."]
    #[doc = ""]
    pub fn bgfx_encoder_set_texture(
        _this: *mut bgfx_encoder_t,
        _stage: u8,
        _sampler: bgfx_uniform_handle_t,
        _handle: bgfx_texture_handle_t,
        _flags: u32,
    );
}
extern "C" {
    #[doc = " Submit an empty primitive for rendering. Uniforms and draw state"]
    #[doc = " will be applied but no geometry will be submitted. Useful in cases"]
    #[doc = " when no other draw/compute primitive is submitted to view, but it's"]
    #[doc = " desired to execute clear view."]
    #[doc = " @remark"]
    #[doc = "   These empty draw calls will sort before ordinary draw calls."]
    #[doc = ""]
    #[doc = " @param[in] _id View id."]
    #[doc = ""]
    pub fn bgfx_encoder_touch(_this: *mut bgfx_encoder_t, _id: bgfx_view_id_t);
}
extern "C" {
    #[doc = " Submit primitive for rendering."]
    #[doc = ""]
    #[doc = " @param[in] _id View id."]
    #[doc = " @param[in] _program Program."]
    #[doc = " @param[in] _depth Depth for sorting."]
    #[doc = " @param[in] _flags Discard or preserve states. See `BGFX_DISCARD_*`."]
    #[doc = ""]
    pub fn bgfx_encoder_submit(
        _this: *mut bgfx_encoder_t,
        _id: bgfx_view_id_t,
        _program: bgfx_program_handle_t,
        _depth: u32,
        _flags: u8,
    );
}
extern "C" {
    #[doc = " Submit primitive with occlusion query for rendering."]
    #[doc = ""]
    #[doc = " @param[in] _id View id."]
    #[doc = " @param[in] _program Program."]
    #[doc = " @param[in] _occlusionQuery Occlusion query."]
    #[doc = " @param[in] _depth Depth for sorting."]
    #[doc = " @param[in] _flags Discard or preserve states. See `BGFX_DISCARD_*`."]
    #[doc = ""]
    pub fn bgfx_encoder_submit_occlusion_query(
        _this: *mut bgfx_encoder_t,
        _id: bgfx_view_id_t,
        _program: bgfx_program_handle_t,
        _occlusionQuery: bgfx_occlusion_query_handle_t,
        _depth: u32,
        _flags: u8,
    );
}
extern "C" {
    #[doc = " Submit primitive for rendering with index and instance data info from"]
    #[doc = " indirect buffer."]
    #[doc = ""]
    #[doc = " @param[in] _id View id."]
    #[doc = " @param[in] _program Program."]
    #[doc = " @param[in] _indirectHandle Indirect buffer."]
    #[doc = " @param[in] _start First element in indirect buffer."]
    #[doc = " @param[in] _num Number of dispatches."]
    #[doc = " @param[in] _depth Depth for sorting."]
    #[doc = " @param[in] _flags Discard or preserve states. See `BGFX_DISCARD_*`."]
    #[doc = ""]
    pub fn bgfx_encoder_submit_indirect(
        _this: *mut bgfx_encoder_t,
        _id: bgfx_view_id_t,
        _program: bgfx_program_handle_t,
        _indirectHandle: bgfx_indirect_buffer_handle_t,
        _start: u16,
        _num: u16,
        _depth: u32,
        _flags: u8,
    );
}
extern "C" {
    #[doc = " Set compute index buffer."]
    #[doc = ""]
    #[doc = " @param[in] _stage Compute stage."]
    #[doc = " @param[in] _handle Index buffer handle."]
    #[doc = " @param[in] _access Buffer access. See `Access::Enum`."]
    #[doc = ""]
    pub fn bgfx_encoder_set_compute_index_buffer(
        _this: *mut bgfx_encoder_t,
        _stage: u8,
        _handle: bgfx_index_buffer_handle_t,
        _access: bgfx_access_t,
    );
}
extern "C" {
    #[doc = " Set compute vertex buffer."]
    #[doc = ""]
    #[doc = " @param[in] _stage Compute stage."]
    #[doc = " @param[in] _handle Vertex buffer handle."]
    #[doc = " @param[in] _access Buffer access. See `Access::Enum`."]
    #[doc = ""]
    pub fn bgfx_encoder_set_compute_vertex_buffer(
        _this: *mut bgfx_encoder_t,
        _stage: u8,
        _handle: bgfx_vertex_buffer_handle_t,
        _access: bgfx_access_t,
    );
}
extern "C" {
    #[doc = " Set compute dynamic index buffer."]
    #[doc = ""]
    #[doc = " @param[in] _stage Compute stage."]
    #[doc = " @param[in] _handle Dynamic index buffer handle."]
    #[doc = " @param[in] _access Buffer access. See `Access::Enum`."]
    #[doc = ""]
    pub fn bgfx_encoder_set_compute_dynamic_index_buffer(
        _this: *mut bgfx_encoder_t,
        _stage: u8,
        _handle: bgfx_dynamic_index_buffer_handle_t,
        _access: bgfx_access_t,
    );
}
extern "C" {
    #[doc = " Set compute dynamic vertex buffer."]
    #[doc = ""]
    #[doc = " @param[in] _stage Compute stage."]
    #[doc = " @param[in] _handle Dynamic vertex buffer handle."]
    #[doc = " @param[in] _access Buffer access. See `Access::Enum`."]
    #[doc = ""]
    pub fn bgfx_encoder_set_compute_dynamic_vertex_buffer(
        _this: *mut bgfx_encoder_t,
        _stage: u8,
        _handle: bgfx_dynamic_vertex_buffer_handle_t,
        _access: bgfx_access_t,
    );
}
extern "C" {
    #[doc = " Set compute indirect buffer."]
    #[doc = ""]
    #[doc = " @param[in] _stage Compute stage."]
    #[doc = " @param[in] _handle Indirect buffer handle."]
    #[doc = " @param[in] _access Buffer access. See `Access::Enum`."]
    #[doc = ""]
    pub fn bgfx_encoder_set_compute_indirect_buffer(
        _this: *mut bgfx_encoder_t,
        _stage: u8,
        _handle: bgfx_indirect_buffer_handle_t,
        _access: bgfx_access_t,
    );
}
extern "C" {
    #[doc = " Set compute image from texture."]
    #[doc = ""]
    #[doc = " @param[in] _stage Compute stage."]
    #[doc = " @param[in] _handle Texture handle."]
    #[doc = " @param[in] _mip Mip level."]
    #[doc = " @param[in] _access Image access. See `Access::Enum`."]
    #[doc = " @param[in] _format Texture format. See: `TextureFormat::Enum`."]
    #[doc = ""]
    pub fn bgfx_encoder_set_image(
        _this: *mut bgfx_encoder_t,
        _stage: u8,
        _handle: bgfx_texture_handle_t,
        _mip: u8,
        _access: bgfx_access_t,
        _format: bgfx_texture_format_t,
    );
}
extern "C" {
    #[doc = " Dispatch compute."]
    #[doc = ""]
    #[doc = " @param[in] _id View id."]
    #[doc = " @param[in] _program Compute program."]
    #[doc = " @param[in] _numX Number of groups X."]
    #[doc = " @param[in] _numY Number of groups Y."]
    #[doc = " @param[in] _numZ Number of groups Z."]
    #[doc = " @param[in] _flags Discard or preserve states. See `BGFX_DISCARD_*`."]
    #[doc = ""]
    pub fn bgfx_encoder_dispatch(
        _this: *mut bgfx_encoder_t,
        _id: bgfx_view_id_t,
        _program: bgfx_program_handle_t,
        _numX: u32,
        _numY: u32,
        _numZ: u32,
        _flags: u8,
    );
}
extern "C" {
    #[doc = " Dispatch compute indirect."]
    #[doc = ""]
    #[doc = " @param[in] _id View id."]
    #[doc = " @param[in] _program Compute program."]
    #[doc = " @param[in] _indirectHandle Indirect buffer."]
    #[doc = " @param[in] _start First element in indirect buffer."]
    #[doc = " @param[in] _num Number of dispatches."]
    #[doc = " @param[in] _flags Discard or preserve states. See `BGFX_DISCARD_*`."]
    #[doc = ""]
    pub fn bgfx_encoder_dispatch_indirect(
        _this: *mut bgfx_encoder_t,
        _id: bgfx_view_id_t,
        _program: bgfx_program_handle_t,
        _indirectHandle: bgfx_indirect_buffer_handle_t,
        _start: u16,
        _num: u16,
        _flags: u8,
    );
}
extern "C" {
    #[doc = " Discard previously set state for draw or compute call."]
    #[doc = ""]
    #[doc = " @param[in] _flags Discard or preserve states. See `BGFX_DISCARD_*`."]
    #[doc = ""]
    pub fn bgfx_encoder_discard(_this: *mut bgfx_encoder_t, _flags: u8);
}
extern "C" {
    #[doc = " Blit 2D texture region between two 2D textures."]
    #[doc = " @attention Destination texture must be created with `BGFX_TEXTURE_BLIT_DST` flag."]
    #[doc = " @attention Availability depends on: `BGFX_CAPS_TEXTURE_BLIT`."]
    #[doc = ""]
    #[doc = " @param[in] _id View id."]
    #[doc = " @param[in] _dst Destination texture handle."]
    #[doc = " @param[in] _dstMip Destination texture mip level."]
    #[doc = " @param[in] _dstX Destination texture X position."]
    #[doc = " @param[in] _dstY Destination texture Y position."]
    #[doc = " @param[in] _dstZ If texture is 2D this argument should be 0. If destination texture is cube"]
    #[doc = "  this argument represents destination texture cube face. For 3D texture this argument"]
    #[doc = "  represents destination texture Z position."]
    #[doc = " @param[in] _src Source texture handle."]
    #[doc = " @param[in] _srcMip Source texture mip level."]
    #[doc = " @param[in] _srcX Source texture X position."]
    #[doc = " @param[in] _srcY Source texture Y position."]
    #[doc = " @param[in] _srcZ If texture is 2D this argument should be 0. If source texture is cube"]
    #[doc = "  this argument represents source texture cube face. For 3D texture this argument"]
    #[doc = "  represents source texture Z position."]
    #[doc = " @param[in] _width Width of region."]
    #[doc = " @param[in] _height Height of region."]
    #[doc = " @param[in] _depth If texture is 3D this argument represents depth of region, otherwise it's"]
    #[doc = "  unused."]
    #[doc = ""]
    pub fn bgfx_encoder_blit(
        _this: *mut bgfx_encoder_t,
        _id: bgfx_view_id_t,
        _dst: bgfx_texture_handle_t,
        _dstMip: u8,
        _dstX: u16,
        _dstY: u16,
        _dstZ: u16,
        _src: bgfx_texture_handle_t,
        _srcMip: u8,
        _srcX: u16,
        _srcY: u16,
        _srcZ: u16,
        _width: u16,
        _height: u16,
        _depth: u16,
    );
}
extern "C" {
    #[doc = " Request screen shot of window back buffer."]
    #[doc = " @remarks"]
    #[doc = "   `bgfx::CallbackI::screenShot` must be implemented."]
    #[doc = " @attention Frame buffer handle must be created with OS' target native window handle."]
    #[doc = ""]
    #[doc = " @param[in] _handle Frame buffer handle. If handle is `BGFX_INVALID_HANDLE` request will be"]
    #[doc = "  made for main window back buffer."]
    #[doc = " @param[in] _filePath Will be passed to `bgfx::CallbackI::screenShot` callback."]
    #[doc = ""]
    pub fn bgfx_request_screen_shot(
        _handle: bgfx_frame_buffer_handle_t,
        _filePath: *const ::std::os::raw::c_char,
    );
}
extern "C" {
    #[doc = " Render frame."]
    #[doc = " @attention `bgfx::renderFrame` is blocking call. It waits for"]
    #[doc = "   `bgfx::frame` to be called from API thread to process frame."]
    #[doc = "   If timeout value is passed call will timeout and return even"]
    #[doc = "   if `bgfx::frame` is not called."]
    #[doc = " @warning This call should be only used on platforms that don't"]
    #[doc = "   allow creating separate rendering thread. If it is called before"]
    #[doc = "   to bgfx::init, render thread won't be created by bgfx::init call."]
    #[doc = ""]
    #[doc = " @param[in] _msecs Timeout in milliseconds."]
    #[doc = ""]
    #[doc = " @returns Current renderer context state. See: `bgfx::RenderFrame`."]
    #[doc = ""]
    pub fn bgfx_render_frame(_msecs: i32) -> bgfx_render_frame_t;
}
extern "C" {
    #[doc = " Set platform data."]
    #[doc = " @warning Must be called before `bgfx::init`."]
    #[doc = ""]
    #[doc = " @param[in] _data Platform data."]
    #[doc = ""]
    pub fn bgfx_set_platform_data(_data: *const bgfx_platform_data_t);
}
extern "C" {
    #[doc = " Get internal data for interop."]
    #[doc = " @attention It's expected you understand some bgfx internals before you"]
    #[doc = "   use this call."]
    #[doc = " @warning Must be called only on render thread."]
    #[doc = ""]
    pub fn bgfx_get_internal_data() -> *const bgfx_internal_data_t;
}
extern "C" {
    #[doc = " Override internal texture with externally created texture. Previously"]
    #[doc = " created internal texture will released."]
    #[doc = " @attention It's expected you understand some bgfx internals before you"]
    #[doc = "   use this call."]
    #[doc = " @warning Must be called only on render thread."]
    #[doc = ""]
    #[doc = " @param[in] _handle Texture handle."]
    #[doc = " @param[in] _ptr Native API pointer to texture."]
    #[doc = ""]
    #[doc = " @returns Native API pointer to texture. If result is 0, texture is not created"]
    #[doc = "  yet from the main thread."]
    #[doc = ""]
    pub fn bgfx_override_internal_texture_ptr(_handle: bgfx_texture_handle_t, _ptr: usize)
        -> usize;
}
extern "C" {
    #[doc = " Override internal texture by creating new texture. Previously created"]
    #[doc = " internal texture will released."]
    #[doc = " @attention It's expected you understand some bgfx internals before you"]
    #[doc = "   use this call."]
    #[doc = " @returns Native API pointer to texture. If result is 0, texture is not created yet from the"]
    #[doc = "   main thread."]
    #[doc = " @warning Must be called only on render thread."]
    #[doc = ""]
    #[doc = " @param[in] _handle Texture handle."]
    #[doc = " @param[in] _width Width."]
    #[doc = " @param[in] _height Height."]
    #[doc = " @param[in] _numMips Number of mip-maps."]
    #[doc = " @param[in] _format Texture format. See: `TextureFormat::Enum`."]
    #[doc = " @param[in] _flags Texture creation (see `BGFX_TEXTURE_*`.), and sampler (see `BGFX_SAMPLER_*`)"]
    #[doc = "  flags. Default texture sampling mode is linear, and wrap mode is repeat."]
    #[doc = "  - `BGFX_SAMPLER_[U/V/W]_[MIRROR/CLAMP]` - Mirror or clamp to edge wrap"]
    #[doc = "    mode."]
    #[doc = "  - `BGFX_SAMPLER_[MIN/MAG/MIP]_[POINT/ANISOTROPIC]` - Point or anisotropic"]
    #[doc = "    sampling."]
    #[doc = ""]
    #[doc = " @returns Native API pointer to texture. If result is 0, texture is not created"]
    #[doc = "  yet from the main thread."]
    #[doc = ""]
    pub fn bgfx_override_internal_texture(
        _handle: bgfx_texture_handle_t,
        _width: u16,
        _height: u16,
        _numMips: u8,
        _format: bgfx_texture_format_t,
        _flags: u64,
    ) -> usize;
}
extern "C" {
    #[doc = " Sets a debug marker. This allows you to group graphics calls together for easy browsing in"]
    #[doc = " graphics debugging tools."]
    #[doc = ""]
    #[doc = " @param[in] _marker Marker string."]
    #[doc = ""]
    pub fn bgfx_set_marker(_marker: *const ::std::os::raw::c_char);
}
extern "C" {
    #[doc = " Set render states for draw primitive."]
    #[doc = " @remarks"]
    #[doc = "   1. To setup more complex states use:"]
    #[doc = "      `BGFX_STATE_ALPHA_REF(_ref)`,"]
    #[doc = "      `BGFX_STATE_POINT_SIZE(_size)`,"]
    #[doc = "      `BGFX_STATE_BLEND_FUNC(_src, _dst)`,"]
    #[doc = "      `BGFX_STATE_BLEND_FUNC_SEPARATE(_srcRGB, _dstRGB, _srcA, _dstA)`,"]
    #[doc = "      `BGFX_STATE_BLEND_EQUATION(_equation)`,"]
    #[doc = "      `BGFX_STATE_BLEND_EQUATION_SEPARATE(_equationRGB, _equationA)`"]
    #[doc = "   2. `BGFX_STATE_BLEND_EQUATION_ADD` is set when no other blend"]
    #[doc = "      equation is specified."]
    #[doc = ""]
    #[doc = " @param[in] _state State flags. Default state for primitive type is"]
    #[doc = "    triangles. See: `BGFX_STATE_DEFAULT`."]
    #[doc = "    - `BGFX_STATE_DEPTH_TEST_*` - Depth test function."]
    #[doc = "    - `BGFX_STATE_BLEND_*` - See remark 1 about BGFX_STATE_BLEND_FUNC."]
    #[doc = "    - `BGFX_STATE_BLEND_EQUATION_*` - See remark 2."]
    #[doc = "    - `BGFX_STATE_CULL_*` - Backface culling mode."]
    #[doc = "    - `BGFX_STATE_WRITE_*` - Enable R, G, B, A or Z write."]
    #[doc = "    - `BGFX_STATE_MSAA` - Enable hardware multisample antialiasing."]
    #[doc = "    - `BGFX_STATE_PT_[TRISTRIP/LINES/POINTS]` - Primitive type."]
    #[doc = " @param[in] _rgba Sets blend factor used by `BGFX_STATE_BLEND_FACTOR` and"]
    #[doc = "    `BGFX_STATE_BLEND_INV_FACTOR` blend modes."]
    #[doc = ""]
    pub fn bgfx_set_state(_state: u64, _rgba: u32);
}
extern "C" {
    #[doc = " Set condition for rendering."]
    #[doc = ""]
    #[doc = " @param[in] _handle Occlusion query handle."]
    #[doc = " @param[in] _visible Render if occlusion query is visible."]
    #[doc = ""]
    pub fn bgfx_set_condition(_handle: bgfx_occlusion_query_handle_t, _visible: bool);
}
extern "C" {
    #[doc = " Set stencil test state."]
    #[doc = ""]
    #[doc = " @param[in] _fstencil Front stencil state."]
    #[doc = " @param[in] _bstencil Back stencil state. If back is set to `BGFX_STENCIL_NONE`"]
    #[doc = "  _fstencil is applied to both front and back facing primitives."]
    #[doc = ""]
    pub fn bgfx_set_stencil(_fstencil: u32, _bstencil: u32);
}
extern "C" {
    #[doc = " Set scissor for draw primitive."]
    #[doc = " @remark"]
    #[doc = "   To scissor for all primitives in view see `bgfx::setViewScissor`."]
    #[doc = ""]
    #[doc = " @param[in] _x Position x from the left corner of the window."]
    #[doc = " @param[in] _y Position y from the top corner of the window."]
    #[doc = " @param[in] _width Width of view scissor region."]
    #[doc = " @param[in] _height Height of view scissor region."]
    #[doc = ""]
    #[doc = " @returns Scissor cache index."]
    #[doc = ""]
    pub fn bgfx_set_scissor(_x: u16, _y: u16, _width: u16, _height: u16) -> u16;
}
extern "C" {
    #[doc = " Set scissor from cache for draw primitive."]
    #[doc = " @remark"]
    #[doc = "   To scissor for all primitives in view see `bgfx::setViewScissor`."]
    #[doc = ""]
    #[doc = " @param[in] _cache Index in scissor cache."]
    #[doc = ""]
    pub fn bgfx_set_scissor_cached(_cache: u16);
}
extern "C" {
    #[doc = " Set model matrix for draw primitive. If it is not called,"]
    #[doc = " the model will be rendered with an identity model matrix."]
    #[doc = ""]
    #[doc = " @param[in] _mtx Pointer to first matrix in array."]
    #[doc = " @param[in] _num Number of matrices in array."]
    #[doc = ""]
    #[doc = " @returns Index into matrix cache in case the same model matrix has"]
    #[doc = "  to be used for other draw primitive call."]
    #[doc = ""]
    pub fn bgfx_set_transform(_mtx: *const ::std::os::raw::c_void, _num: u16) -> u32;
}
extern "C" {
    #[doc = "  Set model matrix from matrix cache for draw primitive."]
    #[doc = ""]
    #[doc = " @param[in] _cache Index in matrix cache."]
    #[doc = " @param[in] _num Number of matrices from cache."]
    #[doc = ""]
    pub fn bgfx_set_transform_cached(_cache: u32, _num: u16);
}
extern "C" {
    #[doc = " Reserve matrices in internal matrix cache."]
    #[doc = " @attention Pointer returned can be modifed until `bgfx::frame` is called."]
    #[doc = ""]
    #[doc = " @param[out] _transform Pointer to `Transform` structure."]
    #[doc = " @param[in] _num Number of matrices."]
    #[doc = ""]
    #[doc = " @returns Index in matrix cache."]
    #[doc = ""]
    pub fn bgfx_alloc_transform(_transform: *mut bgfx_transform_t, _num: u16) -> u32;
}
extern "C" {
    #[doc = " Set shader uniform parameter for draw primitive."]
    #[doc = ""]
    #[doc = " @param[in] _handle Uniform."]
    #[doc = " @param[in] _value Pointer to uniform data."]
    #[doc = " @param[in] _num Number of elements. Passing `UINT16_MAX` will"]
    #[doc = "  use the _num passed on uniform creation."]
    #[doc = ""]
    pub fn bgfx_set_uniform(
        _handle: bgfx_uniform_handle_t,
        _value: *const ::std::os::raw::c_void,
        _num: u16,
    );
}
extern "C" {
    #[doc = " Set index buffer for draw primitive."]
    #[doc = ""]
    #[doc = " @param[in] _handle Index buffer."]
    #[doc = " @param[in] _firstIndex First index to render."]
    #[doc = " @param[in] _numIndices Number of indices to render."]
    #[doc = ""]
    pub fn bgfx_set_index_buffer(
        _handle: bgfx_index_buffer_handle_t,
        _firstIndex: u32,
        _numIndices: u32,
    );
}
extern "C" {
    #[doc = " Set index buffer for draw primitive."]
    #[doc = ""]
    #[doc = " @param[in] _handle Dynamic index buffer."]
    #[doc = " @param[in] _firstIndex First index to render."]
    #[doc = " @param[in] _numIndices Number of indices to render."]
    #[doc = ""]
    pub fn bgfx_set_dynamic_index_buffer(
        _handle: bgfx_dynamic_index_buffer_handle_t,
        _firstIndex: u32,
        _numIndices: u32,
    );
}
extern "C" {
    #[doc = " Set index buffer for draw primitive."]
    #[doc = ""]
    #[doc = " @param[in] _tib Transient index buffer."]
    #[doc = " @param[in] _firstIndex First index to render."]
    #[doc = " @param[in] _numIndices Number of indices to render."]
    #[doc = ""]
    pub fn bgfx_set_transient_index_buffer(
        _tib: *const bgfx_transient_index_buffer_t,
        _firstIndex: u32,
        _numIndices: u32,
    );
}
extern "C" {
    #[doc = " Set vertex buffer for draw primitive."]
    #[doc = ""]
    #[doc = " @param[in] _stream Vertex stream."]
    #[doc = " @param[in] _handle Vertex buffer."]
    #[doc = " @param[in] _startVertex First vertex to render."]
    #[doc = " @param[in] _numVertices Number of vertices to render."]
    #[doc = ""]
    pub fn bgfx_set_vertex_buffer(
        _stream: u8,
        _handle: bgfx_vertex_buffer_handle_t,
        _startVertex: u32,
        _numVertices: u32,
    );
}
extern "C" {
    #[doc = " Set vertex buffer for draw primitive."]
    #[doc = ""]
    #[doc = " @param[in] _stream Vertex stream."]
    #[doc = " @param[in] _handle Vertex buffer."]
    #[doc = " @param[in] _startVertex First vertex to render."]
    #[doc = " @param[in] _numVertices Number of vertices to render."]
    #[doc = " @param[in] _layoutHandle Vertex layout for aliasing vertex buffer. If invalid"]
    #[doc = "  handle is used, vertex layout used for creation"]
    #[doc = "  of vertex buffer will be used."]
    #[doc = ""]
    pub fn bgfx_set_vertex_buffer_with_layout(
        _stream: u8,
        _handle: bgfx_vertex_buffer_handle_t,
        _startVertex: u32,
        _numVertices: u32,
        _layoutHandle: bgfx_vertex_layout_handle_t,
    );
}
extern "C" {
    #[doc = " Set vertex buffer for draw primitive."]
    #[doc = ""]
    #[doc = " @param[in] _stream Vertex stream."]
    #[doc = " @param[in] _handle Dynamic vertex buffer."]
    #[doc = " @param[in] _startVertex First vertex to render."]
    #[doc = " @param[in] _numVertices Number of vertices to render."]
    #[doc = ""]
    pub fn bgfx_set_dynamic_vertex_buffer(
        _stream: u8,
        _handle: bgfx_dynamic_vertex_buffer_handle_t,
        _startVertex: u32,
        _numVertices: u32,
    );
}
extern "C" {
    #[doc = " Set vertex buffer for draw primitive."]
    #[doc = ""]
    #[doc = " @param[in] _stream Vertex stream."]
    #[doc = " @param[in] _handle Dynamic vertex buffer."]
    #[doc = " @param[in] _startVertex First vertex to render."]
    #[doc = " @param[in] _numVertices Number of vertices to render."]
    #[doc = " @param[in] _layoutHandle Vertex layout for aliasing vertex buffer. If invalid"]
    #[doc = "  handle is used, vertex layout used for creation"]
    #[doc = "  of vertex buffer will be used."]
    #[doc = ""]
    pub fn bgfx_set_dynamic_vertex_buffer_with_layout(
        _stream: u8,
        _handle: bgfx_dynamic_vertex_buffer_handle_t,
        _startVertex: u32,
        _numVertices: u32,
        _layoutHandle: bgfx_vertex_layout_handle_t,
    );
}
extern "C" {
    #[doc = " Set vertex buffer for draw primitive."]
    #[doc = ""]
    #[doc = " @param[in] _stream Vertex stream."]
    #[doc = " @param[in] _tvb Transient vertex buffer."]
    #[doc = " @param[in] _startVertex First vertex to render."]
    #[doc = " @param[in] _numVertices Number of vertices to render."]
    #[doc = ""]
    pub fn bgfx_set_transient_vertex_buffer(
        _stream: u8,
        _tvb: *const bgfx_transient_vertex_buffer_t,
        _startVertex: u32,
        _numVertices: u32,
    );
}
extern "C" {
    #[doc = " Set vertex buffer for draw primitive."]
    #[doc = ""]
    #[doc = " @param[in] _stream Vertex stream."]
    #[doc = " @param[in] _tvb Transient vertex buffer."]
    #[doc = " @param[in] _startVertex First vertex to render."]
    #[doc = " @param[in] _numVertices Number of vertices to render."]
    #[doc = " @param[in] _layoutHandle Vertex layout for aliasing vertex buffer. If invalid"]
    #[doc = "  handle is used, vertex layout used for creation"]
    #[doc = "  of vertex buffer will be used."]
    #[doc = ""]
    pub fn bgfx_set_transient_vertex_buffer_with_layout(
        _stream: u8,
        _tvb: *const bgfx_transient_vertex_buffer_t,
        _startVertex: u32,
        _numVertices: u32,
        _layoutHandle: bgfx_vertex_layout_handle_t,
    );
}
extern "C" {
    #[doc = " Set number of vertices for auto generated vertices use in conjuction"]
    #[doc = " with gl_VertexID."]
    #[doc = " @attention Availability depends on: `BGFX_CAPS_VERTEX_ID`."]
    #[doc = ""]
    #[doc = " @param[in] _numVertices Number of vertices."]
    #[doc = ""]
    pub fn bgfx_set_vertex_count(_numVertices: u32);
}
extern "C" {
    #[doc = " Set instance data buffer for draw primitive."]
    #[doc = ""]
    #[doc = " @param[in] _idb Transient instance data buffer."]
    #[doc = " @param[in] _start First instance data."]
    #[doc = " @param[in] _num Number of data instances."]
    #[doc = ""]
    pub fn bgfx_set_instance_data_buffer(
        _idb: *const bgfx_instance_data_buffer_t,
        _start: u32,
        _num: u32,
    );
}
extern "C" {
    #[doc = " Set instance data buffer for draw primitive."]
    #[doc = ""]
    #[doc = " @param[in] _handle Vertex buffer."]
    #[doc = " @param[in] _startVertex First instance data."]
    #[doc = " @param[in] _num Number of data instances."]
    #[doc = "  Set instance data buffer for draw primitive."]
    #[doc = ""]
    pub fn bgfx_set_instance_data_from_vertex_buffer(
        _handle: bgfx_vertex_buffer_handle_t,
        _startVertex: u32,
        _num: u32,
    );
}
extern "C" {
    #[doc = " Set instance data buffer for draw primitive."]
    #[doc = ""]
    #[doc = " @param[in] _handle Dynamic vertex buffer."]
    #[doc = " @param[in] _startVertex First instance data."]
    #[doc = " @param[in] _num Number of data instances."]
    #[doc = ""]
    pub fn bgfx_set_instance_data_from_dynamic_vertex_buffer(
        _handle: bgfx_dynamic_vertex_buffer_handle_t,
        _startVertex: u32,
        _num: u32,
    );
}
extern "C" {
    #[doc = " Set number of instances for auto generated instances use in conjuction"]
    #[doc = " with gl_InstanceID."]
    #[doc = " @attention Availability depends on: `BGFX_CAPS_VERTEX_ID`."]
    #[doc = ""]
    #[doc = " @param[in] _numInstances"]
    #[doc = ""]
    pub fn bgfx_set_instance_count(_numInstances: u32);
}
extern "C" {
    #[doc = " Set texture stage for draw primitive."]
    #[doc = ""]
    #[doc = " @param[in] _stage Texture unit."]
    #[doc = " @param[in] _sampler Program sampler."]
    #[doc = " @param[in] _handle Texture handle."]
    #[doc = " @param[in] _flags Texture sampling mode. Default value UINT32_MAX uses"]
    #[doc = "    texture sampling settings from the texture."]
    #[doc = "    - `BGFX_SAMPLER_[U/V/W]_[MIRROR/CLAMP]` - Mirror or clamp to edge wrap"]
    #[doc = "      mode."]
    #[doc = "    - `BGFX_SAMPLER_[MIN/MAG/MIP]_[POINT/ANISOTROPIC]` - Point or anisotropic"]
    #[doc = "      sampling."]
    #[doc = ""]
    pub fn bgfx_set_texture(
        _stage: u8,
        _sampler: bgfx_uniform_handle_t,
        _handle: bgfx_texture_handle_t,
        _flags: u32,
    );
}
extern "C" {
    #[doc = " Submit an empty primitive for rendering. Uniforms and draw state"]
    #[doc = " will be applied but no geometry will be submitted."]
    #[doc = " @remark"]
    #[doc = "   These empty draw calls will sort before ordinary draw calls."]
    #[doc = ""]
    #[doc = " @param[in] _id View id."]
    #[doc = ""]
    pub fn bgfx_touch(_id: bgfx_view_id_t);
}
extern "C" {
    #[doc = " Submit primitive for rendering."]
    #[doc = ""]
    #[doc = " @param[in] _id View id."]
    #[doc = " @param[in] _program Program."]
    #[doc = " @param[in] _depth Depth for sorting."]
    #[doc = " @param[in] _flags Which states to discard for next draw. See `BGFX_DISCARD_*`."]
    #[doc = ""]
    pub fn bgfx_submit(
        _id: bgfx_view_id_t,
        _program: bgfx_program_handle_t,
        _depth: u32,
        _flags: u8,
    );
}
extern "C" {
    #[doc = " Submit primitive with occlusion query for rendering."]
    #[doc = ""]
    #[doc = " @param[in] _id View id."]
    #[doc = " @param[in] _program Program."]
    #[doc = " @param[in] _occlusionQuery Occlusion query."]
    #[doc = " @param[in] _depth Depth for sorting."]
    #[doc = " @param[in] _flags Which states to discard for next draw. See `BGFX_DISCARD_*`."]
    #[doc = ""]
    pub fn bgfx_submit_occlusion_query(
        _id: bgfx_view_id_t,
        _program: bgfx_program_handle_t,
        _occlusionQuery: bgfx_occlusion_query_handle_t,
        _depth: u32,
        _flags: u8,
    );
}
extern "C" {
    #[doc = " Submit primitive for rendering with index and instance data info from"]
    #[doc = " indirect buffer."]
    #[doc = ""]
    #[doc = " @param[in] _id View id."]
    #[doc = " @param[in] _program Program."]
    #[doc = " @param[in] _indirectHandle Indirect buffer."]
    #[doc = " @param[in] _start First element in indirect buffer."]
    #[doc = " @param[in] _num Number of dispatches."]
    #[doc = " @param[in] _depth Depth for sorting."]
    #[doc = " @param[in] _flags Which states to discard for next draw. See `BGFX_DISCARD_*`."]
    #[doc = ""]
    pub fn bgfx_submit_indirect(
        _id: bgfx_view_id_t,
        _program: bgfx_program_handle_t,
        _indirectHandle: bgfx_indirect_buffer_handle_t,
        _start: u16,
        _num: u16,
        _depth: u32,
        _flags: u8,
    );
}
extern "C" {
    #[doc = " Set compute index buffer."]
    #[doc = ""]
    #[doc = " @param[in] _stage Compute stage."]
    #[doc = " @param[in] _handle Index buffer handle."]
    #[doc = " @param[in] _access Buffer access. See `Access::Enum`."]
    #[doc = ""]
    pub fn bgfx_set_compute_index_buffer(
        _stage: u8,
        _handle: bgfx_index_buffer_handle_t,
        _access: bgfx_access_t,
    );
}
extern "C" {
    #[doc = " Set compute vertex buffer."]
    #[doc = ""]
    #[doc = " @param[in] _stage Compute stage."]
    #[doc = " @param[in] _handle Vertex buffer handle."]
    #[doc = " @param[in] _access Buffer access. See `Access::Enum`."]
    #[doc = ""]
    pub fn bgfx_set_compute_vertex_buffer(
        _stage: u8,
        _handle: bgfx_vertex_buffer_handle_t,
        _access: bgfx_access_t,
    );
}
extern "C" {
    #[doc = " Set compute dynamic index buffer."]
    #[doc = ""]
    #[doc = " @param[in] _stage Compute stage."]
    #[doc = " @param[in] _handle Dynamic index buffer handle."]
    #[doc = " @param[in] _access Buffer access. See `Access::Enum`."]
    #[doc = ""]
    pub fn bgfx_set_compute_dynamic_index_buffer(
        _stage: u8,
        _handle: bgfx_dynamic_index_buffer_handle_t,
        _access: bgfx_access_t,
    );
}
extern "C" {
    #[doc = " Set compute dynamic vertex buffer."]
    #[doc = ""]
    #[doc = " @param[in] _stage Compute stage."]
    #[doc = " @param[in] _handle Dynamic vertex buffer handle."]
    #[doc = " @param[in] _access Buffer access. See `Access::Enum`."]
    #[doc = ""]
    pub fn bgfx_set_compute_dynamic_vertex_buffer(
        _stage: u8,
        _handle: bgfx_dynamic_vertex_buffer_handle_t,
        _access: bgfx_access_t,
    );
}
extern "C" {
    #[doc = " Set compute indirect buffer."]
    #[doc = ""]
    #[doc = " @param[in] _stage Compute stage."]
    #[doc = " @param[in] _handle Indirect buffer handle."]
    #[doc = " @param[in] _access Buffer access. See `Access::Enum`."]
    #[doc = ""]
    pub fn bgfx_set_compute_indirect_buffer(
        _stage: u8,
        _handle: bgfx_indirect_buffer_handle_t,
        _access: bgfx_access_t,
    );
}
extern "C" {
    #[doc = " Set compute image from texture."]
    #[doc = ""]
    #[doc = " @param[in] _stage Compute stage."]
    #[doc = " @param[in] _handle Texture handle."]
    #[doc = " @param[in] _mip Mip level."]
    #[doc = " @param[in] _access Image access. See `Access::Enum`."]
    #[doc = " @param[in] _format Texture format. See: `TextureFormat::Enum`."]
    #[doc = ""]
    pub fn bgfx_set_image(
        _stage: u8,
        _handle: bgfx_texture_handle_t,
        _mip: u8,
        _access: bgfx_access_t,
        _format: bgfx_texture_format_t,
    );
}
extern "C" {
    #[doc = " Dispatch compute."]
    #[doc = ""]
    #[doc = " @param[in] _id View id."]
    #[doc = " @param[in] _program Compute program."]
    #[doc = " @param[in] _numX Number of groups X."]
    #[doc = " @param[in] _numY Number of groups Y."]
    #[doc = " @param[in] _numZ Number of groups Z."]
    #[doc = " @param[in] _flags Discard or preserve states. See `BGFX_DISCARD_*`."]
    #[doc = ""]
    pub fn bgfx_dispatch(
        _id: bgfx_view_id_t,
        _program: bgfx_program_handle_t,
        _numX: u32,
        _numY: u32,
        _numZ: u32,
        _flags: u8,
    );
}
extern "C" {
    #[doc = " Dispatch compute indirect."]
    #[doc = ""]
    #[doc = " @param[in] _id View id."]
    #[doc = " @param[in] _program Compute program."]
    #[doc = " @param[in] _indirectHandle Indirect buffer."]
    #[doc = " @param[in] _start First element in indirect buffer."]
    #[doc = " @param[in] _num Number of dispatches."]
    #[doc = " @param[in] _flags Discard or preserve states. See `BGFX_DISCARD_*`."]
    #[doc = ""]
    pub fn bgfx_dispatch_indirect(
        _id: bgfx_view_id_t,
        _program: bgfx_program_handle_t,
        _indirectHandle: bgfx_indirect_buffer_handle_t,
        _start: u16,
        _num: u16,
        _flags: u8,
    );
}
extern "C" {
    #[doc = " Discard previously set state for draw or compute call."]
    #[doc = ""]
    #[doc = " @param[in] _flags Draw/compute states to discard."]
    #[doc = ""]
    pub fn bgfx_discard(_flags: u8);
}
extern "C" {
    #[doc = " Blit 2D texture region between two 2D textures."]
    #[doc = " @attention Destination texture must be created with `BGFX_TEXTURE_BLIT_DST` flag."]
    #[doc = " @attention Availability depends on: `BGFX_CAPS_TEXTURE_BLIT`."]
    #[doc = ""]
    #[doc = " @param[in] _id View id."]
    #[doc = " @param[in] _dst Destination texture handle."]
    #[doc = " @param[in] _dstMip Destination texture mip level."]
    #[doc = " @param[in] _dstX Destination texture X position."]
    #[doc = " @param[in] _dstY Destination texture Y position."]
    #[doc = " @param[in] _dstZ If texture is 2D this argument should be 0. If destination texture is cube"]
    #[doc = "  this argument represents destination texture cube face. For 3D texture this argument"]
    #[doc = "  represents destination texture Z position."]
    #[doc = " @param[in] _src Source texture handle."]
    #[doc = " @param[in] _srcMip Source texture mip level."]
    #[doc = " @param[in] _srcX Source texture X position."]
    #[doc = " @param[in] _srcY Source texture Y position."]
    #[doc = " @param[in] _srcZ If texture is 2D this argument should be 0. If source texture is cube"]
    #[doc = "  this argument represents source texture cube face. For 3D texture this argument"]
    #[doc = "  represents source texture Z position."]
    #[doc = " @param[in] _width Width of region."]
    #[doc = " @param[in] _height Height of region."]
    #[doc = " @param[in] _depth If texture is 3D this argument represents depth of region, otherwise it's"]
    #[doc = "  unused."]
    #[doc = ""]
    pub fn bgfx_blit(
        _id: bgfx_view_id_t,
        _dst: bgfx_texture_handle_t,
        _dstMip: u8,
        _dstX: u16,
        _dstY: u16,
        _dstZ: u16,
        _src: bgfx_texture_handle_t,
        _srcMip: u8,
        _srcX: u16,
        _srcY: u16,
        _srcZ: u16,
        _width: u16,
        _height: u16,
        _depth: u16,
    );
}
pub const BGFX_FUNCTION_ID_ATTACHMENT_INIT: bgfx_function_id = 0;
pub const BGFX_FUNCTION_ID_VERTEX_LAYOUT_BEGIN: bgfx_function_id = 1;
pub const BGFX_FUNCTION_ID_VERTEX_LAYOUT_ADD: bgfx_function_id = 2;
pub const BGFX_FUNCTION_ID_VERTEX_LAYOUT_DECODE: bgfx_function_id = 3;
pub const BGFX_FUNCTION_ID_VERTEX_LAYOUT_HAS: bgfx_function_id = 4;
pub const BGFX_FUNCTION_ID_VERTEX_LAYOUT_SKIP: bgfx_function_id = 5;
pub const BGFX_FUNCTION_ID_VERTEX_LAYOUT_END: bgfx_function_id = 6;
pub const BGFX_FUNCTION_ID_VERTEX_PACK: bgfx_function_id = 7;
pub const BGFX_FUNCTION_ID_VERTEX_UNPACK: bgfx_function_id = 8;
pub const BGFX_FUNCTION_ID_VERTEX_CONVERT: bgfx_function_id = 9;
pub const BGFX_FUNCTION_ID_WELD_VERTICES: bgfx_function_id = 10;
pub const BGFX_FUNCTION_ID_TOPOLOGY_CONVERT: bgfx_function_id = 11;
pub const BGFX_FUNCTION_ID_TOPOLOGY_SORT_TRI_LIST: bgfx_function_id = 12;
pub const BGFX_FUNCTION_ID_GET_SUPPORTED_RENDERERS: bgfx_function_id = 13;
pub const BGFX_FUNCTION_ID_GET_RENDERER_NAME: bgfx_function_id = 14;
pub const BGFX_FUNCTION_ID_INIT_CTOR: bgfx_function_id = 15;
pub const BGFX_FUNCTION_ID_INIT: bgfx_function_id = 16;
pub const BGFX_FUNCTION_ID_SHUTDOWN: bgfx_function_id = 17;
pub const BGFX_FUNCTION_ID_RESET: bgfx_function_id = 18;
pub const BGFX_FUNCTION_ID_FRAME: bgfx_function_id = 19;
pub const BGFX_FUNCTION_ID_GET_RENDERER_TYPE: bgfx_function_id = 20;
pub const BGFX_FUNCTION_ID_GET_CAPS: bgfx_function_id = 21;
pub const BGFX_FUNCTION_ID_GET_STATS: bgfx_function_id = 22;
pub const BGFX_FUNCTION_ID_ALLOC: bgfx_function_id = 23;
pub const BGFX_FUNCTION_ID_COPY: bgfx_function_id = 24;
pub const BGFX_FUNCTION_ID_MAKE_REF: bgfx_function_id = 25;
pub const BGFX_FUNCTION_ID_MAKE_REF_RELEASE: bgfx_function_id = 26;
pub const BGFX_FUNCTION_ID_SET_DEBUG: bgfx_function_id = 27;
pub const BGFX_FUNCTION_ID_DBG_TEXT_CLEAR: bgfx_function_id = 28;
pub const BGFX_FUNCTION_ID_DBG_TEXT_PRINTF: bgfx_function_id = 29;
pub const BGFX_FUNCTION_ID_DBG_TEXT_VPRINTF: bgfx_function_id = 30;
pub const BGFX_FUNCTION_ID_DBG_TEXT_IMAGE: bgfx_function_id = 31;
pub const BGFX_FUNCTION_ID_CREATE_INDEX_BUFFER: bgfx_function_id = 32;
pub const BGFX_FUNCTION_ID_SET_INDEX_BUFFER_NAME: bgfx_function_id = 33;
pub const BGFX_FUNCTION_ID_DESTROY_INDEX_BUFFER: bgfx_function_id = 34;
pub const BGFX_FUNCTION_ID_CREATE_VERTEX_LAYOUT: bgfx_function_id = 35;
pub const BGFX_FUNCTION_ID_DESTROY_VERTEX_LAYOUT: bgfx_function_id = 36;
pub const BGFX_FUNCTION_ID_CREATE_VERTEX_BUFFER: bgfx_function_id = 37;
pub const BGFX_FUNCTION_ID_SET_VERTEX_BUFFER_NAME: bgfx_function_id = 38;
pub const BGFX_FUNCTION_ID_DESTROY_VERTEX_BUFFER: bgfx_function_id = 39;
pub const BGFX_FUNCTION_ID_CREATE_DYNAMIC_INDEX_BUFFER: bgfx_function_id = 40;
pub const BGFX_FUNCTION_ID_CREATE_DYNAMIC_INDEX_BUFFER_MEM: bgfx_function_id = 41;
pub const BGFX_FUNCTION_ID_UPDATE_DYNAMIC_INDEX_BUFFER: bgfx_function_id = 42;
pub const BGFX_FUNCTION_ID_DESTROY_DYNAMIC_INDEX_BUFFER: bgfx_function_id = 43;
pub const BGFX_FUNCTION_ID_CREATE_DYNAMIC_VERTEX_BUFFER: bgfx_function_id = 44;
pub const BGFX_FUNCTION_ID_CREATE_DYNAMIC_VERTEX_BUFFER_MEM: bgfx_function_id = 45;
pub const BGFX_FUNCTION_ID_UPDATE_DYNAMIC_VERTEX_BUFFER: bgfx_function_id = 46;
pub const BGFX_FUNCTION_ID_DESTROY_DYNAMIC_VERTEX_BUFFER: bgfx_function_id = 47;
pub const BGFX_FUNCTION_ID_GET_AVAIL_TRANSIENT_INDEX_BUFFER: bgfx_function_id = 48;
pub const BGFX_FUNCTION_ID_GET_AVAIL_TRANSIENT_VERTEX_BUFFER: bgfx_function_id = 49;
pub const BGFX_FUNCTION_ID_GET_AVAIL_INSTANCE_DATA_BUFFER: bgfx_function_id = 50;
pub const BGFX_FUNCTION_ID_ALLOC_TRANSIENT_INDEX_BUFFER: bgfx_function_id = 51;
pub const BGFX_FUNCTION_ID_ALLOC_TRANSIENT_VERTEX_BUFFER: bgfx_function_id = 52;
pub const BGFX_FUNCTION_ID_ALLOC_TRANSIENT_BUFFERS: bgfx_function_id = 53;
pub const BGFX_FUNCTION_ID_ALLOC_INSTANCE_DATA_BUFFER: bgfx_function_id = 54;
pub const BGFX_FUNCTION_ID_CREATE_INDIRECT_BUFFER: bgfx_function_id = 55;
pub const BGFX_FUNCTION_ID_DESTROY_INDIRECT_BUFFER: bgfx_function_id = 56;
pub const BGFX_FUNCTION_ID_CREATE_SHADER: bgfx_function_id = 57;
pub const BGFX_FUNCTION_ID_GET_SHADER_UNIFORMS: bgfx_function_id = 58;
pub const BGFX_FUNCTION_ID_SET_SHADER_NAME: bgfx_function_id = 59;
pub const BGFX_FUNCTION_ID_DESTROY_SHADER: bgfx_function_id = 60;
pub const BGFX_FUNCTION_ID_CREATE_PROGRAM: bgfx_function_id = 61;
pub const BGFX_FUNCTION_ID_CREATE_COMPUTE_PROGRAM: bgfx_function_id = 62;
pub const BGFX_FUNCTION_ID_DESTROY_PROGRAM: bgfx_function_id = 63;
pub const BGFX_FUNCTION_ID_IS_TEXTURE_VALID: bgfx_function_id = 64;
pub const BGFX_FUNCTION_ID_IS_FRAME_BUFFER_VALID: bgfx_function_id = 65;
pub const BGFX_FUNCTION_ID_CALC_TEXTURE_SIZE: bgfx_function_id = 66;
pub const BGFX_FUNCTION_ID_CREATE_TEXTURE: bgfx_function_id = 67;
pub const BGFX_FUNCTION_ID_CREATE_TEXTURE_2D: bgfx_function_id = 68;
pub const BGFX_FUNCTION_ID_CREATE_TEXTURE_2D_SCALED: bgfx_function_id = 69;
pub const BGFX_FUNCTION_ID_CREATE_TEXTURE_3D: bgfx_function_id = 70;
pub const BGFX_FUNCTION_ID_CREATE_TEXTURE_CUBE: bgfx_function_id = 71;
pub const BGFX_FUNCTION_ID_UPDATE_TEXTURE_2D: bgfx_function_id = 72;
pub const BGFX_FUNCTION_ID_UPDATE_TEXTURE_3D: bgfx_function_id = 73;
pub const BGFX_FUNCTION_ID_UPDATE_TEXTURE_CUBE: bgfx_function_id = 74;
pub const BGFX_FUNCTION_ID_READ_TEXTURE: bgfx_function_id = 75;
pub const BGFX_FUNCTION_ID_SET_TEXTURE_NAME: bgfx_function_id = 76;
pub const BGFX_FUNCTION_ID_GET_DIRECT_ACCESS_PTR: bgfx_function_id = 77;
pub const BGFX_FUNCTION_ID_DESTROY_TEXTURE: bgfx_function_id = 78;
pub const BGFX_FUNCTION_ID_CREATE_FRAME_BUFFER: bgfx_function_id = 79;
pub const BGFX_FUNCTION_ID_CREATE_FRAME_BUFFER_SCALED: bgfx_function_id = 80;
pub const BGFX_FUNCTION_ID_CREATE_FRAME_BUFFER_FROM_HANDLES: bgfx_function_id = 81;
pub const BGFX_FUNCTION_ID_CREATE_FRAME_BUFFER_FROM_ATTACHMENT: bgfx_function_id = 82;
pub const BGFX_FUNCTION_ID_CREATE_FRAME_BUFFER_FROM_NWH: bgfx_function_id = 83;
pub const BGFX_FUNCTION_ID_SET_FRAME_BUFFER_NAME: bgfx_function_id = 84;
pub const BGFX_FUNCTION_ID_GET_TEXTURE: bgfx_function_id = 85;
pub const BGFX_FUNCTION_ID_DESTROY_FRAME_BUFFER: bgfx_function_id = 86;
pub const BGFX_FUNCTION_ID_CREATE_UNIFORM: bgfx_function_id = 87;
pub const BGFX_FUNCTION_ID_GET_UNIFORM_INFO: bgfx_function_id = 88;
pub const BGFX_FUNCTION_ID_DESTROY_UNIFORM: bgfx_function_id = 89;
pub const BGFX_FUNCTION_ID_CREATE_OCCLUSION_QUERY: bgfx_function_id = 90;
pub const BGFX_FUNCTION_ID_GET_RESULT: bgfx_function_id = 91;
pub const BGFX_FUNCTION_ID_DESTROY_OCCLUSION_QUERY: bgfx_function_id = 92;
pub const BGFX_FUNCTION_ID_SET_PALETTE_COLOR: bgfx_function_id = 93;
pub const BGFX_FUNCTION_ID_SET_PALETTE_COLOR_RGBA8: bgfx_function_id = 94;
pub const BGFX_FUNCTION_ID_SET_VIEW_NAME: bgfx_function_id = 95;
pub const BGFX_FUNCTION_ID_SET_VIEW_RECT: bgfx_function_id = 96;
pub const BGFX_FUNCTION_ID_SET_VIEW_RECT_RATIO: bgfx_function_id = 97;
pub const BGFX_FUNCTION_ID_SET_VIEW_SCISSOR: bgfx_function_id = 98;
pub const BGFX_FUNCTION_ID_SET_VIEW_CLEAR: bgfx_function_id = 99;
pub const BGFX_FUNCTION_ID_SET_VIEW_CLEAR_MRT: bgfx_function_id = 100;
pub const BGFX_FUNCTION_ID_SET_VIEW_MODE: bgfx_function_id = 101;
pub const BGFX_FUNCTION_ID_SET_VIEW_FRAME_BUFFER: bgfx_function_id = 102;
pub const BGFX_FUNCTION_ID_SET_VIEW_TRANSFORM: bgfx_function_id = 103;
pub const BGFX_FUNCTION_ID_SET_VIEW_ORDER: bgfx_function_id = 104;
pub const BGFX_FUNCTION_ID_RESET_VIEW: bgfx_function_id = 105;
pub const BGFX_FUNCTION_ID_ENCODER_BEGIN: bgfx_function_id = 106;
pub const BGFX_FUNCTION_ID_ENCODER_END: bgfx_function_id = 107;
pub const BGFX_FUNCTION_ID_ENCODER_SET_MARKER: bgfx_function_id = 108;
pub const BGFX_FUNCTION_ID_ENCODER_SET_STATE: bgfx_function_id = 109;
pub const BGFX_FUNCTION_ID_ENCODER_SET_CONDITION: bgfx_function_id = 110;
pub const BGFX_FUNCTION_ID_ENCODER_SET_STENCIL: bgfx_function_id = 111;
pub const BGFX_FUNCTION_ID_ENCODER_SET_SCISSOR: bgfx_function_id = 112;
pub const BGFX_FUNCTION_ID_ENCODER_SET_SCISSOR_CACHED: bgfx_function_id = 113;
pub const BGFX_FUNCTION_ID_ENCODER_SET_TRANSFORM: bgfx_function_id = 114;
pub const BGFX_FUNCTION_ID_ENCODER_SET_TRANSFORM_CACHED: bgfx_function_id = 115;
pub const BGFX_FUNCTION_ID_ENCODER_ALLOC_TRANSFORM: bgfx_function_id = 116;
pub const BGFX_FUNCTION_ID_ENCODER_SET_UNIFORM: bgfx_function_id = 117;
pub const BGFX_FUNCTION_ID_ENCODER_SET_INDEX_BUFFER: bgfx_function_id = 118;
pub const BGFX_FUNCTION_ID_ENCODER_SET_DYNAMIC_INDEX_BUFFER: bgfx_function_id = 119;
pub const BGFX_FUNCTION_ID_ENCODER_SET_TRANSIENT_INDEX_BUFFER: bgfx_function_id = 120;
pub const BGFX_FUNCTION_ID_ENCODER_SET_VERTEX_BUFFER: bgfx_function_id = 121;
pub const BGFX_FUNCTION_ID_ENCODER_SET_VERTEX_BUFFER_WITH_LAYOUT: bgfx_function_id = 122;
pub const BGFX_FUNCTION_ID_ENCODER_SET_DYNAMIC_VERTEX_BUFFER: bgfx_function_id = 123;
pub const BGFX_FUNCTION_ID_ENCODER_SET_DYNAMIC_VERTEX_BUFFER_WITH_LAYOUT: bgfx_function_id = 124;
pub const BGFX_FUNCTION_ID_ENCODER_SET_TRANSIENT_VERTEX_BUFFER: bgfx_function_id = 125;
pub const BGFX_FUNCTION_ID_ENCODER_SET_TRANSIENT_VERTEX_BUFFER_WITH_LAYOUT: bgfx_function_id = 126;
pub const BGFX_FUNCTION_ID_ENCODER_SET_VERTEX_COUNT: bgfx_function_id = 127;
pub const BGFX_FUNCTION_ID_ENCODER_SET_INSTANCE_DATA_BUFFER: bgfx_function_id = 128;
pub const BGFX_FUNCTION_ID_ENCODER_SET_INSTANCE_DATA_FROM_VERTEX_BUFFER: bgfx_function_id = 129;
pub const BGFX_FUNCTION_ID_ENCODER_SET_INSTANCE_DATA_FROM_DYNAMIC_VERTEX_BUFFER: bgfx_function_id =
    130;
pub const BGFX_FUNCTION_ID_ENCODER_SET_INSTANCE_COUNT: bgfx_function_id = 131;
pub const BGFX_FUNCTION_ID_ENCODER_SET_TEXTURE: bgfx_function_id = 132;
pub const BGFX_FUNCTION_ID_ENCODER_TOUCH: bgfx_function_id = 133;
pub const BGFX_FUNCTION_ID_ENCODER_SUBMIT: bgfx_function_id = 134;
pub const BGFX_FUNCTION_ID_ENCODER_SUBMIT_OCCLUSION_QUERY: bgfx_function_id = 135;
pub const BGFX_FUNCTION_ID_ENCODER_SUBMIT_INDIRECT: bgfx_function_id = 136;
pub const BGFX_FUNCTION_ID_ENCODER_SET_COMPUTE_INDEX_BUFFER: bgfx_function_id = 137;
pub const BGFX_FUNCTION_ID_ENCODER_SET_COMPUTE_VERTEX_BUFFER: bgfx_function_id = 138;
pub const BGFX_FUNCTION_ID_ENCODER_SET_COMPUTE_DYNAMIC_INDEX_BUFFER: bgfx_function_id = 139;
pub const BGFX_FUNCTION_ID_ENCODER_SET_COMPUTE_DYNAMIC_VERTEX_BUFFER: bgfx_function_id = 140;
pub const BGFX_FUNCTION_ID_ENCODER_SET_COMPUTE_INDIRECT_BUFFER: bgfx_function_id = 141;
pub const BGFX_FUNCTION_ID_ENCODER_SET_IMAGE: bgfx_function_id = 142;
pub const BGFX_FUNCTION_ID_ENCODER_DISPATCH: bgfx_function_id = 143;
pub const BGFX_FUNCTION_ID_ENCODER_DISPATCH_INDIRECT: bgfx_function_id = 144;
pub const BGFX_FUNCTION_ID_ENCODER_DISCARD: bgfx_function_id = 145;
pub const BGFX_FUNCTION_ID_ENCODER_BLIT: bgfx_function_id = 146;
pub const BGFX_FUNCTION_ID_REQUEST_SCREEN_SHOT: bgfx_function_id = 147;
pub const BGFX_FUNCTION_ID_RENDER_FRAME: bgfx_function_id = 148;
pub const BGFX_FUNCTION_ID_SET_PLATFORM_DATA: bgfx_function_id = 149;
pub const BGFX_FUNCTION_ID_GET_INTERNAL_DATA: bgfx_function_id = 150;
pub const BGFX_FUNCTION_ID_OVERRIDE_INTERNAL_TEXTURE_PTR: bgfx_function_id = 151;
pub const BGFX_FUNCTION_ID_OVERRIDE_INTERNAL_TEXTURE: bgfx_function_id = 152;
pub const BGFX_FUNCTION_ID_SET_MARKER: bgfx_function_id = 153;
pub const BGFX_FUNCTION_ID_SET_STATE: bgfx_function_id = 154;
pub const BGFX_FUNCTION_ID_SET_CONDITION: bgfx_function_id = 155;
pub const BGFX_FUNCTION_ID_SET_STENCIL: bgfx_function_id = 156;
pub const BGFX_FUNCTION_ID_SET_SCISSOR: bgfx_function_id = 157;
pub const BGFX_FUNCTION_ID_SET_SCISSOR_CACHED: bgfx_function_id = 158;
pub const BGFX_FUNCTION_ID_SET_TRANSFORM: bgfx_function_id = 159;
pub const BGFX_FUNCTION_ID_SET_TRANSFORM_CACHED: bgfx_function_id = 160;
pub const BGFX_FUNCTION_ID_ALLOC_TRANSFORM: bgfx_function_id = 161;
pub const BGFX_FUNCTION_ID_SET_UNIFORM: bgfx_function_id = 162;
pub const BGFX_FUNCTION_ID_SET_INDEX_BUFFER: bgfx_function_id = 163;
pub const BGFX_FUNCTION_ID_SET_DYNAMIC_INDEX_BUFFER: bgfx_function_id = 164;
pub const BGFX_FUNCTION_ID_SET_TRANSIENT_INDEX_BUFFER: bgfx_function_id = 165;
pub const BGFX_FUNCTION_ID_SET_VERTEX_BUFFER: bgfx_function_id = 166;
pub const BGFX_FUNCTION_ID_SET_VERTEX_BUFFER_WITH_LAYOUT: bgfx_function_id = 167;
pub const BGFX_FUNCTION_ID_SET_DYNAMIC_VERTEX_BUFFER: bgfx_function_id = 168;
pub const BGFX_FUNCTION_ID_SET_DYNAMIC_VERTEX_BUFFER_WITH_LAYOUT: bgfx_function_id = 169;
pub const BGFX_FUNCTION_ID_SET_TRANSIENT_VERTEX_BUFFER: bgfx_function_id = 170;
pub const BGFX_FUNCTION_ID_SET_TRANSIENT_VERTEX_BUFFER_WITH_LAYOUT: bgfx_function_id = 171;
pub const BGFX_FUNCTION_ID_SET_VERTEX_COUNT: bgfx_function_id = 172;
pub const BGFX_FUNCTION_ID_SET_INSTANCE_DATA_BUFFER: bgfx_function_id = 173;
pub const BGFX_FUNCTION_ID_SET_INSTANCE_DATA_FROM_VERTEX_BUFFER: bgfx_function_id = 174;
pub const BGFX_FUNCTION_ID_SET_INSTANCE_DATA_FROM_DYNAMIC_VERTEX_BUFFER: bgfx_function_id = 175;
pub const BGFX_FUNCTION_ID_SET_INSTANCE_COUNT: bgfx_function_id = 176;
pub const BGFX_FUNCTION_ID_SET_TEXTURE: bgfx_function_id = 177;
pub const BGFX_FUNCTION_ID_TOUCH: bgfx_function_id = 178;
pub const BGFX_FUNCTION_ID_SUBMIT: bgfx_function_id = 179;
pub const BGFX_FUNCTION_ID_SUBMIT_OCCLUSION_QUERY: bgfx_function_id = 180;
pub const BGFX_FUNCTION_ID_SUBMIT_INDIRECT: bgfx_function_id = 181;
pub const BGFX_FUNCTION_ID_SET_COMPUTE_INDEX_BUFFER: bgfx_function_id = 182;
pub const BGFX_FUNCTION_ID_SET_COMPUTE_VERTEX_BUFFER: bgfx_function_id = 183;
pub const BGFX_FUNCTION_ID_SET_COMPUTE_DYNAMIC_INDEX_BUFFER: bgfx_function_id = 184;
pub const BGFX_FUNCTION_ID_SET_COMPUTE_DYNAMIC_VERTEX_BUFFER: bgfx_function_id = 185;
pub const BGFX_FUNCTION_ID_SET_COMPUTE_INDIRECT_BUFFER: bgfx_function_id = 186;
pub const BGFX_FUNCTION_ID_SET_IMAGE: bgfx_function_id = 187;
pub const BGFX_FUNCTION_ID_DISPATCH: bgfx_function_id = 188;
pub const BGFX_FUNCTION_ID_DISPATCH_INDIRECT: bgfx_function_id = 189;
pub const BGFX_FUNCTION_ID_DISCARD: bgfx_function_id = 190;
pub const BGFX_FUNCTION_ID_BLIT: bgfx_function_id = 191;
pub const BGFX_FUNCTION_ID_COUNT: bgfx_function_id = 192;
pub type bgfx_function_id = u32;
pub use self::bgfx_function_id as bgfx_function_id_t;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct bgfx_interface_vtbl {
    pub attachment_init: ::std::option::Option<
        unsafe extern "C" fn(
            _this: *mut bgfx_attachment_t,
            _handle: bgfx_texture_handle_t,
            _access: bgfx_access_t,
            _layer: u16,
            _numLayers: u16,
            _mip: u16,
            _resolve: u8,
        ),
    >,
    pub vertex_layout_begin: ::std::option::Option<
        unsafe extern "C" fn(
            _this: *mut bgfx_vertex_layout_t,
            _rendererType: bgfx_renderer_type_t,
        ) -> *mut bgfx_vertex_layout_t,
    >,
    pub vertex_layout_add: ::std::option::Option<
        unsafe extern "C" fn(
            _this: *mut bgfx_vertex_layout_t,
            _attrib: bgfx_attrib_t,
            _num: u8,
            _type: bgfx_attrib_type_t,
            _normalized: bool,
            _asInt: bool,
        ) -> *mut bgfx_vertex_layout_t,
    >,
    pub vertex_layout_decode: ::std::option::Option<
        unsafe extern "C" fn(
            _this: *const bgfx_vertex_layout_t,
            _attrib: bgfx_attrib_t,
            _num: *mut u8,
            _type: *mut bgfx_attrib_type_t,
            _normalized: *mut bool,
            _asInt: *mut bool,
        ),
    >,
    pub vertex_layout_has: ::std::option::Option<
        unsafe extern "C" fn(_this: *const bgfx_vertex_layout_t, _attrib: bgfx_attrib_t) -> bool,
    >,
    pub vertex_layout_skip: ::std::option::Option<
        unsafe extern "C" fn(
            _this: *mut bgfx_vertex_layout_t,
            _num: u8,
        ) -> *mut bgfx_vertex_layout_t,
    >,
    pub vertex_layout_end:
        ::std::option::Option<unsafe extern "C" fn(_this: *mut bgfx_vertex_layout_t)>,
    pub vertex_pack: ::std::option::Option<
        unsafe extern "C" fn(
            _input: *const f32,
            _inputNormalized: bool,
            _attr: bgfx_attrib_t,
            _layout: *const bgfx_vertex_layout_t,
            _data: *mut ::std::os::raw::c_void,
            _index: u32,
        ),
    >,
    pub vertex_unpack: ::std::option::Option<
        unsafe extern "C" fn(
            _output: *mut f32,
            _attr: bgfx_attrib_t,
            _layout: *const bgfx_vertex_layout_t,
            _data: *const ::std::os::raw::c_void,
            _index: u32,
        ),
    >,
    pub vertex_convert: ::std::option::Option<
        unsafe extern "C" fn(
            _dstLayout: *const bgfx_vertex_layout_t,
            _dstData: *mut ::std::os::raw::c_void,
            _srcLayout: *const bgfx_vertex_layout_t,
            _srcData: *const ::std::os::raw::c_void,
            _num: u32,
        ),
    >,
    pub weld_vertices: ::std::option::Option<
        unsafe extern "C" fn(
            _output: *mut ::std::os::raw::c_void,
            _layout: *const bgfx_vertex_layout_t,
            _data: *const ::std::os::raw::c_void,
            _num: u32,
            _index32: bool,
            _epsilon: f32,
        ) -> u32,
    >,
    pub topology_convert: ::std::option::Option<
        unsafe extern "C" fn(
            _conversion: bgfx_topology_convert_t,
            _dst: *mut ::std::os::raw::c_void,
            _dstSize: u32,
            _indices: *const ::std::os::raw::c_void,
            _numIndices: u32,
            _index32: bool,
        ) -> u32,
    >,
    pub topology_sort_tri_list: ::std::option::Option<
        unsafe extern "C" fn(
            _sort: bgfx_topology_sort_t,
            _dst: *mut ::std::os::raw::c_void,
            _dstSize: u32,
            _dir: *const f32,
            _pos: *const f32,
            _vertices: *const ::std::os::raw::c_void,
            _stride: u32,
            _indices: *const ::std::os::raw::c_void,
            _numIndices: u32,
            _index32: bool,
        ),
    >,
    pub get_supported_renderers: ::std::option::Option<
        unsafe extern "C" fn(_max: u8, _enum: *mut bgfx_renderer_type_t) -> u8,
    >,
    pub get_renderer_name: ::std::option::Option<
        unsafe extern "C" fn(_type: bgfx_renderer_type_t) -> *const ::std::os::raw::c_char,
    >,
    pub init_ctor: ::std::option::Option<unsafe extern "C" fn(_init: *mut bgfx_init_t)>,
    pub init: ::std::option::Option<unsafe extern "C" fn(_init: *const bgfx_init_t) -> bool>,
    pub shutdown: ::std::option::Option<unsafe extern "C" fn()>,
    pub reset: ::std::option::Option<
        unsafe extern "C" fn(
            _width: u32,
            _height: u32,
            _flags: u32,
            _format: bgfx_texture_format_t,
        ),
    >,
    pub frame: ::std::option::Option<unsafe extern "C" fn(_capture: bool) -> u32>,
    pub get_renderer_type: ::std::option::Option<unsafe extern "C" fn() -> bgfx_renderer_type_t>,
    pub get_caps: ::std::option::Option<unsafe extern "C" fn() -> *const bgfx_caps_t>,
    pub get_stats: ::std::option::Option<unsafe extern "C" fn() -> *const bgfx_stats_t>,
    pub alloc: ::std::option::Option<unsafe extern "C" fn(_size: u32) -> *const bgfx_memory_t>,
    pub copy: ::std::option::Option<
        unsafe extern "C" fn(
            _data: *const ::std::os::raw::c_void,
            _size: u32,
        ) -> *const bgfx_memory_t,
    >,
    pub make_ref: ::std::option::Option<
        unsafe extern "C" fn(
            _data: *const ::std::os::raw::c_void,
            _size: u32,
        ) -> *const bgfx_memory_t,
    >,
    pub make_ref_release: ::std::option::Option<
        unsafe extern "C" fn(
            _data: *const ::std::os::raw::c_void,
            _size: u32,
            _releaseFn: bgfx_release_fn_t,
            _userData: *mut ::std::os::raw::c_void,
        ) -> *const bgfx_memory_t,
    >,
    pub set_debug: ::std::option::Option<unsafe extern "C" fn(_debug: u32)>,
    pub dbg_text_clear: ::std::option::Option<unsafe extern "C" fn(_attr: u8, _small: bool)>,
    pub dbg_text_printf: ::std::option::Option<
        unsafe extern "C" fn(
            _x: u16,
            _y: u16,
            _attr: u8,
            _format: *const ::std::os::raw::c_char,
            ...
        ),
    >,
    pub dbg_text_vprintf: ::std::option::Option<
        unsafe extern "C" fn(
            _x: u16,
            _y: u16,
            _attr: u8,
            _format: *const ::std::os::raw::c_char,
            _argList: *mut __va_list_tag,
        ),
    >,
    pub dbg_text_image: ::std::option::Option<
        unsafe extern "C" fn(
            _x: u16,
            _y: u16,
            _width: u16,
            _height: u16,
            _data: *const ::std::os::raw::c_void,
            _pitch: u16,
        ),
    >,
    pub create_index_buffer: ::std::option::Option<
        unsafe extern "C" fn(_mem: *const bgfx_memory_t, _flags: u16) -> bgfx_index_buffer_handle_t,
    >,
    pub set_index_buffer_name: ::std::option::Option<
        unsafe extern "C" fn(
            _handle: bgfx_index_buffer_handle_t,
            _name: *const ::std::os::raw::c_char,
            _len: i32,
        ),
    >,
    pub destroy_index_buffer:
        ::std::option::Option<unsafe extern "C" fn(_handle: bgfx_index_buffer_handle_t)>,
    pub create_vertex_layout: ::std::option::Option<
        unsafe extern "C" fn(_layout: *const bgfx_vertex_layout_t) -> bgfx_vertex_layout_handle_t,
    >,
    pub destroy_vertex_layout:
        ::std::option::Option<unsafe extern "C" fn(_layoutHandle: bgfx_vertex_layout_handle_t)>,
    pub create_vertex_buffer: ::std::option::Option<
        unsafe extern "C" fn(
            _mem: *const bgfx_memory_t,
            _layout: *const bgfx_vertex_layout_t,
            _flags: u16,
        ) -> bgfx_vertex_buffer_handle_t,
    >,
    pub set_vertex_buffer_name: ::std::option::Option<
        unsafe extern "C" fn(
            _handle: bgfx_vertex_buffer_handle_t,
            _name: *const ::std::os::raw::c_char,
            _len: i32,
        ),
    >,
    pub destroy_vertex_buffer:
        ::std::option::Option<unsafe extern "C" fn(_handle: bgfx_vertex_buffer_handle_t)>,
    pub create_dynamic_index_buffer: ::std::option::Option<
        unsafe extern "C" fn(_num: u32, _flags: u16) -> bgfx_dynamic_index_buffer_handle_t,
    >,
    pub create_dynamic_index_buffer_mem: ::std::option::Option<
        unsafe extern "C" fn(
            _mem: *const bgfx_memory_t,
            _flags: u16,
        ) -> bgfx_dynamic_index_buffer_handle_t,
    >,
    pub update_dynamic_index_buffer: ::std::option::Option<
        unsafe extern "C" fn(
            _handle: bgfx_dynamic_index_buffer_handle_t,
            _startIndex: u32,
            _mem: *const bgfx_memory_t,
        ),
    >,
    pub destroy_dynamic_index_buffer:
        ::std::option::Option<unsafe extern "C" fn(_handle: bgfx_dynamic_index_buffer_handle_t)>,
    pub create_dynamic_vertex_buffer: ::std::option::Option<
        unsafe extern "C" fn(
            _num: u32,
            _layout: *const bgfx_vertex_layout_t,
            _flags: u16,
        ) -> bgfx_dynamic_vertex_buffer_handle_t,
    >,
    pub create_dynamic_vertex_buffer_mem: ::std::option::Option<
        unsafe extern "C" fn(
            _mem: *const bgfx_memory_t,
            _layout: *const bgfx_vertex_layout_t,
            _flags: u16,
        ) -> bgfx_dynamic_vertex_buffer_handle_t,
    >,
    pub update_dynamic_vertex_buffer: ::std::option::Option<
        unsafe extern "C" fn(
            _handle: bgfx_dynamic_vertex_buffer_handle_t,
            _startVertex: u32,
            _mem: *const bgfx_memory_t,
        ),
    >,
    pub destroy_dynamic_vertex_buffer:
        ::std::option::Option<unsafe extern "C" fn(_handle: bgfx_dynamic_vertex_buffer_handle_t)>,
    pub get_avail_transient_index_buffer:
        ::std::option::Option<unsafe extern "C" fn(_num: u32, _index32: bool) -> u32>,
    pub get_avail_transient_vertex_buffer: ::std::option::Option<
        unsafe extern "C" fn(_num: u32, _layout: *const bgfx_vertex_layout_t) -> u32,
    >,
    pub get_avail_instance_data_buffer:
        ::std::option::Option<unsafe extern "C" fn(_num: u32, _stride: u16) -> u32>,
    pub alloc_transient_index_buffer: ::std::option::Option<
        unsafe extern "C" fn(_tib: *mut bgfx_transient_index_buffer_t, _num: u32, _index32: bool),
    >,
    pub alloc_transient_vertex_buffer: ::std::option::Option<
        unsafe extern "C" fn(
            _tvb: *mut bgfx_transient_vertex_buffer_t,
            _num: u32,
            _layout: *const bgfx_vertex_layout_t,
        ),
    >,
    pub alloc_transient_buffers: ::std::option::Option<
        unsafe extern "C" fn(
            _tvb: *mut bgfx_transient_vertex_buffer_t,
            _layout: *const bgfx_vertex_layout_t,
            _numVertices: u32,
            _tib: *mut bgfx_transient_index_buffer_t,
            _numIndices: u32,
            _index32: bool,
        ) -> bool,
    >,
    pub alloc_instance_data_buffer: ::std::option::Option<
        unsafe extern "C" fn(_idb: *mut bgfx_instance_data_buffer_t, _num: u32, _stride: u16),
    >,
    pub create_indirect_buffer:
        ::std::option::Option<unsafe extern "C" fn(_num: u32) -> bgfx_indirect_buffer_handle_t>,
    pub destroy_indirect_buffer:
        ::std::option::Option<unsafe extern "C" fn(_handle: bgfx_indirect_buffer_handle_t)>,
    pub create_shader: ::std::option::Option<
        unsafe extern "C" fn(_mem: *const bgfx_memory_t) -> bgfx_shader_handle_t,
    >,
    pub get_shader_uniforms: ::std::option::Option<
        unsafe extern "C" fn(
            _handle: bgfx_shader_handle_t,
            _uniforms: *mut bgfx_uniform_handle_t,
            _max: u16,
        ) -> u16,
    >,
    pub set_shader_name: ::std::option::Option<
        unsafe extern "C" fn(
            _handle: bgfx_shader_handle_t,
            _name: *const ::std::os::raw::c_char,
            _len: i32,
        ),
    >,
    pub destroy_shader: ::std::option::Option<unsafe extern "C" fn(_handle: bgfx_shader_handle_t)>,
    pub create_program: ::std::option::Option<
        unsafe extern "C" fn(
            _vsh: bgfx_shader_handle_t,
            _fsh: bgfx_shader_handle_t,
            _destroyShaders: bool,
        ) -> bgfx_program_handle_t,
    >,
    pub create_compute_program: ::std::option::Option<
        unsafe extern "C" fn(
            _csh: bgfx_shader_handle_t,
            _destroyShaders: bool,
        ) -> bgfx_program_handle_t,
    >,
    pub destroy_program:
        ::std::option::Option<unsafe extern "C" fn(_handle: bgfx_program_handle_t)>,
    pub is_texture_valid: ::std::option::Option<
        unsafe extern "C" fn(
            _depth: u16,
            _cubeMap: bool,
            _numLayers: u16,
            _format: bgfx_texture_format_t,
            _flags: u64,
        ) -> bool,
    >,
    pub is_frame_buffer_valid: ::std::option::Option<
        unsafe extern "C" fn(_num: u8, _attachment: *const bgfx_attachment_t) -> bool,
    >,
    pub calc_texture_size: ::std::option::Option<
        unsafe extern "C" fn(
            _info: *mut bgfx_texture_info_t,
            _width: u16,
            _height: u16,
            _depth: u16,
            _cubeMap: bool,
            _hasMips: bool,
            _numLayers: u16,
            _format: bgfx_texture_format_t,
        ),
    >,
    pub create_texture: ::std::option::Option<
        unsafe extern "C" fn(
            _mem: *const bgfx_memory_t,
            _flags: u64,
            _skip: u8,
            _info: *mut bgfx_texture_info_t,
        ) -> bgfx_texture_handle_t,
    >,
    pub create_texture_2d: ::std::option::Option<
        unsafe extern "C" fn(
            _width: u16,
            _height: u16,
            _hasMips: bool,
            _numLayers: u16,
            _format: bgfx_texture_format_t,
            _flags: u64,
            _mem: *const bgfx_memory_t,
        ) -> bgfx_texture_handle_t,
    >,
    pub create_texture_2d_scaled: ::std::option::Option<
        unsafe extern "C" fn(
            _ratio: bgfx_backbuffer_ratio_t,
            _hasMips: bool,
            _numLayers: u16,
            _format: bgfx_texture_format_t,
            _flags: u64,
        ) -> bgfx_texture_handle_t,
    >,
    pub create_texture_3d: ::std::option::Option<
        unsafe extern "C" fn(
            _width: u16,
            _height: u16,
            _depth: u16,
            _hasMips: bool,
            _format: bgfx_texture_format_t,
            _flags: u64,
            _mem: *const bgfx_memory_t,
        ) -> bgfx_texture_handle_t,
    >,
    pub create_texture_cube: ::std::option::Option<
        unsafe extern "C" fn(
            _size: u16,
            _hasMips: bool,
            _numLayers: u16,
            _format: bgfx_texture_format_t,
            _flags: u64,
            _mem: *const bgfx_memory_t,
        ) -> bgfx_texture_handle_t,
    >,
    pub update_texture_2d: ::std::option::Option<
        unsafe extern "C" fn(
            _handle: bgfx_texture_handle_t,
            _layer: u16,
            _mip: u8,
            _x: u16,
            _y: u16,
            _width: u16,
            _height: u16,
            _mem: *const bgfx_memory_t,
            _pitch: u16,
        ),
    >,
    pub update_texture_3d: ::std::option::Option<
        unsafe extern "C" fn(
            _handle: bgfx_texture_handle_t,
            _mip: u8,
            _x: u16,
            _y: u16,
            _z: u16,
            _width: u16,
            _height: u16,
            _depth: u16,
            _mem: *const bgfx_memory_t,
        ),
    >,
    pub update_texture_cube: ::std::option::Option<
        unsafe extern "C" fn(
            _handle: bgfx_texture_handle_t,
            _layer: u16,
            _side: u8,
            _mip: u8,
            _x: u16,
            _y: u16,
            _width: u16,
            _height: u16,
            _mem: *const bgfx_memory_t,
            _pitch: u16,
        ),
    >,
    pub read_texture: ::std::option::Option<
        unsafe extern "C" fn(
            _handle: bgfx_texture_handle_t,
            _data: *mut ::std::os::raw::c_void,
            _mip: u8,
        ) -> u32,
    >,
    pub set_texture_name: ::std::option::Option<
        unsafe extern "C" fn(
            _handle: bgfx_texture_handle_t,
            _name: *const ::std::os::raw::c_char,
            _len: i32,
        ),
    >,
    pub get_direct_access_ptr: ::std::option::Option<
        unsafe extern "C" fn(_handle: bgfx_texture_handle_t) -> *mut ::std::os::raw::c_void,
    >,
    pub destroy_texture:
        ::std::option::Option<unsafe extern "C" fn(_handle: bgfx_texture_handle_t)>,
    pub create_frame_buffer: ::std::option::Option<
        unsafe extern "C" fn(
            _width: u16,
            _height: u16,
            _format: bgfx_texture_format_t,
            _textureFlags: u64,
        ) -> bgfx_frame_buffer_handle_t,
    >,
    pub create_frame_buffer_scaled: ::std::option::Option<
        unsafe extern "C" fn(
            _ratio: bgfx_backbuffer_ratio_t,
            _format: bgfx_texture_format_t,
            _textureFlags: u64,
        ) -> bgfx_frame_buffer_handle_t,
    >,
    pub create_frame_buffer_from_handles: ::std::option::Option<
        unsafe extern "C" fn(
            _num: u8,
            _handles: *const bgfx_texture_handle_t,
            _destroyTexture: bool,
        ) -> bgfx_frame_buffer_handle_t,
    >,
    pub create_frame_buffer_from_attachment: ::std::option::Option<
        unsafe extern "C" fn(
            _num: u8,
            _attachment: *const bgfx_attachment_t,
            _destroyTexture: bool,
        ) -> bgfx_frame_buffer_handle_t,
    >,
    pub create_frame_buffer_from_nwh: ::std::option::Option<
        unsafe extern "C" fn(
            _nwh: *mut ::std::os::raw::c_void,
            _width: u16,
            _height: u16,
            _format: bgfx_texture_format_t,
            _depthFormat: bgfx_texture_format_t,
        ) -> bgfx_frame_buffer_handle_t,
    >,
    pub set_frame_buffer_name: ::std::option::Option<
        unsafe extern "C" fn(
            _handle: bgfx_frame_buffer_handle_t,
            _name: *const ::std::os::raw::c_char,
            _len: i32,
        ),
    >,
    pub get_texture: ::std::option::Option<
        unsafe extern "C" fn(
            _handle: bgfx_frame_buffer_handle_t,
            _attachment: u8,
        ) -> bgfx_texture_handle_t,
    >,
    pub destroy_frame_buffer:
        ::std::option::Option<unsafe extern "C" fn(_handle: bgfx_frame_buffer_handle_t)>,
    pub create_uniform: ::std::option::Option<
        unsafe extern "C" fn(
            _name: *const ::std::os::raw::c_char,
            _type: bgfx_uniform_type_t,
            _num: u16,
        ) -> bgfx_uniform_handle_t,
    >,
    pub get_uniform_info: ::std::option::Option<
        unsafe extern "C" fn(_handle: bgfx_uniform_handle_t, _info: *mut bgfx_uniform_info_t),
    >,
    pub destroy_uniform:
        ::std::option::Option<unsafe extern "C" fn(_handle: bgfx_uniform_handle_t)>,
    pub create_occlusion_query:
        ::std::option::Option<unsafe extern "C" fn() -> bgfx_occlusion_query_handle_t>,
    pub get_result: ::std::option::Option<
        unsafe extern "C" fn(
            _handle: bgfx_occlusion_query_handle_t,
            _result: *mut i32,
        ) -> bgfx_occlusion_query_result_t,
    >,
    pub destroy_occlusion_query:
        ::std::option::Option<unsafe extern "C" fn(_handle: bgfx_occlusion_query_handle_t)>,
    pub set_palette_color:
        ::std::option::Option<unsafe extern "C" fn(_index: u8, _rgba: *const f32)>,
    pub set_palette_color_rgba8:
        ::std::option::Option<unsafe extern "C" fn(_index: u8, _rgba: u32)>,
    pub set_view_name: ::std::option::Option<
        unsafe extern "C" fn(_id: bgfx_view_id_t, _name: *const ::std::os::raw::c_char),
    >,
    pub set_view_rect: ::std::option::Option<
        unsafe extern "C" fn(_id: bgfx_view_id_t, _x: u16, _y: u16, _width: u16, _height: u16),
    >,
    pub set_view_rect_ratio: ::std::option::Option<
        unsafe extern "C" fn(
            _id: bgfx_view_id_t,
            _x: u16,
            _y: u16,
            _ratio: bgfx_backbuffer_ratio_t,
        ),
    >,
    pub set_view_scissor: ::std::option::Option<
        unsafe extern "C" fn(_id: bgfx_view_id_t, _x: u16, _y: u16, _width: u16, _height: u16),
    >,
    pub set_view_clear: ::std::option::Option<
        unsafe extern "C" fn(
            _id: bgfx_view_id_t,
            _flags: u16,
            _rgba: u32,
            _depth: f32,
            _stencil: u8,
        ),
    >,
    pub set_view_clear_mrt: ::std::option::Option<
        unsafe extern "C" fn(
            _id: bgfx_view_id_t,
            _flags: u16,
            _depth: f32,
            _stencil: u8,
            _c0: u8,
            _c1: u8,
            _c2: u8,
            _c3: u8,
            _c4: u8,
            _c5: u8,
            _c6: u8,
            _c7: u8,
        ),
    >,
    pub set_view_mode:
        ::std::option::Option<unsafe extern "C" fn(_id: bgfx_view_id_t, _mode: bgfx_view_mode_t)>,
    pub set_view_frame_buffer: ::std::option::Option<
        unsafe extern "C" fn(_id: bgfx_view_id_t, _handle: bgfx_frame_buffer_handle_t),
    >,
    pub set_view_transform: ::std::option::Option<
        unsafe extern "C" fn(
            _id: bgfx_view_id_t,
            _view: *const ::std::os::raw::c_void,
            _proj: *const ::std::os::raw::c_void,
        ),
    >,
    pub set_view_order: ::std::option::Option<
        unsafe extern "C" fn(_id: bgfx_view_id_t, _num: u16, _order: *const bgfx_view_id_t),
    >,
    pub reset_view: ::std::option::Option<unsafe extern "C" fn(_id: bgfx_view_id_t)>,
    pub encoder_begin:
        ::std::option::Option<unsafe extern "C" fn(_forThread: bool) -> *mut bgfx_encoder_t>,
    pub encoder_end: ::std::option::Option<unsafe extern "C" fn(_encoder: *mut bgfx_encoder_t)>,
    pub encoder_set_marker: ::std::option::Option<
        unsafe extern "C" fn(_this: *mut bgfx_encoder_t, _marker: *const ::std::os::raw::c_char),
    >,
    pub encoder_set_state: ::std::option::Option<
        unsafe extern "C" fn(_this: *mut bgfx_encoder_t, _state: u64, _rgba: u32),
    >,
    pub encoder_set_condition: ::std::option::Option<
        unsafe extern "C" fn(
            _this: *mut bgfx_encoder_t,
            _handle: bgfx_occlusion_query_handle_t,
            _visible: bool,
        ),
    >,
    pub encoder_set_stencil: ::std::option::Option<
        unsafe extern "C" fn(_this: *mut bgfx_encoder_t, _fstencil: u32, _bstencil: u32),
    >,
    pub encoder_set_scissor: ::std::option::Option<
        unsafe extern "C" fn(
            _this: *mut bgfx_encoder_t,
            _x: u16,
            _y: u16,
            _width: u16,
            _height: u16,
        ) -> u16,
    >,
    pub encoder_set_scissor_cached:
        ::std::option::Option<unsafe extern "C" fn(_this: *mut bgfx_encoder_t, _cache: u16)>,
    pub encoder_set_transform: ::std::option::Option<
        unsafe extern "C" fn(
            _this: *mut bgfx_encoder_t,
            _mtx: *const ::std::os::raw::c_void,
            _num: u16,
        ) -> u32,
    >,
    pub encoder_set_transform_cached: ::std::option::Option<
        unsafe extern "C" fn(_this: *mut bgfx_encoder_t, _cache: u32, _num: u16),
    >,
    pub encoder_alloc_transform: ::std::option::Option<
        unsafe extern "C" fn(
            _this: *mut bgfx_encoder_t,
            _transform: *mut bgfx_transform_t,
            _num: u16,
        ) -> u32,
    >,
    pub encoder_set_uniform: ::std::option::Option<
        unsafe extern "C" fn(
            _this: *mut bgfx_encoder_t,
            _handle: bgfx_uniform_handle_t,
            _value: *const ::std::os::raw::c_void,
            _num: u16,
        ),
    >,
    pub encoder_set_index_buffer: ::std::option::Option<
        unsafe extern "C" fn(
            _this: *mut bgfx_encoder_t,
            _handle: bgfx_index_buffer_handle_t,
            _firstIndex: u32,
            _numIndices: u32,
        ),
    >,
    pub encoder_set_dynamic_index_buffer: ::std::option::Option<
        unsafe extern "C" fn(
            _this: *mut bgfx_encoder_t,
            _handle: bgfx_dynamic_index_buffer_handle_t,
            _firstIndex: u32,
            _numIndices: u32,
        ),
    >,
    pub encoder_set_transient_index_buffer: ::std::option::Option<
        unsafe extern "C" fn(
            _this: *mut bgfx_encoder_t,
            _tib: *const bgfx_transient_index_buffer_t,
            _firstIndex: u32,
            _numIndices: u32,
        ),
    >,
    pub encoder_set_vertex_buffer: ::std::option::Option<
        unsafe extern "C" fn(
            _this: *mut bgfx_encoder_t,
            _stream: u8,
            _handle: bgfx_vertex_buffer_handle_t,
            _startVertex: u32,
            _numVertices: u32,
        ),
    >,
    pub encoder_set_vertex_buffer_with_layout: ::std::option::Option<
        unsafe extern "C" fn(
            _this: *mut bgfx_encoder_t,
            _stream: u8,
            _handle: bgfx_vertex_buffer_handle_t,
            _startVertex: u32,
            _numVertices: u32,
            _layoutHandle: bgfx_vertex_layout_handle_t,
        ),
    >,
    pub encoder_set_dynamic_vertex_buffer: ::std::option::Option<
        unsafe extern "C" fn(
            _this: *mut bgfx_encoder_t,
            _stream: u8,
            _handle: bgfx_dynamic_vertex_buffer_handle_t,
            _startVertex: u32,
            _numVertices: u32,
        ),
    >,
    pub encoder_set_dynamic_vertex_buffer_with_layout: ::std::option::Option<
        unsafe extern "C" fn(
            _this: *mut bgfx_encoder_t,
            _stream: u8,
            _handle: bgfx_dynamic_vertex_buffer_handle_t,
            _startVertex: u32,
            _numVertices: u32,
            _layoutHandle: bgfx_vertex_layout_handle_t,
        ),
    >,
    pub encoder_set_transient_vertex_buffer: ::std::option::Option<
        unsafe extern "C" fn(
            _this: *mut bgfx_encoder_t,
            _stream: u8,
            _tvb: *const bgfx_transient_vertex_buffer_t,
            _startVertex: u32,
            _numVertices: u32,
        ),
    >,
    pub encoder_set_transient_vertex_buffer_with_layout: ::std::option::Option<
        unsafe extern "C" fn(
            _this: *mut bgfx_encoder_t,
            _stream: u8,
            _tvb: *const bgfx_transient_vertex_buffer_t,
            _startVertex: u32,
            _numVertices: u32,
            _layoutHandle: bgfx_vertex_layout_handle_t,
        ),
    >,
    pub encoder_set_vertex_count:
        ::std::option::Option<unsafe extern "C" fn(_this: *mut bgfx_encoder_t, _numVertices: u32)>,
    pub encoder_set_instance_data_buffer: ::std::option::Option<
        unsafe extern "C" fn(
            _this: *mut bgfx_encoder_t,
            _idb: *const bgfx_instance_data_buffer_t,
            _start: u32,
            _num: u32,
        ),
    >,
    pub encoder_set_instance_data_from_vertex_buffer: ::std::option::Option<
        unsafe extern "C" fn(
            _this: *mut bgfx_encoder_t,
            _handle: bgfx_vertex_buffer_handle_t,
            _startVertex: u32,
            _num: u32,
        ),
    >,
    pub encoder_set_instance_data_from_dynamic_vertex_buffer: ::std::option::Option<
        unsafe extern "C" fn(
            _this: *mut bgfx_encoder_t,
            _handle: bgfx_dynamic_vertex_buffer_handle_t,
            _startVertex: u32,
            _num: u32,
        ),
    >,
    pub encoder_set_instance_count:
        ::std::option::Option<unsafe extern "C" fn(_this: *mut bgfx_encoder_t, _numInstances: u32)>,
    pub encoder_set_texture: ::std::option::Option<
        unsafe extern "C" fn(
            _this: *mut bgfx_encoder_t,
            _stage: u8,
            _sampler: bgfx_uniform_handle_t,
            _handle: bgfx_texture_handle_t,
            _flags: u32,
        ),
    >,
    pub encoder_touch: ::std::option::Option<
        unsafe extern "C" fn(_this: *mut bgfx_encoder_t, _id: bgfx_view_id_t),
    >,
    pub encoder_submit: ::std::option::Option<
        unsafe extern "C" fn(
            _this: *mut bgfx_encoder_t,
            _id: bgfx_view_id_t,
            _program: bgfx_program_handle_t,
            _depth: u32,
            _flags: u8,
        ),
    >,
    pub encoder_submit_occlusion_query: ::std::option::Option<
        unsafe extern "C" fn(
            _this: *mut bgfx_encoder_t,
            _id: bgfx_view_id_t,
            _program: bgfx_program_handle_t,
            _occlusionQuery: bgfx_occlusion_query_handle_t,
            _depth: u32,
            _flags: u8,
        ),
    >,
    pub encoder_submit_indirect: ::std::option::Option<
        unsafe extern "C" fn(
            _this: *mut bgfx_encoder_t,
            _id: bgfx_view_id_t,
            _program: bgfx_program_handle_t,
            _indirectHandle: bgfx_indirect_buffer_handle_t,
            _start: u16,
            _num: u16,
            _depth: u32,
            _flags: u8,
        ),
    >,
    pub encoder_set_compute_index_buffer: ::std::option::Option<
        unsafe extern "C" fn(
            _this: *mut bgfx_encoder_t,
            _stage: u8,
            _handle: bgfx_index_buffer_handle_t,
            _access: bgfx_access_t,
        ),
    >,
    pub encoder_set_compute_vertex_buffer: ::std::option::Option<
        unsafe extern "C" fn(
            _this: *mut bgfx_encoder_t,
            _stage: u8,
            _handle: bgfx_vertex_buffer_handle_t,
            _access: bgfx_access_t,
        ),
    >,
    pub encoder_set_compute_dynamic_index_buffer: ::std::option::Option<
        unsafe extern "C" fn(
            _this: *mut bgfx_encoder_t,
            _stage: u8,
            _handle: bgfx_dynamic_index_buffer_handle_t,
            _access: bgfx_access_t,
        ),
    >,
    pub encoder_set_compute_dynamic_vertex_buffer: ::std::option::Option<
        unsafe extern "C" fn(
            _this: *mut bgfx_encoder_t,
            _stage: u8,
            _handle: bgfx_dynamic_vertex_buffer_handle_t,
            _access: bgfx_access_t,
        ),
    >,
    pub encoder_set_compute_indirect_buffer: ::std::option::Option<
        unsafe extern "C" fn(
            _this: *mut bgfx_encoder_t,
            _stage: u8,
            _handle: bgfx_indirect_buffer_handle_t,
            _access: bgfx_access_t,
        ),
    >,
    pub encoder_set_image: ::std::option::Option<
        unsafe extern "C" fn(
            _this: *mut bgfx_encoder_t,
            _stage: u8,
            _handle: bgfx_texture_handle_t,
            _mip: u8,
            _access: bgfx_access_t,
            _format: bgfx_texture_format_t,
        ),
    >,
    pub encoder_dispatch: ::std::option::Option<
        unsafe extern "C" fn(
            _this: *mut bgfx_encoder_t,
            _id: bgfx_view_id_t,
            _program: bgfx_program_handle_t,
            _numX: u32,
            _numY: u32,
            _numZ: u32,
            _flags: u8,
        ),
    >,
    pub encoder_dispatch_indirect: ::std::option::Option<
        unsafe extern "C" fn(
            _this: *mut bgfx_encoder_t,
            _id: bgfx_view_id_t,
            _program: bgfx_program_handle_t,
            _indirectHandle: bgfx_indirect_buffer_handle_t,
            _start: u16,
            _num: u16,
            _flags: u8,
        ),
    >,
    pub encoder_discard:
        ::std::option::Option<unsafe extern "C" fn(_this: *mut bgfx_encoder_t, _flags: u8)>,
    pub encoder_blit: ::std::option::Option<
        unsafe extern "C" fn(
            _this: *mut bgfx_encoder_t,
            _id: bgfx_view_id_t,
            _dst: bgfx_texture_handle_t,
            _dstMip: u8,
            _dstX: u16,
            _dstY: u16,
            _dstZ: u16,
            _src: bgfx_texture_handle_t,
            _srcMip: u8,
            _srcX: u16,
            _srcY: u16,
            _srcZ: u16,
            _width: u16,
            _height: u16,
            _depth: u16,
        ),
    >,
    pub request_screen_shot: ::std::option::Option<
        unsafe extern "C" fn(
            _handle: bgfx_frame_buffer_handle_t,
            _filePath: *const ::std::os::raw::c_char,
        ),
    >,
    pub render_frame:
        ::std::option::Option<unsafe extern "C" fn(_msecs: i32) -> bgfx_render_frame_t>,
    pub set_platform_data:
        ::std::option::Option<unsafe extern "C" fn(_data: *const bgfx_platform_data_t)>,
    pub get_internal_data:
        ::std::option::Option<unsafe extern "C" fn() -> *const bgfx_internal_data_t>,
    pub override_internal_texture_ptr: ::std::option::Option<
        unsafe extern "C" fn(_handle: bgfx_texture_handle_t, _ptr: usize) -> usize,
    >,
    pub override_internal_texture: ::std::option::Option<
        unsafe extern "C" fn(
            _handle: bgfx_texture_handle_t,
            _width: u16,
            _height: u16,
            _numMips: u8,
            _format: bgfx_texture_format_t,
            _flags: u64,
        ) -> usize,
    >,
    pub set_marker:
        ::std::option::Option<unsafe extern "C" fn(_marker: *const ::std::os::raw::c_char)>,
    pub set_state: ::std::option::Option<unsafe extern "C" fn(_state: u64, _rgba: u32)>,
    pub set_condition: ::std::option::Option<
        unsafe extern "C" fn(_handle: bgfx_occlusion_query_handle_t, _visible: bool),
    >,
    pub set_stencil: ::std::option::Option<unsafe extern "C" fn(_fstencil: u32, _bstencil: u32)>,
    pub set_scissor: ::std::option::Option<
        unsafe extern "C" fn(_x: u16, _y: u16, _width: u16, _height: u16) -> u16,
    >,
    pub set_scissor_cached: ::std::option::Option<unsafe extern "C" fn(_cache: u16)>,
    pub set_transform: ::std::option::Option<
        unsafe extern "C" fn(_mtx: *const ::std::os::raw::c_void, _num: u16) -> u32,
    >,
    pub set_transform_cached: ::std::option::Option<unsafe extern "C" fn(_cache: u32, _num: u16)>,
    pub alloc_transform: ::std::option::Option<
        unsafe extern "C" fn(_transform: *mut bgfx_transform_t, _num: u16) -> u32,
    >,
    pub set_uniform: ::std::option::Option<
        unsafe extern "C" fn(
            _handle: bgfx_uniform_handle_t,
            _value: *const ::std::os::raw::c_void,
            _num: u16,
        ),
    >,
    pub set_index_buffer: ::std::option::Option<
        unsafe extern "C" fn(
            _handle: bgfx_index_buffer_handle_t,
            _firstIndex: u32,
            _numIndices: u32,
        ),
    >,
    pub set_dynamic_index_buffer: ::std::option::Option<
        unsafe extern "C" fn(
            _handle: bgfx_dynamic_index_buffer_handle_t,
            _firstIndex: u32,
            _numIndices: u32,
        ),
    >,
    pub set_transient_index_buffer: ::std::option::Option<
        unsafe extern "C" fn(
            _tib: *const bgfx_transient_index_buffer_t,
            _firstIndex: u32,
            _numIndices: u32,
        ),
    >,
    pub set_vertex_buffer: ::std::option::Option<
        unsafe extern "C" fn(
            _stream: u8,
            _handle: bgfx_vertex_buffer_handle_t,
            _startVertex: u32,
            _numVertices: u32,
        ),
    >,
    pub set_vertex_buffer_with_layout: ::std::option::Option<
        unsafe extern "C" fn(
            _stream: u8,
            _handle: bgfx_vertex_buffer_handle_t,
            _startVertex: u32,
            _numVertices: u32,
            _layoutHandle: bgfx_vertex_layout_handle_t,
        ),
    >,
    pub set_dynamic_vertex_buffer: ::std::option::Option<
        unsafe extern "C" fn(
            _stream: u8,
            _handle: bgfx_dynamic_vertex_buffer_handle_t,
            _startVertex: u32,
            _numVertices: u32,
        ),
    >,
    pub set_dynamic_vertex_buffer_with_layout: ::std::option::Option<
        unsafe extern "C" fn(
            _stream: u8,
            _handle: bgfx_dynamic_vertex_buffer_handle_t,
            _startVertex: u32,
            _numVertices: u32,
            _layoutHandle: bgfx_vertex_layout_handle_t,
        ),
    >,
    pub set_transient_vertex_buffer: ::std::option::Option<
        unsafe extern "C" fn(
            _stream: u8,
            _tvb: *const bgfx_transient_vertex_buffer_t,
            _startVertex: u32,
            _numVertices: u32,
        ),
    >,
    pub set_transient_vertex_buffer_with_layout: ::std::option::Option<
        unsafe extern "C" fn(
            _stream: u8,
            _tvb: *const bgfx_transient_vertex_buffer_t,
            _startVertex: u32,
            _numVertices: u32,
            _layoutHandle: bgfx_vertex_layout_handle_t,
        ),
    >,
    pub set_vertex_count: ::std::option::Option<unsafe extern "C" fn(_numVertices: u32)>,
    pub set_instance_data_buffer: ::std::option::Option<
        unsafe extern "C" fn(_idb: *const bgfx_instance_data_buffer_t, _start: u32, _num: u32),
    >,
    pub set_instance_data_from_vertex_buffer: ::std::option::Option<
        unsafe extern "C" fn(_handle: bgfx_vertex_buffer_handle_t, _startVertex: u32, _num: u32),
    >,
    pub set_instance_data_from_dynamic_vertex_buffer: ::std::option::Option<
        unsafe extern "C" fn(
            _handle: bgfx_dynamic_vertex_buffer_handle_t,
            _startVertex: u32,
            _num: u32,
        ),
    >,
    pub set_instance_count: ::std::option::Option<unsafe extern "C" fn(_numInstances: u32)>,
    pub set_texture: ::std::option::Option<
        unsafe extern "C" fn(
            _stage: u8,
            _sampler: bgfx_uniform_handle_t,
            _handle: bgfx_texture_handle_t,
            _flags: u32,
        ),
    >,
    pub touch: ::std::option::Option<unsafe extern "C" fn(_id: bgfx_view_id_t)>,
    pub submit: ::std::option::Option<
        unsafe extern "C" fn(
            _id: bgfx_view_id_t,
            _program: bgfx_program_handle_t,
            _depth: u32,
            _flags: u8,
        ),
    >,
    pub submit_occlusion_query: ::std::option::Option<
        unsafe extern "C" fn(
            _id: bgfx_view_id_t,
            _program: bgfx_program_handle_t,
            _occlusionQuery: bgfx_occlusion_query_handle_t,
            _depth: u32,
            _flags: u8,
        ),
    >,
    pub submit_indirect: ::std::option::Option<
        unsafe extern "C" fn(
            _id: bgfx_view_id_t,
            _program: bgfx_program_handle_t,
            _indirectHandle: bgfx_indirect_buffer_handle_t,
            _start: u16,
            _num: u16,
            _depth: u32,
            _flags: u8,
        ),
    >,
    pub set_compute_index_buffer: ::std::option::Option<
        unsafe extern "C" fn(
            _stage: u8,
            _handle: bgfx_index_buffer_handle_t,
            _access: bgfx_access_t,
        ),
    >,
    pub set_compute_vertex_buffer: ::std::option::Option<
        unsafe extern "C" fn(
            _stage: u8,
            _handle: bgfx_vertex_buffer_handle_t,
            _access: bgfx_access_t,
        ),
    >,
    pub set_compute_dynamic_index_buffer: ::std::option::Option<
        unsafe extern "C" fn(
            _stage: u8,
            _handle: bgfx_dynamic_index_buffer_handle_t,
            _access: bgfx_access_t,
        ),
    >,
    pub set_compute_dynamic_vertex_buffer: ::std::option::Option<
        unsafe extern "C" fn(
            _stage: u8,
            _handle: bgfx_dynamic_vertex_buffer_handle_t,
            _access: bgfx_access_t,
        ),
    >,
    pub set_compute_indirect_buffer: ::std::option::Option<
        unsafe extern "C" fn(
            _stage: u8,
            _handle: bgfx_indirect_buffer_handle_t,
            _access: bgfx_access_t,
        ),
    >,
    pub set_image: ::std::option::Option<
        unsafe extern "C" fn(
            _stage: u8,
            _handle: bgfx_texture_handle_t,
            _mip: u8,
            _access: bgfx_access_t,
            _format: bgfx_texture_format_t,
        ),
    >,
    pub dispatch: ::std::option::Option<
        unsafe extern "C" fn(
            _id: bgfx_view_id_t,
            _program: bgfx_program_handle_t,
            _numX: u32,
            _numY: u32,
            _numZ: u32,
            _flags: u8,
        ),
    >,
    pub dispatch_indirect: ::std::option::Option<
        unsafe extern "C" fn(
            _id: bgfx_view_id_t,
            _program: bgfx_program_handle_t,
            _indirectHandle: bgfx_indirect_buffer_handle_t,
            _start: u16,
            _num: u16,
            _flags: u8,
        ),
    >,
    pub discard: ::std::option::Option<unsafe extern "C" fn(_flags: u8)>,
    pub blit: ::std::option::Option<
        unsafe extern "C" fn(
            _id: bgfx_view_id_t,
            _dst: bgfx_texture_handle_t,
            _dstMip: u8,
            _dstX: u16,
            _dstY: u16,
            _dstZ: u16,
            _src: bgfx_texture_handle_t,
            _srcMip: u8,
            _srcX: u16,
            _srcY: u16,
            _srcZ: u16,
            _width: u16,
            _height: u16,
            _depth: u16,
        ),
    >,
}
pub type PFN_BGFX_GET_INTERFACE =
    ::std::option::Option<unsafe extern "C" fn(_version: u32) -> *mut bgfx_interface_vtbl_t>;
extern "C" {
    pub fn bgfx_get_interface(_version: u32) -> *mut bgfx_interface_vtbl_t;
}
pub type __builtin_va_list = [__va_list_tag; 1usize];
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct __va_list_tag {
    pub gp_offset: ::std::os::raw::c_uint,
    pub fp_offset: ::std::os::raw::c_uint,
    pub overflow_arg_area: *mut ::std::os::raw::c_void,
    pub reg_save_area: *mut ::std::os::raw::c_void,
}
