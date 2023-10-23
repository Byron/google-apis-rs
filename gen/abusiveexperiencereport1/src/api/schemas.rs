use super::*;
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
    /// The site's Abusive Experience Report status.
    #[serde(rename="abusiveStatus")]
    
    pub abusive_status: Option<SiteSummaryResponseAbusiveStatusEnum>,
    /// The time at which [enforcement](https://support.google.com/webtools/answer/7538608) against the site began or will begin. Not set when the filter_status is OFF.
    #[serde(rename="enforcementTime")]
    
    pub enforcement_time: Option<client::chrono::DateTime<client::chrono::offset::Utc>>,
    /// The site's [enforcement status](https://support.google.com/webtools/answer/7538608).
    #[serde(rename="filterStatus")]
    
    pub filter_status: Option<SiteSummaryResponseFilterStatusEnum>,
    /// The time at which the site's status last changed.
    #[serde(rename="lastChangeTime")]
    
    pub last_change_time: Option<client::chrono::DateTime<client::chrono::offset::Utc>>,
    /// A link to the full Abusive Experience Report for the site. Not set in ViolatingSitesResponse. Note that you must complete the [Search Console verification process](https://support.google.com/webmasters/answer/9008080) for the site before you can access the full report.
    #[serde(rename="reportUrl")]
    
    pub report_url: Option<String>,
    /// The name of the reviewed site, e.g. `google.com`.
    #[serde(rename="reviewedSite")]
    
    pub reviewed_site: Option<String>,
    /// Whether the site is currently under review.
    #[serde(rename="underReview")]
    
    pub under_review: Option<bool>,
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


