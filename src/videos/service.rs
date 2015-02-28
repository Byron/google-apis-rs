use std::cell::RefCell;
use std::borrow::BorrowMut;
use std::marker::PhantomData;

use rustc_serialize;

use hyper;


/// The videos service - provides actual functionality through builders.
pub struct Service<'a, C, NC>
    where NC: 'a,
           C: 'a {

    client: &'a RefCell<C>,

    _m: PhantomData<NC>
}

impl<'a, C, NC> Service<'a, C, NC>
    where  NC: hyper::net::NetworkConnector,
            C: BorrowMut<hyper::Client<NC>> + 'a {

    pub fn new(client: &'a RefCell<C>) -> Service<'a, C, NC> {
        Service {
            client: client,
            _m: PhantomData,
        }
    }
}



#[cfg(test)]
mod tests {



}