<!---
DO NOT EDIT !
This file was generated automatically from 'src/mako/api/README.md.mako'
DO NOT EDIT !
-->
The `google-container1` library allows access to all features of the *Google Container* service.

This documentation was generated from *Container* crate version *1.0.7+20180917*, where *20180917* is the exact revision of the *container:v1* schema built by the [mako](http://www.makotemplates.org/) code generator *v1.0.7*.

Everything else about the *Container* *v1* API can be found at the
[official documentation site](https://cloud.google.com/container-engine/).
# Features

Handle the following *Resources* with ease from the central [hub](https://docs.rs/google-container1/1.0.7+20180917/google_container1/struct.Container.html) ... 

* projects
 * [*locations clusters complete ip rotation*](https://docs.rs/google-container1/1.0.7+20180917/google_container1/struct.ProjectLocationClusterCompleteIpRotationCall.html), [*locations clusters create*](https://docs.rs/google-container1/1.0.7+20180917/google_container1/struct.ProjectLocationClusterCreateCall.html), [*locations clusters delete*](https://docs.rs/google-container1/1.0.7+20180917/google_container1/struct.ProjectLocationClusterDeleteCall.html), [*locations clusters get*](https://docs.rs/google-container1/1.0.7+20180917/google_container1/struct.ProjectLocationClusterGetCall.html), [*locations clusters list*](https://docs.rs/google-container1/1.0.7+20180917/google_container1/struct.ProjectLocationClusterListCall.html), [*locations clusters node pools create*](https://docs.rs/google-container1/1.0.7+20180917/google_container1/struct.ProjectLocationClusterNodePoolCreateCall.html), [*locations clusters node pools delete*](https://docs.rs/google-container1/1.0.7+20180917/google_container1/struct.ProjectLocationClusterNodePoolDeleteCall.html), [*locations clusters node pools get*](https://docs.rs/google-container1/1.0.7+20180917/google_container1/struct.ProjectLocationClusterNodePoolGetCall.html), [*locations clusters node pools list*](https://docs.rs/google-container1/1.0.7+20180917/google_container1/struct.ProjectLocationClusterNodePoolListCall.html), [*locations clusters node pools rollback*](https://docs.rs/google-container1/1.0.7+20180917/google_container1/struct.ProjectLocationClusterNodePoolRollbackCall.html), [*locations clusters node pools set autoscaling*](https://docs.rs/google-container1/1.0.7+20180917/google_container1/struct.ProjectLocationClusterNodePoolSetAutoscalingCall.html), [*locations clusters node pools set management*](https://docs.rs/google-container1/1.0.7+20180917/google_container1/struct.ProjectLocationClusterNodePoolSetManagementCall.html), [*locations clusters node pools set size*](https://docs.rs/google-container1/1.0.7+20180917/google_container1/struct.ProjectLocationClusterNodePoolSetSizeCall.html), [*locations clusters node pools update*](https://docs.rs/google-container1/1.0.7+20180917/google_container1/struct.ProjectLocationClusterNodePoolUpdateCall.html), [*locations clusters set addons*](https://docs.rs/google-container1/1.0.7+20180917/google_container1/struct.ProjectLocationClusterSetAddonCall.html), [*locations clusters set legacy abac*](https://docs.rs/google-container1/1.0.7+20180917/google_container1/struct.ProjectLocationClusterSetLegacyAbacCall.html), [*locations clusters set locations*](https://docs.rs/google-container1/1.0.7+20180917/google_container1/struct.ProjectLocationClusterSetLocationCall.html), [*locations clusters set logging*](https://docs.rs/google-container1/1.0.7+20180917/google_container1/struct.ProjectLocationClusterSetLoggingCall.html), [*locations clusters set maintenance policy*](https://docs.rs/google-container1/1.0.7+20180917/google_container1/struct.ProjectLocationClusterSetMaintenancePolicyCall.html), [*locations clusters set master auth*](https://docs.rs/google-container1/1.0.7+20180917/google_container1/struct.ProjectLocationClusterSetMasterAuthCall.html), [*locations clusters set monitoring*](https://docs.rs/google-container1/1.0.7+20180917/google_container1/struct.ProjectLocationClusterSetMonitoringCall.html), [*locations clusters set network policy*](https://docs.rs/google-container1/1.0.7+20180917/google_container1/struct.ProjectLocationClusterSetNetworkPolicyCall.html), [*locations clusters set resource labels*](https://docs.rs/google-container1/1.0.7+20180917/google_container1/struct.ProjectLocationClusterSetResourceLabelCall.html), [*locations clusters start ip rotation*](https://docs.rs/google-container1/1.0.7+20180917/google_container1/struct.ProjectLocationClusterStartIpRotationCall.html), [*locations clusters update*](https://docs.rs/google-container1/1.0.7+20180917/google_container1/struct.ProjectLocationClusterUpdateCall.html), [*locations clusters update master*](https://docs.rs/google-container1/1.0.7+20180917/google_container1/struct.ProjectLocationClusterUpdateMasterCall.html), [*locations get server config*](https://docs.rs/google-container1/1.0.7+20180917/google_container1/struct.ProjectLocationGetServerConfigCall.html), [*locations operations cancel*](https://docs.rs/google-container1/1.0.7+20180917/google_container1/struct.ProjectLocationOperationCancelCall.html), [*locations operations get*](https://docs.rs/google-container1/1.0.7+20180917/google_container1/struct.ProjectLocationOperationGetCall.html), [*locations operations list*](https://docs.rs/google-container1/1.0.7+20180917/google_container1/struct.ProjectLocationOperationListCall.html), [*zones clusters addons*](https://docs.rs/google-container1/1.0.7+20180917/google_container1/struct.ProjectZoneClusterAddonCall.html), [*zones clusters complete ip rotation*](https://docs.rs/google-container1/1.0.7+20180917/google_container1/struct.ProjectZoneClusterCompleteIpRotationCall.html), [*zones clusters create*](https://docs.rs/google-container1/1.0.7+20180917/google_container1/struct.ProjectZoneClusterCreateCall.html), [*zones clusters delete*](https://docs.rs/google-container1/1.0.7+20180917/google_container1/struct.ProjectZoneClusterDeleteCall.html), [*zones clusters get*](https://docs.rs/google-container1/1.0.7+20180917/google_container1/struct.ProjectZoneClusterGetCall.html), [*zones clusters legacy abac*](https://docs.rs/google-container1/1.0.7+20180917/google_container1/struct.ProjectZoneClusterLegacyAbacCall.html), [*zones clusters list*](https://docs.rs/google-container1/1.0.7+20180917/google_container1/struct.ProjectZoneClusterListCall.html), [*zones clusters locations*](https://docs.rs/google-container1/1.0.7+20180917/google_container1/struct.ProjectZoneClusterLocationCall.html), [*zones clusters logging*](https://docs.rs/google-container1/1.0.7+20180917/google_container1/struct.ProjectZoneClusterLoggingCall.html), [*zones clusters master*](https://docs.rs/google-container1/1.0.7+20180917/google_container1/struct.ProjectZoneClusterMasterCall.html), [*zones clusters monitoring*](https://docs.rs/google-container1/1.0.7+20180917/google_container1/struct.ProjectZoneClusterMonitoringCall.html), [*zones clusters node pools autoscaling*](https://docs.rs/google-container1/1.0.7+20180917/google_container1/struct.ProjectZoneClusterNodePoolAutoscalingCall.html), [*zones clusters node pools create*](https://docs.rs/google-container1/1.0.7+20180917/google_container1/struct.ProjectZoneClusterNodePoolCreateCall.html), [*zones clusters node pools delete*](https://docs.rs/google-container1/1.0.7+20180917/google_container1/struct.ProjectZoneClusterNodePoolDeleteCall.html), [*zones clusters node pools get*](https://docs.rs/google-container1/1.0.7+20180917/google_container1/struct.ProjectZoneClusterNodePoolGetCall.html), [*zones clusters node pools list*](https://docs.rs/google-container1/1.0.7+20180917/google_container1/struct.ProjectZoneClusterNodePoolListCall.html), [*zones clusters node pools rollback*](https://docs.rs/google-container1/1.0.7+20180917/google_container1/struct.ProjectZoneClusterNodePoolRollbackCall.html), [*zones clusters node pools set management*](https://docs.rs/google-container1/1.0.7+20180917/google_container1/struct.ProjectZoneClusterNodePoolSetManagementCall.html), [*zones clusters node pools set size*](https://docs.rs/google-container1/1.0.7+20180917/google_container1/struct.ProjectZoneClusterNodePoolSetSizeCall.html), [*zones clusters node pools update*](https://docs.rs/google-container1/1.0.7+20180917/google_container1/struct.ProjectZoneClusterNodePoolUpdateCall.html), [*zones clusters resource labels*](https://docs.rs/google-container1/1.0.7+20180917/google_container1/struct.ProjectZoneClusterResourceLabelCall.html), [*zones clusters set maintenance policy*](https://docs.rs/google-container1/1.0.7+20180917/google_container1/struct.ProjectZoneClusterSetMaintenancePolicyCall.html), [*zones clusters set master auth*](https://docs.rs/google-container1/1.0.7+20180917/google_container1/struct.ProjectZoneClusterSetMasterAuthCall.html), [*zones clusters set network policy*](https://docs.rs/google-container1/1.0.7+20180917/google_container1/struct.ProjectZoneClusterSetNetworkPolicyCall.html), [*zones clusters start ip rotation*](https://docs.rs/google-container1/1.0.7+20180917/google_container1/struct.ProjectZoneClusterStartIpRotationCall.html), [*zones clusters update*](https://docs.rs/google-container1/1.0.7+20180917/google_container1/struct.ProjectZoneClusterUpdateCall.html), [*zones get serverconfig*](https://docs.rs/google-container1/1.0.7+20180917/google_container1/struct.ProjectZoneGetServerconfigCall.html), [*zones operations cancel*](https://docs.rs/google-container1/1.0.7+20180917/google_container1/struct.ProjectZoneOperationCancelCall.html), [*zones operations get*](https://docs.rs/google-container1/1.0.7+20180917/google_container1/struct.ProjectZoneOperationGetCall.html) and [*zones operations list*](https://docs.rs/google-container1/1.0.7+20180917/google_container1/struct.ProjectZoneOperationListCall.html)




# Structure of this Library

The API is structured into the following primary items:

* **[Hub](https://docs.rs/google-container1/1.0.7+20180917/google_container1/struct.Container.html)**
    * a central object to maintain state and allow accessing all *Activities*
    * creates [*Method Builders*](https://docs.rs/google-container1/1.0.7+20180917/google_container1/trait.MethodsBuilder.html) which in turn
      allow access to individual [*Call Builders*](https://docs.rs/google-container1/1.0.7+20180917/google_container1/trait.CallBuilder.html)
* **[Resources](https://docs.rs/google-container1/1.0.7+20180917/google_container1/trait.Resource.html)**
    * primary types that you can apply *Activities* to
    * a collection of properties and *Parts*
    * **[Parts](https://docs.rs/google-container1/1.0.7+20180917/google_container1/trait.Part.html)**
        * a collection of properties
        * never directly used in *Activities*
* **[Activities](https://docs.rs/google-container1/1.0.7+20180917/google_container1/trait.CallBuilder.html)**
    * operations to apply to *Resources*

All *structures* are marked with applicable traits to further categorize them and ease browsing.

Generally speaking, you can invoke *Activities* like this:

```Rust,ignore
let r = hub.resource().activity(...).doit()
```

Or specifically ...

```ignore
let r = hub.projects().zones_clusters_node_pools_rollback(...).doit()
let r = hub.projects().locations_clusters_node_pools_set_autoscaling(...).doit()
let r = hub.projects().zones_clusters_node_pools_create(...).doit()
let r = hub.projects().zones_clusters_monitoring(...).doit()
let r = hub.projects().locations_clusters_update(...).doit()
let r = hub.projects().zones_clusters_delete(...).doit()
let r = hub.projects().zones_clusters_legacy_abac(...).doit()
let r = hub.projects().zones_clusters_set_master_auth(...).doit()
let r = hub.projects().locations_clusters_delete(...).doit()
let r = hub.projects().locations_clusters_node_pools_delete(...).doit()
let r = hub.projects().zones_clusters_start_ip_rotation(...).doit()
let r = hub.projects().zones_clusters_create(...).doit()
let r = hub.projects().locations_clusters_set_addons(...).doit()
let r = hub.projects().locations_clusters_complete_ip_rotation(...).doit()
let r = hub.projects().locations_clusters_node_pools_update(...).doit()
let r = hub.projects().locations_clusters_node_pools_set_size(...).doit()
let r = hub.projects().locations_clusters_set_network_policy(...).doit()
let r = hub.projects().zones_clusters_node_pools_set_management(...).doit()
let r = hub.projects().zones_operations_get(...).doit()
let r = hub.projects().locations_clusters_set_monitoring(...).doit()
let r = hub.projects().locations_clusters_set_locations(...).doit()
let r = hub.projects().zones_clusters_logging(...).doit()
let r = hub.projects().zones_clusters_update(...).doit()
let r = hub.projects().zones_clusters_node_pools_autoscaling(...).doit()
let r = hub.projects().zones_clusters_node_pools_set_size(...).doit()
let r = hub.projects().locations_clusters_set_legacy_abac(...).doit()
let r = hub.projects().locations_clusters_create(...).doit()
let r = hub.projects().locations_clusters_node_pools_set_management(...).doit()
let r = hub.projects().locations_clusters_update_master(...).doit()
let r = hub.projects().zones_clusters_set_maintenance_policy(...).doit()
let r = hub.projects().locations_clusters_node_pools_rollback(...).doit()
let r = hub.projects().zones_clusters_node_pools_delete(...).doit()
let r = hub.projects().zones_clusters_locations(...).doit()
let r = hub.projects().locations_clusters_start_ip_rotation(...).doit()
let r = hub.projects().locations_clusters_set_resource_labels(...).doit()
let r = hub.projects().locations_clusters_node_pools_create(...).doit()
let r = hub.projects().zones_clusters_master(...).doit()
let r = hub.projects().zones_clusters_node_pools_update(...).doit()
let r = hub.projects().zones_clusters_set_network_policy(...).doit()
let r = hub.projects().locations_clusters_set_maintenance_policy(...).doit()
let r = hub.projects().zones_clusters_addons(...).doit()
let r = hub.projects().locations_clusters_set_master_auth(...).doit()
let r = hub.projects().zones_clusters_complete_ip_rotation(...).doit()
let r = hub.projects().zones_clusters_resource_labels(...).doit()
let r = hub.projects().locations_clusters_set_logging(...).doit()
let r = hub.projects().locations_operations_get(...).doit()
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
google-container1 = "*"
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
extern crate google_container1 as container1;
use container1::RollbackNodePoolUpgradeRequest;
use container1::{Result, Error};
use std::default::Default;
use oauth2::{Authenticator, DefaultAuthenticatorDelegate, ApplicationSecret, MemoryStorage};
use container1::Container;

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
let mut hub = Container::new(hyper::Client::with_connector(hyper::net::HttpsConnector::new(hyper_rustls::TlsClient::new())), auth);
// As the method needs a request, you would usually fill it with the desired information
// into the respective structure. Some of the parts shown here might not be applicable !
// Values shown here are possibly random and not representative !
let mut req = RollbackNodePoolUpgradeRequest::default();

// You can configure optional parameters by calling the respective setters at will, and
// execute the final call using `doit()`.
// Values shown here are possibly random and not representative !
let result = hub.projects().zones_clusters_node_pools_rollback(req, "projectId", "zone", "clusterId", "nodePoolId")
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

All errors produced by the system are provided either as [Result](https://docs.rs/google-container1/1.0.7+20180917/google_container1/enum.Result.html) enumeration as return value of 
the doit() methods, or handed as possibly intermediate results to either the 
[Hub Delegate](https://docs.rs/google-container1/1.0.7+20180917/google_container1/trait.Delegate.html), or the [Authenticator Delegate](https://docs.rs/yup-oauth2/*/yup_oauth2/trait.AuthenticatorDelegate.html).

When delegates handle errors or intermediate values, they may have a chance to instruct the system to retry. This 
makes the system potentially resilient to all kinds of errors.

## Uploads and Downloads
If a method supports downloads, the response body, which is part of the [Result](https://docs.rs/google-container1/1.0.7+20180917/google_container1/enum.Result.html), should be
read by you to obtain the media.
If such a method also supports a [Response Result](https://docs.rs/google-container1/1.0.7+20180917/google_container1/trait.ResponseResult.html), it will return that by default.
You can see it as meta-data for the actual media. To trigger a media download, you will have to set up the builder by making
this call: `.param("alt", "media")`.

Methods supporting uploads can do so using up to 2 different protocols: 
*simple* and *resumable*. The distinctiveness of each is represented by customized 
`doit(...)` methods, which are then named `upload(...)` and `upload_resumable(...)` respectively.

## Customization and Callbacks

You may alter the way an `doit()` method is called by providing a [delegate](https://docs.rs/google-container1/1.0.7+20180917/google_container1/trait.Delegate.html) to the 
[Method Builder](https://docs.rs/google-container1/1.0.7+20180917/google_container1/trait.CallBuilder.html) before making the final `doit()` call. 
Respective methods will be called to provide progress information, as well as determine whether the system should 
retry on failure.

The [delegate trait](https://docs.rs/google-container1/1.0.7+20180917/google_container1/trait.Delegate.html) is default-implemented, allowing you to customize it with minimal effort.

## Optional Parts in Server-Requests

All structures provided by this library are made to be [enocodable](https://docs.rs/google-container1/1.0.7+20180917/google_container1/trait.RequestValue.html) and 
[decodable](https://docs.rs/google-container1/1.0.7+20180917/google_container1/trait.ResponseResult.html) via *json*. Optionals are used to indicate that partial requests are responses 
are valid.
Most optionals are are considered [Parts](https://docs.rs/google-container1/1.0.7+20180917/google_container1/trait.Part.html) which are identifiable by name, which will be sent to 
the server to indicate either the set parts of the request or the desired parts in the response.

## Builder Arguments

Using [method builders](https://docs.rs/google-container1/1.0.7+20180917/google_container1/trait.CallBuilder.html), you are able to prepare an action call by repeatedly calling it's methods.
These will always take a single argument, for which the following statements are true.

* [PODs][wiki-pod] are handed by copy
* strings are passed as `&str`
* [request values](https://docs.rs/google-container1/1.0.7+20180917/google_container1/trait.RequestValue.html) are moved

Arguments will always be copied or cloned into the builder, to make them independent of their original life times.

[wiki-pod]: http://en.wikipedia.org/wiki/Plain_old_data_structure
[builder-pattern]: http://en.wikipedia.org/wiki/Builder_pattern
[google-go-api]: https://github.com/google/google-api-go-client

# License
The **container1** library was generated by Sebastian Thiel, and is placed 
under the *MIT* license.
You can read the full text at the repository's [license file][repo-license].

[repo-license]: https://github.com/Byron/google-apis-rsblob/master/LICENSE.md
