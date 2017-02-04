// DO NOT EDIT !
// This file was generated automatically from 'src/mako/api/lib.rs.mako'
// DO NOT EDIT !

//! This documentation was generated from *partners* crate version *1.0.4+20151009*, where *20151009* is the exact revision of the *partners:v2* schema built by the [mako](http://www.makotemplates.org/) code generator *v1.0.4*.
//! 
//! Everything else about the *partners* *v2* API can be found at the
//! [official documentation site](https://developers.google.com/partners/).
//! The original source code is [on github](https://github.com/Byron/google-apis-rs/tree/master/gen/partners2).
//! # Features
//! 
//! Handle the following *Resources* with ease from the central [hub](struct.Partners.html) ... 
//! 
//! * client messages
//!  * [*log*](struct.ClientMessageLogCall.html)
//! * [companies](struct.Company.html)
//!  * [*get*](struct.CompanyGetCall.html), [*leads create*](struct.CompanyLeadCreateCall.html) and [*list*](struct.CompanyListCall.html)
//! * user events
//!  * [*log*](struct.UserEventLogCall.html)
//! * user states
//!  * [*list*](struct.UserStateListCall.html)
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
//! * **[Hub](struct.Partners.html)**
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
//! let r = hub.user_events().log(...).doit()
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
//! google-partners2 = "*"
//! ```
//! 
//! ## A complete example
//! 
//! ```test_harness,no_run
//! extern crate hyper;
//! extern crate yup_oauth2 as oauth2;
//! extern crate google_partners2 as partners2;
//! use partners2::LogUserEventRequest;
//! use partners2::{Result, Error};
//! # #[test] fn egal() {
//! use std::default::Default;
//! use oauth2::{Authenticator, DefaultAuthenticatorDelegate, ApplicationSecret, MemoryStorage};
//! use partners2::Partners;
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
//! let mut hub = Partners::new(hyper::Client::new(), auth);
//! // As the method needs a request, you would usually fill it with the desired information
//! // into the respective structure. Some of the parts shown here might not be applicable !
//! // Values shown here are possibly random and not representative !
//! let mut req = LogUserEventRequest::default();
//! 
//! // You can configure optional parameters by calling the respective setters at will, and
//! // execute the final call using `doit()`.
//! // Values shown here are possibly random and not representative !
//! let result = hub.user_events().log(req)
//!              .doit();
//! 
//! match result {
//!     Err(e) => match e {
//!         // The Error enum provides details about what exactly happened.
//!         // You can also just use its `Debug`, `Display` or `Error` traits
//!          Error::HttpError(_)
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
//! All errors produced by the system are provided either as [Result](enum.Result.html) enumeration as return value of 
//! the doit() methods, or handed as possibly intermediate results to either the 
//! [Hub Delegate](trait.Delegate.html), or the [Authenticator Delegate](https://docs.rs/yup-oauth2/*/yup_oauth2/trait.AuthenticatorDelegate.html).
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
//! * [request values](trait.RequestValue.html) are moved
//! 
//! Arguments will always be copied or cloned into the builder, to make them independent of their original life times.
//! 
//! [wiki-pod]: http://en.wikipedia.org/wiki/Plain_old_data_structure
//! [builder-pattern]: http://en.wikipedia.org/wiki/Builder_pattern
//! [google-go-api]: https://github.com/google/google-api-go-client
//! 
//! 

// Unused attributes happen thanks to defined, but unused structures
// We don't warn about this, as depending on the API, some data structures or facilities are never used.
// Instead of pre-determining this, we just disable the lint. It's manually tuned to not have any 
// unused imports in fully featured APIs. Same with unused_mut ... .
#![allow(unused_imports, unused_mut, dead_code)]

// DO NOT EDIT !
// This file was generated automatically from 'src/mako/api/lib.rs.mako'
// DO NOT EDIT !

#[macro_use]
extern crate serde_derive;

extern crate hyper;
extern crate serde;
extern crate serde_json;
extern crate yup_oauth2 as oauth2;
extern crate mime;
extern crate url;

mod cmn;

use std::collections::HashMap;
use std::cell::RefCell;
use std::borrow::BorrowMut;
use std::default::Default;
use std::collections::BTreeMap;
use serde_json as json;
use std::io;
use std::fs;
use std::thread::sleep;
use std::time::Duration;

pub use cmn::{MultiPartReader, ToParts, MethodInfo, Result, Error, CallBuilder, Hub, ReadSeek, Part,
              ResponseResult, RequestValue, NestedType, Delegate, DefaultDelegate, MethodsBuilder,
              Resource, ErrorResponse, remove_json_null_values};


// ##############
// UTILITIES ###
// ############




// ########
// HUB ###
// ######

/// Central instance to access all Partners related resource activities
///
/// # Examples
///
/// Instantiate a new hub
///
/// ```test_harness,no_run
/// extern crate hyper;
/// extern crate yup_oauth2 as oauth2;
/// extern crate google_partners2 as partners2;
/// use partners2::LogUserEventRequest;
/// use partners2::{Result, Error};
/// # #[test] fn egal() {
/// use std::default::Default;
/// use oauth2::{Authenticator, DefaultAuthenticatorDelegate, ApplicationSecret, MemoryStorage};
/// use partners2::Partners;
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
/// let mut hub = Partners::new(hyper::Client::new(), auth);
/// // As the method needs a request, you would usually fill it with the desired information
/// // into the respective structure. Some of the parts shown here might not be applicable !
/// // Values shown here are possibly random and not representative !
/// let mut req = LogUserEventRequest::default();
/// 
/// // You can configure optional parameters by calling the respective setters at will, and
/// // execute the final call using `doit()`.
/// // Values shown here are possibly random and not representative !
/// let result = hub.user_events().log(req)
///              .doit();
/// 
/// match result {
///     Err(e) => match e {
///         // The Error enum provides details about what exactly happened.
///         // You can also just use its `Debug`, `Display` or `Error` traits
///          Error::HttpError(_)
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
pub struct Partners<C, A> {
    client: RefCell<C>,
    auth: RefCell<A>,
    _user_agent: String,
}

impl<'a, C, A> Hub for Partners<C, A> {}

impl<'a, C, A> Partners<C, A>
    where  C: BorrowMut<hyper::Client>, A: oauth2::GetToken {

    pub fn new(client: C, authenticator: A) -> Partners<C, A> {
        Partners {
            client: RefCell::new(client),
            auth: RefCell::new(authenticator),
            _user_agent: "google-api-rust-client/1.0.4".to_string(),
        }
    }

    pub fn client_messages(&'a self) -> ClientMessageMethods<'a, C, A> {
        ClientMessageMethods { hub: &self }
    }
    pub fn companies(&'a self) -> CompanyMethods<'a, C, A> {
        CompanyMethods { hub: &self }
    }
    pub fn user_events(&'a self) -> UserEventMethods<'a, C, A> {
        UserEventMethods { hub: &self }
    }
    pub fn user_states(&'a self) -> UserStateMethods<'a, C, A> {
        UserStateMethods { hub: &self }
    }

    /// Set the user-agent header field to use in all requests to the server.
    /// It defaults to `google-api-rust-client/1.0.4`.
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
/// Response message for LogUserEvent.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [log user events](struct.UserEventLogCall.html) (response)
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct LogUserEventResponse {
    /// Current response metadata.
    #[serde(rename="responseMetadata")]
    pub response_metadata: Option<ResponseMetadata>,
}

impl ResponseResult for LogUserEventResponse {}


/// Source of traffic for the current request.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct TrafficSource {
    /// Second level identifier to indicate where the traffic comes from. An identifier has multiple letters created by a team which redirected the traffic to us.
    #[serde(rename="trafficSubId")]
    pub traffic_sub_id: Option<String>,
    /// Identifier to indicate where the traffic comes from. An identifier has multiple letters created by a team which redirected the traffic to us.
    #[serde(rename="trafficSourceId")]
    pub traffic_source_id: Option<String>,
}

impl Part for TrafficSource {}


/// Google Partners certification status.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct CertificationStatus {
    /// The type of the certification.
    #[serde(rename="type")]
    pub type_: Option<String>,
    /// Whether certification is passing.
    #[serde(rename="isCertified")]
    pub is_certified: Option<bool>,
    /// List of certification exam statuses.
    #[serde(rename="examStatuses")]
    pub exam_statuses: Option<Vec<CertificationExamStatus>>,
}

impl Part for CertificationStatus {}


/// Response message for LogClientMessage.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [log client messages](struct.ClientMessageLogCall.html) (response)
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct LogMessageResponse {
    /// Current response metadata.
    #[serde(rename="responseMetadata")]
    pub response_metadata: Option<ResponseMetadata>,
}

impl ResponseResult for LogMessageResponse {}


/// Values to use instead of the user's respective defaults. These are only honored by whitelisted products.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct UserOverrides {
    /// IP address to use instead of the user's geo-located IP address.
    #[serde(rename="ipAddress")]
    pub ip_address: Option<String>,
    /// Logged-in user ID to impersonate instead of the user's ID.
    #[serde(rename="userId")]
    pub user_id: Option<String>,
}

impl Part for UserOverrides {}


/// A lead resource that represents an advertiser contact for a `Company`. These are usually generated via Google Partner Search (the advertiser portal).
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct Lead {
    /// List of reasons for using Google Partner Search and creating a lead.
    #[serde(rename="gpsMotivations")]
    pub gps_motivations: Option<Vec<String>>,
    /// Last name of lead source.
    #[serde(rename="familyName")]
    pub family_name: Option<String>,
    /// The minimum monthly budget lead source is willing to spend.
    #[serde(rename="minMonthlyBudget")]
    pub min_monthly_budget: Option<Money>,
    /// Comments lead source gave.
    pub comments: Option<String>,
    /// ID of the lead.
    pub id: Option<String>,
    /// Website URL of lead source.
    #[serde(rename="websiteUrl")]
    pub website_url: Option<String>,
    /// Phone number of lead source.
    #[serde(rename="phoneNumber")]
    pub phone_number: Option<String>,
    /// First name of lead source.
    #[serde(rename="givenName")]
    pub given_name: Option<String>,
    /// Type of lead.
    #[serde(rename="type")]
    pub type_: Option<String>,
    /// Email address of lead source.
    pub email: Option<String>,
}

impl Part for Lead {}


/// Debug information about this request.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct DebugInfo {
    /// Server-side debug stack trace.
    #[serde(rename="serverTraceInfo")]
    pub server_trace_info: Option<String>,
    /// URL of the service that handled this request.
    #[serde(rename="serviceUrl")]
    pub service_url: Option<String>,
    /// Info about the server that serviced this request.
    #[serde(rename="serverInfo")]
    pub server_info: Option<String>,
}

impl Part for DebugInfo {}


/// Request message for LogUserEvent.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [log user events](struct.UserEventLogCall.html) (request)
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct LogUserEventRequest {
    /// Current request metadata.
    #[serde(rename="requestMetadata")]
    pub request_metadata: Option<RequestMetadata>,
    /// Advertiser lead information.
    pub lead: Option<Lead>,
    /// The URL where the event occurred.
    pub url: Option<String>,
    /// List of event data for the event.
    #[serde(rename="eventDatas")]
    pub event_datas: Option<Vec<EventData>>,
    /// The action that occurred.
    #[serde(rename="eventAction")]
    pub event_action: Option<String>,
    /// The scope of the event.
    #[serde(rename="eventScope")]
    pub event_scope: Option<String>,
    /// The category the action belongs to.
    #[serde(rename="eventCategory")]
    pub event_category: Option<String>,
}

impl RequestValue for LogUserEventRequest {}


/// The localized company information.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct LocalizedCompanyInfo {
    /// List of country codes for the localized company info.
    #[serde(rename="countryCodes")]
    pub country_codes: Option<Vec<String>>,
    /// Language code of the localized company info, as defined by BCP 47 (IETF BCP 47, "Tags for Identifying Languages").
    #[serde(rename="languageCode")]
    pub language_code: Option<String>,
    /// Localized display name.
    #[serde(rename="displayName")]
    pub display_name: Option<String>,
    /// Localized brief description that the company uses to advertise themselves.
    pub overview: Option<String>,
}

impl Part for LocalizedCompanyInfo {}


/// A location with address and geographic coordinates.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct Location {
    /// The latitude and longitude of the location, in degrees.
    #[serde(rename="latLng")]
    pub lat_lng: Option<LatLng>,
    /// The complete address of the location.
    pub address: Option<String>,
}

impl Part for Location {}


/// Response message for ListCompanies.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [list companies](struct.CompanyListCall.html) (response)
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct ListCompaniesResponse {
    /// A token to retrieve next page of results. Pass this value in the `ListCompaniesRequest.page_token` field in the subsequent call to ListCompanies to retrieve the next page of results.
    #[serde(rename="nextPageToken")]
    pub next_page_token: Option<String>,
    /// The list of companies.
    pub companies: Option<Vec<Company>>,
    /// Current response metadata.
    #[serde(rename="responseMetadata")]
    pub response_metadata: Option<ResponseMetadata>,
}

impl ResponseResult for ListCompaniesResponse {}


/// reCaptcha challenge info.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct RecaptchaChallenge {
    /// The ID of the reCaptcha challenge.
    pub id: Option<String>,
    /// The response to the reCaptcha challenge.
    pub response: Option<String>,
}

impl Part for RecaptchaChallenge {}


/// Status for a Google Partners certification exam.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct CertificationExamStatus {
    /// The number of people who have passed the certification exam.
    #[serde(rename="numberUsersPass")]
    pub number_users_pass: Option<i32>,
    /// The type of certification exam.
    #[serde(rename="type")]
    pub type_: Option<String>,
}

impl Part for CertificationExamStatus {}


/// Response message for CreateLead. Debug information about this request.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [leads create companies](struct.CompanyLeadCreateCall.html) (response)
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct CreateLeadResponse {
    /// Lead that was created depending on the outcome of reCaptcha validation.
    pub lead: Option<Lead>,
    /// The outcome of reCaptcha validation.
    #[serde(rename="recaptchaStatus")]
    pub recaptcha_status: Option<String>,
    /// Current response metadata.
    #[serde(rename="responseMetadata")]
    pub response_metadata: Option<ResponseMetadata>,
}

impl ResponseResult for CreateLeadResponse {}


/// Represents an amount of money with its currency type.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct Money {
    /// The whole units of the amount. For example if `currencyCode` is `"USD"`, then 1 unit is one US dollar.
    pub units: Option<String>,
    /// Number of nano (10^-9) units of the amount. The value must be between -999,999,999 and +999,999,999 inclusive. If `units` is positive, `nanos` must be positive or zero. If `units` is zero, `nanos` can be positive, zero, or negative. If `units` is negative, `nanos` must be negative or zero. For example $-1.75 is represented as `units`=-1 and `nanos`=-750,000,000.
    pub nanos: Option<i32>,
    /// The 3-letter currency code defined in ISO 4217.
    #[serde(rename="currencyCode")]
    pub currency_code: Option<String>,
}

impl Part for Money {}


/// A company resource in the Google Partners API. Once certified, it qualifies for being searched by advertisers.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct Company {
    /// Industries the company can help with.
    pub industries: Option<Vec<String>>,
    /// The unconverted minimum monthly budget that the company accepts for partner business.
    #[serde(rename="originalMinMonthlyBudget")]
    pub original_min_monthly_budget: Option<Money>,
    /// The minimum monthly budget that the company accepts for partner business, converted to the requested currency code.
    #[serde(rename="convertedMinMonthlyBudget")]
    pub converted_min_monthly_budget: Option<Money>,
    /// The name of the company.
    pub name: Option<String>,
    /// The list of localized info for the company.
    #[serde(rename="localizedInfos")]
    pub localized_infos: Option<Vec<LocalizedCompanyInfo>>,
    /// The list of company locations.
    pub locations: Option<Vec<Location>>,
    /// URL of the company's website.
    #[serde(rename="websiteUrl")]
    pub website_url: Option<String>,
    /// Information related to the ranking of the company within the list of companies.
    pub ranks: Option<Vec<Rank>>,
    /// Services the company can help with.
    pub services: Option<Vec<String>>,
    /// The list of Google Partners certification statuses for the company.
    #[serde(rename="certificationStatuses")]
    pub certification_statuses: Option<Vec<CertificationStatus>>,
    /// Basic information from the company's public profile.
    #[serde(rename="publicProfile")]
    pub public_profile: Option<PublicProfile>,
    /// The ID of the company.
    pub id: Option<String>,
}

impl Part for Company {}


/// Request message for LogClientMessage.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [log client messages](struct.ClientMessageLogCall.html) (request)
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct LogMessageRequest {
    /// Current request metadata.
    #[serde(rename="requestMetadata")]
    pub request_metadata: Option<RequestMetadata>,
    /// Map of client info, such as URL, browser navigator, browser platform, etc.
    #[serde(rename="clientInfo")]
    pub client_info: Option<HashMap<String, String>>,
    /// Details about the client message.
    pub details: Option<String>,
    /// Message level of client message.
    pub level: Option<String>,
}

impl RequestValue for LogMessageRequest {}


/// Response message for GetCompany.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [get companies](struct.CompanyGetCall.html) (response)
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GetCompanyResponse {
    /// The company.
    pub company: Option<Company>,
    /// Current response metadata.
    #[serde(rename="responseMetadata")]
    pub response_metadata: Option<ResponseMetadata>,
}

impl ResponseResult for GetCompanyResponse {}


/// Information related to ranking of results.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct Rank {
    /// The type of rank.
    #[serde(rename="type")]
    pub type_: Option<String>,
    /// The numerical value of the rank.
    pub value: Option<f64>,
}

impl Part for Rank {}


/// Request message for CreateLead.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [leads create companies](struct.CompanyLeadCreateCall.html) (request)
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct CreateLeadRequest {
    /// Current request metadata.
    #[serde(rename="requestMetadata")]
    pub request_metadata: Option<RequestMetadata>,
    /// reCaptcha challenge info.
    #[serde(rename="recaptchaChallenge")]
    pub recaptcha_challenge: Option<RecaptchaChallenge>,
    /// The lead resource. The `LeadType` must not be `LEAD_TYPE_UNSPECIFIED` and either `email` or `phone_number` must be provided.
    pub lead: Option<Lead>,
}

impl RequestValue for CreateLeadRequest {}


/// Common data that is in each API request.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct RequestMetadata {
    /// Locale to use for the current request.
    pub locale: Option<String>,
    /// Google Partners session ID.
    #[serde(rename="partnersSessionId")]
    pub partners_session_id: Option<String>,
    /// Experiment IDs the current request belongs to.
    #[serde(rename="experimentIds")]
    pub experiment_ids: Option<Vec<String>>,
    /// Values to use instead of the user's respective defaults for the current request. These are only honored by whitelisted products.
    #[serde(rename="userOverrides")]
    pub user_overrides: Option<UserOverrides>,
    /// Source of traffic for the current request.
    #[serde(rename="trafficSource")]
    pub traffic_source: Option<TrafficSource>,
}

impl Part for RequestMetadata {}


/// Response message for ListUserStates.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [list user states](struct.UserStateListCall.html) (response)
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct ListUserStatesResponse {
    /// User's states.
    #[serde(rename="userStates")]
    pub user_states: Option<Vec<String>>,
    /// Current response metadata.
    #[serde(rename="responseMetadata")]
    pub response_metadata: Option<ResponseMetadata>,
}

impl ResponseResult for ListUserStatesResponse {}


/// Common data that is in each API response.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct ResponseMetadata {
    /// Debug information about this request.
    #[serde(rename="debugInfo")]
    pub debug_info: Option<DebugInfo>,
}

impl Part for ResponseMetadata {}


/// Key value data pair for an event.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct EventData {
    /// Data values.
    pub values: Option<Vec<String>>,
    /// Data type.
    pub key: Option<String>,
}

impl Part for EventData {}


/// Basic information from a public profile.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct PublicProfile {
    /// The URL of the public profile.
    pub url: Option<String>,
    /// The URL to the main display image of the public profile.
    #[serde(rename="displayImageUrl")]
    pub display_image_url: Option<String>,
    /// The display name of the public profile.
    #[serde(rename="displayName")]
    pub display_name: Option<String>,
    /// The ID which can be used to retrieve more details about the public profile.
    pub id: Option<String>,
}

impl Part for PublicProfile {}


/// An object representing a latitude/longitude pair. This is expressed as a pair of doubles representing degrees latitude and degrees longitude. Unless specified otherwise, this must conform to the WGS84 standard. Values must be within normalized ranges. Example of normalization code in Python: def NormalizeLongitude(longitude): """Wrapsdecimal degrees longitude to [-180.0, 180.0].""" q, r = divmod(longitude, 360.0) if r > 180.0 or (r == 180.0 and q <= -1.0): return r - 360.0 return r def NormalizeLatLng(latitude, longitude): """Wraps decimal degrees latitude and longitude to [-180.0, 180.0] and [-90.0, 90.0], respectively.""" r = latitude % 360.0 if r = 270.0: return r - 360, NormalizeLongitude(longitude) else: return 180 - r, NormalizeLongitude(longitude + 180.0) assert 180.0 == NormalizeLongitude(180.0) assert -180.0 == NormalizeLongitude(-180.0) assert -179.0 == NormalizeLongitude(181.0) assert (0.0, 0.0) == NormalizeLatLng(360.0, 0.0) assert (0.0, 0.0) == NormalizeLatLng(-360.0, 0.0) assert (85.0, 180.0) == NormalizeLatLng(95.0, 0.0) assert (-85.0, -170.0) == NormalizeLatLng(-95.0, 10.0) assert (90.0, 10.0) == NormalizeLatLng(90.0, 10.0) assert (-90.0, -10.0) == NormalizeLatLng(-90.0, -10.0) assert (0.0, -170.0) == NormalizeLatLng(-180.0, 10.0) assert (0.0, -170.0) == NormalizeLatLng(180.0, 10.0) assert (-90.0, 10.0) == NormalizeLatLng(270.0, 10.0) assert (90.0, 10.0) == NormalizeLatLng(-270.0, 10.0)
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct LatLng {
    /// The latitude in degrees. It must be in the range [-90.0, +90.0].
    pub latitude: Option<f64>,
    /// The longitude in degrees. It must be in the range [-180.0, +180.0].
    pub longitude: Option<f64>,
}

impl Part for LatLng {}



// ###################
// MethodBuilders ###
// #################

/// A builder providing access to all methods supported on *userEvent* resources.
/// It is not used directly, but through the `Partners` hub.
///
/// # Example
///
/// Instantiate a resource builder
///
/// ```test_harness,no_run
/// extern crate hyper;
/// extern crate yup_oauth2 as oauth2;
/// extern crate google_partners2 as partners2;
/// 
/// # #[test] fn egal() {
/// use std::default::Default;
/// use oauth2::{Authenticator, DefaultAuthenticatorDelegate, ApplicationSecret, MemoryStorage};
/// use partners2::Partners;
/// 
/// let secret: ApplicationSecret = Default::default();
/// let auth = Authenticator::new(&secret, DefaultAuthenticatorDelegate,
///                               hyper::Client::new(),
///                               <MemoryStorage as Default>::default(), None);
/// let mut hub = Partners::new(hyper::Client::new(), auth);
/// // Usually you wouldn't bind this to a variable, but keep calling *CallBuilders*
/// // like `log(...)`
/// // to build up your call.
/// let rb = hub.user_events();
/// # }
/// ```
pub struct UserEventMethods<'a, C, A>
    where C: 'a, A: 'a {

    hub: &'a Partners<C, A>,
}

impl<'a, C, A> MethodsBuilder for UserEventMethods<'a, C, A> {}

impl<'a, C, A> UserEventMethods<'a, C, A> {
    
    /// Create a builder to help you perform the following task:
    ///
    /// Logs a user event.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    pub fn log(&self, request: LogUserEventRequest) -> UserEventLogCall<'a, C, A> {
        UserEventLogCall {
            hub: self.hub,
            _request: request,
            _delegate: Default::default(),
            _additional_params: Default::default(),
        }
    }
}



/// A builder providing access to all methods supported on *clientMessage* resources.
/// It is not used directly, but through the `Partners` hub.
///
/// # Example
///
/// Instantiate a resource builder
///
/// ```test_harness,no_run
/// extern crate hyper;
/// extern crate yup_oauth2 as oauth2;
/// extern crate google_partners2 as partners2;
/// 
/// # #[test] fn egal() {
/// use std::default::Default;
/// use oauth2::{Authenticator, DefaultAuthenticatorDelegate, ApplicationSecret, MemoryStorage};
/// use partners2::Partners;
/// 
/// let secret: ApplicationSecret = Default::default();
/// let auth = Authenticator::new(&secret, DefaultAuthenticatorDelegate,
///                               hyper::Client::new(),
///                               <MemoryStorage as Default>::default(), None);
/// let mut hub = Partners::new(hyper::Client::new(), auth);
/// // Usually you wouldn't bind this to a variable, but keep calling *CallBuilders*
/// // like `log(...)`
/// // to build up your call.
/// let rb = hub.client_messages();
/// # }
/// ```
pub struct ClientMessageMethods<'a, C, A>
    where C: 'a, A: 'a {

    hub: &'a Partners<C, A>,
}

impl<'a, C, A> MethodsBuilder for ClientMessageMethods<'a, C, A> {}

impl<'a, C, A> ClientMessageMethods<'a, C, A> {
    
    /// Create a builder to help you perform the following task:
    ///
    /// Logs a generic message from the client, such as `Failed to render component`, `Profile page is running slow`, `More than 500 users have accessed this result.`, etc.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    pub fn log(&self, request: LogMessageRequest) -> ClientMessageLogCall<'a, C, A> {
        ClientMessageLogCall {
            hub: self.hub,
            _request: request,
            _delegate: Default::default(),
            _additional_params: Default::default(),
        }
    }
}



/// A builder providing access to all methods supported on *company* resources.
/// It is not used directly, but through the `Partners` hub.
///
/// # Example
///
/// Instantiate a resource builder
///
/// ```test_harness,no_run
/// extern crate hyper;
/// extern crate yup_oauth2 as oauth2;
/// extern crate google_partners2 as partners2;
/// 
/// # #[test] fn egal() {
/// use std::default::Default;
/// use oauth2::{Authenticator, DefaultAuthenticatorDelegate, ApplicationSecret, MemoryStorage};
/// use partners2::Partners;
/// 
/// let secret: ApplicationSecret = Default::default();
/// let auth = Authenticator::new(&secret, DefaultAuthenticatorDelegate,
///                               hyper::Client::new(),
///                               <MemoryStorage as Default>::default(), None);
/// let mut hub = Partners::new(hyper::Client::new(), auth);
/// // Usually you wouldn't bind this to a variable, but keep calling *CallBuilders*
/// // like `get(...)`, `leads_create(...)` and `list(...)`
/// // to build up your call.
/// let rb = hub.companies();
/// # }
/// ```
pub struct CompanyMethods<'a, C, A>
    where C: 'a, A: 'a {

    hub: &'a Partners<C, A>,
}

impl<'a, C, A> MethodsBuilder for CompanyMethods<'a, C, A> {}

impl<'a, C, A> CompanyMethods<'a, C, A> {
    
    /// Create a builder to help you perform the following task:
    ///
    /// Gets a company.
    /// 
    /// # Arguments
    ///
    /// * `companyId` - The ID of the company to retrieve.
    pub fn get(&self, company_id: &str) -> CompanyGetCall<'a, C, A> {
        CompanyGetCall {
            hub: self.hub,
            _company_id: company_id.to_string(),
            _view: Default::default(),
            _request_metadata_user_overrides_user_id: Default::default(),
            _request_metadata_user_overrides_ip_address: Default::default(),
            _request_metadata_traffic_source_traffic_sub_id: Default::default(),
            _request_metadata_traffic_source_traffic_source_id: Default::default(),
            _request_metadata_partners_session_id: Default::default(),
            _request_metadata_locale: Default::default(),
            _request_metadata_experiment_ids: Default::default(),
            _order_by: Default::default(),
            _currency_code: Default::default(),
            _address: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Creates an advertiser lead for the given company ID.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `companyId` - The ID of the company to contact.
    pub fn leads_create(&self, request: CreateLeadRequest, company_id: &str) -> CompanyLeadCreateCall<'a, C, A> {
        CompanyLeadCreateCall {
            hub: self.hub,
            _request: request,
            _company_id: company_id.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Lists companies.
    pub fn list(&self) -> CompanyListCall<'a, C, A> {
        CompanyListCall {
            hub: self.hub,
            _website_url: Default::default(),
            _view: Default::default(),
            _services: Default::default(),
            _request_metadata_user_overrides_user_id: Default::default(),
            _request_metadata_user_overrides_ip_address: Default::default(),
            _request_metadata_traffic_source_traffic_sub_id: Default::default(),
            _request_metadata_traffic_source_traffic_source_id: Default::default(),
            _request_metadata_partners_session_id: Default::default(),
            _request_metadata_locale: Default::default(),
            _request_metadata_experiment_ids: Default::default(),
            _page_token: Default::default(),
            _page_size: Default::default(),
            _order_by: Default::default(),
            _min_monthly_budget_units: Default::default(),
            _min_monthly_budget_nanos: Default::default(),
            _min_monthly_budget_currency_code: Default::default(),
            _max_monthly_budget_units: Default::default(),
            _max_monthly_budget_nanos: Default::default(),
            _max_monthly_budget_currency_code: Default::default(),
            _language_codes: Default::default(),
            _industries: Default::default(),
            _gps_motivations: Default::default(),
            _company_name: Default::default(),
            _address: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
        }
    }
}



/// A builder providing access to all methods supported on *userState* resources.
/// It is not used directly, but through the `Partners` hub.
///
/// # Example
///
/// Instantiate a resource builder
///
/// ```test_harness,no_run
/// extern crate hyper;
/// extern crate yup_oauth2 as oauth2;
/// extern crate google_partners2 as partners2;
/// 
/// # #[test] fn egal() {
/// use std::default::Default;
/// use oauth2::{Authenticator, DefaultAuthenticatorDelegate, ApplicationSecret, MemoryStorage};
/// use partners2::Partners;
/// 
/// let secret: ApplicationSecret = Default::default();
/// let auth = Authenticator::new(&secret, DefaultAuthenticatorDelegate,
///                               hyper::Client::new(),
///                               <MemoryStorage as Default>::default(), None);
/// let mut hub = Partners::new(hyper::Client::new(), auth);
/// // Usually you wouldn't bind this to a variable, but keep calling *CallBuilders*
/// // like `list(...)`
/// // to build up your call.
/// let rb = hub.user_states();
/// # }
/// ```
pub struct UserStateMethods<'a, C, A>
    where C: 'a, A: 'a {

    hub: &'a Partners<C, A>,
}

impl<'a, C, A> MethodsBuilder for UserStateMethods<'a, C, A> {}

impl<'a, C, A> UserStateMethods<'a, C, A> {
    
    /// Create a builder to help you perform the following task:
    ///
    /// Lists states for current user.
    pub fn list(&self) -> UserStateListCall<'a, C, A> {
        UserStateListCall {
            hub: self.hub,
            _request_metadata_user_overrides_user_id: Default::default(),
            _request_metadata_user_overrides_ip_address: Default::default(),
            _request_metadata_traffic_source_traffic_sub_id: Default::default(),
            _request_metadata_traffic_source_traffic_source_id: Default::default(),
            _request_metadata_partners_session_id: Default::default(),
            _request_metadata_locale: Default::default(),
            _request_metadata_experiment_ids: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
        }
    }
}





// ###################
// CallBuilders   ###
// #################

/// Logs a user event.
///
/// A builder for the *log* method supported by a *userEvent* resource.
/// It is not used directly, but through a `UserEventMethods` instance.
///
/// # Example
///
/// Instantiate a resource method builder
///
/// ```test_harness,no_run
/// # extern crate hyper;
/// # extern crate yup_oauth2 as oauth2;
/// # extern crate google_partners2 as partners2;
/// use partners2::LogUserEventRequest;
/// # #[test] fn egal() {
/// # use std::default::Default;
/// # use oauth2::{Authenticator, DefaultAuthenticatorDelegate, ApplicationSecret, MemoryStorage};
/// # use partners2::Partners;
/// 
/// # let secret: ApplicationSecret = Default::default();
/// # let auth = Authenticator::new(&secret, DefaultAuthenticatorDelegate,
/// #                               hyper::Client::new(),
/// #                               <MemoryStorage as Default>::default(), None);
/// # let mut hub = Partners::new(hyper::Client::new(), auth);
/// // As the method needs a request, you would usually fill it with the desired information
/// // into the respective structure. Some of the parts shown here might not be applicable !
/// // Values shown here are possibly random and not representative !
/// let mut req = LogUserEventRequest::default();
/// 
/// // You can configure optional parameters by calling the respective setters at will, and
/// // execute the final call using `doit()`.
/// // Values shown here are possibly random and not representative !
/// let result = hub.user_events().log(req)
///              .doit();
/// # }
/// ```
pub struct UserEventLogCall<'a, C, A>
    where C: 'a, A: 'a {

    hub: &'a Partners<C, A>,
    _request: LogUserEventRequest,
    _delegate: Option<&'a mut Delegate>,
    _additional_params: HashMap<String, String>,
}

impl<'a, C, A> CallBuilder for UserEventLogCall<'a, C, A> {}

impl<'a, C, A> UserEventLogCall<'a, C, A> where C: BorrowMut<hyper::Client>, A: oauth2::GetToken {


    /// Perform the operation you have build so far.
    pub fn doit(mut self) -> Result<(hyper::client::Response, LogUserEventResponse)> {
        use std::io::{Read, Seek};
        use hyper::header::{ContentType, ContentLength, Authorization, Bearer, UserAgent, Location};
        let mut dd = DefaultDelegate;
        let mut dlg: &mut Delegate = match self._delegate {
            Some(d) => d,
            None => &mut dd
        };
        dlg.begin(MethodInfo { id: "partners.userEvents.log",
                               http_method: hyper::method::Method::Post });
        let mut params: Vec<(&str, String)> = Vec::with_capacity((3 + self._additional_params.len()));
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

        let mut url = "https://partners.googleapis.com/v2/userEvents:log".to_string();
        
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
            url.push_str(&url::form_urlencoded::serialize(params));
        }

        let mut json_mime_type = mime::Mime(mime::TopLevel::Application, mime::SubLevel::Json, Default::default());
        let mut request_value_reader =
            {
                let mut value = json::value::to_value(&self._request).expect("serde to work");
                remove_json_null_values(&mut value);
                let mut dst = io::Cursor::new(Vec::with_capacity(128));
                json::to_writer(&mut dst, &value).unwrap();
                dst
            };
        let request_size = request_value_reader.seek(io::SeekFrom::End(0)).unwrap();
        request_value_reader.seek(io::SeekFrom::Start(0)).unwrap();


        loop {
            request_value_reader.seek(io::SeekFrom::Start(0)).unwrap();
            let mut req_result = {
                let mut client = &mut *self.hub.client.borrow_mut();
                let mut req = client.borrow_mut().request(hyper::method::Method::Post, &url)
                    .header(UserAgent(self.hub._user_agent.clone()))
                    .header(ContentType(json_mime_type.clone()))
                    .header(ContentLength(request_size as u64))
                    .body(&mut request_value_reader);

                dlg.pre_request();
                req.send()
            };

            match req_result {
                Err(err) => {
                    if let oauth2::Retry::After(d) = dlg.http_error(&err) {
                        sleep(d);
                        continue;
                    }
                    dlg.finished(false);
                    return Err(Error::HttpError(err))
                }
                Ok(mut res) => {
                    if !res.status.is_success() {
                        let mut json_err = String::new();
                        res.read_to_string(&mut json_err).unwrap();
                        if let oauth2::Retry::After(d) = dlg.http_failure(&res,
                                                              json::from_str(&json_err).ok(),
                                                              json::from_str(&json_err).ok()) {
                            sleep(d);
                            continue;
                        }
                        dlg.finished(false);
                        return match json::from_str::<ErrorResponse>(&json_err){
                            Err(_) => Err(Error::Failure(res)),
                            Ok(serr) => Err(Error::BadRequest(serr))
                        }
                    }
                    let result_value = {
                        let mut json_response = String::new();
                        res.read_to_string(&mut json_response).unwrap();
                        match json::from_str(&json_response) {
                            Ok(decoded) => (res, decoded),
                            Err(err) => {
                                dlg.response_json_decode_error(&json_response, &err);
                                return Err(Error::JsonDecodeError(json_response, err));
                            }
                        }
                    };

                    dlg.finished(true);
                    return Ok(result_value)
                }
            }
        }
    }


    ///
    /// Sets the *request* property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call,
    /// we provide this method for API completeness.
    pub fn request(mut self, new_value: LogUserEventRequest) -> UserEventLogCall<'a, C, A> {
        self._request = new_value;
        self
    }
    /// The delegate implementation is consulted whenever there is an intermediate result, or if something goes wrong
    /// while executing the actual API request.
    /// 
    /// It should be used to handle progress information, and to implement a certain level of resilience.
    ///
    /// Sets the *delegate* property to the given value.
    pub fn delegate(mut self, new_value: &'a mut Delegate) -> UserEventLogCall<'a, C, A> {
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
    /// * *bearer_token* (query-string) - OAuth bearer token.
    /// * *pp* (query-boolean) - Pretty-print response.
    /// * *prettyPrint* (query-boolean) - Returns response with indentations and line breaks.
    /// * *upload_protocol* (query-string) - Upload protocol for media (e.g. "raw", "multipart").
    /// * *access_token* (query-string) - OAuth access token.
    /// * *fields* (query-string) - Selector specifying which fields to include in a partial response.
    /// * *quotaUser* (query-string) - Available to use for quota purposes for server-side applications. Can be any arbitrary string assigned to a user, but should not exceed 40 characters.
    /// * *callback* (query-string) - JSONP
    /// * *oauth_token* (query-string) - OAuth 2.0 token for the current user.
    /// * *key* (query-string) - API key. Your API key identifies your project and provides you with API access, quota, and reports. Required unless you provide an OAuth 2.0 token.
    /// * *uploadType* (query-string) - Legacy upload protocol for media (e.g. "media", "multipart").
    /// * *alt* (query-string) - Data format for response.
    /// * *$.xgafv* (query-string) - V1 error format.
    pub fn param<T>(mut self, name: T, value: T) -> UserEventLogCall<'a, C, A>
                                                        where T: AsRef<str> {
        self._additional_params.insert(name.as_ref().to_string(), value.as_ref().to_string());
        self
    }

}


/// Logs a generic message from the client, such as `Failed to render component`, `Profile page is running slow`, `More than 500 users have accessed this result.`, etc.
///
/// A builder for the *log* method supported by a *clientMessage* resource.
/// It is not used directly, but through a `ClientMessageMethods` instance.
///
/// # Example
///
/// Instantiate a resource method builder
///
/// ```test_harness,no_run
/// # extern crate hyper;
/// # extern crate yup_oauth2 as oauth2;
/// # extern crate google_partners2 as partners2;
/// use partners2::LogMessageRequest;
/// # #[test] fn egal() {
/// # use std::default::Default;
/// # use oauth2::{Authenticator, DefaultAuthenticatorDelegate, ApplicationSecret, MemoryStorage};
/// # use partners2::Partners;
/// 
/// # let secret: ApplicationSecret = Default::default();
/// # let auth = Authenticator::new(&secret, DefaultAuthenticatorDelegate,
/// #                               hyper::Client::new(),
/// #                               <MemoryStorage as Default>::default(), None);
/// # let mut hub = Partners::new(hyper::Client::new(), auth);
/// // As the method needs a request, you would usually fill it with the desired information
/// // into the respective structure. Some of the parts shown here might not be applicable !
/// // Values shown here are possibly random and not representative !
/// let mut req = LogMessageRequest::default();
/// 
/// // You can configure optional parameters by calling the respective setters at will, and
/// // execute the final call using `doit()`.
/// // Values shown here are possibly random and not representative !
/// let result = hub.client_messages().log(req)
///              .doit();
/// # }
/// ```
pub struct ClientMessageLogCall<'a, C, A>
    where C: 'a, A: 'a {

    hub: &'a Partners<C, A>,
    _request: LogMessageRequest,
    _delegate: Option<&'a mut Delegate>,
    _additional_params: HashMap<String, String>,
}

impl<'a, C, A> CallBuilder for ClientMessageLogCall<'a, C, A> {}

impl<'a, C, A> ClientMessageLogCall<'a, C, A> where C: BorrowMut<hyper::Client>, A: oauth2::GetToken {


    /// Perform the operation you have build so far.
    pub fn doit(mut self) -> Result<(hyper::client::Response, LogMessageResponse)> {
        use std::io::{Read, Seek};
        use hyper::header::{ContentType, ContentLength, Authorization, Bearer, UserAgent, Location};
        let mut dd = DefaultDelegate;
        let mut dlg: &mut Delegate = match self._delegate {
            Some(d) => d,
            None => &mut dd
        };
        dlg.begin(MethodInfo { id: "partners.clientMessages.log",
                               http_method: hyper::method::Method::Post });
        let mut params: Vec<(&str, String)> = Vec::with_capacity((3 + self._additional_params.len()));
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

        let mut url = "https://partners.googleapis.com/v2/clientMessages:log".to_string();
        
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
            url.push_str(&url::form_urlencoded::serialize(params));
        }

        let mut json_mime_type = mime::Mime(mime::TopLevel::Application, mime::SubLevel::Json, Default::default());
        let mut request_value_reader =
            {
                let mut value = json::value::to_value(&self._request).expect("serde to work");
                remove_json_null_values(&mut value);
                let mut dst = io::Cursor::new(Vec::with_capacity(128));
                json::to_writer(&mut dst, &value).unwrap();
                dst
            };
        let request_size = request_value_reader.seek(io::SeekFrom::End(0)).unwrap();
        request_value_reader.seek(io::SeekFrom::Start(0)).unwrap();


        loop {
            request_value_reader.seek(io::SeekFrom::Start(0)).unwrap();
            let mut req_result = {
                let mut client = &mut *self.hub.client.borrow_mut();
                let mut req = client.borrow_mut().request(hyper::method::Method::Post, &url)
                    .header(UserAgent(self.hub._user_agent.clone()))
                    .header(ContentType(json_mime_type.clone()))
                    .header(ContentLength(request_size as u64))
                    .body(&mut request_value_reader);

                dlg.pre_request();
                req.send()
            };

            match req_result {
                Err(err) => {
                    if let oauth2::Retry::After(d) = dlg.http_error(&err) {
                        sleep(d);
                        continue;
                    }
                    dlg.finished(false);
                    return Err(Error::HttpError(err))
                }
                Ok(mut res) => {
                    if !res.status.is_success() {
                        let mut json_err = String::new();
                        res.read_to_string(&mut json_err).unwrap();
                        if let oauth2::Retry::After(d) = dlg.http_failure(&res,
                                                              json::from_str(&json_err).ok(),
                                                              json::from_str(&json_err).ok()) {
                            sleep(d);
                            continue;
                        }
                        dlg.finished(false);
                        return match json::from_str::<ErrorResponse>(&json_err){
                            Err(_) => Err(Error::Failure(res)),
                            Ok(serr) => Err(Error::BadRequest(serr))
                        }
                    }
                    let result_value = {
                        let mut json_response = String::new();
                        res.read_to_string(&mut json_response).unwrap();
                        match json::from_str(&json_response) {
                            Ok(decoded) => (res, decoded),
                            Err(err) => {
                                dlg.response_json_decode_error(&json_response, &err);
                                return Err(Error::JsonDecodeError(json_response, err));
                            }
                        }
                    };

                    dlg.finished(true);
                    return Ok(result_value)
                }
            }
        }
    }


    ///
    /// Sets the *request* property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call,
    /// we provide this method for API completeness.
    pub fn request(mut self, new_value: LogMessageRequest) -> ClientMessageLogCall<'a, C, A> {
        self._request = new_value;
        self
    }
    /// The delegate implementation is consulted whenever there is an intermediate result, or if something goes wrong
    /// while executing the actual API request.
    /// 
    /// It should be used to handle progress information, and to implement a certain level of resilience.
    ///
    /// Sets the *delegate* property to the given value.
    pub fn delegate(mut self, new_value: &'a mut Delegate) -> ClientMessageLogCall<'a, C, A> {
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
    /// * *bearer_token* (query-string) - OAuth bearer token.
    /// * *pp* (query-boolean) - Pretty-print response.
    /// * *prettyPrint* (query-boolean) - Returns response with indentations and line breaks.
    /// * *upload_protocol* (query-string) - Upload protocol for media (e.g. "raw", "multipart").
    /// * *access_token* (query-string) - OAuth access token.
    /// * *fields* (query-string) - Selector specifying which fields to include in a partial response.
    /// * *quotaUser* (query-string) - Available to use for quota purposes for server-side applications. Can be any arbitrary string assigned to a user, but should not exceed 40 characters.
    /// * *callback* (query-string) - JSONP
    /// * *oauth_token* (query-string) - OAuth 2.0 token for the current user.
    /// * *key* (query-string) - API key. Your API key identifies your project and provides you with API access, quota, and reports. Required unless you provide an OAuth 2.0 token.
    /// * *uploadType* (query-string) - Legacy upload protocol for media (e.g. "media", "multipart").
    /// * *alt* (query-string) - Data format for response.
    /// * *$.xgafv* (query-string) - V1 error format.
    pub fn param<T>(mut self, name: T, value: T) -> ClientMessageLogCall<'a, C, A>
                                                        where T: AsRef<str> {
        self._additional_params.insert(name.as_ref().to_string(), value.as_ref().to_string());
        self
    }

}


/// Gets a company.
///
/// A builder for the *get* method supported by a *company* resource.
/// It is not used directly, but through a `CompanyMethods` instance.
///
/// # Example
///
/// Instantiate a resource method builder
///
/// ```test_harness,no_run
/// # extern crate hyper;
/// # extern crate yup_oauth2 as oauth2;
/// # extern crate google_partners2 as partners2;
/// # #[test] fn egal() {
/// # use std::default::Default;
/// # use oauth2::{Authenticator, DefaultAuthenticatorDelegate, ApplicationSecret, MemoryStorage};
/// # use partners2::Partners;
/// 
/// # let secret: ApplicationSecret = Default::default();
/// # let auth = Authenticator::new(&secret, DefaultAuthenticatorDelegate,
/// #                               hyper::Client::new(),
/// #                               <MemoryStorage as Default>::default(), None);
/// # let mut hub = Partners::new(hyper::Client::new(), auth);
/// // You can configure optional parameters by calling the respective setters at will, and
/// // execute the final call using `doit()`.
/// // Values shown here are possibly random and not representative !
/// let result = hub.companies().get("companyId")
///              .view("sit")
///              .request_metadata_user_overrides_user_id("Stet")
///              .request_metadata_user_overrides_ip_address("sed")
///              .request_metadata_traffic_source_traffic_sub_id("et")
///              .request_metadata_traffic_source_traffic_source_id("dolores")
///              .request_metadata_partners_session_id("kasd")
///              .request_metadata_locale("accusam")
///              .add_request_metadata_experiment_ids("takimata")
///              .order_by("justo")
///              .currency_code("amet.")
///              .address("erat")
///              .doit();
/// # }
/// ```
pub struct CompanyGetCall<'a, C, A>
    where C: 'a, A: 'a {

    hub: &'a Partners<C, A>,
    _company_id: String,
    _view: Option<String>,
    _request_metadata_user_overrides_user_id: Option<String>,
    _request_metadata_user_overrides_ip_address: Option<String>,
    _request_metadata_traffic_source_traffic_sub_id: Option<String>,
    _request_metadata_traffic_source_traffic_source_id: Option<String>,
    _request_metadata_partners_session_id: Option<String>,
    _request_metadata_locale: Option<String>,
    _request_metadata_experiment_ids: Vec<String>,
    _order_by: Option<String>,
    _currency_code: Option<String>,
    _address: Option<String>,
    _delegate: Option<&'a mut Delegate>,
    _additional_params: HashMap<String, String>,
}

impl<'a, C, A> CallBuilder for CompanyGetCall<'a, C, A> {}

impl<'a, C, A> CompanyGetCall<'a, C, A> where C: BorrowMut<hyper::Client>, A: oauth2::GetToken {


    /// Perform the operation you have build so far.
    pub fn doit(mut self) -> Result<(hyper::client::Response, GetCompanyResponse)> {
        use std::io::{Read, Seek};
        use hyper::header::{ContentType, ContentLength, Authorization, Bearer, UserAgent, Location};
        let mut dd = DefaultDelegate;
        let mut dlg: &mut Delegate = match self._delegate {
            Some(d) => d,
            None => &mut dd
        };
        dlg.begin(MethodInfo { id: "partners.companies.get",
                               http_method: hyper::method::Method::Get });
        let mut params: Vec<(&str, String)> = Vec::with_capacity((14 + self._additional_params.len()));
        params.push(("companyId", self._company_id.to_string()));
        if let Some(value) = self._view {
            params.push(("view", value.to_string()));
        }
        if let Some(value) = self._request_metadata_user_overrides_user_id {
            params.push(("requestMetadata.userOverrides.userId", value.to_string()));
        }
        if let Some(value) = self._request_metadata_user_overrides_ip_address {
            params.push(("requestMetadata.userOverrides.ipAddress", value.to_string()));
        }
        if let Some(value) = self._request_metadata_traffic_source_traffic_sub_id {
            params.push(("requestMetadata.trafficSource.trafficSubId", value.to_string()));
        }
        if let Some(value) = self._request_metadata_traffic_source_traffic_source_id {
            params.push(("requestMetadata.trafficSource.trafficSourceId", value.to_string()));
        }
        if let Some(value) = self._request_metadata_partners_session_id {
            params.push(("requestMetadata.partnersSessionId", value.to_string()));
        }
        if let Some(value) = self._request_metadata_locale {
            params.push(("requestMetadata.locale", value.to_string()));
        }
        if self._request_metadata_experiment_ids.len() > 0 {
            for f in self._request_metadata_experiment_ids.iter() {
                params.push(("requestMetadata.experimentIds", f.to_string()));
            }
        }
        if let Some(value) = self._order_by {
            params.push(("orderBy", value.to_string()));
        }
        if let Some(value) = self._currency_code {
            params.push(("currencyCode", value.to_string()));
        }
        if let Some(value) = self._address {
            params.push(("address", value.to_string()));
        }
        for &field in ["alt", "companyId", "view", "requestMetadata.userOverrides.userId", "requestMetadata.userOverrides.ipAddress", "requestMetadata.trafficSource.trafficSubId", "requestMetadata.trafficSource.trafficSourceId", "requestMetadata.partnersSessionId", "requestMetadata.locale", "requestMetadata.experimentIds", "orderBy", "currencyCode", "address"].iter() {
            if self._additional_params.contains_key(field) {
                dlg.finished(false);
                return Err(Error::FieldClash(field));
            }
        }
        for (name, value) in self._additional_params.iter() {
            params.push((&name, value.clone()));
        }

        params.push(("alt", "json".to_string()));

        let mut url = "https://partners.googleapis.com/v2/companies/{companyId}".to_string();
        
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

        for &(find_this, param_name) in [("{companyId}", "companyId")].iter() {
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
            for param_name in ["companyId"].iter() {
                if let Some(index) = params.iter().position(|t| &t.0 == param_name) {
                    indices_for_removal.push(index);
                }
            }
            for &index in indices_for_removal.iter() {
                params.remove(index);
            }
        }

        if params.len() > 0 {
            url.push('?');
            url.push_str(&url::form_urlencoded::serialize(params));
        }



        loop {
            let mut req_result = {
                let mut client = &mut *self.hub.client.borrow_mut();
                let mut req = client.borrow_mut().request(hyper::method::Method::Get, &url)
                    .header(UserAgent(self.hub._user_agent.clone()));

                dlg.pre_request();
                req.send()
            };

            match req_result {
                Err(err) => {
                    if let oauth2::Retry::After(d) = dlg.http_error(&err) {
                        sleep(d);
                        continue;
                    }
                    dlg.finished(false);
                    return Err(Error::HttpError(err))
                }
                Ok(mut res) => {
                    if !res.status.is_success() {
                        let mut json_err = String::new();
                        res.read_to_string(&mut json_err).unwrap();
                        if let oauth2::Retry::After(d) = dlg.http_failure(&res,
                                                              json::from_str(&json_err).ok(),
                                                              json::from_str(&json_err).ok()) {
                            sleep(d);
                            continue;
                        }
                        dlg.finished(false);
                        return match json::from_str::<ErrorResponse>(&json_err){
                            Err(_) => Err(Error::Failure(res)),
                            Ok(serr) => Err(Error::BadRequest(serr))
                        }
                    }
                    let result_value = {
                        let mut json_response = String::new();
                        res.read_to_string(&mut json_response).unwrap();
                        match json::from_str(&json_response) {
                            Ok(decoded) => (res, decoded),
                            Err(err) => {
                                dlg.response_json_decode_error(&json_response, &err);
                                return Err(Error::JsonDecodeError(json_response, err));
                            }
                        }
                    };

                    dlg.finished(true);
                    return Ok(result_value)
                }
            }
        }
    }


    /// The ID of the company to retrieve.
    ///
    /// Sets the *company id* path property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call,
    /// we provide this method for API completeness.
    pub fn company_id(mut self, new_value: &str) -> CompanyGetCall<'a, C, A> {
        self._company_id = new_value.to_string();
        self
    }
    /// The view of `Company` resource to be returned. This must not be `COMPANY_VIEW_UNSPECIFIED`.
    ///
    /// Sets the *view* query property to the given value.
    pub fn view(mut self, new_value: &str) -> CompanyGetCall<'a, C, A> {
        self._view = Some(new_value.to_string());
        self
    }
    /// Logged-in user ID to impersonate instead of the user's ID.
    ///
    /// Sets the *request metadata.user overrides.user id* query property to the given value.
    pub fn request_metadata_user_overrides_user_id(mut self, new_value: &str) -> CompanyGetCall<'a, C, A> {
        self._request_metadata_user_overrides_user_id = Some(new_value.to_string());
        self
    }
    /// IP address to use instead of the user's geo-located IP address.
    ///
    /// Sets the *request metadata.user overrides.ip address* query property to the given value.
    pub fn request_metadata_user_overrides_ip_address(mut self, new_value: &str) -> CompanyGetCall<'a, C, A> {
        self._request_metadata_user_overrides_ip_address = Some(new_value.to_string());
        self
    }
    /// Second level identifier to indicate where the traffic comes from. An identifier has multiple letters created by a team which redirected the traffic to us.
    ///
    /// Sets the *request metadata.traffic source.traffic sub id* query property to the given value.
    pub fn request_metadata_traffic_source_traffic_sub_id(mut self, new_value: &str) -> CompanyGetCall<'a, C, A> {
        self._request_metadata_traffic_source_traffic_sub_id = Some(new_value.to_string());
        self
    }
    /// Identifier to indicate where the traffic comes from. An identifier has multiple letters created by a team which redirected the traffic to us.
    ///
    /// Sets the *request metadata.traffic source.traffic source id* query property to the given value.
    pub fn request_metadata_traffic_source_traffic_source_id(mut self, new_value: &str) -> CompanyGetCall<'a, C, A> {
        self._request_metadata_traffic_source_traffic_source_id = Some(new_value.to_string());
        self
    }
    /// Google Partners session ID.
    ///
    /// Sets the *request metadata.partners session id* query property to the given value.
    pub fn request_metadata_partners_session_id(mut self, new_value: &str) -> CompanyGetCall<'a, C, A> {
        self._request_metadata_partners_session_id = Some(new_value.to_string());
        self
    }
    /// Locale to use for the current request.
    ///
    /// Sets the *request metadata.locale* query property to the given value.
    pub fn request_metadata_locale(mut self, new_value: &str) -> CompanyGetCall<'a, C, A> {
        self._request_metadata_locale = Some(new_value.to_string());
        self
    }
    /// Experiment IDs the current request belongs to.
    ///
    /// Append the given value to the *request metadata.experiment ids* query property.
    /// Each appended value will retain its original ordering and be '/'-separated in the URL's parameters.
    pub fn add_request_metadata_experiment_ids(mut self, new_value: &str) -> CompanyGetCall<'a, C, A> {
        self._request_metadata_experiment_ids.push(new_value.to_string());
        self
    }
    /// How to order addresses within the returned company. Currently, only `address` and `address desc` is supported which will sorted by closest to farthest in distance from given address and farthest to closest distance from given address respectively.
    ///
    /// Sets the *order by* query property to the given value.
    pub fn order_by(mut self, new_value: &str) -> CompanyGetCall<'a, C, A> {
        self._order_by = Some(new_value.to_string());
        self
    }
    /// If the company's budget is in a different currency code than this one, then the converted budget is converted to this currency code.
    ///
    /// Sets the *currency code* query property to the given value.
    pub fn currency_code(mut self, new_value: &str) -> CompanyGetCall<'a, C, A> {
        self._currency_code = Some(new_value.to_string());
        self
    }
    /// The address to use for sorting the company's addresses by proximity. If not given, the geo-located address of the request is used. Used when order_by is set.
    ///
    /// Sets the *address* query property to the given value.
    pub fn address(mut self, new_value: &str) -> CompanyGetCall<'a, C, A> {
        self._address = Some(new_value.to_string());
        self
    }
    /// The delegate implementation is consulted whenever there is an intermediate result, or if something goes wrong
    /// while executing the actual API request.
    /// 
    /// It should be used to handle progress information, and to implement a certain level of resilience.
    ///
    /// Sets the *delegate* property to the given value.
    pub fn delegate(mut self, new_value: &'a mut Delegate) -> CompanyGetCall<'a, C, A> {
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
    /// * *bearer_token* (query-string) - OAuth bearer token.
    /// * *pp* (query-boolean) - Pretty-print response.
    /// * *prettyPrint* (query-boolean) - Returns response with indentations and line breaks.
    /// * *upload_protocol* (query-string) - Upload protocol for media (e.g. "raw", "multipart").
    /// * *access_token* (query-string) - OAuth access token.
    /// * *fields* (query-string) - Selector specifying which fields to include in a partial response.
    /// * *quotaUser* (query-string) - Available to use for quota purposes for server-side applications. Can be any arbitrary string assigned to a user, but should not exceed 40 characters.
    /// * *callback* (query-string) - JSONP
    /// * *oauth_token* (query-string) - OAuth 2.0 token for the current user.
    /// * *key* (query-string) - API key. Your API key identifies your project and provides you with API access, quota, and reports. Required unless you provide an OAuth 2.0 token.
    /// * *uploadType* (query-string) - Legacy upload protocol for media (e.g. "media", "multipart").
    /// * *alt* (query-string) - Data format for response.
    /// * *$.xgafv* (query-string) - V1 error format.
    pub fn param<T>(mut self, name: T, value: T) -> CompanyGetCall<'a, C, A>
                                                        where T: AsRef<str> {
        self._additional_params.insert(name.as_ref().to_string(), value.as_ref().to_string());
        self
    }

}


/// Creates an advertiser lead for the given company ID.
///
/// A builder for the *leads.create* method supported by a *company* resource.
/// It is not used directly, but through a `CompanyMethods` instance.
///
/// # Example
///
/// Instantiate a resource method builder
///
/// ```test_harness,no_run
/// # extern crate hyper;
/// # extern crate yup_oauth2 as oauth2;
/// # extern crate google_partners2 as partners2;
/// use partners2::CreateLeadRequest;
/// # #[test] fn egal() {
/// # use std::default::Default;
/// # use oauth2::{Authenticator, DefaultAuthenticatorDelegate, ApplicationSecret, MemoryStorage};
/// # use partners2::Partners;
/// 
/// # let secret: ApplicationSecret = Default::default();
/// # let auth = Authenticator::new(&secret, DefaultAuthenticatorDelegate,
/// #                               hyper::Client::new(),
/// #                               <MemoryStorage as Default>::default(), None);
/// # let mut hub = Partners::new(hyper::Client::new(), auth);
/// // As the method needs a request, you would usually fill it with the desired information
/// // into the respective structure. Some of the parts shown here might not be applicable !
/// // Values shown here are possibly random and not representative !
/// let mut req = CreateLeadRequest::default();
/// 
/// // You can configure optional parameters by calling the respective setters at will, and
/// // execute the final call using `doit()`.
/// // Values shown here are possibly random and not representative !
/// let result = hub.companies().leads_create(req, "companyId")
///              .doit();
/// # }
/// ```
pub struct CompanyLeadCreateCall<'a, C, A>
    where C: 'a, A: 'a {

    hub: &'a Partners<C, A>,
    _request: CreateLeadRequest,
    _company_id: String,
    _delegate: Option<&'a mut Delegate>,
    _additional_params: HashMap<String, String>,
}

impl<'a, C, A> CallBuilder for CompanyLeadCreateCall<'a, C, A> {}

impl<'a, C, A> CompanyLeadCreateCall<'a, C, A> where C: BorrowMut<hyper::Client>, A: oauth2::GetToken {


    /// Perform the operation you have build so far.
    pub fn doit(mut self) -> Result<(hyper::client::Response, CreateLeadResponse)> {
        use std::io::{Read, Seek};
        use hyper::header::{ContentType, ContentLength, Authorization, Bearer, UserAgent, Location};
        let mut dd = DefaultDelegate;
        let mut dlg: &mut Delegate = match self._delegate {
            Some(d) => d,
            None => &mut dd
        };
        dlg.begin(MethodInfo { id: "partners.companies.leads.create",
                               http_method: hyper::method::Method::Post });
        let mut params: Vec<(&str, String)> = Vec::with_capacity((4 + self._additional_params.len()));
        params.push(("companyId", self._company_id.to_string()));
        for &field in ["alt", "companyId"].iter() {
            if self._additional_params.contains_key(field) {
                dlg.finished(false);
                return Err(Error::FieldClash(field));
            }
        }
        for (name, value) in self._additional_params.iter() {
            params.push((&name, value.clone()));
        }

        params.push(("alt", "json".to_string()));

        let mut url = "https://partners.googleapis.com/v2/companies/{companyId}/leads".to_string();
        
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

        for &(find_this, param_name) in [("{companyId}", "companyId")].iter() {
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
            for param_name in ["companyId"].iter() {
                if let Some(index) = params.iter().position(|t| &t.0 == param_name) {
                    indices_for_removal.push(index);
                }
            }
            for &index in indices_for_removal.iter() {
                params.remove(index);
            }
        }

        if params.len() > 0 {
            url.push('?');
            url.push_str(&url::form_urlencoded::serialize(params));
        }

        let mut json_mime_type = mime::Mime(mime::TopLevel::Application, mime::SubLevel::Json, Default::default());
        let mut request_value_reader =
            {
                let mut value = json::value::to_value(&self._request).expect("serde to work");
                remove_json_null_values(&mut value);
                let mut dst = io::Cursor::new(Vec::with_capacity(128));
                json::to_writer(&mut dst, &value).unwrap();
                dst
            };
        let request_size = request_value_reader.seek(io::SeekFrom::End(0)).unwrap();
        request_value_reader.seek(io::SeekFrom::Start(0)).unwrap();


        loop {
            request_value_reader.seek(io::SeekFrom::Start(0)).unwrap();
            let mut req_result = {
                let mut client = &mut *self.hub.client.borrow_mut();
                let mut req = client.borrow_mut().request(hyper::method::Method::Post, &url)
                    .header(UserAgent(self.hub._user_agent.clone()))
                    .header(ContentType(json_mime_type.clone()))
                    .header(ContentLength(request_size as u64))
                    .body(&mut request_value_reader);

                dlg.pre_request();
                req.send()
            };

            match req_result {
                Err(err) => {
                    if let oauth2::Retry::After(d) = dlg.http_error(&err) {
                        sleep(d);
                        continue;
                    }
                    dlg.finished(false);
                    return Err(Error::HttpError(err))
                }
                Ok(mut res) => {
                    if !res.status.is_success() {
                        let mut json_err = String::new();
                        res.read_to_string(&mut json_err).unwrap();
                        if let oauth2::Retry::After(d) = dlg.http_failure(&res,
                                                              json::from_str(&json_err).ok(),
                                                              json::from_str(&json_err).ok()) {
                            sleep(d);
                            continue;
                        }
                        dlg.finished(false);
                        return match json::from_str::<ErrorResponse>(&json_err){
                            Err(_) => Err(Error::Failure(res)),
                            Ok(serr) => Err(Error::BadRequest(serr))
                        }
                    }
                    let result_value = {
                        let mut json_response = String::new();
                        res.read_to_string(&mut json_response).unwrap();
                        match json::from_str(&json_response) {
                            Ok(decoded) => (res, decoded),
                            Err(err) => {
                                dlg.response_json_decode_error(&json_response, &err);
                                return Err(Error::JsonDecodeError(json_response, err));
                            }
                        }
                    };

                    dlg.finished(true);
                    return Ok(result_value)
                }
            }
        }
    }


    ///
    /// Sets the *request* property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call,
    /// we provide this method for API completeness.
    pub fn request(mut self, new_value: CreateLeadRequest) -> CompanyLeadCreateCall<'a, C, A> {
        self._request = new_value;
        self
    }
    /// The ID of the company to contact.
    ///
    /// Sets the *company id* path property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call,
    /// we provide this method for API completeness.
    pub fn company_id(mut self, new_value: &str) -> CompanyLeadCreateCall<'a, C, A> {
        self._company_id = new_value.to_string();
        self
    }
    /// The delegate implementation is consulted whenever there is an intermediate result, or if something goes wrong
    /// while executing the actual API request.
    /// 
    /// It should be used to handle progress information, and to implement a certain level of resilience.
    ///
    /// Sets the *delegate* property to the given value.
    pub fn delegate(mut self, new_value: &'a mut Delegate) -> CompanyLeadCreateCall<'a, C, A> {
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
    /// * *bearer_token* (query-string) - OAuth bearer token.
    /// * *pp* (query-boolean) - Pretty-print response.
    /// * *prettyPrint* (query-boolean) - Returns response with indentations and line breaks.
    /// * *upload_protocol* (query-string) - Upload protocol for media (e.g. "raw", "multipart").
    /// * *access_token* (query-string) - OAuth access token.
    /// * *fields* (query-string) - Selector specifying which fields to include in a partial response.
    /// * *quotaUser* (query-string) - Available to use for quota purposes for server-side applications. Can be any arbitrary string assigned to a user, but should not exceed 40 characters.
    /// * *callback* (query-string) - JSONP
    /// * *oauth_token* (query-string) - OAuth 2.0 token for the current user.
    /// * *key* (query-string) - API key. Your API key identifies your project and provides you with API access, quota, and reports. Required unless you provide an OAuth 2.0 token.
    /// * *uploadType* (query-string) - Legacy upload protocol for media (e.g. "media", "multipart").
    /// * *alt* (query-string) - Data format for response.
    /// * *$.xgafv* (query-string) - V1 error format.
    pub fn param<T>(mut self, name: T, value: T) -> CompanyLeadCreateCall<'a, C, A>
                                                        where T: AsRef<str> {
        self._additional_params.insert(name.as_ref().to_string(), value.as_ref().to_string());
        self
    }

}


/// Lists companies.
///
/// A builder for the *list* method supported by a *company* resource.
/// It is not used directly, but through a `CompanyMethods` instance.
///
/// # Example
///
/// Instantiate a resource method builder
///
/// ```test_harness,no_run
/// # extern crate hyper;
/// # extern crate yup_oauth2 as oauth2;
/// # extern crate google_partners2 as partners2;
/// # #[test] fn egal() {
/// # use std::default::Default;
/// # use oauth2::{Authenticator, DefaultAuthenticatorDelegate, ApplicationSecret, MemoryStorage};
/// # use partners2::Partners;
/// 
/// # let secret: ApplicationSecret = Default::default();
/// # let auth = Authenticator::new(&secret, DefaultAuthenticatorDelegate,
/// #                               hyper::Client::new(),
/// #                               <MemoryStorage as Default>::default(), None);
/// # let mut hub = Partners::new(hyper::Client::new(), auth);
/// // You can configure optional parameters by calling the respective setters at will, and
/// // execute the final call using `doit()`.
/// // Values shown here are possibly random and not representative !
/// let result = hub.companies().list()
///              .website_url("sea")
///              .view("nonumy")
///              .add_services("dolores")
///              .request_metadata_user_overrides_user_id("gubergren")
///              .request_metadata_user_overrides_ip_address("sadipscing")
///              .request_metadata_traffic_source_traffic_sub_id("aliquyam")
///              .request_metadata_traffic_source_traffic_source_id("ea")
///              .request_metadata_partners_session_id("no")
///              .request_metadata_locale("justo")
///              .add_request_metadata_experiment_ids("justo")
///              .page_token("et")
///              .page_size(-17)
///              .order_by("diam")
///              .min_monthly_budget_units("ipsum")
///              .min_monthly_budget_nanos(-5)
///              .min_monthly_budget_currency_code("et")
///              .max_monthly_budget_units("duo")
///              .max_monthly_budget_nanos(-32)
///              .max_monthly_budget_currency_code("sea")
///              .add_language_codes("Lorem")
///              .add_industries("eos")
///              .add_gps_motivations("erat")
///              .company_name("sadipscing")
///              .address("dolor")
///              .doit();
/// # }
/// ```
pub struct CompanyListCall<'a, C, A>
    where C: 'a, A: 'a {

    hub: &'a Partners<C, A>,
    _website_url: Option<String>,
    _view: Option<String>,
    _services: Vec<String>,
    _request_metadata_user_overrides_user_id: Option<String>,
    _request_metadata_user_overrides_ip_address: Option<String>,
    _request_metadata_traffic_source_traffic_sub_id: Option<String>,
    _request_metadata_traffic_source_traffic_source_id: Option<String>,
    _request_metadata_partners_session_id: Option<String>,
    _request_metadata_locale: Option<String>,
    _request_metadata_experiment_ids: Vec<String>,
    _page_token: Option<String>,
    _page_size: Option<i32>,
    _order_by: Option<String>,
    _min_monthly_budget_units: Option<String>,
    _min_monthly_budget_nanos: Option<i32>,
    _min_monthly_budget_currency_code: Option<String>,
    _max_monthly_budget_units: Option<String>,
    _max_monthly_budget_nanos: Option<i32>,
    _max_monthly_budget_currency_code: Option<String>,
    _language_codes: Vec<String>,
    _industries: Vec<String>,
    _gps_motivations: Vec<String>,
    _company_name: Option<String>,
    _address: Option<String>,
    _delegate: Option<&'a mut Delegate>,
    _additional_params: HashMap<String, String>,
}

impl<'a, C, A> CallBuilder for CompanyListCall<'a, C, A> {}

impl<'a, C, A> CompanyListCall<'a, C, A> where C: BorrowMut<hyper::Client>, A: oauth2::GetToken {


    /// Perform the operation you have build so far.
    pub fn doit(mut self) -> Result<(hyper::client::Response, ListCompaniesResponse)> {
        use std::io::{Read, Seek};
        use hyper::header::{ContentType, ContentLength, Authorization, Bearer, UserAgent, Location};
        let mut dd = DefaultDelegate;
        let mut dlg: &mut Delegate = match self._delegate {
            Some(d) => d,
            None => &mut dd
        };
        dlg.begin(MethodInfo { id: "partners.companies.list",
                               http_method: hyper::method::Method::Get });
        let mut params: Vec<(&str, String)> = Vec::with_capacity((26 + self._additional_params.len()));
        if let Some(value) = self._website_url {
            params.push(("websiteUrl", value.to_string()));
        }
        if let Some(value) = self._view {
            params.push(("view", value.to_string()));
        }
        if self._services.len() > 0 {
            for f in self._services.iter() {
                params.push(("services", f.to_string()));
            }
        }
        if let Some(value) = self._request_metadata_user_overrides_user_id {
            params.push(("requestMetadata.userOverrides.userId", value.to_string()));
        }
        if let Some(value) = self._request_metadata_user_overrides_ip_address {
            params.push(("requestMetadata.userOverrides.ipAddress", value.to_string()));
        }
        if let Some(value) = self._request_metadata_traffic_source_traffic_sub_id {
            params.push(("requestMetadata.trafficSource.trafficSubId", value.to_string()));
        }
        if let Some(value) = self._request_metadata_traffic_source_traffic_source_id {
            params.push(("requestMetadata.trafficSource.trafficSourceId", value.to_string()));
        }
        if let Some(value) = self._request_metadata_partners_session_id {
            params.push(("requestMetadata.partnersSessionId", value.to_string()));
        }
        if let Some(value) = self._request_metadata_locale {
            params.push(("requestMetadata.locale", value.to_string()));
        }
        if self._request_metadata_experiment_ids.len() > 0 {
            for f in self._request_metadata_experiment_ids.iter() {
                params.push(("requestMetadata.experimentIds", f.to_string()));
            }
        }
        if let Some(value) = self._page_token {
            params.push(("pageToken", value.to_string()));
        }
        if let Some(value) = self._page_size {
            params.push(("pageSize", value.to_string()));
        }
        if let Some(value) = self._order_by {
            params.push(("orderBy", value.to_string()));
        }
        if let Some(value) = self._min_monthly_budget_units {
            params.push(("minMonthlyBudget.units", value.to_string()));
        }
        if let Some(value) = self._min_monthly_budget_nanos {
            params.push(("minMonthlyBudget.nanos", value.to_string()));
        }
        if let Some(value) = self._min_monthly_budget_currency_code {
            params.push(("minMonthlyBudget.currencyCode", value.to_string()));
        }
        if let Some(value) = self._max_monthly_budget_units {
            params.push(("maxMonthlyBudget.units", value.to_string()));
        }
        if let Some(value) = self._max_monthly_budget_nanos {
            params.push(("maxMonthlyBudget.nanos", value.to_string()));
        }
        if let Some(value) = self._max_monthly_budget_currency_code {
            params.push(("maxMonthlyBudget.currencyCode", value.to_string()));
        }
        if self._language_codes.len() > 0 {
            for f in self._language_codes.iter() {
                params.push(("languageCodes", f.to_string()));
            }
        }
        if self._industries.len() > 0 {
            for f in self._industries.iter() {
                params.push(("industries", f.to_string()));
            }
        }
        if self._gps_motivations.len() > 0 {
            for f in self._gps_motivations.iter() {
                params.push(("gpsMotivations", f.to_string()));
            }
        }
        if let Some(value) = self._company_name {
            params.push(("companyName", value.to_string()));
        }
        if let Some(value) = self._address {
            params.push(("address", value.to_string()));
        }
        for &field in ["alt", "websiteUrl", "view", "services", "requestMetadata.userOverrides.userId", "requestMetadata.userOverrides.ipAddress", "requestMetadata.trafficSource.trafficSubId", "requestMetadata.trafficSource.trafficSourceId", "requestMetadata.partnersSessionId", "requestMetadata.locale", "requestMetadata.experimentIds", "pageToken", "pageSize", "orderBy", "minMonthlyBudget.units", "minMonthlyBudget.nanos", "minMonthlyBudget.currencyCode", "maxMonthlyBudget.units", "maxMonthlyBudget.nanos", "maxMonthlyBudget.currencyCode", "languageCodes", "industries", "gpsMotivations", "companyName", "address"].iter() {
            if self._additional_params.contains_key(field) {
                dlg.finished(false);
                return Err(Error::FieldClash(field));
            }
        }
        for (name, value) in self._additional_params.iter() {
            params.push((&name, value.clone()));
        }

        params.push(("alt", "json".to_string()));

        let mut url = "https://partners.googleapis.com/v2/companies".to_string();
        
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
            url.push_str(&url::form_urlencoded::serialize(params));
        }



        loop {
            let mut req_result = {
                let mut client = &mut *self.hub.client.borrow_mut();
                let mut req = client.borrow_mut().request(hyper::method::Method::Get, &url)
                    .header(UserAgent(self.hub._user_agent.clone()));

                dlg.pre_request();
                req.send()
            };

            match req_result {
                Err(err) => {
                    if let oauth2::Retry::After(d) = dlg.http_error(&err) {
                        sleep(d);
                        continue;
                    }
                    dlg.finished(false);
                    return Err(Error::HttpError(err))
                }
                Ok(mut res) => {
                    if !res.status.is_success() {
                        let mut json_err = String::new();
                        res.read_to_string(&mut json_err).unwrap();
                        if let oauth2::Retry::After(d) = dlg.http_failure(&res,
                                                              json::from_str(&json_err).ok(),
                                                              json::from_str(&json_err).ok()) {
                            sleep(d);
                            continue;
                        }
                        dlg.finished(false);
                        return match json::from_str::<ErrorResponse>(&json_err){
                            Err(_) => Err(Error::Failure(res)),
                            Ok(serr) => Err(Error::BadRequest(serr))
                        }
                    }
                    let result_value = {
                        let mut json_response = String::new();
                        res.read_to_string(&mut json_response).unwrap();
                        match json::from_str(&json_response) {
                            Ok(decoded) => (res, decoded),
                            Err(err) => {
                                dlg.response_json_decode_error(&json_response, &err);
                                return Err(Error::JsonDecodeError(json_response, err));
                            }
                        }
                    };

                    dlg.finished(true);
                    return Ok(result_value)
                }
            }
        }
    }


    /// Website URL that will help to find a better matched company. .
    ///
    /// Sets the *website url* query property to the given value.
    pub fn website_url(mut self, new_value: &str) -> CompanyListCall<'a, C, A> {
        self._website_url = Some(new_value.to_string());
        self
    }
    /// The view of the `Company` resource to be returned. This must not be `COMPANY_VIEW_UNSPECIFIED`.
    ///
    /// Sets the *view* query property to the given value.
    pub fn view(mut self, new_value: &str) -> CompanyListCall<'a, C, A> {
        self._view = Some(new_value.to_string());
        self
    }
    /// List of services the company can help with.
    ///
    /// Append the given value to the *services* query property.
    /// Each appended value will retain its original ordering and be '/'-separated in the URL's parameters.
    pub fn add_services(mut self, new_value: &str) -> CompanyListCall<'a, C, A> {
        self._services.push(new_value.to_string());
        self
    }
    /// Logged-in user ID to impersonate instead of the user's ID.
    ///
    /// Sets the *request metadata.user overrides.user id* query property to the given value.
    pub fn request_metadata_user_overrides_user_id(mut self, new_value: &str) -> CompanyListCall<'a, C, A> {
        self._request_metadata_user_overrides_user_id = Some(new_value.to_string());
        self
    }
    /// IP address to use instead of the user's geo-located IP address.
    ///
    /// Sets the *request metadata.user overrides.ip address* query property to the given value.
    pub fn request_metadata_user_overrides_ip_address(mut self, new_value: &str) -> CompanyListCall<'a, C, A> {
        self._request_metadata_user_overrides_ip_address = Some(new_value.to_string());
        self
    }
    /// Second level identifier to indicate where the traffic comes from. An identifier has multiple letters created by a team which redirected the traffic to us.
    ///
    /// Sets the *request metadata.traffic source.traffic sub id* query property to the given value.
    pub fn request_metadata_traffic_source_traffic_sub_id(mut self, new_value: &str) -> CompanyListCall<'a, C, A> {
        self._request_metadata_traffic_source_traffic_sub_id = Some(new_value.to_string());
        self
    }
    /// Identifier to indicate where the traffic comes from. An identifier has multiple letters created by a team which redirected the traffic to us.
    ///
    /// Sets the *request metadata.traffic source.traffic source id* query property to the given value.
    pub fn request_metadata_traffic_source_traffic_source_id(mut self, new_value: &str) -> CompanyListCall<'a, C, A> {
        self._request_metadata_traffic_source_traffic_source_id = Some(new_value.to_string());
        self
    }
    /// Google Partners session ID.
    ///
    /// Sets the *request metadata.partners session id* query property to the given value.
    pub fn request_metadata_partners_session_id(mut self, new_value: &str) -> CompanyListCall<'a, C, A> {
        self._request_metadata_partners_session_id = Some(new_value.to_string());
        self
    }
    /// Locale to use for the current request.
    ///
    /// Sets the *request metadata.locale* query property to the given value.
    pub fn request_metadata_locale(mut self, new_value: &str) -> CompanyListCall<'a, C, A> {
        self._request_metadata_locale = Some(new_value.to_string());
        self
    }
    /// Experiment IDs the current request belongs to.
    ///
    /// Append the given value to the *request metadata.experiment ids* query property.
    /// Each appended value will retain its original ordering and be '/'-separated in the URL's parameters.
    pub fn add_request_metadata_experiment_ids(mut self, new_value: &str) -> CompanyListCall<'a, C, A> {
        self._request_metadata_experiment_ids.push(new_value.to_string());
        self
    }
    /// A token identifying a page of results that the server returns. Typically, this is the value of `ListCompaniesResponse.next_page_token` returned from the previous call to ListCompanies.
    ///
    /// Sets the *page token* query property to the given value.
    pub fn page_token(mut self, new_value: &str) -> CompanyListCall<'a, C, A> {
        self._page_token = Some(new_value.to_string());
        self
    }
    /// Requested page size. Server may return fewer companies than requested. If unspecified, server picks an appropriate default.
    ///
    /// Sets the *page size* query property to the given value.
    pub fn page_size(mut self, new_value: i32) -> CompanyListCall<'a, C, A> {
        self._page_size = Some(new_value);
        self
    }
    /// How to order addresses within the returned companies. Currently, only `address` and `address desc` is supported which will sorted by closest to farthest in distance from given address and farthest to closest distance from given address respectively.
    ///
    /// Sets the *order by* query property to the given value.
    pub fn order_by(mut self, new_value: &str) -> CompanyListCall<'a, C, A> {
        self._order_by = Some(new_value.to_string());
        self
    }
    /// The whole units of the amount. For example if `currencyCode` is `"USD"`, then 1 unit is one US dollar.
    ///
    /// Sets the *min monthly budget.units* query property to the given value.
    pub fn min_monthly_budget_units(mut self, new_value: &str) -> CompanyListCall<'a, C, A> {
        self._min_monthly_budget_units = Some(new_value.to_string());
        self
    }
    /// Number of nano (10^-9) units of the amount. The value must be between -999,999,999 and +999,999,999 inclusive. If `units` is positive, `nanos` must be positive or zero. If `units` is zero, `nanos` can be positive, zero, or negative. If `units` is negative, `nanos` must be negative or zero. For example $-1.75 is represented as `units`=-1 and `nanos`=-750,000,000.
    ///
    /// Sets the *min monthly budget.nanos* query property to the given value.
    pub fn min_monthly_budget_nanos(mut self, new_value: i32) -> CompanyListCall<'a, C, A> {
        self._min_monthly_budget_nanos = Some(new_value);
        self
    }
    /// The 3-letter currency code defined in ISO 4217.
    ///
    /// Sets the *min monthly budget.currency code* query property to the given value.
    pub fn min_monthly_budget_currency_code(mut self, new_value: &str) -> CompanyListCall<'a, C, A> {
        self._min_monthly_budget_currency_code = Some(new_value.to_string());
        self
    }
    /// The whole units of the amount. For example if `currencyCode` is `"USD"`, then 1 unit is one US dollar.
    ///
    /// Sets the *max monthly budget.units* query property to the given value.
    pub fn max_monthly_budget_units(mut self, new_value: &str) -> CompanyListCall<'a, C, A> {
        self._max_monthly_budget_units = Some(new_value.to_string());
        self
    }
    /// Number of nano (10^-9) units of the amount. The value must be between -999,999,999 and +999,999,999 inclusive. If `units` is positive, `nanos` must be positive or zero. If `units` is zero, `nanos` can be positive, zero, or negative. If `units` is negative, `nanos` must be negative or zero. For example $-1.75 is represented as `units`=-1 and `nanos`=-750,000,000.
    ///
    /// Sets the *max monthly budget.nanos* query property to the given value.
    pub fn max_monthly_budget_nanos(mut self, new_value: i32) -> CompanyListCall<'a, C, A> {
        self._max_monthly_budget_nanos = Some(new_value);
        self
    }
    /// The 3-letter currency code defined in ISO 4217.
    ///
    /// Sets the *max monthly budget.currency code* query property to the given value.
    pub fn max_monthly_budget_currency_code(mut self, new_value: &str) -> CompanyListCall<'a, C, A> {
        self._max_monthly_budget_currency_code = Some(new_value.to_string());
        self
    }
    /// List of language codes that company can support. Only primary language subtags are accepted as defined by BCP 47 (IETF BCP 47, "Tags for Identifying Languages").
    ///
    /// Append the given value to the *language codes* query property.
    /// Each appended value will retain its original ordering and be '/'-separated in the URL's parameters.
    pub fn add_language_codes(mut self, new_value: &str) -> CompanyListCall<'a, C, A> {
        self._language_codes.push(new_value.to_string());
        self
    }
    /// List of industries the company can help with.
    ///
    /// Append the given value to the *industries* query property.
    /// Each appended value will retain its original ordering and be '/'-separated in the URL's parameters.
    pub fn add_industries(mut self, new_value: &str) -> CompanyListCall<'a, C, A> {
        self._industries.push(new_value.to_string());
        self
    }
    /// List of reasons for using Google Partner Search to get companies.
    ///
    /// Append the given value to the *gps motivations* query property.
    /// Each appended value will retain its original ordering and be '/'-separated in the URL's parameters.
    pub fn add_gps_motivations(mut self, new_value: &str) -> CompanyListCall<'a, C, A> {
        self._gps_motivations.push(new_value.to_string());
        self
    }
    /// Company name to search for.
    ///
    /// Sets the *company name* query property to the given value.
    pub fn company_name(mut self, new_value: &str) -> CompanyListCall<'a, C, A> {
        self._company_name = Some(new_value.to_string());
        self
    }
    /// The address to use when searching for companies. If not given, the geo-located address of the request is used.
    ///
    /// Sets the *address* query property to the given value.
    pub fn address(mut self, new_value: &str) -> CompanyListCall<'a, C, A> {
        self._address = Some(new_value.to_string());
        self
    }
    /// The delegate implementation is consulted whenever there is an intermediate result, or if something goes wrong
    /// while executing the actual API request.
    /// 
    /// It should be used to handle progress information, and to implement a certain level of resilience.
    ///
    /// Sets the *delegate* property to the given value.
    pub fn delegate(mut self, new_value: &'a mut Delegate) -> CompanyListCall<'a, C, A> {
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
    /// * *bearer_token* (query-string) - OAuth bearer token.
    /// * *pp* (query-boolean) - Pretty-print response.
    /// * *prettyPrint* (query-boolean) - Returns response with indentations and line breaks.
    /// * *upload_protocol* (query-string) - Upload protocol for media (e.g. "raw", "multipart").
    /// * *access_token* (query-string) - OAuth access token.
    /// * *fields* (query-string) - Selector specifying which fields to include in a partial response.
    /// * *quotaUser* (query-string) - Available to use for quota purposes for server-side applications. Can be any arbitrary string assigned to a user, but should not exceed 40 characters.
    /// * *callback* (query-string) - JSONP
    /// * *oauth_token* (query-string) - OAuth 2.0 token for the current user.
    /// * *key* (query-string) - API key. Your API key identifies your project and provides you with API access, quota, and reports. Required unless you provide an OAuth 2.0 token.
    /// * *uploadType* (query-string) - Legacy upload protocol for media (e.g. "media", "multipart").
    /// * *alt* (query-string) - Data format for response.
    /// * *$.xgafv* (query-string) - V1 error format.
    pub fn param<T>(mut self, name: T, value: T) -> CompanyListCall<'a, C, A>
                                                        where T: AsRef<str> {
        self._additional_params.insert(name.as_ref().to_string(), value.as_ref().to_string());
        self
    }

}


/// Lists states for current user.
///
/// A builder for the *list* method supported by a *userState* resource.
/// It is not used directly, but through a `UserStateMethods` instance.
///
/// # Example
///
/// Instantiate a resource method builder
///
/// ```test_harness,no_run
/// # extern crate hyper;
/// # extern crate yup_oauth2 as oauth2;
/// # extern crate google_partners2 as partners2;
/// # #[test] fn egal() {
/// # use std::default::Default;
/// # use oauth2::{Authenticator, DefaultAuthenticatorDelegate, ApplicationSecret, MemoryStorage};
/// # use partners2::Partners;
/// 
/// # let secret: ApplicationSecret = Default::default();
/// # let auth = Authenticator::new(&secret, DefaultAuthenticatorDelegate,
/// #                               hyper::Client::new(),
/// #                               <MemoryStorage as Default>::default(), None);
/// # let mut hub = Partners::new(hyper::Client::new(), auth);
/// // You can configure optional parameters by calling the respective setters at will, and
/// // execute the final call using `doit()`.
/// // Values shown here are possibly random and not representative !
/// let result = hub.user_states().list()
///              .request_metadata_user_overrides_user_id("eirmod")
///              .request_metadata_user_overrides_ip_address("elitr")
///              .request_metadata_traffic_source_traffic_sub_id("amet")
///              .request_metadata_traffic_source_traffic_source_id("no")
///              .request_metadata_partners_session_id("labore")
///              .request_metadata_locale("eirmod")
///              .add_request_metadata_experiment_ids("dolore")
///              .doit();
/// # }
/// ```
pub struct UserStateListCall<'a, C, A>
    where C: 'a, A: 'a {

    hub: &'a Partners<C, A>,
    _request_metadata_user_overrides_user_id: Option<String>,
    _request_metadata_user_overrides_ip_address: Option<String>,
    _request_metadata_traffic_source_traffic_sub_id: Option<String>,
    _request_metadata_traffic_source_traffic_source_id: Option<String>,
    _request_metadata_partners_session_id: Option<String>,
    _request_metadata_locale: Option<String>,
    _request_metadata_experiment_ids: Vec<String>,
    _delegate: Option<&'a mut Delegate>,
    _additional_params: HashMap<String, String>,
}

impl<'a, C, A> CallBuilder for UserStateListCall<'a, C, A> {}

impl<'a, C, A> UserStateListCall<'a, C, A> where C: BorrowMut<hyper::Client>, A: oauth2::GetToken {


    /// Perform the operation you have build so far.
    pub fn doit(mut self) -> Result<(hyper::client::Response, ListUserStatesResponse)> {
        use std::io::{Read, Seek};
        use hyper::header::{ContentType, ContentLength, Authorization, Bearer, UserAgent, Location};
        let mut dd = DefaultDelegate;
        let mut dlg: &mut Delegate = match self._delegate {
            Some(d) => d,
            None => &mut dd
        };
        dlg.begin(MethodInfo { id: "partners.userStates.list",
                               http_method: hyper::method::Method::Get });
        let mut params: Vec<(&str, String)> = Vec::with_capacity((9 + self._additional_params.len()));
        if let Some(value) = self._request_metadata_user_overrides_user_id {
            params.push(("requestMetadata.userOverrides.userId", value.to_string()));
        }
        if let Some(value) = self._request_metadata_user_overrides_ip_address {
            params.push(("requestMetadata.userOverrides.ipAddress", value.to_string()));
        }
        if let Some(value) = self._request_metadata_traffic_source_traffic_sub_id {
            params.push(("requestMetadata.trafficSource.trafficSubId", value.to_string()));
        }
        if let Some(value) = self._request_metadata_traffic_source_traffic_source_id {
            params.push(("requestMetadata.trafficSource.trafficSourceId", value.to_string()));
        }
        if let Some(value) = self._request_metadata_partners_session_id {
            params.push(("requestMetadata.partnersSessionId", value.to_string()));
        }
        if let Some(value) = self._request_metadata_locale {
            params.push(("requestMetadata.locale", value.to_string()));
        }
        if self._request_metadata_experiment_ids.len() > 0 {
            for f in self._request_metadata_experiment_ids.iter() {
                params.push(("requestMetadata.experimentIds", f.to_string()));
            }
        }
        for &field in ["alt", "requestMetadata.userOverrides.userId", "requestMetadata.userOverrides.ipAddress", "requestMetadata.trafficSource.trafficSubId", "requestMetadata.trafficSource.trafficSourceId", "requestMetadata.partnersSessionId", "requestMetadata.locale", "requestMetadata.experimentIds"].iter() {
            if self._additional_params.contains_key(field) {
                dlg.finished(false);
                return Err(Error::FieldClash(field));
            }
        }
        for (name, value) in self._additional_params.iter() {
            params.push((&name, value.clone()));
        }

        params.push(("alt", "json".to_string()));

        let mut url = "https://partners.googleapis.com/v2/userStates".to_string();
        
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
            url.push_str(&url::form_urlencoded::serialize(params));
        }



        loop {
            let mut req_result = {
                let mut client = &mut *self.hub.client.borrow_mut();
                let mut req = client.borrow_mut().request(hyper::method::Method::Get, &url)
                    .header(UserAgent(self.hub._user_agent.clone()));

                dlg.pre_request();
                req.send()
            };

            match req_result {
                Err(err) => {
                    if let oauth2::Retry::After(d) = dlg.http_error(&err) {
                        sleep(d);
                        continue;
                    }
                    dlg.finished(false);
                    return Err(Error::HttpError(err))
                }
                Ok(mut res) => {
                    if !res.status.is_success() {
                        let mut json_err = String::new();
                        res.read_to_string(&mut json_err).unwrap();
                        if let oauth2::Retry::After(d) = dlg.http_failure(&res,
                                                              json::from_str(&json_err).ok(),
                                                              json::from_str(&json_err).ok()) {
                            sleep(d);
                            continue;
                        }
                        dlg.finished(false);
                        return match json::from_str::<ErrorResponse>(&json_err){
                            Err(_) => Err(Error::Failure(res)),
                            Ok(serr) => Err(Error::BadRequest(serr))
                        }
                    }
                    let result_value = {
                        let mut json_response = String::new();
                        res.read_to_string(&mut json_response).unwrap();
                        match json::from_str(&json_response) {
                            Ok(decoded) => (res, decoded),
                            Err(err) => {
                                dlg.response_json_decode_error(&json_response, &err);
                                return Err(Error::JsonDecodeError(json_response, err));
                            }
                        }
                    };

                    dlg.finished(true);
                    return Ok(result_value)
                }
            }
        }
    }


    /// Logged-in user ID to impersonate instead of the user's ID.
    ///
    /// Sets the *request metadata.user overrides.user id* query property to the given value.
    pub fn request_metadata_user_overrides_user_id(mut self, new_value: &str) -> UserStateListCall<'a, C, A> {
        self._request_metadata_user_overrides_user_id = Some(new_value.to_string());
        self
    }
    /// IP address to use instead of the user's geo-located IP address.
    ///
    /// Sets the *request metadata.user overrides.ip address* query property to the given value.
    pub fn request_metadata_user_overrides_ip_address(mut self, new_value: &str) -> UserStateListCall<'a, C, A> {
        self._request_metadata_user_overrides_ip_address = Some(new_value.to_string());
        self
    }
    /// Second level identifier to indicate where the traffic comes from. An identifier has multiple letters created by a team which redirected the traffic to us.
    ///
    /// Sets the *request metadata.traffic source.traffic sub id* query property to the given value.
    pub fn request_metadata_traffic_source_traffic_sub_id(mut self, new_value: &str) -> UserStateListCall<'a, C, A> {
        self._request_metadata_traffic_source_traffic_sub_id = Some(new_value.to_string());
        self
    }
    /// Identifier to indicate where the traffic comes from. An identifier has multiple letters created by a team which redirected the traffic to us.
    ///
    /// Sets the *request metadata.traffic source.traffic source id* query property to the given value.
    pub fn request_metadata_traffic_source_traffic_source_id(mut self, new_value: &str) -> UserStateListCall<'a, C, A> {
        self._request_metadata_traffic_source_traffic_source_id = Some(new_value.to_string());
        self
    }
    /// Google Partners session ID.
    ///
    /// Sets the *request metadata.partners session id* query property to the given value.
    pub fn request_metadata_partners_session_id(mut self, new_value: &str) -> UserStateListCall<'a, C, A> {
        self._request_metadata_partners_session_id = Some(new_value.to_string());
        self
    }
    /// Locale to use for the current request.
    ///
    /// Sets the *request metadata.locale* query property to the given value.
    pub fn request_metadata_locale(mut self, new_value: &str) -> UserStateListCall<'a, C, A> {
        self._request_metadata_locale = Some(new_value.to_string());
        self
    }
    /// Experiment IDs the current request belongs to.
    ///
    /// Append the given value to the *request metadata.experiment ids* query property.
    /// Each appended value will retain its original ordering and be '/'-separated in the URL's parameters.
    pub fn add_request_metadata_experiment_ids(mut self, new_value: &str) -> UserStateListCall<'a, C, A> {
        self._request_metadata_experiment_ids.push(new_value.to_string());
        self
    }
    /// The delegate implementation is consulted whenever there is an intermediate result, or if something goes wrong
    /// while executing the actual API request.
    /// 
    /// It should be used to handle progress information, and to implement a certain level of resilience.
    ///
    /// Sets the *delegate* property to the given value.
    pub fn delegate(mut self, new_value: &'a mut Delegate) -> UserStateListCall<'a, C, A> {
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
    /// * *bearer_token* (query-string) - OAuth bearer token.
    /// * *pp* (query-boolean) - Pretty-print response.
    /// * *prettyPrint* (query-boolean) - Returns response with indentations and line breaks.
    /// * *upload_protocol* (query-string) - Upload protocol for media (e.g. "raw", "multipart").
    /// * *access_token* (query-string) - OAuth access token.
    /// * *fields* (query-string) - Selector specifying which fields to include in a partial response.
    /// * *quotaUser* (query-string) - Available to use for quota purposes for server-side applications. Can be any arbitrary string assigned to a user, but should not exceed 40 characters.
    /// * *callback* (query-string) - JSONP
    /// * *oauth_token* (query-string) - OAuth 2.0 token for the current user.
    /// * *key* (query-string) - API key. Your API key identifies your project and provides you with API access, quota, and reports. Required unless you provide an OAuth 2.0 token.
    /// * *uploadType* (query-string) - Legacy upload protocol for media (e.g. "media", "multipart").
    /// * *alt* (query-string) - Data format for response.
    /// * *$.xgafv* (query-string) - V1 error format.
    pub fn param<T>(mut self, name: T, value: T) -> UserStateListCall<'a, C, A>
                                                        where T: AsRef<str> {
        self._additional_params.insert(name.as_ref().to_string(), value.as_ref().to_string());
        self
    }

}



