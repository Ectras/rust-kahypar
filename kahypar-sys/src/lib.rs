#![allow(non_camel_case_types, dead_code, non_upper_case_globals)]

// link-cplusplus is needed to link the C++ standard library.
extern crate link_cplusplus;

include!(concat!(env!("OUT_DIR"), "/bindings.rs"));
