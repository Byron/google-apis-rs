// DO NOT EDIT !
// This file was generated automatically from 'src/mako/api/lib.rs.mako'
// DO NOT EDIT !

//! This documentation was generated from *compute* crate version *0.1.13+20160328*, where *20160328* is the exact revision of the *compute:v1* schema built by the [mako](http://www.makotemplates.org/) code generator *v0.1.13*.
//! 
//! Everything else about the *compute* *v1* API can be found at the
//! [official documentation site](https://developers.google.com/compute/docs/reference/latest/).
//! The original source code is [on github](https://github.com/Byron/google-apis-rs/tree/master/gen/compute1).
//! # Features
//! 
//! Handle the following *Resources* with ease from the central [hub](struct.Compute.html) ... 
//! 
//! * addresses
//!  * [*aggregated list*](struct.AddresseAggregatedListCall.html), [*delete*](struct.AddresseDeleteCall.html), [*get*](struct.AddresseGetCall.html), [*insert*](struct.AddresseInsertCall.html) and [*list*](struct.AddresseListCall.html)
//! * [autoscalers](struct.Autoscaler.html)
//!  * [*aggregated list*](struct.AutoscalerAggregatedListCall.html), [*delete*](struct.AutoscalerDeleteCall.html), [*get*](struct.AutoscalerGetCall.html), [*insert*](struct.AutoscalerInsertCall.html), [*list*](struct.AutoscalerListCall.html), [*patch*](struct.AutoscalerPatchCall.html) and [*update*](struct.AutoscalerUpdateCall.html)
//! * [backend services](struct.BackendService.html)
//!  * [*delete*](struct.BackendServiceDeleteCall.html), [*get*](struct.BackendServiceGetCall.html), [*get health*](struct.BackendServiceGetHealthCall.html), [*insert*](struct.BackendServiceInsertCall.html), [*list*](struct.BackendServiceListCall.html), [*patch*](struct.BackendServicePatchCall.html) and [*update*](struct.BackendServiceUpdateCall.html)
//! * [disk types](struct.DiskType.html)
//!  * [*aggregated list*](struct.DiskTypeAggregatedListCall.html), [*get*](struct.DiskTypeGetCall.html) and [*list*](struct.DiskTypeListCall.html)
//! * [disks](struct.Disk.html)
//!  * [*aggregated list*](struct.DiskAggregatedListCall.html), [*create snapshot*](struct.DiskCreateSnapshotCall.html), [*delete*](struct.DiskDeleteCall.html), [*get*](struct.DiskGetCall.html), [*insert*](struct.DiskInsertCall.html), [*list*](struct.DiskListCall.html) and [*resize*](struct.DiskResizeCall.html)
//! * [firewalls](struct.Firewall.html)
//!  * [*delete*](struct.FirewallDeleteCall.html), [*get*](struct.FirewallGetCall.html), [*insert*](struct.FirewallInsertCall.html), [*list*](struct.FirewallListCall.html), [*patch*](struct.FirewallPatchCall.html) and [*update*](struct.FirewallUpdateCall.html)
//! * [forwarding rules](struct.ForwardingRule.html)
//!  * [*aggregated list*](struct.ForwardingRuleAggregatedListCall.html), [*delete*](struct.ForwardingRuleDeleteCall.html), [*get*](struct.ForwardingRuleGetCall.html), [*insert*](struct.ForwardingRuleInsertCall.html), [*list*](struct.ForwardingRuleListCall.html) and [*set target*](struct.ForwardingRuleSetTargetCall.html)
//! * global addresses
//!  * [*delete*](struct.GlobalAddresseDeleteCall.html), [*get*](struct.GlobalAddresseGetCall.html), [*insert*](struct.GlobalAddresseInsertCall.html) and [*list*](struct.GlobalAddresseListCall.html)
//! * global forwarding rules
//!  * [*delete*](struct.GlobalForwardingRuleDeleteCall.html), [*get*](struct.GlobalForwardingRuleGetCall.html), [*insert*](struct.GlobalForwardingRuleInsertCall.html), [*list*](struct.GlobalForwardingRuleListCall.html) and [*set target*](struct.GlobalForwardingRuleSetTargetCall.html)
//! * global operations
//!  * [*aggregated list*](struct.GlobalOperationAggregatedListCall.html), [*delete*](struct.GlobalOperationDeleteCall.html), [*get*](struct.GlobalOperationGetCall.html) and [*list*](struct.GlobalOperationListCall.html)
//! * [http health checks](struct.HttpHealthCheck.html)
//!  * [*delete*](struct.HttpHealthCheckDeleteCall.html), [*get*](struct.HttpHealthCheckGetCall.html), [*insert*](struct.HttpHealthCheckInsertCall.html), [*list*](struct.HttpHealthCheckListCall.html), [*patch*](struct.HttpHealthCheckPatchCall.html) and [*update*](struct.HttpHealthCheckUpdateCall.html)
//! * [https health checks](struct.HttpsHealthCheck.html)
//!  * [*delete*](struct.HttpsHealthCheckDeleteCall.html), [*get*](struct.HttpsHealthCheckGetCall.html), [*insert*](struct.HttpsHealthCheckInsertCall.html), [*list*](struct.HttpsHealthCheckListCall.html), [*patch*](struct.HttpsHealthCheckPatchCall.html) and [*update*](struct.HttpsHealthCheckUpdateCall.html)
//! * [images](struct.Image.html)
//!  * [*delete*](struct.ImageDeleteCall.html), [*deprecate*](struct.ImageDeprecateCall.html), [*get*](struct.ImageGetCall.html), [*insert*](struct.ImageInsertCall.html) and [*list*](struct.ImageListCall.html)
//! * [instance group managers](struct.InstanceGroupManager.html)
//!  * [*abandon instances*](struct.InstanceGroupManagerAbandonInstanceCall.html), [*aggregated list*](struct.InstanceGroupManagerAggregatedListCall.html), [*delete*](struct.InstanceGroupManagerDeleteCall.html), [*delete instances*](struct.InstanceGroupManagerDeleteInstanceCall.html), [*get*](struct.InstanceGroupManagerGetCall.html), [*insert*](struct.InstanceGroupManagerInsertCall.html), [*list*](struct.InstanceGroupManagerListCall.html), [*list managed instances*](struct.InstanceGroupManagerListManagedInstanceCall.html), [*recreate instances*](struct.InstanceGroupManagerRecreateInstanceCall.html), [*resize*](struct.InstanceGroupManagerResizeCall.html), [*set instance template*](struct.InstanceGroupManagerSetInstanceTemplateCall.html) and [*set target pools*](struct.InstanceGroupManagerSetTargetPoolCall.html)
//! * [instance groups](struct.InstanceGroup.html)
//!  * [*add instances*](struct.InstanceGroupAddInstanceCall.html), [*aggregated list*](struct.InstanceGroupAggregatedListCall.html), [*delete*](struct.InstanceGroupDeleteCall.html), [*get*](struct.InstanceGroupGetCall.html), [*insert*](struct.InstanceGroupInsertCall.html), [*list*](struct.InstanceGroupListCall.html), [*list instances*](struct.InstanceGroupListInstanceCall.html), [*remove instances*](struct.InstanceGroupRemoveInstanceCall.html) and [*set named ports*](struct.InstanceGroupSetNamedPortCall.html)
//! * [instance templates](struct.InstanceTemplate.html)
//!  * [*delete*](struct.InstanceTemplateDeleteCall.html), [*get*](struct.InstanceTemplateGetCall.html), [*insert*](struct.InstanceTemplateInsertCall.html) and [*list*](struct.InstanceTemplateListCall.html)
//! * [instances](struct.Instance.html)
//!  * [*add access config*](struct.InstanceAddAccessConfigCall.html), [*aggregated list*](struct.InstanceAggregatedListCall.html), [*attach disk*](struct.InstanceAttachDiskCall.html), [*delete*](struct.InstanceDeleteCall.html), [*delete access config*](struct.InstanceDeleteAccessConfigCall.html), [*detach disk*](struct.InstanceDetachDiskCall.html), [*get*](struct.InstanceGetCall.html), [*get serial port output*](struct.InstanceGetSerialPortOutputCall.html), [*insert*](struct.InstanceInsertCall.html), [*list*](struct.InstanceListCall.html), [*reset*](struct.InstanceResetCall.html), [*set disk auto delete*](struct.InstanceSetDiskAutoDeleteCall.html), [*set machine type*](struct.InstanceSetMachineTypeCall.html), [*set metadata*](struct.InstanceSetMetadataCall.html), [*set scheduling*](struct.InstanceSetSchedulingCall.html), [*set tags*](struct.InstanceSetTagCall.html), [*start*](struct.InstanceStartCall.html) and [*stop*](struct.InstanceStopCall.html)
//! * [licenses](struct.License.html)
//!  * [*get*](struct.LicenseGetCall.html)
//! * [machine types](struct.MachineType.html)
//!  * [*aggregated list*](struct.MachineTypeAggregatedListCall.html), [*get*](struct.MachineTypeGetCall.html) and [*list*](struct.MachineTypeListCall.html)
//! * [networks](struct.Network.html)
//!  * [*delete*](struct.NetworkDeleteCall.html), [*get*](struct.NetworkGetCall.html), [*insert*](struct.NetworkInsertCall.html) and [*list*](struct.NetworkListCall.html)
//! * [projects](struct.Project.html)
//!  * [*get*](struct.ProjectGetCall.html), [*move disk*](struct.ProjectMoveDiskCall.html), [*move instance*](struct.ProjectMoveInstanceCall.html), [*set common instance metadata*](struct.ProjectSetCommonInstanceMetadataCall.html) and [*set usage export bucket*](struct.ProjectSetUsageExportBucketCall.html)
//! * region operations
//!  * [*delete*](struct.RegionOperationDeleteCall.html), [*get*](struct.RegionOperationGetCall.html) and [*list*](struct.RegionOperationListCall.html)
//! * [regions](struct.Region.html)
//!  * [*get*](struct.RegionGetCall.html) and [*list*](struct.RegionListCall.html)
//! * [routes](struct.Route.html)
//!  * [*delete*](struct.RouteDeleteCall.html), [*get*](struct.RouteGetCall.html), [*insert*](struct.RouteInsertCall.html) and [*list*](struct.RouteListCall.html)
//! * [snapshots](struct.Snapshot.html)
//!  * [*delete*](struct.SnapshotDeleteCall.html), [*get*](struct.SnapshotGetCall.html) and [*list*](struct.SnapshotListCall.html)
//! * [ssl certificates](struct.SslCertificate.html)
//!  * [*delete*](struct.SslCertificateDeleteCall.html), [*get*](struct.SslCertificateGetCall.html), [*insert*](struct.SslCertificateInsertCall.html) and [*list*](struct.SslCertificateListCall.html)
//! * [subnetworks](struct.Subnetwork.html)
//!  * [*aggregated list*](struct.SubnetworkAggregatedListCall.html), [*delete*](struct.SubnetworkDeleteCall.html), [*get*](struct.SubnetworkGetCall.html), [*insert*](struct.SubnetworkInsertCall.html) and [*list*](struct.SubnetworkListCall.html)
//! * [target http proxies](struct.TargetHttpProxy.html)
//!  * [*delete*](struct.TargetHttpProxyDeleteCall.html), [*get*](struct.TargetHttpProxyGetCall.html), [*insert*](struct.TargetHttpProxyInsertCall.html), [*list*](struct.TargetHttpProxyListCall.html) and [*set url map*](struct.TargetHttpProxySetUrlMapCall.html)
//! * [target https proxies](struct.TargetHttpsProxy.html)
//!  * [*delete*](struct.TargetHttpsProxyDeleteCall.html), [*get*](struct.TargetHttpsProxyGetCall.html), [*insert*](struct.TargetHttpsProxyInsertCall.html), [*list*](struct.TargetHttpsProxyListCall.html), [*set ssl certificates*](struct.TargetHttpsProxySetSslCertificateCall.html) and [*set url map*](struct.TargetHttpsProxySetUrlMapCall.html)
//! * [target instances](struct.TargetInstance.html)
//!  * [*aggregated list*](struct.TargetInstanceAggregatedListCall.html), [*delete*](struct.TargetInstanceDeleteCall.html), [*get*](struct.TargetInstanceGetCall.html), [*insert*](struct.TargetInstanceInsertCall.html) and [*list*](struct.TargetInstanceListCall.html)
//! * [target pools](struct.TargetPool.html)
//!  * [*add health check*](struct.TargetPoolAddHealthCheckCall.html), [*add instance*](struct.TargetPoolAddInstanceCall.html), [*aggregated list*](struct.TargetPoolAggregatedListCall.html), [*delete*](struct.TargetPoolDeleteCall.html), [*get*](struct.TargetPoolGetCall.html), [*get health*](struct.TargetPoolGetHealthCall.html), [*insert*](struct.TargetPoolInsertCall.html), [*list*](struct.TargetPoolListCall.html), [*remove health check*](struct.TargetPoolRemoveHealthCheckCall.html), [*remove instance*](struct.TargetPoolRemoveInstanceCall.html) and [*set backup*](struct.TargetPoolSetBackupCall.html)
//! * [target vpn gateways](struct.TargetVpnGateway.html)
//!  * [*aggregated list*](struct.TargetVpnGatewayAggregatedListCall.html), [*delete*](struct.TargetVpnGatewayDeleteCall.html), [*get*](struct.TargetVpnGatewayGetCall.html), [*insert*](struct.TargetVpnGatewayInsertCall.html) and [*list*](struct.TargetVpnGatewayListCall.html)
//! * [url maps](struct.UrlMap.html)
//!  * [*delete*](struct.UrlMapDeleteCall.html), [*get*](struct.UrlMapGetCall.html), [*insert*](struct.UrlMapInsertCall.html), [*list*](struct.UrlMapListCall.html), [*patch*](struct.UrlMapPatchCall.html), [*update*](struct.UrlMapUpdateCall.html) and [*validate*](struct.UrlMapValidateCall.html)
//! * [vpn tunnels](struct.VpnTunnel.html)
//!  * [*aggregated list*](struct.VpnTunnelAggregatedListCall.html), [*delete*](struct.VpnTunnelDeleteCall.html), [*get*](struct.VpnTunnelGetCall.html), [*insert*](struct.VpnTunnelInsertCall.html) and [*list*](struct.VpnTunnelListCall.html)
//! * zone operations
//!  * [*delete*](struct.ZoneOperationDeleteCall.html), [*get*](struct.ZoneOperationGetCall.html) and [*list*](struct.ZoneOperationListCall.html)
//! * [zones](struct.Zone.html)
//!  * [*get*](struct.ZoneGetCall.html) and [*list*](struct.ZoneListCall.html)
//! 
//! 
//! 
//! 
//! Not what you are looking for ? Find all other Google APIs in their Rust [documentation index](../index.html).
//! 
//! # Structure of this Library
//! 
//! The API is structured into the following primary items:
//! 
//! * **[Hub](struct.Compute.html)**
//!     * a central object to maintain state and allow accessing all *Activities*
//!     * creates [*Method Builders*](trait.MethodsBuilder.html) which in turn
//!       allow access to individual [*Call Builders*](trait.CallBuilder.html)
//! * **[Resources](trait.Resource.html)**
//!     * primary types that you can apply *Activities* to
//!     * a collection of properties and *Parts*
//!     * **[Parts](trait.Part.html)**
//!         * a collection of properties
//!         * never directly used in *Activities*
//! * **[Activities](trait.CallBuilder.html)**
//!     * operations to apply to *Resources*
//! 
//! All *structures* are marked with applicable traits to further categorize them and ease browsing.
//! 
//! Generally speaking, you can invoke *Activities* like this:
//! 
//! ```Rust,ignore
//! let r = hub.resource().activity(...).doit()
//! ```
//! 
//! Or specifically ...
//! 
//! ```ignore
//! let r = hub.instance_group_managers().resize(...).doit()
//! let r = hub.ssl_certificates().delete(...).doit()
//! let r = hub.autoscalers().patch(...).doit()
//! let r = hub.target_https_proxies().delete(...).doit()
//! let r = hub.firewalls().patch(...).doit()
//! let r = hub.vpn_tunnels().delete(...).doit()
//! let r = hub.backend_services().delete(...).doit()
//! let r = hub.global_forwarding_rules().set_target(...).doit()
//! let r = hub.instances().insert(...).doit()
//! let r = hub.global_forwarding_rules().delete(...).doit()
//! let r = hub.vpn_tunnels().insert(...).doit()
//! let r = hub.disks().delete(...).doit()
//! let r = hub.instance_groups().add_instances(...).doit()
//! let r = hub.target_http_proxies().delete(...).doit()
//! let r = hub.instances().set_scheduling(...).doit()
//! let r = hub.instances().delete(...).doit()
//! let r = hub.target_pools().add_health_check(...).doit()
//! let r = hub.instance_group_managers().insert(...).doit()
//! let r = hub.global_addresses().insert(...).doit()
//! let r = hub.https_health_checks().delete(...).doit()
//! let r = hub.autoscalers().insert(...).doit()
//! let r = hub.instance_groups().insert(...).doit()
//! let r = hub.instances().set_tags(...).doit()
//! let r = hub.instance_group_managers().delete_instances(...).doit()
//! let r = hub.instance_group_managers().set_instance_template(...).doit()
//! let r = hub.disks().resize(...).doit()
//! let r = hub.target_pools().insert(...).doit()
//! let r = hub.instances().set_disk_auto_delete(...).doit()
//! let r = hub.instance_group_managers().set_target_pools(...).doit()
//! let r = hub.disks().insert(...).doit()
//! let r = hub.https_health_checks().insert(...).doit()
//! let r = hub.target_pools().remove_health_check(...).doit()
//! let r = hub.target_https_proxies().set_url_map(...).doit()
//! let r = hub.instances().set_machine_type(...).doit()
//! let r = hub.url_maps().insert(...).doit()
//! let r = hub.instances().add_access_config(...).doit()
//! let r = hub.addresses().insert(...).doit()
//! let r = hub.global_forwarding_rules().insert(...).doit()
//! let r = hub.global_operations().get(...).doit()
//! let r = hub.addresses().delete(...).doit()
//! let r = hub.target_vpn_gateways().insert(...).doit()
//! let r = hub.disks().create_snapshot(...).doit()
//! let r = hub.instance_groups().remove_instances(...).doit()
//! let r = hub.instances().set_metadata(...).doit()
//! let r = hub.forwarding_rules().insert(...).doit()
//! let r = hub.instances().start(...).doit()
//! let r = hub.instance_group_managers().delete(...).doit()
//! let r = hub.target_instances().insert(...).doit()
//! let r = hub.forwarding_rules().set_target(...).doit()
//! let r = hub.projects().move_disk(...).doit()
//! let r = hub.firewalls().insert(...).doit()
//! let r = hub.target_pools().set_backup(...).doit()
//! let r = hub.routes().insert(...).doit()
//! let r = hub.instance_groups().delete(...).doit()
//! let r = hub.instances().delete_access_config(...).doit()
//! let r = hub.target_https_proxies().set_ssl_certificates(...).doit()
//! let r = hub.networks().insert(...).doit()
//! let r = hub.instance_group_managers().abandon_instances(...).doit()
//! let r = hub.subnetworks().insert(...).doit()
//! let r = hub.snapshots().delete(...).doit()
//! let r = hub.target_https_proxies().insert(...).doit()
//! let r = hub.instances().detach_disk(...).doit()
//! let r = hub.backend_services().update(...).doit()
//! let r = hub.instance_group_managers().recreate_instances(...).doit()
//! let r = hub.images().delete(...).doit()
//! let r = hub.projects().set_common_instance_metadata(...).doit()
//! let r = hub.region_operations().get(...).doit()
//! let r = hub.backend_services().patch(...).doit()
//! let r = hub.target_http_proxies().set_url_map(...).doit()
//! let r = hub.images().deprecate(...).doit()
//! let r = hub.http_health_checks().patch(...).doit()
//! let r = hub.images().insert(...).doit()
//! let r = hub.instance_groups().set_named_ports(...).doit()
//! let r = hub.ssl_certificates().insert(...).doit()
//! let r = hub.projects().move_instance(...).doit()
//! let r = hub.autoscalers().delete(...).doit()
//! let r = hub.https_health_checks().update(...).doit()
//! let r = hub.url_maps().patch(...).doit()
//! let r = hub.subnetworks().delete(...).doit()
//! let r = hub.instances().stop(...).doit()
//! let r = hub.target_pools().add_instance(...).doit()
//! let r = hub.target_pools().remove_instance(...).doit()
//! let r = hub.target_pools().delete(...).doit()
//! let r = hub.firewalls().update(...).doit()
//! let r = hub.instance_templates().delete(...).doit()
//! let r = hub.projects().set_usage_export_bucket(...).doit()
//! let r = hub.target_http_proxies().insert(...).doit()
//! let r = hub.url_maps().update(...).doit()
//! let r = hub.instance_templates().insert(...).doit()
//! let r = hub.target_instances().delete(...).doit()
//! let r = hub.target_vpn_gateways().delete(...).doit()
//! let r = hub.global_addresses().delete(...).doit()
//! let r = hub.https_health_checks().patch(...).doit()
//! let r = hub.networks().delete(...).doit()
//! let r = hub.url_maps().delete(...).doit()
//! let r = hub.instances().reset(...).doit()
//! let r = hub.backend_services().insert(...).doit()
//! let r = hub.http_health_checks().delete(...).doit()
//! let r = hub.http_health_checks().insert(...).doit()
//! let r = hub.instances().attach_disk(...).doit()
//! let r = hub.autoscalers().update(...).doit()
//! let r = hub.forwarding_rules().delete(...).doit()
//! let r = hub.firewalls().delete(...).doit()
//! let r = hub.zone_operations().get(...).doit()
//! let r = hub.http_health_checks().update(...).doit()
//! let r = hub.routes().delete(...).doit()
//! ```
//! 
//! The `resource()` and `activity(...)` calls create [builders][builder-pattern]. The second one dealing with `Activities` 
//! supports various methods to configure the impending operation (not shown here). It is made such that all required arguments have to be 
//! specified right away (i.e. `(...)`), whereas all optional ones can be [build up][builder-pattern] as desired.
//! The `doit()` method performs the actual communication with the server and returns the respective result.
//! 
//! # Usage
//! 
//! ## Setting up your Project
//! 
//! To use this library, you would put the following lines into your `Cargo.toml` file:
//! 
//! ```toml
//! [dependencies]
//! google-compute1 = "*"
//! ```
//! 
//! ## A complete example
//! 
//! ```test_harness,no_run
//! extern crate hyper;
//! extern crate yup_oauth2 as oauth2;
//! extern crate google_compute1 as compute1;
//! use compute1::{Result, Error};
//! # #[test] fn egal() {
//! use std::default::Default;
//! use oauth2::{Authenticator, DefaultAuthenticatorDelegate, ApplicationSecret, MemoryStorage};
//! use compute1::Compute;
//! 
//! // Get an ApplicationSecret instance by some means. It contains the `client_id` and 
//! // `client_secret`, among other things.
//! let secret: ApplicationSecret = Default::default();
//! // Instantiate the authenticator. It will choose a suitable authentication flow for you, 
//! // unless you replace  `None` with the desired Flow.
//! // Provide your own `AuthenticatorDelegate` to adjust the way it operates and get feedback about 
//! // what's going on. You probably want to bring in your own `TokenStorage` to persist tokens and
//! // retrieve them from storage.
//! let auth = Authenticator::new(&secret, DefaultAuthenticatorDelegate,
//!                               hyper::Client::new(),
//!                               <MemoryStorage as Default>::default(), None);
//! let mut hub = Compute::new(hyper::Client::new(), auth);
//! // You can configure optional parameters by calling the respective setters at will, and
//! // execute the final call using `doit()`.
//! // Values shown here are possibly random and not representative !
//! let result = hub.instances().set_disk_auto_delete("project", "zone", "instance", false, "deviceName")
//!              .doit();
//! 
//! match result {
//!     Err(e) => match e {
//!         // The Error enum provides details about what exactly happened.
//!         // You can also just use its `Debug`, `Display` or `Error` traits
//!          Error::HttpError(_)
//!         |Error::MissingAPIKey
//!         |Error::MissingToken(_)
//!         |Error::Cancelled
//!         |Error::UploadSizeLimitExceeded(_, _)
//!         |Error::Failure(_)
//!         |Error::BadRequest(_)
//!         |Error::FieldClash(_)
//!         |Error::JsonDecodeError(_, _) => println!("{}", e),
//!     },
//!     Ok(res) => println!("Success: {:?}", res),
//! }
//! # }
//! ```
//! ## Handling Errors
//! 
//! All errors produced by the system are provided either as [Result](enum.Result.html) enumeration as return value of 
//! the doit() methods, or handed as possibly intermediate results to either the 
//! [Hub Delegate](trait.Delegate.html), or the [Authenticator Delegate](../yup-oauth2/trait.AuthenticatorDelegate.html).
//! 
//! When delegates handle errors or intermediate values, they may have a chance to instruct the system to retry. This 
//! makes the system potentially resilient to all kinds of errors.
//! 
//! ## Uploads and Downloads
//! If a method supports downloads, the response body, which is part of the [Result](enum.Result.html), should be
//! read by you to obtain the media.
//! If such a method also supports a [Response Result](trait.ResponseResult.html), it will return that by default.
//! You can see it as meta-data for the actual media. To trigger a media download, you will have to set up the builder by making
//! this call: `.param("alt", "media")`.
//! 
//! Methods supporting uploads can do so using up to 2 different protocols: 
//! *simple* and *resumable*. The distinctiveness of each is represented by customized 
//! `doit(...)` methods, which are then named `upload(...)` and `upload_resumable(...)` respectively.
//! 
//! ## Customization and Callbacks
//! 
//! You may alter the way an `doit()` method is called by providing a [delegate](trait.Delegate.html) to the 
//! [Method Builder](trait.CallBuilder.html) before making the final `doit()` call. 
//! Respective methods will be called to provide progress information, as well as determine whether the system should 
//! retry on failure.
//! 
//! The [delegate trait](trait.Delegate.html) is default-implemented, allowing you to customize it with minimal effort.
//! 
//! ## Optional Parts in Server-Requests
//! 
//! All structures provided by this library are made to be [enocodable](trait.RequestValue.html) and 
//! [decodable](trait.ResponseResult.html) via *json*. Optionals are used to indicate that partial requests are responses 
//! are valid.
//! Most optionals are are considered [Parts](trait.Part.html) which are identifiable by name, which will be sent to 
//! the server to indicate either the set parts of the request or the desired parts in the response.
//! 
//! ## Builder Arguments
//! 
//! Using [method builders](trait.CallBuilder.html), you are able to prepare an action call by repeatedly calling it's methods.
//! These will always take a single argument, for which the following statements are true.
//! 
//! * [PODs][wiki-pod] are handed by copy
//! * strings are passed as `&str`
//! * [request values](trait.RequestValue.html) are moved
//! 
//! Arguments will always be copied or cloned into the builder, to make them independent of their original life times.
//! 
//! [wiki-pod]: http://en.wikipedia.org/wiki/Plain_old_data_structure
//! [builder-pattern]: http://en.wikipedia.org/wiki/Builder_pattern
//! [google-go-api]: https://github.com/google/google-api-go-client
//! 
//! 

// Unused attributes happen thanks to defined, but unused structures
// We don't warn about this, as depending on the API, some data structures or facilities are never used.
// Instead of pre-determining this, we just disable the lint. It's manually tuned to not have any 
// unused imports in fully featured APIs. Same with unused_mut ... .
#![cfg_attr(feature = "nightly", feature(custom_derive, custom_attribute, plugin))]
#![cfg_attr(feature = "nightly", plugin(serde_macros))]
#![allow(unused_imports, unused_mut, dead_code)]

#[cfg(feature = "nightly")]
include!("lib.rs.in");

#[cfg(feature = "with-syntex")]
include!(concat!(env!("OUT_DIR"), "/lib.rs"));