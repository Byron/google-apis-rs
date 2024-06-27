<!---
DO NOT EDIT !
This file was generated automatically from 'src/generator/templates/api/README.md.mako'
DO NOT EDIT !
-->
The `google-prod_tt_sasportal1_alpha1` library allows access to all features of the *Google SAS Portal Testing* service.

This documentation was generated from *SAS Portal Testing* crate version *5.0.5+20240626*, where *20240626* is the exact revision of the *prod_tt_sasportal:v1alpha1* schema built by the [mako](http://www.makotemplates.org/) code generator *v5.0.5*.

Everything else about the *SAS Portal Testing* *v1_alpha1* API can be found at the
[official documentation site](https://developers.google.com/spectrum-access-system/).
# Features

Handle the following *Resources* with ease from the central [hub](https://docs.rs/google-prod_tt_sasportal1_alpha1/5.0.5+20240626/google_prod_tt_sasportal1_alpha1/SASPortalTesting) ... 

* customers
 * [*deployments create*](https://docs.rs/google-prod_tt_sasportal1_alpha1/5.0.5+20240626/google_prod_tt_sasportal1_alpha1/api::CustomerDeploymentCreateCall), [*deployments delete*](https://docs.rs/google-prod_tt_sasportal1_alpha1/5.0.5+20240626/google_prod_tt_sasportal1_alpha1/api::CustomerDeploymentDeleteCall), [*deployments devices create*](https://docs.rs/google-prod_tt_sasportal1_alpha1/5.0.5+20240626/google_prod_tt_sasportal1_alpha1/api::CustomerDeploymentDeviceCreateCall), [*deployments devices create signed*](https://docs.rs/google-prod_tt_sasportal1_alpha1/5.0.5+20240626/google_prod_tt_sasportal1_alpha1/api::CustomerDeploymentDeviceCreateSignedCall), [*deployments devices list*](https://docs.rs/google-prod_tt_sasportal1_alpha1/5.0.5+20240626/google_prod_tt_sasportal1_alpha1/api::CustomerDeploymentDeviceListCall), [*deployments get*](https://docs.rs/google-prod_tt_sasportal1_alpha1/5.0.5+20240626/google_prod_tt_sasportal1_alpha1/api::CustomerDeploymentGetCall), [*deployments list*](https://docs.rs/google-prod_tt_sasportal1_alpha1/5.0.5+20240626/google_prod_tt_sasportal1_alpha1/api::CustomerDeploymentListCall), [*deployments move*](https://docs.rs/google-prod_tt_sasportal1_alpha1/5.0.5+20240626/google_prod_tt_sasportal1_alpha1/api::CustomerDeploymentMoveCall), [*deployments patch*](https://docs.rs/google-prod_tt_sasportal1_alpha1/5.0.5+20240626/google_prod_tt_sasportal1_alpha1/api::CustomerDeploymentPatchCall), [*devices create*](https://docs.rs/google-prod_tt_sasportal1_alpha1/5.0.5+20240626/google_prod_tt_sasportal1_alpha1/api::CustomerDeviceCreateCall), [*devices create signed*](https://docs.rs/google-prod_tt_sasportal1_alpha1/5.0.5+20240626/google_prod_tt_sasportal1_alpha1/api::CustomerDeviceCreateSignedCall), [*devices delete*](https://docs.rs/google-prod_tt_sasportal1_alpha1/5.0.5+20240626/google_prod_tt_sasportal1_alpha1/api::CustomerDeviceDeleteCall), [*devices get*](https://docs.rs/google-prod_tt_sasportal1_alpha1/5.0.5+20240626/google_prod_tt_sasportal1_alpha1/api::CustomerDeviceGetCall), [*devices list*](https://docs.rs/google-prod_tt_sasportal1_alpha1/5.0.5+20240626/google_prod_tt_sasportal1_alpha1/api::CustomerDeviceListCall), [*devices move*](https://docs.rs/google-prod_tt_sasportal1_alpha1/5.0.5+20240626/google_prod_tt_sasportal1_alpha1/api::CustomerDeviceMoveCall), [*devices patch*](https://docs.rs/google-prod_tt_sasportal1_alpha1/5.0.5+20240626/google_prod_tt_sasportal1_alpha1/api::CustomerDevicePatchCall), [*devices sign device*](https://docs.rs/google-prod_tt_sasportal1_alpha1/5.0.5+20240626/google_prod_tt_sasportal1_alpha1/api::CustomerDeviceSignDeviceCall), [*devices update signed*](https://docs.rs/google-prod_tt_sasportal1_alpha1/5.0.5+20240626/google_prod_tt_sasportal1_alpha1/api::CustomerDeviceUpdateSignedCall), [*get*](https://docs.rs/google-prod_tt_sasportal1_alpha1/5.0.5+20240626/google_prod_tt_sasportal1_alpha1/api::CustomerGetCall), [*list*](https://docs.rs/google-prod_tt_sasportal1_alpha1/5.0.5+20240626/google_prod_tt_sasportal1_alpha1/api::CustomerListCall), [*list gcp project deployments*](https://docs.rs/google-prod_tt_sasportal1_alpha1/5.0.5+20240626/google_prod_tt_sasportal1_alpha1/api::CustomerListGcpProjectDeploymentCall), [*list legacy organizations*](https://docs.rs/google-prod_tt_sasportal1_alpha1/5.0.5+20240626/google_prod_tt_sasportal1_alpha1/api::CustomerListLegacyOrganizationCall), [*migrate organization*](https://docs.rs/google-prod_tt_sasportal1_alpha1/5.0.5+20240626/google_prod_tt_sasportal1_alpha1/api::CustomerMigrateOrganizationCall), [*nodes create*](https://docs.rs/google-prod_tt_sasportal1_alpha1/5.0.5+20240626/google_prod_tt_sasportal1_alpha1/api::CustomerNodeCreateCall), [*nodes delete*](https://docs.rs/google-prod_tt_sasportal1_alpha1/5.0.5+20240626/google_prod_tt_sasportal1_alpha1/api::CustomerNodeDeleteCall), [*nodes deployments create*](https://docs.rs/google-prod_tt_sasportal1_alpha1/5.0.5+20240626/google_prod_tt_sasportal1_alpha1/api::CustomerNodeDeploymentCreateCall), [*nodes deployments list*](https://docs.rs/google-prod_tt_sasportal1_alpha1/5.0.5+20240626/google_prod_tt_sasportal1_alpha1/api::CustomerNodeDeploymentListCall), [*nodes devices create*](https://docs.rs/google-prod_tt_sasportal1_alpha1/5.0.5+20240626/google_prod_tt_sasportal1_alpha1/api::CustomerNodeDeviceCreateCall), [*nodes devices create signed*](https://docs.rs/google-prod_tt_sasportal1_alpha1/5.0.5+20240626/google_prod_tt_sasportal1_alpha1/api::CustomerNodeDeviceCreateSignedCall), [*nodes devices list*](https://docs.rs/google-prod_tt_sasportal1_alpha1/5.0.5+20240626/google_prod_tt_sasportal1_alpha1/api::CustomerNodeDeviceListCall), [*nodes get*](https://docs.rs/google-prod_tt_sasportal1_alpha1/5.0.5+20240626/google_prod_tt_sasportal1_alpha1/api::CustomerNodeGetCall), [*nodes list*](https://docs.rs/google-prod_tt_sasportal1_alpha1/5.0.5+20240626/google_prod_tt_sasportal1_alpha1/api::CustomerNodeListCall), [*nodes move*](https://docs.rs/google-prod_tt_sasportal1_alpha1/5.0.5+20240626/google_prod_tt_sasportal1_alpha1/api::CustomerNodeMoveCall), [*nodes nodes create*](https://docs.rs/google-prod_tt_sasportal1_alpha1/5.0.5+20240626/google_prod_tt_sasportal1_alpha1/api::CustomerNodeNodeCreateCall), [*nodes nodes list*](https://docs.rs/google-prod_tt_sasportal1_alpha1/5.0.5+20240626/google_prod_tt_sasportal1_alpha1/api::CustomerNodeNodeListCall), [*nodes patch*](https://docs.rs/google-prod_tt_sasportal1_alpha1/5.0.5+20240626/google_prod_tt_sasportal1_alpha1/api::CustomerNodePatchCall), [*patch*](https://docs.rs/google-prod_tt_sasportal1_alpha1/5.0.5+20240626/google_prod_tt_sasportal1_alpha1/api::CustomerPatchCall), [*provision deployment*](https://docs.rs/google-prod_tt_sasportal1_alpha1/5.0.5+20240626/google_prod_tt_sasportal1_alpha1/api::CustomerProvisionDeploymentCall) and [*setup sas analytics*](https://docs.rs/google-prod_tt_sasportal1_alpha1/5.0.5+20240626/google_prod_tt_sasportal1_alpha1/api::CustomerSetupSasAnalyticCall)
* deployments
 * [*devices delete*](https://docs.rs/google-prod_tt_sasportal1_alpha1/5.0.5+20240626/google_prod_tt_sasportal1_alpha1/api::DeploymentDeviceDeleteCall), [*devices get*](https://docs.rs/google-prod_tt_sasportal1_alpha1/5.0.5+20240626/google_prod_tt_sasportal1_alpha1/api::DeploymentDeviceGetCall), [*devices move*](https://docs.rs/google-prod_tt_sasportal1_alpha1/5.0.5+20240626/google_prod_tt_sasportal1_alpha1/api::DeploymentDeviceMoveCall), [*devices patch*](https://docs.rs/google-prod_tt_sasportal1_alpha1/5.0.5+20240626/google_prod_tt_sasportal1_alpha1/api::DeploymentDevicePatchCall), [*devices sign device*](https://docs.rs/google-prod_tt_sasportal1_alpha1/5.0.5+20240626/google_prod_tt_sasportal1_alpha1/api::DeploymentDeviceSignDeviceCall), [*devices update signed*](https://docs.rs/google-prod_tt_sasportal1_alpha1/5.0.5+20240626/google_prod_tt_sasportal1_alpha1/api::DeploymentDeviceUpdateSignedCall) and [*get*](https://docs.rs/google-prod_tt_sasportal1_alpha1/5.0.5+20240626/google_prod_tt_sasportal1_alpha1/api::DeploymentGetCall)
* installer
 * [*generate secret*](https://docs.rs/google-prod_tt_sasportal1_alpha1/5.0.5+20240626/google_prod_tt_sasportal1_alpha1/api::InstallerGenerateSecretCall) and [*validate*](https://docs.rs/google-prod_tt_sasportal1_alpha1/5.0.5+20240626/google_prod_tt_sasportal1_alpha1/api::InstallerValidateCall)
* nodes
 * [*deployments delete*](https://docs.rs/google-prod_tt_sasportal1_alpha1/5.0.5+20240626/google_prod_tt_sasportal1_alpha1/api::NodeDeploymentDeleteCall), [*deployments devices create*](https://docs.rs/google-prod_tt_sasportal1_alpha1/5.0.5+20240626/google_prod_tt_sasportal1_alpha1/api::NodeDeploymentDeviceCreateCall), [*deployments devices create signed*](https://docs.rs/google-prod_tt_sasportal1_alpha1/5.0.5+20240626/google_prod_tt_sasportal1_alpha1/api::NodeDeploymentDeviceCreateSignedCall), [*deployments devices list*](https://docs.rs/google-prod_tt_sasportal1_alpha1/5.0.5+20240626/google_prod_tt_sasportal1_alpha1/api::NodeDeploymentDeviceListCall), [*deployments get*](https://docs.rs/google-prod_tt_sasportal1_alpha1/5.0.5+20240626/google_prod_tt_sasportal1_alpha1/api::NodeDeploymentGetCall), [*deployments list*](https://docs.rs/google-prod_tt_sasportal1_alpha1/5.0.5+20240626/google_prod_tt_sasportal1_alpha1/api::NodeDeploymentListCall), [*deployments move*](https://docs.rs/google-prod_tt_sasportal1_alpha1/5.0.5+20240626/google_prod_tt_sasportal1_alpha1/api::NodeDeploymentMoveCall), [*deployments patch*](https://docs.rs/google-prod_tt_sasportal1_alpha1/5.0.5+20240626/google_prod_tt_sasportal1_alpha1/api::NodeDeploymentPatchCall), [*devices create*](https://docs.rs/google-prod_tt_sasportal1_alpha1/5.0.5+20240626/google_prod_tt_sasportal1_alpha1/api::NodeDeviceCreateCall), [*devices create signed*](https://docs.rs/google-prod_tt_sasportal1_alpha1/5.0.5+20240626/google_prod_tt_sasportal1_alpha1/api::NodeDeviceCreateSignedCall), [*devices delete*](https://docs.rs/google-prod_tt_sasportal1_alpha1/5.0.5+20240626/google_prod_tt_sasportal1_alpha1/api::NodeDeviceDeleteCall), [*devices get*](https://docs.rs/google-prod_tt_sasportal1_alpha1/5.0.5+20240626/google_prod_tt_sasportal1_alpha1/api::NodeDeviceGetCall), [*devices list*](https://docs.rs/google-prod_tt_sasportal1_alpha1/5.0.5+20240626/google_prod_tt_sasportal1_alpha1/api::NodeDeviceListCall), [*devices move*](https://docs.rs/google-prod_tt_sasportal1_alpha1/5.0.5+20240626/google_prod_tt_sasportal1_alpha1/api::NodeDeviceMoveCall), [*devices patch*](https://docs.rs/google-prod_tt_sasportal1_alpha1/5.0.5+20240626/google_prod_tt_sasportal1_alpha1/api::NodeDevicePatchCall), [*devices sign device*](https://docs.rs/google-prod_tt_sasportal1_alpha1/5.0.5+20240626/google_prod_tt_sasportal1_alpha1/api::NodeDeviceSignDeviceCall), [*devices update signed*](https://docs.rs/google-prod_tt_sasportal1_alpha1/5.0.5+20240626/google_prod_tt_sasportal1_alpha1/api::NodeDeviceUpdateSignedCall), [*get*](https://docs.rs/google-prod_tt_sasportal1_alpha1/5.0.5+20240626/google_prod_tt_sasportal1_alpha1/api::NodeGetCall), [*nodes create*](https://docs.rs/google-prod_tt_sasportal1_alpha1/5.0.5+20240626/google_prod_tt_sasportal1_alpha1/api::NodeNodeCreateCall), [*nodes delete*](https://docs.rs/google-prod_tt_sasportal1_alpha1/5.0.5+20240626/google_prod_tt_sasportal1_alpha1/api::NodeNodeDeleteCall), [*nodes deployments create*](https://docs.rs/google-prod_tt_sasportal1_alpha1/5.0.5+20240626/google_prod_tt_sasportal1_alpha1/api::NodeNodeDeploymentCreateCall), [*nodes deployments list*](https://docs.rs/google-prod_tt_sasportal1_alpha1/5.0.5+20240626/google_prod_tt_sasportal1_alpha1/api::NodeNodeDeploymentListCall), [*nodes devices create*](https://docs.rs/google-prod_tt_sasportal1_alpha1/5.0.5+20240626/google_prod_tt_sasportal1_alpha1/api::NodeNodeDeviceCreateCall), [*nodes devices create signed*](https://docs.rs/google-prod_tt_sasportal1_alpha1/5.0.5+20240626/google_prod_tt_sasportal1_alpha1/api::NodeNodeDeviceCreateSignedCall), [*nodes devices list*](https://docs.rs/google-prod_tt_sasportal1_alpha1/5.0.5+20240626/google_prod_tt_sasportal1_alpha1/api::NodeNodeDeviceListCall), [*nodes get*](https://docs.rs/google-prod_tt_sasportal1_alpha1/5.0.5+20240626/google_prod_tt_sasportal1_alpha1/api::NodeNodeGetCall), [*nodes list*](https://docs.rs/google-prod_tt_sasportal1_alpha1/5.0.5+20240626/google_prod_tt_sasportal1_alpha1/api::NodeNodeListCall), [*nodes move*](https://docs.rs/google-prod_tt_sasportal1_alpha1/5.0.5+20240626/google_prod_tt_sasportal1_alpha1/api::NodeNodeMoveCall), [*nodes nodes create*](https://docs.rs/google-prod_tt_sasportal1_alpha1/5.0.5+20240626/google_prod_tt_sasportal1_alpha1/api::NodeNodeNodeCreateCall), [*nodes nodes list*](https://docs.rs/google-prod_tt_sasportal1_alpha1/5.0.5+20240626/google_prod_tt_sasportal1_alpha1/api::NodeNodeNodeListCall) and [*nodes patch*](https://docs.rs/google-prod_tt_sasportal1_alpha1/5.0.5+20240626/google_prod_tt_sasportal1_alpha1/api::NodeNodePatchCall)
* policies
 * [*get*](https://docs.rs/google-prod_tt_sasportal1_alpha1/5.0.5+20240626/google_prod_tt_sasportal1_alpha1/api::PolicyGetCall), [*set*](https://docs.rs/google-prod_tt_sasportal1_alpha1/5.0.5+20240626/google_prod_tt_sasportal1_alpha1/api::PolicySetCall) and [*test*](https://docs.rs/google-prod_tt_sasportal1_alpha1/5.0.5+20240626/google_prod_tt_sasportal1_alpha1/api::PolicyTestCall)




# Structure of this Library

The API is structured into the following primary items:

* **[Hub](https://docs.rs/google-prod_tt_sasportal1_alpha1/5.0.5+20240626/google_prod_tt_sasportal1_alpha1/SASPortalTesting)**
    * a central object to maintain state and allow accessing all *Activities*
    * creates [*Method Builders*](https://docs.rs/google-prod_tt_sasportal1_alpha1/5.0.5+20240626/google_prod_tt_sasportal1_alpha1/client::MethodsBuilder) which in turn
      allow access to individual [*Call Builders*](https://docs.rs/google-prod_tt_sasportal1_alpha1/5.0.5+20240626/google_prod_tt_sasportal1_alpha1/client::CallBuilder)
* **[Resources](https://docs.rs/google-prod_tt_sasportal1_alpha1/5.0.5+20240626/google_prod_tt_sasportal1_alpha1/client::Resource)**
    * primary types that you can apply *Activities* to
    * a collection of properties and *Parts*
    * **[Parts](https://docs.rs/google-prod_tt_sasportal1_alpha1/5.0.5+20240626/google_prod_tt_sasportal1_alpha1/client::Part)**
        * a collection of properties
        * never directly used in *Activities*
* **[Activities](https://docs.rs/google-prod_tt_sasportal1_alpha1/5.0.5+20240626/google_prod_tt_sasportal1_alpha1/client::CallBuilder)**
    * operations to apply to *Resources*

All *structures* are marked with applicable traits to further categorize them and ease browsing.

Generally speaking, you can invoke *Activities* like this:

```Rust,ignore
let r = hub.resource().activity(...).doit().await
```

Or specifically ...

```ignore
let r = hub.customers().deployments_devices_create(...).doit().await
let r = hub.customers().deployments_devices_create_signed(...).doit().await
let r = hub.customers().devices_create(...).doit().await
let r = hub.customers().devices_create_signed(...).doit().await
let r = hub.customers().devices_get(...).doit().await
let r = hub.customers().devices_patch(...).doit().await
let r = hub.customers().devices_update_signed(...).doit().await
let r = hub.customers().nodes_devices_create(...).doit().await
let r = hub.customers().nodes_devices_create_signed(...).doit().await
let r = hub.deployments().devices_get(...).doit().await
let r = hub.deployments().devices_patch(...).doit().await
let r = hub.deployments().devices_update_signed(...).doit().await
let r = hub.nodes().deployments_devices_create(...).doit().await
let r = hub.nodes().deployments_devices_create_signed(...).doit().await
let r = hub.nodes().devices_create(...).doit().await
let r = hub.nodes().devices_create_signed(...).doit().await
let r = hub.nodes().devices_get(...).doit().await
let r = hub.nodes().devices_patch(...).doit().await
let r = hub.nodes().devices_update_signed(...).doit().await
let r = hub.nodes().nodes_devices_create(...).doit().await
let r = hub.nodes().nodes_devices_create_signed(...).doit().await
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
google-prod_tt_sasportal1_alpha1 = "*"
serde = "^1.0"
serde_json = "^1.0"
```

## A complete example

```Rust
extern crate hyper;
extern crate hyper_rustls;
extern crate google_prod_tt_sasportal1_alpha1 as prod_tt_sasportal1_alpha1;
use prod_tt_sasportal1_alpha1::api::SasPortalDevice;
use prod_tt_sasportal1_alpha1::{Result, Error};
use std::default::Default;
use prod_tt_sasportal1_alpha1::{SASPortalTesting, oauth2, hyper, hyper_rustls, chrono, FieldMask};

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
let mut hub = SASPortalTesting::new(hyper::Client::builder().build(hyper_rustls::HttpsConnectorBuilder::new().with_native_roots().unwrap().https_or_http().enable_http1().build()), auth);
// As the method needs a request, you would usually fill it with the desired information
// into the respective structure. Some of the parts shown here might not be applicable !
// Values shown here are possibly random and not representative !
let mut req = SasPortalDevice::default();

// You can configure optional parameters by calling the respective setters at will, and
// execute the final call using `doit()`.
// Values shown here are possibly random and not representative !
let result = hub.customers().devices_patch(req, "name")
             .update_mask(FieldMask::new::<&str>(&[]))
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

All errors produced by the system are provided either as [Result](https://docs.rs/google-prod_tt_sasportal1_alpha1/5.0.5+20240626/google_prod_tt_sasportal1_alpha1/client::Result) enumeration as return value of
the doit() methods, or handed as possibly intermediate results to either the 
[Hub Delegate](https://docs.rs/google-prod_tt_sasportal1_alpha1/5.0.5+20240626/google_prod_tt_sasportal1_alpha1/client::Delegate), or the [Authenticator Delegate](https://docs.rs/yup-oauth2/*/yup_oauth2/trait.AuthenticatorDelegate.html).

When delegates handle errors or intermediate values, they may have a chance to instruct the system to retry. This 
makes the system potentially resilient to all kinds of errors.

## Uploads and Downloads
If a method supports downloads, the response body, which is part of the [Result](https://docs.rs/google-prod_tt_sasportal1_alpha1/5.0.5+20240626/google_prod_tt_sasportal1_alpha1/client::Result), should be
read by you to obtain the media.
If such a method also supports a [Response Result](https://docs.rs/google-prod_tt_sasportal1_alpha1/5.0.5+20240626/google_prod_tt_sasportal1_alpha1/client::ResponseResult), it will return that by default.
You can see it as meta-data for the actual media. To trigger a media download, you will have to set up the builder by making
this call: `.param("alt", "media")`.

Methods supporting uploads can do so using up to 2 different protocols: 
*simple* and *resumable*. The distinctiveness of each is represented by customized 
`doit(...)` methods, which are then named `upload(...)` and `upload_resumable(...)` respectively.

## Customization and Callbacks

You may alter the way an `doit()` method is called by providing a [delegate](https://docs.rs/google-prod_tt_sasportal1_alpha1/5.0.5+20240626/google_prod_tt_sasportal1_alpha1/client::Delegate) to the 
[Method Builder](https://docs.rs/google-prod_tt_sasportal1_alpha1/5.0.5+20240626/google_prod_tt_sasportal1_alpha1/client::CallBuilder) before making the final `doit()` call. 
Respective methods will be called to provide progress information, as well as determine whether the system should 
retry on failure.

The [delegate trait](https://docs.rs/google-prod_tt_sasportal1_alpha1/5.0.5+20240626/google_prod_tt_sasportal1_alpha1/client::Delegate) is default-implemented, allowing you to customize it with minimal effort.

## Optional Parts in Server-Requests

All structures provided by this library are made to be [encodable](https://docs.rs/google-prod_tt_sasportal1_alpha1/5.0.5+20240626/google_prod_tt_sasportal1_alpha1/client::RequestValue) and 
[decodable](https://docs.rs/google-prod_tt_sasportal1_alpha1/5.0.5+20240626/google_prod_tt_sasportal1_alpha1/client::ResponseResult) via *json*. Optionals are used to indicate that partial requests are responses 
are valid.
Most optionals are are considered [Parts](https://docs.rs/google-prod_tt_sasportal1_alpha1/5.0.5+20240626/google_prod_tt_sasportal1_alpha1/client::Part) which are identifiable by name, which will be sent to 
the server to indicate either the set parts of the request or the desired parts in the response.

## Builder Arguments

Using [method builders](https://docs.rs/google-prod_tt_sasportal1_alpha1/5.0.5+20240626/google_prod_tt_sasportal1_alpha1/client::CallBuilder), you are able to prepare an action call by repeatedly calling it's methods.
These will always take a single argument, for which the following statements are true.

* [PODs][wiki-pod] are handed by copy
* strings are passed as `&str`
* [request values](https://docs.rs/google-prod_tt_sasportal1_alpha1/5.0.5+20240626/google_prod_tt_sasportal1_alpha1/client::RequestValue) are moved

Arguments will always be copied or cloned into the builder, to make them independent of their original life times.

[wiki-pod]: http://en.wikipedia.org/wiki/Plain_old_data_structure
[builder-pattern]: http://en.wikipedia.org/wiki/Builder_pattern
[google-go-api]: https://github.com/google/google-api-go-client

## Cargo Features

* `utoipa` - Add support for [utoipa](https://crates.io/crates/utoipa) and derive `utoipa::ToSchema` on all
the types. You'll have to import and register the required types in `#[openapi(schemas(...))]`, otherwise the
generated `openapi` spec would be invalid.


# License
The **prod_tt_sasportal1_alpha1** library was generated by Sebastian Thiel, and is placed 
under the *MIT* license.
You can read the full text at the repository's [license file][repo-license].

[repo-license]: https://github.com/Byron/google-apis-rsblob/main/LICENSE.md

