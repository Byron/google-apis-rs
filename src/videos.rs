use std::cell::RefCell;
use std::borrow::BorrowMut;
use std::marker::PhantomData;

use hyper;

pub struct VideosService<'a, C, NC>
    where NC: 'a,
           C: 'a {

    client: &'a RefCell<C>,

    _m: PhantomData<NC>
}

impl<'a, C, NC> VideosService<'a, C, NC>
    where  NC: hyper::net::NetworkConnector,
            C: BorrowMut<hyper::Client<NC>> + 'a {

    pub fn new(client: &'a RefCell<C>) -> VideosService<'a, C, NC> {
        VideosService {
            client: client,
            _m: PhantomData,
        }
    }
}



#[cfg(test)]
mod tests {



}