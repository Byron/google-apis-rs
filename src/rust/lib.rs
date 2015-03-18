#![feature(core,io)]
#![allow(dead_code)]
//! library with code shared by all generated implementations
extern crate hyper;
extern crate mime;
extern crate "rustc-serialize" as rustc_serialize;
extern crate "yup-oauth2" as oauth2;

// just pull it in the check if it compiles
mod cmn;

/// This module is for testing only, its code is used in mako templates
#[cfg(test)]
mod tests {
    extern crate "yup-hyper-mock" as hyper_mock;
    use super::cmn::*;
    use self::hyper_mock::*;
    use std::io::Read;
    use std::default::Default;


    #[test]
    fn multi_part_reader() {
        let mut r1 = MockStream::with_input(b"foo");
        let mut r2 = MockStream::with_input(b"bar");
        let mut mpr: MultiPartReader = Default::default();

        mpr.add_part(&mut r1, 50, &"application/json".parse().unwrap())
           .add_part(&mut r2, 25, &"application/plain".parse().unwrap());

        let mut res = String::new();
        assert_eq!(mpr.read_to_string(&mut res), Ok(57));
        println!("{}", res);
        assert!(false);
    }
}