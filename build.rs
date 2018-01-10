extern crate bindgen;

use std::env;
use std::path::Path;

fn main() {
    println!("cargo:rustc-link-lib=speechd");
    let out_dir = env::var("OUT_DIR").unwrap();
    let _ = bindgen::builder()
        .header("wrapper.h")
        .rustified_enum("SPDConnectionMode")
        .rustified_enum("SPDPriority")
        .rustified_enum("SPDVoiceType")
        .rustified_enum("SPDDataMode")
        .rustified_enum("SPDNotification")
        .rustified_enum("SPDPunctuation")
        .rustified_enum("SPDCapitalLetters")
        .rustified_enum("SPDSpelling")
        .use_core()
        .generate().unwrap()
        .write_to_file(Path::new(&out_dir).join("speech_dispatcher_sys.rs"));
}
