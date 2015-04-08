// DO NOT EDIT !
// This file was generated automatically from 'src/mako/api/lib.rs.mako'
// DO NOT EDIT !

//! This documentation was generated from *Civic Info* crate version *0.1.5+20150302*, where *20150302* is the exact revision of the *civicinfo:v2* schema built by the [mako](http://www.makotemplates.org/) code generator *v0.1.5*.
//! 
//! Everything else about the *Civic Info* *v2* API can be found at the
//! [official documentation site](https://developers.google.com/civic-information).
//! The original source code is [on github](https://github.com/Byron/google-apis-rs/tree/master/gen/civicinfo2).
//! # Features
//! 
//! Handle the following *Resources* with ease from the central [hub](struct.CivicInfo.html) ... 
//! 
//! * divisions
//!  * [*search*](struct.DivisionSearchCall.html)
//! * [elections](struct.Election.html)
//!  * [*election query*](struct.ElectionElectionQueryCall.html) and [*voter info query*](struct.ElectionVoterInfoQueryCall.html)
//! * representatives
//!  * [*representative info by address*](struct.RepresentativeRepresentativeInfoByAddresCall.html) and [*representative info by division*](struct.RepresentativeRepresentativeInfoByDivisionCall.html)
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
//! * **[Hub](struct.CivicInfo.html)**
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
//! let r = hub.elections().voter_info_query(...).doit()
//! let r = hub.elections().election_query(...).doit()
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
//! google-civicinfo2 = "*"
//! ```
//! 
//! ## A complete example
//! 
//! ```test_harness,no_run
//! extern crate hyper;
//! extern crate yup_oauth2 as oauth2;
//! extern crate google_civicinfo2 as civicinfo2;
//! use civicinfo2::{Result, Error};
//! # #[test] fn egal() {
//! use std::default::Default;
//! use oauth2::{Authenticator, DefaultAuthenticatorDelegate, ApplicationSecret, MemoryStorage};
//! use civicinfo2::CivicInfo;
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
//! let mut hub = CivicInfo::new(hyper::Client::new(), auth);
//! // You can configure optional parameters by calling the respective setters at will, and
//! // execute the final call using `doit()`.
//! // Values shown here are possibly random and not representative !
//! let result = hub.elections().voter_info_query("address")
//!              .official_only(false)
//!              .election_id("dolores")
//!              .doit();
//! 
//! match result {
//!     Err(e) => match e {
//!         Error::HttpError(err) => println!("HTTPERROR: {:?}", err),
//!         Error::MissingAPIKey => println!("Auth: Missing API Key - used if there are no scopes"),
//!         Error::MissingToken => println!("OAuth2: Missing Token"),
//!         Error::Cancelled => println!("Operation canceled by user"),
//!         Error::UploadSizeLimitExceeded(size, max_size) => println!("Upload size too big: {} of {}", size, max_size),
//!         Error::Failure(_) => println!("General Failure (hyper::client::Response doesn't print)"),
//!         Error::FieldClash(clashed_field) => println!("You added custom parameter which is part of builder: {:?}", clashed_field),
//!         Error::JsonDecodeError(err) => println!("Couldn't understand server reply - maybe API needs update: {:?}", err),
//!     },
//!     Ok(_) => println!("Success (value doesn't print)"),
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
//! * [request values](trait.RequestValue.html) are borrowed
//! 
//! Arguments will always be copied or cloned into the builder, to make them independent of their original life times.
//! 
//! [wiki-pod]: http://en.wikipedia.org/wiki/Plain_old_data_structure
//! [builder-pattern]: http://en.wikipedia.org/wiki/Builder_pattern
//! [google-go-api]: https://github.com/google/google-api-go-client
//! 
//! 
#![feature(std_misc)]
// Unused attributes happen thanks to defined, but unused structures
// We don't warn about this, as depending on the API, some data structures or facilities are never used.
// Instead of pre-determining this, we just disable the lint. It's manually tuned to not have any 
// unused imports in fully featured APIs. Same with unused_mut ... .
#![allow(unused_imports, unused_mut, dead_code)]
// Required for serde annotations
#![feature(custom_derive, custom_attribute, plugin, slice_patterns)]
#![plugin(serde_macros)]

#[macro_use]
extern crate hyper;
extern crate serde;
extern crate yup_oauth2 as oauth2;
extern crate mime;
extern crate url;

mod cmn;

use std::collections::HashMap;
use std::cell::RefCell;
use std::borrow::BorrowMut;
use std::default::Default;
use std::collections::BTreeMap;
use serde::json;
use std::io;
use std::fs;
use std::thread::sleep_ms;

pub use cmn::{MultiPartReader, ToParts, MethodInfo, Result, Error, CallBuilder, Hub, ReadSeek, Part, ResponseResult, RequestValue, NestedType, Delegate, DefaultDelegate, MethodsBuilder, Resource, JsonServerError};


// ##############
// UTILITIES ###
// ############




// ########
// HUB ###
// ######

/// Central instance to access all CivicInfo related resource activities
///
/// # Examples
///
/// Instantiate a new hub
///
/// ```test_harness,no_run
/// extern crate hyper;
/// extern crate yup_oauth2 as oauth2;
/// extern crate google_civicinfo2 as civicinfo2;
/// use civicinfo2::{Result, Error};
/// # #[test] fn egal() {
/// use std::default::Default;
/// use oauth2::{Authenticator, DefaultAuthenticatorDelegate, ApplicationSecret, MemoryStorage};
/// use civicinfo2::CivicInfo;
/// 
/// // Get an ApplicationSecret instance by some means. It contains the `client_id` and 
/// // `client_secret`, among other things.
/// let secret: ApplicationSecret = Default::default();
/// // Instantiate the authenticator. It will choose a suitable authentication flow for you, 
/// // unless you replace  `None` with the desired Flow.
/// // Provide your own `AuthenticatorDelegate` to adjust the way it operates and get feedback about 
/// // what's going on. You probably want to bring in your own `TokenStorage` to persist tokens and
/// // retrieve them from storage.
/// let auth = Authenticator::new(&secret, DefaultAuthenticatorDelegate,
///                               hyper::Client::new(),
///                               <MemoryStorage as Default>::default(), None);
/// let mut hub = CivicInfo::new(hyper::Client::new(), auth);
/// // You can configure optional parameters by calling the respective setters at will, and
/// // execute the final call using `doit()`.
/// // Values shown here are possibly random and not representative !
/// let result = hub.elections().voter_info_query("address")
///              .official_only(true)
///              .election_id("takimata")
///              .doit();
/// 
/// match result {
///     Err(e) => match e {
///         Error::HttpError(err) => println!("HTTPERROR: {:?}", err),
///         Error::MissingAPIKey => println!("Auth: Missing API Key - used if there are no scopes"),
///         Error::MissingToken => println!("OAuth2: Missing Token"),
///         Error::Cancelled => println!("Operation canceled by user"),
///         Error::UploadSizeLimitExceeded(size, max_size) => println!("Upload size too big: {} of {}", size, max_size),
///         Error::Failure(_) => println!("General Failure (hyper::client::Response doesn't print)"),
///         Error::FieldClash(clashed_field) => println!("You added custom parameter which is part of builder: {:?}", clashed_field),
///         Error::JsonDecodeError(err) => println!("Couldn't understand server reply - maybe API needs update: {:?}", err),
///     },
///     Ok(_) => println!("Success (value doesn't print)"),
/// }
/// # }
/// ```
pub struct CivicInfo<C, A> {
    client: RefCell<C>,
    auth: RefCell<A>,
    _user_agent: String,
}

impl<'a, C, A> Hub for CivicInfo<C, A> {}

impl<'a, C, A> CivicInfo<C, A>
    where  C: BorrowMut<hyper::Client>, A: oauth2::GetToken {

    pub fn new(client: C, authenticator: A) -> CivicInfo<C, A> {
        CivicInfo {
            client: RefCell::new(client),
            auth: RefCell::new(authenticator),
            _user_agent: "google-api-rust-client/0.1.5".to_string(),
        }
    }

    pub fn divisions(&'a self) -> DivisionMethods<'a, C, A> {
        DivisionMethods { hub: &self }
    }
    pub fn elections(&'a self) -> ElectionMethods<'a, C, A> {
        ElectionMethods { hub: &self }
    }
    pub fn representatives(&'a self) -> RepresentativeMethods<'a, C, A> {
        RepresentativeMethods { hub: &self }
    }

    /// Set the user-agent header field to use in all requests to the server.
    /// It defaults to `google-api-rust-client/0.1.5`.
    ///
    /// Returns the previously set user-agent.
    pub fn user_agent(&mut self, agent_name: String) -> String {
        let prev = self._user_agent.clone();
        self._user_agent = agent_name;
        prev
    }
}


// ############
// SCHEMAS ###
// ##########
/// Describes information about a regional election administrative area.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Deserialize)]
pub struct AdministrationRegion {
    /// The election administration body for this area.
    #[serde(rename="electionAdministrationBody")]
    pub election_administration_body: AdministrativeBody,
    /// The city or county that provides election information for this voter. This object can have the same elements as state.
    pub local_jurisdiction: Option<Box<AdministrationRegion>>,
    /// An ID for this object. IDs may change in future requests and should not be cached. Access to this field requires special access that can be requested from the Request more link on the Quotas page.
    pub id: String,
    /// The name of the jurisdiction.
    pub name: String,
    /// A list of sources for this area. If multiple sources are listed the data has been aggregated from those sources.
    pub sources: Vec<Source>,
}

impl Part for AdministrationRegion {}


/// Information about a person holding an elected office.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Deserialize)]
pub struct Official {
    /// The official's name.
    pub name: String,
    /// A URL for a photo of the official.
    #[serde(rename="photoUrl")]
    pub photo_url: String,
    /// The official's public contact phone numbers.
    pub phones: Vec<String>,
    /// A list of known (social) media channels for this official.
    pub channels: Vec<Channel>,
    /// The official's public website URLs.
    pub urls: Vec<String>,
    /// Addresses at which to contact the official.
    pub address: Vec<SimpleAddressType>,
    /// The full name of the party the official belongs to.
    pub party: String,
    /// The direct email addresses for the official.
    pub emails: Vec<String>,
}

impl Part for Official {}


/// Describes the geographic scope of a contest.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Deserialize)]
pub struct ElectoralDistrict {
    /// The geographic scope of this district. If unspecified the district's geography is not known. One of: national, statewide, congressional, stateUpper, stateLower, countywide, judicial, schoolBoard, cityWide, township, countyCouncil, cityCouncil, ward, special
    pub scope: String,
    /// An identifier for this district, relative to its scope. For example, the 34th State Senate district would have id "34" and a scope of stateUpper.
    pub id: String,
    /// The name of the district.
    pub name: String,
}

impl Part for ElectoralDistrict {}


/// The result of a division search query.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [search divisions](struct.DivisionSearchCall.html) (response)
/// 
#[derive(Default, Clone, Debug, Deserialize)]
pub struct DivisionSearchResponse {
    /// Identifies what kind of resource this is. Value: the fixed string "civicinfo#divisionSearchResponse".
    pub kind: String,
    /// no description provided
    pub results: Vec<DivisionSearchResult>,
}

impl ResponseResult for DivisionSearchResponse {}


/// Information about a candidate running for elected office.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Deserialize)]
pub struct Candidate {
    /// The candidate's name.
    pub name: String,
    /// A URL for a photo of the candidate.
    #[serde(rename="photoUrl")]
    pub photo_url: String,
    /// The URL for the candidate's campaign web site.
    #[serde(rename="candidateUrl")]
    pub candidate_url: String,
    /// A list of known (social) media channels for this candidate.
    pub channels: Vec<Channel>,
    /// The voice phone number for the candidate's campaign office.
    pub phone: String,
    /// The order the candidate appears on the ballot for this contest.
    #[serde(rename="orderOnBallot")]
    pub order_on_ballot: String,
    /// The full name of the party the candidate is a member of.
    pub party: String,
    /// The email address for the candidate's campaign.
    pub email: String,
}

impl Part for Candidate {}


/// Information about a contest that appears on a voter's ballot.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Deserialize)]
pub struct Contest {
    /// The name of the office for this contest.
    pub office: String,
    /// A list of sources for this contest. If multiple sources are listed, the data has been aggregated from those sources.
    pub sources: Vec<Source>,
    /// If this is a partisan election, the name of the party it is for.
    #[serde(rename="primaryParty")]
    pub primary_party: String,
    /// A description of any additional eligibility requirements for voting in this contest.
    #[serde(rename="electorateSpecifications")]
    pub electorate_specifications: String,
    /// The number of candidates that will be elected to office in this contest.
    #[serde(rename="numberElected")]
    pub number_elected: String,
    /// A link to the referendum. This field is only populated for contests of type 'Referendum'.
    #[serde(rename="referendumUrl")]
    pub referendum_url: String,
    /// The title of the referendum (e.g. 'Proposition 42'). This field is only populated for contests of type 'Referendum'.
    #[serde(rename="referendumTitle")]
    pub referendum_title: String,
    /// An ID for this object. IDs may change in future requests and should not be cached. Access to this field requires special access that can be requested from the Request more link on the Quotas page.
    pub id: String,
    /// "Yes" or "No" depending on whether this a contest being held outside the normal election cycle.
    pub special: String,
    /// The number of candidates that a voter may vote for in this contest.
    #[serde(rename="numberVotingFor")]
    pub number_voting_for: String,
    /// Information about the electoral district that this contest is in.
    pub district: ElectoralDistrict,
    /// The roles which this office fulfills.
    pub roles: Vec<String>,
    /// The levels of government of the office for this contest. There may be more than one in cases where a jurisdiction effectively acts at two different levels of government; for example, the mayor of the District of Columbia acts at "locality" level, but also effectively at both "administrative-area-2" and "administrative-area-1".
    pub level: Vec<String>,
    /// A number specifying the position of this contest on the voter's ballot.
    #[serde(rename="ballotPlacement")]
    pub ballot_placement: String,
    /// A brief description of the referendum. This field is only populated for contests of type 'Referendum'.
    #[serde(rename="referendumSubtitle")]
    pub referendum_subtitle: String,
    /// The candidate choices for this contest.
    pub candidates: Vec<Candidate>,
    /// The type of contest. Usually this will be 'General', 'Primary', or 'Run-off' for contests with candidates. For referenda this will be 'Referendum'.
    #[serde(rename="type")]
    pub type_: String,
}

impl Part for Contest {}


/// The list of elections available for this version of the API.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [election query elections](struct.ElectionElectionQueryCall.html) (response)
/// 
#[derive(Default, Clone, Debug, Deserialize)]
pub struct ElectionsQueryResponse {
    /// Identifies what kind of resource this is. Value: the fixed string "civicinfo#electionsQueryResponse".
    pub kind: String,
    /// A list of available elections
    pub elections: Vec<Election>,
}

impl ResponseResult for ElectionsQueryResponse {}


/// The result of a representative info lookup query.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [representative info by address representatives](struct.RepresentativeRepresentativeInfoByAddresCall.html) (response)
/// 
#[derive(Default, Clone, Debug, Deserialize)]
pub struct RepresentativeInfoResponse {
    /// Political geographic divisions that contain the requested address.
    pub divisions: HashMap<String, GeographicDivision>,
    /// Identifies what kind of resource this is. Value: the fixed string "civicinfo#representativeInfoResponse".
    pub kind: String,
    /// Officials holding the offices listed above. Will only be present if includeOffices was true in the request.
    pub officials: Vec<Official>,
    /// Elected offices referenced by the divisions listed above. Will only be present if includeOffices was true in the request.
    pub offices: Vec<Office>,
    /// The normalized version of the requested address
    #[serde(rename="normalizedInput")]
    pub normalized_input: SimpleAddressType,
}

impl ResponseResult for RepresentativeInfoResponse {}


/// Contains information about the data source for the element containing it.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Deserialize)]
pub struct Source {
    /// Whether this data comes from an official government source.
    pub official: bool,
    /// The name of the data source.
    pub name: String,
}

impl Part for Source {}


/// Information about individual election officials.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Deserialize)]
pub struct ElectionOfficial {
    /// The fax number of the election official.
    #[serde(rename="faxNumber")]
    pub fax_number: String,
    /// The email address of the election official.
    #[serde(rename="emailAddress")]
    pub email_address: String,
    /// The full name of the election official.
    pub name: String,
    /// The office phone number of the election official.
    #[serde(rename="officePhoneNumber")]
    pub office_phone_number: String,
    /// The title of the election official.
    pub title: String,
}

impl Part for ElectionOfficial {}


/// Information about an election administrative body (e.g. County Board of Elections).
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Deserialize)]
pub struct AdministrativeBody {
    /// A URL provided by this administrative body for information on absentee voting.
    #[serde(rename="absenteeVotingInfoUrl")]
    pub absentee_voting_info_url: String,
    /// A URL provided by this administrative body for looking up where to vote.
    #[serde(rename="votingLocationFinderUrl")]
    pub voting_location_finder_url: String,
    /// The name of this election administrative body.
    pub name: String,
    /// A URL provided by this administrative body for confirming that the voter is registered to vote.
    #[serde(rename="electionRegistrationConfirmationUrl")]
    pub election_registration_confirmation_url: String,
    /// A URL provided by this administrative body for looking up general election information.
    #[serde(rename="electionInfoUrl")]
    pub election_info_url: String,
    /// A URL provided by this administrative body for looking up how to register to vote.
    #[serde(rename="electionRegistrationUrl")]
    pub election_registration_url: String,
    /// The election officials for this election administrative body.
    #[serde(rename="electionOfficials")]
    pub election_officials: Vec<ElectionOfficial>,
    /// The mailing address of this administrative body.
    #[serde(rename="correspondenceAddress")]
    pub correspondence_address: SimpleAddressType,
    /// A URL provided by this administrative body describing election rules to the voter.
    #[serde(rename="electionRulesUrl")]
    pub election_rules_url: String,
    /// A description of the services this administrative body may provide.
    pub voter_services: Vec<String>,
    /// A URL provided by this administrative body to give contest information to the voter.
    #[serde(rename="ballotInfoUrl")]
    pub ballot_info_url: String,
    /// A description of the hours of operation for this administrative body.
    #[serde(rename="hoursOfOperation")]
    pub hours_of_operation: String,
    /// The physical address of this administrative body.
    #[serde(rename="physicalAddress")]
    pub physical_address: SimpleAddressType,
}

impl Part for AdministrativeBody {}


/// A social media or web channel for a candidate.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Deserialize)]
pub struct Channel {
    /// The type of channel. The following is a list of types of channels, but is not exhaustive. More channel types may be added at a later time. One of: GooglePlus, YouTube, Facebook, Twitter
    #[serde(rename="type")]
    pub type_: String,
    /// The unique public identifier for the candidate's channel.
    pub id: String,
}

impl Part for Channel {}


/// Information about an Office held by one or more Officials.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Deserialize)]
pub struct Office {
    /// A list of sources for this office. If multiple sources are listed, the data has been aggregated from those sources.
    pub sources: Vec<Source>,
    /// The levels of government of which this office is part. There may be more than one in cases where a jurisdiction effectively acts at two different levels of government; for example, the mayor of the District of Columbia acts at "locality" level, but also effectively at both "administrative-area-2" and "administrative-area-1".
    pub levels: Vec<String>,
    /// The human-readable name of the office.
    pub name: String,
    /// The roles which this office fulfills. Roles are not meant to be exhaustive, or to exactly specify the entire set of responsibilities of a given office, but are meant to be rough categories that are useful for general selection from or sorting of a list of offices.
    pub roles: Vec<String>,
    /// The OCD ID of the division with which this office is associated.
    #[serde(rename="divisionId")]
    pub division_id: String,
    /// List of indices in the officials array of people who presently hold this office.
    #[serde(rename="officialIndices")]
    pub official_indices: Vec<u32>,
}

impl Part for Office {}


/// Represents a political geographic division that matches the requested query.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Deserialize)]
pub struct DivisionSearchResult {
    /// The unique Open Civic Data identifier for this division.
    #[serde(rename="ocdId")]
    pub ocd_id: String,
    /// The name of the division.
    pub name: String,
    /// Other Open Civic Data identifiers that refer to the same division -- for example, those that refer to other political divisions whose boundaries are defined to be coterminous with this one. For example, ocd-division/country:us/state:wy will include an alias of ocd-division/country:us/state:wy/cd:1, since Wyoming has only one Congressional district.
    pub aliases: Vec<String>,
}

impl Part for DivisionSearchResult {}


/// A simple representation of an address.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Deserialize)]
pub struct SimpleAddressType {
    /// The name of the location.
    #[serde(rename="locationName")]
    pub location_name: String,
    /// The city or town for the address.
    pub city: String,
    /// The US two letter state abbreviation of the address.
    pub state: String,
    /// The US Postal Zip Code of the address.
    pub zip: String,
    /// The third line of the address, if needed.
    pub line3: String,
    /// The second line the address, if needed.
    pub line2: String,
    /// The street name and number of this address.
    pub line1: String,
}

impl Part for SimpleAddressType {}


/// Describes a political geography.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Deserialize)]
pub struct GeographicDivision {
    /// List of indices in the offices array, one for each office elected from this division. Will only be present if includeOffices was true (or absent) in the request.
    #[serde(rename="officeIndices")]
    pub office_indices: Vec<u32>,
    /// Any other valid OCD IDs that refer to the same division.
    /// 
    /// Because OCD IDs are meant to be human-readable and at least somewhat predictable, there are occasionally several identifiers for a single division. These identifiers are defined to be equivalent to one another, and one is always indicated as the primary identifier. The primary identifier will be returned in ocd_id above, and any other equivalent valid identifiers will be returned in this list.
    /// 
    /// For example, if this division's OCD ID is ocd-division/country:us/district:dc, this will contain ocd-division/country:us/state:dc.
    #[serde(rename="alsoKnownAs")]
    pub also_known_as: Vec<String>,
    /// The name of the division.
    pub name: String,
}

impl Part for GeographicDivision {}


/// A location where a voter can vote. This may be an early vote site, an election day voting location, or a drop off location for a completed ballot.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Deserialize)]
pub struct PollingLocation {
    /// The first date that this early vote site or drop off location may be used. This field is not populated for polling locations.
    #[serde(rename="startDate")]
    pub start_date: String,
    /// A description of when this location is open.
    #[serde(rename="pollingHours")]
    pub polling_hours: String,
    /// The last date that this early vote site or drop off location may be used. This field is not populated for polling locations.
    #[serde(rename="endDate")]
    pub end_date: String,
    /// The name of the early vote site or drop off location. This field is not populated for polling locations.
    pub name: String,
    /// Notes about this location (e.g. accessibility ramp or entrance to use).
    pub notes: String,
    /// The address of the location.
    pub address: SimpleAddressType,
    /// A list of sources for this location. If multiple sources are listed the data has been aggregated from those sources.
    pub sources: Vec<Source>,
    /// The services provided by this early vote site or drop off location. This field is not populated for polling locations.
    #[serde(rename="voterServices")]
    pub voter_services: String,
    /// An ID for this object. IDs may change in future requests and should not be cached. Access to this field requires special access that can be requested from the Request more link on the Quotas page.
    pub id: String,
}

impl Part for PollingLocation {}


/// There is no detailed description.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [representative info by division representatives](struct.RepresentativeRepresentativeInfoByDivisionCall.html) (response)
/// 
#[derive(Default, Clone, Debug, Deserialize)]
pub struct RepresentativeInfoData {
    /// Political geographic divisions that contain the requested address.
    pub divisions: HashMap<String, GeographicDivision>,
    /// Officials holding the offices listed above. Will only be present if includeOffices was true in the request.
    pub officials: Vec<Official>,
    /// Elected offices referenced by the divisions listed above. Will only be present if includeOffices was true in the request.
    pub offices: Vec<Office>,
}

impl ResponseResult for RepresentativeInfoData {}


/// The result of a voter info lookup query.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [voter info query elections](struct.ElectionVoterInfoQueryCall.html) (response)
/// 
#[derive(Default, Clone, Debug, Deserialize)]
pub struct VoterInfoResponse {
    /// Locations where the voter is eligible to vote early, prior to election day.
    #[serde(rename="earlyVoteSites")]
    pub early_vote_sites: Vec<PollingLocation>,
    /// Locations where a voter is eligible to drop off a completed ballot. The voter must have received and completed a ballot prior to arriving at the location. The location may not have ballots available on the premises. These locations could be open on or before election day as indicated in the pollingHours field.
    #[serde(rename="dropOffLocations")]
    pub drop_off_locations: Vec<PollingLocation>,
    /// If no election ID was specified in the query, and there was more than one election with data for the given voter, this will contain information about the other elections that could apply.
    #[serde(rename="otherElections")]
    pub other_elections: Vec<Election>,
    /// Identifies what kind of resource this is. Value: the fixed string "civicinfo#voterInfoResponse".
    pub kind: String,
    /// The normalized version of the requested address
    #[serde(rename="normalizedInput")]
    pub normalized_input: SimpleAddressType,
    /// Local Election Information for the state that the voter votes in. For the US, there will only be one element in this array.
    pub state: Vec<AdministrationRegion>,
    /// no description provided
    #[serde(rename="precinctId")]
    pub precinct_id: String,
    /// The election that was queried.
    pub election: Election,
    /// Locations where the voter is eligible to vote on election day.
    #[serde(rename="pollingLocations")]
    pub polling_locations: Vec<PollingLocation>,
    /// Contests that will appear on the voter's ballot.
    pub contests: Vec<Contest>,
}

impl ResponseResult for VoterInfoResponse {}


/// Information about the election that was queried.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [voter info query elections](struct.ElectionVoterInfoQueryCall.html) (none)
/// * [election query elections](struct.ElectionElectionQueryCall.html) (none)
/// 
#[derive(Default, Clone, Debug, Deserialize)]
pub struct Election {
    /// Day of the election in YYYY-MM-DD format.
    #[serde(rename="electionDay")]
    pub election_day: Option<String>,
    /// The unique ID of this election.
    pub id: Option<String>,
    /// A displayable name for the election.
    pub name: Option<String>,
}

impl Resource for Election {}



// ###################
// MethodBuilders ###
// #################

/// A builder providing access to all methods supported on *division* resources.
/// It is not used directly, but through the `CivicInfo` hub.
///
/// # Example
///
/// Instantiate a resource builder
///
/// ```test_harness,no_run
/// extern crate hyper;
/// extern crate yup_oauth2 as oauth2;
/// extern crate google_civicinfo2 as civicinfo2;
/// 
/// # #[test] fn egal() {
/// use std::default::Default;
/// use oauth2::{Authenticator, DefaultAuthenticatorDelegate, ApplicationSecret, MemoryStorage};
/// use civicinfo2::CivicInfo;
/// 
/// let secret: ApplicationSecret = Default::default();
/// let auth = Authenticator::new(&secret, DefaultAuthenticatorDelegate,
///                               hyper::Client::new(),
///                               <MemoryStorage as Default>::default(), None);
/// let mut hub = CivicInfo::new(hyper::Client::new(), auth);
/// // Usually you wouldn't bind this to a variable, but keep calling *CallBuilders*
/// // like `search(...)`
/// // to build up your call.
/// let rb = hub.divisions();
/// # }
/// ```
pub struct DivisionMethods<'a, C, A>
    where C: 'a, A: 'a {

    hub: &'a CivicInfo<C, A>,
}

impl<'a, C, A> MethodsBuilder for DivisionMethods<'a, C, A> {}

impl<'a, C, A> DivisionMethods<'a, C, A> {
    
    /// Create a builder to help you perform the following task:
    ///
    /// Searches for political divisions by their natural name or OCD ID.
    pub fn search(&self) -> DivisionSearchCall<'a, C, A> {
        DivisionSearchCall {
            hub: self.hub,
            _query: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
        }
    }
}



/// A builder providing access to all methods supported on *election* resources.
/// It is not used directly, but through the `CivicInfo` hub.
///
/// # Example
///
/// Instantiate a resource builder
///
/// ```test_harness,no_run
/// extern crate hyper;
/// extern crate yup_oauth2 as oauth2;
/// extern crate google_civicinfo2 as civicinfo2;
/// 
/// # #[test] fn egal() {
/// use std::default::Default;
/// use oauth2::{Authenticator, DefaultAuthenticatorDelegate, ApplicationSecret, MemoryStorage};
/// use civicinfo2::CivicInfo;
/// 
/// let secret: ApplicationSecret = Default::default();
/// let auth = Authenticator::new(&secret, DefaultAuthenticatorDelegate,
///                               hyper::Client::new(),
///                               <MemoryStorage as Default>::default(), None);
/// let mut hub = CivicInfo::new(hyper::Client::new(), auth);
/// // Usually you wouldn't bind this to a variable, but keep calling *CallBuilders*
/// // like `election_query(...)` and `voter_info_query(...)`
/// // to build up your call.
/// let rb = hub.elections();
/// # }
/// ```
pub struct ElectionMethods<'a, C, A>
    where C: 'a, A: 'a {

    hub: &'a CivicInfo<C, A>,
}

impl<'a, C, A> MethodsBuilder for ElectionMethods<'a, C, A> {}

impl<'a, C, A> ElectionMethods<'a, C, A> {
    
    /// Create a builder to help you perform the following task:
    ///
    /// List of available elections to query.
    pub fn election_query(&self) -> ElectionElectionQueryCall<'a, C, A> {
        ElectionElectionQueryCall {
            hub: self.hub,
            _delegate: Default::default(),
            _additional_params: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Looks up information relevant to a voter based on the voter's registered address.
    /// 
    /// # Arguments
    ///
    /// * `address` - The registered address of the voter to look up.
    pub fn voter_info_query(&self, address: &str) -> ElectionVoterInfoQueryCall<'a, C, A> {
        ElectionVoterInfoQueryCall {
            hub: self.hub,
            _address: address.to_string(),
            _official_only: Default::default(),
            _election_id: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
        }
    }
}



/// A builder providing access to all methods supported on *representative* resources.
/// It is not used directly, but through the `CivicInfo` hub.
///
/// # Example
///
/// Instantiate a resource builder
///
/// ```test_harness,no_run
/// extern crate hyper;
/// extern crate yup_oauth2 as oauth2;
/// extern crate google_civicinfo2 as civicinfo2;
/// 
/// # #[test] fn egal() {
/// use std::default::Default;
/// use oauth2::{Authenticator, DefaultAuthenticatorDelegate, ApplicationSecret, MemoryStorage};
/// use civicinfo2::CivicInfo;
/// 
/// let secret: ApplicationSecret = Default::default();
/// let auth = Authenticator::new(&secret, DefaultAuthenticatorDelegate,
///                               hyper::Client::new(),
///                               <MemoryStorage as Default>::default(), None);
/// let mut hub = CivicInfo::new(hyper::Client::new(), auth);
/// // Usually you wouldn't bind this to a variable, but keep calling *CallBuilders*
/// // like `representative_info_by_address(...)` and `representative_info_by_division(...)`
/// // to build up your call.
/// let rb = hub.representatives();
/// # }
/// ```
pub struct RepresentativeMethods<'a, C, A>
    where C: 'a, A: 'a {

    hub: &'a CivicInfo<C, A>,
}

impl<'a, C, A> MethodsBuilder for RepresentativeMethods<'a, C, A> {}

impl<'a, C, A> RepresentativeMethods<'a, C, A> {
    
    /// Create a builder to help you perform the following task:
    ///
    /// Looks up political geography and representative information for a single address.
    pub fn representative_info_by_address(&self) -> RepresentativeRepresentativeInfoByAddresCall<'a, C, A> {
        RepresentativeRepresentativeInfoByAddresCall {
            hub: self.hub,
            _roles: Default::default(),
            _levels: Default::default(),
            _include_offices: Default::default(),
            _address: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Looks up representative information for a single geographic division.
    /// 
    /// # Arguments
    ///
    /// * `ocdId` - The Open Civic Data division identifier of the division to look up.
    pub fn representative_info_by_division(&self, ocd_id: &str) -> RepresentativeRepresentativeInfoByDivisionCall<'a, C, A> {
        RepresentativeRepresentativeInfoByDivisionCall {
            hub: self.hub,
            _ocd_id: ocd_id.to_string(),
            _roles: Default::default(),
            _recursive: Default::default(),
            _levels: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
        }
    }
}





// ###################
// CallBuilders   ###
// #################

/// Searches for political divisions by their natural name or OCD ID.
///
/// A builder for the *search* method supported by a *division* resource.
/// It is not used directly, but through a `DivisionMethods` instance.
///
/// # Example
///
/// Instantiate a resource method builder
///
/// ```test_harness,no_run
/// # extern crate hyper;
/// # extern crate yup_oauth2 as oauth2;
/// # extern crate google_civicinfo2 as civicinfo2;
/// # #[test] fn egal() {
/// # use std::default::Default;
/// # use oauth2::{Authenticator, DefaultAuthenticatorDelegate, ApplicationSecret, MemoryStorage};
/// # use civicinfo2::CivicInfo;
/// 
/// # let secret: ApplicationSecret = Default::default();
/// # let auth = Authenticator::new(&secret, DefaultAuthenticatorDelegate,
/// #                               hyper::Client::new(),
/// #                               <MemoryStorage as Default>::default(), None);
/// # let mut hub = CivicInfo::new(hyper::Client::new(), auth);
/// // You can configure optional parameters by calling the respective setters at will, and
/// // execute the final call using `doit()`.
/// // Values shown here are possibly random and not representative !
/// let result = hub.divisions().search()
///              .query("justo")
///              .doit();
/// # }
/// ```
pub struct DivisionSearchCall<'a, C, A>
    where C: 'a, A: 'a {

    hub: &'a CivicInfo<C, A>,
    _query: Option<String>,
    _delegate: Option<&'a mut Delegate>,
    _additional_params: HashMap<String, String>,
}

impl<'a, C, A> CallBuilder for DivisionSearchCall<'a, C, A> {}

impl<'a, C, A> DivisionSearchCall<'a, C, A> where C: BorrowMut<hyper::Client>, A: oauth2::GetToken {


    /// Perform the operation you have build so far.
    pub fn doit(mut self) -> Result<(hyper::client::Response, DivisionSearchResponse)> {
        use std::io::{Read, Seek};
        use hyper::header::{ContentType, ContentLength, Authorization, UserAgent, Location};
        let mut dd = DefaultDelegate;
        let mut dlg: &mut Delegate = match self._delegate {
            Some(d) => d,
            None => &mut dd
        };
        dlg.begin(MethodInfo { id: "civicinfo.divisions.search", 
                               http_method: hyper::method::Method::Get });
        let mut params: Vec<(&str, String)> = Vec::with_capacity((3 + self._additional_params.len()));
        if let Some(value) = self._query {
            params.push(("query", value.to_string()));
        }
        for &field in ["alt", "query"].iter() {
            if self._additional_params.contains_key(field) {
                dlg.finished(false);
                return Err(Error::FieldClash(field));
            }
        }
        for (name, value) in self._additional_params.iter() {
            params.push((&name, value.clone()));
        }

        params.push(("alt", "json".to_string()));

        let mut url = "https://www.googleapis.com/civicinfo/v2/divisions".to_string();
        
        let mut key = self.hub.auth.borrow_mut().api_key();
        if key.is_none() {
            key = dlg.api_key();
        }
        match key {
            Some(value) => params.push(("key", value)),
            None => {
                dlg.finished(false);
                return Err(Error::MissingAPIKey)
            }
        }

        
        if params.len() > 0 {
            url.push('?');
            url.push_str(&url::form_urlencoded::serialize(params.iter().map(|t| (t.0, t.1.as_ref()))));
        }



        loop {
            let mut req_result = {
                let mut client = &mut *self.hub.client.borrow_mut();
                let mut req = client.borrow_mut().request(hyper::method::Method::Get, url.as_ref())
                    .header(UserAgent(self.hub._user_agent.clone()));

                dlg.pre_request();
                req.send()
            };

            match req_result {
                Err(err) => {
                    if let oauth2::Retry::After(d) = dlg.http_error(&err) {
                        sleep_ms(d.num_milliseconds() as u32);
                        continue;
                    }
                    dlg.finished(false);
                    return Err(Error::HttpError(err))
                }
                Ok(mut res) => {
                    if !res.status.is_success() {
                        let mut json_err = String::new();
                        res.read_to_string(&mut json_err).unwrap();
                        if let oauth2::Retry::After(d) = dlg.http_failure(&res, json::from_str(&json_err).ok()) {
                            sleep_ms(d.num_milliseconds() as u32);
                            continue;
                        }
                        dlg.finished(false);
                        return Err(Error::Failure(res))
                    }
                    let result_value = {
                        let mut json_response = String::new();
                        res.read_to_string(&mut json_response).unwrap();
                        match json::from_str(&json_response) {
                            Ok(decoded) => (res, decoded),
                            Err(err) => {
                                dlg.response_json_decode_error(&json_response, &err);
                                return Err(Error::JsonDecodeError(err));
                            }
                        }
                    };

                    dlg.finished(true);
                    return Ok(result_value)
                }
            }
        }
    }


    /// Sets the *query* query property to the given value.
    ///
    /// 
    /// The search query. Queries can cover any parts of a OCD ID or a human readable division name. All words given in the query are treated as required patterns. In addition to that, most query operators of the Apache Lucene library are supported. See http://lucene.apache.org/core/2_9_4/queryparsersyntax.html
    pub fn query(mut self, new_value: &str) -> DivisionSearchCall<'a, C, A> {
        self._query = Some(new_value.to_string());
        self
    }
    /// Sets the *delegate* property to the given value.
    ///
    /// 
    /// The delegate implementation is consulted whenever there is an intermediate result, or if something goes wrong
    /// while executing the actual API request.
    /// 
    /// It should be used to handle progress information, and to implement a certain level of resilience.
    pub fn delegate(mut self, new_value: &'a mut Delegate) -> DivisionSearchCall<'a, C, A> {
        self._delegate = Some(new_value);
        self
    }

    /// Set any additional parameter of the query string used in the request.
    /// It should be used to set parameters which are not yet available through their own 
    /// setters.
    ///
    /// Please note that this method must not be used to set any of the known paramters
    /// which have their own setter method. If done anyway, the request will fail.
    /// 
    /// # Additional Parameters
    ///
    /// * *quotaUser* (query-string) - Available to use for quota purposes for server-side applications. Can be any arbitrary string assigned to a user, but should not exceed 40 characters. Overrides userIp if both are provided.
    /// * *oauth_token* (query-string) - OAuth 2.0 token for the current user.
    /// * *key* (query-string) - API key. Your API key identifies your project and provides you with API access, quota, and reports. Required unless you provide an OAuth 2.0 token.
    /// * *prettyPrint* (query-boolean) - Returns response with indentations and line breaks.
    /// * *userIp* (query-string) - IP address of the site where the request originates. Use this if you want to enforce per-user limits.
    /// * *fields* (query-string) - Selector specifying which fields to include in a partial response.
    /// * *alt* (query-string) - Data format for the response.
    pub fn param<T>(mut self, name: T, value: T) -> DivisionSearchCall<'a, C, A>
                                                        where T: AsRef<str> {
        self._additional_params.insert(name.as_ref().to_string(), value.as_ref().to_string());
        self
    }

}


/// List of available elections to query.
///
/// A builder for the *electionQuery* method supported by a *election* resource.
/// It is not used directly, but through a `ElectionMethods` instance.
///
/// # Example
///
/// Instantiate a resource method builder
///
/// ```test_harness,no_run
/// # extern crate hyper;
/// # extern crate yup_oauth2 as oauth2;
/// # extern crate google_civicinfo2 as civicinfo2;
/// # #[test] fn egal() {
/// # use std::default::Default;
/// # use oauth2::{Authenticator, DefaultAuthenticatorDelegate, ApplicationSecret, MemoryStorage};
/// # use civicinfo2::CivicInfo;
/// 
/// # let secret: ApplicationSecret = Default::default();
/// # let auth = Authenticator::new(&secret, DefaultAuthenticatorDelegate,
/// #                               hyper::Client::new(),
/// #                               <MemoryStorage as Default>::default(), None);
/// # let mut hub = CivicInfo::new(hyper::Client::new(), auth);
/// // You can configure optional parameters by calling the respective setters at will, and
/// // execute the final call using `doit()`.
/// // Values shown here are possibly random and not representative !
/// let result = hub.elections().election_query()
///              .doit();
/// # }
/// ```
pub struct ElectionElectionQueryCall<'a, C, A>
    where C: 'a, A: 'a {

    hub: &'a CivicInfo<C, A>,
    _delegate: Option<&'a mut Delegate>,
    _additional_params: HashMap<String, String>,
}

impl<'a, C, A> CallBuilder for ElectionElectionQueryCall<'a, C, A> {}

impl<'a, C, A> ElectionElectionQueryCall<'a, C, A> where C: BorrowMut<hyper::Client>, A: oauth2::GetToken {


    /// Perform the operation you have build so far.
    pub fn doit(mut self) -> Result<(hyper::client::Response, ElectionsQueryResponse)> {
        use std::io::{Read, Seek};
        use hyper::header::{ContentType, ContentLength, Authorization, UserAgent, Location};
        let mut dd = DefaultDelegate;
        let mut dlg: &mut Delegate = match self._delegate {
            Some(d) => d,
            None => &mut dd
        };
        dlg.begin(MethodInfo { id: "civicinfo.elections.electionQuery", 
                               http_method: hyper::method::Method::Get });
        let mut params: Vec<(&str, String)> = Vec::with_capacity((2 + self._additional_params.len()));
        for &field in ["alt"].iter() {
            if self._additional_params.contains_key(field) {
                dlg.finished(false);
                return Err(Error::FieldClash(field));
            }
        }
        for (name, value) in self._additional_params.iter() {
            params.push((&name, value.clone()));
        }

        params.push(("alt", "json".to_string()));

        let mut url = "https://www.googleapis.com/civicinfo/v2/elections".to_string();
        
        let mut key = self.hub.auth.borrow_mut().api_key();
        if key.is_none() {
            key = dlg.api_key();
        }
        match key {
            Some(value) => params.push(("key", value)),
            None => {
                dlg.finished(false);
                return Err(Error::MissingAPIKey)
            }
        }

        
        if params.len() > 0 {
            url.push('?');
            url.push_str(&url::form_urlencoded::serialize(params.iter().map(|t| (t.0, t.1.as_ref()))));
        }



        loop {
            let mut req_result = {
                let mut client = &mut *self.hub.client.borrow_mut();
                let mut req = client.borrow_mut().request(hyper::method::Method::Get, url.as_ref())
                    .header(UserAgent(self.hub._user_agent.clone()));

                dlg.pre_request();
                req.send()
            };

            match req_result {
                Err(err) => {
                    if let oauth2::Retry::After(d) = dlg.http_error(&err) {
                        sleep_ms(d.num_milliseconds() as u32);
                        continue;
                    }
                    dlg.finished(false);
                    return Err(Error::HttpError(err))
                }
                Ok(mut res) => {
                    if !res.status.is_success() {
                        let mut json_err = String::new();
                        res.read_to_string(&mut json_err).unwrap();
                        if let oauth2::Retry::After(d) = dlg.http_failure(&res, json::from_str(&json_err).ok()) {
                            sleep_ms(d.num_milliseconds() as u32);
                            continue;
                        }
                        dlg.finished(false);
                        return Err(Error::Failure(res))
                    }
                    let result_value = {
                        let mut json_response = String::new();
                        res.read_to_string(&mut json_response).unwrap();
                        match json::from_str(&json_response) {
                            Ok(decoded) => (res, decoded),
                            Err(err) => {
                                dlg.response_json_decode_error(&json_response, &err);
                                return Err(Error::JsonDecodeError(err));
                            }
                        }
                    };

                    dlg.finished(true);
                    return Ok(result_value)
                }
            }
        }
    }


    /// Sets the *delegate* property to the given value.
    ///
    /// 
    /// The delegate implementation is consulted whenever there is an intermediate result, or if something goes wrong
    /// while executing the actual API request.
    /// 
    /// It should be used to handle progress information, and to implement a certain level of resilience.
    pub fn delegate(mut self, new_value: &'a mut Delegate) -> ElectionElectionQueryCall<'a, C, A> {
        self._delegate = Some(new_value);
        self
    }

    /// Set any additional parameter of the query string used in the request.
    /// It should be used to set parameters which are not yet available through their own 
    /// setters.
    ///
    /// Please note that this method must not be used to set any of the known paramters
    /// which have their own setter method. If done anyway, the request will fail.
    /// 
    /// # Additional Parameters
    ///
    /// * *quotaUser* (query-string) - Available to use for quota purposes for server-side applications. Can be any arbitrary string assigned to a user, but should not exceed 40 characters. Overrides userIp if both are provided.
    /// * *oauth_token* (query-string) - OAuth 2.0 token for the current user.
    /// * *key* (query-string) - API key. Your API key identifies your project and provides you with API access, quota, and reports. Required unless you provide an OAuth 2.0 token.
    /// * *prettyPrint* (query-boolean) - Returns response with indentations and line breaks.
    /// * *userIp* (query-string) - IP address of the site where the request originates. Use this if you want to enforce per-user limits.
    /// * *fields* (query-string) - Selector specifying which fields to include in a partial response.
    /// * *alt* (query-string) - Data format for the response.
    pub fn param<T>(mut self, name: T, value: T) -> ElectionElectionQueryCall<'a, C, A>
                                                        where T: AsRef<str> {
        self._additional_params.insert(name.as_ref().to_string(), value.as_ref().to_string());
        self
    }

}


/// Looks up information relevant to a voter based on the voter's registered address.
///
/// A builder for the *voterInfoQuery* method supported by a *election* resource.
/// It is not used directly, but through a `ElectionMethods` instance.
///
/// # Example
///
/// Instantiate a resource method builder
///
/// ```test_harness,no_run
/// # extern crate hyper;
/// # extern crate yup_oauth2 as oauth2;
/// # extern crate google_civicinfo2 as civicinfo2;
/// # #[test] fn egal() {
/// # use std::default::Default;
/// # use oauth2::{Authenticator, DefaultAuthenticatorDelegate, ApplicationSecret, MemoryStorage};
/// # use civicinfo2::CivicInfo;
/// 
/// # let secret: ApplicationSecret = Default::default();
/// # let auth = Authenticator::new(&secret, DefaultAuthenticatorDelegate,
/// #                               hyper::Client::new(),
/// #                               <MemoryStorage as Default>::default(), None);
/// # let mut hub = CivicInfo::new(hyper::Client::new(), auth);
/// // You can configure optional parameters by calling the respective setters at will, and
/// // execute the final call using `doit()`.
/// // Values shown here are possibly random and not representative !
/// let result = hub.elections().voter_info_query("address")
///              .official_only(false)
///              .election_id("labore")
///              .doit();
/// # }
/// ```
pub struct ElectionVoterInfoQueryCall<'a, C, A>
    where C: 'a, A: 'a {

    hub: &'a CivicInfo<C, A>,
    _address: String,
    _official_only: Option<bool>,
    _election_id: Option<String>,
    _delegate: Option<&'a mut Delegate>,
    _additional_params: HashMap<String, String>,
}

impl<'a, C, A> CallBuilder for ElectionVoterInfoQueryCall<'a, C, A> {}

impl<'a, C, A> ElectionVoterInfoQueryCall<'a, C, A> where C: BorrowMut<hyper::Client>, A: oauth2::GetToken {


    /// Perform the operation you have build so far.
    pub fn doit(mut self) -> Result<(hyper::client::Response, VoterInfoResponse)> {
        use std::io::{Read, Seek};
        use hyper::header::{ContentType, ContentLength, Authorization, UserAgent, Location};
        let mut dd = DefaultDelegate;
        let mut dlg: &mut Delegate = match self._delegate {
            Some(d) => d,
            None => &mut dd
        };
        dlg.begin(MethodInfo { id: "civicinfo.elections.voterInfoQuery", 
                               http_method: hyper::method::Method::Get });
        let mut params: Vec<(&str, String)> = Vec::with_capacity((5 + self._additional_params.len()));
        params.push(("address", self._address.to_string()));
        if let Some(value) = self._official_only {
            params.push(("officialOnly", value.to_string()));
        }
        if let Some(value) = self._election_id {
            params.push(("electionId", value.to_string()));
        }
        for &field in ["alt", "address", "officialOnly", "electionId"].iter() {
            if self._additional_params.contains_key(field) {
                dlg.finished(false);
                return Err(Error::FieldClash(field));
            }
        }
        for (name, value) in self._additional_params.iter() {
            params.push((&name, value.clone()));
        }

        params.push(("alt", "json".to_string()));

        let mut url = "https://www.googleapis.com/civicinfo/v2/voterinfo".to_string();
        
        let mut key = self.hub.auth.borrow_mut().api_key();
        if key.is_none() {
            key = dlg.api_key();
        }
        match key {
            Some(value) => params.push(("key", value)),
            None => {
                dlg.finished(false);
                return Err(Error::MissingAPIKey)
            }
        }

        
        if params.len() > 0 {
            url.push('?');
            url.push_str(&url::form_urlencoded::serialize(params.iter().map(|t| (t.0, t.1.as_ref()))));
        }



        loop {
            let mut req_result = {
                let mut client = &mut *self.hub.client.borrow_mut();
                let mut req = client.borrow_mut().request(hyper::method::Method::Get, url.as_ref())
                    .header(UserAgent(self.hub._user_agent.clone()));

                dlg.pre_request();
                req.send()
            };

            match req_result {
                Err(err) => {
                    if let oauth2::Retry::After(d) = dlg.http_error(&err) {
                        sleep_ms(d.num_milliseconds() as u32);
                        continue;
                    }
                    dlg.finished(false);
                    return Err(Error::HttpError(err))
                }
                Ok(mut res) => {
                    if !res.status.is_success() {
                        let mut json_err = String::new();
                        res.read_to_string(&mut json_err).unwrap();
                        if let oauth2::Retry::After(d) = dlg.http_failure(&res, json::from_str(&json_err).ok()) {
                            sleep_ms(d.num_milliseconds() as u32);
                            continue;
                        }
                        dlg.finished(false);
                        return Err(Error::Failure(res))
                    }
                    let result_value = {
                        let mut json_response = String::new();
                        res.read_to_string(&mut json_response).unwrap();
                        match json::from_str(&json_response) {
                            Ok(decoded) => (res, decoded),
                            Err(err) => {
                                dlg.response_json_decode_error(&json_response, &err);
                                return Err(Error::JsonDecodeError(err));
                            }
                        }
                    };

                    dlg.finished(true);
                    return Ok(result_value)
                }
            }
        }
    }


    /// Sets the *address* query property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call, 
    /// we provide this method for API completeness.
    /// 
    /// The registered address of the voter to look up.
    pub fn address(mut self, new_value: &str) -> ElectionVoterInfoQueryCall<'a, C, A> {
        self._address = new_value.to_string();
        self
    }
    /// Sets the *official only* query property to the given value.
    ///
    /// 
    /// If set to true, only data from official state sources will be returned.
    pub fn official_only(mut self, new_value: bool) -> ElectionVoterInfoQueryCall<'a, C, A> {
        self._official_only = Some(new_value);
        self
    }
    /// Sets the *election id* query property to the given value.
    ///
    /// 
    /// The unique ID of the election to look up. A list of election IDs can be obtained at https://www.googleapis.com/civicinfo/{version}/elections
    pub fn election_id(mut self, new_value: &str) -> ElectionVoterInfoQueryCall<'a, C, A> {
        self._election_id = Some(new_value.to_string());
        self
    }
    /// Sets the *delegate* property to the given value.
    ///
    /// 
    /// The delegate implementation is consulted whenever there is an intermediate result, or if something goes wrong
    /// while executing the actual API request.
    /// 
    /// It should be used to handle progress information, and to implement a certain level of resilience.
    pub fn delegate(mut self, new_value: &'a mut Delegate) -> ElectionVoterInfoQueryCall<'a, C, A> {
        self._delegate = Some(new_value);
        self
    }

    /// Set any additional parameter of the query string used in the request.
    /// It should be used to set parameters which are not yet available through their own 
    /// setters.
    ///
    /// Please note that this method must not be used to set any of the known paramters
    /// which have their own setter method. If done anyway, the request will fail.
    /// 
    /// # Additional Parameters
    ///
    /// * *quotaUser* (query-string) - Available to use for quota purposes for server-side applications. Can be any arbitrary string assigned to a user, but should not exceed 40 characters. Overrides userIp if both are provided.
    /// * *oauth_token* (query-string) - OAuth 2.0 token for the current user.
    /// * *key* (query-string) - API key. Your API key identifies your project and provides you with API access, quota, and reports. Required unless you provide an OAuth 2.0 token.
    /// * *prettyPrint* (query-boolean) - Returns response with indentations and line breaks.
    /// * *userIp* (query-string) - IP address of the site where the request originates. Use this if you want to enforce per-user limits.
    /// * *fields* (query-string) - Selector specifying which fields to include in a partial response.
    /// * *alt* (query-string) - Data format for the response.
    pub fn param<T>(mut self, name: T, value: T) -> ElectionVoterInfoQueryCall<'a, C, A>
                                                        where T: AsRef<str> {
        self._additional_params.insert(name.as_ref().to_string(), value.as_ref().to_string());
        self
    }

}


/// Looks up political geography and representative information for a single address.
///
/// A builder for the *representativeInfoByAddress* method supported by a *representative* resource.
/// It is not used directly, but through a `RepresentativeMethods` instance.
///
/// # Example
///
/// Instantiate a resource method builder
///
/// ```test_harness,no_run
/// # extern crate hyper;
/// # extern crate yup_oauth2 as oauth2;
/// # extern crate google_civicinfo2 as civicinfo2;
/// # #[test] fn egal() {
/// # use std::default::Default;
/// # use oauth2::{Authenticator, DefaultAuthenticatorDelegate, ApplicationSecret, MemoryStorage};
/// # use civicinfo2::CivicInfo;
/// 
/// # let secret: ApplicationSecret = Default::default();
/// # let auth = Authenticator::new(&secret, DefaultAuthenticatorDelegate,
/// #                               hyper::Client::new(),
/// #                               <MemoryStorage as Default>::default(), None);
/// # let mut hub = CivicInfo::new(hyper::Client::new(), auth);
/// // You can configure optional parameters by calling the respective setters at will, and
/// // execute the final call using `doit()`.
/// // Values shown here are possibly random and not representative !
/// let result = hub.representatives().representative_info_by_address()
///              .add_roles("sea")
///              .add_levels("nonumy")
///              .include_offices(true)
///              .address("gubergren")
///              .doit();
/// # }
/// ```
pub struct RepresentativeRepresentativeInfoByAddresCall<'a, C, A>
    where C: 'a, A: 'a {

    hub: &'a CivicInfo<C, A>,
    _roles: Vec<String>,
    _levels: Vec<String>,
    _include_offices: Option<bool>,
    _address: Option<String>,
    _delegate: Option<&'a mut Delegate>,
    _additional_params: HashMap<String, String>,
}

impl<'a, C, A> CallBuilder for RepresentativeRepresentativeInfoByAddresCall<'a, C, A> {}

impl<'a, C, A> RepresentativeRepresentativeInfoByAddresCall<'a, C, A> where C: BorrowMut<hyper::Client>, A: oauth2::GetToken {


    /// Perform the operation you have build so far.
    pub fn doit(mut self) -> Result<(hyper::client::Response, RepresentativeInfoResponse)> {
        use std::io::{Read, Seek};
        use hyper::header::{ContentType, ContentLength, Authorization, UserAgent, Location};
        let mut dd = DefaultDelegate;
        let mut dlg: &mut Delegate = match self._delegate {
            Some(d) => d,
            None => &mut dd
        };
        dlg.begin(MethodInfo { id: "civicinfo.representatives.representativeInfoByAddress", 
                               http_method: hyper::method::Method::Get });
        let mut params: Vec<(&str, String)> = Vec::with_capacity((6 + self._additional_params.len()));
        if self._roles.len() > 0 {
            let mut s = String::new();
            for f in self._roles.iter() {
                s.push_str(&("/".to_string() + &f.to_string()));
            }
            params.push(("roles", s));
        }
        if self._levels.len() > 0 {
            let mut s = String::new();
            for f in self._levels.iter() {
                s.push_str(&("/".to_string() + &f.to_string()));
            }
            params.push(("levels", s));
        }
        if let Some(value) = self._include_offices {
            params.push(("includeOffices", value.to_string()));
        }
        if let Some(value) = self._address {
            params.push(("address", value.to_string()));
        }
        for &field in ["alt", "roles", "levels", "includeOffices", "address"].iter() {
            if self._additional_params.contains_key(field) {
                dlg.finished(false);
                return Err(Error::FieldClash(field));
            }
        }
        for (name, value) in self._additional_params.iter() {
            params.push((&name, value.clone()));
        }

        params.push(("alt", "json".to_string()));

        let mut url = "https://www.googleapis.com/civicinfo/v2/representatives".to_string();
        
        let mut key = self.hub.auth.borrow_mut().api_key();
        if key.is_none() {
            key = dlg.api_key();
        }
        match key {
            Some(value) => params.push(("key", value)),
            None => {
                dlg.finished(false);
                return Err(Error::MissingAPIKey)
            }
        }

        
        if params.len() > 0 {
            url.push('?');
            url.push_str(&url::form_urlencoded::serialize(params.iter().map(|t| (t.0, t.1.as_ref()))));
        }



        loop {
            let mut req_result = {
                let mut client = &mut *self.hub.client.borrow_mut();
                let mut req = client.borrow_mut().request(hyper::method::Method::Get, url.as_ref())
                    .header(UserAgent(self.hub._user_agent.clone()));

                dlg.pre_request();
                req.send()
            };

            match req_result {
                Err(err) => {
                    if let oauth2::Retry::After(d) = dlg.http_error(&err) {
                        sleep_ms(d.num_milliseconds() as u32);
                        continue;
                    }
                    dlg.finished(false);
                    return Err(Error::HttpError(err))
                }
                Ok(mut res) => {
                    if !res.status.is_success() {
                        let mut json_err = String::new();
                        res.read_to_string(&mut json_err).unwrap();
                        if let oauth2::Retry::After(d) = dlg.http_failure(&res, json::from_str(&json_err).ok()) {
                            sleep_ms(d.num_milliseconds() as u32);
                            continue;
                        }
                        dlg.finished(false);
                        return Err(Error::Failure(res))
                    }
                    let result_value = {
                        let mut json_response = String::new();
                        res.read_to_string(&mut json_response).unwrap();
                        match json::from_str(&json_response) {
                            Ok(decoded) => (res, decoded),
                            Err(err) => {
                                dlg.response_json_decode_error(&json_response, &err);
                                return Err(Error::JsonDecodeError(err));
                            }
                        }
                    };

                    dlg.finished(true);
                    return Ok(result_value)
                }
            }
        }
    }


    /// Append the given value to the *roles* query property.
    /// Each appended value will retain its original ordering and be '/'-separated in the URL's parameters.
    ///
    /// 
    /// A list of office roles to filter by. Only offices fulfilling one of these roles will be returned. Divisions that don't contain a matching office will not be returned.
    pub fn add_roles(mut self, new_value: &str) -> RepresentativeRepresentativeInfoByAddresCall<'a, C, A> {
        self._roles.push(new_value.to_string());
        self
    }
    /// Append the given value to the *levels* query property.
    /// Each appended value will retain its original ordering and be '/'-separated in the URL's parameters.
    ///
    /// 
    /// A list of office levels to filter by. Only offices that serve at least one of these levels will be returned. Divisions that don't contain a matching office will not be returned.
    pub fn add_levels(mut self, new_value: &str) -> RepresentativeRepresentativeInfoByAddresCall<'a, C, A> {
        self._levels.push(new_value.to_string());
        self
    }
    /// Sets the *include offices* query property to the given value.
    ///
    /// 
    /// Whether to return information about offices and officials. If false, only the top-level district information will be returned.
    pub fn include_offices(mut self, new_value: bool) -> RepresentativeRepresentativeInfoByAddresCall<'a, C, A> {
        self._include_offices = Some(new_value);
        self
    }
    /// Sets the *address* query property to the given value.
    ///
    /// 
    /// The address to look up. May only be specified if the field ocdId is not given in the URL.
    pub fn address(mut self, new_value: &str) -> RepresentativeRepresentativeInfoByAddresCall<'a, C, A> {
        self._address = Some(new_value.to_string());
        self
    }
    /// Sets the *delegate* property to the given value.
    ///
    /// 
    /// The delegate implementation is consulted whenever there is an intermediate result, or if something goes wrong
    /// while executing the actual API request.
    /// 
    /// It should be used to handle progress information, and to implement a certain level of resilience.
    pub fn delegate(mut self, new_value: &'a mut Delegate) -> RepresentativeRepresentativeInfoByAddresCall<'a, C, A> {
        self._delegate = Some(new_value);
        self
    }

    /// Set any additional parameter of the query string used in the request.
    /// It should be used to set parameters which are not yet available through their own 
    /// setters.
    ///
    /// Please note that this method must not be used to set any of the known paramters
    /// which have their own setter method. If done anyway, the request will fail.
    /// 
    /// # Additional Parameters
    ///
    /// * *quotaUser* (query-string) - Available to use for quota purposes for server-side applications. Can be any arbitrary string assigned to a user, but should not exceed 40 characters. Overrides userIp if both are provided.
    /// * *oauth_token* (query-string) - OAuth 2.0 token for the current user.
    /// * *key* (query-string) - API key. Your API key identifies your project and provides you with API access, quota, and reports. Required unless you provide an OAuth 2.0 token.
    /// * *prettyPrint* (query-boolean) - Returns response with indentations and line breaks.
    /// * *userIp* (query-string) - IP address of the site where the request originates. Use this if you want to enforce per-user limits.
    /// * *fields* (query-string) - Selector specifying which fields to include in a partial response.
    /// * *alt* (query-string) - Data format for the response.
    pub fn param<T>(mut self, name: T, value: T) -> RepresentativeRepresentativeInfoByAddresCall<'a, C, A>
                                                        where T: AsRef<str> {
        self._additional_params.insert(name.as_ref().to_string(), value.as_ref().to_string());
        self
    }

}


/// Looks up representative information for a single geographic division.
///
/// A builder for the *representativeInfoByDivision* method supported by a *representative* resource.
/// It is not used directly, but through a `RepresentativeMethods` instance.
///
/// # Example
///
/// Instantiate a resource method builder
///
/// ```test_harness,no_run
/// # extern crate hyper;
/// # extern crate yup_oauth2 as oauth2;
/// # extern crate google_civicinfo2 as civicinfo2;
/// # #[test] fn egal() {
/// # use std::default::Default;
/// # use oauth2::{Authenticator, DefaultAuthenticatorDelegate, ApplicationSecret, MemoryStorage};
/// # use civicinfo2::CivicInfo;
/// 
/// # let secret: ApplicationSecret = Default::default();
/// # let auth = Authenticator::new(&secret, DefaultAuthenticatorDelegate,
/// #                               hyper::Client::new(),
/// #                               <MemoryStorage as Default>::default(), None);
/// # let mut hub = CivicInfo::new(hyper::Client::new(), auth);
/// // You can configure optional parameters by calling the respective setters at will, and
/// // execute the final call using `doit()`.
/// // Values shown here are possibly random and not representative !
/// let result = hub.representatives().representative_info_by_division("ocdId")
///              .add_roles("aliquyam")
///              .recursive(false)
///              .add_levels("no")
///              .doit();
/// # }
/// ```
pub struct RepresentativeRepresentativeInfoByDivisionCall<'a, C, A>
    where C: 'a, A: 'a {

    hub: &'a CivicInfo<C, A>,
    _ocd_id: String,
    _roles: Vec<String>,
    _recursive: Option<bool>,
    _levels: Vec<String>,
    _delegate: Option<&'a mut Delegate>,
    _additional_params: HashMap<String, String>,
}

impl<'a, C, A> CallBuilder for RepresentativeRepresentativeInfoByDivisionCall<'a, C, A> {}

impl<'a, C, A> RepresentativeRepresentativeInfoByDivisionCall<'a, C, A> where C: BorrowMut<hyper::Client>, A: oauth2::GetToken {


    /// Perform the operation you have build so far.
    pub fn doit(mut self) -> Result<(hyper::client::Response, RepresentativeInfoData)> {
        use std::io::{Read, Seek};
        use hyper::header::{ContentType, ContentLength, Authorization, UserAgent, Location};
        let mut dd = DefaultDelegate;
        let mut dlg: &mut Delegate = match self._delegate {
            Some(d) => d,
            None => &mut dd
        };
        dlg.begin(MethodInfo { id: "civicinfo.representatives.representativeInfoByDivision", 
                               http_method: hyper::method::Method::Get });
        let mut params: Vec<(&str, String)> = Vec::with_capacity((6 + self._additional_params.len()));
        params.push(("ocdId", self._ocd_id.to_string()));
        if self._roles.len() > 0 {
            let mut s = String::new();
            for f in self._roles.iter() {
                s.push_str(&("/".to_string() + &f.to_string()));
            }
            params.push(("roles", s));
        }
        if let Some(value) = self._recursive {
            params.push(("recursive", value.to_string()));
        }
        if self._levels.len() > 0 {
            let mut s = String::new();
            for f in self._levels.iter() {
                s.push_str(&("/".to_string() + &f.to_string()));
            }
            params.push(("levels", s));
        }
        for &field in ["alt", "ocdId", "roles", "recursive", "levels"].iter() {
            if self._additional_params.contains_key(field) {
                dlg.finished(false);
                return Err(Error::FieldClash(field));
            }
        }
        for (name, value) in self._additional_params.iter() {
            params.push((&name, value.clone()));
        }

        params.push(("alt", "json".to_string()));

        let mut url = "https://www.googleapis.com/civicinfo/v2/representatives/{ocdId}".to_string();
        
        let mut key = self.hub.auth.borrow_mut().api_key();
        if key.is_none() {
            key = dlg.api_key();
        }
        match key {
            Some(value) => params.push(("key", value)),
            None => {
                dlg.finished(false);
                return Err(Error::MissingAPIKey)
            }
        }

        for &(find_this, param_name) in [("{ocdId}", "ocdId")].iter() {
                        let mut replace_with: Option<&str> = None;
            for &(name, ref value) in params.iter() {
                if name == param_name {
                    replace_with = Some(value);
                    break;
                }
            }
            url = url.replace(find_this, replace_with.expect("to find substitution value in params"));
        }
        {
            let mut indices_for_removal: Vec<usize> = Vec::with_capacity(1);
            for param_name in ["ocdId"].iter() {
                for (index, &(ref name, _)) in params.iter().rev().enumerate() {
                    if name == param_name {
                        indices_for_removal.push(params.len() - index - 1);
                        break;
                    }
                }
            }
            for &index in indices_for_removal.iter() {
                params.remove(index);
            }
        }
        
        if params.len() > 0 {
            url.push('?');
            url.push_str(&url::form_urlencoded::serialize(params.iter().map(|t| (t.0, t.1.as_ref()))));
        }



        loop {
            let mut req_result = {
                let mut client = &mut *self.hub.client.borrow_mut();
                let mut req = client.borrow_mut().request(hyper::method::Method::Get, url.as_ref())
                    .header(UserAgent(self.hub._user_agent.clone()));

                dlg.pre_request();
                req.send()
            };

            match req_result {
                Err(err) => {
                    if let oauth2::Retry::After(d) = dlg.http_error(&err) {
                        sleep_ms(d.num_milliseconds() as u32);
                        continue;
                    }
                    dlg.finished(false);
                    return Err(Error::HttpError(err))
                }
                Ok(mut res) => {
                    if !res.status.is_success() {
                        let mut json_err = String::new();
                        res.read_to_string(&mut json_err).unwrap();
                        if let oauth2::Retry::After(d) = dlg.http_failure(&res, json::from_str(&json_err).ok()) {
                            sleep_ms(d.num_milliseconds() as u32);
                            continue;
                        }
                        dlg.finished(false);
                        return Err(Error::Failure(res))
                    }
                    let result_value = {
                        let mut json_response = String::new();
                        res.read_to_string(&mut json_response).unwrap();
                        match json::from_str(&json_response) {
                            Ok(decoded) => (res, decoded),
                            Err(err) => {
                                dlg.response_json_decode_error(&json_response, &err);
                                return Err(Error::JsonDecodeError(err));
                            }
                        }
                    };

                    dlg.finished(true);
                    return Ok(result_value)
                }
            }
        }
    }


    /// Sets the *ocd id* path property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call, 
    /// we provide this method for API completeness.
    /// 
    /// The Open Civic Data division identifier of the division to look up.
    pub fn ocd_id(mut self, new_value: &str) -> RepresentativeRepresentativeInfoByDivisionCall<'a, C, A> {
        self._ocd_id = new_value.to_string();
        self
    }
    /// Append the given value to the *roles* query property.
    /// Each appended value will retain its original ordering and be '/'-separated in the URL's parameters.
    ///
    /// 
    /// A list of office roles to filter by. Only offices fulfilling one of these roles will be returned. Divisions that don't contain a matching office will not be returned.
    pub fn add_roles(mut self, new_value: &str) -> RepresentativeRepresentativeInfoByDivisionCall<'a, C, A> {
        self._roles.push(new_value.to_string());
        self
    }
    /// Sets the *recursive* query property to the given value.
    ///
    /// 
    /// If true, information about all divisions contained in the division requested will be included as well. For example, if querying ocd-division/country:us/district:dc, this would also return all DC's wards and ANCs.
    pub fn recursive(mut self, new_value: bool) -> RepresentativeRepresentativeInfoByDivisionCall<'a, C, A> {
        self._recursive = Some(new_value);
        self
    }
    /// Append the given value to the *levels* query property.
    /// Each appended value will retain its original ordering and be '/'-separated in the URL's parameters.
    ///
    /// 
    /// A list of office levels to filter by. Only offices that serve at least one of these levels will be returned. Divisions that don't contain a matching office will not be returned.
    pub fn add_levels(mut self, new_value: &str) -> RepresentativeRepresentativeInfoByDivisionCall<'a, C, A> {
        self._levels.push(new_value.to_string());
        self
    }
    /// Sets the *delegate* property to the given value.
    ///
    /// 
    /// The delegate implementation is consulted whenever there is an intermediate result, or if something goes wrong
    /// while executing the actual API request.
    /// 
    /// It should be used to handle progress information, and to implement a certain level of resilience.
    pub fn delegate(mut self, new_value: &'a mut Delegate) -> RepresentativeRepresentativeInfoByDivisionCall<'a, C, A> {
        self._delegate = Some(new_value);
        self
    }

    /// Set any additional parameter of the query string used in the request.
    /// It should be used to set parameters which are not yet available through their own 
    /// setters.
    ///
    /// Please note that this method must not be used to set any of the known paramters
    /// which have their own setter method. If done anyway, the request will fail.
    /// 
    /// # Additional Parameters
    ///
    /// * *quotaUser* (query-string) - Available to use for quota purposes for server-side applications. Can be any arbitrary string assigned to a user, but should not exceed 40 characters. Overrides userIp if both are provided.
    /// * *oauth_token* (query-string) - OAuth 2.0 token for the current user.
    /// * *key* (query-string) - API key. Your API key identifies your project and provides you with API access, quota, and reports. Required unless you provide an OAuth 2.0 token.
    /// * *prettyPrint* (query-boolean) - Returns response with indentations and line breaks.
    /// * *userIp* (query-string) - IP address of the site where the request originates. Use this if you want to enforce per-user limits.
    /// * *fields* (query-string) - Selector specifying which fields to include in a partial response.
    /// * *alt* (query-string) - Data format for the response.
    pub fn param<T>(mut self, name: T, value: T) -> RepresentativeRepresentativeInfoByDivisionCall<'a, C, A>
                                                        where T: AsRef<str> {
        self._additional_params.insert(name.as_ref().to_string(), value.as_ref().to_string());
        self
    }

}


