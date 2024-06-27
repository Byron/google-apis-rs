<!---
DO NOT EDIT !
This file was generated automatically from 'src/generator/templates/api/README.md.mako'
DO NOT EDIT !
-->
The `google-networkservices1` library allows access to all features of the *Google NetworkServices* service.

This documentation was generated from *NetworkServices* crate version *5.0.5+20240620*, where *20240620* is the exact revision of the *networkservices:v1* schema built by the [mako](http://www.makotemplates.org/) code generator *v5.0.5*.

Everything else about the *NetworkServices* *v1* API can be found at the
[official documentation site](https://cloud.google.com/networking).
# Features

Handle the following *Resources* with ease from the central [hub](https://docs.rs/google-networkservices1/5.0.5+20240620/google_networkservices1/NetworkServices) ... 

* projects
 * [*locations edge cache keysets get iam policy*](https://docs.rs/google-networkservices1/5.0.5+20240620/google_networkservices1/api::ProjectLocationEdgeCacheKeysetGetIamPolicyCall), [*locations edge cache keysets set iam policy*](https://docs.rs/google-networkservices1/5.0.5+20240620/google_networkservices1/api::ProjectLocationEdgeCacheKeysetSetIamPolicyCall), [*locations edge cache keysets test iam permissions*](https://docs.rs/google-networkservices1/5.0.5+20240620/google_networkservices1/api::ProjectLocationEdgeCacheKeysetTestIamPermissionCall), [*locations edge cache origins get iam policy*](https://docs.rs/google-networkservices1/5.0.5+20240620/google_networkservices1/api::ProjectLocationEdgeCacheOriginGetIamPolicyCall), [*locations edge cache origins set iam policy*](https://docs.rs/google-networkservices1/5.0.5+20240620/google_networkservices1/api::ProjectLocationEdgeCacheOriginSetIamPolicyCall), [*locations edge cache origins test iam permissions*](https://docs.rs/google-networkservices1/5.0.5+20240620/google_networkservices1/api::ProjectLocationEdgeCacheOriginTestIamPermissionCall), [*locations edge cache services get iam policy*](https://docs.rs/google-networkservices1/5.0.5+20240620/google_networkservices1/api::ProjectLocationEdgeCacheServiceGetIamPolicyCall), [*locations edge cache services set iam policy*](https://docs.rs/google-networkservices1/5.0.5+20240620/google_networkservices1/api::ProjectLocationEdgeCacheServiceSetIamPolicyCall), [*locations edge cache services test iam permissions*](https://docs.rs/google-networkservices1/5.0.5+20240620/google_networkservices1/api::ProjectLocationEdgeCacheServiceTestIamPermissionCall), [*locations endpoint policies create*](https://docs.rs/google-networkservices1/5.0.5+20240620/google_networkservices1/api::ProjectLocationEndpointPolicyCreateCall), [*locations endpoint policies delete*](https://docs.rs/google-networkservices1/5.0.5+20240620/google_networkservices1/api::ProjectLocationEndpointPolicyDeleteCall), [*locations endpoint policies get*](https://docs.rs/google-networkservices1/5.0.5+20240620/google_networkservices1/api::ProjectLocationEndpointPolicyGetCall), [*locations endpoint policies get iam policy*](https://docs.rs/google-networkservices1/5.0.5+20240620/google_networkservices1/api::ProjectLocationEndpointPolicyGetIamPolicyCall), [*locations endpoint policies list*](https://docs.rs/google-networkservices1/5.0.5+20240620/google_networkservices1/api::ProjectLocationEndpointPolicyListCall), [*locations endpoint policies patch*](https://docs.rs/google-networkservices1/5.0.5+20240620/google_networkservices1/api::ProjectLocationEndpointPolicyPatchCall), [*locations endpoint policies set iam policy*](https://docs.rs/google-networkservices1/5.0.5+20240620/google_networkservices1/api::ProjectLocationEndpointPolicySetIamPolicyCall), [*locations endpoint policies test iam permissions*](https://docs.rs/google-networkservices1/5.0.5+20240620/google_networkservices1/api::ProjectLocationEndpointPolicyTestIamPermissionCall), [*locations gateways create*](https://docs.rs/google-networkservices1/5.0.5+20240620/google_networkservices1/api::ProjectLocationGatewayCreateCall), [*locations gateways delete*](https://docs.rs/google-networkservices1/5.0.5+20240620/google_networkservices1/api::ProjectLocationGatewayDeleteCall), [*locations gateways get*](https://docs.rs/google-networkservices1/5.0.5+20240620/google_networkservices1/api::ProjectLocationGatewayGetCall), [*locations gateways get iam policy*](https://docs.rs/google-networkservices1/5.0.5+20240620/google_networkservices1/api::ProjectLocationGatewayGetIamPolicyCall), [*locations gateways list*](https://docs.rs/google-networkservices1/5.0.5+20240620/google_networkservices1/api::ProjectLocationGatewayListCall), [*locations gateways patch*](https://docs.rs/google-networkservices1/5.0.5+20240620/google_networkservices1/api::ProjectLocationGatewayPatchCall), [*locations gateways set iam policy*](https://docs.rs/google-networkservices1/5.0.5+20240620/google_networkservices1/api::ProjectLocationGatewaySetIamPolicyCall), [*locations gateways test iam permissions*](https://docs.rs/google-networkservices1/5.0.5+20240620/google_networkservices1/api::ProjectLocationGatewayTestIamPermissionCall), [*locations get*](https://docs.rs/google-networkservices1/5.0.5+20240620/google_networkservices1/api::ProjectLocationGetCall), [*locations grpc routes create*](https://docs.rs/google-networkservices1/5.0.5+20240620/google_networkservices1/api::ProjectLocationGrpcRouteCreateCall), [*locations grpc routes delete*](https://docs.rs/google-networkservices1/5.0.5+20240620/google_networkservices1/api::ProjectLocationGrpcRouteDeleteCall), [*locations grpc routes get*](https://docs.rs/google-networkservices1/5.0.5+20240620/google_networkservices1/api::ProjectLocationGrpcRouteGetCall), [*locations grpc routes list*](https://docs.rs/google-networkservices1/5.0.5+20240620/google_networkservices1/api::ProjectLocationGrpcRouteListCall), [*locations grpc routes patch*](https://docs.rs/google-networkservices1/5.0.5+20240620/google_networkservices1/api::ProjectLocationGrpcRoutePatchCall), [*locations http routes create*](https://docs.rs/google-networkservices1/5.0.5+20240620/google_networkservices1/api::ProjectLocationHttpRouteCreateCall), [*locations http routes delete*](https://docs.rs/google-networkservices1/5.0.5+20240620/google_networkservices1/api::ProjectLocationHttpRouteDeleteCall), [*locations http routes get*](https://docs.rs/google-networkservices1/5.0.5+20240620/google_networkservices1/api::ProjectLocationHttpRouteGetCall), [*locations http routes list*](https://docs.rs/google-networkservices1/5.0.5+20240620/google_networkservices1/api::ProjectLocationHttpRouteListCall), [*locations http routes patch*](https://docs.rs/google-networkservices1/5.0.5+20240620/google_networkservices1/api::ProjectLocationHttpRoutePatchCall), [*locations lb route extensions create*](https://docs.rs/google-networkservices1/5.0.5+20240620/google_networkservices1/api::ProjectLocationLbRouteExtensionCreateCall), [*locations lb route extensions delete*](https://docs.rs/google-networkservices1/5.0.5+20240620/google_networkservices1/api::ProjectLocationLbRouteExtensionDeleteCall), [*locations lb route extensions get*](https://docs.rs/google-networkservices1/5.0.5+20240620/google_networkservices1/api::ProjectLocationLbRouteExtensionGetCall), [*locations lb route extensions list*](https://docs.rs/google-networkservices1/5.0.5+20240620/google_networkservices1/api::ProjectLocationLbRouteExtensionListCall), [*locations lb route extensions patch*](https://docs.rs/google-networkservices1/5.0.5+20240620/google_networkservices1/api::ProjectLocationLbRouteExtensionPatchCall), [*locations lb traffic extensions create*](https://docs.rs/google-networkservices1/5.0.5+20240620/google_networkservices1/api::ProjectLocationLbTrafficExtensionCreateCall), [*locations lb traffic extensions delete*](https://docs.rs/google-networkservices1/5.0.5+20240620/google_networkservices1/api::ProjectLocationLbTrafficExtensionDeleteCall), [*locations lb traffic extensions get*](https://docs.rs/google-networkservices1/5.0.5+20240620/google_networkservices1/api::ProjectLocationLbTrafficExtensionGetCall), [*locations lb traffic extensions list*](https://docs.rs/google-networkservices1/5.0.5+20240620/google_networkservices1/api::ProjectLocationLbTrafficExtensionListCall), [*locations lb traffic extensions patch*](https://docs.rs/google-networkservices1/5.0.5+20240620/google_networkservices1/api::ProjectLocationLbTrafficExtensionPatchCall), [*locations list*](https://docs.rs/google-networkservices1/5.0.5+20240620/google_networkservices1/api::ProjectLocationListCall), [*locations meshes create*](https://docs.rs/google-networkservices1/5.0.5+20240620/google_networkservices1/api::ProjectLocationMeshCreateCall), [*locations meshes delete*](https://docs.rs/google-networkservices1/5.0.5+20240620/google_networkservices1/api::ProjectLocationMeshDeleteCall), [*locations meshes get*](https://docs.rs/google-networkservices1/5.0.5+20240620/google_networkservices1/api::ProjectLocationMeshGetCall), [*locations meshes get iam policy*](https://docs.rs/google-networkservices1/5.0.5+20240620/google_networkservices1/api::ProjectLocationMeshGetIamPolicyCall), [*locations meshes list*](https://docs.rs/google-networkservices1/5.0.5+20240620/google_networkservices1/api::ProjectLocationMeshListCall), [*locations meshes patch*](https://docs.rs/google-networkservices1/5.0.5+20240620/google_networkservices1/api::ProjectLocationMeshPatchCall), [*locations meshes set iam policy*](https://docs.rs/google-networkservices1/5.0.5+20240620/google_networkservices1/api::ProjectLocationMeshSetIamPolicyCall), [*locations meshes test iam permissions*](https://docs.rs/google-networkservices1/5.0.5+20240620/google_networkservices1/api::ProjectLocationMeshTestIamPermissionCall), [*locations operations cancel*](https://docs.rs/google-networkservices1/5.0.5+20240620/google_networkservices1/api::ProjectLocationOperationCancelCall), [*locations operations delete*](https://docs.rs/google-networkservices1/5.0.5+20240620/google_networkservices1/api::ProjectLocationOperationDeleteCall), [*locations operations get*](https://docs.rs/google-networkservices1/5.0.5+20240620/google_networkservices1/api::ProjectLocationOperationGetCall), [*locations operations list*](https://docs.rs/google-networkservices1/5.0.5+20240620/google_networkservices1/api::ProjectLocationOperationListCall), [*locations service bindings create*](https://docs.rs/google-networkservices1/5.0.5+20240620/google_networkservices1/api::ProjectLocationServiceBindingCreateCall), [*locations service bindings delete*](https://docs.rs/google-networkservices1/5.0.5+20240620/google_networkservices1/api::ProjectLocationServiceBindingDeleteCall), [*locations service bindings get*](https://docs.rs/google-networkservices1/5.0.5+20240620/google_networkservices1/api::ProjectLocationServiceBindingGetCall), [*locations service bindings get iam policy*](https://docs.rs/google-networkservices1/5.0.5+20240620/google_networkservices1/api::ProjectLocationServiceBindingGetIamPolicyCall), [*locations service bindings list*](https://docs.rs/google-networkservices1/5.0.5+20240620/google_networkservices1/api::ProjectLocationServiceBindingListCall), [*locations service bindings set iam policy*](https://docs.rs/google-networkservices1/5.0.5+20240620/google_networkservices1/api::ProjectLocationServiceBindingSetIamPolicyCall), [*locations service bindings test iam permissions*](https://docs.rs/google-networkservices1/5.0.5+20240620/google_networkservices1/api::ProjectLocationServiceBindingTestIamPermissionCall), [*locations service lb policies create*](https://docs.rs/google-networkservices1/5.0.5+20240620/google_networkservices1/api::ProjectLocationServiceLbPolicyCreateCall), [*locations service lb policies delete*](https://docs.rs/google-networkservices1/5.0.5+20240620/google_networkservices1/api::ProjectLocationServiceLbPolicyDeleteCall), [*locations service lb policies get*](https://docs.rs/google-networkservices1/5.0.5+20240620/google_networkservices1/api::ProjectLocationServiceLbPolicyGetCall), [*locations service lb policies get iam policy*](https://docs.rs/google-networkservices1/5.0.5+20240620/google_networkservices1/api::ProjectLocationServiceLbPolicyGetIamPolicyCall), [*locations service lb policies list*](https://docs.rs/google-networkservices1/5.0.5+20240620/google_networkservices1/api::ProjectLocationServiceLbPolicyListCall), [*locations service lb policies patch*](https://docs.rs/google-networkservices1/5.0.5+20240620/google_networkservices1/api::ProjectLocationServiceLbPolicyPatchCall), [*locations service lb policies set iam policy*](https://docs.rs/google-networkservices1/5.0.5+20240620/google_networkservices1/api::ProjectLocationServiceLbPolicySetIamPolicyCall), [*locations service lb policies test iam permissions*](https://docs.rs/google-networkservices1/5.0.5+20240620/google_networkservices1/api::ProjectLocationServiceLbPolicyTestIamPermissionCall), [*locations tcp routes create*](https://docs.rs/google-networkservices1/5.0.5+20240620/google_networkservices1/api::ProjectLocationTcpRouteCreateCall), [*locations tcp routes delete*](https://docs.rs/google-networkservices1/5.0.5+20240620/google_networkservices1/api::ProjectLocationTcpRouteDeleteCall), [*locations tcp routes get*](https://docs.rs/google-networkservices1/5.0.5+20240620/google_networkservices1/api::ProjectLocationTcpRouteGetCall), [*locations tcp routes list*](https://docs.rs/google-networkservices1/5.0.5+20240620/google_networkservices1/api::ProjectLocationTcpRouteListCall), [*locations tcp routes patch*](https://docs.rs/google-networkservices1/5.0.5+20240620/google_networkservices1/api::ProjectLocationTcpRoutePatchCall), [*locations tls routes create*](https://docs.rs/google-networkservices1/5.0.5+20240620/google_networkservices1/api::ProjectLocationTlsRouteCreateCall), [*locations tls routes delete*](https://docs.rs/google-networkservices1/5.0.5+20240620/google_networkservices1/api::ProjectLocationTlsRouteDeleteCall), [*locations tls routes get*](https://docs.rs/google-networkservices1/5.0.5+20240620/google_networkservices1/api::ProjectLocationTlsRouteGetCall), [*locations tls routes list*](https://docs.rs/google-networkservices1/5.0.5+20240620/google_networkservices1/api::ProjectLocationTlsRouteListCall) and [*locations tls routes patch*](https://docs.rs/google-networkservices1/5.0.5+20240620/google_networkservices1/api::ProjectLocationTlsRoutePatchCall)




# Structure of this Library

The API is structured into the following primary items:

* **[Hub](https://docs.rs/google-networkservices1/5.0.5+20240620/google_networkservices1/NetworkServices)**
    * a central object to maintain state and allow accessing all *Activities*
    * creates [*Method Builders*](https://docs.rs/google-networkservices1/5.0.5+20240620/google_networkservices1/client::MethodsBuilder) which in turn
      allow access to individual [*Call Builders*](https://docs.rs/google-networkservices1/5.0.5+20240620/google_networkservices1/client::CallBuilder)
* **[Resources](https://docs.rs/google-networkservices1/5.0.5+20240620/google_networkservices1/client::Resource)**
    * primary types that you can apply *Activities* to
    * a collection of properties and *Parts*
    * **[Parts](https://docs.rs/google-networkservices1/5.0.5+20240620/google_networkservices1/client::Part)**
        * a collection of properties
        * never directly used in *Activities*
* **[Activities](https://docs.rs/google-networkservices1/5.0.5+20240620/google_networkservices1/client::CallBuilder)**
    * operations to apply to *Resources*

All *structures* are marked with applicable traits to further categorize them and ease browsing.

Generally speaking, you can invoke *Activities* like this:

```Rust,ignore
let r = hub.resource().activity(...).doit().await
```

Or specifically ...

```ignore
let r = hub.projects().locations_endpoint_policies_create(...).doit().await
let r = hub.projects().locations_endpoint_policies_delete(...).doit().await
let r = hub.projects().locations_endpoint_policies_patch(...).doit().await
let r = hub.projects().locations_gateways_create(...).doit().await
let r = hub.projects().locations_gateways_delete(...).doit().await
let r = hub.projects().locations_gateways_patch(...).doit().await
let r = hub.projects().locations_grpc_routes_create(...).doit().await
let r = hub.projects().locations_grpc_routes_delete(...).doit().await
let r = hub.projects().locations_grpc_routes_patch(...).doit().await
let r = hub.projects().locations_http_routes_create(...).doit().await
let r = hub.projects().locations_http_routes_delete(...).doit().await
let r = hub.projects().locations_http_routes_patch(...).doit().await
let r = hub.projects().locations_lb_route_extensions_create(...).doit().await
let r = hub.projects().locations_lb_route_extensions_delete(...).doit().await
let r = hub.projects().locations_lb_route_extensions_patch(...).doit().await
let r = hub.projects().locations_lb_traffic_extensions_create(...).doit().await
let r = hub.projects().locations_lb_traffic_extensions_delete(...).doit().await
let r = hub.projects().locations_lb_traffic_extensions_patch(...).doit().await
let r = hub.projects().locations_meshes_create(...).doit().await
let r = hub.projects().locations_meshes_delete(...).doit().await
let r = hub.projects().locations_meshes_patch(...).doit().await
let r = hub.projects().locations_operations_get(...).doit().await
let r = hub.projects().locations_service_bindings_create(...).doit().await
let r = hub.projects().locations_service_bindings_delete(...).doit().await
let r = hub.projects().locations_service_lb_policies_create(...).doit().await
let r = hub.projects().locations_service_lb_policies_delete(...).doit().await
let r = hub.projects().locations_service_lb_policies_patch(...).doit().await
let r = hub.projects().locations_tcp_routes_create(...).doit().await
let r = hub.projects().locations_tcp_routes_delete(...).doit().await
let r = hub.projects().locations_tcp_routes_patch(...).doit().await
let r = hub.projects().locations_tls_routes_create(...).doit().await
let r = hub.projects().locations_tls_routes_delete(...).doit().await
let r = hub.projects().locations_tls_routes_patch(...).doit().await
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
google-networkservices1 = "*"
serde = "^1.0"
serde_json = "^1.0"
```

## A complete example

```Rust
extern crate hyper;
extern crate hyper_rustls;
extern crate google_networkservices1 as networkservices1;
use networkservices1::api::LbRouteExtension;
use networkservices1::{Result, Error};
use std::default::Default;
use networkservices1::{NetworkServices, oauth2, hyper, hyper_rustls, chrono, FieldMask};

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
let mut hub = NetworkServices::new(hyper::Client::builder().build(hyper_rustls::HttpsConnectorBuilder::new().with_native_roots().unwrap().https_or_http().enable_http1().build()), auth);
// As the method needs a request, you would usually fill it with the desired information
// into the respective structure. Some of the parts shown here might not be applicable !
// Values shown here are possibly random and not representative !
let mut req = LbRouteExtension::default();

// You can configure optional parameters by calling the respective setters at will, and
// execute the final call using `doit()`.
// Values shown here are possibly random and not representative !
let result = hub.projects().locations_lb_route_extensions_create(req, "parent")
             .request_id("magna")
             .lb_route_extension_id("no")
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

All errors produced by the system are provided either as [Result](https://docs.rs/google-networkservices1/5.0.5+20240620/google_networkservices1/client::Result) enumeration as return value of
the doit() methods, or handed as possibly intermediate results to either the 
[Hub Delegate](https://docs.rs/google-networkservices1/5.0.5+20240620/google_networkservices1/client::Delegate), or the [Authenticator Delegate](https://docs.rs/yup-oauth2/*/yup_oauth2/trait.AuthenticatorDelegate.html).

When delegates handle errors or intermediate values, they may have a chance to instruct the system to retry. This 
makes the system potentially resilient to all kinds of errors.

## Uploads and Downloads
If a method supports downloads, the response body, which is part of the [Result](https://docs.rs/google-networkservices1/5.0.5+20240620/google_networkservices1/client::Result), should be
read by you to obtain the media.
If such a method also supports a [Response Result](https://docs.rs/google-networkservices1/5.0.5+20240620/google_networkservices1/client::ResponseResult), it will return that by default.
You can see it as meta-data for the actual media. To trigger a media download, you will have to set up the builder by making
this call: `.param("alt", "media")`.

Methods supporting uploads can do so using up to 2 different protocols: 
*simple* and *resumable*. The distinctiveness of each is represented by customized 
`doit(...)` methods, which are then named `upload(...)` and `upload_resumable(...)` respectively.

## Customization and Callbacks

You may alter the way an `doit()` method is called by providing a [delegate](https://docs.rs/google-networkservices1/5.0.5+20240620/google_networkservices1/client::Delegate) to the 
[Method Builder](https://docs.rs/google-networkservices1/5.0.5+20240620/google_networkservices1/client::CallBuilder) before making the final `doit()` call. 
Respective methods will be called to provide progress information, as well as determine whether the system should 
retry on failure.

The [delegate trait](https://docs.rs/google-networkservices1/5.0.5+20240620/google_networkservices1/client::Delegate) is default-implemented, allowing you to customize it with minimal effort.

## Optional Parts in Server-Requests

All structures provided by this library are made to be [encodable](https://docs.rs/google-networkservices1/5.0.5+20240620/google_networkservices1/client::RequestValue) and 
[decodable](https://docs.rs/google-networkservices1/5.0.5+20240620/google_networkservices1/client::ResponseResult) via *json*. Optionals are used to indicate that partial requests are responses 
are valid.
Most optionals are are considered [Parts](https://docs.rs/google-networkservices1/5.0.5+20240620/google_networkservices1/client::Part) which are identifiable by name, which will be sent to 
the server to indicate either the set parts of the request or the desired parts in the response.

## Builder Arguments

Using [method builders](https://docs.rs/google-networkservices1/5.0.5+20240620/google_networkservices1/client::CallBuilder), you are able to prepare an action call by repeatedly calling it's methods.
These will always take a single argument, for which the following statements are true.

* [PODs][wiki-pod] are handed by copy
* strings are passed as `&str`
* [request values](https://docs.rs/google-networkservices1/5.0.5+20240620/google_networkservices1/client::RequestValue) are moved

Arguments will always be copied or cloned into the builder, to make them independent of their original life times.

[wiki-pod]: http://en.wikipedia.org/wiki/Plain_old_data_structure
[builder-pattern]: http://en.wikipedia.org/wiki/Builder_pattern
[google-go-api]: https://github.com/google/google-api-go-client

## Cargo Features

* `utoipa` - Add support for [utoipa](https://crates.io/crates/utoipa) and derive `utoipa::ToSchema` on all
the types. You'll have to import and register the required types in `#[openapi(schemas(...))]`, otherwise the
generated `openapi` spec would be invalid.


# License
The **networkservices1** library was generated by Sebastian Thiel, and is placed 
under the *MIT* license.
You can read the full text at the repository's [license file][repo-license].

[repo-license]: https://github.com/Byron/google-apis-rsblob/main/LICENSE.md

