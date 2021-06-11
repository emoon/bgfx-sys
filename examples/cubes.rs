use bgfx_sys::*;
use glam::{EulerRot, Mat4, Vec3};
use glfw::{Action, Context, Key, Window};
use raw_window_handle::{HasRawWindowHandle, RawWindowHandle};
use std::{ffi::c_void, mem::MaybeUninit, path::PathBuf, time::Instant};

const WIDTH: usize = 1280;
const HEIGHT: usize = 720;

#[repr(packed)]
struct PosColorVertex {
    _x: f32,
    _y: f32,
    _z: f32,
    _abgr: u32,
}

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

#[cfg(target_os = "linux")]
unsafe fn update_platform_handle(pd: &mut bgfx_platform_data_t, window: &Window) {
    match window.raw_window_handle() {
        RawWindowHandle::Xlib(x_data) => {
            pd.ndt = x_data.display;
            pd.nwh = x_data.window as *mut c_void;
        }
        _ => panic!("Unsupported window type"),
    }
}

#[cfg(target_os = "windows")]
unsafe fn update_platform_handle(pd: &mut bgfx_platform_data_t, window: &Window) {
    match window.raw_window_handle() {
        RawWindowHandle::Windows(data) => {
            pd.nwh = data.hwnd as *mut c_void;
        }
        _ => panic!("Unsupported window type"),
    }
}

fn load_shader_file(name: &str) -> std::io::Result<Vec<u8>> {
    let mut path = PathBuf::with_capacity(512);
    path.push("bgfx/examples/runtime/shaders");

    match unsafe { bgfx_get_renderer_type() } {
        BGFX_RENDERER_TYPE_DIRECT3D11 => path.push("dx11"),
        BGFX_RENDERER_TYPE_OPENGL => path.push("glsl"),
        BGFX_RENDERER_TYPE_METAL => path.push("metal"),
        BGFX_RENDERER_TYPE_OPENGLES => path.push("essl"),
        e => panic!("Unsupported render type {}", e),
    }

    path.push(format!("{}.bin", name));

    let mut data = std::fs::read(path)?;
    data.push(0); // this is to terminate the data
    Ok(data)
}

// load shaders and create shader program
unsafe fn load_shader_program(vs: &str, ps: &str) -> std::io::Result<bgfx_program_handle_t> {
    let vs_data = load_shader_file(vs)?;
    let ps_data = load_shader_file(ps)?;

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

fn main() -> std::io::Result<()> {
    let mut glfw = glfw::init(glfw::FAIL_ON_ERRORS).unwrap();

    let (mut window, events) = glfw
        .create_window(
            WIDTH as _,
            HEIGHT as _,
            "cubes.rs bgfx-sys example - ESC to close",
            glfw::WindowMode::Windowed,
        )
        .expect("Failed to create GLFW window.");

    window.set_key_polling(true);
    window.make_current();

    unsafe {
        let pd = MaybeUninit::<bgfx_platform_data_t>::zeroed();
        let mut pd = pd.assume_init();

        update_platform_handle(&mut pd, &window);
        bgfx_set_platform_data(&pd);

        let init = MaybeUninit::<bgfx_init_t>::zeroed();
        let mut init = init.assume_init();
        bgfx_init_ctor(&mut init);

        init.type_ = BGFX_RENDERER_TYPE_COUNT;
        init.resolution.width = WIDTH as u32;
        init.resolution.height = HEIGHT as u32;
        init.resolution.reset = BGFX_RESET_VSYNC;
        init.platformData = pd;

        if !bgfx_init(&mut init) {
            panic!("failed to init bgfx");
        }

        bgfx_set_debug(BGFX_DEBUG_TEXT);
        bgfx_set_view_clear(0, BGFX_CLEAR_COLOR | BGFX_CLEAR_DEPTH, 0x103030ff, 1.0, 0);

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

        let vbh = bgfx_create_vertex_buffer(vertices_mem, &mut vertex_layout, BGFX_BUFFER_NONE);
        let ibh = bgfx_create_index_buffer(index_mem, BGFX_BUFFER_NONE);

        // create the shaders

        let shader_program = load_shader_program("vs_cubes", "fs_cubes")?;

        //BGFX_C_API const bgfx_memory_t* bgfx_make_ref(const void* _data, uint32_t _size);

        let mut old_size = (0, 0);

        let at = Vec3::new(0.0, 0.0, 0.0);
        let eye = Vec3::new(0.0, 0.0, -35.0);
        let up = Vec3::new(0.0, 1.0, 0.0);

        let state = BGFX_STATE_WRITE_R
            | BGFX_STATE_WRITE_G
            | BGFX_STATE_WRITE_B
            | BGFX_STATE_WRITE_A
            | BGFX_STATE_WRITE_Z
            | BGFX_STATE_DEPTH_TEST_LESS
            | BGFX_STATE_CULL_CW;

        let time = Instant::now();

        while !window.should_close() {
            glfw.poll_events();
            for (_, event) in glfw::flush_messages(&events) {
                if let glfw::WindowEvent::Key(Key::Escape, _, Action::Press, _) = event {
                    window.set_should_close(true)
                }
            }

            let t = time.elapsed().as_secs_f32();

            let size = window.get_framebuffer_size();

            if old_size != size {
                bgfx_reset(
                    size.0 as _,
                    size.1 as _,
                    BGFX_RESET_NONE,
                    BGFX_TEXTURE_FORMAT_COUNT,
                );
                old_size = size;
            }

            let aspect = size.0 as f32 / size.1 as f32;

            let persp = Mat4::perspective_lh(60.0 * (std::f32::consts::PI / 180.0), aspect, 0.1, 100.0);
            let view = Mat4::look_at_lh(eye, at, up);

            bgfx_set_view_rect(0, 0, 0, size.0 as _, size.1 as _);
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
                    bgfx_set_vertex_buffer(0, vbh, 0, std::u32::MAX);
                    bgfx_set_index_buffer(ibh, 0, std::u32::MAX);
                    bgfx_set_state(state, 0);
                    bgfx_submit(0, shader_program, 0, BGFX_DISCARD_ALL);
                }
            }

            bgfx_frame(false);
        }

        bgfx_shutdown();
    }

    Ok(())
}
