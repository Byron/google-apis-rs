<!---
DO NOT EDIT !
This file was generated automatically from 'src/generator/templates/api/README.md.mako'
DO NOT EDIT !
-->
The `google-networkservices1` library allows access to all features of the *Google NetworkServices* service.

This documentation was generated from *NetworkServices* crate version *7.0.0+20251203*, where *20251203* is the exact revision of the *networkservices:v1* schema built by the [mako](http://www.makotemplates.org/) code generator *v7.0.0*.

Everything else about the *NetworkServices* *v1* API can be found at the
[official documentation site](https://cloud.google.com/networking).
# Features

Handle the following *Resources* with ease from the central [hub](https://docs.rs/google-networkservices1/7.0.0+20251203/google_networkservices1/NetworkServices) ...

* projects
 * [*locations authz extensions create*](https://docs.rs/google-networkservices1/7.0.0+20251203/google_networkservices1/api::ProjectLocationAuthzExtensionCreateCall), [*locations authz extensions delete*](https://docs.rs/google-networkservices1/7.0.0+20251203/google_networkservices1/api::ProjectLocationAuthzExtensionDeleteCall), [*locations authz extensions get*](https://docs.rs/google-networkservices1/7.0.0+20251203/google_networkservices1/api::ProjectLocationAuthzExtensionGetCall), [*locations authz extensions list*](https://docs.rs/google-networkservices1/7.0.0+20251203/google_networkservices1/api::ProjectLocationAuthzExtensionListCall), [*locations authz extensions patch*](https://docs.rs/google-networkservices1/7.0.0+20251203/google_networkservices1/api::ProjectLocationAuthzExtensionPatchCall), [*locations edge cache keysets get iam policy*](https://docs.rs/google-networkservices1/7.0.0+20251203/google_networkservices1/api::ProjectLocationEdgeCacheKeysetGetIamPolicyCall), [*locations edge cache keysets set iam policy*](https://docs.rs/google-networkservices1/7.0.0+20251203/google_networkservices1/api::ProjectLocationEdgeCacheKeysetSetIamPolicyCall), [*locations edge cache keysets test iam permissions*](https://docs.rs/google-networkservices1/7.0.0+20251203/google_networkservices1/api::ProjectLocationEdgeCacheKeysetTestIamPermissionCall), [*locations edge cache origins get iam policy*](https://docs.rs/google-networkservices1/7.0.0+20251203/google_networkservices1/api::ProjectLocationEdgeCacheOriginGetIamPolicyCall), [*locations edge cache origins set iam policy*](https://docs.rs/google-networkservices1/7.0.0+20251203/google_networkservices1/api::ProjectLocationEdgeCacheOriginSetIamPolicyCall), [*locations edge cache origins test iam permissions*](https://docs.rs/google-networkservices1/7.0.0+20251203/google_networkservices1/api::ProjectLocationEdgeCacheOriginTestIamPermissionCall), [*locations edge cache services get iam policy*](https://docs.rs/google-networkservices1/7.0.0+20251203/google_networkservices1/api::ProjectLocationEdgeCacheServiceGetIamPolicyCall), [*locations edge cache services set iam policy*](https://docs.rs/google-networkservices1/7.0.0+20251203/google_networkservices1/api::ProjectLocationEdgeCacheServiceSetIamPolicyCall), [*locations edge cache services test iam permissions*](https://docs.rs/google-networkservices1/7.0.0+20251203/google_networkservices1/api::ProjectLocationEdgeCacheServiceTestIamPermissionCall), [*locations endpoint policies create*](https://docs.rs/google-networkservices1/7.0.0+20251203/google_networkservices1/api::ProjectLocationEndpointPolicyCreateCall), [*locations endpoint policies delete*](https://docs.rs/google-networkservices1/7.0.0+20251203/google_networkservices1/api::ProjectLocationEndpointPolicyDeleteCall), [*locations endpoint policies get*](https://docs.rs/google-networkservices1/7.0.0+20251203/google_networkservices1/api::ProjectLocationEndpointPolicyGetCall), [*locations endpoint policies list*](https://docs.rs/google-networkservices1/7.0.0+20251203/google_networkservices1/api::ProjectLocationEndpointPolicyListCall), [*locations endpoint policies patch*](https://docs.rs/google-networkservices1/7.0.0+20251203/google_networkservices1/api::ProjectLocationEndpointPolicyPatchCall), [*locations gateways create*](https://docs.rs/google-networkservices1/7.0.0+20251203/google_networkservices1/api::ProjectLocationGatewayCreateCall), [*locations gateways delete*](https://docs.rs/google-networkservices1/7.0.0+20251203/google_networkservices1/api::ProjectLocationGatewayDeleteCall), [*locations gateways get*](https://docs.rs/google-networkservices1/7.0.0+20251203/google_networkservices1/api::ProjectLocationGatewayGetCall), [*locations gateways list*](https://docs.rs/google-networkservices1/7.0.0+20251203/google_networkservices1/api::ProjectLocationGatewayListCall), [*locations gateways patch*](https://docs.rs/google-networkservices1/7.0.0+20251203/google_networkservices1/api::ProjectLocationGatewayPatchCall), [*locations gateways route views get*](https://docs.rs/google-networkservices1/7.0.0+20251203/google_networkservices1/api::ProjectLocationGatewayRouteViewGetCall), [*locations gateways route views list*](https://docs.rs/google-networkservices1/7.0.0+20251203/google_networkservices1/api::ProjectLocationGatewayRouteViewListCall), [*locations get*](https://docs.rs/google-networkservices1/7.0.0+20251203/google_networkservices1/api::ProjectLocationGetCall), [*locations grpc routes create*](https://docs.rs/google-networkservices1/7.0.0+20251203/google_networkservices1/api::ProjectLocationGrpcRouteCreateCall), [*locations grpc routes delete*](https://docs.rs/google-networkservices1/7.0.0+20251203/google_networkservices1/api::ProjectLocationGrpcRouteDeleteCall), [*locations grpc routes get*](https://docs.rs/google-networkservices1/7.0.0+20251203/google_networkservices1/api::ProjectLocationGrpcRouteGetCall), [*locations grpc routes list*](https://docs.rs/google-networkservices1/7.0.0+20251203/google_networkservices1/api::ProjectLocationGrpcRouteListCall), [*locations grpc routes patch*](https://docs.rs/google-networkservices1/7.0.0+20251203/google_networkservices1/api::ProjectLocationGrpcRoutePatchCall), [*locations http routes create*](https://docs.rs/google-networkservices1/7.0.0+20251203/google_networkservices1/api::ProjectLocationHttpRouteCreateCall), [*locations http routes delete*](https://docs.rs/google-networkservices1/7.0.0+20251203/google_networkservices1/api::ProjectLocationHttpRouteDeleteCall), [*locations http routes get*](https://docs.rs/google-networkservices1/7.0.0+20251203/google_networkservices1/api::ProjectLocationHttpRouteGetCall), [*locations http routes list*](https://docs.rs/google-networkservices1/7.0.0+20251203/google_networkservices1/api::ProjectLocationHttpRouteListCall), [*locations http routes patch*](https://docs.rs/google-networkservices1/7.0.0+20251203/google_networkservices1/api::ProjectLocationHttpRoutePatchCall), [*locations lb edge extensions create*](https://docs.rs/google-networkservices1/7.0.0+20251203/google_networkservices1/api::ProjectLocationLbEdgeExtensionCreateCall), [*locations lb edge extensions delete*](https://docs.rs/google-networkservices1/7.0.0+20251203/google_networkservices1/api::ProjectLocationLbEdgeExtensionDeleteCall), [*locations lb edge extensions get*](https://docs.rs/google-networkservices1/7.0.0+20251203/google_networkservices1/api::ProjectLocationLbEdgeExtensionGetCall), [*locations lb edge extensions list*](https://docs.rs/google-networkservices1/7.0.0+20251203/google_networkservices1/api::ProjectLocationLbEdgeExtensionListCall), [*locations lb edge extensions patch*](https://docs.rs/google-networkservices1/7.0.0+20251203/google_networkservices1/api::ProjectLocationLbEdgeExtensionPatchCall), [*locations lb route extensions create*](https://docs.rs/google-networkservices1/7.0.0+20251203/google_networkservices1/api::ProjectLocationLbRouteExtensionCreateCall), [*locations lb route extensions delete*](https://docs.rs/google-networkservices1/7.0.0+20251203/google_networkservices1/api::ProjectLocationLbRouteExtensionDeleteCall), [*locations lb route extensions get*](https://docs.rs/google-networkservices1/7.0.0+20251203/google_networkservices1/api::ProjectLocationLbRouteExtensionGetCall), [*locations lb route extensions list*](https://docs.rs/google-networkservices1/7.0.0+20251203/google_networkservices1/api::ProjectLocationLbRouteExtensionListCall), [*locations lb route extensions patch*](https://docs.rs/google-networkservices1/7.0.0+20251203/google_networkservices1/api::ProjectLocationLbRouteExtensionPatchCall), [*locations lb traffic extensions create*](https://docs.rs/google-networkservices1/7.0.0+20251203/google_networkservices1/api::ProjectLocationLbTrafficExtensionCreateCall), [*locations lb traffic extensions delete*](https://docs.rs/google-networkservices1/7.0.0+20251203/google_networkservices1/api::ProjectLocationLbTrafficExtensionDeleteCall), [*locations lb traffic extensions get*](https://docs.rs/google-networkservices1/7.0.0+20251203/google_networkservices1/api::ProjectLocationLbTrafficExtensionGetCall), [*locations lb traffic extensions list*](https://docs.rs/google-networkservices1/7.0.0+20251203/google_networkservices1/api::ProjectLocationLbTrafficExtensionListCall), [*locations lb traffic extensions patch*](https://docs.rs/google-networkservices1/7.0.0+20251203/google_networkservices1/api::ProjectLocationLbTrafficExtensionPatchCall), [*locations list*](https://docs.rs/google-networkservices1/7.0.0+20251203/google_networkservices1/api::ProjectLocationListCall), [*locations meshes create*](https://docs.rs/google-networkservices1/7.0.0+20251203/google_networkservices1/api::ProjectLocationMeshCreateCall), [*locations meshes delete*](https://docs.rs/google-networkservices1/7.0.0+20251203/google_networkservices1/api::ProjectLocationMeshDeleteCall), [*locations meshes get*](https://docs.rs/google-networkservices1/7.0.0+20251203/google_networkservices1/api::ProjectLocationMeshGetCall), [*locations meshes list*](https://docs.rs/google-networkservices1/7.0.0+20251203/google_networkservices1/api::ProjectLocationMeshListCall), [*locations meshes patch*](https://docs.rs/google-networkservices1/7.0.0+20251203/google_networkservices1/api::ProjectLocationMeshPatchCall), [*locations meshes route views get*](https://docs.rs/google-networkservices1/7.0.0+20251203/google_networkservices1/api::ProjectLocationMeshRouteViewGetCall), [*locations meshes route views list*](https://docs.rs/google-networkservices1/7.0.0+20251203/google_networkservices1/api::ProjectLocationMeshRouteViewListCall), [*locations operations cancel*](https://docs.rs/google-networkservices1/7.0.0+20251203/google_networkservices1/api::ProjectLocationOperationCancelCall), [*locations operations delete*](https://docs.rs/google-networkservices1/7.0.0+20251203/google_networkservices1/api::ProjectLocationOperationDeleteCall), [*locations operations get*](https://docs.rs/google-networkservices1/7.0.0+20251203/google_networkservices1/api::ProjectLocationOperationGetCall), [*locations operations list*](https://docs.rs/google-networkservices1/7.0.0+20251203/google_networkservices1/api::ProjectLocationOperationListCall), [*locations service bindings create*](https://docs.rs/google-networkservices1/7.0.0+20251203/google_networkservices1/api::ProjectLocationServiceBindingCreateCall), [*locations service bindings delete*](https://docs.rs/google-networkservices1/7.0.0+20251203/google_networkservices1/api::ProjectLocationServiceBindingDeleteCall), [*locations service bindings get*](https://docs.rs/google-networkservices1/7.0.0+20251203/google_networkservices1/api::ProjectLocationServiceBindingGetCall), [*locations service bindings list*](https://docs.rs/google-networkservices1/7.0.0+20251203/google_networkservices1/api::ProjectLocationServiceBindingListCall), [*locations service bindings patch*](https://docs.rs/google-networkservices1/7.0.0+20251203/google_networkservices1/api::ProjectLocationServiceBindingPatchCall), [*locations service lb policies create*](https://docs.rs/google-networkservices1/7.0.0+20251203/google_networkservices1/api::ProjectLocationServiceLbPolicyCreateCall), [*locations service lb policies delete*](https://docs.rs/google-networkservices1/7.0.0+20251203/google_networkservices1/api::ProjectLocationServiceLbPolicyDeleteCall), [*locations service lb policies get*](https://docs.rs/google-networkservices1/7.0.0+20251203/google_networkservices1/api::ProjectLocationServiceLbPolicyGetCall), [*locations service lb policies list*](https://docs.rs/google-networkservices1/7.0.0+20251203/google_networkservices1/api::ProjectLocationServiceLbPolicyListCall), [*locations service lb policies patch*](https://docs.rs/google-networkservices1/7.0.0+20251203/google_networkservices1/api::ProjectLocationServiceLbPolicyPatchCall), [*locations tcp routes create*](https://docs.rs/google-networkservices1/7.0.0+20251203/google_networkservices1/api::ProjectLocationTcpRouteCreateCall), [*locations tcp routes delete*](https://docs.rs/google-networkservices1/7.0.0+20251203/google_networkservices1/api::ProjectLocationTcpRouteDeleteCall), [*locations tcp routes get*](https://docs.rs/google-networkservices1/7.0.0+20251203/google_networkservices1/api::ProjectLocationTcpRouteGetCall), [*locations tcp routes list*](https://docs.rs/google-networkservices1/7.0.0+20251203/google_networkservices1/api::ProjectLocationTcpRouteListCall), [*locations tcp routes patch*](https://docs.rs/google-networkservices1/7.0.0+20251203/google_networkservices1/api::ProjectLocationTcpRoutePatchCall), [*locations tls routes create*](https://docs.rs/google-networkservices1/7.0.0+20251203/google_networkservices1/api::ProjectLocationTlsRouteCreateCall), [*locations tls routes delete*](https://docs.rs/google-networkservices1/7.0.0+20251203/google_networkservices1/api::ProjectLocationTlsRouteDeleteCall), [*locations tls routes get*](https://docs.rs/google-networkservices1/7.0.0+20251203/google_networkservices1/api::ProjectLocationTlsRouteGetCall), [*locations tls routes list*](https://docs.rs/google-networkservices1/7.0.0+20251203/google_networkservices1/api::ProjectLocationTlsRouteListCall), [*locations tls routes patch*](https://docs.rs/google-networkservices1/7.0.0+20251203/google_networkservices1/api::ProjectLocationTlsRoutePatchCall), [*locations wasm plugins create*](https://docs.rs/google-networkservices1/7.0.0+20251203/google_networkservices1/api::ProjectLocationWasmPluginCreateCall), [*locations wasm plugins delete*](https://docs.rs/google-networkservices1/7.0.0+20251203/google_networkservices1/api::ProjectLocationWasmPluginDeleteCall), [*locations wasm plugins get*](https://docs.rs/google-networkservices1/7.0.0+20251203/google_networkservices1/api::ProjectLocationWasmPluginGetCall), [*locations wasm plugins list*](https://docs.rs/google-networkservices1/7.0.0+20251203/google_networkservices1/api::ProjectLocationWasmPluginListCall), [*locations wasm plugins patch*](https://docs.rs/google-networkservices1/7.0.0+20251203/google_networkservices1/api::ProjectLocationWasmPluginPatchCall), [*locations wasm plugins versions create*](https://docs.rs/google-networkservices1/7.0.0+20251203/google_networkservices1/api::ProjectLocationWasmPluginVersionCreateCall), [*locations wasm plugins versions delete*](https://docs.rs/google-networkservices1/7.0.0+20251203/google_networkservices1/api::ProjectLocationWasmPluginVersionDeleteCall), [*locations wasm plugins versions get*](https://docs.rs/google-networkservices1/7.0.0+20251203/google_networkservices1/api::ProjectLocationWasmPluginVersionGetCall) and [*locations wasm plugins versions list*](https://docs.rs/google-networkservices1/7.0.0+20251203/google_networkservices1/api::ProjectLocationWasmPluginVersionListCall)




# Structure of this Library

The API is structured into the following primary items:

* **[Hub](https://docs.rs/google-networkservices1/7.0.0+20251203/google_networkservices1/NetworkServices)**
    * a central object to maintain state and allow accessing all *Activities*
    * creates [*Method Builders*](https://docs.rs/google-networkservices1/7.0.0+20251203/google_networkservices1/common::MethodsBuilder) which in turn
      allow access to individual [*Call Builders*](https://docs.rs/google-networkservices1/7.0.0+20251203/google_networkservices1/common::CallBuilder)
* **[Resources](https://docs.rs/google-networkservices1/7.0.0+20251203/google_networkservices1/common::Resource)**
    * primary types that you can apply *Activities* to
    * a collection of properties and *Parts*
    * **[Parts](https://docs.rs/google-networkservices1/7.0.0+20251203/google_networkservices1/common::Part)**
        * a collection of properties
        * never directly used in *Activities*
* **[Activities](https://docs.rs/google-networkservices1/7.0.0+20251203/google_networkservices1/common::CallBuilder)**
    * operations to apply to *Resources*

All *structures* are marked with applicable traits to further categorize them and ease browsing.

Generally speaking, you can invoke *Activities* like this:

```Rust,ignore
let r = hub.resource().activity(...).doit().await
```

Or specifically ...

```ignore
let r = hub.projects().locations_authz_extensions_create(...).doit().await
let r = hub.projects().locations_authz_extensions_delete(...).doit().await
let r = hub.projects().locations_authz_extensions_patch(...).doit().await
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
let r = hub.projects().locations_lb_edge_extensions_create(...).doit().await
let r = hub.projects().locations_lb_edge_extensions_delete(...).doit().await
let r = hub.projects().locations_lb_edge_extensions_patch(...).doit().await
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
let r = hub.projects().locations_service_bindings_patch(...).doit().await
let r = hub.projects().locations_service_lb_policies_create(...).doit().await
let r = hub.projects().locations_service_lb_policies_delete(...).doit().await
let r = hub.projects().locations_service_lb_policies_patch(...).doit().await
let r = hub.projects().locations_tcp_routes_create(...).doit().await
let r = hub.projects().locations_tcp_routes_delete(...).doit().await
let r = hub.projects().locations_tcp_routes_patch(...).doit().await
let r = hub.projects().locations_tls_routes_create(...).doit().await
let r = hub.projects().locations_tls_routes_delete(...).doit().await
let r = hub.projects().locations_tls_routes_patch(...).doit().await
let r = hub.projects().locations_wasm_plugins_versions_create(...).doit().await
let r = hub.projects().locations_wasm_plugins_versions_delete(...).doit().await
let r = hub.projects().locations_wasm_plugins_create(...).doit().await
let r = hub.projects().locations_wasm_plugins_delete(...).doit().await
let r = hub.projects().locations_wasm_plugins_patch(...).doit().await
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
serde = "1"
serde_json = "1"
```

## A complete example

```Rust
extern crate hyper;
extern crate hyper_rustls;
extern crate google_networkservices1 as networkservices1;
use networkservices1::api::AuthzExtension;
use networkservices1::{Result, Error};
use networkservices1::{NetworkServices, FieldMask, hyper_rustls, hyper_util, yup_oauth2};

// Get an ApplicationSecret instance by some means. It contains the `client_id` and
// `client_secret`, among other things.
let secret: yup_oauth2::ApplicationSecret = Default::default();
// Instantiate the authenticator. It will choose a suitable authentication flow for you,
// unless you replace  `None` with the desired Flow.
// Provide your own `AuthenticatorDelegate` to adjust the way it operates and get feedback about
// what's going on. You probably want to bring in your own `TokenStorage` to persist tokens and
// retrieve them from storage.
let connector = hyper_rustls::HttpsConnectorBuilder::new()
    .with_native_roots()
    .unwrap()
    .https_only()
    .enable_http2()
    .build();

let executor = hyper_util::rt::TokioExecutor::new();
let auth = yup_oauth2::InstalledFlowAuthenticator::with_client(
    secret,
    yup_oauth2::InstalledFlowReturnMethod::HTTPRedirect,
    yup_oauth2::client::CustomHyperClientBuilder::from(
        hyper_util::client::legacy::Client::builder(executor).build(connector),
    ),
).build().await.unwrap();

let client = hyper_util::client::legacy::Client::builder(
    hyper_util::rt::TokioExecutor::new()
)
.build(
    hyper_rustls::HttpsConnectorBuilder::new()
        .with_native_roots()
        .unwrap()
        .https_or_http()
        .enable_http2()
        .build()
);
let mut hub = NetworkServices::new(client, auth);
// As the method needs a request, you would usually fill it with the desired information
// into the respective structure. Some of the parts shown here might not be applicable !
// Values shown here are possibly random and not representative !
let mut req = AuthzExtension::default();

// You can configure optional parameters by calling the respective setters at will, and
// execute the final call using `doit()`.
// Values shown here are possibly random and not representative !
let result = hub.projects().locations_authz_extensions_create(req, "parent")
             .request_id("magna")
             .authz_extension_id("no")
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

All errors produced by the system are provided either as [Result](https://docs.rs/google-networkservices1/7.0.0+20251203/google_networkservices1/common::Result) enumeration as return value of
the doit() methods, or handed as possibly intermediate results to either the
[Hub Delegate](https://docs.rs/google-networkservices1/7.0.0+20251203/google_networkservices1/common::Delegate), or the [Authenticator Delegate](https://docs.rs/yup-oauth2/*/yup_oauth2/trait.AuthenticatorDelegate.html).

When delegates handle errors or intermediate values, they may have a chance to instruct the system to retry. This
makes the system potentially resilient to all kinds of errors.

## Uploads and Downloads
If a method supports downloads, the response body, which is part of the [Result](https://docs.rs/google-networkservices1/7.0.0+20251203/google_networkservices1/common::Result), should be
read by you to obtain the media.
If such a method also supports a [Response Result](https://docs.rs/google-networkservices1/7.0.0+20251203/google_networkservices1/common::ResponseResult), it will return that by default.
You can see it as meta-data for the actual media. To trigger a media download, you will have to set up the builder by making
this call: `.param("alt", "media")`.

Methods supporting uploads can do so using up to 2 different protocols:
*simple* and *resumable*. The distinctiveness of each is represented by customized
`doit(...)` methods, which are then named `upload(...)` and `upload_resumable(...)` respectively.

## Customization and Callbacks

You may alter the way an `doit()` method is called by providing a [delegate](https://docs.rs/google-networkservices1/7.0.0+20251203/google_networkservices1/common::Delegate) to the
[Method Builder](https://docs.rs/google-networkservices1/7.0.0+20251203/google_networkservices1/common::CallBuilder) before making the final `doit()` call.
Respective methods will be called to provide progress information, as well as determine whether the system should
retry on failure.

The [delegate trait](https://docs.rs/google-networkservices1/7.0.0+20251203/google_networkservices1/common::Delegate) is default-implemented, allowing you to customize it with minimal effort.

## Optional Parts in Server-Requests

All structures provided by this library are made to be [encodable](https://docs.rs/google-networkservices1/7.0.0+20251203/google_networkservices1/common::RequestValue) and
[decodable](https://docs.rs/google-networkservices1/7.0.0+20251203/google_networkservices1/common::ResponseResult) via *json*. Optionals are used to indicate that partial requests are responses
are valid.
Most optionals are are considered [Parts](https://docs.rs/google-networkservices1/7.0.0+20251203/google_networkservices1/common::Part) which are identifiable by name, which will be sent to
the server to indicate either the set parts of the request or the desired parts in the response.

## Builder Arguments

Using [method builders](https://docs.rs/google-networkservices1/7.0.0+20251203/google_networkservices1/common::CallBuilder), you are able to prepare an action call by repeatedly calling it's methods.
These will always take a single argument, for which the following statements are true.

* [PODs][wiki-pod] are handed by copy
* strings are passed as `&str`
* [request values](https://docs.rs/google-networkservices1/7.0.0+20251203/google_networkservices1/common::RequestValue) are moved

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

