<!---
DO NOT EDIT !
This file was generated automatically from 'src/mako/api/README.md.mako'
DO NOT EDIT !
-->
The `google-compute1` library allows access to all features of the *Google compute* service.

This documentation was generated from *compute* crate version *1.0.8+20190320*, where *20190320* is the exact revision of the *compute:v1* schema built by the [mako](http://www.makotemplates.org/) code generator *v1.0.8*.

Everything else about the *compute* *v1* API can be found at the
[official documentation site](https://developers.google.com/compute/docs/reference/latest/).
# Features

Handle the following *Resources* with ease from the central [hub](https://docs.rs/google-compute1/1.0.8+20190320/google_compute1/struct.Compute.html) ... 

* [accelerator types](https://docs.rs/google-compute1/1.0.8+20190320/google_compute1/struct.AcceleratorType.html)
 * [*aggregated list*](https://docs.rs/google-compute1/1.0.8+20190320/google_compute1/struct.AcceleratorTypeAggregatedListCall.html), [*get*](https://docs.rs/google-compute1/1.0.8+20190320/google_compute1/struct.AcceleratorTypeGetCall.html) and [*list*](https://docs.rs/google-compute1/1.0.8+20190320/google_compute1/struct.AcceleratorTypeListCall.html)
* addresses
 * [*aggregated list*](https://docs.rs/google-compute1/1.0.8+20190320/google_compute1/struct.AddresseAggregatedListCall.html), [*delete*](https://docs.rs/google-compute1/1.0.8+20190320/google_compute1/struct.AddresseDeleteCall.html), [*get*](https://docs.rs/google-compute1/1.0.8+20190320/google_compute1/struct.AddresseGetCall.html), [*insert*](https://docs.rs/google-compute1/1.0.8+20190320/google_compute1/struct.AddresseInsertCall.html) and [*list*](https://docs.rs/google-compute1/1.0.8+20190320/google_compute1/struct.AddresseListCall.html)
* [autoscalers](https://docs.rs/google-compute1/1.0.8+20190320/google_compute1/struct.Autoscaler.html)
 * [*aggregated list*](https://docs.rs/google-compute1/1.0.8+20190320/google_compute1/struct.AutoscalerAggregatedListCall.html), [*delete*](https://docs.rs/google-compute1/1.0.8+20190320/google_compute1/struct.AutoscalerDeleteCall.html), [*get*](https://docs.rs/google-compute1/1.0.8+20190320/google_compute1/struct.AutoscalerGetCall.html), [*insert*](https://docs.rs/google-compute1/1.0.8+20190320/google_compute1/struct.AutoscalerInsertCall.html), [*list*](https://docs.rs/google-compute1/1.0.8+20190320/google_compute1/struct.AutoscalerListCall.html), [*patch*](https://docs.rs/google-compute1/1.0.8+20190320/google_compute1/struct.AutoscalerPatchCall.html) and [*update*](https://docs.rs/google-compute1/1.0.8+20190320/google_compute1/struct.AutoscalerUpdateCall.html)
* [backend buckets](https://docs.rs/google-compute1/1.0.8+20190320/google_compute1/struct.BackendBucket.html)
 * [*add signed url key*](https://docs.rs/google-compute1/1.0.8+20190320/google_compute1/struct.BackendBucketAddSignedUrlKeyCall.html), [*delete*](https://docs.rs/google-compute1/1.0.8+20190320/google_compute1/struct.BackendBucketDeleteCall.html), [*delete signed url key*](https://docs.rs/google-compute1/1.0.8+20190320/google_compute1/struct.BackendBucketDeleteSignedUrlKeyCall.html), [*get*](https://docs.rs/google-compute1/1.0.8+20190320/google_compute1/struct.BackendBucketGetCall.html), [*insert*](https://docs.rs/google-compute1/1.0.8+20190320/google_compute1/struct.BackendBucketInsertCall.html), [*list*](https://docs.rs/google-compute1/1.0.8+20190320/google_compute1/struct.BackendBucketListCall.html), [*patch*](https://docs.rs/google-compute1/1.0.8+20190320/google_compute1/struct.BackendBucketPatchCall.html) and [*update*](https://docs.rs/google-compute1/1.0.8+20190320/google_compute1/struct.BackendBucketUpdateCall.html)
* [backend services](https://docs.rs/google-compute1/1.0.8+20190320/google_compute1/struct.BackendService.html)
 * [*add signed url key*](https://docs.rs/google-compute1/1.0.8+20190320/google_compute1/struct.BackendServiceAddSignedUrlKeyCall.html), [*aggregated list*](https://docs.rs/google-compute1/1.0.8+20190320/google_compute1/struct.BackendServiceAggregatedListCall.html), [*delete*](https://docs.rs/google-compute1/1.0.8+20190320/google_compute1/struct.BackendServiceDeleteCall.html), [*delete signed url key*](https://docs.rs/google-compute1/1.0.8+20190320/google_compute1/struct.BackendServiceDeleteSignedUrlKeyCall.html), [*get*](https://docs.rs/google-compute1/1.0.8+20190320/google_compute1/struct.BackendServiceGetCall.html), [*get health*](https://docs.rs/google-compute1/1.0.8+20190320/google_compute1/struct.BackendServiceGetHealthCall.html), [*insert*](https://docs.rs/google-compute1/1.0.8+20190320/google_compute1/struct.BackendServiceInsertCall.html), [*list*](https://docs.rs/google-compute1/1.0.8+20190320/google_compute1/struct.BackendServiceListCall.html), [*patch*](https://docs.rs/google-compute1/1.0.8+20190320/google_compute1/struct.BackendServicePatchCall.html), [*set security policy*](https://docs.rs/google-compute1/1.0.8+20190320/google_compute1/struct.BackendServiceSetSecurityPolicyCall.html) and [*update*](https://docs.rs/google-compute1/1.0.8+20190320/google_compute1/struct.BackendServiceUpdateCall.html)
* [disk types](https://docs.rs/google-compute1/1.0.8+20190320/google_compute1/struct.DiskType.html)
 * [*aggregated list*](https://docs.rs/google-compute1/1.0.8+20190320/google_compute1/struct.DiskTypeAggregatedListCall.html), [*get*](https://docs.rs/google-compute1/1.0.8+20190320/google_compute1/struct.DiskTypeGetCall.html) and [*list*](https://docs.rs/google-compute1/1.0.8+20190320/google_compute1/struct.DiskTypeListCall.html)
* [disks](https://docs.rs/google-compute1/1.0.8+20190320/google_compute1/struct.Disk.html)
 * [*aggregated list*](https://docs.rs/google-compute1/1.0.8+20190320/google_compute1/struct.DiskAggregatedListCall.html), [*create snapshot*](https://docs.rs/google-compute1/1.0.8+20190320/google_compute1/struct.DiskCreateSnapshotCall.html), [*delete*](https://docs.rs/google-compute1/1.0.8+20190320/google_compute1/struct.DiskDeleteCall.html), [*get*](https://docs.rs/google-compute1/1.0.8+20190320/google_compute1/struct.DiskGetCall.html), [*get iam policy*](https://docs.rs/google-compute1/1.0.8+20190320/google_compute1/struct.DiskGetIamPolicyCall.html), [*insert*](https://docs.rs/google-compute1/1.0.8+20190320/google_compute1/struct.DiskInsertCall.html), [*list*](https://docs.rs/google-compute1/1.0.8+20190320/google_compute1/struct.DiskListCall.html), [*resize*](https://docs.rs/google-compute1/1.0.8+20190320/google_compute1/struct.DiskResizeCall.html), [*set iam policy*](https://docs.rs/google-compute1/1.0.8+20190320/google_compute1/struct.DiskSetIamPolicyCall.html), [*set labels*](https://docs.rs/google-compute1/1.0.8+20190320/google_compute1/struct.DiskSetLabelCall.html) and [*test iam permissions*](https://docs.rs/google-compute1/1.0.8+20190320/google_compute1/struct.DiskTestIamPermissionCall.html)
* [firewalls](https://docs.rs/google-compute1/1.0.8+20190320/google_compute1/struct.Firewall.html)
 * [*delete*](https://docs.rs/google-compute1/1.0.8+20190320/google_compute1/struct.FirewallDeleteCall.html), [*get*](https://docs.rs/google-compute1/1.0.8+20190320/google_compute1/struct.FirewallGetCall.html), [*insert*](https://docs.rs/google-compute1/1.0.8+20190320/google_compute1/struct.FirewallInsertCall.html), [*list*](https://docs.rs/google-compute1/1.0.8+20190320/google_compute1/struct.FirewallListCall.html), [*patch*](https://docs.rs/google-compute1/1.0.8+20190320/google_compute1/struct.FirewallPatchCall.html) and [*update*](https://docs.rs/google-compute1/1.0.8+20190320/google_compute1/struct.FirewallUpdateCall.html)
* [forwarding rules](https://docs.rs/google-compute1/1.0.8+20190320/google_compute1/struct.ForwardingRule.html)
 * [*aggregated list*](https://docs.rs/google-compute1/1.0.8+20190320/google_compute1/struct.ForwardingRuleAggregatedListCall.html), [*delete*](https://docs.rs/google-compute1/1.0.8+20190320/google_compute1/struct.ForwardingRuleDeleteCall.html), [*get*](https://docs.rs/google-compute1/1.0.8+20190320/google_compute1/struct.ForwardingRuleGetCall.html), [*insert*](https://docs.rs/google-compute1/1.0.8+20190320/google_compute1/struct.ForwardingRuleInsertCall.html), [*list*](https://docs.rs/google-compute1/1.0.8+20190320/google_compute1/struct.ForwardingRuleListCall.html) and [*set target*](https://docs.rs/google-compute1/1.0.8+20190320/google_compute1/struct.ForwardingRuleSetTargetCall.html)
* global addresses
 * [*delete*](https://docs.rs/google-compute1/1.0.8+20190320/google_compute1/struct.GlobalAddresseDeleteCall.html), [*get*](https://docs.rs/google-compute1/1.0.8+20190320/google_compute1/struct.GlobalAddresseGetCall.html), [*insert*](https://docs.rs/google-compute1/1.0.8+20190320/google_compute1/struct.GlobalAddresseInsertCall.html) and [*list*](https://docs.rs/google-compute1/1.0.8+20190320/google_compute1/struct.GlobalAddresseListCall.html)
* global forwarding rules
 * [*delete*](https://docs.rs/google-compute1/1.0.8+20190320/google_compute1/struct.GlobalForwardingRuleDeleteCall.html), [*get*](https://docs.rs/google-compute1/1.0.8+20190320/google_compute1/struct.GlobalForwardingRuleGetCall.html), [*insert*](https://docs.rs/google-compute1/1.0.8+20190320/google_compute1/struct.GlobalForwardingRuleInsertCall.html), [*list*](https://docs.rs/google-compute1/1.0.8+20190320/google_compute1/struct.GlobalForwardingRuleListCall.html) and [*set target*](https://docs.rs/google-compute1/1.0.8+20190320/google_compute1/struct.GlobalForwardingRuleSetTargetCall.html)
* global operations
 * [*aggregated list*](https://docs.rs/google-compute1/1.0.8+20190320/google_compute1/struct.GlobalOperationAggregatedListCall.html), [*delete*](https://docs.rs/google-compute1/1.0.8+20190320/google_compute1/struct.GlobalOperationDeleteCall.html), [*get*](https://docs.rs/google-compute1/1.0.8+20190320/google_compute1/struct.GlobalOperationGetCall.html) and [*list*](https://docs.rs/google-compute1/1.0.8+20190320/google_compute1/struct.GlobalOperationListCall.html)
* [health checks](https://docs.rs/google-compute1/1.0.8+20190320/google_compute1/struct.HealthCheck.html)
 * [*delete*](https://docs.rs/google-compute1/1.0.8+20190320/google_compute1/struct.HealthCheckDeleteCall.html), [*get*](https://docs.rs/google-compute1/1.0.8+20190320/google_compute1/struct.HealthCheckGetCall.html), [*insert*](https://docs.rs/google-compute1/1.0.8+20190320/google_compute1/struct.HealthCheckInsertCall.html), [*list*](https://docs.rs/google-compute1/1.0.8+20190320/google_compute1/struct.HealthCheckListCall.html), [*patch*](https://docs.rs/google-compute1/1.0.8+20190320/google_compute1/struct.HealthCheckPatchCall.html) and [*update*](https://docs.rs/google-compute1/1.0.8+20190320/google_compute1/struct.HealthCheckUpdateCall.html)
* [http health checks](https://docs.rs/google-compute1/1.0.8+20190320/google_compute1/struct.HttpHealthCheck.html)
 * [*delete*](https://docs.rs/google-compute1/1.0.8+20190320/google_compute1/struct.HttpHealthCheckDeleteCall.html), [*get*](https://docs.rs/google-compute1/1.0.8+20190320/google_compute1/struct.HttpHealthCheckGetCall.html), [*insert*](https://docs.rs/google-compute1/1.0.8+20190320/google_compute1/struct.HttpHealthCheckInsertCall.html), [*list*](https://docs.rs/google-compute1/1.0.8+20190320/google_compute1/struct.HttpHealthCheckListCall.html), [*patch*](https://docs.rs/google-compute1/1.0.8+20190320/google_compute1/struct.HttpHealthCheckPatchCall.html) and [*update*](https://docs.rs/google-compute1/1.0.8+20190320/google_compute1/struct.HttpHealthCheckUpdateCall.html)
* [https health checks](https://docs.rs/google-compute1/1.0.8+20190320/google_compute1/struct.HttpsHealthCheck.html)
 * [*delete*](https://docs.rs/google-compute1/1.0.8+20190320/google_compute1/struct.HttpsHealthCheckDeleteCall.html), [*get*](https://docs.rs/google-compute1/1.0.8+20190320/google_compute1/struct.HttpsHealthCheckGetCall.html), [*insert*](https://docs.rs/google-compute1/1.0.8+20190320/google_compute1/struct.HttpsHealthCheckInsertCall.html), [*list*](https://docs.rs/google-compute1/1.0.8+20190320/google_compute1/struct.HttpsHealthCheckListCall.html), [*patch*](https://docs.rs/google-compute1/1.0.8+20190320/google_compute1/struct.HttpsHealthCheckPatchCall.html) and [*update*](https://docs.rs/google-compute1/1.0.8+20190320/google_compute1/struct.HttpsHealthCheckUpdateCall.html)
* [images](https://docs.rs/google-compute1/1.0.8+20190320/google_compute1/struct.Image.html)
 * [*delete*](https://docs.rs/google-compute1/1.0.8+20190320/google_compute1/struct.ImageDeleteCall.html), [*deprecate*](https://docs.rs/google-compute1/1.0.8+20190320/google_compute1/struct.ImageDeprecateCall.html), [*get*](https://docs.rs/google-compute1/1.0.8+20190320/google_compute1/struct.ImageGetCall.html), [*get from family*](https://docs.rs/google-compute1/1.0.8+20190320/google_compute1/struct.ImageGetFromFamilyCall.html), [*get iam policy*](https://docs.rs/google-compute1/1.0.8+20190320/google_compute1/struct.ImageGetIamPolicyCall.html), [*insert*](https://docs.rs/google-compute1/1.0.8+20190320/google_compute1/struct.ImageInsertCall.html), [*list*](https://docs.rs/google-compute1/1.0.8+20190320/google_compute1/struct.ImageListCall.html), [*set iam policy*](https://docs.rs/google-compute1/1.0.8+20190320/google_compute1/struct.ImageSetIamPolicyCall.html), [*set labels*](https://docs.rs/google-compute1/1.0.8+20190320/google_compute1/struct.ImageSetLabelCall.html) and [*test iam permissions*](https://docs.rs/google-compute1/1.0.8+20190320/google_compute1/struct.ImageTestIamPermissionCall.html)
* [instance group managers](https://docs.rs/google-compute1/1.0.8+20190320/google_compute1/struct.InstanceGroupManager.html)
 * [*abandon instances*](https://docs.rs/google-compute1/1.0.8+20190320/google_compute1/struct.InstanceGroupManagerAbandonInstanceCall.html), [*aggregated list*](https://docs.rs/google-compute1/1.0.8+20190320/google_compute1/struct.InstanceGroupManagerAggregatedListCall.html), [*delete*](https://docs.rs/google-compute1/1.0.8+20190320/google_compute1/struct.InstanceGroupManagerDeleteCall.html), [*delete instances*](https://docs.rs/google-compute1/1.0.8+20190320/google_compute1/struct.InstanceGroupManagerDeleteInstanceCall.html), [*get*](https://docs.rs/google-compute1/1.0.8+20190320/google_compute1/struct.InstanceGroupManagerGetCall.html), [*insert*](https://docs.rs/google-compute1/1.0.8+20190320/google_compute1/struct.InstanceGroupManagerInsertCall.html), [*list*](https://docs.rs/google-compute1/1.0.8+20190320/google_compute1/struct.InstanceGroupManagerListCall.html), [*list managed instances*](https://docs.rs/google-compute1/1.0.8+20190320/google_compute1/struct.InstanceGroupManagerListManagedInstanceCall.html), [*patch*](https://docs.rs/google-compute1/1.0.8+20190320/google_compute1/struct.InstanceGroupManagerPatchCall.html), [*recreate instances*](https://docs.rs/google-compute1/1.0.8+20190320/google_compute1/struct.InstanceGroupManagerRecreateInstanceCall.html), [*resize*](https://docs.rs/google-compute1/1.0.8+20190320/google_compute1/struct.InstanceGroupManagerResizeCall.html), [*set instance template*](https://docs.rs/google-compute1/1.0.8+20190320/google_compute1/struct.InstanceGroupManagerSetInstanceTemplateCall.html) and [*set target pools*](https://docs.rs/google-compute1/1.0.8+20190320/google_compute1/struct.InstanceGroupManagerSetTargetPoolCall.html)
* [instance groups](https://docs.rs/google-compute1/1.0.8+20190320/google_compute1/struct.InstanceGroup.html)
 * [*add instances*](https://docs.rs/google-compute1/1.0.8+20190320/google_compute1/struct.InstanceGroupAddInstanceCall.html), [*aggregated list*](https://docs.rs/google-compute1/1.0.8+20190320/google_compute1/struct.InstanceGroupAggregatedListCall.html), [*delete*](https://docs.rs/google-compute1/1.0.8+20190320/google_compute1/struct.InstanceGroupDeleteCall.html), [*get*](https://docs.rs/google-compute1/1.0.8+20190320/google_compute1/struct.InstanceGroupGetCall.html), [*insert*](https://docs.rs/google-compute1/1.0.8+20190320/google_compute1/struct.InstanceGroupInsertCall.html), [*list*](https://docs.rs/google-compute1/1.0.8+20190320/google_compute1/struct.InstanceGroupListCall.html), [*list instances*](https://docs.rs/google-compute1/1.0.8+20190320/google_compute1/struct.InstanceGroupListInstanceCall.html), [*remove instances*](https://docs.rs/google-compute1/1.0.8+20190320/google_compute1/struct.InstanceGroupRemoveInstanceCall.html) and [*set named ports*](https://docs.rs/google-compute1/1.0.8+20190320/google_compute1/struct.InstanceGroupSetNamedPortCall.html)
* [instance templates](https://docs.rs/google-compute1/1.0.8+20190320/google_compute1/struct.InstanceTemplate.html)
 * [*delete*](https://docs.rs/google-compute1/1.0.8+20190320/google_compute1/struct.InstanceTemplateDeleteCall.html), [*get*](https://docs.rs/google-compute1/1.0.8+20190320/google_compute1/struct.InstanceTemplateGetCall.html), [*get iam policy*](https://docs.rs/google-compute1/1.0.8+20190320/google_compute1/struct.InstanceTemplateGetIamPolicyCall.html), [*insert*](https://docs.rs/google-compute1/1.0.8+20190320/google_compute1/struct.InstanceTemplateInsertCall.html), [*list*](https://docs.rs/google-compute1/1.0.8+20190320/google_compute1/struct.InstanceTemplateListCall.html), [*set iam policy*](https://docs.rs/google-compute1/1.0.8+20190320/google_compute1/struct.InstanceTemplateSetIamPolicyCall.html) and [*test iam permissions*](https://docs.rs/google-compute1/1.0.8+20190320/google_compute1/struct.InstanceTemplateTestIamPermissionCall.html)
* [instances](https://docs.rs/google-compute1/1.0.8+20190320/google_compute1/struct.Instance.html)
 * [*add access config*](https://docs.rs/google-compute1/1.0.8+20190320/google_compute1/struct.InstanceAddAccessConfigCall.html), [*aggregated list*](https://docs.rs/google-compute1/1.0.8+20190320/google_compute1/struct.InstanceAggregatedListCall.html), [*attach disk*](https://docs.rs/google-compute1/1.0.8+20190320/google_compute1/struct.InstanceAttachDiskCall.html), [*delete*](https://docs.rs/google-compute1/1.0.8+20190320/google_compute1/struct.InstanceDeleteCall.html), [*delete access config*](https://docs.rs/google-compute1/1.0.8+20190320/google_compute1/struct.InstanceDeleteAccessConfigCall.html), [*detach disk*](https://docs.rs/google-compute1/1.0.8+20190320/google_compute1/struct.InstanceDetachDiskCall.html), [*get*](https://docs.rs/google-compute1/1.0.8+20190320/google_compute1/struct.InstanceGetCall.html), [*get iam policy*](https://docs.rs/google-compute1/1.0.8+20190320/google_compute1/struct.InstanceGetIamPolicyCall.html), [*get serial port output*](https://docs.rs/google-compute1/1.0.8+20190320/google_compute1/struct.InstanceGetSerialPortOutputCall.html), [*get shielded instance identity*](https://docs.rs/google-compute1/1.0.8+20190320/google_compute1/struct.InstanceGetShieldedInstanceIdentityCall.html), [*insert*](https://docs.rs/google-compute1/1.0.8+20190320/google_compute1/struct.InstanceInsertCall.html), [*list*](https://docs.rs/google-compute1/1.0.8+20190320/google_compute1/struct.InstanceListCall.html), [*list referrers*](https://docs.rs/google-compute1/1.0.8+20190320/google_compute1/struct.InstanceListReferrerCall.html), [*reset*](https://docs.rs/google-compute1/1.0.8+20190320/google_compute1/struct.InstanceResetCall.html), [*set deletion protection*](https://docs.rs/google-compute1/1.0.8+20190320/google_compute1/struct.InstanceSetDeletionProtectionCall.html), [*set disk auto delete*](https://docs.rs/google-compute1/1.0.8+20190320/google_compute1/struct.InstanceSetDiskAutoDeleteCall.html), [*set iam policy*](https://docs.rs/google-compute1/1.0.8+20190320/google_compute1/struct.InstanceSetIamPolicyCall.html), [*set labels*](https://docs.rs/google-compute1/1.0.8+20190320/google_compute1/struct.InstanceSetLabelCall.html), [*set machine resources*](https://docs.rs/google-compute1/1.0.8+20190320/google_compute1/struct.InstanceSetMachineResourceCall.html), [*set machine type*](https://docs.rs/google-compute1/1.0.8+20190320/google_compute1/struct.InstanceSetMachineTypeCall.html), [*set metadata*](https://docs.rs/google-compute1/1.0.8+20190320/google_compute1/struct.InstanceSetMetadataCall.html), [*set min cpu platform*](https://docs.rs/google-compute1/1.0.8+20190320/google_compute1/struct.InstanceSetMinCpuPlatformCall.html), [*set scheduling*](https://docs.rs/google-compute1/1.0.8+20190320/google_compute1/struct.InstanceSetSchedulingCall.html), [*set service account*](https://docs.rs/google-compute1/1.0.8+20190320/google_compute1/struct.InstanceSetServiceAccountCall.html), [*set shielded instance integrity policy*](https://docs.rs/google-compute1/1.0.8+20190320/google_compute1/struct.InstanceSetShieldedInstanceIntegrityPolicyCall.html), [*set tags*](https://docs.rs/google-compute1/1.0.8+20190320/google_compute1/struct.InstanceSetTagCall.html), [*simulate maintenance event*](https://docs.rs/google-compute1/1.0.8+20190320/google_compute1/struct.InstanceSimulateMaintenanceEventCall.html), [*start*](https://docs.rs/google-compute1/1.0.8+20190320/google_compute1/struct.InstanceStartCall.html), [*start with encryption key*](https://docs.rs/google-compute1/1.0.8+20190320/google_compute1/struct.InstanceStartWithEncryptionKeyCall.html), [*stop*](https://docs.rs/google-compute1/1.0.8+20190320/google_compute1/struct.InstanceStopCall.html), [*test iam permissions*](https://docs.rs/google-compute1/1.0.8+20190320/google_compute1/struct.InstanceTestIamPermissionCall.html), [*update access config*](https://docs.rs/google-compute1/1.0.8+20190320/google_compute1/struct.InstanceUpdateAccessConfigCall.html), [*update network interface*](https://docs.rs/google-compute1/1.0.8+20190320/google_compute1/struct.InstanceUpdateNetworkInterfaceCall.html) and [*update shielded instance config*](https://docs.rs/google-compute1/1.0.8+20190320/google_compute1/struct.InstanceUpdateShieldedInstanceConfigCall.html)
* [interconnect attachments](https://docs.rs/google-compute1/1.0.8+20190320/google_compute1/struct.InterconnectAttachment.html)
 * [*aggregated list*](https://docs.rs/google-compute1/1.0.8+20190320/google_compute1/struct.InterconnectAttachmentAggregatedListCall.html), [*delete*](https://docs.rs/google-compute1/1.0.8+20190320/google_compute1/struct.InterconnectAttachmentDeleteCall.html), [*get*](https://docs.rs/google-compute1/1.0.8+20190320/google_compute1/struct.InterconnectAttachmentGetCall.html), [*insert*](https://docs.rs/google-compute1/1.0.8+20190320/google_compute1/struct.InterconnectAttachmentInsertCall.html), [*list*](https://docs.rs/google-compute1/1.0.8+20190320/google_compute1/struct.InterconnectAttachmentListCall.html) and [*patch*](https://docs.rs/google-compute1/1.0.8+20190320/google_compute1/struct.InterconnectAttachmentPatchCall.html)
* [interconnect locations](https://docs.rs/google-compute1/1.0.8+20190320/google_compute1/struct.InterconnectLocation.html)
 * [*get*](https://docs.rs/google-compute1/1.0.8+20190320/google_compute1/struct.InterconnectLocationGetCall.html) and [*list*](https://docs.rs/google-compute1/1.0.8+20190320/google_compute1/struct.InterconnectLocationListCall.html)
* [interconnects](https://docs.rs/google-compute1/1.0.8+20190320/google_compute1/struct.Interconnect.html)
 * [*delete*](https://docs.rs/google-compute1/1.0.8+20190320/google_compute1/struct.InterconnectDeleteCall.html), [*get*](https://docs.rs/google-compute1/1.0.8+20190320/google_compute1/struct.InterconnectGetCall.html), [*get diagnostics*](https://docs.rs/google-compute1/1.0.8+20190320/google_compute1/struct.InterconnectGetDiagnosticCall.html), [*insert*](https://docs.rs/google-compute1/1.0.8+20190320/google_compute1/struct.InterconnectInsertCall.html), [*list*](https://docs.rs/google-compute1/1.0.8+20190320/google_compute1/struct.InterconnectListCall.html) and [*patch*](https://docs.rs/google-compute1/1.0.8+20190320/google_compute1/struct.InterconnectPatchCall.html)
* [license codes](https://docs.rs/google-compute1/1.0.8+20190320/google_compute1/struct.LicenseCode.html)
 * [*get*](https://docs.rs/google-compute1/1.0.8+20190320/google_compute1/struct.LicenseCodeGetCall.html) and [*test iam permissions*](https://docs.rs/google-compute1/1.0.8+20190320/google_compute1/struct.LicenseCodeTestIamPermissionCall.html)
* [licenses](https://docs.rs/google-compute1/1.0.8+20190320/google_compute1/struct.License.html)
 * [*delete*](https://docs.rs/google-compute1/1.0.8+20190320/google_compute1/struct.LicenseDeleteCall.html), [*get*](https://docs.rs/google-compute1/1.0.8+20190320/google_compute1/struct.LicenseGetCall.html), [*get iam policy*](https://docs.rs/google-compute1/1.0.8+20190320/google_compute1/struct.LicenseGetIamPolicyCall.html), [*insert*](https://docs.rs/google-compute1/1.0.8+20190320/google_compute1/struct.LicenseInsertCall.html), [*list*](https://docs.rs/google-compute1/1.0.8+20190320/google_compute1/struct.LicenseListCall.html), [*set iam policy*](https://docs.rs/google-compute1/1.0.8+20190320/google_compute1/struct.LicenseSetIamPolicyCall.html) and [*test iam permissions*](https://docs.rs/google-compute1/1.0.8+20190320/google_compute1/struct.LicenseTestIamPermissionCall.html)
* [machine types](https://docs.rs/google-compute1/1.0.8+20190320/google_compute1/struct.MachineType.html)
 * [*aggregated list*](https://docs.rs/google-compute1/1.0.8+20190320/google_compute1/struct.MachineTypeAggregatedListCall.html), [*get*](https://docs.rs/google-compute1/1.0.8+20190320/google_compute1/struct.MachineTypeGetCall.html) and [*list*](https://docs.rs/google-compute1/1.0.8+20190320/google_compute1/struct.MachineTypeListCall.html)
* [network endpoint groups](https://docs.rs/google-compute1/1.0.8+20190320/google_compute1/struct.NetworkEndpointGroup.html)
 * [*aggregated list*](https://docs.rs/google-compute1/1.0.8+20190320/google_compute1/struct.NetworkEndpointGroupAggregatedListCall.html), [*attach network endpoints*](https://docs.rs/google-compute1/1.0.8+20190320/google_compute1/struct.NetworkEndpointGroupAttachNetworkEndpointCall.html), [*delete*](https://docs.rs/google-compute1/1.0.8+20190320/google_compute1/struct.NetworkEndpointGroupDeleteCall.html), [*detach network endpoints*](https://docs.rs/google-compute1/1.0.8+20190320/google_compute1/struct.NetworkEndpointGroupDetachNetworkEndpointCall.html), [*get*](https://docs.rs/google-compute1/1.0.8+20190320/google_compute1/struct.NetworkEndpointGroupGetCall.html), [*insert*](https://docs.rs/google-compute1/1.0.8+20190320/google_compute1/struct.NetworkEndpointGroupInsertCall.html), [*list*](https://docs.rs/google-compute1/1.0.8+20190320/google_compute1/struct.NetworkEndpointGroupListCall.html), [*list network endpoints*](https://docs.rs/google-compute1/1.0.8+20190320/google_compute1/struct.NetworkEndpointGroupListNetworkEndpointCall.html) and [*test iam permissions*](https://docs.rs/google-compute1/1.0.8+20190320/google_compute1/struct.NetworkEndpointGroupTestIamPermissionCall.html)
* [networks](https://docs.rs/google-compute1/1.0.8+20190320/google_compute1/struct.Network.html)
 * [*add peering*](https://docs.rs/google-compute1/1.0.8+20190320/google_compute1/struct.NetworkAddPeeringCall.html), [*delete*](https://docs.rs/google-compute1/1.0.8+20190320/google_compute1/struct.NetworkDeleteCall.html), [*get*](https://docs.rs/google-compute1/1.0.8+20190320/google_compute1/struct.NetworkGetCall.html), [*insert*](https://docs.rs/google-compute1/1.0.8+20190320/google_compute1/struct.NetworkInsertCall.html), [*list*](https://docs.rs/google-compute1/1.0.8+20190320/google_compute1/struct.NetworkListCall.html), [*patch*](https://docs.rs/google-compute1/1.0.8+20190320/google_compute1/struct.NetworkPatchCall.html), [*remove peering*](https://docs.rs/google-compute1/1.0.8+20190320/google_compute1/struct.NetworkRemovePeeringCall.html) and [*switch to custom mode*](https://docs.rs/google-compute1/1.0.8+20190320/google_compute1/struct.NetworkSwitchToCustomModeCall.html)
* [node groups](https://docs.rs/google-compute1/1.0.8+20190320/google_compute1/struct.NodeGroup.html)
 * [*add nodes*](https://docs.rs/google-compute1/1.0.8+20190320/google_compute1/struct.NodeGroupAddNodeCall.html), [*aggregated list*](https://docs.rs/google-compute1/1.0.8+20190320/google_compute1/struct.NodeGroupAggregatedListCall.html), [*delete*](https://docs.rs/google-compute1/1.0.8+20190320/google_compute1/struct.NodeGroupDeleteCall.html), [*delete nodes*](https://docs.rs/google-compute1/1.0.8+20190320/google_compute1/struct.NodeGroupDeleteNodeCall.html), [*get*](https://docs.rs/google-compute1/1.0.8+20190320/google_compute1/struct.NodeGroupGetCall.html), [*get iam policy*](https://docs.rs/google-compute1/1.0.8+20190320/google_compute1/struct.NodeGroupGetIamPolicyCall.html), [*insert*](https://docs.rs/google-compute1/1.0.8+20190320/google_compute1/struct.NodeGroupInsertCall.html), [*list*](https://docs.rs/google-compute1/1.0.8+20190320/google_compute1/struct.NodeGroupListCall.html), [*list nodes*](https://docs.rs/google-compute1/1.0.8+20190320/google_compute1/struct.NodeGroupListNodeCall.html), [*set iam policy*](https://docs.rs/google-compute1/1.0.8+20190320/google_compute1/struct.NodeGroupSetIamPolicyCall.html), [*set node template*](https://docs.rs/google-compute1/1.0.8+20190320/google_compute1/struct.NodeGroupSetNodeTemplateCall.html) and [*test iam permissions*](https://docs.rs/google-compute1/1.0.8+20190320/google_compute1/struct.NodeGroupTestIamPermissionCall.html)
* [node templates](https://docs.rs/google-compute1/1.0.8+20190320/google_compute1/struct.NodeTemplate.html)
 * [*aggregated list*](https://docs.rs/google-compute1/1.0.8+20190320/google_compute1/struct.NodeTemplateAggregatedListCall.html), [*delete*](https://docs.rs/google-compute1/1.0.8+20190320/google_compute1/struct.NodeTemplateDeleteCall.html), [*get*](https://docs.rs/google-compute1/1.0.8+20190320/google_compute1/struct.NodeTemplateGetCall.html), [*get iam policy*](https://docs.rs/google-compute1/1.0.8+20190320/google_compute1/struct.NodeTemplateGetIamPolicyCall.html), [*insert*](https://docs.rs/google-compute1/1.0.8+20190320/google_compute1/struct.NodeTemplateInsertCall.html), [*list*](https://docs.rs/google-compute1/1.0.8+20190320/google_compute1/struct.NodeTemplateListCall.html), [*set iam policy*](https://docs.rs/google-compute1/1.0.8+20190320/google_compute1/struct.NodeTemplateSetIamPolicyCall.html) and [*test iam permissions*](https://docs.rs/google-compute1/1.0.8+20190320/google_compute1/struct.NodeTemplateTestIamPermissionCall.html)
* [node types](https://docs.rs/google-compute1/1.0.8+20190320/google_compute1/struct.NodeType.html)
 * [*aggregated list*](https://docs.rs/google-compute1/1.0.8+20190320/google_compute1/struct.NodeTypeAggregatedListCall.html), [*get*](https://docs.rs/google-compute1/1.0.8+20190320/google_compute1/struct.NodeTypeGetCall.html) and [*list*](https://docs.rs/google-compute1/1.0.8+20190320/google_compute1/struct.NodeTypeListCall.html)
* [projects](https://docs.rs/google-compute1/1.0.8+20190320/google_compute1/struct.Project.html)
 * [*disable xpn host*](https://docs.rs/google-compute1/1.0.8+20190320/google_compute1/struct.ProjectDisableXpnHostCall.html), [*disable xpn resource*](https://docs.rs/google-compute1/1.0.8+20190320/google_compute1/struct.ProjectDisableXpnResourceCall.html), [*enable xpn host*](https://docs.rs/google-compute1/1.0.8+20190320/google_compute1/struct.ProjectEnableXpnHostCall.html), [*enable xpn resource*](https://docs.rs/google-compute1/1.0.8+20190320/google_compute1/struct.ProjectEnableXpnResourceCall.html), [*get*](https://docs.rs/google-compute1/1.0.8+20190320/google_compute1/struct.ProjectGetCall.html), [*get xpn host*](https://docs.rs/google-compute1/1.0.8+20190320/google_compute1/struct.ProjectGetXpnHostCall.html), [*get xpn resources*](https://docs.rs/google-compute1/1.0.8+20190320/google_compute1/struct.ProjectGetXpnResourceCall.html), [*list xpn hosts*](https://docs.rs/google-compute1/1.0.8+20190320/google_compute1/struct.ProjectListXpnHostCall.html), [*move disk*](https://docs.rs/google-compute1/1.0.8+20190320/google_compute1/struct.ProjectMoveDiskCall.html), [*move instance*](https://docs.rs/google-compute1/1.0.8+20190320/google_compute1/struct.ProjectMoveInstanceCall.html), [*set common instance metadata*](https://docs.rs/google-compute1/1.0.8+20190320/google_compute1/struct.ProjectSetCommonInstanceMetadataCall.html), [*set default network tier*](https://docs.rs/google-compute1/1.0.8+20190320/google_compute1/struct.ProjectSetDefaultNetworkTierCall.html) and [*set usage export bucket*](https://docs.rs/google-compute1/1.0.8+20190320/google_compute1/struct.ProjectSetUsageExportBucketCall.html)
* region autoscalers
 * [*delete*](https://docs.rs/google-compute1/1.0.8+20190320/google_compute1/struct.RegionAutoscalerDeleteCall.html), [*get*](https://docs.rs/google-compute1/1.0.8+20190320/google_compute1/struct.RegionAutoscalerGetCall.html), [*insert*](https://docs.rs/google-compute1/1.0.8+20190320/google_compute1/struct.RegionAutoscalerInsertCall.html), [*list*](https://docs.rs/google-compute1/1.0.8+20190320/google_compute1/struct.RegionAutoscalerListCall.html), [*patch*](https://docs.rs/google-compute1/1.0.8+20190320/google_compute1/struct.RegionAutoscalerPatchCall.html) and [*update*](https://docs.rs/google-compute1/1.0.8+20190320/google_compute1/struct.RegionAutoscalerUpdateCall.html)
* region backend services
 * [*delete*](https://docs.rs/google-compute1/1.0.8+20190320/google_compute1/struct.RegionBackendServiceDeleteCall.html), [*get*](https://docs.rs/google-compute1/1.0.8+20190320/google_compute1/struct.RegionBackendServiceGetCall.html), [*get health*](https://docs.rs/google-compute1/1.0.8+20190320/google_compute1/struct.RegionBackendServiceGetHealthCall.html), [*insert*](https://docs.rs/google-compute1/1.0.8+20190320/google_compute1/struct.RegionBackendServiceInsertCall.html), [*list*](https://docs.rs/google-compute1/1.0.8+20190320/google_compute1/struct.RegionBackendServiceListCall.html), [*patch*](https://docs.rs/google-compute1/1.0.8+20190320/google_compute1/struct.RegionBackendServicePatchCall.html) and [*update*](https://docs.rs/google-compute1/1.0.8+20190320/google_compute1/struct.RegionBackendServiceUpdateCall.html)
* region commitments
 * [*aggregated list*](https://docs.rs/google-compute1/1.0.8+20190320/google_compute1/struct.RegionCommitmentAggregatedListCall.html), [*get*](https://docs.rs/google-compute1/1.0.8+20190320/google_compute1/struct.RegionCommitmentGetCall.html), [*insert*](https://docs.rs/google-compute1/1.0.8+20190320/google_compute1/struct.RegionCommitmentInsertCall.html) and [*list*](https://docs.rs/google-compute1/1.0.8+20190320/google_compute1/struct.RegionCommitmentListCall.html)
* region disk types
 * [*get*](https://docs.rs/google-compute1/1.0.8+20190320/google_compute1/struct.RegionDiskTypeGetCall.html) and [*list*](https://docs.rs/google-compute1/1.0.8+20190320/google_compute1/struct.RegionDiskTypeListCall.html)
* region disks
 * [*create snapshot*](https://docs.rs/google-compute1/1.0.8+20190320/google_compute1/struct.RegionDiskCreateSnapshotCall.html), [*delete*](https://docs.rs/google-compute1/1.0.8+20190320/google_compute1/struct.RegionDiskDeleteCall.html), [*get*](https://docs.rs/google-compute1/1.0.8+20190320/google_compute1/struct.RegionDiskGetCall.html), [*insert*](https://docs.rs/google-compute1/1.0.8+20190320/google_compute1/struct.RegionDiskInsertCall.html), [*list*](https://docs.rs/google-compute1/1.0.8+20190320/google_compute1/struct.RegionDiskListCall.html), [*resize*](https://docs.rs/google-compute1/1.0.8+20190320/google_compute1/struct.RegionDiskResizeCall.html), [*set labels*](https://docs.rs/google-compute1/1.0.8+20190320/google_compute1/struct.RegionDiskSetLabelCall.html) and [*test iam permissions*](https://docs.rs/google-compute1/1.0.8+20190320/google_compute1/struct.RegionDiskTestIamPermissionCall.html)
* region instance group managers
 * [*abandon instances*](https://docs.rs/google-compute1/1.0.8+20190320/google_compute1/struct.RegionInstanceGroupManagerAbandonInstanceCall.html), [*delete*](https://docs.rs/google-compute1/1.0.8+20190320/google_compute1/struct.RegionInstanceGroupManagerDeleteCall.html), [*delete instances*](https://docs.rs/google-compute1/1.0.8+20190320/google_compute1/struct.RegionInstanceGroupManagerDeleteInstanceCall.html), [*get*](https://docs.rs/google-compute1/1.0.8+20190320/google_compute1/struct.RegionInstanceGroupManagerGetCall.html), [*insert*](https://docs.rs/google-compute1/1.0.8+20190320/google_compute1/struct.RegionInstanceGroupManagerInsertCall.html), [*list*](https://docs.rs/google-compute1/1.0.8+20190320/google_compute1/struct.RegionInstanceGroupManagerListCall.html), [*list managed instances*](https://docs.rs/google-compute1/1.0.8+20190320/google_compute1/struct.RegionInstanceGroupManagerListManagedInstanceCall.html), [*patch*](https://docs.rs/google-compute1/1.0.8+20190320/google_compute1/struct.RegionInstanceGroupManagerPatchCall.html), [*recreate instances*](https://docs.rs/google-compute1/1.0.8+20190320/google_compute1/struct.RegionInstanceGroupManagerRecreateInstanceCall.html), [*resize*](https://docs.rs/google-compute1/1.0.8+20190320/google_compute1/struct.RegionInstanceGroupManagerResizeCall.html), [*set instance template*](https://docs.rs/google-compute1/1.0.8+20190320/google_compute1/struct.RegionInstanceGroupManagerSetInstanceTemplateCall.html) and [*set target pools*](https://docs.rs/google-compute1/1.0.8+20190320/google_compute1/struct.RegionInstanceGroupManagerSetTargetPoolCall.html)
* region instance groups
 * [*get*](https://docs.rs/google-compute1/1.0.8+20190320/google_compute1/struct.RegionInstanceGroupGetCall.html), [*list*](https://docs.rs/google-compute1/1.0.8+20190320/google_compute1/struct.RegionInstanceGroupListCall.html), [*list instances*](https://docs.rs/google-compute1/1.0.8+20190320/google_compute1/struct.RegionInstanceGroupListInstanceCall.html) and [*set named ports*](https://docs.rs/google-compute1/1.0.8+20190320/google_compute1/struct.RegionInstanceGroupSetNamedPortCall.html)
* region operations
 * [*delete*](https://docs.rs/google-compute1/1.0.8+20190320/google_compute1/struct.RegionOperationDeleteCall.html), [*get*](https://docs.rs/google-compute1/1.0.8+20190320/google_compute1/struct.RegionOperationGetCall.html) and [*list*](https://docs.rs/google-compute1/1.0.8+20190320/google_compute1/struct.RegionOperationListCall.html)
* [regions](https://docs.rs/google-compute1/1.0.8+20190320/google_compute1/struct.Region.html)
 * [*get*](https://docs.rs/google-compute1/1.0.8+20190320/google_compute1/struct.RegionGetCall.html) and [*list*](https://docs.rs/google-compute1/1.0.8+20190320/google_compute1/struct.RegionListCall.html)
* [routers](https://docs.rs/google-compute1/1.0.8+20190320/google_compute1/struct.Router.html)
 * [*aggregated list*](https://docs.rs/google-compute1/1.0.8+20190320/google_compute1/struct.RouterAggregatedListCall.html), [*delete*](https://docs.rs/google-compute1/1.0.8+20190320/google_compute1/struct.RouterDeleteCall.html), [*get*](https://docs.rs/google-compute1/1.0.8+20190320/google_compute1/struct.RouterGetCall.html), [*get nat mapping info*](https://docs.rs/google-compute1/1.0.8+20190320/google_compute1/struct.RouterGetNatMappingInfoCall.html), [*get router status*](https://docs.rs/google-compute1/1.0.8+20190320/google_compute1/struct.RouterGetRouterStatuCall.html), [*insert*](https://docs.rs/google-compute1/1.0.8+20190320/google_compute1/struct.RouterInsertCall.html), [*list*](https://docs.rs/google-compute1/1.0.8+20190320/google_compute1/struct.RouterListCall.html), [*patch*](https://docs.rs/google-compute1/1.0.8+20190320/google_compute1/struct.RouterPatchCall.html), [*preview*](https://docs.rs/google-compute1/1.0.8+20190320/google_compute1/struct.RouterPreviewCall.html) and [*update*](https://docs.rs/google-compute1/1.0.8+20190320/google_compute1/struct.RouterUpdateCall.html)
* [routes](https://docs.rs/google-compute1/1.0.8+20190320/google_compute1/struct.Route.html)
 * [*delete*](https://docs.rs/google-compute1/1.0.8+20190320/google_compute1/struct.RouteDeleteCall.html), [*get*](https://docs.rs/google-compute1/1.0.8+20190320/google_compute1/struct.RouteGetCall.html), [*insert*](https://docs.rs/google-compute1/1.0.8+20190320/google_compute1/struct.RouteInsertCall.html) and [*list*](https://docs.rs/google-compute1/1.0.8+20190320/google_compute1/struct.RouteListCall.html)
* [security policies](https://docs.rs/google-compute1/1.0.8+20190320/google_compute1/struct.SecurityPolicy.html)
 * [*add rule*](https://docs.rs/google-compute1/1.0.8+20190320/google_compute1/struct.SecurityPolicyAddRuleCall.html), [*delete*](https://docs.rs/google-compute1/1.0.8+20190320/google_compute1/struct.SecurityPolicyDeleteCall.html), [*get*](https://docs.rs/google-compute1/1.0.8+20190320/google_compute1/struct.SecurityPolicyGetCall.html), [*get rule*](https://docs.rs/google-compute1/1.0.8+20190320/google_compute1/struct.SecurityPolicyGetRuleCall.html), [*insert*](https://docs.rs/google-compute1/1.0.8+20190320/google_compute1/struct.SecurityPolicyInsertCall.html), [*list*](https://docs.rs/google-compute1/1.0.8+20190320/google_compute1/struct.SecurityPolicyListCall.html), [*patch*](https://docs.rs/google-compute1/1.0.8+20190320/google_compute1/struct.SecurityPolicyPatchCall.html), [*patch rule*](https://docs.rs/google-compute1/1.0.8+20190320/google_compute1/struct.SecurityPolicyPatchRuleCall.html) and [*remove rule*](https://docs.rs/google-compute1/1.0.8+20190320/google_compute1/struct.SecurityPolicyRemoveRuleCall.html)
* [snapshots](https://docs.rs/google-compute1/1.0.8+20190320/google_compute1/struct.Snapshot.html)
 * [*delete*](https://docs.rs/google-compute1/1.0.8+20190320/google_compute1/struct.SnapshotDeleteCall.html), [*get*](https://docs.rs/google-compute1/1.0.8+20190320/google_compute1/struct.SnapshotGetCall.html), [*get iam policy*](https://docs.rs/google-compute1/1.0.8+20190320/google_compute1/struct.SnapshotGetIamPolicyCall.html), [*list*](https://docs.rs/google-compute1/1.0.8+20190320/google_compute1/struct.SnapshotListCall.html), [*set iam policy*](https://docs.rs/google-compute1/1.0.8+20190320/google_compute1/struct.SnapshotSetIamPolicyCall.html), [*set labels*](https://docs.rs/google-compute1/1.0.8+20190320/google_compute1/struct.SnapshotSetLabelCall.html) and [*test iam permissions*](https://docs.rs/google-compute1/1.0.8+20190320/google_compute1/struct.SnapshotTestIamPermissionCall.html)
* [ssl certificates](https://docs.rs/google-compute1/1.0.8+20190320/google_compute1/struct.SslCertificate.html)
 * [*delete*](https://docs.rs/google-compute1/1.0.8+20190320/google_compute1/struct.SslCertificateDeleteCall.html), [*get*](https://docs.rs/google-compute1/1.0.8+20190320/google_compute1/struct.SslCertificateGetCall.html), [*insert*](https://docs.rs/google-compute1/1.0.8+20190320/google_compute1/struct.SslCertificateInsertCall.html) and [*list*](https://docs.rs/google-compute1/1.0.8+20190320/google_compute1/struct.SslCertificateListCall.html)
* [ssl policies](https://docs.rs/google-compute1/1.0.8+20190320/google_compute1/struct.SslPolicy.html)
 * [*delete*](https://docs.rs/google-compute1/1.0.8+20190320/google_compute1/struct.SslPolicyDeleteCall.html), [*get*](https://docs.rs/google-compute1/1.0.8+20190320/google_compute1/struct.SslPolicyGetCall.html), [*insert*](https://docs.rs/google-compute1/1.0.8+20190320/google_compute1/struct.SslPolicyInsertCall.html), [*list*](https://docs.rs/google-compute1/1.0.8+20190320/google_compute1/struct.SslPolicyListCall.html), [*list available features*](https://docs.rs/google-compute1/1.0.8+20190320/google_compute1/struct.SslPolicyListAvailableFeatureCall.html) and [*patch*](https://docs.rs/google-compute1/1.0.8+20190320/google_compute1/struct.SslPolicyPatchCall.html)
* [subnetworks](https://docs.rs/google-compute1/1.0.8+20190320/google_compute1/struct.Subnetwork.html)
 * [*aggregated list*](https://docs.rs/google-compute1/1.0.8+20190320/google_compute1/struct.SubnetworkAggregatedListCall.html), [*delete*](https://docs.rs/google-compute1/1.0.8+20190320/google_compute1/struct.SubnetworkDeleteCall.html), [*expand ip cidr range*](https://docs.rs/google-compute1/1.0.8+20190320/google_compute1/struct.SubnetworkExpandIpCidrRangeCall.html), [*get*](https://docs.rs/google-compute1/1.0.8+20190320/google_compute1/struct.SubnetworkGetCall.html), [*get iam policy*](https://docs.rs/google-compute1/1.0.8+20190320/google_compute1/struct.SubnetworkGetIamPolicyCall.html), [*insert*](https://docs.rs/google-compute1/1.0.8+20190320/google_compute1/struct.SubnetworkInsertCall.html), [*list*](https://docs.rs/google-compute1/1.0.8+20190320/google_compute1/struct.SubnetworkListCall.html), [*list usable*](https://docs.rs/google-compute1/1.0.8+20190320/google_compute1/struct.SubnetworkListUsableCall.html), [*patch*](https://docs.rs/google-compute1/1.0.8+20190320/google_compute1/struct.SubnetworkPatchCall.html), [*set iam policy*](https://docs.rs/google-compute1/1.0.8+20190320/google_compute1/struct.SubnetworkSetIamPolicyCall.html), [*set private ip google access*](https://docs.rs/google-compute1/1.0.8+20190320/google_compute1/struct.SubnetworkSetPrivateIpGoogleAccesCall.html) and [*test iam permissions*](https://docs.rs/google-compute1/1.0.8+20190320/google_compute1/struct.SubnetworkTestIamPermissionCall.html)
* [target http proxies](https://docs.rs/google-compute1/1.0.8+20190320/google_compute1/struct.TargetHttpProxy.html)
 * [*delete*](https://docs.rs/google-compute1/1.0.8+20190320/google_compute1/struct.TargetHttpProxyDeleteCall.html), [*get*](https://docs.rs/google-compute1/1.0.8+20190320/google_compute1/struct.TargetHttpProxyGetCall.html), [*insert*](https://docs.rs/google-compute1/1.0.8+20190320/google_compute1/struct.TargetHttpProxyInsertCall.html), [*list*](https://docs.rs/google-compute1/1.0.8+20190320/google_compute1/struct.TargetHttpProxyListCall.html) and [*set url map*](https://docs.rs/google-compute1/1.0.8+20190320/google_compute1/struct.TargetHttpProxySetUrlMapCall.html)
* [target https proxies](https://docs.rs/google-compute1/1.0.8+20190320/google_compute1/struct.TargetHttpsProxy.html)
 * [*delete*](https://docs.rs/google-compute1/1.0.8+20190320/google_compute1/struct.TargetHttpsProxyDeleteCall.html), [*get*](https://docs.rs/google-compute1/1.0.8+20190320/google_compute1/struct.TargetHttpsProxyGetCall.html), [*insert*](https://docs.rs/google-compute1/1.0.8+20190320/google_compute1/struct.TargetHttpsProxyInsertCall.html), [*list*](https://docs.rs/google-compute1/1.0.8+20190320/google_compute1/struct.TargetHttpsProxyListCall.html), [*set quic override*](https://docs.rs/google-compute1/1.0.8+20190320/google_compute1/struct.TargetHttpsProxySetQuicOverrideCall.html), [*set ssl certificates*](https://docs.rs/google-compute1/1.0.8+20190320/google_compute1/struct.TargetHttpsProxySetSslCertificateCall.html), [*set ssl policy*](https://docs.rs/google-compute1/1.0.8+20190320/google_compute1/struct.TargetHttpsProxySetSslPolicyCall.html) and [*set url map*](https://docs.rs/google-compute1/1.0.8+20190320/google_compute1/struct.TargetHttpsProxySetUrlMapCall.html)
* [target instances](https://docs.rs/google-compute1/1.0.8+20190320/google_compute1/struct.TargetInstance.html)
 * [*aggregated list*](https://docs.rs/google-compute1/1.0.8+20190320/google_compute1/struct.TargetInstanceAggregatedListCall.html), [*delete*](https://docs.rs/google-compute1/1.0.8+20190320/google_compute1/struct.TargetInstanceDeleteCall.html), [*get*](https://docs.rs/google-compute1/1.0.8+20190320/google_compute1/struct.TargetInstanceGetCall.html), [*insert*](https://docs.rs/google-compute1/1.0.8+20190320/google_compute1/struct.TargetInstanceInsertCall.html) and [*list*](https://docs.rs/google-compute1/1.0.8+20190320/google_compute1/struct.TargetInstanceListCall.html)
* [target pools](https://docs.rs/google-compute1/1.0.8+20190320/google_compute1/struct.TargetPool.html)
 * [*add health check*](https://docs.rs/google-compute1/1.0.8+20190320/google_compute1/struct.TargetPoolAddHealthCheckCall.html), [*add instance*](https://docs.rs/google-compute1/1.0.8+20190320/google_compute1/struct.TargetPoolAddInstanceCall.html), [*aggregated list*](https://docs.rs/google-compute1/1.0.8+20190320/google_compute1/struct.TargetPoolAggregatedListCall.html), [*delete*](https://docs.rs/google-compute1/1.0.8+20190320/google_compute1/struct.TargetPoolDeleteCall.html), [*get*](https://docs.rs/google-compute1/1.0.8+20190320/google_compute1/struct.TargetPoolGetCall.html), [*get health*](https://docs.rs/google-compute1/1.0.8+20190320/google_compute1/struct.TargetPoolGetHealthCall.html), [*insert*](https://docs.rs/google-compute1/1.0.8+20190320/google_compute1/struct.TargetPoolInsertCall.html), [*list*](https://docs.rs/google-compute1/1.0.8+20190320/google_compute1/struct.TargetPoolListCall.html), [*remove health check*](https://docs.rs/google-compute1/1.0.8+20190320/google_compute1/struct.TargetPoolRemoveHealthCheckCall.html), [*remove instance*](https://docs.rs/google-compute1/1.0.8+20190320/google_compute1/struct.TargetPoolRemoveInstanceCall.html) and [*set backup*](https://docs.rs/google-compute1/1.0.8+20190320/google_compute1/struct.TargetPoolSetBackupCall.html)
* [target ssl proxies](https://docs.rs/google-compute1/1.0.8+20190320/google_compute1/struct.TargetSslProxy.html)
 * [*delete*](https://docs.rs/google-compute1/1.0.8+20190320/google_compute1/struct.TargetSslProxyDeleteCall.html), [*get*](https://docs.rs/google-compute1/1.0.8+20190320/google_compute1/struct.TargetSslProxyGetCall.html), [*insert*](https://docs.rs/google-compute1/1.0.8+20190320/google_compute1/struct.TargetSslProxyInsertCall.html), [*list*](https://docs.rs/google-compute1/1.0.8+20190320/google_compute1/struct.TargetSslProxyListCall.html), [*set backend service*](https://docs.rs/google-compute1/1.0.8+20190320/google_compute1/struct.TargetSslProxySetBackendServiceCall.html), [*set proxy header*](https://docs.rs/google-compute1/1.0.8+20190320/google_compute1/struct.TargetSslProxySetProxyHeaderCall.html), [*set ssl certificates*](https://docs.rs/google-compute1/1.0.8+20190320/google_compute1/struct.TargetSslProxySetSslCertificateCall.html) and [*set ssl policy*](https://docs.rs/google-compute1/1.0.8+20190320/google_compute1/struct.TargetSslProxySetSslPolicyCall.html)
* [target tcp proxies](https://docs.rs/google-compute1/1.0.8+20190320/google_compute1/struct.TargetTcpProxy.html)
 * [*delete*](https://docs.rs/google-compute1/1.0.8+20190320/google_compute1/struct.TargetTcpProxyDeleteCall.html), [*get*](https://docs.rs/google-compute1/1.0.8+20190320/google_compute1/struct.TargetTcpProxyGetCall.html), [*insert*](https://docs.rs/google-compute1/1.0.8+20190320/google_compute1/struct.TargetTcpProxyInsertCall.html), [*list*](https://docs.rs/google-compute1/1.0.8+20190320/google_compute1/struct.TargetTcpProxyListCall.html), [*set backend service*](https://docs.rs/google-compute1/1.0.8+20190320/google_compute1/struct.TargetTcpProxySetBackendServiceCall.html) and [*set proxy header*](https://docs.rs/google-compute1/1.0.8+20190320/google_compute1/struct.TargetTcpProxySetProxyHeaderCall.html)
* [target vpn gateways](https://docs.rs/google-compute1/1.0.8+20190320/google_compute1/struct.TargetVpnGateway.html)
 * [*aggregated list*](https://docs.rs/google-compute1/1.0.8+20190320/google_compute1/struct.TargetVpnGatewayAggregatedListCall.html), [*delete*](https://docs.rs/google-compute1/1.0.8+20190320/google_compute1/struct.TargetVpnGatewayDeleteCall.html), [*get*](https://docs.rs/google-compute1/1.0.8+20190320/google_compute1/struct.TargetVpnGatewayGetCall.html), [*insert*](https://docs.rs/google-compute1/1.0.8+20190320/google_compute1/struct.TargetVpnGatewayInsertCall.html) and [*list*](https://docs.rs/google-compute1/1.0.8+20190320/google_compute1/struct.TargetVpnGatewayListCall.html)
* [url maps](https://docs.rs/google-compute1/1.0.8+20190320/google_compute1/struct.UrlMap.html)
 * [*delete*](https://docs.rs/google-compute1/1.0.8+20190320/google_compute1/struct.UrlMapDeleteCall.html), [*get*](https://docs.rs/google-compute1/1.0.8+20190320/google_compute1/struct.UrlMapGetCall.html), [*insert*](https://docs.rs/google-compute1/1.0.8+20190320/google_compute1/struct.UrlMapInsertCall.html), [*invalidate cache*](https://docs.rs/google-compute1/1.0.8+20190320/google_compute1/struct.UrlMapInvalidateCacheCall.html), [*list*](https://docs.rs/google-compute1/1.0.8+20190320/google_compute1/struct.UrlMapListCall.html), [*patch*](https://docs.rs/google-compute1/1.0.8+20190320/google_compute1/struct.UrlMapPatchCall.html), [*update*](https://docs.rs/google-compute1/1.0.8+20190320/google_compute1/struct.UrlMapUpdateCall.html) and [*validate*](https://docs.rs/google-compute1/1.0.8+20190320/google_compute1/struct.UrlMapValidateCall.html)
* [vpn tunnels](https://docs.rs/google-compute1/1.0.8+20190320/google_compute1/struct.VpnTunnel.html)
 * [*aggregated list*](https://docs.rs/google-compute1/1.0.8+20190320/google_compute1/struct.VpnTunnelAggregatedListCall.html), [*delete*](https://docs.rs/google-compute1/1.0.8+20190320/google_compute1/struct.VpnTunnelDeleteCall.html), [*get*](https://docs.rs/google-compute1/1.0.8+20190320/google_compute1/struct.VpnTunnelGetCall.html), [*insert*](https://docs.rs/google-compute1/1.0.8+20190320/google_compute1/struct.VpnTunnelInsertCall.html) and [*list*](https://docs.rs/google-compute1/1.0.8+20190320/google_compute1/struct.VpnTunnelListCall.html)
* zone operations
 * [*delete*](https://docs.rs/google-compute1/1.0.8+20190320/google_compute1/struct.ZoneOperationDeleteCall.html), [*get*](https://docs.rs/google-compute1/1.0.8+20190320/google_compute1/struct.ZoneOperationGetCall.html) and [*list*](https://docs.rs/google-compute1/1.0.8+20190320/google_compute1/struct.ZoneOperationListCall.html)
* [zones](https://docs.rs/google-compute1/1.0.8+20190320/google_compute1/struct.Zone.html)
 * [*get*](https://docs.rs/google-compute1/1.0.8+20190320/google_compute1/struct.ZoneGetCall.html) and [*list*](https://docs.rs/google-compute1/1.0.8+20190320/google_compute1/struct.ZoneListCall.html)




# Structure of this Library

The API is structured into the following primary items:

* **[Hub](https://docs.rs/google-compute1/1.0.8+20190320/google_compute1/struct.Compute.html)**
    * a central object to maintain state and allow accessing all *Activities*
    * creates [*Method Builders*](https://docs.rs/google-compute1/1.0.8+20190320/google_compute1/trait.MethodsBuilder.html) which in turn
      allow access to individual [*Call Builders*](https://docs.rs/google-compute1/1.0.8+20190320/google_compute1/trait.CallBuilder.html)
* **[Resources](https://docs.rs/google-compute1/1.0.8+20190320/google_compute1/trait.Resource.html)**
    * primary types that you can apply *Activities* to
    * a collection of properties and *Parts*
    * **[Parts](https://docs.rs/google-compute1/1.0.8+20190320/google_compute1/trait.Part.html)**
        * a collection of properties
        * never directly used in *Activities*
* **[Activities](https://docs.rs/google-compute1/1.0.8+20190320/google_compute1/trait.CallBuilder.html)**
    * operations to apply to *Resources*

All *structures* are marked with applicable traits to further categorize them and ease browsing.

Generally speaking, you can invoke *Activities* like this:

```Rust,ignore
let r = hub.resource().activity(...).doit()
```

Or specifically ...

```ignore
let r = hub.backend_buckets().delete(...).doit()
let r = hub.target_ssl_proxies().set_ssl_policy(...).doit()
let r = hub.ssl_certificates().delete(...).doit()
let r = hub.region_disks().resize(...).doit()
let r = hub.autoscalers().patch(...).doit()
let r = hub.instance_group_managers().resize(...).doit()
let r = hub.instance_templates().delete(...).doit()
let r = hub.region_instance_group_managers().patch(...).doit()
let r = hub.region_backend_services().delete(...).doit()
let r = hub.interconnects().delete(...).doit()
let r = hub.instances().update_shielded_instance_config(...).doit()
let r = hub.firewalls().patch(...).doit()
let r = hub.vpn_tunnels().delete(...).doit()
let r = hub.region_disks().set_labels(...).doit()
let r = hub.backend_services().delete(...).doit()
let r = hub.https_health_checks().delete(...).doit()
let r = hub.node_groups().delete(...).doit()
let r = hub.ssl_policies().delete(...).doit()
let r = hub.ssl_policies().insert(...).doit()
let r = hub.projects().disable_xpn_host(...).doit()
let r = hub.instances().set_service_account(...).doit()
let r = hub.global_forwarding_rules().delete(...).doit()
let r = hub.vpn_tunnels().insert(...).doit()
let r = hub.disks().delete(...).doit()
let r = hub.networks().insert(...).doit()
let r = hub.backend_buckets().add_signed_url_key(...).doit()
let r = hub.health_checks().patch(...).doit()
let r = hub.instances().set_scheduling(...).doit()
let r = hub.instances().delete(...).doit()
let r = hub.target_pools().add_health_check(...).doit()
let r = hub.instance_group_managers().insert(...).doit()
let r = hub.global_addresses().insert(...).doit()
let r = hub.target_ssl_proxies().set_ssl_certificates(...).doit()
let r = hub.global_forwarding_rules().set_target(...).doit()
let r = hub.autoscalers().insert(...).doit()
let r = hub.security_policies().add_rule(...).doit()
let r = hub.instance_groups().insert(...).doit()
let r = hub.subnetworks().insert(...).doit()
let r = hub.routes().insert(...).doit()
let r = hub.disks().resize(...).doit()
let r = hub.target_tcp_proxies().delete(...).doit()
let r = hub.target_ssl_proxies().set_backend_service(...).doit()
let r = hub.projects().set_default_network_tier(...).doit()
let r = hub.instance_group_managers().set_instance_template(...).doit()
let r = hub.node_groups().set_node_template(...).doit()
let r = hub.region_autoscalers().delete(...).doit()
let r = hub.security_policies().insert(...).doit()
let r = hub.networks().remove_peering(...).doit()
let r = hub.ssl_policies().patch(...).doit()
let r = hub.interconnect_attachments().delete(...).doit()
let r = hub.target_pools().insert(...).doit()
let r = hub.node_templates().insert(...).doit()
let r = hub.projects().disable_xpn_resource(...).doit()
let r = hub.instances().set_disk_auto_delete(...).doit()
let r = hub.region_instance_group_managers().recreate_instances(...).doit()
let r = hub.instance_group_managers().set_target_pools(...).doit()
let r = hub.disks().insert(...).doit()
let r = hub.target_ssl_proxies().delete(...).doit()
let r = hub.region_backend_services().insert(...).doit()
let r = hub.https_health_checks().insert(...).doit()
let r = hub.instances().simulate_maintenance_event(...).doit()
let r = hub.target_pools().remove_health_check(...).doit()
let r = hub.target_https_proxies().set_url_map(...).doit()
let r = hub.region_disks().create_snapshot(...).doit()
let r = hub.instances().set_machine_type(...).doit()
let r = hub.instances().update_network_interface(...).doit()
let r = hub.url_maps().insert(...).doit()
let r = hub.instances().add_access_config(...).doit()
let r = hub.instances().set_machine_resources(...).doit()
let r = hub.instances().update_access_config(...).doit()
let r = hub.networks().delete(...).doit()
let r = hub.addresses().insert(...).doit()
let r = hub.health_checks().insert(...).doit()
let r = hub.global_forwarding_rules().insert(...).doit()
let r = hub.target_https_proxies().delete(...).doit()
let r = hub.target_tcp_proxies().insert(...).doit()
let r = hub.global_operations().get(...).doit()
let r = hub.addresses().delete(...).doit()
let r = hub.target_vpn_gateways().insert(...).doit()
let r = hub.target_tcp_proxies().set_backend_service(...).doit()
let r = hub.disks().create_snapshot(...).doit()
let r = hub.health_checks().update(...).doit()
let r = hub.instance_groups().remove_instances(...).doit()
let r = hub.instances().set_metadata(...).doit()
let r = hub.region_commitments().insert(...).doit()
let r = hub.backend_services().add_signed_url_key(...).doit()
let r = hub.forwarding_rules().insert(...).doit()
let r = hub.instances().start(...).doit()
let r = hub.instance_group_managers().delete(...).doit()
let r = hub.instances().set_deletion_protection(...).doit()
let r = hub.subnetworks().expand_ip_cidr_range(...).doit()
let r = hub.target_https_proxies().set_quic_override(...).doit()
let r = hub.projects().move_disk(...).doit()
let r = hub.interconnects().patch(...).doit()
let r = hub.firewalls().insert(...).doit()
let r = hub.disks().set_labels(...).doit()
let r = hub.target_pools().set_backup(...).doit()
let r = hub.instance_group_managers().delete_instances(...).doit()
let r = hub.backend_services().set_security_policy(...).doit()
let r = hub.region_instance_group_managers().delete_instances(...).doit()
let r = hub.instances().detach_disk(...).doit()
let r = hub.region_disks().insert(...).doit()
let r = hub.instance_groups().delete(...).doit()
let r = hub.instances().delete_access_config(...).doit()
let r = hub.target_https_proxies().set_ssl_certificates(...).doit()
let r = hub.instance_groups().add_instances(...).doit()
let r = hub.instance_group_managers().abandon_instances(...).doit()
let r = hub.instances().set_tags(...).doit()
let r = hub.network_endpoint_groups().attach_network_endpoints(...).doit()
let r = hub.snapshots().delete(...).doit()
let r = hub.target_https_proxies().insert(...).doit()
let r = hub.node_groups().add_nodes(...).doit()
let r = hub.target_tcp_proxies().set_proxy_header(...).doit()
let r = hub.backend_services().update(...).doit()
let r = hub.node_groups().delete_nodes(...).doit()
let r = hub.licenses().delete(...).doit()
let r = hub.instance_group_managers().patch(...).doit()
let r = hub.subnetworks().patch(...).doit()
let r = hub.target_https_proxies().set_ssl_policy(...).doit()
let r = hub.instances().attach_disk(...).doit()
let r = hub.projects().set_common_instance_metadata(...).doit()
let r = hub.node_templates().delete(...).doit()
let r = hub.region_operations().get(...).doit()
let r = hub.backend_buckets().insert(...).doit()
let r = hub.node_groups().insert(...).doit()
let r = hub.licenses().insert(...).doit()
let r = hub.backend_services().patch(...).doit()
let r = hub.target_http_proxies().set_url_map(...).doit()
let r = hub.network_endpoint_groups().delete(...).doit()
let r = hub.region_instance_group_managers().delete(...).doit()
let r = hub.images().deprecate(...).doit()
let r = hub.http_health_checks().patch(...).doit()
let r = hub.region_autoscalers().insert(...).doit()
let r = hub.security_policies().patch_rule(...).doit()
let r = hub.interconnect_attachments().patch(...).doit()
let r = hub.images().insert(...).doit()
let r = hub.backend_buckets().delete_signed_url_key(...).doit()
let r = hub.url_maps().invalidate_cache(...).doit()
let r = hub.instance_groups().set_named_ports(...).doit()
let r = hub.ssl_certificates().insert(...).doit()
let r = hub.interconnects().insert(...).doit()
let r = hub.networks().switch_to_custom_mode(...).doit()
let r = hub.projects().enable_xpn_resource(...).doit()
let r = hub.projects().move_instance(...).doit()
let r = hub.security_policies().patch(...).doit()
let r = hub.autoscalers().delete(...).doit()
let r = hub.snapshots().set_labels(...).doit()
let r = hub.instance_group_managers().recreate_instances(...).doit()
let r = hub.https_health_checks().update(...).doit()
let r = hub.region_autoscalers().patch(...).doit()
let r = hub.url_maps().patch(...).doit()
let r = hub.subnetworks().delete(...).doit()
let r = hub.region_disks().delete(...).doit()
let r = hub.instances().stop(...).doit()
let r = hub.target_pools().add_instance(...).doit()
let r = hub.target_pools().remove_instance(...).doit()
let r = hub.network_endpoint_groups().detach_network_endpoints(...).doit()
let r = hub.target_pools().delete(...).doit()
let r = hub.firewalls().update(...).doit()
let r = hub.instances().insert(...).doit()
let r = hub.projects().set_usage_export_bucket(...).doit()
let r = hub.region_backend_services().patch(...).doit()
let r = hub.images().set_labels(...).doit()
let r = hub.target_instances().insert(...).doit()
let r = hub.target_http_proxies().insert(...).doit()
let r = hub.interconnect_attachments().insert(...).doit()
let r = hub.networks().patch(...).doit()
let r = hub.url_maps().update(...).doit()
let r = hub.region_instance_groups().set_named_ports(...).doit()
let r = hub.instance_templates().insert(...).doit()
let r = hub.target_instances().delete(...).doit()
let r = hub.region_instance_group_managers().set_instance_template(...).doit()
let r = hub.projects().enable_xpn_host(...).doit()
let r = hub.forwarding_rules().set_target(...).doit()
let r = hub.routers().delete(...).doit()
let r = hub.region_instance_group_managers().resize(...).doit()
let r = hub.target_vpn_gateways().delete(...).doit()
let r = hub.health_checks().delete(...).doit()
let r = hub.backend_services().insert(...).doit()
let r = hub.instances().set_labels(...).doit()
let r = hub.region_backend_services().update(...).doit()
let r = hub.instances().set_shielded_instance_integrity_policy(...).doit()
let r = hub.target_http_proxies().delete(...).doit()
let r = hub.backend_buckets().patch(...).doit()
let r = hub.global_addresses().delete(...).doit()
let r = hub.https_health_checks().patch(...).doit()
let r = hub.security_policies().remove_rule(...).doit()
let r = hub.url_maps().delete(...).doit()
let r = hub.routers().insert(...).doit()
let r = hub.instances().reset(...).doit()
let r = hub.routers().update(...).doit()
let r = hub.routers().patch(...).doit()
let r = hub.networks().add_peering(...).doit()
let r = hub.security_policies().delete(...).doit()
let r = hub.target_ssl_proxies().set_proxy_header(...).doit()
let r = hub.http_health_checks().delete(...).doit()
let r = hub.http_health_checks().insert(...).doit()
let r = hub.images().delete(...).doit()
let r = hub.region_instance_group_managers().abandon_instances(...).doit()
let r = hub.subnetworks().set_private_ip_google_access(...).doit()
let r = hub.autoscalers().update(...).doit()
let r = hub.region_instance_group_managers().set_target_pools(...).doit()
let r = hub.region_autoscalers().update(...).doit()
let r = hub.target_ssl_proxies().insert(...).doit()
let r = hub.instances().start_with_encryption_key(...).doit()
let r = hub.instances().set_min_cpu_platform(...).doit()
let r = hub.forwarding_rules().delete(...).doit()
let r = hub.network_endpoint_groups().insert(...).doit()
let r = hub.backend_buckets().update(...).doit()
let r = hub.region_instance_group_managers().insert(...).doit()
let r = hub.firewalls().delete(...).doit()
let r = hub.zone_operations().get(...).doit()
let r = hub.http_health_checks().update(...).doit()
let r = hub.routes().delete(...).doit()
let r = hub.backend_services().delete_signed_url_key(...).doit()
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
                              hyper::Client::with_connector(hyper::net::HttpsConnector::new(hyper_rustls::TlsClient::new())),
                              <MemoryStorage as Default>::default(), None);
let mut hub = Compute::new(hyper::Client::with_connector(hyper::net::HttpsConnector::new(hyper_rustls::TlsClient::new())), auth);
// You can configure optional parameters by calling the respective setters at will, and
// execute the final call using `doit()`.
// Values shown here are possibly random and not representative !
let result = hub.instances().set_disk_auto_delete("project", "zone", "instance", true, "deviceName")
             .request_id("dolores")
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

All errors produced by the system are provided either as [Result](https://docs.rs/google-compute1/1.0.8+20190320/google_compute1/enum.Result.html) enumeration as return value of 
the doit() methods, or handed as possibly intermediate results to either the 
[Hub Delegate](https://docs.rs/google-compute1/1.0.8+20190320/google_compute1/trait.Delegate.html), or the [Authenticator Delegate](https://docs.rs/yup-oauth2/*/yup_oauth2/trait.AuthenticatorDelegate.html).

When delegates handle errors or intermediate values, they may have a chance to instruct the system to retry. This 
makes the system potentially resilient to all kinds of errors.

## Uploads and Downloads
If a method supports downloads, the response body, which is part of the [Result](https://docs.rs/google-compute1/1.0.8+20190320/google_compute1/enum.Result.html), should be
read by you to obtain the media.
If such a method also supports a [Response Result](https://docs.rs/google-compute1/1.0.8+20190320/google_compute1/trait.ResponseResult.html), it will return that by default.
You can see it as meta-data for the actual media. To trigger a media download, you will have to set up the builder by making
this call: `.param("alt", "media")`.

Methods supporting uploads can do so using up to 2 different protocols: 
*simple* and *resumable*. The distinctiveness of each is represented by customized 
`doit(...)` methods, which are then named `upload(...)` and `upload_resumable(...)` respectively.

## Customization and Callbacks

You may alter the way an `doit()` method is called by providing a [delegate](https://docs.rs/google-compute1/1.0.8+20190320/google_compute1/trait.Delegate.html) to the 
[Method Builder](https://docs.rs/google-compute1/1.0.8+20190320/google_compute1/trait.CallBuilder.html) before making the final `doit()` call. 
Respective methods will be called to provide progress information, as well as determine whether the system should 
retry on failure.

The [delegate trait](https://docs.rs/google-compute1/1.0.8+20190320/google_compute1/trait.Delegate.html) is default-implemented, allowing you to customize it with minimal effort.

## Optional Parts in Server-Requests

All structures provided by this library are made to be [enocodable](https://docs.rs/google-compute1/1.0.8+20190320/google_compute1/trait.RequestValue.html) and 
[decodable](https://docs.rs/google-compute1/1.0.8+20190320/google_compute1/trait.ResponseResult.html) via *json*. Optionals are used to indicate that partial requests are responses 
are valid.
Most optionals are are considered [Parts](https://docs.rs/google-compute1/1.0.8+20190320/google_compute1/trait.Part.html) which are identifiable by name, which will be sent to 
the server to indicate either the set parts of the request or the desired parts in the response.

## Builder Arguments

Using [method builders](https://docs.rs/google-compute1/1.0.8+20190320/google_compute1/trait.CallBuilder.html), you are able to prepare an action call by repeatedly calling it's methods.
These will always take a single argument, for which the following statements are true.

* [PODs][wiki-pod] are handed by copy
* strings are passed as `&str`
* [request values](https://docs.rs/google-compute1/1.0.8+20190320/google_compute1/trait.RequestValue.html) are moved

Arguments will always be copied or cloned into the builder, to make them independent of their original life times.

[wiki-pod]: http://en.wikipedia.org/wiki/Plain_old_data_structure
[builder-pattern]: http://en.wikipedia.org/wiki/Builder_pattern
[google-go-api]: https://github.com/google/google-api-go-client

# License
The **compute1** library was generated by Sebastian Thiel, and is placed 
under the *MIT* license.
You can read the full text at the repository's [license file][repo-license].

[repo-license]: https://github.com/Byron/google-apis-rsblob/master/LICENSE.md
