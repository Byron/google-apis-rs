// DO NOT EDIT !
// This file was generated automatically from 'src/mako/cli/main.rs.mako'
// DO NOT EDIT !
#![allow(unused_variables, unused_imports, dead_code, unused_mut)]

#[macro_use]
extern crate clap;
extern crate yup_oauth2 as oauth2;
extern crate yup_hyper_mock as mock;
extern crate hyper_rustls;
extern crate serde;
extern crate serde_json;
extern crate hyper;
extern crate mime;
extern crate strsim;
extern crate google_realtimebidding1 as api;

use std::env;
use std::io::{self, Write};
use clap::{App, SubCommand, Arg};

mod cmn;

use cmn::{InvalidOptionsError, CLIError, JsonTokenStorage, arg_from_str, writer_from_opts, parse_kv_arg,
          input_file_from_opts, input_mime_from_opts, FieldCursor, FieldError, CallType, UploadProtocol,
          calltype_from_str, remove_json_null_values, ComplexType, JsonType, JsonTypeInfo};

use std::default::Default;
use std::str::FromStr;

use oauth2::{Authenticator, DefaultAuthenticatorDelegate, FlowType};
use serde_json as json;
use clap::ArgMatches;

enum DoitError {
    IoError(String, io::Error),
    ApiError(api::Error),
}

struct Engine<'n> {
    opt: ArgMatches<'n>,
    hub: api::RealTimeBidding<hyper::Client, Authenticator<DefaultAuthenticatorDelegate, JsonTokenStorage, hyper::Client>>,
    gp: Vec<&'static str>,
    gpm: Vec<(&'static str, &'static str)>,
}


impl<'n> Engine<'n> {
    fn _bidders_creatives_list(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        let mut call = self.hub.bidders().creatives_list(opt.value_of("parent").unwrap_or(""));
        for parg in opt.values_of("v").map(|i|i.collect()).unwrap_or(Vec::new()).iter() {
            let (key, value) = parse_kv_arg(&*parg, err, false);
            match key {
                "view" => {
                    call = call.view(value.unwrap_or(""));
                },
                "page-token" => {
                    call = call.page_token(value.unwrap_or(""));
                },
                "page-size" => {
                    call = call.page_size(arg_from_str(value.unwrap_or("-0"), err, "page-size", "integer"));
                },
                "filter" => {
                    call = call.filter(value.unwrap_or(""));
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
                                                                           v.extend(["filter", "page-token", "page-size", "view"].iter().map(|v|*v));
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
                CallType::Standard => call.doit(),
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

    fn _bidders_creatives_watch(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
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
                    _ => {
                        let suggestion = FieldCursor::did_you_mean(key, &vec![]);
                        err.issues.push(CLIError::Field(FieldError::Unknown(temp_cursor.to_string(), suggestion, value.map(|v| v.to_string()))));
                        None
                    }
                };
            if let Some((field_cursor_str, type_info)) = type_info {
                FieldCursor::from(field_cursor_str).set_json_value(&mut object, value.unwrap(), type_info, err, &temp_cursor);
            }
        }
        let mut request: api::WatchCreativesRequest = json::value::from_value(object).unwrap();
        let mut call = self.hub.bidders().creatives_watch(request, opt.value_of("parent").unwrap_or(""));
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
                CallType::Standard => call.doit(),
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

    fn _buyers_creatives_create(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
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
                    "declared-vendor-ids" => Some(("declaredVendorIds", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Vec })),
                    "impression-tracking-urls" => Some(("impressionTrackingUrls", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Vec })),
                    "name" => Some(("name", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "agency-id" => Some(("agencyId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "declared-attributes" => Some(("declaredAttributes", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Vec })),
                    "html.snippet" => Some(("html.snippet", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "html.width" => Some(("html.width", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "html.height" => Some(("html.height", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "declared-restricted-categories" => Some(("declaredRestrictedCategories", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Vec })),
                    "deal-ids" => Some(("dealIds", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Vec })),
                    "advertiser-name" => Some(("advertiserName", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "creative-serving-decision.open-auction-serving-status.status" => Some(("creativeServingDecision.openAuctionServingStatus.status", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "creative-serving-decision.detected-product-categories" => Some(("creativeServingDecision.detectedProductCategories", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Vec })),
                    "creative-serving-decision.china-serving-status.status" => Some(("creativeServingDecision.chinaServingStatus.status", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "creative-serving-decision.ad-technology-providers.has-unidentified-provider" => Some(("creativeServingDecision.adTechnologyProviders.hasUnidentifiedProvider", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "creative-serving-decision.ad-technology-providers.detected-provider-ids" => Some(("creativeServingDecision.adTechnologyProviders.detectedProviderIds", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Vec })),
                    "creative-serving-decision.last-status-update" => Some(("creativeServingDecision.lastStatusUpdate", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "creative-serving-decision.deals-serving-status.status" => Some(("creativeServingDecision.dealsServingStatus.status", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "creative-serving-decision.russia-serving-status.status" => Some(("creativeServingDecision.russiaServingStatus.status", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "creative-serving-decision.detected-domains" => Some(("creativeServingDecision.detectedDomains", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Vec })),
                    "creative-serving-decision.detected-vendor-ids" => Some(("creativeServingDecision.detectedVendorIds", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Vec })),
                    "creative-serving-decision.detected-click-through-urls" => Some(("creativeServingDecision.detectedClickThroughUrls", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Vec })),
                    "creative-serving-decision.detected-languages" => Some(("creativeServingDecision.detectedLanguages", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Vec })),
                    "creative-serving-decision.detected-attributes" => Some(("creativeServingDecision.detectedAttributes", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Vec })),
                    "creative-serving-decision.detected-sensitive-categories" => Some(("creativeServingDecision.detectedSensitiveCategories", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Vec })),
                    "version" => Some(("version", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "video.video-url" => Some(("video.videoUrl", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "video.video-vast-xml" => Some(("video.videoVastXml", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "video.video-metadata.duration" => Some(("video.videoMetadata.duration", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "video.video-metadata.is-valid-vast" => Some(("video.videoMetadata.isValidVast", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "video.video-metadata.vast-version" => Some(("video.videoMetadata.vastVersion", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "video.video-metadata.skip-offset" => Some(("video.videoMetadata.skipOffset", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "video.video-metadata.is-vpaid" => Some(("video.videoMetadata.isVpaid", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "declared-click-through-urls" => Some(("declaredClickThroughUrls", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Vec })),
                    "api-update-time" => Some(("apiUpdateTime", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "restricted-categories" => Some(("restrictedCategories", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Vec })),
                    "creative-format" => Some(("creativeFormat", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "creative-id" => Some(("creativeId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "native.body" => Some(("native.body", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "native.video-url" => Some(("native.videoUrl", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "native.headline" => Some(("native.headline", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "native.image.url" => Some(("native.image.url", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "native.image.width" => Some(("native.image.width", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "native.image.height" => Some(("native.image.height", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "native.star-rating" => Some(("native.starRating", JsonTypeInfo { jtype: JsonType::Float, ctype: ComplexType::Pod })),
                    "native.advertiser-name" => Some(("native.advertiserName", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "native.price-display-text" => Some(("native.priceDisplayText", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "native.call-to-action" => Some(("native.callToAction", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "native.logo.url" => Some(("native.logo.url", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "native.logo.width" => Some(("native.logo.width", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "native.logo.height" => Some(("native.logo.height", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "native.app-icon.url" => Some(("native.appIcon.url", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "native.app-icon.width" => Some(("native.appIcon.width", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "native.app-icon.height" => Some(("native.appIcon.height", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "native.click-tracking-url" => Some(("native.clickTrackingUrl", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "native.click-link-url" => Some(("native.clickLinkUrl", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "ad-choices-destination-url" => Some(("adChoicesDestinationUrl", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "account-id" => Some(("accountId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    _ => {
                        let suggestion = FieldCursor::did_you_mean(key, &vec!["account-id", "ad-choices-destination-url", "ad-technology-providers", "advertiser-name", "agency-id", "api-update-time", "app-icon", "body", "call-to-action", "china-serving-status", "click-link-url", "click-tracking-url", "creative-format", "creative-id", "creative-serving-decision", "deal-ids", "deals-serving-status", "declared-attributes", "declared-click-through-urls", "declared-restricted-categories", "declared-vendor-ids", "detected-attributes", "detected-click-through-urls", "detected-domains", "detected-languages", "detected-product-categories", "detected-provider-ids", "detected-sensitive-categories", "detected-vendor-ids", "duration", "has-unidentified-provider", "headline", "height", "html", "image", "impression-tracking-urls", "is-valid-vast", "is-vpaid", "last-status-update", "logo", "name", "native", "open-auction-serving-status", "price-display-text", "restricted-categories", "russia-serving-status", "skip-offset", "snippet", "star-rating", "status", "url", "vast-version", "version", "video", "video-metadata", "video-url", "video-vast-xml", "width"]);
                        err.issues.push(CLIError::Field(FieldError::Unknown(temp_cursor.to_string(), suggestion, value.map(|v| v.to_string()))));
                        None
                    }
                };
            if let Some((field_cursor_str, type_info)) = type_info {
                FieldCursor::from(field_cursor_str).set_json_value(&mut object, value.unwrap(), type_info, err, &temp_cursor);
            }
        }
        let mut request: api::Creative = json::value::from_value(object).unwrap();
        let mut call = self.hub.buyers().creatives_create(request, opt.value_of("parent").unwrap_or(""));
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
                CallType::Standard => call.doit(),
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

    fn _buyers_creatives_get(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        let mut call = self.hub.buyers().creatives_get(opt.value_of("name").unwrap_or(""));
        for parg in opt.values_of("v").map(|i|i.collect()).unwrap_or(Vec::new()).iter() {
            let (key, value) = parse_kv_arg(&*parg, err, false);
            match key {
                "view" => {
                    call = call.view(value.unwrap_or(""));
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
                                                                           v.extend(["view"].iter().map(|v|*v));
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
                CallType::Standard => call.doit(),
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

    fn _buyers_creatives_list(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        let mut call = self.hub.buyers().creatives_list(opt.value_of("parent").unwrap_or(""));
        for parg in opt.values_of("v").map(|i|i.collect()).unwrap_or(Vec::new()).iter() {
            let (key, value) = parse_kv_arg(&*parg, err, false);
            match key {
                "view" => {
                    call = call.view(value.unwrap_or(""));
                },
                "page-token" => {
                    call = call.page_token(value.unwrap_or(""));
                },
                "page-size" => {
                    call = call.page_size(arg_from_str(value.unwrap_or("-0"), err, "page-size", "integer"));
                },
                "filter" => {
                    call = call.filter(value.unwrap_or(""));
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
                                                                           v.extend(["filter", "page-token", "page-size", "view"].iter().map(|v|*v));
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
                CallType::Standard => call.doit(),
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

    fn _buyers_creatives_patch(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
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
                    "declared-vendor-ids" => Some(("declaredVendorIds", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Vec })),
                    "impression-tracking-urls" => Some(("impressionTrackingUrls", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Vec })),
                    "name" => Some(("name", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "agency-id" => Some(("agencyId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "declared-attributes" => Some(("declaredAttributes", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Vec })),
                    "html.snippet" => Some(("html.snippet", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "html.width" => Some(("html.width", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "html.height" => Some(("html.height", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "declared-restricted-categories" => Some(("declaredRestrictedCategories", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Vec })),
                    "deal-ids" => Some(("dealIds", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Vec })),
                    "advertiser-name" => Some(("advertiserName", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "creative-serving-decision.open-auction-serving-status.status" => Some(("creativeServingDecision.openAuctionServingStatus.status", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "creative-serving-decision.detected-product-categories" => Some(("creativeServingDecision.detectedProductCategories", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Vec })),
                    "creative-serving-decision.china-serving-status.status" => Some(("creativeServingDecision.chinaServingStatus.status", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "creative-serving-decision.ad-technology-providers.has-unidentified-provider" => Some(("creativeServingDecision.adTechnologyProviders.hasUnidentifiedProvider", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "creative-serving-decision.ad-technology-providers.detected-provider-ids" => Some(("creativeServingDecision.adTechnologyProviders.detectedProviderIds", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Vec })),
                    "creative-serving-decision.last-status-update" => Some(("creativeServingDecision.lastStatusUpdate", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "creative-serving-decision.deals-serving-status.status" => Some(("creativeServingDecision.dealsServingStatus.status", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "creative-serving-decision.russia-serving-status.status" => Some(("creativeServingDecision.russiaServingStatus.status", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "creative-serving-decision.detected-domains" => Some(("creativeServingDecision.detectedDomains", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Vec })),
                    "creative-serving-decision.detected-vendor-ids" => Some(("creativeServingDecision.detectedVendorIds", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Vec })),
                    "creative-serving-decision.detected-click-through-urls" => Some(("creativeServingDecision.detectedClickThroughUrls", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Vec })),
                    "creative-serving-decision.detected-languages" => Some(("creativeServingDecision.detectedLanguages", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Vec })),
                    "creative-serving-decision.detected-attributes" => Some(("creativeServingDecision.detectedAttributes", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Vec })),
                    "creative-serving-decision.detected-sensitive-categories" => Some(("creativeServingDecision.detectedSensitiveCategories", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Vec })),
                    "version" => Some(("version", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "video.video-url" => Some(("video.videoUrl", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "video.video-vast-xml" => Some(("video.videoVastXml", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "video.video-metadata.duration" => Some(("video.videoMetadata.duration", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "video.video-metadata.is-valid-vast" => Some(("video.videoMetadata.isValidVast", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "video.video-metadata.vast-version" => Some(("video.videoMetadata.vastVersion", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "video.video-metadata.skip-offset" => Some(("video.videoMetadata.skipOffset", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "video.video-metadata.is-vpaid" => Some(("video.videoMetadata.isVpaid", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "declared-click-through-urls" => Some(("declaredClickThroughUrls", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Vec })),
                    "api-update-time" => Some(("apiUpdateTime", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "restricted-categories" => Some(("restrictedCategories", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Vec })),
                    "creative-format" => Some(("creativeFormat", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "creative-id" => Some(("creativeId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "native.body" => Some(("native.body", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "native.video-url" => Some(("native.videoUrl", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "native.headline" => Some(("native.headline", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "native.image.url" => Some(("native.image.url", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "native.image.width" => Some(("native.image.width", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "native.image.height" => Some(("native.image.height", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "native.star-rating" => Some(("native.starRating", JsonTypeInfo { jtype: JsonType::Float, ctype: ComplexType::Pod })),
                    "native.advertiser-name" => Some(("native.advertiserName", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "native.price-display-text" => Some(("native.priceDisplayText", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "native.call-to-action" => Some(("native.callToAction", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "native.logo.url" => Some(("native.logo.url", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "native.logo.width" => Some(("native.logo.width", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "native.logo.height" => Some(("native.logo.height", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "native.app-icon.url" => Some(("native.appIcon.url", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "native.app-icon.width" => Some(("native.appIcon.width", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "native.app-icon.height" => Some(("native.appIcon.height", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "native.click-tracking-url" => Some(("native.clickTrackingUrl", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "native.click-link-url" => Some(("native.clickLinkUrl", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "ad-choices-destination-url" => Some(("adChoicesDestinationUrl", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "account-id" => Some(("accountId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    _ => {
                        let suggestion = FieldCursor::did_you_mean(key, &vec!["account-id", "ad-choices-destination-url", "ad-technology-providers", "advertiser-name", "agency-id", "api-update-time", "app-icon", "body", "call-to-action", "china-serving-status", "click-link-url", "click-tracking-url", "creative-format", "creative-id", "creative-serving-decision", "deal-ids", "deals-serving-status", "declared-attributes", "declared-click-through-urls", "declared-restricted-categories", "declared-vendor-ids", "detected-attributes", "detected-click-through-urls", "detected-domains", "detected-languages", "detected-product-categories", "detected-provider-ids", "detected-sensitive-categories", "detected-vendor-ids", "duration", "has-unidentified-provider", "headline", "height", "html", "image", "impression-tracking-urls", "is-valid-vast", "is-vpaid", "last-status-update", "logo", "name", "native", "open-auction-serving-status", "price-display-text", "restricted-categories", "russia-serving-status", "skip-offset", "snippet", "star-rating", "status", "url", "vast-version", "version", "video", "video-metadata", "video-url", "video-vast-xml", "width"]);
                        err.issues.push(CLIError::Field(FieldError::Unknown(temp_cursor.to_string(), suggestion, value.map(|v| v.to_string()))));
                        None
                    }
                };
            if let Some((field_cursor_str, type_info)) = type_info {
                FieldCursor::from(field_cursor_str).set_json_value(&mut object, value.unwrap(), type_info, err, &temp_cursor);
            }
        }
        let mut request: api::Creative = json::value::from_value(object).unwrap();
        let mut call = self.hub.buyers().creatives_patch(request, opt.value_of("name").unwrap_or(""));
        for parg in opt.values_of("v").map(|i|i.collect()).unwrap_or(Vec::new()).iter() {
            let (key, value) = parse_kv_arg(&*parg, err, false);
            match key {
                "update-mask" => {
                    call = call.update_mask(value.unwrap_or(""));
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
                CallType::Standard => call.doit(),
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

    fn _buyers_get_remarketing_tag(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        let mut call = self.hub.buyers().get_remarketing_tag(opt.value_of("name").unwrap_or(""));
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
                CallType::Standard => call.doit(),
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

    fn _buyers_user_lists_close(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
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
                    _ => {
                        let suggestion = FieldCursor::did_you_mean(key, &vec![]);
                        err.issues.push(CLIError::Field(FieldError::Unknown(temp_cursor.to_string(), suggestion, value.map(|v| v.to_string()))));
                        None
                    }
                };
            if let Some((field_cursor_str, type_info)) = type_info {
                FieldCursor::from(field_cursor_str).set_json_value(&mut object, value.unwrap(), type_info, err, &temp_cursor);
            }
        }
        let mut request: api::CloseUserListRequest = json::value::from_value(object).unwrap();
        let mut call = self.hub.buyers().user_lists_close(request, opt.value_of("name").unwrap_or(""));
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
                CallType::Standard => call.doit(),
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

    fn _buyers_user_lists_create(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
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
                    "status" => Some(("status", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "display-name" => Some(("displayName", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "description" => Some(("description", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "membership-duration-days" => Some(("membershipDurationDays", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "url-restriction.url" => Some(("urlRestriction.url", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "url-restriction.start-date.year" => Some(("urlRestriction.startDate.year", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "url-restriction.start-date.day" => Some(("urlRestriction.startDate.day", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "url-restriction.start-date.month" => Some(("urlRestriction.startDate.month", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "url-restriction.end-date.year" => Some(("urlRestriction.endDate.year", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "url-restriction.end-date.day" => Some(("urlRestriction.endDate.day", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "url-restriction.end-date.month" => Some(("urlRestriction.endDate.month", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "url-restriction.restriction-type" => Some(("urlRestriction.restrictionType", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "name" => Some(("name", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    _ => {
                        let suggestion = FieldCursor::did_you_mean(key, &vec!["day", "description", "display-name", "end-date", "membership-duration-days", "month", "name", "restriction-type", "start-date", "status", "url", "url-restriction", "year"]);
                        err.issues.push(CLIError::Field(FieldError::Unknown(temp_cursor.to_string(), suggestion, value.map(|v| v.to_string()))));
                        None
                    }
                };
            if let Some((field_cursor_str, type_info)) = type_info {
                FieldCursor::from(field_cursor_str).set_json_value(&mut object, value.unwrap(), type_info, err, &temp_cursor);
            }
        }
        let mut request: api::UserList = json::value::from_value(object).unwrap();
        let mut call = self.hub.buyers().user_lists_create(request, opt.value_of("parent").unwrap_or(""));
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
                CallType::Standard => call.doit(),
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

    fn _buyers_user_lists_get(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        let mut call = self.hub.buyers().user_lists_get(opt.value_of("name").unwrap_or(""));
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
                CallType::Standard => call.doit(),
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

    fn _buyers_user_lists_get_remarketing_tag(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        let mut call = self.hub.buyers().user_lists_get_remarketing_tag(opt.value_of("name").unwrap_or(""));
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
                CallType::Standard => call.doit(),
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

    fn _buyers_user_lists_list(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        let mut call = self.hub.buyers().user_lists_list(opt.value_of("parent").unwrap_or(""));
        for parg in opt.values_of("v").map(|i|i.collect()).unwrap_or(Vec::new()).iter() {
            let (key, value) = parse_kv_arg(&*parg, err, false);
            match key {
                "page-token" => {
                    call = call.page_token(value.unwrap_or(""));
                },
                "page-size" => {
                    call = call.page_size(arg_from_str(value.unwrap_or("-0"), err, "page-size", "integer"));
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
                                                                           v.extend(["page-token", "page-size"].iter().map(|v|*v));
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
                CallType::Standard => call.doit(),
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

    fn _buyers_user_lists_open(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
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
                    _ => {
                        let suggestion = FieldCursor::did_you_mean(key, &vec![]);
                        err.issues.push(CLIError::Field(FieldError::Unknown(temp_cursor.to_string(), suggestion, value.map(|v| v.to_string()))));
                        None
                    }
                };
            if let Some((field_cursor_str, type_info)) = type_info {
                FieldCursor::from(field_cursor_str).set_json_value(&mut object, value.unwrap(), type_info, err, &temp_cursor);
            }
        }
        let mut request: api::OpenUserListRequest = json::value::from_value(object).unwrap();
        let mut call = self.hub.buyers().user_lists_open(request, opt.value_of("name").unwrap_or(""));
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
                CallType::Standard => call.doit(),
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

    fn _buyers_user_lists_update(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
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
                    "status" => Some(("status", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "display-name" => Some(("displayName", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "description" => Some(("description", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "membership-duration-days" => Some(("membershipDurationDays", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "url-restriction.url" => Some(("urlRestriction.url", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "url-restriction.start-date.year" => Some(("urlRestriction.startDate.year", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "url-restriction.start-date.day" => Some(("urlRestriction.startDate.day", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "url-restriction.start-date.month" => Some(("urlRestriction.startDate.month", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "url-restriction.end-date.year" => Some(("urlRestriction.endDate.year", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "url-restriction.end-date.day" => Some(("urlRestriction.endDate.day", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "url-restriction.end-date.month" => Some(("urlRestriction.endDate.month", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "url-restriction.restriction-type" => Some(("urlRestriction.restrictionType", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "name" => Some(("name", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    _ => {
                        let suggestion = FieldCursor::did_you_mean(key, &vec!["day", "description", "display-name", "end-date", "membership-duration-days", "month", "name", "restriction-type", "start-date", "status", "url", "url-restriction", "year"]);
                        err.issues.push(CLIError::Field(FieldError::Unknown(temp_cursor.to_string(), suggestion, value.map(|v| v.to_string()))));
                        None
                    }
                };
            if let Some((field_cursor_str, type_info)) = type_info {
                FieldCursor::from(field_cursor_str).set_json_value(&mut object, value.unwrap(), type_info, err, &temp_cursor);
            }
        }
        let mut request: api::UserList = json::value::from_value(object).unwrap();
        let mut call = self.hub.buyers().user_lists_update(request, opt.value_of("name").unwrap_or(""));
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
                CallType::Standard => call.doit(),
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

    fn _doit(&self, dry_run: bool) -> Result<Result<(), DoitError>, Option<InvalidOptionsError>> {
        let mut err = InvalidOptionsError::new();
        let mut call_result: Result<(), DoitError> = Ok(());
        let mut err_opt: Option<InvalidOptionsError> = None;
        match self.opt.subcommand() {
            ("bidders", Some(opt)) => {
                match opt.subcommand() {
                    ("creatives-list", Some(opt)) => {
                        call_result = self._bidders_creatives_list(opt, dry_run, &mut err);
                    },
                    ("creatives-watch", Some(opt)) => {
                        call_result = self._bidders_creatives_watch(opt, dry_run, &mut err);
                    },
                    _ => {
                        err.issues.push(CLIError::MissingMethodError("bidders".to_string()));
                        writeln!(io::stderr(), "{}\n", opt.usage()).ok();
                    }
                }
            },
            ("buyers", Some(opt)) => {
                match opt.subcommand() {
                    ("creatives-create", Some(opt)) => {
                        call_result = self._buyers_creatives_create(opt, dry_run, &mut err);
                    },
                    ("creatives-get", Some(opt)) => {
                        call_result = self._buyers_creatives_get(opt, dry_run, &mut err);
                    },
                    ("creatives-list", Some(opt)) => {
                        call_result = self._buyers_creatives_list(opt, dry_run, &mut err);
                    },
                    ("creatives-patch", Some(opt)) => {
                        call_result = self._buyers_creatives_patch(opt, dry_run, &mut err);
                    },
                    ("get-remarketing-tag", Some(opt)) => {
                        call_result = self._buyers_get_remarketing_tag(opt, dry_run, &mut err);
                    },
                    ("user-lists-close", Some(opt)) => {
                        call_result = self._buyers_user_lists_close(opt, dry_run, &mut err);
                    },
                    ("user-lists-create", Some(opt)) => {
                        call_result = self._buyers_user_lists_create(opt, dry_run, &mut err);
                    },
                    ("user-lists-get", Some(opt)) => {
                        call_result = self._buyers_user_lists_get(opt, dry_run, &mut err);
                    },
                    ("user-lists-get-remarketing-tag", Some(opt)) => {
                        call_result = self._buyers_user_lists_get_remarketing_tag(opt, dry_run, &mut err);
                    },
                    ("user-lists-list", Some(opt)) => {
                        call_result = self._buyers_user_lists_list(opt, dry_run, &mut err);
                    },
                    ("user-lists-open", Some(opt)) => {
                        call_result = self._buyers_user_lists_open(opt, dry_run, &mut err);
                    },
                    ("user-lists-update", Some(opt)) => {
                        call_result = self._buyers_user_lists_update(opt, dry_run, &mut err);
                    },
                    _ => {
                        err.issues.push(CLIError::MissingMethodError("buyers".to_string()));
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
    fn new(opt: ArgMatches<'n>) -> Result<Engine<'n>, InvalidOptionsError> {
        let (config_dir, secret) = {
            let config_dir = match cmn::assure_config_dir_exists(opt.value_of("folder").unwrap_or("~/.google-service-cli")) {
                Err(e) => return Err(InvalidOptionsError::single(e, 3)),
                Ok(p) => p,
            };

            match cmn::application_secret_from_directory(&config_dir, "realtimebidding1-secret.json",
                                                         "{\"installed\":{\"auth_uri\":\"https://accounts.google.com/o/oauth2/auth\",\"client_secret\":\"hCsslbCUyfehWMmbkG8vTYxG\",\"token_uri\":\"https://accounts.google.com/o/oauth2/token\",\"client_email\":\"\",\"redirect_uris\":[\"urn:ietf:wg:oauth:2.0:oob\",\"oob\"],\"client_x509_cert_url\":\"\",\"client_id\":\"620010449518-9ngf7o4dhs0dka470npqvor6dc5lqb9b.apps.googleusercontent.com\",\"auth_provider_x509_cert_url\":\"https://www.googleapis.com/oauth2/v1/certs\"}}") {
                Ok(secret) => (config_dir, secret),
                Err(e) => return Err(InvalidOptionsError::single(e, 4))
            }
        };

        let auth = Authenticator::new(  &secret, DefaultAuthenticatorDelegate,
                                        if opt.is_present("debug-auth") {
                                            hyper::Client::with_connector(mock::TeeConnector {
                                                    connector: hyper::net::HttpsConnector::new(hyper_rustls::TlsClient::new())
                                                })
                                        } else {
                                            hyper::Client::with_connector(hyper::net::HttpsConnector::new(hyper_rustls::TlsClient::new()))
                                        },
                                        JsonTokenStorage {
                                          program_name: "realtimebidding1",
                                          db_dir: config_dir.clone(),
                                        }, Some(FlowType::InstalledRedirect(54324)));

        let client =
            if opt.is_present("debug") {
                hyper::Client::with_connector(mock::TeeConnector {
                        connector: hyper::net::HttpsConnector::new(hyper_rustls::TlsClient::new())
                    })
            } else {
                hyper::Client::with_connector(hyper::net::HttpsConnector::new(hyper_rustls::TlsClient::new()))
            };
        let engine = Engine {
            opt: opt,
            hub: api::RealTimeBidding::new(client, auth),
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

        match engine._doit(true) {
            Err(Some(err)) => Err(err),
            Err(None)      => Ok(engine),
            Ok(_)          => unreachable!(),
        }
    }

    fn doit(&self) -> Result<(), DoitError> {
        match self._doit(false) {
            Ok(res) => res,
            Err(_) => unreachable!(),
        }
    }
}

fn main() {
    let mut exit_status = 0i32;
    let arg_data = [
        ("bidders", "methods: 'creatives-list' and 'creatives-watch'", vec![
            ("creatives-list",
                    Some(r##"Lists creatives."##),
                    "Details at http://byron.github.io/google-apis-rs/google_realtimebidding1_cli/bidders_creatives-list",
                  vec![
                    (Some(r##"parent"##),
                     None,
                     Some(r##"Required. Name of the parent buyer that owns the creatives.
        
        The pattern for this resource is either `buyers/{buyerAccountId}` or
        `bidders/{bidderAccountId}`.
        
        For `buyers/{buyerAccountId}`, the `buyerAccountId` can be one of the
        following:
        1. The ID of the buyer that is accessing their own creatives.
        2. The ID of the child seat buyer under a bidder account.
           So for listing creatives pertaining to the child seat buyer (`456`)
           under bidder account (`123`), you would use the pattern: `buyers/456`.
        3. The ID of the bidder itself.
           So for listing creatives pertaining to bidder (`123`),
           you would use `buyers/123`.
        
        If you want to access all creatives pertaining to both the bidder and all
        of its child seat accounts, you would use `bidders/{bidderAccountId}`,
        e.g., for all creatives pertaining to bidder (`123`), use `bidders/123`."##),
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
            ("creatives-watch",
                    Some(r##"Watches all creatives pertaining to a bidder. It is sufficient to invoke
        this endpoint once per bidder. A Pub/Sub topic will be created and
        notifications will be pushed to the topic when any of the bidder's
        creatives change status. All of the bidder's service accounts will have
        access to read from the topic.
        Subsequent invocations of this method will return the existing
        Pub/Sub configuration."##),
                    "Details at http://byron.github.io/google-apis-rs/google_realtimebidding1_cli/bidders_creatives-watch",
                  vec![
                    (Some(r##"parent"##),
                     None,
                     Some(r##"Required. To watch all creatives pertaining to the bidder and all its child seat
        accounts, the bidder must follow the pattern `bidders/{bidderAccountId}`."##),
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
            ]),
        
        ("buyers", "methods: 'creatives-create', 'creatives-get', 'creatives-list', 'creatives-patch', 'get-remarketing-tag', 'user-lists-close', 'user-lists-create', 'user-lists-get', 'user-lists-get-remarketing-tag', 'user-lists-list', 'user-lists-open' and 'user-lists-update'", vec![
            ("creatives-create",
                    Some(r##"Creates a creative."##),
                    "Details at http://byron.github.io/google-apis-rs/google_realtimebidding1_cli/buyers_creatives-create",
                  vec![
                    (Some(r##"parent"##),
                     None,
                     Some(r##"Required. The name of the parent buyer that the new creative belongs to that must
        follow the pattern `buyers/{buyerAccountId}`, where `{buyerAccountId}`
        represents the account ID of the buyer who owns a creative. For a bidder
        accessing creatives on behalf of a child seat buyer, `{buyerAccountId}`
        should represent the account ID of the child seat buyer."##),
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
            ("creatives-get",
                    Some(r##"Gets a creative."##),
                    "Details at http://byron.github.io/google-apis-rs/google_realtimebidding1_cli/buyers_creatives-get",
                  vec![
                    (Some(r##"name"##),
                     None,
                     Some(r##"Required. Name of the creative to retrieve. See
        creative.name."##),
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
            ("creatives-list",
                    Some(r##"Lists creatives."##),
                    "Details at http://byron.github.io/google-apis-rs/google_realtimebidding1_cli/buyers_creatives-list",
                  vec![
                    (Some(r##"parent"##),
                     None,
                     Some(r##"Required. Name of the parent buyer that owns the creatives.
        
        The pattern for this resource is either `buyers/{buyerAccountId}` or
        `bidders/{bidderAccountId}`.
        
        For `buyers/{buyerAccountId}`, the `buyerAccountId` can be one of the
        following:
        1. The ID of the buyer that is accessing their own creatives.
        2. The ID of the child seat buyer under a bidder account.
           So for listing creatives pertaining to the child seat buyer (`456`)
           under bidder account (`123`), you would use the pattern: `buyers/456`.
        3. The ID of the bidder itself.
           So for listing creatives pertaining to bidder (`123`),
           you would use `buyers/123`.
        
        If you want to access all creatives pertaining to both the bidder and all
        of its child seat accounts, you would use `bidders/{bidderAccountId}`,
        e.g., for all creatives pertaining to bidder (`123`), use `bidders/123`."##),
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
            ("creatives-patch",
                    Some(r##"Updates a creative."##),
                    "Details at http://byron.github.io/google-apis-rs/google_realtimebidding1_cli/buyers_creatives-patch",
                  vec![
                    (Some(r##"name"##),
                     None,
                     Some(r##"Name of the creative to update. See
        creative.name."##),
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
            ("get-remarketing-tag",
                    Some(r##"Gets remarketing tag for a buyer. A remarketing tag is a piece of
        JavaScript code that can be placed on a web page. When a user
        visits a page containing a remarketing tag, Google adds the user to a user
        list."##),
                    "Details at http://byron.github.io/google-apis-rs/google_realtimebidding1_cli/buyers_get-remarketing-tag",
                  vec![
                    (Some(r##"name"##),
                     None,
                     Some(r##"Required. To fetch remarketing tag for an account, name must follow the pattern
        `buyers/{accountId}` where `{accountId}`
        represents ID of a buyer that owns the remarketing tag. For a
        bidder accessing remarketing tag on behalf of a child seat buyer,
        `{accountId}` should represent the ID of the child seat buyer.
        To fetch remarketing tag for a specific user list, name
        must follow the pattern
        `buyers/{accountId}/userLists/{userListId}`. See
        UserList.name."##),
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
            ("user-lists-close",
                    Some(r##"Change the status of a user list to CLOSED. This prevents new users from
        being added to the user list."##),
                    "Details at http://byron.github.io/google-apis-rs/google_realtimebidding1_cli/buyers_user-lists-close",
                  vec![
                    (Some(r##"name"##),
                     None,
                     Some(r##"Required. The name of the user list to close.
        See UserList.name"##),
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
            ("user-lists-create",
                    Some(r##"Create a new user list."##),
                    "Details at http://byron.github.io/google-apis-rs/google_realtimebidding1_cli/buyers_user-lists-create",
                  vec![
                    (Some(r##"parent"##),
                     None,
                     Some(r##"Required. The name of the parent buyer of the user list to be retrieved that
        must follow the pattern `buyers/{buyerAccountId}`, where `{buyerAccountId}`
        represents the account ID of the buyer who owns user lists. For a bidder
        accessing user lists on behalf of a child seat buyer , `{buyerAccountId}`
        should represent the account ID of the child seat buyer."##),
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
            ("user-lists-get",
                    Some(r##"Gets a user list by its name."##),
                    "Details at http://byron.github.io/google-apis-rs/google_realtimebidding1_cli/buyers_user-lists-get",
                  vec![
                    (Some(r##"name"##),
                     None,
                     Some(r##"Required. The name of the user list to be retrieved. See
        UserList.name."##),
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
            ("user-lists-get-remarketing-tag",
                    Some(r##"Gets remarketing tag for a buyer. A remarketing tag is a piece of
        JavaScript code that can be placed on a web page. When a user
        visits a page containing a remarketing tag, Google adds the user to a user
        list."##),
                    "Details at http://byron.github.io/google-apis-rs/google_realtimebidding1_cli/buyers_user-lists-get-remarketing-tag",
                  vec![
                    (Some(r##"name"##),
                     None,
                     Some(r##"Required. To fetch remarketing tag for an account, name must follow the pattern
        `buyers/{accountId}` where `{accountId}`
        represents ID of a buyer that owns the remarketing tag. For a
        bidder accessing remarketing tag on behalf of a child seat buyer,
        `{accountId}` should represent the ID of the child seat buyer.
        To fetch remarketing tag for a specific user list, name
        must follow the pattern
        `buyers/{accountId}/userLists/{userListId}`. See
        UserList.name."##),
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
            ("user-lists-list",
                    Some(r##"Lists the user lists visible to the current user."##),
                    "Details at http://byron.github.io/google-apis-rs/google_realtimebidding1_cli/buyers_user-lists-list",
                  vec![
                    (Some(r##"parent"##),
                     None,
                     Some(r##"Required. The name of the parent buyer for the user lists to be returned that must
        follow the pattern `buyers/{buyerAccountId}`, where `{buyerAccountId}`
        represents the account ID of the buyer who owns user lists. For a bidder
        accessing user lists on behalf of a child seat buyer , `{buyerAccountId}`
        should represent the account ID of the child seat buyer."##),
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
            ("user-lists-open",
                    Some(r##"Change the status of a user list to OPEN. This allows new users to be added
        to the user list."##),
                    "Details at http://byron.github.io/google-apis-rs/google_realtimebidding1_cli/buyers_user-lists-open",
                  vec![
                    (Some(r##"name"##),
                     None,
                     Some(r##"Required. The name of the user list to open.
        See UserList.name"##),
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
            ("user-lists-update",
                    Some(r##"Update the given user list. Only user lists with URLRestrictions can be
        updated."##),
                    "Details at http://byron.github.io/google-apis-rs/google_realtimebidding1_cli/buyers_user-lists-update",
                  vec![
                    (Some(r##"name"##),
                     None,
                     Some(r##"Output only. Name of the user list that must follow the pattern
        `buyers/{buyer}/userLists/{user_list}`, where `{buyer}` represents
        the account ID of the buyer who owns the user list. For a bidder accessing
        user lists on behalf of a child seat buyer, `{buyer}` represents
        the account ID of the child seat buyer. `{user_list}` is an int64
        identifier assigned by Google to uniquely identify a user list."##),
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
            ]),
        
    ];
    
    let mut app = App::new("realtimebidding1")
           .author("Sebastian Thiel <byronimo@gmail.com>")
           .version("1.0.14+20200709")
           .about("Allows external bidders to manage their RTB integration with Google. This includes managing bidder endpoints, QPS quotas, configuring what ad inventory to receive via pretargeting, submitting creatives for verification, and accessing creative metadata such as approval status.")
           .after_help("All documentation details can be found at http://byron.github.io/google-apis-rs/google_realtimebidding1_cli")
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
                   .help("Output all server communication to standard error. `tx` and `rx` are placed into the same stream.")
                   .multiple(false)
                   .takes_value(false))
           .arg(Arg::with_name("debug-auth")
                   .long("debug-auth")
                   .help("Output all communication related to authentication to standard error. `tx` and `rx` are placed into the same stream.")
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

    let debug = matches.is_present("debug");
    match Engine::new(matches) {
        Err(err) => {
            exit_status = err.exit_code;
            writeln!(io::stderr(), "{}", err).ok();
        },
        Ok(engine) => {
            if let Err(doit_err) = engine.doit() {
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