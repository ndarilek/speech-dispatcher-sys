#![feature(untagged_unions)]
#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

extern crate core;

include!(concat!(env!("OUT_DIR"), "/speech_dispatcher_sys.rs"));
