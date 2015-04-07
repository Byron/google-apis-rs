<!---
DO NOT EDIT !
This file was generated automatically from 'src/mako/api/README.md.mako'
DO NOT EDIT !
-->
The `google-compute1` library allows access to all features of the *Google compute* service.

This documentation was generated from *compute* crate version *0.1.3+20150326*, where *20150326* is the exact revision of the *compute:v1* schema built by the [mako](http://www.makotemplates.org/) code generator *v0.1.3*.

Everything else about the *compute* *v1* API can be found at the
[official documentation site](https://developers.google.com/compute/docs/reference/latest/).
# Features

Handle the following *Resources* with ease from the central [hub](http://byron.github.io/google-apis-rs/google-compute1/struct.Compute.html) ... 

* addresses
 * [*aggregated list*](http://byron.github.io/google-apis-rs/google-compute1/struct.AddresseAggregatedListCall.html), [*delete*](http://byron.github.io/google-apis-rs/google-compute1/struct.AddresseDeleteCall.html), [*get*](http://byron.github.io/google-apis-rs/google-compute1/struct.AddresseGetCall.html), [*insert*](http://byron.github.io/google-apis-rs/google-compute1/struct.AddresseInsertCall.html) and [*list*](http://byron.github.io/google-apis-rs/google-compute1/struct.AddresseListCall.html)
* [backend services](http://byron.github.io/google-apis-rs/google-compute1/struct.BackendService.html)
 * [*delete*](http://byron.github.io/google-apis-rs/google-compute1/struct.BackendServiceDeleteCall.html), [*get*](http://byron.github.io/google-apis-rs/google-compute1/struct.BackendServiceGetCall.html), [*get health*](http://byron.github.io/google-apis-rs/google-compute1/struct.BackendServiceGetHealthCall.html), [*insert*](http://byron.github.io/google-apis-rs/google-compute1/struct.BackendServiceInsertCall.html), [*list*](http://byron.github.io/google-apis-rs/google-compute1/struct.BackendServiceListCall.html), [*patch*](http://byron.github.io/google-apis-rs/google-compute1/struct.BackendServicePatchCall.html) and [*update*](http://byron.github.io/google-apis-rs/google-compute1/struct.BackendServiceUpdateCall.html)
* [disk types](http://byron.github.io/google-apis-rs/google-compute1/struct.DiskType.html)
 * [*aggregated list*](http://byron.github.io/google-apis-rs/google-compute1/struct.DiskTypeAggregatedListCall.html), [*get*](http://byron.github.io/google-apis-rs/google-compute1/struct.DiskTypeGetCall.html) and [*list*](http://byron.github.io/google-apis-rs/google-compute1/struct.DiskTypeListCall.html)
* [disks](http://byron.github.io/google-apis-rs/google-compute1/struct.Disk.html)
 * [*aggregated list*](http://byron.github.io/google-apis-rs/google-compute1/struct.DiskAggregatedListCall.html), [*create snapshot*](http://byron.github.io/google-apis-rs/google-compute1/struct.DiskCreateSnapshotCall.html), [*delete*](http://byron.github.io/google-apis-rs/google-compute1/struct.DiskDeleteCall.html), [*get*](http://byron.github.io/google-apis-rs/google-compute1/struct.DiskGetCall.html), [*insert*](http://byron.github.io/google-apis-rs/google-compute1/struct.DiskInsertCall.html) and [*list*](http://byron.github.io/google-apis-rs/google-compute1/struct.DiskListCall.html)
* [firewalls](http://byron.github.io/google-apis-rs/google-compute1/struct.Firewall.html)
 * [*delete*](http://byron.github.io/google-apis-rs/google-compute1/struct.FirewallDeleteCall.html), [*get*](http://byron.github.io/google-apis-rs/google-compute1/struct.FirewallGetCall.html), [*insert*](http://byron.github.io/google-apis-rs/google-compute1/struct.FirewallInsertCall.html), [*list*](http://byron.github.io/google-apis-rs/google-compute1/struct.FirewallListCall.html), [*patch*](http://byron.github.io/google-apis-rs/google-compute1/struct.FirewallPatchCall.html) and [*update*](http://byron.github.io/google-apis-rs/google-compute1/struct.FirewallUpdateCall.html)
* [forwarding rules](http://byron.github.io/google-apis-rs/google-compute1/struct.ForwardingRule.html)
 * [*aggregated list*](http://byron.github.io/google-apis-rs/google-compute1/struct.ForwardingRuleAggregatedListCall.html), [*delete*](http://byron.github.io/google-apis-rs/google-compute1/struct.ForwardingRuleDeleteCall.html), [*get*](http://byron.github.io/google-apis-rs/google-compute1/struct.ForwardingRuleGetCall.html), [*insert*](http://byron.github.io/google-apis-rs/google-compute1/struct.ForwardingRuleInsertCall.html), [*list*](http://byron.github.io/google-apis-rs/google-compute1/struct.ForwardingRuleListCall.html) and [*set target*](http://byron.github.io/google-apis-rs/google-compute1/struct.ForwardingRuleSetTargetCall.html)
* global addresses
 * [*delete*](http://byron.github.io/google-apis-rs/google-compute1/struct.GlobalAddresseDeleteCall.html), [*get*](http://byron.github.io/google-apis-rs/google-compute1/struct.GlobalAddresseGetCall.html), [*insert*](http://byron.github.io/google-apis-rs/google-compute1/struct.GlobalAddresseInsertCall.html) and [*list*](http://byron.github.io/google-apis-rs/google-compute1/struct.GlobalAddresseListCall.html)
* global forwarding rules
 * [*delete*](http://byron.github.io/google-apis-rs/google-compute1/struct.GlobalForwardingRuleDeleteCall.html), [*get*](http://byron.github.io/google-apis-rs/google-compute1/struct.GlobalForwardingRuleGetCall.html), [*insert*](http://byron.github.io/google-apis-rs/google-compute1/struct.GlobalForwardingRuleInsertCall.html), [*list*](http://byron.github.io/google-apis-rs/google-compute1/struct.GlobalForwardingRuleListCall.html) and [*set target*](http://byron.github.io/google-apis-rs/google-compute1/struct.GlobalForwardingRuleSetTargetCall.html)
* global operations
 * [*aggregated list*](http://byron.github.io/google-apis-rs/google-compute1/struct.GlobalOperationAggregatedListCall.html), [*delete*](http://byron.github.io/google-apis-rs/google-compute1/struct.GlobalOperationDeleteCall.html), [*get*](http://byron.github.io/google-apis-rs/google-compute1/struct.GlobalOperationGetCall.html) and [*list*](http://byron.github.io/google-apis-rs/google-compute1/struct.GlobalOperationListCall.html)
* [http health checks](http://byron.github.io/google-apis-rs/google-compute1/struct.HttpHealthCheck.html)
 * [*delete*](http://byron.github.io/google-apis-rs/google-compute1/struct.HttpHealthCheckDeleteCall.html), [*get*](http://byron.github.io/google-apis-rs/google-compute1/struct.HttpHealthCheckGetCall.html), [*insert*](http://byron.github.io/google-apis-rs/google-compute1/struct.HttpHealthCheckInsertCall.html), [*list*](http://byron.github.io/google-apis-rs/google-compute1/struct.HttpHealthCheckListCall.html), [*patch*](http://byron.github.io/google-apis-rs/google-compute1/struct.HttpHealthCheckPatchCall.html) and [*update*](http://byron.github.io/google-apis-rs/google-compute1/struct.HttpHealthCheckUpdateCall.html)
* [images](http://byron.github.io/google-apis-rs/google-compute1/struct.Image.html)
 * [*delete*](http://byron.github.io/google-apis-rs/google-compute1/struct.ImageDeleteCall.html), [*deprecate*](http://byron.github.io/google-apis-rs/google-compute1/struct.ImageDeprecateCall.html), [*get*](http://byron.github.io/google-apis-rs/google-compute1/struct.ImageGetCall.html), [*insert*](http://byron.github.io/google-apis-rs/google-compute1/struct.ImageInsertCall.html) and [*list*](http://byron.github.io/google-apis-rs/google-compute1/struct.ImageListCall.html)
* [instance templates](http://byron.github.io/google-apis-rs/google-compute1/struct.InstanceTemplate.html)
 * [*delete*](http://byron.github.io/google-apis-rs/google-compute1/struct.InstanceTemplateDeleteCall.html), [*get*](http://byron.github.io/google-apis-rs/google-compute1/struct.InstanceTemplateGetCall.html), [*insert*](http://byron.github.io/google-apis-rs/google-compute1/struct.InstanceTemplateInsertCall.html) and [*list*](http://byron.github.io/google-apis-rs/google-compute1/struct.InstanceTemplateListCall.html)
* [instances](http://byron.github.io/google-apis-rs/google-compute1/struct.Instance.html)
 * [*add access config*](http://byron.github.io/google-apis-rs/google-compute1/struct.InstanceAddAccessConfigCall.html), [*aggregated list*](http://byron.github.io/google-apis-rs/google-compute1/struct.InstanceAggregatedListCall.html), [*attach disk*](http://byron.github.io/google-apis-rs/google-compute1/struct.InstanceAttachDiskCall.html), [*delete*](http://byron.github.io/google-apis-rs/google-compute1/struct.InstanceDeleteCall.html), [*delete access config*](http://byron.github.io/google-apis-rs/google-compute1/struct.InstanceDeleteAccessConfigCall.html), [*detach disk*](http://byron.github.io/google-apis-rs/google-compute1/struct.InstanceDetachDiskCall.html), [*get*](http://byron.github.io/google-apis-rs/google-compute1/struct.InstanceGetCall.html), [*get serial port output*](http://byron.github.io/google-apis-rs/google-compute1/struct.InstanceGetSerialPortOutputCall.html), [*insert*](http://byron.github.io/google-apis-rs/google-compute1/struct.InstanceInsertCall.html), [*list*](http://byron.github.io/google-apis-rs/google-compute1/struct.InstanceListCall.html), [*reset*](http://byron.github.io/google-apis-rs/google-compute1/struct.InstanceResetCall.html), [*set disk auto delete*](http://byron.github.io/google-apis-rs/google-compute1/struct.InstanceSetDiskAutoDeleteCall.html), [*set metadata*](http://byron.github.io/google-apis-rs/google-compute1/struct.InstanceSetMetadataCall.html), [*set scheduling*](http://byron.github.io/google-apis-rs/google-compute1/struct.InstanceSetSchedulingCall.html), [*set tags*](http://byron.github.io/google-apis-rs/google-compute1/struct.InstanceSetTagCall.html), [*start*](http://byron.github.io/google-apis-rs/google-compute1/struct.InstanceStartCall.html) and [*stop*](http://byron.github.io/google-apis-rs/google-compute1/struct.InstanceStopCall.html)
* [licenses](http://byron.github.io/google-apis-rs/google-compute1/struct.License.html)
 * [*get*](http://byron.github.io/google-apis-rs/google-compute1/struct.LicenseGetCall.html)
* [machine types](http://byron.github.io/google-apis-rs/google-compute1/struct.MachineType.html)
 * [*aggregated list*](http://byron.github.io/google-apis-rs/google-compute1/struct.MachineTypeAggregatedListCall.html), [*get*](http://byron.github.io/google-apis-rs/google-compute1/struct.MachineTypeGetCall.html) and [*list*](http://byron.github.io/google-apis-rs/google-compute1/struct.MachineTypeListCall.html)
* [networks](http://byron.github.io/google-apis-rs/google-compute1/struct.Network.html)
 * [*delete*](http://byron.github.io/google-apis-rs/google-compute1/struct.NetworkDeleteCall.html), [*get*](http://byron.github.io/google-apis-rs/google-compute1/struct.NetworkGetCall.html), [*insert*](http://byron.github.io/google-apis-rs/google-compute1/struct.NetworkInsertCall.html) and [*list*](http://byron.github.io/google-apis-rs/google-compute1/struct.NetworkListCall.html)
* [projects](http://byron.github.io/google-apis-rs/google-compute1/struct.Project.html)
 * [*get*](http://byron.github.io/google-apis-rs/google-compute1/struct.ProjectGetCall.html), [*move disk*](http://byron.github.io/google-apis-rs/google-compute1/struct.ProjectMoveDiskCall.html), [*move instance*](http://byron.github.io/google-apis-rs/google-compute1/struct.ProjectMoveInstanceCall.html), [*set common instance metadata*](http://byron.github.io/google-apis-rs/google-compute1/struct.ProjectSetCommonInstanceMetadataCall.html) and [*set usage export bucket*](http://byron.github.io/google-apis-rs/google-compute1/struct.ProjectSetUsageExportBucketCall.html)
* region operations
 * [*delete*](http://byron.github.io/google-apis-rs/google-compute1/struct.RegionOperationDeleteCall.html), [*get*](http://byron.github.io/google-apis-rs/google-compute1/struct.RegionOperationGetCall.html) and [*list*](http://byron.github.io/google-apis-rs/google-compute1/struct.RegionOperationListCall.html)
* [regions](http://byron.github.io/google-apis-rs/google-compute1/struct.Region.html)
 * [*get*](http://byron.github.io/google-apis-rs/google-compute1/struct.RegionGetCall.html) and [*list*](http://byron.github.io/google-apis-rs/google-compute1/struct.RegionListCall.html)
* [routes](http://byron.github.io/google-apis-rs/google-compute1/struct.Route.html)
 * [*delete*](http://byron.github.io/google-apis-rs/google-compute1/struct.RouteDeleteCall.html), [*get*](http://byron.github.io/google-apis-rs/google-compute1/struct.RouteGetCall.html), [*insert*](http://byron.github.io/google-apis-rs/google-compute1/struct.RouteInsertCall.html) and [*list*](http://byron.github.io/google-apis-rs/google-compute1/struct.RouteListCall.html)
* [snapshots](http://byron.github.io/google-apis-rs/google-compute1/struct.Snapshot.html)
 * [*delete*](http://byron.github.io/google-apis-rs/google-compute1/struct.SnapshotDeleteCall.html), [*get*](http://byron.github.io/google-apis-rs/google-compute1/struct.SnapshotGetCall.html) and [*list*](http://byron.github.io/google-apis-rs/google-compute1/struct.SnapshotListCall.html)
* [target http proxies](http://byron.github.io/google-apis-rs/google-compute1/struct.TargetHttpProxy.html)
 * [*delete*](http://byron.github.io/google-apis-rs/google-compute1/struct.TargetHttpProxyDeleteCall.html), [*get*](http://byron.github.io/google-apis-rs/google-compute1/struct.TargetHttpProxyGetCall.html), [*insert*](http://byron.github.io/google-apis-rs/google-compute1/struct.TargetHttpProxyInsertCall.html), [*list*](http://byron.github.io/google-apis-rs/google-compute1/struct.TargetHttpProxyListCall.html) and [*set url map*](http://byron.github.io/google-apis-rs/google-compute1/struct.TargetHttpProxySetUrlMapCall.html)
* [target instances](http://byron.github.io/google-apis-rs/google-compute1/struct.TargetInstance.html)
 * [*aggregated list*](http://byron.github.io/google-apis-rs/google-compute1/struct.TargetInstanceAggregatedListCall.html), [*delete*](http://byron.github.io/google-apis-rs/google-compute1/struct.TargetInstanceDeleteCall.html), [*get*](http://byron.github.io/google-apis-rs/google-compute1/struct.TargetInstanceGetCall.html), [*insert*](http://byron.github.io/google-apis-rs/google-compute1/struct.TargetInstanceInsertCall.html) and [*list*](http://byron.github.io/google-apis-rs/google-compute1/struct.TargetInstanceListCall.html)
* [target pools](http://byron.github.io/google-apis-rs/google-compute1/struct.TargetPool.html)
 * [*add health check*](http://byron.github.io/google-apis-rs/google-compute1/struct.TargetPoolAddHealthCheckCall.html), [*add instance*](http://byron.github.io/google-apis-rs/google-compute1/struct.TargetPoolAddInstanceCall.html), [*aggregated list*](http://byron.github.io/google-apis-rs/google-compute1/struct.TargetPoolAggregatedListCall.html), [*delete*](http://byron.github.io/google-apis-rs/google-compute1/struct.TargetPoolDeleteCall.html), [*get*](http://byron.github.io/google-apis-rs/google-compute1/struct.TargetPoolGetCall.html), [*get health*](http://byron.github.io/google-apis-rs/google-compute1/struct.TargetPoolGetHealthCall.html), [*insert*](http://byron.github.io/google-apis-rs/google-compute1/struct.TargetPoolInsertCall.html), [*list*](http://byron.github.io/google-apis-rs/google-compute1/struct.TargetPoolListCall.html), [*remove health check*](http://byron.github.io/google-apis-rs/google-compute1/struct.TargetPoolRemoveHealthCheckCall.html), [*remove instance*](http://byron.github.io/google-apis-rs/google-compute1/struct.TargetPoolRemoveInstanceCall.html) and [*set backup*](http://byron.github.io/google-apis-rs/google-compute1/struct.TargetPoolSetBackupCall.html)
* [target vpn gateways](http://byron.github.io/google-apis-rs/google-compute1/struct.TargetVpnGateway.html)
 * [*aggregated list*](http://byron.github.io/google-apis-rs/google-compute1/struct.TargetVpnGatewayAggregatedListCall.html), [*delete*](http://byron.github.io/google-apis-rs/google-compute1/struct.TargetVpnGatewayDeleteCall.html), [*get*](http://byron.github.io/google-apis-rs/google-compute1/struct.TargetVpnGatewayGetCall.html), [*insert*](http://byron.github.io/google-apis-rs/google-compute1/struct.TargetVpnGatewayInsertCall.html) and [*list*](http://byron.github.io/google-apis-rs/google-compute1/struct.TargetVpnGatewayListCall.html)
* [url maps](http://byron.github.io/google-apis-rs/google-compute1/struct.UrlMap.html)
 * [*delete*](http://byron.github.io/google-apis-rs/google-compute1/struct.UrlMapDeleteCall.html), [*get*](http://byron.github.io/google-apis-rs/google-compute1/struct.UrlMapGetCall.html), [*insert*](http://byron.github.io/google-apis-rs/google-compute1/struct.UrlMapInsertCall.html), [*list*](http://byron.github.io/google-apis-rs/google-compute1/struct.UrlMapListCall.html), [*patch*](http://byron.github.io/google-apis-rs/google-compute1/struct.UrlMapPatchCall.html), [*update*](http://byron.github.io/google-apis-rs/google-compute1/struct.UrlMapUpdateCall.html) and [*validate*](http://byron.github.io/google-apis-rs/google-compute1/struct.UrlMapValidateCall.html)
* [vpn tunnels](http://byron.github.io/google-apis-rs/google-compute1/struct.VpnTunnel.html)
 * [*aggregated list*](http://byron.github.io/google-apis-rs/google-compute1/struct.VpnTunnelAggregatedListCall.html), [*delete*](http://byron.github.io/google-apis-rs/google-compute1/struct.VpnTunnelDeleteCall.html), [*get*](http://byron.github.io/google-apis-rs/google-compute1/struct.VpnTunnelGetCall.html), [*insert*](http://byron.github.io/google-apis-rs/google-compute1/struct.VpnTunnelInsertCall.html) and [*list*](http://byron.github.io/google-apis-rs/google-compute1/struct.VpnTunnelListCall.html)
* zone operations
 * [*delete*](http://byron.github.io/google-apis-rs/google-compute1/struct.ZoneOperationDeleteCall.html), [*get*](http://byron.github.io/google-apis-rs/google-compute1/struct.ZoneOperationGetCall.html) and [*list*](http://byron.github.io/google-apis-rs/google-compute1/struct.ZoneOperationListCall.html)
* [zones](http://byron.github.io/google-apis-rs/google-compute1/struct.Zone.html)
 * [*get*](http://byron.github.io/google-apis-rs/google-compute1/struct.ZoneGetCall.html) and [*list*](http://byron.github.io/google-apis-rs/google-compute1/struct.ZoneListCall.html)




# Structure of this Library

The API is structured into the following primary items:

* **[Hub](http://byron.github.io/google-apis-rs/google-compute1/struct.Compute.html)**
    * a central object to maintain state and allow accessing all *Activities*
    * creates [*Method Builders*](http://byron.github.io/google-apis-rs/google-compute1/trait.MethodsBuilder.html) which in turn
      allow access to individual [*Call Builders*](http://byron.github.io/google-apis-rs/google-compute1/trait.CallBuilder.html)
* **[Resources](http://byron.github.io/google-apis-rs/google-compute1/trait.Resource.html)**
    * primary types that you can apply *Activities* to
    * a collection of properties and *Parts*
    * **[Parts](http://byron.github.io/google-apis-rs/google-compute1/trait.Part.html)**
        * a collection of properties
        * never directly used in *Activities*
* **[Activities](http://byron.github.io/google-apis-rs/google-compute1/trait.CallBuilder.html)**
    * operations to apply to *Resources*

All *structures* are marked with applicable traits to further categorize them and ease browsing.

Generally speaking, you can invoke *Activities* like this:

```Rust,ignore
let r = hub.resource().activity(...).doit()
```

Or specifically ...

```ignore
let r = hub.target_instances().delete(...).doit()
let r = hub.instances().stop(...).doit()
let r = hub.target_pools().add_instance(...).doit()
let r = hub.target_pools().add_health_check(...).doit()
let r = hub.target_pools().remove_health_check(...).doit()
let r = hub.backend_services().update(...).doit()
let r = hub.target_pools().delete(...).doit()
let r = hub.firewalls().update(...).doit()
let r = hub.networks().insert(...).doit()
let r = hub.addresses().insert(...).doit()
let r = hub.backend_services().patch(...).doit()
let r = hub.instances().set_tags(...).doit()
let r = hub.instances().set_disk_auto_delete(...).doit()
let r = hub.target_http_proxies().insert(...).doit()
let r = hub.projects().set_usage_export_bucket(...).doit()
let r = hub.url_maps().update(...).doit()
let r = hub.instance_templates().insert(...).doit()
let r = hub.firewalls().patch(...).doit()
let r = hub.vpn_tunnels().delete(...).doit()
let r = hub.firewalls().delete(...).doit()
let r = hub.instances().detach_disk(...).doit()
let r = hub.instances().delete_access_config(...).doit()
let r = hub.backend_services().delete(...).doit()
let r = hub.instances().insert(...).doit()
let r = hub.global_forwarding_rules().set_target(...).doit()
let r = hub.url_maps().insert(...).doit()
let r = hub.instances().add_access_config(...).doit()
let r = hub.target_vpn_gateways().delete(...).doit()
let r = hub.region_operations().get(...).doit()
let r = hub.backend_services().insert(...).doit()
let r = hub.global_operations().get(...).doit()
let r = hub.disks().insert(...).doit()
let r = hub.instance_templates().delete(...).doit()
let r = hub.global_forwarding_rules().delete(...).doit()
let r = hub.global_forwarding_rules().insert(...).doit()
let r = hub.disks().delete(...).doit()
let r = hub.global_addresses().delete(...).doit()
let r = hub.projects().set_common_instance_metadata(...).doit()
let r = hub.target_pools().insert(...).doit()
let r = hub.target_http_proxies().delete(...).doit()
let r = hub.networks().delete(...).doit()
let r = hub.target_vpn_gateways().insert(...).doit()
let r = hub.url_maps().delete(...).doit()
let r = hub.instances().set_scheduling(...).doit()
let r = hub.instances().attach_disk(...).doit()
let r = hub.addresses().delete(...).doit()
let r = hub.instances().reset(...).doit()
let r = hub.target_http_proxies().set_url_map(...).doit()
let r = hub.disks().create_snapshot(...).doit()
let r = hub.routes().insert(...).doit()
let r = hub.global_addresses().insert(...).doit()
let r = hub.images().deprecate(...).doit()
let r = hub.http_health_checks().patch(...).doit()
let r = hub.http_health_checks().delete(...).doit()
let r = hub.instances().set_metadata(...).doit()
let r = hub.http_health_checks().insert(...).doit()
let r = hub.images().delete(...).doit()
let r = hub.snapshots().delete(...).doit()
let r = hub.forwarding_rules().insert(...).doit()
let r = hub.images().insert(...).doit()
let r = hub.instances().start(...).doit()
let r = hub.target_pools().set_backup(...).doit()
let r = hub.target_instances().insert(...).doit()
let r = hub.forwarding_rules().delete(...).doit()
let r = hub.projects().move_instance(...).doit()
let r = hub.forwarding_rules().set_target(...).doit()
let r = hub.projects().move_disk(...).doit()
let r = hub.vpn_tunnels().insert(...).doit()
let r = hub.firewalls().insert(...).doit()
let r = hub.target_pools().remove_instance(...).doit()
let r = hub.instances().delete(...).doit()
let r = hub.zone_operations().get(...).doit()
let r = hub.http_health_checks().update(...).doit()
let r = hub.routes().delete(...).doit()
let r = hub.url_maps().patch(...).doit()
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
google-compute1 = "*"
```

## A complete example

```Rust
extern crate hyper;
extern crate yup_oauth2 as oauth2;
extern crate google_compute1 as compute1;
use compute1::{Result, Error};
use std::default::Default;
use oauth2::{Authenticator, DefaultAuthenticatorDelegate, ApplicationSecret, MemoryStorage};
use compute1::Compute;

// Get an ApplicationSecret instance by some means. It contains the `client_id` and 
// `client_secret`, among other things.
let secret: ApplicationSecret = Default::default();
// Instantiate the authenticator. It will choose a suitable authentication flow for you, 
// unless you replace  `None` with the desired Flow.
// Provide your own `AuthenticatorDelegate` to adjust the way it operates and get feedback about 
// what's going on. You probably want to bring in your own `TokenStorage` to persist tokens and
// retrieve them from storage.
let auth = Authenticator::new(&secret, DefaultAuthenticatorDelegate,
                              hyper::Client::new(),
                              <MemoryStorage as Default>::default(), None);
let mut hub = Compute::new(hyper::Client::new(), auth);
// You can configure optional parameters by calling the respective setters at will, and
// execute the final call using `doit()`.
// Values shown here are possibly random and not representative !
let result = hub.instances().set_disk_auto_delete("project", "zone", "instance", true, "deviceName")
             .doit();

match result {
    Err(e) => match e {
        Error::HttpError(err) => println!("HTTPERROR: {:?}", err),
        Error::MissingAPIKey => println!("Auth: Missing API Key - used if there are no scopes"),
        Error::MissingToken => println!("OAuth2: Missing Token"),
        Error::Cancelled => println!("Operation canceled by user"),
        Error::UploadSizeLimitExceeded(size, max_size) => println!("Upload size too big: {} of {}", size, max_size),
        Error::Failure(_) => println!("General Failure (hyper::client::Response doesn't print)"),
        Error::FieldClash(clashed_field) => println!("You added custom parameter which is part of builder: {:?}", clashed_field),
        Error::JsonDecodeError(err) => println!("Couldn't understand server reply - maybe API needs update: {:?}", err),
    },
    Ok(_) => println!("Success (value doesn't print)"),
}

```
## Handling Errors

All errors produced by the system are provided either as [Result](http://byron.github.io/google-apis-rs/google-compute1/enum.Result.html) enumeration as return value of 
the doit() methods, or handed as possibly intermediate results to either the 
[Hub Delegate](http://byron.github.io/google-apis-rs/google-compute1/trait.Delegate.html), or the [Authenticator Delegate](http://byron.github.io/google-apis-rs/google-compute1/../yup-oauth2/trait.AuthenticatorDelegate.html).

When delegates handle errors or intermediate values, they may have a chance to instruct the system to retry. This 
makes the system potentially resilient to all kinds of errors.

## Uploads and Downloads
If a method supports downloads, the response body, which is part of the [Result](http://byron.github.io/google-apis-rs/google-compute1/enum.Result.html), should be
read by you to obtain the media.
If such a method also supports a [Response Result](http://byron.github.io/google-apis-rs/google-compute1/trait.ResponseResult.html), it will return that by default.
You can see it as meta-data for the actual media. To trigger a media download, you will have to set up the builder by making
this call: `.param("alt", "media")`.

Methods supporting uploads can do so using up to 2 different protocols: 
*simple* and *resumable*. The distinctiveness of each is represented by customized 
`doit(...)` methods, which are then named `upload(...)` and `upload_resumable(...)` respectively.

## Customization and Callbacks

You may alter the way an `doit()` method is called by providing a [delegate](http://byron.github.io/google-apis-rs/google-compute1/trait.Delegate.html) to the 
[Method Builder](http://byron.github.io/google-apis-rs/google-compute1/trait.CallBuilder.html) before making the final `doit()` call. 
Respective methods will be called to provide progress information, as well as determine whether the system should 
retry on failure.

The [delegate trait](http://byron.github.io/google-apis-rs/google-compute1/trait.Delegate.html) is default-implemented, allowing you to customize it with minimal effort.

## Optional Parts in Server-Requests

All structures provided by this library are made to be [enocodable](http://byron.github.io/google-apis-rs/google-compute1/trait.RequestValue.html) and 
[decodable](http://byron.github.io/google-apis-rs/google-compute1/trait.ResponseResult.html) via *json*. Optionals are used to indicate that partial requests are responses 
are valid.
Most optionals are are considered [Parts](http://byron.github.io/google-apis-rs/google-compute1/trait.Part.html) which are identifiable by name, which will be sent to 
the server to indicate either the set parts of the request or the desired parts in the response.

## Builder Arguments

Using [method builders](http://byron.github.io/google-apis-rs/google-compute1/trait.CallBuilder.html), you are able to prepare an action call by repeatedly calling it's methods.
These will always take a single argument, for which the following statements are true.

* [PODs][wiki-pod] are handed by copy
* strings are passed as `&str`
* [request values](http://byron.github.io/google-apis-rs/google-compute1/trait.RequestValue.html) are borrowed

Arguments will always be copied or cloned into the builder, to make them independent of their original life times.

[wiki-pod]: http://en.wikipedia.org/wiki/Plain_old_data_structure
[builder-pattern]: http://en.wikipedia.org/wiki/Builder_pattern
[google-go-api]: https://github.com/google/google-api-go-client

# License
The **compute1** library was generated by Sebastian Thiel, and is placed 
under the *MIT* license.
You can read the full text at the repository's [license file][repo-license].

[repo-license]: https://github.com/Byron/google-apis-rs/LICENSE.md
