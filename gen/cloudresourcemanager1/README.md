<!---
DO NOT EDIT !
This file was generated automatically from 'src/mako/api/README.md.mako'
DO NOT EDIT !
-->
The `google-cloudresourcemanager1` library allows access to all features of the *Google Cloud Resource Manager* service.

This documentation was generated from *Cloud Resource Manager* crate version *1.0.12+20190701*, where *20190701* is the exact revision of the *cloudresourcemanager:v1* schema built by the [mako](http://www.makotemplates.org/) code generator *v1.0.12*.

Everything else about the *Cloud Resource Manager* *v1* API can be found at the
[official documentation site](https://cloud.google.com/resource-manager).
# Features

Handle the following *Resources* with ease from the central [hub](https://docs.rs/google-cloudresourcemanager1/1.0.12+20190701/google_cloudresourcemanager1/struct.CloudResourceManager.html) ... 

* folders
 * [*clear org policy*](https://docs.rs/google-cloudresourcemanager1/1.0.12+20190701/google_cloudresourcemanager1/struct.FolderClearOrgPolicyCall.html), [*get effective org policy*](https://docs.rs/google-cloudresourcemanager1/1.0.12+20190701/google_cloudresourcemanager1/struct.FolderGetEffectiveOrgPolicyCall.html), [*get org policy*](https://docs.rs/google-cloudresourcemanager1/1.0.12+20190701/google_cloudresourcemanager1/struct.FolderGetOrgPolicyCall.html), [*list available org policy constraints*](https://docs.rs/google-cloudresourcemanager1/1.0.12+20190701/google_cloudresourcemanager1/struct.FolderListAvailableOrgPolicyConstraintCall.html), [*list org policies*](https://docs.rs/google-cloudresourcemanager1/1.0.12+20190701/google_cloudresourcemanager1/struct.FolderListOrgPolicyCall.html) and [*set org policy*](https://docs.rs/google-cloudresourcemanager1/1.0.12+20190701/google_cloudresourcemanager1/struct.FolderSetOrgPolicyCall.html)
* [liens](https://docs.rs/google-cloudresourcemanager1/1.0.12+20190701/google_cloudresourcemanager1/struct.Lien.html)
 * [*create*](https://docs.rs/google-cloudresourcemanager1/1.0.12+20190701/google_cloudresourcemanager1/struct.LienCreateCall.html), [*delete*](https://docs.rs/google-cloudresourcemanager1/1.0.12+20190701/google_cloudresourcemanager1/struct.LienDeleteCall.html), [*get*](https://docs.rs/google-cloudresourcemanager1/1.0.12+20190701/google_cloudresourcemanager1/struct.LienGetCall.html) and [*list*](https://docs.rs/google-cloudresourcemanager1/1.0.12+20190701/google_cloudresourcemanager1/struct.LienListCall.html)
* [operations](https://docs.rs/google-cloudresourcemanager1/1.0.12+20190701/google_cloudresourcemanager1/struct.Operation.html)
 * [*get*](https://docs.rs/google-cloudresourcemanager1/1.0.12+20190701/google_cloudresourcemanager1/struct.OperationGetCall.html)
* [organizations](https://docs.rs/google-cloudresourcemanager1/1.0.12+20190701/google_cloudresourcemanager1/struct.Organization.html)
 * [*clear org policy*](https://docs.rs/google-cloudresourcemanager1/1.0.12+20190701/google_cloudresourcemanager1/struct.OrganizationClearOrgPolicyCall.html), [*get*](https://docs.rs/google-cloudresourcemanager1/1.0.12+20190701/google_cloudresourcemanager1/struct.OrganizationGetCall.html), [*get effective org policy*](https://docs.rs/google-cloudresourcemanager1/1.0.12+20190701/google_cloudresourcemanager1/struct.OrganizationGetEffectiveOrgPolicyCall.html), [*get iam policy*](https://docs.rs/google-cloudresourcemanager1/1.0.12+20190701/google_cloudresourcemanager1/struct.OrganizationGetIamPolicyCall.html), [*get org policy*](https://docs.rs/google-cloudresourcemanager1/1.0.12+20190701/google_cloudresourcemanager1/struct.OrganizationGetOrgPolicyCall.html), [*list available org policy constraints*](https://docs.rs/google-cloudresourcemanager1/1.0.12+20190701/google_cloudresourcemanager1/struct.OrganizationListAvailableOrgPolicyConstraintCall.html), [*list org policies*](https://docs.rs/google-cloudresourcemanager1/1.0.12+20190701/google_cloudresourcemanager1/struct.OrganizationListOrgPolicyCall.html), [*search*](https://docs.rs/google-cloudresourcemanager1/1.0.12+20190701/google_cloudresourcemanager1/struct.OrganizationSearchCall.html), [*set iam policy*](https://docs.rs/google-cloudresourcemanager1/1.0.12+20190701/google_cloudresourcemanager1/struct.OrganizationSetIamPolicyCall.html), [*set org policy*](https://docs.rs/google-cloudresourcemanager1/1.0.12+20190701/google_cloudresourcemanager1/struct.OrganizationSetOrgPolicyCall.html) and [*test iam permissions*](https://docs.rs/google-cloudresourcemanager1/1.0.12+20190701/google_cloudresourcemanager1/struct.OrganizationTestIamPermissionCall.html)
* [projects](https://docs.rs/google-cloudresourcemanager1/1.0.12+20190701/google_cloudresourcemanager1/struct.Project.html)
 * [*clear org policy*](https://docs.rs/google-cloudresourcemanager1/1.0.12+20190701/google_cloudresourcemanager1/struct.ProjectClearOrgPolicyCall.html), [*create*](https://docs.rs/google-cloudresourcemanager1/1.0.12+20190701/google_cloudresourcemanager1/struct.ProjectCreateCall.html), [*delete*](https://docs.rs/google-cloudresourcemanager1/1.0.12+20190701/google_cloudresourcemanager1/struct.ProjectDeleteCall.html), [*get*](https://docs.rs/google-cloudresourcemanager1/1.0.12+20190701/google_cloudresourcemanager1/struct.ProjectGetCall.html), [*get ancestry*](https://docs.rs/google-cloudresourcemanager1/1.0.12+20190701/google_cloudresourcemanager1/struct.ProjectGetAncestryCall.html), [*get effective org policy*](https://docs.rs/google-cloudresourcemanager1/1.0.12+20190701/google_cloudresourcemanager1/struct.ProjectGetEffectiveOrgPolicyCall.html), [*get iam policy*](https://docs.rs/google-cloudresourcemanager1/1.0.12+20190701/google_cloudresourcemanager1/struct.ProjectGetIamPolicyCall.html), [*get org policy*](https://docs.rs/google-cloudresourcemanager1/1.0.12+20190701/google_cloudresourcemanager1/struct.ProjectGetOrgPolicyCall.html), [*list*](https://docs.rs/google-cloudresourcemanager1/1.0.12+20190701/google_cloudresourcemanager1/struct.ProjectListCall.html), [*list available org policy constraints*](https://docs.rs/google-cloudresourcemanager1/1.0.12+20190701/google_cloudresourcemanager1/struct.ProjectListAvailableOrgPolicyConstraintCall.html), [*list org policies*](https://docs.rs/google-cloudresourcemanager1/1.0.12+20190701/google_cloudresourcemanager1/struct.ProjectListOrgPolicyCall.html), [*set iam policy*](https://docs.rs/google-cloudresourcemanager1/1.0.12+20190701/google_cloudresourcemanager1/struct.ProjectSetIamPolicyCall.html), [*set org policy*](https://docs.rs/google-cloudresourcemanager1/1.0.12+20190701/google_cloudresourcemanager1/struct.ProjectSetOrgPolicyCall.html), [*test iam permissions*](https://docs.rs/google-cloudresourcemanager1/1.0.12+20190701/google_cloudresourcemanager1/struct.ProjectTestIamPermissionCall.html), [*undelete*](https://docs.rs/google-cloudresourcemanager1/1.0.12+20190701/google_cloudresourcemanager1/struct.ProjectUndeleteCall.html) and [*update*](https://docs.rs/google-cloudresourcemanager1/1.0.12+20190701/google_cloudresourcemanager1/struct.ProjectUpdateCall.html)




# Structure of this Library

The API is structured into the following primary items:

* **[Hub](https://docs.rs/google-cloudresourcemanager1/1.0.12+20190701/google_cloudresourcemanager1/struct.CloudResourceManager.html)**
    * a central object to maintain state and allow accessing all *Activities*
    * creates [*Method Builders*](https://docs.rs/google-cloudresourcemanager1/1.0.12+20190701/google_cloudresourcemanager1/trait.MethodsBuilder.html) which in turn
      allow access to individual [*Call Builders*](https://docs.rs/google-cloudresourcemanager1/1.0.12+20190701/google_cloudresourcemanager1/trait.CallBuilder.html)
* **[Resources](https://docs.rs/google-cloudresourcemanager1/1.0.12+20190701/google_cloudresourcemanager1/trait.Resource.html)**
    * primary types that you can apply *Activities* to
    * a collection of properties and *Parts*
    * **[Parts](https://docs.rs/google-cloudresourcemanager1/1.0.12+20190701/google_cloudresourcemanager1/trait.Part.html)**
        * a collection of properties
        * never directly used in *Activities*
* **[Activities](https://docs.rs/google-cloudresourcemanager1/1.0.12+20190701/google_cloudresourcemanager1/trait.CallBuilder.html)**
    * operations to apply to *Resources*

All *structures* are marked with applicable traits to further categorize them and ease browsing.

Generally speaking, you can invoke *Activities* like this:

```Rust,ignore
let r = hub.resource().activity(...).doit()
```

Or specifically ...

```ignore
let r = hub.projects().test_iam_permissions(...).doit()
let r = hub.projects().undelete(...).doit()
let r = hub.projects().list_org_policies(...).doit()
let r = hub.projects().get_effective_org_policy(...).doit()
let r = hub.projects().get(...).doit()
let r = hub.projects().get_ancestry(...).doit()
let r = hub.projects().update(...).doit()
let r = hub.projects().get_iam_policy(...).doit()
let r = hub.projects().delete(...).doit()
let r = hub.projects().list(...).doit()
let r = hub.projects().create(...).doit()
let r = hub.projects().set_iam_policy(...).doit()
let r = hub.projects().get_org_policy(...).doit()
let r = hub.projects().set_org_policy(...).doit()
let r = hub.projects().list_available_org_policy_constraints(...).doit()
let r = hub.projects().clear_org_policy(...).doit()
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
google-cloudresourcemanager1 = "*"
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
extern crate google_cloudresourcemanager1 as cloudresourcemanager1;
use cloudresourcemanager1::{Result, Error};
use std::default::Default;
use oauth2::{Authenticator, DefaultAuthenticatorDelegate, ApplicationSecret, MemoryStorage};
use cloudresourcemanager1::CloudResourceManager;

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
let mut hub = CloudResourceManager::new(hyper::Client::with_connector(hyper::net::HttpsConnector::new(hyper_rustls::TlsClient::new())), auth);
// You can configure optional parameters by calling the respective setters at will, and
// execute the final call using `doit()`.
// Values shown here are possibly random and not representative !
let result = hub.projects().list()
             .page_token("eirmod")
             .page_size(-48)
             .filter("Stet")
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

All errors produced by the system are provided either as [Result](https://docs.rs/google-cloudresourcemanager1/1.0.12+20190701/google_cloudresourcemanager1/enum.Result.html) enumeration as return value of 
the doit() methods, or handed as possibly intermediate results to either the 
[Hub Delegate](https://docs.rs/google-cloudresourcemanager1/1.0.12+20190701/google_cloudresourcemanager1/trait.Delegate.html), or the [Authenticator Delegate](https://docs.rs/yup-oauth2/*/yup_oauth2/trait.AuthenticatorDelegate.html).

When delegates handle errors or intermediate values, they may have a chance to instruct the system to retry. This 
makes the system potentially resilient to all kinds of errors.

## Uploads and Downloads
If a method supports downloads, the response body, which is part of the [Result](https://docs.rs/google-cloudresourcemanager1/1.0.12+20190701/google_cloudresourcemanager1/enum.Result.html), should be
read by you to obtain the media.
If such a method also supports a [Response Result](https://docs.rs/google-cloudresourcemanager1/1.0.12+20190701/google_cloudresourcemanager1/trait.ResponseResult.html), it will return that by default.
You can see it as meta-data for the actual media. To trigger a media download, you will have to set up the builder by making
this call: `.param("alt", "media")`.

Methods supporting uploads can do so using up to 2 different protocols: 
*simple* and *resumable*. The distinctiveness of each is represented by customized 
`doit(...)` methods, which are then named `upload(...)` and `upload_resumable(...)` respectively.

## Customization and Callbacks

You may alter the way an `doit()` method is called by providing a [delegate](https://docs.rs/google-cloudresourcemanager1/1.0.12+20190701/google_cloudresourcemanager1/trait.Delegate.html) to the 
[Method Builder](https://docs.rs/google-cloudresourcemanager1/1.0.12+20190701/google_cloudresourcemanager1/trait.CallBuilder.html) before making the final `doit()` call. 
Respective methods will be called to provide progress information, as well as determine whether the system should 
retry on failure.

The [delegate trait](https://docs.rs/google-cloudresourcemanager1/1.0.12+20190701/google_cloudresourcemanager1/trait.Delegate.html) is default-implemented, allowing you to customize it with minimal effort.

## Optional Parts in Server-Requests

All structures provided by this library are made to be [enocodable](https://docs.rs/google-cloudresourcemanager1/1.0.12+20190701/google_cloudresourcemanager1/trait.RequestValue.html) and 
[decodable](https://docs.rs/google-cloudresourcemanager1/1.0.12+20190701/google_cloudresourcemanager1/trait.ResponseResult.html) via *json*. Optionals are used to indicate that partial requests are responses 
are valid.
Most optionals are are considered [Parts](https://docs.rs/google-cloudresourcemanager1/1.0.12+20190701/google_cloudresourcemanager1/trait.Part.html) which are identifiable by name, which will be sent to 
the server to indicate either the set parts of the request or the desired parts in the response.

## Builder Arguments

Using [method builders](https://docs.rs/google-cloudresourcemanager1/1.0.12+20190701/google_cloudresourcemanager1/trait.CallBuilder.html), you are able to prepare an action call by repeatedly calling it's methods.
These will always take a single argument, for which the following statements are true.

* [PODs][wiki-pod] are handed by copy
* strings are passed as `&str`
* [request values](https://docs.rs/google-cloudresourcemanager1/1.0.12+20190701/google_cloudresourcemanager1/trait.RequestValue.html) are moved

Arguments will always be copied or cloned into the builder, to make them independent of their original life times.

[wiki-pod]: http://en.wikipedia.org/wiki/Plain_old_data_structure
[builder-pattern]: http://en.wikipedia.org/wiki/Builder_pattern
[google-go-api]: https://github.com/google/google-api-go-client

# License
The **cloudresourcemanager1** library was generated by Sebastian Thiel, and is placed 
under the *MIT* license.
You can read the full text at the repository's [license file][repo-license].

[repo-license]: https://github.com/Byron/google-apis-rsblob/master/LICENSE.md
