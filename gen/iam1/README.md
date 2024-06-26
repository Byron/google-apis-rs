<!---
DO NOT EDIT !
This file was generated automatically from 'src/generator/templates/api/README.md.mako'
DO NOT EDIT !
-->
The `google-iam1` library allows access to all features of the *Google Iam* service.

This documentation was generated from *Iam* crate version *5.0.5+20240621*, where *20240621* is the exact revision of the *iam:v1* schema built by the [mako](http://www.makotemplates.org/) code generator *v5.0.5*.

Everything else about the *Iam* *v1* API can be found at the
[official documentation site](https://cloud.google.com/iam/).
# Features

Handle the following *Resources* with ease from the central [hub](https://docs.rs/google-iam1/5.0.5+20240621/google_iam1/Iam) ... 

* iam policies
 * [*lint policy*](https://docs.rs/google-iam1/5.0.5+20240621/google_iam1/api::IamPolicyLintPolicyCall) and [*query auditable services*](https://docs.rs/google-iam1/5.0.5+20240621/google_iam1/api::IamPolicyQueryAuditableServiceCall)
* locations
 * [*workforce pools create*](https://docs.rs/google-iam1/5.0.5+20240621/google_iam1/api::LocationWorkforcePoolCreateCall), [*workforce pools delete*](https://docs.rs/google-iam1/5.0.5+20240621/google_iam1/api::LocationWorkforcePoolDeleteCall), [*workforce pools get*](https://docs.rs/google-iam1/5.0.5+20240621/google_iam1/api::LocationWorkforcePoolGetCall), [*workforce pools get iam policy*](https://docs.rs/google-iam1/5.0.5+20240621/google_iam1/api::LocationWorkforcePoolGetIamPolicyCall), [*workforce pools list*](https://docs.rs/google-iam1/5.0.5+20240621/google_iam1/api::LocationWorkforcePoolListCall), [*workforce pools operations get*](https://docs.rs/google-iam1/5.0.5+20240621/google_iam1/api::LocationWorkforcePoolOperationGetCall), [*workforce pools patch*](https://docs.rs/google-iam1/5.0.5+20240621/google_iam1/api::LocationWorkforcePoolPatchCall), [*workforce pools providers create*](https://docs.rs/google-iam1/5.0.5+20240621/google_iam1/api::LocationWorkforcePoolProviderCreateCall), [*workforce pools providers delete*](https://docs.rs/google-iam1/5.0.5+20240621/google_iam1/api::LocationWorkforcePoolProviderDeleteCall), [*workforce pools providers get*](https://docs.rs/google-iam1/5.0.5+20240621/google_iam1/api::LocationWorkforcePoolProviderGetCall), [*workforce pools providers keys create*](https://docs.rs/google-iam1/5.0.5+20240621/google_iam1/api::LocationWorkforcePoolProviderKeyCreateCall), [*workforce pools providers keys delete*](https://docs.rs/google-iam1/5.0.5+20240621/google_iam1/api::LocationWorkforcePoolProviderKeyDeleteCall), [*workforce pools providers keys get*](https://docs.rs/google-iam1/5.0.5+20240621/google_iam1/api::LocationWorkforcePoolProviderKeyGetCall), [*workforce pools providers keys list*](https://docs.rs/google-iam1/5.0.5+20240621/google_iam1/api::LocationWorkforcePoolProviderKeyListCall), [*workforce pools providers keys operations get*](https://docs.rs/google-iam1/5.0.5+20240621/google_iam1/api::LocationWorkforcePoolProviderKeyOperationGetCall), [*workforce pools providers keys undelete*](https://docs.rs/google-iam1/5.0.5+20240621/google_iam1/api::LocationWorkforcePoolProviderKeyUndeleteCall), [*workforce pools providers list*](https://docs.rs/google-iam1/5.0.5+20240621/google_iam1/api::LocationWorkforcePoolProviderListCall), [*workforce pools providers operations get*](https://docs.rs/google-iam1/5.0.5+20240621/google_iam1/api::LocationWorkforcePoolProviderOperationGetCall), [*workforce pools providers patch*](https://docs.rs/google-iam1/5.0.5+20240621/google_iam1/api::LocationWorkforcePoolProviderPatchCall), [*workforce pools providers undelete*](https://docs.rs/google-iam1/5.0.5+20240621/google_iam1/api::LocationWorkforcePoolProviderUndeleteCall), [*workforce pools set iam policy*](https://docs.rs/google-iam1/5.0.5+20240621/google_iam1/api::LocationWorkforcePoolSetIamPolicyCall), [*workforce pools subjects delete*](https://docs.rs/google-iam1/5.0.5+20240621/google_iam1/api::LocationWorkforcePoolSubjectDeleteCall), [*workforce pools subjects operations get*](https://docs.rs/google-iam1/5.0.5+20240621/google_iam1/api::LocationWorkforcePoolSubjectOperationGetCall), [*workforce pools subjects undelete*](https://docs.rs/google-iam1/5.0.5+20240621/google_iam1/api::LocationWorkforcePoolSubjectUndeleteCall), [*workforce pools test iam permissions*](https://docs.rs/google-iam1/5.0.5+20240621/google_iam1/api::LocationWorkforcePoolTestIamPermissionCall) and [*workforce pools undelete*](https://docs.rs/google-iam1/5.0.5+20240621/google_iam1/api::LocationWorkforcePoolUndeleteCall)
* organizations
 * [*roles create*](https://docs.rs/google-iam1/5.0.5+20240621/google_iam1/api::OrganizationRoleCreateCall), [*roles delete*](https://docs.rs/google-iam1/5.0.5+20240621/google_iam1/api::OrganizationRoleDeleteCall), [*roles get*](https://docs.rs/google-iam1/5.0.5+20240621/google_iam1/api::OrganizationRoleGetCall), [*roles list*](https://docs.rs/google-iam1/5.0.5+20240621/google_iam1/api::OrganizationRoleListCall), [*roles patch*](https://docs.rs/google-iam1/5.0.5+20240621/google_iam1/api::OrganizationRolePatchCall) and [*roles undelete*](https://docs.rs/google-iam1/5.0.5+20240621/google_iam1/api::OrganizationRoleUndeleteCall)
* [permissions](https://docs.rs/google-iam1/5.0.5+20240621/google_iam1/api::Permission)
 * [*query testable permissions*](https://docs.rs/google-iam1/5.0.5+20240621/google_iam1/api::PermissionQueryTestablePermissionCall)
* projects
 * [*locations oauth clients create*](https://docs.rs/google-iam1/5.0.5+20240621/google_iam1/api::ProjectLocationOauthClientCreateCall), [*locations oauth clients credentials create*](https://docs.rs/google-iam1/5.0.5+20240621/google_iam1/api::ProjectLocationOauthClientCredentialCreateCall), [*locations oauth clients credentials delete*](https://docs.rs/google-iam1/5.0.5+20240621/google_iam1/api::ProjectLocationOauthClientCredentialDeleteCall), [*locations oauth clients credentials get*](https://docs.rs/google-iam1/5.0.5+20240621/google_iam1/api::ProjectLocationOauthClientCredentialGetCall), [*locations oauth clients credentials list*](https://docs.rs/google-iam1/5.0.5+20240621/google_iam1/api::ProjectLocationOauthClientCredentialListCall), [*locations oauth clients credentials patch*](https://docs.rs/google-iam1/5.0.5+20240621/google_iam1/api::ProjectLocationOauthClientCredentialPatchCall), [*locations oauth clients delete*](https://docs.rs/google-iam1/5.0.5+20240621/google_iam1/api::ProjectLocationOauthClientDeleteCall), [*locations oauth clients get*](https://docs.rs/google-iam1/5.0.5+20240621/google_iam1/api::ProjectLocationOauthClientGetCall), [*locations oauth clients list*](https://docs.rs/google-iam1/5.0.5+20240621/google_iam1/api::ProjectLocationOauthClientListCall), [*locations oauth clients patch*](https://docs.rs/google-iam1/5.0.5+20240621/google_iam1/api::ProjectLocationOauthClientPatchCall), [*locations oauth clients undelete*](https://docs.rs/google-iam1/5.0.5+20240621/google_iam1/api::ProjectLocationOauthClientUndeleteCall), [*locations workload identity pools create*](https://docs.rs/google-iam1/5.0.5+20240621/google_iam1/api::ProjectLocationWorkloadIdentityPoolCreateCall), [*locations workload identity pools delete*](https://docs.rs/google-iam1/5.0.5+20240621/google_iam1/api::ProjectLocationWorkloadIdentityPoolDeleteCall), [*locations workload identity pools get*](https://docs.rs/google-iam1/5.0.5+20240621/google_iam1/api::ProjectLocationWorkloadIdentityPoolGetCall), [*locations workload identity pools list*](https://docs.rs/google-iam1/5.0.5+20240621/google_iam1/api::ProjectLocationWorkloadIdentityPoolListCall), [*locations workload identity pools namespaces managed identities operations get*](https://docs.rs/google-iam1/5.0.5+20240621/google_iam1/api::ProjectLocationWorkloadIdentityPoolNamespaceManagedIdentityOperationGetCall), [*locations workload identity pools namespaces managed identities workload sources operations get*](https://docs.rs/google-iam1/5.0.5+20240621/google_iam1/api::ProjectLocationWorkloadIdentityPoolNamespaceManagedIdentityWorkloadSourceOperationGetCall), [*locations workload identity pools namespaces operations get*](https://docs.rs/google-iam1/5.0.5+20240621/google_iam1/api::ProjectLocationWorkloadIdentityPoolNamespaceOperationGetCall), [*locations workload identity pools operations get*](https://docs.rs/google-iam1/5.0.5+20240621/google_iam1/api::ProjectLocationWorkloadIdentityPoolOperationGetCall), [*locations workload identity pools patch*](https://docs.rs/google-iam1/5.0.5+20240621/google_iam1/api::ProjectLocationWorkloadIdentityPoolPatchCall), [*locations workload identity pools providers create*](https://docs.rs/google-iam1/5.0.5+20240621/google_iam1/api::ProjectLocationWorkloadIdentityPoolProviderCreateCall), [*locations workload identity pools providers delete*](https://docs.rs/google-iam1/5.0.5+20240621/google_iam1/api::ProjectLocationWorkloadIdentityPoolProviderDeleteCall), [*locations workload identity pools providers get*](https://docs.rs/google-iam1/5.0.5+20240621/google_iam1/api::ProjectLocationWorkloadIdentityPoolProviderGetCall), [*locations workload identity pools providers keys create*](https://docs.rs/google-iam1/5.0.5+20240621/google_iam1/api::ProjectLocationWorkloadIdentityPoolProviderKeyCreateCall), [*locations workload identity pools providers keys delete*](https://docs.rs/google-iam1/5.0.5+20240621/google_iam1/api::ProjectLocationWorkloadIdentityPoolProviderKeyDeleteCall), [*locations workload identity pools providers keys get*](https://docs.rs/google-iam1/5.0.5+20240621/google_iam1/api::ProjectLocationWorkloadIdentityPoolProviderKeyGetCall), [*locations workload identity pools providers keys list*](https://docs.rs/google-iam1/5.0.5+20240621/google_iam1/api::ProjectLocationWorkloadIdentityPoolProviderKeyListCall), [*locations workload identity pools providers keys operations get*](https://docs.rs/google-iam1/5.0.5+20240621/google_iam1/api::ProjectLocationWorkloadIdentityPoolProviderKeyOperationGetCall), [*locations workload identity pools providers keys undelete*](https://docs.rs/google-iam1/5.0.5+20240621/google_iam1/api::ProjectLocationWorkloadIdentityPoolProviderKeyUndeleteCall), [*locations workload identity pools providers list*](https://docs.rs/google-iam1/5.0.5+20240621/google_iam1/api::ProjectLocationWorkloadIdentityPoolProviderListCall), [*locations workload identity pools providers operations get*](https://docs.rs/google-iam1/5.0.5+20240621/google_iam1/api::ProjectLocationWorkloadIdentityPoolProviderOperationGetCall), [*locations workload identity pools providers patch*](https://docs.rs/google-iam1/5.0.5+20240621/google_iam1/api::ProjectLocationWorkloadIdentityPoolProviderPatchCall), [*locations workload identity pools providers undelete*](https://docs.rs/google-iam1/5.0.5+20240621/google_iam1/api::ProjectLocationWorkloadIdentityPoolProviderUndeleteCall), [*locations workload identity pools undelete*](https://docs.rs/google-iam1/5.0.5+20240621/google_iam1/api::ProjectLocationWorkloadIdentityPoolUndeleteCall), [*roles create*](https://docs.rs/google-iam1/5.0.5+20240621/google_iam1/api::ProjectRoleCreateCall), [*roles delete*](https://docs.rs/google-iam1/5.0.5+20240621/google_iam1/api::ProjectRoleDeleteCall), [*roles get*](https://docs.rs/google-iam1/5.0.5+20240621/google_iam1/api::ProjectRoleGetCall), [*roles list*](https://docs.rs/google-iam1/5.0.5+20240621/google_iam1/api::ProjectRoleListCall), [*roles patch*](https://docs.rs/google-iam1/5.0.5+20240621/google_iam1/api::ProjectRolePatchCall), [*roles undelete*](https://docs.rs/google-iam1/5.0.5+20240621/google_iam1/api::ProjectRoleUndeleteCall), [*service accounts create*](https://docs.rs/google-iam1/5.0.5+20240621/google_iam1/api::ProjectServiceAccountCreateCall), [*service accounts delete*](https://docs.rs/google-iam1/5.0.5+20240621/google_iam1/api::ProjectServiceAccountDeleteCall), [*service accounts disable*](https://docs.rs/google-iam1/5.0.5+20240621/google_iam1/api::ProjectServiceAccountDisableCall), [*service accounts enable*](https://docs.rs/google-iam1/5.0.5+20240621/google_iam1/api::ProjectServiceAccountEnableCall), [*service accounts get*](https://docs.rs/google-iam1/5.0.5+20240621/google_iam1/api::ProjectServiceAccountGetCall), [*service accounts get iam policy*](https://docs.rs/google-iam1/5.0.5+20240621/google_iam1/api::ProjectServiceAccountGetIamPolicyCall), [*service accounts keys create*](https://docs.rs/google-iam1/5.0.5+20240621/google_iam1/api::ProjectServiceAccountKeyCreateCall), [*service accounts keys delete*](https://docs.rs/google-iam1/5.0.5+20240621/google_iam1/api::ProjectServiceAccountKeyDeleteCall), [*service accounts keys disable*](https://docs.rs/google-iam1/5.0.5+20240621/google_iam1/api::ProjectServiceAccountKeyDisableCall), [*service accounts keys enable*](https://docs.rs/google-iam1/5.0.5+20240621/google_iam1/api::ProjectServiceAccountKeyEnableCall), [*service accounts keys get*](https://docs.rs/google-iam1/5.0.5+20240621/google_iam1/api::ProjectServiceAccountKeyGetCall), [*service accounts keys list*](https://docs.rs/google-iam1/5.0.5+20240621/google_iam1/api::ProjectServiceAccountKeyListCall), [*service accounts keys patch*](https://docs.rs/google-iam1/5.0.5+20240621/google_iam1/api::ProjectServiceAccountKeyPatchCall), [*service accounts keys upload*](https://docs.rs/google-iam1/5.0.5+20240621/google_iam1/api::ProjectServiceAccountKeyUploadCall), [*service accounts list*](https://docs.rs/google-iam1/5.0.5+20240621/google_iam1/api::ProjectServiceAccountListCall), [*service accounts patch*](https://docs.rs/google-iam1/5.0.5+20240621/google_iam1/api::ProjectServiceAccountPatchCall), [*service accounts set iam policy*](https://docs.rs/google-iam1/5.0.5+20240621/google_iam1/api::ProjectServiceAccountSetIamPolicyCall), [*service accounts sign blob*](https://docs.rs/google-iam1/5.0.5+20240621/google_iam1/api::ProjectServiceAccountSignBlobCall), [*service accounts sign jwt*](https://docs.rs/google-iam1/5.0.5+20240621/google_iam1/api::ProjectServiceAccountSignJwtCall), [*service accounts test iam permissions*](https://docs.rs/google-iam1/5.0.5+20240621/google_iam1/api::ProjectServiceAccountTestIamPermissionCall), [*service accounts undelete*](https://docs.rs/google-iam1/5.0.5+20240621/google_iam1/api::ProjectServiceAccountUndeleteCall) and [*service accounts update*](https://docs.rs/google-iam1/5.0.5+20240621/google_iam1/api::ProjectServiceAccountUpdateCall)
* [roles](https://docs.rs/google-iam1/5.0.5+20240621/google_iam1/api::Role)
 * [*get*](https://docs.rs/google-iam1/5.0.5+20240621/google_iam1/api::RoleGetCall), [*list*](https://docs.rs/google-iam1/5.0.5+20240621/google_iam1/api::RoleListCall) and [*query grantable roles*](https://docs.rs/google-iam1/5.0.5+20240621/google_iam1/api::RoleQueryGrantableRoleCall)




# Structure of this Library

The API is structured into the following primary items:

* **[Hub](https://docs.rs/google-iam1/5.0.5+20240621/google_iam1/Iam)**
    * a central object to maintain state and allow accessing all *Activities*
    * creates [*Method Builders*](https://docs.rs/google-iam1/5.0.5+20240621/google_iam1/client::MethodsBuilder) which in turn
      allow access to individual [*Call Builders*](https://docs.rs/google-iam1/5.0.5+20240621/google_iam1/client::CallBuilder)
* **[Resources](https://docs.rs/google-iam1/5.0.5+20240621/google_iam1/client::Resource)**
    * primary types that you can apply *Activities* to
    * a collection of properties and *Parts*
    * **[Parts](https://docs.rs/google-iam1/5.0.5+20240621/google_iam1/client::Part)**
        * a collection of properties
        * never directly used in *Activities*
* **[Activities](https://docs.rs/google-iam1/5.0.5+20240621/google_iam1/client::CallBuilder)**
    * operations to apply to *Resources*

All *structures* are marked with applicable traits to further categorize them and ease browsing.

Generally speaking, you can invoke *Activities* like this:

```Rust,ignore
let r = hub.resource().activity(...).doit().await
```

Or specifically ...

```ignore
let r = hub.locations().workforce_pools_operations_get(...).doit().await
let r = hub.locations().workforce_pools_providers_keys_operations_get(...).doit().await
let r = hub.locations().workforce_pools_providers_keys_create(...).doit().await
let r = hub.locations().workforce_pools_providers_keys_delete(...).doit().await
let r = hub.locations().workforce_pools_providers_keys_undelete(...).doit().await
let r = hub.locations().workforce_pools_providers_operations_get(...).doit().await
let r = hub.locations().workforce_pools_providers_create(...).doit().await
let r = hub.locations().workforce_pools_providers_delete(...).doit().await
let r = hub.locations().workforce_pools_providers_patch(...).doit().await
let r = hub.locations().workforce_pools_providers_undelete(...).doit().await
let r = hub.locations().workforce_pools_subjects_operations_get(...).doit().await
let r = hub.locations().workforce_pools_subjects_delete(...).doit().await
let r = hub.locations().workforce_pools_subjects_undelete(...).doit().await
let r = hub.locations().workforce_pools_create(...).doit().await
let r = hub.locations().workforce_pools_delete(...).doit().await
let r = hub.locations().workforce_pools_patch(...).doit().await
let r = hub.locations().workforce_pools_undelete(...).doit().await
let r = hub.projects().locations_workload_identity_pools_namespaces_managed_identities_operations_get(...).doit().await
let r = hub.projects().locations_workload_identity_pools_namespaces_managed_identities_workload_sources_operations_get(...).doit().await
let r = hub.projects().locations_workload_identity_pools_namespaces_operations_get(...).doit().await
let r = hub.projects().locations_workload_identity_pools_operations_get(...).doit().await
let r = hub.projects().locations_workload_identity_pools_providers_keys_operations_get(...).doit().await
let r = hub.projects().locations_workload_identity_pools_providers_keys_create(...).doit().await
let r = hub.projects().locations_workload_identity_pools_providers_keys_delete(...).doit().await
let r = hub.projects().locations_workload_identity_pools_providers_keys_undelete(...).doit().await
let r = hub.projects().locations_workload_identity_pools_providers_operations_get(...).doit().await
let r = hub.projects().locations_workload_identity_pools_providers_create(...).doit().await
let r = hub.projects().locations_workload_identity_pools_providers_delete(...).doit().await
let r = hub.projects().locations_workload_identity_pools_providers_patch(...).doit().await
let r = hub.projects().locations_workload_identity_pools_providers_undelete(...).doit().await
let r = hub.projects().locations_workload_identity_pools_create(...).doit().await
let r = hub.projects().locations_workload_identity_pools_delete(...).doit().await
let r = hub.projects().locations_workload_identity_pools_patch(...).doit().await
let r = hub.projects().locations_workload_identity_pools_undelete(...).doit().await
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
serde = "^1.0"
serde_json = "^1.0"
```

## A complete example

```Rust
extern crate hyper;
extern crate hyper_rustls;
extern crate google_iam1 as iam1;
use iam1::api::WorkforcePoolProviderKey;
use iam1::{Result, Error};
use std::default::Default;
use iam1::{Iam, oauth2, hyper, hyper_rustls, chrono, FieldMask};

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
let mut hub = Iam::new(hyper::Client::builder().build(hyper_rustls::HttpsConnectorBuilder::new().with_native_roots().unwrap().https_or_http().enable_http1().build()), auth);
// As the method needs a request, you would usually fill it with the desired information
// into the respective structure. Some of the parts shown here might not be applicable !
// Values shown here are possibly random and not representative !
let mut req = WorkforcePoolProviderKey::default();

// You can configure optional parameters by calling the respective setters at will, and
// execute the final call using `doit()`.
// Values shown here are possibly random and not representative !
let result = hub.locations().workforce_pools_providers_keys_create(req, "parent")
             .workforce_pool_provider_key_id("magna")
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

All errors produced by the system are provided either as [Result](https://docs.rs/google-iam1/5.0.5+20240621/google_iam1/client::Result) enumeration as return value of
the doit() methods, or handed as possibly intermediate results to either the 
[Hub Delegate](https://docs.rs/google-iam1/5.0.5+20240621/google_iam1/client::Delegate), or the [Authenticator Delegate](https://docs.rs/yup-oauth2/*/yup_oauth2/trait.AuthenticatorDelegate.html).

When delegates handle errors or intermediate values, they may have a chance to instruct the system to retry. This 
makes the system potentially resilient to all kinds of errors.

## Uploads and Downloads
If a method supports downloads, the response body, which is part of the [Result](https://docs.rs/google-iam1/5.0.5+20240621/google_iam1/client::Result), should be
read by you to obtain the media.
If such a method also supports a [Response Result](https://docs.rs/google-iam1/5.0.5+20240621/google_iam1/client::ResponseResult), it will return that by default.
You can see it as meta-data for the actual media. To trigger a media download, you will have to set up the builder by making
this call: `.param("alt", "media")`.

Methods supporting uploads can do so using up to 2 different protocols: 
*simple* and *resumable*. The distinctiveness of each is represented by customized 
`doit(...)` methods, which are then named `upload(...)` and `upload_resumable(...)` respectively.

## Customization and Callbacks

You may alter the way an `doit()` method is called by providing a [delegate](https://docs.rs/google-iam1/5.0.5+20240621/google_iam1/client::Delegate) to the 
[Method Builder](https://docs.rs/google-iam1/5.0.5+20240621/google_iam1/client::CallBuilder) before making the final `doit()` call. 
Respective methods will be called to provide progress information, as well as determine whether the system should 
retry on failure.

The [delegate trait](https://docs.rs/google-iam1/5.0.5+20240621/google_iam1/client::Delegate) is default-implemented, allowing you to customize it with minimal effort.

## Optional Parts in Server-Requests

All structures provided by this library are made to be [encodable](https://docs.rs/google-iam1/5.0.5+20240621/google_iam1/client::RequestValue) and 
[decodable](https://docs.rs/google-iam1/5.0.5+20240621/google_iam1/client::ResponseResult) via *json*. Optionals are used to indicate that partial requests are responses 
are valid.
Most optionals are are considered [Parts](https://docs.rs/google-iam1/5.0.5+20240621/google_iam1/client::Part) which are identifiable by name, which will be sent to 
the server to indicate either the set parts of the request or the desired parts in the response.

## Builder Arguments

Using [method builders](https://docs.rs/google-iam1/5.0.5+20240621/google_iam1/client::CallBuilder), you are able to prepare an action call by repeatedly calling it's methods.
These will always take a single argument, for which the following statements are true.

* [PODs][wiki-pod] are handed by copy
* strings are passed as `&str`
* [request values](https://docs.rs/google-iam1/5.0.5+20240621/google_iam1/client::RequestValue) are moved

Arguments will always be copied or cloned into the builder, to make them independent of their original life times.

[wiki-pod]: http://en.wikipedia.org/wiki/Plain_old_data_structure
[builder-pattern]: http://en.wikipedia.org/wiki/Builder_pattern
[google-go-api]: https://github.com/google/google-api-go-client

## Cargo Features

* `utoipa` - Add support for [utoipa](https://crates.io/crates/utoipa) and derive `utoipa::ToSchema` on all
the types. You'll have to import and register the required types in `#[openapi(schemas(...))]`, otherwise the
generated `openapi` spec would be invalid.


# License
The **iam1** library was generated by Sebastian Thiel, and is placed 
under the *MIT* license.
You can read the full text at the repository's [license file][repo-license].

[repo-license]: https://github.com/Byron/google-apis-rsblob/main/LICENSE.md

