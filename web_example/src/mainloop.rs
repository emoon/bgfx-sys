use core::ffi::c_void;
use std::mem::transmute;

#[repr(C)]
pub(crate) struct WrappedMainData {
    pub(crate) func: *const c_void,
}

type EmCallbackFunc = unsafe extern "C" fn(user_data: *mut c_void);

unsafe extern "C" fn mainloop_trampoline(func: *mut c_void) {
    let f: &mut &mut (dyn FnMut() + 'static) = transmute(func);
    f();
}

#[cfg(target_os = "emscripten")]
extern "C" {
    pub fn emscripten_set_main_loop_arg(func: EmCallbackFunc, data: *const c_void, fps: i32, simulate_infinite_loop: i32);
}
    
#[cfg(target_os = "emscripten")]
pub fn run<'a, F>(func: F) where F: FnMut() + 'a {
    // Having the data on the stack is safe as the mainloop only exits after the application is about to end
    let f: Box<Box<dyn FnMut() + 'a>> = Box::new(Box::new(func));
    let func = Box::into_raw(f) as *const _;

    /*
    let wrapped_data = BoxWrappedMainData {
        func: Box::into_raw(f) as *const _,
    };
    */

    unsafe {
        emscripten_set_main_loop_arg(mainloop_trampoline, func, 0, 1);
    }
}

#[cfg(not(target_os = "emscripten"))]
pub fn run<'a, F>(mut func: F) where F: FnMut() + 'a {
    func();
}


