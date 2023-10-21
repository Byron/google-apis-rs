use super::*;

/// Central instance to access all Compute related resource activities
///
/// # Examples
///
/// Instantiate a new hub
///
/// ```test_harness,no_run
/// extern crate hyper;
/// extern crate hyper_rustls;
/// extern crate google_compute1 as compute1;
/// use compute1::api::Instance;
/// use compute1::{Result, Error};
/// # async fn dox() {
/// use std::default::Default;
/// use compute1::{Compute, oauth2, hyper, hyper_rustls, chrono, FieldMask};
/// 
/// // Get an ApplicationSecret instance by some means. It contains the `client_id` and
/// // `client_secret`, among other things.
/// let secret: oauth2::ApplicationSecret = Default::default();
/// // Instantiate the authenticator. It will choose a suitable authentication flow for you,
/// // unless you replace  `None` with the desired Flow.
/// // Provide your own `AuthenticatorDelegate` to adjust the way it operates and get feedback about
/// // what's going on. You probably want to bring in your own `TokenStorage` to persist tokens and
/// // retrieve them from storage.
/// let auth = oauth2::InstalledFlowAuthenticator::builder(
///         secret,
///         oauth2::InstalledFlowReturnMethod::HTTPRedirect,
///     ).build().await.unwrap();
/// let mut hub = Compute::new(hyper::Client::builder().build(hyper_rustls::HttpsConnectorBuilder::new().with_native_roots().https_or_http().enable_http1().build()), auth);
/// // As the method needs a request, you would usually fill it with the desired information
/// // into the respective structure. Some of the parts shown here might not be applicable !
/// // Values shown here are possibly random and not representative !
/// let mut req = Instance::default();
/// 
/// // You can configure optional parameters by calling the respective setters at will, and
/// // execute the final call using `doit()`.
/// // Values shown here are possibly random and not representative !
/// let result = hub.instances().update(req, "project", "zone", "instance")
///              .request_id("gubergren")
///              .most_disruptive_allowed_action("eos")
///              .minimal_action("dolor")
///              .doit().await;
/// 
/// match result {
///     Err(e) => match e {
///         // The Error enum provides details about what exactly happened.
///         // You can also just use its `Debug`, `Display` or `Error` traits
///          Error::HttpError(_)
///         |Error::Io(_)
///         |Error::MissingAPIKey
///         |Error::MissingToken(_)
///         |Error::Cancelled
///         |Error::UploadSizeLimitExceeded(_, _)
///         |Error::Failure(_)
///         |Error::BadRequest(_)
///         |Error::FieldClash(_)
///         |Error::JsonDecodeError(_, _) => println!("{}", e),
///     },
///     Ok(res) => println!("Success: {:?}", res),
/// }
/// # }
/// ```
#[derive(Clone)]
pub struct Compute<S> {
    pub client: hyper::Client<S, hyper::body::Body>,
    pub auth: Box<dyn client::GetToken>,
    pub(super) _user_agent: String,
    pub(super) _base_url: String,
    pub(super) _root_url: String,
}

impl<'a, S> client::Hub for Compute<S> {}

impl<'a, S> Compute<S> {

    pub fn new<A: 'static + client::GetToken>(client: hyper::Client<S, hyper::body::Body>, auth: A) -> Compute<S> {
        Compute {
            client,
            auth: Box::new(auth),
            _user_agent: "google-api-rust-client/5.0.3".to_string(),
            _base_url: "https://compute.googleapis.com/compute/v1/".to_string(),
            _root_url: "https://compute.googleapis.com/".to_string(),
        }
    }

    pub fn accelerator_types(&'a self) -> AcceleratorTypeMethods<'a, S> {
        AcceleratorTypeMethods { hub: &self }
    }
    pub fn addresses(&'a self) -> AddressMethods<'a, S> {
        AddressMethods { hub: &self }
    }
    pub fn autoscalers(&'a self) -> AutoscalerMethods<'a, S> {
        AutoscalerMethods { hub: &self }
    }
    pub fn backend_buckets(&'a self) -> BackendBucketMethods<'a, S> {
        BackendBucketMethods { hub: &self }
    }
    pub fn backend_services(&'a self) -> BackendServiceMethods<'a, S> {
        BackendServiceMethods { hub: &self }
    }
    pub fn disk_types(&'a self) -> DiskTypeMethods<'a, S> {
        DiskTypeMethods { hub: &self }
    }
    pub fn disks(&'a self) -> DiskMethods<'a, S> {
        DiskMethods { hub: &self }
    }
    pub fn external_vpn_gateways(&'a self) -> ExternalVpnGatewayMethods<'a, S> {
        ExternalVpnGatewayMethods { hub: &self }
    }
    pub fn firewall_policies(&'a self) -> FirewallPolicyMethods<'a, S> {
        FirewallPolicyMethods { hub: &self }
    }
    pub fn firewalls(&'a self) -> FirewallMethods<'a, S> {
        FirewallMethods { hub: &self }
    }
    pub fn forwarding_rules(&'a self) -> ForwardingRuleMethods<'a, S> {
        ForwardingRuleMethods { hub: &self }
    }
    pub fn global_addresses(&'a self) -> GlobalAddressMethods<'a, S> {
        GlobalAddressMethods { hub: &self }
    }
    pub fn global_forwarding_rules(&'a self) -> GlobalForwardingRuleMethods<'a, S> {
        GlobalForwardingRuleMethods { hub: &self }
    }
    pub fn global_network_endpoint_groups(&'a self) -> GlobalNetworkEndpointGroupMethods<'a, S> {
        GlobalNetworkEndpointGroupMethods { hub: &self }
    }
    pub fn global_operations(&'a self) -> GlobalOperationMethods<'a, S> {
        GlobalOperationMethods { hub: &self }
    }
    pub fn global_organization_operations(&'a self) -> GlobalOrganizationOperationMethods<'a, S> {
        GlobalOrganizationOperationMethods { hub: &self }
    }
    pub fn global_public_delegated_prefixes(&'a self) -> GlobalPublicDelegatedPrefixMethods<'a, S> {
        GlobalPublicDelegatedPrefixMethods { hub: &self }
    }
    pub fn health_checks(&'a self) -> HealthCheckMethods<'a, S> {
        HealthCheckMethods { hub: &self }
    }
    pub fn http_health_checks(&'a self) -> HttpHealthCheckMethods<'a, S> {
        HttpHealthCheckMethods { hub: &self }
    }
    pub fn https_health_checks(&'a self) -> HttpsHealthCheckMethods<'a, S> {
        HttpsHealthCheckMethods { hub: &self }
    }
    pub fn image_family_views(&'a self) -> ImageFamilyViewMethods<'a, S> {
        ImageFamilyViewMethods { hub: &self }
    }
    pub fn images(&'a self) -> ImageMethods<'a, S> {
        ImageMethods { hub: &self }
    }
    pub fn instance_group_managers(&'a self) -> InstanceGroupManagerMethods<'a, S> {
        InstanceGroupManagerMethods { hub: &self }
    }
    pub fn instance_groups(&'a self) -> InstanceGroupMethods<'a, S> {
        InstanceGroupMethods { hub: &self }
    }
    pub fn instance_templates(&'a self) -> InstanceTemplateMethods<'a, S> {
        InstanceTemplateMethods { hub: &self }
    }
    pub fn instances(&'a self) -> InstanceMethods<'a, S> {
        InstanceMethods { hub: &self }
    }
    pub fn interconnect_attachments(&'a self) -> InterconnectAttachmentMethods<'a, S> {
        InterconnectAttachmentMethods { hub: &self }
    }
    pub fn interconnect_locations(&'a self) -> InterconnectLocationMethods<'a, S> {
        InterconnectLocationMethods { hub: &self }
    }
    pub fn interconnects(&'a self) -> InterconnectMethods<'a, S> {
        InterconnectMethods { hub: &self }
    }
    pub fn license_codes(&'a self) -> LicenseCodeMethods<'a, S> {
        LicenseCodeMethods { hub: &self }
    }
    pub fn licenses(&'a self) -> LicenseMethods<'a, S> {
        LicenseMethods { hub: &self }
    }
    pub fn machine_images(&'a self) -> MachineImageMethods<'a, S> {
        MachineImageMethods { hub: &self }
    }
    pub fn machine_types(&'a self) -> MachineTypeMethods<'a, S> {
        MachineTypeMethods { hub: &self }
    }
    pub fn network_attachments(&'a self) -> NetworkAttachmentMethods<'a, S> {
        NetworkAttachmentMethods { hub: &self }
    }
    pub fn network_edge_security_services(&'a self) -> NetworkEdgeSecurityServiceMethods<'a, S> {
        NetworkEdgeSecurityServiceMethods { hub: &self }
    }
    pub fn network_endpoint_groups(&'a self) -> NetworkEndpointGroupMethods<'a, S> {
        NetworkEndpointGroupMethods { hub: &self }
    }
    pub fn network_firewall_policies(&'a self) -> NetworkFirewallPolicyMethods<'a, S> {
        NetworkFirewallPolicyMethods { hub: &self }
    }
    pub fn networks(&'a self) -> NetworkMethods<'a, S> {
        NetworkMethods { hub: &self }
    }
    pub fn node_groups(&'a self) -> NodeGroupMethods<'a, S> {
        NodeGroupMethods { hub: &self }
    }
    pub fn node_templates(&'a self) -> NodeTemplateMethods<'a, S> {
        NodeTemplateMethods { hub: &self }
    }
    pub fn node_types(&'a self) -> NodeTypeMethods<'a, S> {
        NodeTypeMethods { hub: &self }
    }
    pub fn packet_mirrorings(&'a self) -> PacketMirroringMethods<'a, S> {
        PacketMirroringMethods { hub: &self }
    }
    pub fn projects(&'a self) -> ProjectMethods<'a, S> {
        ProjectMethods { hub: &self }
    }
    pub fn public_advertised_prefixes(&'a self) -> PublicAdvertisedPrefixMethods<'a, S> {
        PublicAdvertisedPrefixMethods { hub: &self }
    }
    pub fn public_delegated_prefixes(&'a self) -> PublicDelegatedPrefixMethods<'a, S> {
        PublicDelegatedPrefixMethods { hub: &self }
    }
    pub fn region_autoscalers(&'a self) -> RegionAutoscalerMethods<'a, S> {
        RegionAutoscalerMethods { hub: &self }
    }
    pub fn region_backend_services(&'a self) -> RegionBackendServiceMethods<'a, S> {
        RegionBackendServiceMethods { hub: &self }
    }
    pub fn region_commitments(&'a self) -> RegionCommitmentMethods<'a, S> {
        RegionCommitmentMethods { hub: &self }
    }
    pub fn region_disk_types(&'a self) -> RegionDiskTypeMethods<'a, S> {
        RegionDiskTypeMethods { hub: &self }
    }
    pub fn region_disks(&'a self) -> RegionDiskMethods<'a, S> {
        RegionDiskMethods { hub: &self }
    }
    pub fn region_health_check_services(&'a self) -> RegionHealthCheckServiceMethods<'a, S> {
        RegionHealthCheckServiceMethods { hub: &self }
    }
    pub fn region_health_checks(&'a self) -> RegionHealthCheckMethods<'a, S> {
        RegionHealthCheckMethods { hub: &self }
    }
    pub fn region_instance_group_managers(&'a self) -> RegionInstanceGroupManagerMethods<'a, S> {
        RegionInstanceGroupManagerMethods { hub: &self }
    }
    pub fn region_instance_groups(&'a self) -> RegionInstanceGroupMethods<'a, S> {
        RegionInstanceGroupMethods { hub: &self }
    }
    pub fn region_instances(&'a self) -> RegionInstanceMethods<'a, S> {
        RegionInstanceMethods { hub: &self }
    }
    pub fn region_network_endpoint_groups(&'a self) -> RegionNetworkEndpointGroupMethods<'a, S> {
        RegionNetworkEndpointGroupMethods { hub: &self }
    }
    pub fn region_network_firewall_policies(&'a self) -> RegionNetworkFirewallPolicyMethods<'a, S> {
        RegionNetworkFirewallPolicyMethods { hub: &self }
    }
    pub fn region_notification_endpoints(&'a self) -> RegionNotificationEndpointMethods<'a, S> {
        RegionNotificationEndpointMethods { hub: &self }
    }
    pub fn region_operations(&'a self) -> RegionOperationMethods<'a, S> {
        RegionOperationMethods { hub: &self }
    }
    pub fn region_security_policies(&'a self) -> RegionSecurityPolicyMethods<'a, S> {
        RegionSecurityPolicyMethods { hub: &self }
    }
    pub fn region_ssl_certificates(&'a self) -> RegionSslCertificateMethods<'a, S> {
        RegionSslCertificateMethods { hub: &self }
    }
    pub fn region_ssl_policies(&'a self) -> RegionSslPolicyMethods<'a, S> {
        RegionSslPolicyMethods { hub: &self }
    }
    pub fn region_target_http_proxies(&'a self) -> RegionTargetHttpProxyMethods<'a, S> {
        RegionTargetHttpProxyMethods { hub: &self }
    }
    pub fn region_target_https_proxies(&'a self) -> RegionTargetHttpsProxyMethods<'a, S> {
        RegionTargetHttpsProxyMethods { hub: &self }
    }
    pub fn region_target_tcp_proxies(&'a self) -> RegionTargetTcpProxyMethods<'a, S> {
        RegionTargetTcpProxyMethods { hub: &self }
    }
    pub fn region_url_maps(&'a self) -> RegionUrlMapMethods<'a, S> {
        RegionUrlMapMethods { hub: &self }
    }
    pub fn regions(&'a self) -> RegionMethods<'a, S> {
        RegionMethods { hub: &self }
    }
    pub fn reservations(&'a self) -> ReservationMethods<'a, S> {
        ReservationMethods { hub: &self }
    }
    pub fn resource_policies(&'a self) -> ResourcePolicyMethods<'a, S> {
        ResourcePolicyMethods { hub: &self }
    }
    pub fn routers(&'a self) -> RouterMethods<'a, S> {
        RouterMethods { hub: &self }
    }
    pub fn routes(&'a self) -> RouteMethods<'a, S> {
        RouteMethods { hub: &self }
    }
    pub fn security_policies(&'a self) -> SecurityPolicyMethods<'a, S> {
        SecurityPolicyMethods { hub: &self }
    }
    pub fn service_attachments(&'a self) -> ServiceAttachmentMethods<'a, S> {
        ServiceAttachmentMethods { hub: &self }
    }
    pub fn snapshots(&'a self) -> SnapshotMethods<'a, S> {
        SnapshotMethods { hub: &self }
    }
    pub fn ssl_certificates(&'a self) -> SslCertificateMethods<'a, S> {
        SslCertificateMethods { hub: &self }
    }
    pub fn ssl_policies(&'a self) -> SslPolicyMethods<'a, S> {
        SslPolicyMethods { hub: &self }
    }
    pub fn subnetworks(&'a self) -> SubnetworkMethods<'a, S> {
        SubnetworkMethods { hub: &self }
    }
    pub fn target_grpc_proxies(&'a self) -> TargetGrpcProxyMethods<'a, S> {
        TargetGrpcProxyMethods { hub: &self }
    }
    pub fn target_http_proxies(&'a self) -> TargetHttpProxyMethods<'a, S> {
        TargetHttpProxyMethods { hub: &self }
    }
    pub fn target_https_proxies(&'a self) -> TargetHttpsProxyMethods<'a, S> {
        TargetHttpsProxyMethods { hub: &self }
    }
    pub fn target_instances(&'a self) -> TargetInstanceMethods<'a, S> {
        TargetInstanceMethods { hub: &self }
    }
    pub fn target_pools(&'a self) -> TargetPoolMethods<'a, S> {
        TargetPoolMethods { hub: &self }
    }
    pub fn target_ssl_proxies(&'a self) -> TargetSslProxyMethods<'a, S> {
        TargetSslProxyMethods { hub: &self }
    }
    pub fn target_tcp_proxies(&'a self) -> TargetTcpProxyMethods<'a, S> {
        TargetTcpProxyMethods { hub: &self }
    }
    pub fn target_vpn_gateways(&'a self) -> TargetVpnGatewayMethods<'a, S> {
        TargetVpnGatewayMethods { hub: &self }
    }
    pub fn url_maps(&'a self) -> UrlMapMethods<'a, S> {
        UrlMapMethods { hub: &self }
    }
    pub fn vpn_gateways(&'a self) -> VpnGatewayMethods<'a, S> {
        VpnGatewayMethods { hub: &self }
    }
    pub fn vpn_tunnels(&'a self) -> VpnTunnelMethods<'a, S> {
        VpnTunnelMethods { hub: &self }
    }
    pub fn zone_operations(&'a self) -> ZoneOperationMethods<'a, S> {
        ZoneOperationMethods { hub: &self }
    }
    pub fn zones(&'a self) -> ZoneMethods<'a, S> {
        ZoneMethods { hub: &self }
    }

    /// Set the user-agent header field to use in all requests to the server.
    /// It defaults to `google-api-rust-client/5.0.3`.
    ///
    /// Returns the previously set user-agent.
    pub fn user_agent(&mut self, agent_name: String) -> String {
        mem::replace(&mut self._user_agent, agent_name)
    }

    /// Set the base url to use in all requests to the server.
    /// It defaults to `https://compute.googleapis.com/compute/v1/`.
    ///
    /// Returns the previously set base url.
    pub fn base_url(&mut self, new_base_url: String) -> String {
        mem::replace(&mut self._base_url, new_base_url)
    }

    /// Set the root url to use in all requests to the server.
    /// It defaults to `https://compute.googleapis.com/`.
    ///
    /// Returns the previously set root url.
    pub fn root_url(&mut self, new_root_url: String) -> String {
        mem::replace(&mut self._root_url, new_root_url)
    }
}
