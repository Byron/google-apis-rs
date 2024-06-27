<!---
DO NOT EDIT !
This file was generated automatically from 'src/generator/templates/api/README.md.mako'
DO NOT EDIT !
-->
The `google-classroom1` library allows access to all features of the *Google classroom* service.

This documentation was generated from *classroom* crate version *5.0.5+20240617*, where *20240617* is the exact revision of the *classroom:v1* schema built by the [mako](http://www.makotemplates.org/) code generator *v5.0.5*.

Everything else about the *classroom* *v1* API can be found at the
[official documentation site](https://developers.google.com/classroom/).
# Features

Handle the following *Resources* with ease from the central [hub](https://docs.rs/google-classroom1/5.0.5+20240617/google_classroom1/Classroom) ... 

* [courses](https://docs.rs/google-classroom1/5.0.5+20240617/google_classroom1/api::Course)
 * [*aliases create*](https://docs.rs/google-classroom1/5.0.5+20240617/google_classroom1/api::CourseAliasCreateCall), [*aliases delete*](https://docs.rs/google-classroom1/5.0.5+20240617/google_classroom1/api::CourseAliasDeleteCall), [*aliases list*](https://docs.rs/google-classroom1/5.0.5+20240617/google_classroom1/api::CourseAliasListCall), [*announcements add on attachments create*](https://docs.rs/google-classroom1/5.0.5+20240617/google_classroom1/api::CourseAnnouncementAddOnAttachmentCreateCall), [*announcements add on attachments delete*](https://docs.rs/google-classroom1/5.0.5+20240617/google_classroom1/api::CourseAnnouncementAddOnAttachmentDeleteCall), [*announcements add on attachments get*](https://docs.rs/google-classroom1/5.0.5+20240617/google_classroom1/api::CourseAnnouncementAddOnAttachmentGetCall), [*announcements add on attachments list*](https://docs.rs/google-classroom1/5.0.5+20240617/google_classroom1/api::CourseAnnouncementAddOnAttachmentListCall), [*announcements add on attachments patch*](https://docs.rs/google-classroom1/5.0.5+20240617/google_classroom1/api::CourseAnnouncementAddOnAttachmentPatchCall), [*announcements create*](https://docs.rs/google-classroom1/5.0.5+20240617/google_classroom1/api::CourseAnnouncementCreateCall), [*announcements delete*](https://docs.rs/google-classroom1/5.0.5+20240617/google_classroom1/api::CourseAnnouncementDeleteCall), [*announcements get*](https://docs.rs/google-classroom1/5.0.5+20240617/google_classroom1/api::CourseAnnouncementGetCall), [*announcements get add on context*](https://docs.rs/google-classroom1/5.0.5+20240617/google_classroom1/api::CourseAnnouncementGetAddOnContextCall), [*announcements list*](https://docs.rs/google-classroom1/5.0.5+20240617/google_classroom1/api::CourseAnnouncementListCall), [*announcements modify assignees*](https://docs.rs/google-classroom1/5.0.5+20240617/google_classroom1/api::CourseAnnouncementModifyAssigneeCall), [*announcements patch*](https://docs.rs/google-classroom1/5.0.5+20240617/google_classroom1/api::CourseAnnouncementPatchCall), [*course work add on attachments create*](https://docs.rs/google-classroom1/5.0.5+20240617/google_classroom1/api::CourseCourseWorkAddOnAttachmentCreateCall), [*course work add on attachments delete*](https://docs.rs/google-classroom1/5.0.5+20240617/google_classroom1/api::CourseCourseWorkAddOnAttachmentDeleteCall), [*course work add on attachments get*](https://docs.rs/google-classroom1/5.0.5+20240617/google_classroom1/api::CourseCourseWorkAddOnAttachmentGetCall), [*course work add on attachments list*](https://docs.rs/google-classroom1/5.0.5+20240617/google_classroom1/api::CourseCourseWorkAddOnAttachmentListCall), [*course work add on attachments patch*](https://docs.rs/google-classroom1/5.0.5+20240617/google_classroom1/api::CourseCourseWorkAddOnAttachmentPatchCall), [*course work add on attachments student submissions get*](https://docs.rs/google-classroom1/5.0.5+20240617/google_classroom1/api::CourseCourseWorkAddOnAttachmentStudentSubmissionGetCall), [*course work add on attachments student submissions patch*](https://docs.rs/google-classroom1/5.0.5+20240617/google_classroom1/api::CourseCourseWorkAddOnAttachmentStudentSubmissionPatchCall), [*course work create*](https://docs.rs/google-classroom1/5.0.5+20240617/google_classroom1/api::CourseCourseWorkCreateCall), [*course work delete*](https://docs.rs/google-classroom1/5.0.5+20240617/google_classroom1/api::CourseCourseWorkDeleteCall), [*course work get*](https://docs.rs/google-classroom1/5.0.5+20240617/google_classroom1/api::CourseCourseWorkGetCall), [*course work get add on context*](https://docs.rs/google-classroom1/5.0.5+20240617/google_classroom1/api::CourseCourseWorkGetAddOnContextCall), [*course work list*](https://docs.rs/google-classroom1/5.0.5+20240617/google_classroom1/api::CourseCourseWorkListCall), [*course work modify assignees*](https://docs.rs/google-classroom1/5.0.5+20240617/google_classroom1/api::CourseCourseWorkModifyAssigneeCall), [*course work patch*](https://docs.rs/google-classroom1/5.0.5+20240617/google_classroom1/api::CourseCourseWorkPatchCall), [*course work student submissions get*](https://docs.rs/google-classroom1/5.0.5+20240617/google_classroom1/api::CourseCourseWorkStudentSubmissionGetCall), [*course work student submissions list*](https://docs.rs/google-classroom1/5.0.5+20240617/google_classroom1/api::CourseCourseWorkStudentSubmissionListCall), [*course work student submissions modify attachments*](https://docs.rs/google-classroom1/5.0.5+20240617/google_classroom1/api::CourseCourseWorkStudentSubmissionModifyAttachmentCall), [*course work student submissions patch*](https://docs.rs/google-classroom1/5.0.5+20240617/google_classroom1/api::CourseCourseWorkStudentSubmissionPatchCall), [*course work student submissions reclaim*](https://docs.rs/google-classroom1/5.0.5+20240617/google_classroom1/api::CourseCourseWorkStudentSubmissionReclaimCall), [*course work student submissions return*](https://docs.rs/google-classroom1/5.0.5+20240617/google_classroom1/api::CourseCourseWorkStudentSubmissionReturnCall), [*course work student submissions turn in*](https://docs.rs/google-classroom1/5.0.5+20240617/google_classroom1/api::CourseCourseWorkStudentSubmissionTurnInCall), [*course work materials add on attachments create*](https://docs.rs/google-classroom1/5.0.5+20240617/google_classroom1/api::CourseCourseWorkMaterialAddOnAttachmentCreateCall), [*course work materials add on attachments delete*](https://docs.rs/google-classroom1/5.0.5+20240617/google_classroom1/api::CourseCourseWorkMaterialAddOnAttachmentDeleteCall), [*course work materials add on attachments get*](https://docs.rs/google-classroom1/5.0.5+20240617/google_classroom1/api::CourseCourseWorkMaterialAddOnAttachmentGetCall), [*course work materials add on attachments list*](https://docs.rs/google-classroom1/5.0.5+20240617/google_classroom1/api::CourseCourseWorkMaterialAddOnAttachmentListCall), [*course work materials add on attachments patch*](https://docs.rs/google-classroom1/5.0.5+20240617/google_classroom1/api::CourseCourseWorkMaterialAddOnAttachmentPatchCall), [*course work materials create*](https://docs.rs/google-classroom1/5.0.5+20240617/google_classroom1/api::CourseCourseWorkMaterialCreateCall), [*course work materials delete*](https://docs.rs/google-classroom1/5.0.5+20240617/google_classroom1/api::CourseCourseWorkMaterialDeleteCall), [*course work materials get*](https://docs.rs/google-classroom1/5.0.5+20240617/google_classroom1/api::CourseCourseWorkMaterialGetCall), [*course work materials get add on context*](https://docs.rs/google-classroom1/5.0.5+20240617/google_classroom1/api::CourseCourseWorkMaterialGetAddOnContextCall), [*course work materials list*](https://docs.rs/google-classroom1/5.0.5+20240617/google_classroom1/api::CourseCourseWorkMaterialListCall), [*course work materials patch*](https://docs.rs/google-classroom1/5.0.5+20240617/google_classroom1/api::CourseCourseWorkMaterialPatchCall), [*create*](https://docs.rs/google-classroom1/5.0.5+20240617/google_classroom1/api::CourseCreateCall), [*delete*](https://docs.rs/google-classroom1/5.0.5+20240617/google_classroom1/api::CourseDeleteCall), [*get*](https://docs.rs/google-classroom1/5.0.5+20240617/google_classroom1/api::CourseGetCall), [*list*](https://docs.rs/google-classroom1/5.0.5+20240617/google_classroom1/api::CourseListCall), [*patch*](https://docs.rs/google-classroom1/5.0.5+20240617/google_classroom1/api::CoursePatchCall), [*posts add on attachments create*](https://docs.rs/google-classroom1/5.0.5+20240617/google_classroom1/api::CoursePostAddOnAttachmentCreateCall), [*posts add on attachments delete*](https://docs.rs/google-classroom1/5.0.5+20240617/google_classroom1/api::CoursePostAddOnAttachmentDeleteCall), [*posts add on attachments get*](https://docs.rs/google-classroom1/5.0.5+20240617/google_classroom1/api::CoursePostAddOnAttachmentGetCall), [*posts add on attachments list*](https://docs.rs/google-classroom1/5.0.5+20240617/google_classroom1/api::CoursePostAddOnAttachmentListCall), [*posts add on attachments patch*](https://docs.rs/google-classroom1/5.0.5+20240617/google_classroom1/api::CoursePostAddOnAttachmentPatchCall), [*posts add on attachments student submissions get*](https://docs.rs/google-classroom1/5.0.5+20240617/google_classroom1/api::CoursePostAddOnAttachmentStudentSubmissionGetCall), [*posts add on attachments student submissions patch*](https://docs.rs/google-classroom1/5.0.5+20240617/google_classroom1/api::CoursePostAddOnAttachmentStudentSubmissionPatchCall), [*posts get add on context*](https://docs.rs/google-classroom1/5.0.5+20240617/google_classroom1/api::CoursePostGetAddOnContextCall), [*students create*](https://docs.rs/google-classroom1/5.0.5+20240617/google_classroom1/api::CourseStudentCreateCall), [*students delete*](https://docs.rs/google-classroom1/5.0.5+20240617/google_classroom1/api::CourseStudentDeleteCall), [*students get*](https://docs.rs/google-classroom1/5.0.5+20240617/google_classroom1/api::CourseStudentGetCall), [*students list*](https://docs.rs/google-classroom1/5.0.5+20240617/google_classroom1/api::CourseStudentListCall), [*teachers create*](https://docs.rs/google-classroom1/5.0.5+20240617/google_classroom1/api::CourseTeacherCreateCall), [*teachers delete*](https://docs.rs/google-classroom1/5.0.5+20240617/google_classroom1/api::CourseTeacherDeleteCall), [*teachers get*](https://docs.rs/google-classroom1/5.0.5+20240617/google_classroom1/api::CourseTeacherGetCall), [*teachers list*](https://docs.rs/google-classroom1/5.0.5+20240617/google_classroom1/api::CourseTeacherListCall), [*topics create*](https://docs.rs/google-classroom1/5.0.5+20240617/google_classroom1/api::CourseTopicCreateCall), [*topics delete*](https://docs.rs/google-classroom1/5.0.5+20240617/google_classroom1/api::CourseTopicDeleteCall), [*topics get*](https://docs.rs/google-classroom1/5.0.5+20240617/google_classroom1/api::CourseTopicGetCall), [*topics list*](https://docs.rs/google-classroom1/5.0.5+20240617/google_classroom1/api::CourseTopicListCall), [*topics patch*](https://docs.rs/google-classroom1/5.0.5+20240617/google_classroom1/api::CourseTopicPatchCall) and [*update*](https://docs.rs/google-classroom1/5.0.5+20240617/google_classroom1/api::CourseUpdateCall)
* [invitations](https://docs.rs/google-classroom1/5.0.5+20240617/google_classroom1/api::Invitation)
 * [*accept*](https://docs.rs/google-classroom1/5.0.5+20240617/google_classroom1/api::InvitationAcceptCall), [*create*](https://docs.rs/google-classroom1/5.0.5+20240617/google_classroom1/api::InvitationCreateCall), [*delete*](https://docs.rs/google-classroom1/5.0.5+20240617/google_classroom1/api::InvitationDeleteCall), [*get*](https://docs.rs/google-classroom1/5.0.5+20240617/google_classroom1/api::InvitationGetCall) and [*list*](https://docs.rs/google-classroom1/5.0.5+20240617/google_classroom1/api::InvitationListCall)
* [registrations](https://docs.rs/google-classroom1/5.0.5+20240617/google_classroom1/api::Registration)
 * [*create*](https://docs.rs/google-classroom1/5.0.5+20240617/google_classroom1/api::RegistrationCreateCall) and [*delete*](https://docs.rs/google-classroom1/5.0.5+20240617/google_classroom1/api::RegistrationDeleteCall)
* [user profiles](https://docs.rs/google-classroom1/5.0.5+20240617/google_classroom1/api::UserProfile)
 * [*get*](https://docs.rs/google-classroom1/5.0.5+20240617/google_classroom1/api::UserProfileGetCall), [*guardian invitations create*](https://docs.rs/google-classroom1/5.0.5+20240617/google_classroom1/api::UserProfileGuardianInvitationCreateCall), [*guardian invitations get*](https://docs.rs/google-classroom1/5.0.5+20240617/google_classroom1/api::UserProfileGuardianInvitationGetCall), [*guardian invitations list*](https://docs.rs/google-classroom1/5.0.5+20240617/google_classroom1/api::UserProfileGuardianInvitationListCall), [*guardian invitations patch*](https://docs.rs/google-classroom1/5.0.5+20240617/google_classroom1/api::UserProfileGuardianInvitationPatchCall), [*guardians delete*](https://docs.rs/google-classroom1/5.0.5+20240617/google_classroom1/api::UserProfileGuardianDeleteCall), [*guardians get*](https://docs.rs/google-classroom1/5.0.5+20240617/google_classroom1/api::UserProfileGuardianGetCall) and [*guardians list*](https://docs.rs/google-classroom1/5.0.5+20240617/google_classroom1/api::UserProfileGuardianListCall)




# Structure of this Library

The API is structured into the following primary items:

* **[Hub](https://docs.rs/google-classroom1/5.0.5+20240617/google_classroom1/Classroom)**
    * a central object to maintain state and allow accessing all *Activities*
    * creates [*Method Builders*](https://docs.rs/google-classroom1/5.0.5+20240617/google_classroom1/client::MethodsBuilder) which in turn
      allow access to individual [*Call Builders*](https://docs.rs/google-classroom1/5.0.5+20240617/google_classroom1/client::CallBuilder)
* **[Resources](https://docs.rs/google-classroom1/5.0.5+20240617/google_classroom1/client::Resource)**
    * primary types that you can apply *Activities* to
    * a collection of properties and *Parts*
    * **[Parts](https://docs.rs/google-classroom1/5.0.5+20240617/google_classroom1/client::Part)**
        * a collection of properties
        * never directly used in *Activities*
* **[Activities](https://docs.rs/google-classroom1/5.0.5+20240617/google_classroom1/client::CallBuilder)**
    * operations to apply to *Resources*

All *structures* are marked with applicable traits to further categorize them and ease browsing.

Generally speaking, you can invoke *Activities* like this:

```Rust,ignore
let r = hub.resource().activity(...).doit().await
```

Or specifically ...

```ignore
let r = hub.courses().aliases_create(...).doit().await
let r = hub.courses().aliases_delete(...).doit().await
let r = hub.courses().aliases_list(...).doit().await
let r = hub.courses().announcements_add_on_attachments_create(...).doit().await
let r = hub.courses().announcements_add_on_attachments_delete(...).doit().await
let r = hub.courses().announcements_add_on_attachments_get(...).doit().await
let r = hub.courses().announcements_add_on_attachments_list(...).doit().await
let r = hub.courses().announcements_add_on_attachments_patch(...).doit().await
let r = hub.courses().announcements_create(...).doit().await
let r = hub.courses().announcements_delete(...).doit().await
let r = hub.courses().announcements_get(...).doit().await
let r = hub.courses().announcements_get_add_on_context(...).doit().await
let r = hub.courses().announcements_list(...).doit().await
let r = hub.courses().announcements_modify_assignees(...).doit().await
let r = hub.courses().announcements_patch(...).doit().await
let r = hub.courses().course_work_add_on_attachments_student_submissions_get(...).doit().await
let r = hub.courses().course_work_add_on_attachments_student_submissions_patch(...).doit().await
let r = hub.courses().course_work_add_on_attachments_create(...).doit().await
let r = hub.courses().course_work_add_on_attachments_delete(...).doit().await
let r = hub.courses().course_work_add_on_attachments_get(...).doit().await
let r = hub.courses().course_work_add_on_attachments_list(...).doit().await
let r = hub.courses().course_work_add_on_attachments_patch(...).doit().await
let r = hub.courses().course_work_student_submissions_get(...).doit().await
let r = hub.courses().course_work_student_submissions_list(...).doit().await
let r = hub.courses().course_work_student_submissions_modify_attachments(...).doit().await
let r = hub.courses().course_work_student_submissions_patch(...).doit().await
let r = hub.courses().course_work_student_submissions_reclaim(...).doit().await
let r = hub.courses().course_work_student_submissions_return(...).doit().await
let r = hub.courses().course_work_student_submissions_turn_in(...).doit().await
let r = hub.courses().course_work_create(...).doit().await
let r = hub.courses().course_work_delete(...).doit().await
let r = hub.courses().course_work_get(...).doit().await
let r = hub.courses().course_work_get_add_on_context(...).doit().await
let r = hub.courses().course_work_list(...).doit().await
let r = hub.courses().course_work_modify_assignees(...).doit().await
let r = hub.courses().course_work_patch(...).doit().await
let r = hub.courses().course_work_materials_add_on_attachments_create(...).doit().await
let r = hub.courses().course_work_materials_add_on_attachments_delete(...).doit().await
let r = hub.courses().course_work_materials_add_on_attachments_get(...).doit().await
let r = hub.courses().course_work_materials_add_on_attachments_list(...).doit().await
let r = hub.courses().course_work_materials_add_on_attachments_patch(...).doit().await
let r = hub.courses().course_work_materials_create(...).doit().await
let r = hub.courses().course_work_materials_delete(...).doit().await
let r = hub.courses().course_work_materials_get(...).doit().await
let r = hub.courses().course_work_materials_get_add_on_context(...).doit().await
let r = hub.courses().course_work_materials_list(...).doit().await
let r = hub.courses().course_work_materials_patch(...).doit().await
let r = hub.courses().posts_add_on_attachments_student_submissions_get(...).doit().await
let r = hub.courses().posts_add_on_attachments_student_submissions_patch(...).doit().await
let r = hub.courses().posts_add_on_attachments_create(...).doit().await
let r = hub.courses().posts_add_on_attachments_delete(...).doit().await
let r = hub.courses().posts_add_on_attachments_get(...).doit().await
let r = hub.courses().posts_add_on_attachments_list(...).doit().await
let r = hub.courses().posts_add_on_attachments_patch(...).doit().await
let r = hub.courses().posts_get_add_on_context(...).doit().await
let r = hub.courses().students_create(...).doit().await
let r = hub.courses().students_delete(...).doit().await
let r = hub.courses().students_get(...).doit().await
let r = hub.courses().students_list(...).doit().await
let r = hub.courses().teachers_create(...).doit().await
let r = hub.courses().teachers_delete(...).doit().await
let r = hub.courses().teachers_get(...).doit().await
let r = hub.courses().teachers_list(...).doit().await
let r = hub.courses().topics_create(...).doit().await
let r = hub.courses().topics_delete(...).doit().await
let r = hub.courses().topics_get(...).doit().await
let r = hub.courses().topics_list(...).doit().await
let r = hub.courses().topics_patch(...).doit().await
let r = hub.courses().create(...).doit().await
let r = hub.courses().delete(...).doit().await
let r = hub.courses().get(...).doit().await
let r = hub.courses().list(...).doit().await
let r = hub.courses().patch(...).doit().await
let r = hub.courses().update(...).doit().await
```

The `resource()` and `activity(...)` calls create [builders][builder-pattern]. The second one dealing with `Activities` 
supports various methods to configure the impending operation (not shown here). It is made such that all required arguments have to be 
specified right away (i.e. `(...)`), whereas all optional ones can be [build up][builder-pattern] as desired.
The `doit()` method performs the actual communication with the server and returns the respective result.

# Usage

## Setting up your Project

To use this library, you would put the following lines into your `Cargo.toml` file:

```toml
[dependencies]
google-classroom1 = "*"
serde = "^1.0"
serde_json = "^1.0"
```

## A complete example

```Rust
extern crate hyper;
extern crate hyper_rustls;
extern crate google_classroom1 as classroom1;
use classroom1::api::AddOnAttachmentStudentSubmission;
use classroom1::{Result, Error};
use std::default::Default;
use classroom1::{Classroom, oauth2, hyper, hyper_rustls, chrono, FieldMask};

// Get an ApplicationSecret instance by some means. It contains the `client_id` and 
// `client_secret`, among other things.
let secret: oauth2::ApplicationSecret = Default::default();
// Instantiate the authenticator. It will choose a suitable authentication flow for you, 
// unless you replace  `None` with the desired Flow.
// Provide your own `AuthenticatorDelegate` to adjust the way it operates and get feedback about 
// what's going on. You probably want to bring in your own `TokenStorage` to persist tokens and
// retrieve them from storage.
let auth = oauth2::InstalledFlowAuthenticator::builder(
        secret,
        oauth2::InstalledFlowReturnMethod::HTTPRedirect,
    ).build().await.unwrap();
let mut hub = Classroom::new(hyper::Client::builder().build(hyper_rustls::HttpsConnectorBuilder::new().with_native_roots().unwrap().https_or_http().enable_http1().build()), auth);
// As the method needs a request, you would usually fill it with the desired information
// into the respective structure. Some of the parts shown here might not be applicable !
// Values shown here are possibly random and not representative !
let mut req = AddOnAttachmentStudentSubmission::default();

// You can configure optional parameters by calling the respective setters at will, and
// execute the final call using `doit()`.
// Values shown here are possibly random and not representative !
let result = hub.courses().course_work_add_on_attachments_student_submissions_patch(req, "courseId", "itemId", "attachmentId", "submissionId")
             .update_mask(FieldMask::new::<&str>(&[]))
             .post_id("voluptua.")
             .doit().await;

match result {
    Err(e) => match e {
        // The Error enum provides details about what exactly happened.
        // You can also just use its `Debug`, `Display` or `Error` traits
         Error::HttpError(_)
        |Error::Io(_)
        |Error::MissingAPIKey
        |Error::MissingToken(_)
        |Error::Cancelled
        |Error::UploadSizeLimitExceeded(_, _)
        |Error::Failure(_)
        |Error::BadRequest(_)
        |Error::FieldClash(_)
        |Error::JsonDecodeError(_, _) => println!("{}", e),
    },
    Ok(res) => println!("Success: {:?}", res),
}

```
## Handling Errors

All errors produced by the system are provided either as [Result](https://docs.rs/google-classroom1/5.0.5+20240617/google_classroom1/client::Result) enumeration as return value of
the doit() methods, or handed as possibly intermediate results to either the 
[Hub Delegate](https://docs.rs/google-classroom1/5.0.5+20240617/google_classroom1/client::Delegate), or the [Authenticator Delegate](https://docs.rs/yup-oauth2/*/yup_oauth2/trait.AuthenticatorDelegate.html).

When delegates handle errors or intermediate values, they may have a chance to instruct the system to retry. This 
makes the system potentially resilient to all kinds of errors.

## Uploads and Downloads
If a method supports downloads, the response body, which is part of the [Result](https://docs.rs/google-classroom1/5.0.5+20240617/google_classroom1/client::Result), should be
read by you to obtain the media.
If such a method also supports a [Response Result](https://docs.rs/google-classroom1/5.0.5+20240617/google_classroom1/client::ResponseResult), it will return that by default.
You can see it as meta-data for the actual media. To trigger a media download, you will have to set up the builder by making
this call: `.param("alt", "media")`.

Methods supporting uploads can do so using up to 2 different protocols: 
*simple* and *resumable*. The distinctiveness of each is represented by customized 
`doit(...)` methods, which are then named `upload(...)` and `upload_resumable(...)` respectively.

## Customization and Callbacks

You may alter the way an `doit()` method is called by providing a [delegate](https://docs.rs/google-classroom1/5.0.5+20240617/google_classroom1/client::Delegate) to the 
[Method Builder](https://docs.rs/google-classroom1/5.0.5+20240617/google_classroom1/client::CallBuilder) before making the final `doit()` call. 
Respective methods will be called to provide progress information, as well as determine whether the system should 
retry on failure.

The [delegate trait](https://docs.rs/google-classroom1/5.0.5+20240617/google_classroom1/client::Delegate) is default-implemented, allowing you to customize it with minimal effort.

## Optional Parts in Server-Requests

All structures provided by this library are made to be [encodable](https://docs.rs/google-classroom1/5.0.5+20240617/google_classroom1/client::RequestValue) and 
[decodable](https://docs.rs/google-classroom1/5.0.5+20240617/google_classroom1/client::ResponseResult) via *json*. Optionals are used to indicate that partial requests are responses 
are valid.
Most optionals are are considered [Parts](https://docs.rs/google-classroom1/5.0.5+20240617/google_classroom1/client::Part) which are identifiable by name, which will be sent to 
the server to indicate either the set parts of the request or the desired parts in the response.

## Builder Arguments

Using [method builders](https://docs.rs/google-classroom1/5.0.5+20240617/google_classroom1/client::CallBuilder), you are able to prepare an action call by repeatedly calling it's methods.
These will always take a single argument, for which the following statements are true.

* [PODs][wiki-pod] are handed by copy
* strings are passed as `&str`
* [request values](https://docs.rs/google-classroom1/5.0.5+20240617/google_classroom1/client::RequestValue) are moved

Arguments will always be copied or cloned into the builder, to make them independent of their original life times.

[wiki-pod]: http://en.wikipedia.org/wiki/Plain_old_data_structure
[builder-pattern]: http://en.wikipedia.org/wiki/Builder_pattern
[google-go-api]: https://github.com/google/google-api-go-client

## Cargo Features

* `utoipa` - Add support for [utoipa](https://crates.io/crates/utoipa) and derive `utoipa::ToSchema` on all
the types. You'll have to import and register the required types in `#[openapi(schemas(...))]`, otherwise the
generated `openapi` spec would be invalid.


# License
The **classroom1** library was generated by Sebastian Thiel, and is placed 
under the *MIT* license.
You can read the full text at the repository's [license file][repo-license].

[repo-license]: https://github.com/Byron/google-apis-rsblob/main/LICENSE.md

