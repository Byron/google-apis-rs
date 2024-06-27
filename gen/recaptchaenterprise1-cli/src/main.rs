// DO NOT EDIT !
// This file was generated automatically from 'src/generator/templates/cli/main.rs.mako'
// DO NOT EDIT !
#![allow(unused_variables, unused_imports, dead_code, unused_mut)]

#[macro_use]
extern crate clap;

use std::env;
use std::io::{self, Write};
use clap::{App, SubCommand, Arg};

use google_recaptchaenterprise1::{api, Error, oauth2, client::chrono, FieldMask};


use google_clis_common as client;

use client::{InvalidOptionsError, CLIError, arg_from_str, writer_from_opts, parse_kv_arg,
          input_file_from_opts, input_mime_from_opts, FieldCursor, FieldError, CallType, UploadProtocol,
          calltype_from_str, remove_json_null_values, ComplexType, JsonType, JsonTypeInfo};

use std::default::Default;
use std::error::Error as StdError;
use std::str::FromStr;

use serde_json as json;
use clap::ArgMatches;
use http::Uri;
use hyper::client::connect;
use tokio::io::{AsyncRead, AsyncWrite};
use tower_service;

enum DoitError {
    IoError(String, io::Error),
    ApiError(Error),
}

struct Engine<'n, S> {
    opt: ArgMatches<'n>,
    hub: api::RecaptchaEnterprise<S>,
    gp: Vec<&'static str>,
    gpm: Vec<(&'static str, &'static str)>,
}


impl<'n, S> Engine<'n, S>
where
    S: tower_service::Service<Uri> + Clone + Send + Sync + 'static,
    S::Response: hyper::client::connect::Connection + AsyncRead + AsyncWrite + Send + Unpin + 'static,
    S::Future: Send + Unpin + 'static,
    S::Error: Into<Box<dyn StdError + Send + Sync>>,
{
    async fn _projects_assessments_annotate(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        
        let mut field_cursor = FieldCursor::default();
        let mut object = json::value::Value::Object(Default::default());
        
        for kvarg in opt.values_of("kv").map(|i|i.collect()).unwrap_or(Vec::new()).iter() {
            let last_errc = err.issues.len();
            let (key, value) = parse_kv_arg(&*kvarg, err, false);
            let mut temp_cursor = field_cursor.clone();
            if let Err(field_err) = temp_cursor.set(&*key) {
                err.issues.push(field_err);
            }
            if value.is_none() {
                field_cursor = temp_cursor.clone();
                if err.issues.len() > last_errc {
                    err.issues.remove(last_errc);
                }
                continue;
            }
        
            let type_info: Option<(&'static str, JsonTypeInfo)> =
                match &temp_cursor.to_string()[..] {
                    "account-id" => Some(("accountId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "annotation" => Some(("annotation", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "hashed-account-id" => Some(("hashedAccountId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "reasons" => Some(("reasons", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Vec })),
                    "transaction-event.event-time" => Some(("transactionEvent.eventTime", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "transaction-event.event-type" => Some(("transactionEvent.eventType", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "transaction-event.reason" => Some(("transactionEvent.reason", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "transaction-event.value" => Some(("transactionEvent.value", JsonTypeInfo { jtype: JsonType::Float, ctype: ComplexType::Pod })),
                    _ => {
                        let suggestion = FieldCursor::did_you_mean(key, &vec!["account-id", "annotation", "event-time", "event-type", "hashed-account-id", "reason", "reasons", "transaction-event", "value"]);
                        err.issues.push(CLIError::Field(FieldError::Unknown(temp_cursor.to_string(), suggestion, value.map(|v| v.to_string()))));
                        None
                    }
                };
            if let Some((field_cursor_str, type_info)) = type_info {
                FieldCursor::from(field_cursor_str).set_json_value(&mut object, value.unwrap(), type_info, err, &temp_cursor);
            }
        }
        let mut request: api::GoogleCloudRecaptchaenterpriseV1AnnotateAssessmentRequest = json::value::from_value(object).unwrap();
        let mut call = self.hub.projects().assessments_annotate(request, opt.value_of("name").unwrap_or(""));
        for parg in opt.values_of("v").map(|i|i.collect()).unwrap_or(Vec::new()).iter() {
            let (key, value) = parse_kv_arg(&*parg, err, false);
            match key {
                _ => {
                    let mut found = false;
                    for param in &self.gp {
                        if key == *param {
                            found = true;
                            call = call.param(self.gpm.iter().find(|t| t.0 == key).unwrap_or(&("", key)).1, value.unwrap_or("unset"));
                            break;
                        }
                    }
                    if !found {
                        err.issues.push(CLIError::UnknownParameter(key.to_string(),
                                                                  {let mut v = Vec::new();
                                                                           v.extend(self.gp.iter().map(|v|*v));
                                                                           v } ));
                    }
                }
            }
        }
        let protocol = CallType::Standard;
        if dry_run {
            Ok(())
        } else {
            assert!(err.issues.len() == 0);
            for scope in self.opt.values_of("url").map(|i|i.collect()).unwrap_or(Vec::new()).iter() {
                call = call.add_scope(scope);
            }
            let mut ostream = match writer_from_opts(opt.value_of("out")) {
                Ok(mut f) => f,
                Err(io_err) => return Err(DoitError::IoError(opt.value_of("out").unwrap_or("-").to_string(), io_err)),
            };
            match match protocol {
                CallType::Standard => call.doit().await,
                _ => unreachable!()
            } {
                Err(api_err) => Err(DoitError::ApiError(api_err)),
                Ok((mut response, output_schema)) => {
                    let mut value = json::value::to_value(&output_schema).expect("serde to work");
                    remove_json_null_values(&mut value);
                    json::to_writer_pretty(&mut ostream, &value).unwrap();
                    ostream.flush().unwrap();
                    Ok(())
                }
            }
        }
    }

    async fn _projects_assessments_create(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        
        let mut field_cursor = FieldCursor::default();
        let mut object = json::value::Value::Object(Default::default());
        
        for kvarg in opt.values_of("kv").map(|i|i.collect()).unwrap_or(Vec::new()).iter() {
            let last_errc = err.issues.len();
            let (key, value) = parse_kv_arg(&*kvarg, err, false);
            let mut temp_cursor = field_cursor.clone();
            if let Err(field_err) = temp_cursor.set(&*key) {
                err.issues.push(field_err);
            }
            if value.is_none() {
                field_cursor = temp_cursor.clone();
                if err.issues.len() > last_errc {
                    err.issues.remove(last_errc);
                }
                continue;
            }
        
            let type_info: Option<(&'static str, JsonTypeInfo)> =
                match &temp_cursor.to_string()[..] {
                    "account-defender-assessment.labels" => Some(("accountDefenderAssessment.labels", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Vec })),
                    "account-verification.language-code" => Some(("accountVerification.languageCode", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "account-verification.latest-verification-result" => Some(("accountVerification.latestVerificationResult", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "account-verification.username" => Some(("accountVerification.username", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "event.expected-action" => Some(("event.expectedAction", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "event.express" => Some(("event.express", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "event.firewall-policy-evaluation" => Some(("event.firewallPolicyEvaluation", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "event.fraud-prevention" => Some(("event.fraudPrevention", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "event.hashed-account-id" => Some(("event.hashedAccountId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "event.headers" => Some(("event.headers", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Vec })),
                    "event.ja3" => Some(("event.ja3", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "event.requested-uri" => Some(("event.requestedUri", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "event.site-key" => Some(("event.siteKey", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "event.token" => Some(("event.token", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "event.transaction-data.billing-address.address" => Some(("event.transactionData.billingAddress.address", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Vec })),
                    "event.transaction-data.billing-address.administrative-area" => Some(("event.transactionData.billingAddress.administrativeArea", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "event.transaction-data.billing-address.locality" => Some(("event.transactionData.billingAddress.locality", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "event.transaction-data.billing-address.postal-code" => Some(("event.transactionData.billingAddress.postalCode", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "event.transaction-data.billing-address.recipient" => Some(("event.transactionData.billingAddress.recipient", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "event.transaction-data.billing-address.region-code" => Some(("event.transactionData.billingAddress.regionCode", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "event.transaction-data.card-bin" => Some(("event.transactionData.cardBin", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "event.transaction-data.card-last-four" => Some(("event.transactionData.cardLastFour", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "event.transaction-data.currency-code" => Some(("event.transactionData.currencyCode", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "event.transaction-data.gateway-info.avs-response-code" => Some(("event.transactionData.gatewayInfo.avsResponseCode", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "event.transaction-data.gateway-info.cvv-response-code" => Some(("event.transactionData.gatewayInfo.cvvResponseCode", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "event.transaction-data.gateway-info.gateway-response-code" => Some(("event.transactionData.gatewayInfo.gatewayResponseCode", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "event.transaction-data.gateway-info.name" => Some(("event.transactionData.gatewayInfo.name", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "event.transaction-data.payment-method" => Some(("event.transactionData.paymentMethod", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "event.transaction-data.shipping-address.address" => Some(("event.transactionData.shippingAddress.address", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Vec })),
                    "event.transaction-data.shipping-address.administrative-area" => Some(("event.transactionData.shippingAddress.administrativeArea", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "event.transaction-data.shipping-address.locality" => Some(("event.transactionData.shippingAddress.locality", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "event.transaction-data.shipping-address.postal-code" => Some(("event.transactionData.shippingAddress.postalCode", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "event.transaction-data.shipping-address.recipient" => Some(("event.transactionData.shippingAddress.recipient", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "event.transaction-data.shipping-address.region-code" => Some(("event.transactionData.shippingAddress.regionCode", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "event.transaction-data.shipping-value" => Some(("event.transactionData.shippingValue", JsonTypeInfo { jtype: JsonType::Float, ctype: ComplexType::Pod })),
                    "event.transaction-data.transaction-id" => Some(("event.transactionData.transactionId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "event.transaction-data.user.account-id" => Some(("event.transactionData.user.accountId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "event.transaction-data.user.creation-ms" => Some(("event.transactionData.user.creationMs", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "event.transaction-data.user.email" => Some(("event.transactionData.user.email", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "event.transaction-data.user.email-verified" => Some(("event.transactionData.user.emailVerified", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "event.transaction-data.user.phone-number" => Some(("event.transactionData.user.phoneNumber", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "event.transaction-data.user.phone-verified" => Some(("event.transactionData.user.phoneVerified", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "event.transaction-data.value" => Some(("event.transactionData.value", JsonTypeInfo { jtype: JsonType::Float, ctype: ComplexType::Pod })),
                    "event.user-agent" => Some(("event.userAgent", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "event.user-info.account-id" => Some(("event.userInfo.accountId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "event.user-info.create-account-time" => Some(("event.userInfo.createAccountTime", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "event.user-ip-address" => Some(("event.userIpAddress", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "event.waf-token-assessment" => Some(("event.wafTokenAssessment", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "firewall-policy-assessment.error.code" => Some(("firewallPolicyAssessment.error.code", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "firewall-policy-assessment.error.message" => Some(("firewallPolicyAssessment.error.message", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "firewall-policy-assessment.firewall-policy.condition" => Some(("firewallPolicyAssessment.firewallPolicy.condition", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "firewall-policy-assessment.firewall-policy.description" => Some(("firewallPolicyAssessment.firewallPolicy.description", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "firewall-policy-assessment.firewall-policy.name" => Some(("firewallPolicyAssessment.firewallPolicy.name", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "firewall-policy-assessment.firewall-policy.path" => Some(("firewallPolicyAssessment.firewallPolicy.path", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "fraud-prevention-assessment.behavioral-trust-verdict.trust" => Some(("fraudPreventionAssessment.behavioralTrustVerdict.trust", JsonTypeInfo { jtype: JsonType::Float, ctype: ComplexType::Pod })),
                    "fraud-prevention-assessment.card-testing-verdict.risk" => Some(("fraudPreventionAssessment.cardTestingVerdict.risk", JsonTypeInfo { jtype: JsonType::Float, ctype: ComplexType::Pod })),
                    "fraud-prevention-assessment.stolen-instrument-verdict.risk" => Some(("fraudPreventionAssessment.stolenInstrumentVerdict.risk", JsonTypeInfo { jtype: JsonType::Float, ctype: ComplexType::Pod })),
                    "fraud-prevention-assessment.transaction-risk" => Some(("fraudPreventionAssessment.transactionRisk", JsonTypeInfo { jtype: JsonType::Float, ctype: ComplexType::Pod })),
                    "fraud-signals.card-signals.card-labels" => Some(("fraudSignals.cardSignals.cardLabels", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Vec })),
                    "fraud-signals.user-signals.active-days-lower-bound" => Some(("fraudSignals.userSignals.activeDaysLowerBound", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "fraud-signals.user-signals.synthetic-risk" => Some(("fraudSignals.userSignals.syntheticRisk", JsonTypeInfo { jtype: JsonType::Float, ctype: ComplexType::Pod })),
                    "name" => Some(("name", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "phone-fraud-assessment.sms-toll-fraud-verdict.reasons" => Some(("phoneFraudAssessment.smsTollFraudVerdict.reasons", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Vec })),
                    "phone-fraud-assessment.sms-toll-fraud-verdict.risk" => Some(("phoneFraudAssessment.smsTollFraudVerdict.risk", JsonTypeInfo { jtype: JsonType::Float, ctype: ComplexType::Pod })),
                    "private-password-leak-verification.encrypted-leak-match-prefixes" => Some(("privatePasswordLeakVerification.encryptedLeakMatchPrefixes", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Vec })),
                    "private-password-leak-verification.encrypted-user-credentials-hash" => Some(("privatePasswordLeakVerification.encryptedUserCredentialsHash", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "private-password-leak-verification.lookup-hash-prefix" => Some(("privatePasswordLeakVerification.lookupHashPrefix", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "private-password-leak-verification.reencrypted-user-credentials-hash" => Some(("privatePasswordLeakVerification.reencryptedUserCredentialsHash", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "risk-analysis.extended-verdict-reasons" => Some(("riskAnalysis.extendedVerdictReasons", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Vec })),
                    "risk-analysis.reasons" => Some(("riskAnalysis.reasons", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Vec })),
                    "risk-analysis.score" => Some(("riskAnalysis.score", JsonTypeInfo { jtype: JsonType::Float, ctype: ComplexType::Pod })),
                    "token-properties.action" => Some(("tokenProperties.action", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "token-properties.android-package-name" => Some(("tokenProperties.androidPackageName", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "token-properties.create-time" => Some(("tokenProperties.createTime", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "token-properties.hostname" => Some(("tokenProperties.hostname", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "token-properties.invalid-reason" => Some(("tokenProperties.invalidReason", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "token-properties.ios-bundle-id" => Some(("tokenProperties.iosBundleId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "token-properties.valid" => Some(("tokenProperties.valid", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    _ => {
                        let suggestion = FieldCursor::did_you_mean(key, &vec!["account-defender-assessment", "account-id", "account-verification", "action", "active-days-lower-bound", "address", "administrative-area", "android-package-name", "avs-response-code", "behavioral-trust-verdict", "billing-address", "card-bin", "card-labels", "card-last-four", "card-signals", "card-testing-verdict", "code", "condition", "create-account-time", "create-time", "creation-ms", "currency-code", "cvv-response-code", "description", "email", "email-verified", "encrypted-leak-match-prefixes", "encrypted-user-credentials-hash", "error", "event", "expected-action", "express", "extended-verdict-reasons", "firewall-policy", "firewall-policy-assessment", "firewall-policy-evaluation", "fraud-prevention", "fraud-prevention-assessment", "fraud-signals", "gateway-info", "gateway-response-code", "hashed-account-id", "headers", "hostname", "invalid-reason", "ios-bundle-id", "ja3", "labels", "language-code", "latest-verification-result", "locality", "lookup-hash-prefix", "message", "name", "path", "payment-method", "phone-fraud-assessment", "phone-number", "phone-verified", "postal-code", "private-password-leak-verification", "reasons", "recipient", "reencrypted-user-credentials-hash", "region-code", "requested-uri", "risk", "risk-analysis", "score", "shipping-address", "shipping-value", "site-key", "sms-toll-fraud-verdict", "stolen-instrument-verdict", "synthetic-risk", "token", "token-properties", "transaction-data", "transaction-id", "transaction-risk", "trust", "user", "user-agent", "user-info", "user-ip-address", "user-signals", "username", "valid", "value", "waf-token-assessment"]);
                        err.issues.push(CLIError::Field(FieldError::Unknown(temp_cursor.to_string(), suggestion, value.map(|v| v.to_string()))));
                        None
                    }
                };
            if let Some((field_cursor_str, type_info)) = type_info {
                FieldCursor::from(field_cursor_str).set_json_value(&mut object, value.unwrap(), type_info, err, &temp_cursor);
            }
        }
        let mut request: api::GoogleCloudRecaptchaenterpriseV1Assessment = json::value::from_value(object).unwrap();
        let mut call = self.hub.projects().assessments_create(request, opt.value_of("parent").unwrap_or(""));
        for parg in opt.values_of("v").map(|i|i.collect()).unwrap_or(Vec::new()).iter() {
            let (key, value) = parse_kv_arg(&*parg, err, false);
            match key {
                _ => {
                    let mut found = false;
                    for param in &self.gp {
                        if key == *param {
                            found = true;
                            call = call.param(self.gpm.iter().find(|t| t.0 == key).unwrap_or(&("", key)).1, value.unwrap_or("unset"));
                            break;
                        }
                    }
                    if !found {
                        err.issues.push(CLIError::UnknownParameter(key.to_string(),
                                                                  {let mut v = Vec::new();
                                                                           v.extend(self.gp.iter().map(|v|*v));
                                                                           v } ));
                    }
                }
            }
        }
        let protocol = CallType::Standard;
        if dry_run {
            Ok(())
        } else {
            assert!(err.issues.len() == 0);
            for scope in self.opt.values_of("url").map(|i|i.collect()).unwrap_or(Vec::new()).iter() {
                call = call.add_scope(scope);
            }
            let mut ostream = match writer_from_opts(opt.value_of("out")) {
                Ok(mut f) => f,
                Err(io_err) => return Err(DoitError::IoError(opt.value_of("out").unwrap_or("-").to_string(), io_err)),
            };
            match match protocol {
                CallType::Standard => call.doit().await,
                _ => unreachable!()
            } {
                Err(api_err) => Err(DoitError::ApiError(api_err)),
                Ok((mut response, output_schema)) => {
                    let mut value = json::value::to_value(&output_schema).expect("serde to work");
                    remove_json_null_values(&mut value);
                    json::to_writer_pretty(&mut ostream, &value).unwrap();
                    ostream.flush().unwrap();
                    Ok(())
                }
            }
        }
    }

    async fn _projects_firewallpolicies_create(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        
        let mut field_cursor = FieldCursor::default();
        let mut object = json::value::Value::Object(Default::default());
        
        for kvarg in opt.values_of("kv").map(|i|i.collect()).unwrap_or(Vec::new()).iter() {
            let last_errc = err.issues.len();
            let (key, value) = parse_kv_arg(&*kvarg, err, false);
            let mut temp_cursor = field_cursor.clone();
            if let Err(field_err) = temp_cursor.set(&*key) {
                err.issues.push(field_err);
            }
            if value.is_none() {
                field_cursor = temp_cursor.clone();
                if err.issues.len() > last_errc {
                    err.issues.remove(last_errc);
                }
                continue;
            }
        
            let type_info: Option<(&'static str, JsonTypeInfo)> =
                match &temp_cursor.to_string()[..] {
                    "condition" => Some(("condition", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "description" => Some(("description", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "name" => Some(("name", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "path" => Some(("path", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    _ => {
                        let suggestion = FieldCursor::did_you_mean(key, &vec!["condition", "description", "name", "path"]);
                        err.issues.push(CLIError::Field(FieldError::Unknown(temp_cursor.to_string(), suggestion, value.map(|v| v.to_string()))));
                        None
                    }
                };
            if let Some((field_cursor_str, type_info)) = type_info {
                FieldCursor::from(field_cursor_str).set_json_value(&mut object, value.unwrap(), type_info, err, &temp_cursor);
            }
        }
        let mut request: api::GoogleCloudRecaptchaenterpriseV1FirewallPolicy = json::value::from_value(object).unwrap();
        let mut call = self.hub.projects().firewallpolicies_create(request, opt.value_of("parent").unwrap_or(""));
        for parg in opt.values_of("v").map(|i|i.collect()).unwrap_or(Vec::new()).iter() {
            let (key, value) = parse_kv_arg(&*parg, err, false);
            match key {
                _ => {
                    let mut found = false;
                    for param in &self.gp {
                        if key == *param {
                            found = true;
                            call = call.param(self.gpm.iter().find(|t| t.0 == key).unwrap_or(&("", key)).1, value.unwrap_or("unset"));
                            break;
                        }
                    }
                    if !found {
                        err.issues.push(CLIError::UnknownParameter(key.to_string(),
                                                                  {let mut v = Vec::new();
                                                                           v.extend(self.gp.iter().map(|v|*v));
                                                                           v } ));
                    }
                }
            }
        }
        let protocol = CallType::Standard;
        if dry_run {
            Ok(())
        } else {
            assert!(err.issues.len() == 0);
            for scope in self.opt.values_of("url").map(|i|i.collect()).unwrap_or(Vec::new()).iter() {
                call = call.add_scope(scope);
            }
            let mut ostream = match writer_from_opts(opt.value_of("out")) {
                Ok(mut f) => f,
                Err(io_err) => return Err(DoitError::IoError(opt.value_of("out").unwrap_or("-").to_string(), io_err)),
            };
            match match protocol {
                CallType::Standard => call.doit().await,
                _ => unreachable!()
            } {
                Err(api_err) => Err(DoitError::ApiError(api_err)),
                Ok((mut response, output_schema)) => {
                    let mut value = json::value::to_value(&output_schema).expect("serde to work");
                    remove_json_null_values(&mut value);
                    json::to_writer_pretty(&mut ostream, &value).unwrap();
                    ostream.flush().unwrap();
                    Ok(())
                }
            }
        }
    }

    async fn _projects_firewallpolicies_delete(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        let mut call = self.hub.projects().firewallpolicies_delete(opt.value_of("name").unwrap_or(""));
        for parg in opt.values_of("v").map(|i|i.collect()).unwrap_or(Vec::new()).iter() {
            let (key, value) = parse_kv_arg(&*parg, err, false);
            match key {
                _ => {
                    let mut found = false;
                    for param in &self.gp {
                        if key == *param {
                            found = true;
                            call = call.param(self.gpm.iter().find(|t| t.0 == key).unwrap_or(&("", key)).1, value.unwrap_or("unset"));
                            break;
                        }
                    }
                    if !found {
                        err.issues.push(CLIError::UnknownParameter(key.to_string(),
                                                                  {let mut v = Vec::new();
                                                                           v.extend(self.gp.iter().map(|v|*v));
                                                                           v } ));
                    }
                }
            }
        }
        let protocol = CallType::Standard;
        if dry_run {
            Ok(())
        } else {
            assert!(err.issues.len() == 0);
            for scope in self.opt.values_of("url").map(|i|i.collect()).unwrap_or(Vec::new()).iter() {
                call = call.add_scope(scope);
            }
            let mut ostream = match writer_from_opts(opt.value_of("out")) {
                Ok(mut f) => f,
                Err(io_err) => return Err(DoitError::IoError(opt.value_of("out").unwrap_or("-").to_string(), io_err)),
            };
            match match protocol {
                CallType::Standard => call.doit().await,
                _ => unreachable!()
            } {
                Err(api_err) => Err(DoitError::ApiError(api_err)),
                Ok((mut response, output_schema)) => {
                    let mut value = json::value::to_value(&output_schema).expect("serde to work");
                    remove_json_null_values(&mut value);
                    json::to_writer_pretty(&mut ostream, &value).unwrap();
                    ostream.flush().unwrap();
                    Ok(())
                }
            }
        }
    }

    async fn _projects_firewallpolicies_get(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        let mut call = self.hub.projects().firewallpolicies_get(opt.value_of("name").unwrap_or(""));
        for parg in opt.values_of("v").map(|i|i.collect()).unwrap_or(Vec::new()).iter() {
            let (key, value) = parse_kv_arg(&*parg, err, false);
            match key {
                _ => {
                    let mut found = false;
                    for param in &self.gp {
                        if key == *param {
                            found = true;
                            call = call.param(self.gpm.iter().find(|t| t.0 == key).unwrap_or(&("", key)).1, value.unwrap_or("unset"));
                            break;
                        }
                    }
                    if !found {
                        err.issues.push(CLIError::UnknownParameter(key.to_string(),
                                                                  {let mut v = Vec::new();
                                                                           v.extend(self.gp.iter().map(|v|*v));
                                                                           v } ));
                    }
                }
            }
        }
        let protocol = CallType::Standard;
        if dry_run {
            Ok(())
        } else {
            assert!(err.issues.len() == 0);
            for scope in self.opt.values_of("url").map(|i|i.collect()).unwrap_or(Vec::new()).iter() {
                call = call.add_scope(scope);
            }
            let mut ostream = match writer_from_opts(opt.value_of("out")) {
                Ok(mut f) => f,
                Err(io_err) => return Err(DoitError::IoError(opt.value_of("out").unwrap_or("-").to_string(), io_err)),
            };
            match match protocol {
                CallType::Standard => call.doit().await,
                _ => unreachable!()
            } {
                Err(api_err) => Err(DoitError::ApiError(api_err)),
                Ok((mut response, output_schema)) => {
                    let mut value = json::value::to_value(&output_schema).expect("serde to work");
                    remove_json_null_values(&mut value);
                    json::to_writer_pretty(&mut ostream, &value).unwrap();
                    ostream.flush().unwrap();
                    Ok(())
                }
            }
        }
    }

    async fn _projects_firewallpolicies_list(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        let mut call = self.hub.projects().firewallpolicies_list(opt.value_of("parent").unwrap_or(""));
        for parg in opt.values_of("v").map(|i|i.collect()).unwrap_or(Vec::new()).iter() {
            let (key, value) = parse_kv_arg(&*parg, err, false);
            match key {
                "page-token" => {
                    call = call.page_token(value.unwrap_or(""));
                },
                "page-size" => {
                    call = call.page_size(        value.map(|v| arg_from_str(v, err, "page-size", "int32")).unwrap_or(-0));
                },
                _ => {
                    let mut found = false;
                    for param in &self.gp {
                        if key == *param {
                            found = true;
                            call = call.param(self.gpm.iter().find(|t| t.0 == key).unwrap_or(&("", key)).1, value.unwrap_or("unset"));
                            break;
                        }
                    }
                    if !found {
                        err.issues.push(CLIError::UnknownParameter(key.to_string(),
                                                                  {let mut v = Vec::new();
                                                                           v.extend(self.gp.iter().map(|v|*v));
                                                                           v.extend(["page-size", "page-token"].iter().map(|v|*v));
                                                                           v } ));
                    }
                }
            }
        }
        let protocol = CallType::Standard;
        if dry_run {
            Ok(())
        } else {
            assert!(err.issues.len() == 0);
            for scope in self.opt.values_of("url").map(|i|i.collect()).unwrap_or(Vec::new()).iter() {
                call = call.add_scope(scope);
            }
            let mut ostream = match writer_from_opts(opt.value_of("out")) {
                Ok(mut f) => f,
                Err(io_err) => return Err(DoitError::IoError(opt.value_of("out").unwrap_or("-").to_string(), io_err)),
            };
            match match protocol {
                CallType::Standard => call.doit().await,
                _ => unreachable!()
            } {
                Err(api_err) => Err(DoitError::ApiError(api_err)),
                Ok((mut response, output_schema)) => {
                    let mut value = json::value::to_value(&output_schema).expect("serde to work");
                    remove_json_null_values(&mut value);
                    json::to_writer_pretty(&mut ostream, &value).unwrap();
                    ostream.flush().unwrap();
                    Ok(())
                }
            }
        }
    }

    async fn _projects_firewallpolicies_patch(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        
        let mut field_cursor = FieldCursor::default();
        let mut object = json::value::Value::Object(Default::default());
        
        for kvarg in opt.values_of("kv").map(|i|i.collect()).unwrap_or(Vec::new()).iter() {
            let last_errc = err.issues.len();
            let (key, value) = parse_kv_arg(&*kvarg, err, false);
            let mut temp_cursor = field_cursor.clone();
            if let Err(field_err) = temp_cursor.set(&*key) {
                err.issues.push(field_err);
            }
            if value.is_none() {
                field_cursor = temp_cursor.clone();
                if err.issues.len() > last_errc {
                    err.issues.remove(last_errc);
                }
                continue;
            }
        
            let type_info: Option<(&'static str, JsonTypeInfo)> =
                match &temp_cursor.to_string()[..] {
                    "condition" => Some(("condition", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "description" => Some(("description", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "name" => Some(("name", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "path" => Some(("path", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    _ => {
                        let suggestion = FieldCursor::did_you_mean(key, &vec!["condition", "description", "name", "path"]);
                        err.issues.push(CLIError::Field(FieldError::Unknown(temp_cursor.to_string(), suggestion, value.map(|v| v.to_string()))));
                        None
                    }
                };
            if let Some((field_cursor_str, type_info)) = type_info {
                FieldCursor::from(field_cursor_str).set_json_value(&mut object, value.unwrap(), type_info, err, &temp_cursor);
            }
        }
        let mut request: api::GoogleCloudRecaptchaenterpriseV1FirewallPolicy = json::value::from_value(object).unwrap();
        let mut call = self.hub.projects().firewallpolicies_patch(request, opt.value_of("name").unwrap_or(""));
        for parg in opt.values_of("v").map(|i|i.collect()).unwrap_or(Vec::new()).iter() {
            let (key, value) = parse_kv_arg(&*parg, err, false);
            match key {
                "update-mask" => {
                    call = call.update_mask(        value.map(|v| arg_from_str(v, err, "update-mask", "google-fieldmask")).unwrap_or(FieldMask::default()));
                },
                _ => {
                    let mut found = false;
                    for param in &self.gp {
                        if key == *param {
                            found = true;
                            call = call.param(self.gpm.iter().find(|t| t.0 == key).unwrap_or(&("", key)).1, value.unwrap_or("unset"));
                            break;
                        }
                    }
                    if !found {
                        err.issues.push(CLIError::UnknownParameter(key.to_string(),
                                                                  {let mut v = Vec::new();
                                                                           v.extend(self.gp.iter().map(|v|*v));
                                                                           v.extend(["update-mask"].iter().map(|v|*v));
                                                                           v } ));
                    }
                }
            }
        }
        let protocol = CallType::Standard;
        if dry_run {
            Ok(())
        } else {
            assert!(err.issues.len() == 0);
            for scope in self.opt.values_of("url").map(|i|i.collect()).unwrap_or(Vec::new()).iter() {
                call = call.add_scope(scope);
            }
            let mut ostream = match writer_from_opts(opt.value_of("out")) {
                Ok(mut f) => f,
                Err(io_err) => return Err(DoitError::IoError(opt.value_of("out").unwrap_or("-").to_string(), io_err)),
            };
            match match protocol {
                CallType::Standard => call.doit().await,
                _ => unreachable!()
            } {
                Err(api_err) => Err(DoitError::ApiError(api_err)),
                Ok((mut response, output_schema)) => {
                    let mut value = json::value::to_value(&output_schema).expect("serde to work");
                    remove_json_null_values(&mut value);
                    json::to_writer_pretty(&mut ostream, &value).unwrap();
                    ostream.flush().unwrap();
                    Ok(())
                }
            }
        }
    }

    async fn _projects_firewallpolicies_reorder(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        
        let mut field_cursor = FieldCursor::default();
        let mut object = json::value::Value::Object(Default::default());
        
        for kvarg in opt.values_of("kv").map(|i|i.collect()).unwrap_or(Vec::new()).iter() {
            let last_errc = err.issues.len();
            let (key, value) = parse_kv_arg(&*kvarg, err, false);
            let mut temp_cursor = field_cursor.clone();
            if let Err(field_err) = temp_cursor.set(&*key) {
                err.issues.push(field_err);
            }
            if value.is_none() {
                field_cursor = temp_cursor.clone();
                if err.issues.len() > last_errc {
                    err.issues.remove(last_errc);
                }
                continue;
            }
        
            let type_info: Option<(&'static str, JsonTypeInfo)> =
                match &temp_cursor.to_string()[..] {
                    "names" => Some(("names", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Vec })),
                    _ => {
                        let suggestion = FieldCursor::did_you_mean(key, &vec!["names"]);
                        err.issues.push(CLIError::Field(FieldError::Unknown(temp_cursor.to_string(), suggestion, value.map(|v| v.to_string()))));
                        None
                    }
                };
            if let Some((field_cursor_str, type_info)) = type_info {
                FieldCursor::from(field_cursor_str).set_json_value(&mut object, value.unwrap(), type_info, err, &temp_cursor);
            }
        }
        let mut request: api::GoogleCloudRecaptchaenterpriseV1ReorderFirewallPoliciesRequest = json::value::from_value(object).unwrap();
        let mut call = self.hub.projects().firewallpolicies_reorder(request, opt.value_of("parent").unwrap_or(""));
        for parg in opt.values_of("v").map(|i|i.collect()).unwrap_or(Vec::new()).iter() {
            let (key, value) = parse_kv_arg(&*parg, err, false);
            match key {
                _ => {
                    let mut found = false;
                    for param in &self.gp {
                        if key == *param {
                            found = true;
                            call = call.param(self.gpm.iter().find(|t| t.0 == key).unwrap_or(&("", key)).1, value.unwrap_or("unset"));
                            break;
                        }
                    }
                    if !found {
                        err.issues.push(CLIError::UnknownParameter(key.to_string(),
                                                                  {let mut v = Vec::new();
                                                                           v.extend(self.gp.iter().map(|v|*v));
                                                                           v } ));
                    }
                }
            }
        }
        let protocol = CallType::Standard;
        if dry_run {
            Ok(())
        } else {
            assert!(err.issues.len() == 0);
            for scope in self.opt.values_of("url").map(|i|i.collect()).unwrap_or(Vec::new()).iter() {
                call = call.add_scope(scope);
            }
            let mut ostream = match writer_from_opts(opt.value_of("out")) {
                Ok(mut f) => f,
                Err(io_err) => return Err(DoitError::IoError(opt.value_of("out").unwrap_or("-").to_string(), io_err)),
            };
            match match protocol {
                CallType::Standard => call.doit().await,
                _ => unreachable!()
            } {
                Err(api_err) => Err(DoitError::ApiError(api_err)),
                Ok((mut response, output_schema)) => {
                    let mut value = json::value::to_value(&output_schema).expect("serde to work");
                    remove_json_null_values(&mut value);
                    json::to_writer_pretty(&mut ostream, &value).unwrap();
                    ostream.flush().unwrap();
                    Ok(())
                }
            }
        }
    }

    async fn _projects_keys_create(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        
        let mut field_cursor = FieldCursor::default();
        let mut object = json::value::Value::Object(Default::default());
        
        for kvarg in opt.values_of("kv").map(|i|i.collect()).unwrap_or(Vec::new()).iter() {
            let last_errc = err.issues.len();
            let (key, value) = parse_kv_arg(&*kvarg, err, false);
            let mut temp_cursor = field_cursor.clone();
            if let Err(field_err) = temp_cursor.set(&*key) {
                err.issues.push(field_err);
            }
            if value.is_none() {
                field_cursor = temp_cursor.clone();
                if err.issues.len() > last_errc {
                    err.issues.remove(last_errc);
                }
                continue;
            }
        
            let type_info: Option<(&'static str, JsonTypeInfo)> =
                match &temp_cursor.to_string()[..] {
                    "android-settings.allow-all-package-names" => Some(("androidSettings.allowAllPackageNames", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "android-settings.allowed-package-names" => Some(("androidSettings.allowedPackageNames", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Vec })),
                    "android-settings.support-non-google-app-store-distribution" => Some(("androidSettings.supportNonGoogleAppStoreDistribution", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "create-time" => Some(("createTime", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "display-name" => Some(("displayName", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "ios-settings.allow-all-bundle-ids" => Some(("iosSettings.allowAllBundleIds", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "ios-settings.allowed-bundle-ids" => Some(("iosSettings.allowedBundleIds", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Vec })),
                    "ios-settings.apple-developer-id.key-id" => Some(("iosSettings.appleDeveloperId.keyId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "ios-settings.apple-developer-id.private-key" => Some(("iosSettings.appleDeveloperId.privateKey", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "ios-settings.apple-developer-id.team-id" => Some(("iosSettings.appleDeveloperId.teamId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "labels" => Some(("labels", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Map })),
                    "name" => Some(("name", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "testing-options.testing-challenge" => Some(("testingOptions.testingChallenge", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "testing-options.testing-score" => Some(("testingOptions.testingScore", JsonTypeInfo { jtype: JsonType::Float, ctype: ComplexType::Pod })),
                    "waf-settings.waf-feature" => Some(("wafSettings.wafFeature", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "waf-settings.waf-service" => Some(("wafSettings.wafService", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "web-settings.allow-all-domains" => Some(("webSettings.allowAllDomains", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "web-settings.allow-amp-traffic" => Some(("webSettings.allowAmpTraffic", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "web-settings.allowed-domains" => Some(("webSettings.allowedDomains", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Vec })),
                    "web-settings.challenge-security-preference" => Some(("webSettings.challengeSecurityPreference", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "web-settings.integration-type" => Some(("webSettings.integrationType", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    _ => {
                        let suggestion = FieldCursor::did_you_mean(key, &vec!["allow-all-bundle-ids", "allow-all-domains", "allow-all-package-names", "allow-amp-traffic", "allowed-bundle-ids", "allowed-domains", "allowed-package-names", "android-settings", "apple-developer-id", "challenge-security-preference", "create-time", "display-name", "integration-type", "ios-settings", "key-id", "labels", "name", "private-key", "support-non-google-app-store-distribution", "team-id", "testing-challenge", "testing-options", "testing-score", "waf-feature", "waf-service", "waf-settings", "web-settings"]);
                        err.issues.push(CLIError::Field(FieldError::Unknown(temp_cursor.to_string(), suggestion, value.map(|v| v.to_string()))));
                        None
                    }
                };
            if let Some((field_cursor_str, type_info)) = type_info {
                FieldCursor::from(field_cursor_str).set_json_value(&mut object, value.unwrap(), type_info, err, &temp_cursor);
            }
        }
        let mut request: api::GoogleCloudRecaptchaenterpriseV1Key = json::value::from_value(object).unwrap();
        let mut call = self.hub.projects().keys_create(request, opt.value_of("parent").unwrap_or(""));
        for parg in opt.values_of("v").map(|i|i.collect()).unwrap_or(Vec::new()).iter() {
            let (key, value) = parse_kv_arg(&*parg, err, false);
            match key {
                _ => {
                    let mut found = false;
                    for param in &self.gp {
                        if key == *param {
                            found = true;
                            call = call.param(self.gpm.iter().find(|t| t.0 == key).unwrap_or(&("", key)).1, value.unwrap_or("unset"));
                            break;
                        }
                    }
                    if !found {
                        err.issues.push(CLIError::UnknownParameter(key.to_string(),
                                                                  {let mut v = Vec::new();
                                                                           v.extend(self.gp.iter().map(|v|*v));
                                                                           v } ));
                    }
                }
            }
        }
        let protocol = CallType::Standard;
        if dry_run {
            Ok(())
        } else {
            assert!(err.issues.len() == 0);
            for scope in self.opt.values_of("url").map(|i|i.collect()).unwrap_or(Vec::new()).iter() {
                call = call.add_scope(scope);
            }
            let mut ostream = match writer_from_opts(opt.value_of("out")) {
                Ok(mut f) => f,
                Err(io_err) => return Err(DoitError::IoError(opt.value_of("out").unwrap_or("-").to_string(), io_err)),
            };
            match match protocol {
                CallType::Standard => call.doit().await,
                _ => unreachable!()
            } {
                Err(api_err) => Err(DoitError::ApiError(api_err)),
                Ok((mut response, output_schema)) => {
                    let mut value = json::value::to_value(&output_schema).expect("serde to work");
                    remove_json_null_values(&mut value);
                    json::to_writer_pretty(&mut ostream, &value).unwrap();
                    ostream.flush().unwrap();
                    Ok(())
                }
            }
        }
    }

    async fn _projects_keys_delete(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        let mut call = self.hub.projects().keys_delete(opt.value_of("name").unwrap_or(""));
        for parg in opt.values_of("v").map(|i|i.collect()).unwrap_or(Vec::new()).iter() {
            let (key, value) = parse_kv_arg(&*parg, err, false);
            match key {
                _ => {
                    let mut found = false;
                    for param in &self.gp {
                        if key == *param {
                            found = true;
                            call = call.param(self.gpm.iter().find(|t| t.0 == key).unwrap_or(&("", key)).1, value.unwrap_or("unset"));
                            break;
                        }
                    }
                    if !found {
                        err.issues.push(CLIError::UnknownParameter(key.to_string(),
                                                                  {let mut v = Vec::new();
                                                                           v.extend(self.gp.iter().map(|v|*v));
                                                                           v } ));
                    }
                }
            }
        }
        let protocol = CallType::Standard;
        if dry_run {
            Ok(())
        } else {
            assert!(err.issues.len() == 0);
            for scope in self.opt.values_of("url").map(|i|i.collect()).unwrap_or(Vec::new()).iter() {
                call = call.add_scope(scope);
            }
            let mut ostream = match writer_from_opts(opt.value_of("out")) {
                Ok(mut f) => f,
                Err(io_err) => return Err(DoitError::IoError(opt.value_of("out").unwrap_or("-").to_string(), io_err)),
            };
            match match protocol {
                CallType::Standard => call.doit().await,
                _ => unreachable!()
            } {
                Err(api_err) => Err(DoitError::ApiError(api_err)),
                Ok((mut response, output_schema)) => {
                    let mut value = json::value::to_value(&output_schema).expect("serde to work");
                    remove_json_null_values(&mut value);
                    json::to_writer_pretty(&mut ostream, &value).unwrap();
                    ostream.flush().unwrap();
                    Ok(())
                }
            }
        }
    }

    async fn _projects_keys_get(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        let mut call = self.hub.projects().keys_get(opt.value_of("name").unwrap_or(""));
        for parg in opt.values_of("v").map(|i|i.collect()).unwrap_or(Vec::new()).iter() {
            let (key, value) = parse_kv_arg(&*parg, err, false);
            match key {
                _ => {
                    let mut found = false;
                    for param in &self.gp {
                        if key == *param {
                            found = true;
                            call = call.param(self.gpm.iter().find(|t| t.0 == key).unwrap_or(&("", key)).1, value.unwrap_or("unset"));
                            break;
                        }
                    }
                    if !found {
                        err.issues.push(CLIError::UnknownParameter(key.to_string(),
                                                                  {let mut v = Vec::new();
                                                                           v.extend(self.gp.iter().map(|v|*v));
                                                                           v } ));
                    }
                }
            }
        }
        let protocol = CallType::Standard;
        if dry_run {
            Ok(())
        } else {
            assert!(err.issues.len() == 0);
            for scope in self.opt.values_of("url").map(|i|i.collect()).unwrap_or(Vec::new()).iter() {
                call = call.add_scope(scope);
            }
            let mut ostream = match writer_from_opts(opt.value_of("out")) {
                Ok(mut f) => f,
                Err(io_err) => return Err(DoitError::IoError(opt.value_of("out").unwrap_or("-").to_string(), io_err)),
            };
            match match protocol {
                CallType::Standard => call.doit().await,
                _ => unreachable!()
            } {
                Err(api_err) => Err(DoitError::ApiError(api_err)),
                Ok((mut response, output_schema)) => {
                    let mut value = json::value::to_value(&output_schema).expect("serde to work");
                    remove_json_null_values(&mut value);
                    json::to_writer_pretty(&mut ostream, &value).unwrap();
                    ostream.flush().unwrap();
                    Ok(())
                }
            }
        }
    }

    async fn _projects_keys_get_metrics(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        let mut call = self.hub.projects().keys_get_metrics(opt.value_of("name").unwrap_or(""));
        for parg in opt.values_of("v").map(|i|i.collect()).unwrap_or(Vec::new()).iter() {
            let (key, value) = parse_kv_arg(&*parg, err, false);
            match key {
                _ => {
                    let mut found = false;
                    for param in &self.gp {
                        if key == *param {
                            found = true;
                            call = call.param(self.gpm.iter().find(|t| t.0 == key).unwrap_or(&("", key)).1, value.unwrap_or("unset"));
                            break;
                        }
                    }
                    if !found {
                        err.issues.push(CLIError::UnknownParameter(key.to_string(),
                                                                  {let mut v = Vec::new();
                                                                           v.extend(self.gp.iter().map(|v|*v));
                                                                           v } ));
                    }
                }
            }
        }
        let protocol = CallType::Standard;
        if dry_run {
            Ok(())
        } else {
            assert!(err.issues.len() == 0);
            for scope in self.opt.values_of("url").map(|i|i.collect()).unwrap_or(Vec::new()).iter() {
                call = call.add_scope(scope);
            }
            let mut ostream = match writer_from_opts(opt.value_of("out")) {
                Ok(mut f) => f,
                Err(io_err) => return Err(DoitError::IoError(opt.value_of("out").unwrap_or("-").to_string(), io_err)),
            };
            match match protocol {
                CallType::Standard => call.doit().await,
                _ => unreachable!()
            } {
                Err(api_err) => Err(DoitError::ApiError(api_err)),
                Ok((mut response, output_schema)) => {
                    let mut value = json::value::to_value(&output_schema).expect("serde to work");
                    remove_json_null_values(&mut value);
                    json::to_writer_pretty(&mut ostream, &value).unwrap();
                    ostream.flush().unwrap();
                    Ok(())
                }
            }
        }
    }

    async fn _projects_keys_list(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        let mut call = self.hub.projects().keys_list(opt.value_of("parent").unwrap_or(""));
        for parg in opt.values_of("v").map(|i|i.collect()).unwrap_or(Vec::new()).iter() {
            let (key, value) = parse_kv_arg(&*parg, err, false);
            match key {
                "page-token" => {
                    call = call.page_token(value.unwrap_or(""));
                },
                "page-size" => {
                    call = call.page_size(        value.map(|v| arg_from_str(v, err, "page-size", "int32")).unwrap_or(-0));
                },
                _ => {
                    let mut found = false;
                    for param in &self.gp {
                        if key == *param {
                            found = true;
                            call = call.param(self.gpm.iter().find(|t| t.0 == key).unwrap_or(&("", key)).1, value.unwrap_or("unset"));
                            break;
                        }
                    }
                    if !found {
                        err.issues.push(CLIError::UnknownParameter(key.to_string(),
                                                                  {let mut v = Vec::new();
                                                                           v.extend(self.gp.iter().map(|v|*v));
                                                                           v.extend(["page-size", "page-token"].iter().map(|v|*v));
                                                                           v } ));
                    }
                }
            }
        }
        let protocol = CallType::Standard;
        if dry_run {
            Ok(())
        } else {
            assert!(err.issues.len() == 0);
            for scope in self.opt.values_of("url").map(|i|i.collect()).unwrap_or(Vec::new()).iter() {
                call = call.add_scope(scope);
            }
            let mut ostream = match writer_from_opts(opt.value_of("out")) {
                Ok(mut f) => f,
                Err(io_err) => return Err(DoitError::IoError(opt.value_of("out").unwrap_or("-").to_string(), io_err)),
            };
            match match protocol {
                CallType::Standard => call.doit().await,
                _ => unreachable!()
            } {
                Err(api_err) => Err(DoitError::ApiError(api_err)),
                Ok((mut response, output_schema)) => {
                    let mut value = json::value::to_value(&output_schema).expect("serde to work");
                    remove_json_null_values(&mut value);
                    json::to_writer_pretty(&mut ostream, &value).unwrap();
                    ostream.flush().unwrap();
                    Ok(())
                }
            }
        }
    }

    async fn _projects_keys_migrate(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        
        let mut field_cursor = FieldCursor::default();
        let mut object = json::value::Value::Object(Default::default());
        
        for kvarg in opt.values_of("kv").map(|i|i.collect()).unwrap_or(Vec::new()).iter() {
            let last_errc = err.issues.len();
            let (key, value) = parse_kv_arg(&*kvarg, err, false);
            let mut temp_cursor = field_cursor.clone();
            if let Err(field_err) = temp_cursor.set(&*key) {
                err.issues.push(field_err);
            }
            if value.is_none() {
                field_cursor = temp_cursor.clone();
                if err.issues.len() > last_errc {
                    err.issues.remove(last_errc);
                }
                continue;
            }
        
            let type_info: Option<(&'static str, JsonTypeInfo)> =
                match &temp_cursor.to_string()[..] {
                    "skip-billing-check" => Some(("skipBillingCheck", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    _ => {
                        let suggestion = FieldCursor::did_you_mean(key, &vec!["skip-billing-check"]);
                        err.issues.push(CLIError::Field(FieldError::Unknown(temp_cursor.to_string(), suggestion, value.map(|v| v.to_string()))));
                        None
                    }
                };
            if let Some((field_cursor_str, type_info)) = type_info {
                FieldCursor::from(field_cursor_str).set_json_value(&mut object, value.unwrap(), type_info, err, &temp_cursor);
            }
        }
        let mut request: api::GoogleCloudRecaptchaenterpriseV1MigrateKeyRequest = json::value::from_value(object).unwrap();
        let mut call = self.hub.projects().keys_migrate(request, opt.value_of("name").unwrap_or(""));
        for parg in opt.values_of("v").map(|i|i.collect()).unwrap_or(Vec::new()).iter() {
            let (key, value) = parse_kv_arg(&*parg, err, false);
            match key {
                _ => {
                    let mut found = false;
                    for param in &self.gp {
                        if key == *param {
                            found = true;
                            call = call.param(self.gpm.iter().find(|t| t.0 == key).unwrap_or(&("", key)).1, value.unwrap_or("unset"));
                            break;
                        }
                    }
                    if !found {
                        err.issues.push(CLIError::UnknownParameter(key.to_string(),
                                                                  {let mut v = Vec::new();
                                                                           v.extend(self.gp.iter().map(|v|*v));
                                                                           v } ));
                    }
                }
            }
        }
        let protocol = CallType::Standard;
        if dry_run {
            Ok(())
        } else {
            assert!(err.issues.len() == 0);
            for scope in self.opt.values_of("url").map(|i|i.collect()).unwrap_or(Vec::new()).iter() {
                call = call.add_scope(scope);
            }
            let mut ostream = match writer_from_opts(opt.value_of("out")) {
                Ok(mut f) => f,
                Err(io_err) => return Err(DoitError::IoError(opt.value_of("out").unwrap_or("-").to_string(), io_err)),
            };
            match match protocol {
                CallType::Standard => call.doit().await,
                _ => unreachable!()
            } {
                Err(api_err) => Err(DoitError::ApiError(api_err)),
                Ok((mut response, output_schema)) => {
                    let mut value = json::value::to_value(&output_schema).expect("serde to work");
                    remove_json_null_values(&mut value);
                    json::to_writer_pretty(&mut ostream, &value).unwrap();
                    ostream.flush().unwrap();
                    Ok(())
                }
            }
        }
    }

    async fn _projects_keys_patch(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        
        let mut field_cursor = FieldCursor::default();
        let mut object = json::value::Value::Object(Default::default());
        
        for kvarg in opt.values_of("kv").map(|i|i.collect()).unwrap_or(Vec::new()).iter() {
            let last_errc = err.issues.len();
            let (key, value) = parse_kv_arg(&*kvarg, err, false);
            let mut temp_cursor = field_cursor.clone();
            if let Err(field_err) = temp_cursor.set(&*key) {
                err.issues.push(field_err);
            }
            if value.is_none() {
                field_cursor = temp_cursor.clone();
                if err.issues.len() > last_errc {
                    err.issues.remove(last_errc);
                }
                continue;
            }
        
            let type_info: Option<(&'static str, JsonTypeInfo)> =
                match &temp_cursor.to_string()[..] {
                    "android-settings.allow-all-package-names" => Some(("androidSettings.allowAllPackageNames", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "android-settings.allowed-package-names" => Some(("androidSettings.allowedPackageNames", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Vec })),
                    "android-settings.support-non-google-app-store-distribution" => Some(("androidSettings.supportNonGoogleAppStoreDistribution", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "create-time" => Some(("createTime", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "display-name" => Some(("displayName", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "ios-settings.allow-all-bundle-ids" => Some(("iosSettings.allowAllBundleIds", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "ios-settings.allowed-bundle-ids" => Some(("iosSettings.allowedBundleIds", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Vec })),
                    "ios-settings.apple-developer-id.key-id" => Some(("iosSettings.appleDeveloperId.keyId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "ios-settings.apple-developer-id.private-key" => Some(("iosSettings.appleDeveloperId.privateKey", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "ios-settings.apple-developer-id.team-id" => Some(("iosSettings.appleDeveloperId.teamId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "labels" => Some(("labels", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Map })),
                    "name" => Some(("name", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "testing-options.testing-challenge" => Some(("testingOptions.testingChallenge", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "testing-options.testing-score" => Some(("testingOptions.testingScore", JsonTypeInfo { jtype: JsonType::Float, ctype: ComplexType::Pod })),
                    "waf-settings.waf-feature" => Some(("wafSettings.wafFeature", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "waf-settings.waf-service" => Some(("wafSettings.wafService", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "web-settings.allow-all-domains" => Some(("webSettings.allowAllDomains", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "web-settings.allow-amp-traffic" => Some(("webSettings.allowAmpTraffic", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "web-settings.allowed-domains" => Some(("webSettings.allowedDomains", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Vec })),
                    "web-settings.challenge-security-preference" => Some(("webSettings.challengeSecurityPreference", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "web-settings.integration-type" => Some(("webSettings.integrationType", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    _ => {
                        let suggestion = FieldCursor::did_you_mean(key, &vec!["allow-all-bundle-ids", "allow-all-domains", "allow-all-package-names", "allow-amp-traffic", "allowed-bundle-ids", "allowed-domains", "allowed-package-names", "android-settings", "apple-developer-id", "challenge-security-preference", "create-time", "display-name", "integration-type", "ios-settings", "key-id", "labels", "name", "private-key", "support-non-google-app-store-distribution", "team-id", "testing-challenge", "testing-options", "testing-score", "waf-feature", "waf-service", "waf-settings", "web-settings"]);
                        err.issues.push(CLIError::Field(FieldError::Unknown(temp_cursor.to_string(), suggestion, value.map(|v| v.to_string()))));
                        None
                    }
                };
            if let Some((field_cursor_str, type_info)) = type_info {
                FieldCursor::from(field_cursor_str).set_json_value(&mut object, value.unwrap(), type_info, err, &temp_cursor);
            }
        }
        let mut request: api::GoogleCloudRecaptchaenterpriseV1Key = json::value::from_value(object).unwrap();
        let mut call = self.hub.projects().keys_patch(request, opt.value_of("name").unwrap_or(""));
        for parg in opt.values_of("v").map(|i|i.collect()).unwrap_or(Vec::new()).iter() {
            let (key, value) = parse_kv_arg(&*parg, err, false);
            match key {
                "update-mask" => {
                    call = call.update_mask(        value.map(|v| arg_from_str(v, err, "update-mask", "google-fieldmask")).unwrap_or(FieldMask::default()));
                },
                _ => {
                    let mut found = false;
                    for param in &self.gp {
                        if key == *param {
                            found = true;
                            call = call.param(self.gpm.iter().find(|t| t.0 == key).unwrap_or(&("", key)).1, value.unwrap_or("unset"));
                            break;
                        }
                    }
                    if !found {
                        err.issues.push(CLIError::UnknownParameter(key.to_string(),
                                                                  {let mut v = Vec::new();
                                                                           v.extend(self.gp.iter().map(|v|*v));
                                                                           v.extend(["update-mask"].iter().map(|v|*v));
                                                                           v } ));
                    }
                }
            }
        }
        let protocol = CallType::Standard;
        if dry_run {
            Ok(())
        } else {
            assert!(err.issues.len() == 0);
            for scope in self.opt.values_of("url").map(|i|i.collect()).unwrap_or(Vec::new()).iter() {
                call = call.add_scope(scope);
            }
            let mut ostream = match writer_from_opts(opt.value_of("out")) {
                Ok(mut f) => f,
                Err(io_err) => return Err(DoitError::IoError(opt.value_of("out").unwrap_or("-").to_string(), io_err)),
            };
            match match protocol {
                CallType::Standard => call.doit().await,
                _ => unreachable!()
            } {
                Err(api_err) => Err(DoitError::ApiError(api_err)),
                Ok((mut response, output_schema)) => {
                    let mut value = json::value::to_value(&output_schema).expect("serde to work");
                    remove_json_null_values(&mut value);
                    json::to_writer_pretty(&mut ostream, &value).unwrap();
                    ostream.flush().unwrap();
                    Ok(())
                }
            }
        }
    }

    async fn _projects_keys_retrieve_legacy_secret_key(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        let mut call = self.hub.projects().keys_retrieve_legacy_secret_key(opt.value_of("key").unwrap_or(""));
        for parg in opt.values_of("v").map(|i|i.collect()).unwrap_or(Vec::new()).iter() {
            let (key, value) = parse_kv_arg(&*parg, err, false);
            match key {
                _ => {
                    let mut found = false;
                    for param in &self.gp {
                        if key == *param {
                            found = true;
                            call = call.param(self.gpm.iter().find(|t| t.0 == key).unwrap_or(&("", key)).1, value.unwrap_or("unset"));
                            break;
                        }
                    }
                    if !found {
                        err.issues.push(CLIError::UnknownParameter(key.to_string(),
                                                                  {let mut v = Vec::new();
                                                                           v.extend(self.gp.iter().map(|v|*v));
                                                                           v } ));
                    }
                }
            }
        }
        let protocol = CallType::Standard;
        if dry_run {
            Ok(())
        } else {
            assert!(err.issues.len() == 0);
            for scope in self.opt.values_of("url").map(|i|i.collect()).unwrap_or(Vec::new()).iter() {
                call = call.add_scope(scope);
            }
            let mut ostream = match writer_from_opts(opt.value_of("out")) {
                Ok(mut f) => f,
                Err(io_err) => return Err(DoitError::IoError(opt.value_of("out").unwrap_or("-").to_string(), io_err)),
            };
            match match protocol {
                CallType::Standard => call.doit().await,
                _ => unreachable!()
            } {
                Err(api_err) => Err(DoitError::ApiError(api_err)),
                Ok((mut response, output_schema)) => {
                    let mut value = json::value::to_value(&output_schema).expect("serde to work");
                    remove_json_null_values(&mut value);
                    json::to_writer_pretty(&mut ostream, &value).unwrap();
                    ostream.flush().unwrap();
                    Ok(())
                }
            }
        }
    }

    async fn _projects_relatedaccountgroupmemberships_search(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        
        let mut field_cursor = FieldCursor::default();
        let mut object = json::value::Value::Object(Default::default());
        
        for kvarg in opt.values_of("kv").map(|i|i.collect()).unwrap_or(Vec::new()).iter() {
            let last_errc = err.issues.len();
            let (key, value) = parse_kv_arg(&*kvarg, err, false);
            let mut temp_cursor = field_cursor.clone();
            if let Err(field_err) = temp_cursor.set(&*key) {
                err.issues.push(field_err);
            }
            if value.is_none() {
                field_cursor = temp_cursor.clone();
                if err.issues.len() > last_errc {
                    err.issues.remove(last_errc);
                }
                continue;
            }
        
            let type_info: Option<(&'static str, JsonTypeInfo)> =
                match &temp_cursor.to_string()[..] {
                    "account-id" => Some(("accountId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "hashed-account-id" => Some(("hashedAccountId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "page-size" => Some(("pageSize", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "page-token" => Some(("pageToken", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    _ => {
                        let suggestion = FieldCursor::did_you_mean(key, &vec!["account-id", "hashed-account-id", "page-size", "page-token"]);
                        err.issues.push(CLIError::Field(FieldError::Unknown(temp_cursor.to_string(), suggestion, value.map(|v| v.to_string()))));
                        None
                    }
                };
            if let Some((field_cursor_str, type_info)) = type_info {
                FieldCursor::from(field_cursor_str).set_json_value(&mut object, value.unwrap(), type_info, err, &temp_cursor);
            }
        }
        let mut request: api::GoogleCloudRecaptchaenterpriseV1SearchRelatedAccountGroupMembershipsRequest = json::value::from_value(object).unwrap();
        let mut call = self.hub.projects().relatedaccountgroupmemberships_search(request, opt.value_of("project").unwrap_or(""));
        for parg in opt.values_of("v").map(|i|i.collect()).unwrap_or(Vec::new()).iter() {
            let (key, value) = parse_kv_arg(&*parg, err, false);
            match key {
                _ => {
                    let mut found = false;
                    for param in &self.gp {
                        if key == *param {
                            found = true;
                            call = call.param(self.gpm.iter().find(|t| t.0 == key).unwrap_or(&("", key)).1, value.unwrap_or("unset"));
                            break;
                        }
                    }
                    if !found {
                        err.issues.push(CLIError::UnknownParameter(key.to_string(),
                                                                  {let mut v = Vec::new();
                                                                           v.extend(self.gp.iter().map(|v|*v));
                                                                           v } ));
                    }
                }
            }
        }
        let protocol = CallType::Standard;
        if dry_run {
            Ok(())
        } else {
            assert!(err.issues.len() == 0);
            for scope in self.opt.values_of("url").map(|i|i.collect()).unwrap_or(Vec::new()).iter() {
                call = call.add_scope(scope);
            }
            let mut ostream = match writer_from_opts(opt.value_of("out")) {
                Ok(mut f) => f,
                Err(io_err) => return Err(DoitError::IoError(opt.value_of("out").unwrap_or("-").to_string(), io_err)),
            };
            match match protocol {
                CallType::Standard => call.doit().await,
                _ => unreachable!()
            } {
                Err(api_err) => Err(DoitError::ApiError(api_err)),
                Ok((mut response, output_schema)) => {
                    let mut value = json::value::to_value(&output_schema).expect("serde to work");
                    remove_json_null_values(&mut value);
                    json::to_writer_pretty(&mut ostream, &value).unwrap();
                    ostream.flush().unwrap();
                    Ok(())
                }
            }
        }
    }

    async fn _projects_relatedaccountgroups_list(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        let mut call = self.hub.projects().relatedaccountgroups_list(opt.value_of("parent").unwrap_or(""));
        for parg in opt.values_of("v").map(|i|i.collect()).unwrap_or(Vec::new()).iter() {
            let (key, value) = parse_kv_arg(&*parg, err, false);
            match key {
                "page-token" => {
                    call = call.page_token(value.unwrap_or(""));
                },
                "page-size" => {
                    call = call.page_size(        value.map(|v| arg_from_str(v, err, "page-size", "int32")).unwrap_or(-0));
                },
                _ => {
                    let mut found = false;
                    for param in &self.gp {
                        if key == *param {
                            found = true;
                            call = call.param(self.gpm.iter().find(|t| t.0 == key).unwrap_or(&("", key)).1, value.unwrap_or("unset"));
                            break;
                        }
                    }
                    if !found {
                        err.issues.push(CLIError::UnknownParameter(key.to_string(),
                                                                  {let mut v = Vec::new();
                                                                           v.extend(self.gp.iter().map(|v|*v));
                                                                           v.extend(["page-size", "page-token"].iter().map(|v|*v));
                                                                           v } ));
                    }
                }
            }
        }
        let protocol = CallType::Standard;
        if dry_run {
            Ok(())
        } else {
            assert!(err.issues.len() == 0);
            for scope in self.opt.values_of("url").map(|i|i.collect()).unwrap_or(Vec::new()).iter() {
                call = call.add_scope(scope);
            }
            let mut ostream = match writer_from_opts(opt.value_of("out")) {
                Ok(mut f) => f,
                Err(io_err) => return Err(DoitError::IoError(opt.value_of("out").unwrap_or("-").to_string(), io_err)),
            };
            match match protocol {
                CallType::Standard => call.doit().await,
                _ => unreachable!()
            } {
                Err(api_err) => Err(DoitError::ApiError(api_err)),
                Ok((mut response, output_schema)) => {
                    let mut value = json::value::to_value(&output_schema).expect("serde to work");
                    remove_json_null_values(&mut value);
                    json::to_writer_pretty(&mut ostream, &value).unwrap();
                    ostream.flush().unwrap();
                    Ok(())
                }
            }
        }
    }

    async fn _projects_relatedaccountgroups_memberships_list(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        let mut call = self.hub.projects().relatedaccountgroups_memberships_list(opt.value_of("parent").unwrap_or(""));
        for parg in opt.values_of("v").map(|i|i.collect()).unwrap_or(Vec::new()).iter() {
            let (key, value) = parse_kv_arg(&*parg, err, false);
            match key {
                "page-token" => {
                    call = call.page_token(value.unwrap_or(""));
                },
                "page-size" => {
                    call = call.page_size(        value.map(|v| arg_from_str(v, err, "page-size", "int32")).unwrap_or(-0));
                },
                _ => {
                    let mut found = false;
                    for param in &self.gp {
                        if key == *param {
                            found = true;
                            call = call.param(self.gpm.iter().find(|t| t.0 == key).unwrap_or(&("", key)).1, value.unwrap_or("unset"));
                            break;
                        }
                    }
                    if !found {
                        err.issues.push(CLIError::UnknownParameter(key.to_string(),
                                                                  {let mut v = Vec::new();
                                                                           v.extend(self.gp.iter().map(|v|*v));
                                                                           v.extend(["page-size", "page-token"].iter().map(|v|*v));
                                                                           v } ));
                    }
                }
            }
        }
        let protocol = CallType::Standard;
        if dry_run {
            Ok(())
        } else {
            assert!(err.issues.len() == 0);
            for scope in self.opt.values_of("url").map(|i|i.collect()).unwrap_or(Vec::new()).iter() {
                call = call.add_scope(scope);
            }
            let mut ostream = match writer_from_opts(opt.value_of("out")) {
                Ok(mut f) => f,
                Err(io_err) => return Err(DoitError::IoError(opt.value_of("out").unwrap_or("-").to_string(), io_err)),
            };
            match match protocol {
                CallType::Standard => call.doit().await,
                _ => unreachable!()
            } {
                Err(api_err) => Err(DoitError::ApiError(api_err)),
                Ok((mut response, output_schema)) => {
                    let mut value = json::value::to_value(&output_schema).expect("serde to work");
                    remove_json_null_values(&mut value);
                    json::to_writer_pretty(&mut ostream, &value).unwrap();
                    ostream.flush().unwrap();
                    Ok(())
                }
            }
        }
    }

    async fn _doit(&self, dry_run: bool) -> Result<Result<(), DoitError>, Option<InvalidOptionsError>> {
        let mut err = InvalidOptionsError::new();
        let mut call_result: Result<(), DoitError> = Ok(());
        let mut err_opt: Option<InvalidOptionsError> = None;
        match self.opt.subcommand() {
            ("projects", Some(opt)) => {
                match opt.subcommand() {
                    ("assessments-annotate", Some(opt)) => {
                        call_result = self._projects_assessments_annotate(opt, dry_run, &mut err).await;
                    },
                    ("assessments-create", Some(opt)) => {
                        call_result = self._projects_assessments_create(opt, dry_run, &mut err).await;
                    },
                    ("firewallpolicies-create", Some(opt)) => {
                        call_result = self._projects_firewallpolicies_create(opt, dry_run, &mut err).await;
                    },
                    ("firewallpolicies-delete", Some(opt)) => {
                        call_result = self._projects_firewallpolicies_delete(opt, dry_run, &mut err).await;
                    },
                    ("firewallpolicies-get", Some(opt)) => {
                        call_result = self._projects_firewallpolicies_get(opt, dry_run, &mut err).await;
                    },
                    ("firewallpolicies-list", Some(opt)) => {
                        call_result = self._projects_firewallpolicies_list(opt, dry_run, &mut err).await;
                    },
                    ("firewallpolicies-patch", Some(opt)) => {
                        call_result = self._projects_firewallpolicies_patch(opt, dry_run, &mut err).await;
                    },
                    ("firewallpolicies-reorder", Some(opt)) => {
                        call_result = self._projects_firewallpolicies_reorder(opt, dry_run, &mut err).await;
                    },
                    ("keys-create", Some(opt)) => {
                        call_result = self._projects_keys_create(opt, dry_run, &mut err).await;
                    },
                    ("keys-delete", Some(opt)) => {
                        call_result = self._projects_keys_delete(opt, dry_run, &mut err).await;
                    },
                    ("keys-get", Some(opt)) => {
                        call_result = self._projects_keys_get(opt, dry_run, &mut err).await;
                    },
                    ("keys-get-metrics", Some(opt)) => {
                        call_result = self._projects_keys_get_metrics(opt, dry_run, &mut err).await;
                    },
                    ("keys-list", Some(opt)) => {
                        call_result = self._projects_keys_list(opt, dry_run, &mut err).await;
                    },
                    ("keys-migrate", Some(opt)) => {
                        call_result = self._projects_keys_migrate(opt, dry_run, &mut err).await;
                    },
                    ("keys-patch", Some(opt)) => {
                        call_result = self._projects_keys_patch(opt, dry_run, &mut err).await;
                    },
                    ("keys-retrieve-legacy-secret-key", Some(opt)) => {
                        call_result = self._projects_keys_retrieve_legacy_secret_key(opt, dry_run, &mut err).await;
                    },
                    ("relatedaccountgroupmemberships-search", Some(opt)) => {
                        call_result = self._projects_relatedaccountgroupmemberships_search(opt, dry_run, &mut err).await;
                    },
                    ("relatedaccountgroups-list", Some(opt)) => {
                        call_result = self._projects_relatedaccountgroups_list(opt, dry_run, &mut err).await;
                    },
                    ("relatedaccountgroups-memberships-list", Some(opt)) => {
                        call_result = self._projects_relatedaccountgroups_memberships_list(opt, dry_run, &mut err).await;
                    },
                    _ => {
                        err.issues.push(CLIError::MissingMethodError("projects".to_string()));
                        writeln!(io::stderr(), "{}\n", opt.usage()).ok();
                    }
                }
            },
            _ => {
                err.issues.push(CLIError::MissingCommandError);
                writeln!(io::stderr(), "{}\n", self.opt.usage()).ok();
            }
        }

        if dry_run {
            if err.issues.len() > 0 {
                err_opt = Some(err);
            }
            Err(err_opt)
        } else {
            Ok(call_result)
        }
    }

    // Please note that this call will fail if any part of the opt can't be handled
    async fn new(opt: ArgMatches<'n>, connector: S) -> Result<Engine<'n, S>, InvalidOptionsError> {
        let (config_dir, secret) = {
            let config_dir = match client::assure_config_dir_exists(opt.value_of("folder").unwrap_or("~/.google-service-cli")) {
                Err(e) => return Err(InvalidOptionsError::single(e, 3)),
                Ok(p) => p,
            };

            match client::application_secret_from_directory(&config_dir, "recaptchaenterprise1-secret.json",
                                                         "{\"installed\":{\"auth_uri\":\"https://accounts.google.com/o/oauth2/auth\",\"client_secret\":\"hCsslbCUyfehWMmbkG8vTYxG\",\"token_uri\":\"https://accounts.google.com/o/oauth2/token\",\"client_email\":\"\",\"redirect_uris\":[\"urn:ietf:wg:oauth:2.0:oob\",\"oob\"],\"client_x509_cert_url\":\"\",\"client_id\":\"620010449518-9ngf7o4dhs0dka470npqvor6dc5lqb9b.apps.googleusercontent.com\",\"auth_provider_x509_cert_url\":\"https://www.googleapis.com/oauth2/v1/certs\"}}") {
                Ok(secret) => (config_dir, secret),
                Err(e) => return Err(InvalidOptionsError::single(e, 4))
            }
        };

        let client = hyper::Client::builder().build(connector);

        let auth = oauth2::InstalledFlowAuthenticator::with_client(
            secret,
            oauth2::InstalledFlowReturnMethod::HTTPRedirect,
            client.clone(),
        ).persist_tokens_to_disk(format!("{}/recaptchaenterprise1", config_dir)).build().await.unwrap();

        let engine = Engine {
            opt: opt,
            hub: api::RecaptchaEnterprise::new(client, auth),
            gp: vec!["$-xgafv", "access-token", "alt", "callback", "fields", "key", "oauth-token", "pretty-print", "quota-user", "upload-type", "upload-protocol"],
            gpm: vec![
                    ("$-xgafv", "$.xgafv"),
                    ("access-token", "access_token"),
                    ("oauth-token", "oauth_token"),
                    ("pretty-print", "prettyPrint"),
                    ("quota-user", "quotaUser"),
                    ("upload-type", "uploadType"),
                    ("upload-protocol", "upload_protocol"),
                ]
        };

        match engine._doit(true).await {
            Err(Some(err)) => Err(err),
            Err(None)      => Ok(engine),
            Ok(_)          => unreachable!(),
        }
    }

    async fn doit(&self) -> Result<(), DoitError> {
        match self._doit(false).await {
            Ok(res) => res,
            Err(_) => unreachable!(),
        }
    }
}

#[tokio::main]
async fn main() {
    let mut exit_status = 0i32;
    let arg_data = [
        ("projects", "methods: 'assessments-annotate', 'assessments-create', 'firewallpolicies-create', 'firewallpolicies-delete', 'firewallpolicies-get', 'firewallpolicies-list', 'firewallpolicies-patch', 'firewallpolicies-reorder', 'keys-create', 'keys-delete', 'keys-get', 'keys-get-metrics', 'keys-list', 'keys-migrate', 'keys-patch', 'keys-retrieve-legacy-secret-key', 'relatedaccountgroupmemberships-search', 'relatedaccountgroups-list' and 'relatedaccountgroups-memberships-list'", vec![
            ("assessments-annotate",
                    Some(r##"Annotates a previously created Assessment to provide additional information on whether the event turned out to be authentic or fraudulent."##),
                    "Details at http://byron.github.io/google-apis-rs/google_recaptchaenterprise1_cli/projects_assessments-annotate",
                  vec![
                    (Some(r##"name"##),
                     None,
                     Some(r##"Required. The resource name of the Assessment, in the format `projects/{project}/assessments/{assessment}`."##),
                     Some(true),
                     Some(false)),
        
                    (Some(r##"kv"##),
                     Some(r##"r"##),
                     Some(r##"Set various fields of the request structure, matching the key=value form"##),
                     Some(true),
                     Some(true)),
        
                    (Some(r##"v"##),
                     Some(r##"p"##),
                     Some(r##"Set various optional parameters, matching the key=value form"##),
                     Some(false),
                     Some(true)),
        
                    (Some(r##"out"##),
                     Some(r##"o"##),
                     Some(r##"Specify the file into which to write the program's output"##),
                     Some(false),
                     Some(false)),
                  ]),
            ("assessments-create",
                    Some(r##"Creates an Assessment of the likelihood an event is legitimate."##),
                    "Details at http://byron.github.io/google-apis-rs/google_recaptchaenterprise1_cli/projects_assessments-create",
                  vec![
                    (Some(r##"parent"##),
                     None,
                     Some(r##"Required. The name of the project in which the assessment will be created, in the format `projects/{project}`."##),
                     Some(true),
                     Some(false)),
        
                    (Some(r##"kv"##),
                     Some(r##"r"##),
                     Some(r##"Set various fields of the request structure, matching the key=value form"##),
                     Some(true),
                     Some(true)),
        
                    (Some(r##"v"##),
                     Some(r##"p"##),
                     Some(r##"Set various optional parameters, matching the key=value form"##),
                     Some(false),
                     Some(true)),
        
                    (Some(r##"out"##),
                     Some(r##"o"##),
                     Some(r##"Specify the file into which to write the program's output"##),
                     Some(false),
                     Some(false)),
                  ]),
            ("firewallpolicies-create",
                    Some(r##"Creates a new FirewallPolicy, specifying conditions at which reCAPTCHA Enterprise actions can be executed. A project may have a maximum of 1000 policies."##),
                    "Details at http://byron.github.io/google-apis-rs/google_recaptchaenterprise1_cli/projects_firewallpolicies-create",
                  vec![
                    (Some(r##"parent"##),
                     None,
                     Some(r##"Required. The name of the project this policy will apply to, in the format `projects/{project}`."##),
                     Some(true),
                     Some(false)),
        
                    (Some(r##"kv"##),
                     Some(r##"r"##),
                     Some(r##"Set various fields of the request structure, matching the key=value form"##),
                     Some(true),
                     Some(true)),
        
                    (Some(r##"v"##),
                     Some(r##"p"##),
                     Some(r##"Set various optional parameters, matching the key=value form"##),
                     Some(false),
                     Some(true)),
        
                    (Some(r##"out"##),
                     Some(r##"o"##),
                     Some(r##"Specify the file into which to write the program's output"##),
                     Some(false),
                     Some(false)),
                  ]),
            ("firewallpolicies-delete",
                    Some(r##"Deletes the specified firewall policy."##),
                    "Details at http://byron.github.io/google-apis-rs/google_recaptchaenterprise1_cli/projects_firewallpolicies-delete",
                  vec![
                    (Some(r##"name"##),
                     None,
                     Some(r##"Required. The name of the policy to be deleted, in the format `projects/{project}/firewallpolicies/{firewallpolicy}`."##),
                     Some(true),
                     Some(false)),
        
                    (Some(r##"v"##),
                     Some(r##"p"##),
                     Some(r##"Set various optional parameters, matching the key=value form"##),
                     Some(false),
                     Some(true)),
        
                    (Some(r##"out"##),
                     Some(r##"o"##),
                     Some(r##"Specify the file into which to write the program's output"##),
                     Some(false),
                     Some(false)),
                  ]),
            ("firewallpolicies-get",
                    Some(r##"Returns the specified firewall policy."##),
                    "Details at http://byron.github.io/google-apis-rs/google_recaptchaenterprise1_cli/projects_firewallpolicies-get",
                  vec![
                    (Some(r##"name"##),
                     None,
                     Some(r##"Required. The name of the requested policy, in the format `projects/{project}/firewallpolicies/{firewallpolicy}`."##),
                     Some(true),
                     Some(false)),
        
                    (Some(r##"v"##),
                     Some(r##"p"##),
                     Some(r##"Set various optional parameters, matching the key=value form"##),
                     Some(false),
                     Some(true)),
        
                    (Some(r##"out"##),
                     Some(r##"o"##),
                     Some(r##"Specify the file into which to write the program's output"##),
                     Some(false),
                     Some(false)),
                  ]),
            ("firewallpolicies-list",
                    Some(r##"Returns the list of all firewall policies that belong to a project."##),
                    "Details at http://byron.github.io/google-apis-rs/google_recaptchaenterprise1_cli/projects_firewallpolicies-list",
                  vec![
                    (Some(r##"parent"##),
                     None,
                     Some(r##"Required. The name of the project to list the policies for, in the format `projects/{project}`."##),
                     Some(true),
                     Some(false)),
        
                    (Some(r##"v"##),
                     Some(r##"p"##),
                     Some(r##"Set various optional parameters, matching the key=value form"##),
                     Some(false),
                     Some(true)),
        
                    (Some(r##"out"##),
                     Some(r##"o"##),
                     Some(r##"Specify the file into which to write the program's output"##),
                     Some(false),
                     Some(false)),
                  ]),
            ("firewallpolicies-patch",
                    Some(r##"Updates the specified firewall policy."##),
                    "Details at http://byron.github.io/google-apis-rs/google_recaptchaenterprise1_cli/projects_firewallpolicies-patch",
                  vec![
                    (Some(r##"name"##),
                     None,
                     Some(r##"Identifier. The resource name for the FirewallPolicy in the format `projects/{project}/firewallpolicies/{firewallpolicy}`."##),
                     Some(true),
                     Some(false)),
        
                    (Some(r##"kv"##),
                     Some(r##"r"##),
                     Some(r##"Set various fields of the request structure, matching the key=value form"##),
                     Some(true),
                     Some(true)),
        
                    (Some(r##"v"##),
                     Some(r##"p"##),
                     Some(r##"Set various optional parameters, matching the key=value form"##),
                     Some(false),
                     Some(true)),
        
                    (Some(r##"out"##),
                     Some(r##"o"##),
                     Some(r##"Specify the file into which to write the program's output"##),
                     Some(false),
                     Some(false)),
                  ]),
            ("firewallpolicies-reorder",
                    Some(r##"Reorders all firewall policies."##),
                    "Details at http://byron.github.io/google-apis-rs/google_recaptchaenterprise1_cli/projects_firewallpolicies-reorder",
                  vec![
                    (Some(r##"parent"##),
                     None,
                     Some(r##"Required. The name of the project to list the policies for, in the format `projects/{project}`."##),
                     Some(true),
                     Some(false)),
        
                    (Some(r##"kv"##),
                     Some(r##"r"##),
                     Some(r##"Set various fields of the request structure, matching the key=value form"##),
                     Some(true),
                     Some(true)),
        
                    (Some(r##"v"##),
                     Some(r##"p"##),
                     Some(r##"Set various optional parameters, matching the key=value form"##),
                     Some(false),
                     Some(true)),
        
                    (Some(r##"out"##),
                     Some(r##"o"##),
                     Some(r##"Specify the file into which to write the program's output"##),
                     Some(false),
                     Some(false)),
                  ]),
            ("keys-create",
                    Some(r##"Creates a new reCAPTCHA Enterprise key."##),
                    "Details at http://byron.github.io/google-apis-rs/google_recaptchaenterprise1_cli/projects_keys-create",
                  vec![
                    (Some(r##"parent"##),
                     None,
                     Some(r##"Required. The name of the project in which the key will be created, in the format `projects/{project}`."##),
                     Some(true),
                     Some(false)),
        
                    (Some(r##"kv"##),
                     Some(r##"r"##),
                     Some(r##"Set various fields of the request structure, matching the key=value form"##),
                     Some(true),
                     Some(true)),
        
                    (Some(r##"v"##),
                     Some(r##"p"##),
                     Some(r##"Set various optional parameters, matching the key=value form"##),
                     Some(false),
                     Some(true)),
        
                    (Some(r##"out"##),
                     Some(r##"o"##),
                     Some(r##"Specify the file into which to write the program's output"##),
                     Some(false),
                     Some(false)),
                  ]),
            ("keys-delete",
                    Some(r##"Deletes the specified key."##),
                    "Details at http://byron.github.io/google-apis-rs/google_recaptchaenterprise1_cli/projects_keys-delete",
                  vec![
                    (Some(r##"name"##),
                     None,
                     Some(r##"Required. The name of the key to be deleted, in the format `projects/{project}/keys/{key}`."##),
                     Some(true),
                     Some(false)),
        
                    (Some(r##"v"##),
                     Some(r##"p"##),
                     Some(r##"Set various optional parameters, matching the key=value form"##),
                     Some(false),
                     Some(true)),
        
                    (Some(r##"out"##),
                     Some(r##"o"##),
                     Some(r##"Specify the file into which to write the program's output"##),
                     Some(false),
                     Some(false)),
                  ]),
            ("keys-get",
                    Some(r##"Returns the specified key."##),
                    "Details at http://byron.github.io/google-apis-rs/google_recaptchaenterprise1_cli/projects_keys-get",
                  vec![
                    (Some(r##"name"##),
                     None,
                     Some(r##"Required. The name of the requested key, in the format `projects/{project}/keys/{key}`."##),
                     Some(true),
                     Some(false)),
        
                    (Some(r##"v"##),
                     Some(r##"p"##),
                     Some(r##"Set various optional parameters, matching the key=value form"##),
                     Some(false),
                     Some(true)),
        
                    (Some(r##"out"##),
                     Some(r##"o"##),
                     Some(r##"Specify the file into which to write the program's output"##),
                     Some(false),
                     Some(false)),
                  ]),
            ("keys-get-metrics",
                    Some(r##"Get some aggregated metrics for a Key. This data can be used to build dashboards."##),
                    "Details at http://byron.github.io/google-apis-rs/google_recaptchaenterprise1_cli/projects_keys-get-metrics",
                  vec![
                    (Some(r##"name"##),
                     None,
                     Some(r##"Required. The name of the requested metrics, in the format `projects/{project}/keys/{key}/metrics`."##),
                     Some(true),
                     Some(false)),
        
                    (Some(r##"v"##),
                     Some(r##"p"##),
                     Some(r##"Set various optional parameters, matching the key=value form"##),
                     Some(false),
                     Some(true)),
        
                    (Some(r##"out"##),
                     Some(r##"o"##),
                     Some(r##"Specify the file into which to write the program's output"##),
                     Some(false),
                     Some(false)),
                  ]),
            ("keys-list",
                    Some(r##"Returns the list of all keys that belong to a project."##),
                    "Details at http://byron.github.io/google-apis-rs/google_recaptchaenterprise1_cli/projects_keys-list",
                  vec![
                    (Some(r##"parent"##),
                     None,
                     Some(r##"Required. The name of the project that contains the keys that will be listed, in the format `projects/{project}`."##),
                     Some(true),
                     Some(false)),
        
                    (Some(r##"v"##),
                     Some(r##"p"##),
                     Some(r##"Set various optional parameters, matching the key=value form"##),
                     Some(false),
                     Some(true)),
        
                    (Some(r##"out"##),
                     Some(r##"o"##),
                     Some(r##"Specify the file into which to write the program's output"##),
                     Some(false),
                     Some(false)),
                  ]),
            ("keys-migrate",
                    Some(r##"Migrates an existing key from reCAPTCHA to reCAPTCHA Enterprise. Once a key is migrated, it can be used from either product. SiteVerify requests are billed as CreateAssessment calls. You must be authenticated as one of the current owners of the reCAPTCHA Key, and your user must have the reCAPTCHA Enterprise Admin IAM role in the destination project."##),
                    "Details at http://byron.github.io/google-apis-rs/google_recaptchaenterprise1_cli/projects_keys-migrate",
                  vec![
                    (Some(r##"name"##),
                     None,
                     Some(r##"Required. The name of the key to be migrated, in the format `projects/{project}/keys/{key}`."##),
                     Some(true),
                     Some(false)),
        
                    (Some(r##"kv"##),
                     Some(r##"r"##),
                     Some(r##"Set various fields of the request structure, matching the key=value form"##),
                     Some(true),
                     Some(true)),
        
                    (Some(r##"v"##),
                     Some(r##"p"##),
                     Some(r##"Set various optional parameters, matching the key=value form"##),
                     Some(false),
                     Some(true)),
        
                    (Some(r##"out"##),
                     Some(r##"o"##),
                     Some(r##"Specify the file into which to write the program's output"##),
                     Some(false),
                     Some(false)),
                  ]),
            ("keys-patch",
                    Some(r##"Updates the specified key."##),
                    "Details at http://byron.github.io/google-apis-rs/google_recaptchaenterprise1_cli/projects_keys-patch",
                  vec![
                    (Some(r##"name"##),
                     None,
                     Some(r##"Identifier. The resource name for the Key in the format `projects/{project}/keys/{key}`."##),
                     Some(true),
                     Some(false)),
        
                    (Some(r##"kv"##),
                     Some(r##"r"##),
                     Some(r##"Set various fields of the request structure, matching the key=value form"##),
                     Some(true),
                     Some(true)),
        
                    (Some(r##"v"##),
                     Some(r##"p"##),
                     Some(r##"Set various optional parameters, matching the key=value form"##),
                     Some(false),
                     Some(true)),
        
                    (Some(r##"out"##),
                     Some(r##"o"##),
                     Some(r##"Specify the file into which to write the program's output"##),
                     Some(false),
                     Some(false)),
                  ]),
            ("keys-retrieve-legacy-secret-key",
                    Some(r##"Returns the secret key related to the specified public key. You must use the legacy secret key only in a 3rd party integration with legacy reCAPTCHA."##),
                    "Details at http://byron.github.io/google-apis-rs/google_recaptchaenterprise1_cli/projects_keys-retrieve-legacy-secret-key",
                  vec![
                    (Some(r##"key"##),
                     None,
                     Some(r##"Required. The public key name linked to the requested secret key in the format `projects/{project}/keys/{key}`."##),
                     Some(true),
                     Some(false)),
        
                    (Some(r##"v"##),
                     Some(r##"p"##),
                     Some(r##"Set various optional parameters, matching the key=value form"##),
                     Some(false),
                     Some(true)),
        
                    (Some(r##"out"##),
                     Some(r##"o"##),
                     Some(r##"Specify the file into which to write the program's output"##),
                     Some(false),
                     Some(false)),
                  ]),
            ("relatedaccountgroupmemberships-search",
                    Some(r##"Search group memberships related to a given account."##),
                    "Details at http://byron.github.io/google-apis-rs/google_recaptchaenterprise1_cli/projects_relatedaccountgroupmemberships-search",
                  vec![
                    (Some(r##"project"##),
                     None,
                     Some(r##"Required. The name of the project to search related account group memberships from. Specify the project name in the following format: `projects/{project}`."##),
                     Some(true),
                     Some(false)),
        
                    (Some(r##"kv"##),
                     Some(r##"r"##),
                     Some(r##"Set various fields of the request structure, matching the key=value form"##),
                     Some(true),
                     Some(true)),
        
                    (Some(r##"v"##),
                     Some(r##"p"##),
                     Some(r##"Set various optional parameters, matching the key=value form"##),
                     Some(false),
                     Some(true)),
        
                    (Some(r##"out"##),
                     Some(r##"o"##),
                     Some(r##"Specify the file into which to write the program's output"##),
                     Some(false),
                     Some(false)),
                  ]),
            ("relatedaccountgroups-list",
                    Some(r##"List groups of related accounts."##),
                    "Details at http://byron.github.io/google-apis-rs/google_recaptchaenterprise1_cli/projects_relatedaccountgroups-list",
                  vec![
                    (Some(r##"parent"##),
                     None,
                     Some(r##"Required. The name of the project to list related account groups from, in the format `projects/{project}`."##),
                     Some(true),
                     Some(false)),
        
                    (Some(r##"v"##),
                     Some(r##"p"##),
                     Some(r##"Set various optional parameters, matching the key=value form"##),
                     Some(false),
                     Some(true)),
        
                    (Some(r##"out"##),
                     Some(r##"o"##),
                     Some(r##"Specify the file into which to write the program's output"##),
                     Some(false),
                     Some(false)),
                  ]),
            ("relatedaccountgroups-memberships-list",
                    Some(r##"Get memberships in a group of related accounts."##),
                    "Details at http://byron.github.io/google-apis-rs/google_recaptchaenterprise1_cli/projects_relatedaccountgroups-memberships-list",
                  vec![
                    (Some(r##"parent"##),
                     None,
                     Some(r##"Required. The resource name for the related account group in the format `projects/{project}/relatedaccountgroups/{relatedaccountgroup}`."##),
                     Some(true),
                     Some(false)),
        
                    (Some(r##"v"##),
                     Some(r##"p"##),
                     Some(r##"Set various optional parameters, matching the key=value form"##),
                     Some(false),
                     Some(true)),
        
                    (Some(r##"out"##),
                     Some(r##"o"##),
                     Some(r##"Specify the file into which to write the program's output"##),
                     Some(false),
                     Some(false)),
                  ]),
            ]),
        
    ];
    
    let mut app = App::new("recaptchaenterprise1")
           .author("Sebastian Thiel <byronimo@gmail.com>")
           .version("5.0.5+20240623")
           .about("Help protect your website from fraudulent activity, spam, and abuse without creating friction.")
           .after_help("All documentation details can be found at http://byron.github.io/google-apis-rs/google_recaptchaenterprise1_cli")
           .arg(Arg::with_name("url")
                   .long("scope")
                   .help("Specify the authentication a method should be executed in. Each scope requires the user to grant this application permission to use it.If unset, it defaults to the shortest scope url for a particular method.")
                   .multiple(true)
                   .takes_value(true))
           .arg(Arg::with_name("folder")
                   .long("config-dir")
                   .help("A directory into which we will store our persistent data. Defaults to a user-writable directory that we will create during the first invocation.[default: ~/.google-service-cli")
                   .multiple(false)
                   .takes_value(true))
           .arg(Arg::with_name("debug")
                   .long("debug")
                   .help("Debug print all errors")
                   .multiple(false)
                   .takes_value(false));
           
           for &(main_command_name, about, ref subcommands) in arg_data.iter() {
               let mut mcmd = SubCommand::with_name(main_command_name).about(about);
           
               for &(sub_command_name, ref desc, url_info, ref args) in subcommands {
                   let mut scmd = SubCommand::with_name(sub_command_name);
                   if let &Some(desc) = desc {
                       scmd = scmd.about(desc);
                   }
                   scmd = scmd.after_help(url_info);
           
                   for &(ref arg_name, ref flag, ref desc, ref required, ref multi) in args {
                       let arg_name_str =
                           match (arg_name, flag) {
                                   (&Some(an), _       ) => an,
                                   (_        , &Some(f)) => f,
                                    _                    => unreachable!(),
                            };
                       let mut arg = Arg::with_name(arg_name_str)
                                         .empty_values(false);
                       if let &Some(short_flag) = flag {
                           arg = arg.short(short_flag);
                       }
                       if let &Some(desc) = desc {
                           arg = arg.help(desc);
                       }
                       if arg_name.is_some() && flag.is_some() {
                           arg = arg.takes_value(true);
                       }
                       if let &Some(required) = required {
                           arg = arg.required(required);
                       }
                       if let &Some(multi) = multi {
                           arg = arg.multiple(multi);
                       }
                       scmd = scmd.arg(arg);
                   }
                   mcmd = mcmd.subcommand(scmd);
               }
               app = app.subcommand(mcmd);
           }
           
        let matches = app.get_matches();

    let debug = matches.is_present("adebug");
    let connector = hyper_rustls::HttpsConnectorBuilder::new().with_native_roots()
        .unwrap()
        .https_or_http()
        .enable_http1()
        .build();

    match Engine::new(matches, connector).await {
        Err(err) => {
            exit_status = err.exit_code;
            writeln!(io::stderr(), "{}", err).ok();
        },
        Ok(engine) => {
            if let Err(doit_err) = engine.doit().await {
                exit_status = 1;
                match doit_err {
                    DoitError::IoError(path, err) => {
                        writeln!(io::stderr(), "Failed to open output file '{}': {}", path, err).ok();
                    },
                    DoitError::ApiError(err) => {
                        if debug {
                            writeln!(io::stderr(), "{:#?}", err).ok();
                        } else {
                            writeln!(io::stderr(), "{}", err).ok();
                        }
                    }
                }
            }
        }
    }

    std::process::exit(exit_status);
}
