<!---
DO NOT EDIT !
This file was generated automatically from 'src/generator/templates/api/README.md.mako'
DO NOT EDIT !
-->
The `google-compute1` library allows access to all features of the *Google compute* service.

This documentation was generated from *compute* crate version *5.0.5+20240604*, where *20240604* is the exact revision of the *compute:v1* schema built by the [mako](http://www.makotemplates.org/) code generator *v5.0.5*.

Everything else about the *compute* *v1* API can be found at the
[official documentation site](https://cloud.google.com/compute/).
# Features

Handle the following *Resources* with ease from the central [hub](https://docs.rs/google-compute1/5.0.5+20240604/google_compute1/Compute) ... 

* [accelerator types](https://docs.rs/google-compute1/5.0.5+20240604/google_compute1/api::AcceleratorType)
 * [*aggregated list*](https://docs.rs/google-compute1/5.0.5+20240604/google_compute1/api::AcceleratorTypeAggregatedListCall), [*get*](https://docs.rs/google-compute1/5.0.5+20240604/google_compute1/api::AcceleratorTypeGetCall) and [*list*](https://docs.rs/google-compute1/5.0.5+20240604/google_compute1/api::AcceleratorTypeListCall)
* [addresses](https://docs.rs/google-compute1/5.0.5+20240604/google_compute1/api::Address)
 * [*aggregated list*](https://docs.rs/google-compute1/5.0.5+20240604/google_compute1/api::AddressAggregatedListCall), [*delete*](https://docs.rs/google-compute1/5.0.5+20240604/google_compute1/api::AddressDeleteCall), [*get*](https://docs.rs/google-compute1/5.0.5+20240604/google_compute1/api::AddressGetCall), [*insert*](https://docs.rs/google-compute1/5.0.5+20240604/google_compute1/api::AddressInsertCall), [*list*](https://docs.rs/google-compute1/5.0.5+20240604/google_compute1/api::AddressListCall), [*move*](https://docs.rs/google-compute1/5.0.5+20240604/google_compute1/api::AddressMoveCall) and [*set labels*](https://docs.rs/google-compute1/5.0.5+20240604/google_compute1/api::AddressSetLabelCall)
* [autoscalers](https://docs.rs/google-compute1/5.0.5+20240604/google_compute1/api::Autoscaler)
 * [*aggregated list*](https://docs.rs/google-compute1/5.0.5+20240604/google_compute1/api::AutoscalerAggregatedListCall), [*delete*](https://docs.rs/google-compute1/5.0.5+20240604/google_compute1/api::AutoscalerDeleteCall), [*get*](https://docs.rs/google-compute1/5.0.5+20240604/google_compute1/api::AutoscalerGetCall), [*insert*](https://docs.rs/google-compute1/5.0.5+20240604/google_compute1/api::AutoscalerInsertCall), [*list*](https://docs.rs/google-compute1/5.0.5+20240604/google_compute1/api::AutoscalerListCall), [*patch*](https://docs.rs/google-compute1/5.0.5+20240604/google_compute1/api::AutoscalerPatchCall) and [*update*](https://docs.rs/google-compute1/5.0.5+20240604/google_compute1/api::AutoscalerUpdateCall)
* [backend buckets](https://docs.rs/google-compute1/5.0.5+20240604/google_compute1/api::BackendBucket)
 * [*add signed url key*](https://docs.rs/google-compute1/5.0.5+20240604/google_compute1/api::BackendBucketAddSignedUrlKeyCall), [*delete*](https://docs.rs/google-compute1/5.0.5+20240604/google_compute1/api::BackendBucketDeleteCall), [*delete signed url key*](https://docs.rs/google-compute1/5.0.5+20240604/google_compute1/api::BackendBucketDeleteSignedUrlKeyCall), [*get*](https://docs.rs/google-compute1/5.0.5+20240604/google_compute1/api::BackendBucketGetCall), [*get iam policy*](https://docs.rs/google-compute1/5.0.5+20240604/google_compute1/api::BackendBucketGetIamPolicyCall), [*insert*](https://docs.rs/google-compute1/5.0.5+20240604/google_compute1/api::BackendBucketInsertCall), [*list*](https://docs.rs/google-compute1/5.0.5+20240604/google_compute1/api::BackendBucketListCall), [*patch*](https://docs.rs/google-compute1/5.0.5+20240604/google_compute1/api::BackendBucketPatchCall), [*set edge security policy*](https://docs.rs/google-compute1/5.0.5+20240604/google_compute1/api::BackendBucketSetEdgeSecurityPolicyCall), [*set iam policy*](https://docs.rs/google-compute1/5.0.5+20240604/google_compute1/api::BackendBucketSetIamPolicyCall), [*test iam permissions*](https://docs.rs/google-compute1/5.0.5+20240604/google_compute1/api::BackendBucketTestIamPermissionCall) and [*update*](https://docs.rs/google-compute1/5.0.5+20240604/google_compute1/api::BackendBucketUpdateCall)
* [backend services](https://docs.rs/google-compute1/5.0.5+20240604/google_compute1/api::BackendService)
 * [*add signed url key*](https://docs.rs/google-compute1/5.0.5+20240604/google_compute1/api::BackendServiceAddSignedUrlKeyCall), [*aggregated list*](https://docs.rs/google-compute1/5.0.5+20240604/google_compute1/api::BackendServiceAggregatedListCall), [*delete*](https://docs.rs/google-compute1/5.0.5+20240604/google_compute1/api::BackendServiceDeleteCall), [*delete signed url key*](https://docs.rs/google-compute1/5.0.5+20240604/google_compute1/api::BackendServiceDeleteSignedUrlKeyCall), [*get*](https://docs.rs/google-compute1/5.0.5+20240604/google_compute1/api::BackendServiceGetCall), [*get health*](https://docs.rs/google-compute1/5.0.5+20240604/google_compute1/api::BackendServiceGetHealthCall), [*get iam policy*](https://docs.rs/google-compute1/5.0.5+20240604/google_compute1/api::BackendServiceGetIamPolicyCall), [*insert*](https://docs.rs/google-compute1/5.0.5+20240604/google_compute1/api::BackendServiceInsertCall), [*list*](https://docs.rs/google-compute1/5.0.5+20240604/google_compute1/api::BackendServiceListCall), [*list usable*](https://docs.rs/google-compute1/5.0.5+20240604/google_compute1/api::BackendServiceListUsableCall), [*patch*](https://docs.rs/google-compute1/5.0.5+20240604/google_compute1/api::BackendServicePatchCall), [*set edge security policy*](https://docs.rs/google-compute1/5.0.5+20240604/google_compute1/api::BackendServiceSetEdgeSecurityPolicyCall), [*set iam policy*](https://docs.rs/google-compute1/5.0.5+20240604/google_compute1/api::BackendServiceSetIamPolicyCall), [*set security policy*](https://docs.rs/google-compute1/5.0.5+20240604/google_compute1/api::BackendServiceSetSecurityPolicyCall), [*test iam permissions*](https://docs.rs/google-compute1/5.0.5+20240604/google_compute1/api::BackendServiceTestIamPermissionCall) and [*update*](https://docs.rs/google-compute1/5.0.5+20240604/google_compute1/api::BackendServiceUpdateCall)
* [disk types](https://docs.rs/google-compute1/5.0.5+20240604/google_compute1/api::DiskType)
 * [*aggregated list*](https://docs.rs/google-compute1/5.0.5+20240604/google_compute1/api::DiskTypeAggregatedListCall), [*get*](https://docs.rs/google-compute1/5.0.5+20240604/google_compute1/api::DiskTypeGetCall) and [*list*](https://docs.rs/google-compute1/5.0.5+20240604/google_compute1/api::DiskTypeListCall)
* [disks](https://docs.rs/google-compute1/5.0.5+20240604/google_compute1/api::Disk)
 * [*add resource policies*](https://docs.rs/google-compute1/5.0.5+20240604/google_compute1/api::DiskAddResourcePolicyCall), [*aggregated list*](https://docs.rs/google-compute1/5.0.5+20240604/google_compute1/api::DiskAggregatedListCall), [*bulk insert*](https://docs.rs/google-compute1/5.0.5+20240604/google_compute1/api::DiskBulkInsertCall), [*create snapshot*](https://docs.rs/google-compute1/5.0.5+20240604/google_compute1/api::DiskCreateSnapshotCall), [*delete*](https://docs.rs/google-compute1/5.0.5+20240604/google_compute1/api::DiskDeleteCall), [*get*](https://docs.rs/google-compute1/5.0.5+20240604/google_compute1/api::DiskGetCall), [*get iam policy*](https://docs.rs/google-compute1/5.0.5+20240604/google_compute1/api::DiskGetIamPolicyCall), [*insert*](https://docs.rs/google-compute1/5.0.5+20240604/google_compute1/api::DiskInsertCall), [*list*](https://docs.rs/google-compute1/5.0.5+20240604/google_compute1/api::DiskListCall), [*remove resource policies*](https://docs.rs/google-compute1/5.0.5+20240604/google_compute1/api::DiskRemoveResourcePolicyCall), [*resize*](https://docs.rs/google-compute1/5.0.5+20240604/google_compute1/api::DiskResizeCall), [*set iam policy*](https://docs.rs/google-compute1/5.0.5+20240604/google_compute1/api::DiskSetIamPolicyCall), [*set labels*](https://docs.rs/google-compute1/5.0.5+20240604/google_compute1/api::DiskSetLabelCall), [*start async replication*](https://docs.rs/google-compute1/5.0.5+20240604/google_compute1/api::DiskStartAsyncReplicationCall), [*stop async replication*](https://docs.rs/google-compute1/5.0.5+20240604/google_compute1/api::DiskStopAsyncReplicationCall), [*stop group async replication*](https://docs.rs/google-compute1/5.0.5+20240604/google_compute1/api::DiskStopGroupAsyncReplicationCall), [*test iam permissions*](https://docs.rs/google-compute1/5.0.5+20240604/google_compute1/api::DiskTestIamPermissionCall) and [*update*](https://docs.rs/google-compute1/5.0.5+20240604/google_compute1/api::DiskUpdateCall)
* [external vpn gateways](https://docs.rs/google-compute1/5.0.5+20240604/google_compute1/api::ExternalVpnGateway)
 * [*delete*](https://docs.rs/google-compute1/5.0.5+20240604/google_compute1/api::ExternalVpnGatewayDeleteCall), [*get*](https://docs.rs/google-compute1/5.0.5+20240604/google_compute1/api::ExternalVpnGatewayGetCall), [*insert*](https://docs.rs/google-compute1/5.0.5+20240604/google_compute1/api::ExternalVpnGatewayInsertCall), [*list*](https://docs.rs/google-compute1/5.0.5+20240604/google_compute1/api::ExternalVpnGatewayListCall), [*set labels*](https://docs.rs/google-compute1/5.0.5+20240604/google_compute1/api::ExternalVpnGatewaySetLabelCall) and [*test iam permissions*](https://docs.rs/google-compute1/5.0.5+20240604/google_compute1/api::ExternalVpnGatewayTestIamPermissionCall)
* [firewall policies](https://docs.rs/google-compute1/5.0.5+20240604/google_compute1/api::FirewallPolicy)
 * [*add association*](https://docs.rs/google-compute1/5.0.5+20240604/google_compute1/api::FirewallPolicyAddAssociationCall), [*add rule*](https://docs.rs/google-compute1/5.0.5+20240604/google_compute1/api::FirewallPolicyAddRuleCall), [*clone rules*](https://docs.rs/google-compute1/5.0.5+20240604/google_compute1/api::FirewallPolicyCloneRuleCall), [*delete*](https://docs.rs/google-compute1/5.0.5+20240604/google_compute1/api::FirewallPolicyDeleteCall), [*get*](https://docs.rs/google-compute1/5.0.5+20240604/google_compute1/api::FirewallPolicyGetCall), [*get association*](https://docs.rs/google-compute1/5.0.5+20240604/google_compute1/api::FirewallPolicyGetAssociationCall), [*get iam policy*](https://docs.rs/google-compute1/5.0.5+20240604/google_compute1/api::FirewallPolicyGetIamPolicyCall), [*get rule*](https://docs.rs/google-compute1/5.0.5+20240604/google_compute1/api::FirewallPolicyGetRuleCall), [*insert*](https://docs.rs/google-compute1/5.0.5+20240604/google_compute1/api::FirewallPolicyInsertCall), [*list*](https://docs.rs/google-compute1/5.0.5+20240604/google_compute1/api::FirewallPolicyListCall), [*list associations*](https://docs.rs/google-compute1/5.0.5+20240604/google_compute1/api::FirewallPolicyListAssociationCall), [*move*](https://docs.rs/google-compute1/5.0.5+20240604/google_compute1/api::FirewallPolicyMoveCall), [*patch*](https://docs.rs/google-compute1/5.0.5+20240604/google_compute1/api::FirewallPolicyPatchCall), [*patch rule*](https://docs.rs/google-compute1/5.0.5+20240604/google_compute1/api::FirewallPolicyPatchRuleCall), [*remove association*](https://docs.rs/google-compute1/5.0.5+20240604/google_compute1/api::FirewallPolicyRemoveAssociationCall), [*remove rule*](https://docs.rs/google-compute1/5.0.5+20240604/google_compute1/api::FirewallPolicyRemoveRuleCall), [*set iam policy*](https://docs.rs/google-compute1/5.0.5+20240604/google_compute1/api::FirewallPolicySetIamPolicyCall) and [*test iam permissions*](https://docs.rs/google-compute1/5.0.5+20240604/google_compute1/api::FirewallPolicyTestIamPermissionCall)
* [firewalls](https://docs.rs/google-compute1/5.0.5+20240604/google_compute1/api::Firewall)
 * [*delete*](https://docs.rs/google-compute1/5.0.5+20240604/google_compute1/api::FirewallDeleteCall), [*get*](https://docs.rs/google-compute1/5.0.5+20240604/google_compute1/api::FirewallGetCall), [*insert*](https://docs.rs/google-compute1/5.0.5+20240604/google_compute1/api::FirewallInsertCall), [*list*](https://docs.rs/google-compute1/5.0.5+20240604/google_compute1/api::FirewallListCall), [*patch*](https://docs.rs/google-compute1/5.0.5+20240604/google_compute1/api::FirewallPatchCall) and [*update*](https://docs.rs/google-compute1/5.0.5+20240604/google_compute1/api::FirewallUpdateCall)
* [forwarding rules](https://docs.rs/google-compute1/5.0.5+20240604/google_compute1/api::ForwardingRule)
 * [*aggregated list*](https://docs.rs/google-compute1/5.0.5+20240604/google_compute1/api::ForwardingRuleAggregatedListCall), [*delete*](https://docs.rs/google-compute1/5.0.5+20240604/google_compute1/api::ForwardingRuleDeleteCall), [*get*](https://docs.rs/google-compute1/5.0.5+20240604/google_compute1/api::ForwardingRuleGetCall), [*insert*](https://docs.rs/google-compute1/5.0.5+20240604/google_compute1/api::ForwardingRuleInsertCall), [*list*](https://docs.rs/google-compute1/5.0.5+20240604/google_compute1/api::ForwardingRuleListCall), [*patch*](https://docs.rs/google-compute1/5.0.5+20240604/google_compute1/api::ForwardingRulePatchCall), [*set labels*](https://docs.rs/google-compute1/5.0.5+20240604/google_compute1/api::ForwardingRuleSetLabelCall) and [*set target*](https://docs.rs/google-compute1/5.0.5+20240604/google_compute1/api::ForwardingRuleSetTargetCall)
* global addresses
 * [*delete*](https://docs.rs/google-compute1/5.0.5+20240604/google_compute1/api::GlobalAddressDeleteCall), [*get*](https://docs.rs/google-compute1/5.0.5+20240604/google_compute1/api::GlobalAddressGetCall), [*insert*](https://docs.rs/google-compute1/5.0.5+20240604/google_compute1/api::GlobalAddressInsertCall), [*list*](https://docs.rs/google-compute1/5.0.5+20240604/google_compute1/api::GlobalAddressListCall), [*move*](https://docs.rs/google-compute1/5.0.5+20240604/google_compute1/api::GlobalAddressMoveCall) and [*set labels*](https://docs.rs/google-compute1/5.0.5+20240604/google_compute1/api::GlobalAddressSetLabelCall)
* global forwarding rules
 * [*delete*](https://docs.rs/google-compute1/5.0.5+20240604/google_compute1/api::GlobalForwardingRuleDeleteCall), [*get*](https://docs.rs/google-compute1/5.0.5+20240604/google_compute1/api::GlobalForwardingRuleGetCall), [*insert*](https://docs.rs/google-compute1/5.0.5+20240604/google_compute1/api::GlobalForwardingRuleInsertCall), [*list*](https://docs.rs/google-compute1/5.0.5+20240604/google_compute1/api::GlobalForwardingRuleListCall), [*patch*](https://docs.rs/google-compute1/5.0.5+20240604/google_compute1/api::GlobalForwardingRulePatchCall), [*set labels*](https://docs.rs/google-compute1/5.0.5+20240604/google_compute1/api::GlobalForwardingRuleSetLabelCall) and [*set target*](https://docs.rs/google-compute1/5.0.5+20240604/google_compute1/api::GlobalForwardingRuleSetTargetCall)
* global network endpoint groups
 * [*attach network endpoints*](https://docs.rs/google-compute1/5.0.5+20240604/google_compute1/api::GlobalNetworkEndpointGroupAttachNetworkEndpointCall), [*delete*](https://docs.rs/google-compute1/5.0.5+20240604/google_compute1/api::GlobalNetworkEndpointGroupDeleteCall), [*detach network endpoints*](https://docs.rs/google-compute1/5.0.5+20240604/google_compute1/api::GlobalNetworkEndpointGroupDetachNetworkEndpointCall), [*get*](https://docs.rs/google-compute1/5.0.5+20240604/google_compute1/api::GlobalNetworkEndpointGroupGetCall), [*insert*](https://docs.rs/google-compute1/5.0.5+20240604/google_compute1/api::GlobalNetworkEndpointGroupInsertCall), [*list*](https://docs.rs/google-compute1/5.0.5+20240604/google_compute1/api::GlobalNetworkEndpointGroupListCall) and [*list network endpoints*](https://docs.rs/google-compute1/5.0.5+20240604/google_compute1/api::GlobalNetworkEndpointGroupListNetworkEndpointCall)
* global operations
 * [*aggregated list*](https://docs.rs/google-compute1/5.0.5+20240604/google_compute1/api::GlobalOperationAggregatedListCall), [*delete*](https://docs.rs/google-compute1/5.0.5+20240604/google_compute1/api::GlobalOperationDeleteCall), [*get*](https://docs.rs/google-compute1/5.0.5+20240604/google_compute1/api::GlobalOperationGetCall), [*list*](https://docs.rs/google-compute1/5.0.5+20240604/google_compute1/api::GlobalOperationListCall) and [*wait*](https://docs.rs/google-compute1/5.0.5+20240604/google_compute1/api::GlobalOperationWaitCall)
* global organization operations
 * [*delete*](https://docs.rs/google-compute1/5.0.5+20240604/google_compute1/api::GlobalOrganizationOperationDeleteCall), [*get*](https://docs.rs/google-compute1/5.0.5+20240604/google_compute1/api::GlobalOrganizationOperationGetCall) and [*list*](https://docs.rs/google-compute1/5.0.5+20240604/google_compute1/api::GlobalOrganizationOperationListCall)
* global public delegated prefixes
 * [*delete*](https://docs.rs/google-compute1/5.0.5+20240604/google_compute1/api::GlobalPublicDelegatedPrefixDeleteCall), [*get*](https://docs.rs/google-compute1/5.0.5+20240604/google_compute1/api::GlobalPublicDelegatedPrefixGetCall), [*insert*](https://docs.rs/google-compute1/5.0.5+20240604/google_compute1/api::GlobalPublicDelegatedPrefixInsertCall), [*list*](https://docs.rs/google-compute1/5.0.5+20240604/google_compute1/api::GlobalPublicDelegatedPrefixListCall) and [*patch*](https://docs.rs/google-compute1/5.0.5+20240604/google_compute1/api::GlobalPublicDelegatedPrefixPatchCall)
* [health checks](https://docs.rs/google-compute1/5.0.5+20240604/google_compute1/api::HealthCheck)
 * [*aggregated list*](https://docs.rs/google-compute1/5.0.5+20240604/google_compute1/api::HealthCheckAggregatedListCall), [*delete*](https://docs.rs/google-compute1/5.0.5+20240604/google_compute1/api::HealthCheckDeleteCall), [*get*](https://docs.rs/google-compute1/5.0.5+20240604/google_compute1/api::HealthCheckGetCall), [*insert*](https://docs.rs/google-compute1/5.0.5+20240604/google_compute1/api::HealthCheckInsertCall), [*list*](https://docs.rs/google-compute1/5.0.5+20240604/google_compute1/api::HealthCheckListCall), [*patch*](https://docs.rs/google-compute1/5.0.5+20240604/google_compute1/api::HealthCheckPatchCall) and [*update*](https://docs.rs/google-compute1/5.0.5+20240604/google_compute1/api::HealthCheckUpdateCall)
* [http health checks](https://docs.rs/google-compute1/5.0.5+20240604/google_compute1/api::HttpHealthCheck)
 * [*delete*](https://docs.rs/google-compute1/5.0.5+20240604/google_compute1/api::HttpHealthCheckDeleteCall), [*get*](https://docs.rs/google-compute1/5.0.5+20240604/google_compute1/api::HttpHealthCheckGetCall), [*insert*](https://docs.rs/google-compute1/5.0.5+20240604/google_compute1/api::HttpHealthCheckInsertCall), [*list*](https://docs.rs/google-compute1/5.0.5+20240604/google_compute1/api::HttpHealthCheckListCall), [*patch*](https://docs.rs/google-compute1/5.0.5+20240604/google_compute1/api::HttpHealthCheckPatchCall) and [*update*](https://docs.rs/google-compute1/5.0.5+20240604/google_compute1/api::HttpHealthCheckUpdateCall)
* [https health checks](https://docs.rs/google-compute1/5.0.5+20240604/google_compute1/api::HttpsHealthCheck)
 * [*delete*](https://docs.rs/google-compute1/5.0.5+20240604/google_compute1/api::HttpsHealthCheckDeleteCall), [*get*](https://docs.rs/google-compute1/5.0.5+20240604/google_compute1/api::HttpsHealthCheckGetCall), [*insert*](https://docs.rs/google-compute1/5.0.5+20240604/google_compute1/api::HttpsHealthCheckInsertCall), [*list*](https://docs.rs/google-compute1/5.0.5+20240604/google_compute1/api::HttpsHealthCheckListCall), [*patch*](https://docs.rs/google-compute1/5.0.5+20240604/google_compute1/api::HttpsHealthCheckPatchCall) and [*update*](https://docs.rs/google-compute1/5.0.5+20240604/google_compute1/api::HttpsHealthCheckUpdateCall)
* [image family views](https://docs.rs/google-compute1/5.0.5+20240604/google_compute1/api::ImageFamilyView)
 * [*get*](https://docs.rs/google-compute1/5.0.5+20240604/google_compute1/api::ImageFamilyViewGetCall)
* [images](https://docs.rs/google-compute1/5.0.5+20240604/google_compute1/api::Image)
 * [*delete*](https://docs.rs/google-compute1/5.0.5+20240604/google_compute1/api::ImageDeleteCall), [*deprecate*](https://docs.rs/google-compute1/5.0.5+20240604/google_compute1/api::ImageDeprecateCall), [*get*](https://docs.rs/google-compute1/5.0.5+20240604/google_compute1/api::ImageGetCall), [*get from family*](https://docs.rs/google-compute1/5.0.5+20240604/google_compute1/api::ImageGetFromFamilyCall), [*get iam policy*](https://docs.rs/google-compute1/5.0.5+20240604/google_compute1/api::ImageGetIamPolicyCall), [*insert*](https://docs.rs/google-compute1/5.0.5+20240604/google_compute1/api::ImageInsertCall), [*list*](https://docs.rs/google-compute1/5.0.5+20240604/google_compute1/api::ImageListCall), [*patch*](https://docs.rs/google-compute1/5.0.5+20240604/google_compute1/api::ImagePatchCall), [*set iam policy*](https://docs.rs/google-compute1/5.0.5+20240604/google_compute1/api::ImageSetIamPolicyCall), [*set labels*](https://docs.rs/google-compute1/5.0.5+20240604/google_compute1/api::ImageSetLabelCall) and [*test iam permissions*](https://docs.rs/google-compute1/5.0.5+20240604/google_compute1/api::ImageTestIamPermissionCall)
* [instance group manager resize requests](https://docs.rs/google-compute1/5.0.5+20240604/google_compute1/api::InstanceGroupManagerResizeRequest)
 * [*cancel*](https://docs.rs/google-compute1/5.0.5+20240604/google_compute1/api::InstanceGroupManagerResizeRequestCancelCall), [*delete*](https://docs.rs/google-compute1/5.0.5+20240604/google_compute1/api::InstanceGroupManagerResizeRequestDeleteCall), [*get*](https://docs.rs/google-compute1/5.0.5+20240604/google_compute1/api::InstanceGroupManagerResizeRequestGetCall), [*insert*](https://docs.rs/google-compute1/5.0.5+20240604/google_compute1/api::InstanceGroupManagerResizeRequestInsertCall) and [*list*](https://docs.rs/google-compute1/5.0.5+20240604/google_compute1/api::InstanceGroupManagerResizeRequestListCall)
* [instance group managers](https://docs.rs/google-compute1/5.0.5+20240604/google_compute1/api::InstanceGroupManager)
 * [*abandon instances*](https://docs.rs/google-compute1/5.0.5+20240604/google_compute1/api::InstanceGroupManagerAbandonInstanceCall), [*aggregated list*](https://docs.rs/google-compute1/5.0.5+20240604/google_compute1/api::InstanceGroupManagerAggregatedListCall), [*apply updates to instances*](https://docs.rs/google-compute1/5.0.5+20240604/google_compute1/api::InstanceGroupManagerApplyUpdatesToInstanceCall), [*create instances*](https://docs.rs/google-compute1/5.0.5+20240604/google_compute1/api::InstanceGroupManagerCreateInstanceCall), [*delete*](https://docs.rs/google-compute1/5.0.5+20240604/google_compute1/api::InstanceGroupManagerDeleteCall), [*delete instances*](https://docs.rs/google-compute1/5.0.5+20240604/google_compute1/api::InstanceGroupManagerDeleteInstanceCall), [*delete per instance configs*](https://docs.rs/google-compute1/5.0.5+20240604/google_compute1/api::InstanceGroupManagerDeletePerInstanceConfigCall), [*get*](https://docs.rs/google-compute1/5.0.5+20240604/google_compute1/api::InstanceGroupManagerGetCall), [*insert*](https://docs.rs/google-compute1/5.0.5+20240604/google_compute1/api::InstanceGroupManagerInsertCall), [*list*](https://docs.rs/google-compute1/5.0.5+20240604/google_compute1/api::InstanceGroupManagerListCall), [*list errors*](https://docs.rs/google-compute1/5.0.5+20240604/google_compute1/api::InstanceGroupManagerListErrorCall), [*list managed instances*](https://docs.rs/google-compute1/5.0.5+20240604/google_compute1/api::InstanceGroupManagerListManagedInstanceCall), [*list per instance configs*](https://docs.rs/google-compute1/5.0.5+20240604/google_compute1/api::InstanceGroupManagerListPerInstanceConfigCall), [*patch*](https://docs.rs/google-compute1/5.0.5+20240604/google_compute1/api::InstanceGroupManagerPatchCall), [*patch per instance configs*](https://docs.rs/google-compute1/5.0.5+20240604/google_compute1/api::InstanceGroupManagerPatchPerInstanceConfigCall), [*recreate instances*](https://docs.rs/google-compute1/5.0.5+20240604/google_compute1/api::InstanceGroupManagerRecreateInstanceCall), [*resize*](https://docs.rs/google-compute1/5.0.5+20240604/google_compute1/api::InstanceGroupManagerResizeCall), [*set instance template*](https://docs.rs/google-compute1/5.0.5+20240604/google_compute1/api::InstanceGroupManagerSetInstanceTemplateCall), [*set target pools*](https://docs.rs/google-compute1/5.0.5+20240604/google_compute1/api::InstanceGroupManagerSetTargetPoolCall) and [*update per instance configs*](https://docs.rs/google-compute1/5.0.5+20240604/google_compute1/api::InstanceGroupManagerUpdatePerInstanceConfigCall)
* [instance groups](https://docs.rs/google-compute1/5.0.5+20240604/google_compute1/api::InstanceGroup)
 * [*add instances*](https://docs.rs/google-compute1/5.0.5+20240604/google_compute1/api::InstanceGroupAddInstanceCall), [*aggregated list*](https://docs.rs/google-compute1/5.0.5+20240604/google_compute1/api::InstanceGroupAggregatedListCall), [*delete*](https://docs.rs/google-compute1/5.0.5+20240604/google_compute1/api::InstanceGroupDeleteCall), [*get*](https://docs.rs/google-compute1/5.0.5+20240604/google_compute1/api::InstanceGroupGetCall), [*insert*](https://docs.rs/google-compute1/5.0.5+20240604/google_compute1/api::InstanceGroupInsertCall), [*list*](https://docs.rs/google-compute1/5.0.5+20240604/google_compute1/api::InstanceGroupListCall), [*list instances*](https://docs.rs/google-compute1/5.0.5+20240604/google_compute1/api::InstanceGroupListInstanceCall), [*remove instances*](https://docs.rs/google-compute1/5.0.5+20240604/google_compute1/api::InstanceGroupRemoveInstanceCall) and [*set named ports*](https://docs.rs/google-compute1/5.0.5+20240604/google_compute1/api::InstanceGroupSetNamedPortCall)
* instance settings
 * [*get*](https://docs.rs/google-compute1/5.0.5+20240604/google_compute1/api::InstanceSettingGetCall) and [*patch*](https://docs.rs/google-compute1/5.0.5+20240604/google_compute1/api::InstanceSettingPatchCall)
* [instance templates](https://docs.rs/google-compute1/5.0.5+20240604/google_compute1/api::InstanceTemplate)
 * [*aggregated list*](https://docs.rs/google-compute1/5.0.5+20240604/google_compute1/api::InstanceTemplateAggregatedListCall), [*delete*](https://docs.rs/google-compute1/5.0.5+20240604/google_compute1/api::InstanceTemplateDeleteCall), [*get*](https://docs.rs/google-compute1/5.0.5+20240604/google_compute1/api::InstanceTemplateGetCall), [*get iam policy*](https://docs.rs/google-compute1/5.0.5+20240604/google_compute1/api::InstanceTemplateGetIamPolicyCall), [*insert*](https://docs.rs/google-compute1/5.0.5+20240604/google_compute1/api::InstanceTemplateInsertCall), [*list*](https://docs.rs/google-compute1/5.0.5+20240604/google_compute1/api::InstanceTemplateListCall), [*set iam policy*](https://docs.rs/google-compute1/5.0.5+20240604/google_compute1/api::InstanceTemplateSetIamPolicyCall) and [*test iam permissions*](https://docs.rs/google-compute1/5.0.5+20240604/google_compute1/api::InstanceTemplateTestIamPermissionCall)
* [instances](https://docs.rs/google-compute1/5.0.5+20240604/google_compute1/api::Instance)
 * [*add access config*](https://docs.rs/google-compute1/5.0.5+20240604/google_compute1/api::InstanceAddAccessConfigCall), [*add resource policies*](https://docs.rs/google-compute1/5.0.5+20240604/google_compute1/api::InstanceAddResourcePolicyCall), [*aggregated list*](https://docs.rs/google-compute1/5.0.5+20240604/google_compute1/api::InstanceAggregatedListCall), [*attach disk*](https://docs.rs/google-compute1/5.0.5+20240604/google_compute1/api::InstanceAttachDiskCall), [*bulk insert*](https://docs.rs/google-compute1/5.0.5+20240604/google_compute1/api::InstanceBulkInsertCall), [*delete*](https://docs.rs/google-compute1/5.0.5+20240604/google_compute1/api::InstanceDeleteCall), [*delete access config*](https://docs.rs/google-compute1/5.0.5+20240604/google_compute1/api::InstanceDeleteAccessConfigCall), [*detach disk*](https://docs.rs/google-compute1/5.0.5+20240604/google_compute1/api::InstanceDetachDiskCall), [*get*](https://docs.rs/google-compute1/5.0.5+20240604/google_compute1/api::InstanceGetCall), [*get effective firewalls*](https://docs.rs/google-compute1/5.0.5+20240604/google_compute1/api::InstanceGetEffectiveFirewallCall), [*get guest attributes*](https://docs.rs/google-compute1/5.0.5+20240604/google_compute1/api::InstanceGetGuestAttributeCall), [*get iam policy*](https://docs.rs/google-compute1/5.0.5+20240604/google_compute1/api::InstanceGetIamPolicyCall), [*get screenshot*](https://docs.rs/google-compute1/5.0.5+20240604/google_compute1/api::InstanceGetScreenshotCall), [*get serial port output*](https://docs.rs/google-compute1/5.0.5+20240604/google_compute1/api::InstanceGetSerialPortOutputCall), [*get shielded instance identity*](https://docs.rs/google-compute1/5.0.5+20240604/google_compute1/api::InstanceGetShieldedInstanceIdentityCall), [*insert*](https://docs.rs/google-compute1/5.0.5+20240604/google_compute1/api::InstanceInsertCall), [*list*](https://docs.rs/google-compute1/5.0.5+20240604/google_compute1/api::InstanceListCall), [*list referrers*](https://docs.rs/google-compute1/5.0.5+20240604/google_compute1/api::InstanceListReferrerCall), [*perform maintenance*](https://docs.rs/google-compute1/5.0.5+20240604/google_compute1/api::InstancePerformMaintenanceCall), [*remove resource policies*](https://docs.rs/google-compute1/5.0.5+20240604/google_compute1/api::InstanceRemoveResourcePolicyCall), [*reset*](https://docs.rs/google-compute1/5.0.5+20240604/google_compute1/api::InstanceResetCall), [*resume*](https://docs.rs/google-compute1/5.0.5+20240604/google_compute1/api::InstanceResumeCall), [*send diagnostic interrupt*](https://docs.rs/google-compute1/5.0.5+20240604/google_compute1/api::InstanceSendDiagnosticInterruptCall), [*set deletion protection*](https://docs.rs/google-compute1/5.0.5+20240604/google_compute1/api::InstanceSetDeletionProtectionCall), [*set disk auto delete*](https://docs.rs/google-compute1/5.0.5+20240604/google_compute1/api::InstanceSetDiskAutoDeleteCall), [*set iam policy*](https://docs.rs/google-compute1/5.0.5+20240604/google_compute1/api::InstanceSetIamPolicyCall), [*set labels*](https://docs.rs/google-compute1/5.0.5+20240604/google_compute1/api::InstanceSetLabelCall), [*set machine resources*](https://docs.rs/google-compute1/5.0.5+20240604/google_compute1/api::InstanceSetMachineResourceCall), [*set machine type*](https://docs.rs/google-compute1/5.0.5+20240604/google_compute1/api::InstanceSetMachineTypeCall), [*set metadata*](https://docs.rs/google-compute1/5.0.5+20240604/google_compute1/api::InstanceSetMetadataCall), [*set min cpu platform*](https://docs.rs/google-compute1/5.0.5+20240604/google_compute1/api::InstanceSetMinCpuPlatformCall), [*set name*](https://docs.rs/google-compute1/5.0.5+20240604/google_compute1/api::InstanceSetNameCall), [*set scheduling*](https://docs.rs/google-compute1/5.0.5+20240604/google_compute1/api::InstanceSetSchedulingCall), [*set security policy*](https://docs.rs/google-compute1/5.0.5+20240604/google_compute1/api::InstanceSetSecurityPolicyCall), [*set service account*](https://docs.rs/google-compute1/5.0.5+20240604/google_compute1/api::InstanceSetServiceAccountCall), [*set shielded instance integrity policy*](https://docs.rs/google-compute1/5.0.5+20240604/google_compute1/api::InstanceSetShieldedInstanceIntegrityPolicyCall), [*set tags*](https://docs.rs/google-compute1/5.0.5+20240604/google_compute1/api::InstanceSetTagCall), [*simulate maintenance event*](https://docs.rs/google-compute1/5.0.5+20240604/google_compute1/api::InstanceSimulateMaintenanceEventCall), [*start*](https://docs.rs/google-compute1/5.0.5+20240604/google_compute1/api::InstanceStartCall), [*start with encryption key*](https://docs.rs/google-compute1/5.0.5+20240604/google_compute1/api::InstanceStartWithEncryptionKeyCall), [*stop*](https://docs.rs/google-compute1/5.0.5+20240604/google_compute1/api::InstanceStopCall), [*suspend*](https://docs.rs/google-compute1/5.0.5+20240604/google_compute1/api::InstanceSuspendCall), [*test iam permissions*](https://docs.rs/google-compute1/5.0.5+20240604/google_compute1/api::InstanceTestIamPermissionCall), [*update*](https://docs.rs/google-compute1/5.0.5+20240604/google_compute1/api::InstanceUpdateCall), [*update access config*](https://docs.rs/google-compute1/5.0.5+20240604/google_compute1/api::InstanceUpdateAccessConfigCall), [*update display device*](https://docs.rs/google-compute1/5.0.5+20240604/google_compute1/api::InstanceUpdateDisplayDeviceCall), [*update network interface*](https://docs.rs/google-compute1/5.0.5+20240604/google_compute1/api::InstanceUpdateNetworkInterfaceCall) and [*update shielded instance config*](https://docs.rs/google-compute1/5.0.5+20240604/google_compute1/api::InstanceUpdateShieldedInstanceConfigCall)
* [instant snapshots](https://docs.rs/google-compute1/5.0.5+20240604/google_compute1/api::InstantSnapshot)
 * [*aggregated list*](https://docs.rs/google-compute1/5.0.5+20240604/google_compute1/api::InstantSnapshotAggregatedListCall), [*delete*](https://docs.rs/google-compute1/5.0.5+20240604/google_compute1/api::InstantSnapshotDeleteCall), [*get*](https://docs.rs/google-compute1/5.0.5+20240604/google_compute1/api::InstantSnapshotGetCall), [*get iam policy*](https://docs.rs/google-compute1/5.0.5+20240604/google_compute1/api::InstantSnapshotGetIamPolicyCall), [*insert*](https://docs.rs/google-compute1/5.0.5+20240604/google_compute1/api::InstantSnapshotInsertCall), [*list*](https://docs.rs/google-compute1/5.0.5+20240604/google_compute1/api::InstantSnapshotListCall), [*set iam policy*](https://docs.rs/google-compute1/5.0.5+20240604/google_compute1/api::InstantSnapshotSetIamPolicyCall), [*set labels*](https://docs.rs/google-compute1/5.0.5+20240604/google_compute1/api::InstantSnapshotSetLabelCall) and [*test iam permissions*](https://docs.rs/google-compute1/5.0.5+20240604/google_compute1/api::InstantSnapshotTestIamPermissionCall)
* [interconnect attachments](https://docs.rs/google-compute1/5.0.5+20240604/google_compute1/api::InterconnectAttachment)
 * [*aggregated list*](https://docs.rs/google-compute1/5.0.5+20240604/google_compute1/api::InterconnectAttachmentAggregatedListCall), [*delete*](https://docs.rs/google-compute1/5.0.5+20240604/google_compute1/api::InterconnectAttachmentDeleteCall), [*get*](https://docs.rs/google-compute1/5.0.5+20240604/google_compute1/api::InterconnectAttachmentGetCall), [*insert*](https://docs.rs/google-compute1/5.0.5+20240604/google_compute1/api::InterconnectAttachmentInsertCall), [*list*](https://docs.rs/google-compute1/5.0.5+20240604/google_compute1/api::InterconnectAttachmentListCall), [*patch*](https://docs.rs/google-compute1/5.0.5+20240604/google_compute1/api::InterconnectAttachmentPatchCall) and [*set labels*](https://docs.rs/google-compute1/5.0.5+20240604/google_compute1/api::InterconnectAttachmentSetLabelCall)
* [interconnect locations](https://docs.rs/google-compute1/5.0.5+20240604/google_compute1/api::InterconnectLocation)
 * [*get*](https://docs.rs/google-compute1/5.0.5+20240604/google_compute1/api::InterconnectLocationGetCall) and [*list*](https://docs.rs/google-compute1/5.0.5+20240604/google_compute1/api::InterconnectLocationListCall)
* [interconnect remote locations](https://docs.rs/google-compute1/5.0.5+20240604/google_compute1/api::InterconnectRemoteLocation)
 * [*get*](https://docs.rs/google-compute1/5.0.5+20240604/google_compute1/api::InterconnectRemoteLocationGetCall) and [*list*](https://docs.rs/google-compute1/5.0.5+20240604/google_compute1/api::InterconnectRemoteLocationListCall)
* [interconnects](https://docs.rs/google-compute1/5.0.5+20240604/google_compute1/api::Interconnect)
 * [*delete*](https://docs.rs/google-compute1/5.0.5+20240604/google_compute1/api::InterconnectDeleteCall), [*get*](https://docs.rs/google-compute1/5.0.5+20240604/google_compute1/api::InterconnectGetCall), [*get diagnostics*](https://docs.rs/google-compute1/5.0.5+20240604/google_compute1/api::InterconnectGetDiagnosticCall), [*get macsec config*](https://docs.rs/google-compute1/5.0.5+20240604/google_compute1/api::InterconnectGetMacsecConfigCall), [*insert*](https://docs.rs/google-compute1/5.0.5+20240604/google_compute1/api::InterconnectInsertCall), [*list*](https://docs.rs/google-compute1/5.0.5+20240604/google_compute1/api::InterconnectListCall), [*patch*](https://docs.rs/google-compute1/5.0.5+20240604/google_compute1/api::InterconnectPatchCall) and [*set labels*](https://docs.rs/google-compute1/5.0.5+20240604/google_compute1/api::InterconnectSetLabelCall)
* [license codes](https://docs.rs/google-compute1/5.0.5+20240604/google_compute1/api::LicenseCode)
 * [*get*](https://docs.rs/google-compute1/5.0.5+20240604/google_compute1/api::LicenseCodeGetCall) and [*test iam permissions*](https://docs.rs/google-compute1/5.0.5+20240604/google_compute1/api::LicenseCodeTestIamPermissionCall)
* [licenses](https://docs.rs/google-compute1/5.0.5+20240604/google_compute1/api::License)
 * [*delete*](https://docs.rs/google-compute1/5.0.5+20240604/google_compute1/api::LicenseDeleteCall), [*get*](https://docs.rs/google-compute1/5.0.5+20240604/google_compute1/api::LicenseGetCall), [*get iam policy*](https://docs.rs/google-compute1/5.0.5+20240604/google_compute1/api::LicenseGetIamPolicyCall), [*insert*](https://docs.rs/google-compute1/5.0.5+20240604/google_compute1/api::LicenseInsertCall), [*list*](https://docs.rs/google-compute1/5.0.5+20240604/google_compute1/api::LicenseListCall), [*set iam policy*](https://docs.rs/google-compute1/5.0.5+20240604/google_compute1/api::LicenseSetIamPolicyCall) and [*test iam permissions*](https://docs.rs/google-compute1/5.0.5+20240604/google_compute1/api::LicenseTestIamPermissionCall)
* [machine images](https://docs.rs/google-compute1/5.0.5+20240604/google_compute1/api::MachineImage)
 * [*delete*](https://docs.rs/google-compute1/5.0.5+20240604/google_compute1/api::MachineImageDeleteCall), [*get*](https://docs.rs/google-compute1/5.0.5+20240604/google_compute1/api::MachineImageGetCall), [*get iam policy*](https://docs.rs/google-compute1/5.0.5+20240604/google_compute1/api::MachineImageGetIamPolicyCall), [*insert*](https://docs.rs/google-compute1/5.0.5+20240604/google_compute1/api::MachineImageInsertCall), [*list*](https://docs.rs/google-compute1/5.0.5+20240604/google_compute1/api::MachineImageListCall), [*set iam policy*](https://docs.rs/google-compute1/5.0.5+20240604/google_compute1/api::MachineImageSetIamPolicyCall) and [*test iam permissions*](https://docs.rs/google-compute1/5.0.5+20240604/google_compute1/api::MachineImageTestIamPermissionCall)
* [machine types](https://docs.rs/google-compute1/5.0.5+20240604/google_compute1/api::MachineType)
 * [*aggregated list*](https://docs.rs/google-compute1/5.0.5+20240604/google_compute1/api::MachineTypeAggregatedListCall), [*get*](https://docs.rs/google-compute1/5.0.5+20240604/google_compute1/api::MachineTypeGetCall) and [*list*](https://docs.rs/google-compute1/5.0.5+20240604/google_compute1/api::MachineTypeListCall)
* [network attachments](https://docs.rs/google-compute1/5.0.5+20240604/google_compute1/api::NetworkAttachment)
 * [*aggregated list*](https://docs.rs/google-compute1/5.0.5+20240604/google_compute1/api::NetworkAttachmentAggregatedListCall), [*delete*](https://docs.rs/google-compute1/5.0.5+20240604/google_compute1/api::NetworkAttachmentDeleteCall), [*get*](https://docs.rs/google-compute1/5.0.5+20240604/google_compute1/api::NetworkAttachmentGetCall), [*get iam policy*](https://docs.rs/google-compute1/5.0.5+20240604/google_compute1/api::NetworkAttachmentGetIamPolicyCall), [*insert*](https://docs.rs/google-compute1/5.0.5+20240604/google_compute1/api::NetworkAttachmentInsertCall), [*list*](https://docs.rs/google-compute1/5.0.5+20240604/google_compute1/api::NetworkAttachmentListCall), [*patch*](https://docs.rs/google-compute1/5.0.5+20240604/google_compute1/api::NetworkAttachmentPatchCall), [*set iam policy*](https://docs.rs/google-compute1/5.0.5+20240604/google_compute1/api::NetworkAttachmentSetIamPolicyCall) and [*test iam permissions*](https://docs.rs/google-compute1/5.0.5+20240604/google_compute1/api::NetworkAttachmentTestIamPermissionCall)
* [network edge security services](https://docs.rs/google-compute1/5.0.5+20240604/google_compute1/api::NetworkEdgeSecurityService)
 * [*aggregated list*](https://docs.rs/google-compute1/5.0.5+20240604/google_compute1/api::NetworkEdgeSecurityServiceAggregatedListCall), [*delete*](https://docs.rs/google-compute1/5.0.5+20240604/google_compute1/api::NetworkEdgeSecurityServiceDeleteCall), [*get*](https://docs.rs/google-compute1/5.0.5+20240604/google_compute1/api::NetworkEdgeSecurityServiceGetCall), [*insert*](https://docs.rs/google-compute1/5.0.5+20240604/google_compute1/api::NetworkEdgeSecurityServiceInsertCall) and [*patch*](https://docs.rs/google-compute1/5.0.5+20240604/google_compute1/api::NetworkEdgeSecurityServicePatchCall)
* [network endpoint groups](https://docs.rs/google-compute1/5.0.5+20240604/google_compute1/api::NetworkEndpointGroup)
 * [*aggregated list*](https://docs.rs/google-compute1/5.0.5+20240604/google_compute1/api::NetworkEndpointGroupAggregatedListCall), [*attach network endpoints*](https://docs.rs/google-compute1/5.0.5+20240604/google_compute1/api::NetworkEndpointGroupAttachNetworkEndpointCall), [*delete*](https://docs.rs/google-compute1/5.0.5+20240604/google_compute1/api::NetworkEndpointGroupDeleteCall), [*detach network endpoints*](https://docs.rs/google-compute1/5.0.5+20240604/google_compute1/api::NetworkEndpointGroupDetachNetworkEndpointCall), [*get*](https://docs.rs/google-compute1/5.0.5+20240604/google_compute1/api::NetworkEndpointGroupGetCall), [*insert*](https://docs.rs/google-compute1/5.0.5+20240604/google_compute1/api::NetworkEndpointGroupInsertCall), [*list*](https://docs.rs/google-compute1/5.0.5+20240604/google_compute1/api::NetworkEndpointGroupListCall), [*list network endpoints*](https://docs.rs/google-compute1/5.0.5+20240604/google_compute1/api::NetworkEndpointGroupListNetworkEndpointCall) and [*test iam permissions*](https://docs.rs/google-compute1/5.0.5+20240604/google_compute1/api::NetworkEndpointGroupTestIamPermissionCall)
* network firewall policies
 * [*add association*](https://docs.rs/google-compute1/5.0.5+20240604/google_compute1/api::NetworkFirewallPolicyAddAssociationCall), [*add rule*](https://docs.rs/google-compute1/5.0.5+20240604/google_compute1/api::NetworkFirewallPolicyAddRuleCall), [*clone rules*](https://docs.rs/google-compute1/5.0.5+20240604/google_compute1/api::NetworkFirewallPolicyCloneRuleCall), [*delete*](https://docs.rs/google-compute1/5.0.5+20240604/google_compute1/api::NetworkFirewallPolicyDeleteCall), [*get*](https://docs.rs/google-compute1/5.0.5+20240604/google_compute1/api::NetworkFirewallPolicyGetCall), [*get association*](https://docs.rs/google-compute1/5.0.5+20240604/google_compute1/api::NetworkFirewallPolicyGetAssociationCall), [*get iam policy*](https://docs.rs/google-compute1/5.0.5+20240604/google_compute1/api::NetworkFirewallPolicyGetIamPolicyCall), [*get rule*](https://docs.rs/google-compute1/5.0.5+20240604/google_compute1/api::NetworkFirewallPolicyGetRuleCall), [*insert*](https://docs.rs/google-compute1/5.0.5+20240604/google_compute1/api::NetworkFirewallPolicyInsertCall), [*list*](https://docs.rs/google-compute1/5.0.5+20240604/google_compute1/api::NetworkFirewallPolicyListCall), [*patch*](https://docs.rs/google-compute1/5.0.5+20240604/google_compute1/api::NetworkFirewallPolicyPatchCall), [*patch rule*](https://docs.rs/google-compute1/5.0.5+20240604/google_compute1/api::NetworkFirewallPolicyPatchRuleCall), [*remove association*](https://docs.rs/google-compute1/5.0.5+20240604/google_compute1/api::NetworkFirewallPolicyRemoveAssociationCall), [*remove rule*](https://docs.rs/google-compute1/5.0.5+20240604/google_compute1/api::NetworkFirewallPolicyRemoveRuleCall), [*set iam policy*](https://docs.rs/google-compute1/5.0.5+20240604/google_compute1/api::NetworkFirewallPolicySetIamPolicyCall) and [*test iam permissions*](https://docs.rs/google-compute1/5.0.5+20240604/google_compute1/api::NetworkFirewallPolicyTestIamPermissionCall)
* [networks](https://docs.rs/google-compute1/5.0.5+20240604/google_compute1/api::Network)
 * [*add peering*](https://docs.rs/google-compute1/5.0.5+20240604/google_compute1/api::NetworkAddPeeringCall), [*delete*](https://docs.rs/google-compute1/5.0.5+20240604/google_compute1/api::NetworkDeleteCall), [*get*](https://docs.rs/google-compute1/5.0.5+20240604/google_compute1/api::NetworkGetCall), [*get effective firewalls*](https://docs.rs/google-compute1/5.0.5+20240604/google_compute1/api::NetworkGetEffectiveFirewallCall), [*insert*](https://docs.rs/google-compute1/5.0.5+20240604/google_compute1/api::NetworkInsertCall), [*list*](https://docs.rs/google-compute1/5.0.5+20240604/google_compute1/api::NetworkListCall), [*list peering routes*](https://docs.rs/google-compute1/5.0.5+20240604/google_compute1/api::NetworkListPeeringRouteCall), [*patch*](https://docs.rs/google-compute1/5.0.5+20240604/google_compute1/api::NetworkPatchCall), [*remove peering*](https://docs.rs/google-compute1/5.0.5+20240604/google_compute1/api::NetworkRemovePeeringCall), [*switch to custom mode*](https://docs.rs/google-compute1/5.0.5+20240604/google_compute1/api::NetworkSwitchToCustomModeCall) and [*update peering*](https://docs.rs/google-compute1/5.0.5+20240604/google_compute1/api::NetworkUpdatePeeringCall)
* [node groups](https://docs.rs/google-compute1/5.0.5+20240604/google_compute1/api::NodeGroup)
 * [*add nodes*](https://docs.rs/google-compute1/5.0.5+20240604/google_compute1/api::NodeGroupAddNodeCall), [*aggregated list*](https://docs.rs/google-compute1/5.0.5+20240604/google_compute1/api::NodeGroupAggregatedListCall), [*delete*](https://docs.rs/google-compute1/5.0.5+20240604/google_compute1/api::NodeGroupDeleteCall), [*delete nodes*](https://docs.rs/google-compute1/5.0.5+20240604/google_compute1/api::NodeGroupDeleteNodeCall), [*get*](https://docs.rs/google-compute1/5.0.5+20240604/google_compute1/api::NodeGroupGetCall), [*get iam policy*](https://docs.rs/google-compute1/5.0.5+20240604/google_compute1/api::NodeGroupGetIamPolicyCall), [*insert*](https://docs.rs/google-compute1/5.0.5+20240604/google_compute1/api::NodeGroupInsertCall), [*list*](https://docs.rs/google-compute1/5.0.5+20240604/google_compute1/api::NodeGroupListCall), [*list nodes*](https://docs.rs/google-compute1/5.0.5+20240604/google_compute1/api::NodeGroupListNodeCall), [*patch*](https://docs.rs/google-compute1/5.0.5+20240604/google_compute1/api::NodeGroupPatchCall), [*perform maintenance*](https://docs.rs/google-compute1/5.0.5+20240604/google_compute1/api::NodeGroupPerformMaintenanceCall), [*set iam policy*](https://docs.rs/google-compute1/5.0.5+20240604/google_compute1/api::NodeGroupSetIamPolicyCall), [*set node template*](https://docs.rs/google-compute1/5.0.5+20240604/google_compute1/api::NodeGroupSetNodeTemplateCall), [*simulate maintenance event*](https://docs.rs/google-compute1/5.0.5+20240604/google_compute1/api::NodeGroupSimulateMaintenanceEventCall) and [*test iam permissions*](https://docs.rs/google-compute1/5.0.5+20240604/google_compute1/api::NodeGroupTestIamPermissionCall)
* [node templates](https://docs.rs/google-compute1/5.0.5+20240604/google_compute1/api::NodeTemplate)
 * [*aggregated list*](https://docs.rs/google-compute1/5.0.5+20240604/google_compute1/api::NodeTemplateAggregatedListCall), [*delete*](https://docs.rs/google-compute1/5.0.5+20240604/google_compute1/api::NodeTemplateDeleteCall), [*get*](https://docs.rs/google-compute1/5.0.5+20240604/google_compute1/api::NodeTemplateGetCall), [*get iam policy*](https://docs.rs/google-compute1/5.0.5+20240604/google_compute1/api::NodeTemplateGetIamPolicyCall), [*insert*](https://docs.rs/google-compute1/5.0.5+20240604/google_compute1/api::NodeTemplateInsertCall), [*list*](https://docs.rs/google-compute1/5.0.5+20240604/google_compute1/api::NodeTemplateListCall), [*set iam policy*](https://docs.rs/google-compute1/5.0.5+20240604/google_compute1/api::NodeTemplateSetIamPolicyCall) and [*test iam permissions*](https://docs.rs/google-compute1/5.0.5+20240604/google_compute1/api::NodeTemplateTestIamPermissionCall)
* [node types](https://docs.rs/google-compute1/5.0.5+20240604/google_compute1/api::NodeType)
 * [*aggregated list*](https://docs.rs/google-compute1/5.0.5+20240604/google_compute1/api::NodeTypeAggregatedListCall), [*get*](https://docs.rs/google-compute1/5.0.5+20240604/google_compute1/api::NodeTypeGetCall) and [*list*](https://docs.rs/google-compute1/5.0.5+20240604/google_compute1/api::NodeTypeListCall)
* [packet mirrorings](https://docs.rs/google-compute1/5.0.5+20240604/google_compute1/api::PacketMirroring)
 * [*aggregated list*](https://docs.rs/google-compute1/5.0.5+20240604/google_compute1/api::PacketMirroringAggregatedListCall), [*delete*](https://docs.rs/google-compute1/5.0.5+20240604/google_compute1/api::PacketMirroringDeleteCall), [*get*](https://docs.rs/google-compute1/5.0.5+20240604/google_compute1/api::PacketMirroringGetCall), [*insert*](https://docs.rs/google-compute1/5.0.5+20240604/google_compute1/api::PacketMirroringInsertCall), [*list*](https://docs.rs/google-compute1/5.0.5+20240604/google_compute1/api::PacketMirroringListCall), [*patch*](https://docs.rs/google-compute1/5.0.5+20240604/google_compute1/api::PacketMirroringPatchCall) and [*test iam permissions*](https://docs.rs/google-compute1/5.0.5+20240604/google_compute1/api::PacketMirroringTestIamPermissionCall)
* [projects](https://docs.rs/google-compute1/5.0.5+20240604/google_compute1/api::Project)
 * [*disable xpn host*](https://docs.rs/google-compute1/5.0.5+20240604/google_compute1/api::ProjectDisableXpnHostCall), [*disable xpn resource*](https://docs.rs/google-compute1/5.0.5+20240604/google_compute1/api::ProjectDisableXpnResourceCall), [*enable xpn host*](https://docs.rs/google-compute1/5.0.5+20240604/google_compute1/api::ProjectEnableXpnHostCall), [*enable xpn resource*](https://docs.rs/google-compute1/5.0.5+20240604/google_compute1/api::ProjectEnableXpnResourceCall), [*get*](https://docs.rs/google-compute1/5.0.5+20240604/google_compute1/api::ProjectGetCall), [*get xpn host*](https://docs.rs/google-compute1/5.0.5+20240604/google_compute1/api::ProjectGetXpnHostCall), [*get xpn resources*](https://docs.rs/google-compute1/5.0.5+20240604/google_compute1/api::ProjectGetXpnResourceCall), [*list xpn hosts*](https://docs.rs/google-compute1/5.0.5+20240604/google_compute1/api::ProjectListXpnHostCall), [*move disk*](https://docs.rs/google-compute1/5.0.5+20240604/google_compute1/api::ProjectMoveDiskCall), [*move instance*](https://docs.rs/google-compute1/5.0.5+20240604/google_compute1/api::ProjectMoveInstanceCall), [*set cloud armor tier*](https://docs.rs/google-compute1/5.0.5+20240604/google_compute1/api::ProjectSetCloudArmorTierCall), [*set common instance metadata*](https://docs.rs/google-compute1/5.0.5+20240604/google_compute1/api::ProjectSetCommonInstanceMetadataCall), [*set default network tier*](https://docs.rs/google-compute1/5.0.5+20240604/google_compute1/api::ProjectSetDefaultNetworkTierCall) and [*set usage export bucket*](https://docs.rs/google-compute1/5.0.5+20240604/google_compute1/api::ProjectSetUsageExportBucketCall)
* [public advertised prefixes](https://docs.rs/google-compute1/5.0.5+20240604/google_compute1/api::PublicAdvertisedPrefix)
 * [*announce*](https://docs.rs/google-compute1/5.0.5+20240604/google_compute1/api::PublicAdvertisedPrefixAnnounceCall), [*delete*](https://docs.rs/google-compute1/5.0.5+20240604/google_compute1/api::PublicAdvertisedPrefixDeleteCall), [*get*](https://docs.rs/google-compute1/5.0.5+20240604/google_compute1/api::PublicAdvertisedPrefixGetCall), [*insert*](https://docs.rs/google-compute1/5.0.5+20240604/google_compute1/api::PublicAdvertisedPrefixInsertCall), [*list*](https://docs.rs/google-compute1/5.0.5+20240604/google_compute1/api::PublicAdvertisedPrefixListCall), [*patch*](https://docs.rs/google-compute1/5.0.5+20240604/google_compute1/api::PublicAdvertisedPrefixPatchCall) and [*withdraw*](https://docs.rs/google-compute1/5.0.5+20240604/google_compute1/api::PublicAdvertisedPrefixWithdrawCall)
* [public delegated prefixes](https://docs.rs/google-compute1/5.0.5+20240604/google_compute1/api::PublicDelegatedPrefix)
 * [*aggregated list*](https://docs.rs/google-compute1/5.0.5+20240604/google_compute1/api::PublicDelegatedPrefixAggregatedListCall), [*announce*](https://docs.rs/google-compute1/5.0.5+20240604/google_compute1/api::PublicDelegatedPrefixAnnounceCall), [*delete*](https://docs.rs/google-compute1/5.0.5+20240604/google_compute1/api::PublicDelegatedPrefixDeleteCall), [*get*](https://docs.rs/google-compute1/5.0.5+20240604/google_compute1/api::PublicDelegatedPrefixGetCall), [*insert*](https://docs.rs/google-compute1/5.0.5+20240604/google_compute1/api::PublicDelegatedPrefixInsertCall), [*list*](https://docs.rs/google-compute1/5.0.5+20240604/google_compute1/api::PublicDelegatedPrefixListCall), [*patch*](https://docs.rs/google-compute1/5.0.5+20240604/google_compute1/api::PublicDelegatedPrefixPatchCall) and [*withdraw*](https://docs.rs/google-compute1/5.0.5+20240604/google_compute1/api::PublicDelegatedPrefixWithdrawCall)
* region autoscalers
 * [*delete*](https://docs.rs/google-compute1/5.0.5+20240604/google_compute1/api::RegionAutoscalerDeleteCall), [*get*](https://docs.rs/google-compute1/5.0.5+20240604/google_compute1/api::RegionAutoscalerGetCall), [*insert*](https://docs.rs/google-compute1/5.0.5+20240604/google_compute1/api::RegionAutoscalerInsertCall), [*list*](https://docs.rs/google-compute1/5.0.5+20240604/google_compute1/api::RegionAutoscalerListCall), [*patch*](https://docs.rs/google-compute1/5.0.5+20240604/google_compute1/api::RegionAutoscalerPatchCall) and [*update*](https://docs.rs/google-compute1/5.0.5+20240604/google_compute1/api::RegionAutoscalerUpdateCall)
* region backend services
 * [*delete*](https://docs.rs/google-compute1/5.0.5+20240604/google_compute1/api::RegionBackendServiceDeleteCall), [*get*](https://docs.rs/google-compute1/5.0.5+20240604/google_compute1/api::RegionBackendServiceGetCall), [*get health*](https://docs.rs/google-compute1/5.0.5+20240604/google_compute1/api::RegionBackendServiceGetHealthCall), [*get iam policy*](https://docs.rs/google-compute1/5.0.5+20240604/google_compute1/api::RegionBackendServiceGetIamPolicyCall), [*insert*](https://docs.rs/google-compute1/5.0.5+20240604/google_compute1/api::RegionBackendServiceInsertCall), [*list*](https://docs.rs/google-compute1/5.0.5+20240604/google_compute1/api::RegionBackendServiceListCall), [*list usable*](https://docs.rs/google-compute1/5.0.5+20240604/google_compute1/api::RegionBackendServiceListUsableCall), [*patch*](https://docs.rs/google-compute1/5.0.5+20240604/google_compute1/api::RegionBackendServicePatchCall), [*set iam policy*](https://docs.rs/google-compute1/5.0.5+20240604/google_compute1/api::RegionBackendServiceSetIamPolicyCall), [*set security policy*](https://docs.rs/google-compute1/5.0.5+20240604/google_compute1/api::RegionBackendServiceSetSecurityPolicyCall), [*test iam permissions*](https://docs.rs/google-compute1/5.0.5+20240604/google_compute1/api::RegionBackendServiceTestIamPermissionCall) and [*update*](https://docs.rs/google-compute1/5.0.5+20240604/google_compute1/api::RegionBackendServiceUpdateCall)
* region commitments
 * [*aggregated list*](https://docs.rs/google-compute1/5.0.5+20240604/google_compute1/api::RegionCommitmentAggregatedListCall), [*get*](https://docs.rs/google-compute1/5.0.5+20240604/google_compute1/api::RegionCommitmentGetCall), [*insert*](https://docs.rs/google-compute1/5.0.5+20240604/google_compute1/api::RegionCommitmentInsertCall), [*list*](https://docs.rs/google-compute1/5.0.5+20240604/google_compute1/api::RegionCommitmentListCall) and [*update*](https://docs.rs/google-compute1/5.0.5+20240604/google_compute1/api::RegionCommitmentUpdateCall)
* region disk types
 * [*get*](https://docs.rs/google-compute1/5.0.5+20240604/google_compute1/api::RegionDiskTypeGetCall) and [*list*](https://docs.rs/google-compute1/5.0.5+20240604/google_compute1/api::RegionDiskTypeListCall)
* region disks
 * [*add resource policies*](https://docs.rs/google-compute1/5.0.5+20240604/google_compute1/api::RegionDiskAddResourcePolicyCall), [*bulk insert*](https://docs.rs/google-compute1/5.0.5+20240604/google_compute1/api::RegionDiskBulkInsertCall), [*create snapshot*](https://docs.rs/google-compute1/5.0.5+20240604/google_compute1/api::RegionDiskCreateSnapshotCall), [*delete*](https://docs.rs/google-compute1/5.0.5+20240604/google_compute1/api::RegionDiskDeleteCall), [*get*](https://docs.rs/google-compute1/5.0.5+20240604/google_compute1/api::RegionDiskGetCall), [*get iam policy*](https://docs.rs/google-compute1/5.0.5+20240604/google_compute1/api::RegionDiskGetIamPolicyCall), [*insert*](https://docs.rs/google-compute1/5.0.5+20240604/google_compute1/api::RegionDiskInsertCall), [*list*](https://docs.rs/google-compute1/5.0.5+20240604/google_compute1/api::RegionDiskListCall), [*remove resource policies*](https://docs.rs/google-compute1/5.0.5+20240604/google_compute1/api::RegionDiskRemoveResourcePolicyCall), [*resize*](https://docs.rs/google-compute1/5.0.5+20240604/google_compute1/api::RegionDiskResizeCall), [*set iam policy*](https://docs.rs/google-compute1/5.0.5+20240604/google_compute1/api::RegionDiskSetIamPolicyCall), [*set labels*](https://docs.rs/google-compute1/5.0.5+20240604/google_compute1/api::RegionDiskSetLabelCall), [*start async replication*](https://docs.rs/google-compute1/5.0.5+20240604/google_compute1/api::RegionDiskStartAsyncReplicationCall), [*stop async replication*](https://docs.rs/google-compute1/5.0.5+20240604/google_compute1/api::RegionDiskStopAsyncReplicationCall), [*stop group async replication*](https://docs.rs/google-compute1/5.0.5+20240604/google_compute1/api::RegionDiskStopGroupAsyncReplicationCall), [*test iam permissions*](https://docs.rs/google-compute1/5.0.5+20240604/google_compute1/api::RegionDiskTestIamPermissionCall) and [*update*](https://docs.rs/google-compute1/5.0.5+20240604/google_compute1/api::RegionDiskUpdateCall)
* region health check services
 * [*delete*](https://docs.rs/google-compute1/5.0.5+20240604/google_compute1/api::RegionHealthCheckServiceDeleteCall), [*get*](https://docs.rs/google-compute1/5.0.5+20240604/google_compute1/api::RegionHealthCheckServiceGetCall), [*insert*](https://docs.rs/google-compute1/5.0.5+20240604/google_compute1/api::RegionHealthCheckServiceInsertCall), [*list*](https://docs.rs/google-compute1/5.0.5+20240604/google_compute1/api::RegionHealthCheckServiceListCall) and [*patch*](https://docs.rs/google-compute1/5.0.5+20240604/google_compute1/api::RegionHealthCheckServicePatchCall)
* region health checks
 * [*delete*](https://docs.rs/google-compute1/5.0.5+20240604/google_compute1/api::RegionHealthCheckDeleteCall), [*get*](https://docs.rs/google-compute1/5.0.5+20240604/google_compute1/api::RegionHealthCheckGetCall), [*insert*](https://docs.rs/google-compute1/5.0.5+20240604/google_compute1/api::RegionHealthCheckInsertCall), [*list*](https://docs.rs/google-compute1/5.0.5+20240604/google_compute1/api::RegionHealthCheckListCall), [*patch*](https://docs.rs/google-compute1/5.0.5+20240604/google_compute1/api::RegionHealthCheckPatchCall) and [*update*](https://docs.rs/google-compute1/5.0.5+20240604/google_compute1/api::RegionHealthCheckUpdateCall)
* region instance group managers
 * [*abandon instances*](https://docs.rs/google-compute1/5.0.5+20240604/google_compute1/api::RegionInstanceGroupManagerAbandonInstanceCall), [*apply updates to instances*](https://docs.rs/google-compute1/5.0.5+20240604/google_compute1/api::RegionInstanceGroupManagerApplyUpdatesToInstanceCall), [*create instances*](https://docs.rs/google-compute1/5.0.5+20240604/google_compute1/api::RegionInstanceGroupManagerCreateInstanceCall), [*delete*](https://docs.rs/google-compute1/5.0.5+20240604/google_compute1/api::RegionInstanceGroupManagerDeleteCall), [*delete instances*](https://docs.rs/google-compute1/5.0.5+20240604/google_compute1/api::RegionInstanceGroupManagerDeleteInstanceCall), [*delete per instance configs*](https://docs.rs/google-compute1/5.0.5+20240604/google_compute1/api::RegionInstanceGroupManagerDeletePerInstanceConfigCall), [*get*](https://docs.rs/google-compute1/5.0.5+20240604/google_compute1/api::RegionInstanceGroupManagerGetCall), [*insert*](https://docs.rs/google-compute1/5.0.5+20240604/google_compute1/api::RegionInstanceGroupManagerInsertCall), [*list*](https://docs.rs/google-compute1/5.0.5+20240604/google_compute1/api::RegionInstanceGroupManagerListCall), [*list errors*](https://docs.rs/google-compute1/5.0.5+20240604/google_compute1/api::RegionInstanceGroupManagerListErrorCall), [*list managed instances*](https://docs.rs/google-compute1/5.0.5+20240604/google_compute1/api::RegionInstanceGroupManagerListManagedInstanceCall), [*list per instance configs*](https://docs.rs/google-compute1/5.0.5+20240604/google_compute1/api::RegionInstanceGroupManagerListPerInstanceConfigCall), [*patch*](https://docs.rs/google-compute1/5.0.5+20240604/google_compute1/api::RegionInstanceGroupManagerPatchCall), [*patch per instance configs*](https://docs.rs/google-compute1/5.0.5+20240604/google_compute1/api::RegionInstanceGroupManagerPatchPerInstanceConfigCall), [*recreate instances*](https://docs.rs/google-compute1/5.0.5+20240604/google_compute1/api::RegionInstanceGroupManagerRecreateInstanceCall), [*resize*](https://docs.rs/google-compute1/5.0.5+20240604/google_compute1/api::RegionInstanceGroupManagerResizeCall), [*set instance template*](https://docs.rs/google-compute1/5.0.5+20240604/google_compute1/api::RegionInstanceGroupManagerSetInstanceTemplateCall), [*set target pools*](https://docs.rs/google-compute1/5.0.5+20240604/google_compute1/api::RegionInstanceGroupManagerSetTargetPoolCall) and [*update per instance configs*](https://docs.rs/google-compute1/5.0.5+20240604/google_compute1/api::RegionInstanceGroupManagerUpdatePerInstanceConfigCall)
* region instance groups
 * [*get*](https://docs.rs/google-compute1/5.0.5+20240604/google_compute1/api::RegionInstanceGroupGetCall), [*list*](https://docs.rs/google-compute1/5.0.5+20240604/google_compute1/api::RegionInstanceGroupListCall), [*list instances*](https://docs.rs/google-compute1/5.0.5+20240604/google_compute1/api::RegionInstanceGroupListInstanceCall) and [*set named ports*](https://docs.rs/google-compute1/5.0.5+20240604/google_compute1/api::RegionInstanceGroupSetNamedPortCall)
* region instance templates
 * [*delete*](https://docs.rs/google-compute1/5.0.5+20240604/google_compute1/api::RegionInstanceTemplateDeleteCall), [*get*](https://docs.rs/google-compute1/5.0.5+20240604/google_compute1/api::RegionInstanceTemplateGetCall), [*insert*](https://docs.rs/google-compute1/5.0.5+20240604/google_compute1/api::RegionInstanceTemplateInsertCall) and [*list*](https://docs.rs/google-compute1/5.0.5+20240604/google_compute1/api::RegionInstanceTemplateListCall)
* region instances
 * [*bulk insert*](https://docs.rs/google-compute1/5.0.5+20240604/google_compute1/api::RegionInstanceBulkInsertCall)
* region instant snapshots
 * [*delete*](https://docs.rs/google-compute1/5.0.5+20240604/google_compute1/api::RegionInstantSnapshotDeleteCall), [*get*](https://docs.rs/google-compute1/5.0.5+20240604/google_compute1/api::RegionInstantSnapshotGetCall), [*get iam policy*](https://docs.rs/google-compute1/5.0.5+20240604/google_compute1/api::RegionInstantSnapshotGetIamPolicyCall), [*insert*](https://docs.rs/google-compute1/5.0.5+20240604/google_compute1/api::RegionInstantSnapshotInsertCall), [*list*](https://docs.rs/google-compute1/5.0.5+20240604/google_compute1/api::RegionInstantSnapshotListCall), [*set iam policy*](https://docs.rs/google-compute1/5.0.5+20240604/google_compute1/api::RegionInstantSnapshotSetIamPolicyCall), [*set labels*](https://docs.rs/google-compute1/5.0.5+20240604/google_compute1/api::RegionInstantSnapshotSetLabelCall) and [*test iam permissions*](https://docs.rs/google-compute1/5.0.5+20240604/google_compute1/api::RegionInstantSnapshotTestIamPermissionCall)
* region network endpoint groups
 * [*attach network endpoints*](https://docs.rs/google-compute1/5.0.5+20240604/google_compute1/api::RegionNetworkEndpointGroupAttachNetworkEndpointCall), [*delete*](https://docs.rs/google-compute1/5.0.5+20240604/google_compute1/api::RegionNetworkEndpointGroupDeleteCall), [*detach network endpoints*](https://docs.rs/google-compute1/5.0.5+20240604/google_compute1/api::RegionNetworkEndpointGroupDetachNetworkEndpointCall), [*get*](https://docs.rs/google-compute1/5.0.5+20240604/google_compute1/api::RegionNetworkEndpointGroupGetCall), [*insert*](https://docs.rs/google-compute1/5.0.5+20240604/google_compute1/api::RegionNetworkEndpointGroupInsertCall), [*list*](https://docs.rs/google-compute1/5.0.5+20240604/google_compute1/api::RegionNetworkEndpointGroupListCall) and [*list network endpoints*](https://docs.rs/google-compute1/5.0.5+20240604/google_compute1/api::RegionNetworkEndpointGroupListNetworkEndpointCall)
* region network firewall policies
 * [*add association*](https://docs.rs/google-compute1/5.0.5+20240604/google_compute1/api::RegionNetworkFirewallPolicyAddAssociationCall), [*add rule*](https://docs.rs/google-compute1/5.0.5+20240604/google_compute1/api::RegionNetworkFirewallPolicyAddRuleCall), [*clone rules*](https://docs.rs/google-compute1/5.0.5+20240604/google_compute1/api::RegionNetworkFirewallPolicyCloneRuleCall), [*delete*](https://docs.rs/google-compute1/5.0.5+20240604/google_compute1/api::RegionNetworkFirewallPolicyDeleteCall), [*get*](https://docs.rs/google-compute1/5.0.5+20240604/google_compute1/api::RegionNetworkFirewallPolicyGetCall), [*get association*](https://docs.rs/google-compute1/5.0.5+20240604/google_compute1/api::RegionNetworkFirewallPolicyGetAssociationCall), [*get effective firewalls*](https://docs.rs/google-compute1/5.0.5+20240604/google_compute1/api::RegionNetworkFirewallPolicyGetEffectiveFirewallCall), [*get iam policy*](https://docs.rs/google-compute1/5.0.5+20240604/google_compute1/api::RegionNetworkFirewallPolicyGetIamPolicyCall), [*get rule*](https://docs.rs/google-compute1/5.0.5+20240604/google_compute1/api::RegionNetworkFirewallPolicyGetRuleCall), [*insert*](https://docs.rs/google-compute1/5.0.5+20240604/google_compute1/api::RegionNetworkFirewallPolicyInsertCall), [*list*](https://docs.rs/google-compute1/5.0.5+20240604/google_compute1/api::RegionNetworkFirewallPolicyListCall), [*patch*](https://docs.rs/google-compute1/5.0.5+20240604/google_compute1/api::RegionNetworkFirewallPolicyPatchCall), [*patch rule*](https://docs.rs/google-compute1/5.0.5+20240604/google_compute1/api::RegionNetworkFirewallPolicyPatchRuleCall), [*remove association*](https://docs.rs/google-compute1/5.0.5+20240604/google_compute1/api::RegionNetworkFirewallPolicyRemoveAssociationCall), [*remove rule*](https://docs.rs/google-compute1/5.0.5+20240604/google_compute1/api::RegionNetworkFirewallPolicyRemoveRuleCall), [*set iam policy*](https://docs.rs/google-compute1/5.0.5+20240604/google_compute1/api::RegionNetworkFirewallPolicySetIamPolicyCall) and [*test iam permissions*](https://docs.rs/google-compute1/5.0.5+20240604/google_compute1/api::RegionNetworkFirewallPolicyTestIamPermissionCall)
* region notification endpoints
 * [*delete*](https://docs.rs/google-compute1/5.0.5+20240604/google_compute1/api::RegionNotificationEndpointDeleteCall), [*get*](https://docs.rs/google-compute1/5.0.5+20240604/google_compute1/api::RegionNotificationEndpointGetCall), [*insert*](https://docs.rs/google-compute1/5.0.5+20240604/google_compute1/api::RegionNotificationEndpointInsertCall) and [*list*](https://docs.rs/google-compute1/5.0.5+20240604/google_compute1/api::RegionNotificationEndpointListCall)
* region operations
 * [*delete*](https://docs.rs/google-compute1/5.0.5+20240604/google_compute1/api::RegionOperationDeleteCall), [*get*](https://docs.rs/google-compute1/5.0.5+20240604/google_compute1/api::RegionOperationGetCall), [*list*](https://docs.rs/google-compute1/5.0.5+20240604/google_compute1/api::RegionOperationListCall) and [*wait*](https://docs.rs/google-compute1/5.0.5+20240604/google_compute1/api::RegionOperationWaitCall)
* region security policies
 * [*add rule*](https://docs.rs/google-compute1/5.0.5+20240604/google_compute1/api::RegionSecurityPolicyAddRuleCall), [*delete*](https://docs.rs/google-compute1/5.0.5+20240604/google_compute1/api::RegionSecurityPolicyDeleteCall), [*get*](https://docs.rs/google-compute1/5.0.5+20240604/google_compute1/api::RegionSecurityPolicyGetCall), [*get rule*](https://docs.rs/google-compute1/5.0.5+20240604/google_compute1/api::RegionSecurityPolicyGetRuleCall), [*insert*](https://docs.rs/google-compute1/5.0.5+20240604/google_compute1/api::RegionSecurityPolicyInsertCall), [*list*](https://docs.rs/google-compute1/5.0.5+20240604/google_compute1/api::RegionSecurityPolicyListCall), [*patch*](https://docs.rs/google-compute1/5.0.5+20240604/google_compute1/api::RegionSecurityPolicyPatchCall), [*patch rule*](https://docs.rs/google-compute1/5.0.5+20240604/google_compute1/api::RegionSecurityPolicyPatchRuleCall) and [*remove rule*](https://docs.rs/google-compute1/5.0.5+20240604/google_compute1/api::RegionSecurityPolicyRemoveRuleCall)
* region ssl certificates
 * [*delete*](https://docs.rs/google-compute1/5.0.5+20240604/google_compute1/api::RegionSslCertificateDeleteCall), [*get*](https://docs.rs/google-compute1/5.0.5+20240604/google_compute1/api::RegionSslCertificateGetCall), [*insert*](https://docs.rs/google-compute1/5.0.5+20240604/google_compute1/api::RegionSslCertificateInsertCall) and [*list*](https://docs.rs/google-compute1/5.0.5+20240604/google_compute1/api::RegionSslCertificateListCall)
* region ssl policies
 * [*delete*](https://docs.rs/google-compute1/5.0.5+20240604/google_compute1/api::RegionSslPolicyDeleteCall), [*get*](https://docs.rs/google-compute1/5.0.5+20240604/google_compute1/api::RegionSslPolicyGetCall), [*insert*](https://docs.rs/google-compute1/5.0.5+20240604/google_compute1/api::RegionSslPolicyInsertCall), [*list*](https://docs.rs/google-compute1/5.0.5+20240604/google_compute1/api::RegionSslPolicyListCall), [*list available features*](https://docs.rs/google-compute1/5.0.5+20240604/google_compute1/api::RegionSslPolicyListAvailableFeatureCall) and [*patch*](https://docs.rs/google-compute1/5.0.5+20240604/google_compute1/api::RegionSslPolicyPatchCall)
* region target http proxies
 * [*delete*](https://docs.rs/google-compute1/5.0.5+20240604/google_compute1/api::RegionTargetHttpProxyDeleteCall), [*get*](https://docs.rs/google-compute1/5.0.5+20240604/google_compute1/api::RegionTargetHttpProxyGetCall), [*insert*](https://docs.rs/google-compute1/5.0.5+20240604/google_compute1/api::RegionTargetHttpProxyInsertCall), [*list*](https://docs.rs/google-compute1/5.0.5+20240604/google_compute1/api::RegionTargetHttpProxyListCall) and [*set url map*](https://docs.rs/google-compute1/5.0.5+20240604/google_compute1/api::RegionTargetHttpProxySetUrlMapCall)
* region target https proxies
 * [*delete*](https://docs.rs/google-compute1/5.0.5+20240604/google_compute1/api::RegionTargetHttpsProxyDeleteCall), [*get*](https://docs.rs/google-compute1/5.0.5+20240604/google_compute1/api::RegionTargetHttpsProxyGetCall), [*insert*](https://docs.rs/google-compute1/5.0.5+20240604/google_compute1/api::RegionTargetHttpsProxyInsertCall), [*list*](https://docs.rs/google-compute1/5.0.5+20240604/google_compute1/api::RegionTargetHttpsProxyListCall), [*patch*](https://docs.rs/google-compute1/5.0.5+20240604/google_compute1/api::RegionTargetHttpsProxyPatchCall), [*set ssl certificates*](https://docs.rs/google-compute1/5.0.5+20240604/google_compute1/api::RegionTargetHttpsProxySetSslCertificateCall) and [*set url map*](https://docs.rs/google-compute1/5.0.5+20240604/google_compute1/api::RegionTargetHttpsProxySetUrlMapCall)
* region target tcp proxies
 * [*delete*](https://docs.rs/google-compute1/5.0.5+20240604/google_compute1/api::RegionTargetTcpProxyDeleteCall), [*get*](https://docs.rs/google-compute1/5.0.5+20240604/google_compute1/api::RegionTargetTcpProxyGetCall), [*insert*](https://docs.rs/google-compute1/5.0.5+20240604/google_compute1/api::RegionTargetTcpProxyInsertCall) and [*list*](https://docs.rs/google-compute1/5.0.5+20240604/google_compute1/api::RegionTargetTcpProxyListCall)
* region url maps
 * [*delete*](https://docs.rs/google-compute1/5.0.5+20240604/google_compute1/api::RegionUrlMapDeleteCall), [*get*](https://docs.rs/google-compute1/5.0.5+20240604/google_compute1/api::RegionUrlMapGetCall), [*insert*](https://docs.rs/google-compute1/5.0.5+20240604/google_compute1/api::RegionUrlMapInsertCall), [*list*](https://docs.rs/google-compute1/5.0.5+20240604/google_compute1/api::RegionUrlMapListCall), [*patch*](https://docs.rs/google-compute1/5.0.5+20240604/google_compute1/api::RegionUrlMapPatchCall), [*update*](https://docs.rs/google-compute1/5.0.5+20240604/google_compute1/api::RegionUrlMapUpdateCall) and [*validate*](https://docs.rs/google-compute1/5.0.5+20240604/google_compute1/api::RegionUrlMapValidateCall)
* region zones
 * [*list*](https://docs.rs/google-compute1/5.0.5+20240604/google_compute1/api::RegionZoneListCall)
* [regions](https://docs.rs/google-compute1/5.0.5+20240604/google_compute1/api::Region)
 * [*get*](https://docs.rs/google-compute1/5.0.5+20240604/google_compute1/api::RegionGetCall) and [*list*](https://docs.rs/google-compute1/5.0.5+20240604/google_compute1/api::RegionListCall)
* [reservations](https://docs.rs/google-compute1/5.0.5+20240604/google_compute1/api::Reservation)
 * [*aggregated list*](https://docs.rs/google-compute1/5.0.5+20240604/google_compute1/api::ReservationAggregatedListCall), [*delete*](https://docs.rs/google-compute1/5.0.5+20240604/google_compute1/api::ReservationDeleteCall), [*get*](https://docs.rs/google-compute1/5.0.5+20240604/google_compute1/api::ReservationGetCall), [*get iam policy*](https://docs.rs/google-compute1/5.0.5+20240604/google_compute1/api::ReservationGetIamPolicyCall), [*insert*](https://docs.rs/google-compute1/5.0.5+20240604/google_compute1/api::ReservationInsertCall), [*list*](https://docs.rs/google-compute1/5.0.5+20240604/google_compute1/api::ReservationListCall), [*resize*](https://docs.rs/google-compute1/5.0.5+20240604/google_compute1/api::ReservationResizeCall), [*set iam policy*](https://docs.rs/google-compute1/5.0.5+20240604/google_compute1/api::ReservationSetIamPolicyCall), [*test iam permissions*](https://docs.rs/google-compute1/5.0.5+20240604/google_compute1/api::ReservationTestIamPermissionCall) and [*update*](https://docs.rs/google-compute1/5.0.5+20240604/google_compute1/api::ReservationUpdateCall)
* [resource policies](https://docs.rs/google-compute1/5.0.5+20240604/google_compute1/api::ResourcePolicy)
 * [*aggregated list*](https://docs.rs/google-compute1/5.0.5+20240604/google_compute1/api::ResourcePolicyAggregatedListCall), [*delete*](https://docs.rs/google-compute1/5.0.5+20240604/google_compute1/api::ResourcePolicyDeleteCall), [*get*](https://docs.rs/google-compute1/5.0.5+20240604/google_compute1/api::ResourcePolicyGetCall), [*get iam policy*](https://docs.rs/google-compute1/5.0.5+20240604/google_compute1/api::ResourcePolicyGetIamPolicyCall), [*insert*](https://docs.rs/google-compute1/5.0.5+20240604/google_compute1/api::ResourcePolicyInsertCall), [*list*](https://docs.rs/google-compute1/5.0.5+20240604/google_compute1/api::ResourcePolicyListCall), [*patch*](https://docs.rs/google-compute1/5.0.5+20240604/google_compute1/api::ResourcePolicyPatchCall), [*set iam policy*](https://docs.rs/google-compute1/5.0.5+20240604/google_compute1/api::ResourcePolicySetIamPolicyCall) and [*test iam permissions*](https://docs.rs/google-compute1/5.0.5+20240604/google_compute1/api::ResourcePolicyTestIamPermissionCall)
* [routers](https://docs.rs/google-compute1/5.0.5+20240604/google_compute1/api::Router)
 * [*aggregated list*](https://docs.rs/google-compute1/5.0.5+20240604/google_compute1/api::RouterAggregatedListCall), [*delete*](https://docs.rs/google-compute1/5.0.5+20240604/google_compute1/api::RouterDeleteCall), [*get*](https://docs.rs/google-compute1/5.0.5+20240604/google_compute1/api::RouterGetCall), [*get nat ip info*](https://docs.rs/google-compute1/5.0.5+20240604/google_compute1/api::RouterGetNatIpInfoCall), [*get nat mapping info*](https://docs.rs/google-compute1/5.0.5+20240604/google_compute1/api::RouterGetNatMappingInfoCall), [*get router status*](https://docs.rs/google-compute1/5.0.5+20240604/google_compute1/api::RouterGetRouterStatuCall), [*insert*](https://docs.rs/google-compute1/5.0.5+20240604/google_compute1/api::RouterInsertCall), [*list*](https://docs.rs/google-compute1/5.0.5+20240604/google_compute1/api::RouterListCall), [*patch*](https://docs.rs/google-compute1/5.0.5+20240604/google_compute1/api::RouterPatchCall), [*preview*](https://docs.rs/google-compute1/5.0.5+20240604/google_compute1/api::RouterPreviewCall) and [*update*](https://docs.rs/google-compute1/5.0.5+20240604/google_compute1/api::RouterUpdateCall)
* [routes](https://docs.rs/google-compute1/5.0.5+20240604/google_compute1/api::Route)
 * [*delete*](https://docs.rs/google-compute1/5.0.5+20240604/google_compute1/api::RouteDeleteCall), [*get*](https://docs.rs/google-compute1/5.0.5+20240604/google_compute1/api::RouteGetCall), [*insert*](https://docs.rs/google-compute1/5.0.5+20240604/google_compute1/api::RouteInsertCall) and [*list*](https://docs.rs/google-compute1/5.0.5+20240604/google_compute1/api::RouteListCall)
* [security policies](https://docs.rs/google-compute1/5.0.5+20240604/google_compute1/api::SecurityPolicy)
 * [*add rule*](https://docs.rs/google-compute1/5.0.5+20240604/google_compute1/api::SecurityPolicyAddRuleCall), [*aggregated list*](https://docs.rs/google-compute1/5.0.5+20240604/google_compute1/api::SecurityPolicyAggregatedListCall), [*delete*](https://docs.rs/google-compute1/5.0.5+20240604/google_compute1/api::SecurityPolicyDeleteCall), [*get*](https://docs.rs/google-compute1/5.0.5+20240604/google_compute1/api::SecurityPolicyGetCall), [*get rule*](https://docs.rs/google-compute1/5.0.5+20240604/google_compute1/api::SecurityPolicyGetRuleCall), [*insert*](https://docs.rs/google-compute1/5.0.5+20240604/google_compute1/api::SecurityPolicyInsertCall), [*list*](https://docs.rs/google-compute1/5.0.5+20240604/google_compute1/api::SecurityPolicyListCall), [*list preconfigured expression sets*](https://docs.rs/google-compute1/5.0.5+20240604/google_compute1/api::SecurityPolicyListPreconfiguredExpressionSetCall), [*patch*](https://docs.rs/google-compute1/5.0.5+20240604/google_compute1/api::SecurityPolicyPatchCall), [*patch rule*](https://docs.rs/google-compute1/5.0.5+20240604/google_compute1/api::SecurityPolicyPatchRuleCall), [*remove rule*](https://docs.rs/google-compute1/5.0.5+20240604/google_compute1/api::SecurityPolicyRemoveRuleCall) and [*set labels*](https://docs.rs/google-compute1/5.0.5+20240604/google_compute1/api::SecurityPolicySetLabelCall)
* [service attachments](https://docs.rs/google-compute1/5.0.5+20240604/google_compute1/api::ServiceAttachment)
 * [*aggregated list*](https://docs.rs/google-compute1/5.0.5+20240604/google_compute1/api::ServiceAttachmentAggregatedListCall), [*delete*](https://docs.rs/google-compute1/5.0.5+20240604/google_compute1/api::ServiceAttachmentDeleteCall), [*get*](https://docs.rs/google-compute1/5.0.5+20240604/google_compute1/api::ServiceAttachmentGetCall), [*get iam policy*](https://docs.rs/google-compute1/5.0.5+20240604/google_compute1/api::ServiceAttachmentGetIamPolicyCall), [*insert*](https://docs.rs/google-compute1/5.0.5+20240604/google_compute1/api::ServiceAttachmentInsertCall), [*list*](https://docs.rs/google-compute1/5.0.5+20240604/google_compute1/api::ServiceAttachmentListCall), [*patch*](https://docs.rs/google-compute1/5.0.5+20240604/google_compute1/api::ServiceAttachmentPatchCall), [*set iam policy*](https://docs.rs/google-compute1/5.0.5+20240604/google_compute1/api::ServiceAttachmentSetIamPolicyCall) and [*test iam permissions*](https://docs.rs/google-compute1/5.0.5+20240604/google_compute1/api::ServiceAttachmentTestIamPermissionCall)
* snapshot settings
 * [*get*](https://docs.rs/google-compute1/5.0.5+20240604/google_compute1/api::SnapshotSettingGetCall) and [*patch*](https://docs.rs/google-compute1/5.0.5+20240604/google_compute1/api::SnapshotSettingPatchCall)
* [snapshots](https://docs.rs/google-compute1/5.0.5+20240604/google_compute1/api::Snapshot)
 * [*delete*](https://docs.rs/google-compute1/5.0.5+20240604/google_compute1/api::SnapshotDeleteCall), [*get*](https://docs.rs/google-compute1/5.0.5+20240604/google_compute1/api::SnapshotGetCall), [*get iam policy*](https://docs.rs/google-compute1/5.0.5+20240604/google_compute1/api::SnapshotGetIamPolicyCall), [*insert*](https://docs.rs/google-compute1/5.0.5+20240604/google_compute1/api::SnapshotInsertCall), [*list*](https://docs.rs/google-compute1/5.0.5+20240604/google_compute1/api::SnapshotListCall), [*set iam policy*](https://docs.rs/google-compute1/5.0.5+20240604/google_compute1/api::SnapshotSetIamPolicyCall), [*set labels*](https://docs.rs/google-compute1/5.0.5+20240604/google_compute1/api::SnapshotSetLabelCall) and [*test iam permissions*](https://docs.rs/google-compute1/5.0.5+20240604/google_compute1/api::SnapshotTestIamPermissionCall)
* [ssl certificates](https://docs.rs/google-compute1/5.0.5+20240604/google_compute1/api::SslCertificate)
 * [*aggregated list*](https://docs.rs/google-compute1/5.0.5+20240604/google_compute1/api::SslCertificateAggregatedListCall), [*delete*](https://docs.rs/google-compute1/5.0.5+20240604/google_compute1/api::SslCertificateDeleteCall), [*get*](https://docs.rs/google-compute1/5.0.5+20240604/google_compute1/api::SslCertificateGetCall), [*insert*](https://docs.rs/google-compute1/5.0.5+20240604/google_compute1/api::SslCertificateInsertCall) and [*list*](https://docs.rs/google-compute1/5.0.5+20240604/google_compute1/api::SslCertificateListCall)
* [ssl policies](https://docs.rs/google-compute1/5.0.5+20240604/google_compute1/api::SslPolicy)
 * [*aggregated list*](https://docs.rs/google-compute1/5.0.5+20240604/google_compute1/api::SslPolicyAggregatedListCall), [*delete*](https://docs.rs/google-compute1/5.0.5+20240604/google_compute1/api::SslPolicyDeleteCall), [*get*](https://docs.rs/google-compute1/5.0.5+20240604/google_compute1/api::SslPolicyGetCall), [*insert*](https://docs.rs/google-compute1/5.0.5+20240604/google_compute1/api::SslPolicyInsertCall), [*list*](https://docs.rs/google-compute1/5.0.5+20240604/google_compute1/api::SslPolicyListCall), [*list available features*](https://docs.rs/google-compute1/5.0.5+20240604/google_compute1/api::SslPolicyListAvailableFeatureCall) and [*patch*](https://docs.rs/google-compute1/5.0.5+20240604/google_compute1/api::SslPolicyPatchCall)
* [storage pool types](https://docs.rs/google-compute1/5.0.5+20240604/google_compute1/api::StoragePoolType)
 * [*aggregated list*](https://docs.rs/google-compute1/5.0.5+20240604/google_compute1/api::StoragePoolTypeAggregatedListCall), [*get*](https://docs.rs/google-compute1/5.0.5+20240604/google_compute1/api::StoragePoolTypeGetCall) and [*list*](https://docs.rs/google-compute1/5.0.5+20240604/google_compute1/api::StoragePoolTypeListCall)
* [storage pools](https://docs.rs/google-compute1/5.0.5+20240604/google_compute1/api::StoragePool)
 * [*aggregated list*](https://docs.rs/google-compute1/5.0.5+20240604/google_compute1/api::StoragePoolAggregatedListCall), [*delete*](https://docs.rs/google-compute1/5.0.5+20240604/google_compute1/api::StoragePoolDeleteCall), [*get*](https://docs.rs/google-compute1/5.0.5+20240604/google_compute1/api::StoragePoolGetCall), [*get iam policy*](https://docs.rs/google-compute1/5.0.5+20240604/google_compute1/api::StoragePoolGetIamPolicyCall), [*insert*](https://docs.rs/google-compute1/5.0.5+20240604/google_compute1/api::StoragePoolInsertCall), [*list*](https://docs.rs/google-compute1/5.0.5+20240604/google_compute1/api::StoragePoolListCall), [*list disks*](https://docs.rs/google-compute1/5.0.5+20240604/google_compute1/api::StoragePoolListDiskCall), [*set iam policy*](https://docs.rs/google-compute1/5.0.5+20240604/google_compute1/api::StoragePoolSetIamPolicyCall), [*test iam permissions*](https://docs.rs/google-compute1/5.0.5+20240604/google_compute1/api::StoragePoolTestIamPermissionCall) and [*update*](https://docs.rs/google-compute1/5.0.5+20240604/google_compute1/api::StoragePoolUpdateCall)
* [subnetworks](https://docs.rs/google-compute1/5.0.5+20240604/google_compute1/api::Subnetwork)
 * [*aggregated list*](https://docs.rs/google-compute1/5.0.5+20240604/google_compute1/api::SubnetworkAggregatedListCall), [*delete*](https://docs.rs/google-compute1/5.0.5+20240604/google_compute1/api::SubnetworkDeleteCall), [*expand ip cidr range*](https://docs.rs/google-compute1/5.0.5+20240604/google_compute1/api::SubnetworkExpandIpCidrRangeCall), [*get*](https://docs.rs/google-compute1/5.0.5+20240604/google_compute1/api::SubnetworkGetCall), [*get iam policy*](https://docs.rs/google-compute1/5.0.5+20240604/google_compute1/api::SubnetworkGetIamPolicyCall), [*insert*](https://docs.rs/google-compute1/5.0.5+20240604/google_compute1/api::SubnetworkInsertCall), [*list*](https://docs.rs/google-compute1/5.0.5+20240604/google_compute1/api::SubnetworkListCall), [*list usable*](https://docs.rs/google-compute1/5.0.5+20240604/google_compute1/api::SubnetworkListUsableCall), [*patch*](https://docs.rs/google-compute1/5.0.5+20240604/google_compute1/api::SubnetworkPatchCall), [*set iam policy*](https://docs.rs/google-compute1/5.0.5+20240604/google_compute1/api::SubnetworkSetIamPolicyCall), [*set private ip google access*](https://docs.rs/google-compute1/5.0.5+20240604/google_compute1/api::SubnetworkSetPrivateIpGoogleAccesCall) and [*test iam permissions*](https://docs.rs/google-compute1/5.0.5+20240604/google_compute1/api::SubnetworkTestIamPermissionCall)
* [target grpc proxies](https://docs.rs/google-compute1/5.0.5+20240604/google_compute1/api::TargetGrpcProxy)
 * [*delete*](https://docs.rs/google-compute1/5.0.5+20240604/google_compute1/api::TargetGrpcProxyDeleteCall), [*get*](https://docs.rs/google-compute1/5.0.5+20240604/google_compute1/api::TargetGrpcProxyGetCall), [*insert*](https://docs.rs/google-compute1/5.0.5+20240604/google_compute1/api::TargetGrpcProxyInsertCall), [*list*](https://docs.rs/google-compute1/5.0.5+20240604/google_compute1/api::TargetGrpcProxyListCall) and [*patch*](https://docs.rs/google-compute1/5.0.5+20240604/google_compute1/api::TargetGrpcProxyPatchCall)
* [target http proxies](https://docs.rs/google-compute1/5.0.5+20240604/google_compute1/api::TargetHttpProxy)
 * [*aggregated list*](https://docs.rs/google-compute1/5.0.5+20240604/google_compute1/api::TargetHttpProxyAggregatedListCall), [*delete*](https://docs.rs/google-compute1/5.0.5+20240604/google_compute1/api::TargetHttpProxyDeleteCall), [*get*](https://docs.rs/google-compute1/5.0.5+20240604/google_compute1/api::TargetHttpProxyGetCall), [*insert*](https://docs.rs/google-compute1/5.0.5+20240604/google_compute1/api::TargetHttpProxyInsertCall), [*list*](https://docs.rs/google-compute1/5.0.5+20240604/google_compute1/api::TargetHttpProxyListCall), [*patch*](https://docs.rs/google-compute1/5.0.5+20240604/google_compute1/api::TargetHttpProxyPatchCall) and [*set url map*](https://docs.rs/google-compute1/5.0.5+20240604/google_compute1/api::TargetHttpProxySetUrlMapCall)
* [target https proxies](https://docs.rs/google-compute1/5.0.5+20240604/google_compute1/api::TargetHttpsProxy)
 * [*aggregated list*](https://docs.rs/google-compute1/5.0.5+20240604/google_compute1/api::TargetHttpsProxyAggregatedListCall), [*delete*](https://docs.rs/google-compute1/5.0.5+20240604/google_compute1/api::TargetHttpsProxyDeleteCall), [*get*](https://docs.rs/google-compute1/5.0.5+20240604/google_compute1/api::TargetHttpsProxyGetCall), [*insert*](https://docs.rs/google-compute1/5.0.5+20240604/google_compute1/api::TargetHttpsProxyInsertCall), [*list*](https://docs.rs/google-compute1/5.0.5+20240604/google_compute1/api::TargetHttpsProxyListCall), [*patch*](https://docs.rs/google-compute1/5.0.5+20240604/google_compute1/api::TargetHttpsProxyPatchCall), [*set certificate map*](https://docs.rs/google-compute1/5.0.5+20240604/google_compute1/api::TargetHttpsProxySetCertificateMapCall), [*set quic override*](https://docs.rs/google-compute1/5.0.5+20240604/google_compute1/api::TargetHttpsProxySetQuicOverrideCall), [*set ssl certificates*](https://docs.rs/google-compute1/5.0.5+20240604/google_compute1/api::TargetHttpsProxySetSslCertificateCall), [*set ssl policy*](https://docs.rs/google-compute1/5.0.5+20240604/google_compute1/api::TargetHttpsProxySetSslPolicyCall) and [*set url map*](https://docs.rs/google-compute1/5.0.5+20240604/google_compute1/api::TargetHttpsProxySetUrlMapCall)
* [target instances](https://docs.rs/google-compute1/5.0.5+20240604/google_compute1/api::TargetInstance)
 * [*aggregated list*](https://docs.rs/google-compute1/5.0.5+20240604/google_compute1/api::TargetInstanceAggregatedListCall), [*delete*](https://docs.rs/google-compute1/5.0.5+20240604/google_compute1/api::TargetInstanceDeleteCall), [*get*](https://docs.rs/google-compute1/5.0.5+20240604/google_compute1/api::TargetInstanceGetCall), [*insert*](https://docs.rs/google-compute1/5.0.5+20240604/google_compute1/api::TargetInstanceInsertCall), [*list*](https://docs.rs/google-compute1/5.0.5+20240604/google_compute1/api::TargetInstanceListCall) and [*set security policy*](https://docs.rs/google-compute1/5.0.5+20240604/google_compute1/api::TargetInstanceSetSecurityPolicyCall)
* [target pools](https://docs.rs/google-compute1/5.0.5+20240604/google_compute1/api::TargetPool)
 * [*add health check*](https://docs.rs/google-compute1/5.0.5+20240604/google_compute1/api::TargetPoolAddHealthCheckCall), [*add instance*](https://docs.rs/google-compute1/5.0.5+20240604/google_compute1/api::TargetPoolAddInstanceCall), [*aggregated list*](https://docs.rs/google-compute1/5.0.5+20240604/google_compute1/api::TargetPoolAggregatedListCall), [*delete*](https://docs.rs/google-compute1/5.0.5+20240604/google_compute1/api::TargetPoolDeleteCall), [*get*](https://docs.rs/google-compute1/5.0.5+20240604/google_compute1/api::TargetPoolGetCall), [*get health*](https://docs.rs/google-compute1/5.0.5+20240604/google_compute1/api::TargetPoolGetHealthCall), [*insert*](https://docs.rs/google-compute1/5.0.5+20240604/google_compute1/api::TargetPoolInsertCall), [*list*](https://docs.rs/google-compute1/5.0.5+20240604/google_compute1/api::TargetPoolListCall), [*remove health check*](https://docs.rs/google-compute1/5.0.5+20240604/google_compute1/api::TargetPoolRemoveHealthCheckCall), [*remove instance*](https://docs.rs/google-compute1/5.0.5+20240604/google_compute1/api::TargetPoolRemoveInstanceCall), [*set backup*](https://docs.rs/google-compute1/5.0.5+20240604/google_compute1/api::TargetPoolSetBackupCall) and [*set security policy*](https://docs.rs/google-compute1/5.0.5+20240604/google_compute1/api::TargetPoolSetSecurityPolicyCall)
* [target ssl proxies](https://docs.rs/google-compute1/5.0.5+20240604/google_compute1/api::TargetSslProxy)
 * [*delete*](https://docs.rs/google-compute1/5.0.5+20240604/google_compute1/api::TargetSslProxyDeleteCall), [*get*](https://docs.rs/google-compute1/5.0.5+20240604/google_compute1/api::TargetSslProxyGetCall), [*insert*](https://docs.rs/google-compute1/5.0.5+20240604/google_compute1/api::TargetSslProxyInsertCall), [*list*](https://docs.rs/google-compute1/5.0.5+20240604/google_compute1/api::TargetSslProxyListCall), [*set backend service*](https://docs.rs/google-compute1/5.0.5+20240604/google_compute1/api::TargetSslProxySetBackendServiceCall), [*set certificate map*](https://docs.rs/google-compute1/5.0.5+20240604/google_compute1/api::TargetSslProxySetCertificateMapCall), [*set proxy header*](https://docs.rs/google-compute1/5.0.5+20240604/google_compute1/api::TargetSslProxySetProxyHeaderCall), [*set ssl certificates*](https://docs.rs/google-compute1/5.0.5+20240604/google_compute1/api::TargetSslProxySetSslCertificateCall) and [*set ssl policy*](https://docs.rs/google-compute1/5.0.5+20240604/google_compute1/api::TargetSslProxySetSslPolicyCall)
* [target tcp proxies](https://docs.rs/google-compute1/5.0.5+20240604/google_compute1/api::TargetTcpProxy)
 * [*aggregated list*](https://docs.rs/google-compute1/5.0.5+20240604/google_compute1/api::TargetTcpProxyAggregatedListCall), [*delete*](https://docs.rs/google-compute1/5.0.5+20240604/google_compute1/api::TargetTcpProxyDeleteCall), [*get*](https://docs.rs/google-compute1/5.0.5+20240604/google_compute1/api::TargetTcpProxyGetCall), [*insert*](https://docs.rs/google-compute1/5.0.5+20240604/google_compute1/api::TargetTcpProxyInsertCall), [*list*](https://docs.rs/google-compute1/5.0.5+20240604/google_compute1/api::TargetTcpProxyListCall), [*set backend service*](https://docs.rs/google-compute1/5.0.5+20240604/google_compute1/api::TargetTcpProxySetBackendServiceCall) and [*set proxy header*](https://docs.rs/google-compute1/5.0.5+20240604/google_compute1/api::TargetTcpProxySetProxyHeaderCall)
* [target vpn gateways](https://docs.rs/google-compute1/5.0.5+20240604/google_compute1/api::TargetVpnGateway)
 * [*aggregated list*](https://docs.rs/google-compute1/5.0.5+20240604/google_compute1/api::TargetVpnGatewayAggregatedListCall), [*delete*](https://docs.rs/google-compute1/5.0.5+20240604/google_compute1/api::TargetVpnGatewayDeleteCall), [*get*](https://docs.rs/google-compute1/5.0.5+20240604/google_compute1/api::TargetVpnGatewayGetCall), [*insert*](https://docs.rs/google-compute1/5.0.5+20240604/google_compute1/api::TargetVpnGatewayInsertCall), [*list*](https://docs.rs/google-compute1/5.0.5+20240604/google_compute1/api::TargetVpnGatewayListCall) and [*set labels*](https://docs.rs/google-compute1/5.0.5+20240604/google_compute1/api::TargetVpnGatewaySetLabelCall)
* [url maps](https://docs.rs/google-compute1/5.0.5+20240604/google_compute1/api::UrlMap)
 * [*aggregated list*](https://docs.rs/google-compute1/5.0.5+20240604/google_compute1/api::UrlMapAggregatedListCall), [*delete*](https://docs.rs/google-compute1/5.0.5+20240604/google_compute1/api::UrlMapDeleteCall), [*get*](https://docs.rs/google-compute1/5.0.5+20240604/google_compute1/api::UrlMapGetCall), [*insert*](https://docs.rs/google-compute1/5.0.5+20240604/google_compute1/api::UrlMapInsertCall), [*invalidate cache*](https://docs.rs/google-compute1/5.0.5+20240604/google_compute1/api::UrlMapInvalidateCacheCall), [*list*](https://docs.rs/google-compute1/5.0.5+20240604/google_compute1/api::UrlMapListCall), [*patch*](https://docs.rs/google-compute1/5.0.5+20240604/google_compute1/api::UrlMapPatchCall), [*update*](https://docs.rs/google-compute1/5.0.5+20240604/google_compute1/api::UrlMapUpdateCall) and [*validate*](https://docs.rs/google-compute1/5.0.5+20240604/google_compute1/api::UrlMapValidateCall)
* [vpn gateways](https://docs.rs/google-compute1/5.0.5+20240604/google_compute1/api::VpnGateway)
 * [*aggregated list*](https://docs.rs/google-compute1/5.0.5+20240604/google_compute1/api::VpnGatewayAggregatedListCall), [*delete*](https://docs.rs/google-compute1/5.0.5+20240604/google_compute1/api::VpnGatewayDeleteCall), [*get*](https://docs.rs/google-compute1/5.0.5+20240604/google_compute1/api::VpnGatewayGetCall), [*get status*](https://docs.rs/google-compute1/5.0.5+20240604/google_compute1/api::VpnGatewayGetStatuCall), [*insert*](https://docs.rs/google-compute1/5.0.5+20240604/google_compute1/api::VpnGatewayInsertCall), [*list*](https://docs.rs/google-compute1/5.0.5+20240604/google_compute1/api::VpnGatewayListCall), [*set labels*](https://docs.rs/google-compute1/5.0.5+20240604/google_compute1/api::VpnGatewaySetLabelCall) and [*test iam permissions*](https://docs.rs/google-compute1/5.0.5+20240604/google_compute1/api::VpnGatewayTestIamPermissionCall)
* [vpn tunnels](https://docs.rs/google-compute1/5.0.5+20240604/google_compute1/api::VpnTunnel)
 * [*aggregated list*](https://docs.rs/google-compute1/5.0.5+20240604/google_compute1/api::VpnTunnelAggregatedListCall), [*delete*](https://docs.rs/google-compute1/5.0.5+20240604/google_compute1/api::VpnTunnelDeleteCall), [*get*](https://docs.rs/google-compute1/5.0.5+20240604/google_compute1/api::VpnTunnelGetCall), [*insert*](https://docs.rs/google-compute1/5.0.5+20240604/google_compute1/api::VpnTunnelInsertCall), [*list*](https://docs.rs/google-compute1/5.0.5+20240604/google_compute1/api::VpnTunnelListCall) and [*set labels*](https://docs.rs/google-compute1/5.0.5+20240604/google_compute1/api::VpnTunnelSetLabelCall)
* zone operations
 * [*delete*](https://docs.rs/google-compute1/5.0.5+20240604/google_compute1/api::ZoneOperationDeleteCall), [*get*](https://docs.rs/google-compute1/5.0.5+20240604/google_compute1/api::ZoneOperationGetCall), [*list*](https://docs.rs/google-compute1/5.0.5+20240604/google_compute1/api::ZoneOperationListCall) and [*wait*](https://docs.rs/google-compute1/5.0.5+20240604/google_compute1/api::ZoneOperationWaitCall)
* [zones](https://docs.rs/google-compute1/5.0.5+20240604/google_compute1/api::Zone)
 * [*get*](https://docs.rs/google-compute1/5.0.5+20240604/google_compute1/api::ZoneGetCall) and [*list*](https://docs.rs/google-compute1/5.0.5+20240604/google_compute1/api::ZoneListCall)




# Structure of this Library

The API is structured into the following primary items:

* **[Hub](https://docs.rs/google-compute1/5.0.5+20240604/google_compute1/Compute)**
    * a central object to maintain state and allow accessing all *Activities*
    * creates [*Method Builders*](https://docs.rs/google-compute1/5.0.5+20240604/google_compute1/client::MethodsBuilder) which in turn
      allow access to individual [*Call Builders*](https://docs.rs/google-compute1/5.0.5+20240604/google_compute1/client::CallBuilder)
* **[Resources](https://docs.rs/google-compute1/5.0.5+20240604/google_compute1/client::Resource)**
    * primary types that you can apply *Activities* to
    * a collection of properties and *Parts*
    * **[Parts](https://docs.rs/google-compute1/5.0.5+20240604/google_compute1/client::Part)**
        * a collection of properties
        * never directly used in *Activities*
* **[Activities](https://docs.rs/google-compute1/5.0.5+20240604/google_compute1/client::CallBuilder)**
    * operations to apply to *Resources*

All *structures* are marked with applicable traits to further categorize them and ease browsing.

Generally speaking, you can invoke *Activities* like this:

```Rust,ignore
let r = hub.resource().activity(...).doit().await
```

Or specifically ...

```ignore
let r = hub.addresses().delete(...).doit().await
let r = hub.addresses().insert(...).doit().await
let r = hub.addresses().move_(...).doit().await
let r = hub.addresses().set_labels(...).doit().await
let r = hub.autoscalers().delete(...).doit().await
let r = hub.autoscalers().insert(...).doit().await
let r = hub.autoscalers().patch(...).doit().await
let r = hub.autoscalers().update(...).doit().await
let r = hub.backend_buckets().add_signed_url_key(...).doit().await
let r = hub.backend_buckets().delete(...).doit().await
let r = hub.backend_buckets().delete_signed_url_key(...).doit().await
let r = hub.backend_buckets().insert(...).doit().await
let r = hub.backend_buckets().patch(...).doit().await
let r = hub.backend_buckets().set_edge_security_policy(...).doit().await
let r = hub.backend_buckets().update(...).doit().await
let r = hub.backend_services().add_signed_url_key(...).doit().await
let r = hub.backend_services().delete(...).doit().await
let r = hub.backend_services().delete_signed_url_key(...).doit().await
let r = hub.backend_services().insert(...).doit().await
let r = hub.backend_services().patch(...).doit().await
let r = hub.backend_services().set_edge_security_policy(...).doit().await
let r = hub.backend_services().set_security_policy(...).doit().await
let r = hub.backend_services().update(...).doit().await
let r = hub.disks().add_resource_policies(...).doit().await
let r = hub.disks().bulk_insert(...).doit().await
let r = hub.disks().create_snapshot(...).doit().await
let r = hub.disks().delete(...).doit().await
let r = hub.disks().insert(...).doit().await
let r = hub.disks().remove_resource_policies(...).doit().await
let r = hub.disks().resize(...).doit().await
let r = hub.disks().set_labels(...).doit().await
let r = hub.disks().start_async_replication(...).doit().await
let r = hub.disks().stop_async_replication(...).doit().await
let r = hub.disks().stop_group_async_replication(...).doit().await
let r = hub.disks().update(...).doit().await
let r = hub.external_vpn_gateways().delete(...).doit().await
let r = hub.external_vpn_gateways().insert(...).doit().await
let r = hub.external_vpn_gateways().set_labels(...).doit().await
let r = hub.firewall_policies().add_association(...).doit().await
let r = hub.firewall_policies().add_rule(...).doit().await
let r = hub.firewall_policies().clone_rules(...).doit().await
let r = hub.firewall_policies().delete(...).doit().await
let r = hub.firewall_policies().insert(...).doit().await
let r = hub.firewall_policies().move_(...).doit().await
let r = hub.firewall_policies().patch(...).doit().await
let r = hub.firewall_policies().patch_rule(...).doit().await
let r = hub.firewall_policies().remove_association(...).doit().await
let r = hub.firewall_policies().remove_rule(...).doit().await
let r = hub.firewalls().delete(...).doit().await
let r = hub.firewalls().insert(...).doit().await
let r = hub.firewalls().patch(...).doit().await
let r = hub.firewalls().update(...).doit().await
let r = hub.forwarding_rules().delete(...).doit().await
let r = hub.forwarding_rules().insert(...).doit().await
let r = hub.forwarding_rules().patch(...).doit().await
let r = hub.forwarding_rules().set_labels(...).doit().await
let r = hub.forwarding_rules().set_target(...).doit().await
let r = hub.global_addresses().delete(...).doit().await
let r = hub.global_addresses().insert(...).doit().await
let r = hub.global_addresses().move_(...).doit().await
let r = hub.global_addresses().set_labels(...).doit().await
let r = hub.global_forwarding_rules().delete(...).doit().await
let r = hub.global_forwarding_rules().insert(...).doit().await
let r = hub.global_forwarding_rules().patch(...).doit().await
let r = hub.global_forwarding_rules().set_labels(...).doit().await
let r = hub.global_forwarding_rules().set_target(...).doit().await
let r = hub.global_network_endpoint_groups().attach_network_endpoints(...).doit().await
let r = hub.global_network_endpoint_groups().delete(...).doit().await
let r = hub.global_network_endpoint_groups().detach_network_endpoints(...).doit().await
let r = hub.global_network_endpoint_groups().insert(...).doit().await
let r = hub.global_operations().get(...).doit().await
let r = hub.global_operations().wait(...).doit().await
let r = hub.global_organization_operations().get(...).doit().await
let r = hub.global_public_delegated_prefixes().delete(...).doit().await
let r = hub.global_public_delegated_prefixes().insert(...).doit().await
let r = hub.global_public_delegated_prefixes().patch(...).doit().await
let r = hub.health_checks().delete(...).doit().await
let r = hub.health_checks().insert(...).doit().await
let r = hub.health_checks().patch(...).doit().await
let r = hub.health_checks().update(...).doit().await
let r = hub.http_health_checks().delete(...).doit().await
let r = hub.http_health_checks().insert(...).doit().await
let r = hub.http_health_checks().patch(...).doit().await
let r = hub.http_health_checks().update(...).doit().await
let r = hub.https_health_checks().delete(...).doit().await
let r = hub.https_health_checks().insert(...).doit().await
let r = hub.https_health_checks().patch(...).doit().await
let r = hub.https_health_checks().update(...).doit().await
let r = hub.images().delete(...).doit().await
let r = hub.images().deprecate(...).doit().await
let r = hub.images().insert(...).doit().await
let r = hub.images().patch(...).doit().await
let r = hub.images().set_labels(...).doit().await
let r = hub.instance_group_manager_resize_requests().cancel(...).doit().await
let r = hub.instance_group_manager_resize_requests().delete(...).doit().await
let r = hub.instance_group_manager_resize_requests().insert(...).doit().await
let r = hub.instance_group_managers().abandon_instances(...).doit().await
let r = hub.instance_group_managers().apply_updates_to_instances(...).doit().await
let r = hub.instance_group_managers().create_instances(...).doit().await
let r = hub.instance_group_managers().delete(...).doit().await
let r = hub.instance_group_managers().delete_instances(...).doit().await
let r = hub.instance_group_managers().delete_per_instance_configs(...).doit().await
let r = hub.instance_group_managers().insert(...).doit().await
let r = hub.instance_group_managers().patch(...).doit().await
let r = hub.instance_group_managers().patch_per_instance_configs(...).doit().await
let r = hub.instance_group_managers().recreate_instances(...).doit().await
let r = hub.instance_group_managers().resize(...).doit().await
let r = hub.instance_group_managers().set_instance_template(...).doit().await
let r = hub.instance_group_managers().set_target_pools(...).doit().await
let r = hub.instance_group_managers().update_per_instance_configs(...).doit().await
let r = hub.instance_groups().add_instances(...).doit().await
let r = hub.instance_groups().delete(...).doit().await
let r = hub.instance_groups().insert(...).doit().await
let r = hub.instance_groups().remove_instances(...).doit().await
let r = hub.instance_groups().set_named_ports(...).doit().await
let r = hub.instance_settings().patch(...).doit().await
let r = hub.instance_templates().delete(...).doit().await
let r = hub.instance_templates().insert(...).doit().await
let r = hub.instances().add_access_config(...).doit().await
let r = hub.instances().add_resource_policies(...).doit().await
let r = hub.instances().attach_disk(...).doit().await
let r = hub.instances().bulk_insert(...).doit().await
let r = hub.instances().delete(...).doit().await
let r = hub.instances().delete_access_config(...).doit().await
let r = hub.instances().detach_disk(...).doit().await
let r = hub.instances().insert(...).doit().await
let r = hub.instances().perform_maintenance(...).doit().await
let r = hub.instances().remove_resource_policies(...).doit().await
let r = hub.instances().reset(...).doit().await
let r = hub.instances().resume(...).doit().await
let r = hub.instances().set_deletion_protection(...).doit().await
let r = hub.instances().set_disk_auto_delete(...).doit().await
let r = hub.instances().set_labels(...).doit().await
let r = hub.instances().set_machine_resources(...).doit().await
let r = hub.instances().set_machine_type(...).doit().await
let r = hub.instances().set_metadata(...).doit().await
let r = hub.instances().set_min_cpu_platform(...).doit().await
let r = hub.instances().set_name(...).doit().await
let r = hub.instances().set_scheduling(...).doit().await
let r = hub.instances().set_security_policy(...).doit().await
let r = hub.instances().set_service_account(...).doit().await
let r = hub.instances().set_shielded_instance_integrity_policy(...).doit().await
let r = hub.instances().set_tags(...).doit().await
let r = hub.instances().simulate_maintenance_event(...).doit().await
let r = hub.instances().start(...).doit().await
let r = hub.instances().start_with_encryption_key(...).doit().await
let r = hub.instances().stop(...).doit().await
let r = hub.instances().suspend(...).doit().await
let r = hub.instances().update(...).doit().await
let r = hub.instances().update_access_config(...).doit().await
let r = hub.instances().update_display_device(...).doit().await
let r = hub.instances().update_network_interface(...).doit().await
let r = hub.instances().update_shielded_instance_config(...).doit().await
let r = hub.instant_snapshots().delete(...).doit().await
let r = hub.instant_snapshots().insert(...).doit().await
let r = hub.instant_snapshots().set_labels(...).doit().await
let r = hub.interconnect_attachments().delete(...).doit().await
let r = hub.interconnect_attachments().insert(...).doit().await
let r = hub.interconnect_attachments().patch(...).doit().await
let r = hub.interconnect_attachments().set_labels(...).doit().await
let r = hub.interconnects().delete(...).doit().await
let r = hub.interconnects().insert(...).doit().await
let r = hub.interconnects().patch(...).doit().await
let r = hub.interconnects().set_labels(...).doit().await
let r = hub.licenses().delete(...).doit().await
let r = hub.licenses().insert(...).doit().await
let r = hub.machine_images().delete(...).doit().await
let r = hub.machine_images().insert(...).doit().await
let r = hub.network_attachments().delete(...).doit().await
let r = hub.network_attachments().insert(...).doit().await
let r = hub.network_attachments().patch(...).doit().await
let r = hub.network_edge_security_services().delete(...).doit().await
let r = hub.network_edge_security_services().insert(...).doit().await
let r = hub.network_edge_security_services().patch(...).doit().await
let r = hub.network_endpoint_groups().attach_network_endpoints(...).doit().await
let r = hub.network_endpoint_groups().delete(...).doit().await
let r = hub.network_endpoint_groups().detach_network_endpoints(...).doit().await
let r = hub.network_endpoint_groups().insert(...).doit().await
let r = hub.network_firewall_policies().add_association(...).doit().await
let r = hub.network_firewall_policies().add_rule(...).doit().await
let r = hub.network_firewall_policies().clone_rules(...).doit().await
let r = hub.network_firewall_policies().delete(...).doit().await
let r = hub.network_firewall_policies().insert(...).doit().await
let r = hub.network_firewall_policies().patch(...).doit().await
let r = hub.network_firewall_policies().patch_rule(...).doit().await
let r = hub.network_firewall_policies().remove_association(...).doit().await
let r = hub.network_firewall_policies().remove_rule(...).doit().await
let r = hub.networks().add_peering(...).doit().await
let r = hub.networks().delete(...).doit().await
let r = hub.networks().insert(...).doit().await
let r = hub.networks().patch(...).doit().await
let r = hub.networks().remove_peering(...).doit().await
let r = hub.networks().switch_to_custom_mode(...).doit().await
let r = hub.networks().update_peering(...).doit().await
let r = hub.node_groups().add_nodes(...).doit().await
let r = hub.node_groups().delete(...).doit().await
let r = hub.node_groups().delete_nodes(...).doit().await
let r = hub.node_groups().insert(...).doit().await
let r = hub.node_groups().patch(...).doit().await
let r = hub.node_groups().perform_maintenance(...).doit().await
let r = hub.node_groups().set_node_template(...).doit().await
let r = hub.node_groups().simulate_maintenance_event(...).doit().await
let r = hub.node_templates().delete(...).doit().await
let r = hub.node_templates().insert(...).doit().await
let r = hub.packet_mirrorings().delete(...).doit().await
let r = hub.packet_mirrorings().insert(...).doit().await
let r = hub.packet_mirrorings().patch(...).doit().await
let r = hub.projects().disable_xpn_host(...).doit().await
let r = hub.projects().disable_xpn_resource(...).doit().await
let r = hub.projects().enable_xpn_host(...).doit().await
let r = hub.projects().enable_xpn_resource(...).doit().await
let r = hub.projects().move_disk(...).doit().await
let r = hub.projects().move_instance(...).doit().await
let r = hub.projects().set_cloud_armor_tier(...).doit().await
let r = hub.projects().set_common_instance_metadata(...).doit().await
let r = hub.projects().set_default_network_tier(...).doit().await
let r = hub.projects().set_usage_export_bucket(...).doit().await
let r = hub.public_advertised_prefixes().announce(...).doit().await
let r = hub.public_advertised_prefixes().delete(...).doit().await
let r = hub.public_advertised_prefixes().insert(...).doit().await
let r = hub.public_advertised_prefixes().patch(...).doit().await
let r = hub.public_advertised_prefixes().withdraw(...).doit().await
let r = hub.public_delegated_prefixes().announce(...).doit().await
let r = hub.public_delegated_prefixes().delete(...).doit().await
let r = hub.public_delegated_prefixes().insert(...).doit().await
let r = hub.public_delegated_prefixes().patch(...).doit().await
let r = hub.public_delegated_prefixes().withdraw(...).doit().await
let r = hub.region_autoscalers().delete(...).doit().await
let r = hub.region_autoscalers().insert(...).doit().await
let r = hub.region_autoscalers().patch(...).doit().await
let r = hub.region_autoscalers().update(...).doit().await
let r = hub.region_backend_services().delete(...).doit().await
let r = hub.region_backend_services().insert(...).doit().await
let r = hub.region_backend_services().patch(...).doit().await
let r = hub.region_backend_services().set_security_policy(...).doit().await
let r = hub.region_backend_services().update(...).doit().await
let r = hub.region_commitments().insert(...).doit().await
let r = hub.region_commitments().update(...).doit().await
let r = hub.region_disks().add_resource_policies(...).doit().await
let r = hub.region_disks().bulk_insert(...).doit().await
let r = hub.region_disks().create_snapshot(...).doit().await
let r = hub.region_disks().delete(...).doit().await
let r = hub.region_disks().insert(...).doit().await
let r = hub.region_disks().remove_resource_policies(...).doit().await
let r = hub.region_disks().resize(...).doit().await
let r = hub.region_disks().set_labels(...).doit().await
let r = hub.region_disks().start_async_replication(...).doit().await
let r = hub.region_disks().stop_async_replication(...).doit().await
let r = hub.region_disks().stop_group_async_replication(...).doit().await
let r = hub.region_disks().update(...).doit().await
let r = hub.region_health_check_services().delete(...).doit().await
let r = hub.region_health_check_services().insert(...).doit().await
let r = hub.region_health_check_services().patch(...).doit().await
let r = hub.region_health_checks().delete(...).doit().await
let r = hub.region_health_checks().insert(...).doit().await
let r = hub.region_health_checks().patch(...).doit().await
let r = hub.region_health_checks().update(...).doit().await
let r = hub.region_instance_group_managers().abandon_instances(...).doit().await
let r = hub.region_instance_group_managers().apply_updates_to_instances(...).doit().await
let r = hub.region_instance_group_managers().create_instances(...).doit().await
let r = hub.region_instance_group_managers().delete(...).doit().await
let r = hub.region_instance_group_managers().delete_instances(...).doit().await
let r = hub.region_instance_group_managers().delete_per_instance_configs(...).doit().await
let r = hub.region_instance_group_managers().insert(...).doit().await
let r = hub.region_instance_group_managers().patch(...).doit().await
let r = hub.region_instance_group_managers().patch_per_instance_configs(...).doit().await
let r = hub.region_instance_group_managers().recreate_instances(...).doit().await
let r = hub.region_instance_group_managers().resize(...).doit().await
let r = hub.region_instance_group_managers().set_instance_template(...).doit().await
let r = hub.region_instance_group_managers().set_target_pools(...).doit().await
let r = hub.region_instance_group_managers().update_per_instance_configs(...).doit().await
let r = hub.region_instance_groups().set_named_ports(...).doit().await
let r = hub.region_instance_templates().delete(...).doit().await
let r = hub.region_instance_templates().insert(...).doit().await
let r = hub.region_instances().bulk_insert(...).doit().await
let r = hub.region_instant_snapshots().delete(...).doit().await
let r = hub.region_instant_snapshots().insert(...).doit().await
let r = hub.region_instant_snapshots().set_labels(...).doit().await
let r = hub.region_network_endpoint_groups().attach_network_endpoints(...).doit().await
let r = hub.region_network_endpoint_groups().delete(...).doit().await
let r = hub.region_network_endpoint_groups().detach_network_endpoints(...).doit().await
let r = hub.region_network_endpoint_groups().insert(...).doit().await
let r = hub.region_network_firewall_policies().add_association(...).doit().await
let r = hub.region_network_firewall_policies().add_rule(...).doit().await
let r = hub.region_network_firewall_policies().clone_rules(...).doit().await
let r = hub.region_network_firewall_policies().delete(...).doit().await
let r = hub.region_network_firewall_policies().insert(...).doit().await
let r = hub.region_network_firewall_policies().patch(...).doit().await
let r = hub.region_network_firewall_policies().patch_rule(...).doit().await
let r = hub.region_network_firewall_policies().remove_association(...).doit().await
let r = hub.region_network_firewall_policies().remove_rule(...).doit().await
let r = hub.region_notification_endpoints().delete(...).doit().await
let r = hub.region_notification_endpoints().insert(...).doit().await
let r = hub.region_operations().get(...).doit().await
let r = hub.region_operations().wait(...).doit().await
let r = hub.region_security_policies().add_rule(...).doit().await
let r = hub.region_security_policies().delete(...).doit().await
let r = hub.region_security_policies().insert(...).doit().await
let r = hub.region_security_policies().patch(...).doit().await
let r = hub.region_security_policies().patch_rule(...).doit().await
let r = hub.region_security_policies().remove_rule(...).doit().await
let r = hub.region_ssl_certificates().delete(...).doit().await
let r = hub.region_ssl_certificates().insert(...).doit().await
let r = hub.region_ssl_policies().delete(...).doit().await
let r = hub.region_ssl_policies().insert(...).doit().await
let r = hub.region_ssl_policies().patch(...).doit().await
let r = hub.region_target_http_proxies().delete(...).doit().await
let r = hub.region_target_http_proxies().insert(...).doit().await
let r = hub.region_target_http_proxies().set_url_map(...).doit().await
let r = hub.region_target_https_proxies().delete(...).doit().await
let r = hub.region_target_https_proxies().insert(...).doit().await
let r = hub.region_target_https_proxies().patch(...).doit().await
let r = hub.region_target_https_proxies().set_ssl_certificates(...).doit().await
let r = hub.region_target_https_proxies().set_url_map(...).doit().await
let r = hub.region_target_tcp_proxies().delete(...).doit().await
let r = hub.region_target_tcp_proxies().insert(...).doit().await
let r = hub.region_url_maps().delete(...).doit().await
let r = hub.region_url_maps().insert(...).doit().await
let r = hub.region_url_maps().patch(...).doit().await
let r = hub.region_url_maps().update(...).doit().await
let r = hub.reservations().delete(...).doit().await
let r = hub.reservations().insert(...).doit().await
let r = hub.reservations().resize(...).doit().await
let r = hub.reservations().update(...).doit().await
let r = hub.resource_policies().delete(...).doit().await
let r = hub.resource_policies().insert(...).doit().await
let r = hub.resource_policies().patch(...).doit().await
let r = hub.routers().delete(...).doit().await
let r = hub.routers().insert(...).doit().await
let r = hub.routers().patch(...).doit().await
let r = hub.routers().update(...).doit().await
let r = hub.routes().delete(...).doit().await
let r = hub.routes().insert(...).doit().await
let r = hub.security_policies().add_rule(...).doit().await
let r = hub.security_policies().delete(...).doit().await
let r = hub.security_policies().insert(...).doit().await
let r = hub.security_policies().patch(...).doit().await
let r = hub.security_policies().patch_rule(...).doit().await
let r = hub.security_policies().remove_rule(...).doit().await
let r = hub.security_policies().set_labels(...).doit().await
let r = hub.service_attachments().delete(...).doit().await
let r = hub.service_attachments().insert(...).doit().await
let r = hub.service_attachments().patch(...).doit().await
let r = hub.snapshot_settings().patch(...).doit().await
let r = hub.snapshots().delete(...).doit().await
let r = hub.snapshots().insert(...).doit().await
let r = hub.snapshots().set_labels(...).doit().await
let r = hub.ssl_certificates().delete(...).doit().await
let r = hub.ssl_certificates().insert(...).doit().await
let r = hub.ssl_policies().delete(...).doit().await
let r = hub.ssl_policies().insert(...).doit().await
let r = hub.ssl_policies().patch(...).doit().await
let r = hub.storage_pools().delete(...).doit().await
let r = hub.storage_pools().insert(...).doit().await
let r = hub.storage_pools().update(...).doit().await
let r = hub.subnetworks().delete(...).doit().await
let r = hub.subnetworks().expand_ip_cidr_range(...).doit().await
let r = hub.subnetworks().insert(...).doit().await
let r = hub.subnetworks().patch(...).doit().await
let r = hub.subnetworks().set_private_ip_google_access(...).doit().await
let r = hub.target_grpc_proxies().delete(...).doit().await
let r = hub.target_grpc_proxies().insert(...).doit().await
let r = hub.target_grpc_proxies().patch(...).doit().await
let r = hub.target_http_proxies().delete(...).doit().await
let r = hub.target_http_proxies().insert(...).doit().await
let r = hub.target_http_proxies().patch(...).doit().await
let r = hub.target_http_proxies().set_url_map(...).doit().await
let r = hub.target_https_proxies().delete(...).doit().await
let r = hub.target_https_proxies().insert(...).doit().await
let r = hub.target_https_proxies().patch(...).doit().await
let r = hub.target_https_proxies().set_certificate_map(...).doit().await
let r = hub.target_https_proxies().set_quic_override(...).doit().await
let r = hub.target_https_proxies().set_ssl_certificates(...).doit().await
let r = hub.target_https_proxies().set_ssl_policy(...).doit().await
let r = hub.target_https_proxies().set_url_map(...).doit().await
let r = hub.target_instances().delete(...).doit().await
let r = hub.target_instances().insert(...).doit().await
let r = hub.target_instances().set_security_policy(...).doit().await
let r = hub.target_pools().add_health_check(...).doit().await
let r = hub.target_pools().add_instance(...).doit().await
let r = hub.target_pools().delete(...).doit().await
let r = hub.target_pools().insert(...).doit().await
let r = hub.target_pools().remove_health_check(...).doit().await
let r = hub.target_pools().remove_instance(...).doit().await
let r = hub.target_pools().set_backup(...).doit().await
let r = hub.target_pools().set_security_policy(...).doit().await
let r = hub.target_ssl_proxies().delete(...).doit().await
let r = hub.target_ssl_proxies().insert(...).doit().await
let r = hub.target_ssl_proxies().set_backend_service(...).doit().await
let r = hub.target_ssl_proxies().set_certificate_map(...).doit().await
let r = hub.target_ssl_proxies().set_proxy_header(...).doit().await
let r = hub.target_ssl_proxies().set_ssl_certificates(...).doit().await
let r = hub.target_ssl_proxies().set_ssl_policy(...).doit().await
let r = hub.target_tcp_proxies().delete(...).doit().await
let r = hub.target_tcp_proxies().insert(...).doit().await
let r = hub.target_tcp_proxies().set_backend_service(...).doit().await
let r = hub.target_tcp_proxies().set_proxy_header(...).doit().await
let r = hub.target_vpn_gateways().delete(...).doit().await
let r = hub.target_vpn_gateways().insert(...).doit().await
let r = hub.target_vpn_gateways().set_labels(...).doit().await
let r = hub.url_maps().delete(...).doit().await
let r = hub.url_maps().insert(...).doit().await
let r = hub.url_maps().invalidate_cache(...).doit().await
let r = hub.url_maps().patch(...).doit().await
let r = hub.url_maps().update(...).doit().await
let r = hub.vpn_gateways().delete(...).doit().await
let r = hub.vpn_gateways().insert(...).doit().await
let r = hub.vpn_gateways().set_labels(...).doit().await
let r = hub.vpn_tunnels().delete(...).doit().await
let r = hub.vpn_tunnels().insert(...).doit().await
let r = hub.vpn_tunnels().set_labels(...).doit().await
let r = hub.zone_operations().get(...).doit().await
let r = hub.zone_operations().wait(...).doit().await
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
serde = "^1.0"
serde_json = "^1.0"
```

## A complete example

```Rust
extern crate hyper;
extern crate hyper_rustls;
extern crate google_compute1 as compute1;
use compute1::api::Disk;
use compute1::{Result, Error};
use std::default::Default;
use compute1::{Compute, oauth2, hyper, hyper_rustls, chrono, FieldMask};

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
let mut hub = Compute::new(hyper::Client::builder().build(hyper_rustls::HttpsConnectorBuilder::new().with_native_roots().unwrap().https_or_http().enable_http1().build()), auth);
// As the method needs a request, you would usually fill it with the desired information
// into the respective structure. Some of the parts shown here might not be applicable !
// Values shown here are possibly random and not representative !
let mut req = Disk::default();

// You can configure optional parameters by calling the respective setters at will, and
// execute the final call using `doit()`.
// Values shown here are possibly random and not representative !
let result = hub.disks().update(req, "project", "zone", "disk")
             .update_mask(FieldMask::new::<&str>(&[]))
             .request_id("ipsum")
             .add_paths("voluptua.")
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

All errors produced by the system are provided either as [Result](https://docs.rs/google-compute1/5.0.5+20240604/google_compute1/client::Result) enumeration as return value of
the doit() methods, or handed as possibly intermediate results to either the 
[Hub Delegate](https://docs.rs/google-compute1/5.0.5+20240604/google_compute1/client::Delegate), or the [Authenticator Delegate](https://docs.rs/yup-oauth2/*/yup_oauth2/trait.AuthenticatorDelegate.html).

When delegates handle errors or intermediate values, they may have a chance to instruct the system to retry. This 
makes the system potentially resilient to all kinds of errors.

## Uploads and Downloads
If a method supports downloads, the response body, which is part of the [Result](https://docs.rs/google-compute1/5.0.5+20240604/google_compute1/client::Result), should be
read by you to obtain the media.
If such a method also supports a [Response Result](https://docs.rs/google-compute1/5.0.5+20240604/google_compute1/client::ResponseResult), it will return that by default.
You can see it as meta-data for the actual media. To trigger a media download, you will have to set up the builder by making
this call: `.param("alt", "media")`.

Methods supporting uploads can do so using up to 2 different protocols: 
*simple* and *resumable*. The distinctiveness of each is represented by customized 
`doit(...)` methods, which are then named `upload(...)` and `upload_resumable(...)` respectively.

## Customization and Callbacks

You may alter the way an `doit()` method is called by providing a [delegate](https://docs.rs/google-compute1/5.0.5+20240604/google_compute1/client::Delegate) to the 
[Method Builder](https://docs.rs/google-compute1/5.0.5+20240604/google_compute1/client::CallBuilder) before making the final `doit()` call. 
Respective methods will be called to provide progress information, as well as determine whether the system should 
retry on failure.

The [delegate trait](https://docs.rs/google-compute1/5.0.5+20240604/google_compute1/client::Delegate) is default-implemented, allowing you to customize it with minimal effort.

## Optional Parts in Server-Requests

All structures provided by this library are made to be [encodable](https://docs.rs/google-compute1/5.0.5+20240604/google_compute1/client::RequestValue) and 
[decodable](https://docs.rs/google-compute1/5.0.5+20240604/google_compute1/client::ResponseResult) via *json*. Optionals are used to indicate that partial requests are responses 
are valid.
Most optionals are are considered [Parts](https://docs.rs/google-compute1/5.0.5+20240604/google_compute1/client::Part) which are identifiable by name, which will be sent to 
the server to indicate either the set parts of the request or the desired parts in the response.

## Builder Arguments

Using [method builders](https://docs.rs/google-compute1/5.0.5+20240604/google_compute1/client::CallBuilder), you are able to prepare an action call by repeatedly calling it's methods.
These will always take a single argument, for which the following statements are true.

* [PODs][wiki-pod] are handed by copy
* strings are passed as `&str`
* [request values](https://docs.rs/google-compute1/5.0.5+20240604/google_compute1/client::RequestValue) are moved

Arguments will always be copied or cloned into the builder, to make them independent of their original life times.

[wiki-pod]: http://en.wikipedia.org/wiki/Plain_old_data_structure
[builder-pattern]: http://en.wikipedia.org/wiki/Builder_pattern
[google-go-api]: https://github.com/google/google-api-go-client

## Cargo Features

* `utoipa` - Add support for [utoipa](https://crates.io/crates/utoipa) and derive `utoipa::ToSchema` on all
the types. You'll have to import and register the required types in `#[openapi(schemas(...))]`, otherwise the
generated `openapi` spec would be invalid.


# License
The **compute1** library was generated by Sebastian Thiel, and is placed 
under the *MIT* license.
You can read the full text at the repository's [license file][repo-license].

[repo-license]: https://github.com/Byron/google-apis-rsblob/main/LICENSE.md

