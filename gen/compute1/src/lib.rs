// DO NOT EDIT !
// This file was generated automatically from 'src/generator/templates/api/lib.rs.mako'
// DO NOT EDIT !

//! This documentation was generated from *compute* crate version *5.0.5+20240604*, where *20240604* is the exact revision of the *compute:v1* schema built by the [mako](http://www.makotemplates.org/) code generator *v5.0.5*.
//! 
//! Everything else about the *compute* *v1* API can be found at the
//! [official documentation site](https://cloud.google.com/compute/).
//! The original source code is [on github](https://github.com/Byron/google-apis-rs/tree/main/gen/compute1).
//! # Features
//! 
//! Handle the following *Resources* with ease from the central [hub](Compute) ... 
//! 
//! * [accelerator types](api::AcceleratorType)
//!  * [*aggregated list*](api::AcceleratorTypeAggregatedListCall), [*get*](api::AcceleratorTypeGetCall) and [*list*](api::AcceleratorTypeListCall)
//! * [addresses](api::Address)
//!  * [*aggregated list*](api::AddressAggregatedListCall), [*delete*](api::AddressDeleteCall), [*get*](api::AddressGetCall), [*insert*](api::AddressInsertCall), [*list*](api::AddressListCall), [*move*](api::AddressMoveCall) and [*set labels*](api::AddressSetLabelCall)
//! * [autoscalers](api::Autoscaler)
//!  * [*aggregated list*](api::AutoscalerAggregatedListCall), [*delete*](api::AutoscalerDeleteCall), [*get*](api::AutoscalerGetCall), [*insert*](api::AutoscalerInsertCall), [*list*](api::AutoscalerListCall), [*patch*](api::AutoscalerPatchCall) and [*update*](api::AutoscalerUpdateCall)
//! * [backend buckets](api::BackendBucket)
//!  * [*add signed url key*](api::BackendBucketAddSignedUrlKeyCall), [*delete*](api::BackendBucketDeleteCall), [*delete signed url key*](api::BackendBucketDeleteSignedUrlKeyCall), [*get*](api::BackendBucketGetCall), [*get iam policy*](api::BackendBucketGetIamPolicyCall), [*insert*](api::BackendBucketInsertCall), [*list*](api::BackendBucketListCall), [*patch*](api::BackendBucketPatchCall), [*set edge security policy*](api::BackendBucketSetEdgeSecurityPolicyCall), [*set iam policy*](api::BackendBucketSetIamPolicyCall), [*test iam permissions*](api::BackendBucketTestIamPermissionCall) and [*update*](api::BackendBucketUpdateCall)
//! * [backend services](api::BackendService)
//!  * [*add signed url key*](api::BackendServiceAddSignedUrlKeyCall), [*aggregated list*](api::BackendServiceAggregatedListCall), [*delete*](api::BackendServiceDeleteCall), [*delete signed url key*](api::BackendServiceDeleteSignedUrlKeyCall), [*get*](api::BackendServiceGetCall), [*get health*](api::BackendServiceGetHealthCall), [*get iam policy*](api::BackendServiceGetIamPolicyCall), [*insert*](api::BackendServiceInsertCall), [*list*](api::BackendServiceListCall), [*list usable*](api::BackendServiceListUsableCall), [*patch*](api::BackendServicePatchCall), [*set edge security policy*](api::BackendServiceSetEdgeSecurityPolicyCall), [*set iam policy*](api::BackendServiceSetIamPolicyCall), [*set security policy*](api::BackendServiceSetSecurityPolicyCall), [*test iam permissions*](api::BackendServiceTestIamPermissionCall) and [*update*](api::BackendServiceUpdateCall)
//! * [disk types](api::DiskType)
//!  * [*aggregated list*](api::DiskTypeAggregatedListCall), [*get*](api::DiskTypeGetCall) and [*list*](api::DiskTypeListCall)
//! * [disks](api::Disk)
//!  * [*add resource policies*](api::DiskAddResourcePolicyCall), [*aggregated list*](api::DiskAggregatedListCall), [*bulk insert*](api::DiskBulkInsertCall), [*create snapshot*](api::DiskCreateSnapshotCall), [*delete*](api::DiskDeleteCall), [*get*](api::DiskGetCall), [*get iam policy*](api::DiskGetIamPolicyCall), [*insert*](api::DiskInsertCall), [*list*](api::DiskListCall), [*remove resource policies*](api::DiskRemoveResourcePolicyCall), [*resize*](api::DiskResizeCall), [*set iam policy*](api::DiskSetIamPolicyCall), [*set labels*](api::DiskSetLabelCall), [*start async replication*](api::DiskStartAsyncReplicationCall), [*stop async replication*](api::DiskStopAsyncReplicationCall), [*stop group async replication*](api::DiskStopGroupAsyncReplicationCall), [*test iam permissions*](api::DiskTestIamPermissionCall) and [*update*](api::DiskUpdateCall)
//! * [external vpn gateways](api::ExternalVpnGateway)
//!  * [*delete*](api::ExternalVpnGatewayDeleteCall), [*get*](api::ExternalVpnGatewayGetCall), [*insert*](api::ExternalVpnGatewayInsertCall), [*list*](api::ExternalVpnGatewayListCall), [*set labels*](api::ExternalVpnGatewaySetLabelCall) and [*test iam permissions*](api::ExternalVpnGatewayTestIamPermissionCall)
//! * [firewall policies](api::FirewallPolicy)
//!  * [*add association*](api::FirewallPolicyAddAssociationCall), [*add rule*](api::FirewallPolicyAddRuleCall), [*clone rules*](api::FirewallPolicyCloneRuleCall), [*delete*](api::FirewallPolicyDeleteCall), [*get*](api::FirewallPolicyGetCall), [*get association*](api::FirewallPolicyGetAssociationCall), [*get iam policy*](api::FirewallPolicyGetIamPolicyCall), [*get rule*](api::FirewallPolicyGetRuleCall), [*insert*](api::FirewallPolicyInsertCall), [*list*](api::FirewallPolicyListCall), [*list associations*](api::FirewallPolicyListAssociationCall), [*move*](api::FirewallPolicyMoveCall), [*patch*](api::FirewallPolicyPatchCall), [*patch rule*](api::FirewallPolicyPatchRuleCall), [*remove association*](api::FirewallPolicyRemoveAssociationCall), [*remove rule*](api::FirewallPolicyRemoveRuleCall), [*set iam policy*](api::FirewallPolicySetIamPolicyCall) and [*test iam permissions*](api::FirewallPolicyTestIamPermissionCall)
//! * [firewalls](api::Firewall)
//!  * [*delete*](api::FirewallDeleteCall), [*get*](api::FirewallGetCall), [*insert*](api::FirewallInsertCall), [*list*](api::FirewallListCall), [*patch*](api::FirewallPatchCall) and [*update*](api::FirewallUpdateCall)
//! * [forwarding rules](api::ForwardingRule)
//!  * [*aggregated list*](api::ForwardingRuleAggregatedListCall), [*delete*](api::ForwardingRuleDeleteCall), [*get*](api::ForwardingRuleGetCall), [*insert*](api::ForwardingRuleInsertCall), [*list*](api::ForwardingRuleListCall), [*patch*](api::ForwardingRulePatchCall), [*set labels*](api::ForwardingRuleSetLabelCall) and [*set target*](api::ForwardingRuleSetTargetCall)
//! * global addresses
//!  * [*delete*](api::GlobalAddressDeleteCall), [*get*](api::GlobalAddressGetCall), [*insert*](api::GlobalAddressInsertCall), [*list*](api::GlobalAddressListCall), [*move*](api::GlobalAddressMoveCall) and [*set labels*](api::GlobalAddressSetLabelCall)
//! * global forwarding rules
//!  * [*delete*](api::GlobalForwardingRuleDeleteCall), [*get*](api::GlobalForwardingRuleGetCall), [*insert*](api::GlobalForwardingRuleInsertCall), [*list*](api::GlobalForwardingRuleListCall), [*patch*](api::GlobalForwardingRulePatchCall), [*set labels*](api::GlobalForwardingRuleSetLabelCall) and [*set target*](api::GlobalForwardingRuleSetTargetCall)
//! * global network endpoint groups
//!  * [*attach network endpoints*](api::GlobalNetworkEndpointGroupAttachNetworkEndpointCall), [*delete*](api::GlobalNetworkEndpointGroupDeleteCall), [*detach network endpoints*](api::GlobalNetworkEndpointGroupDetachNetworkEndpointCall), [*get*](api::GlobalNetworkEndpointGroupGetCall), [*insert*](api::GlobalNetworkEndpointGroupInsertCall), [*list*](api::GlobalNetworkEndpointGroupListCall) and [*list network endpoints*](api::GlobalNetworkEndpointGroupListNetworkEndpointCall)
//! * global operations
//!  * [*aggregated list*](api::GlobalOperationAggregatedListCall), [*delete*](api::GlobalOperationDeleteCall), [*get*](api::GlobalOperationGetCall), [*list*](api::GlobalOperationListCall) and [*wait*](api::GlobalOperationWaitCall)
//! * global organization operations
//!  * [*delete*](api::GlobalOrganizationOperationDeleteCall), [*get*](api::GlobalOrganizationOperationGetCall) and [*list*](api::GlobalOrganizationOperationListCall)
//! * global public delegated prefixes
//!  * [*delete*](api::GlobalPublicDelegatedPrefixDeleteCall), [*get*](api::GlobalPublicDelegatedPrefixGetCall), [*insert*](api::GlobalPublicDelegatedPrefixInsertCall), [*list*](api::GlobalPublicDelegatedPrefixListCall) and [*patch*](api::GlobalPublicDelegatedPrefixPatchCall)
//! * [health checks](api::HealthCheck)
//!  * [*aggregated list*](api::HealthCheckAggregatedListCall), [*delete*](api::HealthCheckDeleteCall), [*get*](api::HealthCheckGetCall), [*insert*](api::HealthCheckInsertCall), [*list*](api::HealthCheckListCall), [*patch*](api::HealthCheckPatchCall) and [*update*](api::HealthCheckUpdateCall)
//! * [http health checks](api::HttpHealthCheck)
//!  * [*delete*](api::HttpHealthCheckDeleteCall), [*get*](api::HttpHealthCheckGetCall), [*insert*](api::HttpHealthCheckInsertCall), [*list*](api::HttpHealthCheckListCall), [*patch*](api::HttpHealthCheckPatchCall) and [*update*](api::HttpHealthCheckUpdateCall)
//! * [https health checks](api::HttpsHealthCheck)
//!  * [*delete*](api::HttpsHealthCheckDeleteCall), [*get*](api::HttpsHealthCheckGetCall), [*insert*](api::HttpsHealthCheckInsertCall), [*list*](api::HttpsHealthCheckListCall), [*patch*](api::HttpsHealthCheckPatchCall) and [*update*](api::HttpsHealthCheckUpdateCall)
//! * [image family views](api::ImageFamilyView)
//!  * [*get*](api::ImageFamilyViewGetCall)
//! * [images](api::Image)
//!  * [*delete*](api::ImageDeleteCall), [*deprecate*](api::ImageDeprecateCall), [*get*](api::ImageGetCall), [*get from family*](api::ImageGetFromFamilyCall), [*get iam policy*](api::ImageGetIamPolicyCall), [*insert*](api::ImageInsertCall), [*list*](api::ImageListCall), [*patch*](api::ImagePatchCall), [*set iam policy*](api::ImageSetIamPolicyCall), [*set labels*](api::ImageSetLabelCall) and [*test iam permissions*](api::ImageTestIamPermissionCall)
//! * [instance group manager resize requests](api::InstanceGroupManagerResizeRequest)
//!  * [*cancel*](api::InstanceGroupManagerResizeRequestCancelCall), [*delete*](api::InstanceGroupManagerResizeRequestDeleteCall), [*get*](api::InstanceGroupManagerResizeRequestGetCall), [*insert*](api::InstanceGroupManagerResizeRequestInsertCall) and [*list*](api::InstanceGroupManagerResizeRequestListCall)
//! * [instance group managers](api::InstanceGroupManager)
//!  * [*abandon instances*](api::InstanceGroupManagerAbandonInstanceCall), [*aggregated list*](api::InstanceGroupManagerAggregatedListCall), [*apply updates to instances*](api::InstanceGroupManagerApplyUpdatesToInstanceCall), [*create instances*](api::InstanceGroupManagerCreateInstanceCall), [*delete*](api::InstanceGroupManagerDeleteCall), [*delete instances*](api::InstanceGroupManagerDeleteInstanceCall), [*delete per instance configs*](api::InstanceGroupManagerDeletePerInstanceConfigCall), [*get*](api::InstanceGroupManagerGetCall), [*insert*](api::InstanceGroupManagerInsertCall), [*list*](api::InstanceGroupManagerListCall), [*list errors*](api::InstanceGroupManagerListErrorCall), [*list managed instances*](api::InstanceGroupManagerListManagedInstanceCall), [*list per instance configs*](api::InstanceGroupManagerListPerInstanceConfigCall), [*patch*](api::InstanceGroupManagerPatchCall), [*patch per instance configs*](api::InstanceGroupManagerPatchPerInstanceConfigCall), [*recreate instances*](api::InstanceGroupManagerRecreateInstanceCall), [*resize*](api::InstanceGroupManagerResizeCall), [*set instance template*](api::InstanceGroupManagerSetInstanceTemplateCall), [*set target pools*](api::InstanceGroupManagerSetTargetPoolCall) and [*update per instance configs*](api::InstanceGroupManagerUpdatePerInstanceConfigCall)
//! * [instance groups](api::InstanceGroup)
//!  * [*add instances*](api::InstanceGroupAddInstanceCall), [*aggregated list*](api::InstanceGroupAggregatedListCall), [*delete*](api::InstanceGroupDeleteCall), [*get*](api::InstanceGroupGetCall), [*insert*](api::InstanceGroupInsertCall), [*list*](api::InstanceGroupListCall), [*list instances*](api::InstanceGroupListInstanceCall), [*remove instances*](api::InstanceGroupRemoveInstanceCall) and [*set named ports*](api::InstanceGroupSetNamedPortCall)
//! * instance settings
//!  * [*get*](api::InstanceSettingGetCall) and [*patch*](api::InstanceSettingPatchCall)
//! * [instance templates](api::InstanceTemplate)
//!  * [*aggregated list*](api::InstanceTemplateAggregatedListCall), [*delete*](api::InstanceTemplateDeleteCall), [*get*](api::InstanceTemplateGetCall), [*get iam policy*](api::InstanceTemplateGetIamPolicyCall), [*insert*](api::InstanceTemplateInsertCall), [*list*](api::InstanceTemplateListCall), [*set iam policy*](api::InstanceTemplateSetIamPolicyCall) and [*test iam permissions*](api::InstanceTemplateTestIamPermissionCall)
//! * [instances](api::Instance)
//!  * [*add access config*](api::InstanceAddAccessConfigCall), [*add resource policies*](api::InstanceAddResourcePolicyCall), [*aggregated list*](api::InstanceAggregatedListCall), [*attach disk*](api::InstanceAttachDiskCall), [*bulk insert*](api::InstanceBulkInsertCall), [*delete*](api::InstanceDeleteCall), [*delete access config*](api::InstanceDeleteAccessConfigCall), [*detach disk*](api::InstanceDetachDiskCall), [*get*](api::InstanceGetCall), [*get effective firewalls*](api::InstanceGetEffectiveFirewallCall), [*get guest attributes*](api::InstanceGetGuestAttributeCall), [*get iam policy*](api::InstanceGetIamPolicyCall), [*get screenshot*](api::InstanceGetScreenshotCall), [*get serial port output*](api::InstanceGetSerialPortOutputCall), [*get shielded instance identity*](api::InstanceGetShieldedInstanceIdentityCall), [*insert*](api::InstanceInsertCall), [*list*](api::InstanceListCall), [*list referrers*](api::InstanceListReferrerCall), [*perform maintenance*](api::InstancePerformMaintenanceCall), [*remove resource policies*](api::InstanceRemoveResourcePolicyCall), [*reset*](api::InstanceResetCall), [*resume*](api::InstanceResumeCall), [*send diagnostic interrupt*](api::InstanceSendDiagnosticInterruptCall), [*set deletion protection*](api::InstanceSetDeletionProtectionCall), [*set disk auto delete*](api::InstanceSetDiskAutoDeleteCall), [*set iam policy*](api::InstanceSetIamPolicyCall), [*set labels*](api::InstanceSetLabelCall), [*set machine resources*](api::InstanceSetMachineResourceCall), [*set machine type*](api::InstanceSetMachineTypeCall), [*set metadata*](api::InstanceSetMetadataCall), [*set min cpu platform*](api::InstanceSetMinCpuPlatformCall), [*set name*](api::InstanceSetNameCall), [*set scheduling*](api::InstanceSetSchedulingCall), [*set security policy*](api::InstanceSetSecurityPolicyCall), [*set service account*](api::InstanceSetServiceAccountCall), [*set shielded instance integrity policy*](api::InstanceSetShieldedInstanceIntegrityPolicyCall), [*set tags*](api::InstanceSetTagCall), [*simulate maintenance event*](api::InstanceSimulateMaintenanceEventCall), [*start*](api::InstanceStartCall), [*start with encryption key*](api::InstanceStartWithEncryptionKeyCall), [*stop*](api::InstanceStopCall), [*suspend*](api::InstanceSuspendCall), [*test iam permissions*](api::InstanceTestIamPermissionCall), [*update*](api::InstanceUpdateCall), [*update access config*](api::InstanceUpdateAccessConfigCall), [*update display device*](api::InstanceUpdateDisplayDeviceCall), [*update network interface*](api::InstanceUpdateNetworkInterfaceCall) and [*update shielded instance config*](api::InstanceUpdateShieldedInstanceConfigCall)
//! * [instant snapshots](api::InstantSnapshot)
//!  * [*aggregated list*](api::InstantSnapshotAggregatedListCall), [*delete*](api::InstantSnapshotDeleteCall), [*get*](api::InstantSnapshotGetCall), [*get iam policy*](api::InstantSnapshotGetIamPolicyCall), [*insert*](api::InstantSnapshotInsertCall), [*list*](api::InstantSnapshotListCall), [*set iam policy*](api::InstantSnapshotSetIamPolicyCall), [*set labels*](api::InstantSnapshotSetLabelCall) and [*test iam permissions*](api::InstantSnapshotTestIamPermissionCall)
//! * [interconnect attachments](api::InterconnectAttachment)
//!  * [*aggregated list*](api::InterconnectAttachmentAggregatedListCall), [*delete*](api::InterconnectAttachmentDeleteCall), [*get*](api::InterconnectAttachmentGetCall), [*insert*](api::InterconnectAttachmentInsertCall), [*list*](api::InterconnectAttachmentListCall), [*patch*](api::InterconnectAttachmentPatchCall) and [*set labels*](api::InterconnectAttachmentSetLabelCall)
//! * [interconnect locations](api::InterconnectLocation)
//!  * [*get*](api::InterconnectLocationGetCall) and [*list*](api::InterconnectLocationListCall)
//! * [interconnect remote locations](api::InterconnectRemoteLocation)
//!  * [*get*](api::InterconnectRemoteLocationGetCall) and [*list*](api::InterconnectRemoteLocationListCall)
//! * [interconnects](api::Interconnect)
//!  * [*delete*](api::InterconnectDeleteCall), [*get*](api::InterconnectGetCall), [*get diagnostics*](api::InterconnectGetDiagnosticCall), [*get macsec config*](api::InterconnectGetMacsecConfigCall), [*insert*](api::InterconnectInsertCall), [*list*](api::InterconnectListCall), [*patch*](api::InterconnectPatchCall) and [*set labels*](api::InterconnectSetLabelCall)
//! * [license codes](api::LicenseCode)
//!  * [*get*](api::LicenseCodeGetCall) and [*test iam permissions*](api::LicenseCodeTestIamPermissionCall)
//! * [licenses](api::License)
//!  * [*delete*](api::LicenseDeleteCall), [*get*](api::LicenseGetCall), [*get iam policy*](api::LicenseGetIamPolicyCall), [*insert*](api::LicenseInsertCall), [*list*](api::LicenseListCall), [*set iam policy*](api::LicenseSetIamPolicyCall) and [*test iam permissions*](api::LicenseTestIamPermissionCall)
//! * [machine images](api::MachineImage)
//!  * [*delete*](api::MachineImageDeleteCall), [*get*](api::MachineImageGetCall), [*get iam policy*](api::MachineImageGetIamPolicyCall), [*insert*](api::MachineImageInsertCall), [*list*](api::MachineImageListCall), [*set iam policy*](api::MachineImageSetIamPolicyCall) and [*test iam permissions*](api::MachineImageTestIamPermissionCall)
//! * [machine types](api::MachineType)
//!  * [*aggregated list*](api::MachineTypeAggregatedListCall), [*get*](api::MachineTypeGetCall) and [*list*](api::MachineTypeListCall)
//! * [network attachments](api::NetworkAttachment)
//!  * [*aggregated list*](api::NetworkAttachmentAggregatedListCall), [*delete*](api::NetworkAttachmentDeleteCall), [*get*](api::NetworkAttachmentGetCall), [*get iam policy*](api::NetworkAttachmentGetIamPolicyCall), [*insert*](api::NetworkAttachmentInsertCall), [*list*](api::NetworkAttachmentListCall), [*patch*](api::NetworkAttachmentPatchCall), [*set iam policy*](api::NetworkAttachmentSetIamPolicyCall) and [*test iam permissions*](api::NetworkAttachmentTestIamPermissionCall)
//! * [network edge security services](api::NetworkEdgeSecurityService)
//!  * [*aggregated list*](api::NetworkEdgeSecurityServiceAggregatedListCall), [*delete*](api::NetworkEdgeSecurityServiceDeleteCall), [*get*](api::NetworkEdgeSecurityServiceGetCall), [*insert*](api::NetworkEdgeSecurityServiceInsertCall) and [*patch*](api::NetworkEdgeSecurityServicePatchCall)
//! * [network endpoint groups](api::NetworkEndpointGroup)
//!  * [*aggregated list*](api::NetworkEndpointGroupAggregatedListCall), [*attach network endpoints*](api::NetworkEndpointGroupAttachNetworkEndpointCall), [*delete*](api::NetworkEndpointGroupDeleteCall), [*detach network endpoints*](api::NetworkEndpointGroupDetachNetworkEndpointCall), [*get*](api::NetworkEndpointGroupGetCall), [*insert*](api::NetworkEndpointGroupInsertCall), [*list*](api::NetworkEndpointGroupListCall), [*list network endpoints*](api::NetworkEndpointGroupListNetworkEndpointCall) and [*test iam permissions*](api::NetworkEndpointGroupTestIamPermissionCall)
//! * network firewall policies
//!  * [*add association*](api::NetworkFirewallPolicyAddAssociationCall), [*add rule*](api::NetworkFirewallPolicyAddRuleCall), [*clone rules*](api::NetworkFirewallPolicyCloneRuleCall), [*delete*](api::NetworkFirewallPolicyDeleteCall), [*get*](api::NetworkFirewallPolicyGetCall), [*get association*](api::NetworkFirewallPolicyGetAssociationCall), [*get iam policy*](api::NetworkFirewallPolicyGetIamPolicyCall), [*get rule*](api::NetworkFirewallPolicyGetRuleCall), [*insert*](api::NetworkFirewallPolicyInsertCall), [*list*](api::NetworkFirewallPolicyListCall), [*patch*](api::NetworkFirewallPolicyPatchCall), [*patch rule*](api::NetworkFirewallPolicyPatchRuleCall), [*remove association*](api::NetworkFirewallPolicyRemoveAssociationCall), [*remove rule*](api::NetworkFirewallPolicyRemoveRuleCall), [*set iam policy*](api::NetworkFirewallPolicySetIamPolicyCall) and [*test iam permissions*](api::NetworkFirewallPolicyTestIamPermissionCall)
//! * [networks](api::Network)
//!  * [*add peering*](api::NetworkAddPeeringCall), [*delete*](api::NetworkDeleteCall), [*get*](api::NetworkGetCall), [*get effective firewalls*](api::NetworkGetEffectiveFirewallCall), [*insert*](api::NetworkInsertCall), [*list*](api::NetworkListCall), [*list peering routes*](api::NetworkListPeeringRouteCall), [*patch*](api::NetworkPatchCall), [*remove peering*](api::NetworkRemovePeeringCall), [*switch to custom mode*](api::NetworkSwitchToCustomModeCall) and [*update peering*](api::NetworkUpdatePeeringCall)
//! * [node groups](api::NodeGroup)
//!  * [*add nodes*](api::NodeGroupAddNodeCall), [*aggregated list*](api::NodeGroupAggregatedListCall), [*delete*](api::NodeGroupDeleteCall), [*delete nodes*](api::NodeGroupDeleteNodeCall), [*get*](api::NodeGroupGetCall), [*get iam policy*](api::NodeGroupGetIamPolicyCall), [*insert*](api::NodeGroupInsertCall), [*list*](api::NodeGroupListCall), [*list nodes*](api::NodeGroupListNodeCall), [*patch*](api::NodeGroupPatchCall), [*perform maintenance*](api::NodeGroupPerformMaintenanceCall), [*set iam policy*](api::NodeGroupSetIamPolicyCall), [*set node template*](api::NodeGroupSetNodeTemplateCall), [*simulate maintenance event*](api::NodeGroupSimulateMaintenanceEventCall) and [*test iam permissions*](api::NodeGroupTestIamPermissionCall)
//! * [node templates](api::NodeTemplate)
//!  * [*aggregated list*](api::NodeTemplateAggregatedListCall), [*delete*](api::NodeTemplateDeleteCall), [*get*](api::NodeTemplateGetCall), [*get iam policy*](api::NodeTemplateGetIamPolicyCall), [*insert*](api::NodeTemplateInsertCall), [*list*](api::NodeTemplateListCall), [*set iam policy*](api::NodeTemplateSetIamPolicyCall) and [*test iam permissions*](api::NodeTemplateTestIamPermissionCall)
//! * [node types](api::NodeType)
//!  * [*aggregated list*](api::NodeTypeAggregatedListCall), [*get*](api::NodeTypeGetCall) and [*list*](api::NodeTypeListCall)
//! * [packet mirrorings](api::PacketMirroring)
//!  * [*aggregated list*](api::PacketMirroringAggregatedListCall), [*delete*](api::PacketMirroringDeleteCall), [*get*](api::PacketMirroringGetCall), [*insert*](api::PacketMirroringInsertCall), [*list*](api::PacketMirroringListCall), [*patch*](api::PacketMirroringPatchCall) and [*test iam permissions*](api::PacketMirroringTestIamPermissionCall)
//! * [projects](api::Project)
//!  * [*disable xpn host*](api::ProjectDisableXpnHostCall), [*disable xpn resource*](api::ProjectDisableXpnResourceCall), [*enable xpn host*](api::ProjectEnableXpnHostCall), [*enable xpn resource*](api::ProjectEnableXpnResourceCall), [*get*](api::ProjectGetCall), [*get xpn host*](api::ProjectGetXpnHostCall), [*get xpn resources*](api::ProjectGetXpnResourceCall), [*list xpn hosts*](api::ProjectListXpnHostCall), [*move disk*](api::ProjectMoveDiskCall), [*move instance*](api::ProjectMoveInstanceCall), [*set cloud armor tier*](api::ProjectSetCloudArmorTierCall), [*set common instance metadata*](api::ProjectSetCommonInstanceMetadataCall), [*set default network tier*](api::ProjectSetDefaultNetworkTierCall) and [*set usage export bucket*](api::ProjectSetUsageExportBucketCall)
//! * [public advertised prefixes](api::PublicAdvertisedPrefix)
//!  * [*announce*](api::PublicAdvertisedPrefixAnnounceCall), [*delete*](api::PublicAdvertisedPrefixDeleteCall), [*get*](api::PublicAdvertisedPrefixGetCall), [*insert*](api::PublicAdvertisedPrefixInsertCall), [*list*](api::PublicAdvertisedPrefixListCall), [*patch*](api::PublicAdvertisedPrefixPatchCall) and [*withdraw*](api::PublicAdvertisedPrefixWithdrawCall)
//! * [public delegated prefixes](api::PublicDelegatedPrefix)
//!  * [*aggregated list*](api::PublicDelegatedPrefixAggregatedListCall), [*announce*](api::PublicDelegatedPrefixAnnounceCall), [*delete*](api::PublicDelegatedPrefixDeleteCall), [*get*](api::PublicDelegatedPrefixGetCall), [*insert*](api::PublicDelegatedPrefixInsertCall), [*list*](api::PublicDelegatedPrefixListCall), [*patch*](api::PublicDelegatedPrefixPatchCall) and [*withdraw*](api::PublicDelegatedPrefixWithdrawCall)
//! * region autoscalers
//!  * [*delete*](api::RegionAutoscalerDeleteCall), [*get*](api::RegionAutoscalerGetCall), [*insert*](api::RegionAutoscalerInsertCall), [*list*](api::RegionAutoscalerListCall), [*patch*](api::RegionAutoscalerPatchCall) and [*update*](api::RegionAutoscalerUpdateCall)
//! * region backend services
//!  * [*delete*](api::RegionBackendServiceDeleteCall), [*get*](api::RegionBackendServiceGetCall), [*get health*](api::RegionBackendServiceGetHealthCall), [*get iam policy*](api::RegionBackendServiceGetIamPolicyCall), [*insert*](api::RegionBackendServiceInsertCall), [*list*](api::RegionBackendServiceListCall), [*list usable*](api::RegionBackendServiceListUsableCall), [*patch*](api::RegionBackendServicePatchCall), [*set iam policy*](api::RegionBackendServiceSetIamPolicyCall), [*set security policy*](api::RegionBackendServiceSetSecurityPolicyCall), [*test iam permissions*](api::RegionBackendServiceTestIamPermissionCall) and [*update*](api::RegionBackendServiceUpdateCall)
//! * region commitments
//!  * [*aggregated list*](api::RegionCommitmentAggregatedListCall), [*get*](api::RegionCommitmentGetCall), [*insert*](api::RegionCommitmentInsertCall), [*list*](api::RegionCommitmentListCall) and [*update*](api::RegionCommitmentUpdateCall)
//! * region disk types
//!  * [*get*](api::RegionDiskTypeGetCall) and [*list*](api::RegionDiskTypeListCall)
//! * region disks
//!  * [*add resource policies*](api::RegionDiskAddResourcePolicyCall), [*bulk insert*](api::RegionDiskBulkInsertCall), [*create snapshot*](api::RegionDiskCreateSnapshotCall), [*delete*](api::RegionDiskDeleteCall), [*get*](api::RegionDiskGetCall), [*get iam policy*](api::RegionDiskGetIamPolicyCall), [*insert*](api::RegionDiskInsertCall), [*list*](api::RegionDiskListCall), [*remove resource policies*](api::RegionDiskRemoveResourcePolicyCall), [*resize*](api::RegionDiskResizeCall), [*set iam policy*](api::RegionDiskSetIamPolicyCall), [*set labels*](api::RegionDiskSetLabelCall), [*start async replication*](api::RegionDiskStartAsyncReplicationCall), [*stop async replication*](api::RegionDiskStopAsyncReplicationCall), [*stop group async replication*](api::RegionDiskStopGroupAsyncReplicationCall), [*test iam permissions*](api::RegionDiskTestIamPermissionCall) and [*update*](api::RegionDiskUpdateCall)
//! * region health check services
//!  * [*delete*](api::RegionHealthCheckServiceDeleteCall), [*get*](api::RegionHealthCheckServiceGetCall), [*insert*](api::RegionHealthCheckServiceInsertCall), [*list*](api::RegionHealthCheckServiceListCall) and [*patch*](api::RegionHealthCheckServicePatchCall)
//! * region health checks
//!  * [*delete*](api::RegionHealthCheckDeleteCall), [*get*](api::RegionHealthCheckGetCall), [*insert*](api::RegionHealthCheckInsertCall), [*list*](api::RegionHealthCheckListCall), [*patch*](api::RegionHealthCheckPatchCall) and [*update*](api::RegionHealthCheckUpdateCall)
//! * region instance group managers
//!  * [*abandon instances*](api::RegionInstanceGroupManagerAbandonInstanceCall), [*apply updates to instances*](api::RegionInstanceGroupManagerApplyUpdatesToInstanceCall), [*create instances*](api::RegionInstanceGroupManagerCreateInstanceCall), [*delete*](api::RegionInstanceGroupManagerDeleteCall), [*delete instances*](api::RegionInstanceGroupManagerDeleteInstanceCall), [*delete per instance configs*](api::RegionInstanceGroupManagerDeletePerInstanceConfigCall), [*get*](api::RegionInstanceGroupManagerGetCall), [*insert*](api::RegionInstanceGroupManagerInsertCall), [*list*](api::RegionInstanceGroupManagerListCall), [*list errors*](api::RegionInstanceGroupManagerListErrorCall), [*list managed instances*](api::RegionInstanceGroupManagerListManagedInstanceCall), [*list per instance configs*](api::RegionInstanceGroupManagerListPerInstanceConfigCall), [*patch*](api::RegionInstanceGroupManagerPatchCall), [*patch per instance configs*](api::RegionInstanceGroupManagerPatchPerInstanceConfigCall), [*recreate instances*](api::RegionInstanceGroupManagerRecreateInstanceCall), [*resize*](api::RegionInstanceGroupManagerResizeCall), [*set instance template*](api::RegionInstanceGroupManagerSetInstanceTemplateCall), [*set target pools*](api::RegionInstanceGroupManagerSetTargetPoolCall) and [*update per instance configs*](api::RegionInstanceGroupManagerUpdatePerInstanceConfigCall)
//! * region instance groups
//!  * [*get*](api::RegionInstanceGroupGetCall), [*list*](api::RegionInstanceGroupListCall), [*list instances*](api::RegionInstanceGroupListInstanceCall) and [*set named ports*](api::RegionInstanceGroupSetNamedPortCall)
//! * region instance templates
//!  * [*delete*](api::RegionInstanceTemplateDeleteCall), [*get*](api::RegionInstanceTemplateGetCall), [*insert*](api::RegionInstanceTemplateInsertCall) and [*list*](api::RegionInstanceTemplateListCall)
//! * region instances
//!  * [*bulk insert*](api::RegionInstanceBulkInsertCall)
//! * region instant snapshots
//!  * [*delete*](api::RegionInstantSnapshotDeleteCall), [*get*](api::RegionInstantSnapshotGetCall), [*get iam policy*](api::RegionInstantSnapshotGetIamPolicyCall), [*insert*](api::RegionInstantSnapshotInsertCall), [*list*](api::RegionInstantSnapshotListCall), [*set iam policy*](api::RegionInstantSnapshotSetIamPolicyCall), [*set labels*](api::RegionInstantSnapshotSetLabelCall) and [*test iam permissions*](api::RegionInstantSnapshotTestIamPermissionCall)
//! * region network endpoint groups
//!  * [*attach network endpoints*](api::RegionNetworkEndpointGroupAttachNetworkEndpointCall), [*delete*](api::RegionNetworkEndpointGroupDeleteCall), [*detach network endpoints*](api::RegionNetworkEndpointGroupDetachNetworkEndpointCall), [*get*](api::RegionNetworkEndpointGroupGetCall), [*insert*](api::RegionNetworkEndpointGroupInsertCall), [*list*](api::RegionNetworkEndpointGroupListCall) and [*list network endpoints*](api::RegionNetworkEndpointGroupListNetworkEndpointCall)
//! * region network firewall policies
//!  * [*add association*](api::RegionNetworkFirewallPolicyAddAssociationCall), [*add rule*](api::RegionNetworkFirewallPolicyAddRuleCall), [*clone rules*](api::RegionNetworkFirewallPolicyCloneRuleCall), [*delete*](api::RegionNetworkFirewallPolicyDeleteCall), [*get*](api::RegionNetworkFirewallPolicyGetCall), [*get association*](api::RegionNetworkFirewallPolicyGetAssociationCall), [*get effective firewalls*](api::RegionNetworkFirewallPolicyGetEffectiveFirewallCall), [*get iam policy*](api::RegionNetworkFirewallPolicyGetIamPolicyCall), [*get rule*](api::RegionNetworkFirewallPolicyGetRuleCall), [*insert*](api::RegionNetworkFirewallPolicyInsertCall), [*list*](api::RegionNetworkFirewallPolicyListCall), [*patch*](api::RegionNetworkFirewallPolicyPatchCall), [*patch rule*](api::RegionNetworkFirewallPolicyPatchRuleCall), [*remove association*](api::RegionNetworkFirewallPolicyRemoveAssociationCall), [*remove rule*](api::RegionNetworkFirewallPolicyRemoveRuleCall), [*set iam policy*](api::RegionNetworkFirewallPolicySetIamPolicyCall) and [*test iam permissions*](api::RegionNetworkFirewallPolicyTestIamPermissionCall)
//! * region notification endpoints
//!  * [*delete*](api::RegionNotificationEndpointDeleteCall), [*get*](api::RegionNotificationEndpointGetCall), [*insert*](api::RegionNotificationEndpointInsertCall) and [*list*](api::RegionNotificationEndpointListCall)
//! * region operations
//!  * [*delete*](api::RegionOperationDeleteCall), [*get*](api::RegionOperationGetCall), [*list*](api::RegionOperationListCall) and [*wait*](api::RegionOperationWaitCall)
//! * region security policies
//!  * [*add rule*](api::RegionSecurityPolicyAddRuleCall), [*delete*](api::RegionSecurityPolicyDeleteCall), [*get*](api::RegionSecurityPolicyGetCall), [*get rule*](api::RegionSecurityPolicyGetRuleCall), [*insert*](api::RegionSecurityPolicyInsertCall), [*list*](api::RegionSecurityPolicyListCall), [*patch*](api::RegionSecurityPolicyPatchCall), [*patch rule*](api::RegionSecurityPolicyPatchRuleCall) and [*remove rule*](api::RegionSecurityPolicyRemoveRuleCall)
//! * region ssl certificates
//!  * [*delete*](api::RegionSslCertificateDeleteCall), [*get*](api::RegionSslCertificateGetCall), [*insert*](api::RegionSslCertificateInsertCall) and [*list*](api::RegionSslCertificateListCall)
//! * region ssl policies
//!  * [*delete*](api::RegionSslPolicyDeleteCall), [*get*](api::RegionSslPolicyGetCall), [*insert*](api::RegionSslPolicyInsertCall), [*list*](api::RegionSslPolicyListCall), [*list available features*](api::RegionSslPolicyListAvailableFeatureCall) and [*patch*](api::RegionSslPolicyPatchCall)
//! * region target http proxies
//!  * [*delete*](api::RegionTargetHttpProxyDeleteCall), [*get*](api::RegionTargetHttpProxyGetCall), [*insert*](api::RegionTargetHttpProxyInsertCall), [*list*](api::RegionTargetHttpProxyListCall) and [*set url map*](api::RegionTargetHttpProxySetUrlMapCall)
//! * region target https proxies
//!  * [*delete*](api::RegionTargetHttpsProxyDeleteCall), [*get*](api::RegionTargetHttpsProxyGetCall), [*insert*](api::RegionTargetHttpsProxyInsertCall), [*list*](api::RegionTargetHttpsProxyListCall), [*patch*](api::RegionTargetHttpsProxyPatchCall), [*set ssl certificates*](api::RegionTargetHttpsProxySetSslCertificateCall) and [*set url map*](api::RegionTargetHttpsProxySetUrlMapCall)
//! * region target tcp proxies
//!  * [*delete*](api::RegionTargetTcpProxyDeleteCall), [*get*](api::RegionTargetTcpProxyGetCall), [*insert*](api::RegionTargetTcpProxyInsertCall) and [*list*](api::RegionTargetTcpProxyListCall)
//! * region url maps
//!  * [*delete*](api::RegionUrlMapDeleteCall), [*get*](api::RegionUrlMapGetCall), [*insert*](api::RegionUrlMapInsertCall), [*list*](api::RegionUrlMapListCall), [*patch*](api::RegionUrlMapPatchCall), [*update*](api::RegionUrlMapUpdateCall) and [*validate*](api::RegionUrlMapValidateCall)
//! * region zones
//!  * [*list*](api::RegionZoneListCall)
//! * [regions](api::Region)
//!  * [*get*](api::RegionGetCall) and [*list*](api::RegionListCall)
//! * [reservations](api::Reservation)
//!  * [*aggregated list*](api::ReservationAggregatedListCall), [*delete*](api::ReservationDeleteCall), [*get*](api::ReservationGetCall), [*get iam policy*](api::ReservationGetIamPolicyCall), [*insert*](api::ReservationInsertCall), [*list*](api::ReservationListCall), [*resize*](api::ReservationResizeCall), [*set iam policy*](api::ReservationSetIamPolicyCall), [*test iam permissions*](api::ReservationTestIamPermissionCall) and [*update*](api::ReservationUpdateCall)
//! * [resource policies](api::ResourcePolicy)
//!  * [*aggregated list*](api::ResourcePolicyAggregatedListCall), [*delete*](api::ResourcePolicyDeleteCall), [*get*](api::ResourcePolicyGetCall), [*get iam policy*](api::ResourcePolicyGetIamPolicyCall), [*insert*](api::ResourcePolicyInsertCall), [*list*](api::ResourcePolicyListCall), [*patch*](api::ResourcePolicyPatchCall), [*set iam policy*](api::ResourcePolicySetIamPolicyCall) and [*test iam permissions*](api::ResourcePolicyTestIamPermissionCall)
//! * [routers](api::Router)
//!  * [*aggregated list*](api::RouterAggregatedListCall), [*delete*](api::RouterDeleteCall), [*get*](api::RouterGetCall), [*get nat ip info*](api::RouterGetNatIpInfoCall), [*get nat mapping info*](api::RouterGetNatMappingInfoCall), [*get router status*](api::RouterGetRouterStatuCall), [*insert*](api::RouterInsertCall), [*list*](api::RouterListCall), [*patch*](api::RouterPatchCall), [*preview*](api::RouterPreviewCall) and [*update*](api::RouterUpdateCall)
//! * [routes](api::Route)
//!  * [*delete*](api::RouteDeleteCall), [*get*](api::RouteGetCall), [*insert*](api::RouteInsertCall) and [*list*](api::RouteListCall)
//! * [security policies](api::SecurityPolicy)
//!  * [*add rule*](api::SecurityPolicyAddRuleCall), [*aggregated list*](api::SecurityPolicyAggregatedListCall), [*delete*](api::SecurityPolicyDeleteCall), [*get*](api::SecurityPolicyGetCall), [*get rule*](api::SecurityPolicyGetRuleCall), [*insert*](api::SecurityPolicyInsertCall), [*list*](api::SecurityPolicyListCall), [*list preconfigured expression sets*](api::SecurityPolicyListPreconfiguredExpressionSetCall), [*patch*](api::SecurityPolicyPatchCall), [*patch rule*](api::SecurityPolicyPatchRuleCall), [*remove rule*](api::SecurityPolicyRemoveRuleCall) and [*set labels*](api::SecurityPolicySetLabelCall)
//! * [service attachments](api::ServiceAttachment)
//!  * [*aggregated list*](api::ServiceAttachmentAggregatedListCall), [*delete*](api::ServiceAttachmentDeleteCall), [*get*](api::ServiceAttachmentGetCall), [*get iam policy*](api::ServiceAttachmentGetIamPolicyCall), [*insert*](api::ServiceAttachmentInsertCall), [*list*](api::ServiceAttachmentListCall), [*patch*](api::ServiceAttachmentPatchCall), [*set iam policy*](api::ServiceAttachmentSetIamPolicyCall) and [*test iam permissions*](api::ServiceAttachmentTestIamPermissionCall)
//! * snapshot settings
//!  * [*get*](api::SnapshotSettingGetCall) and [*patch*](api::SnapshotSettingPatchCall)
//! * [snapshots](api::Snapshot)
//!  * [*delete*](api::SnapshotDeleteCall), [*get*](api::SnapshotGetCall), [*get iam policy*](api::SnapshotGetIamPolicyCall), [*insert*](api::SnapshotInsertCall), [*list*](api::SnapshotListCall), [*set iam policy*](api::SnapshotSetIamPolicyCall), [*set labels*](api::SnapshotSetLabelCall) and [*test iam permissions*](api::SnapshotTestIamPermissionCall)
//! * [ssl certificates](api::SslCertificate)
//!  * [*aggregated list*](api::SslCertificateAggregatedListCall), [*delete*](api::SslCertificateDeleteCall), [*get*](api::SslCertificateGetCall), [*insert*](api::SslCertificateInsertCall) and [*list*](api::SslCertificateListCall)
//! * [ssl policies](api::SslPolicy)
//!  * [*aggregated list*](api::SslPolicyAggregatedListCall), [*delete*](api::SslPolicyDeleteCall), [*get*](api::SslPolicyGetCall), [*insert*](api::SslPolicyInsertCall), [*list*](api::SslPolicyListCall), [*list available features*](api::SslPolicyListAvailableFeatureCall) and [*patch*](api::SslPolicyPatchCall)
//! * [storage pool types](api::StoragePoolType)
//!  * [*aggregated list*](api::StoragePoolTypeAggregatedListCall), [*get*](api::StoragePoolTypeGetCall) and [*list*](api::StoragePoolTypeListCall)
//! * [storage pools](api::StoragePool)
//!  * [*aggregated list*](api::StoragePoolAggregatedListCall), [*delete*](api::StoragePoolDeleteCall), [*get*](api::StoragePoolGetCall), [*get iam policy*](api::StoragePoolGetIamPolicyCall), [*insert*](api::StoragePoolInsertCall), [*list*](api::StoragePoolListCall), [*list disks*](api::StoragePoolListDiskCall), [*set iam policy*](api::StoragePoolSetIamPolicyCall), [*test iam permissions*](api::StoragePoolTestIamPermissionCall) and [*update*](api::StoragePoolUpdateCall)
//! * [subnetworks](api::Subnetwork)
//!  * [*aggregated list*](api::SubnetworkAggregatedListCall), [*delete*](api::SubnetworkDeleteCall), [*expand ip cidr range*](api::SubnetworkExpandIpCidrRangeCall), [*get*](api::SubnetworkGetCall), [*get iam policy*](api::SubnetworkGetIamPolicyCall), [*insert*](api::SubnetworkInsertCall), [*list*](api::SubnetworkListCall), [*list usable*](api::SubnetworkListUsableCall), [*patch*](api::SubnetworkPatchCall), [*set iam policy*](api::SubnetworkSetIamPolicyCall), [*set private ip google access*](api::SubnetworkSetPrivateIpGoogleAccesCall) and [*test iam permissions*](api::SubnetworkTestIamPermissionCall)
//! * [target grpc proxies](api::TargetGrpcProxy)
//!  * [*delete*](api::TargetGrpcProxyDeleteCall), [*get*](api::TargetGrpcProxyGetCall), [*insert*](api::TargetGrpcProxyInsertCall), [*list*](api::TargetGrpcProxyListCall) and [*patch*](api::TargetGrpcProxyPatchCall)
//! * [target http proxies](api::TargetHttpProxy)
//!  * [*aggregated list*](api::TargetHttpProxyAggregatedListCall), [*delete*](api::TargetHttpProxyDeleteCall), [*get*](api::TargetHttpProxyGetCall), [*insert*](api::TargetHttpProxyInsertCall), [*list*](api::TargetHttpProxyListCall), [*patch*](api::TargetHttpProxyPatchCall) and [*set url map*](api::TargetHttpProxySetUrlMapCall)
//! * [target https proxies](api::TargetHttpsProxy)
//!  * [*aggregated list*](api::TargetHttpsProxyAggregatedListCall), [*delete*](api::TargetHttpsProxyDeleteCall), [*get*](api::TargetHttpsProxyGetCall), [*insert*](api::TargetHttpsProxyInsertCall), [*list*](api::TargetHttpsProxyListCall), [*patch*](api::TargetHttpsProxyPatchCall), [*set certificate map*](api::TargetHttpsProxySetCertificateMapCall), [*set quic override*](api::TargetHttpsProxySetQuicOverrideCall), [*set ssl certificates*](api::TargetHttpsProxySetSslCertificateCall), [*set ssl policy*](api::TargetHttpsProxySetSslPolicyCall) and [*set url map*](api::TargetHttpsProxySetUrlMapCall)
//! * [target instances](api::TargetInstance)
//!  * [*aggregated list*](api::TargetInstanceAggregatedListCall), [*delete*](api::TargetInstanceDeleteCall), [*get*](api::TargetInstanceGetCall), [*insert*](api::TargetInstanceInsertCall), [*list*](api::TargetInstanceListCall) and [*set security policy*](api::TargetInstanceSetSecurityPolicyCall)
//! * [target pools](api::TargetPool)
//!  * [*add health check*](api::TargetPoolAddHealthCheckCall), [*add instance*](api::TargetPoolAddInstanceCall), [*aggregated list*](api::TargetPoolAggregatedListCall), [*delete*](api::TargetPoolDeleteCall), [*get*](api::TargetPoolGetCall), [*get health*](api::TargetPoolGetHealthCall), [*insert*](api::TargetPoolInsertCall), [*list*](api::TargetPoolListCall), [*remove health check*](api::TargetPoolRemoveHealthCheckCall), [*remove instance*](api::TargetPoolRemoveInstanceCall), [*set backup*](api::TargetPoolSetBackupCall) and [*set security policy*](api::TargetPoolSetSecurityPolicyCall)
//! * [target ssl proxies](api::TargetSslProxy)
//!  * [*delete*](api::TargetSslProxyDeleteCall), [*get*](api::TargetSslProxyGetCall), [*insert*](api::TargetSslProxyInsertCall), [*list*](api::TargetSslProxyListCall), [*set backend service*](api::TargetSslProxySetBackendServiceCall), [*set certificate map*](api::TargetSslProxySetCertificateMapCall), [*set proxy header*](api::TargetSslProxySetProxyHeaderCall), [*set ssl certificates*](api::TargetSslProxySetSslCertificateCall) and [*set ssl policy*](api::TargetSslProxySetSslPolicyCall)
//! * [target tcp proxies](api::TargetTcpProxy)
//!  * [*aggregated list*](api::TargetTcpProxyAggregatedListCall), [*delete*](api::TargetTcpProxyDeleteCall), [*get*](api::TargetTcpProxyGetCall), [*insert*](api::TargetTcpProxyInsertCall), [*list*](api::TargetTcpProxyListCall), [*set backend service*](api::TargetTcpProxySetBackendServiceCall) and [*set proxy header*](api::TargetTcpProxySetProxyHeaderCall)
//! * [target vpn gateways](api::TargetVpnGateway)
//!  * [*aggregated list*](api::TargetVpnGatewayAggregatedListCall), [*delete*](api::TargetVpnGatewayDeleteCall), [*get*](api::TargetVpnGatewayGetCall), [*insert*](api::TargetVpnGatewayInsertCall), [*list*](api::TargetVpnGatewayListCall) and [*set labels*](api::TargetVpnGatewaySetLabelCall)
//! * [url maps](api::UrlMap)
//!  * [*aggregated list*](api::UrlMapAggregatedListCall), [*delete*](api::UrlMapDeleteCall), [*get*](api::UrlMapGetCall), [*insert*](api::UrlMapInsertCall), [*invalidate cache*](api::UrlMapInvalidateCacheCall), [*list*](api::UrlMapListCall), [*patch*](api::UrlMapPatchCall), [*update*](api::UrlMapUpdateCall) and [*validate*](api::UrlMapValidateCall)
//! * [vpn gateways](api::VpnGateway)
//!  * [*aggregated list*](api::VpnGatewayAggregatedListCall), [*delete*](api::VpnGatewayDeleteCall), [*get*](api::VpnGatewayGetCall), [*get status*](api::VpnGatewayGetStatuCall), [*insert*](api::VpnGatewayInsertCall), [*list*](api::VpnGatewayListCall), [*set labels*](api::VpnGatewaySetLabelCall) and [*test iam permissions*](api::VpnGatewayTestIamPermissionCall)
//! * [vpn tunnels](api::VpnTunnel)
//!  * [*aggregated list*](api::VpnTunnelAggregatedListCall), [*delete*](api::VpnTunnelDeleteCall), [*get*](api::VpnTunnelGetCall), [*insert*](api::VpnTunnelInsertCall), [*list*](api::VpnTunnelListCall) and [*set labels*](api::VpnTunnelSetLabelCall)
//! * zone operations
//!  * [*delete*](api::ZoneOperationDeleteCall), [*get*](api::ZoneOperationGetCall), [*list*](api::ZoneOperationListCall) and [*wait*](api::ZoneOperationWaitCall)
//! * [zones](api::Zone)
//!  * [*get*](api::ZoneGetCall) and [*list*](api::ZoneListCall)
//! 
//! 
//! 
//! 
//! Not what you are looking for ? Find all other Google APIs in their Rust [documentation index](http://byron.github.io/google-apis-rs).
//! 
//! # Structure of this Library
//! 
//! The API is structured into the following primary items:
//! 
//! * **[Hub](Compute)**
//!     * a central object to maintain state and allow accessing all *Activities*
//!     * creates [*Method Builders*](client::MethodsBuilder) which in turn
//!       allow access to individual [*Call Builders*](client::CallBuilder)
//! * **[Resources](client::Resource)**
//!     * primary types that you can apply *Activities* to
//!     * a collection of properties and *Parts*
//!     * **[Parts](client::Part)**
//!         * a collection of properties
//!         * never directly used in *Activities*
//! * **[Activities](client::CallBuilder)**
//!     * operations to apply to *Resources*
//! 
//! All *structures* are marked with applicable traits to further categorize them and ease browsing.
//! 
//! Generally speaking, you can invoke *Activities* like this:
//! 
//! ```Rust,ignore
//! let r = hub.resource().activity(...).doit().await
//! ```
//! 
//! Or specifically ...
//! 
//! ```ignore
//! let r = hub.addresses().delete(...).doit().await
//! let r = hub.addresses().insert(...).doit().await
//! let r = hub.addresses().move_(...).doit().await
//! let r = hub.addresses().set_labels(...).doit().await
//! let r = hub.autoscalers().delete(...).doit().await
//! let r = hub.autoscalers().insert(...).doit().await
//! let r = hub.autoscalers().patch(...).doit().await
//! let r = hub.autoscalers().update(...).doit().await
//! let r = hub.backend_buckets().add_signed_url_key(...).doit().await
//! let r = hub.backend_buckets().delete(...).doit().await
//! let r = hub.backend_buckets().delete_signed_url_key(...).doit().await
//! let r = hub.backend_buckets().insert(...).doit().await
//! let r = hub.backend_buckets().patch(...).doit().await
//! let r = hub.backend_buckets().set_edge_security_policy(...).doit().await
//! let r = hub.backend_buckets().update(...).doit().await
//! let r = hub.backend_services().add_signed_url_key(...).doit().await
//! let r = hub.backend_services().delete(...).doit().await
//! let r = hub.backend_services().delete_signed_url_key(...).doit().await
//! let r = hub.backend_services().insert(...).doit().await
//! let r = hub.backend_services().patch(...).doit().await
//! let r = hub.backend_services().set_edge_security_policy(...).doit().await
//! let r = hub.backend_services().set_security_policy(...).doit().await
//! let r = hub.backend_services().update(...).doit().await
//! let r = hub.disks().add_resource_policies(...).doit().await
//! let r = hub.disks().bulk_insert(...).doit().await
//! let r = hub.disks().create_snapshot(...).doit().await
//! let r = hub.disks().delete(...).doit().await
//! let r = hub.disks().insert(...).doit().await
//! let r = hub.disks().remove_resource_policies(...).doit().await
//! let r = hub.disks().resize(...).doit().await
//! let r = hub.disks().set_labels(...).doit().await
//! let r = hub.disks().start_async_replication(...).doit().await
//! let r = hub.disks().stop_async_replication(...).doit().await
//! let r = hub.disks().stop_group_async_replication(...).doit().await
//! let r = hub.disks().update(...).doit().await
//! let r = hub.external_vpn_gateways().delete(...).doit().await
//! let r = hub.external_vpn_gateways().insert(...).doit().await
//! let r = hub.external_vpn_gateways().set_labels(...).doit().await
//! let r = hub.firewall_policies().add_association(...).doit().await
//! let r = hub.firewall_policies().add_rule(...).doit().await
//! let r = hub.firewall_policies().clone_rules(...).doit().await
//! let r = hub.firewall_policies().delete(...).doit().await
//! let r = hub.firewall_policies().insert(...).doit().await
//! let r = hub.firewall_policies().move_(...).doit().await
//! let r = hub.firewall_policies().patch(...).doit().await
//! let r = hub.firewall_policies().patch_rule(...).doit().await
//! let r = hub.firewall_policies().remove_association(...).doit().await
//! let r = hub.firewall_policies().remove_rule(...).doit().await
//! let r = hub.firewalls().delete(...).doit().await
//! let r = hub.firewalls().insert(...).doit().await
//! let r = hub.firewalls().patch(...).doit().await
//! let r = hub.firewalls().update(...).doit().await
//! let r = hub.forwarding_rules().delete(...).doit().await
//! let r = hub.forwarding_rules().insert(...).doit().await
//! let r = hub.forwarding_rules().patch(...).doit().await
//! let r = hub.forwarding_rules().set_labels(...).doit().await
//! let r = hub.forwarding_rules().set_target(...).doit().await
//! let r = hub.global_addresses().delete(...).doit().await
//! let r = hub.global_addresses().insert(...).doit().await
//! let r = hub.global_addresses().move_(...).doit().await
//! let r = hub.global_addresses().set_labels(...).doit().await
//! let r = hub.global_forwarding_rules().delete(...).doit().await
//! let r = hub.global_forwarding_rules().insert(...).doit().await
//! let r = hub.global_forwarding_rules().patch(...).doit().await
//! let r = hub.global_forwarding_rules().set_labels(...).doit().await
//! let r = hub.global_forwarding_rules().set_target(...).doit().await
//! let r = hub.global_network_endpoint_groups().attach_network_endpoints(...).doit().await
//! let r = hub.global_network_endpoint_groups().delete(...).doit().await
//! let r = hub.global_network_endpoint_groups().detach_network_endpoints(...).doit().await
//! let r = hub.global_network_endpoint_groups().insert(...).doit().await
//! let r = hub.global_operations().get(...).doit().await
//! let r = hub.global_operations().wait(...).doit().await
//! let r = hub.global_organization_operations().get(...).doit().await
//! let r = hub.global_public_delegated_prefixes().delete(...).doit().await
//! let r = hub.global_public_delegated_prefixes().insert(...).doit().await
//! let r = hub.global_public_delegated_prefixes().patch(...).doit().await
//! let r = hub.health_checks().delete(...).doit().await
//! let r = hub.health_checks().insert(...).doit().await
//! let r = hub.health_checks().patch(...).doit().await
//! let r = hub.health_checks().update(...).doit().await
//! let r = hub.http_health_checks().delete(...).doit().await
//! let r = hub.http_health_checks().insert(...).doit().await
//! let r = hub.http_health_checks().patch(...).doit().await
//! let r = hub.http_health_checks().update(...).doit().await
//! let r = hub.https_health_checks().delete(...).doit().await
//! let r = hub.https_health_checks().insert(...).doit().await
//! let r = hub.https_health_checks().patch(...).doit().await
//! let r = hub.https_health_checks().update(...).doit().await
//! let r = hub.images().delete(...).doit().await
//! let r = hub.images().deprecate(...).doit().await
//! let r = hub.images().insert(...).doit().await
//! let r = hub.images().patch(...).doit().await
//! let r = hub.images().set_labels(...).doit().await
//! let r = hub.instance_group_manager_resize_requests().cancel(...).doit().await
//! let r = hub.instance_group_manager_resize_requests().delete(...).doit().await
//! let r = hub.instance_group_manager_resize_requests().insert(...).doit().await
//! let r = hub.instance_group_managers().abandon_instances(...).doit().await
//! let r = hub.instance_group_managers().apply_updates_to_instances(...).doit().await
//! let r = hub.instance_group_managers().create_instances(...).doit().await
//! let r = hub.instance_group_managers().delete(...).doit().await
//! let r = hub.instance_group_managers().delete_instances(...).doit().await
//! let r = hub.instance_group_managers().delete_per_instance_configs(...).doit().await
//! let r = hub.instance_group_managers().insert(...).doit().await
//! let r = hub.instance_group_managers().patch(...).doit().await
//! let r = hub.instance_group_managers().patch_per_instance_configs(...).doit().await
//! let r = hub.instance_group_managers().recreate_instances(...).doit().await
//! let r = hub.instance_group_managers().resize(...).doit().await
//! let r = hub.instance_group_managers().set_instance_template(...).doit().await
//! let r = hub.instance_group_managers().set_target_pools(...).doit().await
//! let r = hub.instance_group_managers().update_per_instance_configs(...).doit().await
//! let r = hub.instance_groups().add_instances(...).doit().await
//! let r = hub.instance_groups().delete(...).doit().await
//! let r = hub.instance_groups().insert(...).doit().await
//! let r = hub.instance_groups().remove_instances(...).doit().await
//! let r = hub.instance_groups().set_named_ports(...).doit().await
//! let r = hub.instance_settings().patch(...).doit().await
//! let r = hub.instance_templates().delete(...).doit().await
//! let r = hub.instance_templates().insert(...).doit().await
//! let r = hub.instances().add_access_config(...).doit().await
//! let r = hub.instances().add_resource_policies(...).doit().await
//! let r = hub.instances().attach_disk(...).doit().await
//! let r = hub.instances().bulk_insert(...).doit().await
//! let r = hub.instances().delete(...).doit().await
//! let r = hub.instances().delete_access_config(...).doit().await
//! let r = hub.instances().detach_disk(...).doit().await
//! let r = hub.instances().insert(...).doit().await
//! let r = hub.instances().perform_maintenance(...).doit().await
//! let r = hub.instances().remove_resource_policies(...).doit().await
//! let r = hub.instances().reset(...).doit().await
//! let r = hub.instances().resume(...).doit().await
//! let r = hub.instances().set_deletion_protection(...).doit().await
//! let r = hub.instances().set_disk_auto_delete(...).doit().await
//! let r = hub.instances().set_labels(...).doit().await
//! let r = hub.instances().set_machine_resources(...).doit().await
//! let r = hub.instances().set_machine_type(...).doit().await
//! let r = hub.instances().set_metadata(...).doit().await
//! let r = hub.instances().set_min_cpu_platform(...).doit().await
//! let r = hub.instances().set_name(...).doit().await
//! let r = hub.instances().set_scheduling(...).doit().await
//! let r = hub.instances().set_security_policy(...).doit().await
//! let r = hub.instances().set_service_account(...).doit().await
//! let r = hub.instances().set_shielded_instance_integrity_policy(...).doit().await
//! let r = hub.instances().set_tags(...).doit().await
//! let r = hub.instances().simulate_maintenance_event(...).doit().await
//! let r = hub.instances().start(...).doit().await
//! let r = hub.instances().start_with_encryption_key(...).doit().await
//! let r = hub.instances().stop(...).doit().await
//! let r = hub.instances().suspend(...).doit().await
//! let r = hub.instances().update(...).doit().await
//! let r = hub.instances().update_access_config(...).doit().await
//! let r = hub.instances().update_display_device(...).doit().await
//! let r = hub.instances().update_network_interface(...).doit().await
//! let r = hub.instances().update_shielded_instance_config(...).doit().await
//! let r = hub.instant_snapshots().delete(...).doit().await
//! let r = hub.instant_snapshots().insert(...).doit().await
//! let r = hub.instant_snapshots().set_labels(...).doit().await
//! let r = hub.interconnect_attachments().delete(...).doit().await
//! let r = hub.interconnect_attachments().insert(...).doit().await
//! let r = hub.interconnect_attachments().patch(...).doit().await
//! let r = hub.interconnect_attachments().set_labels(...).doit().await
//! let r = hub.interconnects().delete(...).doit().await
//! let r = hub.interconnects().insert(...).doit().await
//! let r = hub.interconnects().patch(...).doit().await
//! let r = hub.interconnects().set_labels(...).doit().await
//! let r = hub.licenses().delete(...).doit().await
//! let r = hub.licenses().insert(...).doit().await
//! let r = hub.machine_images().delete(...).doit().await
//! let r = hub.machine_images().insert(...).doit().await
//! let r = hub.network_attachments().delete(...).doit().await
//! let r = hub.network_attachments().insert(...).doit().await
//! let r = hub.network_attachments().patch(...).doit().await
//! let r = hub.network_edge_security_services().delete(...).doit().await
//! let r = hub.network_edge_security_services().insert(...).doit().await
//! let r = hub.network_edge_security_services().patch(...).doit().await
//! let r = hub.network_endpoint_groups().attach_network_endpoints(...).doit().await
//! let r = hub.network_endpoint_groups().delete(...).doit().await
//! let r = hub.network_endpoint_groups().detach_network_endpoints(...).doit().await
//! let r = hub.network_endpoint_groups().insert(...).doit().await
//! let r = hub.network_firewall_policies().add_association(...).doit().await
//! let r = hub.network_firewall_policies().add_rule(...).doit().await
//! let r = hub.network_firewall_policies().clone_rules(...).doit().await
//! let r = hub.network_firewall_policies().delete(...).doit().await
//! let r = hub.network_firewall_policies().insert(...).doit().await
//! let r = hub.network_firewall_policies().patch(...).doit().await
//! let r = hub.network_firewall_policies().patch_rule(...).doit().await
//! let r = hub.network_firewall_policies().remove_association(...).doit().await
//! let r = hub.network_firewall_policies().remove_rule(...).doit().await
//! let r = hub.networks().add_peering(...).doit().await
//! let r = hub.networks().delete(...).doit().await
//! let r = hub.networks().insert(...).doit().await
//! let r = hub.networks().patch(...).doit().await
//! let r = hub.networks().remove_peering(...).doit().await
//! let r = hub.networks().switch_to_custom_mode(...).doit().await
//! let r = hub.networks().update_peering(...).doit().await
//! let r = hub.node_groups().add_nodes(...).doit().await
//! let r = hub.node_groups().delete(...).doit().await
//! let r = hub.node_groups().delete_nodes(...).doit().await
//! let r = hub.node_groups().insert(...).doit().await
//! let r = hub.node_groups().patch(...).doit().await
//! let r = hub.node_groups().perform_maintenance(...).doit().await
//! let r = hub.node_groups().set_node_template(...).doit().await
//! let r = hub.node_groups().simulate_maintenance_event(...).doit().await
//! let r = hub.node_templates().delete(...).doit().await
//! let r = hub.node_templates().insert(...).doit().await
//! let r = hub.packet_mirrorings().delete(...).doit().await
//! let r = hub.packet_mirrorings().insert(...).doit().await
//! let r = hub.packet_mirrorings().patch(...).doit().await
//! let r = hub.projects().disable_xpn_host(...).doit().await
//! let r = hub.projects().disable_xpn_resource(...).doit().await
//! let r = hub.projects().enable_xpn_host(...).doit().await
//! let r = hub.projects().enable_xpn_resource(...).doit().await
//! let r = hub.projects().move_disk(...).doit().await
//! let r = hub.projects().move_instance(...).doit().await
//! let r = hub.projects().set_cloud_armor_tier(...).doit().await
//! let r = hub.projects().set_common_instance_metadata(...).doit().await
//! let r = hub.projects().set_default_network_tier(...).doit().await
//! let r = hub.projects().set_usage_export_bucket(...).doit().await
//! let r = hub.public_advertised_prefixes().announce(...).doit().await
//! let r = hub.public_advertised_prefixes().delete(...).doit().await
//! let r = hub.public_advertised_prefixes().insert(...).doit().await
//! let r = hub.public_advertised_prefixes().patch(...).doit().await
//! let r = hub.public_advertised_prefixes().withdraw(...).doit().await
//! let r = hub.public_delegated_prefixes().announce(...).doit().await
//! let r = hub.public_delegated_prefixes().delete(...).doit().await
//! let r = hub.public_delegated_prefixes().insert(...).doit().await
//! let r = hub.public_delegated_prefixes().patch(...).doit().await
//! let r = hub.public_delegated_prefixes().withdraw(...).doit().await
//! let r = hub.region_autoscalers().delete(...).doit().await
//! let r = hub.region_autoscalers().insert(...).doit().await
//! let r = hub.region_autoscalers().patch(...).doit().await
//! let r = hub.region_autoscalers().update(...).doit().await
//! let r = hub.region_backend_services().delete(...).doit().await
//! let r = hub.region_backend_services().insert(...).doit().await
//! let r = hub.region_backend_services().patch(...).doit().await
//! let r = hub.region_backend_services().set_security_policy(...).doit().await
//! let r = hub.region_backend_services().update(...).doit().await
//! let r = hub.region_commitments().insert(...).doit().await
//! let r = hub.region_commitments().update(...).doit().await
//! let r = hub.region_disks().add_resource_policies(...).doit().await
//! let r = hub.region_disks().bulk_insert(...).doit().await
//! let r = hub.region_disks().create_snapshot(...).doit().await
//! let r = hub.region_disks().delete(...).doit().await
//! let r = hub.region_disks().insert(...).doit().await
//! let r = hub.region_disks().remove_resource_policies(...).doit().await
//! let r = hub.region_disks().resize(...).doit().await
//! let r = hub.region_disks().set_labels(...).doit().await
//! let r = hub.region_disks().start_async_replication(...).doit().await
//! let r = hub.region_disks().stop_async_replication(...).doit().await
//! let r = hub.region_disks().stop_group_async_replication(...).doit().await
//! let r = hub.region_disks().update(...).doit().await
//! let r = hub.region_health_check_services().delete(...).doit().await
//! let r = hub.region_health_check_services().insert(...).doit().await
//! let r = hub.region_health_check_services().patch(...).doit().await
//! let r = hub.region_health_checks().delete(...).doit().await
//! let r = hub.region_health_checks().insert(...).doit().await
//! let r = hub.region_health_checks().patch(...).doit().await
//! let r = hub.region_health_checks().update(...).doit().await
//! let r = hub.region_instance_group_managers().abandon_instances(...).doit().await
//! let r = hub.region_instance_group_managers().apply_updates_to_instances(...).doit().await
//! let r = hub.region_instance_group_managers().create_instances(...).doit().await
//! let r = hub.region_instance_group_managers().delete(...).doit().await
//! let r = hub.region_instance_group_managers().delete_instances(...).doit().await
//! let r = hub.region_instance_group_managers().delete_per_instance_configs(...).doit().await
//! let r = hub.region_instance_group_managers().insert(...).doit().await
//! let r = hub.region_instance_group_managers().patch(...).doit().await
//! let r = hub.region_instance_group_managers().patch_per_instance_configs(...).doit().await
//! let r = hub.region_instance_group_managers().recreate_instances(...).doit().await
//! let r = hub.region_instance_group_managers().resize(...).doit().await
//! let r = hub.region_instance_group_managers().set_instance_template(...).doit().await
//! let r = hub.region_instance_group_managers().set_target_pools(...).doit().await
//! let r = hub.region_instance_group_managers().update_per_instance_configs(...).doit().await
//! let r = hub.region_instance_groups().set_named_ports(...).doit().await
//! let r = hub.region_instance_templates().delete(...).doit().await
//! let r = hub.region_instance_templates().insert(...).doit().await
//! let r = hub.region_instances().bulk_insert(...).doit().await
//! let r = hub.region_instant_snapshots().delete(...).doit().await
//! let r = hub.region_instant_snapshots().insert(...).doit().await
//! let r = hub.region_instant_snapshots().set_labels(...).doit().await
//! let r = hub.region_network_endpoint_groups().attach_network_endpoints(...).doit().await
//! let r = hub.region_network_endpoint_groups().delete(...).doit().await
//! let r = hub.region_network_endpoint_groups().detach_network_endpoints(...).doit().await
//! let r = hub.region_network_endpoint_groups().insert(...).doit().await
//! let r = hub.region_network_firewall_policies().add_association(...).doit().await
//! let r = hub.region_network_firewall_policies().add_rule(...).doit().await
//! let r = hub.region_network_firewall_policies().clone_rules(...).doit().await
//! let r = hub.region_network_firewall_policies().delete(...).doit().await
//! let r = hub.region_network_firewall_policies().insert(...).doit().await
//! let r = hub.region_network_firewall_policies().patch(...).doit().await
//! let r = hub.region_network_firewall_policies().patch_rule(...).doit().await
//! let r = hub.region_network_firewall_policies().remove_association(...).doit().await
//! let r = hub.region_network_firewall_policies().remove_rule(...).doit().await
//! let r = hub.region_notification_endpoints().delete(...).doit().await
//! let r = hub.region_notification_endpoints().insert(...).doit().await
//! let r = hub.region_operations().get(...).doit().await
//! let r = hub.region_operations().wait(...).doit().await
//! let r = hub.region_security_policies().add_rule(...).doit().await
//! let r = hub.region_security_policies().delete(...).doit().await
//! let r = hub.region_security_policies().insert(...).doit().await
//! let r = hub.region_security_policies().patch(...).doit().await
//! let r = hub.region_security_policies().patch_rule(...).doit().await
//! let r = hub.region_security_policies().remove_rule(...).doit().await
//! let r = hub.region_ssl_certificates().delete(...).doit().await
//! let r = hub.region_ssl_certificates().insert(...).doit().await
//! let r = hub.region_ssl_policies().delete(...).doit().await
//! let r = hub.region_ssl_policies().insert(...).doit().await
//! let r = hub.region_ssl_policies().patch(...).doit().await
//! let r = hub.region_target_http_proxies().delete(...).doit().await
//! let r = hub.region_target_http_proxies().insert(...).doit().await
//! let r = hub.region_target_http_proxies().set_url_map(...).doit().await
//! let r = hub.region_target_https_proxies().delete(...).doit().await
//! let r = hub.region_target_https_proxies().insert(...).doit().await
//! let r = hub.region_target_https_proxies().patch(...).doit().await
//! let r = hub.region_target_https_proxies().set_ssl_certificates(...).doit().await
//! let r = hub.region_target_https_proxies().set_url_map(...).doit().await
//! let r = hub.region_target_tcp_proxies().delete(...).doit().await
//! let r = hub.region_target_tcp_proxies().insert(...).doit().await
//! let r = hub.region_url_maps().delete(...).doit().await
//! let r = hub.region_url_maps().insert(...).doit().await
//! let r = hub.region_url_maps().patch(...).doit().await
//! let r = hub.region_url_maps().update(...).doit().await
//! let r = hub.reservations().delete(...).doit().await
//! let r = hub.reservations().insert(...).doit().await
//! let r = hub.reservations().resize(...).doit().await
//! let r = hub.reservations().update(...).doit().await
//! let r = hub.resource_policies().delete(...).doit().await
//! let r = hub.resource_policies().insert(...).doit().await
//! let r = hub.resource_policies().patch(...).doit().await
//! let r = hub.routers().delete(...).doit().await
//! let r = hub.routers().insert(...).doit().await
//! let r = hub.routers().patch(...).doit().await
//! let r = hub.routers().update(...).doit().await
//! let r = hub.routes().delete(...).doit().await
//! let r = hub.routes().insert(...).doit().await
//! let r = hub.security_policies().add_rule(...).doit().await
//! let r = hub.security_policies().delete(...).doit().await
//! let r = hub.security_policies().insert(...).doit().await
//! let r = hub.security_policies().patch(...).doit().await
//! let r = hub.security_policies().patch_rule(...).doit().await
//! let r = hub.security_policies().remove_rule(...).doit().await
//! let r = hub.security_policies().set_labels(...).doit().await
//! let r = hub.service_attachments().delete(...).doit().await
//! let r = hub.service_attachments().insert(...).doit().await
//! let r = hub.service_attachments().patch(...).doit().await
//! let r = hub.snapshot_settings().patch(...).doit().await
//! let r = hub.snapshots().delete(...).doit().await
//! let r = hub.snapshots().insert(...).doit().await
//! let r = hub.snapshots().set_labels(...).doit().await
//! let r = hub.ssl_certificates().delete(...).doit().await
//! let r = hub.ssl_certificates().insert(...).doit().await
//! let r = hub.ssl_policies().delete(...).doit().await
//! let r = hub.ssl_policies().insert(...).doit().await
//! let r = hub.ssl_policies().patch(...).doit().await
//! let r = hub.storage_pools().delete(...).doit().await
//! let r = hub.storage_pools().insert(...).doit().await
//! let r = hub.storage_pools().update(...).doit().await
//! let r = hub.subnetworks().delete(...).doit().await
//! let r = hub.subnetworks().expand_ip_cidr_range(...).doit().await
//! let r = hub.subnetworks().insert(...).doit().await
//! let r = hub.subnetworks().patch(...).doit().await
//! let r = hub.subnetworks().set_private_ip_google_access(...).doit().await
//! let r = hub.target_grpc_proxies().delete(...).doit().await
//! let r = hub.target_grpc_proxies().insert(...).doit().await
//! let r = hub.target_grpc_proxies().patch(...).doit().await
//! let r = hub.target_http_proxies().delete(...).doit().await
//! let r = hub.target_http_proxies().insert(...).doit().await
//! let r = hub.target_http_proxies().patch(...).doit().await
//! let r = hub.target_http_proxies().set_url_map(...).doit().await
//! let r = hub.target_https_proxies().delete(...).doit().await
//! let r = hub.target_https_proxies().insert(...).doit().await
//! let r = hub.target_https_proxies().patch(...).doit().await
//! let r = hub.target_https_proxies().set_certificate_map(...).doit().await
//! let r = hub.target_https_proxies().set_quic_override(...).doit().await
//! let r = hub.target_https_proxies().set_ssl_certificates(...).doit().await
//! let r = hub.target_https_proxies().set_ssl_policy(...).doit().await
//! let r = hub.target_https_proxies().set_url_map(...).doit().await
//! let r = hub.target_instances().delete(...).doit().await
//! let r = hub.target_instances().insert(...).doit().await
//! let r = hub.target_instances().set_security_policy(...).doit().await
//! let r = hub.target_pools().add_health_check(...).doit().await
//! let r = hub.target_pools().add_instance(...).doit().await
//! let r = hub.target_pools().delete(...).doit().await
//! let r = hub.target_pools().insert(...).doit().await
//! let r = hub.target_pools().remove_health_check(...).doit().await
//! let r = hub.target_pools().remove_instance(...).doit().await
//! let r = hub.target_pools().set_backup(...).doit().await
//! let r = hub.target_pools().set_security_policy(...).doit().await
//! let r = hub.target_ssl_proxies().delete(...).doit().await
//! let r = hub.target_ssl_proxies().insert(...).doit().await
//! let r = hub.target_ssl_proxies().set_backend_service(...).doit().await
//! let r = hub.target_ssl_proxies().set_certificate_map(...).doit().await
//! let r = hub.target_ssl_proxies().set_proxy_header(...).doit().await
//! let r = hub.target_ssl_proxies().set_ssl_certificates(...).doit().await
//! let r = hub.target_ssl_proxies().set_ssl_policy(...).doit().await
//! let r = hub.target_tcp_proxies().delete(...).doit().await
//! let r = hub.target_tcp_proxies().insert(...).doit().await
//! let r = hub.target_tcp_proxies().set_backend_service(...).doit().await
//! let r = hub.target_tcp_proxies().set_proxy_header(...).doit().await
//! let r = hub.target_vpn_gateways().delete(...).doit().await
//! let r = hub.target_vpn_gateways().insert(...).doit().await
//! let r = hub.target_vpn_gateways().set_labels(...).doit().await
//! let r = hub.url_maps().delete(...).doit().await
//! let r = hub.url_maps().insert(...).doit().await
//! let r = hub.url_maps().invalidate_cache(...).doit().await
//! let r = hub.url_maps().patch(...).doit().await
//! let r = hub.url_maps().update(...).doit().await
//! let r = hub.vpn_gateways().delete(...).doit().await
//! let r = hub.vpn_gateways().insert(...).doit().await
//! let r = hub.vpn_gateways().set_labels(...).doit().await
//! let r = hub.vpn_tunnels().delete(...).doit().await
//! let r = hub.vpn_tunnels().insert(...).doit().await
//! let r = hub.vpn_tunnels().set_labels(...).doit().await
//! let r = hub.zone_operations().get(...).doit().await
//! let r = hub.zone_operations().wait(...).doit().await
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
//! serde = "^1.0"
//! serde_json = "^1.0"
//! ```
//! 
//! ## A complete example
//! 
//! ```test_harness,no_run
//! extern crate hyper;
//! extern crate hyper_rustls;
//! extern crate google_compute1 as compute1;
//! use compute1::api::Disk;
//! use compute1::{Result, Error};
//! # async fn dox() {
//! use std::default::Default;
//! use compute1::{Compute, oauth2, hyper, hyper_rustls, chrono, FieldMask};
//! 
//! // Get an ApplicationSecret instance by some means. It contains the `client_id` and 
//! // `client_secret`, among other things.
//! let secret: oauth2::ApplicationSecret = Default::default();
//! // Instantiate the authenticator. It will choose a suitable authentication flow for you, 
//! // unless you replace  `None` with the desired Flow.
//! // Provide your own `AuthenticatorDelegate` to adjust the way it operates and get feedback about 
//! // what's going on. You probably want to bring in your own `TokenStorage` to persist tokens and
//! // retrieve them from storage.
//! let auth = oauth2::InstalledFlowAuthenticator::builder(
//!         secret,
//!         oauth2::InstalledFlowReturnMethod::HTTPRedirect,
//!     ).build().await.unwrap();
//! let mut hub = Compute::new(hyper::Client::builder().build(hyper_rustls::HttpsConnectorBuilder::new().with_native_roots().unwrap().https_or_http().enable_http1().build()), auth);
//! // As the method needs a request, you would usually fill it with the desired information
//! // into the respective structure. Some of the parts shown here might not be applicable !
//! // Values shown here are possibly random and not representative !
//! let mut req = Disk::default();
//! 
//! // You can configure optional parameters by calling the respective setters at will, and
//! // execute the final call using `doit()`.
//! // Values shown here are possibly random and not representative !
//! let result = hub.disks().update(req, "project", "zone", "disk")
//!              .update_mask(FieldMask::new::<&str>(&[]))
//!              .request_id("amet.")
//!              .add_paths("takimata")
//!              .doit().await;
//! 
//! match result {
//!     Err(e) => match e {
//!         // The Error enum provides details about what exactly happened.
//!         // You can also just use its `Debug`, `Display` or `Error` traits
//!          Error::HttpError(_)
//!         |Error::Io(_)
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
//! All errors produced by the system are provided either as [Result](client::Result) enumeration as return value of
//! the doit() methods, or handed as possibly intermediate results to either the 
//! [Hub Delegate](client::Delegate), or the [Authenticator Delegate](https://docs.rs/yup-oauth2/*/yup_oauth2/trait.AuthenticatorDelegate.html).
//! 
//! When delegates handle errors or intermediate values, they may have a chance to instruct the system to retry. This 
//! makes the system potentially resilient to all kinds of errors.
//! 
//! ## Uploads and Downloads
//! If a method supports downloads, the response body, which is part of the [Result](client::Result), should be
//! read by you to obtain the media.
//! If such a method also supports a [Response Result](client::ResponseResult), it will return that by default.
//! You can see it as meta-data for the actual media. To trigger a media download, you will have to set up the builder by making
//! this call: `.param("alt", "media")`.
//! 
//! Methods supporting uploads can do so using up to 2 different protocols: 
//! *simple* and *resumable*. The distinctiveness of each is represented by customized 
//! `doit(...)` methods, which are then named `upload(...)` and `upload_resumable(...)` respectively.
//! 
//! ## Customization and Callbacks
//! 
//! You may alter the way an `doit()` method is called by providing a [delegate](client::Delegate) to the 
//! [Method Builder](client::CallBuilder) before making the final `doit()` call. 
//! Respective methods will be called to provide progress information, as well as determine whether the system should 
//! retry on failure.
//! 
//! The [delegate trait](client::Delegate) is default-implemented, allowing you to customize it with minimal effort.
//! 
//! ## Optional Parts in Server-Requests
//! 
//! All structures provided by this library are made to be [encodable](client::RequestValue) and 
//! [decodable](client::ResponseResult) via *json*. Optionals are used to indicate that partial requests are responses 
//! are valid.
//! Most optionals are are considered [Parts](client::Part) which are identifiable by name, which will be sent to 
//! the server to indicate either the set parts of the request or the desired parts in the response.
//! 
//! ## Builder Arguments
//! 
//! Using [method builders](client::CallBuilder), you are able to prepare an action call by repeatedly calling it's methods.
//! These will always take a single argument, for which the following statements are true.
//! 
//! * [PODs][wiki-pod] are handed by copy
//! * strings are passed as `&str`
//! * [request values](client::RequestValue) are moved
//! 
//! Arguments will always be copied or cloned into the builder, to make them independent of their original life times.
//! 
//! [wiki-pod]: http://en.wikipedia.org/wiki/Plain_old_data_structure
//! [builder-pattern]: http://en.wikipedia.org/wiki/Builder_pattern
//! [google-go-api]: https://github.com/google/google-api-go-client
//! 
//! ## Cargo Features
//! 
//! * `utoipa` - Add support for [utoipa](https://crates.io/crates/utoipa) and derive `utoipa::ToSchema` on all
//! the types. You'll have to import and register the required types in `#[openapi(schemas(...))]`, otherwise the
//! generated `openapi` spec would be invalid.
//! 
//! 
//! 

// Unused attributes happen thanks to defined, but unused structures
// We don't warn about this, as depending on the API, some data structures or facilities are never used.
// Instead of pre-determining this, we just disable the lint. It's manually tuned to not have any
// unused imports in fully featured APIs. Same with unused_mut ... .
#![allow(unused_imports, unused_mut, dead_code)]

// DO NOT EDIT !
// This file was generated automatically from 'src/generator/templates/api/lib.rs.mako'
// DO NOT EDIT !

// Re-export the hyper and hyper_rustls crate, they are required to build the hub
pub use hyper;
pub use hyper_rustls;
pub extern crate google_apis_common as client;
pub use client::chrono;
pub mod api;

// Re-export the hub type and some basic client structs
pub use api::Compute;
pub use client::{Result, Error, Delegate, FieldMask};

// Re-export the yup_oauth2 crate, that is required to call some methods of the hub and the client
#[cfg(feature = "yup-oauth2")]
pub use client::oauth2;