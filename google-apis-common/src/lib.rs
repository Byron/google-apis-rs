#![deny(missing_docs)]
//! Common types and traits for Google API crates.

pub mod auth;
/// Utilities for building field masks to specify which fields to return in API responses.
pub mod field_mask;
/// Custom serialization/deserialization helpers for Google API types.
pub mod serde;
/// URL encoding and query parameter utilities for building API requests.
pub mod url;

pub use auth::{GetToken, NoToken};
pub use field_mask::FieldMask;

use std::io::{Cursor, Read, Seek, SeekFrom, Write};
use std::str::FromStr;
use std::time::Duration;

use hyper::header::{HeaderMap, AUTHORIZATION, CONTENT_LENGTH, CONTENT_TYPE, USER_AGENT};
use hyper::Method;
use hyper::StatusCode;
use mime::Mime;
use tokio::time::sleep;

const LINE_ENDING: &str = "\r\n";

/// Type alias for HTTP request/response bodies used throughout the generated Google API clients.
///
/// This is a boxed, type-erased body that can handle streaming data efficiently.
pub type Body = http_body_util::combinators::BoxBody<hyper::body::Bytes, hyper::Error>;

/// Type alias for HTTP responses returned by Google API methods.
pub type Response = hyper::Response<Body>;

/// Type alias for the HTTP client used to make requests to Google APIs.
///
/// The `C` parameter is the connector type that handles the actual network I/O.
pub type Client<C> = hyper_util::client::legacy::Client<C, Body>;

/// Trait bound for connection types that can be used with the Google API clients.
///
/// This trait is automatically implemented for any type that satisfies the required bounds.
pub trait Connector:
    hyper_util::client::legacy::connect::Connect + Clone + Send + Sync + 'static
{
}

impl<T> Connector for T where
    T: hyper_util::client::legacy::connect::Connect + Clone + Send + Sync + 'static
{
}

/// Retry decision returned by delegate callbacks after a failure.
pub enum Retry {
    /// Stop retrying and return the error to the caller.
    Abort,
    /// Retry after waiting for the given duration.
    After(Duration),
}

/// Upload strategy for media-enabled methods.
#[derive(PartialEq, Eq)]
pub enum UploadProtocol {
    /// Upload the full payload in a single request.
    Simple,
    /// Upload in chunks and allow resume after interruption.
    Resumable,
}

/// Marker trait for the top-level hub type of a generated API crate.
pub trait Hub {}

/// Marker trait for builders that expose methods of a single resource.
pub trait MethodsBuilder {}

/// Marker trait for fluent builders that configure one API call.
pub trait CallBuilder {}

/// Marker trait for API resource models used by clients.
pub trait Resource {}

/// Marker trait for types deserialized from API responses.
pub trait ResponseResult {}

/// Marker trait for types serialized into API requests.
pub trait RequestValue {}

/// Marker trait for generated schema types that are currently unused.
pub trait UnusedType {}

/// Marker trait for types only used as fields of other schema types.
pub trait Part {}

/// Marker trait for internal helper schema types.
pub trait NestedType {}

/// Trait alias for upload readers that must be readable, seekable, and sendable.
pub trait ReadSeek: Seek + Read + Send {}
impl<T: Seek + Read + Send> ReadSeek for T {}

/// Converts values into their API `"part"` query-string representation.
pub trait ToParts {
    /// Returns the serialized value used in `"part"` parameters.
    fn to_parts(&self) -> String;
}

/// Callback hooks for observing and controlling request execution.
///
/// The default implementation is conservative and never retries.
pub trait Delegate: Send {
    /// Called at the beginning of any API request. The delegate should store the method
    /// information if he is interesting in knowing more context when further calls to it
    /// are made.
    /// The matching `finished()` call will always be made, no matter whether or not the API
    /// request was successful. That way, the delegate may easily maintain a clean state
    /// between various API calls.
    fn begin(&mut self, _info: MethodInfo) {}

    /// Called whenever there is an [HttpError](hyper_util::client::legacy::Error), usually if
    /// there are network problems.
    ///
    /// If you choose to retry after a duration, the duration should be chosen using the
    /// [exponential backoff algorithm](http://en.wikipedia.org/wiki/Exponential_backoff).
    ///
    /// Return retry information.
    fn http_error(&mut self, _err: &hyper_util::client::legacy::Error) -> Retry {
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
    fn token(
        &mut self,
        e: Box<dyn std::error::Error + Send + Sync>,
    ) -> std::result::Result<Option<String>, Box<dyn std::error::Error + Send + Sync>> {
        Err(e)
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
        json_decode_error: &serde_json::Error,
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
    fn http_failure(&mut self, _: &Response, _err: Option<&serde_json::Value>) -> Retry {
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
    ///   discard all values stored during `store_upload_url`.
    fn finished(&mut self, is_success: bool) {
        let _ = is_success;
    }
}

/// Default [`Delegate`] implementation used when no custom delegate is provided.
#[derive(Default)]
pub struct DefaultDelegate;

impl Delegate for DefaultDelegate {}

#[derive(Debug)]
/// Error type used by generated Google API clients.
pub enum Error {
    /// Transport-level HTTP error while sending a request.
    HttpError(hyper_util::client::legacy::Error),

    /// Upload size (`.0`) exceeds the API's maximum allowed size (`.1`).
    UploadSizeLimitExceeded(u64, u64),

    /// Server returned a structured bad-request payload.
    BadRequest(serde_json::Value),

    /// No API key was available from authenticator or delegate.
    MissingAPIKey,

    /// No OAuth token could be obtained from the authenticator.
    MissingToken(Box<dyn std::error::Error + Send + Sync>),

    /// Operation was cancelled by the delegate.
    Cancelled,

    /// Custom query parameter name conflicts with a built-in parameter.
    FieldClash(&'static str),

    /// Response body could not be decoded as expected JSON.
    JsonDecodeError(String, serde_json::Error),

    /// HTTP response had a non-success status code.
    Failure(Response),

    /// I/O error while reading request or response data.
    Io(std::io::Error),
}

impl Error {
    /// Returns true if the error is transient and can be retried.
    pub fn is_transient(&self) -> bool {
        match self {
            // HttpError usually represents transport or connection-level issues, which are generally retryable.
            Error::HttpError(_) => true,
            Error::Io(err) => matches!(
                err.kind(),
                std::io::ErrorKind::TimedOut
                    | std::io::ErrorKind::Interrupted
                    | std::io::ErrorKind::ConnectionReset
                    | std::io::ErrorKind::ConnectionAborted
            ),
            Error::Failure(res) => {
                let status = res.status();
                status.is_server_error() || status == StatusCode::TOO_MANY_REQUESTS
            }
            _ => false,
        }
    }
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Error::Io(err) => err.fmt(f),
            Error::HttpError(err) => err.fmt(f),
            Error::UploadSizeLimitExceeded(resource_size, max_size) => writeln!(
                f,
                "The media size {resource_size} exceeds the maximum allowed upload size of {max_size}"
            ),
            Error::MissingAPIKey => {
                writeln!(
                    f,
                    "The application's API key was not found in the configuration"
                )?;
                writeln!(
                    f,
                    "It is used as there are no Scopes defined for this method."
                )
            }
            Error::BadRequest(message) => writeln!(f, "Bad Request: {message}"),
            Error::MissingToken(e) => writeln!(f, "Token retrieval failed: {e}"),
            Error::Cancelled => writeln!(f, "Operation cancelled by delegate"),
            Error::FieldClash(field) => writeln!(
                f,
                "The custom parameter '{field}' is already provided natively by the CallBuilder."
            ),
            Error::JsonDecodeError(json_str, err) => writeln!(f, "{err}: {json_str}"),
            Error::Failure(response) => {
                writeln!(f, "Http status indicates failure: {response:?}")
            }
        }
    }
}

impl std::error::Error for Error {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
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

/// Convenience result alias for operations that return [`Error`].
pub type Result<T> = std::result::Result<T, Error>;

/// Contains information about an API request.
pub struct MethodInfo {
    /// Unique identifier for the API method (e.g., "storage.buckets.list")
    pub id: &'static str,
    /// HTTP verb to use for the request (GET, POST, PUT, DELETE, etc.)
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
    // TODO: This should be an associated constant
    /// Returns the `multipart/related` MIME type used for upload requests.
    pub fn mime_type() -> Mime {
        Mime::from_str(&format!("multipart/related;boundary={BOUNDARY}")).expect("valid mimetype")
    }

    /// Pre-allocates storage for exactly `cap` queued parts.
    pub fn reserve_exact(&mut self, cap: usize) {
        self.raw_parts.reserve_exact(cap);
    }

    /// Adds one part that will be emitted in RFC 2387 multipart format.
    ///
    /// # Arguments
    ///
    /// `reader` - Reader for the part body.
    /// `size` - Byte length of the part body, written as `Content-Length`.
    /// `mime_type` - MIME type for this part, written as `Content-Type`.
    pub fn add_part(
        &mut self,
        reader: &'a mut (dyn Read + Send),
        size: u64,
        mime_type: Mime,
    ) -> &mut MultiPartReader<'a> {
        let mut headers = HeaderMap::new();
        headers.insert(
            CONTENT_TYPE,
            hyper::header::HeaderValue::from_str(mime_type.as_ref()).unwrap(),
        );
        headers.insert(CONTENT_LENGTH, size.into());
        self.raw_parts.push((headers, reader));
        self
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

impl Read for MultiPartReader<'_> {
    fn read(&mut self, buf: &mut [u8]) -> std::io::Result<usize> {
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
                use std::fmt::Write as _;
                let (headers, reader) = self.raw_parts.remove(0);

                let mut encoded_headers = String::new();
                for (k, v) in &headers {
                    if !encoded_headers.is_empty() {
                        encoded_headers.push_str(LINE_ENDING);
                    }

                    write!(encoded_headers, "{}: {}", k, v.to_str().unwrap())
                        .map_err(std::io::Error::other)?;
                }

                let mut c = Cursor::new(Vec::<u8>::new());
                //TODO: The first line ending should be omitted for the first part,
                // fortunately Google's API serves don't seem to mind.
                (write!(
                    &mut c,
                    "{LINE_ENDING}--{BOUNDARY}{LINE_ENDING}{encoded_headers}{LINE_ENDING}{LINE_ENDING}"
                ))?;
                c.rewind()?;
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
                            format!("{LINE_ENDING}--{BOUNDARY}--{LINE_ENDING}").into_bytes(),
                        ))
                    }
                    // We are depleted - this can trigger the next part to come in
                    self.current_part = None;
                }
                let mut total_bytes_read = hb + bytes_read;
                while total_bytes_read < buf.len() && !self.is_depleted() {
                    let br = self.read(&mut buf[total_bytes_read..])?;
                    total_bytes_read += br
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
#[derive(PartialEq, Eq, Debug, Clone)]
pub struct XUploadContentType(pub Mime);

impl std::ops::Deref for XUploadContentType {
    type Target = Mime;
    fn deref(&self) -> &Mime {
        &self.0
    }
}
impl std::ops::DerefMut for XUploadContentType {
    fn deref_mut(&mut self) -> &mut Mime {
        &mut self.0
    }
}
impl std::fmt::Display for XUploadContentType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        std::fmt::Display::fmt(&**self, f)
    }
}

/// Inclusive byte range used in resumable upload `Content-Range` headers.
#[derive(Clone, PartialEq, Eq, Debug)]
pub struct Chunk {
    /// Zero-based byte offset where this chunk starts (inclusive)
    pub first: u64,
    /// Zero-based byte offset where this chunk ends (inclusive)
    pub last: u64,
}

impl std::fmt::Display for Chunk {
    fn fmt(&self, fmt: &mut std::fmt::Formatter) -> std::fmt::Result {
        (write!(fmt, "{}-{}", self.first, self.last)).ok();
        Ok(())
    }
}

impl FromStr for Chunk {
    type Err = &'static str;

    /// Parses only `<start>-<end>`; wildcard forms are not supported.
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

/// Serializer for the `Content-Range` request header.
#[derive(Clone, PartialEq, Eq, Debug)]
pub struct ContentRange {
    /// The byte range being sent, or None to query upload status
    pub range: Option<Chunk>,
    /// Total size of the complete resource being uploaded
    pub total_length: u64,
}

impl ContentRange {
    /// Formats this value as a `Content-Range` header string.
    pub fn header_value(&self) -> String {
        format!(
            "bytes {}/{}",
            match self.range {
                Some(ref c) => format!("{c}"),
                None => "*".to_string(),
            },
            self.total_length
        )
    }
}

/// Parsed value of the resumable-upload `Range` response header.
#[derive(Clone, PartialEq, Eq, Debug)]
pub struct RangeResponseHeader(pub Chunk);

impl RangeResponseHeader {
    fn from_bytes(raw: &[u8]) -> Self {
        if !raw.is_empty() {
            if let Ok(s) = std::str::from_utf8(raw) {
                const PREFIX: &str = "bytes ";
                if let Some(stripped) = s.strip_prefix(PREFIX) {
                    if let Ok(c) = <Chunk as FromStr>::from_str(stripped) {
                        return RangeResponseHeader(c);
                    }
                }
            }
        }

        panic!("Unable to parse Range header {raw:?}")
    }
}

/// A utility type to perform a resumable upload from start to end.
pub struct ResumableUploadHelper<'a, A: 'a, C>
where
    C: Connector,
{
    /// HTTP client for making upload requests
    pub client: &'a Client<C>,
    /// Callback interface for monitoring and controlling the upload
    pub delegate: &'a mut dyn Delegate,
    /// If resuming a previous upload, the byte offset to resume from; None to start fresh or query status
    pub start_at: Option<u64>,
    /// Authentication provider for obtaining access tokens
    pub auth: &'a A,
    /// User-Agent header value for identifying the client application
    pub user_agent: &'a str,
    /// Pre-formatted Authorization header value (typically "Bearer <token>")
    pub auth_header: String,
    /// Upload endpoint URL provided by the server
    pub url: &'a str,
    /// Source of upload data; must support seeking for retries and chunked uploads
    pub reader: &'a mut dyn ReadSeek,
    /// MIME type of the content being uploaded (e.g., "image/jpeg", "application/pdf")
    pub media_type: Mime,
    /// Total size in bytes of the complete upload
    pub content_length: u64,
}

impl<A, C> ResumableUploadHelper<'_, A, C>
where
    C: Connector,
{
    async fn query_transfer_status(
        &mut self,
    ) -> std::result::Result<u64, std::result::Result<Response, hyper_util::client::legacy::Error>>
    {
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
                        .body(to_body::<String>(None))
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
                            let (parts, body) = r.into_parts();
                            let body = to_body(to_bytes(body).await);
                            let response = Response::from_parts(parts, body);
                            if let Retry::After(d) = self.delegate.http_failure(&response, None) {
                                sleep(d).await;
                                continue;
                            }
                            return Err(Ok(response));
                        }
                    };
                    return Ok(h.0.last);
                }
                Err(err) => {
                    if let Retry::After(d) = self.delegate.http_error(&err) {
                        sleep(d).await;
                        continue;
                    }
                    return Err(Err(err));
                }
            }
        }
    }

    /// Uploads all remaining chunks.
    ///
    /// Returns `None` if the delegate cancelled upload. Otherwise returns the final
    /// HTTP result (`Ok(response)` or transport `Err`).
    pub async fn upload(
        &mut self,
    ) -> Option<std::result::Result<Response, hyper_util::client::legacy::Error>> {
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
            let mut bytes = vec![];
            section_reader.read_to_end(&mut bytes).unwrap();
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
            match self
                .client
                .request(
                    hyper::Request::builder()
                        .uri(self.url)
                        .method(hyper::Method::POST)
                        .header("Content-Range", range_header.header_value())
                        .header(CONTENT_TYPE, format!("{}", self.media_type))
                        .header(USER_AGENT, self.user_agent.to_string())
                        .body(to_body(bytes.into()))
                        .unwrap(),
                )
                .await
            {
                Ok(response) => {
                    start += request_size;

                    if response.status() == StatusCode::PERMANENT_REDIRECT {
                        continue;
                    }

                    let (parts, body) = response.into_parts();
                    let success = parts.status.is_success();
                    let bytes = to_bytes(body).await.unwrap_or_default();
                    let error = if !success {
                        serde_json::from_str(&to_string(&bytes)).ok()
                    } else {
                        None
                    };
                    let response = to_response(parts, bytes.into());

                    if !success {
                        if let Retry::After(d) =
                            self.delegate.http_failure(&response, error.as_ref())
                        {
                            sleep(d).await;
                            continue;
                        }
                    }
                    return Some(Ok(response));
                }
                Err(err) => {
                    if let Retry::After(d) = self.delegate.http_error(&err) {
                        sleep(d).await;
                        continue;
                    }
                    return Some(Err(err));
                }
            }
        }
    }
}

// TODO(ST): Allow sharing common code between program types
/// Recursively removes all `null` entries from JSON objects and arrays in place.
pub fn remove_json_null_values(value: &mut serde_json::value::Value) {
    match value {
        serde_json::value::Value::Object(map) => {
            map.retain(|_, value| !value.is_null());
            map.values_mut().for_each(remove_json_null_values);
        }
        serde_json::value::Value::Array(arr) => {
            arr.retain(|value| !value.is_null());
            arr.iter_mut().for_each(remove_json_null_values);
        }
        _ => {}
    }
}

#[doc(hidden)]
pub fn to_body<T>(bytes: Option<T>) -> Body
where
    T: Into<hyper::body::Bytes>,
{
    use http_body_util::BodyExt;

    fn falliable(_: std::convert::Infallible) -> hyper::Error {
        unreachable!()
    }

    let bytes = bytes.map(Into::into).unwrap_or_default();
    Body::new(http_body_util::Full::from(bytes).map_err(falliable))
}

#[doc(hidden)]
pub async fn to_bytes<T>(body: T) -> Option<hyper::body::Bytes>
where
    T: hyper::body::Body,
{
    use http_body_util::BodyExt;
    body.collect().await.ok().map(|value| value.to_bytes())
}

#[doc(hidden)]
pub fn to_string(bytes: &hyper::body::Bytes) -> std::borrow::Cow<'_, str> {
    String::from_utf8_lossy(bytes)
}

#[doc(hidden)]
pub fn to_response<T>(parts: http::response::Parts, body: Option<T>) -> Response
where
    T: Into<hyper::body::Bytes>,
{
    Response::from_parts(parts, to_body(body))
}

#[cfg(test)]
mod tests {
    use std::default::Default;
    use std::str::FromStr;

    use ::serde::{Deserialize, Serialize};

    use super::*;

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
        serde_json::to_string(&f).unwrap(); // should work

        let j = "{\"opt\":null,\"req\":0,\"vec\":[]}";
        let _f: Foo = serde_json::from_str(j).unwrap();

        // This fails, unless 'vec' is optional
        // let j = "{\"opt\":null,\"req\":0}";
        // let f: Foo = serde_json::from_str(j).unwrap();

        #[derive(Default, Serialize, Deserialize)]
        struct Bar {
            #[serde(rename = "snooSnoo")]
            snoo_snoo: String,
        }
        serde_json::to_string(&<Bar as Default>::default()).unwrap();

        let j = "{\"snooSnoo\":\"foo\"}";
        let b: Bar = serde_json::from_str(j).unwrap();
        assert_eq!(b.snoo_snoo, "foo");

        // We can't have unknown fields with structs.
        // #[derive(Default, Serialize, Deserialize)]
        // struct BarOpt {
        //     #[serde(rename="snooSnoo")]
        //     snoo_snoo: Option<String>
        // }
        // let j = "{\"snooSnoo\":\"foo\",\"foo\":\"bar\"}";
        // let b: BarOpt = serde_json::from_str(&j).unwrap();
    }

    #[test]
    fn byte_range_from_str() {
        assert_eq!(
            <Chunk as FromStr>::from_str("2-42"),
            Ok(Chunk { first: 2, last: 42 })
        )
    }

    #[test]
    fn dyn_delegate_is_send() {
        fn with_send(_x: impl Send) {}

        let mut dd = DefaultDelegate;
        let dlg: &mut dyn Delegate = &mut dd;
        with_send(dlg);
    }

    #[test]
    fn test_mime() {
        let mime = MultiPartReader::mime_type();

        assert_eq!(mime::MULTIPART, mime.type_());
        assert_eq!("related", mime.subtype());
        assert_eq!(
            Some(BOUNDARY),
            mime.get_param("boundary").map(|x| x.as_str())
        );
    }

    #[test]
    fn test_transient_errors() {
        use http_body_util::{BodyExt, Empty};
        use hyper::body::Bytes;

        fn empty_body() -> crate::Body {
            Empty::<Bytes>::new().map_err(|_| unreachable!()).boxed()
        }

        let res = hyper::Response::builder()
            .status(503)
            .body(empty_body())
            .unwrap();
        let err = Error::Failure(res);
        assert!(err.is_transient(), "503 Service Unavailable -> Transient");

        let res = hyper::Response::builder()
            .status(429)
            .body(empty_body())
            .unwrap();
        let err = Error::Failure(res);
        assert!(err.is_transient(), "429 Too Many Requests -> Transient");

        let res = hyper::Response::builder()
            .status(404)
            .body(empty_body())
            .unwrap();
        let err = Error::Failure(res);
        assert!(!err.is_transient(), "404 Not Found -> Permanent");
    }
}
