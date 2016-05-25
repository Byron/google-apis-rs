// DO NOT EDIT !
// This file was generated automatically from 'src/mako/cli/main.rs.mako'
// DO NOT EDIT !
#![allow(unused_variables, unused_imports, dead_code, unused_mut)]

#[macro_use]
extern crate clap;
extern crate yup_oauth2 as oauth2;
extern crate yup_hyper_mock as mock;
extern crate serde;
extern crate serde_json;
extern crate hyper;
extern crate mime;
extern crate strsim;
extern crate google_gan1_beta1 as api;

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
    hub: api::Gan<hyper::Client, Authenticator<DefaultAuthenticatorDelegate, JsonTokenStorage, hyper::Client>>,
    gp: Vec<&'static str>,
    gpm: Vec<(&'static str, &'static str)>,
}


impl<'n> Engine<'n> {
    fn _advertisers_get(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        let mut call = self.hub.advertisers().get(opt.value_of("role").unwrap_or(""), opt.value_of("role-id").unwrap_or(""));
        for parg in opt.values_of("v").map(|i|i.collect()).unwrap_or(Vec::new()).iter() {
            let (key, value) = parse_kv_arg(&*parg, err, false);
            match key {
                "advertiser-id" => {
                    call = call.advertiser_id(value.unwrap_or(""));
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
                                                                           v.extend(["advertiser-id"].iter().map(|v|*v));
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
                CallType::Standard => call.doit(),
                _ => unreachable!()
            } {
                Err(api_err) => Err(DoitError::ApiError(api_err)),
                Ok((mut response, output_schema)) => {
                    let mut value = json::value::to_value(&output_schema);
                    remove_json_null_values(&mut value);
                    json::to_writer_pretty(&mut ostream, &value).unwrap();
                    ostream.flush().unwrap();
                    Ok(())
                }
            }
        }
    }

    fn _advertisers_list(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        let mut call = self.hub.advertisers().list(opt.value_of("role").unwrap_or(""), opt.value_of("role-id").unwrap_or(""));
        for parg in opt.values_of("v").map(|i|i.collect()).unwrap_or(Vec::new()).iter() {
            let (key, value) = parse_kv_arg(&*parg, err, false);
            match key {
                "relationship-status" => {
                    call = call.relationship_status(value.unwrap_or(""));
                },
                "page-token" => {
                    call = call.page_token(value.unwrap_or(""));
                },
                "min-seven-day-epc" => {
                    call = call.min_seven_day_epc(arg_from_str(value.unwrap_or("0.0"), err, "min-seven-day-epc", "number"));
                },
                "min-payout-rank" => {
                    call = call.min_payout_rank(arg_from_str(value.unwrap_or("-0"), err, "min-payout-rank", "integer"));
                },
                "min-ninety-day-epc" => {
                    call = call.min_ninety_day_epc(arg_from_str(value.unwrap_or("0.0"), err, "min-ninety-day-epc", "number"));
                },
                "max-results" => {
                    call = call.max_results(arg_from_str(value.unwrap_or("-0"), err, "max-results", "integer"));
                },
                "advertiser-category" => {
                    call = call.advertiser_category(value.unwrap_or(""));
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
                                                                           v.extend(["relationship-status", "min-seven-day-epc", "advertiser-category", "max-results", "page-token", "min-ninety-day-epc", "min-payout-rank"].iter().map(|v|*v));
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
                CallType::Standard => call.doit(),
                _ => unreachable!()
            } {
                Err(api_err) => Err(DoitError::ApiError(api_err)),
                Ok((mut response, output_schema)) => {
                    let mut value = json::value::to_value(&output_schema);
                    remove_json_null_values(&mut value);
                    json::to_writer_pretty(&mut ostream, &value).unwrap();
                    ostream.flush().unwrap();
                    Ok(())
                }
            }
        }
    }

    fn _cc_offers_list(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        let mut call = self.hub.cc_offers().list(opt.value_of("publisher").unwrap_or(""));
        for parg in opt.values_of("v").map(|i|i.collect()).unwrap_or(Vec::new()).iter() {
            let (key, value) = parse_kv_arg(&*parg, err, false);
            match key {
                "projection" => {
                    call = call.projection(value.unwrap_or(""));
                },
                "advertiser" => {
                    call = call.add_advertiser(value.unwrap_or(""));
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
                                                                           v.extend(["advertiser", "projection"].iter().map(|v|*v));
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
                CallType::Standard => call.doit(),
                _ => unreachable!()
            } {
                Err(api_err) => Err(DoitError::ApiError(api_err)),
                Ok((mut response, output_schema)) => {
                    let mut value = json::value::to_value(&output_schema);
                    remove_json_null_values(&mut value);
                    json::to_writer_pretty(&mut ostream, &value).unwrap();
                    ostream.flush().unwrap();
                    Ok(())
                }
            }
        }
    }

    fn _events_list(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        let mut call = self.hub.events().list(opt.value_of("role").unwrap_or(""), opt.value_of("role-id").unwrap_or(""));
        for parg in opt.values_of("v").map(|i|i.collect()).unwrap_or(Vec::new()).iter() {
            let (key, value) = parse_kv_arg(&*parg, err, false);
            match key {
                "type" => {
                    call = call.type_(value.unwrap_or(""));
                },
                "status" => {
                    call = call.status(value.unwrap_or(""));
                },
                "sku" => {
                    call = call.sku(value.unwrap_or(""));
                },
                "publisher-id" => {
                    call = call.publisher_id(value.unwrap_or(""));
                },
                "product-category" => {
                    call = call.product_category(value.unwrap_or(""));
                },
                "page-token" => {
                    call = call.page_token(value.unwrap_or(""));
                },
                "order-id" => {
                    call = call.order_id(value.unwrap_or(""));
                },
                "modify-date-min" => {
                    call = call.modify_date_min(value.unwrap_or(""));
                },
                "modify-date-max" => {
                    call = call.modify_date_max(value.unwrap_or(""));
                },
                "member-id" => {
                    call = call.member_id(value.unwrap_or(""));
                },
                "max-results" => {
                    call = call.max_results(arg_from_str(value.unwrap_or("-0"), err, "max-results", "integer"));
                },
                "link-id" => {
                    call = call.link_id(value.unwrap_or(""));
                },
                "event-date-min" => {
                    call = call.event_date_min(value.unwrap_or(""));
                },
                "event-date-max" => {
                    call = call.event_date_max(value.unwrap_or(""));
                },
                "charge-type" => {
                    call = call.charge_type(value.unwrap_or(""));
                },
                "advertiser-id" => {
                    call = call.advertiser_id(value.unwrap_or(""));
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
                                                                           v.extend(["status", "sku", "modify-date-max", "type", "link-id", "event-date-min", "member-id", "product-category", "order-id", "page-token", "advertiser-id", "max-results", "charge-type", "modify-date-min", "event-date-max", "publisher-id"].iter().map(|v|*v));
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
                CallType::Standard => call.doit(),
                _ => unreachable!()
            } {
                Err(api_err) => Err(DoitError::ApiError(api_err)),
                Ok((mut response, output_schema)) => {
                    let mut value = json::value::to_value(&output_schema);
                    remove_json_null_values(&mut value);
                    json::to_writer_pretty(&mut ostream, &value).unwrap();
                    ostream.flush().unwrap();
                    Ok(())
                }
            }
        }
    }

    fn _links_get(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        let mut call = self.hub.links().get(opt.value_of("role").unwrap_or(""), opt.value_of("role-id").unwrap_or(""), opt.value_of("link-id").unwrap_or(""));
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
                CallType::Standard => call.doit(),
                _ => unreachable!()
            } {
                Err(api_err) => Err(DoitError::ApiError(api_err)),
                Ok((mut response, output_schema)) => {
                    let mut value = json::value::to_value(&output_schema);
                    remove_json_null_values(&mut value);
                    json::to_writer_pretty(&mut ostream, &value).unwrap();
                    ostream.flush().unwrap();
                    Ok(())
                }
            }
        }
    }

    fn _links_insert(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
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
                    "link-type" => Some(("linkType", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "start-date" => Some(("startDate", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "kind" => Some(("kind", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "end-date" => Some(("endDate", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "description" => Some(("description", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "name" => Some(("name", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "special-offers.price-cut.amount" => Some(("specialOffers.priceCut.amount", JsonTypeInfo { jtype: JsonType::Float, ctype: ComplexType::Pod })),
                    "special-offers.price-cut.currency-code" => Some(("specialOffers.priceCut.currencyCode", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "special-offers.price-cut-min.amount" => Some(("specialOffers.priceCutMin.amount", JsonTypeInfo { jtype: JsonType::Float, ctype: ComplexType::Pod })),
                    "special-offers.price-cut-min.currency-code" => Some(("specialOffers.priceCutMin.currencyCode", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "special-offers.free-shipping" => Some(("specialOffers.freeShipping", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "special-offers.promotion-codes" => Some(("specialOffers.promotionCodes", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Vec })),
                    "special-offers.percent-off" => Some(("specialOffers.percentOff", JsonTypeInfo { jtype: JsonType::Float, ctype: ComplexType::Pod })),
                    "special-offers.percent-off-min.amount" => Some(("specialOffers.percentOffMin.amount", JsonTypeInfo { jtype: JsonType::Float, ctype: ComplexType::Pod })),
                    "special-offers.percent-off-min.currency-code" => Some(("specialOffers.percentOffMin.currencyCode", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "special-offers.free-gift" => Some(("specialOffers.freeGift", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "special-offers.free-shipping-min.amount" => Some(("specialOffers.freeShippingMin.amount", JsonTypeInfo { jtype: JsonType::Float, ctype: ComplexType::Pod })),
                    "special-offers.free-shipping-min.currency-code" => Some(("specialOffers.freeShippingMin.currencyCode", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "epc-seven-day-average.amount" => Some(("epcSevenDayAverage.amount", JsonTypeInfo { jtype: JsonType::Float, ctype: ComplexType::Pod })),
                    "epc-seven-day-average.currency-code" => Some(("epcSevenDayAverage.currencyCode", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "create-date" => Some(("createDate", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "image-alt-text" => Some(("imageAltText", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "id" => Some(("id", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "advertiser-id" => Some(("advertiserId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "is-active" => Some(("isActive", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "promotion-type" => Some(("promotionType", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "duration" => Some(("duration", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "authorship" => Some(("authorship", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "impression-tracking-url" => Some(("impressionTrackingUrl", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "epc-ninety-day-average.amount" => Some(("epcNinetyDayAverage.amount", JsonTypeInfo { jtype: JsonType::Float, ctype: ComplexType::Pod })),
                    "epc-ninety-day-average.currency-code" => Some(("epcNinetyDayAverage.currencyCode", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "availability" => Some(("availability", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "click-tracking-url" => Some(("clickTrackingUrl", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "destination-url" => Some(("destinationUrl", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    _ => {
                        let suggestion = FieldCursor::did_you_mean(key, &vec!["advertiser-id", "amount", "authorship", "availability", "click-tracking-url", "create-date", "currency-code", "description", "destination-url", "duration", "end-date", "epc-ninety-day-average", "epc-seven-day-average", "free-gift", "free-shipping", "free-shipping-min", "id", "image-alt-text", "impression-tracking-url", "is-active", "kind", "link-type", "name", "percent-off", "percent-off-min", "price-cut", "price-cut-min", "promotion-codes", "promotion-type", "special-offers", "start-date"]);
                        err.issues.push(CLIError::Field(FieldError::Unknown(temp_cursor.to_string(), suggestion, value.map(|v| v.to_string()))));
                        None
                    }
                };
            if let Some((field_cursor_str, type_info)) = type_info {
                FieldCursor::from(field_cursor_str).set_json_value(&mut object, value.unwrap(), type_info, err, &temp_cursor);
            }
        }
        let mut request: api::Link = json::value::from_value(object).unwrap();
        let mut call = self.hub.links().insert(request, opt.value_of("role").unwrap_or(""), opt.value_of("role-id").unwrap_or(""));
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
                CallType::Standard => call.doit(),
                _ => unreachable!()
            } {
                Err(api_err) => Err(DoitError::ApiError(api_err)),
                Ok((mut response, output_schema)) => {
                    let mut value = json::value::to_value(&output_schema);
                    remove_json_null_values(&mut value);
                    json::to_writer_pretty(&mut ostream, &value).unwrap();
                    ostream.flush().unwrap();
                    Ok(())
                }
            }
        }
    }

    fn _links_list(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        let mut call = self.hub.links().list(opt.value_of("role").unwrap_or(""), opt.value_of("role-id").unwrap_or(""));
        for parg in opt.values_of("v").map(|i|i.collect()).unwrap_or(Vec::new()).iter() {
            let (key, value) = parse_kv_arg(&*parg, err, false);
            match key {
                "start-date-min" => {
                    call = call.start_date_min(value.unwrap_or(""));
                },
                "start-date-max" => {
                    call = call.start_date_max(value.unwrap_or(""));
                },
                "search-text" => {
                    call = call.search_text(value.unwrap_or(""));
                },
                "relationship-status" => {
                    call = call.relationship_status(value.unwrap_or(""));
                },
                "promotion-type" => {
                    call = call.add_promotion_type(value.unwrap_or(""));
                },
                "page-token" => {
                    call = call.page_token(value.unwrap_or(""));
                },
                "max-results" => {
                    call = call.max_results(arg_from_str(value.unwrap_or("-0"), err, "max-results", "integer"));
                },
                "link-type" => {
                    call = call.link_type(value.unwrap_or(""));
                },
                "create-date-min" => {
                    call = call.create_date_min(value.unwrap_or(""));
                },
                "create-date-max" => {
                    call = call.create_date_max(value.unwrap_or(""));
                },
                "authorship" => {
                    call = call.authorship(value.unwrap_or(""));
                },
                "asset-size" => {
                    call = call.add_asset_size(value.unwrap_or(""));
                },
                "advertiser-id" => {
                    call = call.add_advertiser_id(value.unwrap_or(""));
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
                                                                           v.extend(["start-date-min", "link-type", "relationship-status", "search-text", "create-date-max", "create-date-min", "asset-size", "start-date-max", "advertiser-id", "page-token", "max-results", "promotion-type", "authorship"].iter().map(|v|*v));
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
                CallType::Standard => call.doit(),
                _ => unreachable!()
            } {
                Err(api_err) => Err(DoitError::ApiError(api_err)),
                Ok((mut response, output_schema)) => {
                    let mut value = json::value::to_value(&output_schema);
                    remove_json_null_values(&mut value);
                    json::to_writer_pretty(&mut ostream, &value).unwrap();
                    ostream.flush().unwrap();
                    Ok(())
                }
            }
        }
    }

    fn _publishers_get(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        let mut call = self.hub.publishers().get(opt.value_of("role").unwrap_or(""), opt.value_of("role-id").unwrap_or(""));
        for parg in opt.values_of("v").map(|i|i.collect()).unwrap_or(Vec::new()).iter() {
            let (key, value) = parse_kv_arg(&*parg, err, false);
            match key {
                "publisher-id" => {
                    call = call.publisher_id(value.unwrap_or(""));
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
                                                                           v.extend(["publisher-id"].iter().map(|v|*v));
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
                CallType::Standard => call.doit(),
                _ => unreachable!()
            } {
                Err(api_err) => Err(DoitError::ApiError(api_err)),
                Ok((mut response, output_schema)) => {
                    let mut value = json::value::to_value(&output_schema);
                    remove_json_null_values(&mut value);
                    json::to_writer_pretty(&mut ostream, &value).unwrap();
                    ostream.flush().unwrap();
                    Ok(())
                }
            }
        }
    }

    fn _publishers_list(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        let mut call = self.hub.publishers().list(opt.value_of("role").unwrap_or(""), opt.value_of("role-id").unwrap_or(""));
        for parg in opt.values_of("v").map(|i|i.collect()).unwrap_or(Vec::new()).iter() {
            let (key, value) = parse_kv_arg(&*parg, err, false);
            match key {
                "relationship-status" => {
                    call = call.relationship_status(value.unwrap_or(""));
                },
                "publisher-category" => {
                    call = call.publisher_category(value.unwrap_or(""));
                },
                "page-token" => {
                    call = call.page_token(value.unwrap_or(""));
                },
                "min-seven-day-epc" => {
                    call = call.min_seven_day_epc(arg_from_str(value.unwrap_or("0.0"), err, "min-seven-day-epc", "number"));
                },
                "min-payout-rank" => {
                    call = call.min_payout_rank(arg_from_str(value.unwrap_or("-0"), err, "min-payout-rank", "integer"));
                },
                "min-ninety-day-epc" => {
                    call = call.min_ninety_day_epc(arg_from_str(value.unwrap_or("0.0"), err, "min-ninety-day-epc", "number"));
                },
                "max-results" => {
                    call = call.max_results(arg_from_str(value.unwrap_or("-0"), err, "max-results", "integer"));
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
                                                                           v.extend(["publisher-category", "relationship-status", "min-seven-day-epc", "min-ninety-day-epc", "page-token", "max-results", "min-payout-rank"].iter().map(|v|*v));
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
                CallType::Standard => call.doit(),
                _ => unreachable!()
            } {
                Err(api_err) => Err(DoitError::ApiError(api_err)),
                Ok((mut response, output_schema)) => {
                    let mut value = json::value::to_value(&output_schema);
                    remove_json_null_values(&mut value);
                    json::to_writer_pretty(&mut ostream, &value).unwrap();
                    ostream.flush().unwrap();
                    Ok(())
                }
            }
        }
    }

    fn _reports_get(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        let mut call = self.hub.reports().get(opt.value_of("role").unwrap_or(""), opt.value_of("role-id").unwrap_or(""), opt.value_of("report-type").unwrap_or(""));
        for parg in opt.values_of("v").map(|i|i.collect()).unwrap_or(Vec::new()).iter() {
            let (key, value) = parse_kv_arg(&*parg, err, false);
            match key {
                "status" => {
                    call = call.status(value.unwrap_or(""));
                },
                "start-index" => {
                    call = call.start_index(arg_from_str(value.unwrap_or("-0"), err, "start-index", "integer"));
                },
                "start-date" => {
                    call = call.start_date(value.unwrap_or(""));
                },
                "publisher-id" => {
                    call = call.add_publisher_id(value.unwrap_or(""));
                },
                "order-id" => {
                    call = call.add_order_id(value.unwrap_or(""));
                },
                "max-results" => {
                    call = call.max_results(arg_from_str(value.unwrap_or("-0"), err, "max-results", "integer"));
                },
                "link-id" => {
                    call = call.add_link_id(value.unwrap_or(""));
                },
                "event-type" => {
                    call = call.event_type(value.unwrap_or(""));
                },
                "end-date" => {
                    call = call.end_date(value.unwrap_or(""));
                },
                "calculate-totals" => {
                    call = call.calculate_totals(arg_from_str(value.unwrap_or("false"), err, "calculate-totals", "boolean"));
                },
                "advertiser-id" => {
                    call = call.add_advertiser_id(value.unwrap_or(""));
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
                                                                           v.extend(["status", "start-date", "end-date", "advertiser-id", "link-id", "max-results", "order-id", "start-index", "event-type", "calculate-totals", "publisher-id"].iter().map(|v|*v));
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
                CallType::Standard => call.doit(),
                _ => unreachable!()
            } {
                Err(api_err) => Err(DoitError::ApiError(api_err)),
                Ok((mut response, output_schema)) => {
                    let mut value = json::value::to_value(&output_schema);
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
            ("advertisers", Some(opt)) => {
                match opt.subcommand() {
                    ("get", Some(opt)) => {
                        call_result = self._advertisers_get(opt, dry_run, &mut err);
                    },
                    ("list", Some(opt)) => {
                        call_result = self._advertisers_list(opt, dry_run, &mut err);
                    },
                    _ => {
                        err.issues.push(CLIError::MissingMethodError("advertisers".to_string()));
                        writeln!(io::stderr(), "{}\n", opt.usage()).ok();
                    }
                }
            },
            ("cc-offers", Some(opt)) => {
                match opt.subcommand() {
                    ("list", Some(opt)) => {
                        call_result = self._cc_offers_list(opt, dry_run, &mut err);
                    },
                    _ => {
                        err.issues.push(CLIError::MissingMethodError("cc-offers".to_string()));
                        writeln!(io::stderr(), "{}\n", opt.usage()).ok();
                    }
                }
            },
            ("events", Some(opt)) => {
                match opt.subcommand() {
                    ("list", Some(opt)) => {
                        call_result = self._events_list(opt, dry_run, &mut err);
                    },
                    _ => {
                        err.issues.push(CLIError::MissingMethodError("events".to_string()));
                        writeln!(io::stderr(), "{}\n", opt.usage()).ok();
                    }
                }
            },
            ("links", Some(opt)) => {
                match opt.subcommand() {
                    ("get", Some(opt)) => {
                        call_result = self._links_get(opt, dry_run, &mut err);
                    },
                    ("insert", Some(opt)) => {
                        call_result = self._links_insert(opt, dry_run, &mut err);
                    },
                    ("list", Some(opt)) => {
                        call_result = self._links_list(opt, dry_run, &mut err);
                    },
                    _ => {
                        err.issues.push(CLIError::MissingMethodError("links".to_string()));
                        writeln!(io::stderr(), "{}\n", opt.usage()).ok();
                    }
                }
            },
            ("publishers", Some(opt)) => {
                match opt.subcommand() {
                    ("get", Some(opt)) => {
                        call_result = self._publishers_get(opt, dry_run, &mut err);
                    },
                    ("list", Some(opt)) => {
                        call_result = self._publishers_list(opt, dry_run, &mut err);
                    },
                    _ => {
                        err.issues.push(CLIError::MissingMethodError("publishers".to_string()));
                        writeln!(io::stderr(), "{}\n", opt.usage()).ok();
                    }
                }
            },
            ("reports", Some(opt)) => {
                match opt.subcommand() {
                    ("get", Some(opt)) => {
                        call_result = self._reports_get(opt, dry_run, &mut err);
                    },
                    _ => {
                        err.issues.push(CLIError::MissingMethodError("reports".to_string()));
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

            match cmn::application_secret_from_directory(&config_dir, "gan1-beta1-secret.json",
                                                         "{\"installed\":{\"auth_uri\":\"https://accounts.google.com/o/oauth2/auth\",\"client_secret\":\"hCsslbCUyfehWMmbkG8vTYxG\",\"token_uri\":\"https://accounts.google.com/o/oauth2/token\",\"client_email\":\"\",\"redirect_uris\":[\"urn:ietf:wg:oauth:2.0:oob\",\"oob\"],\"client_x509_cert_url\":\"\",\"client_id\":\"620010449518-9ngf7o4dhs0dka470npqvor6dc5lqb9b.apps.googleusercontent.com\",\"auth_provider_x509_cert_url\":\"https://www.googleapis.com/oauth2/v1/certs\"}}") {
                Ok(secret) => (config_dir, secret),
                Err(e) => return Err(InvalidOptionsError::single(e, 4))
            }
        };

        let auth = Authenticator::new(  &secret, DefaultAuthenticatorDelegate,
                                        if opt.is_present("debug-auth") {
                                            hyper::Client::with_connector(mock::TeeConnector {
                                                    connector: hyper::net::HttpsConnector::<hyper::net::Openssl>::default()
                                                })
                                        } else {
                                            hyper::Client::new()
                                        },
                                        JsonTokenStorage {
                                          program_name: "gan1-beta1",
                                          db_dir: config_dir.clone(),
                                        }, Some(FlowType::InstalledInteractive));

        let client =
            if opt.is_present("debug") {
                hyper::Client::with_connector(mock::TeeConnector {
                        connector: hyper::net::HttpsConnector::<hyper::net::Openssl>::default()
                    })
            } else {
                hyper::Client::new()
            };
        let engine = Engine {
            opt: opt,
            hub: api::Gan::new(client, auth),
            gp: vec!["alt", "fields", "key", "oauth-token", "pretty-print", "quota-user", "user-ip"],
            gpm: vec![
                    ("oauth-token", "oauth_token"),
                    ("pretty-print", "prettyPrint"),
                    ("quota-user", "quotaUser"),
                    ("user-ip", "userIp"),
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
        ("advertisers", "methods: 'get' and 'list'", vec![
            ("get",
                    Some(r##"Retrieves data about a single advertiser if that the requesting advertiser/publisher has access to it. Only publishers can lookup advertisers. Advertisers can request information about themselves by omitting the advertiserId query parameter."##),
                    "Details at http://byron.github.io/google-apis-rs/google_gan1_beta1_cli/advertisers_get",
                  vec![
                    (Some(r##"role"##),
                     None,
                     Some(r##"The role of the requester. Valid values: 'advertisers' or 'publishers'."##),
                     Some(true),
                     Some(false)),
        
                    (Some(r##"role-id"##),
                     None,
                     Some(r##"The ID of the requesting advertiser or publisher."##),
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
            ("list",
                    Some(r##"Retrieves data about all advertisers that the requesting advertiser/publisher has access to."##),
                    "Details at http://byron.github.io/google-apis-rs/google_gan1_beta1_cli/advertisers_list",
                  vec![
                    (Some(r##"role"##),
                     None,
                     Some(r##"The role of the requester. Valid values: 'advertisers' or 'publishers'."##),
                     Some(true),
                     Some(false)),
        
                    (Some(r##"role-id"##),
                     None,
                     Some(r##"The ID of the requesting advertiser or publisher."##),
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
        
        ("cc-offers", "methods: 'list'", vec![
            ("list",
                    Some(r##"Retrieves credit card offers for the given publisher."##),
                    "Details at http://byron.github.io/google-apis-rs/google_gan1_beta1_cli/cc-offers_list",
                  vec![
                    (Some(r##"publisher"##),
                     None,
                     Some(r##"The ID of the publisher in question."##),
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
        
        ("events", "methods: 'list'", vec![
            ("list",
                    Some(r##"Retrieves event data for a given advertiser/publisher."##),
                    "Details at http://byron.github.io/google-apis-rs/google_gan1_beta1_cli/events_list",
                  vec![
                    (Some(r##"role"##),
                     None,
                     Some(r##"The role of the requester. Valid values: 'advertisers' or 'publishers'."##),
                     Some(true),
                     Some(false)),
        
                    (Some(r##"role-id"##),
                     None,
                     Some(r##"The ID of the requesting advertiser or publisher."##),
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
        
        ("links", "methods: 'get', 'insert' and 'list'", vec![
            ("get",
                    Some(r##"Retrieves data about a single link if the requesting advertiser/publisher has access to it. Advertisers can look up their own links. Publishers can look up visible links or links belonging to advertisers they are in a relationship with."##),
                    "Details at http://byron.github.io/google-apis-rs/google_gan1_beta1_cli/links_get",
                  vec![
                    (Some(r##"role"##),
                     None,
                     Some(r##"The role of the requester. Valid values: 'advertisers' or 'publishers'."##),
                     Some(true),
                     Some(false)),
        
                    (Some(r##"role-id"##),
                     None,
                     Some(r##"The ID of the requesting advertiser or publisher."##),
                     Some(true),
                     Some(false)),
        
                    (Some(r##"link-id"##),
                     None,
                     Some(r##"The ID of the link to look up."##),
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
            ("insert",
                    Some(r##"Inserts a new link."##),
                    "Details at http://byron.github.io/google-apis-rs/google_gan1_beta1_cli/links_insert",
                  vec![
                    (Some(r##"role"##),
                     None,
                     Some(r##"The role of the requester. Valid values: 'advertisers' or 'publishers'."##),
                     Some(true),
                     Some(false)),
        
                    (Some(r##"role-id"##),
                     None,
                     Some(r##"The ID of the requesting advertiser or publisher."##),
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
            ("list",
                    Some(r##"Retrieves all links that match the query parameters."##),
                    "Details at http://byron.github.io/google-apis-rs/google_gan1_beta1_cli/links_list",
                  vec![
                    (Some(r##"role"##),
                     None,
                     Some(r##"The role of the requester. Valid values: 'advertisers' or 'publishers'."##),
                     Some(true),
                     Some(false)),
        
                    (Some(r##"role-id"##),
                     None,
                     Some(r##"The ID of the requesting advertiser or publisher."##),
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
        
        ("publishers", "methods: 'get' and 'list'", vec![
            ("get",
                    Some(r##"Retrieves data about a single advertiser if that the requesting advertiser/publisher has access to it. Only advertisers can look up publishers. Publishers can request information about themselves by omitting the publisherId query parameter."##),
                    "Details at http://byron.github.io/google-apis-rs/google_gan1_beta1_cli/publishers_get",
                  vec![
                    (Some(r##"role"##),
                     None,
                     Some(r##"The role of the requester. Valid values: 'advertisers' or 'publishers'."##),
                     Some(true),
                     Some(false)),
        
                    (Some(r##"role-id"##),
                     None,
                     Some(r##"The ID of the requesting advertiser or publisher."##),
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
            ("list",
                    Some(r##"Retrieves data about all publishers that the requesting advertiser/publisher has access to."##),
                    "Details at http://byron.github.io/google-apis-rs/google_gan1_beta1_cli/publishers_list",
                  vec![
                    (Some(r##"role"##),
                     None,
                     Some(r##"The role of the requester. Valid values: 'advertisers' or 'publishers'."##),
                     Some(true),
                     Some(false)),
        
                    (Some(r##"role-id"##),
                     None,
                     Some(r##"The ID of the requesting advertiser or publisher."##),
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
        
        ("reports", "methods: 'get'", vec![
            ("get",
                    Some(r##"Retrieves a report of the specified type."##),
                    "Details at http://byron.github.io/google-apis-rs/google_gan1_beta1_cli/reports_get",
                  vec![
                    (Some(r##"role"##),
                     None,
                     Some(r##"The role of the requester. Valid values: 'advertisers' or 'publishers'."##),
                     Some(true),
                     Some(false)),
        
                    (Some(r##"role-id"##),
                     None,
                     Some(r##"The ID of the requesting advertiser or publisher."##),
                     Some(true),
                     Some(false)),
        
                    (Some(r##"report-type"##),
                     None,
                     Some(r##"The type of report being requested. Valid values: 'order_delta'. Required."##),
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
    
    let mut app = App::new("gan1-beta1")
           .author("Sebastian Thiel <byronimo@gmail.com>")
           .version("0.3.4+20130205")
           .about("Lets you have programmatic access to your Google Affiliate Network data.")
           .after_help("All documentation details can be found at http://byron.github.io/google-apis-rs/google_gan1_beta1_cli")
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