<!---
DO NOT EDIT !
This file was generated automatically from 'src/mako/api/README.md.mako'
DO NOT EDIT !
-->
The `google-iam1` library allows access to all features of the *Google iam* service.

This documentation was generated from *iam* crate version *1.0.14+20200617*, where *20200617* is the exact revision of the *iam:v1* schema built by the [mako](http://www.makotemplates.org/) code generator *v1.0.14*.

Everything else about the *iam* *v1* API can be found at the
[official documentation site](https://cloud.google.com/iam/).
# Features

Handle the following *Resources* with ease from the central [hub](https://docs.rs/google-iam1/1.0.14+20200617/google_iam1/struct.Iam.html) ... 

* iam policies
 * [*lint policy*](https://docs.rs/google-iam1/1.0.14+20200617/google_iam1/struct.IamPolicyLintPolicyCall.html) and [*query auditable services*](https://docs.rs/google-iam1/1.0.14+20200617/google_iam1/struct.IamPolicyQueryAuditableServiceCall.html)
* organizations
 * [*roles create*](https://docs.rs/google-iam1/1.0.14+20200617/google_iam1/struct.OrganizationRoleCreateCall.html), [*roles delete*](https://docs.rs/google-iam1/1.0.14+20200617/google_iam1/struct.OrganizationRoleDeleteCall.html), [*roles get*](https://docs.rs/google-iam1/1.0.14+20200617/google_iam1/struct.OrganizationRoleGetCall.html), [*roles list*](https://docs.rs/google-iam1/1.0.14+20200617/google_iam1/struct.OrganizationRoleListCall.html), [*roles patch*](https://docs.rs/google-iam1/1.0.14+20200617/google_iam1/struct.OrganizationRolePatchCall.html) and [*roles undelete*](https://docs.rs/google-iam1/1.0.14+20200617/google_iam1/struct.OrganizationRoleUndeleteCall.html)
* [permissions](https://docs.rs/google-iam1/1.0.14+20200617/google_iam1/struct.Permission.html)
 * [*query testable permissions*](https://docs.rs/google-iam1/1.0.14+20200617/google_iam1/struct.PermissionQueryTestablePermissionCall.html)
* projects
 * [*roles create*](https://docs.rs/google-iam1/1.0.14+20200617/google_iam1/struct.ProjectRoleCreateCall.html), [*roles delete*](https://docs.rs/google-iam1/1.0.14+20200617/google_iam1/struct.ProjectRoleDeleteCall.html), [*roles get*](https://docs.rs/google-iam1/1.0.14+20200617/google_iam1/struct.ProjectRoleGetCall.html), [*roles list*](https://docs.rs/google-iam1/1.0.14+20200617/google_iam1/struct.ProjectRoleListCall.html), [*roles patch*](https://docs.rs/google-iam1/1.0.14+20200617/google_iam1/struct.ProjectRolePatchCall.html), [*roles undelete*](https://docs.rs/google-iam1/1.0.14+20200617/google_iam1/struct.ProjectRoleUndeleteCall.html), [*service accounts create*](https://docs.rs/google-iam1/1.0.14+20200617/google_iam1/struct.ProjectServiceAccountCreateCall.html), [*service accounts delete*](https://docs.rs/google-iam1/1.0.14+20200617/google_iam1/struct.ProjectServiceAccountDeleteCall.html), [*service accounts disable*](https://docs.rs/google-iam1/1.0.14+20200617/google_iam1/struct.ProjectServiceAccountDisableCall.html), [*service accounts enable*](https://docs.rs/google-iam1/1.0.14+20200617/google_iam1/struct.ProjectServiceAccountEnableCall.html), [*service accounts get*](https://docs.rs/google-iam1/1.0.14+20200617/google_iam1/struct.ProjectServiceAccountGetCall.html), [*service accounts get iam policy*](https://docs.rs/google-iam1/1.0.14+20200617/google_iam1/struct.ProjectServiceAccountGetIamPolicyCall.html), [*service accounts keys create*](https://docs.rs/google-iam1/1.0.14+20200617/google_iam1/struct.ProjectServiceAccountKeyCreateCall.html), [*service accounts keys delete*](https://docs.rs/google-iam1/1.0.14+20200617/google_iam1/struct.ProjectServiceAccountKeyDeleteCall.html), [*service accounts keys get*](https://docs.rs/google-iam1/1.0.14+20200617/google_iam1/struct.ProjectServiceAccountKeyGetCall.html), [*service accounts keys list*](https://docs.rs/google-iam1/1.0.14+20200617/google_iam1/struct.ProjectServiceAccountKeyListCall.html), [*service accounts keys upload*](https://docs.rs/google-iam1/1.0.14+20200617/google_iam1/struct.ProjectServiceAccountKeyUploadCall.html), [*service accounts list*](https://docs.rs/google-iam1/1.0.14+20200617/google_iam1/struct.ProjectServiceAccountListCall.html), [*service accounts patch*](https://docs.rs/google-iam1/1.0.14+20200617/google_iam1/struct.ProjectServiceAccountPatchCall.html), [*service accounts set iam policy*](https://docs.rs/google-iam1/1.0.14+20200617/google_iam1/struct.ProjectServiceAccountSetIamPolicyCall.html), [*service accounts sign blob*](https://docs.rs/google-iam1/1.0.14+20200617/google_iam1/struct.ProjectServiceAccountSignBlobCall.html), [*service accounts sign jwt*](https://docs.rs/google-iam1/1.0.14+20200617/google_iam1/struct.ProjectServiceAccountSignJwtCall.html), [*service accounts test iam permissions*](https://docs.rs/google-iam1/1.0.14+20200617/google_iam1/struct.ProjectServiceAccountTestIamPermissionCall.html), [*service accounts undelete*](https://docs.rs/google-iam1/1.0.14+20200617/google_iam1/struct.ProjectServiceAccountUndeleteCall.html) and [*service accounts update*](https://docs.rs/google-iam1/1.0.14+20200617/google_iam1/struct.ProjectServiceAccountUpdateCall.html)
* [roles](https://docs.rs/google-iam1/1.0.14+20200617/google_iam1/struct.Role.html)
 * [*get*](https://docs.rs/google-iam1/1.0.14+20200617/google_iam1/struct.RoleGetCall.html), [*list*](https://docs.rs/google-iam1/1.0.14+20200617/google_iam1/struct.RoleListCall.html) and [*query grantable roles*](https://docs.rs/google-iam1/1.0.14+20200617/google_iam1/struct.RoleQueryGrantableRoleCall.html)




# Structure of this Library

The API is structured into the following primary items:

* **[Hub](https://docs.rs/google-iam1/1.0.14+20200617/google_iam1/struct.Iam.html)**
    * a central object to maintain state and allow accessing all *Activities*
    * creates [*Method Builders*](https://docs.rs/google-iam1/1.0.14+20200617/google_iam1/trait.MethodsBuilder.html) which in turn
      allow access to individual [*Call Builders*](https://docs.rs/google-iam1/1.0.14+20200617/google_iam1/trait.CallBuilder.html)
* **[Resources](https://docs.rs/google-iam1/1.0.14+20200617/google_iam1/trait.Resource.html)**
    * primary types that you can apply *Activities* to
    * a collection of properties and *Parts*
    * **[Parts](https://docs.rs/google-iam1/1.0.14+20200617/google_iam1/trait.Part.html)**
        * a collection of properties
        * never directly used in *Activities*
* **[Activities](https://docs.rs/google-iam1/1.0.14+20200617/google_iam1/trait.CallBuilder.html)**
    * operations to apply to *Resources*

All *structures* are marked with applicable traits to further categorize them and ease browsing.

Generally speaking, you can invoke *Activities* like this:

```Rust,ignore
let r = hub.resource().activity(...).doit()
```

Or specifically ...

```ignore
let r = hub.organizations().roles_get(...).doit()
let r = hub.projects().roles_patch(...).doit()
let r = hub.roles().list(...).doit()
let r = hub.organizations().roles_delete(...).doit()
let r = hub.projects().roles_get(...).doit()
let r = hub.roles().get(...).doit()
let r = hub.projects().roles_delete(...).doit()
let r = hub.projects().roles_create(...).doit()
let r = hub.organizations().roles_undelete(...).doit()
let r = hub.organizations().roles_patch(...).doit()
let r = hub.roles().query_grantable_roles(...).doit()
let r = hub.organizations().roles_create(...).doit()
let r = hub.projects().roles_undelete(...).doit()
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
google-iam1 = "*"
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
extern crate google_iam1 as iam1;
use iam1::{Result, Error};
use std::default::Default;
use oauth2::{Authenticator, DefaultAuthenticatorDelegate, ApplicationSecret, MemoryStorage};
use iam1::Iam;

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
let mut hub = Iam::new(hyper::Client::with_connector(hyper::net::HttpsConnector::new(hyper_rustls::TlsClient::new())), auth);
// You can configure optional parameters by calling the respective setters at will, and
// execute the final call using `doit()`.
// Values shown here are possibly random and not representative !
let result = hub.roles().list()
             .view("eirmod")
             .show_deleted(true)
             .parent("Stet")
             .page_token("sed")
             .page_size(-85)
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

All errors produced by the system are provided either as [Result](https://docs.rs/google-iam1/1.0.14+20200617/google_iam1/enum.Result.html) enumeration as return value of 
the doit() methods, or handed as possibly intermediate results to either the 
[Hub Delegate](https://docs.rs/google-iam1/1.0.14+20200617/google_iam1/trait.Delegate.html), or the [Authenticator Delegate](https://docs.rs/yup-oauth2/*/yup_oauth2/trait.AuthenticatorDelegate.html).

When delegates handle errors or intermediate values, they may have a chance to instruct the system to retry. This 
makes the system potentially resilient to all kinds of errors.

## Uploads and Downloads
If a method supports downloads, the response body, which is part of the [Result](https://docs.rs/google-iam1/1.0.14+20200617/google_iam1/enum.Result.html), should be
read by you to obtain the media.
If such a method also supports a [Response Result](https://docs.rs/google-iam1/1.0.14+20200617/google_iam1/trait.ResponseResult.html), it will return that by default.
You can see it as meta-data for the actual media. To trigger a media download, you will have to set up the builder by making
this call: `.param("alt", "media")`.

Methods supporting uploads can do so using up to 2 different protocols: 
*simple* and *resumable*. The distinctiveness of each is represented by customized 
`doit(...)` methods, which are then named `upload(...)` and `upload_resumable(...)` respectively.

## Customization and Callbacks

You may alter the way an `doit()` method is called by providing a [delegate](https://docs.rs/google-iam1/1.0.14+20200617/google_iam1/trait.Delegate.html) to the 
[Method Builder](https://docs.rs/google-iam1/1.0.14+20200617/google_iam1/trait.CallBuilder.html) before making the final `doit()` call. 
Respective methods will be called to provide progress information, as well as determine whether the system should 
retry on failure.

The [delegate trait](https://docs.rs/google-iam1/1.0.14+20200617/google_iam1/trait.Delegate.html) is default-implemented, allowing you to customize it with minimal effort.

## Optional Parts in Server-Requests

All structures provided by this library are made to be [encodable](https://docs.rs/google-iam1/1.0.14+20200617/google_iam1/trait.RequestValue.html) and 
[decodable](https://docs.rs/google-iam1/1.0.14+20200617/google_iam1/trait.ResponseResult.html) via *json*. Optionals are used to indicate that partial requests are responses 
are valid.
Most optionals are are considered [Parts](https://docs.rs/google-iam1/1.0.14+20200617/google_iam1/trait.Part.html) which are identifiable by name, which will be sent to 
the server to indicate either the set parts of the request or the desired parts in the response.

## Builder Arguments

Using [method builders](https://docs.rs/google-iam1/1.0.14+20200617/google_iam1/trait.CallBuilder.html), you are able to prepare an action call by repeatedly calling it's methods.
These will always take a single argument, for which the following statements are true.

* [PODs][wiki-pod] are handed by copy
* strings are passed as `&str`
* [request values](https://docs.rs/google-iam1/1.0.14+20200617/google_iam1/trait.RequestValue.html) are moved

Arguments will always be copied or cloned into the builder, to make them independent of their original life times.

[wiki-pod]: http://en.wikipedia.org/wiki/Plain_old_data_structure
[builder-pattern]: http://en.wikipedia.org/wiki/Builder_pattern
[google-go-api]: https://github.com/google/google-api-go-client

# License
The **iam1** library was generated by Sebastian Thiel, and is placed 
under the *MIT* license.
You can read the full text at the repository's [license file][repo-license].

[repo-license]: https://github.com/Byron/google-apis-rsblob/master/LICENSE.md
