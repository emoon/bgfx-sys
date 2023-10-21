fn main() {
    let mut build = cc::Build::new();
    let env = std::env::var("TARGET").unwrap();

    // windows includes
    if env.contains("windows") {
        build.include("bx/include/compat/msvc");
        build.include("bgfx/3rdparty/directx-headers/include/directx");
        build.flag("/Zc:__cplusplus");
    } else if env.contains("darwin") {
        // macOS includes
        build.include("bx/include/compat/osx");
        build.flag("-std=c++14");
    } else if env.contains("emscripten") {
        build.flag("-std=c++14");
    } else {
    }

    // add shared include dirs
    build.include("bgfx/3rdparty/khronos");
    build.include("bgfx/3rdparty");
    build.include("bgfx/include");
    build.include("bx/include");
    build.include("bx/3rdparty");
    build.include("bimg/include");
    build.include("bimg/3rdparty");
    build.include("bimg/3rdparty/iqa/include");
    build.include("bimg/3rdparty/astc-encoder/include");
    build.include("bimg/3rdparty/tinyexr/deps/miniz");

    // defines - Currently not supporting WebGPU, GNM
    // OS support:
    // Windows - DX11
    // macOS - Metal
    // Posix - OpenGL
    // Android - OpenGL/Vulkan
    // Emscripten - OpenGLES
    // In the future it would be good to make this configurable instead

    build.define("BGFX_CONFIG_RENDERER_WEBGPU", "0");
    build.define("BGFX_CONFIG_RENDERER_GNM", "0");

    // Make it optional to enable bgfx debug setting
    /*
    #[cfg(feature = "bgfx-debug")]
    {
        build.define("BX_CONFIG_DEBUG", "1");
    }

    #[cfg(not(feature = "bgfx-debug"))]
    {
        build.define("BX_CONFIG_DEBUG", "0");
    }
    */

    build.define("BX_CONFIG_DEBUG", "0");

    // Don't include decode of ASTC to reduce code size and is unlikely a common use-case.
    build.define("BIMG_DECODE_ASTC", "0");

    // Never use multi-threading in emscripten
    if env.contains("emscripten") {
        build.define("BGFX_CONFIG_MULTITHREADED", "0");
    } else {
        // Optionally disable multi-threading
        #[cfg(feature = "bgfx-single-threaded")]
        {
            build.define("BGFX_CONFIG_MULTITHREADED", "0");
        }

        #[cfg(not(feature = "bgfx-single-threaded"))]
        {
            build.define("BGFX_CONFIG_MULTITHREADED", "1");
        }
    }

    if env.contains("windows") {
        build.define("BGFX_CONFIG_RENDERER_VULKAN", "1");
        build.define("BGFX_CONFIG_RENDERER_DIRECT3D11", "1");
        build.define("BGFX_CONFIG_RENDERER_DIRECT3D12", "1");
        build.define("BGFX_CONFIG_RENDERER_OPENGL", "1");
        build.define("_WIN32", None);
        build.define("_HAS_EXCEPTIONS", "0");
        build.define("_SCL_SECURE", "0");
        build.define("_SECURE_SCL", "0");
        build.define("__STDC_LIMIT_MACROS", None);
        build.define("__STDC_FORMAT_MACROS", None);
        build.define("__STDC_CONSTANT_MACROS", None);
        build.define("_CRT_SECURE_NO_WARNINGS", None);
        build.define("_CRT_SECURE_NO_DEPRECATE", None);
        build.warnings(false);
    } else if env.contains("darwin") {
        build.define("BGFX_CONFIG_RENDERER_VULKAN", "0");
        build.define("BGFX_CONFIG_RENDERER_METAL", "1");
    } else if env.contains("android") {
        build.define("BGFX_CONFIG_RENDERER_VULKAN", "1");
        build.define("BGFX_CONFIG_RENDERER_OPENGLES", "1");
    } else if env.contains("emscripten") {
        build.define("BGFX_CONFIG_RENDERER_OPENGL", "0");
        build.define("BGFX_CONFIG_RENDERER_OPENGLES", "1");
    } else {
        build.define("BGFX_CONFIG_RENDERER_VULKAN", "1");
        build.define("BGFX_CONFIG_RENDERER_OPENGL", "1");
    }

    // sources
    build.file("bx/src/amalgamated.cpp");
    build.file("bimg/src/image.cpp");
    build.file("bimg/src/image_cubemap_filter.cpp");
    build.file("bimg/src/image_decode.cpp");
    build.file("bimg/src/image_gnf.cpp");
    build.file("bgfx/src/bgfx.cpp");
    build.file("bgfx/src/vertexlayout.cpp");
    build.file("bgfx/src/debug_renderdoc.cpp");
    build.file("bgfx/src/topology.cpp");
    build.file("bgfx/src/shader.cpp");
    build.file("bgfx/src/shader_dxbc.cpp");
    build.file("bgfx/src/renderer_agc.cpp");
    build.file("bgfx/src/renderer_gnm.cpp");
    build.file("bgfx/src/renderer_webgpu.cpp");
    build.file("bgfx/src/renderer_nvn.cpp");
    build.file("bgfx/src/renderer_gl.cpp");
    build.file("bgfx/src/renderer_vk.cpp");
    build.file("bgfx/src/renderer_noop.cpp");
    build.file("bgfx/src/renderer_d3d9.cpp");
    build.file("bgfx/src/renderer_d3d11.cpp");
    build.file("bgfx/src/renderer_d3d12.cpp");

    if env.contains("windows") {
        build.file("bgfx/src/glcontext_wgl.cpp");
        build.file("bgfx/src/nvapi.cpp");
        build.file("bgfx/src/dxgi.cpp");
        build.file("bgfx/src/shader_dx9bc.cpp");
        build.file("bgfx/src/shader_spirv.cpp");
    } else if env.contains("darwin") {
        build.file("bgfx/src/renderer_mtl.mm");
    } else if env.contains("android") {
        build.file("bgfx/src/glcontext_egl.cpp");
    } else if env.contains("emscripten") {
        build.file("bgfx/src/glcontext_html5.cpp");
    } else {
        build.file("bgfx/src/glcontext_egl.cpp");
        build.cpp_link_stdlib("stdc++");
    }

    build.compile("bgfx_sys");

    // linker stuff
    if env.contains("windows") {
        // todo fixme
    } else if env.contains("darwin") {
        println!("cargo:rustc-link-lib=framework=Metal");
        println!("cargo:rustc-link-lib=framework=MetalKit");
        println!("cargo:rustc-link-lib=c++");
    } else if env.contains("android") {
        println!("cargo:rustc-link-lib=c++_shared");
        println!("cargo:rustc-link-lib=GLESv1_CM");
        println!("cargo:rustc-link-lib=GLESv2");
        println!("cargo:rustc-link-lib=EGL");
    } else {
        println!("cargo:rustc-link-lib=pthread");
        println!("cargo:rustc-link-lib=stdc++");
        println!("cargo:rustc-link-lib=GL");
        println!("cargo:rustc-link-lib=X11");
    }
}
