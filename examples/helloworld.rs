use std::{thread, time};
use bgfx_sys::*;
use glfw::{Action, Context, Key, Window};
use raw_window_handle::HasRawWindowHandle;
use raw_window_handle::RawWindowHandle;
use std::ffi::c_void;
use std::mem::MaybeUninit;

const WIDTH: usize = 1280;
const HEIGHT: usize = 720;

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

#[cfg(target_os = "linux")]
fn get_render_type() -> u32 {
    BGFX_RENDERER_TYPE_OPENGL
}

#[cfg(not(target_os = "linux"))]
fn get_render_type() -> u32 {
    BGFX_RENDERER_TYPE_COUNT
}

fn main() {
    let mut glfw = glfw::init(glfw::FAIL_ON_ERRORS).unwrap();
    glfw.window_hint(glfw::WindowHint::ClientApi(glfw::ClientApiHint::NoApi));

    let (mut window, events) = glfw
        .create_window(
            WIDTH as _,
            HEIGHT as _,
            "helloworld.rs bgfx-sys example - ESC to close",
            glfw::WindowMode::Windowed,
        )
        .expect("Failed to create GLFW window.");

    window.set_key_polling(true);

    unsafe {
        let pd = MaybeUninit::<bgfx_platform_data_t>::zeroed();
        let mut pd = pd.assume_init();

        update_platform_handle(&mut pd, &window);
        bgfx_set_platform_data(&pd);

        let init = MaybeUninit::<bgfx_init_t>::zeroed();
        let mut init = init.assume_init();
        bgfx_init_ctor(&mut init);

        init.type_ = get_render_type();
        init.resolution.width = WIDTH as u32;
        init.resolution.height = HEIGHT as u32;
        init.resolution.reset = BGFX_RESET_VSYNC;
        init.platformData = pd;

        if !bgfx_init(&mut init) {
            panic!("failed to init bgfx");
        }

        bgfx_set_debug(BGFX_DEBUG_TEXT);
        bgfx_set_view_clear(0, (BGFX_CLEAR_COLOR | BGFX_CLEAR_DEPTH) as _, 0x103030ff, 1.0, 0);
    }

    let mut old_size = (0, 0);

    while !window.should_close() {
        glfw.poll_events();
        for (_, event) in glfw::flush_messages(&events) {
            if let glfw::WindowEvent::Key(Key::Escape, _, Action::Press, _) = event {
                window.set_should_close(true)
            }
        }

        let size = window.get_framebuffer_size();

        unsafe {
            if old_size != size {
                bgfx_reset(
                    size.0 as _,
                    size.1 as _,
                    BGFX_RESET_NONE,
                    BGFX_TEXTURE_FORMAT_COUNT,
                );
                old_size = size;
            }

            bgfx_set_view_rect(0, 0, 0, size.0 as _, size.1 as _);
            bgfx_touch(0);

            /*
            bgfx_dbg_text_clear(0, false);

            bgfx_dbg_text_printf(0, 1, 0x0f, b"Color can be changed with ANSI \x1b[9;me\x1b[10;ms\x1b[11;mc\x1b[12;ma\x1b[13;mp\x1b[14;me\x1b[0m code too.\0".as_ptr() as _);
            bgfx_dbg_text_printf(80, 1, 0x0f, b"\x1b[;0m    \x1b[;1m    \x1b[; 2m    \x1b[; 3m    \x1b[; 4m    \x1b[; 5m    \x1b[; 6m    \x1b[; 7m    \x1b[0m\0".as_ptr() as _);
            bgfx_dbg_text_printf(80, 2, 0x0f, b"\x1b[;8m    \x1b[;9m    \x1b[;10m    \x1b[;11m    \x1b[;12m    \x1b[;13m    \x1b[;14m    \x1b[;15m    \x1b[0m\0".as_ptr() as _);
            bgfx_dbg_text_printf(
                0,
                4,
                0x3f,
                b"Description: Initialization and debug text with bgfx-sys Rust API.\0".as_ptr()
                    as _,
            );
            */

            bgfx_frame(false);
        }

        //thread::sleep(time::Duration::from_millis(10));
    }

    unsafe {
        bgfx_shutdown();
    }
}
