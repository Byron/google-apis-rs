#![feature(core,io,old_path)]
#![allow(dead_code, deprecated, unused_features)]
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
    use std::old_path::BytesContainer;

    const EXPECTED: &'static str = 
"\r\n--MDuXWGyeE33QFXGchb2VFWc4Z7945d\r\n\
Content-Length: 50\r\n\
Content-Type: application/json\r\n\
\r\n\
foo\r\n\
--MDuXWGyeE33QFXGchb2VFWc4Z7945d\r\n\
Content-Length: 25\r\n\
Content-Type: application/plain\r\n\
\r\n\
bar\r\n\
--MDuXWGyeE33QFXGchb2VFWc4Z7945d";

    const EXPECTED_LEN: usize= 221;

    #[test]
    fn multi_part_reader() {
        let mut r1 = MockStream::with_input(b"foo");
        let mut r2 = MockStream::with_input(b"bar");
        let mut mpr: MultiPartReader = Default::default();

        mpr.add_part(&mut r1, 50, &"application/json".parse().unwrap())
           .add_part(&mut r2, 25, &"application/plain".parse().unwrap());

        let mut res = String::new();
        let r = mpr.read_to_string(&mut res);
        assert_eq!(res.len(), r.clone().unwrap());

        // NOTE: This CAN fail, as the underlying header hashmap is not sorted
        // As the test is just for dev, and doesn't run on travis, we are fine, 
        // for now. Possible solution would be to omit the size field (make it 
        // optional)
        // assert_eq!(res, EXPECTED);
        assert_eq!(r, Ok(EXPECTED_LEN));
    }

    #[test]
    fn multi_part_reader_single_byte_read() {
        let mut r1 = MockStream::with_input(b"foo");
        let mut r2 = MockStream::with_input(b"bar");
        let mut mpr: MultiPartReader = Default::default();

        mpr.add_part(&mut r1, 50, &"application/json".parse().unwrap())
           .add_part(&mut r2, 25, &"application/plain".parse().unwrap());

        let mut buf = &mut [0u8];
        let mut v = Vec::<u8>::new();
        while let Ok(br) = mpr.read(buf) {
            if br == 0 {
                break;
            }
            v.push(buf[0]);
        }
        println!("{:?}", v.len());
        println!("{:?}", v.container_as_str().unwrap());
        assert_eq!(v.len(), EXPECTED_LEN);
        assert_eq!(v.container_as_str().unwrap(), EXPECTED);
    }
}