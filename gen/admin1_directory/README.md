<!---
DO NOT EDIT !
This file was generated automatically from 'src/mako/api/README.md.mako'
DO NOT EDIT !
-->
The `google-admin1_directory` library allows access to all features of the *Google directory* service.

This documentation was generated from *directory* crate version *1.0.12+20190214*, where *20190214* is the exact revision of the *admin:directory_v1* schema built by the [mako](http://www.makotemplates.org/) code generator *v1.0.12*.

Everything else about the *directory* *v1_directory* API can be found at the
[official documentation site](https://developers.google.com/admin-sdk/directory/).
# Features

Handle the following *Resources* with ease from the central [hub](https://docs.rs/google-admin1_directory/1.0.12+20190214/google_admin1_directory/struct.Directory.html) ... 

* [asps](https://docs.rs/google-admin1_directory/1.0.12+20190214/google_admin1_directory/struct.Asp.html)
 * [*delete*](https://docs.rs/google-admin1_directory/1.0.12+20190214/google_admin1_directory/struct.AspDeleteCall.html), [*get*](https://docs.rs/google-admin1_directory/1.0.12+20190214/google_admin1_directory/struct.AspGetCall.html) and [*list*](https://docs.rs/google-admin1_directory/1.0.12+20190214/google_admin1_directory/struct.AspListCall.html)
* [channels](https://docs.rs/google-admin1_directory/1.0.12+20190214/google_admin1_directory/struct.Channel.html)
 * [*stop*](https://docs.rs/google-admin1_directory/1.0.12+20190214/google_admin1_directory/struct.ChannelStopCall.html)
* chromeosdevices
 * [*action*](https://docs.rs/google-admin1_directory/1.0.12+20190214/google_admin1_directory/struct.ChromeosdeviceActionCall.html), [*get*](https://docs.rs/google-admin1_directory/1.0.12+20190214/google_admin1_directory/struct.ChromeosdeviceGetCall.html), [*list*](https://docs.rs/google-admin1_directory/1.0.12+20190214/google_admin1_directory/struct.ChromeosdeviceListCall.html), [*move devices to ou*](https://docs.rs/google-admin1_directory/1.0.12+20190214/google_admin1_directory/struct.ChromeosdeviceMoveDevicesToOuCall.html), [*patch*](https://docs.rs/google-admin1_directory/1.0.12+20190214/google_admin1_directory/struct.ChromeosdevicePatchCall.html) and [*update*](https://docs.rs/google-admin1_directory/1.0.12+20190214/google_admin1_directory/struct.ChromeosdeviceUpdateCall.html)
* [customers](https://docs.rs/google-admin1_directory/1.0.12+20190214/google_admin1_directory/struct.Customer.html)
 * [*get*](https://docs.rs/google-admin1_directory/1.0.12+20190214/google_admin1_directory/struct.CustomerGetCall.html), [*patch*](https://docs.rs/google-admin1_directory/1.0.12+20190214/google_admin1_directory/struct.CustomerPatchCall.html) and [*update*](https://docs.rs/google-admin1_directory/1.0.12+20190214/google_admin1_directory/struct.CustomerUpdateCall.html)
* domain aliases
 * [*delete*](https://docs.rs/google-admin1_directory/1.0.12+20190214/google_admin1_directory/struct.DomainAliaseDeleteCall.html), [*get*](https://docs.rs/google-admin1_directory/1.0.12+20190214/google_admin1_directory/struct.DomainAliaseGetCall.html), [*insert*](https://docs.rs/google-admin1_directory/1.0.12+20190214/google_admin1_directory/struct.DomainAliaseInsertCall.html) and [*list*](https://docs.rs/google-admin1_directory/1.0.12+20190214/google_admin1_directory/struct.DomainAliaseListCall.html)
* domains
 * [*delete*](https://docs.rs/google-admin1_directory/1.0.12+20190214/google_admin1_directory/struct.DomainDeleteCall.html), [*get*](https://docs.rs/google-admin1_directory/1.0.12+20190214/google_admin1_directory/struct.DomainGetCall.html), [*insert*](https://docs.rs/google-admin1_directory/1.0.12+20190214/google_admin1_directory/struct.DomainInsertCall.html) and [*list*](https://docs.rs/google-admin1_directory/1.0.12+20190214/google_admin1_directory/struct.DomainListCall.html)
* [groups](https://docs.rs/google-admin1_directory/1.0.12+20190214/google_admin1_directory/struct.Group.html)
 * [*aliases delete*](https://docs.rs/google-admin1_directory/1.0.12+20190214/google_admin1_directory/struct.GroupAliaseDeleteCall.html), [*aliases insert*](https://docs.rs/google-admin1_directory/1.0.12+20190214/google_admin1_directory/struct.GroupAliaseInsertCall.html), [*aliases list*](https://docs.rs/google-admin1_directory/1.0.12+20190214/google_admin1_directory/struct.GroupAliaseListCall.html), [*delete*](https://docs.rs/google-admin1_directory/1.0.12+20190214/google_admin1_directory/struct.GroupDeleteCall.html), [*get*](https://docs.rs/google-admin1_directory/1.0.12+20190214/google_admin1_directory/struct.GroupGetCall.html), [*insert*](https://docs.rs/google-admin1_directory/1.0.12+20190214/google_admin1_directory/struct.GroupInsertCall.html), [*list*](https://docs.rs/google-admin1_directory/1.0.12+20190214/google_admin1_directory/struct.GroupListCall.html), [*patch*](https://docs.rs/google-admin1_directory/1.0.12+20190214/google_admin1_directory/struct.GroupPatchCall.html) and [*update*](https://docs.rs/google-admin1_directory/1.0.12+20190214/google_admin1_directory/struct.GroupUpdateCall.html)
* [members](https://docs.rs/google-admin1_directory/1.0.12+20190214/google_admin1_directory/struct.Member.html)
 * [*delete*](https://docs.rs/google-admin1_directory/1.0.12+20190214/google_admin1_directory/struct.MemberDeleteCall.html), [*get*](https://docs.rs/google-admin1_directory/1.0.12+20190214/google_admin1_directory/struct.MemberGetCall.html), [*has member*](https://docs.rs/google-admin1_directory/1.0.12+20190214/google_admin1_directory/struct.MemberHasMemberCall.html), [*insert*](https://docs.rs/google-admin1_directory/1.0.12+20190214/google_admin1_directory/struct.MemberInsertCall.html), [*list*](https://docs.rs/google-admin1_directory/1.0.12+20190214/google_admin1_directory/struct.MemberListCall.html), [*patch*](https://docs.rs/google-admin1_directory/1.0.12+20190214/google_admin1_directory/struct.MemberPatchCall.html) and [*update*](https://docs.rs/google-admin1_directory/1.0.12+20190214/google_admin1_directory/struct.MemberUpdateCall.html)
* mobiledevices
 * [*action*](https://docs.rs/google-admin1_directory/1.0.12+20190214/google_admin1_directory/struct.MobiledeviceActionCall.html), [*delete*](https://docs.rs/google-admin1_directory/1.0.12+20190214/google_admin1_directory/struct.MobiledeviceDeleteCall.html), [*get*](https://docs.rs/google-admin1_directory/1.0.12+20190214/google_admin1_directory/struct.MobiledeviceGetCall.html) and [*list*](https://docs.rs/google-admin1_directory/1.0.12+20190214/google_admin1_directory/struct.MobiledeviceListCall.html)
* [notifications](https://docs.rs/google-admin1_directory/1.0.12+20190214/google_admin1_directory/struct.Notification.html)
 * [*delete*](https://docs.rs/google-admin1_directory/1.0.12+20190214/google_admin1_directory/struct.NotificationDeleteCall.html), [*get*](https://docs.rs/google-admin1_directory/1.0.12+20190214/google_admin1_directory/struct.NotificationGetCall.html), [*list*](https://docs.rs/google-admin1_directory/1.0.12+20190214/google_admin1_directory/struct.NotificationListCall.html), [*patch*](https://docs.rs/google-admin1_directory/1.0.12+20190214/google_admin1_directory/struct.NotificationPatchCall.html) and [*update*](https://docs.rs/google-admin1_directory/1.0.12+20190214/google_admin1_directory/struct.NotificationUpdateCall.html)
* orgunits
 * [*delete*](https://docs.rs/google-admin1_directory/1.0.12+20190214/google_admin1_directory/struct.OrgunitDeleteCall.html), [*get*](https://docs.rs/google-admin1_directory/1.0.12+20190214/google_admin1_directory/struct.OrgunitGetCall.html), [*insert*](https://docs.rs/google-admin1_directory/1.0.12+20190214/google_admin1_directory/struct.OrgunitInsertCall.html), [*list*](https://docs.rs/google-admin1_directory/1.0.12+20190214/google_admin1_directory/struct.OrgunitListCall.html), [*patch*](https://docs.rs/google-admin1_directory/1.0.12+20190214/google_admin1_directory/struct.OrgunitPatchCall.html) and [*update*](https://docs.rs/google-admin1_directory/1.0.12+20190214/google_admin1_directory/struct.OrgunitUpdateCall.html)
* [privileges](https://docs.rs/google-admin1_directory/1.0.12+20190214/google_admin1_directory/struct.Privilege.html)
 * [*list*](https://docs.rs/google-admin1_directory/1.0.12+20190214/google_admin1_directory/struct.PrivilegeListCall.html)
* resolved app access settings
 * [*get settings*](https://docs.rs/google-admin1_directory/1.0.12+20190214/google_admin1_directory/struct.ResolvedAppAccessSettingGetSettingCall.html) and [*list trusted apps*](https://docs.rs/google-admin1_directory/1.0.12+20190214/google_admin1_directory/struct.ResolvedAppAccessSettingListTrustedAppCall.html)
* resources
 * [*buildings delete*](https://docs.rs/google-admin1_directory/1.0.12+20190214/google_admin1_directory/struct.ResourceBuildingDeleteCall.html), [*buildings get*](https://docs.rs/google-admin1_directory/1.0.12+20190214/google_admin1_directory/struct.ResourceBuildingGetCall.html), [*buildings insert*](https://docs.rs/google-admin1_directory/1.0.12+20190214/google_admin1_directory/struct.ResourceBuildingInsertCall.html), [*buildings list*](https://docs.rs/google-admin1_directory/1.0.12+20190214/google_admin1_directory/struct.ResourceBuildingListCall.html), [*buildings patch*](https://docs.rs/google-admin1_directory/1.0.12+20190214/google_admin1_directory/struct.ResourceBuildingPatchCall.html), [*buildings update*](https://docs.rs/google-admin1_directory/1.0.12+20190214/google_admin1_directory/struct.ResourceBuildingUpdateCall.html), [*calendars delete*](https://docs.rs/google-admin1_directory/1.0.12+20190214/google_admin1_directory/struct.ResourceCalendarDeleteCall.html), [*calendars get*](https://docs.rs/google-admin1_directory/1.0.12+20190214/google_admin1_directory/struct.ResourceCalendarGetCall.html), [*calendars insert*](https://docs.rs/google-admin1_directory/1.0.12+20190214/google_admin1_directory/struct.ResourceCalendarInsertCall.html), [*calendars list*](https://docs.rs/google-admin1_directory/1.0.12+20190214/google_admin1_directory/struct.ResourceCalendarListCall.html), [*calendars patch*](https://docs.rs/google-admin1_directory/1.0.12+20190214/google_admin1_directory/struct.ResourceCalendarPatchCall.html), [*calendars update*](https://docs.rs/google-admin1_directory/1.0.12+20190214/google_admin1_directory/struct.ResourceCalendarUpdateCall.html), [*features delete*](https://docs.rs/google-admin1_directory/1.0.12+20190214/google_admin1_directory/struct.ResourceFeatureDeleteCall.html), [*features get*](https://docs.rs/google-admin1_directory/1.0.12+20190214/google_admin1_directory/struct.ResourceFeatureGetCall.html), [*features insert*](https://docs.rs/google-admin1_directory/1.0.12+20190214/google_admin1_directory/struct.ResourceFeatureInsertCall.html), [*features list*](https://docs.rs/google-admin1_directory/1.0.12+20190214/google_admin1_directory/struct.ResourceFeatureListCall.html), [*features patch*](https://docs.rs/google-admin1_directory/1.0.12+20190214/google_admin1_directory/struct.ResourceFeaturePatchCall.html), [*features rename*](https://docs.rs/google-admin1_directory/1.0.12+20190214/google_admin1_directory/struct.ResourceFeatureRenameCall.html) and [*features update*](https://docs.rs/google-admin1_directory/1.0.12+20190214/google_admin1_directory/struct.ResourceFeatureUpdateCall.html)
* [role assignments](https://docs.rs/google-admin1_directory/1.0.12+20190214/google_admin1_directory/struct.RoleAssignment.html)
 * [*delete*](https://docs.rs/google-admin1_directory/1.0.12+20190214/google_admin1_directory/struct.RoleAssignmentDeleteCall.html), [*get*](https://docs.rs/google-admin1_directory/1.0.12+20190214/google_admin1_directory/struct.RoleAssignmentGetCall.html), [*insert*](https://docs.rs/google-admin1_directory/1.0.12+20190214/google_admin1_directory/struct.RoleAssignmentInsertCall.html) and [*list*](https://docs.rs/google-admin1_directory/1.0.12+20190214/google_admin1_directory/struct.RoleAssignmentListCall.html)
* [roles](https://docs.rs/google-admin1_directory/1.0.12+20190214/google_admin1_directory/struct.Role.html)
 * [*delete*](https://docs.rs/google-admin1_directory/1.0.12+20190214/google_admin1_directory/struct.RoleDeleteCall.html), [*get*](https://docs.rs/google-admin1_directory/1.0.12+20190214/google_admin1_directory/struct.RoleGetCall.html), [*insert*](https://docs.rs/google-admin1_directory/1.0.12+20190214/google_admin1_directory/struct.RoleInsertCall.html), [*list*](https://docs.rs/google-admin1_directory/1.0.12+20190214/google_admin1_directory/struct.RoleListCall.html), [*patch*](https://docs.rs/google-admin1_directory/1.0.12+20190214/google_admin1_directory/struct.RolePatchCall.html) and [*update*](https://docs.rs/google-admin1_directory/1.0.12+20190214/google_admin1_directory/struct.RoleUpdateCall.html)
* [schemas](https://docs.rs/google-admin1_directory/1.0.12+20190214/google_admin1_directory/struct.Schema.html)
 * [*delete*](https://docs.rs/google-admin1_directory/1.0.12+20190214/google_admin1_directory/struct.SchemaDeleteCall.html), [*get*](https://docs.rs/google-admin1_directory/1.0.12+20190214/google_admin1_directory/struct.SchemaGetCall.html), [*insert*](https://docs.rs/google-admin1_directory/1.0.12+20190214/google_admin1_directory/struct.SchemaInsertCall.html), [*list*](https://docs.rs/google-admin1_directory/1.0.12+20190214/google_admin1_directory/struct.SchemaListCall.html), [*patch*](https://docs.rs/google-admin1_directory/1.0.12+20190214/google_admin1_directory/struct.SchemaPatchCall.html) and [*update*](https://docs.rs/google-admin1_directory/1.0.12+20190214/google_admin1_directory/struct.SchemaUpdateCall.html)
* [tokens](https://docs.rs/google-admin1_directory/1.0.12+20190214/google_admin1_directory/struct.Token.html)
 * [*delete*](https://docs.rs/google-admin1_directory/1.0.12+20190214/google_admin1_directory/struct.TokenDeleteCall.html), [*get*](https://docs.rs/google-admin1_directory/1.0.12+20190214/google_admin1_directory/struct.TokenGetCall.html) and [*list*](https://docs.rs/google-admin1_directory/1.0.12+20190214/google_admin1_directory/struct.TokenListCall.html)
* [users](https://docs.rs/google-admin1_directory/1.0.12+20190214/google_admin1_directory/struct.User.html)
 * [*aliases delete*](https://docs.rs/google-admin1_directory/1.0.12+20190214/google_admin1_directory/struct.UserAliaseDeleteCall.html), [*aliases insert*](https://docs.rs/google-admin1_directory/1.0.12+20190214/google_admin1_directory/struct.UserAliaseInsertCall.html), [*aliases list*](https://docs.rs/google-admin1_directory/1.0.12+20190214/google_admin1_directory/struct.UserAliaseListCall.html), [*aliases watch*](https://docs.rs/google-admin1_directory/1.0.12+20190214/google_admin1_directory/struct.UserAliaseWatchCall.html), [*delete*](https://docs.rs/google-admin1_directory/1.0.12+20190214/google_admin1_directory/struct.UserDeleteCall.html), [*get*](https://docs.rs/google-admin1_directory/1.0.12+20190214/google_admin1_directory/struct.UserGetCall.html), [*insert*](https://docs.rs/google-admin1_directory/1.0.12+20190214/google_admin1_directory/struct.UserInsertCall.html), [*list*](https://docs.rs/google-admin1_directory/1.0.12+20190214/google_admin1_directory/struct.UserListCall.html), [*make admin*](https://docs.rs/google-admin1_directory/1.0.12+20190214/google_admin1_directory/struct.UserMakeAdminCall.html), [*patch*](https://docs.rs/google-admin1_directory/1.0.12+20190214/google_admin1_directory/struct.UserPatchCall.html), [*photos delete*](https://docs.rs/google-admin1_directory/1.0.12+20190214/google_admin1_directory/struct.UserPhotoDeleteCall.html), [*photos get*](https://docs.rs/google-admin1_directory/1.0.12+20190214/google_admin1_directory/struct.UserPhotoGetCall.html), [*photos patch*](https://docs.rs/google-admin1_directory/1.0.12+20190214/google_admin1_directory/struct.UserPhotoPatchCall.html), [*photos update*](https://docs.rs/google-admin1_directory/1.0.12+20190214/google_admin1_directory/struct.UserPhotoUpdateCall.html), [*undelete*](https://docs.rs/google-admin1_directory/1.0.12+20190214/google_admin1_directory/struct.UserUndeleteCall.html), [*update*](https://docs.rs/google-admin1_directory/1.0.12+20190214/google_admin1_directory/struct.UserUpdateCall.html) and [*watch*](https://docs.rs/google-admin1_directory/1.0.12+20190214/google_admin1_directory/struct.UserWatchCall.html)
* [verification codes](https://docs.rs/google-admin1_directory/1.0.12+20190214/google_admin1_directory/struct.VerificationCode.html)
 * [*generate*](https://docs.rs/google-admin1_directory/1.0.12+20190214/google_admin1_directory/struct.VerificationCodeGenerateCall.html), [*invalidate*](https://docs.rs/google-admin1_directory/1.0.12+20190214/google_admin1_directory/struct.VerificationCodeInvalidateCall.html) and [*list*](https://docs.rs/google-admin1_directory/1.0.12+20190214/google_admin1_directory/struct.VerificationCodeListCall.html)


Subscription supported by ...

* [*list users*](https://docs.rs/google-admin1_directory/1.0.12+20190214/google_admin1_directory/struct.UserListCall.html)
* [*aliases watch users*](https://docs.rs/google-admin1_directory/1.0.12+20190214/google_admin1_directory/struct.UserAliaseWatchCall.html)
* [*watch users*](https://docs.rs/google-admin1_directory/1.0.12+20190214/google_admin1_directory/struct.UserWatchCall.html)
* [*aliases list users*](https://docs.rs/google-admin1_directory/1.0.12+20190214/google_admin1_directory/struct.UserAliaseListCall.html)
* [*aliases list groups*](https://docs.rs/google-admin1_directory/1.0.12+20190214/google_admin1_directory/struct.GroupAliaseListCall.html)



# Structure of this Library

The API is structured into the following primary items:

* **[Hub](https://docs.rs/google-admin1_directory/1.0.12+20190214/google_admin1_directory/struct.Directory.html)**
    * a central object to maintain state and allow accessing all *Activities*
    * creates [*Method Builders*](https://docs.rs/google-admin1_directory/1.0.12+20190214/google_admin1_directory/trait.MethodsBuilder.html) which in turn
      allow access to individual [*Call Builders*](https://docs.rs/google-admin1_directory/1.0.12+20190214/google_admin1_directory/trait.CallBuilder.html)
* **[Resources](https://docs.rs/google-admin1_directory/1.0.12+20190214/google_admin1_directory/trait.Resource.html)**
    * primary types that you can apply *Activities* to
    * a collection of properties and *Parts*
    * **[Parts](https://docs.rs/google-admin1_directory/1.0.12+20190214/google_admin1_directory/trait.Part.html)**
        * a collection of properties
        * never directly used in *Activities*
* **[Activities](https://docs.rs/google-admin1_directory/1.0.12+20190214/google_admin1_directory/trait.CallBuilder.html)**
    * operations to apply to *Resources*

All *structures* are marked with applicable traits to further categorize them and ease browsing.

Generally speaking, you can invoke *Activities* like this:

```Rust,ignore
let r = hub.resource().activity(...).doit()
```

Or specifically ...

```ignore
let r = hub.users().photos_patch(...).doit()
let r = hub.users().aliases_delete(...).doit()
let r = hub.users().undelete(...).doit()
let r = hub.users().photos_get(...).doit()
let r = hub.users().update(...).doit()
let r = hub.users().aliases_watch(...).doit()
let r = hub.users().insert(...).doit()
let r = hub.users().photos_delete(...).doit()
let r = hub.users().patch(...).doit()
let r = hub.users().photos_update(...).doit()
let r = hub.users().watch(...).doit()
let r = hub.users().get(...).doit()
let r = hub.users().aliases_insert(...).doit()
let r = hub.users().make_admin(...).doit()
let r = hub.users().aliases_list(...).doit()
let r = hub.users().list(...).doit()
let r = hub.users().delete(...).doit()
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
google-admin1_directory = "*"
# This project intentionally uses an old version of Hyper. See
# https://github.com/Byron/google-apis-rs/issues/173 for more
# information.
hyper = "^0.10"
hyper-rustls = "^0.6"
serde = "^1.0"
serde_json = "^1.0"
yup-oauth2 = "^1.0"
```

## A complete example

```Rust
extern crate hyper;
extern crate hyper_rustls;
extern crate yup_oauth2 as oauth2;
extern crate google_admin1_directory as admin1_directory;
use admin1_directory::Channel;
use admin1_directory::{Result, Error};
use std::default::Default;
use oauth2::{Authenticator, DefaultAuthenticatorDelegate, ApplicationSecret, MemoryStorage};
use admin1_directory::Directory;

// Get an ApplicationSecret instance by some means. It contains the `client_id` and 
// `client_secret`, among other things.
let secret: ApplicationSecret = Default::default();
// Instantiate the authenticator. It will choose a suitable authentication flow for you, 
// unless you replace  `None` with the desired Flow.
// Provide your own `AuthenticatorDelegate` to adjust the way it operates and get feedback about 
// what's going on. You probably want to bring in your own `TokenStorage` to persist tokens and
// retrieve them from storage.
let auth = Authenticator::new(&secret, DefaultAuthenticatorDelegate,
                              hyper::Client::with_connector(hyper::net::HttpsConnector::new(hyper_rustls::TlsClient::new())),
                              <MemoryStorage as Default>::default(), None);
let mut hub = Directory::new(hyper::Client::with_connector(hyper::net::HttpsConnector::new(hyper_rustls::TlsClient::new())), auth);
// As the method needs a request, you would usually fill it with the desired information
// into the respective structure. Some of the parts shown here might not be applicable !
// Values shown here are possibly random and not representative !
let mut req = Channel::default();

// You can configure optional parameters by calling the respective setters at will, and
// execute the final call using `doit()`.
// Values shown here are possibly random and not representative !
let result = hub.users().watch(req)
             .view_type("eirmod")
             .sort_order("sit")
             .show_deleted("Stet")
             .query("sed")
             .projection("et")
             .page_token("dolores")
             .order_by("kasd")
             .max_results(-22)
             .event("takimata")
             .domain("justo")
             .customer("amet.")
             .custom_field_mask("erat")
             .doit();

match result {
    Err(e) => match e {
        // The Error enum provides details about what exactly happened.
        // You can also just use its `Debug`, `Display` or `Error` traits
         Error::HttpError(_)
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

All errors produced by the system are provided either as [Result](https://docs.rs/google-admin1_directory/1.0.12+20190214/google_admin1_directory/enum.Result.html) enumeration as return value of 
the doit() methods, or handed as possibly intermediate results to either the 
[Hub Delegate](https://docs.rs/google-admin1_directory/1.0.12+20190214/google_admin1_directory/trait.Delegate.html), or the [Authenticator Delegate](https://docs.rs/yup-oauth2/*/yup_oauth2/trait.AuthenticatorDelegate.html).

When delegates handle errors or intermediate values, they may have a chance to instruct the system to retry. This 
makes the system potentially resilient to all kinds of errors.

## Uploads and Downloads
If a method supports downloads, the response body, which is part of the [Result](https://docs.rs/google-admin1_directory/1.0.12+20190214/google_admin1_directory/enum.Result.html), should be
read by you to obtain the media.
If such a method also supports a [Response Result](https://docs.rs/google-admin1_directory/1.0.12+20190214/google_admin1_directory/trait.ResponseResult.html), it will return that by default.
You can see it as meta-data for the actual media. To trigger a media download, you will have to set up the builder by making
this call: `.param("alt", "media")`.

Methods supporting uploads can do so using up to 2 different protocols: 
*simple* and *resumable*. The distinctiveness of each is represented by customized 
`doit(...)` methods, which are then named `upload(...)` and `upload_resumable(...)` respectively.

## Customization and Callbacks

You may alter the way an `doit()` method is called by providing a [delegate](https://docs.rs/google-admin1_directory/1.0.12+20190214/google_admin1_directory/trait.Delegate.html) to the 
[Method Builder](https://docs.rs/google-admin1_directory/1.0.12+20190214/google_admin1_directory/trait.CallBuilder.html) before making the final `doit()` call. 
Respective methods will be called to provide progress information, as well as determine whether the system should 
retry on failure.

The [delegate trait](https://docs.rs/google-admin1_directory/1.0.12+20190214/google_admin1_directory/trait.Delegate.html) is default-implemented, allowing you to customize it with minimal effort.

## Optional Parts in Server-Requests

All structures provided by this library are made to be [enocodable](https://docs.rs/google-admin1_directory/1.0.12+20190214/google_admin1_directory/trait.RequestValue.html) and 
[decodable](https://docs.rs/google-admin1_directory/1.0.12+20190214/google_admin1_directory/trait.ResponseResult.html) via *json*. Optionals are used to indicate that partial requests are responses 
are valid.
Most optionals are are considered [Parts](https://docs.rs/google-admin1_directory/1.0.12+20190214/google_admin1_directory/trait.Part.html) which are identifiable by name, which will be sent to 
the server to indicate either the set parts of the request or the desired parts in the response.

## Builder Arguments

Using [method builders](https://docs.rs/google-admin1_directory/1.0.12+20190214/google_admin1_directory/trait.CallBuilder.html), you are able to prepare an action call by repeatedly calling it's methods.
These will always take a single argument, for which the following statements are true.

* [PODs][wiki-pod] are handed by copy
* strings are passed as `&str`
* [request values](https://docs.rs/google-admin1_directory/1.0.12+20190214/google_admin1_directory/trait.RequestValue.html) are moved

Arguments will always be copied or cloned into the builder, to make them independent of their original life times.

[wiki-pod]: http://en.wikipedia.org/wiki/Plain_old_data_structure
[builder-pattern]: http://en.wikipedia.org/wiki/Builder_pattern
[google-go-api]: https://github.com/google/google-api-go-client

# License
The **admin1_directory** library was generated by Sebastian Thiel, and is placed 
under the *MIT* license.
You can read the full text at the repository's [license file][repo-license].

[repo-license]: https://github.com/Byron/google-apis-rsblob/master/LICENSE.md
