use super::*;
/// A builder providing access to all methods supported on *advertiser* resources.
/// It is not used directly, but through the [`DisplayVideo`] hub.
///
/// # Example
///
/// Instantiate a resource builder
///
/// ```test_harness,no_run
/// extern crate hyper;
/// extern crate hyper_rustls;
/// extern crate google_displayvideo1 as displayvideo1;
/// 
/// # async fn dox() {
/// use std::default::Default;
/// use displayvideo1::{DisplayVideo, oauth2, hyper, hyper_rustls, chrono, FieldMask};
/// 
/// let secret: oauth2::ApplicationSecret = Default::default();
/// let auth = oauth2::InstalledFlowAuthenticator::builder(
///         secret,
///         oauth2::InstalledFlowReturnMethod::HTTPRedirect,
///     ).build().await.unwrap();
/// let mut hub = DisplayVideo::new(hyper::Client::builder().build(hyper_rustls::HttpsConnectorBuilder::new().with_native_roots().https_or_http().enable_http1().build()), auth);
/// // Usually you wouldn't bind this to a variable, but keep calling *CallBuilders*
/// // like `assets_upload(...)`, `audit(...)`, `bulk_edit_advertiser_assigned_targeting_options(...)`, `bulk_list_advertiser_assigned_targeting_options(...)`, `campaigns_bulk_list_campaign_assigned_targeting_options(...)`, `campaigns_create(...)`, `campaigns_delete(...)`, `campaigns_get(...)`, `campaigns_list(...)`, `campaigns_patch(...)`, `campaigns_targeting_types_assigned_targeting_options_get(...)`, `campaigns_targeting_types_assigned_targeting_options_list(...)`, `channels_create(...)`, `channels_get(...)`, `channels_list(...)`, `channels_patch(...)`, `channels_sites_bulk_edit(...)`, `channels_sites_create(...)`, `channels_sites_delete(...)`, `channels_sites_list(...)`, `channels_sites_replace(...)`, `create(...)`, `creatives_create(...)`, `creatives_delete(...)`, `creatives_get(...)`, `creatives_list(...)`, `creatives_patch(...)`, `delete(...)`, `get(...)`, `insertion_orders_bulk_list_insertion_order_assigned_targeting_options(...)`, `insertion_orders_create(...)`, `insertion_orders_delete(...)`, `insertion_orders_get(...)`, `insertion_orders_list(...)`, `insertion_orders_patch(...)`, `insertion_orders_targeting_types_assigned_targeting_options_get(...)`, `insertion_orders_targeting_types_assigned_targeting_options_list(...)`, `invoices_list(...)`, `invoices_lookup_invoice_currency(...)`, `line_items_bulk_edit_line_item_assigned_targeting_options(...)`, `line_items_bulk_list_line_item_assigned_targeting_options(...)`, `line_items_create(...)`, `line_items_delete(...)`, `line_items_generate_default(...)`, `line_items_get(...)`, `line_items_list(...)`, `line_items_patch(...)`, `line_items_targeting_types_assigned_targeting_options_create(...)`, `line_items_targeting_types_assigned_targeting_options_delete(...)`, `line_items_targeting_types_assigned_targeting_options_get(...)`, `line_items_targeting_types_assigned_targeting_options_list(...)`, `list(...)`, `location_lists_assigned_locations_bulk_edit(...)`, `location_lists_assigned_locations_create(...)`, `location_lists_assigned_locations_delete(...)`, `location_lists_assigned_locations_list(...)`, `location_lists_create(...)`, `location_lists_get(...)`, `location_lists_list(...)`, `location_lists_patch(...)`, `manual_triggers_activate(...)`, `manual_triggers_create(...)`, `manual_triggers_deactivate(...)`, `manual_triggers_get(...)`, `manual_triggers_list(...)`, `manual_triggers_patch(...)`, `negative_keyword_lists_create(...)`, `negative_keyword_lists_delete(...)`, `negative_keyword_lists_get(...)`, `negative_keyword_lists_list(...)`, `negative_keyword_lists_negative_keywords_bulk_edit(...)`, `negative_keyword_lists_negative_keywords_create(...)`, `negative_keyword_lists_negative_keywords_delete(...)`, `negative_keyword_lists_negative_keywords_list(...)`, `negative_keyword_lists_negative_keywords_replace(...)`, `negative_keyword_lists_patch(...)`, `patch(...)`, `targeting_types_assigned_targeting_options_create(...)`, `targeting_types_assigned_targeting_options_delete(...)`, `targeting_types_assigned_targeting_options_get(...)` and `targeting_types_assigned_targeting_options_list(...)`
/// // to build up your call.
/// let rb = hub.advertisers();
/// # }
/// ```
pub struct AdvertiserMethods<'a, S>
    where S: 'a {

   pub(super) hub: &'a DisplayVideo<S>,
}

impl<'a, S> client::MethodsBuilder for AdvertiserMethods<'a, S> {}

impl<'a, S> AdvertiserMethods<'a, S> {
    
    /// Create a builder to help you perform the following task:
    ///
    /// Uploads an asset. Returns the ID of the newly uploaded asset if successful. The asset file size should be no more than 10 MB for images, 200 MB for ZIP files, and 1 GB for videos. Must be used within the [multipart media upload process](https://developers.google.com/display-video/api/guides/how-tos/upload#multipart). Examples using provided client libraries can be found in our [Creating Creatives guide](https://developers.google.com/display-video/api/guides/creating-creatives/overview#upload_an_asset).
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `advertiserId` - Required. The ID of the advertiser this asset belongs to.
    pub fn assets_upload(&self, request: CreateAssetRequest, advertiser_id: i64) -> AdvertiserAssetUploadCall<'a, S> {
        AdvertiserAssetUploadCall {
            hub: self.hub,
            _request: request,
            _advertiser_id: advertiser_id,
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Gets a single targeting option assigned to a campaign.
    /// 
    /// # Arguments
    ///
    /// * `advertiserId` - Required. The ID of the advertiser the campaign belongs to.
    /// * `campaignId` - Required. The ID of the campaign the assigned targeting option belongs to.
    /// * `targetingType` - Required. Identifies the type of this assigned targeting option. Supported targeting types: * `TARGETING_TYPE_AGE_RANGE` * `TARGETING_TYPE_AUTHORIZED_SELLER_STATUS` * `TARGETING_TYPE_CONTENT_INSTREAM_POSITION` * `TARGETING_TYPE_CONTENT_OUTSTREAM_POSITION` * `TARGETING_TYPE_DIGITAL_CONTENT_LABEL_EXCLUSION` * `TARGETING_TYPE_ENVIRONMENT` * `TARGETING_TYPE_EXCHANGE` * `TARGETING_TYPE_GENDER` * `TARGETING_TYPE_GEO_REGION` * `TARGETING_TYPE_HOUSEHOLD_INCOME` * `TARGETING_TYPE_INVENTORY_SOURCE` * `TARGETING_TYPE_INVENTORY_SOURCE_GROUP` * `TARGETING_TYPE_LANGUAGE` * `TARGETING_TYPE_ON_SCREEN_POSITION` * `TARGETING_TYPE_PARENTAL_STATUS` * `TARGETING_TYPE_SENSITIVE_CATEGORY_EXCLUSION` * `TARGETING_TYPE_SUB_EXCHANGE` * `TARGETING_TYPE_THIRD_PARTY_VERIFIER` * `TARGETING_TYPE_VIEWABILITY`
    /// * `assignedTargetingOptionId` - Required. An identifier unique to the targeting type in this campaign that identifies the assigned targeting option being requested.
    pub fn campaigns_targeting_types_assigned_targeting_options_get(&self, advertiser_id: i64, campaign_id: i64, targeting_type: &AdvertiserTargetingTypeEnum, assigned_targeting_option_id: &str) -> AdvertiserCampaignTargetingTypeAssignedTargetingOptionGetCall<'a, S> {
        AdvertiserCampaignTargetingTypeAssignedTargetingOptionGetCall {
            hub: self.hub,
            _advertiser_id: advertiser_id,
            _campaign_id: campaign_id,
            _targeting_type: targeting_type.clone(),
            _assigned_targeting_option_id: assigned_targeting_option_id.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Lists the targeting options assigned to a campaign for a specified targeting type.
    /// 
    /// # Arguments
    ///
    /// * `advertiserId` - Required. The ID of the advertiser the campaign belongs to.
    /// * `campaignId` - Required. The ID of the campaign to list assigned targeting options for.
    /// * `targetingType` - Required. Identifies the type of assigned targeting options to list. Supported targeting types: * `TARGETING_TYPE_AGE_RANGE` * `TARGETING_TYPE_AUTHORIZED_SELLER_STATUS` * `TARGETING_TYPE_CONTENT_INSTREAM_POSITION` * `TARGETING_TYPE_CONTENT_OUTSTREAM_POSITION` * `TARGETING_TYPE_DIGITAL_CONTENT_LABEL_EXCLUSION` * `TARGETING_TYPE_ENVIRONMENT` * `TARGETING_TYPE_EXCHANGE` * `TARGETING_TYPE_GENDER` * `TARGETING_TYPE_GEO_REGION` * `TARGETING_TYPE_HOUSEHOLD_INCOME` * `TARGETING_TYPE_INVENTORY_SOURCE` * `TARGETING_TYPE_INVENTORY_SOURCE_GROUP` * `TARGETING_TYPE_LANGUAGE` * `TARGETING_TYPE_ON_SCREEN_POSITION` * `TARGETING_TYPE_PARENTAL_STATUS` * `TARGETING_TYPE_SENSITIVE_CATEGORY_EXCLUSION` * `TARGETING_TYPE_SUB_EXCHANGE` * `TARGETING_TYPE_THIRD_PARTY_VERIFIER` * `TARGETING_TYPE_VIEWABILITY`
    pub fn campaigns_targeting_types_assigned_targeting_options_list(&self, advertiser_id: i64, campaign_id: i64, targeting_type: &AdvertiserTargetingTypeEnum) -> AdvertiserCampaignTargetingTypeAssignedTargetingOptionListCall<'a, S> {
        AdvertiserCampaignTargetingTypeAssignedTargetingOptionListCall {
            hub: self.hub,
            _advertiser_id: advertiser_id,
            _campaign_id: campaign_id,
            _targeting_type: targeting_type.clone(),
            _page_token: Default::default(),
            _page_size: Default::default(),
            _order_by: Default::default(),
            _filter: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Lists assigned targeting options of a campaign across targeting types.
    /// 
    /// # Arguments
    ///
    /// * `advertiserId` - Required. The ID of the advertiser the campaign belongs to.
    /// * `campaignId` - Required. The ID of the campaign to list assigned targeting options for.
    pub fn campaigns_bulk_list_campaign_assigned_targeting_options(&self, advertiser_id: i64, campaign_id: i64) -> AdvertiserCampaignBulkListCampaignAssignedTargetingOptionCall<'a, S> {
        AdvertiserCampaignBulkListCampaignAssignedTargetingOptionCall {
            hub: self.hub,
            _advertiser_id: advertiser_id,
            _campaign_id: campaign_id,
            _page_token: Default::default(),
            _page_size: Default::default(),
            _order_by: Default::default(),
            _filter: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Creates a new campaign. Returns the newly created campaign if successful.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `advertiserId` - Output only. The unique ID of the advertiser the campaign belongs to.
    pub fn campaigns_create(&self, request: Campaign, advertiser_id: i64) -> AdvertiserCampaignCreateCall<'a, S> {
        AdvertiserCampaignCreateCall {
            hub: self.hub,
            _request: request,
            _advertiser_id: advertiser_id,
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Permanently deletes a campaign. A deleted campaign cannot be recovered. The campaign should be archived first, i.e. set entity_status to `ENTITY_STATUS_ARCHIVED`, to be able to delete it.
    /// 
    /// # Arguments
    ///
    /// * `advertiserId` - The ID of the advertiser this campaign belongs to.
    /// * `campaignId` - The ID of the campaign we need to delete.
    pub fn campaigns_delete(&self, advertiser_id: i64, campaign_id: i64) -> AdvertiserCampaignDeleteCall<'a, S> {
        AdvertiserCampaignDeleteCall {
            hub: self.hub,
            _advertiser_id: advertiser_id,
            _campaign_id: campaign_id,
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Gets a campaign.
    /// 
    /// # Arguments
    ///
    /// * `advertiserId` - Required. The ID of the advertiser this campaign belongs to.
    /// * `campaignId` - Required. The ID of the campaign to fetch.
    pub fn campaigns_get(&self, advertiser_id: i64, campaign_id: i64) -> AdvertiserCampaignGetCall<'a, S> {
        AdvertiserCampaignGetCall {
            hub: self.hub,
            _advertiser_id: advertiser_id,
            _campaign_id: campaign_id,
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Lists campaigns in an advertiser. The order is defined by the order_by parameter. If a filter by entity_status is not specified, campaigns with `ENTITY_STATUS_ARCHIVED` will not be included in the results.
    /// 
    /// # Arguments
    ///
    /// * `advertiserId` - The ID of the advertiser to list campaigns for.
    pub fn campaigns_list(&self, advertiser_id: i64) -> AdvertiserCampaignListCall<'a, S> {
        AdvertiserCampaignListCall {
            hub: self.hub,
            _advertiser_id: advertiser_id,
            _page_token: Default::default(),
            _page_size: Default::default(),
            _order_by: Default::default(),
            _filter: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Updates an existing campaign. Returns the updated campaign if successful.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `advertiserId` - Output only. The unique ID of the advertiser the campaign belongs to.
    /// * `campaignId` - Output only. The unique ID of the campaign. Assigned by the system.
    pub fn campaigns_patch(&self, request: Campaign, advertiser_id: i64, campaign_id: i64) -> AdvertiserCampaignPatchCall<'a, S> {
        AdvertiserCampaignPatchCall {
            hub: self.hub,
            _request: request,
            _advertiser_id: advertiser_id,
            _campaign_id: campaign_id,
            _update_mask: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Bulk edits sites under a single channel. The operation will delete the sites provided in BulkEditSitesRequest.deleted_sites and then create the sites provided in BulkEditSitesRequest.created_sites.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `advertiserId` - The ID of the advertiser that owns the parent channel.
    /// * `channelId` - Required. The ID of the parent channel to which the sites belong.
    pub fn channels_sites_bulk_edit(&self, request: BulkEditSitesRequest, advertiser_id: i64, channel_id: i64) -> AdvertiserChannelSiteBulkEditCall<'a, S> {
        AdvertiserChannelSiteBulkEditCall {
            hub: self.hub,
            _request: request,
            _advertiser_id: advertiser_id,
            _channel_id: channel_id,
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Creates a site in a channel.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `advertiserId` - The ID of the advertiser that owns the parent channel.
    /// * `channelId` - Required. The ID of the parent channel in which the site will be created.
    pub fn channels_sites_create(&self, request: Site, advertiser_id: i64, channel_id: i64) -> AdvertiserChannelSiteCreateCall<'a, S> {
        AdvertiserChannelSiteCreateCall {
            hub: self.hub,
            _request: request,
            _advertiser_id: advertiser_id,
            _channel_id: channel_id,
            _partner_id: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Deletes a site from a channel.
    /// 
    /// # Arguments
    ///
    /// * `advertiserId` - The ID of the advertiser that owns the parent channel.
    /// * `channelId` - Required. The ID of the parent channel to which the site belongs.
    /// * `urlOrAppId` - Required. The URL or app ID of the site to delete.
    pub fn channels_sites_delete(&self, advertiser_id: i64, channel_id: i64, url_or_app_id: &str) -> AdvertiserChannelSiteDeleteCall<'a, S> {
        AdvertiserChannelSiteDeleteCall {
            hub: self.hub,
            _advertiser_id: advertiser_id,
            _channel_id: channel_id,
            _url_or_app_id: url_or_app_id.to_string(),
            _partner_id: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Lists sites in a channel.
    /// 
    /// # Arguments
    ///
    /// * `advertiserId` - The ID of the advertiser that owns the parent channel.
    /// * `channelId` - Required. The ID of the parent channel to which the requested sites belong.
    pub fn channels_sites_list(&self, advertiser_id: i64, channel_id: i64) -> AdvertiserChannelSiteListCall<'a, S> {
        AdvertiserChannelSiteListCall {
            hub: self.hub,
            _advertiser_id: advertiser_id,
            _channel_id: channel_id,
            _partner_id: Default::default(),
            _page_token: Default::default(),
            _page_size: Default::default(),
            _order_by: Default::default(),
            _filter: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Replaces all of the sites under a single channel. The operation will replace the sites under a channel with the sites provided in ReplaceSitesRequest.new_sites.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `advertiserId` - The ID of the advertiser that owns the parent channel.
    /// * `channelId` - Required. The ID of the parent channel whose sites will be replaced.
    pub fn channels_sites_replace(&self, request: ReplaceSitesRequest, advertiser_id: i64, channel_id: i64) -> AdvertiserChannelSiteReplaceCall<'a, S> {
        AdvertiserChannelSiteReplaceCall {
            hub: self.hub,
            _request: request,
            _advertiser_id: advertiser_id,
            _channel_id: channel_id,
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Creates a new channel. Returns the newly created channel if successful.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `advertiserId` - The ID of the advertiser that owns the created channel.
    pub fn channels_create(&self, request: Channel, advertiser_id: i64) -> AdvertiserChannelCreateCall<'a, S> {
        AdvertiserChannelCreateCall {
            hub: self.hub,
            _request: request,
            _advertiser_id: advertiser_id,
            _partner_id: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Gets a channel for a partner or advertiser.
    /// 
    /// # Arguments
    ///
    /// * `advertiserId` - The ID of the advertiser that owns the fetched channel.
    /// * `channelId` - Required. The ID of the channel to fetch.
    pub fn channels_get(&self, advertiser_id: i64, channel_id: i64) -> AdvertiserChannelGetCall<'a, S> {
        AdvertiserChannelGetCall {
            hub: self.hub,
            _advertiser_id: advertiser_id,
            _channel_id: channel_id,
            _partner_id: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Lists channels for a partner or advertiser.
    /// 
    /// # Arguments
    ///
    /// * `advertiserId` - The ID of the advertiser that owns the channels.
    pub fn channels_list(&self, advertiser_id: i64) -> AdvertiserChannelListCall<'a, S> {
        AdvertiserChannelListCall {
            hub: self.hub,
            _advertiser_id: advertiser_id,
            _partner_id: Default::default(),
            _page_token: Default::default(),
            _page_size: Default::default(),
            _order_by: Default::default(),
            _filter: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Updates a channel. Returns the updated channel if successful.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `advertiserId` - The ID of the advertiser that owns the created channel.
    /// * `channelId` - Output only. The unique ID of the channel. Assigned by the system.
    pub fn channels_patch(&self, request: Channel, advertiser_id: i64, channel_id: i64) -> AdvertiserChannelPatchCall<'a, S> {
        AdvertiserChannelPatchCall {
            hub: self.hub,
            _request: request,
            _advertiser_id: advertiser_id,
            _channel_id: channel_id,
            _update_mask: Default::default(),
            _partner_id: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Creates a new creative. Returns the newly created creative if successful.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `advertiserId` - Output only. The unique ID of the advertiser the creative belongs to.
    pub fn creatives_create(&self, request: Creative, advertiser_id: i64) -> AdvertiserCreativeCreateCall<'a, S> {
        AdvertiserCreativeCreateCall {
            hub: self.hub,
            _request: request,
            _advertiser_id: advertiser_id,
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Deletes a creative. Returns error code `NOT_FOUND` if the creative does not exist. The creative should be archived first, i.e. set entity_status to `ENTITY_STATUS_ARCHIVED`, before it can be deleted.
    /// 
    /// # Arguments
    ///
    /// * `advertiserId` - The ID of the advertiser this creative belongs to.
    /// * `creativeId` - The ID of the creative to be deleted.
    pub fn creatives_delete(&self, advertiser_id: i64, creative_id: i64) -> AdvertiserCreativeDeleteCall<'a, S> {
        AdvertiserCreativeDeleteCall {
            hub: self.hub,
            _advertiser_id: advertiser_id,
            _creative_id: creative_id,
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Gets a creative.
    /// 
    /// # Arguments
    ///
    /// * `advertiserId` - Required. The ID of the advertiser this creative belongs to.
    /// * `creativeId` - Required. The ID of the creative to fetch.
    pub fn creatives_get(&self, advertiser_id: i64, creative_id: i64) -> AdvertiserCreativeGetCall<'a, S> {
        AdvertiserCreativeGetCall {
            hub: self.hub,
            _advertiser_id: advertiser_id,
            _creative_id: creative_id,
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Lists creatives in an advertiser. The order is defined by the order_by parameter. If a filter by entity_status is not specified, creatives with `ENTITY_STATUS_ARCHIVED` will not be included in the results.
    /// 
    /// # Arguments
    ///
    /// * `advertiserId` - Required. The ID of the advertiser to list creatives for.
    pub fn creatives_list(&self, advertiser_id: i64) -> AdvertiserCreativeListCall<'a, S> {
        AdvertiserCreativeListCall {
            hub: self.hub,
            _advertiser_id: advertiser_id,
            _page_token: Default::default(),
            _page_size: Default::default(),
            _order_by: Default::default(),
            _filter: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Updates an existing creative. Returns the updated creative if successful.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `advertiserId` - Output only. The unique ID of the advertiser the creative belongs to.
    /// * `creativeId` - Output only. The unique ID of the creative. Assigned by the system.
    pub fn creatives_patch(&self, request: Creative, advertiser_id: i64, creative_id: i64) -> AdvertiserCreativePatchCall<'a, S> {
        AdvertiserCreativePatchCall {
            hub: self.hub,
            _request: request,
            _advertiser_id: advertiser_id,
            _creative_id: creative_id,
            _update_mask: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Gets a single targeting option assigned to an insertion order.
    /// 
    /// # Arguments
    ///
    /// * `advertiserId` - Required. The ID of the advertiser the insertion order belongs to.
    /// * `insertionOrderId` - Required. The ID of the insertion order the assigned targeting option belongs to.
    /// * `targetingType` - Required. Identifies the type of this assigned targeting option.
    /// * `assignedTargetingOptionId` - Required. An identifier unique to the targeting type in this insertion order that identifies the assigned targeting option being requested.
    pub fn insertion_orders_targeting_types_assigned_targeting_options_get(&self, advertiser_id: i64, insertion_order_id: i64, targeting_type: &AdvertiserTargetingTypeEnum, assigned_targeting_option_id: &str) -> AdvertiserInsertionOrderTargetingTypeAssignedTargetingOptionGetCall<'a, S> {
        AdvertiserInsertionOrderTargetingTypeAssignedTargetingOptionGetCall {
            hub: self.hub,
            _advertiser_id: advertiser_id,
            _insertion_order_id: insertion_order_id,
            _targeting_type: targeting_type.clone(),
            _assigned_targeting_option_id: assigned_targeting_option_id.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Lists the targeting options assigned to an insertion order.
    /// 
    /// # Arguments
    ///
    /// * `advertiserId` - Required. The ID of the advertiser the insertion order belongs to.
    /// * `insertionOrderId` - Required. The ID of the insertion order to list assigned targeting options for.
    /// * `targetingType` - Required. Identifies the type of assigned targeting options to list.
    pub fn insertion_orders_targeting_types_assigned_targeting_options_list(&self, advertiser_id: i64, insertion_order_id: i64, targeting_type: &AdvertiserTargetingTypeEnum) -> AdvertiserInsertionOrderTargetingTypeAssignedTargetingOptionListCall<'a, S> {
        AdvertiserInsertionOrderTargetingTypeAssignedTargetingOptionListCall {
            hub: self.hub,
            _advertiser_id: advertiser_id,
            _insertion_order_id: insertion_order_id,
            _targeting_type: targeting_type.clone(),
            _page_token: Default::default(),
            _page_size: Default::default(),
            _order_by: Default::default(),
            _filter: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Lists assigned targeting options of an insertion order across targeting types.
    /// 
    /// # Arguments
    ///
    /// * `advertiserId` - Required. The ID of the advertiser the insertion order belongs to.
    /// * `insertionOrderId` - Required. The ID of the insertion order to list assigned targeting options for.
    pub fn insertion_orders_bulk_list_insertion_order_assigned_targeting_options(&self, advertiser_id: i64, insertion_order_id: i64) -> AdvertiserInsertionOrderBulkListInsertionOrderAssignedTargetingOptionCall<'a, S> {
        AdvertiserInsertionOrderBulkListInsertionOrderAssignedTargetingOptionCall {
            hub: self.hub,
            _advertiser_id: advertiser_id,
            _insertion_order_id: insertion_order_id,
            _page_token: Default::default(),
            _page_size: Default::default(),
            _order_by: Default::default(),
            _filter: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Creates a new insertion order. Returns the newly created insertion order if successful.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `advertiserId` - Output only. The unique ID of the advertiser the insertion order belongs to.
    pub fn insertion_orders_create(&self, request: InsertionOrder, advertiser_id: i64) -> AdvertiserInsertionOrderCreateCall<'a, S> {
        AdvertiserInsertionOrderCreateCall {
            hub: self.hub,
            _request: request,
            _advertiser_id: advertiser_id,
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Deletes an insertion order. Returns error code `NOT_FOUND` if the insertion order does not exist. The insertion order should be archived first, i.e. set entity_status to `ENTITY_STATUS_ARCHIVED`, to be able to delete it.
    /// 
    /// # Arguments
    ///
    /// * `advertiserId` - The ID of the advertiser this insertion order belongs to.
    /// * `insertionOrderId` - The ID of the insertion order to delete.
    pub fn insertion_orders_delete(&self, advertiser_id: i64, insertion_order_id: i64) -> AdvertiserInsertionOrderDeleteCall<'a, S> {
        AdvertiserInsertionOrderDeleteCall {
            hub: self.hub,
            _advertiser_id: advertiser_id,
            _insertion_order_id: insertion_order_id,
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Gets an insertion order. Returns error code `NOT_FOUND` if the insertion order does not exist.
    /// 
    /// # Arguments
    ///
    /// * `advertiserId` - Required. The ID of the advertiser this insertion order belongs to.
    /// * `insertionOrderId` - Required. The ID of the insertion order to fetch.
    pub fn insertion_orders_get(&self, advertiser_id: i64, insertion_order_id: i64) -> AdvertiserInsertionOrderGetCall<'a, S> {
        AdvertiserInsertionOrderGetCall {
            hub: self.hub,
            _advertiser_id: advertiser_id,
            _insertion_order_id: insertion_order_id,
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Lists insertion orders in an advertiser. The order is defined by the order_by parameter. If a filter by entity_status is not specified, insertion orders with `ENTITY_STATUS_ARCHIVED` will not be included in the results.
    /// 
    /// # Arguments
    ///
    /// * `advertiserId` - Required. The ID of the advertiser to list insertion orders for.
    pub fn insertion_orders_list(&self, advertiser_id: i64) -> AdvertiserInsertionOrderListCall<'a, S> {
        AdvertiserInsertionOrderListCall {
            hub: self.hub,
            _advertiser_id: advertiser_id,
            _page_token: Default::default(),
            _page_size: Default::default(),
            _order_by: Default::default(),
            _filter: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Updates an existing insertion order. Returns the updated insertion order if successful.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `advertiserId` - Output only. The unique ID of the advertiser the insertion order belongs to.
    /// * `insertionOrderId` - Output only. The unique ID of the insertion order. Assigned by the system.
    pub fn insertion_orders_patch(&self, request: InsertionOrder, advertiser_id: i64, insertion_order_id: i64) -> AdvertiserInsertionOrderPatchCall<'a, S> {
        AdvertiserInsertionOrderPatchCall {
            hub: self.hub,
            _request: request,
            _advertiser_id: advertiser_id,
            _insertion_order_id: insertion_order_id,
            _update_mask: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Lists invoices posted for an advertiser in a given month. Invoices generated by billing profiles with a "Partner" invoice level are not retrievable through this method.
    /// 
    /// # Arguments
    ///
    /// * `advertiserId` - Required. The ID of the advertiser to list invoices for.
    pub fn invoices_list(&self, advertiser_id: i64) -> AdvertiserInvoiceListCall<'a, S> {
        AdvertiserInvoiceListCall {
            hub: self.hub,
            _advertiser_id: advertiser_id,
            _page_token: Default::default(),
            _page_size: Default::default(),
            _loi_sapin_invoice_type: Default::default(),
            _issue_month: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Retrieves the invoice currency used by an advertiser in a given month.
    /// 
    /// # Arguments
    ///
    /// * `advertiserId` - Required. The ID of the advertiser to lookup currency for.
    pub fn invoices_lookup_invoice_currency(&self, advertiser_id: i64) -> AdvertiserInvoiceLookupInvoiceCurrencyCall<'a, S> {
        AdvertiserInvoiceLookupInvoiceCurrencyCall {
            hub: self.hub,
            _advertiser_id: advertiser_id,
            _invoice_month: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Assigns a targeting option to a line item. Returns the assigned targeting option if successful. Requests to this endpoint cannot be made concurrently with the following requests updating the same line item: * BulkEditAssignedTargetingOptions * BulkUpdate * UpdateLineItem * DeleteLineItemAssignedTargetingOption
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `advertiserId` - Required. The ID of the advertiser the line item belongs to.
    /// * `lineItemId` - Required. The ID of the line item the assigned targeting option will belong to.
    /// * `targetingType` - Required. Identifies the type of this assigned targeting option.
    pub fn line_items_targeting_types_assigned_targeting_options_create(&self, request: AssignedTargetingOption, advertiser_id: i64, line_item_id: i64, targeting_type: &AdvertiserTargetingTypeEnum) -> AdvertiserLineItemTargetingTypeAssignedTargetingOptionCreateCall<'a, S> {
        AdvertiserLineItemTargetingTypeAssignedTargetingOptionCreateCall {
            hub: self.hub,
            _request: request,
            _advertiser_id: advertiser_id,
            _line_item_id: line_item_id,
            _targeting_type: targeting_type.clone(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Deletes an assigned targeting option from a line item. Requests to this endpoint cannot be made concurrently with the following requests updating the same line item: * BulkEditAssignedTargetingOptions * BulkUpdate * UpdateLineItem * CreateLineItemAssignedTargetingOption
    /// 
    /// # Arguments
    ///
    /// * `advertiserId` - Required. The ID of the advertiser the line item belongs to.
    /// * `lineItemId` - Required. The ID of the line item the assigned targeting option belongs to.
    /// * `targetingType` - Required. Identifies the type of this assigned targeting option.
    /// * `assignedTargetingOptionId` - Required. The ID of the assigned targeting option to delete.
    pub fn line_items_targeting_types_assigned_targeting_options_delete(&self, advertiser_id: i64, line_item_id: i64, targeting_type: &AdvertiserTargetingTypeEnum, assigned_targeting_option_id: &str) -> AdvertiserLineItemTargetingTypeAssignedTargetingOptionDeleteCall<'a, S> {
        AdvertiserLineItemTargetingTypeAssignedTargetingOptionDeleteCall {
            hub: self.hub,
            _advertiser_id: advertiser_id,
            _line_item_id: line_item_id,
            _targeting_type: targeting_type.clone(),
            _assigned_targeting_option_id: assigned_targeting_option_id.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Gets a single targeting option assigned to a line item.
    /// 
    /// # Arguments
    ///
    /// * `advertiserId` - Required. The ID of the advertiser the line item belongs to.
    /// * `lineItemId` - Required. The ID of the line item the assigned targeting option belongs to.
    /// * `targetingType` - Required. Identifies the type of this assigned targeting option.
    /// * `assignedTargetingOptionId` - Required. An identifier unique to the targeting type in this line item that identifies the assigned targeting option being requested.
    pub fn line_items_targeting_types_assigned_targeting_options_get(&self, advertiser_id: i64, line_item_id: i64, targeting_type: &AdvertiserTargetingTypeEnum, assigned_targeting_option_id: &str) -> AdvertiserLineItemTargetingTypeAssignedTargetingOptionGetCall<'a, S> {
        AdvertiserLineItemTargetingTypeAssignedTargetingOptionGetCall {
            hub: self.hub,
            _advertiser_id: advertiser_id,
            _line_item_id: line_item_id,
            _targeting_type: targeting_type.clone(),
            _assigned_targeting_option_id: assigned_targeting_option_id.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Lists the targeting options assigned to a line item.
    /// 
    /// # Arguments
    ///
    /// * `advertiserId` - Required. The ID of the advertiser the line item belongs to.
    /// * `lineItemId` - Required. The ID of the line item to list assigned targeting options for.
    /// * `targetingType` - Required. Identifies the type of assigned targeting options to list.
    pub fn line_items_targeting_types_assigned_targeting_options_list(&self, advertiser_id: i64, line_item_id: i64, targeting_type: &AdvertiserTargetingTypeEnum) -> AdvertiserLineItemTargetingTypeAssignedTargetingOptionListCall<'a, S> {
        AdvertiserLineItemTargetingTypeAssignedTargetingOptionListCall {
            hub: self.hub,
            _advertiser_id: advertiser_id,
            _line_item_id: line_item_id,
            _targeting_type: targeting_type.clone(),
            _page_token: Default::default(),
            _page_size: Default::default(),
            _order_by: Default::default(),
            _filter: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Bulk edits targeting options under a single line item. The operation will delete the assigned targeting options provided in BulkEditLineItemAssignedTargetingOptionsRequest.delete_requests and then create the assigned targeting options provided in BulkEditLineItemAssignedTargetingOptionsRequest.create_requests. Requests to this endpoint cannot be made concurrently with the following requests updating the same line item: * UpdateLineItem * CreateLineItemAssignedTargetingOption * DeleteLineItemAssignedTargetingOption
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `advertiserId` - Required. The ID of the advertiser the line item belongs to.
    /// * `lineItemId` - Required. The ID of the line item the assigned targeting option will belong to.
    pub fn line_items_bulk_edit_line_item_assigned_targeting_options(&self, request: BulkEditLineItemAssignedTargetingOptionsRequest, advertiser_id: i64, line_item_id: i64) -> AdvertiserLineItemBulkEditLineItemAssignedTargetingOptionCall<'a, S> {
        AdvertiserLineItemBulkEditLineItemAssignedTargetingOptionCall {
            hub: self.hub,
            _request: request,
            _advertiser_id: advertiser_id,
            _line_item_id: line_item_id,
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Lists assigned targeting options of a line item across targeting types.
    /// 
    /// # Arguments
    ///
    /// * `advertiserId` - Required. The ID of the advertiser the line item belongs to.
    /// * `lineItemId` - Required. The ID of the line item to list assigned targeting options for.
    pub fn line_items_bulk_list_line_item_assigned_targeting_options(&self, advertiser_id: i64, line_item_id: i64) -> AdvertiserLineItemBulkListLineItemAssignedTargetingOptionCall<'a, S> {
        AdvertiserLineItemBulkListLineItemAssignedTargetingOptionCall {
            hub: self.hub,
            _advertiser_id: advertiser_id,
            _line_item_id: line_item_id,
            _page_token: Default::default(),
            _page_size: Default::default(),
            _order_by: Default::default(),
            _filter: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Creates a new line item. Returns the newly created line item if successful.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `advertiserId` - Output only. The unique ID of the advertiser the line item belongs to.
    pub fn line_items_create(&self, request: LineItem, advertiser_id: i64) -> AdvertiserLineItemCreateCall<'a, S> {
        AdvertiserLineItemCreateCall {
            hub: self.hub,
            _request: request,
            _advertiser_id: advertiser_id,
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Deletes a line item. Returns error code `NOT_FOUND` if the line item does not exist. The line item should be archived first, i.e. set entity_status to `ENTITY_STATUS_ARCHIVED`, to be able to delete it.
    /// 
    /// # Arguments
    ///
    /// * `advertiserId` - The ID of the advertiser this line item belongs to.
    /// * `lineItemId` - The ID of the line item to delete.
    pub fn line_items_delete(&self, advertiser_id: i64, line_item_id: i64) -> AdvertiserLineItemDeleteCall<'a, S> {
        AdvertiserLineItemDeleteCall {
            hub: self.hub,
            _advertiser_id: advertiser_id,
            _line_item_id: line_item_id,
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Creates a new line item with settings (including targeting) inherited from the insertion order and an `ENTITY_STATUS_DRAFT` entity_status. Returns the newly created line item if successful. There are default values based on the three fields: * The insertion order's insertion_order_type * The insertion order's automation_type * The given line_item_type
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `advertiserId` - Required. The ID of the advertiser this line item belongs to.
    pub fn line_items_generate_default(&self, request: GenerateDefaultLineItemRequest, advertiser_id: i64) -> AdvertiserLineItemGenerateDefaultCall<'a, S> {
        AdvertiserLineItemGenerateDefaultCall {
            hub: self.hub,
            _request: request,
            _advertiser_id: advertiser_id,
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Gets a line item.
    /// 
    /// # Arguments
    ///
    /// * `advertiserId` - Required. The ID of the advertiser this line item belongs to.
    /// * `lineItemId` - Required. The ID of the line item to fetch.
    pub fn line_items_get(&self, advertiser_id: i64, line_item_id: i64) -> AdvertiserLineItemGetCall<'a, S> {
        AdvertiserLineItemGetCall {
            hub: self.hub,
            _advertiser_id: advertiser_id,
            _line_item_id: line_item_id,
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Lists line items in an advertiser. The order is defined by the order_by parameter. If a filter by entity_status is not specified, line items with `ENTITY_STATUS_ARCHIVED` will not be included in the results.
    /// 
    /// # Arguments
    ///
    /// * `advertiserId` - Required. The ID of the advertiser to list line items for.
    pub fn line_items_list(&self, advertiser_id: i64) -> AdvertiserLineItemListCall<'a, S> {
        AdvertiserLineItemListCall {
            hub: self.hub,
            _advertiser_id: advertiser_id,
            _page_token: Default::default(),
            _page_size: Default::default(),
            _order_by: Default::default(),
            _filter: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Updates an existing line item. Returns the updated line item if successful. Requests to this endpoint cannot be made concurrently with the following requests updating the same line item: * BulkEditAssignedTargetingOptions * BulkUpdateLineItems * CreateLineItemAssignedTargetingOption * DeleteLineItemAssignedTargetingOption
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `advertiserId` - Output only. The unique ID of the advertiser the line item belongs to.
    /// * `lineItemId` - Output only. The unique ID of the line item. Assigned by the system.
    pub fn line_items_patch(&self, request: LineItem, advertiser_id: i64, line_item_id: i64) -> AdvertiserLineItemPatchCall<'a, S> {
        AdvertiserLineItemPatchCall {
            hub: self.hub,
            _request: request,
            _advertiser_id: advertiser_id,
            _line_item_id: line_item_id,
            _update_mask: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Bulk edits multiple assignments between locations and a single location list. The operation will delete the assigned locations provided in BulkEditAssignedLocationsRequest.deleted_assigned_locations and then create the assigned locations provided in BulkEditAssignedLocationsRequest.created_assigned_locations.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `advertiserId` - Required. The ID of the DV360 advertiser to which the location list belongs.
    /// * `locationListId` - Required. The ID of the location list to which these assignments are assigned.
    pub fn location_lists_assigned_locations_bulk_edit(&self, request: BulkEditAssignedLocationsRequest, advertiser_id: i64, location_list_id: i64) -> AdvertiserLocationListAssignedLocationBulkEditCall<'a, S> {
        AdvertiserLocationListAssignedLocationBulkEditCall {
            hub: self.hub,
            _request: request,
            _advertiser_id: advertiser_id,
            _location_list_id: location_list_id,
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Creates an assignment between a location and a location list.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `advertiserId` - Required. The ID of the DV360 advertiser to which the location list belongs.
    /// * `locationListId` - Required. The ID of the location list for which the assignment will be created.
    pub fn location_lists_assigned_locations_create(&self, request: AssignedLocation, advertiser_id: i64, location_list_id: i64) -> AdvertiserLocationListAssignedLocationCreateCall<'a, S> {
        AdvertiserLocationListAssignedLocationCreateCall {
            hub: self.hub,
            _request: request,
            _advertiser_id: advertiser_id,
            _location_list_id: location_list_id,
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Deletes the assignment between a location and a location list.
    /// 
    /// # Arguments
    ///
    /// * `advertiserId` - Required. The ID of the DV360 advertiser to which the location list belongs.
    /// * `locationListId` - Required. The ID of the location list to which this assignment is assigned.
    /// * `assignedLocationId` - Required. The ID of the assigned location to delete.
    pub fn location_lists_assigned_locations_delete(&self, advertiser_id: i64, location_list_id: i64, assigned_location_id: i64) -> AdvertiserLocationListAssignedLocationDeleteCall<'a, S> {
        AdvertiserLocationListAssignedLocationDeleteCall {
            hub: self.hub,
            _advertiser_id: advertiser_id,
            _location_list_id: location_list_id,
            _assigned_location_id: assigned_location_id,
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Lists locations assigned to a location list.
    /// 
    /// # Arguments
    ///
    /// * `advertiserId` - Required. The ID of the DV360 advertiser to which the location list belongs.
    /// * `locationListId` - Required. The ID of the location list to which these assignments are assigned.
    pub fn location_lists_assigned_locations_list(&self, advertiser_id: i64, location_list_id: i64) -> AdvertiserLocationListAssignedLocationListCall<'a, S> {
        AdvertiserLocationListAssignedLocationListCall {
            hub: self.hub,
            _advertiser_id: advertiser_id,
            _location_list_id: location_list_id,
            _page_token: Default::default(),
            _page_size: Default::default(),
            _order_by: Default::default(),
            _filter: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Creates a new location list. Returns the newly created location list if successful.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `advertiserId` - Required. The ID of the DV360 advertiser to which the location list belongs.
    pub fn location_lists_create(&self, request: LocationList, advertiser_id: i64) -> AdvertiserLocationListCreateCall<'a, S> {
        AdvertiserLocationListCreateCall {
            hub: self.hub,
            _request: request,
            _advertiser_id: advertiser_id,
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Gets a location list.
    /// 
    /// # Arguments
    ///
    /// * `advertiserId` - Required. The ID of the DV360 advertiser to which the fetched location list belongs.
    /// * `locationListId` - Required. The ID of the location list to fetch.
    pub fn location_lists_get(&self, advertiser_id: i64, location_list_id: i64) -> AdvertiserLocationListGetCall<'a, S> {
        AdvertiserLocationListGetCall {
            hub: self.hub,
            _advertiser_id: advertiser_id,
            _location_list_id: location_list_id,
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Lists location lists based on a given advertiser id.
    /// 
    /// # Arguments
    ///
    /// * `advertiserId` - Required. The ID of the DV360 advertiser to which the fetched location lists belong.
    pub fn location_lists_list(&self, advertiser_id: i64) -> AdvertiserLocationListListCall<'a, S> {
        AdvertiserLocationListListCall {
            hub: self.hub,
            _advertiser_id: advertiser_id,
            _page_token: Default::default(),
            _page_size: Default::default(),
            _order_by: Default::default(),
            _filter: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Updates a location list. Returns the updated location list if successful.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `advertiserId` - Required. The ID of the DV360 advertiser to which the location lists belongs.
    /// * `locationListId` - Output only. The unique ID of the location list. Assigned by the system.
    pub fn location_lists_patch(&self, request: LocationList, advertiser_id: i64, location_list_id: i64) -> AdvertiserLocationListPatchCall<'a, S> {
        AdvertiserLocationListPatchCall {
            hub: self.hub,
            _request: request,
            _advertiser_id: advertiser_id,
            _location_list_id: location_list_id,
            _update_mask: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Activates a manual trigger. Each activation of the manual trigger must be at least 5 minutes apart, otherwise an error will be returned.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `advertiserId` - Required. The ID of the advertiser that the manual trigger belongs.
    /// * `triggerId` - Required. The ID of the manual trigger to activate.
    pub fn manual_triggers_activate(&self, request: ActivateManualTriggerRequest, advertiser_id: i64, trigger_id: i64) -> AdvertiserManualTriggerActivateCall<'a, S> {
        AdvertiserManualTriggerActivateCall {
            hub: self.hub,
            _request: request,
            _advertiser_id: advertiser_id,
            _trigger_id: trigger_id,
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Creates a new manual trigger. Returns the newly created manual trigger if successful.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `advertiserId` - Required. Immutable. The unique ID of the advertiser that the manual trigger belongs to.
    pub fn manual_triggers_create(&self, request: ManualTrigger, advertiser_id: i64) -> AdvertiserManualTriggerCreateCall<'a, S> {
        AdvertiserManualTriggerCreateCall {
            hub: self.hub,
            _request: request,
            _advertiser_id: advertiser_id,
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Deactivates a manual trigger.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `advertiserId` - Required. The ID of the advertiser that the manual trigger belongs.
    /// * `triggerId` - Required. The ID of the manual trigger to deactivate.
    pub fn manual_triggers_deactivate(&self, request: DeactivateManualTriggerRequest, advertiser_id: i64, trigger_id: i64) -> AdvertiserManualTriggerDeactivateCall<'a, S> {
        AdvertiserManualTriggerDeactivateCall {
            hub: self.hub,
            _request: request,
            _advertiser_id: advertiser_id,
            _trigger_id: trigger_id,
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Gets a manual trigger.
    /// 
    /// # Arguments
    ///
    /// * `advertiserId` - Required. The ID of the advertiser this manual trigger belongs to.
    /// * `triggerId` - Required. The ID of the manual trigger to fetch.
    pub fn manual_triggers_get(&self, advertiser_id: i64, trigger_id: i64) -> AdvertiserManualTriggerGetCall<'a, S> {
        AdvertiserManualTriggerGetCall {
            hub: self.hub,
            _advertiser_id: advertiser_id,
            _trigger_id: trigger_id,
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Lists manual triggers that are accessible to the current user for a given advertiser ID. The order is defined by the order_by parameter. A single advertiser_id is required.
    /// 
    /// # Arguments
    ///
    /// * `advertiserId` - Required. The ID of the advertiser that the fetched manual triggers belong to.
    pub fn manual_triggers_list(&self, advertiser_id: i64) -> AdvertiserManualTriggerListCall<'a, S> {
        AdvertiserManualTriggerListCall {
            hub: self.hub,
            _advertiser_id: advertiser_id,
            _page_token: Default::default(),
            _page_size: Default::default(),
            _order_by: Default::default(),
            _filter: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Updates a manual trigger. Returns the updated manual trigger if successful.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `advertiserId` - Required. Immutable. The unique ID of the advertiser that the manual trigger belongs to.
    /// * `triggerId` - Output only. The unique ID of the manual trigger.
    pub fn manual_triggers_patch(&self, request: ManualTrigger, advertiser_id: i64, trigger_id: i64) -> AdvertiserManualTriggerPatchCall<'a, S> {
        AdvertiserManualTriggerPatchCall {
            hub: self.hub,
            _request: request,
            _advertiser_id: advertiser_id,
            _trigger_id: trigger_id,
            _update_mask: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Bulk edits negative keywords in a single negative keyword list. The operation will delete the negative keywords provided in BulkEditNegativeKeywordsRequest.deleted_negative_keywords and then create the negative keywords provided in BulkEditNegativeKeywordsRequest.created_negative_keywords. This operation is guaranteed to be atomic and will never result in a partial success or partial failure.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `advertiserId` - Required. The ID of the DV360 advertiser to which the parent negative keyword list belongs.
    /// * `negativeKeywordListId` - Required. The ID of the parent negative keyword list to which the negative keywords belong.
    pub fn negative_keyword_lists_negative_keywords_bulk_edit(&self, request: BulkEditNegativeKeywordsRequest, advertiser_id: i64, negative_keyword_list_id: i64) -> AdvertiserNegativeKeywordListNegativeKeywordBulkEditCall<'a, S> {
        AdvertiserNegativeKeywordListNegativeKeywordBulkEditCall {
            hub: self.hub,
            _request: request,
            _advertiser_id: advertiser_id,
            _negative_keyword_list_id: negative_keyword_list_id,
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Creates a negative keyword in a negative keyword list.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `advertiserId` - Required. The ID of the DV360 advertiser to which the parent negative keyword list belongs.
    /// * `negativeKeywordListId` - Required. The ID of the parent negative keyword list in which the negative keyword will be created.
    pub fn negative_keyword_lists_negative_keywords_create(&self, request: NegativeKeyword, advertiser_id: i64, negative_keyword_list_id: i64) -> AdvertiserNegativeKeywordListNegativeKeywordCreateCall<'a, S> {
        AdvertiserNegativeKeywordListNegativeKeywordCreateCall {
            hub: self.hub,
            _request: request,
            _advertiser_id: advertiser_id,
            _negative_keyword_list_id: negative_keyword_list_id,
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Deletes a negative keyword from a negative keyword list.
    /// 
    /// # Arguments
    ///
    /// * `advertiserId` - Required. The ID of the DV360 advertiser to which the parent negative keyword list belongs.
    /// * `negativeKeywordListId` - Required. The ID of the parent negative keyword list to which the negative keyword belongs.
    /// * `keywordValue` - Required. The keyword value of the negative keyword to delete.
    pub fn negative_keyword_lists_negative_keywords_delete(&self, advertiser_id: i64, negative_keyword_list_id: i64, keyword_value: &str) -> AdvertiserNegativeKeywordListNegativeKeywordDeleteCall<'a, S> {
        AdvertiserNegativeKeywordListNegativeKeywordDeleteCall {
            hub: self.hub,
            _advertiser_id: advertiser_id,
            _negative_keyword_list_id: negative_keyword_list_id,
            _keyword_value: keyword_value.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Lists negative keywords in a negative keyword list.
    /// 
    /// # Arguments
    ///
    /// * `advertiserId` - Required. The ID of the DV360 advertiser to which the parent negative keyword list belongs.
    /// * `negativeKeywordListId` - Required. The ID of the parent negative keyword list to which the requested negative keywords belong.
    pub fn negative_keyword_lists_negative_keywords_list(&self, advertiser_id: i64, negative_keyword_list_id: i64) -> AdvertiserNegativeKeywordListNegativeKeywordListCall<'a, S> {
        AdvertiserNegativeKeywordListNegativeKeywordListCall {
            hub: self.hub,
            _advertiser_id: advertiser_id,
            _negative_keyword_list_id: negative_keyword_list_id,
            _page_token: Default::default(),
            _page_size: Default::default(),
            _order_by: Default::default(),
            _filter: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Replaces all negative keywords in a single negative keyword list. The operation will replace the keywords in a negative keyword list with keywords provided in ReplaceNegativeKeywordsRequest.new_negative_keywords.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `advertiserId` - Required. The ID of the DV360 advertiser to which the parent negative keyword list belongs.
    /// * `negativeKeywordListId` - Required. The ID of the parent negative keyword list to which the negative keywords belong.
    pub fn negative_keyword_lists_negative_keywords_replace(&self, request: ReplaceNegativeKeywordsRequest, advertiser_id: i64, negative_keyword_list_id: i64) -> AdvertiserNegativeKeywordListNegativeKeywordReplaceCall<'a, S> {
        AdvertiserNegativeKeywordListNegativeKeywordReplaceCall {
            hub: self.hub,
            _request: request,
            _advertiser_id: advertiser_id,
            _negative_keyword_list_id: negative_keyword_list_id,
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Creates a new negative keyword list. Returns the newly created negative keyword list if successful.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `advertiserId` - Required. The ID of the DV360 advertiser to which the negative keyword list will belong.
    pub fn negative_keyword_lists_create(&self, request: NegativeKeywordList, advertiser_id: i64) -> AdvertiserNegativeKeywordListCreateCall<'a, S> {
        AdvertiserNegativeKeywordListCreateCall {
            hub: self.hub,
            _request: request,
            _advertiser_id: advertiser_id,
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Deletes a negative keyword list given an advertiser ID and a negative keyword list ID.
    /// 
    /// # Arguments
    ///
    /// * `advertiserId` - Required. The ID of the DV360 advertiser to which the negative keyword list belongs.
    /// * `negativeKeywordListId` - Required. The ID of the negative keyword list to delete.
    pub fn negative_keyword_lists_delete(&self, advertiser_id: i64, negative_keyword_list_id: i64) -> AdvertiserNegativeKeywordListDeleteCall<'a, S> {
        AdvertiserNegativeKeywordListDeleteCall {
            hub: self.hub,
            _advertiser_id: advertiser_id,
            _negative_keyword_list_id: negative_keyword_list_id,
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Gets a negative keyword list given an advertiser ID and a negative keyword list ID.
    /// 
    /// # Arguments
    ///
    /// * `advertiserId` - Required. The ID of the DV360 advertiser to which the fetched negative keyword list belongs.
    /// * `negativeKeywordListId` - Required. The ID of the negative keyword list to fetch.
    pub fn negative_keyword_lists_get(&self, advertiser_id: i64, negative_keyword_list_id: i64) -> AdvertiserNegativeKeywordListGetCall<'a, S> {
        AdvertiserNegativeKeywordListGetCall {
            hub: self.hub,
            _advertiser_id: advertiser_id,
            _negative_keyword_list_id: negative_keyword_list_id,
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Lists negative keyword lists based on a given advertiser id.
    /// 
    /// # Arguments
    ///
    /// * `advertiserId` - Required. The ID of the DV360 advertiser to which the fetched negative keyword lists belong.
    pub fn negative_keyword_lists_list(&self, advertiser_id: i64) -> AdvertiserNegativeKeywordListListCall<'a, S> {
        AdvertiserNegativeKeywordListListCall {
            hub: self.hub,
            _advertiser_id: advertiser_id,
            _page_token: Default::default(),
            _page_size: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Updates a negative keyword list. Returns the updated negative keyword list if successful.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `advertiserId` - Required. The ID of the DV360 advertiser to which the negative keyword list belongs.
    /// * `negativeKeywordListId` - Output only. The unique ID of the negative keyword list. Assigned by the system.
    pub fn negative_keyword_lists_patch(&self, request: NegativeKeywordList, advertiser_id: i64, negative_keyword_list_id: i64) -> AdvertiserNegativeKeywordListPatchCall<'a, S> {
        AdvertiserNegativeKeywordListPatchCall {
            hub: self.hub,
            _request: request,
            _advertiser_id: advertiser_id,
            _negative_keyword_list_id: negative_keyword_list_id,
            _update_mask: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Assigns a targeting option to an advertiser. Returns the assigned targeting option if successful.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `advertiserId` - Required. The ID of the advertiser.
    /// * `targetingType` - Required. Identifies the type of this assigned targeting option. Supported targeting types: * `TARGETING_TYPE_CHANNEL` * `TARGETING_TYPE_DIGITAL_CONTENT_LABEL_EXCLUSION` * `TARGETING_TYPE_OMID` * `TARGETING_TYPE_SENSITIVE_CATEGORY_EXCLUSION`
    pub fn targeting_types_assigned_targeting_options_create(&self, request: AssignedTargetingOption, advertiser_id: i64, targeting_type: &AdvertiserTargetingTypeEnum) -> AdvertiserTargetingTypeAssignedTargetingOptionCreateCall<'a, S> {
        AdvertiserTargetingTypeAssignedTargetingOptionCreateCall {
            hub: self.hub,
            _request: request,
            _advertiser_id: advertiser_id,
            _targeting_type: targeting_type.clone(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Deletes an assigned targeting option from an advertiser.
    /// 
    /// # Arguments
    ///
    /// * `advertiserId` - Required. The ID of the advertiser.
    /// * `targetingType` - Required. Identifies the type of this assigned targeting option. Supported targeting types: * `TARGETING_TYPE_CHANNEL` * `TARGETING_TYPE_DIGITAL_CONTENT_LABEL_EXCLUSION` * `TARGETING_TYPE_OMID` * `TARGETING_TYPE_SENSITIVE_CATEGORY_EXCLUSION`
    /// * `assignedTargetingOptionId` - Required. The ID of the assigned targeting option to delete.
    pub fn targeting_types_assigned_targeting_options_delete(&self, advertiser_id: i64, targeting_type: &AdvertiserTargetingTypeEnum, assigned_targeting_option_id: &str) -> AdvertiserTargetingTypeAssignedTargetingOptionDeleteCall<'a, S> {
        AdvertiserTargetingTypeAssignedTargetingOptionDeleteCall {
            hub: self.hub,
            _advertiser_id: advertiser_id,
            _targeting_type: targeting_type.clone(),
            _assigned_targeting_option_id: assigned_targeting_option_id.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Gets a single targeting option assigned to an advertiser.
    /// 
    /// # Arguments
    ///
    /// * `advertiserId` - Required. The ID of the advertiser.
    /// * `targetingType` - Required. Identifies the type of this assigned targeting option. Supported targeting types: * `TARGETING_TYPE_CHANNEL` * `TARGETING_TYPE_DIGITAL_CONTENT_LABEL_EXCLUSION` * `TARGETING_TYPE_OMID` * `TARGETING_TYPE_SENSITIVE_CATEGORY_EXCLUSION`
    /// * `assignedTargetingOptionId` - Required. An identifier unique to the targeting type in this advertiser that identifies the assigned targeting option being requested.
    pub fn targeting_types_assigned_targeting_options_get(&self, advertiser_id: i64, targeting_type: &AdvertiserTargetingTypeEnum, assigned_targeting_option_id: &str) -> AdvertiserTargetingTypeAssignedTargetingOptionGetCall<'a, S> {
        AdvertiserTargetingTypeAssignedTargetingOptionGetCall {
            hub: self.hub,
            _advertiser_id: advertiser_id,
            _targeting_type: targeting_type.clone(),
            _assigned_targeting_option_id: assigned_targeting_option_id.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Lists the targeting options assigned to an advertiser.
    /// 
    /// # Arguments
    ///
    /// * `advertiserId` - Required. The ID of the advertiser.
    /// * `targetingType` - Required. Identifies the type of assigned targeting options to list. Supported targeting types: * `TARGETING_TYPE_CHANNEL` * `TARGETING_TYPE_DIGITAL_CONTENT_LABEL_EXCLUSION` * `TARGETING_TYPE_OMID` * `TARGETING_TYPE_SENSITIVE_CATEGORY_EXCLUSION`
    pub fn targeting_types_assigned_targeting_options_list(&self, advertiser_id: i64, targeting_type: &AdvertiserTargetingTypeEnum) -> AdvertiserTargetingTypeAssignedTargetingOptionListCall<'a, S> {
        AdvertiserTargetingTypeAssignedTargetingOptionListCall {
            hub: self.hub,
            _advertiser_id: advertiser_id,
            _targeting_type: targeting_type.clone(),
            _page_token: Default::default(),
            _page_size: Default::default(),
            _order_by: Default::default(),
            _filter: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Audits an advertiser. Returns the counts of used entities per resource type under the advertiser provided. Used entities count towards their respective resource limit. See https://support.google.com/displayvideo/answer/6071450.
    /// 
    /// # Arguments
    ///
    /// * `advertiserId` - Required. The ID of the advertiser to audit.
    pub fn audit(&self, advertiser_id: i64) -> AdvertiserAuditCall<'a, S> {
        AdvertiserAuditCall {
            hub: self.hub,
            _advertiser_id: advertiser_id,
            _read_mask: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Bulk edits targeting options under a single advertiser. The operation will delete the assigned targeting options provided in BulkEditAdvertiserAssignedTargetingOptionsRequest.delete_requests and then create the assigned targeting options provided in BulkEditAdvertiserAssignedTargetingOptionsRequest.create_requests .
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `advertiserId` - Required. The ID of the advertiser.
    pub fn bulk_edit_advertiser_assigned_targeting_options(&self, request: BulkEditAdvertiserAssignedTargetingOptionsRequest, advertiser_id: i64) -> AdvertiserBulkEditAdvertiserAssignedTargetingOptionCall<'a, S> {
        AdvertiserBulkEditAdvertiserAssignedTargetingOptionCall {
            hub: self.hub,
            _request: request,
            _advertiser_id: advertiser_id,
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Lists assigned targeting options of an advertiser across targeting types.
    /// 
    /// # Arguments
    ///
    /// * `advertiserId` - Required. The ID of the advertiser the line item belongs to.
    pub fn bulk_list_advertiser_assigned_targeting_options(&self, advertiser_id: i64) -> AdvertiserBulkListAdvertiserAssignedTargetingOptionCall<'a, S> {
        AdvertiserBulkListAdvertiserAssignedTargetingOptionCall {
            hub: self.hub,
            _advertiser_id: advertiser_id,
            _page_token: Default::default(),
            _page_size: Default::default(),
            _order_by: Default::default(),
            _filter: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Creates a new advertiser. Returns the newly created advertiser if successful. This method can take up to 180 seconds to complete.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    pub fn create(&self, request: Advertiser) -> AdvertiserCreateCall<'a, S> {
        AdvertiserCreateCall {
            hub: self.hub,
            _request: request,
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Deletes an advertiser. Deleting an advertiser will delete all of its child resources, for example, campaigns, insertion orders and line items. A deleted advertiser cannot be recovered.
    /// 
    /// # Arguments
    ///
    /// * `advertiserId` - The ID of the advertiser we need to delete.
    pub fn delete(&self, advertiser_id: i64) -> AdvertiserDeleteCall<'a, S> {
        AdvertiserDeleteCall {
            hub: self.hub,
            _advertiser_id: advertiser_id,
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Gets an advertiser.
    /// 
    /// # Arguments
    ///
    /// * `advertiserId` - Required. The ID of the advertiser to fetch.
    pub fn get(&self, advertiser_id: i64) -> AdvertiserGetCall<'a, S> {
        AdvertiserGetCall {
            hub: self.hub,
            _advertiser_id: advertiser_id,
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Lists advertisers that are accessible to the current user. The order is defined by the order_by parameter. A single partner_id is required. Cross-partner listing is not supported.
    pub fn list(&self) -> AdvertiserListCall<'a, S> {
        AdvertiserListCall {
            hub: self.hub,
            _partner_id: Default::default(),
            _page_token: Default::default(),
            _page_size: Default::default(),
            _order_by: Default::default(),
            _filter: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Updates an existing advertiser. Returns the updated advertiser if successful.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `advertiserId` - Output only. The unique ID of the advertiser. Assigned by the system.
    pub fn patch(&self, request: Advertiser, advertiser_id: i64) -> AdvertiserPatchCall<'a, S> {
        AdvertiserPatchCall {
            hub: self.hub,
            _request: request,
            _advertiser_id: advertiser_id,
            _update_mask: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
}



/// A builder providing access to all methods supported on *combinedAudience* resources.
/// It is not used directly, but through the [`DisplayVideo`] hub.
///
/// # Example
///
/// Instantiate a resource builder
///
/// ```test_harness,no_run
/// extern crate hyper;
/// extern crate hyper_rustls;
/// extern crate google_displayvideo1 as displayvideo1;
/// 
/// # async fn dox() {
/// use std::default::Default;
/// use displayvideo1::{DisplayVideo, oauth2, hyper, hyper_rustls, chrono, FieldMask};
/// 
/// let secret: oauth2::ApplicationSecret = Default::default();
/// let auth = oauth2::InstalledFlowAuthenticator::builder(
///         secret,
///         oauth2::InstalledFlowReturnMethod::HTTPRedirect,
///     ).build().await.unwrap();
/// let mut hub = DisplayVideo::new(hyper::Client::builder().build(hyper_rustls::HttpsConnectorBuilder::new().with_native_roots().https_or_http().enable_http1().build()), auth);
/// // Usually you wouldn't bind this to a variable, but keep calling *CallBuilders*
/// // like `get(...)` and `list(...)`
/// // to build up your call.
/// let rb = hub.combined_audiences();
/// # }
/// ```
pub struct CombinedAudienceMethods<'a, S>
    where S: 'a {

   pub(super) hub: &'a DisplayVideo<S>,
}

impl<'a, S> client::MethodsBuilder for CombinedAudienceMethods<'a, S> {}

impl<'a, S> CombinedAudienceMethods<'a, S> {
    
    /// Create a builder to help you perform the following task:
    ///
    /// Gets a combined audience.
    /// 
    /// # Arguments
    ///
    /// * `combinedAudienceId` - Required. The ID of the combined audience to fetch.
    pub fn get(&self, combined_audience_id: i64) -> CombinedAudienceGetCall<'a, S> {
        CombinedAudienceGetCall {
            hub: self.hub,
            _combined_audience_id: combined_audience_id,
            _partner_id: Default::default(),
            _advertiser_id: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Lists combined audiences. The order is defined by the order_by parameter.
    pub fn list(&self) -> CombinedAudienceListCall<'a, S> {
        CombinedAudienceListCall {
            hub: self.hub,
            _partner_id: Default::default(),
            _page_token: Default::default(),
            _page_size: Default::default(),
            _order_by: Default::default(),
            _filter: Default::default(),
            _advertiser_id: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
}



/// A builder providing access to all methods supported on *customBiddingAlgorithm* resources.
/// It is not used directly, but through the [`DisplayVideo`] hub.
///
/// # Example
///
/// Instantiate a resource builder
///
/// ```test_harness,no_run
/// extern crate hyper;
/// extern crate hyper_rustls;
/// extern crate google_displayvideo1 as displayvideo1;
/// 
/// # async fn dox() {
/// use std::default::Default;
/// use displayvideo1::{DisplayVideo, oauth2, hyper, hyper_rustls, chrono, FieldMask};
/// 
/// let secret: oauth2::ApplicationSecret = Default::default();
/// let auth = oauth2::InstalledFlowAuthenticator::builder(
///         secret,
///         oauth2::InstalledFlowReturnMethod::HTTPRedirect,
///     ).build().await.unwrap();
/// let mut hub = DisplayVideo::new(hyper::Client::builder().build(hyper_rustls::HttpsConnectorBuilder::new().with_native_roots().https_or_http().enable_http1().build()), auth);
/// // Usually you wouldn't bind this to a variable, but keep calling *CallBuilders*
/// // like `create(...)`, `get(...)`, `list(...)`, `patch(...)`, `scripts_create(...)`, `scripts_get(...)`, `scripts_list(...)` and `upload_script(...)`
/// // to build up your call.
/// let rb = hub.custom_bidding_algorithms();
/// # }
/// ```
pub struct CustomBiddingAlgorithmMethods<'a, S>
    where S: 'a {

   pub(super) hub: &'a DisplayVideo<S>,
}

impl<'a, S> client::MethodsBuilder for CustomBiddingAlgorithmMethods<'a, S> {}

impl<'a, S> CustomBiddingAlgorithmMethods<'a, S> {
    
    /// Create a builder to help you perform the following task:
    ///
    /// Creates a new custom bidding script. Returns the newly created script if successful.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `customBiddingAlgorithmId` - Required. The ID of the custom bidding algorithm that owns the script.
    pub fn scripts_create(&self, request: CustomBiddingScript, custom_bidding_algorithm_id: i64) -> CustomBiddingAlgorithmScriptCreateCall<'a, S> {
        CustomBiddingAlgorithmScriptCreateCall {
            hub: self.hub,
            _request: request,
            _custom_bidding_algorithm_id: custom_bidding_algorithm_id,
            _partner_id: Default::default(),
            _advertiser_id: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Gets a custom bidding script.
    /// 
    /// # Arguments
    ///
    /// * `customBiddingAlgorithmId` - Required. The ID of the custom bidding algorithm owns the script.
    /// * `customBiddingScriptId` - Required. The ID of the custom bidding script to fetch.
    pub fn scripts_get(&self, custom_bidding_algorithm_id: i64, custom_bidding_script_id: i64) -> CustomBiddingAlgorithmScriptGetCall<'a, S> {
        CustomBiddingAlgorithmScriptGetCall {
            hub: self.hub,
            _custom_bidding_algorithm_id: custom_bidding_algorithm_id,
            _custom_bidding_script_id: custom_bidding_script_id,
            _partner_id: Default::default(),
            _advertiser_id: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Lists custom bidding scripts that belong to the given algorithm. The order is defined by the order_by parameter.
    /// 
    /// # Arguments
    ///
    /// * `customBiddingAlgorithmId` - Required. The ID of the custom bidding algorithm owns the script.
    pub fn scripts_list(&self, custom_bidding_algorithm_id: i64) -> CustomBiddingAlgorithmScriptListCall<'a, S> {
        CustomBiddingAlgorithmScriptListCall {
            hub: self.hub,
            _custom_bidding_algorithm_id: custom_bidding_algorithm_id,
            _partner_id: Default::default(),
            _page_token: Default::default(),
            _page_size: Default::default(),
            _order_by: Default::default(),
            _advertiser_id: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Creates a new custom bidding algorithm. Returns the newly created custom bidding algorithm if successful.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    pub fn create(&self, request: CustomBiddingAlgorithm) -> CustomBiddingAlgorithmCreateCall<'a, S> {
        CustomBiddingAlgorithmCreateCall {
            hub: self.hub,
            _request: request,
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Gets a custom bidding algorithm.
    /// 
    /// # Arguments
    ///
    /// * `customBiddingAlgorithmId` - Required. The ID of the custom bidding algorithm to fetch.
    pub fn get(&self, custom_bidding_algorithm_id: i64) -> CustomBiddingAlgorithmGetCall<'a, S> {
        CustomBiddingAlgorithmGetCall {
            hub: self.hub,
            _custom_bidding_algorithm_id: custom_bidding_algorithm_id,
            _partner_id: Default::default(),
            _advertiser_id: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Lists custom bidding algorithms that are accessible to the current user and can be used in bidding stratgies. The order is defined by the order_by parameter.
    pub fn list(&self) -> CustomBiddingAlgorithmListCall<'a, S> {
        CustomBiddingAlgorithmListCall {
            hub: self.hub,
            _partner_id: Default::default(),
            _page_token: Default::default(),
            _page_size: Default::default(),
            _order_by: Default::default(),
            _filter: Default::default(),
            _advertiser_id: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Updates an existing custom bidding algorithm. Returns the updated custom bidding algorithm if successful.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `customBiddingAlgorithmId` - Output only. The unique ID of the custom bidding algorithm. Assigned by the system.
    pub fn patch(&self, request: CustomBiddingAlgorithm, custom_bidding_algorithm_id: i64) -> CustomBiddingAlgorithmPatchCall<'a, S> {
        CustomBiddingAlgorithmPatchCall {
            hub: self.hub,
            _request: request,
            _custom_bidding_algorithm_id: custom_bidding_algorithm_id,
            _update_mask: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Creates a custom bidding script reference object for a script file. The resulting reference object provides a resource path to which the script file should be uploaded. This reference object should be included in when creating a new custom bidding script object.
    /// 
    /// # Arguments
    ///
    /// * `customBiddingAlgorithmId` - Required. The ID of the custom bidding algorithm owns the script.
    pub fn upload_script(&self, custom_bidding_algorithm_id: i64) -> CustomBiddingAlgorithmUploadScriptCall<'a, S> {
        CustomBiddingAlgorithmUploadScriptCall {
            hub: self.hub,
            _custom_bidding_algorithm_id: custom_bidding_algorithm_id,
            _partner_id: Default::default(),
            _advertiser_id: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
}



/// A builder providing access to all methods supported on *customList* resources.
/// It is not used directly, but through the [`DisplayVideo`] hub.
///
/// # Example
///
/// Instantiate a resource builder
///
/// ```test_harness,no_run
/// extern crate hyper;
/// extern crate hyper_rustls;
/// extern crate google_displayvideo1 as displayvideo1;
/// 
/// # async fn dox() {
/// use std::default::Default;
/// use displayvideo1::{DisplayVideo, oauth2, hyper, hyper_rustls, chrono, FieldMask};
/// 
/// let secret: oauth2::ApplicationSecret = Default::default();
/// let auth = oauth2::InstalledFlowAuthenticator::builder(
///         secret,
///         oauth2::InstalledFlowReturnMethod::HTTPRedirect,
///     ).build().await.unwrap();
/// let mut hub = DisplayVideo::new(hyper::Client::builder().build(hyper_rustls::HttpsConnectorBuilder::new().with_native_roots().https_or_http().enable_http1().build()), auth);
/// // Usually you wouldn't bind this to a variable, but keep calling *CallBuilders*
/// // like `get(...)` and `list(...)`
/// // to build up your call.
/// let rb = hub.custom_lists();
/// # }
/// ```
pub struct CustomListMethods<'a, S>
    where S: 'a {

   pub(super) hub: &'a DisplayVideo<S>,
}

impl<'a, S> client::MethodsBuilder for CustomListMethods<'a, S> {}

impl<'a, S> CustomListMethods<'a, S> {
    
    /// Create a builder to help you perform the following task:
    ///
    /// Gets a custom list.
    /// 
    /// # Arguments
    ///
    /// * `customListId` - Required. The ID of the custom list to fetch.
    pub fn get(&self, custom_list_id: i64) -> CustomListGetCall<'a, S> {
        CustomListGetCall {
            hub: self.hub,
            _custom_list_id: custom_list_id,
            _advertiser_id: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Lists custom lists. The order is defined by the order_by parameter.
    pub fn list(&self) -> CustomListListCall<'a, S> {
        CustomListListCall {
            hub: self.hub,
            _page_token: Default::default(),
            _page_size: Default::default(),
            _order_by: Default::default(),
            _filter: Default::default(),
            _advertiser_id: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
}



/// A builder providing access to all methods supported on *firstAndThirdPartyAudience* resources.
/// It is not used directly, but through the [`DisplayVideo`] hub.
///
/// # Example
///
/// Instantiate a resource builder
///
/// ```test_harness,no_run
/// extern crate hyper;
/// extern crate hyper_rustls;
/// extern crate google_displayvideo1 as displayvideo1;
/// 
/// # async fn dox() {
/// use std::default::Default;
/// use displayvideo1::{DisplayVideo, oauth2, hyper, hyper_rustls, chrono, FieldMask};
/// 
/// let secret: oauth2::ApplicationSecret = Default::default();
/// let auth = oauth2::InstalledFlowAuthenticator::builder(
///         secret,
///         oauth2::InstalledFlowReturnMethod::HTTPRedirect,
///     ).build().await.unwrap();
/// let mut hub = DisplayVideo::new(hyper::Client::builder().build(hyper_rustls::HttpsConnectorBuilder::new().with_native_roots().https_or_http().enable_http1().build()), auth);
/// // Usually you wouldn't bind this to a variable, but keep calling *CallBuilders*
/// // like `create(...)`, `edit_customer_match_members(...)`, `get(...)`, `list(...)` and `patch(...)`
/// // to build up your call.
/// let rb = hub.first_and_third_party_audiences();
/// # }
/// ```
pub struct FirstAndThirdPartyAudienceMethods<'a, S>
    where S: 'a {

   pub(super) hub: &'a DisplayVideo<S>,
}

impl<'a, S> client::MethodsBuilder for FirstAndThirdPartyAudienceMethods<'a, S> {}

impl<'a, S> FirstAndThirdPartyAudienceMethods<'a, S> {
    
    /// Create a builder to help you perform the following task:
    ///
    /// Creates a FirstAndThirdPartyAudience. Only supported for the following audience_type: * `CUSTOMER_MATCH_CONTACT_INFO` * `CUSTOMER_MATCH_DEVICE_ID`
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    pub fn create(&self, request: FirstAndThirdPartyAudience) -> FirstAndThirdPartyAudienceCreateCall<'a, S> {
        FirstAndThirdPartyAudienceCreateCall {
            hub: self.hub,
            _request: request,
            _advertiser_id: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Updates the member list of a Customer Match audience. Only supported for the following audience_type: * `CUSTOMER_MATCH_CONTACT_INFO` * `CUSTOMER_MATCH_DEVICE_ID`
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `firstAndThirdPartyAudienceId` - Required. The ID of the Customer Match FirstAndThirdPartyAudience whose members will be edited.
    pub fn edit_customer_match_members(&self, request: EditCustomerMatchMembersRequest, first_and_third_party_audience_id: i64) -> FirstAndThirdPartyAudienceEditCustomerMatchMemberCall<'a, S> {
        FirstAndThirdPartyAudienceEditCustomerMatchMemberCall {
            hub: self.hub,
            _request: request,
            _first_and_third_party_audience_id: first_and_third_party_audience_id,
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Gets a first and third party audience.
    /// 
    /// # Arguments
    ///
    /// * `firstAndThirdPartyAudienceId` - Required. The ID of the first and third party audience to fetch.
    pub fn get(&self, first_and_third_party_audience_id: i64) -> FirstAndThirdPartyAudienceGetCall<'a, S> {
        FirstAndThirdPartyAudienceGetCall {
            hub: self.hub,
            _first_and_third_party_audience_id: first_and_third_party_audience_id,
            _partner_id: Default::default(),
            _advertiser_id: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Lists first and third party audiences. The order is defined by the order_by parameter.
    pub fn list(&self) -> FirstAndThirdPartyAudienceListCall<'a, S> {
        FirstAndThirdPartyAudienceListCall {
            hub: self.hub,
            _partner_id: Default::default(),
            _page_token: Default::default(),
            _page_size: Default::default(),
            _order_by: Default::default(),
            _filter: Default::default(),
            _advertiser_id: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Updates an existing FirstAndThirdPartyAudience. Only supported for the following audience_type: * `CUSTOMER_MATCH_CONTACT_INFO` * `CUSTOMER_MATCH_DEVICE_ID`
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `firstAndThirdPartyAudienceId` - Output only. The unique ID of the first and third party audience. Assigned by the system.
    pub fn patch(&self, request: FirstAndThirdPartyAudience, first_and_third_party_audience_id: i64) -> FirstAndThirdPartyAudiencePatchCall<'a, S> {
        FirstAndThirdPartyAudiencePatchCall {
            hub: self.hub,
            _request: request,
            _first_and_third_party_audience_id: first_and_third_party_audience_id,
            _update_mask: Default::default(),
            _advertiser_id: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
}



/// A builder providing access to all methods supported on *floodlightGroup* resources.
/// It is not used directly, but through the [`DisplayVideo`] hub.
///
/// # Example
///
/// Instantiate a resource builder
///
/// ```test_harness,no_run
/// extern crate hyper;
/// extern crate hyper_rustls;
/// extern crate google_displayvideo1 as displayvideo1;
/// 
/// # async fn dox() {
/// use std::default::Default;
/// use displayvideo1::{DisplayVideo, oauth2, hyper, hyper_rustls, chrono, FieldMask};
/// 
/// let secret: oauth2::ApplicationSecret = Default::default();
/// let auth = oauth2::InstalledFlowAuthenticator::builder(
///         secret,
///         oauth2::InstalledFlowReturnMethod::HTTPRedirect,
///     ).build().await.unwrap();
/// let mut hub = DisplayVideo::new(hyper::Client::builder().build(hyper_rustls::HttpsConnectorBuilder::new().with_native_roots().https_or_http().enable_http1().build()), auth);
/// // Usually you wouldn't bind this to a variable, but keep calling *CallBuilders*
/// // like `get(...)` and `patch(...)`
/// // to build up your call.
/// let rb = hub.floodlight_groups();
/// # }
/// ```
pub struct FloodlightGroupMethods<'a, S>
    where S: 'a {

   pub(super) hub: &'a DisplayVideo<S>,
}

impl<'a, S> client::MethodsBuilder for FloodlightGroupMethods<'a, S> {}

impl<'a, S> FloodlightGroupMethods<'a, S> {
    
    /// Create a builder to help you perform the following task:
    ///
    /// Gets a Floodlight group.
    /// 
    /// # Arguments
    ///
    /// * `floodlightGroupId` - Required. The ID of the Floodlight group to fetch.
    pub fn get(&self, floodlight_group_id: i64) -> FloodlightGroupGetCall<'a, S> {
        FloodlightGroupGetCall {
            hub: self.hub,
            _floodlight_group_id: floodlight_group_id,
            _partner_id: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Updates an existing Floodlight group. Returns the updated Floodlight group if successful.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `floodlightGroupId` - Output only. The unique ID of the Floodlight group. Assigned by the system.
    pub fn patch(&self, request: FloodlightGroup, floodlight_group_id: i64) -> FloodlightGroupPatchCall<'a, S> {
        FloodlightGroupPatchCall {
            hub: self.hub,
            _request: request,
            _floodlight_group_id: floodlight_group_id,
            _update_mask: Default::default(),
            _partner_id: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
}



/// A builder providing access to all methods supported on *googleAudience* resources.
/// It is not used directly, but through the [`DisplayVideo`] hub.
///
/// # Example
///
/// Instantiate a resource builder
///
/// ```test_harness,no_run
/// extern crate hyper;
/// extern crate hyper_rustls;
/// extern crate google_displayvideo1 as displayvideo1;
/// 
/// # async fn dox() {
/// use std::default::Default;
/// use displayvideo1::{DisplayVideo, oauth2, hyper, hyper_rustls, chrono, FieldMask};
/// 
/// let secret: oauth2::ApplicationSecret = Default::default();
/// let auth = oauth2::InstalledFlowAuthenticator::builder(
///         secret,
///         oauth2::InstalledFlowReturnMethod::HTTPRedirect,
///     ).build().await.unwrap();
/// let mut hub = DisplayVideo::new(hyper::Client::builder().build(hyper_rustls::HttpsConnectorBuilder::new().with_native_roots().https_or_http().enable_http1().build()), auth);
/// // Usually you wouldn't bind this to a variable, but keep calling *CallBuilders*
/// // like `get(...)` and `list(...)`
/// // to build up your call.
/// let rb = hub.google_audiences();
/// # }
/// ```
pub struct GoogleAudienceMethods<'a, S>
    where S: 'a {

   pub(super) hub: &'a DisplayVideo<S>,
}

impl<'a, S> client::MethodsBuilder for GoogleAudienceMethods<'a, S> {}

impl<'a, S> GoogleAudienceMethods<'a, S> {
    
    /// Create a builder to help you perform the following task:
    ///
    /// Gets a Google audience.
    /// 
    /// # Arguments
    ///
    /// * `googleAudienceId` - Required. The ID of the Google audience to fetch.
    pub fn get(&self, google_audience_id: i64) -> GoogleAudienceGetCall<'a, S> {
        GoogleAudienceGetCall {
            hub: self.hub,
            _google_audience_id: google_audience_id,
            _partner_id: Default::default(),
            _advertiser_id: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Lists Google audiences. The order is defined by the order_by parameter.
    pub fn list(&self) -> GoogleAudienceListCall<'a, S> {
        GoogleAudienceListCall {
            hub: self.hub,
            _partner_id: Default::default(),
            _page_token: Default::default(),
            _page_size: Default::default(),
            _order_by: Default::default(),
            _filter: Default::default(),
            _advertiser_id: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
}



/// A builder providing access to all methods supported on *guaranteedOrder* resources.
/// It is not used directly, but through the [`DisplayVideo`] hub.
///
/// # Example
///
/// Instantiate a resource builder
///
/// ```test_harness,no_run
/// extern crate hyper;
/// extern crate hyper_rustls;
/// extern crate google_displayvideo1 as displayvideo1;
/// 
/// # async fn dox() {
/// use std::default::Default;
/// use displayvideo1::{DisplayVideo, oauth2, hyper, hyper_rustls, chrono, FieldMask};
/// 
/// let secret: oauth2::ApplicationSecret = Default::default();
/// let auth = oauth2::InstalledFlowAuthenticator::builder(
///         secret,
///         oauth2::InstalledFlowReturnMethod::HTTPRedirect,
///     ).build().await.unwrap();
/// let mut hub = DisplayVideo::new(hyper::Client::builder().build(hyper_rustls::HttpsConnectorBuilder::new().with_native_roots().https_or_http().enable_http1().build()), auth);
/// // Usually you wouldn't bind this to a variable, but keep calling *CallBuilders*
/// // like `create(...)`, `edit_guaranteed_order_read_accessors(...)`, `get(...)`, `list(...)` and `patch(...)`
/// // to build up your call.
/// let rb = hub.guaranteed_orders();
/// # }
/// ```
pub struct GuaranteedOrderMethods<'a, S>
    where S: 'a {

   pub(super) hub: &'a DisplayVideo<S>,
}

impl<'a, S> client::MethodsBuilder for GuaranteedOrderMethods<'a, S> {}

impl<'a, S> GuaranteedOrderMethods<'a, S> {
    
    /// Create a builder to help you perform the following task:
    ///
    /// Creates a new guaranteed order. Returns the newly created guaranteed order if successful.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    pub fn create(&self, request: GuaranteedOrder) -> GuaranteedOrderCreateCall<'a, S> {
        GuaranteedOrderCreateCall {
            hub: self.hub,
            _request: request,
            _partner_id: Default::default(),
            _advertiser_id: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Edits read advertisers of a guaranteed order.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `guaranteedOrderId` - Required. The ID of the guaranteed order to edit. The ID is of the format `{exchange}-{legacy_guaranteed_order_id}`
    pub fn edit_guaranteed_order_read_accessors(&self, request: EditGuaranteedOrderReadAccessorsRequest, guaranteed_order_id: &str) -> GuaranteedOrderEditGuaranteedOrderReadAccessorCall<'a, S> {
        GuaranteedOrderEditGuaranteedOrderReadAccessorCall {
            hub: self.hub,
            _request: request,
            _guaranteed_order_id: guaranteed_order_id.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Gets a guaranteed order.
    /// 
    /// # Arguments
    ///
    /// * `guaranteedOrderId` - Required. The ID of the guaranteed order to fetch. The ID is of the format `{exchange}-{legacy_guaranteed_order_id}`
    pub fn get(&self, guaranteed_order_id: &str) -> GuaranteedOrderGetCall<'a, S> {
        GuaranteedOrderGetCall {
            hub: self.hub,
            _guaranteed_order_id: guaranteed_order_id.to_string(),
            _partner_id: Default::default(),
            _advertiser_id: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Lists guaranteed orders that are accessible to the current user. The order is defined by the order_by parameter. If a filter by entity_status is not specified, guaranteed orders with entity status `ENTITY_STATUS_ARCHIVED` will not be included in the results.
    pub fn list(&self) -> GuaranteedOrderListCall<'a, S> {
        GuaranteedOrderListCall {
            hub: self.hub,
            _partner_id: Default::default(),
            _page_token: Default::default(),
            _page_size: Default::default(),
            _order_by: Default::default(),
            _filter: Default::default(),
            _advertiser_id: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Updates an existing guaranteed order. Returns the updated guaranteed order if successful.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `guaranteedOrderId` - Output only. The unique identifier of the guaranteed order. The guaranteed order IDs have the format `{exchange}-{legacy_guaranteed_order_id}`.
    pub fn patch(&self, request: GuaranteedOrder, guaranteed_order_id: &str) -> GuaranteedOrderPatchCall<'a, S> {
        GuaranteedOrderPatchCall {
            hub: self.hub,
            _request: request,
            _guaranteed_order_id: guaranteed_order_id.to_string(),
            _update_mask: Default::default(),
            _partner_id: Default::default(),
            _advertiser_id: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
}



/// A builder providing access to all methods supported on *inventorySourceGroup* resources.
/// It is not used directly, but through the [`DisplayVideo`] hub.
///
/// # Example
///
/// Instantiate a resource builder
///
/// ```test_harness,no_run
/// extern crate hyper;
/// extern crate hyper_rustls;
/// extern crate google_displayvideo1 as displayvideo1;
/// 
/// # async fn dox() {
/// use std::default::Default;
/// use displayvideo1::{DisplayVideo, oauth2, hyper, hyper_rustls, chrono, FieldMask};
/// 
/// let secret: oauth2::ApplicationSecret = Default::default();
/// let auth = oauth2::InstalledFlowAuthenticator::builder(
///         secret,
///         oauth2::InstalledFlowReturnMethod::HTTPRedirect,
///     ).build().await.unwrap();
/// let mut hub = DisplayVideo::new(hyper::Client::builder().build(hyper_rustls::HttpsConnectorBuilder::new().with_native_roots().https_or_http().enable_http1().build()), auth);
/// // Usually you wouldn't bind this to a variable, but keep calling *CallBuilders*
/// // like `assigned_inventory_sources_bulk_edit(...)`, `assigned_inventory_sources_create(...)`, `assigned_inventory_sources_delete(...)`, `assigned_inventory_sources_list(...)`, `create(...)`, `delete(...)`, `get(...)`, `list(...)` and `patch(...)`
/// // to build up your call.
/// let rb = hub.inventory_source_groups();
/// # }
/// ```
pub struct InventorySourceGroupMethods<'a, S>
    where S: 'a {

   pub(super) hub: &'a DisplayVideo<S>,
}

impl<'a, S> client::MethodsBuilder for InventorySourceGroupMethods<'a, S> {}

impl<'a, S> InventorySourceGroupMethods<'a, S> {
    
    /// Create a builder to help you perform the following task:
    ///
    /// Bulk edits multiple assignments between inventory sources and a single inventory source group. The operation will delete the assigned inventory sources provided in BulkEditAssignedInventorySourcesRequest.deleted_assigned_inventory_sources and then create the assigned inventory sources provided in BulkEditAssignedInventorySourcesRequest.created_assigned_inventory_sources.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `inventorySourceGroupId` - Required. The ID of the inventory source group to which the assignments are assigned.
    pub fn assigned_inventory_sources_bulk_edit(&self, request: BulkEditAssignedInventorySourcesRequest, inventory_source_group_id: i64) -> InventorySourceGroupAssignedInventorySourceBulkEditCall<'a, S> {
        InventorySourceGroupAssignedInventorySourceBulkEditCall {
            hub: self.hub,
            _request: request,
            _inventory_source_group_id: inventory_source_group_id,
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Creates an assignment between an inventory source and an inventory source group.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `inventorySourceGroupId` - Required. The ID of the inventory source group to which the assignment will be assigned.
    pub fn assigned_inventory_sources_create(&self, request: AssignedInventorySource, inventory_source_group_id: i64) -> InventorySourceGroupAssignedInventorySourceCreateCall<'a, S> {
        InventorySourceGroupAssignedInventorySourceCreateCall {
            hub: self.hub,
            _request: request,
            _inventory_source_group_id: inventory_source_group_id,
            _partner_id: Default::default(),
            _advertiser_id: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Deletes the assignment between an inventory source and an inventory source group.
    /// 
    /// # Arguments
    ///
    /// * `inventorySourceGroupId` - Required. The ID of the inventory source group to which this assignment is assigned.
    /// * `assignedInventorySourceId` - Required. The ID of the assigned inventory source to delete.
    pub fn assigned_inventory_sources_delete(&self, inventory_source_group_id: i64, assigned_inventory_source_id: i64) -> InventorySourceGroupAssignedInventorySourceDeleteCall<'a, S> {
        InventorySourceGroupAssignedInventorySourceDeleteCall {
            hub: self.hub,
            _inventory_source_group_id: inventory_source_group_id,
            _assigned_inventory_source_id: assigned_inventory_source_id,
            _partner_id: Default::default(),
            _advertiser_id: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Lists inventory sources assigned to an inventory source group.
    /// 
    /// # Arguments
    ///
    /// * `inventorySourceGroupId` - Required. The ID of the inventory source group to which these assignments are assigned.
    pub fn assigned_inventory_sources_list(&self, inventory_source_group_id: i64) -> InventorySourceGroupAssignedInventorySourceListCall<'a, S> {
        InventorySourceGroupAssignedInventorySourceListCall {
            hub: self.hub,
            _inventory_source_group_id: inventory_source_group_id,
            _partner_id: Default::default(),
            _page_token: Default::default(),
            _page_size: Default::default(),
            _order_by: Default::default(),
            _filter: Default::default(),
            _advertiser_id: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Creates a new inventory source group. Returns the newly created inventory source group if successful.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    pub fn create(&self, request: InventorySourceGroup) -> InventorySourceGroupCreateCall<'a, S> {
        InventorySourceGroupCreateCall {
            hub: self.hub,
            _request: request,
            _partner_id: Default::default(),
            _advertiser_id: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Deletes an inventory source group.
    /// 
    /// # Arguments
    ///
    /// * `inventorySourceGroupId` - Required. The ID of the inventory source group to delete.
    pub fn delete(&self, inventory_source_group_id: i64) -> InventorySourceGroupDeleteCall<'a, S> {
        InventorySourceGroupDeleteCall {
            hub: self.hub,
            _inventory_source_group_id: inventory_source_group_id,
            _partner_id: Default::default(),
            _advertiser_id: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Gets an inventory source group.
    /// 
    /// # Arguments
    ///
    /// * `inventorySourceGroupId` - Required. The ID of the inventory source group to fetch.
    pub fn get(&self, inventory_source_group_id: i64) -> InventorySourceGroupGetCall<'a, S> {
        InventorySourceGroupGetCall {
            hub: self.hub,
            _inventory_source_group_id: inventory_source_group_id,
            _partner_id: Default::default(),
            _advertiser_id: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Lists inventory source groups that are accessible to the current user. The order is defined by the order_by parameter.
    pub fn list(&self) -> InventorySourceGroupListCall<'a, S> {
        InventorySourceGroupListCall {
            hub: self.hub,
            _partner_id: Default::default(),
            _page_token: Default::default(),
            _page_size: Default::default(),
            _order_by: Default::default(),
            _filter: Default::default(),
            _advertiser_id: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Updates an inventory source group. Returns the updated inventory source group if successful.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `inventorySourceGroupId` - Output only. The unique ID of the inventory source group. Assigned by the system.
    pub fn patch(&self, request: InventorySourceGroup, inventory_source_group_id: i64) -> InventorySourceGroupPatchCall<'a, S> {
        InventorySourceGroupPatchCall {
            hub: self.hub,
            _request: request,
            _inventory_source_group_id: inventory_source_group_id,
            _update_mask: Default::default(),
            _partner_id: Default::default(),
            _advertiser_id: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
}



/// A builder providing access to all methods supported on *inventorySource* resources.
/// It is not used directly, but through the [`DisplayVideo`] hub.
///
/// # Example
///
/// Instantiate a resource builder
///
/// ```test_harness,no_run
/// extern crate hyper;
/// extern crate hyper_rustls;
/// extern crate google_displayvideo1 as displayvideo1;
/// 
/// # async fn dox() {
/// use std::default::Default;
/// use displayvideo1::{DisplayVideo, oauth2, hyper, hyper_rustls, chrono, FieldMask};
/// 
/// let secret: oauth2::ApplicationSecret = Default::default();
/// let auth = oauth2::InstalledFlowAuthenticator::builder(
///         secret,
///         oauth2::InstalledFlowReturnMethod::HTTPRedirect,
///     ).build().await.unwrap();
/// let mut hub = DisplayVideo::new(hyper::Client::builder().build(hyper_rustls::HttpsConnectorBuilder::new().with_native_roots().https_or_http().enable_http1().build()), auth);
/// // Usually you wouldn't bind this to a variable, but keep calling *CallBuilders*
/// // like `create(...)`, `edit_inventory_source_read_write_accessors(...)`, `get(...)`, `list(...)` and `patch(...)`
/// // to build up your call.
/// let rb = hub.inventory_sources();
/// # }
/// ```
pub struct InventorySourceMethods<'a, S>
    where S: 'a {

   pub(super) hub: &'a DisplayVideo<S>,
}

impl<'a, S> client::MethodsBuilder for InventorySourceMethods<'a, S> {}

impl<'a, S> InventorySourceMethods<'a, S> {
    
    /// Create a builder to help you perform the following task:
    ///
    /// Creates a new inventory source. Returns the newly created inventory source if successful.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    pub fn create(&self, request: InventorySource) -> InventorySourceCreateCall<'a, S> {
        InventorySourceCreateCall {
            hub: self.hub,
            _request: request,
            _partner_id: Default::default(),
            _advertiser_id: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Edits read/write accessors of an inventory source. Returns the updated read_write_accessors for the inventory source.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `inventorySourceId` - Required. The ID of inventory source to update.
    pub fn edit_inventory_source_read_write_accessors(&self, request: EditInventorySourceReadWriteAccessorsRequest, inventory_source_id: i64) -> InventorySourceEditInventorySourceReadWriteAccessorCall<'a, S> {
        InventorySourceEditInventorySourceReadWriteAccessorCall {
            hub: self.hub,
            _request: request,
            _inventory_source_id: inventory_source_id,
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Gets an inventory source.
    /// 
    /// # Arguments
    ///
    /// * `inventorySourceId` - Required. The ID of the inventory source to fetch.
    pub fn get(&self, inventory_source_id: i64) -> InventorySourceGetCall<'a, S> {
        InventorySourceGetCall {
            hub: self.hub,
            _inventory_source_id: inventory_source_id,
            _partner_id: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Lists inventory sources that are accessible to the current user. The order is defined by the order_by parameter. If a filter by entity_status is not specified, inventory sources with entity status `ENTITY_STATUS_ARCHIVED` will not be included in the results.
    pub fn list(&self) -> InventorySourceListCall<'a, S> {
        InventorySourceListCall {
            hub: self.hub,
            _partner_id: Default::default(),
            _page_token: Default::default(),
            _page_size: Default::default(),
            _order_by: Default::default(),
            _filter: Default::default(),
            _advertiser_id: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Updates an existing inventory source. Returns the updated inventory source if successful.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `inventorySourceId` - Output only. The unique ID of the inventory source. Assigned by the system.
    pub fn patch(&self, request: InventorySource, inventory_source_id: i64) -> InventorySourcePatchCall<'a, S> {
        InventorySourcePatchCall {
            hub: self.hub,
            _request: request,
            _inventory_source_id: inventory_source_id,
            _update_mask: Default::default(),
            _partner_id: Default::default(),
            _advertiser_id: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
}



/// A builder providing access to all methods supported on *media* resources.
/// It is not used directly, but through the [`DisplayVideo`] hub.
///
/// # Example
///
/// Instantiate a resource builder
///
/// ```test_harness,no_run
/// extern crate hyper;
/// extern crate hyper_rustls;
/// extern crate google_displayvideo1 as displayvideo1;
/// 
/// # async fn dox() {
/// use std::default::Default;
/// use displayvideo1::{DisplayVideo, oauth2, hyper, hyper_rustls, chrono, FieldMask};
/// 
/// let secret: oauth2::ApplicationSecret = Default::default();
/// let auth = oauth2::InstalledFlowAuthenticator::builder(
///         secret,
///         oauth2::InstalledFlowReturnMethod::HTTPRedirect,
///     ).build().await.unwrap();
/// let mut hub = DisplayVideo::new(hyper::Client::builder().build(hyper_rustls::HttpsConnectorBuilder::new().with_native_roots().https_or_http().enable_http1().build()), auth);
/// // Usually you wouldn't bind this to a variable, but keep calling *CallBuilders*
/// // like `download(...)` and `upload(...)`
/// // to build up your call.
/// let rb = hub.media();
/// # }
/// ```
pub struct MediaMethods<'a, S>
    where S: 'a {

   pub(super) hub: &'a DisplayVideo<S>,
}

impl<'a, S> client::MethodsBuilder for MediaMethods<'a, S> {}

impl<'a, S> MediaMethods<'a, S> {
    
    /// Create a builder to help you perform the following task:
    ///
    /// Downloads media. Download is supported on the URI `/download/{resource_name=**}?alt=media.` **Note**: Download requests will not be successful without including `alt=media` query string.
    /// 
    /// # Arguments
    ///
    /// * `resourceName` - Name of the media that is being downloaded. See ReadRequest.resource_name.
    pub fn download(&self, resource_name: &str) -> MediaDownloadCall<'a, S> {
        MediaDownloadCall {
            hub: self.hub,
            _resource_name: resource_name.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Uploads media. Upload is supported on the URI `/upload/media/{resource_name=**}?upload_type=media.` **Note**: Upload requests will not be successful without including `upload_type=media` query string.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `resourceName` - Name of the media that is being downloaded. See ReadRequest.resource_name.
    pub fn upload(&self, request: GoogleBytestreamMedia, resource_name: &str) -> MediaUploadCall<'a, S> {
        MediaUploadCall {
            hub: self.hub,
            _request: request,
            _resource_name: resource_name.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
}



/// A builder providing access to all methods supported on *partner* resources.
/// It is not used directly, but through the [`DisplayVideo`] hub.
///
/// # Example
///
/// Instantiate a resource builder
///
/// ```test_harness,no_run
/// extern crate hyper;
/// extern crate hyper_rustls;
/// extern crate google_displayvideo1 as displayvideo1;
/// 
/// # async fn dox() {
/// use std::default::Default;
/// use displayvideo1::{DisplayVideo, oauth2, hyper, hyper_rustls, chrono, FieldMask};
/// 
/// let secret: oauth2::ApplicationSecret = Default::default();
/// let auth = oauth2::InstalledFlowAuthenticator::builder(
///         secret,
///         oauth2::InstalledFlowReturnMethod::HTTPRedirect,
///     ).build().await.unwrap();
/// let mut hub = DisplayVideo::new(hyper::Client::builder().build(hyper_rustls::HttpsConnectorBuilder::new().with_native_roots().https_or_http().enable_http1().build()), auth);
/// // Usually you wouldn't bind this to a variable, but keep calling *CallBuilders*
/// // like `bulk_edit_partner_assigned_targeting_options(...)`, `channels_create(...)`, `channels_get(...)`, `channels_list(...)`, `channels_patch(...)`, `channels_sites_bulk_edit(...)`, `channels_sites_create(...)`, `channels_sites_delete(...)`, `channels_sites_list(...)`, `channels_sites_replace(...)`, `get(...)`, `list(...)`, `targeting_types_assigned_targeting_options_create(...)`, `targeting_types_assigned_targeting_options_delete(...)`, `targeting_types_assigned_targeting_options_get(...)` and `targeting_types_assigned_targeting_options_list(...)`
/// // to build up your call.
/// let rb = hub.partners();
/// # }
/// ```
pub struct PartnerMethods<'a, S>
    where S: 'a {

   pub(super) hub: &'a DisplayVideo<S>,
}

impl<'a, S> client::MethodsBuilder for PartnerMethods<'a, S> {}

impl<'a, S> PartnerMethods<'a, S> {
    
    /// Create a builder to help you perform the following task:
    ///
    /// Bulk edits sites under a single channel. The operation will delete the sites provided in BulkEditSitesRequest.deleted_sites and then create the sites provided in BulkEditSitesRequest.created_sites.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `partnerId` - The ID of the partner that owns the parent channel.
    /// * `channelId` - Required. The ID of the parent channel to which the sites belong.
    pub fn channels_sites_bulk_edit(&self, request: BulkEditSitesRequest, partner_id: i64, channel_id: i64) -> PartnerChannelSiteBulkEditCall<'a, S> {
        PartnerChannelSiteBulkEditCall {
            hub: self.hub,
            _request: request,
            _partner_id: partner_id,
            _channel_id: channel_id,
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Creates a site in a channel.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `partnerId` - The ID of the partner that owns the parent channel.
    /// * `channelId` - Required. The ID of the parent channel in which the site will be created.
    pub fn channels_sites_create(&self, request: Site, partner_id: i64, channel_id: i64) -> PartnerChannelSiteCreateCall<'a, S> {
        PartnerChannelSiteCreateCall {
            hub: self.hub,
            _request: request,
            _partner_id: partner_id,
            _channel_id: channel_id,
            _advertiser_id: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Deletes a site from a channel.
    /// 
    /// # Arguments
    ///
    /// * `partnerId` - The ID of the partner that owns the parent channel.
    /// * `channelId` - Required. The ID of the parent channel to which the site belongs.
    /// * `urlOrAppId` - Required. The URL or app ID of the site to delete.
    pub fn channels_sites_delete(&self, partner_id: i64, channel_id: i64, url_or_app_id: &str) -> PartnerChannelSiteDeleteCall<'a, S> {
        PartnerChannelSiteDeleteCall {
            hub: self.hub,
            _partner_id: partner_id,
            _channel_id: channel_id,
            _url_or_app_id: url_or_app_id.to_string(),
            _advertiser_id: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Lists sites in a channel.
    /// 
    /// # Arguments
    ///
    /// * `partnerId` - The ID of the partner that owns the parent channel.
    /// * `channelId` - Required. The ID of the parent channel to which the requested sites belong.
    pub fn channels_sites_list(&self, partner_id: i64, channel_id: i64) -> PartnerChannelSiteListCall<'a, S> {
        PartnerChannelSiteListCall {
            hub: self.hub,
            _partner_id: partner_id,
            _channel_id: channel_id,
            _page_token: Default::default(),
            _page_size: Default::default(),
            _order_by: Default::default(),
            _filter: Default::default(),
            _advertiser_id: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Replaces all of the sites under a single channel. The operation will replace the sites under a channel with the sites provided in ReplaceSitesRequest.new_sites.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `partnerId` - The ID of the partner that owns the parent channel.
    /// * `channelId` - Required. The ID of the parent channel whose sites will be replaced.
    pub fn channels_sites_replace(&self, request: ReplaceSitesRequest, partner_id: i64, channel_id: i64) -> PartnerChannelSiteReplaceCall<'a, S> {
        PartnerChannelSiteReplaceCall {
            hub: self.hub,
            _request: request,
            _partner_id: partner_id,
            _channel_id: channel_id,
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Creates a new channel. Returns the newly created channel if successful.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `partnerId` - The ID of the partner that owns the created channel.
    pub fn channels_create(&self, request: Channel, partner_id: i64) -> PartnerChannelCreateCall<'a, S> {
        PartnerChannelCreateCall {
            hub: self.hub,
            _request: request,
            _partner_id: partner_id,
            _advertiser_id: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Gets a channel for a partner or advertiser.
    /// 
    /// # Arguments
    ///
    /// * `partnerId` - The ID of the partner that owns the fetched channel.
    /// * `channelId` - Required. The ID of the channel to fetch.
    pub fn channels_get(&self, partner_id: i64, channel_id: i64) -> PartnerChannelGetCall<'a, S> {
        PartnerChannelGetCall {
            hub: self.hub,
            _partner_id: partner_id,
            _channel_id: channel_id,
            _advertiser_id: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Lists channels for a partner or advertiser.
    /// 
    /// # Arguments
    ///
    /// * `partnerId` - The ID of the partner that owns the channels.
    pub fn channels_list(&self, partner_id: i64) -> PartnerChannelListCall<'a, S> {
        PartnerChannelListCall {
            hub: self.hub,
            _partner_id: partner_id,
            _page_token: Default::default(),
            _page_size: Default::default(),
            _order_by: Default::default(),
            _filter: Default::default(),
            _advertiser_id: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Updates a channel. Returns the updated channel if successful.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `partnerId` - The ID of the partner that owns the created channel.
    /// * `channelId` - Output only. The unique ID of the channel. Assigned by the system.
    pub fn channels_patch(&self, request: Channel, partner_id: i64, channel_id: i64) -> PartnerChannelPatchCall<'a, S> {
        PartnerChannelPatchCall {
            hub: self.hub,
            _request: request,
            _partner_id: partner_id,
            _channel_id: channel_id,
            _update_mask: Default::default(),
            _advertiser_id: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Assigns a targeting option to a partner. Returns the assigned targeting option if successful.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `partnerId` - Required. The ID of the partner.
    /// * `targetingType` - Required. Identifies the type of this assigned targeting option. Supported targeting types: * `TARGETING_TYPE_CHANNEL`
    pub fn targeting_types_assigned_targeting_options_create(&self, request: AssignedTargetingOption, partner_id: i64, targeting_type: &PartnerTargetingTypeEnum) -> PartnerTargetingTypeAssignedTargetingOptionCreateCall<'a, S> {
        PartnerTargetingTypeAssignedTargetingOptionCreateCall {
            hub: self.hub,
            _request: request,
            _partner_id: partner_id,
            _targeting_type: targeting_type.clone(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Deletes an assigned targeting option from a partner.
    /// 
    /// # Arguments
    ///
    /// * `partnerId` - Required. The ID of the partner.
    /// * `targetingType` - Required. Identifies the type of this assigned targeting option. Supported targeting types: * `TARGETING_TYPE_CHANNEL`
    /// * `assignedTargetingOptionId` - Required. The ID of the assigned targeting option to delete.
    pub fn targeting_types_assigned_targeting_options_delete(&self, partner_id: i64, targeting_type: &PartnerTargetingTypeEnum, assigned_targeting_option_id: &str) -> PartnerTargetingTypeAssignedTargetingOptionDeleteCall<'a, S> {
        PartnerTargetingTypeAssignedTargetingOptionDeleteCall {
            hub: self.hub,
            _partner_id: partner_id,
            _targeting_type: targeting_type.clone(),
            _assigned_targeting_option_id: assigned_targeting_option_id.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Gets a single targeting option assigned to a partner.
    /// 
    /// # Arguments
    ///
    /// * `partnerId` - Required. The ID of the partner.
    /// * `targetingType` - Required. Identifies the type of this assigned targeting option. Supported targeting types: * `TARGETING_TYPE_CHANNEL`
    /// * `assignedTargetingOptionId` - Required. An identifier unique to the targeting type in this partner that identifies the assigned targeting option being requested.
    pub fn targeting_types_assigned_targeting_options_get(&self, partner_id: i64, targeting_type: &PartnerTargetingTypeEnum, assigned_targeting_option_id: &str) -> PartnerTargetingTypeAssignedTargetingOptionGetCall<'a, S> {
        PartnerTargetingTypeAssignedTargetingOptionGetCall {
            hub: self.hub,
            _partner_id: partner_id,
            _targeting_type: targeting_type.clone(),
            _assigned_targeting_option_id: assigned_targeting_option_id.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Lists the targeting options assigned to a partner.
    /// 
    /// # Arguments
    ///
    /// * `partnerId` - Required. The ID of the partner.
    /// * `targetingType` - Required. Identifies the type of assigned targeting options to list. Supported targeting types: * `TARGETING_TYPE_CHANNEL`
    pub fn targeting_types_assigned_targeting_options_list(&self, partner_id: i64, targeting_type: &PartnerTargetingTypeEnum) -> PartnerTargetingTypeAssignedTargetingOptionListCall<'a, S> {
        PartnerTargetingTypeAssignedTargetingOptionListCall {
            hub: self.hub,
            _partner_id: partner_id,
            _targeting_type: targeting_type.clone(),
            _page_token: Default::default(),
            _page_size: Default::default(),
            _order_by: Default::default(),
            _filter: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Bulk edits targeting options under a single partner. The operation will delete the assigned targeting options provided in BulkEditPartnerAssignedTargetingOptionsRequest.deleteRequests and then create the assigned targeting options provided in BulkEditPartnerAssignedTargetingOptionsRequest.createRequests .
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `partnerId` - Required. The ID of the partner.
    pub fn bulk_edit_partner_assigned_targeting_options(&self, request: BulkEditPartnerAssignedTargetingOptionsRequest, partner_id: i64) -> PartnerBulkEditPartnerAssignedTargetingOptionCall<'a, S> {
        PartnerBulkEditPartnerAssignedTargetingOptionCall {
            hub: self.hub,
            _request: request,
            _partner_id: partner_id,
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Gets a partner.
    /// 
    /// # Arguments
    ///
    /// * `partnerId` - Required. The ID of the partner to fetch.
    pub fn get(&self, partner_id: i64) -> PartnerGetCall<'a, S> {
        PartnerGetCall {
            hub: self.hub,
            _partner_id: partner_id,
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Lists partners that are accessible to the current user. The order is defined by the order_by parameter.
    pub fn list(&self) -> PartnerListCall<'a, S> {
        PartnerListCall {
            hub: self.hub,
            _page_token: Default::default(),
            _page_size: Default::default(),
            _order_by: Default::default(),
            _filter: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
}



/// A builder providing access to all methods supported on *sdfdownloadtask* resources.
/// It is not used directly, but through the [`DisplayVideo`] hub.
///
/// # Example
///
/// Instantiate a resource builder
///
/// ```test_harness,no_run
/// extern crate hyper;
/// extern crate hyper_rustls;
/// extern crate google_displayvideo1 as displayvideo1;
/// 
/// # async fn dox() {
/// use std::default::Default;
/// use displayvideo1::{DisplayVideo, oauth2, hyper, hyper_rustls, chrono, FieldMask};
/// 
/// let secret: oauth2::ApplicationSecret = Default::default();
/// let auth = oauth2::InstalledFlowAuthenticator::builder(
///         secret,
///         oauth2::InstalledFlowReturnMethod::HTTPRedirect,
///     ).build().await.unwrap();
/// let mut hub = DisplayVideo::new(hyper::Client::builder().build(hyper_rustls::HttpsConnectorBuilder::new().with_native_roots().https_or_http().enable_http1().build()), auth);
/// // Usually you wouldn't bind this to a variable, but keep calling *CallBuilders*
/// // like `create(...)` and `operations_get(...)`
/// // to build up your call.
/// let rb = hub.sdfdownloadtasks();
/// # }
/// ```
pub struct SdfdownloadtaskMethods<'a, S>
    where S: 'a {

   pub(super) hub: &'a DisplayVideo<S>,
}

impl<'a, S> client::MethodsBuilder for SdfdownloadtaskMethods<'a, S> {}

impl<'a, S> SdfdownloadtaskMethods<'a, S> {
    
    /// Create a builder to help you perform the following task:
    ///
    /// Gets the latest state of an asynchronous SDF download task operation. Clients should poll this method at intervals of 30 seconds.
    /// 
    /// # Arguments
    ///
    /// * `name` - The name of the operation resource.
    pub fn operations_get(&self, name: &str) -> SdfdownloadtaskOperationGetCall<'a, S> {
        SdfdownloadtaskOperationGetCall {
            hub: self.hub,
            _name: name.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Creates an SDF Download Task. Returns an Operation. An SDF Download Task is a long-running, asynchronous operation. The metadata type of this operation is SdfDownloadTaskMetadata. If the request is successful, the response type of the operation is SdfDownloadTask. The response will not include the download files, which must be retrieved with media.download. The state of operation can be retrieved with sdfdownloadtask.operations.get. Any errors can be found in the error.message. Note that error.details is expected to be empty.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    pub fn create(&self, request: CreateSdfDownloadTaskRequest) -> SdfdownloadtaskCreateCall<'a, S> {
        SdfdownloadtaskCreateCall {
            hub: self.hub,
            _request: request,
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
}



/// A builder providing access to all methods supported on *targetingType* resources.
/// It is not used directly, but through the [`DisplayVideo`] hub.
///
/// # Example
///
/// Instantiate a resource builder
///
/// ```test_harness,no_run
/// extern crate hyper;
/// extern crate hyper_rustls;
/// extern crate google_displayvideo1 as displayvideo1;
/// 
/// # async fn dox() {
/// use std::default::Default;
/// use displayvideo1::{DisplayVideo, oauth2, hyper, hyper_rustls, chrono, FieldMask};
/// 
/// let secret: oauth2::ApplicationSecret = Default::default();
/// let auth = oauth2::InstalledFlowAuthenticator::builder(
///         secret,
///         oauth2::InstalledFlowReturnMethod::HTTPRedirect,
///     ).build().await.unwrap();
/// let mut hub = DisplayVideo::new(hyper::Client::builder().build(hyper_rustls::HttpsConnectorBuilder::new().with_native_roots().https_or_http().enable_http1().build()), auth);
/// // Usually you wouldn't bind this to a variable, but keep calling *CallBuilders*
/// // like `targeting_options_get(...)`, `targeting_options_list(...)` and `targeting_options_search(...)`
/// // to build up your call.
/// let rb = hub.targeting_types();
/// # }
/// ```
pub struct TargetingTypeMethods<'a, S>
    where S: 'a {

   pub(super) hub: &'a DisplayVideo<S>,
}

impl<'a, S> client::MethodsBuilder for TargetingTypeMethods<'a, S> {}

impl<'a, S> TargetingTypeMethods<'a, S> {
    
    /// Create a builder to help you perform the following task:
    ///
    /// Gets a single targeting option.
    /// 
    /// # Arguments
    ///
    /// * `targetingType` - Required. The type of targeting option to retrieve. Accepted values are: * `TARGETING_TYPE_APP_CATEGORY` * `TARGETING_TYPE_AGE_RANGE` * `TARGETING_TYPE_GENDER` * `TARGETING_TYPE_VIDEO_PLAYER_SIZE` * `TARGETING_TYPE_USER_REWARDED_CONTENT` * `TARGETING_TYPE_PARENTAL_STATUS` * `TARGETING_TYPE_CONTENT_INSTREAM_POSITION` * `TARGETING_TYPE_CONTENT_OUTSTREAM_POSITION` * `TARGETING_TYPE_DEVICE_TYPE` * `TARGETING_TYPE_BROWSER` * `TARGETING_TYPE_HOUSEHOLD_INCOME` * `TARGETING_TYPE_ON_SCREEN_POSITION` * `TARGETING_TYPE_CARRIER_AND_ISP` * `TARGETING_TYPE_OPERATING_SYSTEM` * `TARGETING_TYPE_DEVICE_MAKE_MODEL` * `TARGETING_TYPE_ENVIRONMENT` * `TARGETING_TYPE_CATEGORY` * `TARGETING_TYPE_VIEWABILITY` * `TARGETING_TYPE_AUTHORIZED_SELLER_STATUS` * `TARGETING_TYPE_LANGUAGE` * `TARGETING_TYPE_GEO_REGION` * `TARGETING_TYPE_DIGITAL_CONTENT_LABEL_EXCLUSION` * `TARGETING_TYPE_SENSITIVE_CATEGORY_EXCLUSION` * `TARGETING_TYPE_EXCHANGE` * `TARGETING_TYPE_SUB_EXCHANGE` * `TARGETING_TYPE_NATIVE_CONTENT_POSITION` * `TARGETING_TYPE_OMID`
    /// * `targetingOptionId` - Required. The ID of the of targeting option to retrieve.
    pub fn targeting_options_get(&self, targeting_type: &TargetingTypeTargetingTypeEnum, targeting_option_id: &str) -> TargetingTypeTargetingOptionGetCall<'a, S> {
        TargetingTypeTargetingOptionGetCall {
            hub: self.hub,
            _targeting_type: targeting_type.clone(),
            _targeting_option_id: targeting_option_id.to_string(),
            _advertiser_id: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Lists targeting options of a given type.
    /// 
    /// # Arguments
    ///
    /// * `targetingType` - Required. The type of targeting option to be listed. Accepted values are: * `TARGETING_TYPE_APP_CATEGORY` * `TARGETING_TYPE_AGE_RANGE` * `TARGETING_TYPE_GENDER` * `TARGETING_TYPE_VIDEO_PLAYER_SIZE` * `TARGETING_TYPE_USER_REWARDED_CONTENT` * `TARGETING_TYPE_PARENTAL_STATUS` * `TARGETING_TYPE_CONTENT_INSTREAM_POSITION` * `TARGETING_TYPE_CONTENT_OUTSTREAM_POSITION` * `TARGETING_TYPE_DEVICE_TYPE` * `TARGETING_TYPE_BROWSER` * `TARGETING_TYPE_HOUSEHOLD_INCOME` * `TARGETING_TYPE_ON_SCREEN_POSITION` * `TARGETING_TYPE_CARRIER_AND_ISP` * `TARGETING_TYPE_OPERATING_SYSTEM` * `TARGETING_TYPE_DEVICE_MAKE_MODEL` * `TARGETING_TYPE_ENVIRONMENT` * `TARGETING_TYPE_CATEGORY` * `TARGETING_TYPE_VIEWABILITY` * `TARGETING_TYPE_AUTHORIZED_SELLER_STATUS` * `TARGETING_TYPE_LANGUAGE` * `TARGETING_TYPE_GEO_REGION` * `TARGETING_TYPE_DIGITAL_CONTENT_LABEL_EXCLUSION` * `TARGETING_TYPE_SENSITIVE_CATEGORY_EXCLUSION` * `TARGETING_TYPE_EXCHANGE` * `TARGETING_TYPE_SUB_EXCHANGE` * `TARGETING_TYPE_NATIVE_CONTENT_POSITION` * `TARGETING_TYPE_OMID`
    pub fn targeting_options_list(&self, targeting_type: &TargetingTypeTargetingTypeEnum) -> TargetingTypeTargetingOptionListCall<'a, S> {
        TargetingTypeTargetingOptionListCall {
            hub: self.hub,
            _targeting_type: targeting_type.clone(),
            _page_token: Default::default(),
            _page_size: Default::default(),
            _order_by: Default::default(),
            _filter: Default::default(),
            _advertiser_id: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Searches for targeting options of a given type based on the given search terms.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `targetingType` - Required. The type of targeting options to retrieve. Accepted values are: * `TARGETING_TYPE_GEO_REGION` * `TARGETING_TYPE_POI` * `TARGETING_TYPE_BUSINESS_CHAIN`
    pub fn targeting_options_search(&self, request: SearchTargetingOptionsRequest, targeting_type: &TargetingTypeTargetingTypeEnum) -> TargetingTypeTargetingOptionSearchCall<'a, S> {
        TargetingTypeTargetingOptionSearchCall {
            hub: self.hub,
            _request: request,
            _targeting_type: targeting_type.clone(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
}



/// A builder providing access to all methods supported on *user* resources.
/// It is not used directly, but through the [`DisplayVideo`] hub.
///
/// # Example
///
/// Instantiate a resource builder
///
/// ```test_harness,no_run
/// extern crate hyper;
/// extern crate hyper_rustls;
/// extern crate google_displayvideo1 as displayvideo1;
/// 
/// # async fn dox() {
/// use std::default::Default;
/// use displayvideo1::{DisplayVideo, oauth2, hyper, hyper_rustls, chrono, FieldMask};
/// 
/// let secret: oauth2::ApplicationSecret = Default::default();
/// let auth = oauth2::InstalledFlowAuthenticator::builder(
///         secret,
///         oauth2::InstalledFlowReturnMethod::HTTPRedirect,
///     ).build().await.unwrap();
/// let mut hub = DisplayVideo::new(hyper::Client::builder().build(hyper_rustls::HttpsConnectorBuilder::new().with_native_roots().https_or_http().enable_http1().build()), auth);
/// // Usually you wouldn't bind this to a variable, but keep calling *CallBuilders*
/// // like `bulk_edit_assigned_user_roles(...)`, `create(...)`, `delete(...)`, `get(...)`, `list(...)` and `patch(...)`
/// // to build up your call.
/// let rb = hub.users();
/// # }
/// ```
pub struct UserMethods<'a, S>
    where S: 'a {

   pub(super) hub: &'a DisplayVideo<S>,
}

impl<'a, S> client::MethodsBuilder for UserMethods<'a, S> {}

impl<'a, S> UserMethods<'a, S> {
    
    /// Create a builder to help you perform the following task:
    ///
    /// Bulk edits user roles for a user. The operation will delete the assigned user roles provided in BulkEditAssignedUserRolesRequest.deletedAssignedUserRoles and then assign the user roles provided in BulkEditAssignedUserRolesRequest.createdAssignedUserRoles.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `userId` - Required. The ID of the user to which the assigned user roles belong.
    pub fn bulk_edit_assigned_user_roles(&self, request: BulkEditAssignedUserRolesRequest, user_id: i64) -> UserBulkEditAssignedUserRoleCall<'a, S> {
        UserBulkEditAssignedUserRoleCall {
            hub: self.hub,
            _request: request,
            _user_id: user_id,
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Creates a new user. Returns the newly created user if successful.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    pub fn create(&self, request: User) -> UserCreateCall<'a, S> {
        UserCreateCall {
            hub: self.hub,
            _request: request,
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Deletes a user.
    /// 
    /// # Arguments
    ///
    /// * `userId` - Required. The ID of the user to delete.
    pub fn delete(&self, user_id: i64) -> UserDeleteCall<'a, S> {
        UserDeleteCall {
            hub: self.hub,
            _user_id: user_id,
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Gets a user.
    /// 
    /// # Arguments
    ///
    /// * `userId` - Required. The ID of the user to fetch.
    pub fn get(&self, user_id: i64) -> UserGetCall<'a, S> {
        UserGetCall {
            hub: self.hub,
            _user_id: user_id,
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Lists users that are accessible to the current user. If two users have user roles on the same partner or advertiser, they can access each other.
    pub fn list(&self) -> UserListCall<'a, S> {
        UserListCall {
            hub: self.hub,
            _page_token: Default::default(),
            _page_size: Default::default(),
            _order_by: Default::default(),
            _filter: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Updates an existing user. Returns the updated user if successful.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `userId` - Output only. The unique ID of the user. Assigned by the system.
    pub fn patch(&self, request: User, user_id: i64) -> UserPatchCall<'a, S> {
        UserPatchCall {
            hub: self.hub,
            _request: request,
            _user_id: user_id,
            _update_mask: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
}



