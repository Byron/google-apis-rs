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
use std::cell::RefCell;
use std::borrow::BorrowMut;
use std::default::Default;

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
        videos::Service::new(&self)
    }

    pub fn channel_sections(&'a self) -> ChannelSectionMethodsBuilder<'a, C, NC, A> {
        ChannelSectionMethodsBuilder { hub: &self }
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

    #[test] fn helper_test() {
        use std::default::Default;
        use oauth2::{Authenticator, DefaultAuthenticatorDelegate, ApplicationSecret, MemoryStorage};

        let secret: ApplicationSecret = Default::default();
        let auth = Authenticator::new(&secret, DefaultAuthenticatorDelegate,
                                      hyper::Client::new(),
                                      <MemoryStorage as Default>::default(), None);
        let mut hub = YouTube::new(hyper::Client::new(), auth);
        let result = hub.channel_sections().insert()
                     .delegate(&mut <DefaultDelegate as Default>::default())
                     .doit();
    }
}

pub struct ChannelSectionMethodsBuilder<'a, C, NC, A>
    where NC: 'a,
           C: 'a,
           A: 'a, {

    hub: &'a YouTube<C, NC, A>,
}

impl<'a, C, NC, A> ChannelSectionMethodsBuilder<'a, C, NC, A> {
    
    /// Create a builder to help you perform the following task:
    ///
    /// Adds a channelSection for the authenticated user's channel.    
    pub fn insert(&self) -> ChannelSectionInsertMethodBuilder<'a, C, NC, A> {
        ChannelSectionInsertMethodBuilder {
            hub: self.hub,
            _delegate: Default::default(),
        }
    }
}

pub struct ChannelSectionInsertMethodBuilder<'a, C, NC, A>
    where NC: 'a,
           C: 'a,
           A: 'a, {

    hub: &'a YouTube<C, NC, A>,
    _delegate: Option<&'a mut Delegate>,
}


impl<'a, C, NC, A> ChannelSectionInsertMethodBuilder<'a, C, NC, A> where NC: hyper::net::NetworkConnector, C: BorrowMut<hyper::Client<NC>> + 'a, A: oauth2::GetToken {

    /// Perform the operation you have build so far.
    /// TODO: Build actual call
    pub fn doit(mut self) -> () {
        if self._delegate.is_some() {
            self._delegate.as_mut().unwrap().connection_error(hyper::HttpError::HttpStatusError);
        }
    }

    pub fn delegate(mut self, new_value: &'a mut Delegate) -> ChannelSectionInsertMethodBuilder<'a, C, NC, A> {
        self._delegate = Some(new_value);
        self
    }

}

pub trait Delegate {

    /// Called whenever there is an HttpError, usually if there are network problems.
    /// 
    /// Return retry information.
    fn connection_error(&mut self, hyper::HttpError) -> oauth2::Retry {
        oauth2::Retry::Abort
    }
}

#[derive(Default)]
pub struct DefaultDelegate;

impl Delegate for DefaultDelegate {}
