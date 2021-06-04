use bgfx_sys::*;
use minifb::*;
use raw_window_handle::RawWindowHandle;
use std::ffi::c_void;
use std::mem::MaybeUninit;

const WIDTH: usize = 640;
const HEIGHT: usize = 360;

fn main() {
    let mut window = Window::new(
        "Helloworld bgfx-sys - ESC to exit",
        WIDTH,
        HEIGHT,
        WindowOptions::default(),
    )
    .unwrap_or_else(|e| {
        panic!("{}", e);
    });

    //uint32_t debug  = BGFX_DEBUG_TEXT;
    //uint32_t reset  = BGFX_RESET_VSYNC;

    unsafe {
        let pd = MaybeUninit::<bgfx_platform_data_t>::zeroed();
        let mut pd = pd.assume_init();

        let raw_handle = window.raw_window_handle();

        match raw_handle {
            RawWindowHandle::Xlib(x_data) => {
                pd.ndt = x_data.display;
                pd.nwh = x_data.window as *mut c_void;
            }

            _ => panic!("Unsupported window type"),
        }

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

        // Enable debug text.
        bgfx_set_debug(BGFX_DEBUG_TEXT);

        bgfx_set_view_clear(0, BGFX_CLEAR_COLOR | BGFX_CLEAR_DEPTH, 0x303030ff, 1.0, 0);
    }

    // just make sure we sleep some in case we run unsynced
    window.limit_update_rate(Some(std::time::Duration::from_micros(1000)));

    while window.is_open() && !window.is_key_down(Key::Escape) {
        let size = window.get_size();

        unsafe {
            // Set view 0 default viewport.
            bgfx_set_view_rect(0, 0, 0, size.0 as u16, size.1 as u16);

            // This dummy draw call is here to make sure that view 0 is cleared
            // if no other draw calls are submitted to view 0.
            //let encoder = bgfx_encoder_begin(true);
            //bgfx_encoder_touch(encoder, 0);
            //bgfx_encoder_end(encoder);
            bgfx_touch(0);

        /*
        // Use debug font to print information about this example.
        bgfx_dbg_text_clear(0, false);
        bgfx_dbg_text_image(
              uint16_max( (uint16_t)width /2/8, 20)-20
            , uint16_max( (uint16_t)height/2/16, 6)-6
            , 40
            , 12
            , s_logo
            , 160
            );

        bgfx_dbg_text_printf(0, 1, 0x0f, "Color can be changed with ANSI \x1b[9;me\x1b[10;ms\x1b[11;mc\x1b[12;ma\x1b[13;mp\x1b[14;me\x1b[0m code too.");

        bgfx_dbg_text_printf(80, 1, 0x0f, "\x1b[;0m    \x1b[;1m    \x1b[; 2m    \x1b[; 3m    \x1b[; 4m    \x1b[; 5m    \x1b[; 6m    \x1b[; 7m    \x1b[0m");
        bgfx_dbg_text_printf(80, 2, 0x0f, "\x1b[;8m    \x1b[;9m    \x1b[;10m    \x1b[;11m    \x1b[;12m    \x1b[;13m    \x1b[;14m    \x1b[;15m    \x1b[0m");

        bgfx_dbg_text_printf(0, 3, 0x1f, "bgfx/examples/25-c99");
        bgfx_dbg_text_printf(0, 4, 0x3f, "Description: Initialization and debug text with C99 API.");

        // Advance to next frame. Rendering thread will be kicked to
        // process submitted rendering primitives.
        */

            bgfx_frame(false);
        }

        window.update();
    }

    unsafe {
        bgfx_shutdown();
    }
}
