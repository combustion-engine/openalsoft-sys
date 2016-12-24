extern crate cmake;

use std::env;

const WIN32_LIBS: &'static [&'static str] = &[
    "ole32", "user32", "dsound", "winmm",
];

fn main() {
    let target = env::var("TARGET").unwrap();

    let mut config = cmake::Config::new(".");

    #[cfg(feature = "static")]
    config.define("LIBTYPE", "STATIC");

    config.define("ALSOFT_UTILS", "OFF");
    config.define("ALSOFT_EXAMPLES", "OFF");
    config.define("ALSOFT_TESTS", "OFF");

    #[cfg(feature = "hrtf")]
    config.define("ALSOFT_HRTF_DEFS", "ON");
    #[cfg(feature = "presets")]
    config.define("ALSOFT_AMBDEC_PRESETS", "ON");

    #[cfg(not(feature = "hrtf"))]
    config.define("ALSOFT_HRTF_DEFS", "OFF");
    #[cfg(not(feature = "presets"))]
    config.define("ALSOFT_AMBDEC_PRESETS", "OFF");

    config.define("ALSOFT_CONFIG", "OFF");
    config.define("ALSOFT_INSTALL", "ON");

    let dst = config.build();
    let lib = dst.join("lib");

    println!("cargo:rustc-link-search=native={}", lib.display());

    #[cfg(not(feature = "static"))]
    println!("cargo:rustc-link-lib=static=common");

    if target.contains("windows") {
        for lib in WIN32_LIBS {
            println!("cargo:rustc-link-lib=dylib={}", lib);
        }
    }
}