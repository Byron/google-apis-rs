// DO NOT EDIT !
// This file was generated automatically from 'src/mako/api/lib.rs.mako'
// DO NOT EDIT !

//! This documentation was generated from *classroom* crate version *1.0.5+20170510*, where *20170510* is the exact revision of the *classroom:v1* schema built by the [mako](http://www.makotemplates.org/) code generator *v1.0.5*.
//! 
//! Everything else about the *classroom* *v1* API can be found at the
//! [official documentation site](https://developers.google.com/classroom/).
//! The original source code is [on github](https://github.com/Byron/google-apis-rs/tree/master/gen/classroom1).
//! # Features
//! 
//! Handle the following *Resources* with ease from the central [hub](struct.Classroom.html) ... 
//! 
//! * [courses](struct.Course.html)
//!  * [*aliases create*](struct.CourseAliaseCreateCall.html), [*aliases delete*](struct.CourseAliaseDeleteCall.html), [*aliases list*](struct.CourseAliaseListCall.html), [*course work create*](struct.CourseCourseWorkCreateCall.html), [*course work delete*](struct.CourseCourseWorkDeleteCall.html), [*course work get*](struct.CourseCourseWorkGetCall.html), [*course work list*](struct.CourseCourseWorkListCall.html), [*course work patch*](struct.CourseCourseWorkPatchCall.html), [*course work student submissions get*](struct.CourseCourseWorkStudentSubmissionGetCall.html), [*course work student submissions list*](struct.CourseCourseWorkStudentSubmissionListCall.html), [*course work student submissions modify attachments*](struct.CourseCourseWorkStudentSubmissionModifyAttachmentCall.html), [*course work student submissions patch*](struct.CourseCourseWorkStudentSubmissionPatchCall.html), [*course work student submissions reclaim*](struct.CourseCourseWorkStudentSubmissionReclaimCall.html), [*course work student submissions return*](struct.CourseCourseWorkStudentSubmissionReturnCall.html), [*course work student submissions turn in*](struct.CourseCourseWorkStudentSubmissionTurnInCall.html), [*create*](struct.CourseCreateCall.html), [*delete*](struct.CourseDeleteCall.html), [*get*](struct.CourseGetCall.html), [*list*](struct.CourseListCall.html), [*patch*](struct.CoursePatchCall.html), [*students create*](struct.CourseStudentCreateCall.html), [*students delete*](struct.CourseStudentDeleteCall.html), [*students get*](struct.CourseStudentGetCall.html), [*students list*](struct.CourseStudentListCall.html), [*teachers create*](struct.CourseTeacherCreateCall.html), [*teachers delete*](struct.CourseTeacherDeleteCall.html), [*teachers get*](struct.CourseTeacherGetCall.html), [*teachers list*](struct.CourseTeacherListCall.html) and [*update*](struct.CourseUpdateCall.html)
//! * [invitations](struct.Invitation.html)
//!  * [*accept*](struct.InvitationAcceptCall.html), [*create*](struct.InvitationCreateCall.html), [*delete*](struct.InvitationDeleteCall.html), [*get*](struct.InvitationGetCall.html) and [*list*](struct.InvitationListCall.html)
//! * [user profiles](struct.UserProfile.html)
//!  * [*get*](struct.UserProfileGetCall.html), [*guardian invitations create*](struct.UserProfileGuardianInvitationCreateCall.html), [*guardian invitations get*](struct.UserProfileGuardianInvitationGetCall.html), [*guardian invitations list*](struct.UserProfileGuardianInvitationListCall.html), [*guardian invitations patch*](struct.UserProfileGuardianInvitationPatchCall.html), [*guardians delete*](struct.UserProfileGuardianDeleteCall.html), [*guardians get*](struct.UserProfileGuardianGetCall.html) and [*guardians list*](struct.UserProfileGuardianListCall.html)
//! 
//! 
//! 
//! 
//! Not what you are looking for ? Find all other Google APIs in their Rust [documentation index](http://byron.github.io/google-apis-rs).
//! 
//! # Structure of this Library
//! 
//! The API is structured into the following primary items:
//! 
//! * **[Hub](struct.Classroom.html)**
//!     * a central object to maintain state and allow accessing all *Activities*
//!     * creates [*Method Builders*](trait.MethodsBuilder.html) which in turn
//!       allow access to individual [*Call Builders*](trait.CallBuilder.html)
//! * **[Resources](trait.Resource.html)**
//!     * primary types that you can apply *Activities* to
//!     * a collection of properties and *Parts*
//!     * **[Parts](trait.Part.html)**
//!         * a collection of properties
//!         * never directly used in *Activities*
//! * **[Activities](trait.CallBuilder.html)**
//!     * operations to apply to *Resources*
//! 
//! All *structures* are marked with applicable traits to further categorize them and ease browsing.
//! 
//! Generally speaking, you can invoke *Activities* like this:
//! 
//! ```Rust,ignore
//! let r = hub.resource().activity(...).doit()
//! ```
//! 
//! Or specifically ...
//! 
//! ```ignore
//! let r = hub.courses().course_work_student_submissions_patch(...).doit()
//! let r = hub.courses().course_work_student_submissions_list(...).doit()
//! let r = hub.courses().get(...).doit()
//! let r = hub.courses().update(...).doit()
//! let r = hub.courses().students_get(...).doit()
//! let r = hub.courses().teachers_get(...).doit()
//! let r = hub.courses().course_work_list(...).doit()
//! let r = hub.courses().course_work_get(...).doit()
//! let r = hub.courses().course_work_student_submissions_turn_in(...).doit()
//! let r = hub.courses().course_work_student_submissions_modify_attachments(...).doit()
//! let r = hub.courses().teachers_list(...).doit()
//! let r = hub.courses().course_work_student_submissions_return(...).doit()
//! let r = hub.courses().aliases_list(...).doit()
//! let r = hub.courses().course_work_create(...).doit()
//! let r = hub.courses().list(...).doit()
//! let r = hub.courses().course_work_student_submissions_reclaim(...).doit()
//! let r = hub.courses().aliases_create(...).doit()
//! let r = hub.courses().students_create(...).doit()
//! let r = hub.courses().aliases_delete(...).doit()
//! let r = hub.courses().course_work_delete(...).doit()
//! let r = hub.courses().create(...).doit()
//! let r = hub.courses().students_list(...).doit()
//! let r = hub.courses().delete(...).doit()
//! let r = hub.courses().course_work_patch(...).doit()
//! let r = hub.courses().patch(...).doit()
//! let r = hub.courses().students_delete(...).doit()
//! let r = hub.courses().teachers_delete(...).doit()
//! let r = hub.courses().teachers_create(...).doit()
//! let r = hub.courses().course_work_student_submissions_get(...).doit()
//! ```
//! 
//! The `resource()` and `activity(...)` calls create [builders][builder-pattern]. The second one dealing with `Activities` 
//! supports various methods to configure the impending operation (not shown here). It is made such that all required arguments have to be 
//! specified right away (i.e. `(...)`), whereas all optional ones can be [build up][builder-pattern] as desired.
//! The `doit()` method performs the actual communication with the server and returns the respective result.
//! 
//! # Usage
//! 
//! ## Setting up your Project
//! 
//! To use this library, you would put the following lines into your `Cargo.toml` file:
//! 
//! ```toml
//! [dependencies]
//! google-classroom1 = "*"
//! ```
//! 
//! ## A complete example
//! 
//! ```test_harness,no_run
//! extern crate hyper;
//! extern crate hyper_rustls;
//! extern crate yup_oauth2 as oauth2;
//! extern crate google_classroom1 as classroom1;
//! use classroom1::{Result, Error};
//! # #[test] fn egal() {
//! use std::default::Default;
//! use oauth2::{Authenticator, DefaultAuthenticatorDelegate, ApplicationSecret, MemoryStorage};
//! use classroom1::Classroom;
//! 
//! // Get an ApplicationSecret instance by some means. It contains the `client_id` and 
//! // `client_secret`, among other things.
//! let secret: ApplicationSecret = Default::default();
//! // Instantiate the authenticator. It will choose a suitable authentication flow for you, 
//! // unless you replace  `None` with the desired Flow.
//! // Provide your own `AuthenticatorDelegate` to adjust the way it operates and get feedback about 
//! // what's going on. You probably want to bring in your own `TokenStorage` to persist tokens and
//! // retrieve them from storage.
//! let auth = Authenticator::new(&secret, DefaultAuthenticatorDelegate,
//!                               hyper::Client::with_connector(hyper::net::HttpsConnector::new(hyper_rustls::TlsClient::new())),
//!                               <MemoryStorage as Default>::default(), None);
//! let mut hub = Classroom::new(hyper::Client::with_connector(hyper::net::HttpsConnector::new(hyper_rustls::TlsClient::new())), auth);
//! // You can configure optional parameters by calling the respective setters at will, and
//! // execute the final call using `doit()`.
//! // Values shown here are possibly random and not representative !
//! let result = hub.courses().course_work_student_submissions_list("courseId", "courseWorkId")
//!              .user_id("justo")
//!              .add_states("amet.")
//!              .page_token("erat")
//!              .page_size(-35)
//!              .late("sea")
//!              .doit();
//! 
//! match result {
//!     Err(e) => match e {
//!         // The Error enum provides details about what exactly happened.
//!         // You can also just use its `Debug`, `Display` or `Error` traits
//!          Error::HttpError(_)
//!         |Error::MissingAPIKey
//!         |Error::MissingToken(_)
//!         |Error::Cancelled
//!         |Error::UploadSizeLimitExceeded(_, _)
//!         |Error::Failure(_)
//!         |Error::BadRequest(_)
//!         |Error::FieldClash(_)
//!         |Error::JsonDecodeError(_, _) => println!("{}", e),
//!     },
//!     Ok(res) => println!("Success: {:?}", res),
//! }
//! # }
//! ```
//! ## Handling Errors
//! 
//! All errors produced by the system are provided either as [Result](enum.Result.html) enumeration as return value of 
//! the doit() methods, or handed as possibly intermediate results to either the 
//! [Hub Delegate](trait.Delegate.html), or the [Authenticator Delegate](https://docs.rs/yup-oauth2/*/yup_oauth2/trait.AuthenticatorDelegate.html).
//! 
//! When delegates handle errors or intermediate values, they may have a chance to instruct the system to retry. This 
//! makes the system potentially resilient to all kinds of errors.
//! 
//! ## Uploads and Downloads
//! If a method supports downloads, the response body, which is part of the [Result](enum.Result.html), should be
//! read by you to obtain the media.
//! If such a method also supports a [Response Result](trait.ResponseResult.html), it will return that by default.
//! You can see it as meta-data for the actual media. To trigger a media download, you will have to set up the builder by making
//! this call: `.param("alt", "media")`.
//! 
//! Methods supporting uploads can do so using up to 2 different protocols: 
//! *simple* and *resumable*. The distinctiveness of each is represented by customized 
//! `doit(...)` methods, which are then named `upload(...)` and `upload_resumable(...)` respectively.
//! 
//! ## Customization and Callbacks
//! 
//! You may alter the way an `doit()` method is called by providing a [delegate](trait.Delegate.html) to the 
//! [Method Builder](trait.CallBuilder.html) before making the final `doit()` call. 
//! Respective methods will be called to provide progress information, as well as determine whether the system should 
//! retry on failure.
//! 
//! The [delegate trait](trait.Delegate.html) is default-implemented, allowing you to customize it with minimal effort.
//! 
//! ## Optional Parts in Server-Requests
//! 
//! All structures provided by this library are made to be [enocodable](trait.RequestValue.html) and 
//! [decodable](trait.ResponseResult.html) via *json*. Optionals are used to indicate that partial requests are responses 
//! are valid.
//! Most optionals are are considered [Parts](trait.Part.html) which are identifiable by name, which will be sent to 
//! the server to indicate either the set parts of the request or the desired parts in the response.
//! 
//! ## Builder Arguments
//! 
//! Using [method builders](trait.CallBuilder.html), you are able to prepare an action call by repeatedly calling it's methods.
//! These will always take a single argument, for which the following statements are true.
//! 
//! * [PODs][wiki-pod] are handed by copy
//! * strings are passed as `&str`
//! * [request values](trait.RequestValue.html) are moved
//! 
//! Arguments will always be copied or cloned into the builder, to make them independent of their original life times.
//! 
//! [wiki-pod]: http://en.wikipedia.org/wiki/Plain_old_data_structure
//! [builder-pattern]: http://en.wikipedia.org/wiki/Builder_pattern
//! [google-go-api]: https://github.com/google/google-api-go-client
//! 
//! 

// Unused attributes happen thanks to defined, but unused structures
// We don't warn about this, as depending on the API, some data structures or facilities are never used.
// Instead of pre-determining this, we just disable the lint. It's manually tuned to not have any
// unused imports in fully featured APIs. Same with unused_mut ... .
#![allow(unused_imports, unused_mut, dead_code)]

// DO NOT EDIT !
// This file was generated automatically from 'src/mako/api/lib.rs.mako'
// DO NOT EDIT !

#[macro_use]
extern crate serde_derive;

extern crate hyper;
extern crate serde;
extern crate serde_json;
extern crate yup_oauth2 as oauth2;
extern crate mime;
extern crate url;

mod cmn;

use std::collections::HashMap;
use std::cell::RefCell;
use std::borrow::BorrowMut;
use std::default::Default;
use std::collections::BTreeMap;
use serde_json as json;
use std::io;
use std::fs;
use std::mem;
use std::thread::sleep;
use std::time::Duration;

pub use cmn::{MultiPartReader, ToParts, MethodInfo, Result, Error, CallBuilder, Hub, ReadSeek, Part,
              ResponseResult, RequestValue, NestedType, Delegate, DefaultDelegate, MethodsBuilder,
              Resource, ErrorResponse, remove_json_null_values};


// ##############
// UTILITIES ###
// ############

/// Identifies the an OAuth2 authorization scope.
/// A scope is needed when requesting an
/// [authorization token](https://developers.google.com/youtube/v3/guides/authentication).
#[derive(PartialEq, Eq, Hash)]
pub enum Scope {
    /// View your course work and grades in Google Classroom
    StudentSubmissionMeReadonly,

    /// View your Google Classroom class rosters
    RosterReadonly,

    /// View your course work and grades in Google Classroom
    CourseworkMeReadonly,

    /// View course work and grades for students in the Google Classroom classes you teach or administer
    CourseworkStudentReadonly,

    /// View the email addresses of people in your classes
    ProfileEmail,

    /// View the profile photos of people in your classes
    ProfilePhoto,

    /// Manage course work and grades for students in the Google Classroom classes you teach and view the course work and grades for classes you administer
    CourseworkStudent,

    /// View course work and grades for students in the Google Classroom classes you teach or administer
    StudentSubmissionStudentReadonly,

    /// View your Google Classroom classes
    CourseReadonly,

    /// Manage your Google Classroom class rosters
    Roster,

    /// Manage your course work and view your grades in Google Classroom
    CourseworkMe,

    /// Manage your Google Classroom classes
    Course,
}

impl AsRef<str> for Scope {
    fn as_ref(&self) -> &str {
        match *self {
            Scope::StudentSubmissionMeReadonly => "https://www.googleapis.com/auth/classroom.student-submissions.me.readonly",
            Scope::RosterReadonly => "https://www.googleapis.com/auth/classroom.rosters.readonly",
            Scope::CourseworkMeReadonly => "https://www.googleapis.com/auth/classroom.coursework.me.readonly",
            Scope::CourseworkStudentReadonly => "https://www.googleapis.com/auth/classroom.coursework.students.readonly",
            Scope::ProfileEmail => "https://www.googleapis.com/auth/classroom.profile.emails",
            Scope::ProfilePhoto => "https://www.googleapis.com/auth/classroom.profile.photos",
            Scope::CourseworkStudent => "https://www.googleapis.com/auth/classroom.coursework.students",
            Scope::StudentSubmissionStudentReadonly => "https://www.googleapis.com/auth/classroom.student-submissions.students.readonly",
            Scope::CourseReadonly => "https://www.googleapis.com/auth/classroom.courses.readonly",
            Scope::Roster => "https://www.googleapis.com/auth/classroom.rosters",
            Scope::CourseworkMe => "https://www.googleapis.com/auth/classroom.coursework.me",
            Scope::Course => "https://www.googleapis.com/auth/classroom.courses",
        }
    }
}

impl Default for Scope {
    fn default() -> Scope {
        Scope::StudentSubmissionMeReadonly
    }
}



// ########
// HUB ###
// ######

/// Central instance to access all Classroom related resource activities
///
/// # Examples
///
/// Instantiate a new hub
///
/// ```test_harness,no_run
/// extern crate hyper;
/// extern crate hyper_rustls;
/// extern crate yup_oauth2 as oauth2;
/// extern crate google_classroom1 as classroom1;
/// use classroom1::{Result, Error};
/// # #[test] fn egal() {
/// use std::default::Default;
/// use oauth2::{Authenticator, DefaultAuthenticatorDelegate, ApplicationSecret, MemoryStorage};
/// use classroom1::Classroom;
/// 
/// // Get an ApplicationSecret instance by some means. It contains the `client_id` and 
/// // `client_secret`, among other things.
/// let secret: ApplicationSecret = Default::default();
/// // Instantiate the authenticator. It will choose a suitable authentication flow for you, 
/// // unless you replace  `None` with the desired Flow.
/// // Provide your own `AuthenticatorDelegate` to adjust the way it operates and get feedback about 
/// // what's going on. You probably want to bring in your own `TokenStorage` to persist tokens and
/// // retrieve them from storage.
/// let auth = Authenticator::new(&secret, DefaultAuthenticatorDelegate,
///                               hyper::Client::with_connector(hyper::net::HttpsConnector::new(hyper_rustls::TlsClient::new())),
///                               <MemoryStorage as Default>::default(), None);
/// let mut hub = Classroom::new(hyper::Client::with_connector(hyper::net::HttpsConnector::new(hyper_rustls::TlsClient::new())), auth);
/// // You can configure optional parameters by calling the respective setters at will, and
/// // execute the final call using `doit()`.
/// // Values shown here are possibly random and not representative !
/// let result = hub.courses().course_work_student_submissions_list("courseId", "courseWorkId")
///              .user_id("gubergren")
///              .add_states("sadipscing")
///              .page_token("aliquyam")
///              .page_size(-66)
///              .late("no")
///              .doit();
/// 
/// match result {
///     Err(e) => match e {
///         // The Error enum provides details about what exactly happened.
///         // You can also just use its `Debug`, `Display` or `Error` traits
///          Error::HttpError(_)
///         |Error::MissingAPIKey
///         |Error::MissingToken(_)
///         |Error::Cancelled
///         |Error::UploadSizeLimitExceeded(_, _)
///         |Error::Failure(_)
///         |Error::BadRequest(_)
///         |Error::FieldClash(_)
///         |Error::JsonDecodeError(_, _) => println!("{}", e),
///     },
///     Ok(res) => println!("Success: {:?}", res),
/// }
/// # }
/// ```
pub struct Classroom<C, A> {
    client: RefCell<C>,
    auth: RefCell<A>,
    _user_agent: String,
    _base_url: String,
    _root_url: String,
}

impl<'a, C, A> Hub for Classroom<C, A> {}

impl<'a, C, A> Classroom<C, A>
    where  C: BorrowMut<hyper::Client>, A: oauth2::GetToken {

    pub fn new(client: C, authenticator: A) -> Classroom<C, A> {
        Classroom {
            client: RefCell::new(client),
            auth: RefCell::new(authenticator),
            _user_agent: "google-api-rust-client/1.0.5".to_string(),
            _base_url: "https://classroom.googleapis.com/".to_string(),
            _root_url: "https://classroom.googleapis.com/".to_string(),
        }
    }

    pub fn courses(&'a self) -> CourseMethods<'a, C, A> {
        CourseMethods { hub: &self }
    }
    pub fn invitations(&'a self) -> InvitationMethods<'a, C, A> {
        InvitationMethods { hub: &self }
    }
    pub fn user_profiles(&'a self) -> UserProfileMethods<'a, C, A> {
        UserProfileMethods { hub: &self }
    }

    /// Set the user-agent header field to use in all requests to the server.
    /// It defaults to `google-api-rust-client/1.0.5`.
    ///
    /// Returns the previously set user-agent.
    pub fn user_agent(&mut self, agent_name: String) -> String {
        mem::replace(&mut self._user_agent, agent_name)
    }

    /// Set the base url to use in all requests to the server.
    /// It defaults to `https://classroom.googleapis.com/`.
    ///
    /// Returns the previously set base url.
    pub fn base_url(&mut self, new_base_url: String) -> String {
        mem::replace(&mut self._base_url, new_base_url)
    }

    /// Set the root url to use in all requests to the server.
    /// It defaults to `https://classroom.googleapis.com/`.
    ///
    /// Returns the previously set root url.
    pub fn root_url(&mut self, new_root_url: String) -> String {
        mem::replace(&mut self._root_url, new_root_url)
    }
}


// ############
// SCHEMAS ###
// ##########
/// Response when listing guardians.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [guardians list user profiles](struct.UserProfileGuardianListCall.html) (response)
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct ListGuardiansResponse {
    /// Token identifying the next page of results to return. If empty, no further
    /// results are available.
    #[serde(rename="nextPageToken")]
    pub next_page_token: Option<String>,
    /// Guardians on this page of results that met the criteria specified in
    /// the request.
    pub guardians: Option<Vec<Guardian>>,
}

impl ResponseResult for ListGuardiansResponse {}


/// Student work for a multiple-choice question.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct MultipleChoiceSubmission {
    /// Student's select choice.
    pub answer: Option<String>,
}

impl Part for MultipleChoiceSubmission {}


/// Request to modify the attachments of a student submission.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [course work student submissions modify attachments courses](struct.CourseCourseWorkStudentSubmissionModifyAttachmentCall.html) (request)
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct ModifyAttachmentsRequest {
    /// Attachments to add.
    /// A student submission may not have more than 20 attachments.
    /// 
    /// Form attachments are not supported.
    #[serde(rename="addAttachments")]
    pub add_attachments: Option<Vec<Attachment>>,
}

impl RequestValue for ModifyAttachmentsRequest {}


/// Student work for a short answer question.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct ShortAnswerSubmission {
    /// Student response to a short-answer question.
    pub answer: Option<String>,
}

impl Part for ShortAnswerSubmission {}


/// Attachment added to student assignment work.
/// 
/// When creating attachments, setting the `form` field is not supported.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct Attachment {
    /// Youtube video attachment.
    #[serde(rename="youTubeVideo")]
    pub you_tube_video: Option<YouTubeVideo>,
    /// Google Drive file attachment.
    #[serde(rename="driveFile")]
    pub drive_file: Option<DriveFile>,
    /// Link attachment.
    pub link: Option<Link>,
    /// Google Forms attachment.
    pub form: Option<Form>,
}

impl Part for Attachment {}


/// Represents a time of day. The date and time zone are either not significant
/// or are specified elsewhere. An API may choose to allow leap seconds. Related
/// types are google.type.Date and `google.protobuf.Timestamp`.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct TimeOfDay {
    /// Hours of day in 24 hour format. Should be from 0 to 23. An API may choose
    /// to allow the value "24:00:00" for scenarios like business closing time.
    pub hours: Option<i32>,
    /// Fractions of seconds in nanoseconds. Must be from 0 to 999,999,999.
    pub nanos: Option<i32>,
    /// Seconds of minutes of the time. Must normally be from 0 to 59. An API may
    /// allow the value 60 if it allows leap-seconds.
    pub seconds: Option<i32>,
    /// Minutes of hour of day. Must be from 0 to 59.
    pub minutes: Option<i32>,
}

impl Part for TimeOfDay {}


/// Request to return a student submission.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [course work student submissions return courses](struct.CourseCourseWorkStudentSubmissionReturnCall.html) (request)
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct ReturnStudentSubmissionRequest { _never_set: Option<bool> }

impl RequestValue for ReturnStudentSubmissionRequest {}


/// A generic empty message that you can re-use to avoid defining duplicated
/// empty messages in your APIs. A typical example is to use it as the request
/// or the response type of an API method. For instance:
/// 
///     service Foo {
///       rpc Bar(google.protobuf.Empty) returns (google.protobuf.Empty);
///     }
/// 
/// The JSON representation for `Empty` is empty JSON object `{}`.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [delete invitations](struct.InvitationDeleteCall.html) (response)
/// * [course work delete courses](struct.CourseCourseWorkDeleteCall.html) (response)
/// * [course work student submissions reclaim courses](struct.CourseCourseWorkStudentSubmissionReclaimCall.html) (response)
/// * [teachers delete courses](struct.CourseTeacherDeleteCall.html) (response)
/// * [guardians delete user profiles](struct.UserProfileGuardianDeleteCall.html) (response)
/// * [students delete courses](struct.CourseStudentDeleteCall.html) (response)
/// * [aliases delete courses](struct.CourseAliaseDeleteCall.html) (response)
/// * [course work student submissions turn in courses](struct.CourseCourseWorkStudentSubmissionTurnInCall.html) (response)
/// * [course work student submissions return courses](struct.CourseCourseWorkStudentSubmissionReturnCall.html) (response)
/// * [delete courses](struct.CourseDeleteCall.html) (response)
/// * [accept invitations](struct.InvitationAcceptCall.html) (response)
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct Empty { _never_set: Option<bool> }

impl ResponseResult for Empty {}


/// Global information for a user.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [guardian invitations list user profiles](struct.UserProfileGuardianInvitationListCall.html) (none)
/// * [guardian invitations patch user profiles](struct.UserProfileGuardianInvitationPatchCall.html) (none)
/// * [get user profiles](struct.UserProfileGetCall.html) (response)
/// * [guardian invitations get user profiles](struct.UserProfileGuardianInvitationGetCall.html) (none)
/// * [guardians delete user profiles](struct.UserProfileGuardianDeleteCall.html) (none)
/// * [guardians get user profiles](struct.UserProfileGuardianGetCall.html) (none)
/// * [guardians list user profiles](struct.UserProfileGuardianListCall.html) (none)
/// * [guardian invitations create user profiles](struct.UserProfileGuardianInvitationCreateCall.html) (none)
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct UserProfile {
    /// URL of user's profile photo.
    /// 
    /// Read-only.
    #[serde(rename="photoUrl")]
    pub photo_url: Option<String>,
    /// Email address of the user.
    /// 
    /// Read-only.
    #[serde(rename="emailAddress")]
    pub email_address: Option<String>,
    /// Identifier of the user.
    /// 
    /// Read-only.
    pub id: Option<String>,
    /// Name of the user.
    /// 
    /// Read-only.
    pub name: Option<Name>,
    /// Global permissions of the user.
    /// 
    /// Read-only.
    pub permissions: Option<Vec<GlobalPermission>>,
}

impl Resource for UserProfile {}
impl ResponseResult for UserProfile {}


/// A set of materials that appears on the "About" page of the course.
/// These materials might include a syllabus, schedule, or other background
/// information relating to the course as a whole.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct CourseMaterialSet {
    /// Materials attached to this set.
    pub materials: Option<Vec<CourseMaterial>>,
    /// Title for this set.
    pub title: Option<String>,
}

impl Part for CourseMaterialSet {}


/// Student work for an assignment.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct AssignmentSubmission {
    /// Attachments added by the student.
    /// Drive files that correspond to materials with a share mode of
    /// STUDENT_COPY may not exist yet if the student has not accessed the
    /// assignment in Classroom.
    /// 
    /// Some attachment metadata is only populated if the requesting user has
    /// permission to access it. Identifier and alternate_link fields are always
    /// available, but others (e.g. title) may not be.
    pub attachments: Option<Vec<Attachment>>,
}

impl Part for AssignmentSubmission {}


/// Google Forms item.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct Form {
    /// URL of the form.
    #[serde(rename="formUrl")]
    pub form_url: Option<String>,
    /// URL of a thumbnail image of the Form.
    /// 
    /// Read-only.
    #[serde(rename="thumbnailUrl")]
    pub thumbnail_url: Option<String>,
    /// URL of the form responses document.
    /// Only set if respsonses have been recorded and only when the
    /// requesting user is an editor of the form.
    /// 
    /// Read-only.
    #[serde(rename="responseUrl")]
    pub response_url: Option<String>,
    /// Title of the Form.
    /// 
    /// Read-only.
    pub title: Option<String>,
}

impl Part for Form {}


/// Association between a student and a guardian of that student. The guardian
/// may receive information about the student's course work.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [guardians get user profiles](struct.UserProfileGuardianGetCall.html) (response)
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct Guardian {
    /// Identifier for the guardian.
    #[serde(rename="guardianId")]
    pub guardian_id: Option<String>,
    /// The email address to which the initial guardian invitation was sent.
    /// This field is only visible to domain administrators.
    #[serde(rename="invitedEmailAddress")]
    pub invited_email_address: Option<String>,
    /// Identifier for the student to whom the guardian relationship applies.
    #[serde(rename="studentId")]
    pub student_id: Option<String>,
    /// User profile for the guardian.
    #[serde(rename="guardianProfile")]
    pub guardian_profile: Option<UserProfile>,
}

impl ResponseResult for Guardian {}


/// Request to turn in a student submission.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [course work student submissions turn in courses](struct.CourseCourseWorkStudentSubmissionTurnInCall.html) (request)
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct TurnInStudentSubmissionRequest { _never_set: Option<bool> }

impl RequestValue for TurnInStudentSubmissionRequest {}


/// Material attached to course work.
/// 
/// When creating attachments, setting the `form` field is not supported.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct Material {
    /// Link material. On creation, will be upgraded to a more appropriate type
    /// if possible, and this will be reflected in the response.
    pub link: Option<Link>,
    /// Google Drive file material.
    #[serde(rename="driveFile")]
    pub drive_file: Option<SharedDriveFile>,
    /// YouTube video material.
    #[serde(rename="youtubeVideo")]
    pub youtube_video: Option<YouTubeVideo>,
    /// Google Forms material.
    pub form: Option<Form>,
}

impl Part for Material {}


/// YouTube video item.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct YouTubeVideo {
    /// URL that can be used to view the YouTube video.
    /// 
    /// Read-only.
    #[serde(rename="alternateLink")]
    pub alternate_link: Option<String>,
    /// URL of a thumbnail image of the YouTube video.
    /// 
    /// Read-only.
    #[serde(rename="thumbnailUrl")]
    pub thumbnail_url: Option<String>,
    /// YouTube API resource ID.
    pub id: Option<String>,
    /// Title of the YouTube video.
    /// 
    /// Read-only.
    pub title: Option<String>,
}

impl Part for YouTubeVideo {}


/// Response when listing teachers.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [teachers list courses](struct.CourseTeacherListCall.html) (response)
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct ListTeachersResponse {
    /// Token identifying the next page of results to return. If empty, no further
    /// results are available.
    #[serde(rename="nextPageToken")]
    pub next_page_token: Option<String>,
    /// Teachers who match the list request.
    pub teachers: Option<Vec<Teacher>>,
}

impl ResponseResult for ListTeachersResponse {}


/// Representation of a Google Drive file.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct DriveFile {
    /// Title of the Drive item.
    /// 
    /// Read-only.
    pub title: Option<String>,
    /// URL of a thumbnail image of the Drive item.
    /// 
    /// Read-only.
    #[serde(rename="thumbnailUrl")]
    pub thumbnail_url: Option<String>,
    /// Drive API resource ID.
    pub id: Option<String>,
    /// URL that can be used to access the Drive item.
    /// 
    /// Read-only.
    #[serde(rename="alternateLink")]
    pub alternate_link: Option<String>,
}

impl Part for DriveFile {}


/// URL item.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct Link {
    /// URL to link to.
    /// This must be a valid UTF-8 string containing between 1 and 2024 characters.
    pub url: Option<String>,
    /// URL of a thumbnail image of the target URL.
    /// 
    /// Read-only.
    #[serde(rename="thumbnailUrl")]
    pub thumbnail_url: Option<String>,
    /// Title of the target of the URL.
    /// 
    /// Read-only.
    pub title: Option<String>,
}

impl Part for Link {}


/// Student in a course.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [students create courses](struct.CourseStudentCreateCall.html) (request|response)
/// * [students get courses](struct.CourseStudentGetCall.html) (response)
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct Student {
    /// Identifier of the course.
    /// 
    /// Read-only.
    #[serde(rename="courseId")]
    pub course_id: Option<String>,
    /// Global user information for the student.
    /// 
    /// Read-only.
    pub profile: Option<UserProfile>,
    /// Information about a Drive Folder for this student's work in this course.
    /// Only visible to the student and domain administrators.
    /// 
    /// Read-only.
    #[serde(rename="studentWorkFolder")]
    pub student_work_folder: Option<DriveFolder>,
    /// Identifier of the user.
    /// 
    /// When specified as a parameter of a request, this identifier can be one of
    /// the following:
    /// 
    /// * the numeric identifier for the user
    /// * the email address of the user
    /// * the string literal `"me"`, indicating the requesting user
    #[serde(rename="userId")]
    pub user_id: Option<String>,
}

impl RequestValue for Student {}
impl ResponseResult for Student {}


/// Represents a whole calendar date, e.g. date of birth. The time of day and
/// time zone are either specified elsewhere or are not significant. The date
/// is relative to the Proleptic Gregorian Calendar. The day may be 0 to
/// represent a year and month where the day is not significant, e.g. credit card
/// expiration date. The year may be 0 to represent a month and day independent
/// of year, e.g. anniversary date. Related types are google.type.TimeOfDay
/// and `google.protobuf.Timestamp`.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct Date {
    /// Year of date. Must be from 1 to 9999, or 0 if specifying a date without
    /// a year.
    pub year: Option<i32>,
    /// Day of month. Must be from 1 to 31 and valid for the year and month, or 0
    /// if specifying a year/month where the day is not significant.
    pub day: Option<i32>,
    /// Month of year. Must be from 1 to 12.
    pub month: Option<i32>,
}

impl Part for Date {}


/// Drive file that is used as material for course work.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct SharedDriveFile {
    /// Drive file details.
    #[serde(rename="driveFile")]
    pub drive_file: Option<DriveFile>,
    /// Mechanism by which students access the Drive item.
    #[serde(rename="shareMode")]
    pub share_mode: Option<String>,
}

impl Part for SharedDriveFile {}


/// Response when listing student submissions.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [course work student submissions list courses](struct.CourseCourseWorkStudentSubmissionListCall.html) (response)
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct ListStudentSubmissionsResponse {
    /// Token identifying the next page of results to return. If empty, no further
    /// results are available.
    #[serde(rename="nextPageToken")]
    pub next_page_token: Option<String>,
    /// Student work that matches the request.
    #[serde(rename="studentSubmissions")]
    pub student_submissions: Option<Vec<StudentSubmission>>,
}

impl ResponseResult for ListStudentSubmissionsResponse {}


/// Response when listing courses.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [list courses](struct.CourseListCall.html) (response)
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct ListCoursesResponse {
    /// Token identifying the next page of results to return. If empty, no further
    /// results are available.
    #[serde(rename="nextPageToken")]
    pub next_page_token: Option<String>,
    /// Courses that match the list request.
    pub courses: Option<Vec<Course>>,
}

impl ResponseResult for ListCoursesResponse {}


/// Teacher of a course.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [teachers create courses](struct.CourseTeacherCreateCall.html) (request|response)
/// * [teachers get courses](struct.CourseTeacherGetCall.html) (response)
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct Teacher {
    /// Identifier of the course.
    /// 
    /// Read-only.
    #[serde(rename="courseId")]
    pub course_id: Option<String>,
    /// Global user information for the teacher.
    /// 
    /// Read-only.
    pub profile: Option<UserProfile>,
    /// Identifier of the user.
    /// 
    /// When specified as a parameter of a request, this identifier can be one of
    /// the following:
    /// 
    /// * the numeric identifier for the user
    /// * the email address of the user
    /// * the string literal `"me"`, indicating the requesting user
    #[serde(rename="userId")]
    pub user_id: Option<String>,
}

impl RequestValue for Teacher {}
impl ResponseResult for Teacher {}


/// A material attached to a course as part of a material set.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct CourseMaterial {
    /// Youtube video attachment.
    #[serde(rename="youTubeVideo")]
    pub you_tube_video: Option<YouTubeVideo>,
    /// Google Drive file attachment.
    #[serde(rename="driveFile")]
    pub drive_file: Option<DriveFile>,
    /// Link atatchment.
    pub link: Option<Link>,
    /// Google Forms attachment.
    pub form: Option<Form>,
}

impl Part for CourseMaterial {}


/// Details of the user's name.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct Name {
    /// The user's full name formed by concatenating the first and last name
    /// values.
    /// 
    /// Read-only.
    #[serde(rename="fullName")]
    pub full_name: Option<String>,
    /// The user's first name.
    /// 
    /// Read-only.
    #[serde(rename="givenName")]
    pub given_name: Option<String>,
    /// The user's last name.
    /// 
    /// Read-only.
    #[serde(rename="familyName")]
    pub family_name: Option<String>,
}

impl Part for Name {}


/// Response when listing invitations.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [list invitations](struct.InvitationListCall.html) (response)
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct ListInvitationsResponse {
    /// Token identifying the next page of results to return. If empty, no further
    /// results are available.
    #[serde(rename="nextPageToken")]
    pub next_page_token: Option<String>,
    /// Invitations that match the list request.
    pub invitations: Option<Vec<Invitation>>,
}

impl ResponseResult for ListInvitationsResponse {}


/// Representation of a Google Drive folder.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct DriveFolder {
    /// Title of the Drive folder.
    /// 
    /// Read-only.
    pub title: Option<String>,
    /// Drive API resource ID.
    pub id: Option<String>,
    /// URL that can be used to access the Drive folder.
    /// 
    /// Read-only.
    #[serde(rename="alternateLink")]
    pub alternate_link: Option<String>,
}

impl Part for DriveFolder {}


/// An invitation to join a course.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [list invitations](struct.InvitationListCall.html) (none)
/// * [delete invitations](struct.InvitationDeleteCall.html) (none)
/// * [create invitations](struct.InvitationCreateCall.html) (request|response)
/// * [get invitations](struct.InvitationGetCall.html) (response)
/// * [accept invitations](struct.InvitationAcceptCall.html) (none)
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct Invitation {
    /// Identifier of the course to invite the user to.
    #[serde(rename="courseId")]
    pub course_id: Option<String>,
    /// Role to invite the user to have.
    /// Must not be `COURSE_ROLE_UNSPECIFIED`.
    pub role: Option<String>,
    /// Identifier of the invited user.
    /// 
    /// When specified as a parameter of a request, this identifier can be set to
    /// one of the following:
    /// 
    /// * the numeric identifier for the user
    /// * the email address of the user
    /// * the string literal `"me"`, indicating the requesting user
    #[serde(rename="userId")]
    pub user_id: Option<String>,
    /// Identifier assigned by Classroom.
    /// 
    /// Read-only.
    pub id: Option<String>,
}

impl RequestValue for Invitation {}
impl Resource for Invitation {}
impl ResponseResult for Invitation {}


/// Response when listing students.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [students list courses](struct.CourseStudentListCall.html) (response)
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct ListStudentsResponse {
    /// Students who match the list request.
    pub students: Option<Vec<Student>>,
    /// Token identifying the next page of results to return. If empty, no further
    /// results are available.
    #[serde(rename="nextPageToken")]
    pub next_page_token: Option<String>,
}

impl ResponseResult for ListStudentsResponse {}


/// Additional details for multiple-choice questions.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct MultipleChoiceQuestion {
    /// Possible choices.
    pub choices: Option<Vec<String>>,
}

impl Part for MultipleChoiceQuestion {}


/// Response when listing course aliases.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [aliases list courses](struct.CourseAliaseListCall.html) (response)
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct ListCourseAliasesResponse {
    /// Token identifying the next page of results to return. If empty, no further
    /// results are available.
    #[serde(rename="nextPageToken")]
    pub next_page_token: Option<String>,
    /// The course aliases.
    pub aliases: Option<Vec<CourseAlias>>,
}

impl ResponseResult for ListCourseAliasesResponse {}


/// Request to reclaim a student submission.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [course work student submissions reclaim courses](struct.CourseCourseWorkStudentSubmissionReclaimCall.html) (request)
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct ReclaimStudentSubmissionRequest { _never_set: Option<bool> }

impl RequestValue for ReclaimStudentSubmissionRequest {}


/// Course work created by a teacher for students of the course.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [course work create courses](struct.CourseCourseWorkCreateCall.html) (request|response)
/// * [course work get courses](struct.CourseCourseWorkGetCall.html) (response)
/// * [course work patch courses](struct.CourseCourseWorkPatchCall.html) (request|response)
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct CourseWork {
    /// Timestamp of the most recent change to this course work.
    /// 
    /// Read-only.
    #[serde(rename="updateTime")]
    pub update_time: Option<String>,
    /// Optional description of this course work.
    /// If set, the description must be a valid UTF-8 string containing no more
    /// than 30,000 characters.
    pub description: Option<String>,
    /// Identifier of the course.
    /// 
    /// Read-only.
    #[serde(rename="courseId")]
    pub course_id: Option<String>,
    /// Assignment details.
    /// This is populated only when `work_type` is `ASSIGNMENT`.
    /// 
    /// Read-only.
    pub assignment: Option<Assignment>,
    /// Whether this course work item is associated with the Developer Console
    /// project making the request.
    /// 
    /// See google.classroom.Work.CreateCourseWork for more
    /// details.
    /// 
    /// Read-only.
    #[serde(rename="associatedWithDeveloper")]
    pub associated_with_developer: Option<bool>,
    /// Maximum grade for this course work.
    /// If zero or unspecified, this assignment is considered ungraded.
    /// This must be a non-negative integer value.
    #[serde(rename="maxPoints")]
    pub max_points: Option<f64>,
    /// Optional date, in UTC, that submissions for this this course work are due.
    /// This must be specified if `due_time` is specified.
    #[serde(rename="dueDate")]
    pub due_date: Option<Date>,
    /// Type of this course work.
    /// 
    /// The type is set when the course work is created and cannot be changed.
    #[serde(rename="workType")]
    pub work_type: Option<String>,
    /// Setting to determine when students are allowed to modify submissions.
    /// If unspecified, the default value is `MODIFIABLE_UNTIL_TURNED_IN`.
    #[serde(rename="submissionModificationMode")]
    pub submission_modification_mode: Option<String>,
    /// Absolute link to this course work in the Classroom web UI.
    /// This is only populated if `state` is `PUBLISHED`.
    /// 
    /// Read-only.
    #[serde(rename="alternateLink")]
    pub alternate_link: Option<String>,
    /// Title of this course work.
    /// The title must be a valid UTF-8 string containing between 1 and 3000
    /// characters.
    pub title: Option<String>,
    /// Timestamp when this course work was created.
    /// 
    /// Read-only.
    #[serde(rename="creationTime")]
    pub creation_time: Option<String>,
    /// Classroom-assigned identifier of this course work, unique per course.
    /// 
    /// Read-only.
    pub id: Option<String>,
    /// Status of this course work.
    /// If unspecified, the default state is `DRAFT`.
    pub state: Option<String>,
    /// Additional materials.
    /// 
    /// CourseWork must have no more than 20 material items.
    pub materials: Option<Vec<Material>>,
    /// Optional time of day, in UTC, that submissions for this this course work
    /// are due.
    /// This must be specified if `due_date` is specified.
    #[serde(rename="dueTime")]
    pub due_time: Option<TimeOfDay>,
    /// Multiple choice question details.
    /// For read operations, this field is populated only when `work_type` is
    /// `MULTIPLE_CHOICE_QUESTION`.
    /// For write operations, this field must be specified when creating course
    /// work with a `work_type` of `MULTIPLE_CHOICE_QUESTION`, and it must not be
    /// set otherwise.
    #[serde(rename="multipleChoiceQuestion")]
    pub multiple_choice_question: Option<MultipleChoiceQuestion>,
}

impl RequestValue for CourseWork {}
impl ResponseResult for CourseWork {}


/// Student submission for course work.
/// 
/// StudentSubmission items are generated when a CourseWork item is created.
/// 
/// StudentSubmissions that have never been accessed (i.e. with `state` = NEW)
/// may not have a creation time or update time.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [course work student submissions patch courses](struct.CourseCourseWorkStudentSubmissionPatchCall.html) (request|response)
/// * [course work student submissions get courses](struct.CourseCourseWorkStudentSubmissionGetCall.html) (response)
/// * [course work student submissions modify attachments courses](struct.CourseCourseWorkStudentSubmissionModifyAttachmentCall.html) (response)
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct StudentSubmission {
    /// Last update time of this submission.
    /// This may be unset if the student has not accessed this item.
    /// 
    /// Read-only.
    #[serde(rename="updateTime")]
    pub update_time: Option<String>,
    /// Identifier of the course.
    /// 
    /// Read-only.
    #[serde(rename="courseId")]
    pub course_id: Option<String>,
    /// Identifier for the student that owns this submission.
    /// 
    /// Read-only.
    #[serde(rename="userId")]
    pub user_id: Option<String>,
    /// Whether this student submission is associated with the Developer Console
    /// project making the request.
    /// 
    /// See google.classroom.Work.CreateCourseWork for more
    /// details.
    /// 
    /// Read-only.
    #[serde(rename="associatedWithDeveloper")]
    pub associated_with_developer: Option<bool>,
    /// Type of course work this submission is for.
    /// 
    /// Read-only.
    #[serde(rename="courseWorkType")]
    pub course_work_type: Option<String>,
    /// Optional grade. If unset, no grade was set.
    /// This must be a non-negative integer value.
    /// 
    /// This may be modified only by course teachers.
    #[serde(rename="assignedGrade")]
    pub assigned_grade: Option<f64>,
    /// Classroom-assigned Identifier for the student submission.
    /// This is unique among submissions for the relevant course work.
    /// 
    /// Read-only.
    pub id: Option<String>,
    /// Optional pending grade. If unset, no grade was set.
    /// This must be a non-negative integer value.
    /// 
    /// This is only visible to and modifiable by course teachers.
    #[serde(rename="draftGrade")]
    pub draft_grade: Option<f64>,
    /// Absolute link to the submission in the Classroom web UI.
    /// 
    /// Read-only.
    #[serde(rename="alternateLink")]
    pub alternate_link: Option<String>,
    /// Submission content when course_work_type is MULTIPLE_CHOICE_QUESTION.
    #[serde(rename="multipleChoiceSubmission")]
    pub multiple_choice_submission: Option<MultipleChoiceSubmission>,
    /// Creation time of this submission.
    /// This may be unset if the student has not accessed this item.
    /// 
    /// Read-only.
    #[serde(rename="creationTime")]
    pub creation_time: Option<String>,
    /// Submission content when course_work_type is SHORT_ANSWER_QUESTION.
    #[serde(rename="shortAnswerSubmission")]
    pub short_answer_submission: Option<ShortAnswerSubmission>,
    /// Whether this submission is late.
    /// 
    /// Read-only.
    pub late: Option<bool>,
    /// State of this submission.
    /// 
    /// Read-only.
    pub state: Option<String>,
    /// Identifier for the course work this corresponds to.
    /// 
    /// Read-only.
    #[serde(rename="courseWorkId")]
    pub course_work_id: Option<String>,
    /// Submission content when course_work_type is ASSIGNMENT .
    #[serde(rename="assignmentSubmission")]
    pub assignment_submission: Option<AssignmentSubmission>,
}

impl RequestValue for StudentSubmission {}
impl ResponseResult for StudentSubmission {}


/// Response when listing guardian invitations.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [guardian invitations list user profiles](struct.UserProfileGuardianInvitationListCall.html) (response)
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct ListGuardianInvitationsResponse {
    /// Token identifying the next page of results to return. If empty, no further
    /// results are available.
    #[serde(rename="nextPageToken")]
    pub next_page_token: Option<String>,
    /// Guardian invitations that matched the list request.
    #[serde(rename="guardianInvitations")]
    pub guardian_invitations: Option<Vec<GuardianInvitation>>,
}

impl ResponseResult for ListGuardianInvitationsResponse {}


/// A Course in Classroom.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [course work student submissions patch courses](struct.CourseCourseWorkStudentSubmissionPatchCall.html) (none)
/// * [course work student submissions list courses](struct.CourseCourseWorkStudentSubmissionListCall.html) (none)
/// * [get courses](struct.CourseGetCall.html) (response)
/// * [update courses](struct.CourseUpdateCall.html) (request|response)
/// * [students get courses](struct.CourseStudentGetCall.html) (none)
/// * [teachers get courses](struct.CourseTeacherGetCall.html) (none)
/// * [course work list courses](struct.CourseCourseWorkListCall.html) (none)
/// * [course work get courses](struct.CourseCourseWorkGetCall.html) (none)
/// * [course work student submissions turn in courses](struct.CourseCourseWorkStudentSubmissionTurnInCall.html) (none)
/// * [course work student submissions modify attachments courses](struct.CourseCourseWorkStudentSubmissionModifyAttachmentCall.html) (none)
/// * [teachers list courses](struct.CourseTeacherListCall.html) (none)
/// * [course work student submissions return courses](struct.CourseCourseWorkStudentSubmissionReturnCall.html) (none)
/// * [aliases list courses](struct.CourseAliaseListCall.html) (none)
/// * [course work create courses](struct.CourseCourseWorkCreateCall.html) (none)
/// * [list courses](struct.CourseListCall.html) (none)
/// * [course work student submissions reclaim courses](struct.CourseCourseWorkStudentSubmissionReclaimCall.html) (none)
/// * [aliases create courses](struct.CourseAliaseCreateCall.html) (none)
/// * [students create courses](struct.CourseStudentCreateCall.html) (none)
/// * [aliases delete courses](struct.CourseAliaseDeleteCall.html) (none)
/// * [course work delete courses](struct.CourseCourseWorkDeleteCall.html) (none)
/// * [create courses](struct.CourseCreateCall.html) (request|response)
/// * [students list courses](struct.CourseStudentListCall.html) (none)
/// * [delete courses](struct.CourseDeleteCall.html) (none)
/// * [course work patch courses](struct.CourseCourseWorkPatchCall.html) (none)
/// * [patch courses](struct.CoursePatchCall.html) (request|response)
/// * [students delete courses](struct.CourseStudentDeleteCall.html) (none)
/// * [teachers delete courses](struct.CourseTeacherDeleteCall.html) (none)
/// * [teachers create courses](struct.CourseTeacherCreateCall.html) (none)
/// * [course work student submissions get courses](struct.CourseCourseWorkStudentSubmissionGetCall.html) (none)
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct Course {
    /// Time of the most recent update to this course.
    /// Specifying this field in a course update mask results in an error.
    /// 
    /// Read-only.
    #[serde(rename="updateTime")]
    pub update_time: Option<String>,
    /// Optional description.
    /// For example, "We'll be learning about the structure of living
    /// creatures from a combination of textbooks, guest lectures, and lab work.
    /// Expect to be excited!"
    /// If set, this field must be a valid UTF-8 string and no longer than 30,000
    /// characters.
    pub description: Option<String>,
    /// Enrollment code to use when joining this course.
    /// Specifying this field in a course update mask results in an error.
    /// 
    /// Read-only.
    #[serde(rename="enrollmentCode")]
    pub enrollment_code: Option<String>,
    /// Whether or not guardian notifications are enabled for this course.
    /// 
    /// Read-only.
    #[serde(rename="guardiansEnabled")]
    pub guardians_enabled: Option<bool>,
    /// The email address of a Google group containing all members of the course.
    /// This group does not accept email and can only be used for permissions.
    /// 
    /// Read-only.
    #[serde(rename="courseGroupEmail")]
    pub course_group_email: Option<String>,
    /// Sets of materials that appear on the "about" page of this course.
    /// 
    /// Read-only.
    #[serde(rename="courseMaterialSets")]
    pub course_material_sets: Option<Vec<CourseMaterialSet>>,
    /// State of the course.
    /// If unspecified, the default state is `PROVISIONED`.
    #[serde(rename="courseState")]
    pub course_state: Option<String>,
    /// Identifier for this course assigned by Classroom.
    /// 
    /// When
    /// creating a course,
    /// you may optionally set this identifier to an
    /// alias string in the
    /// request to create a corresponding alias. The `id` is still assigned by
    /// Classroom and cannot be updated after the course is created.
    /// 
    /// Specifying this field in a course update mask results in an error.
    pub id: Option<String>,
    /// Name of the course.
    /// For example, "10th Grade Biology".
    /// The name is required. It must be between 1 and 750 characters and a valid
    /// UTF-8 string.
    pub name: Option<String>,
    /// Optional room location.
    /// For example, "301".
    /// If set, this field must be a valid UTF-8 string and no longer than 650
    /// characters.
    pub room: Option<String>,
    /// Absolute link to this course in the Classroom web UI.
    /// 
    /// Read-only.
    #[serde(rename="alternateLink")]
    pub alternate_link: Option<String>,
    /// Section of the course.
    /// For example, "Period 2".
    /// If set, this field must be a valid UTF-8 string and no longer than 2800
    /// characters.
    pub section: Option<String>,
    /// Creation time of the course.
    /// Specifying this field in a course update mask results in an error.
    /// 
    /// Read-only.
    #[serde(rename="creationTime")]
    pub creation_time: Option<String>,
    /// The email address of a Google group containing all teachers of the course.
    /// This group does not accept email and can only be used for permissions.
    /// 
    /// Read-only.
    #[serde(rename="teacherGroupEmail")]
    pub teacher_group_email: Option<String>,
    /// Information about a Drive Folder that is shared with all teachers of the
    /// course.
    /// 
    /// This field will only be set for teachers of the course and domain administrators.
    /// 
    /// Read-only.
    #[serde(rename="teacherFolder")]
    pub teacher_folder: Option<DriveFolder>,
    /// The identifier of the owner of a course.
    /// 
    /// When specified as a parameter of a
    /// create course request, this
    /// field is required.
    /// The identifier can be one of the following:
    /// 
    /// * the numeric identifier for the user
    /// * the email address of the user
    /// * the string literal `"me"`, indicating the requesting user
    /// 
    /// This must be set in a create request. Specifying this field in a course
    /// update mask results in an `INVALID_ARGUMENT` error.
    #[serde(rename="ownerId")]
    pub owner_id: Option<String>,
    /// Optional heading for the description.
    /// For example, "Welcome to 10th Grade Biology."
    /// If set, this field must be a valid UTF-8 string and no longer than 3600
    /// characters.
    #[serde(rename="descriptionHeading")]
    pub description_heading: Option<String>,
}

impl RequestValue for Course {}
impl Resource for Course {}
impl ResponseResult for Course {}


/// Alternative identifier for a course.
/// 
/// An alias uniquely identifies a course. It must be unique within one of the
/// following scopes:
/// 
/// * domain: A domain-scoped alias is visible to all users within the alias
/// creator's domain and can be created only by a domain admin. A domain-scoped
/// alias is often used when a course has an identifier external to Classroom.
/// 
/// * project: A project-scoped alias is visible to any request from an
/// application using the Developer Console project ID that created the alias
/// and can be created by any project. A project-scoped alias is often used when
/// an application has alternative identifiers. A random value can also be used
/// to avoid duplicate courses in the event of transmission failures, as retrying
/// a request will return `ALREADY_EXISTS` if a previous one has succeeded.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [aliases create courses](struct.CourseAliaseCreateCall.html) (request|response)
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct CourseAlias {
    /// Alias string. The format of the string indicates the desired alias scoping.
    /// 
    /// * `d:<name>` indicates a domain-scoped alias.
    ///   Example: `d:math_101`
    /// * `p:<name>` indicates a project-scoped alias.
    ///   Example: `p:abc123`
    /// 
    /// This field has a maximum length of 256 characters.
    pub alias: Option<String>,
}

impl RequestValue for CourseAlias {}
impl ResponseResult for CourseAlias {}


/// Additional details for assignments.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct Assignment {
    /// Drive folder where attachments from student submissions are placed.
    /// This is only populated for course teachers.
    #[serde(rename="studentWorkFolder")]
    pub student_work_folder: Option<DriveFolder>,
}

impl Part for Assignment {}


/// Global user permission description.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GlobalPermission {
    /// Permission value.
    pub permission: Option<String>,
}

impl Part for GlobalPermission {}


/// Response when listing course work.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [course work list courses](struct.CourseCourseWorkListCall.html) (response)
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct ListCourseWorkResponse {
    /// Token identifying the next page of results to return. If empty, no further
    /// results are available.
    #[serde(rename="nextPageToken")]
    pub next_page_token: Option<String>,
    /// Course work items that match the request.
    #[serde(rename="courseWork")]
    pub course_work: Option<Vec<CourseWork>>,
}

impl ResponseResult for ListCourseWorkResponse {}


/// An invitation to become the guardian of a specified user, sent to a specified
/// email address.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [guardian invitations create user profiles](struct.UserProfileGuardianInvitationCreateCall.html) (request|response)
/// * [guardian invitations patch user profiles](struct.UserProfileGuardianInvitationPatchCall.html) (request|response)
/// * [guardian invitations get user profiles](struct.UserProfileGuardianInvitationGetCall.html) (response)
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GuardianInvitation {
    /// The state that this invitation is in.
    pub state: Option<String>,
    /// ID of the student (in standard format)
    #[serde(rename="studentId")]
    pub student_id: Option<String>,
    /// The time that this invitation was created.
    /// 
    /// Read-only.
    #[serde(rename="creationTime")]
    pub creation_time: Option<String>,
    /// Email address that the invitation was sent to.
    /// This field is only visible to domain administrators.
    #[serde(rename="invitedEmailAddress")]
    pub invited_email_address: Option<String>,
    /// Unique identifier for this invitation.
    /// 
    /// Read-only.
    #[serde(rename="invitationId")]
    pub invitation_id: Option<String>,
}

impl RequestValue for GuardianInvitation {}
impl ResponseResult for GuardianInvitation {}



// ###################
// MethodBuilders ###
// #################

/// A builder providing access to all methods supported on *course* resources.
/// It is not used directly, but through the `Classroom` hub.
///
/// # Example
///
/// Instantiate a resource builder
///
/// ```test_harness,no_run
/// extern crate hyper;
/// extern crate hyper_rustls;
/// extern crate yup_oauth2 as oauth2;
/// extern crate google_classroom1 as classroom1;
/// 
/// # #[test] fn egal() {
/// use std::default::Default;
/// use oauth2::{Authenticator, DefaultAuthenticatorDelegate, ApplicationSecret, MemoryStorage};
/// use classroom1::Classroom;
/// 
/// let secret: ApplicationSecret = Default::default();
/// let auth = Authenticator::new(&secret, DefaultAuthenticatorDelegate,
///                               hyper::Client::with_connector(hyper::net::HttpsConnector::new(hyper_rustls::TlsClient::new())),
///                               <MemoryStorage as Default>::default(), None);
/// let mut hub = Classroom::new(hyper::Client::with_connector(hyper::net::HttpsConnector::new(hyper_rustls::TlsClient::new())), auth);
/// // Usually you wouldn't bind this to a variable, but keep calling *CallBuilders*
/// // like `aliases_create(...)`, `aliases_delete(...)`, `aliases_list(...)`, `course_work_create(...)`, `course_work_delete(...)`, `course_work_get(...)`, `course_work_list(...)`, `course_work_patch(...)`, `course_work_student_submissions_get(...)`, `course_work_student_submissions_list(...)`, `course_work_student_submissions_modify_attachments(...)`, `course_work_student_submissions_patch(...)`, `course_work_student_submissions_reclaim(...)`, `course_work_student_submissions_return(...)`, `course_work_student_submissions_turn_in(...)`, `create(...)`, `delete(...)`, `get(...)`, `list(...)`, `patch(...)`, `students_create(...)`, `students_delete(...)`, `students_get(...)`, `students_list(...)`, `teachers_create(...)`, `teachers_delete(...)`, `teachers_get(...)`, `teachers_list(...)` and `update(...)`
/// // to build up your call.
/// let rb = hub.courses();
/// # }
/// ```
pub struct CourseMethods<'a, C, A>
    where C: 'a, A: 'a {

    hub: &'a Classroom<C, A>,
}

impl<'a, C, A> MethodsBuilder for CourseMethods<'a, C, A> {}

impl<'a, C, A> CourseMethods<'a, C, A> {
    
    /// Create a builder to help you perform the following task:
    ///
    /// Updates one or more fields of a student submission.
    /// 
    /// See google.classroom.v1.StudentSubmission for details
    /// of which fields may be updated and who may change them.
    /// 
    /// This request must be made by the Developer Console project of the
    /// [OAuth client ID](https://support.google.com/cloud/answer/6158849) used to
    /// create the corresponding course work item.
    /// 
    /// This method returns the following error codes:
    /// 
    /// * `PERMISSION_DENIED` if the requesting developer project did not create
    /// the corresponding course work, if the user is not permitted to make the
    /// requested modification to the student submission, or for
    /// access errors.
    /// * `INVALID_ARGUMENT` if the request is malformed.
    /// * `NOT_FOUND` if the requested course, course work, or student submission
    /// does not exist.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `courseId` - Identifier of the course.
    ///                This identifier can be either the Classroom-assigned identifier or an
    ///                alias.
    /// * `courseWorkId` - Identifier of the course work.
    /// * `id` - Identifier of the student submission.
    pub fn course_work_student_submissions_patch(&self, request: StudentSubmission, course_id: &str, course_work_id: &str, id: &str) -> CourseCourseWorkStudentSubmissionPatchCall<'a, C, A> {
        CourseCourseWorkStudentSubmissionPatchCall {
            hub: self.hub,
            _request: request,
            _course_id: course_id.to_string(),
            _course_work_id: course_work_id.to_string(),
            _id: id.to_string(),
            _update_mask: Default::default(),
            _delegate: Default::default(),
            _scopes: Default::default(),
            _additional_params: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Returns a list of student submissions that the requester is permitted to
    /// view, factoring in the OAuth scopes of the request.
    /// `-` may be specified as the `course_work_id` to include student
    /// submissions for multiple course work items.
    /// 
    /// Course students may only view their own work. Course teachers
    /// and domain administrators may view all student submissions.
    /// 
    /// This method returns the following error codes:
    /// 
    /// * `PERMISSION_DENIED` if the requesting user is not permitted to access the
    /// requested course or course work, or for access errors.
    /// * `INVALID_ARGUMENT` if the request is malformed.
    /// * `NOT_FOUND` if the requested course does not exist.
    /// 
    /// # Arguments
    ///
    /// * `courseId` - Identifier of the course.
    ///                This identifier can be either the Classroom-assigned identifier or an
    ///                alias.
    /// * `courseWorkId` - Identifer of the student work to request.
    ///                    This may be set to the string literal `"-"` to request student work for
    ///                    all course work in the specified course.
    pub fn course_work_student_submissions_list(&self, course_id: &str, course_work_id: &str) -> CourseCourseWorkStudentSubmissionListCall<'a, C, A> {
        CourseCourseWorkStudentSubmissionListCall {
            hub: self.hub,
            _course_id: course_id.to_string(),
            _course_work_id: course_work_id.to_string(),
            _user_id: Default::default(),
            _states: Default::default(),
            _page_token: Default::default(),
            _page_size: Default::default(),
            _late: Default::default(),
            _delegate: Default::default(),
            _scopes: Default::default(),
            _additional_params: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Returns a course.
    /// 
    /// This method returns the following error codes:
    /// 
    /// * `PERMISSION_DENIED` if the requesting user is not permitted to access the
    /// requested course or for access errors.
    /// * `NOT_FOUND` if no course exists with the requested ID.
    /// 
    /// # Arguments
    ///
    /// * `id` - Identifier of the course to return.
    ///          This identifier can be either the Classroom-assigned identifier or an
    ///          alias.
    pub fn get(&self, id: &str) -> CourseGetCall<'a, C, A> {
        CourseGetCall {
            hub: self.hub,
            _id: id.to_string(),
            _delegate: Default::default(),
            _scopes: Default::default(),
            _additional_params: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Updates a course.
    /// 
    /// This method returns the following error codes:
    /// 
    /// * `PERMISSION_DENIED` if the requesting user is not permitted to modify the
    /// requested course or for access errors.
    /// * `NOT_FOUND` if no course exists with the requested ID.
    /// * `FAILED_PRECONDITION` for the following request errors:
    ///     * CourseNotModifiable
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `id` - Identifier of the course to update.
    ///          This identifier can be either the Classroom-assigned identifier or an
    ///          alias.
    pub fn update(&self, request: Course, id: &str) -> CourseUpdateCall<'a, C, A> {
        CourseUpdateCall {
            hub: self.hub,
            _request: request,
            _id: id.to_string(),
            _delegate: Default::default(),
            _scopes: Default::default(),
            _additional_params: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Returns a student of a course.
    /// 
    /// This method returns the following error codes:
    /// 
    /// * `PERMISSION_DENIED` if the requesting user is not permitted to view
    /// students of this course or for access errors.
    /// * `NOT_FOUND` if no student of this course has the requested ID or if the
    /// course does not exist.
    /// 
    /// # Arguments
    ///
    /// * `courseId` - Identifier of the course.
    ///                This identifier can be either the Classroom-assigned identifier or an
    ///                alias.
    /// * `userId` - Identifier of the student to return. The identifier can be one of the
    ///              following:
    ///              * the numeric identifier for the user
    ///              * the email address of the user
    ///              * the string literal `"me"`, indicating the requesting user
    pub fn students_get(&self, course_id: &str, user_id: &str) -> CourseStudentGetCall<'a, C, A> {
        CourseStudentGetCall {
            hub: self.hub,
            _course_id: course_id.to_string(),
            _user_id: user_id.to_string(),
            _delegate: Default::default(),
            _scopes: Default::default(),
            _additional_params: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Returns a teacher of a course.
    /// 
    /// This method returns the following error codes:
    /// 
    /// * `PERMISSION_DENIED` if the requesting user is not permitted to view
    /// teachers of this course or for access errors.
    /// * `NOT_FOUND` if no teacher of this course has the requested ID or if the
    /// course does not exist.
    /// 
    /// # Arguments
    ///
    /// * `courseId` - Identifier of the course.
    ///                This identifier can be either the Classroom-assigned identifier or an
    ///                alias.
    /// * `userId` - Identifier of the teacher to return. The identifier can be one of the
    ///              following:
    ///              * the numeric identifier for the user
    ///              * the email address of the user
    ///              * the string literal `"me"`, indicating the requesting user
    pub fn teachers_get(&self, course_id: &str, user_id: &str) -> CourseTeacherGetCall<'a, C, A> {
        CourseTeacherGetCall {
            hub: self.hub,
            _course_id: course_id.to_string(),
            _user_id: user_id.to_string(),
            _delegate: Default::default(),
            _scopes: Default::default(),
            _additional_params: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Returns a list of course work that the requester is permitted to view.
    /// 
    /// Course students may only view `PUBLISHED` course work. Course teachers
    /// and domain administrators may view all course work.
    /// 
    /// This method returns the following error codes:
    /// 
    /// * `PERMISSION_DENIED` if the requesting user is not permitted to access
    /// the requested course or for access errors.
    /// * `INVALID_ARGUMENT` if the request is malformed.
    /// * `NOT_FOUND` if the requested course does not exist.
    /// 
    /// # Arguments
    ///
    /// * `courseId` - Identifier of the course.
    ///                This identifier can be either the Classroom-assigned identifier or an
    ///                alias.
    pub fn course_work_list(&self, course_id: &str) -> CourseCourseWorkListCall<'a, C, A> {
        CourseCourseWorkListCall {
            hub: self.hub,
            _course_id: course_id.to_string(),
            _page_token: Default::default(),
            _page_size: Default::default(),
            _order_by: Default::default(),
            _course_work_states: Default::default(),
            _delegate: Default::default(),
            _scopes: Default::default(),
            _additional_params: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Returns course work.
    /// 
    /// This method returns the following error codes:
    /// 
    /// * `PERMISSION_DENIED` if the requesting user is not permitted to access the
    /// requested course or course work, or for access errors.
    /// * `INVALID_ARGUMENT` if the request is malformed.
    /// * `NOT_FOUND` if the requested course or course work does not exist.
    /// 
    /// # Arguments
    ///
    /// * `courseId` - Identifier of the course.
    ///                This identifier can be either the Classroom-assigned identifier or an
    ///                alias.
    /// * `id` - Identifier of the course work.
    pub fn course_work_get(&self, course_id: &str, id: &str) -> CourseCourseWorkGetCall<'a, C, A> {
        CourseCourseWorkGetCall {
            hub: self.hub,
            _course_id: course_id.to_string(),
            _id: id.to_string(),
            _delegate: Default::default(),
            _scopes: Default::default(),
            _additional_params: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Turns in a student submission.
    /// 
    /// Turning in a student submission transfers ownership of attached Drive
    /// files to the teacher and may also update the submission state.
    /// 
    /// This may only be called by the student that owns the specified student
    /// submission.
    /// 
    /// This request must be made by the Developer Console project of the
    /// [OAuth client ID](https://support.google.com/cloud/answer/6158849) used to
    /// create the corresponding course work item.
    /// 
    /// This method returns the following error codes:
    /// 
    /// * `PERMISSION_DENIED` if the requesting user is not permitted to access the
    /// requested course or course work, turn in the requested student submission,
    /// or for access errors.
    /// * `INVALID_ARGUMENT` if the request is malformed.
    /// * `NOT_FOUND` if the requested course, course work, or student submission
    /// does not exist.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `courseId` - Identifier of the course.
    ///                This identifier can be either the Classroom-assigned identifier or an
    ///                alias.
    /// * `courseWorkId` - Identifier of the course work.
    /// * `id` - Identifier of the student submission.
    pub fn course_work_student_submissions_turn_in(&self, request: TurnInStudentSubmissionRequest, course_id: &str, course_work_id: &str, id: &str) -> CourseCourseWorkStudentSubmissionTurnInCall<'a, C, A> {
        CourseCourseWorkStudentSubmissionTurnInCall {
            hub: self.hub,
            _request: request,
            _course_id: course_id.to_string(),
            _course_work_id: course_work_id.to_string(),
            _id: id.to_string(),
            _delegate: Default::default(),
            _scopes: Default::default(),
            _additional_params: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Modifies attachments of student submission.
    /// 
    /// Attachments may only be added to student submissions belonging to course
    /// work objects with a `workType` of `ASSIGNMENT`.
    /// 
    /// This request must be made by the Developer Console project of the
    /// [OAuth client ID](https://support.google.com/cloud/answer/6158849) used to
    /// create the corresponding course work item.
    /// 
    /// This method returns the following error codes:
    /// 
    /// * `PERMISSION_DENIED` if the requesting user is not permitted to access the
    /// requested course or course work, if the user is not permitted to modify
    /// attachments on the requested student submission, or for
    /// access errors.
    /// * `INVALID_ARGUMENT` if the request is malformed.
    /// * `NOT_FOUND` if the requested course, course work, or student submission
    /// does not exist.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `courseId` - Identifier of the course.
    ///                This identifier can be either the Classroom-assigned identifier or an
    ///                alias.
    /// * `courseWorkId` - Identifier of the course work.
    /// * `id` - Identifier of the student submission.
    pub fn course_work_student_submissions_modify_attachments(&self, request: ModifyAttachmentsRequest, course_id: &str, course_work_id: &str, id: &str) -> CourseCourseWorkStudentSubmissionModifyAttachmentCall<'a, C, A> {
        CourseCourseWorkStudentSubmissionModifyAttachmentCall {
            hub: self.hub,
            _request: request,
            _course_id: course_id.to_string(),
            _course_work_id: course_work_id.to_string(),
            _id: id.to_string(),
            _delegate: Default::default(),
            _scopes: Default::default(),
            _additional_params: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Returns a list of teachers of this course that the requester
    /// is permitted to view.
    /// 
    /// This method returns the following error codes:
    /// 
    /// * `NOT_FOUND` if the course does not exist.
    /// * `PERMISSION_DENIED` for access errors.
    /// 
    /// # Arguments
    ///
    /// * `courseId` - Identifier of the course.
    ///                This identifier can be either the Classroom-assigned identifier or an
    ///                alias.
    pub fn teachers_list(&self, course_id: &str) -> CourseTeacherListCall<'a, C, A> {
        CourseTeacherListCall {
            hub: self.hub,
            _course_id: course_id.to_string(),
            _page_token: Default::default(),
            _page_size: Default::default(),
            _delegate: Default::default(),
            _scopes: Default::default(),
            _additional_params: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Returns a student submission.
    /// 
    /// Returning a student submission transfers ownership of attached Drive
    /// files to the student and may also update the submission state.
    /// Unlike the Classroom application, returning a student submission does not
    /// set assignedGrade to the draftGrade value.
    /// 
    /// Only a teacher of the course that contains the requested student submission
    /// may call this method.
    /// 
    /// This request must be made by the Developer Console project of the
    /// [OAuth client ID](https://support.google.com/cloud/answer/6158849) used to
    /// create the corresponding course work item.
    /// 
    /// This method returns the following error codes:
    /// 
    /// * `PERMISSION_DENIED` if the requesting user is not permitted to access the
    /// requested course or course work, return the requested student submission,
    /// or for access errors.
    /// * `INVALID_ARGUMENT` if the request is malformed.
    /// * `NOT_FOUND` if the requested course, course work, or student submission
    /// does not exist.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `courseId` - Identifier of the course.
    ///                This identifier can be either the Classroom-assigned identifier or an
    ///                alias.
    /// * `courseWorkId` - Identifier of the course work.
    /// * `id` - Identifier of the student submission.
    pub fn course_work_student_submissions_return(&self, request: ReturnStudentSubmissionRequest, course_id: &str, course_work_id: &str, id: &str) -> CourseCourseWorkStudentSubmissionReturnCall<'a, C, A> {
        CourseCourseWorkStudentSubmissionReturnCall {
            hub: self.hub,
            _request: request,
            _course_id: course_id.to_string(),
            _course_work_id: course_work_id.to_string(),
            _id: id.to_string(),
            _delegate: Default::default(),
            _scopes: Default::default(),
            _additional_params: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Returns a list of aliases for a course.
    /// 
    /// This method returns the following error codes:
    /// 
    /// * `PERMISSION_DENIED` if the requesting user is not permitted to access the
    /// course or for access errors.
    /// * `NOT_FOUND` if the course does not exist.
    /// 
    /// # Arguments
    ///
    /// * `courseId` - The identifier of the course.
    ///                This identifier can be either the Classroom-assigned identifier or an
    ///                alias.
    pub fn aliases_list(&self, course_id: &str) -> CourseAliaseListCall<'a, C, A> {
        CourseAliaseListCall {
            hub: self.hub,
            _course_id: course_id.to_string(),
            _page_token: Default::default(),
            _page_size: Default::default(),
            _delegate: Default::default(),
            _scopes: Default::default(),
            _additional_params: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Creates course work.
    /// 
    /// The resulting course work (and corresponding student submissions) are
    /// associated with the Developer Console project of the
    /// [OAuth client ID](https://support.google.com/cloud/answer/6158849) used to
    /// make the request. Classroom API requests to modify course work and student
    /// submissions must be made with an OAuth client ID from the associated
    /// Developer Console project.
    /// 
    /// This method returns the following error codes:
    /// 
    /// * `PERMISSION_DENIED` if the requesting user is not permitted to access the
    /// requested course, create course work in the requested course, share a
    /// Drive attachment, or for access errors.
    /// * `INVALID_ARGUMENT` if the request is malformed.
    /// * `NOT_FOUND` if the requested course does not exist.
    /// * `FAILED_PRECONDITION` for the following request error:
    ///     * AttachmentNotVisible
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `courseId` - Identifier of the course.
    ///                This identifier can be either the Classroom-assigned identifier or an
    ///                alias.
    pub fn course_work_create(&self, request: CourseWork, course_id: &str) -> CourseCourseWorkCreateCall<'a, C, A> {
        CourseCourseWorkCreateCall {
            hub: self.hub,
            _request: request,
            _course_id: course_id.to_string(),
            _delegate: Default::default(),
            _scopes: Default::default(),
            _additional_params: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Returns a list of courses that the requesting user is permitted to view,
    /// restricted to those that match the request.
    /// 
    /// This method returns the following error codes:
    /// 
    /// * `PERMISSION_DENIED` for access errors.
    /// * `INVALID_ARGUMENT` if the query argument is malformed.
    /// * `NOT_FOUND` if any users specified in the query arguments do not exist.
    pub fn list(&self) -> CourseListCall<'a, C, A> {
        CourseListCall {
            hub: self.hub,
            _teacher_id: Default::default(),
            _student_id: Default::default(),
            _page_token: Default::default(),
            _page_size: Default::default(),
            _course_states: Default::default(),
            _delegate: Default::default(),
            _scopes: Default::default(),
            _additional_params: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Reclaims a student submission on behalf of the student that owns it.
    /// 
    /// Reclaiming a student submission transfers ownership of attached Drive
    /// files to the student and update the submission state.
    /// 
    /// Only the student that owns the requested student submission may call this
    /// method, and only for a student submission that has been turned in.
    /// 
    /// This request must be made by the Developer Console project of the
    /// [OAuth client ID](https://support.google.com/cloud/answer/6158849) used to
    /// create the corresponding course work item.
    /// 
    /// This method returns the following error codes:
    /// 
    /// * `PERMISSION_DENIED` if the requesting user is not permitted to access the
    /// requested course or course work, unsubmit the requested student submission,
    /// or for access errors.
    /// * `FAILED_PRECONDITION` if the student submission has not been turned in.
    /// * `INVALID_ARGUMENT` if the request is malformed.
    /// * `NOT_FOUND` if the requested course, course work, or student submission
    /// does not exist.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `courseId` - Identifier of the course.
    ///                This identifier can be either the Classroom-assigned identifier or an
    ///                alias.
    /// * `courseWorkId` - Identifier of the course work.
    /// * `id` - Identifier of the student submission.
    pub fn course_work_student_submissions_reclaim(&self, request: ReclaimStudentSubmissionRequest, course_id: &str, course_work_id: &str, id: &str) -> CourseCourseWorkStudentSubmissionReclaimCall<'a, C, A> {
        CourseCourseWorkStudentSubmissionReclaimCall {
            hub: self.hub,
            _request: request,
            _course_id: course_id.to_string(),
            _course_work_id: course_work_id.to_string(),
            _id: id.to_string(),
            _delegate: Default::default(),
            _scopes: Default::default(),
            _additional_params: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Creates an alias for a course.
    /// 
    /// This method returns the following error codes:
    /// 
    /// * `PERMISSION_DENIED` if the requesting user is not permitted to create the
    /// alias or for access errors.
    /// * `NOT_FOUND` if the course does not exist.
    /// * `ALREADY_EXISTS` if the alias already exists.
    /// * `FAILED_PRECONDITION` if the alias requested does not make sense for the
    ///   requesting user or course (for example, if a user not in a domain
    ///   attempts to access a domain-scoped alias).
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `courseId` - Identifier of the course to alias.
    ///                This identifier can be either the Classroom-assigned identifier or an
    ///                alias.
    pub fn aliases_create(&self, request: CourseAlias, course_id: &str) -> CourseAliaseCreateCall<'a, C, A> {
        CourseAliaseCreateCall {
            hub: self.hub,
            _request: request,
            _course_id: course_id.to_string(),
            _delegate: Default::default(),
            _scopes: Default::default(),
            _additional_params: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Adds a user as a student of a course.
    /// 
    /// This method returns the following error codes:
    /// 
    /// * `PERMISSION_DENIED` if the requesting user is not permitted to create
    /// students in this course or for access errors.
    /// * `NOT_FOUND` if the requested course ID does not exist.
    /// * `FAILED_PRECONDITION` if the requested user's account is disabled,
    /// for the following request errors:
    ///     * CourseMemberLimitReached
    ///     * CourseNotModifiable
    ///     * UserGroupsMembershipLimitReached
    /// * `ALREADY_EXISTS` if the user is already a student or teacher in the
    /// course.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `courseId` - Identifier of the course to create the student in.
    ///                This identifier can be either the Classroom-assigned identifier or an
    ///                alias.
    pub fn students_create(&self, request: Student, course_id: &str) -> CourseStudentCreateCall<'a, C, A> {
        CourseStudentCreateCall {
            hub: self.hub,
            _request: request,
            _course_id: course_id.to_string(),
            _enrollment_code: Default::default(),
            _delegate: Default::default(),
            _scopes: Default::default(),
            _additional_params: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Deletes an alias of a course.
    /// 
    /// This method returns the following error codes:
    /// 
    /// * `PERMISSION_DENIED` if the requesting user is not permitted to remove the
    /// alias or for access errors.
    /// * `NOT_FOUND` if the alias does not exist.
    /// * `FAILED_PRECONDITION` if the alias requested does not make sense for the
    ///   requesting user or course (for example, if a user not in a domain
    ///   attempts to delete a domain-scoped alias).
    /// 
    /// # Arguments
    ///
    /// * `courseId` - Identifier of the course whose alias should be deleted.
    ///                This identifier can be either the Classroom-assigned identifier or an
    ///                alias.
    /// * `alias` - Alias to delete.
    ///             This may not be the Classroom-assigned identifier.
    pub fn aliases_delete(&self, course_id: &str, alias: &str) -> CourseAliaseDeleteCall<'a, C, A> {
        CourseAliaseDeleteCall {
            hub: self.hub,
            _course_id: course_id.to_string(),
            _alias: alias.to_string(),
            _delegate: Default::default(),
            _scopes: Default::default(),
            _additional_params: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Deletes a course work.
    /// 
    /// This request must be made by the Developer Console project of the
    /// [OAuth client ID](https://support.google.com/cloud/answer/6158849) used to
    /// create the corresponding course work item.
    /// 
    /// This method returns the following error codes:
    /// 
    /// * `PERMISSION_DENIED` if the requesting developer project did not create
    /// the corresponding course work, if the requesting user is not permitted
    /// to delete the requested course or for access errors.
    /// * `FAILED_PRECONDITION` if the requested course work has already been
    /// deleted.
    /// * `NOT_FOUND` if no course exists with the requested ID.
    /// 
    /// # Arguments
    ///
    /// * `courseId` - Identifier of the course.
    ///                This identifier can be either the Classroom-assigned identifier or an
    ///                alias.
    /// * `id` - Identifier of the course work to delete.
    ///          This identifier is a Classroom-assigned identifier.
    pub fn course_work_delete(&self, course_id: &str, id: &str) -> CourseCourseWorkDeleteCall<'a, C, A> {
        CourseCourseWorkDeleteCall {
            hub: self.hub,
            _course_id: course_id.to_string(),
            _id: id.to_string(),
            _delegate: Default::default(),
            _scopes: Default::default(),
            _additional_params: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Creates a course.
    /// 
    /// The user specified in `ownerId` is the owner of the created course
    /// and added as a teacher.
    /// 
    /// This method returns the following error codes:
    /// 
    /// * `PERMISSION_DENIED` if the requesting user is not permitted to create
    /// courses or for access errors.
    /// * `NOT_FOUND` if the primary teacher is not a valid user.
    /// * `FAILED_PRECONDITION` if the course owner's account is disabled or for
    /// the following request errors:
    ///     * UserGroupsMembershipLimitReached
    /// * `ALREADY_EXISTS` if an alias was specified in the `id` and
    /// already exists.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    pub fn create(&self, request: Course) -> CourseCreateCall<'a, C, A> {
        CourseCreateCall {
            hub: self.hub,
            _request: request,
            _delegate: Default::default(),
            _scopes: Default::default(),
            _additional_params: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Returns a list of students of this course that the requester
    /// is permitted to view.
    /// 
    /// This method returns the following error codes:
    /// 
    /// * `NOT_FOUND` if the course does not exist.
    /// * `PERMISSION_DENIED` for access errors.
    /// 
    /// # Arguments
    ///
    /// * `courseId` - Identifier of the course.
    ///                This identifier can be either the Classroom-assigned identifier or an
    ///                alias.
    pub fn students_list(&self, course_id: &str) -> CourseStudentListCall<'a, C, A> {
        CourseStudentListCall {
            hub: self.hub,
            _course_id: course_id.to_string(),
            _page_token: Default::default(),
            _page_size: Default::default(),
            _delegate: Default::default(),
            _scopes: Default::default(),
            _additional_params: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Deletes a course.
    /// 
    /// This method returns the following error codes:
    /// 
    /// * `PERMISSION_DENIED` if the requesting user is not permitted to delete the
    /// requested course or for access errors.
    /// * `NOT_FOUND` if no course exists with the requested ID.
    /// 
    /// # Arguments
    ///
    /// * `id` - Identifier of the course to delete.
    ///          This identifier can be either the Classroom-assigned identifier or an
    ///          alias.
    pub fn delete(&self, id: &str) -> CourseDeleteCall<'a, C, A> {
        CourseDeleteCall {
            hub: self.hub,
            _id: id.to_string(),
            _delegate: Default::default(),
            _scopes: Default::default(),
            _additional_params: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Updates one or more fields of a course work.
    /// 
    /// See google.classroom.v1.CourseWork for details
    /// of which fields may be updated and who may change them.
    /// 
    /// This request must be made by the Developer Console project of the
    /// [OAuth client ID](https://support.google.com/cloud/answer/6158849) used to
    /// create the corresponding course work item.
    /// 
    /// This method returns the following error codes:
    /// 
    /// * `PERMISSION_DENIED` if the requesting developer project did not create
    /// the corresponding course work, if the user is not permitted to make the
    /// requested modification to the student submission, or for
    /// access errors.
    /// * `INVALID_ARGUMENT` if the request is malformed.
    /// * `FAILED_PRECONDITION` if the requested course work has already been
    /// deleted.
    /// * `NOT_FOUND` if the requested course, course work, or student submission
    /// does not exist.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `courseId` - Identifier of the course.
    ///                This identifier can be either the Classroom-assigned identifier or an
    ///                alias.
    /// * `id` - Identifier of the course work.
    pub fn course_work_patch(&self, request: CourseWork, course_id: &str, id: &str) -> CourseCourseWorkPatchCall<'a, C, A> {
        CourseCourseWorkPatchCall {
            hub: self.hub,
            _request: request,
            _course_id: course_id.to_string(),
            _id: id.to_string(),
            _update_mask: Default::default(),
            _delegate: Default::default(),
            _scopes: Default::default(),
            _additional_params: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Updates one or more fields in a course.
    /// 
    /// This method returns the following error codes:
    /// 
    /// * `PERMISSION_DENIED` if the requesting user is not permitted to modify the
    /// requested course or for access errors.
    /// * `NOT_FOUND` if no course exists with the requested ID.
    /// * `INVALID_ARGUMENT` if invalid fields are specified in the update mask or
    /// if no update mask is supplied.
    /// * `FAILED_PRECONDITION` for the following request errors:
    ///     * CourseNotModifiable
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `id` - Identifier of the course to update.
    ///          This identifier can be either the Classroom-assigned identifier or an
    ///          alias.
    pub fn patch(&self, request: Course, id: &str) -> CoursePatchCall<'a, C, A> {
        CoursePatchCall {
            hub: self.hub,
            _request: request,
            _id: id.to_string(),
            _update_mask: Default::default(),
            _delegate: Default::default(),
            _scopes: Default::default(),
            _additional_params: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Deletes a student of a course.
    /// 
    /// This method returns the following error codes:
    /// 
    /// * `PERMISSION_DENIED` if the requesting user is not permitted to delete
    /// students of this course or for access errors.
    /// * `NOT_FOUND` if no student of this course has the requested ID or if the
    /// course does not exist.
    /// 
    /// # Arguments
    ///
    /// * `courseId` - Identifier of the course.
    ///                This identifier can be either the Classroom-assigned identifier or an
    ///                alias.
    /// * `userId` - Identifier of the student to delete. The identifier can be one of the
    ///              following:
    ///              * the numeric identifier for the user
    ///              * the email address of the user
    ///              * the string literal `"me"`, indicating the requesting user
    pub fn students_delete(&self, course_id: &str, user_id: &str) -> CourseStudentDeleteCall<'a, C, A> {
        CourseStudentDeleteCall {
            hub: self.hub,
            _course_id: course_id.to_string(),
            _user_id: user_id.to_string(),
            _delegate: Default::default(),
            _scopes: Default::default(),
            _additional_params: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Deletes a teacher of a course.
    /// 
    /// This method returns the following error codes:
    /// 
    /// * `PERMISSION_DENIED` if the requesting user is not permitted to delete
    /// teachers of this course or for access errors.
    /// * `NOT_FOUND` if no teacher of this course has the requested ID or if the
    /// course does not exist.
    /// * `FAILED_PRECONDITION` if the requested ID belongs to the primary teacher
    /// of this course.
    /// 
    /// # Arguments
    ///
    /// * `courseId` - Identifier of the course.
    ///                This identifier can be either the Classroom-assigned identifier or an
    ///                alias.
    /// * `userId` - Identifier of the teacher to delete. The identifier can be one of the
    ///              following:
    ///              * the numeric identifier for the user
    ///              * the email address of the user
    ///              * the string literal `"me"`, indicating the requesting user
    pub fn teachers_delete(&self, course_id: &str, user_id: &str) -> CourseTeacherDeleteCall<'a, C, A> {
        CourseTeacherDeleteCall {
            hub: self.hub,
            _course_id: course_id.to_string(),
            _user_id: user_id.to_string(),
            _delegate: Default::default(),
            _scopes: Default::default(),
            _additional_params: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Creates a teacher of a course.
    /// 
    /// This method returns the following error codes:
    /// 
    /// * `PERMISSION_DENIED` if the requesting user is not  permitted to create
    /// teachers in this course or for access errors.
    /// * `NOT_FOUND` if the requested course ID does not exist.
    /// * `FAILED_PRECONDITION` if the requested user's account is disabled,
    /// for the following request errors:
    ///     * CourseMemberLimitReached
    ///     * CourseNotModifiable
    ///     * CourseTeacherLimitReached
    ///     * UserGroupsMembershipLimitReached
    /// * `ALREADY_EXISTS` if the user is already a teacher or student in the
    /// course.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `courseId` - Identifier of the course.
    ///                This identifier can be either the Classroom-assigned identifier or an
    ///                alias.
    pub fn teachers_create(&self, request: Teacher, course_id: &str) -> CourseTeacherCreateCall<'a, C, A> {
        CourseTeacherCreateCall {
            hub: self.hub,
            _request: request,
            _course_id: course_id.to_string(),
            _delegate: Default::default(),
            _scopes: Default::default(),
            _additional_params: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Returns a student submission.
    /// 
    /// * `PERMISSION_DENIED` if the requesting user is not permitted to access the
    /// requested course, course work, or student submission or for
    /// access errors.
    /// * `INVALID_ARGUMENT` if the request is malformed.
    /// * `NOT_FOUND` if the requested course, course work, or student submission
    /// does not exist.
    /// 
    /// # Arguments
    ///
    /// * `courseId` - Identifier of the course.
    ///                This identifier can be either the Classroom-assigned identifier or an
    ///                alias.
    /// * `courseWorkId` - Identifier of the course work.
    /// * `id` - Identifier of the student submission.
    pub fn course_work_student_submissions_get(&self, course_id: &str, course_work_id: &str, id: &str) -> CourseCourseWorkStudentSubmissionGetCall<'a, C, A> {
        CourseCourseWorkStudentSubmissionGetCall {
            hub: self.hub,
            _course_id: course_id.to_string(),
            _course_work_id: course_work_id.to_string(),
            _id: id.to_string(),
            _delegate: Default::default(),
            _scopes: Default::default(),
            _additional_params: Default::default(),
        }
    }
}



/// A builder providing access to all methods supported on *userProfile* resources.
/// It is not used directly, but through the `Classroom` hub.
///
/// # Example
///
/// Instantiate a resource builder
///
/// ```test_harness,no_run
/// extern crate hyper;
/// extern crate hyper_rustls;
/// extern crate yup_oauth2 as oauth2;
/// extern crate google_classroom1 as classroom1;
/// 
/// # #[test] fn egal() {
/// use std::default::Default;
/// use oauth2::{Authenticator, DefaultAuthenticatorDelegate, ApplicationSecret, MemoryStorage};
/// use classroom1::Classroom;
/// 
/// let secret: ApplicationSecret = Default::default();
/// let auth = Authenticator::new(&secret, DefaultAuthenticatorDelegate,
///                               hyper::Client::with_connector(hyper::net::HttpsConnector::new(hyper_rustls::TlsClient::new())),
///                               <MemoryStorage as Default>::default(), None);
/// let mut hub = Classroom::new(hyper::Client::with_connector(hyper::net::HttpsConnector::new(hyper_rustls::TlsClient::new())), auth);
/// // Usually you wouldn't bind this to a variable, but keep calling *CallBuilders*
/// // like `get(...)`, `guardian_invitations_create(...)`, `guardian_invitations_get(...)`, `guardian_invitations_list(...)`, `guardian_invitations_patch(...)`, `guardians_delete(...)`, `guardians_get(...)` and `guardians_list(...)`
/// // to build up your call.
/// let rb = hub.user_profiles();
/// # }
/// ```
pub struct UserProfileMethods<'a, C, A>
    where C: 'a, A: 'a {

    hub: &'a Classroom<C, A>,
}

impl<'a, C, A> MethodsBuilder for UserProfileMethods<'a, C, A> {}

impl<'a, C, A> UserProfileMethods<'a, C, A> {
    
    /// Create a builder to help you perform the following task:
    ///
    /// Returns a specific guardian.
    /// 
    /// This method returns the following error codes:
    /// 
    /// * `PERMISSION_DENIED` if no user that matches the provided `student_id`
    ///   is visible to the requesting user, if the requesting user is not
    ///   permitted to view guardian information for the student identified by the
    ///   `student_id`, if guardians are not enabled for the domain in question,
    ///   or for other access errors.
    /// * `INVALID_ARGUMENT` if a `student_id` is specified, but its format cannot
    ///   be recognized (it is not an email address, nor a `student_id` from the
    ///   API, nor the literal string `me`).
    /// * `NOT_FOUND` if the requesting user is permitted to view guardians for
    ///   the requested `student_id`, but no `Guardian` record exists for that
    ///   student that matches the provided `guardian_id`.
    /// 
    /// # Arguments
    ///
    /// * `studentId` - The student whose guardian is being requested. One of the following:
    ///                 * the numeric identifier for the user
    ///                 * the email address of the user
    ///                 * the string literal `"me"`, indicating the requesting user
    /// * `guardianId` - The `id` field from a `Guardian`.
    pub fn guardians_get(&self, student_id: &str, guardian_id: &str) -> UserProfileGuardianGetCall<'a, C, A> {
        UserProfileGuardianGetCall {
            hub: self.hub,
            _student_id: student_id.to_string(),
            _guardian_id: guardian_id.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Creates a guardian invitation, and sends an email to the guardian asking
    /// them to confirm that they are the student's guardian.
    /// 
    /// Once the guardian accepts the invitation, their `state` will change to
    /// `COMPLETED` and they will start receiving guardian notifications. A
    /// `Guardian` resource will also be created to represent the active guardian.
    /// 
    /// The request object must have the `student_id` and
    /// `invited_email_address` fields set. Failing to set these fields, or
    /// setting any other fields in the request, will result in an error.
    /// 
    /// This method returns the following error codes:
    /// 
    /// * `PERMISSION_DENIED` if the current user does not have permission to
    ///   manage guardians, if the guardian in question has already rejected
    ///   too many requests for that student, if guardians are not enabled for the
    ///   domain in question, or for other access errors.
    /// * `RESOURCE_EXHAUSTED` if the student or guardian has exceeded the guardian
    ///   link limit.
    /// * `INVALID_ARGUMENT` if the guardian email address is not valid (for
    ///   example, if it is too long), or if the format of the student ID provided
    ///   cannot be recognized (it is not an email address, nor a `user_id` from
    ///   this API). This error will also be returned if read-only fields are set,
    ///   or if the `state` field is set to to a value other than `PENDING`.
    /// * `NOT_FOUND` if the student ID provided is a valid student ID, but
    ///   Classroom has no record of that student.
    /// * `ALREADY_EXISTS` if there is already a pending guardian invitation for
    ///   the student and `invited_email_address` provided, or if the provided
    ///   `invited_email_address` matches the Google account of an existing
    ///   `Guardian` for this user.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `studentId` - ID of the student (in standard format)
    pub fn guardian_invitations_create(&self, request: GuardianInvitation, student_id: &str) -> UserProfileGuardianInvitationCreateCall<'a, C, A> {
        UserProfileGuardianInvitationCreateCall {
            hub: self.hub,
            _request: request,
            _student_id: student_id.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Returns a user profile.
    /// 
    /// This method returns the following error codes:
    /// 
    /// * `PERMISSION_DENIED` if the requesting user is not permitted to access
    /// this user profile, if no profile exists with the requested ID, or for
    /// access errors.
    /// 
    /// # Arguments
    ///
    /// * `userId` - Identifier of the profile to return. The identifier can be one of the
    ///              following:
    ///              * the numeric identifier for the user
    ///              * the email address of the user
    ///              * the string literal `"me"`, indicating the requesting user
    pub fn get(&self, user_id: &str) -> UserProfileGetCall<'a, C, A> {
        UserProfileGetCall {
            hub: self.hub,
            _user_id: user_id.to_string(),
            _delegate: Default::default(),
            _scopes: Default::default(),
            _additional_params: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Modifies a guardian invitation.
    /// 
    /// Currently, the only valid modification is to change the `state` from
    /// `PENDING` to `COMPLETE`. This has the effect of withdrawing the invitation.
    /// 
    /// This method returns the following error codes:
    /// 
    /// * `PERMISSION_DENIED` if the current user does not have permission to
    ///   manage guardians, if guardians are not enabled for the domain in question
    ///   or for other access errors.
    /// * `FAILED_PRECONDITION` if the guardian link is not in the `PENDING` state.
    /// * `INVALID_ARGUMENT` if the format of the student ID provided
    ///   cannot be recognized (it is not an email address, nor a `user_id` from
    ///   this API), or if the passed `GuardianInvitation` has a `state` other than
    ///   `COMPLETE`, or if it modifies fields other than `state`.
    /// * `NOT_FOUND` if the student ID provided is a valid student ID, but
    ///   Classroom has no record of that student, or if the `id` field does not
    ///   refer to a guardian invitation known to Classroom.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `studentId` - The ID of the student whose guardian invitation is to be modified.
    /// * `invitationId` - The `id` field of the `GuardianInvitation` to be modified.
    pub fn guardian_invitations_patch(&self, request: GuardianInvitation, student_id: &str, invitation_id: &str) -> UserProfileGuardianInvitationPatchCall<'a, C, A> {
        UserProfileGuardianInvitationPatchCall {
            hub: self.hub,
            _request: request,
            _student_id: student_id.to_string(),
            _invitation_id: invitation_id.to_string(),
            _update_mask: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Returns a specific guardian invitation.
    /// 
    /// This method returns the following error codes:
    /// 
    /// * `PERMISSION_DENIED` if the requesting user is not permitted to view
    ///   guardian invitations for the student identified by the `student_id`, if
    ///   guardians are not enabled for the domain in question, or for other
    ///   access errors.
    /// * `INVALID_ARGUMENT` if a `student_id` is specified, but its format cannot
    ///   be recognized (it is not an email address, nor a `student_id` from the
    ///   API, nor the literal string `me`).
    /// * `NOT_FOUND` if Classroom cannot find any record of the given student or
    ///   `invitation_id`. May also be returned if the student exists, but the
    ///   requesting user does not have access to see that student.
    /// 
    /// # Arguments
    ///
    /// * `studentId` - The ID of the student whose guardian invitation is being requested.
    /// * `invitationId` - The `id` field of the `GuardianInvitation` being requested.
    pub fn guardian_invitations_get(&self, student_id: &str, invitation_id: &str) -> UserProfileGuardianInvitationGetCall<'a, C, A> {
        UserProfileGuardianInvitationGetCall {
            hub: self.hub,
            _student_id: student_id.to_string(),
            _invitation_id: invitation_id.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Returns a list of guardians that the requesting user is permitted to
    /// view, restricted to those that match the request.
    /// 
    /// To list guardians for any student that the requesting user may view
    /// guardians for, use the literal character `-` for the student ID.
    /// 
    /// This method returns the following error codes:
    /// 
    /// * `PERMISSION_DENIED` if a `student_id` is specified, and the requesting
    ///   user is not permitted to view guardian information for that student, if
    ///   `"-"` is specified as the `student_id` and the user is not a domain
    ///   administrator, if guardians are not enabled for the domain in question,
    ///   if the `invited_email_address` filter is set by a user who is not a
    ///   domain administrator, or for other access errors.
    /// * `INVALID_ARGUMENT` if a `student_id` is specified, but its format cannot
    ///   be recognized (it is not an email address, nor a `student_id` from the
    ///   API, nor the literal string `me`). May also be returned if an invalid
    ///   `page_token` is provided.
    /// * `NOT_FOUND` if a `student_id` is specified, and its format can be
    ///   recognized, but Classroom has no record of that student.
    /// 
    /// # Arguments
    ///
    /// * `studentId` - Filter results by the student who the guardian is linked to.
    ///                 The identifier can be one of the following:
    ///                 * the numeric identifier for the user
    ///                 * the email address of the user
    ///                 * the string literal `"me"`, indicating the requesting user
    ///                 * the string literal `"-"`, indicating that results should be returned for
    ///                   all students that the requesting user has access to view.
    pub fn guardians_list(&self, student_id: &str) -> UserProfileGuardianListCall<'a, C, A> {
        UserProfileGuardianListCall {
            hub: self.hub,
            _student_id: student_id.to_string(),
            _page_token: Default::default(),
            _page_size: Default::default(),
            _invited_email_address: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Returns a list of guardian invitations that the requesting user is
    /// permitted to view, filtered by the parameters provided.
    /// 
    /// This method returns the following error codes:
    /// 
    /// * `PERMISSION_DENIED` if a `student_id` is specified, and the requesting
    ///   user is not permitted to view guardian invitations for that student, if
    ///   `"-"` is specified as the `student_id` and the user is not a domain
    ///   administrator, if guardians are not enabled for the domain in question,
    ///   or for other access errors.
    /// * `INVALID_ARGUMENT` if a `student_id` is specified, but its format cannot
    ///   be recognized (it is not an email address, nor a `student_id` from the
    ///   API, nor the literal string `me`). May also be returned if an invalid
    ///   `page_token` or `state` is provided.
    /// * `NOT_FOUND` if a `student_id` is specified, and its format can be
    ///   recognized, but Classroom has no record of that student.
    /// 
    /// # Arguments
    ///
    /// * `studentId` - The ID of the student whose guardian invitations are to be returned.
    ///                 The identifier can be one of the following:
    ///                 * the numeric identifier for the user
    ///                 * the email address of the user
    ///                 * the string literal `"me"`, indicating the requesting user
    ///                 * the string literal `"-"`, indicating that results should be returned for
    ///                   all students that the requesting user is permitted to view guardian
    ///                   invitations.
    pub fn guardian_invitations_list(&self, student_id: &str) -> UserProfileGuardianInvitationListCall<'a, C, A> {
        UserProfileGuardianInvitationListCall {
            hub: self.hub,
            _student_id: student_id.to_string(),
            _states: Default::default(),
            _page_token: Default::default(),
            _page_size: Default::default(),
            _invited_email_address: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Deletes a guardian.
    /// 
    /// The guardian will no longer receive guardian notifications and the guardian
    /// will no longer be accessible via the API.
    /// 
    /// This method returns the following error codes:
    /// 
    /// * `PERMISSION_DENIED` if no user that matches the provided `student_id`
    ///   is visible to the requesting user, if the requesting user is not
    ///   permitted to manage guardians for the student identified by the
    ///   `student_id`, if guardians are not enabled for the domain in question,
    ///   or for other access errors.
    /// * `INVALID_ARGUMENT` if a `student_id` is specified, but its format cannot
    ///   be recognized (it is not an email address, nor a `student_id` from the
    ///   API).
    /// * `NOT_FOUND` if the requesting user is permitted to modify guardians for
    ///   the requested `student_id`, but no `Guardian` record exists for that
    ///   student with the provided `guardian_id`.
    /// 
    /// # Arguments
    ///
    /// * `studentId` - The student whose guardian is to be deleted. One of the following:
    ///                 * the numeric identifier for the user
    ///                 * the email address of the user
    ///                 * the string literal `"me"`, indicating the requesting user
    /// * `guardianId` - The `id` field from a `Guardian`.
    pub fn guardians_delete(&self, student_id: &str, guardian_id: &str) -> UserProfileGuardianDeleteCall<'a, C, A> {
        UserProfileGuardianDeleteCall {
            hub: self.hub,
            _student_id: student_id.to_string(),
            _guardian_id: guardian_id.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
        }
    }
}



/// A builder providing access to all methods supported on *invitation* resources.
/// It is not used directly, but through the `Classroom` hub.
///
/// # Example
///
/// Instantiate a resource builder
///
/// ```test_harness,no_run
/// extern crate hyper;
/// extern crate hyper_rustls;
/// extern crate yup_oauth2 as oauth2;
/// extern crate google_classroom1 as classroom1;
/// 
/// # #[test] fn egal() {
/// use std::default::Default;
/// use oauth2::{Authenticator, DefaultAuthenticatorDelegate, ApplicationSecret, MemoryStorage};
/// use classroom1::Classroom;
/// 
/// let secret: ApplicationSecret = Default::default();
/// let auth = Authenticator::new(&secret, DefaultAuthenticatorDelegate,
///                               hyper::Client::with_connector(hyper::net::HttpsConnector::new(hyper_rustls::TlsClient::new())),
///                               <MemoryStorage as Default>::default(), None);
/// let mut hub = Classroom::new(hyper::Client::with_connector(hyper::net::HttpsConnector::new(hyper_rustls::TlsClient::new())), auth);
/// // Usually you wouldn't bind this to a variable, but keep calling *CallBuilders*
/// // like `accept(...)`, `create(...)`, `delete(...)`, `get(...)` and `list(...)`
/// // to build up your call.
/// let rb = hub.invitations();
/// # }
/// ```
pub struct InvitationMethods<'a, C, A>
    where C: 'a, A: 'a {

    hub: &'a Classroom<C, A>,
}

impl<'a, C, A> MethodsBuilder for InvitationMethods<'a, C, A> {}

impl<'a, C, A> InvitationMethods<'a, C, A> {
    
    /// Create a builder to help you perform the following task:
    ///
    /// Deletes an invitation.
    /// 
    /// This method returns the following error codes:
    /// 
    /// * `PERMISSION_DENIED` if the requesting user is not permitted to delete the
    /// requested invitation or for access errors.
    /// * `NOT_FOUND` if no invitation exists with the requested ID.
    /// 
    /// # Arguments
    ///
    /// * `id` - Identifier of the invitation to delete.
    pub fn delete(&self, id: &str) -> InvitationDeleteCall<'a, C, A> {
        InvitationDeleteCall {
            hub: self.hub,
            _id: id.to_string(),
            _delegate: Default::default(),
            _scopes: Default::default(),
            _additional_params: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Creates an invitation. Only one invitation for a user and course may exist
    /// at a time. Delete and re-create an invitation to make changes.
    /// 
    /// This method returns the following error codes:
    /// 
    /// * `PERMISSION_DENIED` if the requesting user is not permitted to create
    /// invitations for this course or for access errors.
    /// * `NOT_FOUND` if the course or the user does not exist.
    /// * `FAILED_PRECONDITION` if the requested user's account is disabled or if
    /// the user already has this role or a role with greater permissions.
    /// * `ALREADY_EXISTS` if an invitation for the specified user and course
    /// already exists.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    pub fn create(&self, request: Invitation) -> InvitationCreateCall<'a, C, A> {
        InvitationCreateCall {
            hub: self.hub,
            _request: request,
            _delegate: Default::default(),
            _scopes: Default::default(),
            _additional_params: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Returns a list of invitations that the requesting user is permitted to
    /// view, restricted to those that match the list request.
    /// 
    /// *Note:* At least one of `user_id` or `course_id` must be supplied. Both
    /// fields can be supplied.
    /// 
    /// This method returns the following error codes:
    /// 
    /// * `PERMISSION_DENIED` for access errors.
    pub fn list(&self) -> InvitationListCall<'a, C, A> {
        InvitationListCall {
            hub: self.hub,
            _user_id: Default::default(),
            _page_token: Default::default(),
            _page_size: Default::default(),
            _course_id: Default::default(),
            _delegate: Default::default(),
            _scopes: Default::default(),
            _additional_params: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Returns an invitation.
    /// 
    /// This method returns the following error codes:
    /// 
    /// * `PERMISSION_DENIED` if the requesting user is not permitted to view the
    /// requested invitation or for access errors.
    /// * `NOT_FOUND` if no invitation exists with the requested ID.
    /// 
    /// # Arguments
    ///
    /// * `id` - Identifier of the invitation to return.
    pub fn get(&self, id: &str) -> InvitationGetCall<'a, C, A> {
        InvitationGetCall {
            hub: self.hub,
            _id: id.to_string(),
            _delegate: Default::default(),
            _scopes: Default::default(),
            _additional_params: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Accepts an invitation, removing it and adding the invited user to the
    /// teachers or students (as appropriate) of the specified course. Only the
    /// invited user may accept an invitation.
    /// 
    /// This method returns the following error codes:
    /// 
    /// * `PERMISSION_DENIED` if the requesting user is not permitted to accept the
    /// requested invitation or for access errors.
    /// * `FAILED_PRECONDITION` for the following request errors:
    ///     * CourseMemberLimitReached
    ///     * CourseNotModifiable
    ///     * CourseTeacherLimitReached
    ///     * UserGroupsMembershipLimitReached
    /// * `NOT_FOUND` if no invitation exists with the requested ID.
    /// 
    /// # Arguments
    ///
    /// * `id` - Identifier of the invitation to accept.
    pub fn accept(&self, id: &str) -> InvitationAcceptCall<'a, C, A> {
        InvitationAcceptCall {
            hub: self.hub,
            _id: id.to_string(),
            _delegate: Default::default(),
            _scopes: Default::default(),
            _additional_params: Default::default(),
        }
    }
}





// ###################
// CallBuilders   ###
// #################

/// Updates one or more fields of a student submission.
/// 
/// See google.classroom.v1.StudentSubmission for details
/// of which fields may be updated and who may change them.
/// 
/// This request must be made by the Developer Console project of the
/// [OAuth client ID](https://support.google.com/cloud/answer/6158849) used to
/// create the corresponding course work item.
/// 
/// This method returns the following error codes:
/// 
/// * `PERMISSION_DENIED` if the requesting developer project did not create
/// the corresponding course work, if the user is not permitted to make the
/// requested modification to the student submission, or for
/// access errors.
/// * `INVALID_ARGUMENT` if the request is malformed.
/// * `NOT_FOUND` if the requested course, course work, or student submission
/// does not exist.
///
/// A builder for the *courseWork.studentSubmissions.patch* method supported by a *course* resource.
/// It is not used directly, but through a `CourseMethods` instance.
///
/// # Example
///
/// Instantiate a resource method builder
///
/// ```test_harness,no_run
/// # extern crate hyper;
/// # extern crate hyper_rustls;
/// # extern crate yup_oauth2 as oauth2;
/// # extern crate google_classroom1 as classroom1;
/// use classroom1::StudentSubmission;
/// # #[test] fn egal() {
/// # use std::default::Default;
/// # use oauth2::{Authenticator, DefaultAuthenticatorDelegate, ApplicationSecret, MemoryStorage};
/// # use classroom1::Classroom;
/// 
/// # let secret: ApplicationSecret = Default::default();
/// # let auth = Authenticator::new(&secret, DefaultAuthenticatorDelegate,
/// #                               hyper::Client::with_connector(hyper::net::HttpsConnector::new(hyper_rustls::TlsClient::new())),
/// #                               <MemoryStorage as Default>::default(), None);
/// # let mut hub = Classroom::new(hyper::Client::with_connector(hyper::net::HttpsConnector::new(hyper_rustls::TlsClient::new())), auth);
/// // As the method needs a request, you would usually fill it with the desired information
/// // into the respective structure. Some of the parts shown here might not be applicable !
/// // Values shown here are possibly random and not representative !
/// let mut req = StudentSubmission::default();
/// 
/// // You can configure optional parameters by calling the respective setters at will, and
/// // execute the final call using `doit()`.
/// // Values shown here are possibly random and not representative !
/// let result = hub.courses().course_work_student_submissions_patch(req, "courseId", "courseWorkId", "id")
///              .update_mask("et")
///              .doit();
/// # }
/// ```
pub struct CourseCourseWorkStudentSubmissionPatchCall<'a, C, A>
    where C: 'a, A: 'a {

    hub: &'a Classroom<C, A>,
    _request: StudentSubmission,
    _course_id: String,
    _course_work_id: String,
    _id: String,
    _update_mask: Option<String>,
    _delegate: Option<&'a mut Delegate>,
    _additional_params: HashMap<String, String>,
    _scopes: BTreeMap<String, ()>
}

impl<'a, C, A> CallBuilder for CourseCourseWorkStudentSubmissionPatchCall<'a, C, A> {}

impl<'a, C, A> CourseCourseWorkStudentSubmissionPatchCall<'a, C, A> where C: BorrowMut<hyper::Client>, A: oauth2::GetToken {


    /// Perform the operation you have build so far.
    pub fn doit(mut self) -> Result<(hyper::client::Response, StudentSubmission)> {
        use std::io::{Read, Seek};
        use hyper::header::{ContentType, ContentLength, Authorization, Bearer, UserAgent, Location};
        let mut dd = DefaultDelegate;
        let mut dlg: &mut Delegate = match self._delegate {
            Some(d) => d,
            None => &mut dd
        };
        dlg.begin(MethodInfo { id: "classroom.courses.courseWork.studentSubmissions.patch",
                               http_method: hyper::method::Method::Patch });
        let mut params: Vec<(&str, String)> = Vec::with_capacity((7 + self._additional_params.len()));
        params.push(("courseId", self._course_id.to_string()));
        params.push(("courseWorkId", self._course_work_id.to_string()));
        params.push(("id", self._id.to_string()));
        if let Some(value) = self._update_mask {
            params.push(("updateMask", value.to_string()));
        }
        for &field in ["alt", "courseId", "courseWorkId", "id", "updateMask"].iter() {
            if self._additional_params.contains_key(field) {
                dlg.finished(false);
                return Err(Error::FieldClash(field));
            }
        }
        for (name, value) in self._additional_params.iter() {
            params.push((&name, value.clone()));
        }

        params.push(("alt", "json".to_string()));

        let mut url = self.hub._base_url.clone() + "v1/courses/{courseId}/courseWork/{courseWorkId}/studentSubmissions/{id}";
        if self._scopes.len() == 0 {
            self._scopes.insert(Scope::CourseworkMe.as_ref().to_string(), ());
        }

        for &(find_this, param_name) in [("{courseId}", "courseId"), ("{courseWorkId}", "courseWorkId"), ("{id}", "id")].iter() {
            let mut replace_with: Option<&str> = None;
            for &(name, ref value) in params.iter() {
                if name == param_name {
                    replace_with = Some(value);
                    break;
                }
            }
            url = url.replace(find_this, replace_with.expect("to find substitution value in params"));
        }
        {
            let mut indices_for_removal: Vec<usize> = Vec::with_capacity(3);
            for param_name in ["id", "courseWorkId", "courseId"].iter() {
                if let Some(index) = params.iter().position(|t| &t.0 == param_name) {
                    indices_for_removal.push(index);
                }
            }
            for &index in indices_for_removal.iter() {
                params.remove(index);
            }
        }

        if params.len() > 0 {
            url.push('?');
            url.push_str(&url::form_urlencoded::serialize(params));
        }

        let mut json_mime_type = mime::Mime(mime::TopLevel::Application, mime::SubLevel::Json, Default::default());
        let mut request_value_reader =
            {
                let mut value = json::value::to_value(&self._request).expect("serde to work");
                remove_json_null_values(&mut value);
                let mut dst = io::Cursor::new(Vec::with_capacity(128));
                json::to_writer(&mut dst, &value).unwrap();
                dst
            };
        let request_size = request_value_reader.seek(io::SeekFrom::End(0)).unwrap();
        request_value_reader.seek(io::SeekFrom::Start(0)).unwrap();


        loop {
            let token = match self.hub.auth.borrow_mut().token(self._scopes.keys()) {
                Ok(token) => token,
                Err(err) => {
                    match  dlg.token(&*err) {
                        Some(token) => token,
                        None => {
                            dlg.finished(false);
                            return Err(Error::MissingToken(err))
                        }
                    }
                }
            };
            let auth_header = Authorization(Bearer { token: token.access_token });
            request_value_reader.seek(io::SeekFrom::Start(0)).unwrap();
            let mut req_result = {
                let mut client = &mut *self.hub.client.borrow_mut();
                let mut req = client.borrow_mut().request(hyper::method::Method::Patch, &url)
                    .header(UserAgent(self.hub._user_agent.clone()))
                    .header(auth_header.clone())
                    .header(ContentType(json_mime_type.clone()))
                    .header(ContentLength(request_size as u64))
                    .body(&mut request_value_reader);

                dlg.pre_request();
                req.send()
            };

            match req_result {
                Err(err) => {
                    if let oauth2::Retry::After(d) = dlg.http_error(&err) {
                        sleep(d);
                        continue;
                    }
                    dlg.finished(false);
                    return Err(Error::HttpError(err))
                }
                Ok(mut res) => {
                    if !res.status.is_success() {
                        let mut json_err = String::new();
                        res.read_to_string(&mut json_err).unwrap();
                        if let oauth2::Retry::After(d) = dlg.http_failure(&res,
                                                              json::from_str(&json_err).ok(),
                                                              json::from_str(&json_err).ok()) {
                            sleep(d);
                            continue;
                        }
                        dlg.finished(false);
                        return match json::from_str::<ErrorResponse>(&json_err){
                            Err(_) => Err(Error::Failure(res)),
                            Ok(serr) => Err(Error::BadRequest(serr))
                        }
                    }
                    let result_value = {
                        let mut json_response = String::new();
                        res.read_to_string(&mut json_response).unwrap();
                        match json::from_str(&json_response) {
                            Ok(decoded) => (res, decoded),
                            Err(err) => {
                                dlg.response_json_decode_error(&json_response, &err);
                                return Err(Error::JsonDecodeError(json_response, err));
                            }
                        }
                    };

                    dlg.finished(true);
                    return Ok(result_value)
                }
            }
        }
    }


    ///
    /// Sets the *request* property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call,
    /// we provide this method for API completeness.
    pub fn request(mut self, new_value: StudentSubmission) -> CourseCourseWorkStudentSubmissionPatchCall<'a, C, A> {
        self._request = new_value;
        self
    }
    /// Identifier of the course.
    /// This identifier can be either the Classroom-assigned identifier or an
    /// alias.
    ///
    /// Sets the *course id* path property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call,
    /// we provide this method for API completeness.
    pub fn course_id(mut self, new_value: &str) -> CourseCourseWorkStudentSubmissionPatchCall<'a, C, A> {
        self._course_id = new_value.to_string();
        self
    }
    /// Identifier of the course work.
    ///
    /// Sets the *course work id* path property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call,
    /// we provide this method for API completeness.
    pub fn course_work_id(mut self, new_value: &str) -> CourseCourseWorkStudentSubmissionPatchCall<'a, C, A> {
        self._course_work_id = new_value.to_string();
        self
    }
    /// Identifier of the student submission.
    ///
    /// Sets the *id* path property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call,
    /// we provide this method for API completeness.
    pub fn id(mut self, new_value: &str) -> CourseCourseWorkStudentSubmissionPatchCall<'a, C, A> {
        self._id = new_value.to_string();
        self
    }
    /// Mask that identifies which fields on the student submission to update.
    /// This field is required to do an update. The update fails if invalid
    /// fields are specified.
    /// 
    /// The following fields may be specified by teachers:
    /// * `draft_grade`
    /// * `assigned_grade`
    ///
    /// Sets the *update mask* query property to the given value.
    pub fn update_mask(mut self, new_value: &str) -> CourseCourseWorkStudentSubmissionPatchCall<'a, C, A> {
        self._update_mask = Some(new_value.to_string());
        self
    }
    /// The delegate implementation is consulted whenever there is an intermediate result, or if something goes wrong
    /// while executing the actual API request.
    /// 
    /// It should be used to handle progress information, and to implement a certain level of resilience.
    ///
    /// Sets the *delegate* property to the given value.
    pub fn delegate(mut self, new_value: &'a mut Delegate) -> CourseCourseWorkStudentSubmissionPatchCall<'a, C, A> {
        self._delegate = Some(new_value);
        self
    }

    /// Set any additional parameter of the query string used in the request.
    /// It should be used to set parameters which are not yet available through their own
    /// setters.
    ///
    /// Please note that this method must not be used to set any of the known paramters
    /// which have their own setter method. If done anyway, the request will fail.
    ///
    /// # Additional Parameters
    ///
    /// * *upload_protocol* (query-string) - Upload protocol for media (e.g. "raw", "multipart").
    /// * *pp* (query-boolean) - Pretty-print response.
    /// * *prettyPrint* (query-boolean) - Returns response with indentations and line breaks.
    /// * *bearer_token* (query-string) - OAuth bearer token.
    /// * *access_token* (query-string) - OAuth access token.
    /// * *uploadType* (query-string) - Legacy upload protocol for media (e.g. "media", "multipart").
    /// * *quotaUser* (query-string) - Available to use for quota purposes for server-side applications. Can be any arbitrary string assigned to a user, but should not exceed 40 characters.
    /// * *callback* (query-string) - JSONP
    /// * *oauth_token* (query-string) - OAuth 2.0 token for the current user.
    /// * *key* (query-string) - API key. Your API key identifies your project and provides you with API access, quota, and reports. Required unless you provide an OAuth 2.0 token.
    /// * *fields* (query-string) - Selector specifying which fields to include in a partial response.
    /// * *alt* (query-string) - Data format for response.
    /// * *$.xgafv* (query-string) - V1 error format.
    pub fn param<T>(mut self, name: T, value: T) -> CourseCourseWorkStudentSubmissionPatchCall<'a, C, A>
                                                        where T: AsRef<str> {
        self._additional_params.insert(name.as_ref().to_string(), value.as_ref().to_string());
        self
    }

    /// Identifies the authorization scope for the method you are building.
    ///
    /// Use this method to actively specify which scope should be used, instead the default `Scope` variant
    /// `Scope::CourseworkMe`.
    ///
    /// The `scope` will be added to a set of scopes. This is important as one can maintain access
    /// tokens for more than one scope.
    /// If `None` is specified, then all scopes will be removed and no default scope will be used either.
    /// In that case, you have to specify your API-key using the `key` parameter (see the `param()`
    /// function for details).
    ///
    /// Usually there is more than one suitable scope to authorize an operation, some of which may
    /// encompass more rights than others. For example, for listing resources, a *read-only* scope will be
    /// sufficient, a read-write scope will do as well.
    pub fn add_scope<T, S>(mut self, scope: T) -> CourseCourseWorkStudentSubmissionPatchCall<'a, C, A>
                                                        where T: Into<Option<S>>,
                                                              S: AsRef<str> {
        match scope.into() {
          Some(scope) => self._scopes.insert(scope.as_ref().to_string(), ()),
          None => None,
        };
        self
    }
}


/// Returns a list of student submissions that the requester is permitted to
/// view, factoring in the OAuth scopes of the request.
/// `-` may be specified as the `course_work_id` to include student
/// submissions for multiple course work items.
/// 
/// Course students may only view their own work. Course teachers
/// and domain administrators may view all student submissions.
/// 
/// This method returns the following error codes:
/// 
/// * `PERMISSION_DENIED` if the requesting user is not permitted to access the
/// requested course or course work, or for access errors.
/// * `INVALID_ARGUMENT` if the request is malformed.
/// * `NOT_FOUND` if the requested course does not exist.
///
/// A builder for the *courseWork.studentSubmissions.list* method supported by a *course* resource.
/// It is not used directly, but through a `CourseMethods` instance.
///
/// # Example
///
/// Instantiate a resource method builder
///
/// ```test_harness,no_run
/// # extern crate hyper;
/// # extern crate hyper_rustls;
/// # extern crate yup_oauth2 as oauth2;
/// # extern crate google_classroom1 as classroom1;
/// # #[test] fn egal() {
/// # use std::default::Default;
/// # use oauth2::{Authenticator, DefaultAuthenticatorDelegate, ApplicationSecret, MemoryStorage};
/// # use classroom1::Classroom;
/// 
/// # let secret: ApplicationSecret = Default::default();
/// # let auth = Authenticator::new(&secret, DefaultAuthenticatorDelegate,
/// #                               hyper::Client::with_connector(hyper::net::HttpsConnector::new(hyper_rustls::TlsClient::new())),
/// #                               <MemoryStorage as Default>::default(), None);
/// # let mut hub = Classroom::new(hyper::Client::with_connector(hyper::net::HttpsConnector::new(hyper_rustls::TlsClient::new())), auth);
/// // You can configure optional parameters by calling the respective setters at will, and
/// // execute the final call using `doit()`.
/// // Values shown here are possibly random and not representative !
/// let result = hub.courses().course_work_student_submissions_list("courseId", "courseWorkId")
///              .user_id("Lorem")
///              .add_states("et")
///              .page_token("duo")
///              .page_size(-32)
///              .late("sea")
///              .doit();
/// # }
/// ```
pub struct CourseCourseWorkStudentSubmissionListCall<'a, C, A>
    where C: 'a, A: 'a {

    hub: &'a Classroom<C, A>,
    _course_id: String,
    _course_work_id: String,
    _user_id: Option<String>,
    _states: Vec<String>,
    _page_token: Option<String>,
    _page_size: Option<i32>,
    _late: Option<String>,
    _delegate: Option<&'a mut Delegate>,
    _additional_params: HashMap<String, String>,
    _scopes: BTreeMap<String, ()>
}

impl<'a, C, A> CallBuilder for CourseCourseWorkStudentSubmissionListCall<'a, C, A> {}

impl<'a, C, A> CourseCourseWorkStudentSubmissionListCall<'a, C, A> where C: BorrowMut<hyper::Client>, A: oauth2::GetToken {


    /// Perform the operation you have build so far.
    pub fn doit(mut self) -> Result<(hyper::client::Response, ListStudentSubmissionsResponse)> {
        use std::io::{Read, Seek};
        use hyper::header::{ContentType, ContentLength, Authorization, Bearer, UserAgent, Location};
        let mut dd = DefaultDelegate;
        let mut dlg: &mut Delegate = match self._delegate {
            Some(d) => d,
            None => &mut dd
        };
        dlg.begin(MethodInfo { id: "classroom.courses.courseWork.studentSubmissions.list",
                               http_method: hyper::method::Method::Get });
        let mut params: Vec<(&str, String)> = Vec::with_capacity((9 + self._additional_params.len()));
        params.push(("courseId", self._course_id.to_string()));
        params.push(("courseWorkId", self._course_work_id.to_string()));
        if let Some(value) = self._user_id {
            params.push(("userId", value.to_string()));
        }
        if self._states.len() > 0 {
            for f in self._states.iter() {
                params.push(("states", f.to_string()));
            }
        }
        if let Some(value) = self._page_token {
            params.push(("pageToken", value.to_string()));
        }
        if let Some(value) = self._page_size {
            params.push(("pageSize", value.to_string()));
        }
        if let Some(value) = self._late {
            params.push(("late", value.to_string()));
        }
        for &field in ["alt", "courseId", "courseWorkId", "userId", "states", "pageToken", "pageSize", "late"].iter() {
            if self._additional_params.contains_key(field) {
                dlg.finished(false);
                return Err(Error::FieldClash(field));
            }
        }
        for (name, value) in self._additional_params.iter() {
            params.push((&name, value.clone()));
        }

        params.push(("alt", "json".to_string()));

        let mut url = self.hub._base_url.clone() + "v1/courses/{courseId}/courseWork/{courseWorkId}/studentSubmissions";
        if self._scopes.len() == 0 {
            self._scopes.insert(Scope::CourseworkMeReadonly.as_ref().to_string(), ());
        }

        for &(find_this, param_name) in [("{courseId}", "courseId"), ("{courseWorkId}", "courseWorkId")].iter() {
            let mut replace_with: Option<&str> = None;
            for &(name, ref value) in params.iter() {
                if name == param_name {
                    replace_with = Some(value);
                    break;
                }
            }
            url = url.replace(find_this, replace_with.expect("to find substitution value in params"));
        }
        {
            let mut indices_for_removal: Vec<usize> = Vec::with_capacity(2);
            for param_name in ["courseWorkId", "courseId"].iter() {
                if let Some(index) = params.iter().position(|t| &t.0 == param_name) {
                    indices_for_removal.push(index);
                }
            }
            for &index in indices_for_removal.iter() {
                params.remove(index);
            }
        }

        if params.len() > 0 {
            url.push('?');
            url.push_str(&url::form_urlencoded::serialize(params));
        }



        loop {
            let token = match self.hub.auth.borrow_mut().token(self._scopes.keys()) {
                Ok(token) => token,
                Err(err) => {
                    match  dlg.token(&*err) {
                        Some(token) => token,
                        None => {
                            dlg.finished(false);
                            return Err(Error::MissingToken(err))
                        }
                    }
                }
            };
            let auth_header = Authorization(Bearer { token: token.access_token });
            let mut req_result = {
                let mut client = &mut *self.hub.client.borrow_mut();
                let mut req = client.borrow_mut().request(hyper::method::Method::Get, &url)
                    .header(UserAgent(self.hub._user_agent.clone()))
                    .header(auth_header.clone());

                dlg.pre_request();
                req.send()
            };

            match req_result {
                Err(err) => {
                    if let oauth2::Retry::After(d) = dlg.http_error(&err) {
                        sleep(d);
                        continue;
                    }
                    dlg.finished(false);
                    return Err(Error::HttpError(err))
                }
                Ok(mut res) => {
                    if !res.status.is_success() {
                        let mut json_err = String::new();
                        res.read_to_string(&mut json_err).unwrap();
                        if let oauth2::Retry::After(d) = dlg.http_failure(&res,
                                                              json::from_str(&json_err).ok(),
                                                              json::from_str(&json_err).ok()) {
                            sleep(d);
                            continue;
                        }
                        dlg.finished(false);
                        return match json::from_str::<ErrorResponse>(&json_err){
                            Err(_) => Err(Error::Failure(res)),
                            Ok(serr) => Err(Error::BadRequest(serr))
                        }
                    }
                    let result_value = {
                        let mut json_response = String::new();
                        res.read_to_string(&mut json_response).unwrap();
                        match json::from_str(&json_response) {
                            Ok(decoded) => (res, decoded),
                            Err(err) => {
                                dlg.response_json_decode_error(&json_response, &err);
                                return Err(Error::JsonDecodeError(json_response, err));
                            }
                        }
                    };

                    dlg.finished(true);
                    return Ok(result_value)
                }
            }
        }
    }


    /// Identifier of the course.
    /// This identifier can be either the Classroom-assigned identifier or an
    /// alias.
    ///
    /// Sets the *course id* path property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call,
    /// we provide this method for API completeness.
    pub fn course_id(mut self, new_value: &str) -> CourseCourseWorkStudentSubmissionListCall<'a, C, A> {
        self._course_id = new_value.to_string();
        self
    }
    /// Identifer of the student work to request.
    /// This may be set to the string literal `"-"` to request student work for
    /// all course work in the specified course.
    ///
    /// Sets the *course work id* path property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call,
    /// we provide this method for API completeness.
    pub fn course_work_id(mut self, new_value: &str) -> CourseCourseWorkStudentSubmissionListCall<'a, C, A> {
        self._course_work_id = new_value.to_string();
        self
    }
    /// Optional argument to restrict returned student work to those owned by the
    /// student with the specified identifier. The identifier can be one of the
    /// following:
    /// 
    /// * the numeric identifier for the user
    /// * the email address of the user
    /// * the string literal `"me"`, indicating the requesting user
    ///
    /// Sets the *user id* query property to the given value.
    pub fn user_id(mut self, new_value: &str) -> CourseCourseWorkStudentSubmissionListCall<'a, C, A> {
        self._user_id = Some(new_value.to_string());
        self
    }
    /// Requested submission states. If specified, returned student submissions
    /// match one of the specified submission states.
    ///
    /// Append the given value to the *states* query property.
    /// Each appended value will retain its original ordering and be '/'-separated in the URL's parameters.
    pub fn add_states(mut self, new_value: &str) -> CourseCourseWorkStudentSubmissionListCall<'a, C, A> {
        self._states.push(new_value.to_string());
        self
    }
    /// nextPageToken
    /// value returned from a previous
    /// list call,
    /// indicating that the subsequent page of results should be returned.
    /// 
    /// The list request
    /// must be otherwise identical to the one that resulted in this token.
    ///
    /// Sets the *page token* query property to the given value.
    pub fn page_token(mut self, new_value: &str) -> CourseCourseWorkStudentSubmissionListCall<'a, C, A> {
        self._page_token = Some(new_value.to_string());
        self
    }
    /// Maximum number of items to return. Zero or unspecified indicates that the
    /// server may assign a maximum.
    /// 
    /// The server may return fewer than the specified number of results.
    ///
    /// Sets the *page size* query property to the given value.
    pub fn page_size(mut self, new_value: i32) -> CourseCourseWorkStudentSubmissionListCall<'a, C, A> {
        self._page_size = Some(new_value);
        self
    }
    /// Requested lateness value. If specified, returned student submissions are
    /// restricted by the requested value.
    /// If unspecified, submissions are returned regardless of `late` value.
    ///
    /// Sets the *late* query property to the given value.
    pub fn late(mut self, new_value: &str) -> CourseCourseWorkStudentSubmissionListCall<'a, C, A> {
        self._late = Some(new_value.to_string());
        self
    }
    /// The delegate implementation is consulted whenever there is an intermediate result, or if something goes wrong
    /// while executing the actual API request.
    /// 
    /// It should be used to handle progress information, and to implement a certain level of resilience.
    ///
    /// Sets the *delegate* property to the given value.
    pub fn delegate(mut self, new_value: &'a mut Delegate) -> CourseCourseWorkStudentSubmissionListCall<'a, C, A> {
        self._delegate = Some(new_value);
        self
    }

    /// Set any additional parameter of the query string used in the request.
    /// It should be used to set parameters which are not yet available through their own
    /// setters.
    ///
    /// Please note that this method must not be used to set any of the known paramters
    /// which have their own setter method. If done anyway, the request will fail.
    ///
    /// # Additional Parameters
    ///
    /// * *upload_protocol* (query-string) - Upload protocol for media (e.g. "raw", "multipart").
    /// * *pp* (query-boolean) - Pretty-print response.
    /// * *prettyPrint* (query-boolean) - Returns response with indentations and line breaks.
    /// * *bearer_token* (query-string) - OAuth bearer token.
    /// * *access_token* (query-string) - OAuth access token.
    /// * *uploadType* (query-string) - Legacy upload protocol for media (e.g. "media", "multipart").
    /// * *quotaUser* (query-string) - Available to use for quota purposes for server-side applications. Can be any arbitrary string assigned to a user, but should not exceed 40 characters.
    /// * *callback* (query-string) - JSONP
    /// * *oauth_token* (query-string) - OAuth 2.0 token for the current user.
    /// * *key* (query-string) - API key. Your API key identifies your project and provides you with API access, quota, and reports. Required unless you provide an OAuth 2.0 token.
    /// * *fields* (query-string) - Selector specifying which fields to include in a partial response.
    /// * *alt* (query-string) - Data format for response.
    /// * *$.xgafv* (query-string) - V1 error format.
    pub fn param<T>(mut self, name: T, value: T) -> CourseCourseWorkStudentSubmissionListCall<'a, C, A>
                                                        where T: AsRef<str> {
        self._additional_params.insert(name.as_ref().to_string(), value.as_ref().to_string());
        self
    }

    /// Identifies the authorization scope for the method you are building.
    ///
    /// Use this method to actively specify which scope should be used, instead the default `Scope` variant
    /// `Scope::CourseworkMeReadonly`.
    ///
    /// The `scope` will be added to a set of scopes. This is important as one can maintain access
    /// tokens for more than one scope.
    /// If `None` is specified, then all scopes will be removed and no default scope will be used either.
    /// In that case, you have to specify your API-key using the `key` parameter (see the `param()`
    /// function for details).
    ///
    /// Usually there is more than one suitable scope to authorize an operation, some of which may
    /// encompass more rights than others. For example, for listing resources, a *read-only* scope will be
    /// sufficient, a read-write scope will do as well.
    pub fn add_scope<T, S>(mut self, scope: T) -> CourseCourseWorkStudentSubmissionListCall<'a, C, A>
                                                        where T: Into<Option<S>>,
                                                              S: AsRef<str> {
        match scope.into() {
          Some(scope) => self._scopes.insert(scope.as_ref().to_string(), ()),
          None => None,
        };
        self
    }
}


/// Returns a course.
/// 
/// This method returns the following error codes:
/// 
/// * `PERMISSION_DENIED` if the requesting user is not permitted to access the
/// requested course or for access errors.
/// * `NOT_FOUND` if no course exists with the requested ID.
///
/// A builder for the *get* method supported by a *course* resource.
/// It is not used directly, but through a `CourseMethods` instance.
///
/// # Example
///
/// Instantiate a resource method builder
///
/// ```test_harness,no_run
/// # extern crate hyper;
/// # extern crate hyper_rustls;
/// # extern crate yup_oauth2 as oauth2;
/// # extern crate google_classroom1 as classroom1;
/// # #[test] fn egal() {
/// # use std::default::Default;
/// # use oauth2::{Authenticator, DefaultAuthenticatorDelegate, ApplicationSecret, MemoryStorage};
/// # use classroom1::Classroom;
/// 
/// # let secret: ApplicationSecret = Default::default();
/// # let auth = Authenticator::new(&secret, DefaultAuthenticatorDelegate,
/// #                               hyper::Client::with_connector(hyper::net::HttpsConnector::new(hyper_rustls::TlsClient::new())),
/// #                               <MemoryStorage as Default>::default(), None);
/// # let mut hub = Classroom::new(hyper::Client::with_connector(hyper::net::HttpsConnector::new(hyper_rustls::TlsClient::new())), auth);
/// // You can configure optional parameters by calling the respective setters at will, and
/// // execute the final call using `doit()`.
/// // Values shown here are possibly random and not representative !
/// let result = hub.courses().get("id")
///              .doit();
/// # }
/// ```
pub struct CourseGetCall<'a, C, A>
    where C: 'a, A: 'a {

    hub: &'a Classroom<C, A>,
    _id: String,
    _delegate: Option<&'a mut Delegate>,
    _additional_params: HashMap<String, String>,
    _scopes: BTreeMap<String, ()>
}

impl<'a, C, A> CallBuilder for CourseGetCall<'a, C, A> {}

impl<'a, C, A> CourseGetCall<'a, C, A> where C: BorrowMut<hyper::Client>, A: oauth2::GetToken {


    /// Perform the operation you have build so far.
    pub fn doit(mut self) -> Result<(hyper::client::Response, Course)> {
        use std::io::{Read, Seek};
        use hyper::header::{ContentType, ContentLength, Authorization, Bearer, UserAgent, Location};
        let mut dd = DefaultDelegate;
        let mut dlg: &mut Delegate = match self._delegate {
            Some(d) => d,
            None => &mut dd
        };
        dlg.begin(MethodInfo { id: "classroom.courses.get",
                               http_method: hyper::method::Method::Get });
        let mut params: Vec<(&str, String)> = Vec::with_capacity((3 + self._additional_params.len()));
        params.push(("id", self._id.to_string()));
        for &field in ["alt", "id"].iter() {
            if self._additional_params.contains_key(field) {
                dlg.finished(false);
                return Err(Error::FieldClash(field));
            }
        }
        for (name, value) in self._additional_params.iter() {
            params.push((&name, value.clone()));
        }

        params.push(("alt", "json".to_string()));

        let mut url = self.hub._base_url.clone() + "v1/courses/{id}";
        if self._scopes.len() == 0 {
            self._scopes.insert(Scope::CourseReadonly.as_ref().to_string(), ());
        }

        for &(find_this, param_name) in [("{id}", "id")].iter() {
            let mut replace_with: Option<&str> = None;
            for &(name, ref value) in params.iter() {
                if name == param_name {
                    replace_with = Some(value);
                    break;
                }
            }
            url = url.replace(find_this, replace_with.expect("to find substitution value in params"));
        }
        {
            let mut indices_for_removal: Vec<usize> = Vec::with_capacity(1);
            for param_name in ["id"].iter() {
                if let Some(index) = params.iter().position(|t| &t.0 == param_name) {
                    indices_for_removal.push(index);
                }
            }
            for &index in indices_for_removal.iter() {
                params.remove(index);
            }
        }

        if params.len() > 0 {
            url.push('?');
            url.push_str(&url::form_urlencoded::serialize(params));
        }



        loop {
            let token = match self.hub.auth.borrow_mut().token(self._scopes.keys()) {
                Ok(token) => token,
                Err(err) => {
                    match  dlg.token(&*err) {
                        Some(token) => token,
                        None => {
                            dlg.finished(false);
                            return Err(Error::MissingToken(err))
                        }
                    }
                }
            };
            let auth_header = Authorization(Bearer { token: token.access_token });
            let mut req_result = {
                let mut client = &mut *self.hub.client.borrow_mut();
                let mut req = client.borrow_mut().request(hyper::method::Method::Get, &url)
                    .header(UserAgent(self.hub._user_agent.clone()))
                    .header(auth_header.clone());

                dlg.pre_request();
                req.send()
            };

            match req_result {
                Err(err) => {
                    if let oauth2::Retry::After(d) = dlg.http_error(&err) {
                        sleep(d);
                        continue;
                    }
                    dlg.finished(false);
                    return Err(Error::HttpError(err))
                }
                Ok(mut res) => {
                    if !res.status.is_success() {
                        let mut json_err = String::new();
                        res.read_to_string(&mut json_err).unwrap();
                        if let oauth2::Retry::After(d) = dlg.http_failure(&res,
                                                              json::from_str(&json_err).ok(),
                                                              json::from_str(&json_err).ok()) {
                            sleep(d);
                            continue;
                        }
                        dlg.finished(false);
                        return match json::from_str::<ErrorResponse>(&json_err){
                            Err(_) => Err(Error::Failure(res)),
                            Ok(serr) => Err(Error::BadRequest(serr))
                        }
                    }
                    let result_value = {
                        let mut json_response = String::new();
                        res.read_to_string(&mut json_response).unwrap();
                        match json::from_str(&json_response) {
                            Ok(decoded) => (res, decoded),
                            Err(err) => {
                                dlg.response_json_decode_error(&json_response, &err);
                                return Err(Error::JsonDecodeError(json_response, err));
                            }
                        }
                    };

                    dlg.finished(true);
                    return Ok(result_value)
                }
            }
        }
    }


    /// Identifier of the course to return.
    /// This identifier can be either the Classroom-assigned identifier or an
    /// alias.
    ///
    /// Sets the *id* path property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call,
    /// we provide this method for API completeness.
    pub fn id(mut self, new_value: &str) -> CourseGetCall<'a, C, A> {
        self._id = new_value.to_string();
        self
    }
    /// The delegate implementation is consulted whenever there is an intermediate result, or if something goes wrong
    /// while executing the actual API request.
    /// 
    /// It should be used to handle progress information, and to implement a certain level of resilience.
    ///
    /// Sets the *delegate* property to the given value.
    pub fn delegate(mut self, new_value: &'a mut Delegate) -> CourseGetCall<'a, C, A> {
        self._delegate = Some(new_value);
        self
    }

    /// Set any additional parameter of the query string used in the request.
    /// It should be used to set parameters which are not yet available through their own
    /// setters.
    ///
    /// Please note that this method must not be used to set any of the known paramters
    /// which have their own setter method. If done anyway, the request will fail.
    ///
    /// # Additional Parameters
    ///
    /// * *upload_protocol* (query-string) - Upload protocol for media (e.g. "raw", "multipart").
    /// * *pp* (query-boolean) - Pretty-print response.
    /// * *prettyPrint* (query-boolean) - Returns response with indentations and line breaks.
    /// * *bearer_token* (query-string) - OAuth bearer token.
    /// * *access_token* (query-string) - OAuth access token.
    /// * *uploadType* (query-string) - Legacy upload protocol for media (e.g. "media", "multipart").
    /// * *quotaUser* (query-string) - Available to use for quota purposes for server-side applications. Can be any arbitrary string assigned to a user, but should not exceed 40 characters.
    /// * *callback* (query-string) - JSONP
    /// * *oauth_token* (query-string) - OAuth 2.0 token for the current user.
    /// * *key* (query-string) - API key. Your API key identifies your project and provides you with API access, quota, and reports. Required unless you provide an OAuth 2.0 token.
    /// * *fields* (query-string) - Selector specifying which fields to include in a partial response.
    /// * *alt* (query-string) - Data format for response.
    /// * *$.xgafv* (query-string) - V1 error format.
    pub fn param<T>(mut self, name: T, value: T) -> CourseGetCall<'a, C, A>
                                                        where T: AsRef<str> {
        self._additional_params.insert(name.as_ref().to_string(), value.as_ref().to_string());
        self
    }

    /// Identifies the authorization scope for the method you are building.
    ///
    /// Use this method to actively specify which scope should be used, instead the default `Scope` variant
    /// `Scope::CourseReadonly`.
    ///
    /// The `scope` will be added to a set of scopes. This is important as one can maintain access
    /// tokens for more than one scope.
    /// If `None` is specified, then all scopes will be removed and no default scope will be used either.
    /// In that case, you have to specify your API-key using the `key` parameter (see the `param()`
    /// function for details).
    ///
    /// Usually there is more than one suitable scope to authorize an operation, some of which may
    /// encompass more rights than others. For example, for listing resources, a *read-only* scope will be
    /// sufficient, a read-write scope will do as well.
    pub fn add_scope<T, S>(mut self, scope: T) -> CourseGetCall<'a, C, A>
                                                        where T: Into<Option<S>>,
                                                              S: AsRef<str> {
        match scope.into() {
          Some(scope) => self._scopes.insert(scope.as_ref().to_string(), ()),
          None => None,
        };
        self
    }
}


/// Updates a course.
/// 
/// This method returns the following error codes:
/// 
/// * `PERMISSION_DENIED` if the requesting user is not permitted to modify the
/// requested course or for access errors.
/// * `NOT_FOUND` if no course exists with the requested ID.
/// * `FAILED_PRECONDITION` for the following request errors:
///     * CourseNotModifiable
///
/// A builder for the *update* method supported by a *course* resource.
/// It is not used directly, but through a `CourseMethods` instance.
///
/// # Example
///
/// Instantiate a resource method builder
///
/// ```test_harness,no_run
/// # extern crate hyper;
/// # extern crate hyper_rustls;
/// # extern crate yup_oauth2 as oauth2;
/// # extern crate google_classroom1 as classroom1;
/// use classroom1::Course;
/// # #[test] fn egal() {
/// # use std::default::Default;
/// # use oauth2::{Authenticator, DefaultAuthenticatorDelegate, ApplicationSecret, MemoryStorage};
/// # use classroom1::Classroom;
/// 
/// # let secret: ApplicationSecret = Default::default();
/// # let auth = Authenticator::new(&secret, DefaultAuthenticatorDelegate,
/// #                               hyper::Client::with_connector(hyper::net::HttpsConnector::new(hyper_rustls::TlsClient::new())),
/// #                               <MemoryStorage as Default>::default(), None);
/// # let mut hub = Classroom::new(hyper::Client::with_connector(hyper::net::HttpsConnector::new(hyper_rustls::TlsClient::new())), auth);
/// // As the method needs a request, you would usually fill it with the desired information
/// // into the respective structure. Some of the parts shown here might not be applicable !
/// // Values shown here are possibly random and not representative !
/// let mut req = Course::default();
/// 
/// // You can configure optional parameters by calling the respective setters at will, and
/// // execute the final call using `doit()`.
/// // Values shown here are possibly random and not representative !
/// let result = hub.courses().update(req, "id")
///              .doit();
/// # }
/// ```
pub struct CourseUpdateCall<'a, C, A>
    where C: 'a, A: 'a {

    hub: &'a Classroom<C, A>,
    _request: Course,
    _id: String,
    _delegate: Option<&'a mut Delegate>,
    _additional_params: HashMap<String, String>,
    _scopes: BTreeMap<String, ()>
}

impl<'a, C, A> CallBuilder for CourseUpdateCall<'a, C, A> {}

impl<'a, C, A> CourseUpdateCall<'a, C, A> where C: BorrowMut<hyper::Client>, A: oauth2::GetToken {


    /// Perform the operation you have build so far.
    pub fn doit(mut self) -> Result<(hyper::client::Response, Course)> {
        use std::io::{Read, Seek};
        use hyper::header::{ContentType, ContentLength, Authorization, Bearer, UserAgent, Location};
        let mut dd = DefaultDelegate;
        let mut dlg: &mut Delegate = match self._delegate {
            Some(d) => d,
            None => &mut dd
        };
        dlg.begin(MethodInfo { id: "classroom.courses.update",
                               http_method: hyper::method::Method::Put });
        let mut params: Vec<(&str, String)> = Vec::with_capacity((4 + self._additional_params.len()));
        params.push(("id", self._id.to_string()));
        for &field in ["alt", "id"].iter() {
            if self._additional_params.contains_key(field) {
                dlg.finished(false);
                return Err(Error::FieldClash(field));
            }
        }
        for (name, value) in self._additional_params.iter() {
            params.push((&name, value.clone()));
        }

        params.push(("alt", "json".to_string()));

        let mut url = self.hub._base_url.clone() + "v1/courses/{id}";
        if self._scopes.len() == 0 {
            self._scopes.insert(Scope::Course.as_ref().to_string(), ());
        }

        for &(find_this, param_name) in [("{id}", "id")].iter() {
            let mut replace_with: Option<&str> = None;
            for &(name, ref value) in params.iter() {
                if name == param_name {
                    replace_with = Some(value);
                    break;
                }
            }
            url = url.replace(find_this, replace_with.expect("to find substitution value in params"));
        }
        {
            let mut indices_for_removal: Vec<usize> = Vec::with_capacity(1);
            for param_name in ["id"].iter() {
                if let Some(index) = params.iter().position(|t| &t.0 == param_name) {
                    indices_for_removal.push(index);
                }
            }
            for &index in indices_for_removal.iter() {
                params.remove(index);
            }
        }

        if params.len() > 0 {
            url.push('?');
            url.push_str(&url::form_urlencoded::serialize(params));
        }

        let mut json_mime_type = mime::Mime(mime::TopLevel::Application, mime::SubLevel::Json, Default::default());
        let mut request_value_reader =
            {
                let mut value = json::value::to_value(&self._request).expect("serde to work");
                remove_json_null_values(&mut value);
                let mut dst = io::Cursor::new(Vec::with_capacity(128));
                json::to_writer(&mut dst, &value).unwrap();
                dst
            };
        let request_size = request_value_reader.seek(io::SeekFrom::End(0)).unwrap();
        request_value_reader.seek(io::SeekFrom::Start(0)).unwrap();


        loop {
            let token = match self.hub.auth.borrow_mut().token(self._scopes.keys()) {
                Ok(token) => token,
                Err(err) => {
                    match  dlg.token(&*err) {
                        Some(token) => token,
                        None => {
                            dlg.finished(false);
                            return Err(Error::MissingToken(err))
                        }
                    }
                }
            };
            let auth_header = Authorization(Bearer { token: token.access_token });
            request_value_reader.seek(io::SeekFrom::Start(0)).unwrap();
            let mut req_result = {
                let mut client = &mut *self.hub.client.borrow_mut();
                let mut req = client.borrow_mut().request(hyper::method::Method::Put, &url)
                    .header(UserAgent(self.hub._user_agent.clone()))
                    .header(auth_header.clone())
                    .header(ContentType(json_mime_type.clone()))
                    .header(ContentLength(request_size as u64))
                    .body(&mut request_value_reader);

                dlg.pre_request();
                req.send()
            };

            match req_result {
                Err(err) => {
                    if let oauth2::Retry::After(d) = dlg.http_error(&err) {
                        sleep(d);
                        continue;
                    }
                    dlg.finished(false);
                    return Err(Error::HttpError(err))
                }
                Ok(mut res) => {
                    if !res.status.is_success() {
                        let mut json_err = String::new();
                        res.read_to_string(&mut json_err).unwrap();
                        if let oauth2::Retry::After(d) = dlg.http_failure(&res,
                                                              json::from_str(&json_err).ok(),
                                                              json::from_str(&json_err).ok()) {
                            sleep(d);
                            continue;
                        }
                        dlg.finished(false);
                        return match json::from_str::<ErrorResponse>(&json_err){
                            Err(_) => Err(Error::Failure(res)),
                            Ok(serr) => Err(Error::BadRequest(serr))
                        }
                    }
                    let result_value = {
                        let mut json_response = String::new();
                        res.read_to_string(&mut json_response).unwrap();
                        match json::from_str(&json_response) {
                            Ok(decoded) => (res, decoded),
                            Err(err) => {
                                dlg.response_json_decode_error(&json_response, &err);
                                return Err(Error::JsonDecodeError(json_response, err));
                            }
                        }
                    };

                    dlg.finished(true);
                    return Ok(result_value)
                }
            }
        }
    }


    ///
    /// Sets the *request* property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call,
    /// we provide this method for API completeness.
    pub fn request(mut self, new_value: Course) -> CourseUpdateCall<'a, C, A> {
        self._request = new_value;
        self
    }
    /// Identifier of the course to update.
    /// This identifier can be either the Classroom-assigned identifier or an
    /// alias.
    ///
    /// Sets the *id* path property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call,
    /// we provide this method for API completeness.
    pub fn id(mut self, new_value: &str) -> CourseUpdateCall<'a, C, A> {
        self._id = new_value.to_string();
        self
    }
    /// The delegate implementation is consulted whenever there is an intermediate result, or if something goes wrong
    /// while executing the actual API request.
    /// 
    /// It should be used to handle progress information, and to implement a certain level of resilience.
    ///
    /// Sets the *delegate* property to the given value.
    pub fn delegate(mut self, new_value: &'a mut Delegate) -> CourseUpdateCall<'a, C, A> {
        self._delegate = Some(new_value);
        self
    }

    /// Set any additional parameter of the query string used in the request.
    /// It should be used to set parameters which are not yet available through their own
    /// setters.
    ///
    /// Please note that this method must not be used to set any of the known paramters
    /// which have their own setter method. If done anyway, the request will fail.
    ///
    /// # Additional Parameters
    ///
    /// * *upload_protocol* (query-string) - Upload protocol for media (e.g. "raw", "multipart").
    /// * *pp* (query-boolean) - Pretty-print response.
    /// * *prettyPrint* (query-boolean) - Returns response with indentations and line breaks.
    /// * *bearer_token* (query-string) - OAuth bearer token.
    /// * *access_token* (query-string) - OAuth access token.
    /// * *uploadType* (query-string) - Legacy upload protocol for media (e.g. "media", "multipart").
    /// * *quotaUser* (query-string) - Available to use for quota purposes for server-side applications. Can be any arbitrary string assigned to a user, but should not exceed 40 characters.
    /// * *callback* (query-string) - JSONP
    /// * *oauth_token* (query-string) - OAuth 2.0 token for the current user.
    /// * *key* (query-string) - API key. Your API key identifies your project and provides you with API access, quota, and reports. Required unless you provide an OAuth 2.0 token.
    /// * *fields* (query-string) - Selector specifying which fields to include in a partial response.
    /// * *alt* (query-string) - Data format for response.
    /// * *$.xgafv* (query-string) - V1 error format.
    pub fn param<T>(mut self, name: T, value: T) -> CourseUpdateCall<'a, C, A>
                                                        where T: AsRef<str> {
        self._additional_params.insert(name.as_ref().to_string(), value.as_ref().to_string());
        self
    }

    /// Identifies the authorization scope for the method you are building.
    ///
    /// Use this method to actively specify which scope should be used, instead the default `Scope` variant
    /// `Scope::Course`.
    ///
    /// The `scope` will be added to a set of scopes. This is important as one can maintain access
    /// tokens for more than one scope.
    /// If `None` is specified, then all scopes will be removed and no default scope will be used either.
    /// In that case, you have to specify your API-key using the `key` parameter (see the `param()`
    /// function for details).
    ///
    /// Usually there is more than one suitable scope to authorize an operation, some of which may
    /// encompass more rights than others. For example, for listing resources, a *read-only* scope will be
    /// sufficient, a read-write scope will do as well.
    pub fn add_scope<T, S>(mut self, scope: T) -> CourseUpdateCall<'a, C, A>
                                                        where T: Into<Option<S>>,
                                                              S: AsRef<str> {
        match scope.into() {
          Some(scope) => self._scopes.insert(scope.as_ref().to_string(), ()),
          None => None,
        };
        self
    }
}


/// Returns a student of a course.
/// 
/// This method returns the following error codes:
/// 
/// * `PERMISSION_DENIED` if the requesting user is not permitted to view
/// students of this course or for access errors.
/// * `NOT_FOUND` if no student of this course has the requested ID or if the
/// course does not exist.
///
/// A builder for the *students.get* method supported by a *course* resource.
/// It is not used directly, but through a `CourseMethods` instance.
///
/// # Example
///
/// Instantiate a resource method builder
///
/// ```test_harness,no_run
/// # extern crate hyper;
/// # extern crate hyper_rustls;
/// # extern crate yup_oauth2 as oauth2;
/// # extern crate google_classroom1 as classroom1;
/// # #[test] fn egal() {
/// # use std::default::Default;
/// # use oauth2::{Authenticator, DefaultAuthenticatorDelegate, ApplicationSecret, MemoryStorage};
/// # use classroom1::Classroom;
/// 
/// # let secret: ApplicationSecret = Default::default();
/// # let auth = Authenticator::new(&secret, DefaultAuthenticatorDelegate,
/// #                               hyper::Client::with_connector(hyper::net::HttpsConnector::new(hyper_rustls::TlsClient::new())),
/// #                               <MemoryStorage as Default>::default(), None);
/// # let mut hub = Classroom::new(hyper::Client::with_connector(hyper::net::HttpsConnector::new(hyper_rustls::TlsClient::new())), auth);
/// // You can configure optional parameters by calling the respective setters at will, and
/// // execute the final call using `doit()`.
/// // Values shown here are possibly random and not representative !
/// let result = hub.courses().students_get("courseId", "userId")
///              .doit();
/// # }
/// ```
pub struct CourseStudentGetCall<'a, C, A>
    where C: 'a, A: 'a {

    hub: &'a Classroom<C, A>,
    _course_id: String,
    _user_id: String,
    _delegate: Option<&'a mut Delegate>,
    _additional_params: HashMap<String, String>,
    _scopes: BTreeMap<String, ()>
}

impl<'a, C, A> CallBuilder for CourseStudentGetCall<'a, C, A> {}

impl<'a, C, A> CourseStudentGetCall<'a, C, A> where C: BorrowMut<hyper::Client>, A: oauth2::GetToken {


    /// Perform the operation you have build so far.
    pub fn doit(mut self) -> Result<(hyper::client::Response, Student)> {
        use std::io::{Read, Seek};
        use hyper::header::{ContentType, ContentLength, Authorization, Bearer, UserAgent, Location};
        let mut dd = DefaultDelegate;
        let mut dlg: &mut Delegate = match self._delegate {
            Some(d) => d,
            None => &mut dd
        };
        dlg.begin(MethodInfo { id: "classroom.courses.students.get",
                               http_method: hyper::method::Method::Get });
        let mut params: Vec<(&str, String)> = Vec::with_capacity((4 + self._additional_params.len()));
        params.push(("courseId", self._course_id.to_string()));
        params.push(("userId", self._user_id.to_string()));
        for &field in ["alt", "courseId", "userId"].iter() {
            if self._additional_params.contains_key(field) {
                dlg.finished(false);
                return Err(Error::FieldClash(field));
            }
        }
        for (name, value) in self._additional_params.iter() {
            params.push((&name, value.clone()));
        }

        params.push(("alt", "json".to_string()));

        let mut url = self.hub._base_url.clone() + "v1/courses/{courseId}/students/{userId}";
        if self._scopes.len() == 0 {
            self._scopes.insert(Scope::RosterReadonly.as_ref().to_string(), ());
        }

        for &(find_this, param_name) in [("{courseId}", "courseId"), ("{userId}", "userId")].iter() {
            let mut replace_with: Option<&str> = None;
            for &(name, ref value) in params.iter() {
                if name == param_name {
                    replace_with = Some(value);
                    break;
                }
            }
            url = url.replace(find_this, replace_with.expect("to find substitution value in params"));
        }
        {
            let mut indices_for_removal: Vec<usize> = Vec::with_capacity(2);
            for param_name in ["userId", "courseId"].iter() {
                if let Some(index) = params.iter().position(|t| &t.0 == param_name) {
                    indices_for_removal.push(index);
                }
            }
            for &index in indices_for_removal.iter() {
                params.remove(index);
            }
        }

        if params.len() > 0 {
            url.push('?');
            url.push_str(&url::form_urlencoded::serialize(params));
        }



        loop {
            let token = match self.hub.auth.borrow_mut().token(self._scopes.keys()) {
                Ok(token) => token,
                Err(err) => {
                    match  dlg.token(&*err) {
                        Some(token) => token,
                        None => {
                            dlg.finished(false);
                            return Err(Error::MissingToken(err))
                        }
                    }
                }
            };
            let auth_header = Authorization(Bearer { token: token.access_token });
            let mut req_result = {
                let mut client = &mut *self.hub.client.borrow_mut();
                let mut req = client.borrow_mut().request(hyper::method::Method::Get, &url)
                    .header(UserAgent(self.hub._user_agent.clone()))
                    .header(auth_header.clone());

                dlg.pre_request();
                req.send()
            };

            match req_result {
                Err(err) => {
                    if let oauth2::Retry::After(d) = dlg.http_error(&err) {
                        sleep(d);
                        continue;
                    }
                    dlg.finished(false);
                    return Err(Error::HttpError(err))
                }
                Ok(mut res) => {
                    if !res.status.is_success() {
                        let mut json_err = String::new();
                        res.read_to_string(&mut json_err).unwrap();
                        if let oauth2::Retry::After(d) = dlg.http_failure(&res,
                                                              json::from_str(&json_err).ok(),
                                                              json::from_str(&json_err).ok()) {
                            sleep(d);
                            continue;
                        }
                        dlg.finished(false);
                        return match json::from_str::<ErrorResponse>(&json_err){
                            Err(_) => Err(Error::Failure(res)),
                            Ok(serr) => Err(Error::BadRequest(serr))
                        }
                    }
                    let result_value = {
                        let mut json_response = String::new();
                        res.read_to_string(&mut json_response).unwrap();
                        match json::from_str(&json_response) {
                            Ok(decoded) => (res, decoded),
                            Err(err) => {
                                dlg.response_json_decode_error(&json_response, &err);
                                return Err(Error::JsonDecodeError(json_response, err));
                            }
                        }
                    };

                    dlg.finished(true);
                    return Ok(result_value)
                }
            }
        }
    }


    /// Identifier of the course.
    /// This identifier can be either the Classroom-assigned identifier or an
    /// alias.
    ///
    /// Sets the *course id* path property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call,
    /// we provide this method for API completeness.
    pub fn course_id(mut self, new_value: &str) -> CourseStudentGetCall<'a, C, A> {
        self._course_id = new_value.to_string();
        self
    }
    /// Identifier of the student to return. The identifier can be one of the
    /// following:
    /// 
    /// * the numeric identifier for the user
    /// * the email address of the user
    /// * the string literal `"me"`, indicating the requesting user
    ///
    /// Sets the *user id* path property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call,
    /// we provide this method for API completeness.
    pub fn user_id(mut self, new_value: &str) -> CourseStudentGetCall<'a, C, A> {
        self._user_id = new_value.to_string();
        self
    }
    /// The delegate implementation is consulted whenever there is an intermediate result, or if something goes wrong
    /// while executing the actual API request.
    /// 
    /// It should be used to handle progress information, and to implement a certain level of resilience.
    ///
    /// Sets the *delegate* property to the given value.
    pub fn delegate(mut self, new_value: &'a mut Delegate) -> CourseStudentGetCall<'a, C, A> {
        self._delegate = Some(new_value);
        self
    }

    /// Set any additional parameter of the query string used in the request.
    /// It should be used to set parameters which are not yet available through their own
    /// setters.
    ///
    /// Please note that this method must not be used to set any of the known paramters
    /// which have their own setter method. If done anyway, the request will fail.
    ///
    /// # Additional Parameters
    ///
    /// * *upload_protocol* (query-string) - Upload protocol for media (e.g. "raw", "multipart").
    /// * *pp* (query-boolean) - Pretty-print response.
    /// * *prettyPrint* (query-boolean) - Returns response with indentations and line breaks.
    /// * *bearer_token* (query-string) - OAuth bearer token.
    /// * *access_token* (query-string) - OAuth access token.
    /// * *uploadType* (query-string) - Legacy upload protocol for media (e.g. "media", "multipart").
    /// * *quotaUser* (query-string) - Available to use for quota purposes for server-side applications. Can be any arbitrary string assigned to a user, but should not exceed 40 characters.
    /// * *callback* (query-string) - JSONP
    /// * *oauth_token* (query-string) - OAuth 2.0 token for the current user.
    /// * *key* (query-string) - API key. Your API key identifies your project and provides you with API access, quota, and reports. Required unless you provide an OAuth 2.0 token.
    /// * *fields* (query-string) - Selector specifying which fields to include in a partial response.
    /// * *alt* (query-string) - Data format for response.
    /// * *$.xgafv* (query-string) - V1 error format.
    pub fn param<T>(mut self, name: T, value: T) -> CourseStudentGetCall<'a, C, A>
                                                        where T: AsRef<str> {
        self._additional_params.insert(name.as_ref().to_string(), value.as_ref().to_string());
        self
    }

    /// Identifies the authorization scope for the method you are building.
    ///
    /// Use this method to actively specify which scope should be used, instead the default `Scope` variant
    /// `Scope::RosterReadonly`.
    ///
    /// The `scope` will be added to a set of scopes. This is important as one can maintain access
    /// tokens for more than one scope.
    /// If `None` is specified, then all scopes will be removed and no default scope will be used either.
    /// In that case, you have to specify your API-key using the `key` parameter (see the `param()`
    /// function for details).
    ///
    /// Usually there is more than one suitable scope to authorize an operation, some of which may
    /// encompass more rights than others. For example, for listing resources, a *read-only* scope will be
    /// sufficient, a read-write scope will do as well.
    pub fn add_scope<T, S>(mut self, scope: T) -> CourseStudentGetCall<'a, C, A>
                                                        where T: Into<Option<S>>,
                                                              S: AsRef<str> {
        match scope.into() {
          Some(scope) => self._scopes.insert(scope.as_ref().to_string(), ()),
          None => None,
        };
        self
    }
}


/// Returns a teacher of a course.
/// 
/// This method returns the following error codes:
/// 
/// * `PERMISSION_DENIED` if the requesting user is not permitted to view
/// teachers of this course or for access errors.
/// * `NOT_FOUND` if no teacher of this course has the requested ID or if the
/// course does not exist.
///
/// A builder for the *teachers.get* method supported by a *course* resource.
/// It is not used directly, but through a `CourseMethods` instance.
///
/// # Example
///
/// Instantiate a resource method builder
///
/// ```test_harness,no_run
/// # extern crate hyper;
/// # extern crate hyper_rustls;
/// # extern crate yup_oauth2 as oauth2;
/// # extern crate google_classroom1 as classroom1;
/// # #[test] fn egal() {
/// # use std::default::Default;
/// # use oauth2::{Authenticator, DefaultAuthenticatorDelegate, ApplicationSecret, MemoryStorage};
/// # use classroom1::Classroom;
/// 
/// # let secret: ApplicationSecret = Default::default();
/// # let auth = Authenticator::new(&secret, DefaultAuthenticatorDelegate,
/// #                               hyper::Client::with_connector(hyper::net::HttpsConnector::new(hyper_rustls::TlsClient::new())),
/// #                               <MemoryStorage as Default>::default(), None);
/// # let mut hub = Classroom::new(hyper::Client::with_connector(hyper::net::HttpsConnector::new(hyper_rustls::TlsClient::new())), auth);
/// // You can configure optional parameters by calling the respective setters at will, and
/// // execute the final call using `doit()`.
/// // Values shown here are possibly random and not representative !
/// let result = hub.courses().teachers_get("courseId", "userId")
///              .doit();
/// # }
/// ```
pub struct CourseTeacherGetCall<'a, C, A>
    where C: 'a, A: 'a {

    hub: &'a Classroom<C, A>,
    _course_id: String,
    _user_id: String,
    _delegate: Option<&'a mut Delegate>,
    _additional_params: HashMap<String, String>,
    _scopes: BTreeMap<String, ()>
}

impl<'a, C, A> CallBuilder for CourseTeacherGetCall<'a, C, A> {}

impl<'a, C, A> CourseTeacherGetCall<'a, C, A> where C: BorrowMut<hyper::Client>, A: oauth2::GetToken {


    /// Perform the operation you have build so far.
    pub fn doit(mut self) -> Result<(hyper::client::Response, Teacher)> {
        use std::io::{Read, Seek};
        use hyper::header::{ContentType, ContentLength, Authorization, Bearer, UserAgent, Location};
        let mut dd = DefaultDelegate;
        let mut dlg: &mut Delegate = match self._delegate {
            Some(d) => d,
            None => &mut dd
        };
        dlg.begin(MethodInfo { id: "classroom.courses.teachers.get",
                               http_method: hyper::method::Method::Get });
        let mut params: Vec<(&str, String)> = Vec::with_capacity((4 + self._additional_params.len()));
        params.push(("courseId", self._course_id.to_string()));
        params.push(("userId", self._user_id.to_string()));
        for &field in ["alt", "courseId", "userId"].iter() {
            if self._additional_params.contains_key(field) {
                dlg.finished(false);
                return Err(Error::FieldClash(field));
            }
        }
        for (name, value) in self._additional_params.iter() {
            params.push((&name, value.clone()));
        }

        params.push(("alt", "json".to_string()));

        let mut url = self.hub._base_url.clone() + "v1/courses/{courseId}/teachers/{userId}";
        if self._scopes.len() == 0 {
            self._scopes.insert(Scope::RosterReadonly.as_ref().to_string(), ());
        }

        for &(find_this, param_name) in [("{courseId}", "courseId"), ("{userId}", "userId")].iter() {
            let mut replace_with: Option<&str> = None;
            for &(name, ref value) in params.iter() {
                if name == param_name {
                    replace_with = Some(value);
                    break;
                }
            }
            url = url.replace(find_this, replace_with.expect("to find substitution value in params"));
        }
        {
            let mut indices_for_removal: Vec<usize> = Vec::with_capacity(2);
            for param_name in ["userId", "courseId"].iter() {
                if let Some(index) = params.iter().position(|t| &t.0 == param_name) {
                    indices_for_removal.push(index);
                }
            }
            for &index in indices_for_removal.iter() {
                params.remove(index);
            }
        }

        if params.len() > 0 {
            url.push('?');
            url.push_str(&url::form_urlencoded::serialize(params));
        }



        loop {
            let token = match self.hub.auth.borrow_mut().token(self._scopes.keys()) {
                Ok(token) => token,
                Err(err) => {
                    match  dlg.token(&*err) {
                        Some(token) => token,
                        None => {
                            dlg.finished(false);
                            return Err(Error::MissingToken(err))
                        }
                    }
                }
            };
            let auth_header = Authorization(Bearer { token: token.access_token });
            let mut req_result = {
                let mut client = &mut *self.hub.client.borrow_mut();
                let mut req = client.borrow_mut().request(hyper::method::Method::Get, &url)
                    .header(UserAgent(self.hub._user_agent.clone()))
                    .header(auth_header.clone());

                dlg.pre_request();
                req.send()
            };

            match req_result {
                Err(err) => {
                    if let oauth2::Retry::After(d) = dlg.http_error(&err) {
                        sleep(d);
                        continue;
                    }
                    dlg.finished(false);
                    return Err(Error::HttpError(err))
                }
                Ok(mut res) => {
                    if !res.status.is_success() {
                        let mut json_err = String::new();
                        res.read_to_string(&mut json_err).unwrap();
                        if let oauth2::Retry::After(d) = dlg.http_failure(&res,
                                                              json::from_str(&json_err).ok(),
                                                              json::from_str(&json_err).ok()) {
                            sleep(d);
                            continue;
                        }
                        dlg.finished(false);
                        return match json::from_str::<ErrorResponse>(&json_err){
                            Err(_) => Err(Error::Failure(res)),
                            Ok(serr) => Err(Error::BadRequest(serr))
                        }
                    }
                    let result_value = {
                        let mut json_response = String::new();
                        res.read_to_string(&mut json_response).unwrap();
                        match json::from_str(&json_response) {
                            Ok(decoded) => (res, decoded),
                            Err(err) => {
                                dlg.response_json_decode_error(&json_response, &err);
                                return Err(Error::JsonDecodeError(json_response, err));
                            }
                        }
                    };

                    dlg.finished(true);
                    return Ok(result_value)
                }
            }
        }
    }


    /// Identifier of the course.
    /// This identifier can be either the Classroom-assigned identifier or an
    /// alias.
    ///
    /// Sets the *course id* path property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call,
    /// we provide this method for API completeness.
    pub fn course_id(mut self, new_value: &str) -> CourseTeacherGetCall<'a, C, A> {
        self._course_id = new_value.to_string();
        self
    }
    /// Identifier of the teacher to return. The identifier can be one of the
    /// following:
    /// 
    /// * the numeric identifier for the user
    /// * the email address of the user
    /// * the string literal `"me"`, indicating the requesting user
    ///
    /// Sets the *user id* path property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call,
    /// we provide this method for API completeness.
    pub fn user_id(mut self, new_value: &str) -> CourseTeacherGetCall<'a, C, A> {
        self._user_id = new_value.to_string();
        self
    }
    /// The delegate implementation is consulted whenever there is an intermediate result, or if something goes wrong
    /// while executing the actual API request.
    /// 
    /// It should be used to handle progress information, and to implement a certain level of resilience.
    ///
    /// Sets the *delegate* property to the given value.
    pub fn delegate(mut self, new_value: &'a mut Delegate) -> CourseTeacherGetCall<'a, C, A> {
        self._delegate = Some(new_value);
        self
    }

    /// Set any additional parameter of the query string used in the request.
    /// It should be used to set parameters which are not yet available through their own
    /// setters.
    ///
    /// Please note that this method must not be used to set any of the known paramters
    /// which have their own setter method. If done anyway, the request will fail.
    ///
    /// # Additional Parameters
    ///
    /// * *upload_protocol* (query-string) - Upload protocol for media (e.g. "raw", "multipart").
    /// * *pp* (query-boolean) - Pretty-print response.
    /// * *prettyPrint* (query-boolean) - Returns response with indentations and line breaks.
    /// * *bearer_token* (query-string) - OAuth bearer token.
    /// * *access_token* (query-string) - OAuth access token.
    /// * *uploadType* (query-string) - Legacy upload protocol for media (e.g. "media", "multipart").
    /// * *quotaUser* (query-string) - Available to use for quota purposes for server-side applications. Can be any arbitrary string assigned to a user, but should not exceed 40 characters.
    /// * *callback* (query-string) - JSONP
    /// * *oauth_token* (query-string) - OAuth 2.0 token for the current user.
    /// * *key* (query-string) - API key. Your API key identifies your project and provides you with API access, quota, and reports. Required unless you provide an OAuth 2.0 token.
    /// * *fields* (query-string) - Selector specifying which fields to include in a partial response.
    /// * *alt* (query-string) - Data format for response.
    /// * *$.xgafv* (query-string) - V1 error format.
    pub fn param<T>(mut self, name: T, value: T) -> CourseTeacherGetCall<'a, C, A>
                                                        where T: AsRef<str> {
        self._additional_params.insert(name.as_ref().to_string(), value.as_ref().to_string());
        self
    }

    /// Identifies the authorization scope for the method you are building.
    ///
    /// Use this method to actively specify which scope should be used, instead the default `Scope` variant
    /// `Scope::RosterReadonly`.
    ///
    /// The `scope` will be added to a set of scopes. This is important as one can maintain access
    /// tokens for more than one scope.
    /// If `None` is specified, then all scopes will be removed and no default scope will be used either.
    /// In that case, you have to specify your API-key using the `key` parameter (see the `param()`
    /// function for details).
    ///
    /// Usually there is more than one suitable scope to authorize an operation, some of which may
    /// encompass more rights than others. For example, for listing resources, a *read-only* scope will be
    /// sufficient, a read-write scope will do as well.
    pub fn add_scope<T, S>(mut self, scope: T) -> CourseTeacherGetCall<'a, C, A>
                                                        where T: Into<Option<S>>,
                                                              S: AsRef<str> {
        match scope.into() {
          Some(scope) => self._scopes.insert(scope.as_ref().to_string(), ()),
          None => None,
        };
        self
    }
}


/// Returns a list of course work that the requester is permitted to view.
/// 
/// Course students may only view `PUBLISHED` course work. Course teachers
/// and domain administrators may view all course work.
/// 
/// This method returns the following error codes:
/// 
/// * `PERMISSION_DENIED` if the requesting user is not permitted to access
/// the requested course or for access errors.
/// * `INVALID_ARGUMENT` if the request is malformed.
/// * `NOT_FOUND` if the requested course does not exist.
///
/// A builder for the *courseWork.list* method supported by a *course* resource.
/// It is not used directly, but through a `CourseMethods` instance.
///
/// # Example
///
/// Instantiate a resource method builder
///
/// ```test_harness,no_run
/// # extern crate hyper;
/// # extern crate hyper_rustls;
/// # extern crate yup_oauth2 as oauth2;
/// # extern crate google_classroom1 as classroom1;
/// # #[test] fn egal() {
/// # use std::default::Default;
/// # use oauth2::{Authenticator, DefaultAuthenticatorDelegate, ApplicationSecret, MemoryStorage};
/// # use classroom1::Classroom;
/// 
/// # let secret: ApplicationSecret = Default::default();
/// # let auth = Authenticator::new(&secret, DefaultAuthenticatorDelegate,
/// #                               hyper::Client::with_connector(hyper::net::HttpsConnector::new(hyper_rustls::TlsClient::new())),
/// #                               <MemoryStorage as Default>::default(), None);
/// # let mut hub = Classroom::new(hyper::Client::with_connector(hyper::net::HttpsConnector::new(hyper_rustls::TlsClient::new())), auth);
/// // You can configure optional parameters by calling the respective setters at will, and
/// // execute the final call using `doit()`.
/// // Values shown here are possibly random and not representative !
/// let result = hub.courses().course_work_list("courseId")
///              .page_token("amet")
///              .page_size(-60)
///              .order_by("labore")
///              .add_course_work_states("eirmod")
///              .doit();
/// # }
/// ```
pub struct CourseCourseWorkListCall<'a, C, A>
    where C: 'a, A: 'a {

    hub: &'a Classroom<C, A>,
    _course_id: String,
    _page_token: Option<String>,
    _page_size: Option<i32>,
    _order_by: Option<String>,
    _course_work_states: Vec<String>,
    _delegate: Option<&'a mut Delegate>,
    _additional_params: HashMap<String, String>,
    _scopes: BTreeMap<String, ()>
}

impl<'a, C, A> CallBuilder for CourseCourseWorkListCall<'a, C, A> {}

impl<'a, C, A> CourseCourseWorkListCall<'a, C, A> where C: BorrowMut<hyper::Client>, A: oauth2::GetToken {


    /// Perform the operation you have build so far.
    pub fn doit(mut self) -> Result<(hyper::client::Response, ListCourseWorkResponse)> {
        use std::io::{Read, Seek};
        use hyper::header::{ContentType, ContentLength, Authorization, Bearer, UserAgent, Location};
        let mut dd = DefaultDelegate;
        let mut dlg: &mut Delegate = match self._delegate {
            Some(d) => d,
            None => &mut dd
        };
        dlg.begin(MethodInfo { id: "classroom.courses.courseWork.list",
                               http_method: hyper::method::Method::Get });
        let mut params: Vec<(&str, String)> = Vec::with_capacity((7 + self._additional_params.len()));
        params.push(("courseId", self._course_id.to_string()));
        if let Some(value) = self._page_token {
            params.push(("pageToken", value.to_string()));
        }
        if let Some(value) = self._page_size {
            params.push(("pageSize", value.to_string()));
        }
        if let Some(value) = self._order_by {
            params.push(("orderBy", value.to_string()));
        }
        if self._course_work_states.len() > 0 {
            for f in self._course_work_states.iter() {
                params.push(("courseWorkStates", f.to_string()));
            }
        }
        for &field in ["alt", "courseId", "pageToken", "pageSize", "orderBy", "courseWorkStates"].iter() {
            if self._additional_params.contains_key(field) {
                dlg.finished(false);
                return Err(Error::FieldClash(field));
            }
        }
        for (name, value) in self._additional_params.iter() {
            params.push((&name, value.clone()));
        }

        params.push(("alt", "json".to_string()));

        let mut url = self.hub._base_url.clone() + "v1/courses/{courseId}/courseWork";
        if self._scopes.len() == 0 {
            self._scopes.insert(Scope::CourseworkMeReadonly.as_ref().to_string(), ());
        }

        for &(find_this, param_name) in [("{courseId}", "courseId")].iter() {
            let mut replace_with: Option<&str> = None;
            for &(name, ref value) in params.iter() {
                if name == param_name {
                    replace_with = Some(value);
                    break;
                }
            }
            url = url.replace(find_this, replace_with.expect("to find substitution value in params"));
        }
        {
            let mut indices_for_removal: Vec<usize> = Vec::with_capacity(1);
            for param_name in ["courseId"].iter() {
                if let Some(index) = params.iter().position(|t| &t.0 == param_name) {
                    indices_for_removal.push(index);
                }
            }
            for &index in indices_for_removal.iter() {
                params.remove(index);
            }
        }

        if params.len() > 0 {
            url.push('?');
            url.push_str(&url::form_urlencoded::serialize(params));
        }



        loop {
            let token = match self.hub.auth.borrow_mut().token(self._scopes.keys()) {
                Ok(token) => token,
                Err(err) => {
                    match  dlg.token(&*err) {
                        Some(token) => token,
                        None => {
                            dlg.finished(false);
                            return Err(Error::MissingToken(err))
                        }
                    }
                }
            };
            let auth_header = Authorization(Bearer { token: token.access_token });
            let mut req_result = {
                let mut client = &mut *self.hub.client.borrow_mut();
                let mut req = client.borrow_mut().request(hyper::method::Method::Get, &url)
                    .header(UserAgent(self.hub._user_agent.clone()))
                    .header(auth_header.clone());

                dlg.pre_request();
                req.send()
            };

            match req_result {
                Err(err) => {
                    if let oauth2::Retry::After(d) = dlg.http_error(&err) {
                        sleep(d);
                        continue;
                    }
                    dlg.finished(false);
                    return Err(Error::HttpError(err))
                }
                Ok(mut res) => {
                    if !res.status.is_success() {
                        let mut json_err = String::new();
                        res.read_to_string(&mut json_err).unwrap();
                        if let oauth2::Retry::After(d) = dlg.http_failure(&res,
                                                              json::from_str(&json_err).ok(),
                                                              json::from_str(&json_err).ok()) {
                            sleep(d);
                            continue;
                        }
                        dlg.finished(false);
                        return match json::from_str::<ErrorResponse>(&json_err){
                            Err(_) => Err(Error::Failure(res)),
                            Ok(serr) => Err(Error::BadRequest(serr))
                        }
                    }
                    let result_value = {
                        let mut json_response = String::new();
                        res.read_to_string(&mut json_response).unwrap();
                        match json::from_str(&json_response) {
                            Ok(decoded) => (res, decoded),
                            Err(err) => {
                                dlg.response_json_decode_error(&json_response, &err);
                                return Err(Error::JsonDecodeError(json_response, err));
                            }
                        }
                    };

                    dlg.finished(true);
                    return Ok(result_value)
                }
            }
        }
    }


    /// Identifier of the course.
    /// This identifier can be either the Classroom-assigned identifier or an
    /// alias.
    ///
    /// Sets the *course id* path property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call,
    /// we provide this method for API completeness.
    pub fn course_id(mut self, new_value: &str) -> CourseCourseWorkListCall<'a, C, A> {
        self._course_id = new_value.to_string();
        self
    }
    /// nextPageToken
    /// value returned from a previous
    /// list call,
    /// indicating that the subsequent page of results should be returned.
    /// 
    /// The list request
    /// must be otherwise identical to the one that resulted in this token.
    ///
    /// Sets the *page token* query property to the given value.
    pub fn page_token(mut self, new_value: &str) -> CourseCourseWorkListCall<'a, C, A> {
        self._page_token = Some(new_value.to_string());
        self
    }
    /// Maximum number of items to return. Zero or unspecified indicates that the
    /// server may assign a maximum.
    /// 
    /// The server may return fewer than the specified number of results.
    ///
    /// Sets the *page size* query property to the given value.
    pub fn page_size(mut self, new_value: i32) -> CourseCourseWorkListCall<'a, C, A> {
        self._page_size = Some(new_value);
        self
    }
    /// Optional sort ordering for results. A comma-separated list of fields with
    /// an optional sort direction keyword. Supported fields are `updateTime`
    /// and `dueDate`. Supported direction keywords are `asc` and `desc`.
    /// If not specified, `updateTime desc` is the default behavior.
    /// Examples: `dueDate asc,updateTime desc`, `updateTime,dueDate desc`
    ///
    /// Sets the *order by* query property to the given value.
    pub fn order_by(mut self, new_value: &str) -> CourseCourseWorkListCall<'a, C, A> {
        self._order_by = Some(new_value.to_string());
        self
    }
    /// Restriction on the work status to return. Only courseWork that matches
    /// is returned. If unspecified, items with a work status of `PUBLISHED`
    /// is returned.
    ///
    /// Append the given value to the *course work states* query property.
    /// Each appended value will retain its original ordering and be '/'-separated in the URL's parameters.
    pub fn add_course_work_states(mut self, new_value: &str) -> CourseCourseWorkListCall<'a, C, A> {
        self._course_work_states.push(new_value.to_string());
        self
    }
    /// The delegate implementation is consulted whenever there is an intermediate result, or if something goes wrong
    /// while executing the actual API request.
    /// 
    /// It should be used to handle progress information, and to implement a certain level of resilience.
    ///
    /// Sets the *delegate* property to the given value.
    pub fn delegate(mut self, new_value: &'a mut Delegate) -> CourseCourseWorkListCall<'a, C, A> {
        self._delegate = Some(new_value);
        self
    }

    /// Set any additional parameter of the query string used in the request.
    /// It should be used to set parameters which are not yet available through their own
    /// setters.
    ///
    /// Please note that this method must not be used to set any of the known paramters
    /// which have their own setter method. If done anyway, the request will fail.
    ///
    /// # Additional Parameters
    ///
    /// * *upload_protocol* (query-string) - Upload protocol for media (e.g. "raw", "multipart").
    /// * *pp* (query-boolean) - Pretty-print response.
    /// * *prettyPrint* (query-boolean) - Returns response with indentations and line breaks.
    /// * *bearer_token* (query-string) - OAuth bearer token.
    /// * *access_token* (query-string) - OAuth access token.
    /// * *uploadType* (query-string) - Legacy upload protocol for media (e.g. "media", "multipart").
    /// * *quotaUser* (query-string) - Available to use for quota purposes for server-side applications. Can be any arbitrary string assigned to a user, but should not exceed 40 characters.
    /// * *callback* (query-string) - JSONP
    /// * *oauth_token* (query-string) - OAuth 2.0 token for the current user.
    /// * *key* (query-string) - API key. Your API key identifies your project and provides you with API access, quota, and reports. Required unless you provide an OAuth 2.0 token.
    /// * *fields* (query-string) - Selector specifying which fields to include in a partial response.
    /// * *alt* (query-string) - Data format for response.
    /// * *$.xgafv* (query-string) - V1 error format.
    pub fn param<T>(mut self, name: T, value: T) -> CourseCourseWorkListCall<'a, C, A>
                                                        where T: AsRef<str> {
        self._additional_params.insert(name.as_ref().to_string(), value.as_ref().to_string());
        self
    }

    /// Identifies the authorization scope for the method you are building.
    ///
    /// Use this method to actively specify which scope should be used, instead the default `Scope` variant
    /// `Scope::CourseworkMeReadonly`.
    ///
    /// The `scope` will be added to a set of scopes. This is important as one can maintain access
    /// tokens for more than one scope.
    /// If `None` is specified, then all scopes will be removed and no default scope will be used either.
    /// In that case, you have to specify your API-key using the `key` parameter (see the `param()`
    /// function for details).
    ///
    /// Usually there is more than one suitable scope to authorize an operation, some of which may
    /// encompass more rights than others. For example, for listing resources, a *read-only* scope will be
    /// sufficient, a read-write scope will do as well.
    pub fn add_scope<T, S>(mut self, scope: T) -> CourseCourseWorkListCall<'a, C, A>
                                                        where T: Into<Option<S>>,
                                                              S: AsRef<str> {
        match scope.into() {
          Some(scope) => self._scopes.insert(scope.as_ref().to_string(), ()),
          None => None,
        };
        self
    }
}


/// Returns course work.
/// 
/// This method returns the following error codes:
/// 
/// * `PERMISSION_DENIED` if the requesting user is not permitted to access the
/// requested course or course work, or for access errors.
/// * `INVALID_ARGUMENT` if the request is malformed.
/// * `NOT_FOUND` if the requested course or course work does not exist.
///
/// A builder for the *courseWork.get* method supported by a *course* resource.
/// It is not used directly, but through a `CourseMethods` instance.
///
/// # Example
///
/// Instantiate a resource method builder
///
/// ```test_harness,no_run
/// # extern crate hyper;
/// # extern crate hyper_rustls;
/// # extern crate yup_oauth2 as oauth2;
/// # extern crate google_classroom1 as classroom1;
/// # #[test] fn egal() {
/// # use std::default::Default;
/// # use oauth2::{Authenticator, DefaultAuthenticatorDelegate, ApplicationSecret, MemoryStorage};
/// # use classroom1::Classroom;
/// 
/// # let secret: ApplicationSecret = Default::default();
/// # let auth = Authenticator::new(&secret, DefaultAuthenticatorDelegate,
/// #                               hyper::Client::with_connector(hyper::net::HttpsConnector::new(hyper_rustls::TlsClient::new())),
/// #                               <MemoryStorage as Default>::default(), None);
/// # let mut hub = Classroom::new(hyper::Client::with_connector(hyper::net::HttpsConnector::new(hyper_rustls::TlsClient::new())), auth);
/// // You can configure optional parameters by calling the respective setters at will, and
/// // execute the final call using `doit()`.
/// // Values shown here are possibly random and not representative !
/// let result = hub.courses().course_work_get("courseId", "id")
///              .doit();
/// # }
/// ```
pub struct CourseCourseWorkGetCall<'a, C, A>
    where C: 'a, A: 'a {

    hub: &'a Classroom<C, A>,
    _course_id: String,
    _id: String,
    _delegate: Option<&'a mut Delegate>,
    _additional_params: HashMap<String, String>,
    _scopes: BTreeMap<String, ()>
}

impl<'a, C, A> CallBuilder for CourseCourseWorkGetCall<'a, C, A> {}

impl<'a, C, A> CourseCourseWorkGetCall<'a, C, A> where C: BorrowMut<hyper::Client>, A: oauth2::GetToken {


    /// Perform the operation you have build so far.
    pub fn doit(mut self) -> Result<(hyper::client::Response, CourseWork)> {
        use std::io::{Read, Seek};
        use hyper::header::{ContentType, ContentLength, Authorization, Bearer, UserAgent, Location};
        let mut dd = DefaultDelegate;
        let mut dlg: &mut Delegate = match self._delegate {
            Some(d) => d,
            None => &mut dd
        };
        dlg.begin(MethodInfo { id: "classroom.courses.courseWork.get",
                               http_method: hyper::method::Method::Get });
        let mut params: Vec<(&str, String)> = Vec::with_capacity((4 + self._additional_params.len()));
        params.push(("courseId", self._course_id.to_string()));
        params.push(("id", self._id.to_string()));
        for &field in ["alt", "courseId", "id"].iter() {
            if self._additional_params.contains_key(field) {
                dlg.finished(false);
                return Err(Error::FieldClash(field));
            }
        }
        for (name, value) in self._additional_params.iter() {
            params.push((&name, value.clone()));
        }

        params.push(("alt", "json".to_string()));

        let mut url = self.hub._base_url.clone() + "v1/courses/{courseId}/courseWork/{id}";
        if self._scopes.len() == 0 {
            self._scopes.insert(Scope::CourseworkMeReadonly.as_ref().to_string(), ());
        }

        for &(find_this, param_name) in [("{courseId}", "courseId"), ("{id}", "id")].iter() {
            let mut replace_with: Option<&str> = None;
            for &(name, ref value) in params.iter() {
                if name == param_name {
                    replace_with = Some(value);
                    break;
                }
            }
            url = url.replace(find_this, replace_with.expect("to find substitution value in params"));
        }
        {
            let mut indices_for_removal: Vec<usize> = Vec::with_capacity(2);
            for param_name in ["id", "courseId"].iter() {
                if let Some(index) = params.iter().position(|t| &t.0 == param_name) {
                    indices_for_removal.push(index);
                }
            }
            for &index in indices_for_removal.iter() {
                params.remove(index);
            }
        }

        if params.len() > 0 {
            url.push('?');
            url.push_str(&url::form_urlencoded::serialize(params));
        }



        loop {
            let token = match self.hub.auth.borrow_mut().token(self._scopes.keys()) {
                Ok(token) => token,
                Err(err) => {
                    match  dlg.token(&*err) {
                        Some(token) => token,
                        None => {
                            dlg.finished(false);
                            return Err(Error::MissingToken(err))
                        }
                    }
                }
            };
            let auth_header = Authorization(Bearer { token: token.access_token });
            let mut req_result = {
                let mut client = &mut *self.hub.client.borrow_mut();
                let mut req = client.borrow_mut().request(hyper::method::Method::Get, &url)
                    .header(UserAgent(self.hub._user_agent.clone()))
                    .header(auth_header.clone());

                dlg.pre_request();
                req.send()
            };

            match req_result {
                Err(err) => {
                    if let oauth2::Retry::After(d) = dlg.http_error(&err) {
                        sleep(d);
                        continue;
                    }
                    dlg.finished(false);
                    return Err(Error::HttpError(err))
                }
                Ok(mut res) => {
                    if !res.status.is_success() {
                        let mut json_err = String::new();
                        res.read_to_string(&mut json_err).unwrap();
                        if let oauth2::Retry::After(d) = dlg.http_failure(&res,
                                                              json::from_str(&json_err).ok(),
                                                              json::from_str(&json_err).ok()) {
                            sleep(d);
                            continue;
                        }
                        dlg.finished(false);
                        return match json::from_str::<ErrorResponse>(&json_err){
                            Err(_) => Err(Error::Failure(res)),
                            Ok(serr) => Err(Error::BadRequest(serr))
                        }
                    }
                    let result_value = {
                        let mut json_response = String::new();
                        res.read_to_string(&mut json_response).unwrap();
                        match json::from_str(&json_response) {
                            Ok(decoded) => (res, decoded),
                            Err(err) => {
                                dlg.response_json_decode_error(&json_response, &err);
                                return Err(Error::JsonDecodeError(json_response, err));
                            }
                        }
                    };

                    dlg.finished(true);
                    return Ok(result_value)
                }
            }
        }
    }


    /// Identifier of the course.
    /// This identifier can be either the Classroom-assigned identifier or an
    /// alias.
    ///
    /// Sets the *course id* path property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call,
    /// we provide this method for API completeness.
    pub fn course_id(mut self, new_value: &str) -> CourseCourseWorkGetCall<'a, C, A> {
        self._course_id = new_value.to_string();
        self
    }
    /// Identifier of the course work.
    ///
    /// Sets the *id* path property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call,
    /// we provide this method for API completeness.
    pub fn id(mut self, new_value: &str) -> CourseCourseWorkGetCall<'a, C, A> {
        self._id = new_value.to_string();
        self
    }
    /// The delegate implementation is consulted whenever there is an intermediate result, or if something goes wrong
    /// while executing the actual API request.
    /// 
    /// It should be used to handle progress information, and to implement a certain level of resilience.
    ///
    /// Sets the *delegate* property to the given value.
    pub fn delegate(mut self, new_value: &'a mut Delegate) -> CourseCourseWorkGetCall<'a, C, A> {
        self._delegate = Some(new_value);
        self
    }

    /// Set any additional parameter of the query string used in the request.
    /// It should be used to set parameters which are not yet available through their own
    /// setters.
    ///
    /// Please note that this method must not be used to set any of the known paramters
    /// which have their own setter method. If done anyway, the request will fail.
    ///
    /// # Additional Parameters
    ///
    /// * *upload_protocol* (query-string) - Upload protocol for media (e.g. "raw", "multipart").
    /// * *pp* (query-boolean) - Pretty-print response.
    /// * *prettyPrint* (query-boolean) - Returns response with indentations and line breaks.
    /// * *bearer_token* (query-string) - OAuth bearer token.
    /// * *access_token* (query-string) - OAuth access token.
    /// * *uploadType* (query-string) - Legacy upload protocol for media (e.g. "media", "multipart").
    /// * *quotaUser* (query-string) - Available to use for quota purposes for server-side applications. Can be any arbitrary string assigned to a user, but should not exceed 40 characters.
    /// * *callback* (query-string) - JSONP
    /// * *oauth_token* (query-string) - OAuth 2.0 token for the current user.
    /// * *key* (query-string) - API key. Your API key identifies your project and provides you with API access, quota, and reports. Required unless you provide an OAuth 2.0 token.
    /// * *fields* (query-string) - Selector specifying which fields to include in a partial response.
    /// * *alt* (query-string) - Data format for response.
    /// * *$.xgafv* (query-string) - V1 error format.
    pub fn param<T>(mut self, name: T, value: T) -> CourseCourseWorkGetCall<'a, C, A>
                                                        where T: AsRef<str> {
        self._additional_params.insert(name.as_ref().to_string(), value.as_ref().to_string());
        self
    }

    /// Identifies the authorization scope for the method you are building.
    ///
    /// Use this method to actively specify which scope should be used, instead the default `Scope` variant
    /// `Scope::CourseworkMeReadonly`.
    ///
    /// The `scope` will be added to a set of scopes. This is important as one can maintain access
    /// tokens for more than one scope.
    /// If `None` is specified, then all scopes will be removed and no default scope will be used either.
    /// In that case, you have to specify your API-key using the `key` parameter (see the `param()`
    /// function for details).
    ///
    /// Usually there is more than one suitable scope to authorize an operation, some of which may
    /// encompass more rights than others. For example, for listing resources, a *read-only* scope will be
    /// sufficient, a read-write scope will do as well.
    pub fn add_scope<T, S>(mut self, scope: T) -> CourseCourseWorkGetCall<'a, C, A>
                                                        where T: Into<Option<S>>,
                                                              S: AsRef<str> {
        match scope.into() {
          Some(scope) => self._scopes.insert(scope.as_ref().to_string(), ()),
          None => None,
        };
        self
    }
}


/// Turns in a student submission.
/// 
/// Turning in a student submission transfers ownership of attached Drive
/// files to the teacher and may also update the submission state.
/// 
/// This may only be called by the student that owns the specified student
/// submission.
/// 
/// This request must be made by the Developer Console project of the
/// [OAuth client ID](https://support.google.com/cloud/answer/6158849) used to
/// create the corresponding course work item.
/// 
/// This method returns the following error codes:
/// 
/// * `PERMISSION_DENIED` if the requesting user is not permitted to access the
/// requested course or course work, turn in the requested student submission,
/// or for access errors.
/// * `INVALID_ARGUMENT` if the request is malformed.
/// * `NOT_FOUND` if the requested course, course work, or student submission
/// does not exist.
///
/// A builder for the *courseWork.studentSubmissions.turnIn* method supported by a *course* resource.
/// It is not used directly, but through a `CourseMethods` instance.
///
/// # Example
///
/// Instantiate a resource method builder
///
/// ```test_harness,no_run
/// # extern crate hyper;
/// # extern crate hyper_rustls;
/// # extern crate yup_oauth2 as oauth2;
/// # extern crate google_classroom1 as classroom1;
/// use classroom1::TurnInStudentSubmissionRequest;
/// # #[test] fn egal() {
/// # use std::default::Default;
/// # use oauth2::{Authenticator, DefaultAuthenticatorDelegate, ApplicationSecret, MemoryStorage};
/// # use classroom1::Classroom;
/// 
/// # let secret: ApplicationSecret = Default::default();
/// # let auth = Authenticator::new(&secret, DefaultAuthenticatorDelegate,
/// #                               hyper::Client::with_connector(hyper::net::HttpsConnector::new(hyper_rustls::TlsClient::new())),
/// #                               <MemoryStorage as Default>::default(), None);
/// # let mut hub = Classroom::new(hyper::Client::with_connector(hyper::net::HttpsConnector::new(hyper_rustls::TlsClient::new())), auth);
/// // As the method needs a request, you would usually fill it with the desired information
/// // into the respective structure. Some of the parts shown here might not be applicable !
/// // Values shown here are possibly random and not representative !
/// let mut req = TurnInStudentSubmissionRequest::default();
/// 
/// // You can configure optional parameters by calling the respective setters at will, and
/// // execute the final call using `doit()`.
/// // Values shown here are possibly random and not representative !
/// let result = hub.courses().course_work_student_submissions_turn_in(req, "courseId", "courseWorkId", "id")
///              .doit();
/// # }
/// ```
pub struct CourseCourseWorkStudentSubmissionTurnInCall<'a, C, A>
    where C: 'a, A: 'a {

    hub: &'a Classroom<C, A>,
    _request: TurnInStudentSubmissionRequest,
    _course_id: String,
    _course_work_id: String,
    _id: String,
    _delegate: Option<&'a mut Delegate>,
    _additional_params: HashMap<String, String>,
    _scopes: BTreeMap<String, ()>
}

impl<'a, C, A> CallBuilder for CourseCourseWorkStudentSubmissionTurnInCall<'a, C, A> {}

impl<'a, C, A> CourseCourseWorkStudentSubmissionTurnInCall<'a, C, A> where C: BorrowMut<hyper::Client>, A: oauth2::GetToken {


    /// Perform the operation you have build so far.
    pub fn doit(mut self) -> Result<(hyper::client::Response, Empty)> {
        use std::io::{Read, Seek};
        use hyper::header::{ContentType, ContentLength, Authorization, Bearer, UserAgent, Location};
        let mut dd = DefaultDelegate;
        let mut dlg: &mut Delegate = match self._delegate {
            Some(d) => d,
            None => &mut dd
        };
        dlg.begin(MethodInfo { id: "classroom.courses.courseWork.studentSubmissions.turnIn",
                               http_method: hyper::method::Method::Post });
        let mut params: Vec<(&str, String)> = Vec::with_capacity((6 + self._additional_params.len()));
        params.push(("courseId", self._course_id.to_string()));
        params.push(("courseWorkId", self._course_work_id.to_string()));
        params.push(("id", self._id.to_string()));
        for &field in ["alt", "courseId", "courseWorkId", "id"].iter() {
            if self._additional_params.contains_key(field) {
                dlg.finished(false);
                return Err(Error::FieldClash(field));
            }
        }
        for (name, value) in self._additional_params.iter() {
            params.push((&name, value.clone()));
        }

        params.push(("alt", "json".to_string()));

        let mut url = self.hub._base_url.clone() + "v1/courses/{courseId}/courseWork/{courseWorkId}/studentSubmissions/{id}:turnIn";
        if self._scopes.len() == 0 {
            self._scopes.insert(Scope::CourseworkMe.as_ref().to_string(), ());
        }

        for &(find_this, param_name) in [("{courseId}", "courseId"), ("{courseWorkId}", "courseWorkId"), ("{id}", "id")].iter() {
            let mut replace_with: Option<&str> = None;
            for &(name, ref value) in params.iter() {
                if name == param_name {
                    replace_with = Some(value);
                    break;
                }
            }
            url = url.replace(find_this, replace_with.expect("to find substitution value in params"));
        }
        {
            let mut indices_for_removal: Vec<usize> = Vec::with_capacity(3);
            for param_name in ["id", "courseWorkId", "courseId"].iter() {
                if let Some(index) = params.iter().position(|t| &t.0 == param_name) {
                    indices_for_removal.push(index);
                }
            }
            for &index in indices_for_removal.iter() {
                params.remove(index);
            }
        }

        if params.len() > 0 {
            url.push('?');
            url.push_str(&url::form_urlencoded::serialize(params));
        }

        let mut json_mime_type = mime::Mime(mime::TopLevel::Application, mime::SubLevel::Json, Default::default());
        let mut request_value_reader =
            {
                let mut value = json::value::to_value(&self._request).expect("serde to work");
                remove_json_null_values(&mut value);
                let mut dst = io::Cursor::new(Vec::with_capacity(128));
                json::to_writer(&mut dst, &value).unwrap();
                dst
            };
        let request_size = request_value_reader.seek(io::SeekFrom::End(0)).unwrap();
        request_value_reader.seek(io::SeekFrom::Start(0)).unwrap();


        loop {
            let token = match self.hub.auth.borrow_mut().token(self._scopes.keys()) {
                Ok(token) => token,
                Err(err) => {
                    match  dlg.token(&*err) {
                        Some(token) => token,
                        None => {
                            dlg.finished(false);
                            return Err(Error::MissingToken(err))
                        }
                    }
                }
            };
            let auth_header = Authorization(Bearer { token: token.access_token });
            request_value_reader.seek(io::SeekFrom::Start(0)).unwrap();
            let mut req_result = {
                let mut client = &mut *self.hub.client.borrow_mut();
                let mut req = client.borrow_mut().request(hyper::method::Method::Post, &url)
                    .header(UserAgent(self.hub._user_agent.clone()))
                    .header(auth_header.clone())
                    .header(ContentType(json_mime_type.clone()))
                    .header(ContentLength(request_size as u64))
                    .body(&mut request_value_reader);

                dlg.pre_request();
                req.send()
            };

            match req_result {
                Err(err) => {
                    if let oauth2::Retry::After(d) = dlg.http_error(&err) {
                        sleep(d);
                        continue;
                    }
                    dlg.finished(false);
                    return Err(Error::HttpError(err))
                }
                Ok(mut res) => {
                    if !res.status.is_success() {
                        let mut json_err = String::new();
                        res.read_to_string(&mut json_err).unwrap();
                        if let oauth2::Retry::After(d) = dlg.http_failure(&res,
                                                              json::from_str(&json_err).ok(),
                                                              json::from_str(&json_err).ok()) {
                            sleep(d);
                            continue;
                        }
                        dlg.finished(false);
                        return match json::from_str::<ErrorResponse>(&json_err){
                            Err(_) => Err(Error::Failure(res)),
                            Ok(serr) => Err(Error::BadRequest(serr))
                        }
                    }
                    let result_value = {
                        let mut json_response = String::new();
                        res.read_to_string(&mut json_response).unwrap();
                        match json::from_str(&json_response) {
                            Ok(decoded) => (res, decoded),
                            Err(err) => {
                                dlg.response_json_decode_error(&json_response, &err);
                                return Err(Error::JsonDecodeError(json_response, err));
                            }
                        }
                    };

                    dlg.finished(true);
                    return Ok(result_value)
                }
            }
        }
    }


    ///
    /// Sets the *request* property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call,
    /// we provide this method for API completeness.
    pub fn request(mut self, new_value: TurnInStudentSubmissionRequest) -> CourseCourseWorkStudentSubmissionTurnInCall<'a, C, A> {
        self._request = new_value;
        self
    }
    /// Identifier of the course.
    /// This identifier can be either the Classroom-assigned identifier or an
    /// alias.
    ///
    /// Sets the *course id* path property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call,
    /// we provide this method for API completeness.
    pub fn course_id(mut self, new_value: &str) -> CourseCourseWorkStudentSubmissionTurnInCall<'a, C, A> {
        self._course_id = new_value.to_string();
        self
    }
    /// Identifier of the course work.
    ///
    /// Sets the *course work id* path property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call,
    /// we provide this method for API completeness.
    pub fn course_work_id(mut self, new_value: &str) -> CourseCourseWorkStudentSubmissionTurnInCall<'a, C, A> {
        self._course_work_id = new_value.to_string();
        self
    }
    /// Identifier of the student submission.
    ///
    /// Sets the *id* path property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call,
    /// we provide this method for API completeness.
    pub fn id(mut self, new_value: &str) -> CourseCourseWorkStudentSubmissionTurnInCall<'a, C, A> {
        self._id = new_value.to_string();
        self
    }
    /// The delegate implementation is consulted whenever there is an intermediate result, or if something goes wrong
    /// while executing the actual API request.
    /// 
    /// It should be used to handle progress information, and to implement a certain level of resilience.
    ///
    /// Sets the *delegate* property to the given value.
    pub fn delegate(mut self, new_value: &'a mut Delegate) -> CourseCourseWorkStudentSubmissionTurnInCall<'a, C, A> {
        self._delegate = Some(new_value);
        self
    }

    /// Set any additional parameter of the query string used in the request.
    /// It should be used to set parameters which are not yet available through their own
    /// setters.
    ///
    /// Please note that this method must not be used to set any of the known paramters
    /// which have their own setter method. If done anyway, the request will fail.
    ///
    /// # Additional Parameters
    ///
    /// * *upload_protocol* (query-string) - Upload protocol for media (e.g. "raw", "multipart").
    /// * *pp* (query-boolean) - Pretty-print response.
    /// * *prettyPrint* (query-boolean) - Returns response with indentations and line breaks.
    /// * *bearer_token* (query-string) - OAuth bearer token.
    /// * *access_token* (query-string) - OAuth access token.
    /// * *uploadType* (query-string) - Legacy upload protocol for media (e.g. "media", "multipart").
    /// * *quotaUser* (query-string) - Available to use for quota purposes for server-side applications. Can be any arbitrary string assigned to a user, but should not exceed 40 characters.
    /// * *callback* (query-string) - JSONP
    /// * *oauth_token* (query-string) - OAuth 2.0 token for the current user.
    /// * *key* (query-string) - API key. Your API key identifies your project and provides you with API access, quota, and reports. Required unless you provide an OAuth 2.0 token.
    /// * *fields* (query-string) - Selector specifying which fields to include in a partial response.
    /// * *alt* (query-string) - Data format for response.
    /// * *$.xgafv* (query-string) - V1 error format.
    pub fn param<T>(mut self, name: T, value: T) -> CourseCourseWorkStudentSubmissionTurnInCall<'a, C, A>
                                                        where T: AsRef<str> {
        self._additional_params.insert(name.as_ref().to_string(), value.as_ref().to_string());
        self
    }

    /// Identifies the authorization scope for the method you are building.
    ///
    /// Use this method to actively specify which scope should be used, instead the default `Scope` variant
    /// `Scope::CourseworkMe`.
    ///
    /// The `scope` will be added to a set of scopes. This is important as one can maintain access
    /// tokens for more than one scope.
    /// If `None` is specified, then all scopes will be removed and no default scope will be used either.
    /// In that case, you have to specify your API-key using the `key` parameter (see the `param()`
    /// function for details).
    ///
    /// Usually there is more than one suitable scope to authorize an operation, some of which may
    /// encompass more rights than others. For example, for listing resources, a *read-only* scope will be
    /// sufficient, a read-write scope will do as well.
    pub fn add_scope<T, S>(mut self, scope: T) -> CourseCourseWorkStudentSubmissionTurnInCall<'a, C, A>
                                                        where T: Into<Option<S>>,
                                                              S: AsRef<str> {
        match scope.into() {
          Some(scope) => self._scopes.insert(scope.as_ref().to_string(), ()),
          None => None,
        };
        self
    }
}


/// Modifies attachments of student submission.
/// 
/// Attachments may only be added to student submissions belonging to course
/// work objects with a `workType` of `ASSIGNMENT`.
/// 
/// This request must be made by the Developer Console project of the
/// [OAuth client ID](https://support.google.com/cloud/answer/6158849) used to
/// create the corresponding course work item.
/// 
/// This method returns the following error codes:
/// 
/// * `PERMISSION_DENIED` if the requesting user is not permitted to access the
/// requested course or course work, if the user is not permitted to modify
/// attachments on the requested student submission, or for
/// access errors.
/// * `INVALID_ARGUMENT` if the request is malformed.
/// * `NOT_FOUND` if the requested course, course work, or student submission
/// does not exist.
///
/// A builder for the *courseWork.studentSubmissions.modifyAttachments* method supported by a *course* resource.
/// It is not used directly, but through a `CourseMethods` instance.
///
/// # Example
///
/// Instantiate a resource method builder
///
/// ```test_harness,no_run
/// # extern crate hyper;
/// # extern crate hyper_rustls;
/// # extern crate yup_oauth2 as oauth2;
/// # extern crate google_classroom1 as classroom1;
/// use classroom1::ModifyAttachmentsRequest;
/// # #[test] fn egal() {
/// # use std::default::Default;
/// # use oauth2::{Authenticator, DefaultAuthenticatorDelegate, ApplicationSecret, MemoryStorage};
/// # use classroom1::Classroom;
/// 
/// # let secret: ApplicationSecret = Default::default();
/// # let auth = Authenticator::new(&secret, DefaultAuthenticatorDelegate,
/// #                               hyper::Client::with_connector(hyper::net::HttpsConnector::new(hyper_rustls::TlsClient::new())),
/// #                               <MemoryStorage as Default>::default(), None);
/// # let mut hub = Classroom::new(hyper::Client::with_connector(hyper::net::HttpsConnector::new(hyper_rustls::TlsClient::new())), auth);
/// // As the method needs a request, you would usually fill it with the desired information
/// // into the respective structure. Some of the parts shown here might not be applicable !
/// // Values shown here are possibly random and not representative !
/// let mut req = ModifyAttachmentsRequest::default();
/// 
/// // You can configure optional parameters by calling the respective setters at will, and
/// // execute the final call using `doit()`.
/// // Values shown here are possibly random and not representative !
/// let result = hub.courses().course_work_student_submissions_modify_attachments(req, "courseId", "courseWorkId", "id")
///              .doit();
/// # }
/// ```
pub struct CourseCourseWorkStudentSubmissionModifyAttachmentCall<'a, C, A>
    where C: 'a, A: 'a {

    hub: &'a Classroom<C, A>,
    _request: ModifyAttachmentsRequest,
    _course_id: String,
    _course_work_id: String,
    _id: String,
    _delegate: Option<&'a mut Delegate>,
    _additional_params: HashMap<String, String>,
    _scopes: BTreeMap<String, ()>
}

impl<'a, C, A> CallBuilder for CourseCourseWorkStudentSubmissionModifyAttachmentCall<'a, C, A> {}

impl<'a, C, A> CourseCourseWorkStudentSubmissionModifyAttachmentCall<'a, C, A> where C: BorrowMut<hyper::Client>, A: oauth2::GetToken {


    /// Perform the operation you have build so far.
    pub fn doit(mut self) -> Result<(hyper::client::Response, StudentSubmission)> {
        use std::io::{Read, Seek};
        use hyper::header::{ContentType, ContentLength, Authorization, Bearer, UserAgent, Location};
        let mut dd = DefaultDelegate;
        let mut dlg: &mut Delegate = match self._delegate {
            Some(d) => d,
            None => &mut dd
        };
        dlg.begin(MethodInfo { id: "classroom.courses.courseWork.studentSubmissions.modifyAttachments",
                               http_method: hyper::method::Method::Post });
        let mut params: Vec<(&str, String)> = Vec::with_capacity((6 + self._additional_params.len()));
        params.push(("courseId", self._course_id.to_string()));
        params.push(("courseWorkId", self._course_work_id.to_string()));
        params.push(("id", self._id.to_string()));
        for &field in ["alt", "courseId", "courseWorkId", "id"].iter() {
            if self._additional_params.contains_key(field) {
                dlg.finished(false);
                return Err(Error::FieldClash(field));
            }
        }
        for (name, value) in self._additional_params.iter() {
            params.push((&name, value.clone()));
        }

        params.push(("alt", "json".to_string()));

        let mut url = self.hub._base_url.clone() + "v1/courses/{courseId}/courseWork/{courseWorkId}/studentSubmissions/{id}:modifyAttachments";
        if self._scopes.len() == 0 {
            self._scopes.insert(Scope::CourseworkMe.as_ref().to_string(), ());
        }

        for &(find_this, param_name) in [("{courseId}", "courseId"), ("{courseWorkId}", "courseWorkId"), ("{id}", "id")].iter() {
            let mut replace_with: Option<&str> = None;
            for &(name, ref value) in params.iter() {
                if name == param_name {
                    replace_with = Some(value);
                    break;
                }
            }
            url = url.replace(find_this, replace_with.expect("to find substitution value in params"));
        }
        {
            let mut indices_for_removal: Vec<usize> = Vec::with_capacity(3);
            for param_name in ["id", "courseWorkId", "courseId"].iter() {
                if let Some(index) = params.iter().position(|t| &t.0 == param_name) {
                    indices_for_removal.push(index);
                }
            }
            for &index in indices_for_removal.iter() {
                params.remove(index);
            }
        }

        if params.len() > 0 {
            url.push('?');
            url.push_str(&url::form_urlencoded::serialize(params));
        }

        let mut json_mime_type = mime::Mime(mime::TopLevel::Application, mime::SubLevel::Json, Default::default());
        let mut request_value_reader =
            {
                let mut value = json::value::to_value(&self._request).expect("serde to work");
                remove_json_null_values(&mut value);
                let mut dst = io::Cursor::new(Vec::with_capacity(128));
                json::to_writer(&mut dst, &value).unwrap();
                dst
            };
        let request_size = request_value_reader.seek(io::SeekFrom::End(0)).unwrap();
        request_value_reader.seek(io::SeekFrom::Start(0)).unwrap();


        loop {
            let token = match self.hub.auth.borrow_mut().token(self._scopes.keys()) {
                Ok(token) => token,
                Err(err) => {
                    match  dlg.token(&*err) {
                        Some(token) => token,
                        None => {
                            dlg.finished(false);
                            return Err(Error::MissingToken(err))
                        }
                    }
                }
            };
            let auth_header = Authorization(Bearer { token: token.access_token });
            request_value_reader.seek(io::SeekFrom::Start(0)).unwrap();
            let mut req_result = {
                let mut client = &mut *self.hub.client.borrow_mut();
                let mut req = client.borrow_mut().request(hyper::method::Method::Post, &url)
                    .header(UserAgent(self.hub._user_agent.clone()))
                    .header(auth_header.clone())
                    .header(ContentType(json_mime_type.clone()))
                    .header(ContentLength(request_size as u64))
                    .body(&mut request_value_reader);

                dlg.pre_request();
                req.send()
            };

            match req_result {
                Err(err) => {
                    if let oauth2::Retry::After(d) = dlg.http_error(&err) {
                        sleep(d);
                        continue;
                    }
                    dlg.finished(false);
                    return Err(Error::HttpError(err))
                }
                Ok(mut res) => {
                    if !res.status.is_success() {
                        let mut json_err = String::new();
                        res.read_to_string(&mut json_err).unwrap();
                        if let oauth2::Retry::After(d) = dlg.http_failure(&res,
                                                              json::from_str(&json_err).ok(),
                                                              json::from_str(&json_err).ok()) {
                            sleep(d);
                            continue;
                        }
                        dlg.finished(false);
                        return match json::from_str::<ErrorResponse>(&json_err){
                            Err(_) => Err(Error::Failure(res)),
                            Ok(serr) => Err(Error::BadRequest(serr))
                        }
                    }
                    let result_value = {
                        let mut json_response = String::new();
                        res.read_to_string(&mut json_response).unwrap();
                        match json::from_str(&json_response) {
                            Ok(decoded) => (res, decoded),
                            Err(err) => {
                                dlg.response_json_decode_error(&json_response, &err);
                                return Err(Error::JsonDecodeError(json_response, err));
                            }
                        }
                    };

                    dlg.finished(true);
                    return Ok(result_value)
                }
            }
        }
    }


    ///
    /// Sets the *request* property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call,
    /// we provide this method for API completeness.
    pub fn request(mut self, new_value: ModifyAttachmentsRequest) -> CourseCourseWorkStudentSubmissionModifyAttachmentCall<'a, C, A> {
        self._request = new_value;
        self
    }
    /// Identifier of the course.
    /// This identifier can be either the Classroom-assigned identifier or an
    /// alias.
    ///
    /// Sets the *course id* path property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call,
    /// we provide this method for API completeness.
    pub fn course_id(mut self, new_value: &str) -> CourseCourseWorkStudentSubmissionModifyAttachmentCall<'a, C, A> {
        self._course_id = new_value.to_string();
        self
    }
    /// Identifier of the course work.
    ///
    /// Sets the *course work id* path property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call,
    /// we provide this method for API completeness.
    pub fn course_work_id(mut self, new_value: &str) -> CourseCourseWorkStudentSubmissionModifyAttachmentCall<'a, C, A> {
        self._course_work_id = new_value.to_string();
        self
    }
    /// Identifier of the student submission.
    ///
    /// Sets the *id* path property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call,
    /// we provide this method for API completeness.
    pub fn id(mut self, new_value: &str) -> CourseCourseWorkStudentSubmissionModifyAttachmentCall<'a, C, A> {
        self._id = new_value.to_string();
        self
    }
    /// The delegate implementation is consulted whenever there is an intermediate result, or if something goes wrong
    /// while executing the actual API request.
    /// 
    /// It should be used to handle progress information, and to implement a certain level of resilience.
    ///
    /// Sets the *delegate* property to the given value.
    pub fn delegate(mut self, new_value: &'a mut Delegate) -> CourseCourseWorkStudentSubmissionModifyAttachmentCall<'a, C, A> {
        self._delegate = Some(new_value);
        self
    }

    /// Set any additional parameter of the query string used in the request.
    /// It should be used to set parameters which are not yet available through their own
    /// setters.
    ///
    /// Please note that this method must not be used to set any of the known paramters
    /// which have their own setter method. If done anyway, the request will fail.
    ///
    /// # Additional Parameters
    ///
    /// * *upload_protocol* (query-string) - Upload protocol for media (e.g. "raw", "multipart").
    /// * *pp* (query-boolean) - Pretty-print response.
    /// * *prettyPrint* (query-boolean) - Returns response with indentations and line breaks.
    /// * *bearer_token* (query-string) - OAuth bearer token.
    /// * *access_token* (query-string) - OAuth access token.
    /// * *uploadType* (query-string) - Legacy upload protocol for media (e.g. "media", "multipart").
    /// * *quotaUser* (query-string) - Available to use for quota purposes for server-side applications. Can be any arbitrary string assigned to a user, but should not exceed 40 characters.
    /// * *callback* (query-string) - JSONP
    /// * *oauth_token* (query-string) - OAuth 2.0 token for the current user.
    /// * *key* (query-string) - API key. Your API key identifies your project and provides you with API access, quota, and reports. Required unless you provide an OAuth 2.0 token.
    /// * *fields* (query-string) - Selector specifying which fields to include in a partial response.
    /// * *alt* (query-string) - Data format for response.
    /// * *$.xgafv* (query-string) - V1 error format.
    pub fn param<T>(mut self, name: T, value: T) -> CourseCourseWorkStudentSubmissionModifyAttachmentCall<'a, C, A>
                                                        where T: AsRef<str> {
        self._additional_params.insert(name.as_ref().to_string(), value.as_ref().to_string());
        self
    }

    /// Identifies the authorization scope for the method you are building.
    ///
    /// Use this method to actively specify which scope should be used, instead the default `Scope` variant
    /// `Scope::CourseworkMe`.
    ///
    /// The `scope` will be added to a set of scopes. This is important as one can maintain access
    /// tokens for more than one scope.
    /// If `None` is specified, then all scopes will be removed and no default scope will be used either.
    /// In that case, you have to specify your API-key using the `key` parameter (see the `param()`
    /// function for details).
    ///
    /// Usually there is more than one suitable scope to authorize an operation, some of which may
    /// encompass more rights than others. For example, for listing resources, a *read-only* scope will be
    /// sufficient, a read-write scope will do as well.
    pub fn add_scope<T, S>(mut self, scope: T) -> CourseCourseWorkStudentSubmissionModifyAttachmentCall<'a, C, A>
                                                        where T: Into<Option<S>>,
                                                              S: AsRef<str> {
        match scope.into() {
          Some(scope) => self._scopes.insert(scope.as_ref().to_string(), ()),
          None => None,
        };
        self
    }
}


/// Returns a list of teachers of this course that the requester
/// is permitted to view.
/// 
/// This method returns the following error codes:
/// 
/// * `NOT_FOUND` if the course does not exist.
/// * `PERMISSION_DENIED` for access errors.
///
/// A builder for the *teachers.list* method supported by a *course* resource.
/// It is not used directly, but through a `CourseMethods` instance.
///
/// # Example
///
/// Instantiate a resource method builder
///
/// ```test_harness,no_run
/// # extern crate hyper;
/// # extern crate hyper_rustls;
/// # extern crate yup_oauth2 as oauth2;
/// # extern crate google_classroom1 as classroom1;
/// # #[test] fn egal() {
/// # use std::default::Default;
/// # use oauth2::{Authenticator, DefaultAuthenticatorDelegate, ApplicationSecret, MemoryStorage};
/// # use classroom1::Classroom;
/// 
/// # let secret: ApplicationSecret = Default::default();
/// # let auth = Authenticator::new(&secret, DefaultAuthenticatorDelegate,
/// #                               hyper::Client::with_connector(hyper::net::HttpsConnector::new(hyper_rustls::TlsClient::new())),
/// #                               <MemoryStorage as Default>::default(), None);
/// # let mut hub = Classroom::new(hyper::Client::with_connector(hyper::net::HttpsConnector::new(hyper_rustls::TlsClient::new())), auth);
/// // You can configure optional parameters by calling the respective setters at will, and
/// // execute the final call using `doit()`.
/// // Values shown here are possibly random and not representative !
/// let result = hub.courses().teachers_list("courseId")
///              .page_token("eirmod")
///              .page_size(-58)
///              .doit();
/// # }
/// ```
pub struct CourseTeacherListCall<'a, C, A>
    where C: 'a, A: 'a {

    hub: &'a Classroom<C, A>,
    _course_id: String,
    _page_token: Option<String>,
    _page_size: Option<i32>,
    _delegate: Option<&'a mut Delegate>,
    _additional_params: HashMap<String, String>,
    _scopes: BTreeMap<String, ()>
}

impl<'a, C, A> CallBuilder for CourseTeacherListCall<'a, C, A> {}

impl<'a, C, A> CourseTeacherListCall<'a, C, A> where C: BorrowMut<hyper::Client>, A: oauth2::GetToken {


    /// Perform the operation you have build so far.
    pub fn doit(mut self) -> Result<(hyper::client::Response, ListTeachersResponse)> {
        use std::io::{Read, Seek};
        use hyper::header::{ContentType, ContentLength, Authorization, Bearer, UserAgent, Location};
        let mut dd = DefaultDelegate;
        let mut dlg: &mut Delegate = match self._delegate {
            Some(d) => d,
            None => &mut dd
        };
        dlg.begin(MethodInfo { id: "classroom.courses.teachers.list",
                               http_method: hyper::method::Method::Get });
        let mut params: Vec<(&str, String)> = Vec::with_capacity((5 + self._additional_params.len()));
        params.push(("courseId", self._course_id.to_string()));
        if let Some(value) = self._page_token {
            params.push(("pageToken", value.to_string()));
        }
        if let Some(value) = self._page_size {
            params.push(("pageSize", value.to_string()));
        }
        for &field in ["alt", "courseId", "pageToken", "pageSize"].iter() {
            if self._additional_params.contains_key(field) {
                dlg.finished(false);
                return Err(Error::FieldClash(field));
            }
        }
        for (name, value) in self._additional_params.iter() {
            params.push((&name, value.clone()));
        }

        params.push(("alt", "json".to_string()));

        let mut url = self.hub._base_url.clone() + "v1/courses/{courseId}/teachers";
        if self._scopes.len() == 0 {
            self._scopes.insert(Scope::RosterReadonly.as_ref().to_string(), ());
        }

        for &(find_this, param_name) in [("{courseId}", "courseId")].iter() {
            let mut replace_with: Option<&str> = None;
            for &(name, ref value) in params.iter() {
                if name == param_name {
                    replace_with = Some(value);
                    break;
                }
            }
            url = url.replace(find_this, replace_with.expect("to find substitution value in params"));
        }
        {
            let mut indices_for_removal: Vec<usize> = Vec::with_capacity(1);
            for param_name in ["courseId"].iter() {
                if let Some(index) = params.iter().position(|t| &t.0 == param_name) {
                    indices_for_removal.push(index);
                }
            }
            for &index in indices_for_removal.iter() {
                params.remove(index);
            }
        }

        if params.len() > 0 {
            url.push('?');
            url.push_str(&url::form_urlencoded::serialize(params));
        }



        loop {
            let token = match self.hub.auth.borrow_mut().token(self._scopes.keys()) {
                Ok(token) => token,
                Err(err) => {
                    match  dlg.token(&*err) {
                        Some(token) => token,
                        None => {
                            dlg.finished(false);
                            return Err(Error::MissingToken(err))
                        }
                    }
                }
            };
            let auth_header = Authorization(Bearer { token: token.access_token });
            let mut req_result = {
                let mut client = &mut *self.hub.client.borrow_mut();
                let mut req = client.borrow_mut().request(hyper::method::Method::Get, &url)
                    .header(UserAgent(self.hub._user_agent.clone()))
                    .header(auth_header.clone());

                dlg.pre_request();
                req.send()
            };

            match req_result {
                Err(err) => {
                    if let oauth2::Retry::After(d) = dlg.http_error(&err) {
                        sleep(d);
                        continue;
                    }
                    dlg.finished(false);
                    return Err(Error::HttpError(err))
                }
                Ok(mut res) => {
                    if !res.status.is_success() {
                        let mut json_err = String::new();
                        res.read_to_string(&mut json_err).unwrap();
                        if let oauth2::Retry::After(d) = dlg.http_failure(&res,
                                                              json::from_str(&json_err).ok(),
                                                              json::from_str(&json_err).ok()) {
                            sleep(d);
                            continue;
                        }
                        dlg.finished(false);
                        return match json::from_str::<ErrorResponse>(&json_err){
                            Err(_) => Err(Error::Failure(res)),
                            Ok(serr) => Err(Error::BadRequest(serr))
                        }
                    }
                    let result_value = {
                        let mut json_response = String::new();
                        res.read_to_string(&mut json_response).unwrap();
                        match json::from_str(&json_response) {
                            Ok(decoded) => (res, decoded),
                            Err(err) => {
                                dlg.response_json_decode_error(&json_response, &err);
                                return Err(Error::JsonDecodeError(json_response, err));
                            }
                        }
                    };

                    dlg.finished(true);
                    return Ok(result_value)
                }
            }
        }
    }


    /// Identifier of the course.
    /// This identifier can be either the Classroom-assigned identifier or an
    /// alias.
    ///
    /// Sets the *course id* path property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call,
    /// we provide this method for API completeness.
    pub fn course_id(mut self, new_value: &str) -> CourseTeacherListCall<'a, C, A> {
        self._course_id = new_value.to_string();
        self
    }
    /// nextPageToken
    /// value returned from a previous
    /// list call, indicating that
    /// the subsequent page of results should be returned.
    /// 
    /// The list request must be
    /// otherwise identical to the one that resulted in this token.
    ///
    /// Sets the *page token* query property to the given value.
    pub fn page_token(mut self, new_value: &str) -> CourseTeacherListCall<'a, C, A> {
        self._page_token = Some(new_value.to_string());
        self
    }
    /// Maximum number of items to return. Zero means no maximum.
    /// 
    /// The server may return fewer than the specified number of results.
    ///
    /// Sets the *page size* query property to the given value.
    pub fn page_size(mut self, new_value: i32) -> CourseTeacherListCall<'a, C, A> {
        self._page_size = Some(new_value);
        self
    }
    /// The delegate implementation is consulted whenever there is an intermediate result, or if something goes wrong
    /// while executing the actual API request.
    /// 
    /// It should be used to handle progress information, and to implement a certain level of resilience.
    ///
    /// Sets the *delegate* property to the given value.
    pub fn delegate(mut self, new_value: &'a mut Delegate) -> CourseTeacherListCall<'a, C, A> {
        self._delegate = Some(new_value);
        self
    }

    /// Set any additional parameter of the query string used in the request.
    /// It should be used to set parameters which are not yet available through their own
    /// setters.
    ///
    /// Please note that this method must not be used to set any of the known paramters
    /// which have their own setter method. If done anyway, the request will fail.
    ///
    /// # Additional Parameters
    ///
    /// * *upload_protocol* (query-string) - Upload protocol for media (e.g. "raw", "multipart").
    /// * *pp* (query-boolean) - Pretty-print response.
    /// * *prettyPrint* (query-boolean) - Returns response with indentations and line breaks.
    /// * *bearer_token* (query-string) - OAuth bearer token.
    /// * *access_token* (query-string) - OAuth access token.
    /// * *uploadType* (query-string) - Legacy upload protocol for media (e.g. "media", "multipart").
    /// * *quotaUser* (query-string) - Available to use for quota purposes for server-side applications. Can be any arbitrary string assigned to a user, but should not exceed 40 characters.
    /// * *callback* (query-string) - JSONP
    /// * *oauth_token* (query-string) - OAuth 2.0 token for the current user.
    /// * *key* (query-string) - API key. Your API key identifies your project and provides you with API access, quota, and reports. Required unless you provide an OAuth 2.0 token.
    /// * *fields* (query-string) - Selector specifying which fields to include in a partial response.
    /// * *alt* (query-string) - Data format for response.
    /// * *$.xgafv* (query-string) - V1 error format.
    pub fn param<T>(mut self, name: T, value: T) -> CourseTeacherListCall<'a, C, A>
                                                        where T: AsRef<str> {
        self._additional_params.insert(name.as_ref().to_string(), value.as_ref().to_string());
        self
    }

    /// Identifies the authorization scope for the method you are building.
    ///
    /// Use this method to actively specify which scope should be used, instead the default `Scope` variant
    /// `Scope::RosterReadonly`.
    ///
    /// The `scope` will be added to a set of scopes. This is important as one can maintain access
    /// tokens for more than one scope.
    /// If `None` is specified, then all scopes will be removed and no default scope will be used either.
    /// In that case, you have to specify your API-key using the `key` parameter (see the `param()`
    /// function for details).
    ///
    /// Usually there is more than one suitable scope to authorize an operation, some of which may
    /// encompass more rights than others. For example, for listing resources, a *read-only* scope will be
    /// sufficient, a read-write scope will do as well.
    pub fn add_scope<T, S>(mut self, scope: T) -> CourseTeacherListCall<'a, C, A>
                                                        where T: Into<Option<S>>,
                                                              S: AsRef<str> {
        match scope.into() {
          Some(scope) => self._scopes.insert(scope.as_ref().to_string(), ()),
          None => None,
        };
        self
    }
}


/// Returns a student submission.
/// 
/// Returning a student submission transfers ownership of attached Drive
/// files to the student and may also update the submission state.
/// Unlike the Classroom application, returning a student submission does not
/// set assignedGrade to the draftGrade value.
/// 
/// Only a teacher of the course that contains the requested student submission
/// may call this method.
/// 
/// This request must be made by the Developer Console project of the
/// [OAuth client ID](https://support.google.com/cloud/answer/6158849) used to
/// create the corresponding course work item.
/// 
/// This method returns the following error codes:
/// 
/// * `PERMISSION_DENIED` if the requesting user is not permitted to access the
/// requested course or course work, return the requested student submission,
/// or for access errors.
/// * `INVALID_ARGUMENT` if the request is malformed.
/// * `NOT_FOUND` if the requested course, course work, or student submission
/// does not exist.
///
/// A builder for the *courseWork.studentSubmissions.return* method supported by a *course* resource.
/// It is not used directly, but through a `CourseMethods` instance.
///
/// # Example
///
/// Instantiate a resource method builder
///
/// ```test_harness,no_run
/// # extern crate hyper;
/// # extern crate hyper_rustls;
/// # extern crate yup_oauth2 as oauth2;
/// # extern crate google_classroom1 as classroom1;
/// use classroom1::ReturnStudentSubmissionRequest;
/// # #[test] fn egal() {
/// # use std::default::Default;
/// # use oauth2::{Authenticator, DefaultAuthenticatorDelegate, ApplicationSecret, MemoryStorage};
/// # use classroom1::Classroom;
/// 
/// # let secret: ApplicationSecret = Default::default();
/// # let auth = Authenticator::new(&secret, DefaultAuthenticatorDelegate,
/// #                               hyper::Client::with_connector(hyper::net::HttpsConnector::new(hyper_rustls::TlsClient::new())),
/// #                               <MemoryStorage as Default>::default(), None);
/// # let mut hub = Classroom::new(hyper::Client::with_connector(hyper::net::HttpsConnector::new(hyper_rustls::TlsClient::new())), auth);
/// // As the method needs a request, you would usually fill it with the desired information
/// // into the respective structure. Some of the parts shown here might not be applicable !
/// // Values shown here are possibly random and not representative !
/// let mut req = ReturnStudentSubmissionRequest::default();
/// 
/// // You can configure optional parameters by calling the respective setters at will, and
/// // execute the final call using `doit()`.
/// // Values shown here are possibly random and not representative !
/// let result = hub.courses().course_work_student_submissions_return(req, "courseId", "courseWorkId", "id")
///              .doit();
/// # }
/// ```
pub struct CourseCourseWorkStudentSubmissionReturnCall<'a, C, A>
    where C: 'a, A: 'a {

    hub: &'a Classroom<C, A>,
    _request: ReturnStudentSubmissionRequest,
    _course_id: String,
    _course_work_id: String,
    _id: String,
    _delegate: Option<&'a mut Delegate>,
    _additional_params: HashMap<String, String>,
    _scopes: BTreeMap<String, ()>
}

impl<'a, C, A> CallBuilder for CourseCourseWorkStudentSubmissionReturnCall<'a, C, A> {}

impl<'a, C, A> CourseCourseWorkStudentSubmissionReturnCall<'a, C, A> where C: BorrowMut<hyper::Client>, A: oauth2::GetToken {


    /// Perform the operation you have build so far.
    pub fn doit(mut self) -> Result<(hyper::client::Response, Empty)> {
        use std::io::{Read, Seek};
        use hyper::header::{ContentType, ContentLength, Authorization, Bearer, UserAgent, Location};
        let mut dd = DefaultDelegate;
        let mut dlg: &mut Delegate = match self._delegate {
            Some(d) => d,
            None => &mut dd
        };
        dlg.begin(MethodInfo { id: "classroom.courses.courseWork.studentSubmissions.return",
                               http_method: hyper::method::Method::Post });
        let mut params: Vec<(&str, String)> = Vec::with_capacity((6 + self._additional_params.len()));
        params.push(("courseId", self._course_id.to_string()));
        params.push(("courseWorkId", self._course_work_id.to_string()));
        params.push(("id", self._id.to_string()));
        for &field in ["alt", "courseId", "courseWorkId", "id"].iter() {
            if self._additional_params.contains_key(field) {
                dlg.finished(false);
                return Err(Error::FieldClash(field));
            }
        }
        for (name, value) in self._additional_params.iter() {
            params.push((&name, value.clone()));
        }

        params.push(("alt", "json".to_string()));

        let mut url = self.hub._base_url.clone() + "v1/courses/{courseId}/courseWork/{courseWorkId}/studentSubmissions/{id}:return";
        if self._scopes.len() == 0 {
            self._scopes.insert(Scope::CourseworkStudent.as_ref().to_string(), ());
        }

        for &(find_this, param_name) in [("{courseId}", "courseId"), ("{courseWorkId}", "courseWorkId"), ("{id}", "id")].iter() {
            let mut replace_with: Option<&str> = None;
            for &(name, ref value) in params.iter() {
                if name == param_name {
                    replace_with = Some(value);
                    break;
                }
            }
            url = url.replace(find_this, replace_with.expect("to find substitution value in params"));
        }
        {
            let mut indices_for_removal: Vec<usize> = Vec::with_capacity(3);
            for param_name in ["id", "courseWorkId", "courseId"].iter() {
                if let Some(index) = params.iter().position(|t| &t.0 == param_name) {
                    indices_for_removal.push(index);
                }
            }
            for &index in indices_for_removal.iter() {
                params.remove(index);
            }
        }

        if params.len() > 0 {
            url.push('?');
            url.push_str(&url::form_urlencoded::serialize(params));
        }

        let mut json_mime_type = mime::Mime(mime::TopLevel::Application, mime::SubLevel::Json, Default::default());
        let mut request_value_reader =
            {
                let mut value = json::value::to_value(&self._request).expect("serde to work");
                remove_json_null_values(&mut value);
                let mut dst = io::Cursor::new(Vec::with_capacity(128));
                json::to_writer(&mut dst, &value).unwrap();
                dst
            };
        let request_size = request_value_reader.seek(io::SeekFrom::End(0)).unwrap();
        request_value_reader.seek(io::SeekFrom::Start(0)).unwrap();


        loop {
            let token = match self.hub.auth.borrow_mut().token(self._scopes.keys()) {
                Ok(token) => token,
                Err(err) => {
                    match  dlg.token(&*err) {
                        Some(token) => token,
                        None => {
                            dlg.finished(false);
                            return Err(Error::MissingToken(err))
                        }
                    }
                }
            };
            let auth_header = Authorization(Bearer { token: token.access_token });
            request_value_reader.seek(io::SeekFrom::Start(0)).unwrap();
            let mut req_result = {
                let mut client = &mut *self.hub.client.borrow_mut();
                let mut req = client.borrow_mut().request(hyper::method::Method::Post, &url)
                    .header(UserAgent(self.hub._user_agent.clone()))
                    .header(auth_header.clone())
                    .header(ContentType(json_mime_type.clone()))
                    .header(ContentLength(request_size as u64))
                    .body(&mut request_value_reader);

                dlg.pre_request();
                req.send()
            };

            match req_result {
                Err(err) => {
                    if let oauth2::Retry::After(d) = dlg.http_error(&err) {
                        sleep(d);
                        continue;
                    }
                    dlg.finished(false);
                    return Err(Error::HttpError(err))
                }
                Ok(mut res) => {
                    if !res.status.is_success() {
                        let mut json_err = String::new();
                        res.read_to_string(&mut json_err).unwrap();
                        if let oauth2::Retry::After(d) = dlg.http_failure(&res,
                                                              json::from_str(&json_err).ok(),
                                                              json::from_str(&json_err).ok()) {
                            sleep(d);
                            continue;
                        }
                        dlg.finished(false);
                        return match json::from_str::<ErrorResponse>(&json_err){
                            Err(_) => Err(Error::Failure(res)),
                            Ok(serr) => Err(Error::BadRequest(serr))
                        }
                    }
                    let result_value = {
                        let mut json_response = String::new();
                        res.read_to_string(&mut json_response).unwrap();
                        match json::from_str(&json_response) {
                            Ok(decoded) => (res, decoded),
                            Err(err) => {
                                dlg.response_json_decode_error(&json_response, &err);
                                return Err(Error::JsonDecodeError(json_response, err));
                            }
                        }
                    };

                    dlg.finished(true);
                    return Ok(result_value)
                }
            }
        }
    }


    ///
    /// Sets the *request* property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call,
    /// we provide this method for API completeness.
    pub fn request(mut self, new_value: ReturnStudentSubmissionRequest) -> CourseCourseWorkStudentSubmissionReturnCall<'a, C, A> {
        self._request = new_value;
        self
    }
    /// Identifier of the course.
    /// This identifier can be either the Classroom-assigned identifier or an
    /// alias.
    ///
    /// Sets the *course id* path property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call,
    /// we provide this method for API completeness.
    pub fn course_id(mut self, new_value: &str) -> CourseCourseWorkStudentSubmissionReturnCall<'a, C, A> {
        self._course_id = new_value.to_string();
        self
    }
    /// Identifier of the course work.
    ///
    /// Sets the *course work id* path property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call,
    /// we provide this method for API completeness.
    pub fn course_work_id(mut self, new_value: &str) -> CourseCourseWorkStudentSubmissionReturnCall<'a, C, A> {
        self._course_work_id = new_value.to_string();
        self
    }
    /// Identifier of the student submission.
    ///
    /// Sets the *id* path property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call,
    /// we provide this method for API completeness.
    pub fn id(mut self, new_value: &str) -> CourseCourseWorkStudentSubmissionReturnCall<'a, C, A> {
        self._id = new_value.to_string();
        self
    }
    /// The delegate implementation is consulted whenever there is an intermediate result, or if something goes wrong
    /// while executing the actual API request.
    /// 
    /// It should be used to handle progress information, and to implement a certain level of resilience.
    ///
    /// Sets the *delegate* property to the given value.
    pub fn delegate(mut self, new_value: &'a mut Delegate) -> CourseCourseWorkStudentSubmissionReturnCall<'a, C, A> {
        self._delegate = Some(new_value);
        self
    }

    /// Set any additional parameter of the query string used in the request.
    /// It should be used to set parameters which are not yet available through their own
    /// setters.
    ///
    /// Please note that this method must not be used to set any of the known paramters
    /// which have their own setter method. If done anyway, the request will fail.
    ///
    /// # Additional Parameters
    ///
    /// * *upload_protocol* (query-string) - Upload protocol for media (e.g. "raw", "multipart").
    /// * *pp* (query-boolean) - Pretty-print response.
    /// * *prettyPrint* (query-boolean) - Returns response with indentations and line breaks.
    /// * *bearer_token* (query-string) - OAuth bearer token.
    /// * *access_token* (query-string) - OAuth access token.
    /// * *uploadType* (query-string) - Legacy upload protocol for media (e.g. "media", "multipart").
    /// * *quotaUser* (query-string) - Available to use for quota purposes for server-side applications. Can be any arbitrary string assigned to a user, but should not exceed 40 characters.
    /// * *callback* (query-string) - JSONP
    /// * *oauth_token* (query-string) - OAuth 2.0 token for the current user.
    /// * *key* (query-string) - API key. Your API key identifies your project and provides you with API access, quota, and reports. Required unless you provide an OAuth 2.0 token.
    /// * *fields* (query-string) - Selector specifying which fields to include in a partial response.
    /// * *alt* (query-string) - Data format for response.
    /// * *$.xgafv* (query-string) - V1 error format.
    pub fn param<T>(mut self, name: T, value: T) -> CourseCourseWorkStudentSubmissionReturnCall<'a, C, A>
                                                        where T: AsRef<str> {
        self._additional_params.insert(name.as_ref().to_string(), value.as_ref().to_string());
        self
    }

    /// Identifies the authorization scope for the method you are building.
    ///
    /// Use this method to actively specify which scope should be used, instead the default `Scope` variant
    /// `Scope::CourseworkStudent`.
    ///
    /// The `scope` will be added to a set of scopes. This is important as one can maintain access
    /// tokens for more than one scope.
    /// If `None` is specified, then all scopes will be removed and no default scope will be used either.
    /// In that case, you have to specify your API-key using the `key` parameter (see the `param()`
    /// function for details).
    ///
    /// Usually there is more than one suitable scope to authorize an operation, some of which may
    /// encompass more rights than others. For example, for listing resources, a *read-only* scope will be
    /// sufficient, a read-write scope will do as well.
    pub fn add_scope<T, S>(mut self, scope: T) -> CourseCourseWorkStudentSubmissionReturnCall<'a, C, A>
                                                        where T: Into<Option<S>>,
                                                              S: AsRef<str> {
        match scope.into() {
          Some(scope) => self._scopes.insert(scope.as_ref().to_string(), ()),
          None => None,
        };
        self
    }
}


/// Returns a list of aliases for a course.
/// 
/// This method returns the following error codes:
/// 
/// * `PERMISSION_DENIED` if the requesting user is not permitted to access the
/// course or for access errors.
/// * `NOT_FOUND` if the course does not exist.
///
/// A builder for the *aliases.list* method supported by a *course* resource.
/// It is not used directly, but through a `CourseMethods` instance.
///
/// # Example
///
/// Instantiate a resource method builder
///
/// ```test_harness,no_run
/// # extern crate hyper;
/// # extern crate hyper_rustls;
/// # extern crate yup_oauth2 as oauth2;
/// # extern crate google_classroom1 as classroom1;
/// # #[test] fn egal() {
/// # use std::default::Default;
/// # use oauth2::{Authenticator, DefaultAuthenticatorDelegate, ApplicationSecret, MemoryStorage};
/// # use classroom1::Classroom;
/// 
/// # let secret: ApplicationSecret = Default::default();
/// # let auth = Authenticator::new(&secret, DefaultAuthenticatorDelegate,
/// #                               hyper::Client::with_connector(hyper::net::HttpsConnector::new(hyper_rustls::TlsClient::new())),
/// #                               <MemoryStorage as Default>::default(), None);
/// # let mut hub = Classroom::new(hyper::Client::with_connector(hyper::net::HttpsConnector::new(hyper_rustls::TlsClient::new())), auth);
/// // You can configure optional parameters by calling the respective setters at will, and
/// // execute the final call using `doit()`.
/// // Values shown here are possibly random and not representative !
/// let result = hub.courses().aliases_list("courseId")
///              .page_token("ut")
///              .page_size(-16)
///              .doit();
/// # }
/// ```
pub struct CourseAliaseListCall<'a, C, A>
    where C: 'a, A: 'a {

    hub: &'a Classroom<C, A>,
    _course_id: String,
    _page_token: Option<String>,
    _page_size: Option<i32>,
    _delegate: Option<&'a mut Delegate>,
    _additional_params: HashMap<String, String>,
    _scopes: BTreeMap<String, ()>
}

impl<'a, C, A> CallBuilder for CourseAliaseListCall<'a, C, A> {}

impl<'a, C, A> CourseAliaseListCall<'a, C, A> where C: BorrowMut<hyper::Client>, A: oauth2::GetToken {


    /// Perform the operation you have build so far.
    pub fn doit(mut self) -> Result<(hyper::client::Response, ListCourseAliasesResponse)> {
        use std::io::{Read, Seek};
        use hyper::header::{ContentType, ContentLength, Authorization, Bearer, UserAgent, Location};
        let mut dd = DefaultDelegate;
        let mut dlg: &mut Delegate = match self._delegate {
            Some(d) => d,
            None => &mut dd
        };
        dlg.begin(MethodInfo { id: "classroom.courses.aliases.list",
                               http_method: hyper::method::Method::Get });
        let mut params: Vec<(&str, String)> = Vec::with_capacity((5 + self._additional_params.len()));
        params.push(("courseId", self._course_id.to_string()));
        if let Some(value) = self._page_token {
            params.push(("pageToken", value.to_string()));
        }
        if let Some(value) = self._page_size {
            params.push(("pageSize", value.to_string()));
        }
        for &field in ["alt", "courseId", "pageToken", "pageSize"].iter() {
            if self._additional_params.contains_key(field) {
                dlg.finished(false);
                return Err(Error::FieldClash(field));
            }
        }
        for (name, value) in self._additional_params.iter() {
            params.push((&name, value.clone()));
        }

        params.push(("alt", "json".to_string()));

        let mut url = self.hub._base_url.clone() + "v1/courses/{courseId}/aliases";
        if self._scopes.len() == 0 {
            self._scopes.insert(Scope::CourseReadonly.as_ref().to_string(), ());
        }

        for &(find_this, param_name) in [("{courseId}", "courseId")].iter() {
            let mut replace_with: Option<&str> = None;
            for &(name, ref value) in params.iter() {
                if name == param_name {
                    replace_with = Some(value);
                    break;
                }
            }
            url = url.replace(find_this, replace_with.expect("to find substitution value in params"));
        }
        {
            let mut indices_for_removal: Vec<usize> = Vec::with_capacity(1);
            for param_name in ["courseId"].iter() {
                if let Some(index) = params.iter().position(|t| &t.0 == param_name) {
                    indices_for_removal.push(index);
                }
            }
            for &index in indices_for_removal.iter() {
                params.remove(index);
            }
        }

        if params.len() > 0 {
            url.push('?');
            url.push_str(&url::form_urlencoded::serialize(params));
        }



        loop {
            let token = match self.hub.auth.borrow_mut().token(self._scopes.keys()) {
                Ok(token) => token,
                Err(err) => {
                    match  dlg.token(&*err) {
                        Some(token) => token,
                        None => {
                            dlg.finished(false);
                            return Err(Error::MissingToken(err))
                        }
                    }
                }
            };
            let auth_header = Authorization(Bearer { token: token.access_token });
            let mut req_result = {
                let mut client = &mut *self.hub.client.borrow_mut();
                let mut req = client.borrow_mut().request(hyper::method::Method::Get, &url)
                    .header(UserAgent(self.hub._user_agent.clone()))
                    .header(auth_header.clone());

                dlg.pre_request();
                req.send()
            };

            match req_result {
                Err(err) => {
                    if let oauth2::Retry::After(d) = dlg.http_error(&err) {
                        sleep(d);
                        continue;
                    }
                    dlg.finished(false);
                    return Err(Error::HttpError(err))
                }
                Ok(mut res) => {
                    if !res.status.is_success() {
                        let mut json_err = String::new();
                        res.read_to_string(&mut json_err).unwrap();
                        if let oauth2::Retry::After(d) = dlg.http_failure(&res,
                                                              json::from_str(&json_err).ok(),
                                                              json::from_str(&json_err).ok()) {
                            sleep(d);
                            continue;
                        }
                        dlg.finished(false);
                        return match json::from_str::<ErrorResponse>(&json_err){
                            Err(_) => Err(Error::Failure(res)),
                            Ok(serr) => Err(Error::BadRequest(serr))
                        }
                    }
                    let result_value = {
                        let mut json_response = String::new();
                        res.read_to_string(&mut json_response).unwrap();
                        match json::from_str(&json_response) {
                            Ok(decoded) => (res, decoded),
                            Err(err) => {
                                dlg.response_json_decode_error(&json_response, &err);
                                return Err(Error::JsonDecodeError(json_response, err));
                            }
                        }
                    };

                    dlg.finished(true);
                    return Ok(result_value)
                }
            }
        }
    }


    /// The identifier of the course.
    /// This identifier can be either the Classroom-assigned identifier or an
    /// alias.
    ///
    /// Sets the *course id* path property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call,
    /// we provide this method for API completeness.
    pub fn course_id(mut self, new_value: &str) -> CourseAliaseListCall<'a, C, A> {
        self._course_id = new_value.to_string();
        self
    }
    /// nextPageToken
    /// value returned from a previous
    /// list call,
    /// indicating that the subsequent page of results should be returned.
    /// 
    /// The list request
    /// must be otherwise identical to the one that resulted in this token.
    ///
    /// Sets the *page token* query property to the given value.
    pub fn page_token(mut self, new_value: &str) -> CourseAliaseListCall<'a, C, A> {
        self._page_token = Some(new_value.to_string());
        self
    }
    /// Maximum number of items to return. Zero or unspecified indicates that the
    /// server may assign a maximum.
    /// 
    /// The server may return fewer than the specified number of results.
    ///
    /// Sets the *page size* query property to the given value.
    pub fn page_size(mut self, new_value: i32) -> CourseAliaseListCall<'a, C, A> {
        self._page_size = Some(new_value);
        self
    }
    /// The delegate implementation is consulted whenever there is an intermediate result, or if something goes wrong
    /// while executing the actual API request.
    /// 
    /// It should be used to handle progress information, and to implement a certain level of resilience.
    ///
    /// Sets the *delegate* property to the given value.
    pub fn delegate(mut self, new_value: &'a mut Delegate) -> CourseAliaseListCall<'a, C, A> {
        self._delegate = Some(new_value);
        self
    }

    /// Set any additional parameter of the query string used in the request.
    /// It should be used to set parameters which are not yet available through their own
    /// setters.
    ///
    /// Please note that this method must not be used to set any of the known paramters
    /// which have their own setter method. If done anyway, the request will fail.
    ///
    /// # Additional Parameters
    ///
    /// * *upload_protocol* (query-string) - Upload protocol for media (e.g. "raw", "multipart").
    /// * *pp* (query-boolean) - Pretty-print response.
    /// * *prettyPrint* (query-boolean) - Returns response with indentations and line breaks.
    /// * *bearer_token* (query-string) - OAuth bearer token.
    /// * *access_token* (query-string) - OAuth access token.
    /// * *uploadType* (query-string) - Legacy upload protocol for media (e.g. "media", "multipart").
    /// * *quotaUser* (query-string) - Available to use for quota purposes for server-side applications. Can be any arbitrary string assigned to a user, but should not exceed 40 characters.
    /// * *callback* (query-string) - JSONP
    /// * *oauth_token* (query-string) - OAuth 2.0 token for the current user.
    /// * *key* (query-string) - API key. Your API key identifies your project and provides you with API access, quota, and reports. Required unless you provide an OAuth 2.0 token.
    /// * *fields* (query-string) - Selector specifying which fields to include in a partial response.
    /// * *alt* (query-string) - Data format for response.
    /// * *$.xgafv* (query-string) - V1 error format.
    pub fn param<T>(mut self, name: T, value: T) -> CourseAliaseListCall<'a, C, A>
                                                        where T: AsRef<str> {
        self._additional_params.insert(name.as_ref().to_string(), value.as_ref().to_string());
        self
    }

    /// Identifies the authorization scope for the method you are building.
    ///
    /// Use this method to actively specify which scope should be used, instead the default `Scope` variant
    /// `Scope::CourseReadonly`.
    ///
    /// The `scope` will be added to a set of scopes. This is important as one can maintain access
    /// tokens for more than one scope.
    /// If `None` is specified, then all scopes will be removed and no default scope will be used either.
    /// In that case, you have to specify your API-key using the `key` parameter (see the `param()`
    /// function for details).
    ///
    /// Usually there is more than one suitable scope to authorize an operation, some of which may
    /// encompass more rights than others. For example, for listing resources, a *read-only* scope will be
    /// sufficient, a read-write scope will do as well.
    pub fn add_scope<T, S>(mut self, scope: T) -> CourseAliaseListCall<'a, C, A>
                                                        where T: Into<Option<S>>,
                                                              S: AsRef<str> {
        match scope.into() {
          Some(scope) => self._scopes.insert(scope.as_ref().to_string(), ()),
          None => None,
        };
        self
    }
}


/// Creates course work.
/// 
/// The resulting course work (and corresponding student submissions) are
/// associated with the Developer Console project of the
/// [OAuth client ID](https://support.google.com/cloud/answer/6158849) used to
/// make the request. Classroom API requests to modify course work and student
/// submissions must be made with an OAuth client ID from the associated
/// Developer Console project.
/// 
/// This method returns the following error codes:
/// 
/// * `PERMISSION_DENIED` if the requesting user is not permitted to access the
/// requested course, create course work in the requested course, share a
/// Drive attachment, or for access errors.
/// * `INVALID_ARGUMENT` if the request is malformed.
/// * `NOT_FOUND` if the requested course does not exist.
/// * `FAILED_PRECONDITION` for the following request error:
///     * AttachmentNotVisible
///
/// A builder for the *courseWork.create* method supported by a *course* resource.
/// It is not used directly, but through a `CourseMethods` instance.
///
/// # Example
///
/// Instantiate a resource method builder
///
/// ```test_harness,no_run
/// # extern crate hyper;
/// # extern crate hyper_rustls;
/// # extern crate yup_oauth2 as oauth2;
/// # extern crate google_classroom1 as classroom1;
/// use classroom1::CourseWork;
/// # #[test] fn egal() {
/// # use std::default::Default;
/// # use oauth2::{Authenticator, DefaultAuthenticatorDelegate, ApplicationSecret, MemoryStorage};
/// # use classroom1::Classroom;
/// 
/// # let secret: ApplicationSecret = Default::default();
/// # let auth = Authenticator::new(&secret, DefaultAuthenticatorDelegate,
/// #                               hyper::Client::with_connector(hyper::net::HttpsConnector::new(hyper_rustls::TlsClient::new())),
/// #                               <MemoryStorage as Default>::default(), None);
/// # let mut hub = Classroom::new(hyper::Client::with_connector(hyper::net::HttpsConnector::new(hyper_rustls::TlsClient::new())), auth);
/// // As the method needs a request, you would usually fill it with the desired information
/// // into the respective structure. Some of the parts shown here might not be applicable !
/// // Values shown here are possibly random and not representative !
/// let mut req = CourseWork::default();
/// 
/// // You can configure optional parameters by calling the respective setters at will, and
/// // execute the final call using `doit()`.
/// // Values shown here are possibly random and not representative !
/// let result = hub.courses().course_work_create(req, "courseId")
///              .doit();
/// # }
/// ```
pub struct CourseCourseWorkCreateCall<'a, C, A>
    where C: 'a, A: 'a {

    hub: &'a Classroom<C, A>,
    _request: CourseWork,
    _course_id: String,
    _delegate: Option<&'a mut Delegate>,
    _additional_params: HashMap<String, String>,
    _scopes: BTreeMap<String, ()>
}

impl<'a, C, A> CallBuilder for CourseCourseWorkCreateCall<'a, C, A> {}

impl<'a, C, A> CourseCourseWorkCreateCall<'a, C, A> where C: BorrowMut<hyper::Client>, A: oauth2::GetToken {


    /// Perform the operation you have build so far.
    pub fn doit(mut self) -> Result<(hyper::client::Response, CourseWork)> {
        use std::io::{Read, Seek};
        use hyper::header::{ContentType, ContentLength, Authorization, Bearer, UserAgent, Location};
        let mut dd = DefaultDelegate;
        let mut dlg: &mut Delegate = match self._delegate {
            Some(d) => d,
            None => &mut dd
        };
        dlg.begin(MethodInfo { id: "classroom.courses.courseWork.create",
                               http_method: hyper::method::Method::Post });
        let mut params: Vec<(&str, String)> = Vec::with_capacity((4 + self._additional_params.len()));
        params.push(("courseId", self._course_id.to_string()));
        for &field in ["alt", "courseId"].iter() {
            if self._additional_params.contains_key(field) {
                dlg.finished(false);
                return Err(Error::FieldClash(field));
            }
        }
        for (name, value) in self._additional_params.iter() {
            params.push((&name, value.clone()));
        }

        params.push(("alt", "json".to_string()));

        let mut url = self.hub._base_url.clone() + "v1/courses/{courseId}/courseWork";
        if self._scopes.len() == 0 {
            self._scopes.insert(Scope::CourseworkStudent.as_ref().to_string(), ());
        }

        for &(find_this, param_name) in [("{courseId}", "courseId")].iter() {
            let mut replace_with: Option<&str> = None;
            for &(name, ref value) in params.iter() {
                if name == param_name {
                    replace_with = Some(value);
                    break;
                }
            }
            url = url.replace(find_this, replace_with.expect("to find substitution value in params"));
        }
        {
            let mut indices_for_removal: Vec<usize> = Vec::with_capacity(1);
            for param_name in ["courseId"].iter() {
                if let Some(index) = params.iter().position(|t| &t.0 == param_name) {
                    indices_for_removal.push(index);
                }
            }
            for &index in indices_for_removal.iter() {
                params.remove(index);
            }
        }

        if params.len() > 0 {
            url.push('?');
            url.push_str(&url::form_urlencoded::serialize(params));
        }

        let mut json_mime_type = mime::Mime(mime::TopLevel::Application, mime::SubLevel::Json, Default::default());
        let mut request_value_reader =
            {
                let mut value = json::value::to_value(&self._request).expect("serde to work");
                remove_json_null_values(&mut value);
                let mut dst = io::Cursor::new(Vec::with_capacity(128));
                json::to_writer(&mut dst, &value).unwrap();
                dst
            };
        let request_size = request_value_reader.seek(io::SeekFrom::End(0)).unwrap();
        request_value_reader.seek(io::SeekFrom::Start(0)).unwrap();


        loop {
            let token = match self.hub.auth.borrow_mut().token(self._scopes.keys()) {
                Ok(token) => token,
                Err(err) => {
                    match  dlg.token(&*err) {
                        Some(token) => token,
                        None => {
                            dlg.finished(false);
                            return Err(Error::MissingToken(err))
                        }
                    }
                }
            };
            let auth_header = Authorization(Bearer { token: token.access_token });
            request_value_reader.seek(io::SeekFrom::Start(0)).unwrap();
            let mut req_result = {
                let mut client = &mut *self.hub.client.borrow_mut();
                let mut req = client.borrow_mut().request(hyper::method::Method::Post, &url)
                    .header(UserAgent(self.hub._user_agent.clone()))
                    .header(auth_header.clone())
                    .header(ContentType(json_mime_type.clone()))
                    .header(ContentLength(request_size as u64))
                    .body(&mut request_value_reader);

                dlg.pre_request();
                req.send()
            };

            match req_result {
                Err(err) => {
                    if let oauth2::Retry::After(d) = dlg.http_error(&err) {
                        sleep(d);
                        continue;
                    }
                    dlg.finished(false);
                    return Err(Error::HttpError(err))
                }
                Ok(mut res) => {
                    if !res.status.is_success() {
                        let mut json_err = String::new();
                        res.read_to_string(&mut json_err).unwrap();
                        if let oauth2::Retry::After(d) = dlg.http_failure(&res,
                                                              json::from_str(&json_err).ok(),
                                                              json::from_str(&json_err).ok()) {
                            sleep(d);
                            continue;
                        }
                        dlg.finished(false);
                        return match json::from_str::<ErrorResponse>(&json_err){
                            Err(_) => Err(Error::Failure(res)),
                            Ok(serr) => Err(Error::BadRequest(serr))
                        }
                    }
                    let result_value = {
                        let mut json_response = String::new();
                        res.read_to_string(&mut json_response).unwrap();
                        match json::from_str(&json_response) {
                            Ok(decoded) => (res, decoded),
                            Err(err) => {
                                dlg.response_json_decode_error(&json_response, &err);
                                return Err(Error::JsonDecodeError(json_response, err));
                            }
                        }
                    };

                    dlg.finished(true);
                    return Ok(result_value)
                }
            }
        }
    }


    ///
    /// Sets the *request* property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call,
    /// we provide this method for API completeness.
    pub fn request(mut self, new_value: CourseWork) -> CourseCourseWorkCreateCall<'a, C, A> {
        self._request = new_value;
        self
    }
    /// Identifier of the course.
    /// This identifier can be either the Classroom-assigned identifier or an
    /// alias.
    ///
    /// Sets the *course id* path property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call,
    /// we provide this method for API completeness.
    pub fn course_id(mut self, new_value: &str) -> CourseCourseWorkCreateCall<'a, C, A> {
        self._course_id = new_value.to_string();
        self
    }
    /// The delegate implementation is consulted whenever there is an intermediate result, or if something goes wrong
    /// while executing the actual API request.
    /// 
    /// It should be used to handle progress information, and to implement a certain level of resilience.
    ///
    /// Sets the *delegate* property to the given value.
    pub fn delegate(mut self, new_value: &'a mut Delegate) -> CourseCourseWorkCreateCall<'a, C, A> {
        self._delegate = Some(new_value);
        self
    }

    /// Set any additional parameter of the query string used in the request.
    /// It should be used to set parameters which are not yet available through their own
    /// setters.
    ///
    /// Please note that this method must not be used to set any of the known paramters
    /// which have their own setter method. If done anyway, the request will fail.
    ///
    /// # Additional Parameters
    ///
    /// * *upload_protocol* (query-string) - Upload protocol for media (e.g. "raw", "multipart").
    /// * *pp* (query-boolean) - Pretty-print response.
    /// * *prettyPrint* (query-boolean) - Returns response with indentations and line breaks.
    /// * *bearer_token* (query-string) - OAuth bearer token.
    /// * *access_token* (query-string) - OAuth access token.
    /// * *uploadType* (query-string) - Legacy upload protocol for media (e.g. "media", "multipart").
    /// * *quotaUser* (query-string) - Available to use for quota purposes for server-side applications. Can be any arbitrary string assigned to a user, but should not exceed 40 characters.
    /// * *callback* (query-string) - JSONP
    /// * *oauth_token* (query-string) - OAuth 2.0 token for the current user.
    /// * *key* (query-string) - API key. Your API key identifies your project and provides you with API access, quota, and reports. Required unless you provide an OAuth 2.0 token.
    /// * *fields* (query-string) - Selector specifying which fields to include in a partial response.
    /// * *alt* (query-string) - Data format for response.
    /// * *$.xgafv* (query-string) - V1 error format.
    pub fn param<T>(mut self, name: T, value: T) -> CourseCourseWorkCreateCall<'a, C, A>
                                                        where T: AsRef<str> {
        self._additional_params.insert(name.as_ref().to_string(), value.as_ref().to_string());
        self
    }

    /// Identifies the authorization scope for the method you are building.
    ///
    /// Use this method to actively specify which scope should be used, instead the default `Scope` variant
    /// `Scope::CourseworkStudent`.
    ///
    /// The `scope` will be added to a set of scopes. This is important as one can maintain access
    /// tokens for more than one scope.
    /// If `None` is specified, then all scopes will be removed and no default scope will be used either.
    /// In that case, you have to specify your API-key using the `key` parameter (see the `param()`
    /// function for details).
    ///
    /// Usually there is more than one suitable scope to authorize an operation, some of which may
    /// encompass more rights than others. For example, for listing resources, a *read-only* scope will be
    /// sufficient, a read-write scope will do as well.
    pub fn add_scope<T, S>(mut self, scope: T) -> CourseCourseWorkCreateCall<'a, C, A>
                                                        where T: Into<Option<S>>,
                                                              S: AsRef<str> {
        match scope.into() {
          Some(scope) => self._scopes.insert(scope.as_ref().to_string(), ()),
          None => None,
        };
        self
    }
}


/// Returns a list of courses that the requesting user is permitted to view,
/// restricted to those that match the request.
/// 
/// This method returns the following error codes:
/// 
/// * `PERMISSION_DENIED` for access errors.
/// * `INVALID_ARGUMENT` if the query argument is malformed.
/// * `NOT_FOUND` if any users specified in the query arguments do not exist.
///
/// A builder for the *list* method supported by a *course* resource.
/// It is not used directly, but through a `CourseMethods` instance.
///
/// # Example
///
/// Instantiate a resource method builder
///
/// ```test_harness,no_run
/// # extern crate hyper;
/// # extern crate hyper_rustls;
/// # extern crate yup_oauth2 as oauth2;
/// # extern crate google_classroom1 as classroom1;
/// # #[test] fn egal() {
/// # use std::default::Default;
/// # use oauth2::{Authenticator, DefaultAuthenticatorDelegate, ApplicationSecret, MemoryStorage};
/// # use classroom1::Classroom;
/// 
/// # let secret: ApplicationSecret = Default::default();
/// # let auth = Authenticator::new(&secret, DefaultAuthenticatorDelegate,
/// #                               hyper::Client::with_connector(hyper::net::HttpsConnector::new(hyper_rustls::TlsClient::new())),
/// #                               <MemoryStorage as Default>::default(), None);
/// # let mut hub = Classroom::new(hyper::Client::with_connector(hyper::net::HttpsConnector::new(hyper_rustls::TlsClient::new())), auth);
/// // You can configure optional parameters by calling the respective setters at will, and
/// // execute the final call using `doit()`.
/// // Values shown here are possibly random and not representative !
/// let result = hub.courses().list()
///              .teacher_id("dolor")
///              .student_id("dolor")
///              .page_token("dolor")
///              .page_size(-23)
///              .add_course_states("consetetur")
///              .doit();
/// # }
/// ```
pub struct CourseListCall<'a, C, A>
    where C: 'a, A: 'a {

    hub: &'a Classroom<C, A>,
    _teacher_id: Option<String>,
    _student_id: Option<String>,
    _page_token: Option<String>,
    _page_size: Option<i32>,
    _course_states: Vec<String>,
    _delegate: Option<&'a mut Delegate>,
    _additional_params: HashMap<String, String>,
    _scopes: BTreeMap<String, ()>
}

impl<'a, C, A> CallBuilder for CourseListCall<'a, C, A> {}

impl<'a, C, A> CourseListCall<'a, C, A> where C: BorrowMut<hyper::Client>, A: oauth2::GetToken {


    /// Perform the operation you have build so far.
    pub fn doit(mut self) -> Result<(hyper::client::Response, ListCoursesResponse)> {
        use std::io::{Read, Seek};
        use hyper::header::{ContentType, ContentLength, Authorization, Bearer, UserAgent, Location};
        let mut dd = DefaultDelegate;
        let mut dlg: &mut Delegate = match self._delegate {
            Some(d) => d,
            None => &mut dd
        };
        dlg.begin(MethodInfo { id: "classroom.courses.list",
                               http_method: hyper::method::Method::Get });
        let mut params: Vec<(&str, String)> = Vec::with_capacity((7 + self._additional_params.len()));
        if let Some(value) = self._teacher_id {
            params.push(("teacherId", value.to_string()));
        }
        if let Some(value) = self._student_id {
            params.push(("studentId", value.to_string()));
        }
        if let Some(value) = self._page_token {
            params.push(("pageToken", value.to_string()));
        }
        if let Some(value) = self._page_size {
            params.push(("pageSize", value.to_string()));
        }
        if self._course_states.len() > 0 {
            for f in self._course_states.iter() {
                params.push(("courseStates", f.to_string()));
            }
        }
        for &field in ["alt", "teacherId", "studentId", "pageToken", "pageSize", "courseStates"].iter() {
            if self._additional_params.contains_key(field) {
                dlg.finished(false);
                return Err(Error::FieldClash(field));
            }
        }
        for (name, value) in self._additional_params.iter() {
            params.push((&name, value.clone()));
        }

        params.push(("alt", "json".to_string()));

        let mut url = self.hub._base_url.clone() + "v1/courses";
        if self._scopes.len() == 0 {
            self._scopes.insert(Scope::CourseReadonly.as_ref().to_string(), ());
        }


        if params.len() > 0 {
            url.push('?');
            url.push_str(&url::form_urlencoded::serialize(params));
        }



        loop {
            let token = match self.hub.auth.borrow_mut().token(self._scopes.keys()) {
                Ok(token) => token,
                Err(err) => {
                    match  dlg.token(&*err) {
                        Some(token) => token,
                        None => {
                            dlg.finished(false);
                            return Err(Error::MissingToken(err))
                        }
                    }
                }
            };
            let auth_header = Authorization(Bearer { token: token.access_token });
            let mut req_result = {
                let mut client = &mut *self.hub.client.borrow_mut();
                let mut req = client.borrow_mut().request(hyper::method::Method::Get, &url)
                    .header(UserAgent(self.hub._user_agent.clone()))
                    .header(auth_header.clone());

                dlg.pre_request();
                req.send()
            };

            match req_result {
                Err(err) => {
                    if let oauth2::Retry::After(d) = dlg.http_error(&err) {
                        sleep(d);
                        continue;
                    }
                    dlg.finished(false);
                    return Err(Error::HttpError(err))
                }
                Ok(mut res) => {
                    if !res.status.is_success() {
                        let mut json_err = String::new();
                        res.read_to_string(&mut json_err).unwrap();
                        if let oauth2::Retry::After(d) = dlg.http_failure(&res,
                                                              json::from_str(&json_err).ok(),
                                                              json::from_str(&json_err).ok()) {
                            sleep(d);
                            continue;
                        }
                        dlg.finished(false);
                        return match json::from_str::<ErrorResponse>(&json_err){
                            Err(_) => Err(Error::Failure(res)),
                            Ok(serr) => Err(Error::BadRequest(serr))
                        }
                    }
                    let result_value = {
                        let mut json_response = String::new();
                        res.read_to_string(&mut json_response).unwrap();
                        match json::from_str(&json_response) {
                            Ok(decoded) => (res, decoded),
                            Err(err) => {
                                dlg.response_json_decode_error(&json_response, &err);
                                return Err(Error::JsonDecodeError(json_response, err));
                            }
                        }
                    };

                    dlg.finished(true);
                    return Ok(result_value)
                }
            }
        }
    }


    /// Restricts returned courses to those having a teacher with the specified
    /// identifier. The identifier can be one of the following:
    /// 
    /// * the numeric identifier for the user
    /// * the email address of the user
    /// * the string literal `"me"`, indicating the requesting user
    ///
    /// Sets the *teacher id* query property to the given value.
    pub fn teacher_id(mut self, new_value: &str) -> CourseListCall<'a, C, A> {
        self._teacher_id = Some(new_value.to_string());
        self
    }
    /// Restricts returned courses to those having a student with the specified
    /// identifier. The identifier can be one of the following:
    /// 
    /// * the numeric identifier for the user
    /// * the email address of the user
    /// * the string literal `"me"`, indicating the requesting user
    ///
    /// Sets the *student id* query property to the given value.
    pub fn student_id(mut self, new_value: &str) -> CourseListCall<'a, C, A> {
        self._student_id = Some(new_value.to_string());
        self
    }
    /// nextPageToken
    /// value returned from a previous
    /// list call,
    /// indicating that the subsequent page of results should be returned.
    /// 
    /// The list request must be
    /// otherwise identical to the one that resulted in this token.
    ///
    /// Sets the *page token* query property to the given value.
    pub fn page_token(mut self, new_value: &str) -> CourseListCall<'a, C, A> {
        self._page_token = Some(new_value.to_string());
        self
    }
    /// Maximum number of items to return. Zero or unspecified indicates that the
    /// server may assign a maximum.
    /// 
    /// The server may return fewer than the specified number of results.
    ///
    /// Sets the *page size* query property to the given value.
    pub fn page_size(mut self, new_value: i32) -> CourseListCall<'a, C, A> {
        self._page_size = Some(new_value);
        self
    }
    /// Restricts returned courses to those in one of the specified states
    /// The default value is ACTIVE, ARCHIVED, PROVISIONED, DECLINED.
    ///
    /// Append the given value to the *course states* query property.
    /// Each appended value will retain its original ordering and be '/'-separated in the URL's parameters.
    pub fn add_course_states(mut self, new_value: &str) -> CourseListCall<'a, C, A> {
        self._course_states.push(new_value.to_string());
        self
    }
    /// The delegate implementation is consulted whenever there is an intermediate result, or if something goes wrong
    /// while executing the actual API request.
    /// 
    /// It should be used to handle progress information, and to implement a certain level of resilience.
    ///
    /// Sets the *delegate* property to the given value.
    pub fn delegate(mut self, new_value: &'a mut Delegate) -> CourseListCall<'a, C, A> {
        self._delegate = Some(new_value);
        self
    }

    /// Set any additional parameter of the query string used in the request.
    /// It should be used to set parameters which are not yet available through their own
    /// setters.
    ///
    /// Please note that this method must not be used to set any of the known paramters
    /// which have their own setter method. If done anyway, the request will fail.
    ///
    /// # Additional Parameters
    ///
    /// * *upload_protocol* (query-string) - Upload protocol for media (e.g. "raw", "multipart").
    /// * *pp* (query-boolean) - Pretty-print response.
    /// * *prettyPrint* (query-boolean) - Returns response with indentations and line breaks.
    /// * *bearer_token* (query-string) - OAuth bearer token.
    /// * *access_token* (query-string) - OAuth access token.
    /// * *uploadType* (query-string) - Legacy upload protocol for media (e.g. "media", "multipart").
    /// * *quotaUser* (query-string) - Available to use for quota purposes for server-side applications. Can be any arbitrary string assigned to a user, but should not exceed 40 characters.
    /// * *callback* (query-string) - JSONP
    /// * *oauth_token* (query-string) - OAuth 2.0 token for the current user.
    /// * *key* (query-string) - API key. Your API key identifies your project and provides you with API access, quota, and reports. Required unless you provide an OAuth 2.0 token.
    /// * *fields* (query-string) - Selector specifying which fields to include in a partial response.
    /// * *alt* (query-string) - Data format for response.
    /// * *$.xgafv* (query-string) - V1 error format.
    pub fn param<T>(mut self, name: T, value: T) -> CourseListCall<'a, C, A>
                                                        where T: AsRef<str> {
        self._additional_params.insert(name.as_ref().to_string(), value.as_ref().to_string());
        self
    }

    /// Identifies the authorization scope for the method you are building.
    ///
    /// Use this method to actively specify which scope should be used, instead the default `Scope` variant
    /// `Scope::CourseReadonly`.
    ///
    /// The `scope` will be added to a set of scopes. This is important as one can maintain access
    /// tokens for more than one scope.
    /// If `None` is specified, then all scopes will be removed and no default scope will be used either.
    /// In that case, you have to specify your API-key using the `key` parameter (see the `param()`
    /// function for details).
    ///
    /// Usually there is more than one suitable scope to authorize an operation, some of which may
    /// encompass more rights than others. For example, for listing resources, a *read-only* scope will be
    /// sufficient, a read-write scope will do as well.
    pub fn add_scope<T, S>(mut self, scope: T) -> CourseListCall<'a, C, A>
                                                        where T: Into<Option<S>>,
                                                              S: AsRef<str> {
        match scope.into() {
          Some(scope) => self._scopes.insert(scope.as_ref().to_string(), ()),
          None => None,
        };
        self
    }
}


/// Reclaims a student submission on behalf of the student that owns it.
/// 
/// Reclaiming a student submission transfers ownership of attached Drive
/// files to the student and update the submission state.
/// 
/// Only the student that owns the requested student submission may call this
/// method, and only for a student submission that has been turned in.
/// 
/// This request must be made by the Developer Console project of the
/// [OAuth client ID](https://support.google.com/cloud/answer/6158849) used to
/// create the corresponding course work item.
/// 
/// This method returns the following error codes:
/// 
/// * `PERMISSION_DENIED` if the requesting user is not permitted to access the
/// requested course or course work, unsubmit the requested student submission,
/// or for access errors.
/// * `FAILED_PRECONDITION` if the student submission has not been turned in.
/// * `INVALID_ARGUMENT` if the request is malformed.
/// * `NOT_FOUND` if the requested course, course work, or student submission
/// does not exist.
///
/// A builder for the *courseWork.studentSubmissions.reclaim* method supported by a *course* resource.
/// It is not used directly, but through a `CourseMethods` instance.
///
/// # Example
///
/// Instantiate a resource method builder
///
/// ```test_harness,no_run
/// # extern crate hyper;
/// # extern crate hyper_rustls;
/// # extern crate yup_oauth2 as oauth2;
/// # extern crate google_classroom1 as classroom1;
/// use classroom1::ReclaimStudentSubmissionRequest;
/// # #[test] fn egal() {
/// # use std::default::Default;
/// # use oauth2::{Authenticator, DefaultAuthenticatorDelegate, ApplicationSecret, MemoryStorage};
/// # use classroom1::Classroom;
/// 
/// # let secret: ApplicationSecret = Default::default();
/// # let auth = Authenticator::new(&secret, DefaultAuthenticatorDelegate,
/// #                               hyper::Client::with_connector(hyper::net::HttpsConnector::new(hyper_rustls::TlsClient::new())),
/// #                               <MemoryStorage as Default>::default(), None);
/// # let mut hub = Classroom::new(hyper::Client::with_connector(hyper::net::HttpsConnector::new(hyper_rustls::TlsClient::new())), auth);
/// // As the method needs a request, you would usually fill it with the desired information
/// // into the respective structure. Some of the parts shown here might not be applicable !
/// // Values shown here are possibly random and not representative !
/// let mut req = ReclaimStudentSubmissionRequest::default();
/// 
/// // You can configure optional parameters by calling the respective setters at will, and
/// // execute the final call using `doit()`.
/// // Values shown here are possibly random and not representative !
/// let result = hub.courses().course_work_student_submissions_reclaim(req, "courseId", "courseWorkId", "id")
///              .doit();
/// # }
/// ```
pub struct CourseCourseWorkStudentSubmissionReclaimCall<'a, C, A>
    where C: 'a, A: 'a {

    hub: &'a Classroom<C, A>,
    _request: ReclaimStudentSubmissionRequest,
    _course_id: String,
    _course_work_id: String,
    _id: String,
    _delegate: Option<&'a mut Delegate>,
    _additional_params: HashMap<String, String>,
    _scopes: BTreeMap<String, ()>
}

impl<'a, C, A> CallBuilder for CourseCourseWorkStudentSubmissionReclaimCall<'a, C, A> {}

impl<'a, C, A> CourseCourseWorkStudentSubmissionReclaimCall<'a, C, A> where C: BorrowMut<hyper::Client>, A: oauth2::GetToken {


    /// Perform the operation you have build so far.
    pub fn doit(mut self) -> Result<(hyper::client::Response, Empty)> {
        use std::io::{Read, Seek};
        use hyper::header::{ContentType, ContentLength, Authorization, Bearer, UserAgent, Location};
        let mut dd = DefaultDelegate;
        let mut dlg: &mut Delegate = match self._delegate {
            Some(d) => d,
            None => &mut dd
        };
        dlg.begin(MethodInfo { id: "classroom.courses.courseWork.studentSubmissions.reclaim",
                               http_method: hyper::method::Method::Post });
        let mut params: Vec<(&str, String)> = Vec::with_capacity((6 + self._additional_params.len()));
        params.push(("courseId", self._course_id.to_string()));
        params.push(("courseWorkId", self._course_work_id.to_string()));
        params.push(("id", self._id.to_string()));
        for &field in ["alt", "courseId", "courseWorkId", "id"].iter() {
            if self._additional_params.contains_key(field) {
                dlg.finished(false);
                return Err(Error::FieldClash(field));
            }
        }
        for (name, value) in self._additional_params.iter() {
            params.push((&name, value.clone()));
        }

        params.push(("alt", "json".to_string()));

        let mut url = self.hub._base_url.clone() + "v1/courses/{courseId}/courseWork/{courseWorkId}/studentSubmissions/{id}:reclaim";
        if self._scopes.len() == 0 {
            self._scopes.insert(Scope::CourseworkMe.as_ref().to_string(), ());
        }

        for &(find_this, param_name) in [("{courseId}", "courseId"), ("{courseWorkId}", "courseWorkId"), ("{id}", "id")].iter() {
            let mut replace_with: Option<&str> = None;
            for &(name, ref value) in params.iter() {
                if name == param_name {
                    replace_with = Some(value);
                    break;
                }
            }
            url = url.replace(find_this, replace_with.expect("to find substitution value in params"));
        }
        {
            let mut indices_for_removal: Vec<usize> = Vec::with_capacity(3);
            for param_name in ["id", "courseWorkId", "courseId"].iter() {
                if let Some(index) = params.iter().position(|t| &t.0 == param_name) {
                    indices_for_removal.push(index);
                }
            }
            for &index in indices_for_removal.iter() {
                params.remove(index);
            }
        }

        if params.len() > 0 {
            url.push('?');
            url.push_str(&url::form_urlencoded::serialize(params));
        }

        let mut json_mime_type = mime::Mime(mime::TopLevel::Application, mime::SubLevel::Json, Default::default());
        let mut request_value_reader =
            {
                let mut value = json::value::to_value(&self._request).expect("serde to work");
                remove_json_null_values(&mut value);
                let mut dst = io::Cursor::new(Vec::with_capacity(128));
                json::to_writer(&mut dst, &value).unwrap();
                dst
            };
        let request_size = request_value_reader.seek(io::SeekFrom::End(0)).unwrap();
        request_value_reader.seek(io::SeekFrom::Start(0)).unwrap();


        loop {
            let token = match self.hub.auth.borrow_mut().token(self._scopes.keys()) {
                Ok(token) => token,
                Err(err) => {
                    match  dlg.token(&*err) {
                        Some(token) => token,
                        None => {
                            dlg.finished(false);
                            return Err(Error::MissingToken(err))
                        }
                    }
                }
            };
            let auth_header = Authorization(Bearer { token: token.access_token });
            request_value_reader.seek(io::SeekFrom::Start(0)).unwrap();
            let mut req_result = {
                let mut client = &mut *self.hub.client.borrow_mut();
                let mut req = client.borrow_mut().request(hyper::method::Method::Post, &url)
                    .header(UserAgent(self.hub._user_agent.clone()))
                    .header(auth_header.clone())
                    .header(ContentType(json_mime_type.clone()))
                    .header(ContentLength(request_size as u64))
                    .body(&mut request_value_reader);

                dlg.pre_request();
                req.send()
            };

            match req_result {
                Err(err) => {
                    if let oauth2::Retry::After(d) = dlg.http_error(&err) {
                        sleep(d);
                        continue;
                    }
                    dlg.finished(false);
                    return Err(Error::HttpError(err))
                }
                Ok(mut res) => {
                    if !res.status.is_success() {
                        let mut json_err = String::new();
                        res.read_to_string(&mut json_err).unwrap();
                        if let oauth2::Retry::After(d) = dlg.http_failure(&res,
                                                              json::from_str(&json_err).ok(),
                                                              json::from_str(&json_err).ok()) {
                            sleep(d);
                            continue;
                        }
                        dlg.finished(false);
                        return match json::from_str::<ErrorResponse>(&json_err){
                            Err(_) => Err(Error::Failure(res)),
                            Ok(serr) => Err(Error::BadRequest(serr))
                        }
                    }
                    let result_value = {
                        let mut json_response = String::new();
                        res.read_to_string(&mut json_response).unwrap();
                        match json::from_str(&json_response) {
                            Ok(decoded) => (res, decoded),
                            Err(err) => {
                                dlg.response_json_decode_error(&json_response, &err);
                                return Err(Error::JsonDecodeError(json_response, err));
                            }
                        }
                    };

                    dlg.finished(true);
                    return Ok(result_value)
                }
            }
        }
    }


    ///
    /// Sets the *request* property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call,
    /// we provide this method for API completeness.
    pub fn request(mut self, new_value: ReclaimStudentSubmissionRequest) -> CourseCourseWorkStudentSubmissionReclaimCall<'a, C, A> {
        self._request = new_value;
        self
    }
    /// Identifier of the course.
    /// This identifier can be either the Classroom-assigned identifier or an
    /// alias.
    ///
    /// Sets the *course id* path property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call,
    /// we provide this method for API completeness.
    pub fn course_id(mut self, new_value: &str) -> CourseCourseWorkStudentSubmissionReclaimCall<'a, C, A> {
        self._course_id = new_value.to_string();
        self
    }
    /// Identifier of the course work.
    ///
    /// Sets the *course work id* path property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call,
    /// we provide this method for API completeness.
    pub fn course_work_id(mut self, new_value: &str) -> CourseCourseWorkStudentSubmissionReclaimCall<'a, C, A> {
        self._course_work_id = new_value.to_string();
        self
    }
    /// Identifier of the student submission.
    ///
    /// Sets the *id* path property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call,
    /// we provide this method for API completeness.
    pub fn id(mut self, new_value: &str) -> CourseCourseWorkStudentSubmissionReclaimCall<'a, C, A> {
        self._id = new_value.to_string();
        self
    }
    /// The delegate implementation is consulted whenever there is an intermediate result, or if something goes wrong
    /// while executing the actual API request.
    /// 
    /// It should be used to handle progress information, and to implement a certain level of resilience.
    ///
    /// Sets the *delegate* property to the given value.
    pub fn delegate(mut self, new_value: &'a mut Delegate) -> CourseCourseWorkStudentSubmissionReclaimCall<'a, C, A> {
        self._delegate = Some(new_value);
        self
    }

    /// Set any additional parameter of the query string used in the request.
    /// It should be used to set parameters which are not yet available through their own
    /// setters.
    ///
    /// Please note that this method must not be used to set any of the known paramters
    /// which have their own setter method. If done anyway, the request will fail.
    ///
    /// # Additional Parameters
    ///
    /// * *upload_protocol* (query-string) - Upload protocol for media (e.g. "raw", "multipart").
    /// * *pp* (query-boolean) - Pretty-print response.
    /// * *prettyPrint* (query-boolean) - Returns response with indentations and line breaks.
    /// * *bearer_token* (query-string) - OAuth bearer token.
    /// * *access_token* (query-string) - OAuth access token.
    /// * *uploadType* (query-string) - Legacy upload protocol for media (e.g. "media", "multipart").
    /// * *quotaUser* (query-string) - Available to use for quota purposes for server-side applications. Can be any arbitrary string assigned to a user, but should not exceed 40 characters.
    /// * *callback* (query-string) - JSONP
    /// * *oauth_token* (query-string) - OAuth 2.0 token for the current user.
    /// * *key* (query-string) - API key. Your API key identifies your project and provides you with API access, quota, and reports. Required unless you provide an OAuth 2.0 token.
    /// * *fields* (query-string) - Selector specifying which fields to include in a partial response.
    /// * *alt* (query-string) - Data format for response.
    /// * *$.xgafv* (query-string) - V1 error format.
    pub fn param<T>(mut self, name: T, value: T) -> CourseCourseWorkStudentSubmissionReclaimCall<'a, C, A>
                                                        where T: AsRef<str> {
        self._additional_params.insert(name.as_ref().to_string(), value.as_ref().to_string());
        self
    }

    /// Identifies the authorization scope for the method you are building.
    ///
    /// Use this method to actively specify which scope should be used, instead the default `Scope` variant
    /// `Scope::CourseworkMe`.
    ///
    /// The `scope` will be added to a set of scopes. This is important as one can maintain access
    /// tokens for more than one scope.
    /// If `None` is specified, then all scopes will be removed and no default scope will be used either.
    /// In that case, you have to specify your API-key using the `key` parameter (see the `param()`
    /// function for details).
    ///
    /// Usually there is more than one suitable scope to authorize an operation, some of which may
    /// encompass more rights than others. For example, for listing resources, a *read-only* scope will be
    /// sufficient, a read-write scope will do as well.
    pub fn add_scope<T, S>(mut self, scope: T) -> CourseCourseWorkStudentSubmissionReclaimCall<'a, C, A>
                                                        where T: Into<Option<S>>,
                                                              S: AsRef<str> {
        match scope.into() {
          Some(scope) => self._scopes.insert(scope.as_ref().to_string(), ()),
          None => None,
        };
        self
    }
}


/// Creates an alias for a course.
/// 
/// This method returns the following error codes:
/// 
/// * `PERMISSION_DENIED` if the requesting user is not permitted to create the
/// alias or for access errors.
/// * `NOT_FOUND` if the course does not exist.
/// * `ALREADY_EXISTS` if the alias already exists.
/// * `FAILED_PRECONDITION` if the alias requested does not make sense for the
///   requesting user or course (for example, if a user not in a domain
///   attempts to access a domain-scoped alias).
///
/// A builder for the *aliases.create* method supported by a *course* resource.
/// It is not used directly, but through a `CourseMethods` instance.
///
/// # Example
///
/// Instantiate a resource method builder
///
/// ```test_harness,no_run
/// # extern crate hyper;
/// # extern crate hyper_rustls;
/// # extern crate yup_oauth2 as oauth2;
/// # extern crate google_classroom1 as classroom1;
/// use classroom1::CourseAlias;
/// # #[test] fn egal() {
/// # use std::default::Default;
/// # use oauth2::{Authenticator, DefaultAuthenticatorDelegate, ApplicationSecret, MemoryStorage};
/// # use classroom1::Classroom;
/// 
/// # let secret: ApplicationSecret = Default::default();
/// # let auth = Authenticator::new(&secret, DefaultAuthenticatorDelegate,
/// #                               hyper::Client::with_connector(hyper::net::HttpsConnector::new(hyper_rustls::TlsClient::new())),
/// #                               <MemoryStorage as Default>::default(), None);
/// # let mut hub = Classroom::new(hyper::Client::with_connector(hyper::net::HttpsConnector::new(hyper_rustls::TlsClient::new())), auth);
/// // As the method needs a request, you would usually fill it with the desired information
/// // into the respective structure. Some of the parts shown here might not be applicable !
/// // Values shown here are possibly random and not representative !
/// let mut req = CourseAlias::default();
/// 
/// // You can configure optional parameters by calling the respective setters at will, and
/// // execute the final call using `doit()`.
/// // Values shown here are possibly random and not representative !
/// let result = hub.courses().aliases_create(req, "courseId")
///              .doit();
/// # }
/// ```
pub struct CourseAliaseCreateCall<'a, C, A>
    where C: 'a, A: 'a {

    hub: &'a Classroom<C, A>,
    _request: CourseAlias,
    _course_id: String,
    _delegate: Option<&'a mut Delegate>,
    _additional_params: HashMap<String, String>,
    _scopes: BTreeMap<String, ()>
}

impl<'a, C, A> CallBuilder for CourseAliaseCreateCall<'a, C, A> {}

impl<'a, C, A> CourseAliaseCreateCall<'a, C, A> where C: BorrowMut<hyper::Client>, A: oauth2::GetToken {


    /// Perform the operation you have build so far.
    pub fn doit(mut self) -> Result<(hyper::client::Response, CourseAlias)> {
        use std::io::{Read, Seek};
        use hyper::header::{ContentType, ContentLength, Authorization, Bearer, UserAgent, Location};
        let mut dd = DefaultDelegate;
        let mut dlg: &mut Delegate = match self._delegate {
            Some(d) => d,
            None => &mut dd
        };
        dlg.begin(MethodInfo { id: "classroom.courses.aliases.create",
                               http_method: hyper::method::Method::Post });
        let mut params: Vec<(&str, String)> = Vec::with_capacity((4 + self._additional_params.len()));
        params.push(("courseId", self._course_id.to_string()));
        for &field in ["alt", "courseId"].iter() {
            if self._additional_params.contains_key(field) {
                dlg.finished(false);
                return Err(Error::FieldClash(field));
            }
        }
        for (name, value) in self._additional_params.iter() {
            params.push((&name, value.clone()));
        }

        params.push(("alt", "json".to_string()));

        let mut url = self.hub._base_url.clone() + "v1/courses/{courseId}/aliases";
        if self._scopes.len() == 0 {
            self._scopes.insert(Scope::Course.as_ref().to_string(), ());
        }

        for &(find_this, param_name) in [("{courseId}", "courseId")].iter() {
            let mut replace_with: Option<&str> = None;
            for &(name, ref value) in params.iter() {
                if name == param_name {
                    replace_with = Some(value);
                    break;
                }
            }
            url = url.replace(find_this, replace_with.expect("to find substitution value in params"));
        }
        {
            let mut indices_for_removal: Vec<usize> = Vec::with_capacity(1);
            for param_name in ["courseId"].iter() {
                if let Some(index) = params.iter().position(|t| &t.0 == param_name) {
                    indices_for_removal.push(index);
                }
            }
            for &index in indices_for_removal.iter() {
                params.remove(index);
            }
        }

        if params.len() > 0 {
            url.push('?');
            url.push_str(&url::form_urlencoded::serialize(params));
        }

        let mut json_mime_type = mime::Mime(mime::TopLevel::Application, mime::SubLevel::Json, Default::default());
        let mut request_value_reader =
            {
                let mut value = json::value::to_value(&self._request).expect("serde to work");
                remove_json_null_values(&mut value);
                let mut dst = io::Cursor::new(Vec::with_capacity(128));
                json::to_writer(&mut dst, &value).unwrap();
                dst
            };
        let request_size = request_value_reader.seek(io::SeekFrom::End(0)).unwrap();
        request_value_reader.seek(io::SeekFrom::Start(0)).unwrap();


        loop {
            let token = match self.hub.auth.borrow_mut().token(self._scopes.keys()) {
                Ok(token) => token,
                Err(err) => {
                    match  dlg.token(&*err) {
                        Some(token) => token,
                        None => {
                            dlg.finished(false);
                            return Err(Error::MissingToken(err))
                        }
                    }
                }
            };
            let auth_header = Authorization(Bearer { token: token.access_token });
            request_value_reader.seek(io::SeekFrom::Start(0)).unwrap();
            let mut req_result = {
                let mut client = &mut *self.hub.client.borrow_mut();
                let mut req = client.borrow_mut().request(hyper::method::Method::Post, &url)
                    .header(UserAgent(self.hub._user_agent.clone()))
                    .header(auth_header.clone())
                    .header(ContentType(json_mime_type.clone()))
                    .header(ContentLength(request_size as u64))
                    .body(&mut request_value_reader);

                dlg.pre_request();
                req.send()
            };

            match req_result {
                Err(err) => {
                    if let oauth2::Retry::After(d) = dlg.http_error(&err) {
                        sleep(d);
                        continue;
                    }
                    dlg.finished(false);
                    return Err(Error::HttpError(err))
                }
                Ok(mut res) => {
                    if !res.status.is_success() {
                        let mut json_err = String::new();
                        res.read_to_string(&mut json_err).unwrap();
                        if let oauth2::Retry::After(d) = dlg.http_failure(&res,
                                                              json::from_str(&json_err).ok(),
                                                              json::from_str(&json_err).ok()) {
                            sleep(d);
                            continue;
                        }
                        dlg.finished(false);
                        return match json::from_str::<ErrorResponse>(&json_err){
                            Err(_) => Err(Error::Failure(res)),
                            Ok(serr) => Err(Error::BadRequest(serr))
                        }
                    }
                    let result_value = {
                        let mut json_response = String::new();
                        res.read_to_string(&mut json_response).unwrap();
                        match json::from_str(&json_response) {
                            Ok(decoded) => (res, decoded),
                            Err(err) => {
                                dlg.response_json_decode_error(&json_response, &err);
                                return Err(Error::JsonDecodeError(json_response, err));
                            }
                        }
                    };

                    dlg.finished(true);
                    return Ok(result_value)
                }
            }
        }
    }


    ///
    /// Sets the *request* property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call,
    /// we provide this method for API completeness.
    pub fn request(mut self, new_value: CourseAlias) -> CourseAliaseCreateCall<'a, C, A> {
        self._request = new_value;
        self
    }
    /// Identifier of the course to alias.
    /// This identifier can be either the Classroom-assigned identifier or an
    /// alias.
    ///
    /// Sets the *course id* path property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call,
    /// we provide this method for API completeness.
    pub fn course_id(mut self, new_value: &str) -> CourseAliaseCreateCall<'a, C, A> {
        self._course_id = new_value.to_string();
        self
    }
    /// The delegate implementation is consulted whenever there is an intermediate result, or if something goes wrong
    /// while executing the actual API request.
    /// 
    /// It should be used to handle progress information, and to implement a certain level of resilience.
    ///
    /// Sets the *delegate* property to the given value.
    pub fn delegate(mut self, new_value: &'a mut Delegate) -> CourseAliaseCreateCall<'a, C, A> {
        self._delegate = Some(new_value);
        self
    }

    /// Set any additional parameter of the query string used in the request.
    /// It should be used to set parameters which are not yet available through their own
    /// setters.
    ///
    /// Please note that this method must not be used to set any of the known paramters
    /// which have their own setter method. If done anyway, the request will fail.
    ///
    /// # Additional Parameters
    ///
    /// * *upload_protocol* (query-string) - Upload protocol for media (e.g. "raw", "multipart").
    /// * *pp* (query-boolean) - Pretty-print response.
    /// * *prettyPrint* (query-boolean) - Returns response with indentations and line breaks.
    /// * *bearer_token* (query-string) - OAuth bearer token.
    /// * *access_token* (query-string) - OAuth access token.
    /// * *uploadType* (query-string) - Legacy upload protocol for media (e.g. "media", "multipart").
    /// * *quotaUser* (query-string) - Available to use for quota purposes for server-side applications. Can be any arbitrary string assigned to a user, but should not exceed 40 characters.
    /// * *callback* (query-string) - JSONP
    /// * *oauth_token* (query-string) - OAuth 2.0 token for the current user.
    /// * *key* (query-string) - API key. Your API key identifies your project and provides you with API access, quota, and reports. Required unless you provide an OAuth 2.0 token.
    /// * *fields* (query-string) - Selector specifying which fields to include in a partial response.
    /// * *alt* (query-string) - Data format for response.
    /// * *$.xgafv* (query-string) - V1 error format.
    pub fn param<T>(mut self, name: T, value: T) -> CourseAliaseCreateCall<'a, C, A>
                                                        where T: AsRef<str> {
        self._additional_params.insert(name.as_ref().to_string(), value.as_ref().to_string());
        self
    }

    /// Identifies the authorization scope for the method you are building.
    ///
    /// Use this method to actively specify which scope should be used, instead the default `Scope` variant
    /// `Scope::Course`.
    ///
    /// The `scope` will be added to a set of scopes. This is important as one can maintain access
    /// tokens for more than one scope.
    /// If `None` is specified, then all scopes will be removed and no default scope will be used either.
    /// In that case, you have to specify your API-key using the `key` parameter (see the `param()`
    /// function for details).
    ///
    /// Usually there is more than one suitable scope to authorize an operation, some of which may
    /// encompass more rights than others. For example, for listing resources, a *read-only* scope will be
    /// sufficient, a read-write scope will do as well.
    pub fn add_scope<T, S>(mut self, scope: T) -> CourseAliaseCreateCall<'a, C, A>
                                                        where T: Into<Option<S>>,
                                                              S: AsRef<str> {
        match scope.into() {
          Some(scope) => self._scopes.insert(scope.as_ref().to_string(), ()),
          None => None,
        };
        self
    }
}


/// Adds a user as a student of a course.
/// 
/// This method returns the following error codes:
/// 
/// * `PERMISSION_DENIED` if the requesting user is not permitted to create
/// students in this course or for access errors.
/// * `NOT_FOUND` if the requested course ID does not exist.
/// * `FAILED_PRECONDITION` if the requested user's account is disabled,
/// for the following request errors:
///     * CourseMemberLimitReached
///     * CourseNotModifiable
///     * UserGroupsMembershipLimitReached
/// * `ALREADY_EXISTS` if the user is already a student or teacher in the
/// course.
///
/// A builder for the *students.create* method supported by a *course* resource.
/// It is not used directly, but through a `CourseMethods` instance.
///
/// # Example
///
/// Instantiate a resource method builder
///
/// ```test_harness,no_run
/// # extern crate hyper;
/// # extern crate hyper_rustls;
/// # extern crate yup_oauth2 as oauth2;
/// # extern crate google_classroom1 as classroom1;
/// use classroom1::Student;
/// # #[test] fn egal() {
/// # use std::default::Default;
/// # use oauth2::{Authenticator, DefaultAuthenticatorDelegate, ApplicationSecret, MemoryStorage};
/// # use classroom1::Classroom;
/// 
/// # let secret: ApplicationSecret = Default::default();
/// # let auth = Authenticator::new(&secret, DefaultAuthenticatorDelegate,
/// #                               hyper::Client::with_connector(hyper::net::HttpsConnector::new(hyper_rustls::TlsClient::new())),
/// #                               <MemoryStorage as Default>::default(), None);
/// # let mut hub = Classroom::new(hyper::Client::with_connector(hyper::net::HttpsConnector::new(hyper_rustls::TlsClient::new())), auth);
/// // As the method needs a request, you would usually fill it with the desired information
/// // into the respective structure. Some of the parts shown here might not be applicable !
/// // Values shown here are possibly random and not representative !
/// let mut req = Student::default();
/// 
/// // You can configure optional parameters by calling the respective setters at will, and
/// // execute the final call using `doit()`.
/// // Values shown here are possibly random and not representative !
/// let result = hub.courses().students_create(req, "courseId")
///              .enrollment_code("sit")
///              .doit();
/// # }
/// ```
pub struct CourseStudentCreateCall<'a, C, A>
    where C: 'a, A: 'a {

    hub: &'a Classroom<C, A>,
    _request: Student,
    _course_id: String,
    _enrollment_code: Option<String>,
    _delegate: Option<&'a mut Delegate>,
    _additional_params: HashMap<String, String>,
    _scopes: BTreeMap<String, ()>
}

impl<'a, C, A> CallBuilder for CourseStudentCreateCall<'a, C, A> {}

impl<'a, C, A> CourseStudentCreateCall<'a, C, A> where C: BorrowMut<hyper::Client>, A: oauth2::GetToken {


    /// Perform the operation you have build so far.
    pub fn doit(mut self) -> Result<(hyper::client::Response, Student)> {
        use std::io::{Read, Seek};
        use hyper::header::{ContentType, ContentLength, Authorization, Bearer, UserAgent, Location};
        let mut dd = DefaultDelegate;
        let mut dlg: &mut Delegate = match self._delegate {
            Some(d) => d,
            None => &mut dd
        };
        dlg.begin(MethodInfo { id: "classroom.courses.students.create",
                               http_method: hyper::method::Method::Post });
        let mut params: Vec<(&str, String)> = Vec::with_capacity((5 + self._additional_params.len()));
        params.push(("courseId", self._course_id.to_string()));
        if let Some(value) = self._enrollment_code {
            params.push(("enrollmentCode", value.to_string()));
        }
        for &field in ["alt", "courseId", "enrollmentCode"].iter() {
            if self._additional_params.contains_key(field) {
                dlg.finished(false);
                return Err(Error::FieldClash(field));
            }
        }
        for (name, value) in self._additional_params.iter() {
            params.push((&name, value.clone()));
        }

        params.push(("alt", "json".to_string()));

        let mut url = self.hub._base_url.clone() + "v1/courses/{courseId}/students";
        if self._scopes.len() == 0 {
            self._scopes.insert(Scope::ProfileEmail.as_ref().to_string(), ());
        }

        for &(find_this, param_name) in [("{courseId}", "courseId")].iter() {
            let mut replace_with: Option<&str> = None;
            for &(name, ref value) in params.iter() {
                if name == param_name {
                    replace_with = Some(value);
                    break;
                }
            }
            url = url.replace(find_this, replace_with.expect("to find substitution value in params"));
        }
        {
            let mut indices_for_removal: Vec<usize> = Vec::with_capacity(1);
            for param_name in ["courseId"].iter() {
                if let Some(index) = params.iter().position(|t| &t.0 == param_name) {
                    indices_for_removal.push(index);
                }
            }
            for &index in indices_for_removal.iter() {
                params.remove(index);
            }
        }

        if params.len() > 0 {
            url.push('?');
            url.push_str(&url::form_urlencoded::serialize(params));
        }

        let mut json_mime_type = mime::Mime(mime::TopLevel::Application, mime::SubLevel::Json, Default::default());
        let mut request_value_reader =
            {
                let mut value = json::value::to_value(&self._request).expect("serde to work");
                remove_json_null_values(&mut value);
                let mut dst = io::Cursor::new(Vec::with_capacity(128));
                json::to_writer(&mut dst, &value).unwrap();
                dst
            };
        let request_size = request_value_reader.seek(io::SeekFrom::End(0)).unwrap();
        request_value_reader.seek(io::SeekFrom::Start(0)).unwrap();


        loop {
            let token = match self.hub.auth.borrow_mut().token(self._scopes.keys()) {
                Ok(token) => token,
                Err(err) => {
                    match  dlg.token(&*err) {
                        Some(token) => token,
                        None => {
                            dlg.finished(false);
                            return Err(Error::MissingToken(err))
                        }
                    }
                }
            };
            let auth_header = Authorization(Bearer { token: token.access_token });
            request_value_reader.seek(io::SeekFrom::Start(0)).unwrap();
            let mut req_result = {
                let mut client = &mut *self.hub.client.borrow_mut();
                let mut req = client.borrow_mut().request(hyper::method::Method::Post, &url)
                    .header(UserAgent(self.hub._user_agent.clone()))
                    .header(auth_header.clone())
                    .header(ContentType(json_mime_type.clone()))
                    .header(ContentLength(request_size as u64))
                    .body(&mut request_value_reader);

                dlg.pre_request();
                req.send()
            };

            match req_result {
                Err(err) => {
                    if let oauth2::Retry::After(d) = dlg.http_error(&err) {
                        sleep(d);
                        continue;
                    }
                    dlg.finished(false);
                    return Err(Error::HttpError(err))
                }
                Ok(mut res) => {
                    if !res.status.is_success() {
                        let mut json_err = String::new();
                        res.read_to_string(&mut json_err).unwrap();
                        if let oauth2::Retry::After(d) = dlg.http_failure(&res,
                                                              json::from_str(&json_err).ok(),
                                                              json::from_str(&json_err).ok()) {
                            sleep(d);
                            continue;
                        }
                        dlg.finished(false);
                        return match json::from_str::<ErrorResponse>(&json_err){
                            Err(_) => Err(Error::Failure(res)),
                            Ok(serr) => Err(Error::BadRequest(serr))
                        }
                    }
                    let result_value = {
                        let mut json_response = String::new();
                        res.read_to_string(&mut json_response).unwrap();
                        match json::from_str(&json_response) {
                            Ok(decoded) => (res, decoded),
                            Err(err) => {
                                dlg.response_json_decode_error(&json_response, &err);
                                return Err(Error::JsonDecodeError(json_response, err));
                            }
                        }
                    };

                    dlg.finished(true);
                    return Ok(result_value)
                }
            }
        }
    }


    ///
    /// Sets the *request* property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call,
    /// we provide this method for API completeness.
    pub fn request(mut self, new_value: Student) -> CourseStudentCreateCall<'a, C, A> {
        self._request = new_value;
        self
    }
    /// Identifier of the course to create the student in.
    /// This identifier can be either the Classroom-assigned identifier or an
    /// alias.
    ///
    /// Sets the *course id* path property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call,
    /// we provide this method for API completeness.
    pub fn course_id(mut self, new_value: &str) -> CourseStudentCreateCall<'a, C, A> {
        self._course_id = new_value.to_string();
        self
    }
    /// Enrollment code of the course to create the student in.
    /// This code is required if userId
    /// corresponds to the requesting user; it may be omitted if the requesting
    /// user has administrative permissions to create students for any user.
    ///
    /// Sets the *enrollment code* query property to the given value.
    pub fn enrollment_code(mut self, new_value: &str) -> CourseStudentCreateCall<'a, C, A> {
        self._enrollment_code = Some(new_value.to_string());
        self
    }
    /// The delegate implementation is consulted whenever there is an intermediate result, or if something goes wrong
    /// while executing the actual API request.
    /// 
    /// It should be used to handle progress information, and to implement a certain level of resilience.
    ///
    /// Sets the *delegate* property to the given value.
    pub fn delegate(mut self, new_value: &'a mut Delegate) -> CourseStudentCreateCall<'a, C, A> {
        self._delegate = Some(new_value);
        self
    }

    /// Set any additional parameter of the query string used in the request.
    /// It should be used to set parameters which are not yet available through their own
    /// setters.
    ///
    /// Please note that this method must not be used to set any of the known paramters
    /// which have their own setter method. If done anyway, the request will fail.
    ///
    /// # Additional Parameters
    ///
    /// * *upload_protocol* (query-string) - Upload protocol for media (e.g. "raw", "multipart").
    /// * *pp* (query-boolean) - Pretty-print response.
    /// * *prettyPrint* (query-boolean) - Returns response with indentations and line breaks.
    /// * *bearer_token* (query-string) - OAuth bearer token.
    /// * *access_token* (query-string) - OAuth access token.
    /// * *uploadType* (query-string) - Legacy upload protocol for media (e.g. "media", "multipart").
    /// * *quotaUser* (query-string) - Available to use for quota purposes for server-side applications. Can be any arbitrary string assigned to a user, but should not exceed 40 characters.
    /// * *callback* (query-string) - JSONP
    /// * *oauth_token* (query-string) - OAuth 2.0 token for the current user.
    /// * *key* (query-string) - API key. Your API key identifies your project and provides you with API access, quota, and reports. Required unless you provide an OAuth 2.0 token.
    /// * *fields* (query-string) - Selector specifying which fields to include in a partial response.
    /// * *alt* (query-string) - Data format for response.
    /// * *$.xgafv* (query-string) - V1 error format.
    pub fn param<T>(mut self, name: T, value: T) -> CourseStudentCreateCall<'a, C, A>
                                                        where T: AsRef<str> {
        self._additional_params.insert(name.as_ref().to_string(), value.as_ref().to_string());
        self
    }

    /// Identifies the authorization scope for the method you are building.
    ///
    /// Use this method to actively specify which scope should be used, instead the default `Scope` variant
    /// `Scope::ProfileEmail`.
    ///
    /// The `scope` will be added to a set of scopes. This is important as one can maintain access
    /// tokens for more than one scope.
    /// If `None` is specified, then all scopes will be removed and no default scope will be used either.
    /// In that case, you have to specify your API-key using the `key` parameter (see the `param()`
    /// function for details).
    ///
    /// Usually there is more than one suitable scope to authorize an operation, some of which may
    /// encompass more rights than others. For example, for listing resources, a *read-only* scope will be
    /// sufficient, a read-write scope will do as well.
    pub fn add_scope<T, S>(mut self, scope: T) -> CourseStudentCreateCall<'a, C, A>
                                                        where T: Into<Option<S>>,
                                                              S: AsRef<str> {
        match scope.into() {
          Some(scope) => self._scopes.insert(scope.as_ref().to_string(), ()),
          None => None,
        };
        self
    }
}


/// Deletes an alias of a course.
/// 
/// This method returns the following error codes:
/// 
/// * `PERMISSION_DENIED` if the requesting user is not permitted to remove the
/// alias or for access errors.
/// * `NOT_FOUND` if the alias does not exist.
/// * `FAILED_PRECONDITION` if the alias requested does not make sense for the
///   requesting user or course (for example, if a user not in a domain
///   attempts to delete a domain-scoped alias).
///
/// A builder for the *aliases.delete* method supported by a *course* resource.
/// It is not used directly, but through a `CourseMethods` instance.
///
/// # Example
///
/// Instantiate a resource method builder
///
/// ```test_harness,no_run
/// # extern crate hyper;
/// # extern crate hyper_rustls;
/// # extern crate yup_oauth2 as oauth2;
/// # extern crate google_classroom1 as classroom1;
/// # #[test] fn egal() {
/// # use std::default::Default;
/// # use oauth2::{Authenticator, DefaultAuthenticatorDelegate, ApplicationSecret, MemoryStorage};
/// # use classroom1::Classroom;
/// 
/// # let secret: ApplicationSecret = Default::default();
/// # let auth = Authenticator::new(&secret, DefaultAuthenticatorDelegate,
/// #                               hyper::Client::with_connector(hyper::net::HttpsConnector::new(hyper_rustls::TlsClient::new())),
/// #                               <MemoryStorage as Default>::default(), None);
/// # let mut hub = Classroom::new(hyper::Client::with_connector(hyper::net::HttpsConnector::new(hyper_rustls::TlsClient::new())), auth);
/// // You can configure optional parameters by calling the respective setters at will, and
/// // execute the final call using `doit()`.
/// // Values shown here are possibly random and not representative !
/// let result = hub.courses().aliases_delete("courseId", "alias")
///              .doit();
/// # }
/// ```
pub struct CourseAliaseDeleteCall<'a, C, A>
    where C: 'a, A: 'a {

    hub: &'a Classroom<C, A>,
    _course_id: String,
    _alias: String,
    _delegate: Option<&'a mut Delegate>,
    _additional_params: HashMap<String, String>,
    _scopes: BTreeMap<String, ()>
}

impl<'a, C, A> CallBuilder for CourseAliaseDeleteCall<'a, C, A> {}

impl<'a, C, A> CourseAliaseDeleteCall<'a, C, A> where C: BorrowMut<hyper::Client>, A: oauth2::GetToken {


    /// Perform the operation you have build so far.
    pub fn doit(mut self) -> Result<(hyper::client::Response, Empty)> {
        use std::io::{Read, Seek};
        use hyper::header::{ContentType, ContentLength, Authorization, Bearer, UserAgent, Location};
        let mut dd = DefaultDelegate;
        let mut dlg: &mut Delegate = match self._delegate {
            Some(d) => d,
            None => &mut dd
        };
        dlg.begin(MethodInfo { id: "classroom.courses.aliases.delete",
                               http_method: hyper::method::Method::Delete });
        let mut params: Vec<(&str, String)> = Vec::with_capacity((4 + self._additional_params.len()));
        params.push(("courseId", self._course_id.to_string()));
        params.push(("alias", self._alias.to_string()));
        for &field in ["alt", "courseId", "alias"].iter() {
            if self._additional_params.contains_key(field) {
                dlg.finished(false);
                return Err(Error::FieldClash(field));
            }
        }
        for (name, value) in self._additional_params.iter() {
            params.push((&name, value.clone()));
        }

        params.push(("alt", "json".to_string()));

        let mut url = self.hub._base_url.clone() + "v1/courses/{courseId}/aliases/{alias}";
        if self._scopes.len() == 0 {
            self._scopes.insert(Scope::Course.as_ref().to_string(), ());
        }

        for &(find_this, param_name) in [("{courseId}", "courseId"), ("{alias}", "alias")].iter() {
            let mut replace_with: Option<&str> = None;
            for &(name, ref value) in params.iter() {
                if name == param_name {
                    replace_with = Some(value);
                    break;
                }
            }
            url = url.replace(find_this, replace_with.expect("to find substitution value in params"));
        }
        {
            let mut indices_for_removal: Vec<usize> = Vec::with_capacity(2);
            for param_name in ["alias", "courseId"].iter() {
                if let Some(index) = params.iter().position(|t| &t.0 == param_name) {
                    indices_for_removal.push(index);
                }
            }
            for &index in indices_for_removal.iter() {
                params.remove(index);
            }
        }

        if params.len() > 0 {
            url.push('?');
            url.push_str(&url::form_urlencoded::serialize(params));
        }



        loop {
            let token = match self.hub.auth.borrow_mut().token(self._scopes.keys()) {
                Ok(token) => token,
                Err(err) => {
                    match  dlg.token(&*err) {
                        Some(token) => token,
                        None => {
                            dlg.finished(false);
                            return Err(Error::MissingToken(err))
                        }
                    }
                }
            };
            let auth_header = Authorization(Bearer { token: token.access_token });
            let mut req_result = {
                let mut client = &mut *self.hub.client.borrow_mut();
                let mut req = client.borrow_mut().request(hyper::method::Method::Delete, &url)
                    .header(UserAgent(self.hub._user_agent.clone()))
                    .header(auth_header.clone());

                dlg.pre_request();
                req.send()
            };

            match req_result {
                Err(err) => {
                    if let oauth2::Retry::After(d) = dlg.http_error(&err) {
                        sleep(d);
                        continue;
                    }
                    dlg.finished(false);
                    return Err(Error::HttpError(err))
                }
                Ok(mut res) => {
                    if !res.status.is_success() {
                        let mut json_err = String::new();
                        res.read_to_string(&mut json_err).unwrap();
                        if let oauth2::Retry::After(d) = dlg.http_failure(&res,
                                                              json::from_str(&json_err).ok(),
                                                              json::from_str(&json_err).ok()) {
                            sleep(d);
                            continue;
                        }
                        dlg.finished(false);
                        return match json::from_str::<ErrorResponse>(&json_err){
                            Err(_) => Err(Error::Failure(res)),
                            Ok(serr) => Err(Error::BadRequest(serr))
                        }
                    }
                    let result_value = {
                        let mut json_response = String::new();
                        res.read_to_string(&mut json_response).unwrap();
                        match json::from_str(&json_response) {
                            Ok(decoded) => (res, decoded),
                            Err(err) => {
                                dlg.response_json_decode_error(&json_response, &err);
                                return Err(Error::JsonDecodeError(json_response, err));
                            }
                        }
                    };

                    dlg.finished(true);
                    return Ok(result_value)
                }
            }
        }
    }


    /// Identifier of the course whose alias should be deleted.
    /// This identifier can be either the Classroom-assigned identifier or an
    /// alias.
    ///
    /// Sets the *course id* path property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call,
    /// we provide this method for API completeness.
    pub fn course_id(mut self, new_value: &str) -> CourseAliaseDeleteCall<'a, C, A> {
        self._course_id = new_value.to_string();
        self
    }
    /// Alias to delete.
    /// This may not be the Classroom-assigned identifier.
    ///
    /// Sets the *alias* path property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call,
    /// we provide this method for API completeness.
    pub fn alias(mut self, new_value: &str) -> CourseAliaseDeleteCall<'a, C, A> {
        self._alias = new_value.to_string();
        self
    }
    /// The delegate implementation is consulted whenever there is an intermediate result, or if something goes wrong
    /// while executing the actual API request.
    /// 
    /// It should be used to handle progress information, and to implement a certain level of resilience.
    ///
    /// Sets the *delegate* property to the given value.
    pub fn delegate(mut self, new_value: &'a mut Delegate) -> CourseAliaseDeleteCall<'a, C, A> {
        self._delegate = Some(new_value);
        self
    }

    /// Set any additional parameter of the query string used in the request.
    /// It should be used to set parameters which are not yet available through their own
    /// setters.
    ///
    /// Please note that this method must not be used to set any of the known paramters
    /// which have their own setter method. If done anyway, the request will fail.
    ///
    /// # Additional Parameters
    ///
    /// * *upload_protocol* (query-string) - Upload protocol for media (e.g. "raw", "multipart").
    /// * *pp* (query-boolean) - Pretty-print response.
    /// * *prettyPrint* (query-boolean) - Returns response with indentations and line breaks.
    /// * *bearer_token* (query-string) - OAuth bearer token.
    /// * *access_token* (query-string) - OAuth access token.
    /// * *uploadType* (query-string) - Legacy upload protocol for media (e.g. "media", "multipart").
    /// * *quotaUser* (query-string) - Available to use for quota purposes for server-side applications. Can be any arbitrary string assigned to a user, but should not exceed 40 characters.
    /// * *callback* (query-string) - JSONP
    /// * *oauth_token* (query-string) - OAuth 2.0 token for the current user.
    /// * *key* (query-string) - API key. Your API key identifies your project and provides you with API access, quota, and reports. Required unless you provide an OAuth 2.0 token.
    /// * *fields* (query-string) - Selector specifying which fields to include in a partial response.
    /// * *alt* (query-string) - Data format for response.
    /// * *$.xgafv* (query-string) - V1 error format.
    pub fn param<T>(mut self, name: T, value: T) -> CourseAliaseDeleteCall<'a, C, A>
                                                        where T: AsRef<str> {
        self._additional_params.insert(name.as_ref().to_string(), value.as_ref().to_string());
        self
    }

    /// Identifies the authorization scope for the method you are building.
    ///
    /// Use this method to actively specify which scope should be used, instead the default `Scope` variant
    /// `Scope::Course`.
    ///
    /// The `scope` will be added to a set of scopes. This is important as one can maintain access
    /// tokens for more than one scope.
    /// If `None` is specified, then all scopes will be removed and no default scope will be used either.
    /// In that case, you have to specify your API-key using the `key` parameter (see the `param()`
    /// function for details).
    ///
    /// Usually there is more than one suitable scope to authorize an operation, some of which may
    /// encompass more rights than others. For example, for listing resources, a *read-only* scope will be
    /// sufficient, a read-write scope will do as well.
    pub fn add_scope<T, S>(mut self, scope: T) -> CourseAliaseDeleteCall<'a, C, A>
                                                        where T: Into<Option<S>>,
                                                              S: AsRef<str> {
        match scope.into() {
          Some(scope) => self._scopes.insert(scope.as_ref().to_string(), ()),
          None => None,
        };
        self
    }
}


/// Deletes a course work.
/// 
/// This request must be made by the Developer Console project of the
/// [OAuth client ID](https://support.google.com/cloud/answer/6158849) used to
/// create the corresponding course work item.
/// 
/// This method returns the following error codes:
/// 
/// * `PERMISSION_DENIED` if the requesting developer project did not create
/// the corresponding course work, if the requesting user is not permitted
/// to delete the requested course or for access errors.
/// * `FAILED_PRECONDITION` if the requested course work has already been
/// deleted.
/// * `NOT_FOUND` if no course exists with the requested ID.
///
/// A builder for the *courseWork.delete* method supported by a *course* resource.
/// It is not used directly, but through a `CourseMethods` instance.
///
/// # Example
///
/// Instantiate a resource method builder
///
/// ```test_harness,no_run
/// # extern crate hyper;
/// # extern crate hyper_rustls;
/// # extern crate yup_oauth2 as oauth2;
/// # extern crate google_classroom1 as classroom1;
/// # #[test] fn egal() {
/// # use std::default::Default;
/// # use oauth2::{Authenticator, DefaultAuthenticatorDelegate, ApplicationSecret, MemoryStorage};
/// # use classroom1::Classroom;
/// 
/// # let secret: ApplicationSecret = Default::default();
/// # let auth = Authenticator::new(&secret, DefaultAuthenticatorDelegate,
/// #                               hyper::Client::with_connector(hyper::net::HttpsConnector::new(hyper_rustls::TlsClient::new())),
/// #                               <MemoryStorage as Default>::default(), None);
/// # let mut hub = Classroom::new(hyper::Client::with_connector(hyper::net::HttpsConnector::new(hyper_rustls::TlsClient::new())), auth);
/// // You can configure optional parameters by calling the respective setters at will, and
/// // execute the final call using `doit()`.
/// // Values shown here are possibly random and not representative !
/// let result = hub.courses().course_work_delete("courseId", "id")
///              .doit();
/// # }
/// ```
pub struct CourseCourseWorkDeleteCall<'a, C, A>
    where C: 'a, A: 'a {

    hub: &'a Classroom<C, A>,
    _course_id: String,
    _id: String,
    _delegate: Option<&'a mut Delegate>,
    _additional_params: HashMap<String, String>,
    _scopes: BTreeMap<String, ()>
}

impl<'a, C, A> CallBuilder for CourseCourseWorkDeleteCall<'a, C, A> {}

impl<'a, C, A> CourseCourseWorkDeleteCall<'a, C, A> where C: BorrowMut<hyper::Client>, A: oauth2::GetToken {


    /// Perform the operation you have build so far.
    pub fn doit(mut self) -> Result<(hyper::client::Response, Empty)> {
        use std::io::{Read, Seek};
        use hyper::header::{ContentType, ContentLength, Authorization, Bearer, UserAgent, Location};
        let mut dd = DefaultDelegate;
        let mut dlg: &mut Delegate = match self._delegate {
            Some(d) => d,
            None => &mut dd
        };
        dlg.begin(MethodInfo { id: "classroom.courses.courseWork.delete",
                               http_method: hyper::method::Method::Delete });
        let mut params: Vec<(&str, String)> = Vec::with_capacity((4 + self._additional_params.len()));
        params.push(("courseId", self._course_id.to_string()));
        params.push(("id", self._id.to_string()));
        for &field in ["alt", "courseId", "id"].iter() {
            if self._additional_params.contains_key(field) {
                dlg.finished(false);
                return Err(Error::FieldClash(field));
            }
        }
        for (name, value) in self._additional_params.iter() {
            params.push((&name, value.clone()));
        }

        params.push(("alt", "json".to_string()));

        let mut url = self.hub._base_url.clone() + "v1/courses/{courseId}/courseWork/{id}";
        if self._scopes.len() == 0 {
            self._scopes.insert(Scope::CourseworkStudent.as_ref().to_string(), ());
        }

        for &(find_this, param_name) in [("{courseId}", "courseId"), ("{id}", "id")].iter() {
            let mut replace_with: Option<&str> = None;
            for &(name, ref value) in params.iter() {
                if name == param_name {
                    replace_with = Some(value);
                    break;
                }
            }
            url = url.replace(find_this, replace_with.expect("to find substitution value in params"));
        }
        {
            let mut indices_for_removal: Vec<usize> = Vec::with_capacity(2);
            for param_name in ["id", "courseId"].iter() {
                if let Some(index) = params.iter().position(|t| &t.0 == param_name) {
                    indices_for_removal.push(index);
                }
            }
            for &index in indices_for_removal.iter() {
                params.remove(index);
            }
        }

        if params.len() > 0 {
            url.push('?');
            url.push_str(&url::form_urlencoded::serialize(params));
        }



        loop {
            let token = match self.hub.auth.borrow_mut().token(self._scopes.keys()) {
                Ok(token) => token,
                Err(err) => {
                    match  dlg.token(&*err) {
                        Some(token) => token,
                        None => {
                            dlg.finished(false);
                            return Err(Error::MissingToken(err))
                        }
                    }
                }
            };
            let auth_header = Authorization(Bearer { token: token.access_token });
            let mut req_result = {
                let mut client = &mut *self.hub.client.borrow_mut();
                let mut req = client.borrow_mut().request(hyper::method::Method::Delete, &url)
                    .header(UserAgent(self.hub._user_agent.clone()))
                    .header(auth_header.clone());

                dlg.pre_request();
                req.send()
            };

            match req_result {
                Err(err) => {
                    if let oauth2::Retry::After(d) = dlg.http_error(&err) {
                        sleep(d);
                        continue;
                    }
                    dlg.finished(false);
                    return Err(Error::HttpError(err))
                }
                Ok(mut res) => {
                    if !res.status.is_success() {
                        let mut json_err = String::new();
                        res.read_to_string(&mut json_err).unwrap();
                        if let oauth2::Retry::After(d) = dlg.http_failure(&res,
                                                              json::from_str(&json_err).ok(),
                                                              json::from_str(&json_err).ok()) {
                            sleep(d);
                            continue;
                        }
                        dlg.finished(false);
                        return match json::from_str::<ErrorResponse>(&json_err){
                            Err(_) => Err(Error::Failure(res)),
                            Ok(serr) => Err(Error::BadRequest(serr))
                        }
                    }
                    let result_value = {
                        let mut json_response = String::new();
                        res.read_to_string(&mut json_response).unwrap();
                        match json::from_str(&json_response) {
                            Ok(decoded) => (res, decoded),
                            Err(err) => {
                                dlg.response_json_decode_error(&json_response, &err);
                                return Err(Error::JsonDecodeError(json_response, err));
                            }
                        }
                    };

                    dlg.finished(true);
                    return Ok(result_value)
                }
            }
        }
    }


    /// Identifier of the course.
    /// This identifier can be either the Classroom-assigned identifier or an
    /// alias.
    ///
    /// Sets the *course id* path property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call,
    /// we provide this method for API completeness.
    pub fn course_id(mut self, new_value: &str) -> CourseCourseWorkDeleteCall<'a, C, A> {
        self._course_id = new_value.to_string();
        self
    }
    /// Identifier of the course work to delete.
    /// This identifier is a Classroom-assigned identifier.
    ///
    /// Sets the *id* path property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call,
    /// we provide this method for API completeness.
    pub fn id(mut self, new_value: &str) -> CourseCourseWorkDeleteCall<'a, C, A> {
        self._id = new_value.to_string();
        self
    }
    /// The delegate implementation is consulted whenever there is an intermediate result, or if something goes wrong
    /// while executing the actual API request.
    /// 
    /// It should be used to handle progress information, and to implement a certain level of resilience.
    ///
    /// Sets the *delegate* property to the given value.
    pub fn delegate(mut self, new_value: &'a mut Delegate) -> CourseCourseWorkDeleteCall<'a, C, A> {
        self._delegate = Some(new_value);
        self
    }

    /// Set any additional parameter of the query string used in the request.
    /// It should be used to set parameters which are not yet available through their own
    /// setters.
    ///
    /// Please note that this method must not be used to set any of the known paramters
    /// which have their own setter method. If done anyway, the request will fail.
    ///
    /// # Additional Parameters
    ///
    /// * *upload_protocol* (query-string) - Upload protocol for media (e.g. "raw", "multipart").
    /// * *pp* (query-boolean) - Pretty-print response.
    /// * *prettyPrint* (query-boolean) - Returns response with indentations and line breaks.
    /// * *bearer_token* (query-string) - OAuth bearer token.
    /// * *access_token* (query-string) - OAuth access token.
    /// * *uploadType* (query-string) - Legacy upload protocol for media (e.g. "media", "multipart").
    /// * *quotaUser* (query-string) - Available to use for quota purposes for server-side applications. Can be any arbitrary string assigned to a user, but should not exceed 40 characters.
    /// * *callback* (query-string) - JSONP
    /// * *oauth_token* (query-string) - OAuth 2.0 token for the current user.
    /// * *key* (query-string) - API key. Your API key identifies your project and provides you with API access, quota, and reports. Required unless you provide an OAuth 2.0 token.
    /// * *fields* (query-string) - Selector specifying which fields to include in a partial response.
    /// * *alt* (query-string) - Data format for response.
    /// * *$.xgafv* (query-string) - V1 error format.
    pub fn param<T>(mut self, name: T, value: T) -> CourseCourseWorkDeleteCall<'a, C, A>
                                                        where T: AsRef<str> {
        self._additional_params.insert(name.as_ref().to_string(), value.as_ref().to_string());
        self
    }

    /// Identifies the authorization scope for the method you are building.
    ///
    /// Use this method to actively specify which scope should be used, instead the default `Scope` variant
    /// `Scope::CourseworkStudent`.
    ///
    /// The `scope` will be added to a set of scopes. This is important as one can maintain access
    /// tokens for more than one scope.
    /// If `None` is specified, then all scopes will be removed and no default scope will be used either.
    /// In that case, you have to specify your API-key using the `key` parameter (see the `param()`
    /// function for details).
    ///
    /// Usually there is more than one suitable scope to authorize an operation, some of which may
    /// encompass more rights than others. For example, for listing resources, a *read-only* scope will be
    /// sufficient, a read-write scope will do as well.
    pub fn add_scope<T, S>(mut self, scope: T) -> CourseCourseWorkDeleteCall<'a, C, A>
                                                        where T: Into<Option<S>>,
                                                              S: AsRef<str> {
        match scope.into() {
          Some(scope) => self._scopes.insert(scope.as_ref().to_string(), ()),
          None => None,
        };
        self
    }
}


/// Creates a course.
/// 
/// The user specified in `ownerId` is the owner of the created course
/// and added as a teacher.
/// 
/// This method returns the following error codes:
/// 
/// * `PERMISSION_DENIED` if the requesting user is not permitted to create
/// courses or for access errors.
/// * `NOT_FOUND` if the primary teacher is not a valid user.
/// * `FAILED_PRECONDITION` if the course owner's account is disabled or for
/// the following request errors:
///     * UserGroupsMembershipLimitReached
/// * `ALREADY_EXISTS` if an alias was specified in the `id` and
/// already exists.
///
/// A builder for the *create* method supported by a *course* resource.
/// It is not used directly, but through a `CourseMethods` instance.
///
/// # Example
///
/// Instantiate a resource method builder
///
/// ```test_harness,no_run
/// # extern crate hyper;
/// # extern crate hyper_rustls;
/// # extern crate yup_oauth2 as oauth2;
/// # extern crate google_classroom1 as classroom1;
/// use classroom1::Course;
/// # #[test] fn egal() {
/// # use std::default::Default;
/// # use oauth2::{Authenticator, DefaultAuthenticatorDelegate, ApplicationSecret, MemoryStorage};
/// # use classroom1::Classroom;
/// 
/// # let secret: ApplicationSecret = Default::default();
/// # let auth = Authenticator::new(&secret, DefaultAuthenticatorDelegate,
/// #                               hyper::Client::with_connector(hyper::net::HttpsConnector::new(hyper_rustls::TlsClient::new())),
/// #                               <MemoryStorage as Default>::default(), None);
/// # let mut hub = Classroom::new(hyper::Client::with_connector(hyper::net::HttpsConnector::new(hyper_rustls::TlsClient::new())), auth);
/// // As the method needs a request, you would usually fill it with the desired information
/// // into the respective structure. Some of the parts shown here might not be applicable !
/// // Values shown here are possibly random and not representative !
/// let mut req = Course::default();
/// 
/// // You can configure optional parameters by calling the respective setters at will, and
/// // execute the final call using `doit()`.
/// // Values shown here are possibly random and not representative !
/// let result = hub.courses().create(req)
///              .doit();
/// # }
/// ```
pub struct CourseCreateCall<'a, C, A>
    where C: 'a, A: 'a {

    hub: &'a Classroom<C, A>,
    _request: Course,
    _delegate: Option<&'a mut Delegate>,
    _additional_params: HashMap<String, String>,
    _scopes: BTreeMap<String, ()>
}

impl<'a, C, A> CallBuilder for CourseCreateCall<'a, C, A> {}

impl<'a, C, A> CourseCreateCall<'a, C, A> where C: BorrowMut<hyper::Client>, A: oauth2::GetToken {


    /// Perform the operation you have build so far.
    pub fn doit(mut self) -> Result<(hyper::client::Response, Course)> {
        use std::io::{Read, Seek};
        use hyper::header::{ContentType, ContentLength, Authorization, Bearer, UserAgent, Location};
        let mut dd = DefaultDelegate;
        let mut dlg: &mut Delegate = match self._delegate {
            Some(d) => d,
            None => &mut dd
        };
        dlg.begin(MethodInfo { id: "classroom.courses.create",
                               http_method: hyper::method::Method::Post });
        let mut params: Vec<(&str, String)> = Vec::with_capacity((3 + self._additional_params.len()));
        for &field in ["alt"].iter() {
            if self._additional_params.contains_key(field) {
                dlg.finished(false);
                return Err(Error::FieldClash(field));
            }
        }
        for (name, value) in self._additional_params.iter() {
            params.push((&name, value.clone()));
        }

        params.push(("alt", "json".to_string()));

        let mut url = self.hub._base_url.clone() + "v1/courses";
        if self._scopes.len() == 0 {
            self._scopes.insert(Scope::Course.as_ref().to_string(), ());
        }


        if params.len() > 0 {
            url.push('?');
            url.push_str(&url::form_urlencoded::serialize(params));
        }

        let mut json_mime_type = mime::Mime(mime::TopLevel::Application, mime::SubLevel::Json, Default::default());
        let mut request_value_reader =
            {
                let mut value = json::value::to_value(&self._request).expect("serde to work");
                remove_json_null_values(&mut value);
                let mut dst = io::Cursor::new(Vec::with_capacity(128));
                json::to_writer(&mut dst, &value).unwrap();
                dst
            };
        let request_size = request_value_reader.seek(io::SeekFrom::End(0)).unwrap();
        request_value_reader.seek(io::SeekFrom::Start(0)).unwrap();


        loop {
            let token = match self.hub.auth.borrow_mut().token(self._scopes.keys()) {
                Ok(token) => token,
                Err(err) => {
                    match  dlg.token(&*err) {
                        Some(token) => token,
                        None => {
                            dlg.finished(false);
                            return Err(Error::MissingToken(err))
                        }
                    }
                }
            };
            let auth_header = Authorization(Bearer { token: token.access_token });
            request_value_reader.seek(io::SeekFrom::Start(0)).unwrap();
            let mut req_result = {
                let mut client = &mut *self.hub.client.borrow_mut();
                let mut req = client.borrow_mut().request(hyper::method::Method::Post, &url)
                    .header(UserAgent(self.hub._user_agent.clone()))
                    .header(auth_header.clone())
                    .header(ContentType(json_mime_type.clone()))
                    .header(ContentLength(request_size as u64))
                    .body(&mut request_value_reader);

                dlg.pre_request();
                req.send()
            };

            match req_result {
                Err(err) => {
                    if let oauth2::Retry::After(d) = dlg.http_error(&err) {
                        sleep(d);
                        continue;
                    }
                    dlg.finished(false);
                    return Err(Error::HttpError(err))
                }
                Ok(mut res) => {
                    if !res.status.is_success() {
                        let mut json_err = String::new();
                        res.read_to_string(&mut json_err).unwrap();
                        if let oauth2::Retry::After(d) = dlg.http_failure(&res,
                                                              json::from_str(&json_err).ok(),
                                                              json::from_str(&json_err).ok()) {
                            sleep(d);
                            continue;
                        }
                        dlg.finished(false);
                        return match json::from_str::<ErrorResponse>(&json_err){
                            Err(_) => Err(Error::Failure(res)),
                            Ok(serr) => Err(Error::BadRequest(serr))
                        }
                    }
                    let result_value = {
                        let mut json_response = String::new();
                        res.read_to_string(&mut json_response).unwrap();
                        match json::from_str(&json_response) {
                            Ok(decoded) => (res, decoded),
                            Err(err) => {
                                dlg.response_json_decode_error(&json_response, &err);
                                return Err(Error::JsonDecodeError(json_response, err));
                            }
                        }
                    };

                    dlg.finished(true);
                    return Ok(result_value)
                }
            }
        }
    }


    ///
    /// Sets the *request* property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call,
    /// we provide this method for API completeness.
    pub fn request(mut self, new_value: Course) -> CourseCreateCall<'a, C, A> {
        self._request = new_value;
        self
    }
    /// The delegate implementation is consulted whenever there is an intermediate result, or if something goes wrong
    /// while executing the actual API request.
    /// 
    /// It should be used to handle progress information, and to implement a certain level of resilience.
    ///
    /// Sets the *delegate* property to the given value.
    pub fn delegate(mut self, new_value: &'a mut Delegate) -> CourseCreateCall<'a, C, A> {
        self._delegate = Some(new_value);
        self
    }

    /// Set any additional parameter of the query string used in the request.
    /// It should be used to set parameters which are not yet available through their own
    /// setters.
    ///
    /// Please note that this method must not be used to set any of the known paramters
    /// which have their own setter method. If done anyway, the request will fail.
    ///
    /// # Additional Parameters
    ///
    /// * *upload_protocol* (query-string) - Upload protocol for media (e.g. "raw", "multipart").
    /// * *pp* (query-boolean) - Pretty-print response.
    /// * *prettyPrint* (query-boolean) - Returns response with indentations and line breaks.
    /// * *bearer_token* (query-string) - OAuth bearer token.
    /// * *access_token* (query-string) - OAuth access token.
    /// * *uploadType* (query-string) - Legacy upload protocol for media (e.g. "media", "multipart").
    /// * *quotaUser* (query-string) - Available to use for quota purposes for server-side applications. Can be any arbitrary string assigned to a user, but should not exceed 40 characters.
    /// * *callback* (query-string) - JSONP
    /// * *oauth_token* (query-string) - OAuth 2.0 token for the current user.
    /// * *key* (query-string) - API key. Your API key identifies your project and provides you with API access, quota, and reports. Required unless you provide an OAuth 2.0 token.
    /// * *fields* (query-string) - Selector specifying which fields to include in a partial response.
    /// * *alt* (query-string) - Data format for response.
    /// * *$.xgafv* (query-string) - V1 error format.
    pub fn param<T>(mut self, name: T, value: T) -> CourseCreateCall<'a, C, A>
                                                        where T: AsRef<str> {
        self._additional_params.insert(name.as_ref().to_string(), value.as_ref().to_string());
        self
    }

    /// Identifies the authorization scope for the method you are building.
    ///
    /// Use this method to actively specify which scope should be used, instead the default `Scope` variant
    /// `Scope::Course`.
    ///
    /// The `scope` will be added to a set of scopes. This is important as one can maintain access
    /// tokens for more than one scope.
    /// If `None` is specified, then all scopes will be removed and no default scope will be used either.
    /// In that case, you have to specify your API-key using the `key` parameter (see the `param()`
    /// function for details).
    ///
    /// Usually there is more than one suitable scope to authorize an operation, some of which may
    /// encompass more rights than others. For example, for listing resources, a *read-only* scope will be
    /// sufficient, a read-write scope will do as well.
    pub fn add_scope<T, S>(mut self, scope: T) -> CourseCreateCall<'a, C, A>
                                                        where T: Into<Option<S>>,
                                                              S: AsRef<str> {
        match scope.into() {
          Some(scope) => self._scopes.insert(scope.as_ref().to_string(), ()),
          None => None,
        };
        self
    }
}


/// Returns a list of students of this course that the requester
/// is permitted to view.
/// 
/// This method returns the following error codes:
/// 
/// * `NOT_FOUND` if the course does not exist.
/// * `PERMISSION_DENIED` for access errors.
///
/// A builder for the *students.list* method supported by a *course* resource.
/// It is not used directly, but through a `CourseMethods` instance.
///
/// # Example
///
/// Instantiate a resource method builder
///
/// ```test_harness,no_run
/// # extern crate hyper;
/// # extern crate hyper_rustls;
/// # extern crate yup_oauth2 as oauth2;
/// # extern crate google_classroom1 as classroom1;
/// # #[test] fn egal() {
/// # use std::default::Default;
/// # use oauth2::{Authenticator, DefaultAuthenticatorDelegate, ApplicationSecret, MemoryStorage};
/// # use classroom1::Classroom;
/// 
/// # let secret: ApplicationSecret = Default::default();
/// # let auth = Authenticator::new(&secret, DefaultAuthenticatorDelegate,
/// #                               hyper::Client::with_connector(hyper::net::HttpsConnector::new(hyper_rustls::TlsClient::new())),
/// #                               <MemoryStorage as Default>::default(), None);
/// # let mut hub = Classroom::new(hyper::Client::with_connector(hyper::net::HttpsConnector::new(hyper_rustls::TlsClient::new())), auth);
/// // You can configure optional parameters by calling the respective setters at will, and
/// // execute the final call using `doit()`.
/// // Values shown here are possibly random and not representative !
/// let result = hub.courses().students_list("courseId")
///              .page_token("vero")
///              .page_size(-95)
///              .doit();
/// # }
/// ```
pub struct CourseStudentListCall<'a, C, A>
    where C: 'a, A: 'a {

    hub: &'a Classroom<C, A>,
    _course_id: String,
    _page_token: Option<String>,
    _page_size: Option<i32>,
    _delegate: Option<&'a mut Delegate>,
    _additional_params: HashMap<String, String>,
    _scopes: BTreeMap<String, ()>
}

impl<'a, C, A> CallBuilder for CourseStudentListCall<'a, C, A> {}

impl<'a, C, A> CourseStudentListCall<'a, C, A> where C: BorrowMut<hyper::Client>, A: oauth2::GetToken {


    /// Perform the operation you have build so far.
    pub fn doit(mut self) -> Result<(hyper::client::Response, ListStudentsResponse)> {
        use std::io::{Read, Seek};
        use hyper::header::{ContentType, ContentLength, Authorization, Bearer, UserAgent, Location};
        let mut dd = DefaultDelegate;
        let mut dlg: &mut Delegate = match self._delegate {
            Some(d) => d,
            None => &mut dd
        };
        dlg.begin(MethodInfo { id: "classroom.courses.students.list",
                               http_method: hyper::method::Method::Get });
        let mut params: Vec<(&str, String)> = Vec::with_capacity((5 + self._additional_params.len()));
        params.push(("courseId", self._course_id.to_string()));
        if let Some(value) = self._page_token {
            params.push(("pageToken", value.to_string()));
        }
        if let Some(value) = self._page_size {
            params.push(("pageSize", value.to_string()));
        }
        for &field in ["alt", "courseId", "pageToken", "pageSize"].iter() {
            if self._additional_params.contains_key(field) {
                dlg.finished(false);
                return Err(Error::FieldClash(field));
            }
        }
        for (name, value) in self._additional_params.iter() {
            params.push((&name, value.clone()));
        }

        params.push(("alt", "json".to_string()));

        let mut url = self.hub._base_url.clone() + "v1/courses/{courseId}/students";
        if self._scopes.len() == 0 {
            self._scopes.insert(Scope::RosterReadonly.as_ref().to_string(), ());
        }

        for &(find_this, param_name) in [("{courseId}", "courseId")].iter() {
            let mut replace_with: Option<&str> = None;
            for &(name, ref value) in params.iter() {
                if name == param_name {
                    replace_with = Some(value);
                    break;
                }
            }
            url = url.replace(find_this, replace_with.expect("to find substitution value in params"));
        }
        {
            let mut indices_for_removal: Vec<usize> = Vec::with_capacity(1);
            for param_name in ["courseId"].iter() {
                if let Some(index) = params.iter().position(|t| &t.0 == param_name) {
                    indices_for_removal.push(index);
                }
            }
            for &index in indices_for_removal.iter() {
                params.remove(index);
            }
        }

        if params.len() > 0 {
            url.push('?');
            url.push_str(&url::form_urlencoded::serialize(params));
        }



        loop {
            let token = match self.hub.auth.borrow_mut().token(self._scopes.keys()) {
                Ok(token) => token,
                Err(err) => {
                    match  dlg.token(&*err) {
                        Some(token) => token,
                        None => {
                            dlg.finished(false);
                            return Err(Error::MissingToken(err))
                        }
                    }
                }
            };
            let auth_header = Authorization(Bearer { token: token.access_token });
            let mut req_result = {
                let mut client = &mut *self.hub.client.borrow_mut();
                let mut req = client.borrow_mut().request(hyper::method::Method::Get, &url)
                    .header(UserAgent(self.hub._user_agent.clone()))
                    .header(auth_header.clone());

                dlg.pre_request();
                req.send()
            };

            match req_result {
                Err(err) => {
                    if let oauth2::Retry::After(d) = dlg.http_error(&err) {
                        sleep(d);
                        continue;
                    }
                    dlg.finished(false);
                    return Err(Error::HttpError(err))
                }
                Ok(mut res) => {
                    if !res.status.is_success() {
                        let mut json_err = String::new();
                        res.read_to_string(&mut json_err).unwrap();
                        if let oauth2::Retry::After(d) = dlg.http_failure(&res,
                                                              json::from_str(&json_err).ok(),
                                                              json::from_str(&json_err).ok()) {
                            sleep(d);
                            continue;
                        }
                        dlg.finished(false);
                        return match json::from_str::<ErrorResponse>(&json_err){
                            Err(_) => Err(Error::Failure(res)),
                            Ok(serr) => Err(Error::BadRequest(serr))
                        }
                    }
                    let result_value = {
                        let mut json_response = String::new();
                        res.read_to_string(&mut json_response).unwrap();
                        match json::from_str(&json_response) {
                            Ok(decoded) => (res, decoded),
                            Err(err) => {
                                dlg.response_json_decode_error(&json_response, &err);
                                return Err(Error::JsonDecodeError(json_response, err));
                            }
                        }
                    };

                    dlg.finished(true);
                    return Ok(result_value)
                }
            }
        }
    }


    /// Identifier of the course.
    /// This identifier can be either the Classroom-assigned identifier or an
    /// alias.
    ///
    /// Sets the *course id* path property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call,
    /// we provide this method for API completeness.
    pub fn course_id(mut self, new_value: &str) -> CourseStudentListCall<'a, C, A> {
        self._course_id = new_value.to_string();
        self
    }
    /// nextPageToken
    /// value returned from a previous
    /// list call, indicating that
    /// the subsequent page of results should be returned.
    /// 
    /// The list request must be
    /// otherwise identical to the one that resulted in this token.
    ///
    /// Sets the *page token* query property to the given value.
    pub fn page_token(mut self, new_value: &str) -> CourseStudentListCall<'a, C, A> {
        self._page_token = Some(new_value.to_string());
        self
    }
    /// Maximum number of items to return. Zero means no maximum.
    /// 
    /// The server may return fewer than the specified number of results.
    ///
    /// Sets the *page size* query property to the given value.
    pub fn page_size(mut self, new_value: i32) -> CourseStudentListCall<'a, C, A> {
        self._page_size = Some(new_value);
        self
    }
    /// The delegate implementation is consulted whenever there is an intermediate result, or if something goes wrong
    /// while executing the actual API request.
    /// 
    /// It should be used to handle progress information, and to implement a certain level of resilience.
    ///
    /// Sets the *delegate* property to the given value.
    pub fn delegate(mut self, new_value: &'a mut Delegate) -> CourseStudentListCall<'a, C, A> {
        self._delegate = Some(new_value);
        self
    }

    /// Set any additional parameter of the query string used in the request.
    /// It should be used to set parameters which are not yet available through their own
    /// setters.
    ///
    /// Please note that this method must not be used to set any of the known paramters
    /// which have their own setter method. If done anyway, the request will fail.
    ///
    /// # Additional Parameters
    ///
    /// * *upload_protocol* (query-string) - Upload protocol for media (e.g. "raw", "multipart").
    /// * *pp* (query-boolean) - Pretty-print response.
    /// * *prettyPrint* (query-boolean) - Returns response with indentations and line breaks.
    /// * *bearer_token* (query-string) - OAuth bearer token.
    /// * *access_token* (query-string) - OAuth access token.
    /// * *uploadType* (query-string) - Legacy upload protocol for media (e.g. "media", "multipart").
    /// * *quotaUser* (query-string) - Available to use for quota purposes for server-side applications. Can be any arbitrary string assigned to a user, but should not exceed 40 characters.
    /// * *callback* (query-string) - JSONP
    /// * *oauth_token* (query-string) - OAuth 2.0 token for the current user.
    /// * *key* (query-string) - API key. Your API key identifies your project and provides you with API access, quota, and reports. Required unless you provide an OAuth 2.0 token.
    /// * *fields* (query-string) - Selector specifying which fields to include in a partial response.
    /// * *alt* (query-string) - Data format for response.
    /// * *$.xgafv* (query-string) - V1 error format.
    pub fn param<T>(mut self, name: T, value: T) -> CourseStudentListCall<'a, C, A>
                                                        where T: AsRef<str> {
        self._additional_params.insert(name.as_ref().to_string(), value.as_ref().to_string());
        self
    }

    /// Identifies the authorization scope for the method you are building.
    ///
    /// Use this method to actively specify which scope should be used, instead the default `Scope` variant
    /// `Scope::RosterReadonly`.
    ///
    /// The `scope` will be added to a set of scopes. This is important as one can maintain access
    /// tokens for more than one scope.
    /// If `None` is specified, then all scopes will be removed and no default scope will be used either.
    /// In that case, you have to specify your API-key using the `key` parameter (see the `param()`
    /// function for details).
    ///
    /// Usually there is more than one suitable scope to authorize an operation, some of which may
    /// encompass more rights than others. For example, for listing resources, a *read-only* scope will be
    /// sufficient, a read-write scope will do as well.
    pub fn add_scope<T, S>(mut self, scope: T) -> CourseStudentListCall<'a, C, A>
                                                        where T: Into<Option<S>>,
                                                              S: AsRef<str> {
        match scope.into() {
          Some(scope) => self._scopes.insert(scope.as_ref().to_string(), ()),
          None => None,
        };
        self
    }
}


/// Deletes a course.
/// 
/// This method returns the following error codes:
/// 
/// * `PERMISSION_DENIED` if the requesting user is not permitted to delete the
/// requested course or for access errors.
/// * `NOT_FOUND` if no course exists with the requested ID.
///
/// A builder for the *delete* method supported by a *course* resource.
/// It is not used directly, but through a `CourseMethods` instance.
///
/// # Example
///
/// Instantiate a resource method builder
///
/// ```test_harness,no_run
/// # extern crate hyper;
/// # extern crate hyper_rustls;
/// # extern crate yup_oauth2 as oauth2;
/// # extern crate google_classroom1 as classroom1;
/// # #[test] fn egal() {
/// # use std::default::Default;
/// # use oauth2::{Authenticator, DefaultAuthenticatorDelegate, ApplicationSecret, MemoryStorage};
/// # use classroom1::Classroom;
/// 
/// # let secret: ApplicationSecret = Default::default();
/// # let auth = Authenticator::new(&secret, DefaultAuthenticatorDelegate,
/// #                               hyper::Client::with_connector(hyper::net::HttpsConnector::new(hyper_rustls::TlsClient::new())),
/// #                               <MemoryStorage as Default>::default(), None);
/// # let mut hub = Classroom::new(hyper::Client::with_connector(hyper::net::HttpsConnector::new(hyper_rustls::TlsClient::new())), auth);
/// // You can configure optional parameters by calling the respective setters at will, and
/// // execute the final call using `doit()`.
/// // Values shown here are possibly random and not representative !
/// let result = hub.courses().delete("id")
///              .doit();
/// # }
/// ```
pub struct CourseDeleteCall<'a, C, A>
    where C: 'a, A: 'a {

    hub: &'a Classroom<C, A>,
    _id: String,
    _delegate: Option<&'a mut Delegate>,
    _additional_params: HashMap<String, String>,
    _scopes: BTreeMap<String, ()>
}

impl<'a, C, A> CallBuilder for CourseDeleteCall<'a, C, A> {}

impl<'a, C, A> CourseDeleteCall<'a, C, A> where C: BorrowMut<hyper::Client>, A: oauth2::GetToken {


    /// Perform the operation you have build so far.
    pub fn doit(mut self) -> Result<(hyper::client::Response, Empty)> {
        use std::io::{Read, Seek};
        use hyper::header::{ContentType, ContentLength, Authorization, Bearer, UserAgent, Location};
        let mut dd = DefaultDelegate;
        let mut dlg: &mut Delegate = match self._delegate {
            Some(d) => d,
            None => &mut dd
        };
        dlg.begin(MethodInfo { id: "classroom.courses.delete",
                               http_method: hyper::method::Method::Delete });
        let mut params: Vec<(&str, String)> = Vec::with_capacity((3 + self._additional_params.len()));
        params.push(("id", self._id.to_string()));
        for &field in ["alt", "id"].iter() {
            if self._additional_params.contains_key(field) {
                dlg.finished(false);
                return Err(Error::FieldClash(field));
            }
        }
        for (name, value) in self._additional_params.iter() {
            params.push((&name, value.clone()));
        }

        params.push(("alt", "json".to_string()));

        let mut url = self.hub._base_url.clone() + "v1/courses/{id}";
        if self._scopes.len() == 0 {
            self._scopes.insert(Scope::Course.as_ref().to_string(), ());
        }

        for &(find_this, param_name) in [("{id}", "id")].iter() {
            let mut replace_with: Option<&str> = None;
            for &(name, ref value) in params.iter() {
                if name == param_name {
                    replace_with = Some(value);
                    break;
                }
            }
            url = url.replace(find_this, replace_with.expect("to find substitution value in params"));
        }
        {
            let mut indices_for_removal: Vec<usize> = Vec::with_capacity(1);
            for param_name in ["id"].iter() {
                if let Some(index) = params.iter().position(|t| &t.0 == param_name) {
                    indices_for_removal.push(index);
                }
            }
            for &index in indices_for_removal.iter() {
                params.remove(index);
            }
        }

        if params.len() > 0 {
            url.push('?');
            url.push_str(&url::form_urlencoded::serialize(params));
        }



        loop {
            let token = match self.hub.auth.borrow_mut().token(self._scopes.keys()) {
                Ok(token) => token,
                Err(err) => {
                    match  dlg.token(&*err) {
                        Some(token) => token,
                        None => {
                            dlg.finished(false);
                            return Err(Error::MissingToken(err))
                        }
                    }
                }
            };
            let auth_header = Authorization(Bearer { token: token.access_token });
            let mut req_result = {
                let mut client = &mut *self.hub.client.borrow_mut();
                let mut req = client.borrow_mut().request(hyper::method::Method::Delete, &url)
                    .header(UserAgent(self.hub._user_agent.clone()))
                    .header(auth_header.clone());

                dlg.pre_request();
                req.send()
            };

            match req_result {
                Err(err) => {
                    if let oauth2::Retry::After(d) = dlg.http_error(&err) {
                        sleep(d);
                        continue;
                    }
                    dlg.finished(false);
                    return Err(Error::HttpError(err))
                }
                Ok(mut res) => {
                    if !res.status.is_success() {
                        let mut json_err = String::new();
                        res.read_to_string(&mut json_err).unwrap();
                        if let oauth2::Retry::After(d) = dlg.http_failure(&res,
                                                              json::from_str(&json_err).ok(),
                                                              json::from_str(&json_err).ok()) {
                            sleep(d);
                            continue;
                        }
                        dlg.finished(false);
                        return match json::from_str::<ErrorResponse>(&json_err){
                            Err(_) => Err(Error::Failure(res)),
                            Ok(serr) => Err(Error::BadRequest(serr))
                        }
                    }
                    let result_value = {
                        let mut json_response = String::new();
                        res.read_to_string(&mut json_response).unwrap();
                        match json::from_str(&json_response) {
                            Ok(decoded) => (res, decoded),
                            Err(err) => {
                                dlg.response_json_decode_error(&json_response, &err);
                                return Err(Error::JsonDecodeError(json_response, err));
                            }
                        }
                    };

                    dlg.finished(true);
                    return Ok(result_value)
                }
            }
        }
    }


    /// Identifier of the course to delete.
    /// This identifier can be either the Classroom-assigned identifier or an
    /// alias.
    ///
    /// Sets the *id* path property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call,
    /// we provide this method for API completeness.
    pub fn id(mut self, new_value: &str) -> CourseDeleteCall<'a, C, A> {
        self._id = new_value.to_string();
        self
    }
    /// The delegate implementation is consulted whenever there is an intermediate result, or if something goes wrong
    /// while executing the actual API request.
    /// 
    /// It should be used to handle progress information, and to implement a certain level of resilience.
    ///
    /// Sets the *delegate* property to the given value.
    pub fn delegate(mut self, new_value: &'a mut Delegate) -> CourseDeleteCall<'a, C, A> {
        self._delegate = Some(new_value);
        self
    }

    /// Set any additional parameter of the query string used in the request.
    /// It should be used to set parameters which are not yet available through their own
    /// setters.
    ///
    /// Please note that this method must not be used to set any of the known paramters
    /// which have their own setter method. If done anyway, the request will fail.
    ///
    /// # Additional Parameters
    ///
    /// * *upload_protocol* (query-string) - Upload protocol for media (e.g. "raw", "multipart").
    /// * *pp* (query-boolean) - Pretty-print response.
    /// * *prettyPrint* (query-boolean) - Returns response with indentations and line breaks.
    /// * *bearer_token* (query-string) - OAuth bearer token.
    /// * *access_token* (query-string) - OAuth access token.
    /// * *uploadType* (query-string) - Legacy upload protocol for media (e.g. "media", "multipart").
    /// * *quotaUser* (query-string) - Available to use for quota purposes for server-side applications. Can be any arbitrary string assigned to a user, but should not exceed 40 characters.
    /// * *callback* (query-string) - JSONP
    /// * *oauth_token* (query-string) - OAuth 2.0 token for the current user.
    /// * *key* (query-string) - API key. Your API key identifies your project and provides you with API access, quota, and reports. Required unless you provide an OAuth 2.0 token.
    /// * *fields* (query-string) - Selector specifying which fields to include in a partial response.
    /// * *alt* (query-string) - Data format for response.
    /// * *$.xgafv* (query-string) - V1 error format.
    pub fn param<T>(mut self, name: T, value: T) -> CourseDeleteCall<'a, C, A>
                                                        where T: AsRef<str> {
        self._additional_params.insert(name.as_ref().to_string(), value.as_ref().to_string());
        self
    }

    /// Identifies the authorization scope for the method you are building.
    ///
    /// Use this method to actively specify which scope should be used, instead the default `Scope` variant
    /// `Scope::Course`.
    ///
    /// The `scope` will be added to a set of scopes. This is important as one can maintain access
    /// tokens for more than one scope.
    /// If `None` is specified, then all scopes will be removed and no default scope will be used either.
    /// In that case, you have to specify your API-key using the `key` parameter (see the `param()`
    /// function for details).
    ///
    /// Usually there is more than one suitable scope to authorize an operation, some of which may
    /// encompass more rights than others. For example, for listing resources, a *read-only* scope will be
    /// sufficient, a read-write scope will do as well.
    pub fn add_scope<T, S>(mut self, scope: T) -> CourseDeleteCall<'a, C, A>
                                                        where T: Into<Option<S>>,
                                                              S: AsRef<str> {
        match scope.into() {
          Some(scope) => self._scopes.insert(scope.as_ref().to_string(), ()),
          None => None,
        };
        self
    }
}


/// Updates one or more fields of a course work.
/// 
/// See google.classroom.v1.CourseWork for details
/// of which fields may be updated and who may change them.
/// 
/// This request must be made by the Developer Console project of the
/// [OAuth client ID](https://support.google.com/cloud/answer/6158849) used to
/// create the corresponding course work item.
/// 
/// This method returns the following error codes:
/// 
/// * `PERMISSION_DENIED` if the requesting developer project did not create
/// the corresponding course work, if the user is not permitted to make the
/// requested modification to the student submission, or for
/// access errors.
/// * `INVALID_ARGUMENT` if the request is malformed.
/// * `FAILED_PRECONDITION` if the requested course work has already been
/// deleted.
/// * `NOT_FOUND` if the requested course, course work, or student submission
/// does not exist.
///
/// A builder for the *courseWork.patch* method supported by a *course* resource.
/// It is not used directly, but through a `CourseMethods` instance.
///
/// # Example
///
/// Instantiate a resource method builder
///
/// ```test_harness,no_run
/// # extern crate hyper;
/// # extern crate hyper_rustls;
/// # extern crate yup_oauth2 as oauth2;
/// # extern crate google_classroom1 as classroom1;
/// use classroom1::CourseWork;
/// # #[test] fn egal() {
/// # use std::default::Default;
/// # use oauth2::{Authenticator, DefaultAuthenticatorDelegate, ApplicationSecret, MemoryStorage};
/// # use classroom1::Classroom;
/// 
/// # let secret: ApplicationSecret = Default::default();
/// # let auth = Authenticator::new(&secret, DefaultAuthenticatorDelegate,
/// #                               hyper::Client::with_connector(hyper::net::HttpsConnector::new(hyper_rustls::TlsClient::new())),
/// #                               <MemoryStorage as Default>::default(), None);
/// # let mut hub = Classroom::new(hyper::Client::with_connector(hyper::net::HttpsConnector::new(hyper_rustls::TlsClient::new())), auth);
/// // As the method needs a request, you would usually fill it with the desired information
/// // into the respective structure. Some of the parts shown here might not be applicable !
/// // Values shown here are possibly random and not representative !
/// let mut req = CourseWork::default();
/// 
/// // You can configure optional parameters by calling the respective setters at will, and
/// // execute the final call using `doit()`.
/// // Values shown here are possibly random and not representative !
/// let result = hub.courses().course_work_patch(req, "courseId", "id")
///              .update_mask("duo")
///              .doit();
/// # }
/// ```
pub struct CourseCourseWorkPatchCall<'a, C, A>
    where C: 'a, A: 'a {

    hub: &'a Classroom<C, A>,
    _request: CourseWork,
    _course_id: String,
    _id: String,
    _update_mask: Option<String>,
    _delegate: Option<&'a mut Delegate>,
    _additional_params: HashMap<String, String>,
    _scopes: BTreeMap<String, ()>
}

impl<'a, C, A> CallBuilder for CourseCourseWorkPatchCall<'a, C, A> {}

impl<'a, C, A> CourseCourseWorkPatchCall<'a, C, A> where C: BorrowMut<hyper::Client>, A: oauth2::GetToken {


    /// Perform the operation you have build so far.
    pub fn doit(mut self) -> Result<(hyper::client::Response, CourseWork)> {
        use std::io::{Read, Seek};
        use hyper::header::{ContentType, ContentLength, Authorization, Bearer, UserAgent, Location};
        let mut dd = DefaultDelegate;
        let mut dlg: &mut Delegate = match self._delegate {
            Some(d) => d,
            None => &mut dd
        };
        dlg.begin(MethodInfo { id: "classroom.courses.courseWork.patch",
                               http_method: hyper::method::Method::Patch });
        let mut params: Vec<(&str, String)> = Vec::with_capacity((6 + self._additional_params.len()));
        params.push(("courseId", self._course_id.to_string()));
        params.push(("id", self._id.to_string()));
        if let Some(value) = self._update_mask {
            params.push(("updateMask", value.to_string()));
        }
        for &field in ["alt", "courseId", "id", "updateMask"].iter() {
            if self._additional_params.contains_key(field) {
                dlg.finished(false);
                return Err(Error::FieldClash(field));
            }
        }
        for (name, value) in self._additional_params.iter() {
            params.push((&name, value.clone()));
        }

        params.push(("alt", "json".to_string()));

        let mut url = self.hub._base_url.clone() + "v1/courses/{courseId}/courseWork/{id}";
        if self._scopes.len() == 0 {
            self._scopes.insert(Scope::CourseworkStudent.as_ref().to_string(), ());
        }

        for &(find_this, param_name) in [("{courseId}", "courseId"), ("{id}", "id")].iter() {
            let mut replace_with: Option<&str> = None;
            for &(name, ref value) in params.iter() {
                if name == param_name {
                    replace_with = Some(value);
                    break;
                }
            }
            url = url.replace(find_this, replace_with.expect("to find substitution value in params"));
        }
        {
            let mut indices_for_removal: Vec<usize> = Vec::with_capacity(2);
            for param_name in ["id", "courseId"].iter() {
                if let Some(index) = params.iter().position(|t| &t.0 == param_name) {
                    indices_for_removal.push(index);
                }
            }
            for &index in indices_for_removal.iter() {
                params.remove(index);
            }
        }

        if params.len() > 0 {
            url.push('?');
            url.push_str(&url::form_urlencoded::serialize(params));
        }

        let mut json_mime_type = mime::Mime(mime::TopLevel::Application, mime::SubLevel::Json, Default::default());
        let mut request_value_reader =
            {
                let mut value = json::value::to_value(&self._request).expect("serde to work");
                remove_json_null_values(&mut value);
                let mut dst = io::Cursor::new(Vec::with_capacity(128));
                json::to_writer(&mut dst, &value).unwrap();
                dst
            };
        let request_size = request_value_reader.seek(io::SeekFrom::End(0)).unwrap();
        request_value_reader.seek(io::SeekFrom::Start(0)).unwrap();


        loop {
            let token = match self.hub.auth.borrow_mut().token(self._scopes.keys()) {
                Ok(token) => token,
                Err(err) => {
                    match  dlg.token(&*err) {
                        Some(token) => token,
                        None => {
                            dlg.finished(false);
                            return Err(Error::MissingToken(err))
                        }
                    }
                }
            };
            let auth_header = Authorization(Bearer { token: token.access_token });
            request_value_reader.seek(io::SeekFrom::Start(0)).unwrap();
            let mut req_result = {
                let mut client = &mut *self.hub.client.borrow_mut();
                let mut req = client.borrow_mut().request(hyper::method::Method::Patch, &url)
                    .header(UserAgent(self.hub._user_agent.clone()))
                    .header(auth_header.clone())
                    .header(ContentType(json_mime_type.clone()))
                    .header(ContentLength(request_size as u64))
                    .body(&mut request_value_reader);

                dlg.pre_request();
                req.send()
            };

            match req_result {
                Err(err) => {
                    if let oauth2::Retry::After(d) = dlg.http_error(&err) {
                        sleep(d);
                        continue;
                    }
                    dlg.finished(false);
                    return Err(Error::HttpError(err))
                }
                Ok(mut res) => {
                    if !res.status.is_success() {
                        let mut json_err = String::new();
                        res.read_to_string(&mut json_err).unwrap();
                        if let oauth2::Retry::After(d) = dlg.http_failure(&res,
                                                              json::from_str(&json_err).ok(),
                                                              json::from_str(&json_err).ok()) {
                            sleep(d);
                            continue;
                        }
                        dlg.finished(false);
                        return match json::from_str::<ErrorResponse>(&json_err){
                            Err(_) => Err(Error::Failure(res)),
                            Ok(serr) => Err(Error::BadRequest(serr))
                        }
                    }
                    let result_value = {
                        let mut json_response = String::new();
                        res.read_to_string(&mut json_response).unwrap();
                        match json::from_str(&json_response) {
                            Ok(decoded) => (res, decoded),
                            Err(err) => {
                                dlg.response_json_decode_error(&json_response, &err);
                                return Err(Error::JsonDecodeError(json_response, err));
                            }
                        }
                    };

                    dlg.finished(true);
                    return Ok(result_value)
                }
            }
        }
    }


    ///
    /// Sets the *request* property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call,
    /// we provide this method for API completeness.
    pub fn request(mut self, new_value: CourseWork) -> CourseCourseWorkPatchCall<'a, C, A> {
        self._request = new_value;
        self
    }
    /// Identifier of the course.
    /// This identifier can be either the Classroom-assigned identifier or an
    /// alias.
    ///
    /// Sets the *course id* path property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call,
    /// we provide this method for API completeness.
    pub fn course_id(mut self, new_value: &str) -> CourseCourseWorkPatchCall<'a, C, A> {
        self._course_id = new_value.to_string();
        self
    }
    /// Identifier of the course work.
    ///
    /// Sets the *id* path property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call,
    /// we provide this method for API completeness.
    pub fn id(mut self, new_value: &str) -> CourseCourseWorkPatchCall<'a, C, A> {
        self._id = new_value.to_string();
        self
    }
    /// Mask that identifies which fields on the course work to update.
    /// This field is required to do an update. The update fails if invalid
    /// fields are specified. If a field supports empty values, it can be cleared
    /// by specifying it in the update mask and not in the CourseWork object. If a
    /// field that does not support empty values is included in the update mask and
    /// not set in the CourseWork object, an `INVALID_ARGUMENT` error will be
    /// returned.
    /// 
    /// The following fields may be specified by teachers:
    /// * `title`
    /// * `description`
    /// * `state`
    /// * `due_date`
    /// * `due_time`
    /// * `max_points`
    /// * `submission_modification_mode`
    ///
    /// Sets the *update mask* query property to the given value.
    pub fn update_mask(mut self, new_value: &str) -> CourseCourseWorkPatchCall<'a, C, A> {
        self._update_mask = Some(new_value.to_string());
        self
    }
    /// The delegate implementation is consulted whenever there is an intermediate result, or if something goes wrong
    /// while executing the actual API request.
    /// 
    /// It should be used to handle progress information, and to implement a certain level of resilience.
    ///
    /// Sets the *delegate* property to the given value.
    pub fn delegate(mut self, new_value: &'a mut Delegate) -> CourseCourseWorkPatchCall<'a, C, A> {
        self._delegate = Some(new_value);
        self
    }

    /// Set any additional parameter of the query string used in the request.
    /// It should be used to set parameters which are not yet available through their own
    /// setters.
    ///
    /// Please note that this method must not be used to set any of the known paramters
    /// which have their own setter method. If done anyway, the request will fail.
    ///
    /// # Additional Parameters
    ///
    /// * *upload_protocol* (query-string) - Upload protocol for media (e.g. "raw", "multipart").
    /// * *pp* (query-boolean) - Pretty-print response.
    /// * *prettyPrint* (query-boolean) - Returns response with indentations and line breaks.
    /// * *bearer_token* (query-string) - OAuth bearer token.
    /// * *access_token* (query-string) - OAuth access token.
    /// * *uploadType* (query-string) - Legacy upload protocol for media (e.g. "media", "multipart").
    /// * *quotaUser* (query-string) - Available to use for quota purposes for server-side applications. Can be any arbitrary string assigned to a user, but should not exceed 40 characters.
    /// * *callback* (query-string) - JSONP
    /// * *oauth_token* (query-string) - OAuth 2.0 token for the current user.
    /// * *key* (query-string) - API key. Your API key identifies your project and provides you with API access, quota, and reports. Required unless you provide an OAuth 2.0 token.
    /// * *fields* (query-string) - Selector specifying which fields to include in a partial response.
    /// * *alt* (query-string) - Data format for response.
    /// * *$.xgafv* (query-string) - V1 error format.
    pub fn param<T>(mut self, name: T, value: T) -> CourseCourseWorkPatchCall<'a, C, A>
                                                        where T: AsRef<str> {
        self._additional_params.insert(name.as_ref().to_string(), value.as_ref().to_string());
        self
    }

    /// Identifies the authorization scope for the method you are building.
    ///
    /// Use this method to actively specify which scope should be used, instead the default `Scope` variant
    /// `Scope::CourseworkStudent`.
    ///
    /// The `scope` will be added to a set of scopes. This is important as one can maintain access
    /// tokens for more than one scope.
    /// If `None` is specified, then all scopes will be removed and no default scope will be used either.
    /// In that case, you have to specify your API-key using the `key` parameter (see the `param()`
    /// function for details).
    ///
    /// Usually there is more than one suitable scope to authorize an operation, some of which may
    /// encompass more rights than others. For example, for listing resources, a *read-only* scope will be
    /// sufficient, a read-write scope will do as well.
    pub fn add_scope<T, S>(mut self, scope: T) -> CourseCourseWorkPatchCall<'a, C, A>
                                                        where T: Into<Option<S>>,
                                                              S: AsRef<str> {
        match scope.into() {
          Some(scope) => self._scopes.insert(scope.as_ref().to_string(), ()),
          None => None,
        };
        self
    }
}


/// Updates one or more fields in a course.
/// 
/// This method returns the following error codes:
/// 
/// * `PERMISSION_DENIED` if the requesting user is not permitted to modify the
/// requested course or for access errors.
/// * `NOT_FOUND` if no course exists with the requested ID.
/// * `INVALID_ARGUMENT` if invalid fields are specified in the update mask or
/// if no update mask is supplied.
/// * `FAILED_PRECONDITION` for the following request errors:
///     * CourseNotModifiable
///
/// A builder for the *patch* method supported by a *course* resource.
/// It is not used directly, but through a `CourseMethods` instance.
///
/// # Example
///
/// Instantiate a resource method builder
///
/// ```test_harness,no_run
/// # extern crate hyper;
/// # extern crate hyper_rustls;
/// # extern crate yup_oauth2 as oauth2;
/// # extern crate google_classroom1 as classroom1;
/// use classroom1::Course;
/// # #[test] fn egal() {
/// # use std::default::Default;
/// # use oauth2::{Authenticator, DefaultAuthenticatorDelegate, ApplicationSecret, MemoryStorage};
/// # use classroom1::Classroom;
/// 
/// # let secret: ApplicationSecret = Default::default();
/// # let auth = Authenticator::new(&secret, DefaultAuthenticatorDelegate,
/// #                               hyper::Client::with_connector(hyper::net::HttpsConnector::new(hyper_rustls::TlsClient::new())),
/// #                               <MemoryStorage as Default>::default(), None);
/// # let mut hub = Classroom::new(hyper::Client::with_connector(hyper::net::HttpsConnector::new(hyper_rustls::TlsClient::new())), auth);
/// // As the method needs a request, you would usually fill it with the desired information
/// // into the respective structure. Some of the parts shown here might not be applicable !
/// // Values shown here are possibly random and not representative !
/// let mut req = Course::default();
/// 
/// // You can configure optional parameters by calling the respective setters at will, and
/// // execute the final call using `doit()`.
/// // Values shown here are possibly random and not representative !
/// let result = hub.courses().patch(req, "id")
///              .update_mask("Lorem")
///              .doit();
/// # }
/// ```
pub struct CoursePatchCall<'a, C, A>
    where C: 'a, A: 'a {

    hub: &'a Classroom<C, A>,
    _request: Course,
    _id: String,
    _update_mask: Option<String>,
    _delegate: Option<&'a mut Delegate>,
    _additional_params: HashMap<String, String>,
    _scopes: BTreeMap<String, ()>
}

impl<'a, C, A> CallBuilder for CoursePatchCall<'a, C, A> {}

impl<'a, C, A> CoursePatchCall<'a, C, A> where C: BorrowMut<hyper::Client>, A: oauth2::GetToken {


    /// Perform the operation you have build so far.
    pub fn doit(mut self) -> Result<(hyper::client::Response, Course)> {
        use std::io::{Read, Seek};
        use hyper::header::{ContentType, ContentLength, Authorization, Bearer, UserAgent, Location};
        let mut dd = DefaultDelegate;
        let mut dlg: &mut Delegate = match self._delegate {
            Some(d) => d,
            None => &mut dd
        };
        dlg.begin(MethodInfo { id: "classroom.courses.patch",
                               http_method: hyper::method::Method::Patch });
        let mut params: Vec<(&str, String)> = Vec::with_capacity((5 + self._additional_params.len()));
        params.push(("id", self._id.to_string()));
        if let Some(value) = self._update_mask {
            params.push(("updateMask", value.to_string()));
        }
        for &field in ["alt", "id", "updateMask"].iter() {
            if self._additional_params.contains_key(field) {
                dlg.finished(false);
                return Err(Error::FieldClash(field));
            }
        }
        for (name, value) in self._additional_params.iter() {
            params.push((&name, value.clone()));
        }

        params.push(("alt", "json".to_string()));

        let mut url = self.hub._base_url.clone() + "v1/courses/{id}";
        if self._scopes.len() == 0 {
            self._scopes.insert(Scope::Course.as_ref().to_string(), ());
        }

        for &(find_this, param_name) in [("{id}", "id")].iter() {
            let mut replace_with: Option<&str> = None;
            for &(name, ref value) in params.iter() {
                if name == param_name {
                    replace_with = Some(value);
                    break;
                }
            }
            url = url.replace(find_this, replace_with.expect("to find substitution value in params"));
        }
        {
            let mut indices_for_removal: Vec<usize> = Vec::with_capacity(1);
            for param_name in ["id"].iter() {
                if let Some(index) = params.iter().position(|t| &t.0 == param_name) {
                    indices_for_removal.push(index);
                }
            }
            for &index in indices_for_removal.iter() {
                params.remove(index);
            }
        }

        if params.len() > 0 {
            url.push('?');
            url.push_str(&url::form_urlencoded::serialize(params));
        }

        let mut json_mime_type = mime::Mime(mime::TopLevel::Application, mime::SubLevel::Json, Default::default());
        let mut request_value_reader =
            {
                let mut value = json::value::to_value(&self._request).expect("serde to work");
                remove_json_null_values(&mut value);
                let mut dst = io::Cursor::new(Vec::with_capacity(128));
                json::to_writer(&mut dst, &value).unwrap();
                dst
            };
        let request_size = request_value_reader.seek(io::SeekFrom::End(0)).unwrap();
        request_value_reader.seek(io::SeekFrom::Start(0)).unwrap();


        loop {
            let token = match self.hub.auth.borrow_mut().token(self._scopes.keys()) {
                Ok(token) => token,
                Err(err) => {
                    match  dlg.token(&*err) {
                        Some(token) => token,
                        None => {
                            dlg.finished(false);
                            return Err(Error::MissingToken(err))
                        }
                    }
                }
            };
            let auth_header = Authorization(Bearer { token: token.access_token });
            request_value_reader.seek(io::SeekFrom::Start(0)).unwrap();
            let mut req_result = {
                let mut client = &mut *self.hub.client.borrow_mut();
                let mut req = client.borrow_mut().request(hyper::method::Method::Patch, &url)
                    .header(UserAgent(self.hub._user_agent.clone()))
                    .header(auth_header.clone())
                    .header(ContentType(json_mime_type.clone()))
                    .header(ContentLength(request_size as u64))
                    .body(&mut request_value_reader);

                dlg.pre_request();
                req.send()
            };

            match req_result {
                Err(err) => {
                    if let oauth2::Retry::After(d) = dlg.http_error(&err) {
                        sleep(d);
                        continue;
                    }
                    dlg.finished(false);
                    return Err(Error::HttpError(err))
                }
                Ok(mut res) => {
                    if !res.status.is_success() {
                        let mut json_err = String::new();
                        res.read_to_string(&mut json_err).unwrap();
                        if let oauth2::Retry::After(d) = dlg.http_failure(&res,
                                                              json::from_str(&json_err).ok(),
                                                              json::from_str(&json_err).ok()) {
                            sleep(d);
                            continue;
                        }
                        dlg.finished(false);
                        return match json::from_str::<ErrorResponse>(&json_err){
                            Err(_) => Err(Error::Failure(res)),
                            Ok(serr) => Err(Error::BadRequest(serr))
                        }
                    }
                    let result_value = {
                        let mut json_response = String::new();
                        res.read_to_string(&mut json_response).unwrap();
                        match json::from_str(&json_response) {
                            Ok(decoded) => (res, decoded),
                            Err(err) => {
                                dlg.response_json_decode_error(&json_response, &err);
                                return Err(Error::JsonDecodeError(json_response, err));
                            }
                        }
                    };

                    dlg.finished(true);
                    return Ok(result_value)
                }
            }
        }
    }


    ///
    /// Sets the *request* property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call,
    /// we provide this method for API completeness.
    pub fn request(mut self, new_value: Course) -> CoursePatchCall<'a, C, A> {
        self._request = new_value;
        self
    }
    /// Identifier of the course to update.
    /// This identifier can be either the Classroom-assigned identifier or an
    /// alias.
    ///
    /// Sets the *id* path property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call,
    /// we provide this method for API completeness.
    pub fn id(mut self, new_value: &str) -> CoursePatchCall<'a, C, A> {
        self._id = new_value.to_string();
        self
    }
    /// Mask that identifies which fields on the course to update.
    /// This field is required to do an update. The update will fail if invalid
    /// fields are specified. The following fields are valid:
    /// 
    /// * `name`
    /// * `section`
    /// * `descriptionHeading`
    /// * `description`
    /// * `room`
    /// * `courseState`
    /// 
    /// When set in a query parameter, this field should be specified as
    /// 
    /// `updateMask=<field1>,<field2>,...`
    ///
    /// Sets the *update mask* query property to the given value.
    pub fn update_mask(mut self, new_value: &str) -> CoursePatchCall<'a, C, A> {
        self._update_mask = Some(new_value.to_string());
        self
    }
    /// The delegate implementation is consulted whenever there is an intermediate result, or if something goes wrong
    /// while executing the actual API request.
    /// 
    /// It should be used to handle progress information, and to implement a certain level of resilience.
    ///
    /// Sets the *delegate* property to the given value.
    pub fn delegate(mut self, new_value: &'a mut Delegate) -> CoursePatchCall<'a, C, A> {
        self._delegate = Some(new_value);
        self
    }

    /// Set any additional parameter of the query string used in the request.
    /// It should be used to set parameters which are not yet available through their own
    /// setters.
    ///
    /// Please note that this method must not be used to set any of the known paramters
    /// which have their own setter method. If done anyway, the request will fail.
    ///
    /// # Additional Parameters
    ///
    /// * *upload_protocol* (query-string) - Upload protocol for media (e.g. "raw", "multipart").
    /// * *pp* (query-boolean) - Pretty-print response.
    /// * *prettyPrint* (query-boolean) - Returns response with indentations and line breaks.
    /// * *bearer_token* (query-string) - OAuth bearer token.
    /// * *access_token* (query-string) - OAuth access token.
    /// * *uploadType* (query-string) - Legacy upload protocol for media (e.g. "media", "multipart").
    /// * *quotaUser* (query-string) - Available to use for quota purposes for server-side applications. Can be any arbitrary string assigned to a user, but should not exceed 40 characters.
    /// * *callback* (query-string) - JSONP
    /// * *oauth_token* (query-string) - OAuth 2.0 token for the current user.
    /// * *key* (query-string) - API key. Your API key identifies your project and provides you with API access, quota, and reports. Required unless you provide an OAuth 2.0 token.
    /// * *fields* (query-string) - Selector specifying which fields to include in a partial response.
    /// * *alt* (query-string) - Data format for response.
    /// * *$.xgafv* (query-string) - V1 error format.
    pub fn param<T>(mut self, name: T, value: T) -> CoursePatchCall<'a, C, A>
                                                        where T: AsRef<str> {
        self._additional_params.insert(name.as_ref().to_string(), value.as_ref().to_string());
        self
    }

    /// Identifies the authorization scope for the method you are building.
    ///
    /// Use this method to actively specify which scope should be used, instead the default `Scope` variant
    /// `Scope::Course`.
    ///
    /// The `scope` will be added to a set of scopes. This is important as one can maintain access
    /// tokens for more than one scope.
    /// If `None` is specified, then all scopes will be removed and no default scope will be used either.
    /// In that case, you have to specify your API-key using the `key` parameter (see the `param()`
    /// function for details).
    ///
    /// Usually there is more than one suitable scope to authorize an operation, some of which may
    /// encompass more rights than others. For example, for listing resources, a *read-only* scope will be
    /// sufficient, a read-write scope will do as well.
    pub fn add_scope<T, S>(mut self, scope: T) -> CoursePatchCall<'a, C, A>
                                                        where T: Into<Option<S>>,
                                                              S: AsRef<str> {
        match scope.into() {
          Some(scope) => self._scopes.insert(scope.as_ref().to_string(), ()),
          None => None,
        };
        self
    }
}


/// Deletes a student of a course.
/// 
/// This method returns the following error codes:
/// 
/// * `PERMISSION_DENIED` if the requesting user is not permitted to delete
/// students of this course or for access errors.
/// * `NOT_FOUND` if no student of this course has the requested ID or if the
/// course does not exist.
///
/// A builder for the *students.delete* method supported by a *course* resource.
/// It is not used directly, but through a `CourseMethods` instance.
///
/// # Example
///
/// Instantiate a resource method builder
///
/// ```test_harness,no_run
/// # extern crate hyper;
/// # extern crate hyper_rustls;
/// # extern crate yup_oauth2 as oauth2;
/// # extern crate google_classroom1 as classroom1;
/// # #[test] fn egal() {
/// # use std::default::Default;
/// # use oauth2::{Authenticator, DefaultAuthenticatorDelegate, ApplicationSecret, MemoryStorage};
/// # use classroom1::Classroom;
/// 
/// # let secret: ApplicationSecret = Default::default();
/// # let auth = Authenticator::new(&secret, DefaultAuthenticatorDelegate,
/// #                               hyper::Client::with_connector(hyper::net::HttpsConnector::new(hyper_rustls::TlsClient::new())),
/// #                               <MemoryStorage as Default>::default(), None);
/// # let mut hub = Classroom::new(hyper::Client::with_connector(hyper::net::HttpsConnector::new(hyper_rustls::TlsClient::new())), auth);
/// // You can configure optional parameters by calling the respective setters at will, and
/// // execute the final call using `doit()`.
/// // Values shown here are possibly random and not representative !
/// let result = hub.courses().students_delete("courseId", "userId")
///              .doit();
/// # }
/// ```
pub struct CourseStudentDeleteCall<'a, C, A>
    where C: 'a, A: 'a {

    hub: &'a Classroom<C, A>,
    _course_id: String,
    _user_id: String,
    _delegate: Option<&'a mut Delegate>,
    _additional_params: HashMap<String, String>,
    _scopes: BTreeMap<String, ()>
}

impl<'a, C, A> CallBuilder for CourseStudentDeleteCall<'a, C, A> {}

impl<'a, C, A> CourseStudentDeleteCall<'a, C, A> where C: BorrowMut<hyper::Client>, A: oauth2::GetToken {


    /// Perform the operation you have build so far.
    pub fn doit(mut self) -> Result<(hyper::client::Response, Empty)> {
        use std::io::{Read, Seek};
        use hyper::header::{ContentType, ContentLength, Authorization, Bearer, UserAgent, Location};
        let mut dd = DefaultDelegate;
        let mut dlg: &mut Delegate = match self._delegate {
            Some(d) => d,
            None => &mut dd
        };
        dlg.begin(MethodInfo { id: "classroom.courses.students.delete",
                               http_method: hyper::method::Method::Delete });
        let mut params: Vec<(&str, String)> = Vec::with_capacity((4 + self._additional_params.len()));
        params.push(("courseId", self._course_id.to_string()));
        params.push(("userId", self._user_id.to_string()));
        for &field in ["alt", "courseId", "userId"].iter() {
            if self._additional_params.contains_key(field) {
                dlg.finished(false);
                return Err(Error::FieldClash(field));
            }
        }
        for (name, value) in self._additional_params.iter() {
            params.push((&name, value.clone()));
        }

        params.push(("alt", "json".to_string()));

        let mut url = self.hub._base_url.clone() + "v1/courses/{courseId}/students/{userId}";
        if self._scopes.len() == 0 {
            self._scopes.insert(Scope::Roster.as_ref().to_string(), ());
        }

        for &(find_this, param_name) in [("{courseId}", "courseId"), ("{userId}", "userId")].iter() {
            let mut replace_with: Option<&str> = None;
            for &(name, ref value) in params.iter() {
                if name == param_name {
                    replace_with = Some(value);
                    break;
                }
            }
            url = url.replace(find_this, replace_with.expect("to find substitution value in params"));
        }
        {
            let mut indices_for_removal: Vec<usize> = Vec::with_capacity(2);
            for param_name in ["userId", "courseId"].iter() {
                if let Some(index) = params.iter().position(|t| &t.0 == param_name) {
                    indices_for_removal.push(index);
                }
            }
            for &index in indices_for_removal.iter() {
                params.remove(index);
            }
        }

        if params.len() > 0 {
            url.push('?');
            url.push_str(&url::form_urlencoded::serialize(params));
        }



        loop {
            let token = match self.hub.auth.borrow_mut().token(self._scopes.keys()) {
                Ok(token) => token,
                Err(err) => {
                    match  dlg.token(&*err) {
                        Some(token) => token,
                        None => {
                            dlg.finished(false);
                            return Err(Error::MissingToken(err))
                        }
                    }
                }
            };
            let auth_header = Authorization(Bearer { token: token.access_token });
            let mut req_result = {
                let mut client = &mut *self.hub.client.borrow_mut();
                let mut req = client.borrow_mut().request(hyper::method::Method::Delete, &url)
                    .header(UserAgent(self.hub._user_agent.clone()))
                    .header(auth_header.clone());

                dlg.pre_request();
                req.send()
            };

            match req_result {
                Err(err) => {
                    if let oauth2::Retry::After(d) = dlg.http_error(&err) {
                        sleep(d);
                        continue;
                    }
                    dlg.finished(false);
                    return Err(Error::HttpError(err))
                }
                Ok(mut res) => {
                    if !res.status.is_success() {
                        let mut json_err = String::new();
                        res.read_to_string(&mut json_err).unwrap();
                        if let oauth2::Retry::After(d) = dlg.http_failure(&res,
                                                              json::from_str(&json_err).ok(),
                                                              json::from_str(&json_err).ok()) {
                            sleep(d);
                            continue;
                        }
                        dlg.finished(false);
                        return match json::from_str::<ErrorResponse>(&json_err){
                            Err(_) => Err(Error::Failure(res)),
                            Ok(serr) => Err(Error::BadRequest(serr))
                        }
                    }
                    let result_value = {
                        let mut json_response = String::new();
                        res.read_to_string(&mut json_response).unwrap();
                        match json::from_str(&json_response) {
                            Ok(decoded) => (res, decoded),
                            Err(err) => {
                                dlg.response_json_decode_error(&json_response, &err);
                                return Err(Error::JsonDecodeError(json_response, err));
                            }
                        }
                    };

                    dlg.finished(true);
                    return Ok(result_value)
                }
            }
        }
    }


    /// Identifier of the course.
    /// This identifier can be either the Classroom-assigned identifier or an
    /// alias.
    ///
    /// Sets the *course id* path property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call,
    /// we provide this method for API completeness.
    pub fn course_id(mut self, new_value: &str) -> CourseStudentDeleteCall<'a, C, A> {
        self._course_id = new_value.to_string();
        self
    }
    /// Identifier of the student to delete. The identifier can be one of the
    /// following:
    /// 
    /// * the numeric identifier for the user
    /// * the email address of the user
    /// * the string literal `"me"`, indicating the requesting user
    ///
    /// Sets the *user id* path property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call,
    /// we provide this method for API completeness.
    pub fn user_id(mut self, new_value: &str) -> CourseStudentDeleteCall<'a, C, A> {
        self._user_id = new_value.to_string();
        self
    }
    /// The delegate implementation is consulted whenever there is an intermediate result, or if something goes wrong
    /// while executing the actual API request.
    /// 
    /// It should be used to handle progress information, and to implement a certain level of resilience.
    ///
    /// Sets the *delegate* property to the given value.
    pub fn delegate(mut self, new_value: &'a mut Delegate) -> CourseStudentDeleteCall<'a, C, A> {
        self._delegate = Some(new_value);
        self
    }

    /// Set any additional parameter of the query string used in the request.
    /// It should be used to set parameters which are not yet available through their own
    /// setters.
    ///
    /// Please note that this method must not be used to set any of the known paramters
    /// which have their own setter method. If done anyway, the request will fail.
    ///
    /// # Additional Parameters
    ///
    /// * *upload_protocol* (query-string) - Upload protocol for media (e.g. "raw", "multipart").
    /// * *pp* (query-boolean) - Pretty-print response.
    /// * *prettyPrint* (query-boolean) - Returns response with indentations and line breaks.
    /// * *bearer_token* (query-string) - OAuth bearer token.
    /// * *access_token* (query-string) - OAuth access token.
    /// * *uploadType* (query-string) - Legacy upload protocol for media (e.g. "media", "multipart").
    /// * *quotaUser* (query-string) - Available to use for quota purposes for server-side applications. Can be any arbitrary string assigned to a user, but should not exceed 40 characters.
    /// * *callback* (query-string) - JSONP
    /// * *oauth_token* (query-string) - OAuth 2.0 token for the current user.
    /// * *key* (query-string) - API key. Your API key identifies your project and provides you with API access, quota, and reports. Required unless you provide an OAuth 2.0 token.
    /// * *fields* (query-string) - Selector specifying which fields to include in a partial response.
    /// * *alt* (query-string) - Data format for response.
    /// * *$.xgafv* (query-string) - V1 error format.
    pub fn param<T>(mut self, name: T, value: T) -> CourseStudentDeleteCall<'a, C, A>
                                                        where T: AsRef<str> {
        self._additional_params.insert(name.as_ref().to_string(), value.as_ref().to_string());
        self
    }

    /// Identifies the authorization scope for the method you are building.
    ///
    /// Use this method to actively specify which scope should be used, instead the default `Scope` variant
    /// `Scope::Roster`.
    ///
    /// The `scope` will be added to a set of scopes. This is important as one can maintain access
    /// tokens for more than one scope.
    /// If `None` is specified, then all scopes will be removed and no default scope will be used either.
    /// In that case, you have to specify your API-key using the `key` parameter (see the `param()`
    /// function for details).
    ///
    /// Usually there is more than one suitable scope to authorize an operation, some of which may
    /// encompass more rights than others. For example, for listing resources, a *read-only* scope will be
    /// sufficient, a read-write scope will do as well.
    pub fn add_scope<T, S>(mut self, scope: T) -> CourseStudentDeleteCall<'a, C, A>
                                                        where T: Into<Option<S>>,
                                                              S: AsRef<str> {
        match scope.into() {
          Some(scope) => self._scopes.insert(scope.as_ref().to_string(), ()),
          None => None,
        };
        self
    }
}


/// Deletes a teacher of a course.
/// 
/// This method returns the following error codes:
/// 
/// * `PERMISSION_DENIED` if the requesting user is not permitted to delete
/// teachers of this course or for access errors.
/// * `NOT_FOUND` if no teacher of this course has the requested ID or if the
/// course does not exist.
/// * `FAILED_PRECONDITION` if the requested ID belongs to the primary teacher
/// of this course.
///
/// A builder for the *teachers.delete* method supported by a *course* resource.
/// It is not used directly, but through a `CourseMethods` instance.
///
/// # Example
///
/// Instantiate a resource method builder
///
/// ```test_harness,no_run
/// # extern crate hyper;
/// # extern crate hyper_rustls;
/// # extern crate yup_oauth2 as oauth2;
/// # extern crate google_classroom1 as classroom1;
/// # #[test] fn egal() {
/// # use std::default::Default;
/// # use oauth2::{Authenticator, DefaultAuthenticatorDelegate, ApplicationSecret, MemoryStorage};
/// # use classroom1::Classroom;
/// 
/// # let secret: ApplicationSecret = Default::default();
/// # let auth = Authenticator::new(&secret, DefaultAuthenticatorDelegate,
/// #                               hyper::Client::with_connector(hyper::net::HttpsConnector::new(hyper_rustls::TlsClient::new())),
/// #                               <MemoryStorage as Default>::default(), None);
/// # let mut hub = Classroom::new(hyper::Client::with_connector(hyper::net::HttpsConnector::new(hyper_rustls::TlsClient::new())), auth);
/// // You can configure optional parameters by calling the respective setters at will, and
/// // execute the final call using `doit()`.
/// // Values shown here are possibly random and not representative !
/// let result = hub.courses().teachers_delete("courseId", "userId")
///              .doit();
/// # }
/// ```
pub struct CourseTeacherDeleteCall<'a, C, A>
    where C: 'a, A: 'a {

    hub: &'a Classroom<C, A>,
    _course_id: String,
    _user_id: String,
    _delegate: Option<&'a mut Delegate>,
    _additional_params: HashMap<String, String>,
    _scopes: BTreeMap<String, ()>
}

impl<'a, C, A> CallBuilder for CourseTeacherDeleteCall<'a, C, A> {}

impl<'a, C, A> CourseTeacherDeleteCall<'a, C, A> where C: BorrowMut<hyper::Client>, A: oauth2::GetToken {


    /// Perform the operation you have build so far.
    pub fn doit(mut self) -> Result<(hyper::client::Response, Empty)> {
        use std::io::{Read, Seek};
        use hyper::header::{ContentType, ContentLength, Authorization, Bearer, UserAgent, Location};
        let mut dd = DefaultDelegate;
        let mut dlg: &mut Delegate = match self._delegate {
            Some(d) => d,
            None => &mut dd
        };
        dlg.begin(MethodInfo { id: "classroom.courses.teachers.delete",
                               http_method: hyper::method::Method::Delete });
        let mut params: Vec<(&str, String)> = Vec::with_capacity((4 + self._additional_params.len()));
        params.push(("courseId", self._course_id.to_string()));
        params.push(("userId", self._user_id.to_string()));
        for &field in ["alt", "courseId", "userId"].iter() {
            if self._additional_params.contains_key(field) {
                dlg.finished(false);
                return Err(Error::FieldClash(field));
            }
        }
        for (name, value) in self._additional_params.iter() {
            params.push((&name, value.clone()));
        }

        params.push(("alt", "json".to_string()));

        let mut url = self.hub._base_url.clone() + "v1/courses/{courseId}/teachers/{userId}";
        if self._scopes.len() == 0 {
            self._scopes.insert(Scope::Roster.as_ref().to_string(), ());
        }

        for &(find_this, param_name) in [("{courseId}", "courseId"), ("{userId}", "userId")].iter() {
            let mut replace_with: Option<&str> = None;
            for &(name, ref value) in params.iter() {
                if name == param_name {
                    replace_with = Some(value);
                    break;
                }
            }
            url = url.replace(find_this, replace_with.expect("to find substitution value in params"));
        }
        {
            let mut indices_for_removal: Vec<usize> = Vec::with_capacity(2);
            for param_name in ["userId", "courseId"].iter() {
                if let Some(index) = params.iter().position(|t| &t.0 == param_name) {
                    indices_for_removal.push(index);
                }
            }
            for &index in indices_for_removal.iter() {
                params.remove(index);
            }
        }

        if params.len() > 0 {
            url.push('?');
            url.push_str(&url::form_urlencoded::serialize(params));
        }



        loop {
            let token = match self.hub.auth.borrow_mut().token(self._scopes.keys()) {
                Ok(token) => token,
                Err(err) => {
                    match  dlg.token(&*err) {
                        Some(token) => token,
                        None => {
                            dlg.finished(false);
                            return Err(Error::MissingToken(err))
                        }
                    }
                }
            };
            let auth_header = Authorization(Bearer { token: token.access_token });
            let mut req_result = {
                let mut client = &mut *self.hub.client.borrow_mut();
                let mut req = client.borrow_mut().request(hyper::method::Method::Delete, &url)
                    .header(UserAgent(self.hub._user_agent.clone()))
                    .header(auth_header.clone());

                dlg.pre_request();
                req.send()
            };

            match req_result {
                Err(err) => {
                    if let oauth2::Retry::After(d) = dlg.http_error(&err) {
                        sleep(d);
                        continue;
                    }
                    dlg.finished(false);
                    return Err(Error::HttpError(err))
                }
                Ok(mut res) => {
                    if !res.status.is_success() {
                        let mut json_err = String::new();
                        res.read_to_string(&mut json_err).unwrap();
                        if let oauth2::Retry::After(d) = dlg.http_failure(&res,
                                                              json::from_str(&json_err).ok(),
                                                              json::from_str(&json_err).ok()) {
                            sleep(d);
                            continue;
                        }
                        dlg.finished(false);
                        return match json::from_str::<ErrorResponse>(&json_err){
                            Err(_) => Err(Error::Failure(res)),
                            Ok(serr) => Err(Error::BadRequest(serr))
                        }
                    }
                    let result_value = {
                        let mut json_response = String::new();
                        res.read_to_string(&mut json_response).unwrap();
                        match json::from_str(&json_response) {
                            Ok(decoded) => (res, decoded),
                            Err(err) => {
                                dlg.response_json_decode_error(&json_response, &err);
                                return Err(Error::JsonDecodeError(json_response, err));
                            }
                        }
                    };

                    dlg.finished(true);
                    return Ok(result_value)
                }
            }
        }
    }


    /// Identifier of the course.
    /// This identifier can be either the Classroom-assigned identifier or an
    /// alias.
    ///
    /// Sets the *course id* path property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call,
    /// we provide this method for API completeness.
    pub fn course_id(mut self, new_value: &str) -> CourseTeacherDeleteCall<'a, C, A> {
        self._course_id = new_value.to_string();
        self
    }
    /// Identifier of the teacher to delete. The identifier can be one of the
    /// following:
    /// 
    /// * the numeric identifier for the user
    /// * the email address of the user
    /// * the string literal `"me"`, indicating the requesting user
    ///
    /// Sets the *user id* path property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call,
    /// we provide this method for API completeness.
    pub fn user_id(mut self, new_value: &str) -> CourseTeacherDeleteCall<'a, C, A> {
        self._user_id = new_value.to_string();
        self
    }
    /// The delegate implementation is consulted whenever there is an intermediate result, or if something goes wrong
    /// while executing the actual API request.
    /// 
    /// It should be used to handle progress information, and to implement a certain level of resilience.
    ///
    /// Sets the *delegate* property to the given value.
    pub fn delegate(mut self, new_value: &'a mut Delegate) -> CourseTeacherDeleteCall<'a, C, A> {
        self._delegate = Some(new_value);
        self
    }

    /// Set any additional parameter of the query string used in the request.
    /// It should be used to set parameters which are not yet available through their own
    /// setters.
    ///
    /// Please note that this method must not be used to set any of the known paramters
    /// which have their own setter method. If done anyway, the request will fail.
    ///
    /// # Additional Parameters
    ///
    /// * *upload_protocol* (query-string) - Upload protocol for media (e.g. "raw", "multipart").
    /// * *pp* (query-boolean) - Pretty-print response.
    /// * *prettyPrint* (query-boolean) - Returns response with indentations and line breaks.
    /// * *bearer_token* (query-string) - OAuth bearer token.
    /// * *access_token* (query-string) - OAuth access token.
    /// * *uploadType* (query-string) - Legacy upload protocol for media (e.g. "media", "multipart").
    /// * *quotaUser* (query-string) - Available to use for quota purposes for server-side applications. Can be any arbitrary string assigned to a user, but should not exceed 40 characters.
    /// * *callback* (query-string) - JSONP
    /// * *oauth_token* (query-string) - OAuth 2.0 token for the current user.
    /// * *key* (query-string) - API key. Your API key identifies your project and provides you with API access, quota, and reports. Required unless you provide an OAuth 2.0 token.
    /// * *fields* (query-string) - Selector specifying which fields to include in a partial response.
    /// * *alt* (query-string) - Data format for response.
    /// * *$.xgafv* (query-string) - V1 error format.
    pub fn param<T>(mut self, name: T, value: T) -> CourseTeacherDeleteCall<'a, C, A>
                                                        where T: AsRef<str> {
        self._additional_params.insert(name.as_ref().to_string(), value.as_ref().to_string());
        self
    }

    /// Identifies the authorization scope for the method you are building.
    ///
    /// Use this method to actively specify which scope should be used, instead the default `Scope` variant
    /// `Scope::Roster`.
    ///
    /// The `scope` will be added to a set of scopes. This is important as one can maintain access
    /// tokens for more than one scope.
    /// If `None` is specified, then all scopes will be removed and no default scope will be used either.
    /// In that case, you have to specify your API-key using the `key` parameter (see the `param()`
    /// function for details).
    ///
    /// Usually there is more than one suitable scope to authorize an operation, some of which may
    /// encompass more rights than others. For example, for listing resources, a *read-only* scope will be
    /// sufficient, a read-write scope will do as well.
    pub fn add_scope<T, S>(mut self, scope: T) -> CourseTeacherDeleteCall<'a, C, A>
                                                        where T: Into<Option<S>>,
                                                              S: AsRef<str> {
        match scope.into() {
          Some(scope) => self._scopes.insert(scope.as_ref().to_string(), ()),
          None => None,
        };
        self
    }
}


/// Creates a teacher of a course.
/// 
/// This method returns the following error codes:
/// 
/// * `PERMISSION_DENIED` if the requesting user is not  permitted to create
/// teachers in this course or for access errors.
/// * `NOT_FOUND` if the requested course ID does not exist.
/// * `FAILED_PRECONDITION` if the requested user's account is disabled,
/// for the following request errors:
///     * CourseMemberLimitReached
///     * CourseNotModifiable
///     * CourseTeacherLimitReached
///     * UserGroupsMembershipLimitReached
/// * `ALREADY_EXISTS` if the user is already a teacher or student in the
/// course.
///
/// A builder for the *teachers.create* method supported by a *course* resource.
/// It is not used directly, but through a `CourseMethods` instance.
///
/// # Example
///
/// Instantiate a resource method builder
///
/// ```test_harness,no_run
/// # extern crate hyper;
/// # extern crate hyper_rustls;
/// # extern crate yup_oauth2 as oauth2;
/// # extern crate google_classroom1 as classroom1;
/// use classroom1::Teacher;
/// # #[test] fn egal() {
/// # use std::default::Default;
/// # use oauth2::{Authenticator, DefaultAuthenticatorDelegate, ApplicationSecret, MemoryStorage};
/// # use classroom1::Classroom;
/// 
/// # let secret: ApplicationSecret = Default::default();
/// # let auth = Authenticator::new(&secret, DefaultAuthenticatorDelegate,
/// #                               hyper::Client::with_connector(hyper::net::HttpsConnector::new(hyper_rustls::TlsClient::new())),
/// #                               <MemoryStorage as Default>::default(), None);
/// # let mut hub = Classroom::new(hyper::Client::with_connector(hyper::net::HttpsConnector::new(hyper_rustls::TlsClient::new())), auth);
/// // As the method needs a request, you would usually fill it with the desired information
/// // into the respective structure. Some of the parts shown here might not be applicable !
/// // Values shown here are possibly random and not representative !
/// let mut req = Teacher::default();
/// 
/// // You can configure optional parameters by calling the respective setters at will, and
/// // execute the final call using `doit()`.
/// // Values shown here are possibly random and not representative !
/// let result = hub.courses().teachers_create(req, "courseId")
///              .doit();
/// # }
/// ```
pub struct CourseTeacherCreateCall<'a, C, A>
    where C: 'a, A: 'a {

    hub: &'a Classroom<C, A>,
    _request: Teacher,
    _course_id: String,
    _delegate: Option<&'a mut Delegate>,
    _additional_params: HashMap<String, String>,
    _scopes: BTreeMap<String, ()>
}

impl<'a, C, A> CallBuilder for CourseTeacherCreateCall<'a, C, A> {}

impl<'a, C, A> CourseTeacherCreateCall<'a, C, A> where C: BorrowMut<hyper::Client>, A: oauth2::GetToken {


    /// Perform the operation you have build so far.
    pub fn doit(mut self) -> Result<(hyper::client::Response, Teacher)> {
        use std::io::{Read, Seek};
        use hyper::header::{ContentType, ContentLength, Authorization, Bearer, UserAgent, Location};
        let mut dd = DefaultDelegate;
        let mut dlg: &mut Delegate = match self._delegate {
            Some(d) => d,
            None => &mut dd
        };
        dlg.begin(MethodInfo { id: "classroom.courses.teachers.create",
                               http_method: hyper::method::Method::Post });
        let mut params: Vec<(&str, String)> = Vec::with_capacity((4 + self._additional_params.len()));
        params.push(("courseId", self._course_id.to_string()));
        for &field in ["alt", "courseId"].iter() {
            if self._additional_params.contains_key(field) {
                dlg.finished(false);
                return Err(Error::FieldClash(field));
            }
        }
        for (name, value) in self._additional_params.iter() {
            params.push((&name, value.clone()));
        }

        params.push(("alt", "json".to_string()));

        let mut url = self.hub._base_url.clone() + "v1/courses/{courseId}/teachers";
        if self._scopes.len() == 0 {
            self._scopes.insert(Scope::ProfileEmail.as_ref().to_string(), ());
        }

        for &(find_this, param_name) in [("{courseId}", "courseId")].iter() {
            let mut replace_with: Option<&str> = None;
            for &(name, ref value) in params.iter() {
                if name == param_name {
                    replace_with = Some(value);
                    break;
                }
            }
            url = url.replace(find_this, replace_with.expect("to find substitution value in params"));
        }
        {
            let mut indices_for_removal: Vec<usize> = Vec::with_capacity(1);
            for param_name in ["courseId"].iter() {
                if let Some(index) = params.iter().position(|t| &t.0 == param_name) {
                    indices_for_removal.push(index);
                }
            }
            for &index in indices_for_removal.iter() {
                params.remove(index);
            }
        }

        if params.len() > 0 {
            url.push('?');
            url.push_str(&url::form_urlencoded::serialize(params));
        }

        let mut json_mime_type = mime::Mime(mime::TopLevel::Application, mime::SubLevel::Json, Default::default());
        let mut request_value_reader =
            {
                let mut value = json::value::to_value(&self._request).expect("serde to work");
                remove_json_null_values(&mut value);
                let mut dst = io::Cursor::new(Vec::with_capacity(128));
                json::to_writer(&mut dst, &value).unwrap();
                dst
            };
        let request_size = request_value_reader.seek(io::SeekFrom::End(0)).unwrap();
        request_value_reader.seek(io::SeekFrom::Start(0)).unwrap();


        loop {
            let token = match self.hub.auth.borrow_mut().token(self._scopes.keys()) {
                Ok(token) => token,
                Err(err) => {
                    match  dlg.token(&*err) {
                        Some(token) => token,
                        None => {
                            dlg.finished(false);
                            return Err(Error::MissingToken(err))
                        }
                    }
                }
            };
            let auth_header = Authorization(Bearer { token: token.access_token });
            request_value_reader.seek(io::SeekFrom::Start(0)).unwrap();
            let mut req_result = {
                let mut client = &mut *self.hub.client.borrow_mut();
                let mut req = client.borrow_mut().request(hyper::method::Method::Post, &url)
                    .header(UserAgent(self.hub._user_agent.clone()))
                    .header(auth_header.clone())
                    .header(ContentType(json_mime_type.clone()))
                    .header(ContentLength(request_size as u64))
                    .body(&mut request_value_reader);

                dlg.pre_request();
                req.send()
            };

            match req_result {
                Err(err) => {
                    if let oauth2::Retry::After(d) = dlg.http_error(&err) {
                        sleep(d);
                        continue;
                    }
                    dlg.finished(false);
                    return Err(Error::HttpError(err))
                }
                Ok(mut res) => {
                    if !res.status.is_success() {
                        let mut json_err = String::new();
                        res.read_to_string(&mut json_err).unwrap();
                        if let oauth2::Retry::After(d) = dlg.http_failure(&res,
                                                              json::from_str(&json_err).ok(),
                                                              json::from_str(&json_err).ok()) {
                            sleep(d);
                            continue;
                        }
                        dlg.finished(false);
                        return match json::from_str::<ErrorResponse>(&json_err){
                            Err(_) => Err(Error::Failure(res)),
                            Ok(serr) => Err(Error::BadRequest(serr))
                        }
                    }
                    let result_value = {
                        let mut json_response = String::new();
                        res.read_to_string(&mut json_response).unwrap();
                        match json::from_str(&json_response) {
                            Ok(decoded) => (res, decoded),
                            Err(err) => {
                                dlg.response_json_decode_error(&json_response, &err);
                                return Err(Error::JsonDecodeError(json_response, err));
                            }
                        }
                    };

                    dlg.finished(true);
                    return Ok(result_value)
                }
            }
        }
    }


    ///
    /// Sets the *request* property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call,
    /// we provide this method for API completeness.
    pub fn request(mut self, new_value: Teacher) -> CourseTeacherCreateCall<'a, C, A> {
        self._request = new_value;
        self
    }
    /// Identifier of the course.
    /// This identifier can be either the Classroom-assigned identifier or an
    /// alias.
    ///
    /// Sets the *course id* path property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call,
    /// we provide this method for API completeness.
    pub fn course_id(mut self, new_value: &str) -> CourseTeacherCreateCall<'a, C, A> {
        self._course_id = new_value.to_string();
        self
    }
    /// The delegate implementation is consulted whenever there is an intermediate result, or if something goes wrong
    /// while executing the actual API request.
    /// 
    /// It should be used to handle progress information, and to implement a certain level of resilience.
    ///
    /// Sets the *delegate* property to the given value.
    pub fn delegate(mut self, new_value: &'a mut Delegate) -> CourseTeacherCreateCall<'a, C, A> {
        self._delegate = Some(new_value);
        self
    }

    /// Set any additional parameter of the query string used in the request.
    /// It should be used to set parameters which are not yet available through their own
    /// setters.
    ///
    /// Please note that this method must not be used to set any of the known paramters
    /// which have their own setter method. If done anyway, the request will fail.
    ///
    /// # Additional Parameters
    ///
    /// * *upload_protocol* (query-string) - Upload protocol for media (e.g. "raw", "multipart").
    /// * *pp* (query-boolean) - Pretty-print response.
    /// * *prettyPrint* (query-boolean) - Returns response with indentations and line breaks.
    /// * *bearer_token* (query-string) - OAuth bearer token.
    /// * *access_token* (query-string) - OAuth access token.
    /// * *uploadType* (query-string) - Legacy upload protocol for media (e.g. "media", "multipart").
    /// * *quotaUser* (query-string) - Available to use for quota purposes for server-side applications. Can be any arbitrary string assigned to a user, but should not exceed 40 characters.
    /// * *callback* (query-string) - JSONP
    /// * *oauth_token* (query-string) - OAuth 2.0 token for the current user.
    /// * *key* (query-string) - API key. Your API key identifies your project and provides you with API access, quota, and reports. Required unless you provide an OAuth 2.0 token.
    /// * *fields* (query-string) - Selector specifying which fields to include in a partial response.
    /// * *alt* (query-string) - Data format for response.
    /// * *$.xgafv* (query-string) - V1 error format.
    pub fn param<T>(mut self, name: T, value: T) -> CourseTeacherCreateCall<'a, C, A>
                                                        where T: AsRef<str> {
        self._additional_params.insert(name.as_ref().to_string(), value.as_ref().to_string());
        self
    }

    /// Identifies the authorization scope for the method you are building.
    ///
    /// Use this method to actively specify which scope should be used, instead the default `Scope` variant
    /// `Scope::ProfileEmail`.
    ///
    /// The `scope` will be added to a set of scopes. This is important as one can maintain access
    /// tokens for more than one scope.
    /// If `None` is specified, then all scopes will be removed and no default scope will be used either.
    /// In that case, you have to specify your API-key using the `key` parameter (see the `param()`
    /// function for details).
    ///
    /// Usually there is more than one suitable scope to authorize an operation, some of which may
    /// encompass more rights than others. For example, for listing resources, a *read-only* scope will be
    /// sufficient, a read-write scope will do as well.
    pub fn add_scope<T, S>(mut self, scope: T) -> CourseTeacherCreateCall<'a, C, A>
                                                        where T: Into<Option<S>>,
                                                              S: AsRef<str> {
        match scope.into() {
          Some(scope) => self._scopes.insert(scope.as_ref().to_string(), ()),
          None => None,
        };
        self
    }
}


/// Returns a student submission.
/// 
/// * `PERMISSION_DENIED` if the requesting user is not permitted to access the
/// requested course, course work, or student submission or for
/// access errors.
/// * `INVALID_ARGUMENT` if the request is malformed.
/// * `NOT_FOUND` if the requested course, course work, or student submission
/// does not exist.
///
/// A builder for the *courseWork.studentSubmissions.get* method supported by a *course* resource.
/// It is not used directly, but through a `CourseMethods` instance.
///
/// # Example
///
/// Instantiate a resource method builder
///
/// ```test_harness,no_run
/// # extern crate hyper;
/// # extern crate hyper_rustls;
/// # extern crate yup_oauth2 as oauth2;
/// # extern crate google_classroom1 as classroom1;
/// # #[test] fn egal() {
/// # use std::default::Default;
/// # use oauth2::{Authenticator, DefaultAuthenticatorDelegate, ApplicationSecret, MemoryStorage};
/// # use classroom1::Classroom;
/// 
/// # let secret: ApplicationSecret = Default::default();
/// # let auth = Authenticator::new(&secret, DefaultAuthenticatorDelegate,
/// #                               hyper::Client::with_connector(hyper::net::HttpsConnector::new(hyper_rustls::TlsClient::new())),
/// #                               <MemoryStorage as Default>::default(), None);
/// # let mut hub = Classroom::new(hyper::Client::with_connector(hyper::net::HttpsConnector::new(hyper_rustls::TlsClient::new())), auth);
/// // You can configure optional parameters by calling the respective setters at will, and
/// // execute the final call using `doit()`.
/// // Values shown here are possibly random and not representative !
/// let result = hub.courses().course_work_student_submissions_get("courseId", "courseWorkId", "id")
///              .doit();
/// # }
/// ```
pub struct CourseCourseWorkStudentSubmissionGetCall<'a, C, A>
    where C: 'a, A: 'a {

    hub: &'a Classroom<C, A>,
    _course_id: String,
    _course_work_id: String,
    _id: String,
    _delegate: Option<&'a mut Delegate>,
    _additional_params: HashMap<String, String>,
    _scopes: BTreeMap<String, ()>
}

impl<'a, C, A> CallBuilder for CourseCourseWorkStudentSubmissionGetCall<'a, C, A> {}

impl<'a, C, A> CourseCourseWorkStudentSubmissionGetCall<'a, C, A> where C: BorrowMut<hyper::Client>, A: oauth2::GetToken {


    /// Perform the operation you have build so far.
    pub fn doit(mut self) -> Result<(hyper::client::Response, StudentSubmission)> {
        use std::io::{Read, Seek};
        use hyper::header::{ContentType, ContentLength, Authorization, Bearer, UserAgent, Location};
        let mut dd = DefaultDelegate;
        let mut dlg: &mut Delegate = match self._delegate {
            Some(d) => d,
            None => &mut dd
        };
        dlg.begin(MethodInfo { id: "classroom.courses.courseWork.studentSubmissions.get",
                               http_method: hyper::method::Method::Get });
        let mut params: Vec<(&str, String)> = Vec::with_capacity((5 + self._additional_params.len()));
        params.push(("courseId", self._course_id.to_string()));
        params.push(("courseWorkId", self._course_work_id.to_string()));
        params.push(("id", self._id.to_string()));
        for &field in ["alt", "courseId", "courseWorkId", "id"].iter() {
            if self._additional_params.contains_key(field) {
                dlg.finished(false);
                return Err(Error::FieldClash(field));
            }
        }
        for (name, value) in self._additional_params.iter() {
            params.push((&name, value.clone()));
        }

        params.push(("alt", "json".to_string()));

        let mut url = self.hub._base_url.clone() + "v1/courses/{courseId}/courseWork/{courseWorkId}/studentSubmissions/{id}";
        if self._scopes.len() == 0 {
            self._scopes.insert(Scope::CourseworkMeReadonly.as_ref().to_string(), ());
        }

        for &(find_this, param_name) in [("{courseId}", "courseId"), ("{courseWorkId}", "courseWorkId"), ("{id}", "id")].iter() {
            let mut replace_with: Option<&str> = None;
            for &(name, ref value) in params.iter() {
                if name == param_name {
                    replace_with = Some(value);
                    break;
                }
            }
            url = url.replace(find_this, replace_with.expect("to find substitution value in params"));
        }
        {
            let mut indices_for_removal: Vec<usize> = Vec::with_capacity(3);
            for param_name in ["id", "courseWorkId", "courseId"].iter() {
                if let Some(index) = params.iter().position(|t| &t.0 == param_name) {
                    indices_for_removal.push(index);
                }
            }
            for &index in indices_for_removal.iter() {
                params.remove(index);
            }
        }

        if params.len() > 0 {
            url.push('?');
            url.push_str(&url::form_urlencoded::serialize(params));
        }



        loop {
            let token = match self.hub.auth.borrow_mut().token(self._scopes.keys()) {
                Ok(token) => token,
                Err(err) => {
                    match  dlg.token(&*err) {
                        Some(token) => token,
                        None => {
                            dlg.finished(false);
                            return Err(Error::MissingToken(err))
                        }
                    }
                }
            };
            let auth_header = Authorization(Bearer { token: token.access_token });
            let mut req_result = {
                let mut client = &mut *self.hub.client.borrow_mut();
                let mut req = client.borrow_mut().request(hyper::method::Method::Get, &url)
                    .header(UserAgent(self.hub._user_agent.clone()))
                    .header(auth_header.clone());

                dlg.pre_request();
                req.send()
            };

            match req_result {
                Err(err) => {
                    if let oauth2::Retry::After(d) = dlg.http_error(&err) {
                        sleep(d);
                        continue;
                    }
                    dlg.finished(false);
                    return Err(Error::HttpError(err))
                }
                Ok(mut res) => {
                    if !res.status.is_success() {
                        let mut json_err = String::new();
                        res.read_to_string(&mut json_err).unwrap();
                        if let oauth2::Retry::After(d) = dlg.http_failure(&res,
                                                              json::from_str(&json_err).ok(),
                                                              json::from_str(&json_err).ok()) {
                            sleep(d);
                            continue;
                        }
                        dlg.finished(false);
                        return match json::from_str::<ErrorResponse>(&json_err){
                            Err(_) => Err(Error::Failure(res)),
                            Ok(serr) => Err(Error::BadRequest(serr))
                        }
                    }
                    let result_value = {
                        let mut json_response = String::new();
                        res.read_to_string(&mut json_response).unwrap();
                        match json::from_str(&json_response) {
                            Ok(decoded) => (res, decoded),
                            Err(err) => {
                                dlg.response_json_decode_error(&json_response, &err);
                                return Err(Error::JsonDecodeError(json_response, err));
                            }
                        }
                    };

                    dlg.finished(true);
                    return Ok(result_value)
                }
            }
        }
    }


    /// Identifier of the course.
    /// This identifier can be either the Classroom-assigned identifier or an
    /// alias.
    ///
    /// Sets the *course id* path property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call,
    /// we provide this method for API completeness.
    pub fn course_id(mut self, new_value: &str) -> CourseCourseWorkStudentSubmissionGetCall<'a, C, A> {
        self._course_id = new_value.to_string();
        self
    }
    /// Identifier of the course work.
    ///
    /// Sets the *course work id* path property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call,
    /// we provide this method for API completeness.
    pub fn course_work_id(mut self, new_value: &str) -> CourseCourseWorkStudentSubmissionGetCall<'a, C, A> {
        self._course_work_id = new_value.to_string();
        self
    }
    /// Identifier of the student submission.
    ///
    /// Sets the *id* path property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call,
    /// we provide this method for API completeness.
    pub fn id(mut self, new_value: &str) -> CourseCourseWorkStudentSubmissionGetCall<'a, C, A> {
        self._id = new_value.to_string();
        self
    }
    /// The delegate implementation is consulted whenever there is an intermediate result, or if something goes wrong
    /// while executing the actual API request.
    /// 
    /// It should be used to handle progress information, and to implement a certain level of resilience.
    ///
    /// Sets the *delegate* property to the given value.
    pub fn delegate(mut self, new_value: &'a mut Delegate) -> CourseCourseWorkStudentSubmissionGetCall<'a, C, A> {
        self._delegate = Some(new_value);
        self
    }

    /// Set any additional parameter of the query string used in the request.
    /// It should be used to set parameters which are not yet available through their own
    /// setters.
    ///
    /// Please note that this method must not be used to set any of the known paramters
    /// which have their own setter method. If done anyway, the request will fail.
    ///
    /// # Additional Parameters
    ///
    /// * *upload_protocol* (query-string) - Upload protocol for media (e.g. "raw", "multipart").
    /// * *pp* (query-boolean) - Pretty-print response.
    /// * *prettyPrint* (query-boolean) - Returns response with indentations and line breaks.
    /// * *bearer_token* (query-string) - OAuth bearer token.
    /// * *access_token* (query-string) - OAuth access token.
    /// * *uploadType* (query-string) - Legacy upload protocol for media (e.g. "media", "multipart").
    /// * *quotaUser* (query-string) - Available to use for quota purposes for server-side applications. Can be any arbitrary string assigned to a user, but should not exceed 40 characters.
    /// * *callback* (query-string) - JSONP
    /// * *oauth_token* (query-string) - OAuth 2.0 token for the current user.
    /// * *key* (query-string) - API key. Your API key identifies your project and provides you with API access, quota, and reports. Required unless you provide an OAuth 2.0 token.
    /// * *fields* (query-string) - Selector specifying which fields to include in a partial response.
    /// * *alt* (query-string) - Data format for response.
    /// * *$.xgafv* (query-string) - V1 error format.
    pub fn param<T>(mut self, name: T, value: T) -> CourseCourseWorkStudentSubmissionGetCall<'a, C, A>
                                                        where T: AsRef<str> {
        self._additional_params.insert(name.as_ref().to_string(), value.as_ref().to_string());
        self
    }

    /// Identifies the authorization scope for the method you are building.
    ///
    /// Use this method to actively specify which scope should be used, instead the default `Scope` variant
    /// `Scope::CourseworkMeReadonly`.
    ///
    /// The `scope` will be added to a set of scopes. This is important as one can maintain access
    /// tokens for more than one scope.
    /// If `None` is specified, then all scopes will be removed and no default scope will be used either.
    /// In that case, you have to specify your API-key using the `key` parameter (see the `param()`
    /// function for details).
    ///
    /// Usually there is more than one suitable scope to authorize an operation, some of which may
    /// encompass more rights than others. For example, for listing resources, a *read-only* scope will be
    /// sufficient, a read-write scope will do as well.
    pub fn add_scope<T, S>(mut self, scope: T) -> CourseCourseWorkStudentSubmissionGetCall<'a, C, A>
                                                        where T: Into<Option<S>>,
                                                              S: AsRef<str> {
        match scope.into() {
          Some(scope) => self._scopes.insert(scope.as_ref().to_string(), ()),
          None => None,
        };
        self
    }
}


/// Returns a specific guardian.
/// 
/// This method returns the following error codes:
/// 
/// * `PERMISSION_DENIED` if no user that matches the provided `student_id`
///   is visible to the requesting user, if the requesting user is not
///   permitted to view guardian information for the student identified by the
///   `student_id`, if guardians are not enabled for the domain in question,
///   or for other access errors.
/// * `INVALID_ARGUMENT` if a `student_id` is specified, but its format cannot
///   be recognized (it is not an email address, nor a `student_id` from the
///   API, nor the literal string `me`).
/// * `NOT_FOUND` if the requesting user is permitted to view guardians for
///   the requested `student_id`, but no `Guardian` record exists for that
///   student that matches the provided `guardian_id`.
///
/// A builder for the *guardians.get* method supported by a *userProfile* resource.
/// It is not used directly, but through a `UserProfileMethods` instance.
///
/// # Example
///
/// Instantiate a resource method builder
///
/// ```test_harness,no_run
/// # extern crate hyper;
/// # extern crate hyper_rustls;
/// # extern crate yup_oauth2 as oauth2;
/// # extern crate google_classroom1 as classroom1;
/// # #[test] fn egal() {
/// # use std::default::Default;
/// # use oauth2::{Authenticator, DefaultAuthenticatorDelegate, ApplicationSecret, MemoryStorage};
/// # use classroom1::Classroom;
/// 
/// # let secret: ApplicationSecret = Default::default();
/// # let auth = Authenticator::new(&secret, DefaultAuthenticatorDelegate,
/// #                               hyper::Client::with_connector(hyper::net::HttpsConnector::new(hyper_rustls::TlsClient::new())),
/// #                               <MemoryStorage as Default>::default(), None);
/// # let mut hub = Classroom::new(hyper::Client::with_connector(hyper::net::HttpsConnector::new(hyper_rustls::TlsClient::new())), auth);
/// // You can configure optional parameters by calling the respective setters at will, and
/// // execute the final call using `doit()`.
/// // Values shown here are possibly random and not representative !
/// let result = hub.user_profiles().guardians_get("studentId", "guardianId")
///              .doit();
/// # }
/// ```
pub struct UserProfileGuardianGetCall<'a, C, A>
    where C: 'a, A: 'a {

    hub: &'a Classroom<C, A>,
    _student_id: String,
    _guardian_id: String,
    _delegate: Option<&'a mut Delegate>,
    _additional_params: HashMap<String, String>,
}

impl<'a, C, A> CallBuilder for UserProfileGuardianGetCall<'a, C, A> {}

impl<'a, C, A> UserProfileGuardianGetCall<'a, C, A> where C: BorrowMut<hyper::Client>, A: oauth2::GetToken {


    /// Perform the operation you have build so far.
    pub fn doit(mut self) -> Result<(hyper::client::Response, Guardian)> {
        use std::io::{Read, Seek};
        use hyper::header::{ContentType, ContentLength, Authorization, Bearer, UserAgent, Location};
        let mut dd = DefaultDelegate;
        let mut dlg: &mut Delegate = match self._delegate {
            Some(d) => d,
            None => &mut dd
        };
        dlg.begin(MethodInfo { id: "classroom.userProfiles.guardians.get",
                               http_method: hyper::method::Method::Get });
        let mut params: Vec<(&str, String)> = Vec::with_capacity((4 + self._additional_params.len()));
        params.push(("studentId", self._student_id.to_string()));
        params.push(("guardianId", self._guardian_id.to_string()));
        for &field in ["alt", "studentId", "guardianId"].iter() {
            if self._additional_params.contains_key(field) {
                dlg.finished(false);
                return Err(Error::FieldClash(field));
            }
        }
        for (name, value) in self._additional_params.iter() {
            params.push((&name, value.clone()));
        }

        params.push(("alt", "json".to_string()));

        let mut url = self.hub._base_url.clone() + "v1/userProfiles/{studentId}/guardians/{guardianId}";
        
        let mut key = self.hub.auth.borrow_mut().api_key();
        if key.is_none() {
            key = dlg.api_key();
        }
        match key {
            Some(value) => params.push(("key", value)),
            None => {
                dlg.finished(false);
                return Err(Error::MissingAPIKey)
            }
        }

        for &(find_this, param_name) in [("{studentId}", "studentId"), ("{guardianId}", "guardianId")].iter() {
            let mut replace_with: Option<&str> = None;
            for &(name, ref value) in params.iter() {
                if name == param_name {
                    replace_with = Some(value);
                    break;
                }
            }
            url = url.replace(find_this, replace_with.expect("to find substitution value in params"));
        }
        {
            let mut indices_for_removal: Vec<usize> = Vec::with_capacity(2);
            for param_name in ["guardianId", "studentId"].iter() {
                if let Some(index) = params.iter().position(|t| &t.0 == param_name) {
                    indices_for_removal.push(index);
                }
            }
            for &index in indices_for_removal.iter() {
                params.remove(index);
            }
        }

        if params.len() > 0 {
            url.push('?');
            url.push_str(&url::form_urlencoded::serialize(params));
        }



        loop {
            let mut req_result = {
                let mut client = &mut *self.hub.client.borrow_mut();
                let mut req = client.borrow_mut().request(hyper::method::Method::Get, &url)
                    .header(UserAgent(self.hub._user_agent.clone()));

                dlg.pre_request();
                req.send()
            };

            match req_result {
                Err(err) => {
                    if let oauth2::Retry::After(d) = dlg.http_error(&err) {
                        sleep(d);
                        continue;
                    }
                    dlg.finished(false);
                    return Err(Error::HttpError(err))
                }
                Ok(mut res) => {
                    if !res.status.is_success() {
                        let mut json_err = String::new();
                        res.read_to_string(&mut json_err).unwrap();
                        if let oauth2::Retry::After(d) = dlg.http_failure(&res,
                                                              json::from_str(&json_err).ok(),
                                                              json::from_str(&json_err).ok()) {
                            sleep(d);
                            continue;
                        }
                        dlg.finished(false);
                        return match json::from_str::<ErrorResponse>(&json_err){
                            Err(_) => Err(Error::Failure(res)),
                            Ok(serr) => Err(Error::BadRequest(serr))
                        }
                    }
                    let result_value = {
                        let mut json_response = String::new();
                        res.read_to_string(&mut json_response).unwrap();
                        match json::from_str(&json_response) {
                            Ok(decoded) => (res, decoded),
                            Err(err) => {
                                dlg.response_json_decode_error(&json_response, &err);
                                return Err(Error::JsonDecodeError(json_response, err));
                            }
                        }
                    };

                    dlg.finished(true);
                    return Ok(result_value)
                }
            }
        }
    }


    /// The student whose guardian is being requested. One of the following:
    /// 
    /// * the numeric identifier for the user
    /// * the email address of the user
    /// * the string literal `"me"`, indicating the requesting user
    ///
    /// Sets the *student id* path property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call,
    /// we provide this method for API completeness.
    pub fn student_id(mut self, new_value: &str) -> UserProfileGuardianGetCall<'a, C, A> {
        self._student_id = new_value.to_string();
        self
    }
    /// The `id` field from a `Guardian`.
    ///
    /// Sets the *guardian id* path property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call,
    /// we provide this method for API completeness.
    pub fn guardian_id(mut self, new_value: &str) -> UserProfileGuardianGetCall<'a, C, A> {
        self._guardian_id = new_value.to_string();
        self
    }
    /// The delegate implementation is consulted whenever there is an intermediate result, or if something goes wrong
    /// while executing the actual API request.
    /// 
    /// It should be used to handle progress information, and to implement a certain level of resilience.
    ///
    /// Sets the *delegate* property to the given value.
    pub fn delegate(mut self, new_value: &'a mut Delegate) -> UserProfileGuardianGetCall<'a, C, A> {
        self._delegate = Some(new_value);
        self
    }

    /// Set any additional parameter of the query string used in the request.
    /// It should be used to set parameters which are not yet available through their own
    /// setters.
    ///
    /// Please note that this method must not be used to set any of the known paramters
    /// which have their own setter method. If done anyway, the request will fail.
    ///
    /// # Additional Parameters
    ///
    /// * *upload_protocol* (query-string) - Upload protocol for media (e.g. "raw", "multipart").
    /// * *pp* (query-boolean) - Pretty-print response.
    /// * *prettyPrint* (query-boolean) - Returns response with indentations and line breaks.
    /// * *bearer_token* (query-string) - OAuth bearer token.
    /// * *access_token* (query-string) - OAuth access token.
    /// * *uploadType* (query-string) - Legacy upload protocol for media (e.g. "media", "multipart").
    /// * *quotaUser* (query-string) - Available to use for quota purposes for server-side applications. Can be any arbitrary string assigned to a user, but should not exceed 40 characters.
    /// * *callback* (query-string) - JSONP
    /// * *oauth_token* (query-string) - OAuth 2.0 token for the current user.
    /// * *key* (query-string) - API key. Your API key identifies your project and provides you with API access, quota, and reports. Required unless you provide an OAuth 2.0 token.
    /// * *fields* (query-string) - Selector specifying which fields to include in a partial response.
    /// * *alt* (query-string) - Data format for response.
    /// * *$.xgafv* (query-string) - V1 error format.
    pub fn param<T>(mut self, name: T, value: T) -> UserProfileGuardianGetCall<'a, C, A>
                                                        where T: AsRef<str> {
        self._additional_params.insert(name.as_ref().to_string(), value.as_ref().to_string());
        self
    }

}


/// Creates a guardian invitation, and sends an email to the guardian asking
/// them to confirm that they are the student's guardian.
/// 
/// Once the guardian accepts the invitation, their `state` will change to
/// `COMPLETED` and they will start receiving guardian notifications. A
/// `Guardian` resource will also be created to represent the active guardian.
/// 
/// The request object must have the `student_id` and
/// `invited_email_address` fields set. Failing to set these fields, or
/// setting any other fields in the request, will result in an error.
/// 
/// This method returns the following error codes:
/// 
/// * `PERMISSION_DENIED` if the current user does not have permission to
///   manage guardians, if the guardian in question has already rejected
///   too many requests for that student, if guardians are not enabled for the
///   domain in question, or for other access errors.
/// * `RESOURCE_EXHAUSTED` if the student or guardian has exceeded the guardian
///   link limit.
/// * `INVALID_ARGUMENT` if the guardian email address is not valid (for
///   example, if it is too long), or if the format of the student ID provided
///   cannot be recognized (it is not an email address, nor a `user_id` from
///   this API). This error will also be returned if read-only fields are set,
///   or if the `state` field is set to to a value other than `PENDING`.
/// * `NOT_FOUND` if the student ID provided is a valid student ID, but
///   Classroom has no record of that student.
/// * `ALREADY_EXISTS` if there is already a pending guardian invitation for
///   the student and `invited_email_address` provided, or if the provided
///   `invited_email_address` matches the Google account of an existing
///   `Guardian` for this user.
///
/// A builder for the *guardianInvitations.create* method supported by a *userProfile* resource.
/// It is not used directly, but through a `UserProfileMethods` instance.
///
/// # Example
///
/// Instantiate a resource method builder
///
/// ```test_harness,no_run
/// # extern crate hyper;
/// # extern crate hyper_rustls;
/// # extern crate yup_oauth2 as oauth2;
/// # extern crate google_classroom1 as classroom1;
/// use classroom1::GuardianInvitation;
/// # #[test] fn egal() {
/// # use std::default::Default;
/// # use oauth2::{Authenticator, DefaultAuthenticatorDelegate, ApplicationSecret, MemoryStorage};
/// # use classroom1::Classroom;
/// 
/// # let secret: ApplicationSecret = Default::default();
/// # let auth = Authenticator::new(&secret, DefaultAuthenticatorDelegate,
/// #                               hyper::Client::with_connector(hyper::net::HttpsConnector::new(hyper_rustls::TlsClient::new())),
/// #                               <MemoryStorage as Default>::default(), None);
/// # let mut hub = Classroom::new(hyper::Client::with_connector(hyper::net::HttpsConnector::new(hyper_rustls::TlsClient::new())), auth);
/// // As the method needs a request, you would usually fill it with the desired information
/// // into the respective structure. Some of the parts shown here might not be applicable !
/// // Values shown here are possibly random and not representative !
/// let mut req = GuardianInvitation::default();
/// 
/// // You can configure optional parameters by calling the respective setters at will, and
/// // execute the final call using `doit()`.
/// // Values shown here are possibly random and not representative !
/// let result = hub.user_profiles().guardian_invitations_create(req, "studentId")
///              .doit();
/// # }
/// ```
pub struct UserProfileGuardianInvitationCreateCall<'a, C, A>
    where C: 'a, A: 'a {

    hub: &'a Classroom<C, A>,
    _request: GuardianInvitation,
    _student_id: String,
    _delegate: Option<&'a mut Delegate>,
    _additional_params: HashMap<String, String>,
}

impl<'a, C, A> CallBuilder for UserProfileGuardianInvitationCreateCall<'a, C, A> {}

impl<'a, C, A> UserProfileGuardianInvitationCreateCall<'a, C, A> where C: BorrowMut<hyper::Client>, A: oauth2::GetToken {


    /// Perform the operation you have build so far.
    pub fn doit(mut self) -> Result<(hyper::client::Response, GuardianInvitation)> {
        use std::io::{Read, Seek};
        use hyper::header::{ContentType, ContentLength, Authorization, Bearer, UserAgent, Location};
        let mut dd = DefaultDelegate;
        let mut dlg: &mut Delegate = match self._delegate {
            Some(d) => d,
            None => &mut dd
        };
        dlg.begin(MethodInfo { id: "classroom.userProfiles.guardianInvitations.create",
                               http_method: hyper::method::Method::Post });
        let mut params: Vec<(&str, String)> = Vec::with_capacity((4 + self._additional_params.len()));
        params.push(("studentId", self._student_id.to_string()));
        for &field in ["alt", "studentId"].iter() {
            if self._additional_params.contains_key(field) {
                dlg.finished(false);
                return Err(Error::FieldClash(field));
            }
        }
        for (name, value) in self._additional_params.iter() {
            params.push((&name, value.clone()));
        }

        params.push(("alt", "json".to_string()));

        let mut url = self.hub._base_url.clone() + "v1/userProfiles/{studentId}/guardianInvitations";
        
        let mut key = self.hub.auth.borrow_mut().api_key();
        if key.is_none() {
            key = dlg.api_key();
        }
        match key {
            Some(value) => params.push(("key", value)),
            None => {
                dlg.finished(false);
                return Err(Error::MissingAPIKey)
            }
        }

        for &(find_this, param_name) in [("{studentId}", "studentId")].iter() {
            let mut replace_with: Option<&str> = None;
            for &(name, ref value) in params.iter() {
                if name == param_name {
                    replace_with = Some(value);
                    break;
                }
            }
            url = url.replace(find_this, replace_with.expect("to find substitution value in params"));
        }
        {
            let mut indices_for_removal: Vec<usize> = Vec::with_capacity(1);
            for param_name in ["studentId"].iter() {
                if let Some(index) = params.iter().position(|t| &t.0 == param_name) {
                    indices_for_removal.push(index);
                }
            }
            for &index in indices_for_removal.iter() {
                params.remove(index);
            }
        }

        if params.len() > 0 {
            url.push('?');
            url.push_str(&url::form_urlencoded::serialize(params));
        }

        let mut json_mime_type = mime::Mime(mime::TopLevel::Application, mime::SubLevel::Json, Default::default());
        let mut request_value_reader =
            {
                let mut value = json::value::to_value(&self._request).expect("serde to work");
                remove_json_null_values(&mut value);
                let mut dst = io::Cursor::new(Vec::with_capacity(128));
                json::to_writer(&mut dst, &value).unwrap();
                dst
            };
        let request_size = request_value_reader.seek(io::SeekFrom::End(0)).unwrap();
        request_value_reader.seek(io::SeekFrom::Start(0)).unwrap();


        loop {
            request_value_reader.seek(io::SeekFrom::Start(0)).unwrap();
            let mut req_result = {
                let mut client = &mut *self.hub.client.borrow_mut();
                let mut req = client.borrow_mut().request(hyper::method::Method::Post, &url)
                    .header(UserAgent(self.hub._user_agent.clone()))
                    .header(ContentType(json_mime_type.clone()))
                    .header(ContentLength(request_size as u64))
                    .body(&mut request_value_reader);

                dlg.pre_request();
                req.send()
            };

            match req_result {
                Err(err) => {
                    if let oauth2::Retry::After(d) = dlg.http_error(&err) {
                        sleep(d);
                        continue;
                    }
                    dlg.finished(false);
                    return Err(Error::HttpError(err))
                }
                Ok(mut res) => {
                    if !res.status.is_success() {
                        let mut json_err = String::new();
                        res.read_to_string(&mut json_err).unwrap();
                        if let oauth2::Retry::After(d) = dlg.http_failure(&res,
                                                              json::from_str(&json_err).ok(),
                                                              json::from_str(&json_err).ok()) {
                            sleep(d);
                            continue;
                        }
                        dlg.finished(false);
                        return match json::from_str::<ErrorResponse>(&json_err){
                            Err(_) => Err(Error::Failure(res)),
                            Ok(serr) => Err(Error::BadRequest(serr))
                        }
                    }
                    let result_value = {
                        let mut json_response = String::new();
                        res.read_to_string(&mut json_response).unwrap();
                        match json::from_str(&json_response) {
                            Ok(decoded) => (res, decoded),
                            Err(err) => {
                                dlg.response_json_decode_error(&json_response, &err);
                                return Err(Error::JsonDecodeError(json_response, err));
                            }
                        }
                    };

                    dlg.finished(true);
                    return Ok(result_value)
                }
            }
        }
    }


    ///
    /// Sets the *request* property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call,
    /// we provide this method for API completeness.
    pub fn request(mut self, new_value: GuardianInvitation) -> UserProfileGuardianInvitationCreateCall<'a, C, A> {
        self._request = new_value;
        self
    }
    /// ID of the student (in standard format)
    ///
    /// Sets the *student id* path property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call,
    /// we provide this method for API completeness.
    pub fn student_id(mut self, new_value: &str) -> UserProfileGuardianInvitationCreateCall<'a, C, A> {
        self._student_id = new_value.to_string();
        self
    }
    /// The delegate implementation is consulted whenever there is an intermediate result, or if something goes wrong
    /// while executing the actual API request.
    /// 
    /// It should be used to handle progress information, and to implement a certain level of resilience.
    ///
    /// Sets the *delegate* property to the given value.
    pub fn delegate(mut self, new_value: &'a mut Delegate) -> UserProfileGuardianInvitationCreateCall<'a, C, A> {
        self._delegate = Some(new_value);
        self
    }

    /// Set any additional parameter of the query string used in the request.
    /// It should be used to set parameters which are not yet available through their own
    /// setters.
    ///
    /// Please note that this method must not be used to set any of the known paramters
    /// which have their own setter method. If done anyway, the request will fail.
    ///
    /// # Additional Parameters
    ///
    /// * *upload_protocol* (query-string) - Upload protocol for media (e.g. "raw", "multipart").
    /// * *pp* (query-boolean) - Pretty-print response.
    /// * *prettyPrint* (query-boolean) - Returns response with indentations and line breaks.
    /// * *bearer_token* (query-string) - OAuth bearer token.
    /// * *access_token* (query-string) - OAuth access token.
    /// * *uploadType* (query-string) - Legacy upload protocol for media (e.g. "media", "multipart").
    /// * *quotaUser* (query-string) - Available to use for quota purposes for server-side applications. Can be any arbitrary string assigned to a user, but should not exceed 40 characters.
    /// * *callback* (query-string) - JSONP
    /// * *oauth_token* (query-string) - OAuth 2.0 token for the current user.
    /// * *key* (query-string) - API key. Your API key identifies your project and provides you with API access, quota, and reports. Required unless you provide an OAuth 2.0 token.
    /// * *fields* (query-string) - Selector specifying which fields to include in a partial response.
    /// * *alt* (query-string) - Data format for response.
    /// * *$.xgafv* (query-string) - V1 error format.
    pub fn param<T>(mut self, name: T, value: T) -> UserProfileGuardianInvitationCreateCall<'a, C, A>
                                                        where T: AsRef<str> {
        self._additional_params.insert(name.as_ref().to_string(), value.as_ref().to_string());
        self
    }

}


/// Returns a user profile.
/// 
/// This method returns the following error codes:
/// 
/// * `PERMISSION_DENIED` if the requesting user is not permitted to access
/// this user profile, if no profile exists with the requested ID, or for
/// access errors.
///
/// A builder for the *get* method supported by a *userProfile* resource.
/// It is not used directly, but through a `UserProfileMethods` instance.
///
/// # Example
///
/// Instantiate a resource method builder
///
/// ```test_harness,no_run
/// # extern crate hyper;
/// # extern crate hyper_rustls;
/// # extern crate yup_oauth2 as oauth2;
/// # extern crate google_classroom1 as classroom1;
/// # #[test] fn egal() {
/// # use std::default::Default;
/// # use oauth2::{Authenticator, DefaultAuthenticatorDelegate, ApplicationSecret, MemoryStorage};
/// # use classroom1::Classroom;
/// 
/// # let secret: ApplicationSecret = Default::default();
/// # let auth = Authenticator::new(&secret, DefaultAuthenticatorDelegate,
/// #                               hyper::Client::with_connector(hyper::net::HttpsConnector::new(hyper_rustls::TlsClient::new())),
/// #                               <MemoryStorage as Default>::default(), None);
/// # let mut hub = Classroom::new(hyper::Client::with_connector(hyper::net::HttpsConnector::new(hyper_rustls::TlsClient::new())), auth);
/// // You can configure optional parameters by calling the respective setters at will, and
/// // execute the final call using `doit()`.
/// // Values shown here are possibly random and not representative !
/// let result = hub.user_profiles().get("userId")
///              .doit();
/// # }
/// ```
pub struct UserProfileGetCall<'a, C, A>
    where C: 'a, A: 'a {

    hub: &'a Classroom<C, A>,
    _user_id: String,
    _delegate: Option<&'a mut Delegate>,
    _additional_params: HashMap<String, String>,
    _scopes: BTreeMap<String, ()>
}

impl<'a, C, A> CallBuilder for UserProfileGetCall<'a, C, A> {}

impl<'a, C, A> UserProfileGetCall<'a, C, A> where C: BorrowMut<hyper::Client>, A: oauth2::GetToken {


    /// Perform the operation you have build so far.
    pub fn doit(mut self) -> Result<(hyper::client::Response, UserProfile)> {
        use std::io::{Read, Seek};
        use hyper::header::{ContentType, ContentLength, Authorization, Bearer, UserAgent, Location};
        let mut dd = DefaultDelegate;
        let mut dlg: &mut Delegate = match self._delegate {
            Some(d) => d,
            None => &mut dd
        };
        dlg.begin(MethodInfo { id: "classroom.userProfiles.get",
                               http_method: hyper::method::Method::Get });
        let mut params: Vec<(&str, String)> = Vec::with_capacity((3 + self._additional_params.len()));
        params.push(("userId", self._user_id.to_string()));
        for &field in ["alt", "userId"].iter() {
            if self._additional_params.contains_key(field) {
                dlg.finished(false);
                return Err(Error::FieldClash(field));
            }
        }
        for (name, value) in self._additional_params.iter() {
            params.push((&name, value.clone()));
        }

        params.push(("alt", "json".to_string()));

        let mut url = self.hub._base_url.clone() + "v1/userProfiles/{userId}";
        if self._scopes.len() == 0 {
            self._scopes.insert(Scope::RosterReadonly.as_ref().to_string(), ());
        }

        for &(find_this, param_name) in [("{userId}", "userId")].iter() {
            let mut replace_with: Option<&str> = None;
            for &(name, ref value) in params.iter() {
                if name == param_name {
                    replace_with = Some(value);
                    break;
                }
            }
            url = url.replace(find_this, replace_with.expect("to find substitution value in params"));
        }
        {
            let mut indices_for_removal: Vec<usize> = Vec::with_capacity(1);
            for param_name in ["userId"].iter() {
                if let Some(index) = params.iter().position(|t| &t.0 == param_name) {
                    indices_for_removal.push(index);
                }
            }
            for &index in indices_for_removal.iter() {
                params.remove(index);
            }
        }

        if params.len() > 0 {
            url.push('?');
            url.push_str(&url::form_urlencoded::serialize(params));
        }



        loop {
            let token = match self.hub.auth.borrow_mut().token(self._scopes.keys()) {
                Ok(token) => token,
                Err(err) => {
                    match  dlg.token(&*err) {
                        Some(token) => token,
                        None => {
                            dlg.finished(false);
                            return Err(Error::MissingToken(err))
                        }
                    }
                }
            };
            let auth_header = Authorization(Bearer { token: token.access_token });
            let mut req_result = {
                let mut client = &mut *self.hub.client.borrow_mut();
                let mut req = client.borrow_mut().request(hyper::method::Method::Get, &url)
                    .header(UserAgent(self.hub._user_agent.clone()))
                    .header(auth_header.clone());

                dlg.pre_request();
                req.send()
            };

            match req_result {
                Err(err) => {
                    if let oauth2::Retry::After(d) = dlg.http_error(&err) {
                        sleep(d);
                        continue;
                    }
                    dlg.finished(false);
                    return Err(Error::HttpError(err))
                }
                Ok(mut res) => {
                    if !res.status.is_success() {
                        let mut json_err = String::new();
                        res.read_to_string(&mut json_err).unwrap();
                        if let oauth2::Retry::After(d) = dlg.http_failure(&res,
                                                              json::from_str(&json_err).ok(),
                                                              json::from_str(&json_err).ok()) {
                            sleep(d);
                            continue;
                        }
                        dlg.finished(false);
                        return match json::from_str::<ErrorResponse>(&json_err){
                            Err(_) => Err(Error::Failure(res)),
                            Ok(serr) => Err(Error::BadRequest(serr))
                        }
                    }
                    let result_value = {
                        let mut json_response = String::new();
                        res.read_to_string(&mut json_response).unwrap();
                        match json::from_str(&json_response) {
                            Ok(decoded) => (res, decoded),
                            Err(err) => {
                                dlg.response_json_decode_error(&json_response, &err);
                                return Err(Error::JsonDecodeError(json_response, err));
                            }
                        }
                    };

                    dlg.finished(true);
                    return Ok(result_value)
                }
            }
        }
    }


    /// Identifier of the profile to return. The identifier can be one of the
    /// following:
    /// 
    /// * the numeric identifier for the user
    /// * the email address of the user
    /// * the string literal `"me"`, indicating the requesting user
    ///
    /// Sets the *user id* path property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call,
    /// we provide this method for API completeness.
    pub fn user_id(mut self, new_value: &str) -> UserProfileGetCall<'a, C, A> {
        self._user_id = new_value.to_string();
        self
    }
    /// The delegate implementation is consulted whenever there is an intermediate result, or if something goes wrong
    /// while executing the actual API request.
    /// 
    /// It should be used to handle progress information, and to implement a certain level of resilience.
    ///
    /// Sets the *delegate* property to the given value.
    pub fn delegate(mut self, new_value: &'a mut Delegate) -> UserProfileGetCall<'a, C, A> {
        self._delegate = Some(new_value);
        self
    }

    /// Set any additional parameter of the query string used in the request.
    /// It should be used to set parameters which are not yet available through their own
    /// setters.
    ///
    /// Please note that this method must not be used to set any of the known paramters
    /// which have their own setter method. If done anyway, the request will fail.
    ///
    /// # Additional Parameters
    ///
    /// * *upload_protocol* (query-string) - Upload protocol for media (e.g. "raw", "multipart").
    /// * *pp* (query-boolean) - Pretty-print response.
    /// * *prettyPrint* (query-boolean) - Returns response with indentations and line breaks.
    /// * *bearer_token* (query-string) - OAuth bearer token.
    /// * *access_token* (query-string) - OAuth access token.
    /// * *uploadType* (query-string) - Legacy upload protocol for media (e.g. "media", "multipart").
    /// * *quotaUser* (query-string) - Available to use for quota purposes for server-side applications. Can be any arbitrary string assigned to a user, but should not exceed 40 characters.
    /// * *callback* (query-string) - JSONP
    /// * *oauth_token* (query-string) - OAuth 2.0 token for the current user.
    /// * *key* (query-string) - API key. Your API key identifies your project and provides you with API access, quota, and reports. Required unless you provide an OAuth 2.0 token.
    /// * *fields* (query-string) - Selector specifying which fields to include in a partial response.
    /// * *alt* (query-string) - Data format for response.
    /// * *$.xgafv* (query-string) - V1 error format.
    pub fn param<T>(mut self, name: T, value: T) -> UserProfileGetCall<'a, C, A>
                                                        where T: AsRef<str> {
        self._additional_params.insert(name.as_ref().to_string(), value.as_ref().to_string());
        self
    }

    /// Identifies the authorization scope for the method you are building.
    ///
    /// Use this method to actively specify which scope should be used, instead the default `Scope` variant
    /// `Scope::RosterReadonly`.
    ///
    /// The `scope` will be added to a set of scopes. This is important as one can maintain access
    /// tokens for more than one scope.
    /// If `None` is specified, then all scopes will be removed and no default scope will be used either.
    /// In that case, you have to specify your API-key using the `key` parameter (see the `param()`
    /// function for details).
    ///
    /// Usually there is more than one suitable scope to authorize an operation, some of which may
    /// encompass more rights than others. For example, for listing resources, a *read-only* scope will be
    /// sufficient, a read-write scope will do as well.
    pub fn add_scope<T, S>(mut self, scope: T) -> UserProfileGetCall<'a, C, A>
                                                        where T: Into<Option<S>>,
                                                              S: AsRef<str> {
        match scope.into() {
          Some(scope) => self._scopes.insert(scope.as_ref().to_string(), ()),
          None => None,
        };
        self
    }
}


/// Modifies a guardian invitation.
/// 
/// Currently, the only valid modification is to change the `state` from
/// `PENDING` to `COMPLETE`. This has the effect of withdrawing the invitation.
/// 
/// This method returns the following error codes:
/// 
/// * `PERMISSION_DENIED` if the current user does not have permission to
///   manage guardians, if guardians are not enabled for the domain in question
///   or for other access errors.
/// * `FAILED_PRECONDITION` if the guardian link is not in the `PENDING` state.
/// * `INVALID_ARGUMENT` if the format of the student ID provided
///   cannot be recognized (it is not an email address, nor a `user_id` from
///   this API), or if the passed `GuardianInvitation` has a `state` other than
///   `COMPLETE`, or if it modifies fields other than `state`.
/// * `NOT_FOUND` if the student ID provided is a valid student ID, but
///   Classroom has no record of that student, or if the `id` field does not
///   refer to a guardian invitation known to Classroom.
///
/// A builder for the *guardianInvitations.patch* method supported by a *userProfile* resource.
/// It is not used directly, but through a `UserProfileMethods` instance.
///
/// # Example
///
/// Instantiate a resource method builder
///
/// ```test_harness,no_run
/// # extern crate hyper;
/// # extern crate hyper_rustls;
/// # extern crate yup_oauth2 as oauth2;
/// # extern crate google_classroom1 as classroom1;
/// use classroom1::GuardianInvitation;
/// # #[test] fn egal() {
/// # use std::default::Default;
/// # use oauth2::{Authenticator, DefaultAuthenticatorDelegate, ApplicationSecret, MemoryStorage};
/// # use classroom1::Classroom;
/// 
/// # let secret: ApplicationSecret = Default::default();
/// # let auth = Authenticator::new(&secret, DefaultAuthenticatorDelegate,
/// #                               hyper::Client::with_connector(hyper::net::HttpsConnector::new(hyper_rustls::TlsClient::new())),
/// #                               <MemoryStorage as Default>::default(), None);
/// # let mut hub = Classroom::new(hyper::Client::with_connector(hyper::net::HttpsConnector::new(hyper_rustls::TlsClient::new())), auth);
/// // As the method needs a request, you would usually fill it with the desired information
/// // into the respective structure. Some of the parts shown here might not be applicable !
/// // Values shown here are possibly random and not representative !
/// let mut req = GuardianInvitation::default();
/// 
/// // You can configure optional parameters by calling the respective setters at will, and
/// // execute the final call using `doit()`.
/// // Values shown here are possibly random and not representative !
/// let result = hub.user_profiles().guardian_invitations_patch(req, "studentId", "invitationId")
///              .update_mask("dolore")
///              .doit();
/// # }
/// ```
pub struct UserProfileGuardianInvitationPatchCall<'a, C, A>
    where C: 'a, A: 'a {

    hub: &'a Classroom<C, A>,
    _request: GuardianInvitation,
    _student_id: String,
    _invitation_id: String,
    _update_mask: Option<String>,
    _delegate: Option<&'a mut Delegate>,
    _additional_params: HashMap<String, String>,
}

impl<'a, C, A> CallBuilder for UserProfileGuardianInvitationPatchCall<'a, C, A> {}

impl<'a, C, A> UserProfileGuardianInvitationPatchCall<'a, C, A> where C: BorrowMut<hyper::Client>, A: oauth2::GetToken {


    /// Perform the operation you have build so far.
    pub fn doit(mut self) -> Result<(hyper::client::Response, GuardianInvitation)> {
        use std::io::{Read, Seek};
        use hyper::header::{ContentType, ContentLength, Authorization, Bearer, UserAgent, Location};
        let mut dd = DefaultDelegate;
        let mut dlg: &mut Delegate = match self._delegate {
            Some(d) => d,
            None => &mut dd
        };
        dlg.begin(MethodInfo { id: "classroom.userProfiles.guardianInvitations.patch",
                               http_method: hyper::method::Method::Patch });
        let mut params: Vec<(&str, String)> = Vec::with_capacity((6 + self._additional_params.len()));
        params.push(("studentId", self._student_id.to_string()));
        params.push(("invitationId", self._invitation_id.to_string()));
        if let Some(value) = self._update_mask {
            params.push(("updateMask", value.to_string()));
        }
        for &field in ["alt", "studentId", "invitationId", "updateMask"].iter() {
            if self._additional_params.contains_key(field) {
                dlg.finished(false);
                return Err(Error::FieldClash(field));
            }
        }
        for (name, value) in self._additional_params.iter() {
            params.push((&name, value.clone()));
        }

        params.push(("alt", "json".to_string()));

        let mut url = self.hub._base_url.clone() + "v1/userProfiles/{studentId}/guardianInvitations/{invitationId}";
        
        let mut key = self.hub.auth.borrow_mut().api_key();
        if key.is_none() {
            key = dlg.api_key();
        }
        match key {
            Some(value) => params.push(("key", value)),
            None => {
                dlg.finished(false);
                return Err(Error::MissingAPIKey)
            }
        }

        for &(find_this, param_name) in [("{studentId}", "studentId"), ("{invitationId}", "invitationId")].iter() {
            let mut replace_with: Option<&str> = None;
            for &(name, ref value) in params.iter() {
                if name == param_name {
                    replace_with = Some(value);
                    break;
                }
            }
            url = url.replace(find_this, replace_with.expect("to find substitution value in params"));
        }
        {
            let mut indices_for_removal: Vec<usize> = Vec::with_capacity(2);
            for param_name in ["invitationId", "studentId"].iter() {
                if let Some(index) = params.iter().position(|t| &t.0 == param_name) {
                    indices_for_removal.push(index);
                }
            }
            for &index in indices_for_removal.iter() {
                params.remove(index);
            }
        }

        if params.len() > 0 {
            url.push('?');
            url.push_str(&url::form_urlencoded::serialize(params));
        }

        let mut json_mime_type = mime::Mime(mime::TopLevel::Application, mime::SubLevel::Json, Default::default());
        let mut request_value_reader =
            {
                let mut value = json::value::to_value(&self._request).expect("serde to work");
                remove_json_null_values(&mut value);
                let mut dst = io::Cursor::new(Vec::with_capacity(128));
                json::to_writer(&mut dst, &value).unwrap();
                dst
            };
        let request_size = request_value_reader.seek(io::SeekFrom::End(0)).unwrap();
        request_value_reader.seek(io::SeekFrom::Start(0)).unwrap();


        loop {
            request_value_reader.seek(io::SeekFrom::Start(0)).unwrap();
            let mut req_result = {
                let mut client = &mut *self.hub.client.borrow_mut();
                let mut req = client.borrow_mut().request(hyper::method::Method::Patch, &url)
                    .header(UserAgent(self.hub._user_agent.clone()))
                    .header(ContentType(json_mime_type.clone()))
                    .header(ContentLength(request_size as u64))
                    .body(&mut request_value_reader);

                dlg.pre_request();
                req.send()
            };

            match req_result {
                Err(err) => {
                    if let oauth2::Retry::After(d) = dlg.http_error(&err) {
                        sleep(d);
                        continue;
                    }
                    dlg.finished(false);
                    return Err(Error::HttpError(err))
                }
                Ok(mut res) => {
                    if !res.status.is_success() {
                        let mut json_err = String::new();
                        res.read_to_string(&mut json_err).unwrap();
                        if let oauth2::Retry::After(d) = dlg.http_failure(&res,
                                                              json::from_str(&json_err).ok(),
                                                              json::from_str(&json_err).ok()) {
                            sleep(d);
                            continue;
                        }
                        dlg.finished(false);
                        return match json::from_str::<ErrorResponse>(&json_err){
                            Err(_) => Err(Error::Failure(res)),
                            Ok(serr) => Err(Error::BadRequest(serr))
                        }
                    }
                    let result_value = {
                        let mut json_response = String::new();
                        res.read_to_string(&mut json_response).unwrap();
                        match json::from_str(&json_response) {
                            Ok(decoded) => (res, decoded),
                            Err(err) => {
                                dlg.response_json_decode_error(&json_response, &err);
                                return Err(Error::JsonDecodeError(json_response, err));
                            }
                        }
                    };

                    dlg.finished(true);
                    return Ok(result_value)
                }
            }
        }
    }


    ///
    /// Sets the *request* property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call,
    /// we provide this method for API completeness.
    pub fn request(mut self, new_value: GuardianInvitation) -> UserProfileGuardianInvitationPatchCall<'a, C, A> {
        self._request = new_value;
        self
    }
    /// The ID of the student whose guardian invitation is to be modified.
    ///
    /// Sets the *student id* path property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call,
    /// we provide this method for API completeness.
    pub fn student_id(mut self, new_value: &str) -> UserProfileGuardianInvitationPatchCall<'a, C, A> {
        self._student_id = new_value.to_string();
        self
    }
    /// The `id` field of the `GuardianInvitation` to be modified.
    ///
    /// Sets the *invitation id* path property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call,
    /// we provide this method for API completeness.
    pub fn invitation_id(mut self, new_value: &str) -> UserProfileGuardianInvitationPatchCall<'a, C, A> {
        self._invitation_id = new_value.to_string();
        self
    }
    /// Mask that identifies which fields on the course to update.
    /// This field is required to do an update. The update will fail if invalid
    /// fields are specified. The following fields are valid:
    /// 
    /// * `state`
    /// 
    /// When set in a query parameter, this field should be specified as
    /// 
    /// `updateMask=<field1>,<field2>,...`
    ///
    /// Sets the *update mask* query property to the given value.
    pub fn update_mask(mut self, new_value: &str) -> UserProfileGuardianInvitationPatchCall<'a, C, A> {
        self._update_mask = Some(new_value.to_string());
        self
    }
    /// The delegate implementation is consulted whenever there is an intermediate result, or if something goes wrong
    /// while executing the actual API request.
    /// 
    /// It should be used to handle progress information, and to implement a certain level of resilience.
    ///
    /// Sets the *delegate* property to the given value.
    pub fn delegate(mut self, new_value: &'a mut Delegate) -> UserProfileGuardianInvitationPatchCall<'a, C, A> {
        self._delegate = Some(new_value);
        self
    }

    /// Set any additional parameter of the query string used in the request.
    /// It should be used to set parameters which are not yet available through their own
    /// setters.
    ///
    /// Please note that this method must not be used to set any of the known paramters
    /// which have their own setter method. If done anyway, the request will fail.
    ///
    /// # Additional Parameters
    ///
    /// * *upload_protocol* (query-string) - Upload protocol for media (e.g. "raw", "multipart").
    /// * *pp* (query-boolean) - Pretty-print response.
    /// * *prettyPrint* (query-boolean) - Returns response with indentations and line breaks.
    /// * *bearer_token* (query-string) - OAuth bearer token.
    /// * *access_token* (query-string) - OAuth access token.
    /// * *uploadType* (query-string) - Legacy upload protocol for media (e.g. "media", "multipart").
    /// * *quotaUser* (query-string) - Available to use for quota purposes for server-side applications. Can be any arbitrary string assigned to a user, but should not exceed 40 characters.
    /// * *callback* (query-string) - JSONP
    /// * *oauth_token* (query-string) - OAuth 2.0 token for the current user.
    /// * *key* (query-string) - API key. Your API key identifies your project and provides you with API access, quota, and reports. Required unless you provide an OAuth 2.0 token.
    /// * *fields* (query-string) - Selector specifying which fields to include in a partial response.
    /// * *alt* (query-string) - Data format for response.
    /// * *$.xgafv* (query-string) - V1 error format.
    pub fn param<T>(mut self, name: T, value: T) -> UserProfileGuardianInvitationPatchCall<'a, C, A>
                                                        where T: AsRef<str> {
        self._additional_params.insert(name.as_ref().to_string(), value.as_ref().to_string());
        self
    }

}


/// Returns a specific guardian invitation.
/// 
/// This method returns the following error codes:
/// 
/// * `PERMISSION_DENIED` if the requesting user is not permitted to view
///   guardian invitations for the student identified by the `student_id`, if
///   guardians are not enabled for the domain in question, or for other
///   access errors.
/// * `INVALID_ARGUMENT` if a `student_id` is specified, but its format cannot
///   be recognized (it is not an email address, nor a `student_id` from the
///   API, nor the literal string `me`).
/// * `NOT_FOUND` if Classroom cannot find any record of the given student or
///   `invitation_id`. May also be returned if the student exists, but the
///   requesting user does not have access to see that student.
///
/// A builder for the *guardianInvitations.get* method supported by a *userProfile* resource.
/// It is not used directly, but through a `UserProfileMethods` instance.
///
/// # Example
///
/// Instantiate a resource method builder
///
/// ```test_harness,no_run
/// # extern crate hyper;
/// # extern crate hyper_rustls;
/// # extern crate yup_oauth2 as oauth2;
/// # extern crate google_classroom1 as classroom1;
/// # #[test] fn egal() {
/// # use std::default::Default;
/// # use oauth2::{Authenticator, DefaultAuthenticatorDelegate, ApplicationSecret, MemoryStorage};
/// # use classroom1::Classroom;
/// 
/// # let secret: ApplicationSecret = Default::default();
/// # let auth = Authenticator::new(&secret, DefaultAuthenticatorDelegate,
/// #                               hyper::Client::with_connector(hyper::net::HttpsConnector::new(hyper_rustls::TlsClient::new())),
/// #                               <MemoryStorage as Default>::default(), None);
/// # let mut hub = Classroom::new(hyper::Client::with_connector(hyper::net::HttpsConnector::new(hyper_rustls::TlsClient::new())), auth);
/// // You can configure optional parameters by calling the respective setters at will, and
/// // execute the final call using `doit()`.
/// // Values shown here are possibly random and not representative !
/// let result = hub.user_profiles().guardian_invitations_get("studentId", "invitationId")
///              .doit();
/// # }
/// ```
pub struct UserProfileGuardianInvitationGetCall<'a, C, A>
    where C: 'a, A: 'a {

    hub: &'a Classroom<C, A>,
    _student_id: String,
    _invitation_id: String,
    _delegate: Option<&'a mut Delegate>,
    _additional_params: HashMap<String, String>,
}

impl<'a, C, A> CallBuilder for UserProfileGuardianInvitationGetCall<'a, C, A> {}

impl<'a, C, A> UserProfileGuardianInvitationGetCall<'a, C, A> where C: BorrowMut<hyper::Client>, A: oauth2::GetToken {


    /// Perform the operation you have build so far.
    pub fn doit(mut self) -> Result<(hyper::client::Response, GuardianInvitation)> {
        use std::io::{Read, Seek};
        use hyper::header::{ContentType, ContentLength, Authorization, Bearer, UserAgent, Location};
        let mut dd = DefaultDelegate;
        let mut dlg: &mut Delegate = match self._delegate {
            Some(d) => d,
            None => &mut dd
        };
        dlg.begin(MethodInfo { id: "classroom.userProfiles.guardianInvitations.get",
                               http_method: hyper::method::Method::Get });
        let mut params: Vec<(&str, String)> = Vec::with_capacity((4 + self._additional_params.len()));
        params.push(("studentId", self._student_id.to_string()));
        params.push(("invitationId", self._invitation_id.to_string()));
        for &field in ["alt", "studentId", "invitationId"].iter() {
            if self._additional_params.contains_key(field) {
                dlg.finished(false);
                return Err(Error::FieldClash(field));
            }
        }
        for (name, value) in self._additional_params.iter() {
            params.push((&name, value.clone()));
        }

        params.push(("alt", "json".to_string()));

        let mut url = self.hub._base_url.clone() + "v1/userProfiles/{studentId}/guardianInvitations/{invitationId}";
        
        let mut key = self.hub.auth.borrow_mut().api_key();
        if key.is_none() {
            key = dlg.api_key();
        }
        match key {
            Some(value) => params.push(("key", value)),
            None => {
                dlg.finished(false);
                return Err(Error::MissingAPIKey)
            }
        }

        for &(find_this, param_name) in [("{studentId}", "studentId"), ("{invitationId}", "invitationId")].iter() {
            let mut replace_with: Option<&str> = None;
            for &(name, ref value) in params.iter() {
                if name == param_name {
                    replace_with = Some(value);
                    break;
                }
            }
            url = url.replace(find_this, replace_with.expect("to find substitution value in params"));
        }
        {
            let mut indices_for_removal: Vec<usize> = Vec::with_capacity(2);
            for param_name in ["invitationId", "studentId"].iter() {
                if let Some(index) = params.iter().position(|t| &t.0 == param_name) {
                    indices_for_removal.push(index);
                }
            }
            for &index in indices_for_removal.iter() {
                params.remove(index);
            }
        }

        if params.len() > 0 {
            url.push('?');
            url.push_str(&url::form_urlencoded::serialize(params));
        }



        loop {
            let mut req_result = {
                let mut client = &mut *self.hub.client.borrow_mut();
                let mut req = client.borrow_mut().request(hyper::method::Method::Get, &url)
                    .header(UserAgent(self.hub._user_agent.clone()));

                dlg.pre_request();
                req.send()
            };

            match req_result {
                Err(err) => {
                    if let oauth2::Retry::After(d) = dlg.http_error(&err) {
                        sleep(d);
                        continue;
                    }
                    dlg.finished(false);
                    return Err(Error::HttpError(err))
                }
                Ok(mut res) => {
                    if !res.status.is_success() {
                        let mut json_err = String::new();
                        res.read_to_string(&mut json_err).unwrap();
                        if let oauth2::Retry::After(d) = dlg.http_failure(&res,
                                                              json::from_str(&json_err).ok(),
                                                              json::from_str(&json_err).ok()) {
                            sleep(d);
                            continue;
                        }
                        dlg.finished(false);
                        return match json::from_str::<ErrorResponse>(&json_err){
                            Err(_) => Err(Error::Failure(res)),
                            Ok(serr) => Err(Error::BadRequest(serr))
                        }
                    }
                    let result_value = {
                        let mut json_response = String::new();
                        res.read_to_string(&mut json_response).unwrap();
                        match json::from_str(&json_response) {
                            Ok(decoded) => (res, decoded),
                            Err(err) => {
                                dlg.response_json_decode_error(&json_response, &err);
                                return Err(Error::JsonDecodeError(json_response, err));
                            }
                        }
                    };

                    dlg.finished(true);
                    return Ok(result_value)
                }
            }
        }
    }


    /// The ID of the student whose guardian invitation is being requested.
    ///
    /// Sets the *student id* path property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call,
    /// we provide this method for API completeness.
    pub fn student_id(mut self, new_value: &str) -> UserProfileGuardianInvitationGetCall<'a, C, A> {
        self._student_id = new_value.to_string();
        self
    }
    /// The `id` field of the `GuardianInvitation` being requested.
    ///
    /// Sets the *invitation id* path property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call,
    /// we provide this method for API completeness.
    pub fn invitation_id(mut self, new_value: &str) -> UserProfileGuardianInvitationGetCall<'a, C, A> {
        self._invitation_id = new_value.to_string();
        self
    }
    /// The delegate implementation is consulted whenever there is an intermediate result, or if something goes wrong
    /// while executing the actual API request.
    /// 
    /// It should be used to handle progress information, and to implement a certain level of resilience.
    ///
    /// Sets the *delegate* property to the given value.
    pub fn delegate(mut self, new_value: &'a mut Delegate) -> UserProfileGuardianInvitationGetCall<'a, C, A> {
        self._delegate = Some(new_value);
        self
    }

    /// Set any additional parameter of the query string used in the request.
    /// It should be used to set parameters which are not yet available through their own
    /// setters.
    ///
    /// Please note that this method must not be used to set any of the known paramters
    /// which have their own setter method. If done anyway, the request will fail.
    ///
    /// # Additional Parameters
    ///
    /// * *upload_protocol* (query-string) - Upload protocol for media (e.g. "raw", "multipart").
    /// * *pp* (query-boolean) - Pretty-print response.
    /// * *prettyPrint* (query-boolean) - Returns response with indentations and line breaks.
    /// * *bearer_token* (query-string) - OAuth bearer token.
    /// * *access_token* (query-string) - OAuth access token.
    /// * *uploadType* (query-string) - Legacy upload protocol for media (e.g. "media", "multipart").
    /// * *quotaUser* (query-string) - Available to use for quota purposes for server-side applications. Can be any arbitrary string assigned to a user, but should not exceed 40 characters.
    /// * *callback* (query-string) - JSONP
    /// * *oauth_token* (query-string) - OAuth 2.0 token for the current user.
    /// * *key* (query-string) - API key. Your API key identifies your project and provides you with API access, quota, and reports. Required unless you provide an OAuth 2.0 token.
    /// * *fields* (query-string) - Selector specifying which fields to include in a partial response.
    /// * *alt* (query-string) - Data format for response.
    /// * *$.xgafv* (query-string) - V1 error format.
    pub fn param<T>(mut self, name: T, value: T) -> UserProfileGuardianInvitationGetCall<'a, C, A>
                                                        where T: AsRef<str> {
        self._additional_params.insert(name.as_ref().to_string(), value.as_ref().to_string());
        self
    }

}


/// Returns a list of guardians that the requesting user is permitted to
/// view, restricted to those that match the request.
/// 
/// To list guardians for any student that the requesting user may view
/// guardians for, use the literal character `-` for the student ID.
/// 
/// This method returns the following error codes:
/// 
/// * `PERMISSION_DENIED` if a `student_id` is specified, and the requesting
///   user is not permitted to view guardian information for that student, if
///   `"-"` is specified as the `student_id` and the user is not a domain
///   administrator, if guardians are not enabled for the domain in question,
///   if the `invited_email_address` filter is set by a user who is not a
///   domain administrator, or for other access errors.
/// * `INVALID_ARGUMENT` if a `student_id` is specified, but its format cannot
///   be recognized (it is not an email address, nor a `student_id` from the
///   API, nor the literal string `me`). May also be returned if an invalid
///   `page_token` is provided.
/// * `NOT_FOUND` if a `student_id` is specified, and its format can be
///   recognized, but Classroom has no record of that student.
///
/// A builder for the *guardians.list* method supported by a *userProfile* resource.
/// It is not used directly, but through a `UserProfileMethods` instance.
///
/// # Example
///
/// Instantiate a resource method builder
///
/// ```test_harness,no_run
/// # extern crate hyper;
/// # extern crate hyper_rustls;
/// # extern crate yup_oauth2 as oauth2;
/// # extern crate google_classroom1 as classroom1;
/// # #[test] fn egal() {
/// # use std::default::Default;
/// # use oauth2::{Authenticator, DefaultAuthenticatorDelegate, ApplicationSecret, MemoryStorage};
/// # use classroom1::Classroom;
/// 
/// # let secret: ApplicationSecret = Default::default();
/// # let auth = Authenticator::new(&secret, DefaultAuthenticatorDelegate,
/// #                               hyper::Client::with_connector(hyper::net::HttpsConnector::new(hyper_rustls::TlsClient::new())),
/// #                               <MemoryStorage as Default>::default(), None);
/// # let mut hub = Classroom::new(hyper::Client::with_connector(hyper::net::HttpsConnector::new(hyper_rustls::TlsClient::new())), auth);
/// // You can configure optional parameters by calling the respective setters at will, and
/// // execute the final call using `doit()`.
/// // Values shown here are possibly random and not representative !
/// let result = hub.user_profiles().guardians_list("studentId")
///              .page_token("sit")
///              .page_size(-40)
///              .invited_email_address("consetetur")
///              .doit();
/// # }
/// ```
pub struct UserProfileGuardianListCall<'a, C, A>
    where C: 'a, A: 'a {

    hub: &'a Classroom<C, A>,
    _student_id: String,
    _page_token: Option<String>,
    _page_size: Option<i32>,
    _invited_email_address: Option<String>,
    _delegate: Option<&'a mut Delegate>,
    _additional_params: HashMap<String, String>,
}

impl<'a, C, A> CallBuilder for UserProfileGuardianListCall<'a, C, A> {}

impl<'a, C, A> UserProfileGuardianListCall<'a, C, A> where C: BorrowMut<hyper::Client>, A: oauth2::GetToken {


    /// Perform the operation you have build so far.
    pub fn doit(mut self) -> Result<(hyper::client::Response, ListGuardiansResponse)> {
        use std::io::{Read, Seek};
        use hyper::header::{ContentType, ContentLength, Authorization, Bearer, UserAgent, Location};
        let mut dd = DefaultDelegate;
        let mut dlg: &mut Delegate = match self._delegate {
            Some(d) => d,
            None => &mut dd
        };
        dlg.begin(MethodInfo { id: "classroom.userProfiles.guardians.list",
                               http_method: hyper::method::Method::Get });
        let mut params: Vec<(&str, String)> = Vec::with_capacity((6 + self._additional_params.len()));
        params.push(("studentId", self._student_id.to_string()));
        if let Some(value) = self._page_token {
            params.push(("pageToken", value.to_string()));
        }
        if let Some(value) = self._page_size {
            params.push(("pageSize", value.to_string()));
        }
        if let Some(value) = self._invited_email_address {
            params.push(("invitedEmailAddress", value.to_string()));
        }
        for &field in ["alt", "studentId", "pageToken", "pageSize", "invitedEmailAddress"].iter() {
            if self._additional_params.contains_key(field) {
                dlg.finished(false);
                return Err(Error::FieldClash(field));
            }
        }
        for (name, value) in self._additional_params.iter() {
            params.push((&name, value.clone()));
        }

        params.push(("alt", "json".to_string()));

        let mut url = self.hub._base_url.clone() + "v1/userProfiles/{studentId}/guardians";
        
        let mut key = self.hub.auth.borrow_mut().api_key();
        if key.is_none() {
            key = dlg.api_key();
        }
        match key {
            Some(value) => params.push(("key", value)),
            None => {
                dlg.finished(false);
                return Err(Error::MissingAPIKey)
            }
        }

        for &(find_this, param_name) in [("{studentId}", "studentId")].iter() {
            let mut replace_with: Option<&str> = None;
            for &(name, ref value) in params.iter() {
                if name == param_name {
                    replace_with = Some(value);
                    break;
                }
            }
            url = url.replace(find_this, replace_with.expect("to find substitution value in params"));
        }
        {
            let mut indices_for_removal: Vec<usize> = Vec::with_capacity(1);
            for param_name in ["studentId"].iter() {
                if let Some(index) = params.iter().position(|t| &t.0 == param_name) {
                    indices_for_removal.push(index);
                }
            }
            for &index in indices_for_removal.iter() {
                params.remove(index);
            }
        }

        if params.len() > 0 {
            url.push('?');
            url.push_str(&url::form_urlencoded::serialize(params));
        }



        loop {
            let mut req_result = {
                let mut client = &mut *self.hub.client.borrow_mut();
                let mut req = client.borrow_mut().request(hyper::method::Method::Get, &url)
                    .header(UserAgent(self.hub._user_agent.clone()));

                dlg.pre_request();
                req.send()
            };

            match req_result {
                Err(err) => {
                    if let oauth2::Retry::After(d) = dlg.http_error(&err) {
                        sleep(d);
                        continue;
                    }
                    dlg.finished(false);
                    return Err(Error::HttpError(err))
                }
                Ok(mut res) => {
                    if !res.status.is_success() {
                        let mut json_err = String::new();
                        res.read_to_string(&mut json_err).unwrap();
                        if let oauth2::Retry::After(d) = dlg.http_failure(&res,
                                                              json::from_str(&json_err).ok(),
                                                              json::from_str(&json_err).ok()) {
                            sleep(d);
                            continue;
                        }
                        dlg.finished(false);
                        return match json::from_str::<ErrorResponse>(&json_err){
                            Err(_) => Err(Error::Failure(res)),
                            Ok(serr) => Err(Error::BadRequest(serr))
                        }
                    }
                    let result_value = {
                        let mut json_response = String::new();
                        res.read_to_string(&mut json_response).unwrap();
                        match json::from_str(&json_response) {
                            Ok(decoded) => (res, decoded),
                            Err(err) => {
                                dlg.response_json_decode_error(&json_response, &err);
                                return Err(Error::JsonDecodeError(json_response, err));
                            }
                        }
                    };

                    dlg.finished(true);
                    return Ok(result_value)
                }
            }
        }
    }


    /// Filter results by the student who the guardian is linked to.
    /// The identifier can be one of the following:
    /// 
    /// * the numeric identifier for the user
    /// * the email address of the user
    /// * the string literal `"me"`, indicating the requesting user
    /// * the string literal `"-"`, indicating that results should be returned for
    ///   all students that the requesting user has access to view.
    ///
    /// Sets the *student id* path property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call,
    /// we provide this method for API completeness.
    pub fn student_id(mut self, new_value: &str) -> UserProfileGuardianListCall<'a, C, A> {
        self._student_id = new_value.to_string();
        self
    }
    /// nextPageToken
    /// value returned from a previous
    /// list call,
    /// indicating that the subsequent page of results should be returned.
    /// 
    /// The list request
    /// must be otherwise identical to the one that resulted in this token.
    ///
    /// Sets the *page token* query property to the given value.
    pub fn page_token(mut self, new_value: &str) -> UserProfileGuardianListCall<'a, C, A> {
        self._page_token = Some(new_value.to_string());
        self
    }
    /// Maximum number of items to return. Zero or unspecified indicates that the
    /// server may assign a maximum.
    /// 
    /// The server may return fewer than the specified number of results.
    ///
    /// Sets the *page size* query property to the given value.
    pub fn page_size(mut self, new_value: i32) -> UserProfileGuardianListCall<'a, C, A> {
        self._page_size = Some(new_value);
        self
    }
    /// Filter results by the email address that the original invitation was sent
    /// to, resulting in this guardian link.
    /// This filter can only be used by domain administrators.
    ///
    /// Sets the *invited email address* query property to the given value.
    pub fn invited_email_address(mut self, new_value: &str) -> UserProfileGuardianListCall<'a, C, A> {
        self._invited_email_address = Some(new_value.to_string());
        self
    }
    /// The delegate implementation is consulted whenever there is an intermediate result, or if something goes wrong
    /// while executing the actual API request.
    /// 
    /// It should be used to handle progress information, and to implement a certain level of resilience.
    ///
    /// Sets the *delegate* property to the given value.
    pub fn delegate(mut self, new_value: &'a mut Delegate) -> UserProfileGuardianListCall<'a, C, A> {
        self._delegate = Some(new_value);
        self
    }

    /// Set any additional parameter of the query string used in the request.
    /// It should be used to set parameters which are not yet available through their own
    /// setters.
    ///
    /// Please note that this method must not be used to set any of the known paramters
    /// which have their own setter method. If done anyway, the request will fail.
    ///
    /// # Additional Parameters
    ///
    /// * *upload_protocol* (query-string) - Upload protocol for media (e.g. "raw", "multipart").
    /// * *pp* (query-boolean) - Pretty-print response.
    /// * *prettyPrint* (query-boolean) - Returns response with indentations and line breaks.
    /// * *bearer_token* (query-string) - OAuth bearer token.
    /// * *access_token* (query-string) - OAuth access token.
    /// * *uploadType* (query-string) - Legacy upload protocol for media (e.g. "media", "multipart").
    /// * *quotaUser* (query-string) - Available to use for quota purposes for server-side applications. Can be any arbitrary string assigned to a user, but should not exceed 40 characters.
    /// * *callback* (query-string) - JSONP
    /// * *oauth_token* (query-string) - OAuth 2.0 token for the current user.
    /// * *key* (query-string) - API key. Your API key identifies your project and provides you with API access, quota, and reports. Required unless you provide an OAuth 2.0 token.
    /// * *fields* (query-string) - Selector specifying which fields to include in a partial response.
    /// * *alt* (query-string) - Data format for response.
    /// * *$.xgafv* (query-string) - V1 error format.
    pub fn param<T>(mut self, name: T, value: T) -> UserProfileGuardianListCall<'a, C, A>
                                                        where T: AsRef<str> {
        self._additional_params.insert(name.as_ref().to_string(), value.as_ref().to_string());
        self
    }

}


/// Returns a list of guardian invitations that the requesting user is
/// permitted to view, filtered by the parameters provided.
/// 
/// This method returns the following error codes:
/// 
/// * `PERMISSION_DENIED` if a `student_id` is specified, and the requesting
///   user is not permitted to view guardian invitations for that student, if
///   `"-"` is specified as the `student_id` and the user is not a domain
///   administrator, if guardians are not enabled for the domain in question,
///   or for other access errors.
/// * `INVALID_ARGUMENT` if a `student_id` is specified, but its format cannot
///   be recognized (it is not an email address, nor a `student_id` from the
///   API, nor the literal string `me`). May also be returned if an invalid
///   `page_token` or `state` is provided.
/// * `NOT_FOUND` if a `student_id` is specified, and its format can be
///   recognized, but Classroom has no record of that student.
///
/// A builder for the *guardianInvitations.list* method supported by a *userProfile* resource.
/// It is not used directly, but through a `UserProfileMethods` instance.
///
/// # Example
///
/// Instantiate a resource method builder
///
/// ```test_harness,no_run
/// # extern crate hyper;
/// # extern crate hyper_rustls;
/// # extern crate yup_oauth2 as oauth2;
/// # extern crate google_classroom1 as classroom1;
/// # #[test] fn egal() {
/// # use std::default::Default;
/// # use oauth2::{Authenticator, DefaultAuthenticatorDelegate, ApplicationSecret, MemoryStorage};
/// # use classroom1::Classroom;
/// 
/// # let secret: ApplicationSecret = Default::default();
/// # let auth = Authenticator::new(&secret, DefaultAuthenticatorDelegate,
/// #                               hyper::Client::with_connector(hyper::net::HttpsConnector::new(hyper_rustls::TlsClient::new())),
/// #                               <MemoryStorage as Default>::default(), None);
/// # let mut hub = Classroom::new(hyper::Client::with_connector(hyper::net::HttpsConnector::new(hyper_rustls::TlsClient::new())), auth);
/// // You can configure optional parameters by calling the respective setters at will, and
/// // execute the final call using `doit()`.
/// // Values shown here are possibly random and not representative !
/// let result = hub.user_profiles().guardian_invitations_list("studentId")
///              .add_states("sed")
///              .page_token("ea")
///              .page_size(-62)
///              .invited_email_address("aliquyam")
///              .doit();
/// # }
/// ```
pub struct UserProfileGuardianInvitationListCall<'a, C, A>
    where C: 'a, A: 'a {

    hub: &'a Classroom<C, A>,
    _student_id: String,
    _states: Vec<String>,
    _page_token: Option<String>,
    _page_size: Option<i32>,
    _invited_email_address: Option<String>,
    _delegate: Option<&'a mut Delegate>,
    _additional_params: HashMap<String, String>,
}

impl<'a, C, A> CallBuilder for UserProfileGuardianInvitationListCall<'a, C, A> {}

impl<'a, C, A> UserProfileGuardianInvitationListCall<'a, C, A> where C: BorrowMut<hyper::Client>, A: oauth2::GetToken {


    /// Perform the operation you have build so far.
    pub fn doit(mut self) -> Result<(hyper::client::Response, ListGuardianInvitationsResponse)> {
        use std::io::{Read, Seek};
        use hyper::header::{ContentType, ContentLength, Authorization, Bearer, UserAgent, Location};
        let mut dd = DefaultDelegate;
        let mut dlg: &mut Delegate = match self._delegate {
            Some(d) => d,
            None => &mut dd
        };
        dlg.begin(MethodInfo { id: "classroom.userProfiles.guardianInvitations.list",
                               http_method: hyper::method::Method::Get });
        let mut params: Vec<(&str, String)> = Vec::with_capacity((7 + self._additional_params.len()));
        params.push(("studentId", self._student_id.to_string()));
        if self._states.len() > 0 {
            for f in self._states.iter() {
                params.push(("states", f.to_string()));
            }
        }
        if let Some(value) = self._page_token {
            params.push(("pageToken", value.to_string()));
        }
        if let Some(value) = self._page_size {
            params.push(("pageSize", value.to_string()));
        }
        if let Some(value) = self._invited_email_address {
            params.push(("invitedEmailAddress", value.to_string()));
        }
        for &field in ["alt", "studentId", "states", "pageToken", "pageSize", "invitedEmailAddress"].iter() {
            if self._additional_params.contains_key(field) {
                dlg.finished(false);
                return Err(Error::FieldClash(field));
            }
        }
        for (name, value) in self._additional_params.iter() {
            params.push((&name, value.clone()));
        }

        params.push(("alt", "json".to_string()));

        let mut url = self.hub._base_url.clone() + "v1/userProfiles/{studentId}/guardianInvitations";
        
        let mut key = self.hub.auth.borrow_mut().api_key();
        if key.is_none() {
            key = dlg.api_key();
        }
        match key {
            Some(value) => params.push(("key", value)),
            None => {
                dlg.finished(false);
                return Err(Error::MissingAPIKey)
            }
        }

        for &(find_this, param_name) in [("{studentId}", "studentId")].iter() {
            let mut replace_with: Option<&str> = None;
            for &(name, ref value) in params.iter() {
                if name == param_name {
                    replace_with = Some(value);
                    break;
                }
            }
            url = url.replace(find_this, replace_with.expect("to find substitution value in params"));
        }
        {
            let mut indices_for_removal: Vec<usize> = Vec::with_capacity(1);
            for param_name in ["studentId"].iter() {
                if let Some(index) = params.iter().position(|t| &t.0 == param_name) {
                    indices_for_removal.push(index);
                }
            }
            for &index in indices_for_removal.iter() {
                params.remove(index);
            }
        }

        if params.len() > 0 {
            url.push('?');
            url.push_str(&url::form_urlencoded::serialize(params));
        }



        loop {
            let mut req_result = {
                let mut client = &mut *self.hub.client.borrow_mut();
                let mut req = client.borrow_mut().request(hyper::method::Method::Get, &url)
                    .header(UserAgent(self.hub._user_agent.clone()));

                dlg.pre_request();
                req.send()
            };

            match req_result {
                Err(err) => {
                    if let oauth2::Retry::After(d) = dlg.http_error(&err) {
                        sleep(d);
                        continue;
                    }
                    dlg.finished(false);
                    return Err(Error::HttpError(err))
                }
                Ok(mut res) => {
                    if !res.status.is_success() {
                        let mut json_err = String::new();
                        res.read_to_string(&mut json_err).unwrap();
                        if let oauth2::Retry::After(d) = dlg.http_failure(&res,
                                                              json::from_str(&json_err).ok(),
                                                              json::from_str(&json_err).ok()) {
                            sleep(d);
                            continue;
                        }
                        dlg.finished(false);
                        return match json::from_str::<ErrorResponse>(&json_err){
                            Err(_) => Err(Error::Failure(res)),
                            Ok(serr) => Err(Error::BadRequest(serr))
                        }
                    }
                    let result_value = {
                        let mut json_response = String::new();
                        res.read_to_string(&mut json_response).unwrap();
                        match json::from_str(&json_response) {
                            Ok(decoded) => (res, decoded),
                            Err(err) => {
                                dlg.response_json_decode_error(&json_response, &err);
                                return Err(Error::JsonDecodeError(json_response, err));
                            }
                        }
                    };

                    dlg.finished(true);
                    return Ok(result_value)
                }
            }
        }
    }


    /// The ID of the student whose guardian invitations are to be returned.
    /// The identifier can be one of the following:
    /// 
    /// * the numeric identifier for the user
    /// * the email address of the user
    /// * the string literal `"me"`, indicating the requesting user
    /// * the string literal `"-"`, indicating that results should be returned for
    ///   all students that the requesting user is permitted to view guardian
    ///   invitations.
    ///
    /// Sets the *student id* path property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call,
    /// we provide this method for API completeness.
    pub fn student_id(mut self, new_value: &str) -> UserProfileGuardianInvitationListCall<'a, C, A> {
        self._student_id = new_value.to_string();
        self
    }
    /// If specified, only results with the specified `state` values will be
    /// returned. Otherwise, results with a `state` of `PENDING` will be returned.
    ///
    /// Append the given value to the *states* query property.
    /// Each appended value will retain its original ordering and be '/'-separated in the URL's parameters.
    pub fn add_states(mut self, new_value: &str) -> UserProfileGuardianInvitationListCall<'a, C, A> {
        self._states.push(new_value.to_string());
        self
    }
    /// nextPageToken
    /// value returned from a previous
    /// list call,
    /// indicating that the subsequent page of results should be returned.
    /// 
    /// The list request
    /// must be otherwise identical to the one that resulted in this token.
    ///
    /// Sets the *page token* query property to the given value.
    pub fn page_token(mut self, new_value: &str) -> UserProfileGuardianInvitationListCall<'a, C, A> {
        self._page_token = Some(new_value.to_string());
        self
    }
    /// Maximum number of items to return. Zero or unspecified indicates that the
    /// server may assign a maximum.
    /// 
    /// The server may return fewer than the specified number of results.
    ///
    /// Sets the *page size* query property to the given value.
    pub fn page_size(mut self, new_value: i32) -> UserProfileGuardianInvitationListCall<'a, C, A> {
        self._page_size = Some(new_value);
        self
    }
    /// If specified, only results with the specified `invited_email_address`
    /// will be returned.
    ///
    /// Sets the *invited email address* query property to the given value.
    pub fn invited_email_address(mut self, new_value: &str) -> UserProfileGuardianInvitationListCall<'a, C, A> {
        self._invited_email_address = Some(new_value.to_string());
        self
    }
    /// The delegate implementation is consulted whenever there is an intermediate result, or if something goes wrong
    /// while executing the actual API request.
    /// 
    /// It should be used to handle progress information, and to implement a certain level of resilience.
    ///
    /// Sets the *delegate* property to the given value.
    pub fn delegate(mut self, new_value: &'a mut Delegate) -> UserProfileGuardianInvitationListCall<'a, C, A> {
        self._delegate = Some(new_value);
        self
    }

    /// Set any additional parameter of the query string used in the request.
    /// It should be used to set parameters which are not yet available through their own
    /// setters.
    ///
    /// Please note that this method must not be used to set any of the known paramters
    /// which have their own setter method. If done anyway, the request will fail.
    ///
    /// # Additional Parameters
    ///
    /// * *upload_protocol* (query-string) - Upload protocol for media (e.g. "raw", "multipart").
    /// * *pp* (query-boolean) - Pretty-print response.
    /// * *prettyPrint* (query-boolean) - Returns response with indentations and line breaks.
    /// * *bearer_token* (query-string) - OAuth bearer token.
    /// * *access_token* (query-string) - OAuth access token.
    /// * *uploadType* (query-string) - Legacy upload protocol for media (e.g. "media", "multipart").
    /// * *quotaUser* (query-string) - Available to use for quota purposes for server-side applications. Can be any arbitrary string assigned to a user, but should not exceed 40 characters.
    /// * *callback* (query-string) - JSONP
    /// * *oauth_token* (query-string) - OAuth 2.0 token for the current user.
    /// * *key* (query-string) - API key. Your API key identifies your project and provides you with API access, quota, and reports. Required unless you provide an OAuth 2.0 token.
    /// * *fields* (query-string) - Selector specifying which fields to include in a partial response.
    /// * *alt* (query-string) - Data format for response.
    /// * *$.xgafv* (query-string) - V1 error format.
    pub fn param<T>(mut self, name: T, value: T) -> UserProfileGuardianInvitationListCall<'a, C, A>
                                                        where T: AsRef<str> {
        self._additional_params.insert(name.as_ref().to_string(), value.as_ref().to_string());
        self
    }

}


/// Deletes a guardian.
/// 
/// The guardian will no longer receive guardian notifications and the guardian
/// will no longer be accessible via the API.
/// 
/// This method returns the following error codes:
/// 
/// * `PERMISSION_DENIED` if no user that matches the provided `student_id`
///   is visible to the requesting user, if the requesting user is not
///   permitted to manage guardians for the student identified by the
///   `student_id`, if guardians are not enabled for the domain in question,
///   or for other access errors.
/// * `INVALID_ARGUMENT` if a `student_id` is specified, but its format cannot
///   be recognized (it is not an email address, nor a `student_id` from the
///   API).
/// * `NOT_FOUND` if the requesting user is permitted to modify guardians for
///   the requested `student_id`, but no `Guardian` record exists for that
///   student with the provided `guardian_id`.
///
/// A builder for the *guardians.delete* method supported by a *userProfile* resource.
/// It is not used directly, but through a `UserProfileMethods` instance.
///
/// # Example
///
/// Instantiate a resource method builder
///
/// ```test_harness,no_run
/// # extern crate hyper;
/// # extern crate hyper_rustls;
/// # extern crate yup_oauth2 as oauth2;
/// # extern crate google_classroom1 as classroom1;
/// # #[test] fn egal() {
/// # use std::default::Default;
/// # use oauth2::{Authenticator, DefaultAuthenticatorDelegate, ApplicationSecret, MemoryStorage};
/// # use classroom1::Classroom;
/// 
/// # let secret: ApplicationSecret = Default::default();
/// # let auth = Authenticator::new(&secret, DefaultAuthenticatorDelegate,
/// #                               hyper::Client::with_connector(hyper::net::HttpsConnector::new(hyper_rustls::TlsClient::new())),
/// #                               <MemoryStorage as Default>::default(), None);
/// # let mut hub = Classroom::new(hyper::Client::with_connector(hyper::net::HttpsConnector::new(hyper_rustls::TlsClient::new())), auth);
/// // You can configure optional parameters by calling the respective setters at will, and
/// // execute the final call using `doit()`.
/// // Values shown here are possibly random and not representative !
/// let result = hub.user_profiles().guardians_delete("studentId", "guardianId")
///              .doit();
/// # }
/// ```
pub struct UserProfileGuardianDeleteCall<'a, C, A>
    where C: 'a, A: 'a {

    hub: &'a Classroom<C, A>,
    _student_id: String,
    _guardian_id: String,
    _delegate: Option<&'a mut Delegate>,
    _additional_params: HashMap<String, String>,
}

impl<'a, C, A> CallBuilder for UserProfileGuardianDeleteCall<'a, C, A> {}

impl<'a, C, A> UserProfileGuardianDeleteCall<'a, C, A> where C: BorrowMut<hyper::Client>, A: oauth2::GetToken {


    /// Perform the operation you have build so far.
    pub fn doit(mut self) -> Result<(hyper::client::Response, Empty)> {
        use std::io::{Read, Seek};
        use hyper::header::{ContentType, ContentLength, Authorization, Bearer, UserAgent, Location};
        let mut dd = DefaultDelegate;
        let mut dlg: &mut Delegate = match self._delegate {
            Some(d) => d,
            None => &mut dd
        };
        dlg.begin(MethodInfo { id: "classroom.userProfiles.guardians.delete",
                               http_method: hyper::method::Method::Delete });
        let mut params: Vec<(&str, String)> = Vec::with_capacity((4 + self._additional_params.len()));
        params.push(("studentId", self._student_id.to_string()));
        params.push(("guardianId", self._guardian_id.to_string()));
        for &field in ["alt", "studentId", "guardianId"].iter() {
            if self._additional_params.contains_key(field) {
                dlg.finished(false);
                return Err(Error::FieldClash(field));
            }
        }
        for (name, value) in self._additional_params.iter() {
            params.push((&name, value.clone()));
        }

        params.push(("alt", "json".to_string()));

        let mut url = self.hub._base_url.clone() + "v1/userProfiles/{studentId}/guardians/{guardianId}";
        
        let mut key = self.hub.auth.borrow_mut().api_key();
        if key.is_none() {
            key = dlg.api_key();
        }
        match key {
            Some(value) => params.push(("key", value)),
            None => {
                dlg.finished(false);
                return Err(Error::MissingAPIKey)
            }
        }

        for &(find_this, param_name) in [("{studentId}", "studentId"), ("{guardianId}", "guardianId")].iter() {
            let mut replace_with: Option<&str> = None;
            for &(name, ref value) in params.iter() {
                if name == param_name {
                    replace_with = Some(value);
                    break;
                }
            }
            url = url.replace(find_this, replace_with.expect("to find substitution value in params"));
        }
        {
            let mut indices_for_removal: Vec<usize> = Vec::with_capacity(2);
            for param_name in ["guardianId", "studentId"].iter() {
                if let Some(index) = params.iter().position(|t| &t.0 == param_name) {
                    indices_for_removal.push(index);
                }
            }
            for &index in indices_for_removal.iter() {
                params.remove(index);
            }
        }

        if params.len() > 0 {
            url.push('?');
            url.push_str(&url::form_urlencoded::serialize(params));
        }



        loop {
            let mut req_result = {
                let mut client = &mut *self.hub.client.borrow_mut();
                let mut req = client.borrow_mut().request(hyper::method::Method::Delete, &url)
                    .header(UserAgent(self.hub._user_agent.clone()));

                dlg.pre_request();
                req.send()
            };

            match req_result {
                Err(err) => {
                    if let oauth2::Retry::After(d) = dlg.http_error(&err) {
                        sleep(d);
                        continue;
                    }
                    dlg.finished(false);
                    return Err(Error::HttpError(err))
                }
                Ok(mut res) => {
                    if !res.status.is_success() {
                        let mut json_err = String::new();
                        res.read_to_string(&mut json_err).unwrap();
                        if let oauth2::Retry::After(d) = dlg.http_failure(&res,
                                                              json::from_str(&json_err).ok(),
                                                              json::from_str(&json_err).ok()) {
                            sleep(d);
                            continue;
                        }
                        dlg.finished(false);
                        return match json::from_str::<ErrorResponse>(&json_err){
                            Err(_) => Err(Error::Failure(res)),
                            Ok(serr) => Err(Error::BadRequest(serr))
                        }
                    }
                    let result_value = {
                        let mut json_response = String::new();
                        res.read_to_string(&mut json_response).unwrap();
                        match json::from_str(&json_response) {
                            Ok(decoded) => (res, decoded),
                            Err(err) => {
                                dlg.response_json_decode_error(&json_response, &err);
                                return Err(Error::JsonDecodeError(json_response, err));
                            }
                        }
                    };

                    dlg.finished(true);
                    return Ok(result_value)
                }
            }
        }
    }


    /// The student whose guardian is to be deleted. One of the following:
    /// 
    /// * the numeric identifier for the user
    /// * the email address of the user
    /// * the string literal `"me"`, indicating the requesting user
    ///
    /// Sets the *student id* path property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call,
    /// we provide this method for API completeness.
    pub fn student_id(mut self, new_value: &str) -> UserProfileGuardianDeleteCall<'a, C, A> {
        self._student_id = new_value.to_string();
        self
    }
    /// The `id` field from a `Guardian`.
    ///
    /// Sets the *guardian id* path property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call,
    /// we provide this method for API completeness.
    pub fn guardian_id(mut self, new_value: &str) -> UserProfileGuardianDeleteCall<'a, C, A> {
        self._guardian_id = new_value.to_string();
        self
    }
    /// The delegate implementation is consulted whenever there is an intermediate result, or if something goes wrong
    /// while executing the actual API request.
    /// 
    /// It should be used to handle progress information, and to implement a certain level of resilience.
    ///
    /// Sets the *delegate* property to the given value.
    pub fn delegate(mut self, new_value: &'a mut Delegate) -> UserProfileGuardianDeleteCall<'a, C, A> {
        self._delegate = Some(new_value);
        self
    }

    /// Set any additional parameter of the query string used in the request.
    /// It should be used to set parameters which are not yet available through their own
    /// setters.
    ///
    /// Please note that this method must not be used to set any of the known paramters
    /// which have their own setter method. If done anyway, the request will fail.
    ///
    /// # Additional Parameters
    ///
    /// * *upload_protocol* (query-string) - Upload protocol for media (e.g. "raw", "multipart").
    /// * *pp* (query-boolean) - Pretty-print response.
    /// * *prettyPrint* (query-boolean) - Returns response with indentations and line breaks.
    /// * *bearer_token* (query-string) - OAuth bearer token.
    /// * *access_token* (query-string) - OAuth access token.
    /// * *uploadType* (query-string) - Legacy upload protocol for media (e.g. "media", "multipart").
    /// * *quotaUser* (query-string) - Available to use for quota purposes for server-side applications. Can be any arbitrary string assigned to a user, but should not exceed 40 characters.
    /// * *callback* (query-string) - JSONP
    /// * *oauth_token* (query-string) - OAuth 2.0 token for the current user.
    /// * *key* (query-string) - API key. Your API key identifies your project and provides you with API access, quota, and reports. Required unless you provide an OAuth 2.0 token.
    /// * *fields* (query-string) - Selector specifying which fields to include in a partial response.
    /// * *alt* (query-string) - Data format for response.
    /// * *$.xgafv* (query-string) - V1 error format.
    pub fn param<T>(mut self, name: T, value: T) -> UserProfileGuardianDeleteCall<'a, C, A>
                                                        where T: AsRef<str> {
        self._additional_params.insert(name.as_ref().to_string(), value.as_ref().to_string());
        self
    }

}


/// Deletes an invitation.
/// 
/// This method returns the following error codes:
/// 
/// * `PERMISSION_DENIED` if the requesting user is not permitted to delete the
/// requested invitation or for access errors.
/// * `NOT_FOUND` if no invitation exists with the requested ID.
///
/// A builder for the *delete* method supported by a *invitation* resource.
/// It is not used directly, but through a `InvitationMethods` instance.
///
/// # Example
///
/// Instantiate a resource method builder
///
/// ```test_harness,no_run
/// # extern crate hyper;
/// # extern crate hyper_rustls;
/// # extern crate yup_oauth2 as oauth2;
/// # extern crate google_classroom1 as classroom1;
/// # #[test] fn egal() {
/// # use std::default::Default;
/// # use oauth2::{Authenticator, DefaultAuthenticatorDelegate, ApplicationSecret, MemoryStorage};
/// # use classroom1::Classroom;
/// 
/// # let secret: ApplicationSecret = Default::default();
/// # let auth = Authenticator::new(&secret, DefaultAuthenticatorDelegate,
/// #                               hyper::Client::with_connector(hyper::net::HttpsConnector::new(hyper_rustls::TlsClient::new())),
/// #                               <MemoryStorage as Default>::default(), None);
/// # let mut hub = Classroom::new(hyper::Client::with_connector(hyper::net::HttpsConnector::new(hyper_rustls::TlsClient::new())), auth);
/// // You can configure optional parameters by calling the respective setters at will, and
/// // execute the final call using `doit()`.
/// // Values shown here are possibly random and not representative !
/// let result = hub.invitations().delete("id")
///              .doit();
/// # }
/// ```
pub struct InvitationDeleteCall<'a, C, A>
    where C: 'a, A: 'a {

    hub: &'a Classroom<C, A>,
    _id: String,
    _delegate: Option<&'a mut Delegate>,
    _additional_params: HashMap<String, String>,
    _scopes: BTreeMap<String, ()>
}

impl<'a, C, A> CallBuilder for InvitationDeleteCall<'a, C, A> {}

impl<'a, C, A> InvitationDeleteCall<'a, C, A> where C: BorrowMut<hyper::Client>, A: oauth2::GetToken {


    /// Perform the operation you have build so far.
    pub fn doit(mut self) -> Result<(hyper::client::Response, Empty)> {
        use std::io::{Read, Seek};
        use hyper::header::{ContentType, ContentLength, Authorization, Bearer, UserAgent, Location};
        let mut dd = DefaultDelegate;
        let mut dlg: &mut Delegate = match self._delegate {
            Some(d) => d,
            None => &mut dd
        };
        dlg.begin(MethodInfo { id: "classroom.invitations.delete",
                               http_method: hyper::method::Method::Delete });
        let mut params: Vec<(&str, String)> = Vec::with_capacity((3 + self._additional_params.len()));
        params.push(("id", self._id.to_string()));
        for &field in ["alt", "id"].iter() {
            if self._additional_params.contains_key(field) {
                dlg.finished(false);
                return Err(Error::FieldClash(field));
            }
        }
        for (name, value) in self._additional_params.iter() {
            params.push((&name, value.clone()));
        }

        params.push(("alt", "json".to_string()));

        let mut url = self.hub._base_url.clone() + "v1/invitations/{id}";
        if self._scopes.len() == 0 {
            self._scopes.insert(Scope::Roster.as_ref().to_string(), ());
        }

        for &(find_this, param_name) in [("{id}", "id")].iter() {
            let mut replace_with: Option<&str> = None;
            for &(name, ref value) in params.iter() {
                if name == param_name {
                    replace_with = Some(value);
                    break;
                }
            }
            url = url.replace(find_this, replace_with.expect("to find substitution value in params"));
        }
        {
            let mut indices_for_removal: Vec<usize> = Vec::with_capacity(1);
            for param_name in ["id"].iter() {
                if let Some(index) = params.iter().position(|t| &t.0 == param_name) {
                    indices_for_removal.push(index);
                }
            }
            for &index in indices_for_removal.iter() {
                params.remove(index);
            }
        }

        if params.len() > 0 {
            url.push('?');
            url.push_str(&url::form_urlencoded::serialize(params));
        }



        loop {
            let token = match self.hub.auth.borrow_mut().token(self._scopes.keys()) {
                Ok(token) => token,
                Err(err) => {
                    match  dlg.token(&*err) {
                        Some(token) => token,
                        None => {
                            dlg.finished(false);
                            return Err(Error::MissingToken(err))
                        }
                    }
                }
            };
            let auth_header = Authorization(Bearer { token: token.access_token });
            let mut req_result = {
                let mut client = &mut *self.hub.client.borrow_mut();
                let mut req = client.borrow_mut().request(hyper::method::Method::Delete, &url)
                    .header(UserAgent(self.hub._user_agent.clone()))
                    .header(auth_header.clone());

                dlg.pre_request();
                req.send()
            };

            match req_result {
                Err(err) => {
                    if let oauth2::Retry::After(d) = dlg.http_error(&err) {
                        sleep(d);
                        continue;
                    }
                    dlg.finished(false);
                    return Err(Error::HttpError(err))
                }
                Ok(mut res) => {
                    if !res.status.is_success() {
                        let mut json_err = String::new();
                        res.read_to_string(&mut json_err).unwrap();
                        if let oauth2::Retry::After(d) = dlg.http_failure(&res,
                                                              json::from_str(&json_err).ok(),
                                                              json::from_str(&json_err).ok()) {
                            sleep(d);
                            continue;
                        }
                        dlg.finished(false);
                        return match json::from_str::<ErrorResponse>(&json_err){
                            Err(_) => Err(Error::Failure(res)),
                            Ok(serr) => Err(Error::BadRequest(serr))
                        }
                    }
                    let result_value = {
                        let mut json_response = String::new();
                        res.read_to_string(&mut json_response).unwrap();
                        match json::from_str(&json_response) {
                            Ok(decoded) => (res, decoded),
                            Err(err) => {
                                dlg.response_json_decode_error(&json_response, &err);
                                return Err(Error::JsonDecodeError(json_response, err));
                            }
                        }
                    };

                    dlg.finished(true);
                    return Ok(result_value)
                }
            }
        }
    }


    /// Identifier of the invitation to delete.
    ///
    /// Sets the *id* path property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call,
    /// we provide this method for API completeness.
    pub fn id(mut self, new_value: &str) -> InvitationDeleteCall<'a, C, A> {
        self._id = new_value.to_string();
        self
    }
    /// The delegate implementation is consulted whenever there is an intermediate result, or if something goes wrong
    /// while executing the actual API request.
    /// 
    /// It should be used to handle progress information, and to implement a certain level of resilience.
    ///
    /// Sets the *delegate* property to the given value.
    pub fn delegate(mut self, new_value: &'a mut Delegate) -> InvitationDeleteCall<'a, C, A> {
        self._delegate = Some(new_value);
        self
    }

    /// Set any additional parameter of the query string used in the request.
    /// It should be used to set parameters which are not yet available through their own
    /// setters.
    ///
    /// Please note that this method must not be used to set any of the known paramters
    /// which have their own setter method. If done anyway, the request will fail.
    ///
    /// # Additional Parameters
    ///
    /// * *upload_protocol* (query-string) - Upload protocol for media (e.g. "raw", "multipart").
    /// * *pp* (query-boolean) - Pretty-print response.
    /// * *prettyPrint* (query-boolean) - Returns response with indentations and line breaks.
    /// * *bearer_token* (query-string) - OAuth bearer token.
    /// * *access_token* (query-string) - OAuth access token.
    /// * *uploadType* (query-string) - Legacy upload protocol for media (e.g. "media", "multipart").
    /// * *quotaUser* (query-string) - Available to use for quota purposes for server-side applications. Can be any arbitrary string assigned to a user, but should not exceed 40 characters.
    /// * *callback* (query-string) - JSONP
    /// * *oauth_token* (query-string) - OAuth 2.0 token for the current user.
    /// * *key* (query-string) - API key. Your API key identifies your project and provides you with API access, quota, and reports. Required unless you provide an OAuth 2.0 token.
    /// * *fields* (query-string) - Selector specifying which fields to include in a partial response.
    /// * *alt* (query-string) - Data format for response.
    /// * *$.xgafv* (query-string) - V1 error format.
    pub fn param<T>(mut self, name: T, value: T) -> InvitationDeleteCall<'a, C, A>
                                                        where T: AsRef<str> {
        self._additional_params.insert(name.as_ref().to_string(), value.as_ref().to_string());
        self
    }

    /// Identifies the authorization scope for the method you are building.
    ///
    /// Use this method to actively specify which scope should be used, instead the default `Scope` variant
    /// `Scope::Roster`.
    ///
    /// The `scope` will be added to a set of scopes. This is important as one can maintain access
    /// tokens for more than one scope.
    /// If `None` is specified, then all scopes will be removed and no default scope will be used either.
    /// In that case, you have to specify your API-key using the `key` parameter (see the `param()`
    /// function for details).
    ///
    /// Usually there is more than one suitable scope to authorize an operation, some of which may
    /// encompass more rights than others. For example, for listing resources, a *read-only* scope will be
    /// sufficient, a read-write scope will do as well.
    pub fn add_scope<T, S>(mut self, scope: T) -> InvitationDeleteCall<'a, C, A>
                                                        where T: Into<Option<S>>,
                                                              S: AsRef<str> {
        match scope.into() {
          Some(scope) => self._scopes.insert(scope.as_ref().to_string(), ()),
          None => None,
        };
        self
    }
}


/// Creates an invitation. Only one invitation for a user and course may exist
/// at a time. Delete and re-create an invitation to make changes.
/// 
/// This method returns the following error codes:
/// 
/// * `PERMISSION_DENIED` if the requesting user is not permitted to create
/// invitations for this course or for access errors.
/// * `NOT_FOUND` if the course or the user does not exist.
/// * `FAILED_PRECONDITION` if the requested user's account is disabled or if
/// the user already has this role or a role with greater permissions.
/// * `ALREADY_EXISTS` if an invitation for the specified user and course
/// already exists.
///
/// A builder for the *create* method supported by a *invitation* resource.
/// It is not used directly, but through a `InvitationMethods` instance.
///
/// # Example
///
/// Instantiate a resource method builder
///
/// ```test_harness,no_run
/// # extern crate hyper;
/// # extern crate hyper_rustls;
/// # extern crate yup_oauth2 as oauth2;
/// # extern crate google_classroom1 as classroom1;
/// use classroom1::Invitation;
/// # #[test] fn egal() {
/// # use std::default::Default;
/// # use oauth2::{Authenticator, DefaultAuthenticatorDelegate, ApplicationSecret, MemoryStorage};
/// # use classroom1::Classroom;
/// 
/// # let secret: ApplicationSecret = Default::default();
/// # let auth = Authenticator::new(&secret, DefaultAuthenticatorDelegate,
/// #                               hyper::Client::with_connector(hyper::net::HttpsConnector::new(hyper_rustls::TlsClient::new())),
/// #                               <MemoryStorage as Default>::default(), None);
/// # let mut hub = Classroom::new(hyper::Client::with_connector(hyper::net::HttpsConnector::new(hyper_rustls::TlsClient::new())), auth);
/// // As the method needs a request, you would usually fill it with the desired information
/// // into the respective structure. Some of the parts shown here might not be applicable !
/// // Values shown here are possibly random and not representative !
/// let mut req = Invitation::default();
/// 
/// // You can configure optional parameters by calling the respective setters at will, and
/// // execute the final call using `doit()`.
/// // Values shown here are possibly random and not representative !
/// let result = hub.invitations().create(req)
///              .doit();
/// # }
/// ```
pub struct InvitationCreateCall<'a, C, A>
    where C: 'a, A: 'a {

    hub: &'a Classroom<C, A>,
    _request: Invitation,
    _delegate: Option<&'a mut Delegate>,
    _additional_params: HashMap<String, String>,
    _scopes: BTreeMap<String, ()>
}

impl<'a, C, A> CallBuilder for InvitationCreateCall<'a, C, A> {}

impl<'a, C, A> InvitationCreateCall<'a, C, A> where C: BorrowMut<hyper::Client>, A: oauth2::GetToken {


    /// Perform the operation you have build so far.
    pub fn doit(mut self) -> Result<(hyper::client::Response, Invitation)> {
        use std::io::{Read, Seek};
        use hyper::header::{ContentType, ContentLength, Authorization, Bearer, UserAgent, Location};
        let mut dd = DefaultDelegate;
        let mut dlg: &mut Delegate = match self._delegate {
            Some(d) => d,
            None => &mut dd
        };
        dlg.begin(MethodInfo { id: "classroom.invitations.create",
                               http_method: hyper::method::Method::Post });
        let mut params: Vec<(&str, String)> = Vec::with_capacity((3 + self._additional_params.len()));
        for &field in ["alt"].iter() {
            if self._additional_params.contains_key(field) {
                dlg.finished(false);
                return Err(Error::FieldClash(field));
            }
        }
        for (name, value) in self._additional_params.iter() {
            params.push((&name, value.clone()));
        }

        params.push(("alt", "json".to_string()));

        let mut url = self.hub._base_url.clone() + "v1/invitations";
        if self._scopes.len() == 0 {
            self._scopes.insert(Scope::Roster.as_ref().to_string(), ());
        }


        if params.len() > 0 {
            url.push('?');
            url.push_str(&url::form_urlencoded::serialize(params));
        }

        let mut json_mime_type = mime::Mime(mime::TopLevel::Application, mime::SubLevel::Json, Default::default());
        let mut request_value_reader =
            {
                let mut value = json::value::to_value(&self._request).expect("serde to work");
                remove_json_null_values(&mut value);
                let mut dst = io::Cursor::new(Vec::with_capacity(128));
                json::to_writer(&mut dst, &value).unwrap();
                dst
            };
        let request_size = request_value_reader.seek(io::SeekFrom::End(0)).unwrap();
        request_value_reader.seek(io::SeekFrom::Start(0)).unwrap();


        loop {
            let token = match self.hub.auth.borrow_mut().token(self._scopes.keys()) {
                Ok(token) => token,
                Err(err) => {
                    match  dlg.token(&*err) {
                        Some(token) => token,
                        None => {
                            dlg.finished(false);
                            return Err(Error::MissingToken(err))
                        }
                    }
                }
            };
            let auth_header = Authorization(Bearer { token: token.access_token });
            request_value_reader.seek(io::SeekFrom::Start(0)).unwrap();
            let mut req_result = {
                let mut client = &mut *self.hub.client.borrow_mut();
                let mut req = client.borrow_mut().request(hyper::method::Method::Post, &url)
                    .header(UserAgent(self.hub._user_agent.clone()))
                    .header(auth_header.clone())
                    .header(ContentType(json_mime_type.clone()))
                    .header(ContentLength(request_size as u64))
                    .body(&mut request_value_reader);

                dlg.pre_request();
                req.send()
            };

            match req_result {
                Err(err) => {
                    if let oauth2::Retry::After(d) = dlg.http_error(&err) {
                        sleep(d);
                        continue;
                    }
                    dlg.finished(false);
                    return Err(Error::HttpError(err))
                }
                Ok(mut res) => {
                    if !res.status.is_success() {
                        let mut json_err = String::new();
                        res.read_to_string(&mut json_err).unwrap();
                        if let oauth2::Retry::After(d) = dlg.http_failure(&res,
                                                              json::from_str(&json_err).ok(),
                                                              json::from_str(&json_err).ok()) {
                            sleep(d);
                            continue;
                        }
                        dlg.finished(false);
                        return match json::from_str::<ErrorResponse>(&json_err){
                            Err(_) => Err(Error::Failure(res)),
                            Ok(serr) => Err(Error::BadRequest(serr))
                        }
                    }
                    let result_value = {
                        let mut json_response = String::new();
                        res.read_to_string(&mut json_response).unwrap();
                        match json::from_str(&json_response) {
                            Ok(decoded) => (res, decoded),
                            Err(err) => {
                                dlg.response_json_decode_error(&json_response, &err);
                                return Err(Error::JsonDecodeError(json_response, err));
                            }
                        }
                    };

                    dlg.finished(true);
                    return Ok(result_value)
                }
            }
        }
    }


    ///
    /// Sets the *request* property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call,
    /// we provide this method for API completeness.
    pub fn request(mut self, new_value: Invitation) -> InvitationCreateCall<'a, C, A> {
        self._request = new_value;
        self
    }
    /// The delegate implementation is consulted whenever there is an intermediate result, or if something goes wrong
    /// while executing the actual API request.
    /// 
    /// It should be used to handle progress information, and to implement a certain level of resilience.
    ///
    /// Sets the *delegate* property to the given value.
    pub fn delegate(mut self, new_value: &'a mut Delegate) -> InvitationCreateCall<'a, C, A> {
        self._delegate = Some(new_value);
        self
    }

    /// Set any additional parameter of the query string used in the request.
    /// It should be used to set parameters which are not yet available through their own
    /// setters.
    ///
    /// Please note that this method must not be used to set any of the known paramters
    /// which have their own setter method. If done anyway, the request will fail.
    ///
    /// # Additional Parameters
    ///
    /// * *upload_protocol* (query-string) - Upload protocol for media (e.g. "raw", "multipart").
    /// * *pp* (query-boolean) - Pretty-print response.
    /// * *prettyPrint* (query-boolean) - Returns response with indentations and line breaks.
    /// * *bearer_token* (query-string) - OAuth bearer token.
    /// * *access_token* (query-string) - OAuth access token.
    /// * *uploadType* (query-string) - Legacy upload protocol for media (e.g. "media", "multipart").
    /// * *quotaUser* (query-string) - Available to use for quota purposes for server-side applications. Can be any arbitrary string assigned to a user, but should not exceed 40 characters.
    /// * *callback* (query-string) - JSONP
    /// * *oauth_token* (query-string) - OAuth 2.0 token for the current user.
    /// * *key* (query-string) - API key. Your API key identifies your project and provides you with API access, quota, and reports. Required unless you provide an OAuth 2.0 token.
    /// * *fields* (query-string) - Selector specifying which fields to include in a partial response.
    /// * *alt* (query-string) - Data format for response.
    /// * *$.xgafv* (query-string) - V1 error format.
    pub fn param<T>(mut self, name: T, value: T) -> InvitationCreateCall<'a, C, A>
                                                        where T: AsRef<str> {
        self._additional_params.insert(name.as_ref().to_string(), value.as_ref().to_string());
        self
    }

    /// Identifies the authorization scope for the method you are building.
    ///
    /// Use this method to actively specify which scope should be used, instead the default `Scope` variant
    /// `Scope::Roster`.
    ///
    /// The `scope` will be added to a set of scopes. This is important as one can maintain access
    /// tokens for more than one scope.
    /// If `None` is specified, then all scopes will be removed and no default scope will be used either.
    /// In that case, you have to specify your API-key using the `key` parameter (see the `param()`
    /// function for details).
    ///
    /// Usually there is more than one suitable scope to authorize an operation, some of which may
    /// encompass more rights than others. For example, for listing resources, a *read-only* scope will be
    /// sufficient, a read-write scope will do as well.
    pub fn add_scope<T, S>(mut self, scope: T) -> InvitationCreateCall<'a, C, A>
                                                        where T: Into<Option<S>>,
                                                              S: AsRef<str> {
        match scope.into() {
          Some(scope) => self._scopes.insert(scope.as_ref().to_string(), ()),
          None => None,
        };
        self
    }
}


/// Returns a list of invitations that the requesting user is permitted to
/// view, restricted to those that match the list request.
/// 
/// *Note:* At least one of `user_id` or `course_id` must be supplied. Both
/// fields can be supplied.
/// 
/// This method returns the following error codes:
/// 
/// * `PERMISSION_DENIED` for access errors.
///
/// A builder for the *list* method supported by a *invitation* resource.
/// It is not used directly, but through a `InvitationMethods` instance.
///
/// # Example
///
/// Instantiate a resource method builder
///
/// ```test_harness,no_run
/// # extern crate hyper;
/// # extern crate hyper_rustls;
/// # extern crate yup_oauth2 as oauth2;
/// # extern crate google_classroom1 as classroom1;
/// # #[test] fn egal() {
/// # use std::default::Default;
/// # use oauth2::{Authenticator, DefaultAuthenticatorDelegate, ApplicationSecret, MemoryStorage};
/// # use classroom1::Classroom;
/// 
/// # let secret: ApplicationSecret = Default::default();
/// # let auth = Authenticator::new(&secret, DefaultAuthenticatorDelegate,
/// #                               hyper::Client::with_connector(hyper::net::HttpsConnector::new(hyper_rustls::TlsClient::new())),
/// #                               <MemoryStorage as Default>::default(), None);
/// # let mut hub = Classroom::new(hyper::Client::with_connector(hyper::net::HttpsConnector::new(hyper_rustls::TlsClient::new())), auth);
/// // You can configure optional parameters by calling the respective setters at will, and
/// // execute the final call using `doit()`.
/// // Values shown here are possibly random and not representative !
/// let result = hub.invitations().list()
///              .user_id("labore")
///              .page_token("ipsum")
///              .page_size(-31)
///              .course_id("dolores")
///              .doit();
/// # }
/// ```
pub struct InvitationListCall<'a, C, A>
    where C: 'a, A: 'a {

    hub: &'a Classroom<C, A>,
    _user_id: Option<String>,
    _page_token: Option<String>,
    _page_size: Option<i32>,
    _course_id: Option<String>,
    _delegate: Option<&'a mut Delegate>,
    _additional_params: HashMap<String, String>,
    _scopes: BTreeMap<String, ()>
}

impl<'a, C, A> CallBuilder for InvitationListCall<'a, C, A> {}

impl<'a, C, A> InvitationListCall<'a, C, A> where C: BorrowMut<hyper::Client>, A: oauth2::GetToken {


    /// Perform the operation you have build so far.
    pub fn doit(mut self) -> Result<(hyper::client::Response, ListInvitationsResponse)> {
        use std::io::{Read, Seek};
        use hyper::header::{ContentType, ContentLength, Authorization, Bearer, UserAgent, Location};
        let mut dd = DefaultDelegate;
        let mut dlg: &mut Delegate = match self._delegate {
            Some(d) => d,
            None => &mut dd
        };
        dlg.begin(MethodInfo { id: "classroom.invitations.list",
                               http_method: hyper::method::Method::Get });
        let mut params: Vec<(&str, String)> = Vec::with_capacity((6 + self._additional_params.len()));
        if let Some(value) = self._user_id {
            params.push(("userId", value.to_string()));
        }
        if let Some(value) = self._page_token {
            params.push(("pageToken", value.to_string()));
        }
        if let Some(value) = self._page_size {
            params.push(("pageSize", value.to_string()));
        }
        if let Some(value) = self._course_id {
            params.push(("courseId", value.to_string()));
        }
        for &field in ["alt", "userId", "pageToken", "pageSize", "courseId"].iter() {
            if self._additional_params.contains_key(field) {
                dlg.finished(false);
                return Err(Error::FieldClash(field));
            }
        }
        for (name, value) in self._additional_params.iter() {
            params.push((&name, value.clone()));
        }

        params.push(("alt", "json".to_string()));

        let mut url = self.hub._base_url.clone() + "v1/invitations";
        if self._scopes.len() == 0 {
            self._scopes.insert(Scope::RosterReadonly.as_ref().to_string(), ());
        }


        if params.len() > 0 {
            url.push('?');
            url.push_str(&url::form_urlencoded::serialize(params));
        }



        loop {
            let token = match self.hub.auth.borrow_mut().token(self._scopes.keys()) {
                Ok(token) => token,
                Err(err) => {
                    match  dlg.token(&*err) {
                        Some(token) => token,
                        None => {
                            dlg.finished(false);
                            return Err(Error::MissingToken(err))
                        }
                    }
                }
            };
            let auth_header = Authorization(Bearer { token: token.access_token });
            let mut req_result = {
                let mut client = &mut *self.hub.client.borrow_mut();
                let mut req = client.borrow_mut().request(hyper::method::Method::Get, &url)
                    .header(UserAgent(self.hub._user_agent.clone()))
                    .header(auth_header.clone());

                dlg.pre_request();
                req.send()
            };

            match req_result {
                Err(err) => {
                    if let oauth2::Retry::After(d) = dlg.http_error(&err) {
                        sleep(d);
                        continue;
                    }
                    dlg.finished(false);
                    return Err(Error::HttpError(err))
                }
                Ok(mut res) => {
                    if !res.status.is_success() {
                        let mut json_err = String::new();
                        res.read_to_string(&mut json_err).unwrap();
                        if let oauth2::Retry::After(d) = dlg.http_failure(&res,
                                                              json::from_str(&json_err).ok(),
                                                              json::from_str(&json_err).ok()) {
                            sleep(d);
                            continue;
                        }
                        dlg.finished(false);
                        return match json::from_str::<ErrorResponse>(&json_err){
                            Err(_) => Err(Error::Failure(res)),
                            Ok(serr) => Err(Error::BadRequest(serr))
                        }
                    }
                    let result_value = {
                        let mut json_response = String::new();
                        res.read_to_string(&mut json_response).unwrap();
                        match json::from_str(&json_response) {
                            Ok(decoded) => (res, decoded),
                            Err(err) => {
                                dlg.response_json_decode_error(&json_response, &err);
                                return Err(Error::JsonDecodeError(json_response, err));
                            }
                        }
                    };

                    dlg.finished(true);
                    return Ok(result_value)
                }
            }
        }
    }


    /// Restricts returned invitations to those for a specific user. The identifier
    /// can be one of the following:
    /// 
    /// * the numeric identifier for the user
    /// * the email address of the user
    /// * the string literal `"me"`, indicating the requesting user
    ///
    /// Sets the *user id* query property to the given value.
    pub fn user_id(mut self, new_value: &str) -> InvitationListCall<'a, C, A> {
        self._user_id = Some(new_value.to_string());
        self
    }
    /// nextPageToken
    /// value returned from a previous
    /// list call, indicating
    /// that the subsequent page of results should be returned.
    /// 
    /// The list request must be
    /// otherwise identical to the one that resulted in this token.
    ///
    /// Sets the *page token* query property to the given value.
    pub fn page_token(mut self, new_value: &str) -> InvitationListCall<'a, C, A> {
        self._page_token = Some(new_value.to_string());
        self
    }
    /// Maximum number of items to return. Zero means no maximum.
    /// 
    /// The server may return fewer than the specified number of results.
    ///
    /// Sets the *page size* query property to the given value.
    pub fn page_size(mut self, new_value: i32) -> InvitationListCall<'a, C, A> {
        self._page_size = Some(new_value);
        self
    }
    /// Restricts returned invitations to those for a course with the specified
    /// identifier.
    ///
    /// Sets the *course id* query property to the given value.
    pub fn course_id(mut self, new_value: &str) -> InvitationListCall<'a, C, A> {
        self._course_id = Some(new_value.to_string());
        self
    }
    /// The delegate implementation is consulted whenever there is an intermediate result, or if something goes wrong
    /// while executing the actual API request.
    /// 
    /// It should be used to handle progress information, and to implement a certain level of resilience.
    ///
    /// Sets the *delegate* property to the given value.
    pub fn delegate(mut self, new_value: &'a mut Delegate) -> InvitationListCall<'a, C, A> {
        self._delegate = Some(new_value);
        self
    }

    /// Set any additional parameter of the query string used in the request.
    /// It should be used to set parameters which are not yet available through their own
    /// setters.
    ///
    /// Please note that this method must not be used to set any of the known paramters
    /// which have their own setter method. If done anyway, the request will fail.
    ///
    /// # Additional Parameters
    ///
    /// * *upload_protocol* (query-string) - Upload protocol for media (e.g. "raw", "multipart").
    /// * *pp* (query-boolean) - Pretty-print response.
    /// * *prettyPrint* (query-boolean) - Returns response with indentations and line breaks.
    /// * *bearer_token* (query-string) - OAuth bearer token.
    /// * *access_token* (query-string) - OAuth access token.
    /// * *uploadType* (query-string) - Legacy upload protocol for media (e.g. "media", "multipart").
    /// * *quotaUser* (query-string) - Available to use for quota purposes for server-side applications. Can be any arbitrary string assigned to a user, but should not exceed 40 characters.
    /// * *callback* (query-string) - JSONP
    /// * *oauth_token* (query-string) - OAuth 2.0 token for the current user.
    /// * *key* (query-string) - API key. Your API key identifies your project and provides you with API access, quota, and reports. Required unless you provide an OAuth 2.0 token.
    /// * *fields* (query-string) - Selector specifying which fields to include in a partial response.
    /// * *alt* (query-string) - Data format for response.
    /// * *$.xgafv* (query-string) - V1 error format.
    pub fn param<T>(mut self, name: T, value: T) -> InvitationListCall<'a, C, A>
                                                        where T: AsRef<str> {
        self._additional_params.insert(name.as_ref().to_string(), value.as_ref().to_string());
        self
    }

    /// Identifies the authorization scope for the method you are building.
    ///
    /// Use this method to actively specify which scope should be used, instead the default `Scope` variant
    /// `Scope::RosterReadonly`.
    ///
    /// The `scope` will be added to a set of scopes. This is important as one can maintain access
    /// tokens for more than one scope.
    /// If `None` is specified, then all scopes will be removed and no default scope will be used either.
    /// In that case, you have to specify your API-key using the `key` parameter (see the `param()`
    /// function for details).
    ///
    /// Usually there is more than one suitable scope to authorize an operation, some of which may
    /// encompass more rights than others. For example, for listing resources, a *read-only* scope will be
    /// sufficient, a read-write scope will do as well.
    pub fn add_scope<T, S>(mut self, scope: T) -> InvitationListCall<'a, C, A>
                                                        where T: Into<Option<S>>,
                                                              S: AsRef<str> {
        match scope.into() {
          Some(scope) => self._scopes.insert(scope.as_ref().to_string(), ()),
          None => None,
        };
        self
    }
}


/// Returns an invitation.
/// 
/// This method returns the following error codes:
/// 
/// * `PERMISSION_DENIED` if the requesting user is not permitted to view the
/// requested invitation or for access errors.
/// * `NOT_FOUND` if no invitation exists with the requested ID.
///
/// A builder for the *get* method supported by a *invitation* resource.
/// It is not used directly, but through a `InvitationMethods` instance.
///
/// # Example
///
/// Instantiate a resource method builder
///
/// ```test_harness,no_run
/// # extern crate hyper;
/// # extern crate hyper_rustls;
/// # extern crate yup_oauth2 as oauth2;
/// # extern crate google_classroom1 as classroom1;
/// # #[test] fn egal() {
/// # use std::default::Default;
/// # use oauth2::{Authenticator, DefaultAuthenticatorDelegate, ApplicationSecret, MemoryStorage};
/// # use classroom1::Classroom;
/// 
/// # let secret: ApplicationSecret = Default::default();
/// # let auth = Authenticator::new(&secret, DefaultAuthenticatorDelegate,
/// #                               hyper::Client::with_connector(hyper::net::HttpsConnector::new(hyper_rustls::TlsClient::new())),
/// #                               <MemoryStorage as Default>::default(), None);
/// # let mut hub = Classroom::new(hyper::Client::with_connector(hyper::net::HttpsConnector::new(hyper_rustls::TlsClient::new())), auth);
/// // You can configure optional parameters by calling the respective setters at will, and
/// // execute the final call using `doit()`.
/// // Values shown here are possibly random and not representative !
/// let result = hub.invitations().get("id")
///              .doit();
/// # }
/// ```
pub struct InvitationGetCall<'a, C, A>
    where C: 'a, A: 'a {

    hub: &'a Classroom<C, A>,
    _id: String,
    _delegate: Option<&'a mut Delegate>,
    _additional_params: HashMap<String, String>,
    _scopes: BTreeMap<String, ()>
}

impl<'a, C, A> CallBuilder for InvitationGetCall<'a, C, A> {}

impl<'a, C, A> InvitationGetCall<'a, C, A> where C: BorrowMut<hyper::Client>, A: oauth2::GetToken {


    /// Perform the operation you have build so far.
    pub fn doit(mut self) -> Result<(hyper::client::Response, Invitation)> {
        use std::io::{Read, Seek};
        use hyper::header::{ContentType, ContentLength, Authorization, Bearer, UserAgent, Location};
        let mut dd = DefaultDelegate;
        let mut dlg: &mut Delegate = match self._delegate {
            Some(d) => d,
            None => &mut dd
        };
        dlg.begin(MethodInfo { id: "classroom.invitations.get",
                               http_method: hyper::method::Method::Get });
        let mut params: Vec<(&str, String)> = Vec::with_capacity((3 + self._additional_params.len()));
        params.push(("id", self._id.to_string()));
        for &field in ["alt", "id"].iter() {
            if self._additional_params.contains_key(field) {
                dlg.finished(false);
                return Err(Error::FieldClash(field));
            }
        }
        for (name, value) in self._additional_params.iter() {
            params.push((&name, value.clone()));
        }

        params.push(("alt", "json".to_string()));

        let mut url = self.hub._base_url.clone() + "v1/invitations/{id}";
        if self._scopes.len() == 0 {
            self._scopes.insert(Scope::RosterReadonly.as_ref().to_string(), ());
        }

        for &(find_this, param_name) in [("{id}", "id")].iter() {
            let mut replace_with: Option<&str> = None;
            for &(name, ref value) in params.iter() {
                if name == param_name {
                    replace_with = Some(value);
                    break;
                }
            }
            url = url.replace(find_this, replace_with.expect("to find substitution value in params"));
        }
        {
            let mut indices_for_removal: Vec<usize> = Vec::with_capacity(1);
            for param_name in ["id"].iter() {
                if let Some(index) = params.iter().position(|t| &t.0 == param_name) {
                    indices_for_removal.push(index);
                }
            }
            for &index in indices_for_removal.iter() {
                params.remove(index);
            }
        }

        if params.len() > 0 {
            url.push('?');
            url.push_str(&url::form_urlencoded::serialize(params));
        }



        loop {
            let token = match self.hub.auth.borrow_mut().token(self._scopes.keys()) {
                Ok(token) => token,
                Err(err) => {
                    match  dlg.token(&*err) {
                        Some(token) => token,
                        None => {
                            dlg.finished(false);
                            return Err(Error::MissingToken(err))
                        }
                    }
                }
            };
            let auth_header = Authorization(Bearer { token: token.access_token });
            let mut req_result = {
                let mut client = &mut *self.hub.client.borrow_mut();
                let mut req = client.borrow_mut().request(hyper::method::Method::Get, &url)
                    .header(UserAgent(self.hub._user_agent.clone()))
                    .header(auth_header.clone());

                dlg.pre_request();
                req.send()
            };

            match req_result {
                Err(err) => {
                    if let oauth2::Retry::After(d) = dlg.http_error(&err) {
                        sleep(d);
                        continue;
                    }
                    dlg.finished(false);
                    return Err(Error::HttpError(err))
                }
                Ok(mut res) => {
                    if !res.status.is_success() {
                        let mut json_err = String::new();
                        res.read_to_string(&mut json_err).unwrap();
                        if let oauth2::Retry::After(d) = dlg.http_failure(&res,
                                                              json::from_str(&json_err).ok(),
                                                              json::from_str(&json_err).ok()) {
                            sleep(d);
                            continue;
                        }
                        dlg.finished(false);
                        return match json::from_str::<ErrorResponse>(&json_err){
                            Err(_) => Err(Error::Failure(res)),
                            Ok(serr) => Err(Error::BadRequest(serr))
                        }
                    }
                    let result_value = {
                        let mut json_response = String::new();
                        res.read_to_string(&mut json_response).unwrap();
                        match json::from_str(&json_response) {
                            Ok(decoded) => (res, decoded),
                            Err(err) => {
                                dlg.response_json_decode_error(&json_response, &err);
                                return Err(Error::JsonDecodeError(json_response, err));
                            }
                        }
                    };

                    dlg.finished(true);
                    return Ok(result_value)
                }
            }
        }
    }


    /// Identifier of the invitation to return.
    ///
    /// Sets the *id* path property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call,
    /// we provide this method for API completeness.
    pub fn id(mut self, new_value: &str) -> InvitationGetCall<'a, C, A> {
        self._id = new_value.to_string();
        self
    }
    /// The delegate implementation is consulted whenever there is an intermediate result, or if something goes wrong
    /// while executing the actual API request.
    /// 
    /// It should be used to handle progress information, and to implement a certain level of resilience.
    ///
    /// Sets the *delegate* property to the given value.
    pub fn delegate(mut self, new_value: &'a mut Delegate) -> InvitationGetCall<'a, C, A> {
        self._delegate = Some(new_value);
        self
    }

    /// Set any additional parameter of the query string used in the request.
    /// It should be used to set parameters which are not yet available through their own
    /// setters.
    ///
    /// Please note that this method must not be used to set any of the known paramters
    /// which have their own setter method. If done anyway, the request will fail.
    ///
    /// # Additional Parameters
    ///
    /// * *upload_protocol* (query-string) - Upload protocol for media (e.g. "raw", "multipart").
    /// * *pp* (query-boolean) - Pretty-print response.
    /// * *prettyPrint* (query-boolean) - Returns response with indentations and line breaks.
    /// * *bearer_token* (query-string) - OAuth bearer token.
    /// * *access_token* (query-string) - OAuth access token.
    /// * *uploadType* (query-string) - Legacy upload protocol for media (e.g. "media", "multipart").
    /// * *quotaUser* (query-string) - Available to use for quota purposes for server-side applications. Can be any arbitrary string assigned to a user, but should not exceed 40 characters.
    /// * *callback* (query-string) - JSONP
    /// * *oauth_token* (query-string) - OAuth 2.0 token for the current user.
    /// * *key* (query-string) - API key. Your API key identifies your project and provides you with API access, quota, and reports. Required unless you provide an OAuth 2.0 token.
    /// * *fields* (query-string) - Selector specifying which fields to include in a partial response.
    /// * *alt* (query-string) - Data format for response.
    /// * *$.xgafv* (query-string) - V1 error format.
    pub fn param<T>(mut self, name: T, value: T) -> InvitationGetCall<'a, C, A>
                                                        where T: AsRef<str> {
        self._additional_params.insert(name.as_ref().to_string(), value.as_ref().to_string());
        self
    }

    /// Identifies the authorization scope for the method you are building.
    ///
    /// Use this method to actively specify which scope should be used, instead the default `Scope` variant
    /// `Scope::RosterReadonly`.
    ///
    /// The `scope` will be added to a set of scopes. This is important as one can maintain access
    /// tokens for more than one scope.
    /// If `None` is specified, then all scopes will be removed and no default scope will be used either.
    /// In that case, you have to specify your API-key using the `key` parameter (see the `param()`
    /// function for details).
    ///
    /// Usually there is more than one suitable scope to authorize an operation, some of which may
    /// encompass more rights than others. For example, for listing resources, a *read-only* scope will be
    /// sufficient, a read-write scope will do as well.
    pub fn add_scope<T, S>(mut self, scope: T) -> InvitationGetCall<'a, C, A>
                                                        where T: Into<Option<S>>,
                                                              S: AsRef<str> {
        match scope.into() {
          Some(scope) => self._scopes.insert(scope.as_ref().to_string(), ()),
          None => None,
        };
        self
    }
}


/// Accepts an invitation, removing it and adding the invited user to the
/// teachers or students (as appropriate) of the specified course. Only the
/// invited user may accept an invitation.
/// 
/// This method returns the following error codes:
/// 
/// * `PERMISSION_DENIED` if the requesting user is not permitted to accept the
/// requested invitation or for access errors.
/// * `FAILED_PRECONDITION` for the following request errors:
///     * CourseMemberLimitReached
///     * CourseNotModifiable
///     * CourseTeacherLimitReached
///     * UserGroupsMembershipLimitReached
/// * `NOT_FOUND` if no invitation exists with the requested ID.
///
/// A builder for the *accept* method supported by a *invitation* resource.
/// It is not used directly, but through a `InvitationMethods` instance.
///
/// # Example
///
/// Instantiate a resource method builder
///
/// ```test_harness,no_run
/// # extern crate hyper;
/// # extern crate hyper_rustls;
/// # extern crate yup_oauth2 as oauth2;
/// # extern crate google_classroom1 as classroom1;
/// # #[test] fn egal() {
/// # use std::default::Default;
/// # use oauth2::{Authenticator, DefaultAuthenticatorDelegate, ApplicationSecret, MemoryStorage};
/// # use classroom1::Classroom;
/// 
/// # let secret: ApplicationSecret = Default::default();
/// # let auth = Authenticator::new(&secret, DefaultAuthenticatorDelegate,
/// #                               hyper::Client::with_connector(hyper::net::HttpsConnector::new(hyper_rustls::TlsClient::new())),
/// #                               <MemoryStorage as Default>::default(), None);
/// # let mut hub = Classroom::new(hyper::Client::with_connector(hyper::net::HttpsConnector::new(hyper_rustls::TlsClient::new())), auth);
/// // You can configure optional parameters by calling the respective setters at will, and
/// // execute the final call using `doit()`.
/// // Values shown here are possibly random and not representative !
/// let result = hub.invitations().accept("id")
///              .doit();
/// # }
/// ```
pub struct InvitationAcceptCall<'a, C, A>
    where C: 'a, A: 'a {

    hub: &'a Classroom<C, A>,
    _id: String,
    _delegate: Option<&'a mut Delegate>,
    _additional_params: HashMap<String, String>,
    _scopes: BTreeMap<String, ()>
}

impl<'a, C, A> CallBuilder for InvitationAcceptCall<'a, C, A> {}

impl<'a, C, A> InvitationAcceptCall<'a, C, A> where C: BorrowMut<hyper::Client>, A: oauth2::GetToken {


    /// Perform the operation you have build so far.
    pub fn doit(mut self) -> Result<(hyper::client::Response, Empty)> {
        use std::io::{Read, Seek};
        use hyper::header::{ContentType, ContentLength, Authorization, Bearer, UserAgent, Location};
        let mut dd = DefaultDelegate;
        let mut dlg: &mut Delegate = match self._delegate {
            Some(d) => d,
            None => &mut dd
        };
        dlg.begin(MethodInfo { id: "classroom.invitations.accept",
                               http_method: hyper::method::Method::Post });
        let mut params: Vec<(&str, String)> = Vec::with_capacity((3 + self._additional_params.len()));
        params.push(("id", self._id.to_string()));
        for &field in ["alt", "id"].iter() {
            if self._additional_params.contains_key(field) {
                dlg.finished(false);
                return Err(Error::FieldClash(field));
            }
        }
        for (name, value) in self._additional_params.iter() {
            params.push((&name, value.clone()));
        }

        params.push(("alt", "json".to_string()));

        let mut url = self.hub._base_url.clone() + "v1/invitations/{id}:accept";
        if self._scopes.len() == 0 {
            self._scopes.insert(Scope::Roster.as_ref().to_string(), ());
        }

        for &(find_this, param_name) in [("{id}", "id")].iter() {
            let mut replace_with: Option<&str> = None;
            for &(name, ref value) in params.iter() {
                if name == param_name {
                    replace_with = Some(value);
                    break;
                }
            }
            url = url.replace(find_this, replace_with.expect("to find substitution value in params"));
        }
        {
            let mut indices_for_removal: Vec<usize> = Vec::with_capacity(1);
            for param_name in ["id"].iter() {
                if let Some(index) = params.iter().position(|t| &t.0 == param_name) {
                    indices_for_removal.push(index);
                }
            }
            for &index in indices_for_removal.iter() {
                params.remove(index);
            }
        }

        if params.len() > 0 {
            url.push('?');
            url.push_str(&url::form_urlencoded::serialize(params));
        }



        loop {
            let token = match self.hub.auth.borrow_mut().token(self._scopes.keys()) {
                Ok(token) => token,
                Err(err) => {
                    match  dlg.token(&*err) {
                        Some(token) => token,
                        None => {
                            dlg.finished(false);
                            return Err(Error::MissingToken(err))
                        }
                    }
                }
            };
            let auth_header = Authorization(Bearer { token: token.access_token });
            let mut req_result = {
                let mut client = &mut *self.hub.client.borrow_mut();
                let mut req = client.borrow_mut().request(hyper::method::Method::Post, &url)
                    .header(UserAgent(self.hub._user_agent.clone()))
                    .header(auth_header.clone());

                dlg.pre_request();
                req.send()
            };

            match req_result {
                Err(err) => {
                    if let oauth2::Retry::After(d) = dlg.http_error(&err) {
                        sleep(d);
                        continue;
                    }
                    dlg.finished(false);
                    return Err(Error::HttpError(err))
                }
                Ok(mut res) => {
                    if !res.status.is_success() {
                        let mut json_err = String::new();
                        res.read_to_string(&mut json_err).unwrap();
                        if let oauth2::Retry::After(d) = dlg.http_failure(&res,
                                                              json::from_str(&json_err).ok(),
                                                              json::from_str(&json_err).ok()) {
                            sleep(d);
                            continue;
                        }
                        dlg.finished(false);
                        return match json::from_str::<ErrorResponse>(&json_err){
                            Err(_) => Err(Error::Failure(res)),
                            Ok(serr) => Err(Error::BadRequest(serr))
                        }
                    }
                    let result_value = {
                        let mut json_response = String::new();
                        res.read_to_string(&mut json_response).unwrap();
                        match json::from_str(&json_response) {
                            Ok(decoded) => (res, decoded),
                            Err(err) => {
                                dlg.response_json_decode_error(&json_response, &err);
                                return Err(Error::JsonDecodeError(json_response, err));
                            }
                        }
                    };

                    dlg.finished(true);
                    return Ok(result_value)
                }
            }
        }
    }


    /// Identifier of the invitation to accept.
    ///
    /// Sets the *id* path property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call,
    /// we provide this method for API completeness.
    pub fn id(mut self, new_value: &str) -> InvitationAcceptCall<'a, C, A> {
        self._id = new_value.to_string();
        self
    }
    /// The delegate implementation is consulted whenever there is an intermediate result, or if something goes wrong
    /// while executing the actual API request.
    /// 
    /// It should be used to handle progress information, and to implement a certain level of resilience.
    ///
    /// Sets the *delegate* property to the given value.
    pub fn delegate(mut self, new_value: &'a mut Delegate) -> InvitationAcceptCall<'a, C, A> {
        self._delegate = Some(new_value);
        self
    }

    /// Set any additional parameter of the query string used in the request.
    /// It should be used to set parameters which are not yet available through their own
    /// setters.
    ///
    /// Please note that this method must not be used to set any of the known paramters
    /// which have their own setter method. If done anyway, the request will fail.
    ///
    /// # Additional Parameters
    ///
    /// * *upload_protocol* (query-string) - Upload protocol for media (e.g. "raw", "multipart").
    /// * *pp* (query-boolean) - Pretty-print response.
    /// * *prettyPrint* (query-boolean) - Returns response with indentations and line breaks.
    /// * *bearer_token* (query-string) - OAuth bearer token.
    /// * *access_token* (query-string) - OAuth access token.
    /// * *uploadType* (query-string) - Legacy upload protocol for media (e.g. "media", "multipart").
    /// * *quotaUser* (query-string) - Available to use for quota purposes for server-side applications. Can be any arbitrary string assigned to a user, but should not exceed 40 characters.
    /// * *callback* (query-string) - JSONP
    /// * *oauth_token* (query-string) - OAuth 2.0 token for the current user.
    /// * *key* (query-string) - API key. Your API key identifies your project and provides you with API access, quota, and reports. Required unless you provide an OAuth 2.0 token.
    /// * *fields* (query-string) - Selector specifying which fields to include in a partial response.
    /// * *alt* (query-string) - Data format for response.
    /// * *$.xgafv* (query-string) - V1 error format.
    pub fn param<T>(mut self, name: T, value: T) -> InvitationAcceptCall<'a, C, A>
                                                        where T: AsRef<str> {
        self._additional_params.insert(name.as_ref().to_string(), value.as_ref().to_string());
        self
    }

    /// Identifies the authorization scope for the method you are building.
    ///
    /// Use this method to actively specify which scope should be used, instead the default `Scope` variant
    /// `Scope::Roster`.
    ///
    /// The `scope` will be added to a set of scopes. This is important as one can maintain access
    /// tokens for more than one scope.
    /// If `None` is specified, then all scopes will be removed and no default scope will be used either.
    /// In that case, you have to specify your API-key using the `key` parameter (see the `param()`
    /// function for details).
    ///
    /// Usually there is more than one suitable scope to authorize an operation, some of which may
    /// encompass more rights than others. For example, for listing resources, a *read-only* scope will be
    /// sufficient, a read-write scope will do as well.
    pub fn add_scope<T, S>(mut self, scope: T) -> InvitationAcceptCall<'a, C, A>
                                                        where T: Into<Option<S>>,
                                                              S: AsRef<str> {
        match scope.into() {
          Some(scope) => self._scopes.insert(scope.as_ref().to_string(), ()),
          None => None,
        };
        self
    }
}


