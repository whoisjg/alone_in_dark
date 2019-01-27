#![allow(non_snake_case)]

use std::env;
use std::fs;
use std::path::{PathBuf};
use std::process::Command;

#[derive(PartialEq)]
enum Platform {
    Web,
    Desktop,
    RPI, // raspberry pie
}

#[derive(PartialEq)]
enum PlatformOS {
    Windows,
    Linux,
    BSD,
    OSX,
    Unknown, //
}

enum LibType {
    Static,
    // Shared,
}

enum BuildMode {
    // Release,
    Debug,
}

fn uname() -> String {
    String::from_utf8_lossy(
        &Command::new("uname")
            .output()
            .expect("failed to run uname")
            .stdout,
    )
    .trim()
    .to_owned()
}

fn main() {
    let out_dir = PathBuf::from(env::var("OUT_DIR").unwrap());
    let target = env::var("TARGET").unwrap();
    let release = env::var("PROFILE").unwrap().contains("release");

    // Set compiler defaults
    let mut compiler = None;
    let mut GLFW_CFLAGS = Vec::new();
    let mut CFLAGS = Vec::new();
    let mut CDEFINES = Vec::new();
    let mut INCLUDE_PATHS: Vec<String> = Vec::new();
    let mut LDFLAGS = Vec::new();

    // Set configruation defaults
    let  SHARED = false;
    let mut build_rglfw_object = false;
    let  INCLUDE_AUDIO_MODULE = false;
    let  build_raudio_object = INCLUDE_AUDIO_MODULE;
    let  build_mini_al_object = INCLUDE_AUDIO_MODULE;

    // Define required raylib variables
    let RAYLIB_VERSION = "2.0.0";
    let RAYLIB_API_VERSION = "2";

    let _RAYLIB_PATH = "..";

    let PLATFORM = if target.contains("wasm32") {
        // set env
        env::set_var("EMMAKEN_CFLAGS", "-s USE_GLFW=3");
        Platform::Web
    } else {
        Platform::Desktop
    };

    // Library type used for raylib: STATIC (.a) or SHARED (.so/.dll)
    let _RAYLIB_LIBTYPE = LibType::Static;

    // Build mode for library: DEBUG or RELEASE
    let _RAYLIB_BUILD_MODE = BuildMode::Debug;

    // Included raylib audio module on compilation
    // NOTE: Some programs like tools could not require audio support
    let _INCLUDE_AUDIO_MODULE = true;

    // Use external GLFW library instead of rglfw module
    // TODO: Review usage of examples on Linux.
    let USE_EXTERNAL_GLFW = false;

    // Use Wayland display server protocol on Linux desktop
    // by default it uses X11 windowing system
    let mut _USE_WAYLAND_DISPLAY = false;

    // TODO TRANSLATE THE BELOW
    //     # Use cross-compiler for PLATFORM_RPI
    // ifeq ($(PLATFORM),PLATFORM_RPI)
    //     USE_RPI_CROSS_COMPILER ?= FALSE
    //     ifeq ($(USE_RPI_CROSS_COMPILER),TRUE)
    //         RPI_TOOLCHAIN ?= C:/SysGCC/Raspberry
    //         RPI_TOOLCHAIN_SYSROOT ?= $(RPI_TOOLCHAIN)/arm-linux-gnueabihf/sysroot
    //     endif
    // endif

    // LINE 97
    let PLATFORM_OS = if PLATFORM == Platform::Desktop {
        // Determine PLATFORM_OS in case PLATFORM_DESKTOP selected
        if env::var("OS")
            .unwrap_or("".to_owned())
            .contains("Windows_NT")
        {
            // No uname.exe on MinGW!, but OS=Windows_NT on Windows!
            // ifeq ($(UNAME),Msys) -> Windows
            PlatformOS::Windows
        } else {
            let un: &str = &uname();
            match un {
                "Linux" => PlatformOS::Linux,
                "FreeBSD" => PlatformOS::BSD,
                "OpenBSD" => PlatformOS::BSD,
                "NetBSD" => PlatformOS::BSD,
                "DragonFly" => PlatformOS::BSD,
                "Darwin" => PlatformOS::OSX,
                _ => panic!("Unknown platform {}", uname()),
            }
        }
    } else if PLATFORM == Platform::RPI {
        PlatformOS::Linux
    } else {
        PlatformOS::Unknown
    };

    // TODO translate android stuff

    // LINE 210
    //Define raylib graphics api depending on selected platform
    let GRAPHICS = match PLATFORM {
        Platform::Desktop => "GRAPHICS_API_OPENGL_33",
        Platform::RPI => "GRAPHICS_API_OPENGL_ES2",
        Platform::Web => "GRAPHICS_API_OPENGL_ES2",
    };

    // making sure cc uses the same compiler as raylib
    if PLATFORM == Platform::Desktop {
        if PLATFORM_OS == PlatformOS::OSX {
            compiler = Some("clang");
            GLFW_CFLAGS.push("-x");
            GLFW_CFLAGS.push("objective-c");
        }
        if PLATFORM_OS == PlatformOS::BSD {
            compiler = Some("clang");
        }
    }
    // TODO translate raspberry pi stuff from line 246

    // LINE 254
    if PLATFORM == Platform::Web {
        println!("USINGEMCC");
        compiler = Some("emcc");
    }
    // TODO translate android stuff on LINE 259

    // LINE 271
    CFLAGS.append(&mut vec![
        "-O1",
        "-Wall",
        "-std=c11",
        "-Wno-missing-braces",
        "-Wno-unused-parameter",
        "-Wno-sign-compare",
        "-Wno-unused-function",
        "-Werror=pointer-arith",
        "-Wno-unused-variable",
        "-Wno-missing-field-initializers",
        "-Wno-implicit-function-declaration",
        "-fno-strict-aliasing",
    ]);
    CDEFINES.append(&mut vec!["_DEFAULT_SOURCE"]);

    if !release {
        // CFLAGS.push("-g");
    }

    if PLATFORM == Platform::Desktop {
        CFLAGS.push("-Werror=implicit-function-declaration");
    }

    if PLATFORM == Platform::Web {
        // custom flags
        CFLAGS.append(&mut vec![
            // "-s",
            // "ALLOW_MEMORY_GROWTH=1",
            // "-s",
            // "USE_PTHREADS=1",
            "-s",
            "TOTAL_MEMORY=16777216",
        ]);
        CFLAGS.append(&mut vec![
            "-s",
            "USE_GLFW=3",
            "-s",
            "ASSERTIONS=1",
            "--profiling",
        ]);
        // for bindgen
        CFLAGS.append(&mut vec![
            "-s",
            "WASM=1",
            "-s",
            "RELOCATABLE=1",
            "-s",
            "EMULATED_FUNCTION_POINTERS=1",
        ])
    }
    // TODO android LINE 305

    // LINE 322
    // Define required compilation flags for raylib SHARED lib
    if SHARED {
        // CFLAGS.append(&mut vec!["-fPIC", "-DBUILD_LIBTYPE_SHARED"]);
    }

    // Use Wayland display on Linux desktop
    if PLATFORM == Platform::Desktop && PLATFORM_OS == PlatformOS::Linux {
        _USE_WAYLAND_DISPLAY = true;
        CDEFINES.push("_GLFW_WAYLAND");
    }

    // Define include paths for required headers
    // NOTE: Several external required libraries (stb and others)
    INCLUDE_PATHS.push("raylib/src".to_owned());
    INCLUDE_PATHS.push("raylib/src/external/glfw/include".to_owned());
    INCLUDE_PATHS.push("raylib/src/external/glfw/deps/mingw".to_owned());

    let out_dir_ld = format!("-L{}", out_dir.to_str().unwrap());
    if PLATFORM == Platform::Desktop {
        if PLATFORM_OS == PlatformOS::BSD {
            INCLUDE_PATHS.push("/usr/local/include".to_owned());
            LDFLAGS.push("-Lraylib");
            LDFLAGS.push("-Lraylib/src");
            LDFLAGS.push(&out_dir_ld);
        }
        if USE_EXTERNAL_GLFW {
            LDFLAGS.push("-lglfw")
        }
    }

    // TODO Android and PI support # Define additional directories containing required header files
    // LINE 371

    if PLATFORM == Platform::Desktop && !USE_EXTERNAL_GLFW {
        build_rglfw_object = true;
    }

    // compilation
    let core_file = out_dir.join("raylib/src/core.o");
    let shapes_file = out_dir.join("raylib/src/shapes.o");
    let textures_file = out_dir.join("raylib/src/textures.o");
    let text_file = out_dir.join("raylib/src/text.o");
    let models_file = out_dir.join("raylib/src/models.o");
    let utils_file = out_dir.join("raylib/src/utils.o");
    let rglfw_file = out_dir.join("raylib/src/rglfw.o");
    let raudio_file = out_dir.join("raylib/src/raudio.o");
    let mini_al_file = out_dir.join("raylib/src/mini_al.o");

    let mut OBJS = vec![
        &core_file,
        &shapes_file,
        &textures_file,
        &text_file,
        &models_file,
        &utils_file,
    ];

    // compile core
    // if !Path::new(&core_file).exists() {
    compile_obj(
        "core",
        &compiler,
        &Vec::new(),
        &CFLAGS,
        &CDEFINES,
        &INCLUDE_PATHS,
        &PLATFORM,
        &Some(GRAPHICS),
    );
    // }

    // compile rglfw
    // if !Path::new(&rglfw_file).exists() {
    if build_rglfw_object {
        compile_obj(
            "rglfw",
            &compiler,
            &GLFW_CFLAGS,
            &CFLAGS,
            &CDEFINES,
            &INCLUDE_PATHS,
            &PLATFORM,
            &Some(GRAPHICS),
        );
        OBJS.push(&rglfw_file);
    }
    // }

    // compile shapes
    // if !Path::new(&shapes_file).exists() {
    compile_obj(
        "shapes",
        &compiler,
        &Vec::new(),
        &CFLAGS,
        &CDEFINES,
        &INCLUDE_PATHS,
        &PLATFORM,
        &Some(GRAPHICS),
    );
    // }

    // compile textures
    // if !Path::new(&textures_file).exists() {
    compile_obj(
        "textures",
        &compiler,
        &Vec::new(),
        &CFLAGS,
        &CDEFINES,
        &INCLUDE_PATHS,
        &PLATFORM,
        &Some(GRAPHICS),
    );
    // }

    // compile text
    // if !Path::new(&text_file).exists() {
    compile_obj(
        "text",
        &compiler,
        &Vec::new(),
        &CFLAGS,
        &CDEFINES,
        &INCLUDE_PATHS,
        &PLATFORM,
        &Some(GRAPHICS),
    );
    // }

    // compile models
    // if !Path::new(&models_file).exists() {
    compile_obj(
        "models",
        &compiler,
        &Vec::new(),
        &CFLAGS,
        &CDEFINES,
        &INCLUDE_PATHS,
        &PLATFORM,
        &Some(GRAPHICS),
    );
    // }

    // if !Path::new(&raudio_file).exists() {
    if build_raudio_object {
        // compile audio
        compile_obj(
            "raudio",
            &compiler,
            &Vec::new(),
            &CFLAGS,
            &CDEFINES,
            &INCLUDE_PATHS,
            &PLATFORM,
            &Some(GRAPHICS),
        );
        OBJS.push(&raudio_file);
    }
    // }

    // if !Path::new(&mini_al_file).exists() {
    if build_mini_al_object {
        // compile mini_al
        compile_obj(
            "mini_al",
            &compiler,
            &Vec::new(),
            &CFLAGS,
            &CDEFINES,
            &INCLUDE_PATHS,
            &PLATFORM,
            &Some(GRAPHICS),
        );
        OBJS.push(&mini_al_file);
    }
    // }

    // compile utils
    // if !Path::new(&utils_file).exists() {
    compile_obj(
        "utils",
        &compiler,
        &Vec::new(),
        &CFLAGS,
        &CDEFINES,
        &INCLUDE_PATHS,
        &PLATFORM,
        &Some(GRAPHICS),
    );
    // }

    // compile lib
    let mut builder = cc::Build::new();

    for obj in OBJS {
        builder.object(obj);
    }

    if let Some(c) = compiler {
        builder.compiler(c);
    }

    // flags
    builder
        // .flag("-dynamiclib")
        .flag("-compatibility_version")
        .flag(RAYLIB_API_VERSION)
        .flag("-current_version")
        .flag(RAYLIB_VERSION);
    builder.compile("raylib");

    println!("cargo:rustc-link-search=native={}", out_dir.display());

    // bindgen
    // Generate and write raylib bindings
    if PLATFORM == Platform::Desktop || PLATFORM == Platform::RPI {
        bindgen::Builder::default()
            .header("raylib/src/raylib.h")
            .clang_arg(format!("-I{}", out_dir.join("include").display()))
            .constified_enum_module("*")
            .generate()
            .expect("Failed to generate bindings")
            .write_to_file(out_dir.join("bindings.rs"))
            .expect("Failed to write bindings");
    } else {
        fs::write(out_dir.join("bindings.rs"), include_str!("bindings_web.rs")).expect("failed to write bindings");
    }

    // Generate cargo metadata for linking to raylib
    if PLATFORM == Platform::Desktop {
        if PLATFORM_OS == PlatformOS::Windows {
            println!(
                "cargo:rustc-link-search=native={}",
                out_dir.join("lib").display()
            );
            println!("cargo:rustc-link-lib=static=raylib");
            println!("cargo:rustc-link-lib=gdi32");
            println!("cargo:rustc-link-lib=user32");
        } else {
            // On other platforms read raylib.pc with pkg-config
            fs::write(out_dir.join("raylib.pc"), include_str!("raylib.pc")).expect("failed to write pkg-config");
            env::set_var("PKG_CONFIG_PATH", &out_dir);
            pkg_config::Config::new()
                .atleast_version(RAYLIB_VERSION)
                .statik(true)
                .arg(format!("--define-variable=prefix={}", out_dir.display()))
                .probe("raylib")
                .unwrap();
        }
    }
}

fn compile_obj(
    name: &str,
    compiler: &Option<&str>,
    glfw_cflags: &Vec<&str>,
    cflags: &Vec<&str>,
    cdefines: &Vec<&str>,
    include_paths: &Vec<String>,
    platform: &Platform,
    graphics: &Option<&str>,
) {
    let mut builder = cc::Build::new();

    if let Some(c) = compiler {
        builder.compiler(c);
    }
    // builder.compiler("emcc");

    // set glfw flags
    for flag in glfw_cflags {
        builder.flag(flag);
    }
    // set the cpp file
    builder.file(format!{"raylib/src/{}.c", name});
    // set the flags
    for flag in cflags {
        builder.flag(flag);
    }
    // set the define
    for def in cdefines {
        builder.define(def, None);
    }
    // set include paths
    for path in include_paths {
        builder.include(path);
    }
    let _ = match platform {
        Platform::Desktop => builder.define("PLATFORM_DESKTOP", None),
        Platform::Web => builder.define("PLATFORM_WEB", None),
        Platform::RPI => builder.define("PLATFORM_RPI", None),
    };
    if let Some(g) = graphics {
        builder.define(g, None);
        builder.define("GL_SILENCE_DEPRECATION", None);
    }
    builder.cargo_metadata(false);
    builder.compile(name)
}
