extern crate bindgen;

use std::env;
use std::path::Path;

fn main() {
    println!("cargo:rustc-link-lib=speechd");
    let out_dir = env::var("OUT_DIR").unwrap();
    let _ = bindgen::builder()
        .no_unstable_rust()
        .header("wrapper.h")
        .constified_enum("SPDConnectionMode")
        .constified_enum("SPDPriority")
        .constified_enum("SPDVoiceType")
        .constified_enum("SPDDataMode")
        .constified_enum("SPDNotification")
        .constified_enum("SPDPunctuation")
        .constified_enum("SPDCapitalLetters")
        .constified_enum("SPDSpelling")
        .use_core()
        .generate().unwrap()
        .write_to_file(Path::new(&out_dir).join("speech_dispatcher_sys.rs"));
}
