// DO NOT EDIT !
// This file was generated automatically from 'src/generator/templates/api/lib.rs.mako'
// DO NOT EDIT !

//! This documentation was generated from *classroom* crate version *5.0.5+20240617*, where *20240617* is the exact revision of the *classroom:v1* schema built by the [mako](http://www.makotemplates.org/) code generator *v5.0.5*.
//! 
//! Everything else about the *classroom* *v1* API can be found at the
//! [official documentation site](https://developers.google.com/classroom/).
//! The original source code is [on github](https://github.com/Byron/google-apis-rs/tree/main/gen/classroom1).
//! # Features
//! 
//! Handle the following *Resources* with ease from the central [hub](Classroom) ... 
//! 
//! * [courses](api::Course)
//!  * [*aliases create*](api::CourseAliasCreateCall), [*aliases delete*](api::CourseAliasDeleteCall), [*aliases list*](api::CourseAliasListCall), [*announcements add on attachments create*](api::CourseAnnouncementAddOnAttachmentCreateCall), [*announcements add on attachments delete*](api::CourseAnnouncementAddOnAttachmentDeleteCall), [*announcements add on attachments get*](api::CourseAnnouncementAddOnAttachmentGetCall), [*announcements add on attachments list*](api::CourseAnnouncementAddOnAttachmentListCall), [*announcements add on attachments patch*](api::CourseAnnouncementAddOnAttachmentPatchCall), [*announcements create*](api::CourseAnnouncementCreateCall), [*announcements delete*](api::CourseAnnouncementDeleteCall), [*announcements get*](api::CourseAnnouncementGetCall), [*announcements get add on context*](api::CourseAnnouncementGetAddOnContextCall), [*announcements list*](api::CourseAnnouncementListCall), [*announcements modify assignees*](api::CourseAnnouncementModifyAssigneeCall), [*announcements patch*](api::CourseAnnouncementPatchCall), [*course work add on attachments create*](api::CourseCourseWorkAddOnAttachmentCreateCall), [*course work add on attachments delete*](api::CourseCourseWorkAddOnAttachmentDeleteCall), [*course work add on attachments get*](api::CourseCourseWorkAddOnAttachmentGetCall), [*course work add on attachments list*](api::CourseCourseWorkAddOnAttachmentListCall), [*course work add on attachments patch*](api::CourseCourseWorkAddOnAttachmentPatchCall), [*course work add on attachments student submissions get*](api::CourseCourseWorkAddOnAttachmentStudentSubmissionGetCall), [*course work add on attachments student submissions patch*](api::CourseCourseWorkAddOnAttachmentStudentSubmissionPatchCall), [*course work create*](api::CourseCourseWorkCreateCall), [*course work delete*](api::CourseCourseWorkDeleteCall), [*course work get*](api::CourseCourseWorkGetCall), [*course work get add on context*](api::CourseCourseWorkGetAddOnContextCall), [*course work list*](api::CourseCourseWorkListCall), [*course work modify assignees*](api::CourseCourseWorkModifyAssigneeCall), [*course work patch*](api::CourseCourseWorkPatchCall), [*course work student submissions get*](api::CourseCourseWorkStudentSubmissionGetCall), [*course work student submissions list*](api::CourseCourseWorkStudentSubmissionListCall), [*course work student submissions modify attachments*](api::CourseCourseWorkStudentSubmissionModifyAttachmentCall), [*course work student submissions patch*](api::CourseCourseWorkStudentSubmissionPatchCall), [*course work student submissions reclaim*](api::CourseCourseWorkStudentSubmissionReclaimCall), [*course work student submissions return*](api::CourseCourseWorkStudentSubmissionReturnCall), [*course work student submissions turn in*](api::CourseCourseWorkStudentSubmissionTurnInCall), [*course work materials add on attachments create*](api::CourseCourseWorkMaterialAddOnAttachmentCreateCall), [*course work materials add on attachments delete*](api::CourseCourseWorkMaterialAddOnAttachmentDeleteCall), [*course work materials add on attachments get*](api::CourseCourseWorkMaterialAddOnAttachmentGetCall), [*course work materials add on attachments list*](api::CourseCourseWorkMaterialAddOnAttachmentListCall), [*course work materials add on attachments patch*](api::CourseCourseWorkMaterialAddOnAttachmentPatchCall), [*course work materials create*](api::CourseCourseWorkMaterialCreateCall), [*course work materials delete*](api::CourseCourseWorkMaterialDeleteCall), [*course work materials get*](api::CourseCourseWorkMaterialGetCall), [*course work materials get add on context*](api::CourseCourseWorkMaterialGetAddOnContextCall), [*course work materials list*](api::CourseCourseWorkMaterialListCall), [*course work materials patch*](api::CourseCourseWorkMaterialPatchCall), [*create*](api::CourseCreateCall), [*delete*](api::CourseDeleteCall), [*get*](api::CourseGetCall), [*list*](api::CourseListCall), [*patch*](api::CoursePatchCall), [*posts add on attachments create*](api::CoursePostAddOnAttachmentCreateCall), [*posts add on attachments delete*](api::CoursePostAddOnAttachmentDeleteCall), [*posts add on attachments get*](api::CoursePostAddOnAttachmentGetCall), [*posts add on attachments list*](api::CoursePostAddOnAttachmentListCall), [*posts add on attachments patch*](api::CoursePostAddOnAttachmentPatchCall), [*posts add on attachments student submissions get*](api::CoursePostAddOnAttachmentStudentSubmissionGetCall), [*posts add on attachments student submissions patch*](api::CoursePostAddOnAttachmentStudentSubmissionPatchCall), [*posts get add on context*](api::CoursePostGetAddOnContextCall), [*students create*](api::CourseStudentCreateCall), [*students delete*](api::CourseStudentDeleteCall), [*students get*](api::CourseStudentGetCall), [*students list*](api::CourseStudentListCall), [*teachers create*](api::CourseTeacherCreateCall), [*teachers delete*](api::CourseTeacherDeleteCall), [*teachers get*](api::CourseTeacherGetCall), [*teachers list*](api::CourseTeacherListCall), [*topics create*](api::CourseTopicCreateCall), [*topics delete*](api::CourseTopicDeleteCall), [*topics get*](api::CourseTopicGetCall), [*topics list*](api::CourseTopicListCall), [*topics patch*](api::CourseTopicPatchCall) and [*update*](api::CourseUpdateCall)
//! * [invitations](api::Invitation)
//!  * [*accept*](api::InvitationAcceptCall), [*create*](api::InvitationCreateCall), [*delete*](api::InvitationDeleteCall), [*get*](api::InvitationGetCall) and [*list*](api::InvitationListCall)
//! * [registrations](api::Registration)
//!  * [*create*](api::RegistrationCreateCall) and [*delete*](api::RegistrationDeleteCall)
//! * [user profiles](api::UserProfile)
//!  * [*get*](api::UserProfileGetCall), [*guardian invitations create*](api::UserProfileGuardianInvitationCreateCall), [*guardian invitations get*](api::UserProfileGuardianInvitationGetCall), [*guardian invitations list*](api::UserProfileGuardianInvitationListCall), [*guardian invitations patch*](api::UserProfileGuardianInvitationPatchCall), [*guardians delete*](api::UserProfileGuardianDeleteCall), [*guardians get*](api::UserProfileGuardianGetCall) and [*guardians list*](api::UserProfileGuardianListCall)
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
//! * **[Hub](Classroom)**
//!     * a central object to maintain state and allow accessing all *Activities*
//!     * creates [*Method Builders*](client::MethodsBuilder) which in turn
//!       allow access to individual [*Call Builders*](client::CallBuilder)
//! * **[Resources](client::Resource)**
//!     * primary types that you can apply *Activities* to
//!     * a collection of properties and *Parts*
//!     * **[Parts](client::Part)**
//!         * a collection of properties
//!         * never directly used in *Activities*
//! * **[Activities](client::CallBuilder)**
//!     * operations to apply to *Resources*
//! 
//! All *structures* are marked with applicable traits to further categorize them and ease browsing.
//! 
//! Generally speaking, you can invoke *Activities* like this:
//! 
//! ```Rust,ignore
//! let r = hub.resource().activity(...).doit().await
//! ```
//! 
//! Or specifically ...
//! 
//! ```ignore
//! let r = hub.courses().aliases_create(...).doit().await
//! let r = hub.courses().aliases_delete(...).doit().await
//! let r = hub.courses().aliases_list(...).doit().await
//! let r = hub.courses().announcements_add_on_attachments_create(...).doit().await
//! let r = hub.courses().announcements_add_on_attachments_delete(...).doit().await
//! let r = hub.courses().announcements_add_on_attachments_get(...).doit().await
//! let r = hub.courses().announcements_add_on_attachments_list(...).doit().await
//! let r = hub.courses().announcements_add_on_attachments_patch(...).doit().await
//! let r = hub.courses().announcements_create(...).doit().await
//! let r = hub.courses().announcements_delete(...).doit().await
//! let r = hub.courses().announcements_get(...).doit().await
//! let r = hub.courses().announcements_get_add_on_context(...).doit().await
//! let r = hub.courses().announcements_list(...).doit().await
//! let r = hub.courses().announcements_modify_assignees(...).doit().await
//! let r = hub.courses().announcements_patch(...).doit().await
//! let r = hub.courses().course_work_add_on_attachments_student_submissions_get(...).doit().await
//! let r = hub.courses().course_work_add_on_attachments_student_submissions_patch(...).doit().await
//! let r = hub.courses().course_work_add_on_attachments_create(...).doit().await
//! let r = hub.courses().course_work_add_on_attachments_delete(...).doit().await
//! let r = hub.courses().course_work_add_on_attachments_get(...).doit().await
//! let r = hub.courses().course_work_add_on_attachments_list(...).doit().await
//! let r = hub.courses().course_work_add_on_attachments_patch(...).doit().await
//! let r = hub.courses().course_work_student_submissions_get(...).doit().await
//! let r = hub.courses().course_work_student_submissions_list(...).doit().await
//! let r = hub.courses().course_work_student_submissions_modify_attachments(...).doit().await
//! let r = hub.courses().course_work_student_submissions_patch(...).doit().await
//! let r = hub.courses().course_work_student_submissions_reclaim(...).doit().await
//! let r = hub.courses().course_work_student_submissions_return(...).doit().await
//! let r = hub.courses().course_work_student_submissions_turn_in(...).doit().await
//! let r = hub.courses().course_work_create(...).doit().await
//! let r = hub.courses().course_work_delete(...).doit().await
//! let r = hub.courses().course_work_get(...).doit().await
//! let r = hub.courses().course_work_get_add_on_context(...).doit().await
//! let r = hub.courses().course_work_list(...).doit().await
//! let r = hub.courses().course_work_modify_assignees(...).doit().await
//! let r = hub.courses().course_work_patch(...).doit().await
//! let r = hub.courses().course_work_materials_add_on_attachments_create(...).doit().await
//! let r = hub.courses().course_work_materials_add_on_attachments_delete(...).doit().await
//! let r = hub.courses().course_work_materials_add_on_attachments_get(...).doit().await
//! let r = hub.courses().course_work_materials_add_on_attachments_list(...).doit().await
//! let r = hub.courses().course_work_materials_add_on_attachments_patch(...).doit().await
//! let r = hub.courses().course_work_materials_create(...).doit().await
//! let r = hub.courses().course_work_materials_delete(...).doit().await
//! let r = hub.courses().course_work_materials_get(...).doit().await
//! let r = hub.courses().course_work_materials_get_add_on_context(...).doit().await
//! let r = hub.courses().course_work_materials_list(...).doit().await
//! let r = hub.courses().course_work_materials_patch(...).doit().await
//! let r = hub.courses().posts_add_on_attachments_student_submissions_get(...).doit().await
//! let r = hub.courses().posts_add_on_attachments_student_submissions_patch(...).doit().await
//! let r = hub.courses().posts_add_on_attachments_create(...).doit().await
//! let r = hub.courses().posts_add_on_attachments_delete(...).doit().await
//! let r = hub.courses().posts_add_on_attachments_get(...).doit().await
//! let r = hub.courses().posts_add_on_attachments_list(...).doit().await
//! let r = hub.courses().posts_add_on_attachments_patch(...).doit().await
//! let r = hub.courses().posts_get_add_on_context(...).doit().await
//! let r = hub.courses().students_create(...).doit().await
//! let r = hub.courses().students_delete(...).doit().await
//! let r = hub.courses().students_get(...).doit().await
//! let r = hub.courses().students_list(...).doit().await
//! let r = hub.courses().teachers_create(...).doit().await
//! let r = hub.courses().teachers_delete(...).doit().await
//! let r = hub.courses().teachers_get(...).doit().await
//! let r = hub.courses().teachers_list(...).doit().await
//! let r = hub.courses().topics_create(...).doit().await
//! let r = hub.courses().topics_delete(...).doit().await
//! let r = hub.courses().topics_get(...).doit().await
//! let r = hub.courses().topics_list(...).doit().await
//! let r = hub.courses().topics_patch(...).doit().await
//! let r = hub.courses().create(...).doit().await
//! let r = hub.courses().delete(...).doit().await
//! let r = hub.courses().get(...).doit().await
//! let r = hub.courses().list(...).doit().await
//! let r = hub.courses().patch(...).doit().await
//! let r = hub.courses().update(...).doit().await
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
//! serde = "^1.0"
//! serde_json = "^1.0"
//! ```
//! 
//! ## A complete example
//! 
//! ```test_harness,no_run
//! extern crate hyper;
//! extern crate hyper_rustls;
//! extern crate google_classroom1 as classroom1;
//! use classroom1::api::AddOnAttachmentStudentSubmission;
//! use classroom1::{Result, Error};
//! # async fn dox() {
//! use std::default::Default;
//! use classroom1::{Classroom, oauth2, hyper, hyper_rustls, chrono, FieldMask};
//! 
//! // Get an ApplicationSecret instance by some means. It contains the `client_id` and 
//! // `client_secret`, among other things.
//! let secret: oauth2::ApplicationSecret = Default::default();
//! // Instantiate the authenticator. It will choose a suitable authentication flow for you, 
//! // unless you replace  `None` with the desired Flow.
//! // Provide your own `AuthenticatorDelegate` to adjust the way it operates and get feedback about 
//! // what's going on. You probably want to bring in your own `TokenStorage` to persist tokens and
//! // retrieve them from storage.
//! let auth = oauth2::InstalledFlowAuthenticator::builder(
//!         secret,
//!         oauth2::InstalledFlowReturnMethod::HTTPRedirect,
//!     ).build().await.unwrap();
//! let mut hub = Classroom::new(hyper::Client::builder().build(hyper_rustls::HttpsConnectorBuilder::new().with_native_roots().unwrap().https_or_http().enable_http1().build()), auth);
//! // As the method needs a request, you would usually fill it with the desired information
//! // into the respective structure. Some of the parts shown here might not be applicable !
//! // Values shown here are possibly random and not representative !
//! let mut req = AddOnAttachmentStudentSubmission::default();
//! 
//! // You can configure optional parameters by calling the respective setters at will, and
//! // execute the final call using `doit()`.
//! // Values shown here are possibly random and not representative !
//! let result = hub.courses().course_work_add_on_attachments_student_submissions_patch(req, "courseId", "itemId", "attachmentId", "submissionId")
//!              .update_mask(FieldMask::new::<&str>(&[]))
//!              .post_id("takimata")
//!              .doit().await;
//! 
//! match result {
//!     Err(e) => match e {
//!         // The Error enum provides details about what exactly happened.
//!         // You can also just use its `Debug`, `Display` or `Error` traits
//!          Error::HttpError(_)
//!         |Error::Io(_)
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
//! All errors produced by the system are provided either as [Result](client::Result) enumeration as return value of
//! the doit() methods, or handed as possibly intermediate results to either the 
//! [Hub Delegate](client::Delegate), or the [Authenticator Delegate](https://docs.rs/yup-oauth2/*/yup_oauth2/trait.AuthenticatorDelegate.html).
//! 
//! When delegates handle errors or intermediate values, they may have a chance to instruct the system to retry. This 
//! makes the system potentially resilient to all kinds of errors.
//! 
//! ## Uploads and Downloads
//! If a method supports downloads, the response body, which is part of the [Result](client::Result), should be
//! read by you to obtain the media.
//! If such a method also supports a [Response Result](client::ResponseResult), it will return that by default.
//! You can see it as meta-data for the actual media. To trigger a media download, you will have to set up the builder by making
//! this call: `.param("alt", "media")`.
//! 
//! Methods supporting uploads can do so using up to 2 different protocols: 
//! *simple* and *resumable*. The distinctiveness of each is represented by customized 
//! `doit(...)` methods, which are then named `upload(...)` and `upload_resumable(...)` respectively.
//! 
//! ## Customization and Callbacks
//! 
//! You may alter the way an `doit()` method is called by providing a [delegate](client::Delegate) to the 
//! [Method Builder](client::CallBuilder) before making the final `doit()` call. 
//! Respective methods will be called to provide progress information, as well as determine whether the system should 
//! retry on failure.
//! 
//! The [delegate trait](client::Delegate) is default-implemented, allowing you to customize it with minimal effort.
//! 
//! ## Optional Parts in Server-Requests
//! 
//! All structures provided by this library are made to be [encodable](client::RequestValue) and 
//! [decodable](client::ResponseResult) via *json*. Optionals are used to indicate that partial requests are responses 
//! are valid.
//! Most optionals are are considered [Parts](client::Part) which are identifiable by name, which will be sent to 
//! the server to indicate either the set parts of the request or the desired parts in the response.
//! 
//! ## Builder Arguments
//! 
//! Using [method builders](client::CallBuilder), you are able to prepare an action call by repeatedly calling it's methods.
//! These will always take a single argument, for which the following statements are true.
//! 
//! * [PODs][wiki-pod] are handed by copy
//! * strings are passed as `&str`
//! * [request values](client::RequestValue) are moved
//! 
//! Arguments will always be copied or cloned into the builder, to make them independent of their original life times.
//! 
//! [wiki-pod]: http://en.wikipedia.org/wiki/Plain_old_data_structure
//! [builder-pattern]: http://en.wikipedia.org/wiki/Builder_pattern
//! [google-go-api]: https://github.com/google/google-api-go-client
//! 
//! ## Cargo Features
//! 
//! * `utoipa` - Add support for [utoipa](https://crates.io/crates/utoipa) and derive `utoipa::ToSchema` on all
//! the types. You'll have to import and register the required types in `#[openapi(schemas(...))]`, otherwise the
//! generated `openapi` spec would be invalid.
//! 
//! 
//! 

// Unused attributes happen thanks to defined, but unused structures
// We don't warn about this, as depending on the API, some data structures or facilities are never used.
// Instead of pre-determining this, we just disable the lint. It's manually tuned to not have any
// unused imports in fully featured APIs. Same with unused_mut ... .
#![allow(unused_imports, unused_mut, dead_code)]

// DO NOT EDIT !
// This file was generated automatically from 'src/generator/templates/api/lib.rs.mako'
// DO NOT EDIT !

// Re-export the hyper and hyper_rustls crate, they are required to build the hub
pub use hyper;
pub use hyper_rustls;
pub extern crate google_apis_common as client;
pub use client::chrono;
pub mod api;

// Re-export the hub type and some basic client structs
pub use api::Classroom;
pub use client::{Result, Error, Delegate, FieldMask};

// Re-export the yup_oauth2 crate, that is required to call some methods of the hub and the client
#[cfg(feature = "yup-oauth2")]
pub use client::oauth2;