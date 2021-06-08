use bgfx_sys::*;
use raw_window_handle::RawWindowHandle;
use std::ffi::c_void;
use std::mem::MaybeUninit;

use raw_window_handle::HasRawWindowHandle;

const WIDTH: usize = 640;
const HEIGHT: usize = 360;

#[cfg(target_os = "linux")]
unsafe fn update_platform_handle(pd: &mut bgfx_platform_data_t, window: &Window) {
    let raw_handle = window.raw_window_handle();

    match raw_handle {
        RawWindowHandle::Xlib(x_data) => {
            pd.ndt = x_data.display;
            pd.nwh = x_data.window as *mut c_void;
        }

        _ => panic!("Unsupported window type"),
    }
}

#[cfg(target_os = "windows")]
unsafe fn update_platform_handle(pd: &mut bgfx_platform_data_t, window: &Window) {
    let raw_handle = window.raw_window_handle();

    match raw_handle {
        RawWindowHandle::Windows(data) => {
            pd.nwh = data.hwnd as *mut c_void;
        }

        _ => panic!("Unsupported window type"),
    }
}

use glfw::{Action, Context, Key};

fn handle_window_event(window: &mut glfw::Window, event: glfw::WindowEvent) {
    match event {
        glfw::WindowEvent::Key(Key::Escape, _, Action::Press, _) => {
            window.set_should_close(true)
        }
        _ => {}
    }
}

fn main() {
    let mut glfw = glfw::init(glfw::FAIL_ON_ERRORS).unwrap();

    let (mut window, events) = glfw.create_window(WIDTH as _, HEIGHT as _, "Hello this is window", glfw::WindowMode::Windowed)
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
    }

    while !window.should_close() {
        glfw.poll_events();
        for (_, event) in glfw::flush_messages(&events) {
            handle_window_event(&mut window, event);
        }

        unsafe {
            bgfx_set_view_rect(0, 0, 0, WIDTH as _, HEIGHT as _);
            bgfx_touch(0);
            bgfx_frame(false);
        }
    }

    unsafe {
        bgfx_shutdown();
    }
}

