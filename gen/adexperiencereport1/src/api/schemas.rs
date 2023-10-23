use super::*;
/// A site's Ad Experience Report summary on a single platform.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct PlatformSummary {
    /// The site's Ad Experience Report status on this platform.
    #[serde(rename="betterAdsStatus")]
    
    pub better_ads_status: Option<PlatformSummaryBetterAdsStatusEnum>,
    /// The time at which [enforcement](https://support.google.com/webtools/answer/7308033) against the site began or will begin on this platform. Not set when the filter_status is OFF.
    #[serde(rename="enforcementTime")]
    
    pub enforcement_time: Option<client::chrono::DateTime<client::chrono::offset::Utc>>,
    /// The site's [enforcement status](https://support.google.com/webtools/answer/7308033) on this platform.
    #[serde(rename="filterStatus")]
    
    pub filter_status: Option<PlatformSummaryFilterStatusEnum>,
    /// The time at which the site's status last changed on this platform.
    #[serde(rename="lastChangeTime")]
    
    pub last_change_time: Option<client::chrono::DateTime<client::chrono::offset::Utc>>,
    /// The site's regions on this platform. No longer populated, because there is no longer any semantic difference between sites in different regions.
    
    pub region: Option<Vec<PlatformSummaryRegionEnum>>,
    /// A link to the full Ad Experience Report for the site on this platform.. Not set in ViolatingSitesResponse. Note that you must complete the [Search Console verification process](https://support.google.com/webmasters/answer/9008080) for the site before you can access the full report.
    #[serde(rename="reportUrl")]
    
    pub report_url: Option<String>,
    /// Whether the site is currently under review on this platform.
    #[serde(rename="underReview")]
    
    pub under_review: Option<bool>,
}

impl client::Part for PlatformSummary {}


/// Response message for GetSiteSummary.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [get sites](SiteGetCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct SiteSummaryResponse {
    /// The site's Ad Experience Report summary on desktop.
    #[serde(rename="desktopSummary")]
    
    pub desktop_summary: Option<PlatformSummary>,
    /// The site's Ad Experience Report summary on mobile.
    #[serde(rename="mobileSummary")]
    
    pub mobile_summary: Option<PlatformSummary>,
    /// The name of the reviewed site, e.g. `google.com`.
    #[serde(rename="reviewedSite")]
    
    pub reviewed_site: Option<String>,
}

impl client::ResponseResult for SiteSummaryResponse {}


/// Response message for ListViolatingSites.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [list violating sites](ViolatingSiteListCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct ViolatingSitesResponse {
    /// The list of violating sites.
    #[serde(rename="violatingSites")]
    
    pub violating_sites: Option<Vec<SiteSummaryResponse>>,
}

impl client::ResponseResult for ViolatingSitesResponse {}


