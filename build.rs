extern crate cmake;

use std::env;

fn main() {
    let target = env::var("TARGET").unwrap();

    let mut config = cmake::Config::new(".");

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
    config.define("ALSOFT_INSTALL", "OFF");

    let dst = config.build();

    println!("cargo:rustc-link-search=native={}", dst.join("lib").display());

    if target.contains("windows") {
        println!("cargo:rustc-link-lib=static=OpenAL32");
    } else {
        println!("cargo:rustc-link-lib=static=openal");
    }
}