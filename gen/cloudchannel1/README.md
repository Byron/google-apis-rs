<!---
DO NOT EDIT !
This file was generated automatically from 'src/generator/templates/api/README.md.mako'
DO NOT EDIT !
-->
The `google-cloudchannel1` library allows access to all features of the *Google Cloudchannel* service.

This documentation was generated from *Cloudchannel* crate version *5.0.5+20240625*, where *20240625* is the exact revision of the *cloudchannel:v1* schema built by the [mako](http://www.makotemplates.org/) code generator *v5.0.5*.

Everything else about the *Cloudchannel* *v1* API can be found at the
[official documentation site](https://cloud.google.com/channel).
# Features

Handle the following *Resources* with ease from the central [hub](https://docs.rs/google-cloudchannel1/5.0.5+20240625/google_cloudchannel1/Cloudchannel) ... 

* accounts
 * [*channel partner links channel partner repricing configs create*](https://docs.rs/google-cloudchannel1/5.0.5+20240625/google_cloudchannel1/api::AccountChannelPartnerLinkChannelPartnerRepricingConfigCreateCall), [*channel partner links channel partner repricing configs delete*](https://docs.rs/google-cloudchannel1/5.0.5+20240625/google_cloudchannel1/api::AccountChannelPartnerLinkChannelPartnerRepricingConfigDeleteCall), [*channel partner links channel partner repricing configs get*](https://docs.rs/google-cloudchannel1/5.0.5+20240625/google_cloudchannel1/api::AccountChannelPartnerLinkChannelPartnerRepricingConfigGetCall), [*channel partner links channel partner repricing configs list*](https://docs.rs/google-cloudchannel1/5.0.5+20240625/google_cloudchannel1/api::AccountChannelPartnerLinkChannelPartnerRepricingConfigListCall), [*channel partner links channel partner repricing configs patch*](https://docs.rs/google-cloudchannel1/5.0.5+20240625/google_cloudchannel1/api::AccountChannelPartnerLinkChannelPartnerRepricingConfigPatchCall), [*channel partner links create*](https://docs.rs/google-cloudchannel1/5.0.5+20240625/google_cloudchannel1/api::AccountChannelPartnerLinkCreateCall), [*channel partner links customers create*](https://docs.rs/google-cloudchannel1/5.0.5+20240625/google_cloudchannel1/api::AccountChannelPartnerLinkCustomerCreateCall), [*channel partner links customers delete*](https://docs.rs/google-cloudchannel1/5.0.5+20240625/google_cloudchannel1/api::AccountChannelPartnerLinkCustomerDeleteCall), [*channel partner links customers get*](https://docs.rs/google-cloudchannel1/5.0.5+20240625/google_cloudchannel1/api::AccountChannelPartnerLinkCustomerGetCall), [*channel partner links customers import*](https://docs.rs/google-cloudchannel1/5.0.5+20240625/google_cloudchannel1/api::AccountChannelPartnerLinkCustomerImportCall), [*channel partner links customers list*](https://docs.rs/google-cloudchannel1/5.0.5+20240625/google_cloudchannel1/api::AccountChannelPartnerLinkCustomerListCall), [*channel partner links customers patch*](https://docs.rs/google-cloudchannel1/5.0.5+20240625/google_cloudchannel1/api::AccountChannelPartnerLinkCustomerPatchCall), [*channel partner links get*](https://docs.rs/google-cloudchannel1/5.0.5+20240625/google_cloudchannel1/api::AccountChannelPartnerLinkGetCall), [*channel partner links list*](https://docs.rs/google-cloudchannel1/5.0.5+20240625/google_cloudchannel1/api::AccountChannelPartnerLinkListCall), [*channel partner links patch*](https://docs.rs/google-cloudchannel1/5.0.5+20240625/google_cloudchannel1/api::AccountChannelPartnerLinkPatchCall), [*check cloud identity accounts exist*](https://docs.rs/google-cloudchannel1/5.0.5+20240625/google_cloudchannel1/api::AccountCheckCloudIdentityAccountsExistCall), [*customers create*](https://docs.rs/google-cloudchannel1/5.0.5+20240625/google_cloudchannel1/api::AccountCustomerCreateCall), [*customers customer repricing configs create*](https://docs.rs/google-cloudchannel1/5.0.5+20240625/google_cloudchannel1/api::AccountCustomerCustomerRepricingConfigCreateCall), [*customers customer repricing configs delete*](https://docs.rs/google-cloudchannel1/5.0.5+20240625/google_cloudchannel1/api::AccountCustomerCustomerRepricingConfigDeleteCall), [*customers customer repricing configs get*](https://docs.rs/google-cloudchannel1/5.0.5+20240625/google_cloudchannel1/api::AccountCustomerCustomerRepricingConfigGetCall), [*customers customer repricing configs list*](https://docs.rs/google-cloudchannel1/5.0.5+20240625/google_cloudchannel1/api::AccountCustomerCustomerRepricingConfigListCall), [*customers customer repricing configs patch*](https://docs.rs/google-cloudchannel1/5.0.5+20240625/google_cloudchannel1/api::AccountCustomerCustomerRepricingConfigPatchCall), [*customers delete*](https://docs.rs/google-cloudchannel1/5.0.5+20240625/google_cloudchannel1/api::AccountCustomerDeleteCall), [*customers entitlements activate*](https://docs.rs/google-cloudchannel1/5.0.5+20240625/google_cloudchannel1/api::AccountCustomerEntitlementActivateCall), [*customers entitlements cancel*](https://docs.rs/google-cloudchannel1/5.0.5+20240625/google_cloudchannel1/api::AccountCustomerEntitlementCancelCall), [*customers entitlements change offer*](https://docs.rs/google-cloudchannel1/5.0.5+20240625/google_cloudchannel1/api::AccountCustomerEntitlementChangeOfferCall), [*customers entitlements change parameters*](https://docs.rs/google-cloudchannel1/5.0.5+20240625/google_cloudchannel1/api::AccountCustomerEntitlementChangeParameterCall), [*customers entitlements change renewal settings*](https://docs.rs/google-cloudchannel1/5.0.5+20240625/google_cloudchannel1/api::AccountCustomerEntitlementChangeRenewalSettingCall), [*customers entitlements create*](https://docs.rs/google-cloudchannel1/5.0.5+20240625/google_cloudchannel1/api::AccountCustomerEntitlementCreateCall), [*customers entitlements get*](https://docs.rs/google-cloudchannel1/5.0.5+20240625/google_cloudchannel1/api::AccountCustomerEntitlementGetCall), [*customers entitlements list*](https://docs.rs/google-cloudchannel1/5.0.5+20240625/google_cloudchannel1/api::AccountCustomerEntitlementListCall), [*customers entitlements list entitlement changes*](https://docs.rs/google-cloudchannel1/5.0.5+20240625/google_cloudchannel1/api::AccountCustomerEntitlementListEntitlementChangeCall), [*customers entitlements lookup offer*](https://docs.rs/google-cloudchannel1/5.0.5+20240625/google_cloudchannel1/api::AccountCustomerEntitlementLookupOfferCall), [*customers entitlements start paid service*](https://docs.rs/google-cloudchannel1/5.0.5+20240625/google_cloudchannel1/api::AccountCustomerEntitlementStartPaidServiceCall), [*customers entitlements suspend*](https://docs.rs/google-cloudchannel1/5.0.5+20240625/google_cloudchannel1/api::AccountCustomerEntitlementSuspendCall), [*customers get*](https://docs.rs/google-cloudchannel1/5.0.5+20240625/google_cloudchannel1/api::AccountCustomerGetCall), [*customers import*](https://docs.rs/google-cloudchannel1/5.0.5+20240625/google_cloudchannel1/api::AccountCustomerImportCall), [*customers list*](https://docs.rs/google-cloudchannel1/5.0.5+20240625/google_cloudchannel1/api::AccountCustomerListCall), [*customers list purchasable offers*](https://docs.rs/google-cloudchannel1/5.0.5+20240625/google_cloudchannel1/api::AccountCustomerListPurchasableOfferCall), [*customers list purchasable skus*](https://docs.rs/google-cloudchannel1/5.0.5+20240625/google_cloudchannel1/api::AccountCustomerListPurchasableSkuCall), [*customers patch*](https://docs.rs/google-cloudchannel1/5.0.5+20240625/google_cloudchannel1/api::AccountCustomerPatchCall), [*customers provision cloud identity*](https://docs.rs/google-cloudchannel1/5.0.5+20240625/google_cloudchannel1/api::AccountCustomerProvisionCloudIdentityCall), [*customers query eligible billing accounts*](https://docs.rs/google-cloudchannel1/5.0.5+20240625/google_cloudchannel1/api::AccountCustomerQueryEligibleBillingAccountCall), [*customers transfer entitlements*](https://docs.rs/google-cloudchannel1/5.0.5+20240625/google_cloudchannel1/api::AccountCustomerTransferEntitlementCall), [*customers transfer entitlements to google*](https://docs.rs/google-cloudchannel1/5.0.5+20240625/google_cloudchannel1/api::AccountCustomerTransferEntitlementsToGoogleCall), [*list subscribers*](https://docs.rs/google-cloudchannel1/5.0.5+20240625/google_cloudchannel1/api::AccountListSubscriberCall), [*list transferable offers*](https://docs.rs/google-cloudchannel1/5.0.5+20240625/google_cloudchannel1/api::AccountListTransferableOfferCall), [*list transferable skus*](https://docs.rs/google-cloudchannel1/5.0.5+20240625/google_cloudchannel1/api::AccountListTransferableSkuCall), [*offers list*](https://docs.rs/google-cloudchannel1/5.0.5+20240625/google_cloudchannel1/api::AccountOfferListCall), [*register*](https://docs.rs/google-cloudchannel1/5.0.5+20240625/google_cloudchannel1/api::AccountRegisterCall), [*report jobs fetch report results*](https://docs.rs/google-cloudchannel1/5.0.5+20240625/google_cloudchannel1/api::AccountReportJobFetchReportResultCall), [*reports list*](https://docs.rs/google-cloudchannel1/5.0.5+20240625/google_cloudchannel1/api::AccountReportListCall), [*reports run*](https://docs.rs/google-cloudchannel1/5.0.5+20240625/google_cloudchannel1/api::AccountReportRunCall), [*sku groups billable skus list*](https://docs.rs/google-cloudchannel1/5.0.5+20240625/google_cloudchannel1/api::AccountSkuGroupBillableSkuListCall), [*sku groups list*](https://docs.rs/google-cloudchannel1/5.0.5+20240625/google_cloudchannel1/api::AccountSkuGroupListCall) and [*unregister*](https://docs.rs/google-cloudchannel1/5.0.5+20240625/google_cloudchannel1/api::AccountUnregisterCall)
* operations
 * [*cancel*](https://docs.rs/google-cloudchannel1/5.0.5+20240625/google_cloudchannel1/api::OperationCancelCall), [*delete*](https://docs.rs/google-cloudchannel1/5.0.5+20240625/google_cloudchannel1/api::OperationDeleteCall), [*get*](https://docs.rs/google-cloudchannel1/5.0.5+20240625/google_cloudchannel1/api::OperationGetCall) and [*list*](https://docs.rs/google-cloudchannel1/5.0.5+20240625/google_cloudchannel1/api::OperationListCall)
* products
 * [*list*](https://docs.rs/google-cloudchannel1/5.0.5+20240625/google_cloudchannel1/api::ProductListCall) and [*skus list*](https://docs.rs/google-cloudchannel1/5.0.5+20240625/google_cloudchannel1/api::ProductSkuListCall)




# Structure of this Library

The API is structured into the following primary items:

* **[Hub](https://docs.rs/google-cloudchannel1/5.0.5+20240625/google_cloudchannel1/Cloudchannel)**
    * a central object to maintain state and allow accessing all *Activities*
    * creates [*Method Builders*](https://docs.rs/google-cloudchannel1/5.0.5+20240625/google_cloudchannel1/client::MethodsBuilder) which in turn
      allow access to individual [*Call Builders*](https://docs.rs/google-cloudchannel1/5.0.5+20240625/google_cloudchannel1/client::CallBuilder)
* **[Resources](https://docs.rs/google-cloudchannel1/5.0.5+20240625/google_cloudchannel1/client::Resource)**
    * primary types that you can apply *Activities* to
    * a collection of properties and *Parts*
    * **[Parts](https://docs.rs/google-cloudchannel1/5.0.5+20240625/google_cloudchannel1/client::Part)**
        * a collection of properties
        * never directly used in *Activities*
* **[Activities](https://docs.rs/google-cloudchannel1/5.0.5+20240625/google_cloudchannel1/client::CallBuilder)**
    * operations to apply to *Resources*

All *structures* are marked with applicable traits to further categorize them and ease browsing.

Generally speaking, you can invoke *Activities* like this:

```Rust,ignore
let r = hub.resource().activity(...).doit().await
```

Or specifically ...

```ignore
let r = hub.accounts().customers_entitlements_activate(...).doit().await
let r = hub.accounts().customers_entitlements_cancel(...).doit().await
let r = hub.accounts().customers_entitlements_change_offer(...).doit().await
let r = hub.accounts().customers_entitlements_change_parameters(...).doit().await
let r = hub.accounts().customers_entitlements_change_renewal_settings(...).doit().await
let r = hub.accounts().customers_entitlements_create(...).doit().await
let r = hub.accounts().customers_entitlements_start_paid_service(...).doit().await
let r = hub.accounts().customers_entitlements_suspend(...).doit().await
let r = hub.accounts().customers_provision_cloud_identity(...).doit().await
let r = hub.accounts().customers_transfer_entitlements(...).doit().await
let r = hub.accounts().customers_transfer_entitlements_to_google(...).doit().await
let r = hub.accounts().reports_run(...).doit().await
let r = hub.operations().get(...).doit().await
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
google-cloudchannel1 = "*"
serde = "^1.0"
serde_json = "^1.0"
```

## A complete example

```Rust
extern crate hyper;
extern crate hyper_rustls;
extern crate google_cloudchannel1 as cloudchannel1;
use cloudchannel1::api::GoogleCloudChannelV1ActivateEntitlementRequest;
use cloudchannel1::{Result, Error};
use std::default::Default;
use cloudchannel1::{Cloudchannel, oauth2, hyper, hyper_rustls, chrono, FieldMask};

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
let mut hub = Cloudchannel::new(hyper::Client::builder().build(hyper_rustls::HttpsConnectorBuilder::new().with_native_roots().unwrap().https_or_http().enable_http1().build()), auth);
// As the method needs a request, you would usually fill it with the desired information
// into the respective structure. Some of the parts shown here might not be applicable !
// Values shown here are possibly random and not representative !
let mut req = GoogleCloudChannelV1ActivateEntitlementRequest::default();

// You can configure optional parameters by calling the respective setters at will, and
// execute the final call using `doit()`.
// Values shown here are possibly random and not representative !
let result = hub.accounts().customers_entitlements_activate(req, "name")
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

All errors produced by the system are provided either as [Result](https://docs.rs/google-cloudchannel1/5.0.5+20240625/google_cloudchannel1/client::Result) enumeration as return value of
the doit() methods, or handed as possibly intermediate results to either the 
[Hub Delegate](https://docs.rs/google-cloudchannel1/5.0.5+20240625/google_cloudchannel1/client::Delegate), or the [Authenticator Delegate](https://docs.rs/yup-oauth2/*/yup_oauth2/trait.AuthenticatorDelegate.html).

When delegates handle errors or intermediate values, they may have a chance to instruct the system to retry. This 
makes the system potentially resilient to all kinds of errors.

## Uploads and Downloads
If a method supports downloads, the response body, which is part of the [Result](https://docs.rs/google-cloudchannel1/5.0.5+20240625/google_cloudchannel1/client::Result), should be
read by you to obtain the media.
If such a method also supports a [Response Result](https://docs.rs/google-cloudchannel1/5.0.5+20240625/google_cloudchannel1/client::ResponseResult), it will return that by default.
You can see it as meta-data for the actual media. To trigger a media download, you will have to set up the builder by making
this call: `.param("alt", "media")`.

Methods supporting uploads can do so using up to 2 different protocols: 
*simple* and *resumable*. The distinctiveness of each is represented by customized 
`doit(...)` methods, which are then named `upload(...)` and `upload_resumable(...)` respectively.

## Customization and Callbacks

You may alter the way an `doit()` method is called by providing a [delegate](https://docs.rs/google-cloudchannel1/5.0.5+20240625/google_cloudchannel1/client::Delegate) to the 
[Method Builder](https://docs.rs/google-cloudchannel1/5.0.5+20240625/google_cloudchannel1/client::CallBuilder) before making the final `doit()` call. 
Respective methods will be called to provide progress information, as well as determine whether the system should 
retry on failure.

The [delegate trait](https://docs.rs/google-cloudchannel1/5.0.5+20240625/google_cloudchannel1/client::Delegate) is default-implemented, allowing you to customize it with minimal effort.

## Optional Parts in Server-Requests

All structures provided by this library are made to be [encodable](https://docs.rs/google-cloudchannel1/5.0.5+20240625/google_cloudchannel1/client::RequestValue) and 
[decodable](https://docs.rs/google-cloudchannel1/5.0.5+20240625/google_cloudchannel1/client::ResponseResult) via *json*. Optionals are used to indicate that partial requests are responses 
are valid.
Most optionals are are considered [Parts](https://docs.rs/google-cloudchannel1/5.0.5+20240625/google_cloudchannel1/client::Part) which are identifiable by name, which will be sent to 
the server to indicate either the set parts of the request or the desired parts in the response.

## Builder Arguments

Using [method builders](https://docs.rs/google-cloudchannel1/5.0.5+20240625/google_cloudchannel1/client::CallBuilder), you are able to prepare an action call by repeatedly calling it's methods.
These will always take a single argument, for which the following statements are true.

* [PODs][wiki-pod] are handed by copy
* strings are passed as `&str`
* [request values](https://docs.rs/google-cloudchannel1/5.0.5+20240625/google_cloudchannel1/client::RequestValue) are moved

Arguments will always be copied or cloned into the builder, to make them independent of their original life times.

[wiki-pod]: http://en.wikipedia.org/wiki/Plain_old_data_structure
[builder-pattern]: http://en.wikipedia.org/wiki/Builder_pattern
[google-go-api]: https://github.com/google/google-api-go-client

## Cargo Features

* `utoipa` - Add support for [utoipa](https://crates.io/crates/utoipa) and derive `utoipa::ToSchema` on all
the types. You'll have to import and register the required types in `#[openapi(schemas(...))]`, otherwise the
generated `openapi` spec would be invalid.


# License
The **cloudchannel1** library was generated by Sebastian Thiel, and is placed 
under the *MIT* license.
You can read the full text at the repository's [license file][repo-license].

[repo-license]: https://github.com/Byron/google-apis-rsblob/main/LICENSE.md

