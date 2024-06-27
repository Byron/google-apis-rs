// DO NOT EDIT !
// This file was generated automatically from 'src/generator/templates/cli/main.rs.mako'
// DO NOT EDIT !
#![allow(unused_variables, unused_imports, dead_code, unused_mut)]

#[macro_use]
extern crate clap;

use std::env;
use std::io::{self, Write};
use clap::{App, SubCommand, Arg};

use google_mybusinessbusinessinformation1::{api, Error, oauth2, client::chrono, FieldMask};


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
    hub: api::MyBusinessBusinessInformation<S>,
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
    async fn _accounts_locations_create(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
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
                    "ad-words-location-extensions.ad-phone" => Some(("adWordsLocationExtensions.adPhone", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "categories.primary-category.display-name" => Some(("categories.primaryCategory.displayName", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "categories.primary-category.name" => Some(("categories.primaryCategory.name", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "labels" => Some(("labels", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Vec })),
                    "language-code" => Some(("languageCode", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "latlng.latitude" => Some(("latlng.latitude", JsonTypeInfo { jtype: JsonType::Float, ctype: ComplexType::Pod })),
                    "latlng.longitude" => Some(("latlng.longitude", JsonTypeInfo { jtype: JsonType::Float, ctype: ComplexType::Pod })),
                    "metadata.can-delete" => Some(("metadata.canDelete", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "metadata.can-have-business-calls" => Some(("metadata.canHaveBusinessCalls", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "metadata.can-have-food-menus" => Some(("metadata.canHaveFoodMenus", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "metadata.can-modify-service-list" => Some(("metadata.canModifyServiceList", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "metadata.can-operate-health-data" => Some(("metadata.canOperateHealthData", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "metadata.can-operate-local-post" => Some(("metadata.canOperateLocalPost", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "metadata.can-operate-lodging-data" => Some(("metadata.canOperateLodgingData", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "metadata.duplicate-location" => Some(("metadata.duplicateLocation", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "metadata.has-google-updated" => Some(("metadata.hasGoogleUpdated", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "metadata.has-pending-edits" => Some(("metadata.hasPendingEdits", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "metadata.has-voice-of-merchant" => Some(("metadata.hasVoiceOfMerchant", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "metadata.maps-uri" => Some(("metadata.mapsUri", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "metadata.new-review-uri" => Some(("metadata.newReviewUri", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "metadata.place-id" => Some(("metadata.placeId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "name" => Some(("name", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "open-info.can-reopen" => Some(("openInfo.canReopen", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "open-info.opening-date.day" => Some(("openInfo.openingDate.day", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "open-info.opening-date.month" => Some(("openInfo.openingDate.month", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "open-info.opening-date.year" => Some(("openInfo.openingDate.year", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "open-info.status" => Some(("openInfo.status", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "phone-numbers.additional-phones" => Some(("phoneNumbers.additionalPhones", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Vec })),
                    "phone-numbers.primary-phone" => Some(("phoneNumbers.primaryPhone", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "profile.description" => Some(("profile.description", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "relationship-data.parent-chain" => Some(("relationshipData.parentChain", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "relationship-data.parent-location.place-id" => Some(("relationshipData.parentLocation.placeId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "relationship-data.parent-location.relation-type" => Some(("relationshipData.parentLocation.relationType", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "service-area.business-type" => Some(("serviceArea.businessType", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "service-area.region-code" => Some(("serviceArea.regionCode", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "store-code" => Some(("storeCode", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "storefront-address.address-lines" => Some(("storefrontAddress.addressLines", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Vec })),
                    "storefront-address.administrative-area" => Some(("storefrontAddress.administrativeArea", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "storefront-address.language-code" => Some(("storefrontAddress.languageCode", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "storefront-address.locality" => Some(("storefrontAddress.locality", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "storefront-address.organization" => Some(("storefrontAddress.organization", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "storefront-address.postal-code" => Some(("storefrontAddress.postalCode", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "storefront-address.recipients" => Some(("storefrontAddress.recipients", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Vec })),
                    "storefront-address.region-code" => Some(("storefrontAddress.regionCode", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "storefront-address.revision" => Some(("storefrontAddress.revision", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "storefront-address.sorting-code" => Some(("storefrontAddress.sortingCode", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "storefront-address.sublocality" => Some(("storefrontAddress.sublocality", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "title" => Some(("title", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "website-uri" => Some(("websiteUri", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    _ => {
                        let suggestion = FieldCursor::did_you_mean(key, &vec!["ad-phone", "ad-words-location-extensions", "additional-phones", "address-lines", "administrative-area", "business-type", "can-delete", "can-have-business-calls", "can-have-food-menus", "can-modify-service-list", "can-operate-health-data", "can-operate-local-post", "can-operate-lodging-data", "can-reopen", "categories", "day", "description", "display-name", "duplicate-location", "has-google-updated", "has-pending-edits", "has-voice-of-merchant", "labels", "language-code", "latitude", "latlng", "locality", "longitude", "maps-uri", "metadata", "month", "name", "new-review-uri", "open-info", "opening-date", "organization", "parent-chain", "parent-location", "phone-numbers", "place-id", "postal-code", "primary-category", "primary-phone", "profile", "recipients", "region-code", "relation-type", "relationship-data", "revision", "service-area", "sorting-code", "status", "store-code", "storefront-address", "sublocality", "title", "website-uri", "year"]);
                        err.issues.push(CLIError::Field(FieldError::Unknown(temp_cursor.to_string(), suggestion, value.map(|v| v.to_string()))));
                        None
                    }
                };
            if let Some((field_cursor_str, type_info)) = type_info {
                FieldCursor::from(field_cursor_str).set_json_value(&mut object, value.unwrap(), type_info, err, &temp_cursor);
            }
        }
        let mut request: api::Location = json::value::from_value(object).unwrap();
        let mut call = self.hub.accounts().locations_create(request, opt.value_of("parent").unwrap_or(""));
        for parg in opt.values_of("v").map(|i|i.collect()).unwrap_or(Vec::new()).iter() {
            let (key, value) = parse_kv_arg(&*parg, err, false);
            match key {
                "validate-only" => {
                    call = call.validate_only(        value.map(|v| arg_from_str(v, err, "validate-only", "boolean")).unwrap_or(false));
                },
                "request-id" => {
                    call = call.request_id(value.unwrap_or(""));
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
                                                                           v.extend(["request-id", "validate-only"].iter().map(|v|*v));
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

    async fn _accounts_locations_list(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        let mut call = self.hub.accounts().locations_list(opt.value_of("parent").unwrap_or(""));
        for parg in opt.values_of("v").map(|i|i.collect()).unwrap_or(Vec::new()).iter() {
            let (key, value) = parse_kv_arg(&*parg, err, false);
            match key {
                "read-mask" => {
                    call = call.read_mask(        value.map(|v| arg_from_str(v, err, "read-mask", "google-fieldmask")).unwrap_or(FieldMask::default()));
                },
                "page-token" => {
                    call = call.page_token(value.unwrap_or(""));
                },
                "page-size" => {
                    call = call.page_size(        value.map(|v| arg_from_str(v, err, "page-size", "int32")).unwrap_or(-0));
                },
                "order-by" => {
                    call = call.order_by(value.unwrap_or(""));
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
                                                                           v.extend(["filter", "order-by", "page-size", "page-token", "read-mask"].iter().map(|v|*v));
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

    async fn _attributes_list(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        let mut call = self.hub.attributes().list();
        for parg in opt.values_of("v").map(|i|i.collect()).unwrap_or(Vec::new()).iter() {
            let (key, value) = parse_kv_arg(&*parg, err, false);
            match key {
                "show-all" => {
                    call = call.show_all(        value.map(|v| arg_from_str(v, err, "show-all", "boolean")).unwrap_or(false));
                },
                "region-code" => {
                    call = call.region_code(value.unwrap_or(""));
                },
                "parent" => {
                    call = call.parent(value.unwrap_or(""));
                },
                "page-token" => {
                    call = call.page_token(value.unwrap_or(""));
                },
                "page-size" => {
                    call = call.page_size(        value.map(|v| arg_from_str(v, err, "page-size", "int32")).unwrap_or(-0));
                },
                "language-code" => {
                    call = call.language_code(value.unwrap_or(""));
                },
                "category-name" => {
                    call = call.category_name(value.unwrap_or(""));
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
                                                                           v.extend(["category-name", "language-code", "page-size", "page-token", "parent", "region-code", "show-all"].iter().map(|v|*v));
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

    async fn _categories_batch_get(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        let mut call = self.hub.categories().batch_get();
        for parg in opt.values_of("v").map(|i|i.collect()).unwrap_or(Vec::new()).iter() {
            let (key, value) = parse_kv_arg(&*parg, err, false);
            match key {
                "view" => {
                    call = call.view(value.unwrap_or(""));
                },
                "region-code" => {
                    call = call.region_code(value.unwrap_or(""));
                },
                "names" => {
                    call = call.add_names(value.unwrap_or(""));
                },
                "language-code" => {
                    call = call.language_code(value.unwrap_or(""));
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
                                                                           v.extend(["language-code", "names", "region-code", "view"].iter().map(|v|*v));
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

    async fn _categories_list(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        let mut call = self.hub.categories().list();
        for parg in opt.values_of("v").map(|i|i.collect()).unwrap_or(Vec::new()).iter() {
            let (key, value) = parse_kv_arg(&*parg, err, false);
            match key {
                "view" => {
                    call = call.view(value.unwrap_or(""));
                },
                "region-code" => {
                    call = call.region_code(value.unwrap_or(""));
                },
                "page-token" => {
                    call = call.page_token(value.unwrap_or(""));
                },
                "page-size" => {
                    call = call.page_size(        value.map(|v| arg_from_str(v, err, "page-size", "int32")).unwrap_or(-0));
                },
                "language-code" => {
                    call = call.language_code(value.unwrap_or(""));
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
                                                                           v.extend(["filter", "language-code", "page-size", "page-token", "region-code", "view"].iter().map(|v|*v));
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

    async fn _chains_get(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        let mut call = self.hub.chains().get(opt.value_of("name").unwrap_or(""));
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

    async fn _chains_search(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        let mut call = self.hub.chains().search();
        for parg in opt.values_of("v").map(|i|i.collect()).unwrap_or(Vec::new()).iter() {
            let (key, value) = parse_kv_arg(&*parg, err, false);
            match key {
                "page-size" => {
                    call = call.page_size(        value.map(|v| arg_from_str(v, err, "page-size", "int32")).unwrap_or(-0));
                },
                "chain-name" => {
                    call = call.chain_name(value.unwrap_or(""));
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
                                                                           v.extend(["chain-name", "page-size"].iter().map(|v|*v));
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

    async fn _google_locations_search(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
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
                    "location.ad-words-location-extensions.ad-phone" => Some(("location.adWordsLocationExtensions.adPhone", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "location.categories.primary-category.display-name" => Some(("location.categories.primaryCategory.displayName", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "location.categories.primary-category.name" => Some(("location.categories.primaryCategory.name", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "location.labels" => Some(("location.labels", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Vec })),
                    "location.language-code" => Some(("location.languageCode", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "location.latlng.latitude" => Some(("location.latlng.latitude", JsonTypeInfo { jtype: JsonType::Float, ctype: ComplexType::Pod })),
                    "location.latlng.longitude" => Some(("location.latlng.longitude", JsonTypeInfo { jtype: JsonType::Float, ctype: ComplexType::Pod })),
                    "location.metadata.can-delete" => Some(("location.metadata.canDelete", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "location.metadata.can-have-business-calls" => Some(("location.metadata.canHaveBusinessCalls", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "location.metadata.can-have-food-menus" => Some(("location.metadata.canHaveFoodMenus", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "location.metadata.can-modify-service-list" => Some(("location.metadata.canModifyServiceList", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "location.metadata.can-operate-health-data" => Some(("location.metadata.canOperateHealthData", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "location.metadata.can-operate-local-post" => Some(("location.metadata.canOperateLocalPost", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "location.metadata.can-operate-lodging-data" => Some(("location.metadata.canOperateLodgingData", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "location.metadata.duplicate-location" => Some(("location.metadata.duplicateLocation", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "location.metadata.has-google-updated" => Some(("location.metadata.hasGoogleUpdated", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "location.metadata.has-pending-edits" => Some(("location.metadata.hasPendingEdits", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "location.metadata.has-voice-of-merchant" => Some(("location.metadata.hasVoiceOfMerchant", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "location.metadata.maps-uri" => Some(("location.metadata.mapsUri", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "location.metadata.new-review-uri" => Some(("location.metadata.newReviewUri", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "location.metadata.place-id" => Some(("location.metadata.placeId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "location.name" => Some(("location.name", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "location.open-info.can-reopen" => Some(("location.openInfo.canReopen", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "location.open-info.opening-date.day" => Some(("location.openInfo.openingDate.day", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "location.open-info.opening-date.month" => Some(("location.openInfo.openingDate.month", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "location.open-info.opening-date.year" => Some(("location.openInfo.openingDate.year", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "location.open-info.status" => Some(("location.openInfo.status", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "location.phone-numbers.additional-phones" => Some(("location.phoneNumbers.additionalPhones", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Vec })),
                    "location.phone-numbers.primary-phone" => Some(("location.phoneNumbers.primaryPhone", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "location.profile.description" => Some(("location.profile.description", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "location.relationship-data.parent-chain" => Some(("location.relationshipData.parentChain", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "location.relationship-data.parent-location.place-id" => Some(("location.relationshipData.parentLocation.placeId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "location.relationship-data.parent-location.relation-type" => Some(("location.relationshipData.parentLocation.relationType", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "location.service-area.business-type" => Some(("location.serviceArea.businessType", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "location.service-area.region-code" => Some(("location.serviceArea.regionCode", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "location.store-code" => Some(("location.storeCode", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "location.storefront-address.address-lines" => Some(("location.storefrontAddress.addressLines", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Vec })),
                    "location.storefront-address.administrative-area" => Some(("location.storefrontAddress.administrativeArea", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "location.storefront-address.language-code" => Some(("location.storefrontAddress.languageCode", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "location.storefront-address.locality" => Some(("location.storefrontAddress.locality", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "location.storefront-address.organization" => Some(("location.storefrontAddress.organization", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "location.storefront-address.postal-code" => Some(("location.storefrontAddress.postalCode", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "location.storefront-address.recipients" => Some(("location.storefrontAddress.recipients", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Vec })),
                    "location.storefront-address.region-code" => Some(("location.storefrontAddress.regionCode", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "location.storefront-address.revision" => Some(("location.storefrontAddress.revision", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "location.storefront-address.sorting-code" => Some(("location.storefrontAddress.sortingCode", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "location.storefront-address.sublocality" => Some(("location.storefrontAddress.sublocality", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "location.title" => Some(("location.title", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "location.website-uri" => Some(("location.websiteUri", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "page-size" => Some(("pageSize", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "query" => Some(("query", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    _ => {
                        let suggestion = FieldCursor::did_you_mean(key, &vec!["ad-phone", "ad-words-location-extensions", "additional-phones", "address-lines", "administrative-area", "business-type", "can-delete", "can-have-business-calls", "can-have-food-menus", "can-modify-service-list", "can-operate-health-data", "can-operate-local-post", "can-operate-lodging-data", "can-reopen", "categories", "day", "description", "display-name", "duplicate-location", "has-google-updated", "has-pending-edits", "has-voice-of-merchant", "labels", "language-code", "latitude", "latlng", "locality", "location", "longitude", "maps-uri", "metadata", "month", "name", "new-review-uri", "open-info", "opening-date", "organization", "page-size", "parent-chain", "parent-location", "phone-numbers", "place-id", "postal-code", "primary-category", "primary-phone", "profile", "query", "recipients", "region-code", "relation-type", "relationship-data", "revision", "service-area", "sorting-code", "status", "store-code", "storefront-address", "sublocality", "title", "website-uri", "year"]);
                        err.issues.push(CLIError::Field(FieldError::Unknown(temp_cursor.to_string(), suggestion, value.map(|v| v.to_string()))));
                        None
                    }
                };
            if let Some((field_cursor_str, type_info)) = type_info {
                FieldCursor::from(field_cursor_str).set_json_value(&mut object, value.unwrap(), type_info, err, &temp_cursor);
            }
        }
        let mut request: api::SearchGoogleLocationsRequest = json::value::from_value(object).unwrap();
        let mut call = self.hub.google_locations().search(request);
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

    async fn _locations_attributes_get_google_updated(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        let mut call = self.hub.locations().attributes_get_google_updated(opt.value_of("name").unwrap_or(""));
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

    async fn _locations_delete(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        let mut call = self.hub.locations().delete(opt.value_of("name").unwrap_or(""));
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

    async fn _locations_get(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        let mut call = self.hub.locations().get(opt.value_of("name").unwrap_or(""));
        for parg in opt.values_of("v").map(|i|i.collect()).unwrap_or(Vec::new()).iter() {
            let (key, value) = parse_kv_arg(&*parg, err, false);
            match key {
                "read-mask" => {
                    call = call.read_mask(        value.map(|v| arg_from_str(v, err, "read-mask", "google-fieldmask")).unwrap_or(FieldMask::default()));
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
                                                                           v.extend(["read-mask"].iter().map(|v|*v));
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

    async fn _locations_get_attributes(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        let mut call = self.hub.locations().get_attributes(opt.value_of("name").unwrap_or(""));
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

    async fn _locations_get_google_updated(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        let mut call = self.hub.locations().get_google_updated(opt.value_of("name").unwrap_or(""));
        for parg in opt.values_of("v").map(|i|i.collect()).unwrap_or(Vec::new()).iter() {
            let (key, value) = parse_kv_arg(&*parg, err, false);
            match key {
                "read-mask" => {
                    call = call.read_mask(        value.map(|v| arg_from_str(v, err, "read-mask", "google-fieldmask")).unwrap_or(FieldMask::default()));
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
                                                                           v.extend(["read-mask"].iter().map(|v|*v));
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

    async fn _locations_patch(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
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
                    "ad-words-location-extensions.ad-phone" => Some(("adWordsLocationExtensions.adPhone", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "categories.primary-category.display-name" => Some(("categories.primaryCategory.displayName", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "categories.primary-category.name" => Some(("categories.primaryCategory.name", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "labels" => Some(("labels", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Vec })),
                    "language-code" => Some(("languageCode", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "latlng.latitude" => Some(("latlng.latitude", JsonTypeInfo { jtype: JsonType::Float, ctype: ComplexType::Pod })),
                    "latlng.longitude" => Some(("latlng.longitude", JsonTypeInfo { jtype: JsonType::Float, ctype: ComplexType::Pod })),
                    "metadata.can-delete" => Some(("metadata.canDelete", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "metadata.can-have-business-calls" => Some(("metadata.canHaveBusinessCalls", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "metadata.can-have-food-menus" => Some(("metadata.canHaveFoodMenus", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "metadata.can-modify-service-list" => Some(("metadata.canModifyServiceList", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "metadata.can-operate-health-data" => Some(("metadata.canOperateHealthData", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "metadata.can-operate-local-post" => Some(("metadata.canOperateLocalPost", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "metadata.can-operate-lodging-data" => Some(("metadata.canOperateLodgingData", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "metadata.duplicate-location" => Some(("metadata.duplicateLocation", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "metadata.has-google-updated" => Some(("metadata.hasGoogleUpdated", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "metadata.has-pending-edits" => Some(("metadata.hasPendingEdits", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "metadata.has-voice-of-merchant" => Some(("metadata.hasVoiceOfMerchant", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "metadata.maps-uri" => Some(("metadata.mapsUri", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "metadata.new-review-uri" => Some(("metadata.newReviewUri", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "metadata.place-id" => Some(("metadata.placeId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "name" => Some(("name", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "open-info.can-reopen" => Some(("openInfo.canReopen", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "open-info.opening-date.day" => Some(("openInfo.openingDate.day", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "open-info.opening-date.month" => Some(("openInfo.openingDate.month", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "open-info.opening-date.year" => Some(("openInfo.openingDate.year", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "open-info.status" => Some(("openInfo.status", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "phone-numbers.additional-phones" => Some(("phoneNumbers.additionalPhones", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Vec })),
                    "phone-numbers.primary-phone" => Some(("phoneNumbers.primaryPhone", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "profile.description" => Some(("profile.description", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "relationship-data.parent-chain" => Some(("relationshipData.parentChain", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "relationship-data.parent-location.place-id" => Some(("relationshipData.parentLocation.placeId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "relationship-data.parent-location.relation-type" => Some(("relationshipData.parentLocation.relationType", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "service-area.business-type" => Some(("serviceArea.businessType", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "service-area.region-code" => Some(("serviceArea.regionCode", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "store-code" => Some(("storeCode", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "storefront-address.address-lines" => Some(("storefrontAddress.addressLines", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Vec })),
                    "storefront-address.administrative-area" => Some(("storefrontAddress.administrativeArea", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "storefront-address.language-code" => Some(("storefrontAddress.languageCode", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "storefront-address.locality" => Some(("storefrontAddress.locality", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "storefront-address.organization" => Some(("storefrontAddress.organization", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "storefront-address.postal-code" => Some(("storefrontAddress.postalCode", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "storefront-address.recipients" => Some(("storefrontAddress.recipients", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Vec })),
                    "storefront-address.region-code" => Some(("storefrontAddress.regionCode", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "storefront-address.revision" => Some(("storefrontAddress.revision", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "storefront-address.sorting-code" => Some(("storefrontAddress.sortingCode", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "storefront-address.sublocality" => Some(("storefrontAddress.sublocality", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "title" => Some(("title", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "website-uri" => Some(("websiteUri", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    _ => {
                        let suggestion = FieldCursor::did_you_mean(key, &vec!["ad-phone", "ad-words-location-extensions", "additional-phones", "address-lines", "administrative-area", "business-type", "can-delete", "can-have-business-calls", "can-have-food-menus", "can-modify-service-list", "can-operate-health-data", "can-operate-local-post", "can-operate-lodging-data", "can-reopen", "categories", "day", "description", "display-name", "duplicate-location", "has-google-updated", "has-pending-edits", "has-voice-of-merchant", "labels", "language-code", "latitude", "latlng", "locality", "longitude", "maps-uri", "metadata", "month", "name", "new-review-uri", "open-info", "opening-date", "organization", "parent-chain", "parent-location", "phone-numbers", "place-id", "postal-code", "primary-category", "primary-phone", "profile", "recipients", "region-code", "relation-type", "relationship-data", "revision", "service-area", "sorting-code", "status", "store-code", "storefront-address", "sublocality", "title", "website-uri", "year"]);
                        err.issues.push(CLIError::Field(FieldError::Unknown(temp_cursor.to_string(), suggestion, value.map(|v| v.to_string()))));
                        None
                    }
                };
            if let Some((field_cursor_str, type_info)) = type_info {
                FieldCursor::from(field_cursor_str).set_json_value(&mut object, value.unwrap(), type_info, err, &temp_cursor);
            }
        }
        let mut request: api::Location = json::value::from_value(object).unwrap();
        let mut call = self.hub.locations().patch(request, opt.value_of("name").unwrap_or(""));
        for parg in opt.values_of("v").map(|i|i.collect()).unwrap_or(Vec::new()).iter() {
            let (key, value) = parse_kv_arg(&*parg, err, false);
            match key {
                "validate-only" => {
                    call = call.validate_only(        value.map(|v| arg_from_str(v, err, "validate-only", "boolean")).unwrap_or(false));
                },
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
                                                                           v.extend(["update-mask", "validate-only"].iter().map(|v|*v));
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

    async fn _locations_update_attributes(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
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
                    "name" => Some(("name", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    _ => {
                        let suggestion = FieldCursor::did_you_mean(key, &vec!["name"]);
                        err.issues.push(CLIError::Field(FieldError::Unknown(temp_cursor.to_string(), suggestion, value.map(|v| v.to_string()))));
                        None
                    }
                };
            if let Some((field_cursor_str, type_info)) = type_info {
                FieldCursor::from(field_cursor_str).set_json_value(&mut object, value.unwrap(), type_info, err, &temp_cursor);
            }
        }
        let mut request: api::Attributes = json::value::from_value(object).unwrap();
        let mut call = self.hub.locations().update_attributes(request, opt.value_of("name").unwrap_or(""));
        for parg in opt.values_of("v").map(|i|i.collect()).unwrap_or(Vec::new()).iter() {
            let (key, value) = parse_kv_arg(&*parg, err, false);
            match key {
                "attribute-mask" => {
                    call = call.attribute_mask(        value.map(|v| arg_from_str(v, err, "attribute-mask", "google-fieldmask")).unwrap_or(FieldMask::default()));
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
                                                                           v.extend(["attribute-mask"].iter().map(|v|*v));
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
            ("accounts", Some(opt)) => {
                match opt.subcommand() {
                    ("locations-create", Some(opt)) => {
                        call_result = self._accounts_locations_create(opt, dry_run, &mut err).await;
                    },
                    ("locations-list", Some(opt)) => {
                        call_result = self._accounts_locations_list(opt, dry_run, &mut err).await;
                    },
                    _ => {
                        err.issues.push(CLIError::MissingMethodError("accounts".to_string()));
                        writeln!(io::stderr(), "{}\n", opt.usage()).ok();
                    }
                }
            },
            ("attributes", Some(opt)) => {
                match opt.subcommand() {
                    ("list", Some(opt)) => {
                        call_result = self._attributes_list(opt, dry_run, &mut err).await;
                    },
                    _ => {
                        err.issues.push(CLIError::MissingMethodError("attributes".to_string()));
                        writeln!(io::stderr(), "{}\n", opt.usage()).ok();
                    }
                }
            },
            ("categories", Some(opt)) => {
                match opt.subcommand() {
                    ("batch-get", Some(opt)) => {
                        call_result = self._categories_batch_get(opt, dry_run, &mut err).await;
                    },
                    ("list", Some(opt)) => {
                        call_result = self._categories_list(opt, dry_run, &mut err).await;
                    },
                    _ => {
                        err.issues.push(CLIError::MissingMethodError("categories".to_string()));
                        writeln!(io::stderr(), "{}\n", opt.usage()).ok();
                    }
                }
            },
            ("chains", Some(opt)) => {
                match opt.subcommand() {
                    ("get", Some(opt)) => {
                        call_result = self._chains_get(opt, dry_run, &mut err).await;
                    },
                    ("search", Some(opt)) => {
                        call_result = self._chains_search(opt, dry_run, &mut err).await;
                    },
                    _ => {
                        err.issues.push(CLIError::MissingMethodError("chains".to_string()));
                        writeln!(io::stderr(), "{}\n", opt.usage()).ok();
                    }
                }
            },
            ("google-locations", Some(opt)) => {
                match opt.subcommand() {
                    ("search", Some(opt)) => {
                        call_result = self._google_locations_search(opt, dry_run, &mut err).await;
                    },
                    _ => {
                        err.issues.push(CLIError::MissingMethodError("google-locations".to_string()));
                        writeln!(io::stderr(), "{}\n", opt.usage()).ok();
                    }
                }
            },
            ("locations", Some(opt)) => {
                match opt.subcommand() {
                    ("attributes-get-google-updated", Some(opt)) => {
                        call_result = self._locations_attributes_get_google_updated(opt, dry_run, &mut err).await;
                    },
                    ("delete", Some(opt)) => {
                        call_result = self._locations_delete(opt, dry_run, &mut err).await;
                    },
                    ("get", Some(opt)) => {
                        call_result = self._locations_get(opt, dry_run, &mut err).await;
                    },
                    ("get-attributes", Some(opt)) => {
                        call_result = self._locations_get_attributes(opt, dry_run, &mut err).await;
                    },
                    ("get-google-updated", Some(opt)) => {
                        call_result = self._locations_get_google_updated(opt, dry_run, &mut err).await;
                    },
                    ("patch", Some(opt)) => {
                        call_result = self._locations_patch(opt, dry_run, &mut err).await;
                    },
                    ("update-attributes", Some(opt)) => {
                        call_result = self._locations_update_attributes(opt, dry_run, &mut err).await;
                    },
                    _ => {
                        err.issues.push(CLIError::MissingMethodError("locations".to_string()));
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

            match client::application_secret_from_directory(&config_dir, "mybusinessbusinessinformation1-secret.json",
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
        ).persist_tokens_to_disk(format!("{}/mybusinessbusinessinformation1", config_dir)).build().await.unwrap();

        let engine = Engine {
            opt: opt,
            hub: api::MyBusinessBusinessInformation::new(client, auth),
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
        ("accounts", "methods: 'locations-create' and 'locations-list'", vec![
            ("locations-create",
                    Some(r##"Creates a new Location that will be owned by the logged in user."##),
                    "Details at http://byron.github.io/google-apis-rs/google_mybusinessbusinessinformation1_cli/accounts_locations-create",
                  vec![
                    (Some(r##"parent"##),
                     None,
                     Some(r##"Required. The name of the account in which to create this location."##),
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
            ("locations-list",
                    Some(r##"Lists the locations for the specified account."##),
                    "Details at http://byron.github.io/google-apis-rs/google_mybusinessbusinessinformation1_cli/accounts_locations-list",
                  vec![
                    (Some(r##"parent"##),
                     None,
                     Some(r##"Required. The name of the account to fetch locations from. If the parent Account is of AccountType PERSONAL, only Locations that are directly owned by the Account are returned, otherwise it will return all accessible locations from the Account, either directly or indirectly."##),
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
        
        ("attributes", "methods: 'list'", vec![
            ("list",
                    Some(r##"Returns the list of attributes that would be available for a location with the given primary category and country."##),
                    "Details at http://byron.github.io/google-apis-rs/google_mybusinessbusinessinformation1_cli/attributes_list",
                  vec![
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
        
        ("categories", "methods: 'batch-get' and 'list'", vec![
            ("batch-get",
                    Some(r##"Returns a list of business categories for the provided language and GConcept ids."##),
                    "Details at http://byron.github.io/google-apis-rs/google_mybusinessbusinessinformation1_cli/categories_batch-get",
                  vec![
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
            ("list",
                    Some(r##"Returns a list of business categories. Search will match the category name but not the category ID. Search only matches the front of a category name (that is, 'food' may return 'Food Court' but not 'Fast Food Restaurant')."##),
                    "Details at http://byron.github.io/google-apis-rs/google_mybusinessbusinessinformation1_cli/categories_list",
                  vec![
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
        
        ("chains", "methods: 'get' and 'search'", vec![
            ("get",
                    Some(r##"Gets the specified chain. Returns `NOT_FOUND` if the chain does not exist."##),
                    "Details at http://byron.github.io/google-apis-rs/google_mybusinessbusinessinformation1_cli/chains_get",
                  vec![
                    (Some(r##"name"##),
                     None,
                     Some(r##"Required. The chain's resource name, in the format `chains/{chain_place_id}`."##),
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
            ("search",
                    Some(r##"Searches the chain based on chain name."##),
                    "Details at http://byron.github.io/google-apis-rs/google_mybusinessbusinessinformation1_cli/chains_search",
                  vec![
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
        
        ("google-locations", "methods: 'search'", vec![
            ("search",
                    Some(r##"Search all of the possible locations that are a match to the specified request."##),
                    "Details at http://byron.github.io/google-apis-rs/google_mybusinessbusinessinformation1_cli/google-locations_search",
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
        
        ("locations", "methods: 'attributes-get-google-updated', 'delete', 'get', 'get-attributes', 'get-google-updated', 'patch' and 'update-attributes'", vec![
            ("attributes-get-google-updated",
                    Some(r##"Gets the Google-updated version of the specified location."##),
                    "Details at http://byron.github.io/google-apis-rs/google_mybusinessbusinessinformation1_cli/locations_attributes-get-google-updated",
                  vec![
                    (Some(r##"name"##),
                     None,
                     Some(r##"Required. Google identifier for this location in the form of `locations/{location_id}/attributes`."##),
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
            ("delete",
                    Some(r##"Deletes a location. If this location cannot be deleted using the API and it is marked so in the `google.mybusiness.businessinformation.v1.LocationState`, use the [Google Business Profile](https://business.google.com/manage/) website."##),
                    "Details at http://byron.github.io/google-apis-rs/google_mybusinessbusinessinformation1_cli/locations_delete",
                  vec![
                    (Some(r##"name"##),
                     None,
                     Some(r##"Required. The name of the location to delete."##),
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
            ("get",
                    Some(r##"Returns the specified location."##),
                    "Details at http://byron.github.io/google-apis-rs/google_mybusinessbusinessinformation1_cli/locations_get",
                  vec![
                    (Some(r##"name"##),
                     None,
                     Some(r##"Required. The name of the location to fetch."##),
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
            ("get-attributes",
                    Some(r##"Looks up all the attributes set for a given location."##),
                    "Details at http://byron.github.io/google-apis-rs/google_mybusinessbusinessinformation1_cli/locations_get-attributes",
                  vec![
                    (Some(r##"name"##),
                     None,
                     Some(r##"Required. Google identifier for this location in the form of `locations/{location_id}/attributes`."##),
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
            ("get-google-updated",
                    Some(r##"Gets the Google-updated version of the specified location."##),
                    "Details at http://byron.github.io/google-apis-rs/google_mybusinessbusinessinformation1_cli/locations_get-google-updated",
                  vec![
                    (Some(r##"name"##),
                     None,
                     Some(r##"Required. The name of the location to fetch."##),
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
            ("patch",
                    Some(r##"Updates the specified location."##),
                    "Details at http://byron.github.io/google-apis-rs/google_mybusinessbusinessinformation1_cli/locations_patch",
                  vec![
                    (Some(r##"name"##),
                     None,
                     Some(r##"Google identifier for this location in the form: `locations/{location_id}`."##),
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
            ("update-attributes",
                    Some(r##"Update attributes for a given location."##),
                    "Details at http://byron.github.io/google-apis-rs/google_mybusinessbusinessinformation1_cli/locations_update-attributes",
                  vec![
                    (Some(r##"name"##),
                     None,
                     Some(r##"Required. Google identifier for this location in the form of `locations/{location_id}/attributes`."##),
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
    
    let mut app = App::new("mybusinessbusinessinformation1")
           .author("Sebastian Thiel <byronimo@gmail.com>")
           .version("5.0.5+20240625")
           .about("The My Business Business Information API provides an interface for managing business information. Note - If you have a quota of 0 after enabling the API, please request for GBP API access.")
           .after_help("All documentation details can be found at http://byron.github.io/google-apis-rs/google_mybusinessbusinessinformation1_cli")
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
