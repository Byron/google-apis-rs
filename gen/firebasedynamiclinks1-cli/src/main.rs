// DO NOT EDIT !
// This file was generated automatically from 'src/generator/templates/cli/main.rs.mako'
// DO NOT EDIT !
#![allow(unused_variables, unused_imports, dead_code, unused_mut)]

#[macro_use]
extern crate clap;

use std::env;
use std::io::{self, Write};
use clap::{App, SubCommand, Arg};

use google_firebasedynamiclinks1::{api, Error, oauth2, client::chrono, FieldMask};


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
    hub: api::FirebaseDynamicLinks<S>,
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
    async fn _managed_short_links_create(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
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
                    "dynamic-link-info.analytics-info.google-play-analytics.gclid" => Some(("dynamicLinkInfo.analyticsInfo.googlePlayAnalytics.gclid", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "dynamic-link-info.analytics-info.google-play-analytics.utm-campaign" => Some(("dynamicLinkInfo.analyticsInfo.googlePlayAnalytics.utmCampaign", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "dynamic-link-info.analytics-info.google-play-analytics.utm-content" => Some(("dynamicLinkInfo.analyticsInfo.googlePlayAnalytics.utmContent", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "dynamic-link-info.analytics-info.google-play-analytics.utm-medium" => Some(("dynamicLinkInfo.analyticsInfo.googlePlayAnalytics.utmMedium", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "dynamic-link-info.analytics-info.google-play-analytics.utm-source" => Some(("dynamicLinkInfo.analyticsInfo.googlePlayAnalytics.utmSource", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "dynamic-link-info.analytics-info.google-play-analytics.utm-term" => Some(("dynamicLinkInfo.analyticsInfo.googlePlayAnalytics.utmTerm", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "dynamic-link-info.analytics-info.itunes-connect-analytics.at" => Some(("dynamicLinkInfo.analyticsInfo.itunesConnectAnalytics.at", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "dynamic-link-info.analytics-info.itunes-connect-analytics.ct" => Some(("dynamicLinkInfo.analyticsInfo.itunesConnectAnalytics.ct", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "dynamic-link-info.analytics-info.itunes-connect-analytics.mt" => Some(("dynamicLinkInfo.analyticsInfo.itunesConnectAnalytics.mt", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "dynamic-link-info.analytics-info.itunes-connect-analytics.pt" => Some(("dynamicLinkInfo.analyticsInfo.itunesConnectAnalytics.pt", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "dynamic-link-info.android-info.android-fallback-link" => Some(("dynamicLinkInfo.androidInfo.androidFallbackLink", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "dynamic-link-info.android-info.android-link" => Some(("dynamicLinkInfo.androidInfo.androidLink", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "dynamic-link-info.android-info.android-min-package-version-code" => Some(("dynamicLinkInfo.androidInfo.androidMinPackageVersionCode", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "dynamic-link-info.android-info.android-package-name" => Some(("dynamicLinkInfo.androidInfo.androidPackageName", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "dynamic-link-info.desktop-info.desktop-fallback-link" => Some(("dynamicLinkInfo.desktopInfo.desktopFallbackLink", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "dynamic-link-info.domain-uri-prefix" => Some(("dynamicLinkInfo.domainUriPrefix", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "dynamic-link-info.dynamic-link-domain" => Some(("dynamicLinkInfo.dynamicLinkDomain", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "dynamic-link-info.ios-info.ios-app-store-id" => Some(("dynamicLinkInfo.iosInfo.iosAppStoreId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "dynamic-link-info.ios-info.ios-bundle-id" => Some(("dynamicLinkInfo.iosInfo.iosBundleId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "dynamic-link-info.ios-info.ios-custom-scheme" => Some(("dynamicLinkInfo.iosInfo.iosCustomScheme", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "dynamic-link-info.ios-info.ios-fallback-link" => Some(("dynamicLinkInfo.iosInfo.iosFallbackLink", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "dynamic-link-info.ios-info.ios-ipad-bundle-id" => Some(("dynamicLinkInfo.iosInfo.iosIpadBundleId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "dynamic-link-info.ios-info.ios-ipad-fallback-link" => Some(("dynamicLinkInfo.iosInfo.iosIpadFallbackLink", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "dynamic-link-info.ios-info.ios-minimum-version" => Some(("dynamicLinkInfo.iosInfo.iosMinimumVersion", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "dynamic-link-info.link" => Some(("dynamicLinkInfo.link", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "dynamic-link-info.navigation-info.enable-forced-redirect" => Some(("dynamicLinkInfo.navigationInfo.enableForcedRedirect", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "dynamic-link-info.social-meta-tag-info.social-description" => Some(("dynamicLinkInfo.socialMetaTagInfo.socialDescription", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "dynamic-link-info.social-meta-tag-info.social-image-link" => Some(("dynamicLinkInfo.socialMetaTagInfo.socialImageLink", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "dynamic-link-info.social-meta-tag-info.social-title" => Some(("dynamicLinkInfo.socialMetaTagInfo.socialTitle", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "long-dynamic-link" => Some(("longDynamicLink", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "name" => Some(("name", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "sdk-version" => Some(("sdkVersion", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "suffix.custom-suffix" => Some(("suffix.customSuffix", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "suffix.option" => Some(("suffix.option", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    _ => {
                        let suggestion = FieldCursor::did_you_mean(key, &vec!["analytics-info", "android-fallback-link", "android-info", "android-link", "android-min-package-version-code", "android-package-name", "at", "ct", "custom-suffix", "desktop-fallback-link", "desktop-info", "domain-uri-prefix", "dynamic-link-domain", "dynamic-link-info", "enable-forced-redirect", "gclid", "google-play-analytics", "ios-app-store-id", "ios-bundle-id", "ios-custom-scheme", "ios-fallback-link", "ios-info", "ios-ipad-bundle-id", "ios-ipad-fallback-link", "ios-minimum-version", "itunes-connect-analytics", "link", "long-dynamic-link", "mt", "name", "navigation-info", "option", "pt", "sdk-version", "social-description", "social-image-link", "social-meta-tag-info", "social-title", "suffix", "utm-campaign", "utm-content", "utm-medium", "utm-source", "utm-term"]);
                        err.issues.push(CLIError::Field(FieldError::Unknown(temp_cursor.to_string(), suggestion, value.map(|v| v.to_string()))));
                        None
                    }
                };
            if let Some((field_cursor_str, type_info)) = type_info {
                FieldCursor::from(field_cursor_str).set_json_value(&mut object, value.unwrap(), type_info, err, &temp_cursor);
            }
        }
        let mut request: api::CreateManagedShortLinkRequest = json::value::from_value(object).unwrap();
        let mut call = self.hub.managed_short_links().create(request);
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

    async fn _methods_get_link_stats(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        let mut call = self.hub.methods().get_link_stats(opt.value_of("dynamic-link").unwrap_or(""));
        for parg in opt.values_of("v").map(|i|i.collect()).unwrap_or(Vec::new()).iter() {
            let (key, value) = parse_kv_arg(&*parg, err, false);
            match key {
                "sdk-version" => {
                    call = call.sdk_version(value.unwrap_or(""));
                },
                "duration-days" => {
                    call = call.duration_days(        value.map(|v| arg_from_str(v, err, "duration-days", "int64")).unwrap_or(-0));
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
                                                                           v.extend(["duration-days", "sdk-version"].iter().map(|v|*v));
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

    async fn _methods_install_attribution(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
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
                    "app-installation-time" => Some(("appInstallationTime", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "bundle-id" => Some(("bundleId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "device.device-model-name" => Some(("device.deviceModelName", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "device.language-code" => Some(("device.languageCode", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "device.language-code-from-webview" => Some(("device.languageCodeFromWebview", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "device.language-code-raw" => Some(("device.languageCodeRaw", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "device.screen-resolution-height" => Some(("device.screenResolutionHeight", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "device.screen-resolution-width" => Some(("device.screenResolutionWidth", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "device.timezone" => Some(("device.timezone", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "ios-version" => Some(("iosVersion", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "retrieval-method" => Some(("retrievalMethod", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "sdk-version" => Some(("sdkVersion", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "unique-match-link-to-check" => Some(("uniqueMatchLinkToCheck", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "visual-style" => Some(("visualStyle", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    _ => {
                        let suggestion = FieldCursor::did_you_mean(key, &vec!["app-installation-time", "bundle-id", "device", "device-model-name", "ios-version", "language-code", "language-code-from-webview", "language-code-raw", "retrieval-method", "screen-resolution-height", "screen-resolution-width", "sdk-version", "timezone", "unique-match-link-to-check", "visual-style"]);
                        err.issues.push(CLIError::Field(FieldError::Unknown(temp_cursor.to_string(), suggestion, value.map(|v| v.to_string()))));
                        None
                    }
                };
            if let Some((field_cursor_str, type_info)) = type_info {
                FieldCursor::from(field_cursor_str).set_json_value(&mut object, value.unwrap(), type_info, err, &temp_cursor);
            }
        }
        let mut request: api::GetIosPostInstallAttributionRequest = json::value::from_value(object).unwrap();
        let mut call = self.hub.methods().install_attribution(request);
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

    async fn _methods_reopen_attribution(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
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
                    "bundle-id" => Some(("bundleId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "requested-link" => Some(("requestedLink", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "sdk-version" => Some(("sdkVersion", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    _ => {
                        let suggestion = FieldCursor::did_you_mean(key, &vec!["bundle-id", "requested-link", "sdk-version"]);
                        err.issues.push(CLIError::Field(FieldError::Unknown(temp_cursor.to_string(), suggestion, value.map(|v| v.to_string()))));
                        None
                    }
                };
            if let Some((field_cursor_str, type_info)) = type_info {
                FieldCursor::from(field_cursor_str).set_json_value(&mut object, value.unwrap(), type_info, err, &temp_cursor);
            }
        }
        let mut request: api::GetIosReopenAttributionRequest = json::value::from_value(object).unwrap();
        let mut call = self.hub.methods().reopen_attribution(request);
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

    async fn _short_links_create(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
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
                    "dynamic-link-info.analytics-info.google-play-analytics.gclid" => Some(("dynamicLinkInfo.analyticsInfo.googlePlayAnalytics.gclid", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "dynamic-link-info.analytics-info.google-play-analytics.utm-campaign" => Some(("dynamicLinkInfo.analyticsInfo.googlePlayAnalytics.utmCampaign", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "dynamic-link-info.analytics-info.google-play-analytics.utm-content" => Some(("dynamicLinkInfo.analyticsInfo.googlePlayAnalytics.utmContent", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "dynamic-link-info.analytics-info.google-play-analytics.utm-medium" => Some(("dynamicLinkInfo.analyticsInfo.googlePlayAnalytics.utmMedium", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "dynamic-link-info.analytics-info.google-play-analytics.utm-source" => Some(("dynamicLinkInfo.analyticsInfo.googlePlayAnalytics.utmSource", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "dynamic-link-info.analytics-info.google-play-analytics.utm-term" => Some(("dynamicLinkInfo.analyticsInfo.googlePlayAnalytics.utmTerm", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "dynamic-link-info.analytics-info.itunes-connect-analytics.at" => Some(("dynamicLinkInfo.analyticsInfo.itunesConnectAnalytics.at", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "dynamic-link-info.analytics-info.itunes-connect-analytics.ct" => Some(("dynamicLinkInfo.analyticsInfo.itunesConnectAnalytics.ct", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "dynamic-link-info.analytics-info.itunes-connect-analytics.mt" => Some(("dynamicLinkInfo.analyticsInfo.itunesConnectAnalytics.mt", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "dynamic-link-info.analytics-info.itunes-connect-analytics.pt" => Some(("dynamicLinkInfo.analyticsInfo.itunesConnectAnalytics.pt", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "dynamic-link-info.android-info.android-fallback-link" => Some(("dynamicLinkInfo.androidInfo.androidFallbackLink", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "dynamic-link-info.android-info.android-link" => Some(("dynamicLinkInfo.androidInfo.androidLink", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "dynamic-link-info.android-info.android-min-package-version-code" => Some(("dynamicLinkInfo.androidInfo.androidMinPackageVersionCode", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "dynamic-link-info.android-info.android-package-name" => Some(("dynamicLinkInfo.androidInfo.androidPackageName", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "dynamic-link-info.desktop-info.desktop-fallback-link" => Some(("dynamicLinkInfo.desktopInfo.desktopFallbackLink", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "dynamic-link-info.domain-uri-prefix" => Some(("dynamicLinkInfo.domainUriPrefix", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "dynamic-link-info.dynamic-link-domain" => Some(("dynamicLinkInfo.dynamicLinkDomain", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "dynamic-link-info.ios-info.ios-app-store-id" => Some(("dynamicLinkInfo.iosInfo.iosAppStoreId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "dynamic-link-info.ios-info.ios-bundle-id" => Some(("dynamicLinkInfo.iosInfo.iosBundleId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "dynamic-link-info.ios-info.ios-custom-scheme" => Some(("dynamicLinkInfo.iosInfo.iosCustomScheme", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "dynamic-link-info.ios-info.ios-fallback-link" => Some(("dynamicLinkInfo.iosInfo.iosFallbackLink", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "dynamic-link-info.ios-info.ios-ipad-bundle-id" => Some(("dynamicLinkInfo.iosInfo.iosIpadBundleId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "dynamic-link-info.ios-info.ios-ipad-fallback-link" => Some(("dynamicLinkInfo.iosInfo.iosIpadFallbackLink", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "dynamic-link-info.ios-info.ios-minimum-version" => Some(("dynamicLinkInfo.iosInfo.iosMinimumVersion", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "dynamic-link-info.link" => Some(("dynamicLinkInfo.link", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "dynamic-link-info.navigation-info.enable-forced-redirect" => Some(("dynamicLinkInfo.navigationInfo.enableForcedRedirect", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "dynamic-link-info.social-meta-tag-info.social-description" => Some(("dynamicLinkInfo.socialMetaTagInfo.socialDescription", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "dynamic-link-info.social-meta-tag-info.social-image-link" => Some(("dynamicLinkInfo.socialMetaTagInfo.socialImageLink", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "dynamic-link-info.social-meta-tag-info.social-title" => Some(("dynamicLinkInfo.socialMetaTagInfo.socialTitle", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "long-dynamic-link" => Some(("longDynamicLink", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "sdk-version" => Some(("sdkVersion", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "suffix.custom-suffix" => Some(("suffix.customSuffix", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "suffix.option" => Some(("suffix.option", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    _ => {
                        let suggestion = FieldCursor::did_you_mean(key, &vec!["analytics-info", "android-fallback-link", "android-info", "android-link", "android-min-package-version-code", "android-package-name", "at", "ct", "custom-suffix", "desktop-fallback-link", "desktop-info", "domain-uri-prefix", "dynamic-link-domain", "dynamic-link-info", "enable-forced-redirect", "gclid", "google-play-analytics", "ios-app-store-id", "ios-bundle-id", "ios-custom-scheme", "ios-fallback-link", "ios-info", "ios-ipad-bundle-id", "ios-ipad-fallback-link", "ios-minimum-version", "itunes-connect-analytics", "link", "long-dynamic-link", "mt", "navigation-info", "option", "pt", "sdk-version", "social-description", "social-image-link", "social-meta-tag-info", "social-title", "suffix", "utm-campaign", "utm-content", "utm-medium", "utm-source", "utm-term"]);
                        err.issues.push(CLIError::Field(FieldError::Unknown(temp_cursor.to_string(), suggestion, value.map(|v| v.to_string()))));
                        None
                    }
                };
            if let Some((field_cursor_str, type_info)) = type_info {
                FieldCursor::from(field_cursor_str).set_json_value(&mut object, value.unwrap(), type_info, err, &temp_cursor);
            }
        }
        let mut request: api::CreateShortDynamicLinkRequest = json::value::from_value(object).unwrap();
        let mut call = self.hub.short_links().create(request);
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

    async fn _doit(&self, dry_run: bool) -> Result<Result<(), DoitError>, Option<InvalidOptionsError>> {
        let mut err = InvalidOptionsError::new();
        let mut call_result: Result<(), DoitError> = Ok(());
        let mut err_opt: Option<InvalidOptionsError> = None;
        match self.opt.subcommand() {
            ("managed-short-links", Some(opt)) => {
                match opt.subcommand() {
                    ("create", Some(opt)) => {
                        call_result = self._managed_short_links_create(opt, dry_run, &mut err).await;
                    },
                    _ => {
                        err.issues.push(CLIError::MissingMethodError("managed-short-links".to_string()));
                        writeln!(io::stderr(), "{}\n", opt.usage()).ok();
                    }
                }
            },
            ("methods", Some(opt)) => {
                match opt.subcommand() {
                    ("get-link-stats", Some(opt)) => {
                        call_result = self._methods_get_link_stats(opt, dry_run, &mut err).await;
                    },
                    ("install-attribution", Some(opt)) => {
                        call_result = self._methods_install_attribution(opt, dry_run, &mut err).await;
                    },
                    ("reopen-attribution", Some(opt)) => {
                        call_result = self._methods_reopen_attribution(opt, dry_run, &mut err).await;
                    },
                    _ => {
                        err.issues.push(CLIError::MissingMethodError("methods".to_string()));
                        writeln!(io::stderr(), "{}\n", opt.usage()).ok();
                    }
                }
            },
            ("short-links", Some(opt)) => {
                match opt.subcommand() {
                    ("create", Some(opt)) => {
                        call_result = self._short_links_create(opt, dry_run, &mut err).await;
                    },
                    _ => {
                        err.issues.push(CLIError::MissingMethodError("short-links".to_string()));
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

            match client::application_secret_from_directory(&config_dir, "firebasedynamiclinks1-secret.json",
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
        ).persist_tokens_to_disk(format!("{}/firebasedynamiclinks1", config_dir)).build().await.unwrap();

        let engine = Engine {
            opt: opt,
            hub: api::FirebaseDynamicLinks::new(client, auth),
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
        ("managed-short-links", "methods: 'create'", vec![
            ("create",
                    Some(r##"Creates a managed short Dynamic Link given either a valid long Dynamic Link or details such as Dynamic Link domain, Android and iOS app information. The created short Dynamic Link will not expire. This differs from CreateShortDynamicLink in the following ways: - The request will also contain a name for the link (non unique name for the front end). - The response must be authenticated with an auth token (generated with the admin service account). - The link will appear in the FDL list of links in the console front end. The Dynamic Link domain in the request must be owned by requester's Firebase project."##),
                    "Details at http://byron.github.io/google-apis-rs/google_firebasedynamiclinks1_cli/managed-short-links_create",
                  vec![
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
        
        ("methods", "methods: 'get-link-stats', 'install-attribution' and 'reopen-attribution'", vec![
            ("get-link-stats",
                    Some(r##"Fetches analytics stats of a short Dynamic Link for a given duration. Metrics include number of clicks, redirects, installs, app first opens, and app reopens."##),
                    "Details at http://byron.github.io/google-apis-rs/google_firebasedynamiclinks1_cli/methods_get-link-stats",
                  vec![
                    (Some(r##"dynamic-link"##),
                     None,
                     Some(r##"Dynamic Link URL. e.g. https://abcd.app.goo.gl/wxyz"##),
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
            ("install-attribution",
                    Some(r##"Get iOS strong/weak-match info for post-install attribution."##),
                    "Details at http://byron.github.io/google-apis-rs/google_firebasedynamiclinks1_cli/methods_install-attribution",
                  vec![
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
            ("reopen-attribution",
                    Some(r##"Get iOS reopen attribution for app universal link open deeplinking."##),
                    "Details at http://byron.github.io/google-apis-rs/google_firebasedynamiclinks1_cli/methods_reopen-attribution",
                  vec![
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
        
        ("short-links", "methods: 'create'", vec![
            ("create",
                    Some(r##"Creates a short Dynamic Link given either a valid long Dynamic Link or details such as Dynamic Link domain, Android and iOS app information. The created short Dynamic Link will not expire. Repeated calls with the same long Dynamic Link or Dynamic Link information will produce the same short Dynamic Link. The Dynamic Link domain in the request must be owned by requester's Firebase project."##),
                    "Details at http://byron.github.io/google-apis-rs/google_firebasedynamiclinks1_cli/short-links_create",
                  vec![
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
    
    let mut app = App::new("firebasedynamiclinks1")
           .author("Sebastian Thiel <byronimo@gmail.com>")
           .version("5.0.5+20240625")
           .about("Programmatically creates and manages Firebase Dynamic Links.")
           .after_help("All documentation details can be found at http://byron.github.io/google-apis-rs/google_firebasedynamiclinks1_cli")
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
