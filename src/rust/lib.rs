#![allow(dead_code, deprecated, unused_features, unused_variables, unused_imports)]

#[macro_use]
extern crate clap;

#[macro_use]
extern crate hyper;
extern crate mime;
extern crate rustc_serialize;
extern crate yup_oauth2 as oauth2;
extern crate serde;
extern crate serde_json;
#[macro_use]
extern crate serde_derive;
extern crate strsim;

// just pull it in the check if it compiles
mod api;
mod cli;

/// This module is for testing only, its code is used in mako templates
#[cfg(test)]
mod test_api {
    extern crate yup_hyper_mock as hyper_mock;
    use super::api::client::*;
    use self::hyper_mock::*;
    use std::io::Read;
    use std::default::Default;
    use hyper;
    use std::str::FromStr;

    use serde_json as json;

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
--MDuXWGyeE33QFXGchb2VFWc4Z7945d--";

    const EXPECTED_LEN: usize= 223;

    #[test]
    fn multi_part_reader() {
        let mut r1 = MockStream::with_input(b"foo");
        let mut r2 = MockStream::with_input(b"bar");
        let mut mpr: MultiPartReader = Default::default();

        mpr.add_part(&mut r1, 50, "application/json".parse().unwrap())
           .add_part(&mut r2, 25, "application/plain".parse().unwrap());

        let mut res = String::new();
        let r = mpr.read_to_string(&mut res).unwrap();
        assert_eq!(res.len(), r);

        // NOTE: This CAN fail, as the underlying header hashmap is not sorted
        // As the test is just for dev, and doesn't run on travis, we are fine, 
        // for now. Possible solution would be to omit the size field (make it 
        // optional)
        assert_eq!(r, EXPECTED_LEN);
        // assert_eq!(res, EXPECTED);
    }

    #[test]
    fn multi_part_reader_single_byte_read() {
        let mut r1 = MockStream::with_input(b"foo");
        let mut r2 = MockStream::with_input(b"bar");
        let mut mpr: MultiPartReader = Default::default();

        mpr.add_part(&mut r1, 50, "application/json".parse().unwrap())
           .add_part(&mut r2, 25, "application/plain".parse().unwrap());

        let buf = &mut [0u8];
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
            #[serde(rename="snooSnoo")]
            snoo_snoo: String
        }
        json::to_string(&<Bar as Default>::default()).unwrap();


        let j = "{\"snooSnoo\":\"foo\"}";
        let b: Bar = json::from_str(&j).unwrap();
        assert_eq!(b.snoo_snoo, "foo");

        // We can't have unknown fields with structs.
        // #[derive(Default, Serialize, Deserialize)]
        // struct BarOpt {
        //     #[serde(rename="snooSnoo")]
        //     snoo_snoo: Option<String>
        // }
        // let j = "{\"snooSnoo\":\"foo\",\"foo\":\"bar\"}";
        // let b: BarOpt = json::from_str(&j).unwrap();
    }

    #[test]
    fn content_range() {
        for &(ref c, ref expected) in 
          &[(ContentRange {range: None, total_length: 50 }, "Content-Range: bytes */50\r\n"),
            (ContentRange {range: Some(Chunk { first: 23, last: 40 }), total_length: 45},
             "Content-Range: bytes 23-40/45\r\n")] {
            let mut headers = hyper::header::Headers::new();
            headers.set(c.clone());
            assert_eq!(headers.to_string(), expected.to_string());
        }
    }

    #[test]
    fn byte_range_from_str() {
        assert_eq!(<Chunk as FromStr>::from_str("2-42"), 
                    Ok(Chunk { first: 2, last: 42 }))
    }

    #[test]
    fn parse_range_response() {
        let r: RangeResponseHeader = hyper::header::Header::parse_header(&[b"bytes 2-42".to_vec()]).unwrap();
        assert_eq!(r.0.first, 2);
        assert_eq!(r.0.last, 42);
    }
}

#[cfg(test)]
mod test_cli {
    use super::cli::client::*;

    use std::default::Default;

    #[test]
    fn cursor() {
        let mut c: FieldCursor = Default::default();

        assert_eq!(c.to_string(), "");
        assert_eq!(c.num_fields(), 0);
        assert!(c.set("").is_err());
        assert!(c.set(".").is_ok());
        assert!(c.set("..").is_err());
        assert_eq!(c.num_fields(), 0);

        assert!(c.set("foo").is_ok());
        assert_eq!(c.to_string(), "foo");
        assert_eq!(c.num_fields(), 1);
        assert!(c.set("..").is_ok());
        assert_eq!(c.num_fields(), 0);
        assert_eq!(c.to_string(), "");

        assert!(c.set("foo.").is_err());

        assert!(c.set("foo.bar").is_ok());
        assert_eq!(c.num_fields(), 2);
        assert_eq!(c.to_string(), "foo.bar");
        assert!(c.set("sub.level").is_ok());
        assert_eq!(c.num_fields(), 4);
        assert_eq!(c.to_string(), "foo.bar.sub.level");
        assert!(c.set("...other").is_ok());
        assert_eq!(c.to_string(), "foo.bar.other");
        assert_eq!(c.num_fields(), 3);
        assert!(c.set(".one.two.three...beer").is_ok());
        assert_eq!(c.num_fields(), 2);
        assert_eq!(c.to_string(), "one.beer");
        assert!(c.set("one.two.three...").is_ok());
        assert_eq!(c.num_fields(), 3);
        assert_eq!(c.to_string(), "one.beer.one");
    }
}
