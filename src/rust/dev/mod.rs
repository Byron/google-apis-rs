#![feature(core)]
//! 
//! # Usage
//!
//! ```test_harness
//! extern crate "youtube3-dev" as youtube3;
//! extern crate hyper;
//! 
//! # #[test]
//! # fn test() {
//! # // TODO - generate !
//! # }
//! ```
use std::marker::PhantomData;
use std::borrow::BorrowMut;
use std::cell::RefCell;

use hyper;
use oauth2;

mod common;
pub mod videos;


/// Central instance to access all youtube related services
pub struct YouTube<C, NC, A> {
    client: RefCell<C>,
    auth: RefCell<A>,
    _m: PhantomData<NC>
}

impl<'a, C, NC, A> YouTube<C, NC, A>
    where  NC: hyper::net::NetworkConnector,
            C: BorrowMut<hyper::Client<NC>> + 'a,
            A: oauth2::GetToken {

    pub fn new(client: C, authenticator: A) -> YouTube<C, NC, A> {
        YouTube {
            client: RefCell::new(client),
            auth: RefCell::new(authenticator),
            _m: PhantomData,
        }
    }

    pub fn videos(&'a self) -> videos::Service<'a, C, NC, A> {
        videos::Service::new(&self.client, &self.auth)
    }
}


pub fn new<C, NC, A>(client: C, authenticator: A) -> YouTube<C, NC, A>
    where  NC: hyper::net::NetworkConnector,
            C: BorrowMut<hyper::Client<NC>>,
            A: oauth2::GetToken {
    YouTube::new(client, authenticator)
}

#[cfg(test)]
mod tests {
    use super::*;
    use hyper;
    use oauth2;

    use std::default::Default;


    #[test]
    fn instantiate() {
        let secret = <oauth2::ApplicationSecret as Default>::default();
        let auth = oauth2::Authenticator::new(
                        &secret, 
                        oauth2::DefaultAuthenticatorDelegate,
                        hyper::Client::new(),
                        <oauth2::MemoryStorage as Default>::default(),
                        None);
        let yt = YouTube::new(hyper::Client::new(), auth);

        let v = yt.videos().insert("snippet", &Default::default());
    }
}