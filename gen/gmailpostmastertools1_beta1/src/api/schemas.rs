use super::*;
/// Metric on a particular delivery error type.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct DeliveryError {
    /// The class of delivery error.
    #[serde(rename="errorClass")]
    
    pub error_class: Option<DeliveryErrorErrorClassEnum>,
    /// The ratio of messages where the error occurred vs all authenticated traffic.
    #[serde(rename="errorRatio")]
    
    pub error_ratio: Option<f64>,
    /// The type of delivery error.
    #[serde(rename="errorType")]
    
    pub error_type: Option<DeliveryErrorErrorTypeEnum>,
}

impl client::Part for DeliveryError {}


/// A registered domain resource in the Postmaster API.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [traffic stats get domains](DomainTrafficStatGetCall) (none)
/// * [traffic stats list domains](DomainTrafficStatListCall) (none)
/// * [get domains](DomainGetCall) (response)
/// * [list domains](DomainListCall) (none)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct Domain {
    /// Timestamp when the user registered this domain. Assigned by the server.
    #[serde(rename="createTime")]
    
    pub create_time: Option<client::chrono::DateTime<client::chrono::offset::Utc>>,
    /// The resource name of the Domain. Domain names have the form `domains/{domain_name}`, where domain_name is the fully qualified domain name (i.e., mymail.mydomain.com).
    
    pub name: Option<String>,
    /// User’s permission for this domain. Assigned by the server.
    
    pub permission: Option<DomainPermissionEnum>,
}

impl client::Resource for Domain {}
impl client::ResponseResult for Domain {}


/// [Feedback loop](https://support.google.com/mail/answer/6254652) identifier information.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct FeedbackLoop {
    /// Feedback loop identifier that uniquely identifies individual campaigns.
    
    pub id: Option<String>,
    /// The ratio of user marked spam messages with the identifier vs the total number of inboxed messages with that identifier.
    #[serde(rename="spamRatio")]
    
    pub spam_ratio: Option<f64>,
}

impl client::Part for FeedbackLoop {}


/// IP Reputation information for a set of IPs in a specific reputation category.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct IpReputation {
    /// Total number of unique IPs in this reputation category. This metric only pertains to traffic that passed [SPF](http://www.openspf.org/) or [DKIM](http://www.dkim.org/).
    #[serde(rename="ipCount")]
    
    #[serde_as(as = "Option<::client::serde_with::DisplayFromStr>")]
    pub ip_count: Option<i64>,
    /// Total number of unique IPs in this reputation category. This metric only pertains to traffic that passed [SPF](http://www.openspf.org/) or [DKIM](http://www.dkim.org/). Deprecated to be complied with ApiLinter for Quantities. Use ip_count instead.
    #[serde(rename="numIps")]
    
    #[serde_as(as = "Option<::client::serde_with::DisplayFromStr>")]
    pub num_ips: Option<i64>,
    /// The reputation category this IP reputation represents.
    
    pub reputation: Option<IpReputationReputationEnum>,
    /// A sample of IPs in this reputation category.
    #[serde(rename="sampleIps")]
    
    pub sample_ips: Option<Vec<String>>,
}

impl client::Part for IpReputation {}


/// Response message for ListDomains.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [list domains](DomainListCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct ListDomainsResponse {
    /// The list of domains.
    
    pub domains: Option<Vec<Domain>>,
    /// Token to retrieve the next page of results, or empty if there are no more results in the list.
    #[serde(rename="nextPageToken")]
    
    pub next_page_token: Option<String>,
}

impl client::ResponseResult for ListDomainsResponse {}


/// Response message for ListTrafficStats.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [traffic stats list domains](DomainTrafficStatListCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct ListTrafficStatsResponse {
    /// Token to retrieve the next page of results, or empty if there are no more results in the list.
    #[serde(rename="nextPageToken")]
    
    pub next_page_token: Option<String>,
    /// The list of TrafficStats.
    #[serde(rename="trafficStats")]
    
    pub traffic_stats: Option<Vec<TrafficStats>>,
}

impl client::ResponseResult for ListTrafficStatsResponse {}


/// Email traffic statistics pertaining to a specific date.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [traffic stats get domains](DomainTrafficStatGetCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct TrafficStats {
    /// Delivery errors for the domain. This metric only pertains to traffic that passed [SPF](http://www.openspf.org/) or [DKIM](http://www.dkim.org/).
    #[serde(rename="deliveryErrors")]
    
    pub delivery_errors: Option<Vec<DeliveryError>>,
    /// The ratio of mail that successfully authenticated with DKIM vs. all mail that attempted to authenticate with [DKIM](http://www.dkim.org/). Spoofed mail is excluded.
    #[serde(rename="dkimSuccessRatio")]
    
    pub dkim_success_ratio: Option<f64>,
    /// The ratio of mail that passed [DMARC](https://dmarc.org/) alignment checks vs all mail received from the domain that successfully authenticated with either of [SPF](http://www.openspf.org/) or [DKIM](http://www.dkim.org/).
    #[serde(rename="dmarcSuccessRatio")]
    
    pub dmarc_success_ratio: Option<f64>,
    /// Reputation of the domain.
    #[serde(rename="domainReputation")]
    
    pub domain_reputation: Option<TrafficStatDomainReputationEnum>,
    /// The ratio of incoming mail (to Gmail), that passed secure transport (TLS) vs all mail received from that domain. This metric only pertains to traffic that passed [SPF](http://www.openspf.org/) or [DKIM](http://www.dkim.org/).
    #[serde(rename="inboundEncryptionRatio")]
    
    pub inbound_encryption_ratio: Option<f64>,
    /// Reputation information pertaining to the IP addresses of the email servers for the domain. There is exactly one entry for each reputation category except REPUTATION_CATEGORY_UNSPECIFIED.
    #[serde(rename="ipReputations")]
    
    pub ip_reputations: Option<Vec<IpReputation>>,
    /// The resource name of the traffic statistics. Traffic statistic names have the form `domains/{domain}/trafficStats/{date}`, where domain_name is the fully qualified domain name (i.e., mymail.mydomain.com) of the domain this traffic statistics pertains to and date is the date in yyyymmdd format that these statistics corresponds to. For example: domains/mymail.mydomain.com/trafficStats/20160807
    
    pub name: Option<String>,
    /// The ratio of outgoing mail (from Gmail) that was accepted over secure transport (TLS).
    #[serde(rename="outboundEncryptionRatio")]
    
    pub outbound_encryption_ratio: Option<f64>,
    /// Spammy [Feedback loop identifiers] (https://support.google.com/mail/answer/6254652) with their individual spam rates. This metric only pertains to traffic that is authenticated by [DKIM](http://www.dkim.org/).
    #[serde(rename="spammyFeedbackLoops")]
    
    pub spammy_feedback_loops: Option<Vec<FeedbackLoop>>,
    /// The ratio of mail that successfully authenticated with SPF vs. all mail that attempted to authenticate with [SPF](http://www.openspf.org/). Spoofed mail is excluded.
    #[serde(rename="spfSuccessRatio")]
    
    pub spf_success_ratio: Option<f64>,
    /// The ratio of user-report spam vs. email that was sent to the inbox. This metric only pertains to emails authenticated by [DKIM](http://www.dkim.org/).
    #[serde(rename="userReportedSpamRatio")]
    
    pub user_reported_spam_ratio: Option<f64>,
}

impl client::ResponseResult for TrafficStats {}


