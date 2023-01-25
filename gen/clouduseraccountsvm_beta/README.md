<!---
DO NOT EDIT !
This file was generated automatically from 'src/generator/templates/api/README.md.mako'
DO NOT EDIT !
-->
The `google-clouduseraccountsvm_beta` library allows access to all features of the *Google Cloud User Accounts* service.

This documentation was generated from *Cloud User Accounts* crate version *5.0.2-beta-1+20160316*, where *20160316* is the exact revision of the *clouduseraccounts:vm_beta* schema built by the [mako](http://www.makotemplates.org/) code generator *v5.0.2-beta-1*.

Everything else about the *Cloud User Accounts* *vm_beta* API can be found at the
[official documentation site](https://cloud.google.com/compute/docs/access/user-accounts/api/latest/).
# Features

Handle the following *Resources* with ease from the central [hub](https://docs.rs/google-clouduseraccountsvm_beta/5.0.2-beta-1+20160316/google_clouduseraccountsvm_beta/CloudUserAccounts) ... 

* global accounts operations
 * [*delete*](https://docs.rs/google-clouduseraccountsvm_beta/5.0.2-beta-1+20160316/google_clouduseraccountsvm_beta/api::GlobalAccountsOperationDeleteCall), [*get*](https://docs.rs/google-clouduseraccountsvm_beta/5.0.2-beta-1+20160316/google_clouduseraccountsvm_beta/api::GlobalAccountsOperationGetCall) and [*list*](https://docs.rs/google-clouduseraccountsvm_beta/5.0.2-beta-1+20160316/google_clouduseraccountsvm_beta/api::GlobalAccountsOperationListCall)
* [groups](https://docs.rs/google-clouduseraccountsvm_beta/5.0.2-beta-1+20160316/google_clouduseraccountsvm_beta/api::Group)
 * [*add member*](https://docs.rs/google-clouduseraccountsvm_beta/5.0.2-beta-1+20160316/google_clouduseraccountsvm_beta/api::GroupAddMemberCall), [*delete*](https://docs.rs/google-clouduseraccountsvm_beta/5.0.2-beta-1+20160316/google_clouduseraccountsvm_beta/api::GroupDeleteCall), [*get*](https://docs.rs/google-clouduseraccountsvm_beta/5.0.2-beta-1+20160316/google_clouduseraccountsvm_beta/api::GroupGetCall), [*insert*](https://docs.rs/google-clouduseraccountsvm_beta/5.0.2-beta-1+20160316/google_clouduseraccountsvm_beta/api::GroupInsertCall), [*list*](https://docs.rs/google-clouduseraccountsvm_beta/5.0.2-beta-1+20160316/google_clouduseraccountsvm_beta/api::GroupListCall) and [*remove member*](https://docs.rs/google-clouduseraccountsvm_beta/5.0.2-beta-1+20160316/google_clouduseraccountsvm_beta/api::GroupRemoveMemberCall)
* linux
 * [*get authorized keys view*](https://docs.rs/google-clouduseraccountsvm_beta/5.0.2-beta-1+20160316/google_clouduseraccountsvm_beta/api::LinuxGetAuthorizedKeysViewCall) and [*get linux account views*](https://docs.rs/google-clouduseraccountsvm_beta/5.0.2-beta-1+20160316/google_clouduseraccountsvm_beta/api::LinuxGetLinuxAccountViewCall)
* [users](https://docs.rs/google-clouduseraccountsvm_beta/5.0.2-beta-1+20160316/google_clouduseraccountsvm_beta/api::User)
 * [*add public key*](https://docs.rs/google-clouduseraccountsvm_beta/5.0.2-beta-1+20160316/google_clouduseraccountsvm_beta/api::UserAddPublicKeyCall), [*delete*](https://docs.rs/google-clouduseraccountsvm_beta/5.0.2-beta-1+20160316/google_clouduseraccountsvm_beta/api::UserDeleteCall), [*get*](https://docs.rs/google-clouduseraccountsvm_beta/5.0.2-beta-1+20160316/google_clouduseraccountsvm_beta/api::UserGetCall), [*insert*](https://docs.rs/google-clouduseraccountsvm_beta/5.0.2-beta-1+20160316/google_clouduseraccountsvm_beta/api::UserInsertCall), [*list*](https://docs.rs/google-clouduseraccountsvm_beta/5.0.2-beta-1+20160316/google_clouduseraccountsvm_beta/api::UserListCall) and [*remove public key*](https://docs.rs/google-clouduseraccountsvm_beta/5.0.2-beta-1+20160316/google_clouduseraccountsvm_beta/api::UserRemovePublicKeyCall)




# Structure of this Library

The API is structured into the following primary items:

* **[Hub](https://docs.rs/google-clouduseraccountsvm_beta/5.0.2-beta-1+20160316/google_clouduseraccountsvm_beta/CloudUserAccounts)**
    * a central object to maintain state and allow accessing all *Activities*
    * creates [*Method Builders*](https://docs.rs/google-clouduseraccountsvm_beta/5.0.2-beta-1+20160316/google_clouduseraccountsvm_beta/client::MethodsBuilder) which in turn
      allow access to individual [*Call Builders*](https://docs.rs/google-clouduseraccountsvm_beta/5.0.2-beta-1+20160316/google_clouduseraccountsvm_beta/client::CallBuilder)
* **[Resources](https://docs.rs/google-clouduseraccountsvm_beta/5.0.2-beta-1+20160316/google_clouduseraccountsvm_beta/client::Resource)**
    * primary types that you can apply *Activities* to
    * a collection of properties and *Parts*
    * **[Parts](https://docs.rs/google-clouduseraccountsvm_beta/5.0.2-beta-1+20160316/google_clouduseraccountsvm_beta/client::Part)**
        * a collection of properties
        * never directly used in *Activities*
* **[Activities](https://docs.rs/google-clouduseraccountsvm_beta/5.0.2-beta-1+20160316/google_clouduseraccountsvm_beta/client::CallBuilder)**
    * operations to apply to *Resources*

All *structures* are marked with applicable traits to further categorize them and ease browsing.

Generally speaking, you can invoke *Activities* like this:

```Rust,ignore
let r = hub.resource().activity(...).doit().await
```

Or specifically ...

```ignore
let r = hub.global_accounts_operations().get(...).doit().await
let r = hub.groups().add_member(...).doit().await
let r = hub.groups().delete(...).doit().await
let r = hub.groups().insert(...).doit().await
let r = hub.groups().remove_member(...).doit().await
let r = hub.users().add_public_key(...).doit().await
let r = hub.users().delete(...).doit().await
let r = hub.users().insert(...).doit().await
let r = hub.users().remove_public_key(...).doit().await
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
google-clouduseraccountsvm_beta = "*"
serde = "^1.0"
serde_json = "^1.0"
```

## A complete example

```Rust
extern crate hyper;
extern crate hyper_rustls;
extern crate google_clouduseraccountsvm_beta as clouduseraccountsvm_beta;
use clouduseraccountsvm_beta::api::GroupsAddMemberRequest;
use clouduseraccountsvm_beta::{Result, Error};
use std::default::Default;
use clouduseraccountsvm_beta::{CloudUserAccounts, oauth2, hyper, hyper_rustls, chrono, FieldMask};

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
let mut hub = CloudUserAccounts::new(hyper::Client::builder().build(hyper_rustls::HttpsConnectorBuilder::new().with_native_roots().https_or_http().enable_http1().enable_http2().build()), auth);
// As the method needs a request, you would usually fill it with the desired information
// into the respective structure. Some of the parts shown here might not be applicable !
// Values shown here are possibly random and not representative !
let mut req = GroupsAddMemberRequest::default();

// You can configure optional parameters by calling the respective setters at will, and
// execute the final call using `doit()`.
// Values shown here are possibly random and not representative !
let result = hub.groups().add_member(req, "project", "groupName")
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

All errors produced by the system are provided either as [Result](https://docs.rs/google-clouduseraccountsvm_beta/5.0.2-beta-1+20160316/google_clouduseraccountsvm_beta/client::Result) enumeration as return value of
the doit() methods, or handed as possibly intermediate results to either the 
[Hub Delegate](https://docs.rs/google-clouduseraccountsvm_beta/5.0.2-beta-1+20160316/google_clouduseraccountsvm_beta/client::Delegate), or the [Authenticator Delegate](https://docs.rs/yup-oauth2/*/yup_oauth2/trait.AuthenticatorDelegate.html).

When delegates handle errors or intermediate values, they may have a chance to instruct the system to retry. This 
makes the system potentially resilient to all kinds of errors.

## Uploads and Downloads
If a method supports downloads, the response body, which is part of the [Result](https://docs.rs/google-clouduseraccountsvm_beta/5.0.2-beta-1+20160316/google_clouduseraccountsvm_beta/client::Result), should be
read by you to obtain the media.
If such a method also supports a [Response Result](https://docs.rs/google-clouduseraccountsvm_beta/5.0.2-beta-1+20160316/google_clouduseraccountsvm_beta/client::ResponseResult), it will return that by default.
You can see it as meta-data for the actual media. To trigger a media download, you will have to set up the builder by making
this call: `.param("alt", "media")`.

Methods supporting uploads can do so using up to 2 different protocols: 
*simple* and *resumable*. The distinctiveness of each is represented by customized 
`doit(...)` methods, which are then named `upload(...)` and `upload_resumable(...)` respectively.

## Customization and Callbacks

You may alter the way an `doit()` method is called by providing a [delegate](https://docs.rs/google-clouduseraccountsvm_beta/5.0.2-beta-1+20160316/google_clouduseraccountsvm_beta/client::Delegate) to the 
[Method Builder](https://docs.rs/google-clouduseraccountsvm_beta/5.0.2-beta-1+20160316/google_clouduseraccountsvm_beta/client::CallBuilder) before making the final `doit()` call. 
Respective methods will be called to provide progress information, as well as determine whether the system should 
retry on failure.

The [delegate trait](https://docs.rs/google-clouduseraccountsvm_beta/5.0.2-beta-1+20160316/google_clouduseraccountsvm_beta/client::Delegate) is default-implemented, allowing you to customize it with minimal effort.

## Optional Parts in Server-Requests

All structures provided by this library are made to be [encodable](https://docs.rs/google-clouduseraccountsvm_beta/5.0.2-beta-1+20160316/google_clouduseraccountsvm_beta/client::RequestValue) and 
[decodable](https://docs.rs/google-clouduseraccountsvm_beta/5.0.2-beta-1+20160316/google_clouduseraccountsvm_beta/client::ResponseResult) via *json*. Optionals are used to indicate that partial requests are responses 
are valid.
Most optionals are are considered [Parts](https://docs.rs/google-clouduseraccountsvm_beta/5.0.2-beta-1+20160316/google_clouduseraccountsvm_beta/client::Part) which are identifiable by name, which will be sent to 
the server to indicate either the set parts of the request or the desired parts in the response.

## Builder Arguments

Using [method builders](https://docs.rs/google-clouduseraccountsvm_beta/5.0.2-beta-1+20160316/google_clouduseraccountsvm_beta/client::CallBuilder), you are able to prepare an action call by repeatedly calling it's methods.
These will always take a single argument, for which the following statements are true.

* [PODs][wiki-pod] are handed by copy
* strings are passed as `&str`
* [request values](https://docs.rs/google-clouduseraccountsvm_beta/5.0.2-beta-1+20160316/google_clouduseraccountsvm_beta/client::RequestValue) are moved

Arguments will always be copied or cloned into the builder, to make them independent of their original life times.

[wiki-pod]: http://en.wikipedia.org/wiki/Plain_old_data_structure
[builder-pattern]: http://en.wikipedia.org/wiki/Builder_pattern
[google-go-api]: https://github.com/google/google-api-go-client

# License
The **clouduseraccountsvm_beta** library was generated by Sebastian Thiel, and is placed 
under the *MIT* license.
You can read the full text at the repository's [license file][repo-license].

[repo-license]: https://github.com/Byron/google-apis-rsblob/main/LICENSE.md

