<!---
DO NOT EDIT !
This file was generated automatically from 'src/generator/templates/api/README.md.mako'
DO NOT EDIT !
-->
The `google-androidpublisher3` library allows access to all features of the *Google Android Publisher* service.

This documentation was generated from *Android Publisher* crate version *5.0.5+20240626*, where *20240626* is the exact revision of the *androidpublisher:v3* schema built by the [mako](http://www.makotemplates.org/) code generator *v5.0.5*.

Everything else about the *Android Publisher* *v3* API can be found at the
[official documentation site](https://developers.google.com/android-publisher).
# Features

Handle the following *Resources* with ease from the central [hub](https://docs.rs/google-androidpublisher3/5.0.5+20240626/google_androidpublisher3/AndroidPublisher) ... 

* applications
 * [*data safety*](https://docs.rs/google-androidpublisher3/5.0.5+20240626/google_androidpublisher3/api::ApplicationDataSafetyCall), [*device tier configs create*](https://docs.rs/google-androidpublisher3/5.0.5+20240626/google_androidpublisher3/api::ApplicationDeviceTierConfigCreateCall), [*device tier configs get*](https://docs.rs/google-androidpublisher3/5.0.5+20240626/google_androidpublisher3/api::ApplicationDeviceTierConfigGetCall) and [*device tier configs list*](https://docs.rs/google-androidpublisher3/5.0.5+20240626/google_androidpublisher3/api::ApplicationDeviceTierConfigListCall)
* apprecovery
 * [*add targeting*](https://docs.rs/google-androidpublisher3/5.0.5+20240626/google_androidpublisher3/api::ApprecoveryAddTargetingCall), [*app recoveries*](https://docs.rs/google-androidpublisher3/5.0.5+20240626/google_androidpublisher3/api::ApprecoveryAppRecoveryCall), [*cancel*](https://docs.rs/google-androidpublisher3/5.0.5+20240626/google_androidpublisher3/api::ApprecoveryCancelCall), [*create*](https://docs.rs/google-androidpublisher3/5.0.5+20240626/google_androidpublisher3/api::ApprecoveryCreateCall) and [*deploy*](https://docs.rs/google-androidpublisher3/5.0.5+20240626/google_androidpublisher3/api::ApprecoveryDeployCall)
* edits
 * [*apks addexternallyhosted*](https://docs.rs/google-androidpublisher3/5.0.5+20240626/google_androidpublisher3/api::EditApkAddexternallyhostedCall), [*apks list*](https://docs.rs/google-androidpublisher3/5.0.5+20240626/google_androidpublisher3/api::EditApkListCall), [*apks upload*](https://docs.rs/google-androidpublisher3/5.0.5+20240626/google_androidpublisher3/api::EditApkUploadCall), [*bundles list*](https://docs.rs/google-androidpublisher3/5.0.5+20240626/google_androidpublisher3/api::EditBundleListCall), [*bundles upload*](https://docs.rs/google-androidpublisher3/5.0.5+20240626/google_androidpublisher3/api::EditBundleUploadCall), [*commit*](https://docs.rs/google-androidpublisher3/5.0.5+20240626/google_androidpublisher3/api::EditCommitCall), [*countryavailability get*](https://docs.rs/google-androidpublisher3/5.0.5+20240626/google_androidpublisher3/api::EditCountryavailabilityGetCall), [*delete*](https://docs.rs/google-androidpublisher3/5.0.5+20240626/google_androidpublisher3/api::EditDeleteCall), [*deobfuscationfiles upload*](https://docs.rs/google-androidpublisher3/5.0.5+20240626/google_androidpublisher3/api::EditDeobfuscationfileUploadCall), [*details get*](https://docs.rs/google-androidpublisher3/5.0.5+20240626/google_androidpublisher3/api::EditDetailGetCall), [*details patch*](https://docs.rs/google-androidpublisher3/5.0.5+20240626/google_androidpublisher3/api::EditDetailPatchCall), [*details update*](https://docs.rs/google-androidpublisher3/5.0.5+20240626/google_androidpublisher3/api::EditDetailUpdateCall), [*expansionfiles get*](https://docs.rs/google-androidpublisher3/5.0.5+20240626/google_androidpublisher3/api::EditExpansionfileGetCall), [*expansionfiles patch*](https://docs.rs/google-androidpublisher3/5.0.5+20240626/google_androidpublisher3/api::EditExpansionfilePatchCall), [*expansionfiles update*](https://docs.rs/google-androidpublisher3/5.0.5+20240626/google_androidpublisher3/api::EditExpansionfileUpdateCall), [*expansionfiles upload*](https://docs.rs/google-androidpublisher3/5.0.5+20240626/google_androidpublisher3/api::EditExpansionfileUploadCall), [*get*](https://docs.rs/google-androidpublisher3/5.0.5+20240626/google_androidpublisher3/api::EditGetCall), [*images delete*](https://docs.rs/google-androidpublisher3/5.0.5+20240626/google_androidpublisher3/api::EditImageDeleteCall), [*images deleteall*](https://docs.rs/google-androidpublisher3/5.0.5+20240626/google_androidpublisher3/api::EditImageDeleteallCall), [*images list*](https://docs.rs/google-androidpublisher3/5.0.5+20240626/google_androidpublisher3/api::EditImageListCall), [*images upload*](https://docs.rs/google-androidpublisher3/5.0.5+20240626/google_androidpublisher3/api::EditImageUploadCall), [*insert*](https://docs.rs/google-androidpublisher3/5.0.5+20240626/google_androidpublisher3/api::EditInsertCall), [*listings delete*](https://docs.rs/google-androidpublisher3/5.0.5+20240626/google_androidpublisher3/api::EditListingDeleteCall), [*listings deleteall*](https://docs.rs/google-androidpublisher3/5.0.5+20240626/google_androidpublisher3/api::EditListingDeleteallCall), [*listings get*](https://docs.rs/google-androidpublisher3/5.0.5+20240626/google_androidpublisher3/api::EditListingGetCall), [*listings list*](https://docs.rs/google-androidpublisher3/5.0.5+20240626/google_androidpublisher3/api::EditListingListCall), [*listings patch*](https://docs.rs/google-androidpublisher3/5.0.5+20240626/google_androidpublisher3/api::EditListingPatchCall), [*listings update*](https://docs.rs/google-androidpublisher3/5.0.5+20240626/google_androidpublisher3/api::EditListingUpdateCall), [*testers get*](https://docs.rs/google-androidpublisher3/5.0.5+20240626/google_androidpublisher3/api::EditTesterGetCall), [*testers patch*](https://docs.rs/google-androidpublisher3/5.0.5+20240626/google_androidpublisher3/api::EditTesterPatchCall), [*testers update*](https://docs.rs/google-androidpublisher3/5.0.5+20240626/google_androidpublisher3/api::EditTesterUpdateCall), [*tracks create*](https://docs.rs/google-androidpublisher3/5.0.5+20240626/google_androidpublisher3/api::EditTrackCreateCall), [*tracks get*](https://docs.rs/google-androidpublisher3/5.0.5+20240626/google_androidpublisher3/api::EditTrackGetCall), [*tracks list*](https://docs.rs/google-androidpublisher3/5.0.5+20240626/google_androidpublisher3/api::EditTrackListCall), [*tracks patch*](https://docs.rs/google-androidpublisher3/5.0.5+20240626/google_androidpublisher3/api::EditTrackPatchCall), [*tracks update*](https://docs.rs/google-androidpublisher3/5.0.5+20240626/google_androidpublisher3/api::EditTrackUpdateCall) and [*validate*](https://docs.rs/google-androidpublisher3/5.0.5+20240626/google_androidpublisher3/api::EditValidateCall)
* externaltransactions
 * [*createexternaltransaction*](https://docs.rs/google-androidpublisher3/5.0.5+20240626/google_androidpublisher3/api::ExternaltransactionCreateexternaltransactionCall), [*getexternaltransaction*](https://docs.rs/google-androidpublisher3/5.0.5+20240626/google_androidpublisher3/api::ExternaltransactionGetexternaltransactionCall) and [*refundexternaltransaction*](https://docs.rs/google-androidpublisher3/5.0.5+20240626/google_androidpublisher3/api::ExternaltransactionRefundexternaltransactionCall)
* generatedapks
 * [*download*](https://docs.rs/google-androidpublisher3/5.0.5+20240626/google_androidpublisher3/api::GeneratedapkDownloadCall) and [*list*](https://docs.rs/google-androidpublisher3/5.0.5+20240626/google_androidpublisher3/api::GeneratedapkListCall)
* [grants](https://docs.rs/google-androidpublisher3/5.0.5+20240626/google_androidpublisher3/api::Grant)
 * [*create*](https://docs.rs/google-androidpublisher3/5.0.5+20240626/google_androidpublisher3/api::GrantCreateCall), [*delete*](https://docs.rs/google-androidpublisher3/5.0.5+20240626/google_androidpublisher3/api::GrantDeleteCall) and [*patch*](https://docs.rs/google-androidpublisher3/5.0.5+20240626/google_androidpublisher3/api::GrantPatchCall)
* inappproducts
 * [*batch delete*](https://docs.rs/google-androidpublisher3/5.0.5+20240626/google_androidpublisher3/api::InappproductBatchDeleteCall), [*batch get*](https://docs.rs/google-androidpublisher3/5.0.5+20240626/google_androidpublisher3/api::InappproductBatchGetCall), [*batch update*](https://docs.rs/google-androidpublisher3/5.0.5+20240626/google_androidpublisher3/api::InappproductBatchUpdateCall), [*delete*](https://docs.rs/google-androidpublisher3/5.0.5+20240626/google_androidpublisher3/api::InappproductDeleteCall), [*get*](https://docs.rs/google-androidpublisher3/5.0.5+20240626/google_androidpublisher3/api::InappproductGetCall), [*insert*](https://docs.rs/google-androidpublisher3/5.0.5+20240626/google_androidpublisher3/api::InappproductInsertCall), [*list*](https://docs.rs/google-androidpublisher3/5.0.5+20240626/google_androidpublisher3/api::InappproductListCall), [*patch*](https://docs.rs/google-androidpublisher3/5.0.5+20240626/google_androidpublisher3/api::InappproductPatchCall) and [*update*](https://docs.rs/google-androidpublisher3/5.0.5+20240626/google_androidpublisher3/api::InappproductUpdateCall)
* internalappsharingartifacts
 * [*uploadapk*](https://docs.rs/google-androidpublisher3/5.0.5+20240626/google_androidpublisher3/api::InternalappsharingartifactUploadapkCall) and [*uploadbundle*](https://docs.rs/google-androidpublisher3/5.0.5+20240626/google_androidpublisher3/api::InternalappsharingartifactUploadbundleCall)
* monetization
 * [*convert region prices*](https://docs.rs/google-androidpublisher3/5.0.5+20240626/google_androidpublisher3/api::MonetizationConvertRegionPriceCall), [*subscriptions archive*](https://docs.rs/google-androidpublisher3/5.0.5+20240626/google_androidpublisher3/api::MonetizationSubscriptionArchiveCall), [*subscriptions base plans activate*](https://docs.rs/google-androidpublisher3/5.0.5+20240626/google_androidpublisher3/api::MonetizationSubscriptionBasePlanActivateCall), [*subscriptions base plans batch migrate prices*](https://docs.rs/google-androidpublisher3/5.0.5+20240626/google_androidpublisher3/api::MonetizationSubscriptionBasePlanBatchMigratePriceCall), [*subscriptions base plans batch update states*](https://docs.rs/google-androidpublisher3/5.0.5+20240626/google_androidpublisher3/api::MonetizationSubscriptionBasePlanBatchUpdateStateCall), [*subscriptions base plans deactivate*](https://docs.rs/google-androidpublisher3/5.0.5+20240626/google_androidpublisher3/api::MonetizationSubscriptionBasePlanDeactivateCall), [*subscriptions base plans delete*](https://docs.rs/google-androidpublisher3/5.0.5+20240626/google_androidpublisher3/api::MonetizationSubscriptionBasePlanDeleteCall), [*subscriptions base plans migrate prices*](https://docs.rs/google-androidpublisher3/5.0.5+20240626/google_androidpublisher3/api::MonetizationSubscriptionBasePlanMigratePriceCall), [*subscriptions base plans offers activate*](https://docs.rs/google-androidpublisher3/5.0.5+20240626/google_androidpublisher3/api::MonetizationSubscriptionBasePlanOfferActivateCall), [*subscriptions base plans offers batch get*](https://docs.rs/google-androidpublisher3/5.0.5+20240626/google_androidpublisher3/api::MonetizationSubscriptionBasePlanOfferBatchGetCall), [*subscriptions base plans offers batch update*](https://docs.rs/google-androidpublisher3/5.0.5+20240626/google_androidpublisher3/api::MonetizationSubscriptionBasePlanOfferBatchUpdateCall), [*subscriptions base plans offers batch update states*](https://docs.rs/google-androidpublisher3/5.0.5+20240626/google_androidpublisher3/api::MonetizationSubscriptionBasePlanOfferBatchUpdateStateCall), [*subscriptions base plans offers create*](https://docs.rs/google-androidpublisher3/5.0.5+20240626/google_androidpublisher3/api::MonetizationSubscriptionBasePlanOfferCreateCall), [*subscriptions base plans offers deactivate*](https://docs.rs/google-androidpublisher3/5.0.5+20240626/google_androidpublisher3/api::MonetizationSubscriptionBasePlanOfferDeactivateCall), [*subscriptions base plans offers delete*](https://docs.rs/google-androidpublisher3/5.0.5+20240626/google_androidpublisher3/api::MonetizationSubscriptionBasePlanOfferDeleteCall), [*subscriptions base plans offers get*](https://docs.rs/google-androidpublisher3/5.0.5+20240626/google_androidpublisher3/api::MonetizationSubscriptionBasePlanOfferGetCall), [*subscriptions base plans offers list*](https://docs.rs/google-androidpublisher3/5.0.5+20240626/google_androidpublisher3/api::MonetizationSubscriptionBasePlanOfferListCall), [*subscriptions base plans offers patch*](https://docs.rs/google-androidpublisher3/5.0.5+20240626/google_androidpublisher3/api::MonetizationSubscriptionBasePlanOfferPatchCall), [*subscriptions batch get*](https://docs.rs/google-androidpublisher3/5.0.5+20240626/google_androidpublisher3/api::MonetizationSubscriptionBatchGetCall), [*subscriptions batch update*](https://docs.rs/google-androidpublisher3/5.0.5+20240626/google_androidpublisher3/api::MonetizationSubscriptionBatchUpdateCall), [*subscriptions create*](https://docs.rs/google-androidpublisher3/5.0.5+20240626/google_androidpublisher3/api::MonetizationSubscriptionCreateCall), [*subscriptions delete*](https://docs.rs/google-androidpublisher3/5.0.5+20240626/google_androidpublisher3/api::MonetizationSubscriptionDeleteCall), [*subscriptions get*](https://docs.rs/google-androidpublisher3/5.0.5+20240626/google_androidpublisher3/api::MonetizationSubscriptionGetCall), [*subscriptions list*](https://docs.rs/google-androidpublisher3/5.0.5+20240626/google_androidpublisher3/api::MonetizationSubscriptionListCall) and [*subscriptions patch*](https://docs.rs/google-androidpublisher3/5.0.5+20240626/google_androidpublisher3/api::MonetizationSubscriptionPatchCall)
* orders
 * [*refund*](https://docs.rs/google-androidpublisher3/5.0.5+20240626/google_androidpublisher3/api::OrderRefundCall)
* purchases
 * [*products acknowledge*](https://docs.rs/google-androidpublisher3/5.0.5+20240626/google_androidpublisher3/api::PurchaseProductAcknowledgeCall), [*products consume*](https://docs.rs/google-androidpublisher3/5.0.5+20240626/google_androidpublisher3/api::PurchaseProductConsumeCall), [*products get*](https://docs.rs/google-androidpublisher3/5.0.5+20240626/google_androidpublisher3/api::PurchaseProductGetCall), [*subscriptions acknowledge*](https://docs.rs/google-androidpublisher3/5.0.5+20240626/google_androidpublisher3/api::PurchaseSubscriptionAcknowledgeCall), [*subscriptions cancel*](https://docs.rs/google-androidpublisher3/5.0.5+20240626/google_androidpublisher3/api::PurchaseSubscriptionCancelCall), [*subscriptions defer*](https://docs.rs/google-androidpublisher3/5.0.5+20240626/google_androidpublisher3/api::PurchaseSubscriptionDeferCall), [*subscriptions get*](https://docs.rs/google-androidpublisher3/5.0.5+20240626/google_androidpublisher3/api::PurchaseSubscriptionGetCall), [*subscriptions refund*](https://docs.rs/google-androidpublisher3/5.0.5+20240626/google_androidpublisher3/api::PurchaseSubscriptionRefundCall), [*subscriptions revoke*](https://docs.rs/google-androidpublisher3/5.0.5+20240626/google_androidpublisher3/api::PurchaseSubscriptionRevokeCall), [*subscriptionsv2 get*](https://docs.rs/google-androidpublisher3/5.0.5+20240626/google_androidpublisher3/api::PurchaseSubscriptionsv2GetCall), [*subscriptionsv2 revoke*](https://docs.rs/google-androidpublisher3/5.0.5+20240626/google_androidpublisher3/api::PurchaseSubscriptionsv2RevokeCall) and [*voidedpurchases list*](https://docs.rs/google-androidpublisher3/5.0.5+20240626/google_androidpublisher3/api::PurchaseVoidedpurchaseListCall)
* [reviews](https://docs.rs/google-androidpublisher3/5.0.5+20240626/google_androidpublisher3/api::Review)
 * [*get*](https://docs.rs/google-androidpublisher3/5.0.5+20240626/google_androidpublisher3/api::ReviewGetCall), [*list*](https://docs.rs/google-androidpublisher3/5.0.5+20240626/google_androidpublisher3/api::ReviewListCall) and [*reply*](https://docs.rs/google-androidpublisher3/5.0.5+20240626/google_androidpublisher3/api::ReviewReplyCall)
* systemapks
 * [*variants create*](https://docs.rs/google-androidpublisher3/5.0.5+20240626/google_androidpublisher3/api::SystemapkVariantCreateCall), [*variants download*](https://docs.rs/google-androidpublisher3/5.0.5+20240626/google_androidpublisher3/api::SystemapkVariantDownloadCall), [*variants get*](https://docs.rs/google-androidpublisher3/5.0.5+20240626/google_androidpublisher3/api::SystemapkVariantGetCall) and [*variants list*](https://docs.rs/google-androidpublisher3/5.0.5+20240626/google_androidpublisher3/api::SystemapkVariantListCall)
* [users](https://docs.rs/google-androidpublisher3/5.0.5+20240626/google_androidpublisher3/api::User)
 * [*create*](https://docs.rs/google-androidpublisher3/5.0.5+20240626/google_androidpublisher3/api::UserCreateCall), [*delete*](https://docs.rs/google-androidpublisher3/5.0.5+20240626/google_androidpublisher3/api::UserDeleteCall), [*list*](https://docs.rs/google-androidpublisher3/5.0.5+20240626/google_androidpublisher3/api::UserListCall) and [*patch*](https://docs.rs/google-androidpublisher3/5.0.5+20240626/google_androidpublisher3/api::UserPatchCall)


Upload supported by ...

* [*apks upload edits*](https://docs.rs/google-androidpublisher3/5.0.5+20240626/google_androidpublisher3/api::EditApkUploadCall)
* [*bundles upload edits*](https://docs.rs/google-androidpublisher3/5.0.5+20240626/google_androidpublisher3/api::EditBundleUploadCall)
* [*deobfuscationfiles upload edits*](https://docs.rs/google-androidpublisher3/5.0.5+20240626/google_androidpublisher3/api::EditDeobfuscationfileUploadCall)
* [*expansionfiles upload edits*](https://docs.rs/google-androidpublisher3/5.0.5+20240626/google_androidpublisher3/api::EditExpansionfileUploadCall)
* [*images upload edits*](https://docs.rs/google-androidpublisher3/5.0.5+20240626/google_androidpublisher3/api::EditImageUploadCall)
* [*uploadapk internalappsharingartifacts*](https://docs.rs/google-androidpublisher3/5.0.5+20240626/google_androidpublisher3/api::InternalappsharingartifactUploadapkCall)
* [*uploadbundle internalappsharingartifacts*](https://docs.rs/google-androidpublisher3/5.0.5+20240626/google_androidpublisher3/api::InternalappsharingartifactUploadbundleCall)

Download supported by ...

* [*download generatedapks*](https://docs.rs/google-androidpublisher3/5.0.5+20240626/google_androidpublisher3/api::GeneratedapkDownloadCall)
* [*variants download systemapks*](https://docs.rs/google-androidpublisher3/5.0.5+20240626/google_androidpublisher3/api::SystemapkVariantDownloadCall)



# Structure of this Library

The API is structured into the following primary items:

* **[Hub](https://docs.rs/google-androidpublisher3/5.0.5+20240626/google_androidpublisher3/AndroidPublisher)**
    * a central object to maintain state and allow accessing all *Activities*
    * creates [*Method Builders*](https://docs.rs/google-androidpublisher3/5.0.5+20240626/google_androidpublisher3/client::MethodsBuilder) which in turn
      allow access to individual [*Call Builders*](https://docs.rs/google-androidpublisher3/5.0.5+20240626/google_androidpublisher3/client::CallBuilder)
* **[Resources](https://docs.rs/google-androidpublisher3/5.0.5+20240626/google_androidpublisher3/client::Resource)**
    * primary types that you can apply *Activities* to
    * a collection of properties and *Parts*
    * **[Parts](https://docs.rs/google-androidpublisher3/5.0.5+20240626/google_androidpublisher3/client::Part)**
        * a collection of properties
        * never directly used in *Activities*
* **[Activities](https://docs.rs/google-androidpublisher3/5.0.5+20240626/google_androidpublisher3/client::CallBuilder)**
    * operations to apply to *Resources*

All *structures* are marked with applicable traits to further categorize them and ease browsing.

Generally speaking, you can invoke *Activities* like this:

```Rust,ignore
let r = hub.resource().activity(...).doit().await
```

Or specifically ...

```ignore
let r = hub.monetization().subscriptions_base_plans_activate(...).doit().await
let r = hub.monetization().subscriptions_base_plans_deactivate(...).doit().await
let r = hub.monetization().subscriptions_archive(...).doit().await
let r = hub.monetization().subscriptions_create(...).doit().await
let r = hub.monetization().subscriptions_get(...).doit().await
let r = hub.monetization().subscriptions_patch(...).doit().await
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
google-androidpublisher3 = "*"
serde = "^1.0"
serde_json = "^1.0"
```

## A complete example

```Rust
extern crate hyper;
extern crate hyper_rustls;
extern crate google_androidpublisher3 as androidpublisher3;
use androidpublisher3::api::Subscription;
use androidpublisher3::{Result, Error};
use std::default::Default;
use androidpublisher3::{AndroidPublisher, oauth2, hyper, hyper_rustls, chrono, FieldMask};

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
let mut hub = AndroidPublisher::new(hyper::Client::builder().build(hyper_rustls::HttpsConnectorBuilder::new().with_native_roots().unwrap().https_or_http().enable_http1().build()), auth);
// As the method needs a request, you would usually fill it with the desired information
// into the respective structure. Some of the parts shown here might not be applicable !
// Values shown here are possibly random and not representative !
let mut req = Subscription::default();

// You can configure optional parameters by calling the respective setters at will, and
// execute the final call using `doit()`.
// Values shown here are possibly random and not representative !
let result = hub.monetization().subscriptions_patch(req, "packageName", "productId")
             .update_mask(FieldMask::new::<&str>(&[]))
             .regions_version_version("no")
             .latency_tolerance("ipsum")
             .allow_missing(false)
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

All errors produced by the system are provided either as [Result](https://docs.rs/google-androidpublisher3/5.0.5+20240626/google_androidpublisher3/client::Result) enumeration as return value of
the doit() methods, or handed as possibly intermediate results to either the 
[Hub Delegate](https://docs.rs/google-androidpublisher3/5.0.5+20240626/google_androidpublisher3/client::Delegate), or the [Authenticator Delegate](https://docs.rs/yup-oauth2/*/yup_oauth2/trait.AuthenticatorDelegate.html).

When delegates handle errors or intermediate values, they may have a chance to instruct the system to retry. This 
makes the system potentially resilient to all kinds of errors.

## Uploads and Downloads
If a method supports downloads, the response body, which is part of the [Result](https://docs.rs/google-androidpublisher3/5.0.5+20240626/google_androidpublisher3/client::Result), should be
read by you to obtain the media.
If such a method also supports a [Response Result](https://docs.rs/google-androidpublisher3/5.0.5+20240626/google_androidpublisher3/client::ResponseResult), it will return that by default.
You can see it as meta-data for the actual media. To trigger a media download, you will have to set up the builder by making
this call: `.param("alt", "media")`.

Methods supporting uploads can do so using up to 2 different protocols: 
*simple* and *resumable*. The distinctiveness of each is represented by customized 
`doit(...)` methods, which are then named `upload(...)` and `upload_resumable(...)` respectively.

## Customization and Callbacks

You may alter the way an `doit()` method is called by providing a [delegate](https://docs.rs/google-androidpublisher3/5.0.5+20240626/google_androidpublisher3/client::Delegate) to the 
[Method Builder](https://docs.rs/google-androidpublisher3/5.0.5+20240626/google_androidpublisher3/client::CallBuilder) before making the final `doit()` call. 
Respective methods will be called to provide progress information, as well as determine whether the system should 
retry on failure.

The [delegate trait](https://docs.rs/google-androidpublisher3/5.0.5+20240626/google_androidpublisher3/client::Delegate) is default-implemented, allowing you to customize it with minimal effort.

## Optional Parts in Server-Requests

All structures provided by this library are made to be [encodable](https://docs.rs/google-androidpublisher3/5.0.5+20240626/google_androidpublisher3/client::RequestValue) and 
[decodable](https://docs.rs/google-androidpublisher3/5.0.5+20240626/google_androidpublisher3/client::ResponseResult) via *json*. Optionals are used to indicate that partial requests are responses 
are valid.
Most optionals are are considered [Parts](https://docs.rs/google-androidpublisher3/5.0.5+20240626/google_androidpublisher3/client::Part) which are identifiable by name, which will be sent to 
the server to indicate either the set parts of the request or the desired parts in the response.

## Builder Arguments

Using [method builders](https://docs.rs/google-androidpublisher3/5.0.5+20240626/google_androidpublisher3/client::CallBuilder), you are able to prepare an action call by repeatedly calling it's methods.
These will always take a single argument, for which the following statements are true.

* [PODs][wiki-pod] are handed by copy
* strings are passed as `&str`
* [request values](https://docs.rs/google-androidpublisher3/5.0.5+20240626/google_androidpublisher3/client::RequestValue) are moved

Arguments will always be copied or cloned into the builder, to make them independent of their original life times.

[wiki-pod]: http://en.wikipedia.org/wiki/Plain_old_data_structure
[builder-pattern]: http://en.wikipedia.org/wiki/Builder_pattern
[google-go-api]: https://github.com/google/google-api-go-client

## Cargo Features

* `utoipa` - Add support for [utoipa](https://crates.io/crates/utoipa) and derive `utoipa::ToSchema` on all
the types. You'll have to import and register the required types in `#[openapi(schemas(...))]`, otherwise the
generated `openapi` spec would be invalid.


# License
The **androidpublisher3** library was generated by Sebastian Thiel, and is placed 
under the *MIT* license.
You can read the full text at the repository's [license file][repo-license].

[repo-license]: https://github.com/Byron/google-apis-rsblob/main/LICENSE.md

