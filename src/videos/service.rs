use std::cell::RefCell;
use std::borrow::BorrowMut;
use std::marker::PhantomData;

use rustc_serialize;

use hyper;

/// Reresents all aspects of a youtube video resource. May only be partially 
/// available
#[derive(RustcEncodable, RustcDecodable, Default, Clone)]
pub struct Video {
    pub snippet: Option<VideoSnippet>,
    pub recordingDetails: Option<VideoRecordingDetails>,
    pub status: Option<VideoStatus>,
}

#[allow(non_snake_case)]
#[derive(RustcEncodable, RustcDecodable, Default, Clone)]
pub struct VideoSnippet {
    pub categoryId: String,
    pub description: String,
    pub tags: Vec<String>,
    pub title: String,

    pub status: Option<VideoStatus>,
    pub recordingDetails: Option<VideoRecordingDetails>,
}

impl Video {
    fn parts(&self) -> String {
        let mut res = String::new();
        if self.status.is_some() {
            res = res + "status,";
        }
        if self.recordingDetails.is_some() {
            res = res + "recordingDetails";
        }
        if self.snippet.is_some() {
            res = res + "snippet,";
        }
        res
    }
}

#[allow(non_snake_case)]
#[derive(RustcEncodable, RustcDecodable, Default, Clone)]
pub struct VideoStatus {
    pub privacyStatus: String,
    pub embeddable: bool,
    pub license: String,
    pub publicStatsViewable: bool,
    pub publishAt: String,
}

#[allow(non_snake_case)]
#[derive(RustcEncodable, RustcDecodable, Default, Clone)]
pub struct VideoRecordingDetails {
    locationDescription: String,
    recordingDate: String,
}

#[allow(non_snake_case)]
#[derive(RustcEncodable, RustcDecodable, Default, Clone)]
pub struct GeoPoint {
    altitude: f64,
    latitude: f64,
    longitude: f64,
}

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

    pub fn insert(&self, parts: &str, video: &Video) -> VideosInsertBuilder<'a, C, NC> {
        VideosInsertBuilder {
            client: self.client,
            video: video.clone(),
            parts: parts.to_string(),
            _m: PhantomData,
        }
    }
}

pub struct VideosInsertBuilder<'a, C, NC> 
    where NC: 'a,
           C: 'a {

    client: &'a RefCell<C>,
    video: Video,
    parts: String,

    _m: PhantomData<NC>
}


impl<'a, C, NC> VideosInsertBuilder<'a, C, NC>
    where  NC: hyper::net::NetworkConnector,
            C: BorrowMut<hyper::Client<NC>> + 'a {

}



#[cfg(test)]
mod tests {
    use std::default::Default;
    use super::*;
    use hyper;

    use std::cell::RefCell;

    #[test]
    fn insert() {
        let c = RefCell::new(hyper::Client::new());
        let s = Service::new(&c);
        let v = <Video as Default>::default();
        // todo: set data
        let mut ib = s.insert("id, snippet", &v);
    }


}