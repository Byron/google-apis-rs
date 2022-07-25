// COPY OF 'src/rust/api/client.rs'
// DO NOT EDIT
use std::error;
use std::error::Error as StdError;
use std::fmt::{self, Display};
use std::io::{self, Cursor, Read, Seek, SeekFrom, Write};
use std::str::FromStr;
use std::thread::sleep;
use std::time::Duration;

use itertools::Itertools;

use http::Uri;

use hyper::body::Buf;
use hyper::client::connect;
use hyper::header::{HeaderMap, AUTHORIZATION, CONTENT_LENGTH, CONTENT_TYPE, USER_AGENT};
use hyper::Method;
use hyper::StatusCode;

use mime::{Attr, Mime, SubLevel, TopLevel, Value};

use serde_json as json;

use tokio::io::{AsyncRead, AsyncWrite};
use tower_service;

const LINE_ENDING: &str = "\r\n";

pub enum Retry {
    /// Signal you don't want to retry
    Abort,
    /// Signals you want to retry after the given duration
    After(Duration),
}

/// Identifies the Hub. There is only one per library, this trait is supposed
/// to make intended use more explicit.
/// The hub allows to access all resource methods more easily.
pub trait Hub {}

/// Identifies types for building methods of a particular resource type
pub trait MethodsBuilder {}

/// Identifies types which represent builders for a particular resource method
pub trait CallBuilder {}

/// Identifies types which can be inserted and deleted.
/// Types with this trait are most commonly used by clients of this API.
pub trait Resource {}

/// Identifies types which are used in API responses.
pub trait ResponseResult {}

/// Identifies types which are used in API requests.
pub trait RequestValue {}

/// Identifies types which are not actually used by the API
/// This might be a bug within the google API schema.
pub trait UnusedType {}

/// Identifies types which are only used as part of other types, which
/// usually are carrying the `Resource` trait.
pub trait Part {}

/// Identifies types which are only used by other types internally.
/// They have no special meaning, this trait just marks them for completeness.
pub trait NestedType {}

/// A utility to specify reader types which provide seeking capabilities too
pub trait ReadSeek: Seek + Read + Send {}
impl<T: Seek + Read + Send> ReadSeek for T {}

/// A trait for all types that can convert themselves into a *parts* string
pub trait ToParts {
    fn to_parts(&self) -> String;
}

/// A trait specifying functionality to help controlling any request performed by the API.
/// The trait has a conservative default implementation.
///
/// It contains methods to deal with all common issues, as well with the ones related to
/// uploading media
pub trait Delegate: Send {
    /// Called at the beginning of any API request. The delegate should store the method
    /// information if he is interesting in knowing more context when further calls to it
    /// are made.
    /// The matching `finished()` call will always be made, no matter whether or not the API
    /// request was successful. That way, the delegate may easily maintain a clean state
    /// between various API calls.
    fn begin(&mut self, _info: MethodInfo) {}

    /// Called whenever there is an [HttpError](hyper::Error), usually if there are network problems.
    ///
    /// If you choose to retry after a duration, the duration should be chosen using the
    /// [exponential backoff algorithm](http://en.wikipedia.org/wiki/Exponential_backoff).
    ///
    /// Return retry information.
    fn http_error(&mut self, _err: &hyper::Error) -> Retry {
        Retry::Abort
    }

    /// Called whenever there is the need for your applications API key after
    /// the official authenticator implementation didn't provide one, for some reason.
    /// If this method returns None as well, the underlying operation will fail
    fn api_key(&mut self) -> Option<String> {
        None
    }

    /// Called whenever the Authenticator didn't yield a token. The delegate
    /// may attempt to provide one, or just take it as a general information about the
    /// impending failure.
    /// The given Error provides information about why the token couldn't be acquired in the
    /// first place
    fn token(&mut self, err: &oauth2::Error) -> Option<oauth2::AccessToken> {
        let _ = err;
        None
    }

    /// Called during resumable uploads to provide a URL for the impending upload.
    /// It was saved after a previous call to `store_upload_url(...)`, and if not None,
    /// will be used instead of asking the server for a new upload URL.
    /// This is useful in case a previous resumable upload was aborted/canceled, but should now
    /// be resumed.
    /// The returned URL will be used exactly once - if it fails again and the delegate allows
    /// to retry, we will ask the server for a new upload URL.
    fn upload_url(&mut self) -> Option<String> {
        None
    }

    /// Called after we have retrieved a new upload URL for a resumable upload to store it
    /// in case we fail or cancel. That way, we can attempt to resume the upload later,
    /// see `upload_url()`.
    /// It will also be called with None after a successful upload, which allows the delegate
    /// to forget the URL. That way, we will not attempt to resume an upload that has already
    /// finished.
    fn store_upload_url(&mut self, url: Option<&str>) {
        let _ = url;
    }

    /// Called whenever a server response could not be decoded from json.
    /// It's for informational purposes only, the caller will return with an error
    /// accordingly.
    ///
    /// # Arguments
    ///
    /// * `json_encoded_value` - The json-encoded value which failed to decode.
    /// * `json_decode_error`  - The decoder error
    fn response_json_decode_error(
        &mut self,
        json_encoded_value: &str,
        json_decode_error: &json::Error,
    ) {
        let _ = json_encoded_value;
        let _ = json_decode_error;
    }

    /// Called whenever the http request returns with a non-success status code.
    /// This can involve authentication issues, or anything else that very much
    /// depends on the used API method.
    /// The delegate should check the status, header and decoded json error to decide
    /// whether to retry or not. In the latter case, the underlying call will fail.
    ///
    /// If you choose to retry after a duration, the duration should be chosen using the
    /// [exponential backoff algorithm](http://en.wikipedia.org/wiki/Exponential_backoff).
    fn http_failure(
        &mut self,
        _: &hyper::Response<hyper::body::Body>,
        _err: Option<serde_json::Value>,
    ) -> Retry {
        Retry::Abort
    }

    /// Called prior to sending the main request of the given method. It can be used to time
    /// the call or to print progress information.
    /// It's also useful as you can be sure that a request will definitely be made.
    fn pre_request(&mut self) {}

    /// Return the size of each chunk of a resumable upload.
    /// Must be a power of two, with 1<<18 being the smallest allowed chunk size.
    /// Will be called once before starting any resumable upload.
    fn chunk_size(&mut self) -> u64 {
        1 << 23
    }

    /// Called before the given chunk is uploaded to the server.
    /// If true is returned, the upload will be interrupted.
    /// However, it may be resumable if you stored the upload URL in a previous call
    /// to `store_upload_url()`
    fn cancel_chunk_upload(&mut self, chunk: &ContentRange) -> bool {
        let _ = chunk;
        false
    }

    /// Called before the API request method returns, in every case. It can be used to clean up
    /// internal state between calls to the API.
    /// This call always has a matching call to `begin(...)`.
    ///
    /// # Arguments
    ///
    /// * `is_success` - a true value indicates the operation was successful. If false, you should
    ///                  discard all values stored during `store_upload_url`.
    fn finished(&mut self, is_success: bool) {
        let _ = is_success;
    }
}

/// A delegate with a conservative default implementation, which is used if no other delegate is
/// set.
#[derive(Default)]
pub struct DefaultDelegate;

impl Delegate for DefaultDelegate {}

#[derive(Debug)]
pub enum Error {
    /// The http connection failed
    HttpError(hyper::Error),

    /// An attempt was made to upload a resource with size stored in field `.0`
    /// even though the maximum upload size is what is stored in field `.1`.
    UploadSizeLimitExceeded(u64, u64),

    /// Represents information about a request that was not understood by the server.
    /// Details are included.
    BadRequest(serde_json::Value),

    /// We needed an API key for authentication, but didn't obtain one.
    /// Neither through the authenticator, nor through the Delegate.
    MissingAPIKey,

    /// We required a Token, but didn't get one from the Authenticator
    MissingToken(oauth2::Error),

    /// The delgate instructed to cancel the operation
    Cancelled,

    /// An additional, free form field clashed with one of the built-in optional ones
    FieldClash(&'static str),

    /// Shows that we failed to decode the server response.
    /// This can happen if the protocol changes in conjunction with strict json decoding.
    JsonDecodeError(String, json::Error),

    /// Indicates an HTTP repsonse with a non-success status code
    Failure(hyper::Response<hyper::body::Body>),

    /// An IO error occurred while reading a stream into memory
    Io(std::io::Error),
}

impl Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Error::Io(ref err) => err.fmt(f),
            Error::HttpError(ref err) => err.fmt(f),
            Error::UploadSizeLimitExceeded(ref resource_size, ref max_size) => writeln!(
                f,
                "The media size {} exceeds the maximum allowed upload size of {}",
                resource_size, max_size
            ),
            Error::MissingAPIKey => {
                (writeln!(
                    f,
                    "The application's API key was not found in the configuration"
                ))
                .ok();
                writeln!(
                    f,
                    "It is used as there are no Scopes defined for this method."
                )
            }
            Error::BadRequest(ref message) => {
                writeln!(f, "Bad Request: {}", message)?;
                Ok(())
            }
            Error::MissingToken(ref err) => {
                writeln!(f, "Token retrieval failed with error: {}", err)
            }
            Error::Cancelled => writeln!(f, "Operation cancelled by delegate"),
            Error::FieldClash(field) => writeln!(
                f,
                "The custom parameter '{}' is already provided natively by the CallBuilder.",
                field
            ),
            Error::JsonDecodeError(ref json_str, ref err) => writeln!(f, "{}: {}", err, json_str),
            Error::Failure(ref response) => {
                writeln!(f, "Http status indicates failure: {:?}", response)
            }
        }
    }
}

impl error::Error for Error {
    fn source(&self) -> Option<&(dyn error::Error + 'static)> {
        match *self {
            Error::HttpError(ref err) => err.source(),
            Error::JsonDecodeError(_, ref err) => err.source(),
            _ => None,
        }
    }
}

impl From<std::io::Error> for Error {
    fn from(err: std::io::Error) -> Self {
        Error::Io(err)
    }
}

/// A universal result type used as return for all calls.
pub type Result<T> = std::result::Result<T, Error>;

/// Contains information about an API request.
pub struct MethodInfo {
    pub id: &'static str,
    pub http_method: Method,
}

const BOUNDARY: &str = "MDuXWGyeE33QFXGchb2VFWc4Z7945d";

/// Provides a `Read` interface that converts multiple parts into the protocol
/// identified by [RFC2387](https://tools.ietf.org/html/rfc2387).
/// **Note**: This implementation is just as rich as it needs to be to perform uploads
/// to google APIs, and might not be a fully-featured implementation.
#[derive(Default)]
pub struct MultiPartReader<'a> {
    raw_parts: Vec<(HeaderMap, &'a mut (dyn Read + Send))>,
    current_part: Option<(Cursor<Vec<u8>>, &'a mut (dyn Read + Send))>,
    last_part_boundary: Option<Cursor<Vec<u8>>>,
}

impl<'a> MultiPartReader<'a> {
    /// Reserve memory for exactly the given amount of parts
    pub fn reserve_exact(&mut self, cap: usize) {
        self.raw_parts.reserve_exact(cap);
    }

    /// Add a new part to the queue of parts to be read on the first `read` call.
    ///
    /// # Arguments
    ///
    /// `headers` - identifying the body of the part. It's similar to the header
    ///             in an ordinary single-part call, and should thus contain the
    ///             same information.
    /// `reader`  - a reader providing the part's body
    /// `size`    - the amount of bytes provided by the reader. It will be put onto the header as
    ///             content-size.
    /// `mime`    - It will be put onto the content type
    pub fn add_part(
        &mut self,
        reader: &'a mut (dyn Read + Send),
        size: u64,
        mime_type: Mime,
    ) -> &mut MultiPartReader<'a> {
        let mut headers = HeaderMap::new();
        headers.insert(
            CONTENT_TYPE,
            hyper::header::HeaderValue::from_str(&format!("{}", mime_type)).unwrap(),
        );
        headers.insert(CONTENT_LENGTH, size.into());
        self.raw_parts.push((headers, reader));
        self
    }

    /// Returns the mime-type representing our multi-part message.
    /// Use it with the ContentType header.
    pub fn mime_type(&self) -> Mime {
        Mime(
            TopLevel::Multipart,
            SubLevel::Ext("related".to_string()),
            vec![(
                Attr::Ext("boundary".to_string()),
                Value::Ext(BOUNDARY.to_string()),
            )],
        )
    }

    /// Returns true if we are totally used
    fn is_depleted(&self) -> bool {
        self.raw_parts.is_empty()
            && self.current_part.is_none()
            && self.last_part_boundary.is_none()
    }

    /// Returns true if we are handling our last part
    fn is_last_part(&self) -> bool {
        self.raw_parts.is_empty() && self.current_part.is_some()
    }
}

impl<'a> Read for MultiPartReader<'a> {
    fn read(&mut self, buf: &mut [u8]) -> io::Result<usize> {
        match (
            self.raw_parts.len(),
            self.current_part.is_none(),
            self.last_part_boundary.is_none(),
        ) {
            (_, _, false) => {
                let br = self
                    .last_part_boundary
                    .as_mut()
                    .unwrap()
                    .read(buf)
                    .unwrap_or(0);
                if br < buf.len() {
                    self.last_part_boundary = None;
                }
                return Ok(br);
            }
            (0, true, true) => return Ok(0),
            (n, true, _) if n > 0 => {
                let (headers, reader) = self.raw_parts.remove(0);
                let mut c = Cursor::new(Vec::<u8>::new());
                // TODO: The first line ending should be omitted for the first part,
                // fortunately Google's API serves don't seem to mind.
                (write!(
                    &mut c,
                    "{}--{}{}{}{}{}",
                    LINE_ENDING,
                    BOUNDARY,
                    LINE_ENDING,
                    headers
                        .iter()
                        .map(|(k, v)| format!("{}: {}", k, v.to_str().unwrap()))
                        .join(LINE_ENDING),
                    LINE_ENDING,
                    LINE_ENDING,
                ))
                .unwrap();
                c.seek(SeekFrom::Start(0)).unwrap();
                self.current_part = Some((c, reader));
            }
            _ => {}
        }

        // read headers as long as possible
        let (hb, rr) = {
            let &mut (ref mut c, ref mut reader) = self.current_part.as_mut().unwrap();
            let b = c.read(buf).unwrap_or(0);
            (b, reader.read(&mut buf[b..]))
        };

        match rr {
            Ok(bytes_read) => {
                if hb < buf.len() && bytes_read == 0 {
                    if self.is_last_part() {
                        // before clearing the last part, we will add the boundary that
                        // will be written last
                        self.last_part_boundary = Some(Cursor::new(
                            format!("{}--{}--{}", LINE_ENDING, BOUNDARY, LINE_ENDING).into_bytes(),
                        ))
                    }
                    // We are depleted - this can trigger the next part to come in
                    self.current_part = None;
                }
                let mut total_bytes_read = hb + bytes_read;
                while total_bytes_read < buf.len() && !self.is_depleted() {
                    match self.read(&mut buf[total_bytes_read..]) {
                        Ok(br) => total_bytes_read += br,
                        Err(err) => return Err(err),
                    }
                }
                Ok(total_bytes_read)
            }
            Err(err) => {
                // fail permanently
                self.current_part = None;
                self.last_part_boundary = None;
                self.raw_parts.clear();
                Err(err)
            }
        }
    }
}

/// The `X-Upload-Content-Type` header.
///
/// Generated via rustc --pretty expanded -Z unstable-options, and manually
/// processed to be more readable.
#[derive(PartialEq, Debug, Clone)]
pub struct XUploadContentType(pub Mime);

impl ::std::ops::Deref for XUploadContentType {
    type Target = Mime;
    fn deref(&self) -> &Mime {
        &self.0
    }
}
impl ::std::ops::DerefMut for XUploadContentType {
    fn deref_mut(&mut self) -> &mut Mime {
        &mut self.0
    }
}
impl Display for XUploadContentType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        fmt::Display::fmt(&**self, f)
    }
}

#[derive(Clone, PartialEq, Debug)]
pub struct Chunk {
    pub first: u64,
    pub last: u64,
}

impl fmt::Display for Chunk {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        (write!(fmt, "{}-{}", self.first, self.last)).ok();
        Ok(())
    }
}

impl FromStr for Chunk {
    type Err = &'static str;

    /// NOTE: only implements `%i-%i`, not `*`
    fn from_str(s: &str) -> std::result::Result<Chunk, &'static str> {
        let parts: Vec<&str> = s.split('-').collect();
        if parts.len() != 2 {
            return Err("Expected two parts: %i-%i");
        }
        Ok(Chunk {
            first: match FromStr::from_str(parts[0]) {
                Ok(d) => d,
                _ => return Err("Couldn't parse 'first' as digit"),
            },
            last: match FromStr::from_str(parts[1]) {
                Ok(d) => d,
                _ => return Err("Couldn't parse 'last' as digit"),
            },
        })
    }
}

/// Implements the Content-Range header, for serialization only
#[derive(Clone, PartialEq, Debug)]
pub struct ContentRange {
    pub range: Option<Chunk>,
    pub total_length: u64,
}

impl ContentRange {
    pub fn header_value(&self) -> String {
        format!(
            "bytes {}/{}",
            match self.range {
                Some(ref c) => format!("{}", c),
                None => "*".to_string(),
            },
            self.total_length
        )
    }
}

#[derive(Clone, PartialEq, Debug)]
pub struct RangeResponseHeader(pub Chunk);

impl RangeResponseHeader {
    fn from_bytes(raw: &[u8]) -> Self {
        if !raw.is_empty() {
            if let Ok(s) = std::str::from_utf8(raw) {
                const PREFIX: &str = "bytes ";
                if let Some(stripped) = s.strip_prefix(PREFIX) {
                    if let Ok(c) = <Chunk as FromStr>::from_str(&stripped) {
                        return RangeResponseHeader(c);
                    }
                }
            }
        }

        panic!("Unable to parse Range header {:?}", raw)
    }
}

/// A utility type to perform a resumable upload from start to end.
pub struct ResumableUploadHelper<'a, A: 'a, S>
where
    S: tower_service::Service<Uri> + Clone + Send + Sync + 'static,
    S::Response: hyper::client::connect::Connection + AsyncRead + AsyncWrite + Send + Unpin + 'static,
    S::Future: Send + Unpin + 'static,
    S::Error: Into<Box<dyn StdError + Send + Sync>>,
{
    pub client: &'a hyper::client::Client<
        S,
        hyper::body::Body,
    >,
    pub delegate: &'a mut dyn Delegate,
    pub start_at: Option<u64>,
    pub auth: &'a A,
    pub user_agent: &'a str,
    pub auth_header: String,
    pub url: &'a str,
    pub reader: &'a mut dyn ReadSeek,
    pub media_type: Mime,
    pub content_length: u64,
}

impl<'a, A, S> ResumableUploadHelper<'a, A, S>
where
    S: tower_service::Service<Uri> + Clone + Send + Sync + 'static,
    S::Response: hyper::client::connect::Connection + AsyncRead + AsyncWrite + Send + Unpin + 'static,
    S::Future: Send + Unpin + 'static,
    S::Error: Into<Box<dyn StdError + Send + Sync>>,
{
    async fn query_transfer_status(
        &mut self,
    ) -> std::result::Result<u64, hyper::Result<hyper::Response<hyper::body::Body>>> {
        loop {
            match self
                .client
                .request(
                    hyper::Request::builder()
                        .method(hyper::Method::POST)
                        .uri(self.url)
                        .header(USER_AGENT, self.user_agent.to_string())
                        .header(
                            "Content-Range",
                            ContentRange {
                                range: None,
                                total_length: self.content_length,
                            }
                            .header_value(),
                        )
                        .header(AUTHORIZATION, self.auth_header.clone())
                        .body(hyper::body::Body::empty())
                        .unwrap(),
                )
                .await
            {
                Ok(r) => {
                    // 308 = resume-incomplete == PermanentRedirect
                    let headers = r.headers().clone();
                    let h: RangeResponseHeader = match headers.get("Range") {
                        Some(hh) if r.status() == StatusCode::PERMANENT_REDIRECT => {
                            RangeResponseHeader::from_bytes(hh.as_bytes())
                        }
                        None | Some(_) => {
                            if let Retry::After(d) = self.delegate.http_failure(&r, None) {
                                sleep(d);
                                continue;
                            }
                            return Err(Ok(r));
                        }
                    };
                    return Ok(h.0.last);
                }
                Err(err) => {
                    if let Retry::After(d) = self.delegate.http_error(&err) {
                        sleep(d);
                        continue;
                    }
                    return Err(Err(err));
                }
            }
        }
    }

    /// returns None if operation was cancelled by delegate, or the HttpResult.
    /// It can be that we return the result just because we didn't understand the status code -
    /// caller should check for status himself before assuming it's OK to use
    pub async fn upload(&mut self) -> Option<hyper::Result<hyper::Response<hyper::body::Body>>> {
        let mut start = match self.start_at {
            Some(s) => s,
            None => match self.query_transfer_status().await {
                Ok(s) => s,
                Err(result) => return Some(result),
            },
        };

        const MIN_CHUNK_SIZE: u64 = 1 << 18;
        let chunk_size = match self.delegate.chunk_size() {
            cs if cs > MIN_CHUNK_SIZE => cs,
            _ => MIN_CHUNK_SIZE,
        };

        loop {
            self.reader.seek(SeekFrom::Start(start)).unwrap();

            let request_size = match self.content_length - start {
                rs if rs > chunk_size => chunk_size,
                rs => rs,
            };

            let mut section_reader = self.reader.take(request_size);
            let mut req_bytes = vec![];
            section_reader.read_to_end(&mut req_bytes).unwrap();
            let range_header = ContentRange {
                range: Some(Chunk {
                    first: start,
                    last: start + request_size - 1,
                }),
                total_length: self.content_length,
            };
            if self.delegate.cancel_chunk_upload(&range_header) {
                return None;
            }
            let res = self
                .client
                .request(
                    hyper::Request::builder()
                        .uri(self.url)
                        .method(hyper::Method::POST)
                        .header("Content-Range", range_header.header_value())
                        .header(CONTENT_TYPE, format!("{}", self.media_type))
                        .header(USER_AGENT, self.user_agent.to_string())
                        .body(hyper::body::Body::from(req_bytes))
                        .unwrap(),
                )
                .await;
            match res {
                Ok(res) => {
                    start += request_size;

                    if res.status() == StatusCode::PERMANENT_REDIRECT {
                        continue;
                    }

                    let (res_parts, res_body) = res.into_parts();
                    let res_body_string: String = String::from_utf8(
                        hyper::body::to_bytes(res_body)
                            .await
                            .unwrap()
                            .into_iter()
                            .collect(),
                    )
                    .unwrap();
                    let reconstructed_result =
                        hyper::Response::from_parts(res_parts, res_body_string.clone().into());

                    if !reconstructed_result.status().is_success() {
                        if let Retry::After(d) = self.delegate.http_failure(
                            &reconstructed_result,
                            json::from_str(&res_body_string).ok(),
                        ) {
                            sleep(d);
                            continue;
                        }
                    }
                    return Some(Ok(reconstructed_result));
                }
                Err(err) => {
                    if let Retry::After(d) = self.delegate.http_error(&err) {
                        sleep(d);
                        continue;
                    }
                    return Some(Err(err));
                }
            }
        }
    }
}

// Copy of src/rust/cli/client.rs
// TODO(ST): Allow sharing common code between program types
pub fn remove_json_null_values(value: &mut json::value::Value) {
    match *value {
        json::value::Value::Object(ref mut map) => {
            let mut for_removal = Vec::new();

            for (key, mut value) in map.iter_mut() {
                if value.is_null() {
                    for_removal.push(key.clone());
                } else {
                    remove_json_null_values(&mut value);
                }
            }

            for key in &for_removal {
                map.remove(key);
            }
        }
        json::value::Value::Array(ref mut arr) => {
            let mut i = 0;
            while i < arr.len() {
                if arr[i].is_null() {
                    arr.remove(i);
                } else {
                    remove_json_null_values(&mut arr[i]);
                    i += 1;
                }
            }
        }
        _ => {}
    }
}

// Borrowing the body object as mutable and converts it to a string
pub async fn get_body_as_string(res_body: &mut hyper::Body) -> String {
    let res_body_buf = hyper::body::to_bytes(res_body).await.unwrap();
    let res_body_string = String::from_utf8_lossy(&res_body_buf);
    res_body_string.to_string()
}
