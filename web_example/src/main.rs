use bgfx_sys::*;
use glam::{EulerRot, Mat4, Vec3};
use glfw::Window;
use std::{ffi::c_void, mem::MaybeUninit};
mod mainloop;

const WIDTH: usize = 1280;
const HEIGHT: usize = 720;

#[repr(packed)]
struct PosColorVertex {
    _x: f32,
    _y: f32,
    _z: f32,
    _abgr: u32,
}

static CANVAS_ID: &[u8; 7] = b"canvas\0";

static VS_CUBES: &[u8] = include_bytes!("../shaders/essl/vs_cubes.bin");
static FS_CUBES: &[u8] = include_bytes!("../shaders/essl/fs_cubes.bin");

#[rustfmt::skip]
static CUBE_VERTICES: [PosColorVertex; 8] = [
    PosColorVertex { _x: -1.0, _y:  1.0, _z:  1.0, _abgr: 0xff000000 },
    PosColorVertex { _x:  1.0, _y:  1.0, _z:  1.0, _abgr: 0xff0000ff },
    PosColorVertex { _x: -1.0, _y: -1.0, _z:  1.0, _abgr: 0xff00ff00 },
    PosColorVertex { _x:  1.0, _y: -1.0, _z:  1.0, _abgr: 0xff00ffff },
    PosColorVertex { _x: -1.0, _y:  1.0, _z: -1.0, _abgr: 0xffff0000 },
    PosColorVertex { _x:  1.0, _y:  1.0, _z: -1.0, _abgr: 0xffff00ff },
    PosColorVertex { _x: -1.0, _y: -1.0, _z: -1.0, _abgr: 0xffffff00 },
    PosColorVertex { _x:  1.0, _y: -1.0, _z: -1.0, _abgr: 0xffffffff },
];

#[rustfmt::skip]
static CUBE_INDICES: [u16; 36] = [
    0, 1, 2, // 0
    1, 3, 2,
    4, 6, 5, // 2
    5, 6, 7,
    0, 2, 4, // 4
    4, 2, 6,
    1, 5, 3, // 6
    5, 7, 3,
    0, 4, 1, // 8
    4, 5, 1,
    2, 3, 6, // 10
    6, 3, 7,
];

fn get_platform_data(_window: &Window) -> bgfx_platform_data_t {
    let pd = MaybeUninit::<bgfx_platform_data_t>::zeroed();
    let mut pd = unsafe { pd.assume_init() };
    pd.nwh = CANVAS_ID.as_ptr() as *mut _;
    pd
}

// load shaders and create shader program
unsafe fn load_shader_program(vs: &str, ps: &str) -> std::io::Result<bgfx_program_handle_t> {
    let vs_data = VS_CUBES;//load_shader_file(vs)?;
    let ps_data = FS_CUBES;//load_shader_file(ps)?;

    let vs_data = bgfx_copy(vs_data.as_ptr() as _, vs_data.len() as _);
    let ps_data = bgfx_copy(ps_data.as_ptr() as _, ps_data.len() as _);

    let vs_shader = bgfx_create_shader(vs_data);
    let ps_shader = bgfx_create_shader(ps_data);

    // very crude validation that we were able to create the shaders

    if vs_shader.idx == std::u16::MAX {
        panic!("Unable to create {} shader", vs);
    }

    if ps_shader.idx == std::u16::MAX {
        panic!("Unable to create {} shader", ps);
    }

    let program = bgfx_create_program(vs_shader, ps_shader, false);

    if ps_shader.idx == std::u16::MAX {
        panic!("Unable to create shader program {}:{} shader", vs, ps);
    }

    Ok(program)
}

struct State {
    vbh: bgfx_vertex_buffer_handle_t,
    ibh: bgfx_index_buffer_handle_t,
    shader_program: bgfx_program_handle_t,
    time: f32,
    glfw: glfw::Glfw,
}

fn main_callback(state: &mut State) {
    state.glfw.poll_events();

    let at = Vec3::new(0.0, 0.0, 0.0);
    let eye = Vec3::new(0.0, 0.0, -35.0);
    let up = Vec3::new(0.0, 1.0, 0.0);

    let render_state: u64 = BGFX_STATE_WRITE_R as u64
        | BGFX_STATE_WRITE_G as u64
        | BGFX_STATE_WRITE_B as u64
        | BGFX_STATE_WRITE_A as u64
        | BGFX_STATE_WRITE_Z as u64
        | BGFX_STATE_DEPTH_TEST_LESS as u64
        | BGFX_STATE_CULL_CW as u64;

    unsafe {
        bgfx_set_view_rect(0, 0, 0, WIDTH as _, HEIGHT as _);
        let aspect = WIDTH as f32 / HEIGHT as f32;

        // This should really be a timer, but it seems broken under wasm/emscripten
        let t = state.time;//.elapsed().as_secs_f32();
        state.time += 1.0 / 60.0; 

        let persp = Mat4::perspective_lh(60.0 * (std::f32::consts::PI / 180.0), aspect, 0.1, 100.0);
        let view = Mat4::look_at_lh(eye, at, up);

        bgfx_touch(0);

        let view_raw: *const c_void = std::mem::transmute(&view);
        let persp_raw: *const c_void = std::mem::transmute(&persp);

        bgfx_set_view_transform(0, view_raw, persp_raw);

        // set up for drawing

        for yy in 0..11 {
            for xx in 0..11 {
                let x = -15.0 + (xx as f32) * 3.0;
                let y = -15.0 + (yy as f32) * 3.0;
                let xr = t + (xx as f32) * 0.21;
                let yr = t + (yy as f32) * 0.37;

                let rot = Mat4::from_euler(EulerRot::XYZ, xr, yr, 0.0);
                let transform = Mat4::from_translation(Vec3::new(x, y, 0.0)) * rot;
                let trans_raw: *const c_void = std::mem::transmute(&transform);

                bgfx_set_transform(trans_raw, 1);
                bgfx_set_vertex_buffer(0, state.vbh, 0, std::u32::MAX);
                bgfx_set_index_buffer(state.ibh, 0, std::u32::MAX);
                bgfx_set_state(render_state, 0);
                bgfx_submit(0, state.shader_program, 0, BGFX_DISCARD_ALL as _);
            }
        }

        bgfx_frame(false);
    }
}

fn main() -> std::io::Result<()> {
    let mut glfw = glfw::init(glfw::FAIL_ON_ERRORS).unwrap();
    glfw.window_hint(glfw::WindowHint::ClientApi(glfw::ClientApiHint::NoApi));

    let (mut window, _events) = glfw
        .create_window(
            WIDTH as _,
            HEIGHT as _,
            "cubes.rs bgfx-sys example - ESC to close",
            glfw::WindowMode::Windowed,
        )
        .expect("Failed to create GLFW window.");

    window.set_key_polling(true);

    unsafe {
        let pd = get_platform_data(&window);
        bgfx_set_platform_data(&pd);

        let size_bgfx_init = std::mem::size_of::<bgfx_init_t>();
        println!("size of bgfx_init_t is {}", size_bgfx_init);

        let init = MaybeUninit::<bgfx_init_t>::zeroed();
        let mut init = init.assume_init();
        bgfx_init_ctor(&mut init);

        init.type_ = BGFX_RENDERER_TYPE_OPENGLES;
        init.resolution.width = WIDTH as u32;
        init.resolution.height = HEIGHT as u32;
        init.resolution.reset = BGFX_RESET_VSYNC;
        init.platformData = pd;

        if !bgfx_init(&init) {
            panic!("failed to init bgfx");
        }

        bgfx_set_debug(BGFX_DEBUG_TEXT);
        bgfx_set_view_clear(0, (BGFX_CLEAR_COLOR | BGFX_CLEAR_DEPTH) as _, 0x103030ff, 1.0, 0);

        // Setup vertex layout

        let vertex_layout = MaybeUninit::<bgfx_vertex_layout_t>::zeroed();
        let mut vertex_layout = vertex_layout.assume_init();

        bgfx_vertex_layout_begin(&mut vertex_layout, BGFX_RENDERER_TYPE_NOOP);
        bgfx_vertex_layout_add(
            &mut vertex_layout,
            BGFX_ATTRIB_POSITION,
            3,
            BGFX_ATTRIB_TYPE_FLOAT,
            false,
            false,
        );
        bgfx_vertex_layout_add(
            &mut vertex_layout,
            BGFX_ATTRIB_COLOR0,
            4,
            BGFX_ATTRIB_TYPE_UINT8,
            true,
            false,
        );
        bgfx_vertex_layout_end(&mut vertex_layout);

        // create vertex buffers

        let vertices_mem = bgfx_make_ref(
            CUBE_VERTICES.as_ptr() as _,
            (std::mem::size_of::<PosColorVertex>() * 8) as _,
        );
        let index_mem = bgfx_make_ref(
            CUBE_INDICES.as_ptr() as _,
            (std::mem::size_of::<u16>() * 36) as _,
        );

        let vbh = bgfx_create_vertex_buffer(vertices_mem, &vertex_layout, BGFX_BUFFER_NONE as _);
        let ibh = bgfx_create_index_buffer(index_mem, BGFX_BUFFER_NONE as _);

        let shader_program = load_shader_program("vs_cubes", "fs_cubes")?;

        bgfx_reset(WIDTH as _, HEIGHT as _, BGFX_RESET_NONE, BGFX_TEXTURE_FORMAT_COUNT);
        bgfx_set_view_rect(0, 0, 0, WIDTH as _, HEIGHT as _);

        let state = Box::new(State {
            vbh,
            ibh,
            shader_program,
            time: 0.0,
            glfw,
        });

        mainloop::run(main_callback, state);
    }

    Ok(())
}
