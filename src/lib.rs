#![feature(core)]
//! # Usage
//! ```test_harness
//! extern crate youtube3;
//! extern crate hyper;
//! 
//! # #[test]
//! # fn test() {
//! let youtube = youtube3::new(hyper::Client::new());
//! youtube.videos();
//! # }
extern crate hyper;
extern crate "rustc-serialize" as rustc_serialize;

use std::marker::PhantomData;
use std::borrow::BorrowMut;
use std::cell::RefCell;

mod common;
pub mod videos;


/// Central instance to access all youtube related services
pub struct YouTube<C, NC> {
    client: RefCell<C>,

    _m: PhantomData<NC>
}

impl<'a, C, NC> YouTube<C, NC>
    where  NC: hyper::net::NetworkConnector,
            C: BorrowMut<hyper::Client<NC>> + 'a {

    pub fn new(client: C) -> YouTube<C, NC> {
        YouTube {
            client: RefCell::new(client),
            _m: PhantomData,
        }
    }

    pub fn videos(&'a self) -> videos::Service<'a, C, NC> {
        videos::Service::new(&self.client)
    }
}


pub fn new<C, NC>(client: C) -> YouTube<C, NC>
    where  NC: hyper::net::NetworkConnector,
            C: BorrowMut<hyper::Client<NC>> {
    YouTube::new(client)
}

#[cfg(test)]
mod tests {
    use super::*;
    use hyper;


    #[test]
    fn instantiate() {
        let yt = YouTube::new(hyper::Client::new());
        let v = yt.videos();

        let mut c = hyper::Client::new();
        let yt = YouTube::new(&mut c);
        let  v = yt.videos();
    }
}