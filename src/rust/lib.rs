#![feature(core,io,old_path, custom_derive, custom_attribute, plugin)]
#![allow(dead_code, deprecated, unused_features, unused_variables, unused_imports)]
//! library with code shared by all generated implementations
#![plugin(serde_macros)]
extern crate hyper;
extern crate mime;
extern crate "yup-oauth2" as oauth2;
extern crate serde;

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

    use serde::json;

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

        mpr.add_part(&mut r1, 50, "application/json".parse().unwrap())
           .add_part(&mut r2, 25, "application/plain".parse().unwrap());

        let mut res = String::new();
        let r = mpr.read_to_string(&mut res);
        assert_eq!(res.len(), r.clone().unwrap());

        // NOTE: This CAN fail, as the underlying header hashmap is not sorted
        // As the test is just for dev, and doesn't run on travis, we are fine, 
        // for now. Possible solution would be to omit the size field (make it 
        // optional)
        assert_eq!(r, Ok(EXPECTED_LEN));
        // assert_eq!(res, EXPECTED);
    }

    #[test]
    fn multi_part_reader_single_byte_read() {
        let mut r1 = MockStream::with_input(b"foo");
        let mut r2 = MockStream::with_input(b"bar");
        let mut mpr: MultiPartReader = Default::default();

        mpr.add_part(&mut r1, 50, "application/json".parse().unwrap())
           .add_part(&mut r2, 25, "application/plain".parse().unwrap());

        let mut buf = &mut [0u8];
        let mut v = Vec::<u8>::new();
        while let Ok(br) = mpr.read(buf) {
            if br == 0 {
                break;
            }
            v.push(buf[0]);
        }
        assert_eq!(v.len(), EXPECTED_LEN);
        // See above: headers are unordered
        // assert_eq!(v.container_as_str().unwrap(), EXPECTED);
    }

    #[test]
    fn serde() {
        #[derive(Default, Serialize, Deserialize)]
        struct Foo {
            opt: Option<String>,
            req: u32,
            opt_vec: Option<Vec<String>>,
            vec: Vec<String>,
        }

        let f: Foo = Default::default();
        json::to_string(&f).unwrap(); // should work

        let j = "{\"opt\":null,\"req\":0,\"vec\":[]}";
        let f: Foo = json::from_str(j).unwrap();

        // This fails, unless 'vec' is optional
        // let j = "{\"opt\":null,\"req\":0}";
        // let f: Foo = json::from_str(j).unwrap();

        #[derive(Default, Serialize, Deserialize)]
        struct Bar {
            #[serde(alias="snooSnoo")]
            snoo_snoo: String
        }
        json::to_string(&<Bar as Default>::default()).unwrap();


        let j = "{\"snooSnoo\":\"foo\"}";
        let b: Bar = json::from_str(&j).unwrap();
        assert_eq!(b.snoo_snoo, "foo");
    }
}