// DO NOT EDIT !
// This file was generated automatically from 'src/generator/templates/cli/main.rs.mako'
// DO NOT EDIT !
#![allow(unused_variables, unused_imports, dead_code, unused_mut)]

#[macro_use]
extern crate clap;

use std::env;
use std::io::{self, Write};
use clap::{App, SubCommand, Arg};

use google_partners2::{api, Error, oauth2, client::chrono, FieldMask};


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
    hub: api::Partners<S>,
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
    async fn _analytics_list(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        let mut call = self.hub.analytics().list();
        for parg in opt.values_of("v").map(|i|i.collect()).unwrap_or(Vec::new()).iter() {
            let (key, value) = parse_kv_arg(&*parg, err, false);
            match key {
                "request-metadata-user-overrides-user-id" => {
                    call = call.request_metadata_user_overrides_user_id(value.unwrap_or(""));
                },
                "request-metadata-user-overrides-ip-address" => {
                    call = call.request_metadata_user_overrides_ip_address(value.unwrap_or(""));
                },
                "request-metadata-traffic-source-traffic-sub-id" => {
                    call = call.request_metadata_traffic_source_traffic_sub_id(value.unwrap_or(""));
                },
                "request-metadata-traffic-source-traffic-source-id" => {
                    call = call.request_metadata_traffic_source_traffic_source_id(value.unwrap_or(""));
                },
                "request-metadata-partners-session-id" => {
                    call = call.request_metadata_partners_session_id(value.unwrap_or(""));
                },
                "request-metadata-locale" => {
                    call = call.request_metadata_locale(value.unwrap_or(""));
                },
                "request-metadata-experiment-ids" => {
                    call = call.add_request_metadata_experiment_ids(value.unwrap_or(""));
                },
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
                                                                           v.extend(["page-size", "page-token", "request-metadata-experiment-ids", "request-metadata-locale", "request-metadata-partners-session-id", "request-metadata-traffic-source-traffic-source-id", "request-metadata-traffic-source-traffic-sub-id", "request-metadata-user-overrides-ip-address", "request-metadata-user-overrides-user-id"].iter().map(|v|*v));
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

    async fn _client_messages_log(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
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
                    "client-info" => Some(("clientInfo", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Map })),
                    "request-metadata.locale" => Some(("requestMetadata.locale", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "request-metadata.user-overrides.ip-address" => Some(("requestMetadata.userOverrides.ipAddress", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "request-metadata.user-overrides.user-id" => Some(("requestMetadata.userOverrides.userId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "request-metadata.partners-session-id" => Some(("requestMetadata.partnersSessionId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "request-metadata.experiment-ids" => Some(("requestMetadata.experimentIds", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Vec })),
                    "request-metadata.traffic-source.traffic-source-id" => Some(("requestMetadata.trafficSource.trafficSourceId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "request-metadata.traffic-source.traffic-sub-id" => Some(("requestMetadata.trafficSource.trafficSubId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "level" => Some(("level", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "details" => Some(("details", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    _ => {
                        let suggestion = FieldCursor::did_you_mean(key, &vec!["client-info", "details", "experiment-ids", "ip-address", "level", "locale", "partners-session-id", "request-metadata", "traffic-source", "traffic-source-id", "traffic-sub-id", "user-id", "user-overrides"]);
                        err.issues.push(CLIError::Field(FieldError::Unknown(temp_cursor.to_string(), suggestion, value.map(|v| v.to_string()))));
                        None
                    }
                };
            if let Some((field_cursor_str, type_info)) = type_info {
                FieldCursor::from(field_cursor_str).set_json_value(&mut object, value.unwrap(), type_info, err, &temp_cursor);
            }
        }
        let mut request: api::LogMessageRequest = json::value::from_value(object).unwrap();
        let mut call = self.hub.client_messages().log(request);
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

    async fn _companies_get(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        let mut call = self.hub.companies().get(opt.value_of("company-id").unwrap_or(""));
        for parg in opt.values_of("v").map(|i|i.collect()).unwrap_or(Vec::new()).iter() {
            let (key, value) = parse_kv_arg(&*parg, err, false);
            match key {
                "view" => {
                    call = call.view(value.unwrap_or(""));
                },
                "request-metadata-user-overrides-user-id" => {
                    call = call.request_metadata_user_overrides_user_id(value.unwrap_or(""));
                },
                "request-metadata-user-overrides-ip-address" => {
                    call = call.request_metadata_user_overrides_ip_address(value.unwrap_or(""));
                },
                "request-metadata-traffic-source-traffic-sub-id" => {
                    call = call.request_metadata_traffic_source_traffic_sub_id(value.unwrap_or(""));
                },
                "request-metadata-traffic-source-traffic-source-id" => {
                    call = call.request_metadata_traffic_source_traffic_source_id(value.unwrap_or(""));
                },
                "request-metadata-partners-session-id" => {
                    call = call.request_metadata_partners_session_id(value.unwrap_or(""));
                },
                "request-metadata-locale" => {
                    call = call.request_metadata_locale(value.unwrap_or(""));
                },
                "request-metadata-experiment-ids" => {
                    call = call.add_request_metadata_experiment_ids(value.unwrap_or(""));
                },
                "order-by" => {
                    call = call.order_by(value.unwrap_or(""));
                },
                "currency-code" => {
                    call = call.currency_code(value.unwrap_or(""));
                },
                "address" => {
                    call = call.address(value.unwrap_or(""));
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
                                                                           v.extend(["address", "currency-code", "order-by", "request-metadata-experiment-ids", "request-metadata-locale", "request-metadata-partners-session-id", "request-metadata-traffic-source-traffic-source-id", "request-metadata-traffic-source-traffic-sub-id", "request-metadata-user-overrides-ip-address", "request-metadata-user-overrides-user-id", "view"].iter().map(|v|*v));
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

    async fn _companies_leads_create(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
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
                    "request-metadata.locale" => Some(("requestMetadata.locale", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "request-metadata.user-overrides.ip-address" => Some(("requestMetadata.userOverrides.ipAddress", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "request-metadata.user-overrides.user-id" => Some(("requestMetadata.userOverrides.userId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "request-metadata.partners-session-id" => Some(("requestMetadata.partnersSessionId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "request-metadata.experiment-ids" => Some(("requestMetadata.experimentIds", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Vec })),
                    "request-metadata.traffic-source.traffic-source-id" => Some(("requestMetadata.trafficSource.trafficSourceId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "request-metadata.traffic-source.traffic-sub-id" => Some(("requestMetadata.trafficSource.trafficSubId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "lead.min-monthly-budget.currency-code" => Some(("lead.minMonthlyBudget.currencyCode", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "lead.min-monthly-budget.nanos" => Some(("lead.minMonthlyBudget.nanos", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "lead.min-monthly-budget.units" => Some(("lead.minMonthlyBudget.units", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "lead.given-name" => Some(("lead.givenName", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "lead.language-code" => Some(("lead.languageCode", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "lead.website-url" => Some(("lead.websiteUrl", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "lead.state" => Some(("lead.state", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "lead.gps-motivations" => Some(("lead.gpsMotivations", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Vec })),
                    "lead.email" => Some(("lead.email", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "lead.family-name" => Some(("lead.familyName", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "lead.id" => Some(("lead.id", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "lead.comments" => Some(("lead.comments", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "lead.phone-number" => Some(("lead.phoneNumber", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "lead.adwords-customer-id" => Some(("lead.adwordsCustomerId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "lead.create-time" => Some(("lead.createTime", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "lead.marketing-opt-in" => Some(("lead.marketingOptIn", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "lead.type" => Some(("lead.type", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "recaptcha-challenge.id" => Some(("recaptchaChallenge.id", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "recaptcha-challenge.response" => Some(("recaptchaChallenge.response", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    _ => {
                        let suggestion = FieldCursor::did_you_mean(key, &vec!["adwords-customer-id", "comments", "create-time", "currency-code", "email", "experiment-ids", "family-name", "given-name", "gps-motivations", "id", "ip-address", "language-code", "lead", "locale", "marketing-opt-in", "min-monthly-budget", "nanos", "partners-session-id", "phone-number", "recaptcha-challenge", "request-metadata", "response", "state", "traffic-source", "traffic-source-id", "traffic-sub-id", "type", "units", "user-id", "user-overrides", "website-url"]);
                        err.issues.push(CLIError::Field(FieldError::Unknown(temp_cursor.to_string(), suggestion, value.map(|v| v.to_string()))));
                        None
                    }
                };
            if let Some((field_cursor_str, type_info)) = type_info {
                FieldCursor::from(field_cursor_str).set_json_value(&mut object, value.unwrap(), type_info, err, &temp_cursor);
            }
        }
        let mut request: api::CreateLeadRequest = json::value::from_value(object).unwrap();
        let mut call = self.hub.companies().leads_create(request, opt.value_of("company-id").unwrap_or(""));
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

    async fn _companies_list(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        let mut call = self.hub.companies().list();
        for parg in opt.values_of("v").map(|i|i.collect()).unwrap_or(Vec::new()).iter() {
            let (key, value) = parse_kv_arg(&*parg, err, false);
            match key {
                "website-url" => {
                    call = call.website_url(value.unwrap_or(""));
                },
                "view" => {
                    call = call.view(value.unwrap_or(""));
                },
                "specializations" => {
                    call = call.add_specializations(value.unwrap_or(""));
                },
                "services" => {
                    call = call.add_services(value.unwrap_or(""));
                },
                "request-metadata-user-overrides-user-id" => {
                    call = call.request_metadata_user_overrides_user_id(value.unwrap_or(""));
                },
                "request-metadata-user-overrides-ip-address" => {
                    call = call.request_metadata_user_overrides_ip_address(value.unwrap_or(""));
                },
                "request-metadata-traffic-source-traffic-sub-id" => {
                    call = call.request_metadata_traffic_source_traffic_sub_id(value.unwrap_or(""));
                },
                "request-metadata-traffic-source-traffic-source-id" => {
                    call = call.request_metadata_traffic_source_traffic_source_id(value.unwrap_or(""));
                },
                "request-metadata-partners-session-id" => {
                    call = call.request_metadata_partners_session_id(value.unwrap_or(""));
                },
                "request-metadata-locale" => {
                    call = call.request_metadata_locale(value.unwrap_or(""));
                },
                "request-metadata-experiment-ids" => {
                    call = call.add_request_metadata_experiment_ids(value.unwrap_or(""));
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
                "min-monthly-budget-units" => {
                    call = call.min_monthly_budget_units(        value.map(|v| arg_from_str(v, err, "min-monthly-budget-units", "int64")).unwrap_or(-0));
                },
                "min-monthly-budget-nanos" => {
                    call = call.min_monthly_budget_nanos(        value.map(|v| arg_from_str(v, err, "min-monthly-budget-nanos", "int32")).unwrap_or(-0));
                },
                "min-monthly-budget-currency-code" => {
                    call = call.min_monthly_budget_currency_code(value.unwrap_or(""));
                },
                "max-monthly-budget-units" => {
                    call = call.max_monthly_budget_units(        value.map(|v| arg_from_str(v, err, "max-monthly-budget-units", "int64")).unwrap_or(-0));
                },
                "max-monthly-budget-nanos" => {
                    call = call.max_monthly_budget_nanos(        value.map(|v| arg_from_str(v, err, "max-monthly-budget-nanos", "int32")).unwrap_or(-0));
                },
                "max-monthly-budget-currency-code" => {
                    call = call.max_monthly_budget_currency_code(value.unwrap_or(""));
                },
                "language-codes" => {
                    call = call.add_language_codes(value.unwrap_or(""));
                },
                "industries" => {
                    call = call.add_industries(value.unwrap_or(""));
                },
                "gps-motivations" => {
                    call = call.add_gps_motivations(value.unwrap_or(""));
                },
                "company-name" => {
                    call = call.company_name(value.unwrap_or(""));
                },
                "address" => {
                    call = call.address(value.unwrap_or(""));
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
                                                                           v.extend(["address", "company-name", "gps-motivations", "industries", "language-codes", "max-monthly-budget-currency-code", "max-monthly-budget-nanos", "max-monthly-budget-units", "min-monthly-budget-currency-code", "min-monthly-budget-nanos", "min-monthly-budget-units", "order-by", "page-size", "page-token", "request-metadata-experiment-ids", "request-metadata-locale", "request-metadata-partners-session-id", "request-metadata-traffic-source-traffic-source-id", "request-metadata-traffic-source-traffic-sub-id", "request-metadata-user-overrides-ip-address", "request-metadata-user-overrides-user-id", "services", "specializations", "view", "website-url"].iter().map(|v|*v));
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

    async fn _leads_list(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        let mut call = self.hub.leads().list();
        for parg in opt.values_of("v").map(|i|i.collect()).unwrap_or(Vec::new()).iter() {
            let (key, value) = parse_kv_arg(&*parg, err, false);
            match key {
                "request-metadata-user-overrides-user-id" => {
                    call = call.request_metadata_user_overrides_user_id(value.unwrap_or(""));
                },
                "request-metadata-user-overrides-ip-address" => {
                    call = call.request_metadata_user_overrides_ip_address(value.unwrap_or(""));
                },
                "request-metadata-traffic-source-traffic-sub-id" => {
                    call = call.request_metadata_traffic_source_traffic_sub_id(value.unwrap_or(""));
                },
                "request-metadata-traffic-source-traffic-source-id" => {
                    call = call.request_metadata_traffic_source_traffic_source_id(value.unwrap_or(""));
                },
                "request-metadata-partners-session-id" => {
                    call = call.request_metadata_partners_session_id(value.unwrap_or(""));
                },
                "request-metadata-locale" => {
                    call = call.request_metadata_locale(value.unwrap_or(""));
                },
                "request-metadata-experiment-ids" => {
                    call = call.add_request_metadata_experiment_ids(value.unwrap_or(""));
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
                                                                           v.extend(["order-by", "page-size", "page-token", "request-metadata-experiment-ids", "request-metadata-locale", "request-metadata-partners-session-id", "request-metadata-traffic-source-traffic-source-id", "request-metadata-traffic-source-traffic-sub-id", "request-metadata-user-overrides-ip-address", "request-metadata-user-overrides-user-id"].iter().map(|v|*v));
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

    async fn _methods_get_partnersstatus(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        let mut call = self.hub.methods().get_partnersstatus();
        for parg in opt.values_of("v").map(|i|i.collect()).unwrap_or(Vec::new()).iter() {
            let (key, value) = parse_kv_arg(&*parg, err, false);
            match key {
                "request-metadata-user-overrides-user-id" => {
                    call = call.request_metadata_user_overrides_user_id(value.unwrap_or(""));
                },
                "request-metadata-user-overrides-ip-address" => {
                    call = call.request_metadata_user_overrides_ip_address(value.unwrap_or(""));
                },
                "request-metadata-traffic-source-traffic-sub-id" => {
                    call = call.request_metadata_traffic_source_traffic_sub_id(value.unwrap_or(""));
                },
                "request-metadata-traffic-source-traffic-source-id" => {
                    call = call.request_metadata_traffic_source_traffic_source_id(value.unwrap_or(""));
                },
                "request-metadata-partners-session-id" => {
                    call = call.request_metadata_partners_session_id(value.unwrap_or(""));
                },
                "request-metadata-locale" => {
                    call = call.request_metadata_locale(value.unwrap_or(""));
                },
                "request-metadata-experiment-ids" => {
                    call = call.add_request_metadata_experiment_ids(value.unwrap_or(""));
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
                                                                           v.extend(["request-metadata-experiment-ids", "request-metadata-locale", "request-metadata-partners-session-id", "request-metadata-traffic-source-traffic-source-id", "request-metadata-traffic-source-traffic-sub-id", "request-metadata-user-overrides-ip-address", "request-metadata-user-overrides-user-id"].iter().map(|v|*v));
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

    async fn _methods_update_companies(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
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
                    "profile-status" => Some(("profileStatus", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "primary-language-code" => Some(("primaryLanguageCode", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "converted-min-monthly-budget.currency-code" => Some(("convertedMinMonthlyBudget.currencyCode", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "converted-min-monthly-budget.nanos" => Some(("convertedMinMonthlyBudget.nanos", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "converted-min-monthly-budget.units" => Some(("convertedMinMonthlyBudget.units", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "industries" => Some(("industries", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Vec })),
                    "website-url" => Some(("websiteUrl", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "additional-websites" => Some(("additionalWebsites", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Vec })),
                    "primary-adwords-manager-account-id" => Some(("primaryAdwordsManagerAccountId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "badge-authority-in-awn" => Some(("badgeAuthorityInAwn", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "name" => Some(("name", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "id" => Some(("id", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "public-profile.profile-image" => Some(("publicProfile.profileImage", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "public-profile.display-name" => Some(("publicProfile.displayName", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "public-profile.display-image-url" => Some(("publicProfile.displayImageUrl", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "public-profile.id" => Some(("publicProfile.id", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "public-profile.url" => Some(("publicProfile.url", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "original-min-monthly-budget.currency-code" => Some(("originalMinMonthlyBudget.currencyCode", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "original-min-monthly-budget.nanos" => Some(("originalMinMonthlyBudget.nanos", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "original-min-monthly-budget.units" => Some(("originalMinMonthlyBudget.units", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "services" => Some(("services", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Vec })),
                    "primary-location.administrative-area" => Some(("primaryLocation.administrativeArea", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "primary-location.locality" => Some(("primaryLocation.locality", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "primary-location.lat-lng.latitude" => Some(("primaryLocation.latLng.latitude", JsonTypeInfo { jtype: JsonType::Float, ctype: ComplexType::Pod })),
                    "primary-location.lat-lng.longitude" => Some(("primaryLocation.latLng.longitude", JsonTypeInfo { jtype: JsonType::Float, ctype: ComplexType::Pod })),
                    "primary-location.region-code" => Some(("primaryLocation.regionCode", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "primary-location.dependent-locality" => Some(("primaryLocation.dependentLocality", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "primary-location.address" => Some(("primaryLocation.address", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "primary-location.postal-code" => Some(("primaryLocation.postalCode", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "primary-location.sorting-code" => Some(("primaryLocation.sortingCode", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "primary-location.language-code" => Some(("primaryLocation.languageCode", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "primary-location.address-line" => Some(("primaryLocation.addressLine", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Vec })),
                    "badge-tier" => Some(("badgeTier", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "auto-approval-email-domains" => Some(("autoApprovalEmailDomains", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Vec })),
                    "company-types" => Some(("companyTypes", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Vec })),
                    _ => {
                        let suggestion = FieldCursor::did_you_mean(key, &vec!["additional-websites", "address", "address-line", "administrative-area", "auto-approval-email-domains", "badge-authority-in-awn", "badge-tier", "company-types", "converted-min-monthly-budget", "currency-code", "dependent-locality", "display-image-url", "display-name", "id", "industries", "language-code", "lat-lng", "latitude", "locality", "longitude", "name", "nanos", "original-min-monthly-budget", "postal-code", "primary-adwords-manager-account-id", "primary-language-code", "primary-location", "profile-image", "profile-status", "public-profile", "region-code", "services", "sorting-code", "units", "url", "website-url"]);
                        err.issues.push(CLIError::Field(FieldError::Unknown(temp_cursor.to_string(), suggestion, value.map(|v| v.to_string()))));
                        None
                    }
                };
            if let Some((field_cursor_str, type_info)) = type_info {
                FieldCursor::from(field_cursor_str).set_json_value(&mut object, value.unwrap(), type_info, err, &temp_cursor);
            }
        }
        let mut request: api::Company = json::value::from_value(object).unwrap();
        let mut call = self.hub.methods().update_companies(request);
        for parg in opt.values_of("v").map(|i|i.collect()).unwrap_or(Vec::new()).iter() {
            let (key, value) = parse_kv_arg(&*parg, err, false);
            match key {
                "update-mask" => {
                    call = call.update_mask(        value.map(|v| arg_from_str(v, err, "update-mask", "google-fieldmask")).unwrap_or(FieldMask::default()));
                },
                "request-metadata-user-overrides-user-id" => {
                    call = call.request_metadata_user_overrides_user_id(value.unwrap_or(""));
                },
                "request-metadata-user-overrides-ip-address" => {
                    call = call.request_metadata_user_overrides_ip_address(value.unwrap_or(""));
                },
                "request-metadata-traffic-source-traffic-sub-id" => {
                    call = call.request_metadata_traffic_source_traffic_sub_id(value.unwrap_or(""));
                },
                "request-metadata-traffic-source-traffic-source-id" => {
                    call = call.request_metadata_traffic_source_traffic_source_id(value.unwrap_or(""));
                },
                "request-metadata-partners-session-id" => {
                    call = call.request_metadata_partners_session_id(value.unwrap_or(""));
                },
                "request-metadata-locale" => {
                    call = call.request_metadata_locale(value.unwrap_or(""));
                },
                "request-metadata-experiment-ids" => {
                    call = call.add_request_metadata_experiment_ids(value.unwrap_or(""));
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
                                                                           v.extend(["request-metadata-experiment-ids", "request-metadata-locale", "request-metadata-partners-session-id", "request-metadata-traffic-source-traffic-source-id", "request-metadata-traffic-source-traffic-sub-id", "request-metadata-user-overrides-ip-address", "request-metadata-user-overrides-user-id", "update-mask"].iter().map(|v|*v));
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

    async fn _methods_update_leads(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
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
                    "min-monthly-budget.currency-code" => Some(("minMonthlyBudget.currencyCode", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "min-monthly-budget.nanos" => Some(("minMonthlyBudget.nanos", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "min-monthly-budget.units" => Some(("minMonthlyBudget.units", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "given-name" => Some(("givenName", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "language-code" => Some(("languageCode", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "website-url" => Some(("websiteUrl", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "state" => Some(("state", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "gps-motivations" => Some(("gpsMotivations", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Vec })),
                    "email" => Some(("email", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "family-name" => Some(("familyName", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "id" => Some(("id", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "comments" => Some(("comments", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "phone-number" => Some(("phoneNumber", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "adwords-customer-id" => Some(("adwordsCustomerId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "create-time" => Some(("createTime", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "marketing-opt-in" => Some(("marketingOptIn", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "type" => Some(("type", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    _ => {
                        let suggestion = FieldCursor::did_you_mean(key, &vec!["adwords-customer-id", "comments", "create-time", "currency-code", "email", "family-name", "given-name", "gps-motivations", "id", "language-code", "marketing-opt-in", "min-monthly-budget", "nanos", "phone-number", "state", "type", "units", "website-url"]);
                        err.issues.push(CLIError::Field(FieldError::Unknown(temp_cursor.to_string(), suggestion, value.map(|v| v.to_string()))));
                        None
                    }
                };
            if let Some((field_cursor_str, type_info)) = type_info {
                FieldCursor::from(field_cursor_str).set_json_value(&mut object, value.unwrap(), type_info, err, &temp_cursor);
            }
        }
        let mut request: api::Lead = json::value::from_value(object).unwrap();
        let mut call = self.hub.methods().update_leads(request);
        for parg in opt.values_of("v").map(|i|i.collect()).unwrap_or(Vec::new()).iter() {
            let (key, value) = parse_kv_arg(&*parg, err, false);
            match key {
                "update-mask" => {
                    call = call.update_mask(        value.map(|v| arg_from_str(v, err, "update-mask", "google-fieldmask")).unwrap_or(FieldMask::default()));
                },
                "request-metadata-user-overrides-user-id" => {
                    call = call.request_metadata_user_overrides_user_id(value.unwrap_or(""));
                },
                "request-metadata-user-overrides-ip-address" => {
                    call = call.request_metadata_user_overrides_ip_address(value.unwrap_or(""));
                },
                "request-metadata-traffic-source-traffic-sub-id" => {
                    call = call.request_metadata_traffic_source_traffic_sub_id(value.unwrap_or(""));
                },
                "request-metadata-traffic-source-traffic-source-id" => {
                    call = call.request_metadata_traffic_source_traffic_source_id(value.unwrap_or(""));
                },
                "request-metadata-partners-session-id" => {
                    call = call.request_metadata_partners_session_id(value.unwrap_or(""));
                },
                "request-metadata-locale" => {
                    call = call.request_metadata_locale(value.unwrap_or(""));
                },
                "request-metadata-experiment-ids" => {
                    call = call.add_request_metadata_experiment_ids(value.unwrap_or(""));
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
                                                                           v.extend(["request-metadata-experiment-ids", "request-metadata-locale", "request-metadata-partners-session-id", "request-metadata-traffic-source-traffic-source-id", "request-metadata-traffic-source-traffic-sub-id", "request-metadata-user-overrides-ip-address", "request-metadata-user-overrides-user-id", "update-mask"].iter().map(|v|*v));
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

    async fn _offers_history_list(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        let mut call = self.hub.offers().history_list();
        for parg in opt.values_of("v").map(|i|i.collect()).unwrap_or(Vec::new()).iter() {
            let (key, value) = parse_kv_arg(&*parg, err, false);
            match key {
                "request-metadata-user-overrides-user-id" => {
                    call = call.request_metadata_user_overrides_user_id(value.unwrap_or(""));
                },
                "request-metadata-user-overrides-ip-address" => {
                    call = call.request_metadata_user_overrides_ip_address(value.unwrap_or(""));
                },
                "request-metadata-traffic-source-traffic-sub-id" => {
                    call = call.request_metadata_traffic_source_traffic_sub_id(value.unwrap_or(""));
                },
                "request-metadata-traffic-source-traffic-source-id" => {
                    call = call.request_metadata_traffic_source_traffic_source_id(value.unwrap_or(""));
                },
                "request-metadata-partners-session-id" => {
                    call = call.request_metadata_partners_session_id(value.unwrap_or(""));
                },
                "request-metadata-locale" => {
                    call = call.request_metadata_locale(value.unwrap_or(""));
                },
                "request-metadata-experiment-ids" => {
                    call = call.add_request_metadata_experiment_ids(value.unwrap_or(""));
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
                "entire-company" => {
                    call = call.entire_company(        value.map(|v| arg_from_str(v, err, "entire-company", "boolean")).unwrap_or(false));
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
                                                                           v.extend(["entire-company", "order-by", "page-size", "page-token", "request-metadata-experiment-ids", "request-metadata-locale", "request-metadata-partners-session-id", "request-metadata-traffic-source-traffic-source-id", "request-metadata-traffic-source-traffic-sub-id", "request-metadata-user-overrides-ip-address", "request-metadata-user-overrides-user-id"].iter().map(|v|*v));
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

    async fn _offers_list(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        let mut call = self.hub.offers().list();
        for parg in opt.values_of("v").map(|i|i.collect()).unwrap_or(Vec::new()).iter() {
            let (key, value) = parse_kv_arg(&*parg, err, false);
            match key {
                "request-metadata-user-overrides-user-id" => {
                    call = call.request_metadata_user_overrides_user_id(value.unwrap_or(""));
                },
                "request-metadata-user-overrides-ip-address" => {
                    call = call.request_metadata_user_overrides_ip_address(value.unwrap_or(""));
                },
                "request-metadata-traffic-source-traffic-sub-id" => {
                    call = call.request_metadata_traffic_source_traffic_sub_id(value.unwrap_or(""));
                },
                "request-metadata-traffic-source-traffic-source-id" => {
                    call = call.request_metadata_traffic_source_traffic_source_id(value.unwrap_or(""));
                },
                "request-metadata-partners-session-id" => {
                    call = call.request_metadata_partners_session_id(value.unwrap_or(""));
                },
                "request-metadata-locale" => {
                    call = call.request_metadata_locale(value.unwrap_or(""));
                },
                "request-metadata-experiment-ids" => {
                    call = call.add_request_metadata_experiment_ids(value.unwrap_or(""));
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
                                                                           v.extend(["request-metadata-experiment-ids", "request-metadata-locale", "request-metadata-partners-session-id", "request-metadata-traffic-source-traffic-source-id", "request-metadata-traffic-source-traffic-sub-id", "request-metadata-user-overrides-ip-address", "request-metadata-user-overrides-user-id"].iter().map(|v|*v));
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

    async fn _user_events_log(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
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
                    "url" => Some(("url", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "request-metadata.locale" => Some(("requestMetadata.locale", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "request-metadata.user-overrides.ip-address" => Some(("requestMetadata.userOverrides.ipAddress", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "request-metadata.user-overrides.user-id" => Some(("requestMetadata.userOverrides.userId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "request-metadata.partners-session-id" => Some(("requestMetadata.partnersSessionId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "request-metadata.experiment-ids" => Some(("requestMetadata.experimentIds", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Vec })),
                    "request-metadata.traffic-source.traffic-source-id" => Some(("requestMetadata.trafficSource.trafficSourceId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "request-metadata.traffic-source.traffic-sub-id" => Some(("requestMetadata.trafficSource.trafficSubId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "event-scope" => Some(("eventScope", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "event-category" => Some(("eventCategory", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "lead.min-monthly-budget.currency-code" => Some(("lead.minMonthlyBudget.currencyCode", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "lead.min-monthly-budget.nanos" => Some(("lead.minMonthlyBudget.nanos", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "lead.min-monthly-budget.units" => Some(("lead.minMonthlyBudget.units", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "lead.given-name" => Some(("lead.givenName", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "lead.language-code" => Some(("lead.languageCode", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "lead.website-url" => Some(("lead.websiteUrl", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "lead.state" => Some(("lead.state", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "lead.gps-motivations" => Some(("lead.gpsMotivations", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Vec })),
                    "lead.email" => Some(("lead.email", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "lead.family-name" => Some(("lead.familyName", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "lead.id" => Some(("lead.id", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "lead.comments" => Some(("lead.comments", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "lead.phone-number" => Some(("lead.phoneNumber", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "lead.adwords-customer-id" => Some(("lead.adwordsCustomerId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "lead.create-time" => Some(("lead.createTime", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "lead.marketing-opt-in" => Some(("lead.marketingOptIn", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "lead.type" => Some(("lead.type", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "event-action" => Some(("eventAction", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    _ => {
                        let suggestion = FieldCursor::did_you_mean(key, &vec!["adwords-customer-id", "comments", "create-time", "currency-code", "email", "event-action", "event-category", "event-scope", "experiment-ids", "family-name", "given-name", "gps-motivations", "id", "ip-address", "language-code", "lead", "locale", "marketing-opt-in", "min-monthly-budget", "nanos", "partners-session-id", "phone-number", "request-metadata", "state", "traffic-source", "traffic-source-id", "traffic-sub-id", "type", "units", "url", "user-id", "user-overrides", "website-url"]);
                        err.issues.push(CLIError::Field(FieldError::Unknown(temp_cursor.to_string(), suggestion, value.map(|v| v.to_string()))));
                        None
                    }
                };
            if let Some((field_cursor_str, type_info)) = type_info {
                FieldCursor::from(field_cursor_str).set_json_value(&mut object, value.unwrap(), type_info, err, &temp_cursor);
            }
        }
        let mut request: api::LogUserEventRequest = json::value::from_value(object).unwrap();
        let mut call = self.hub.user_events().log(request);
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

    async fn _user_states_list(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        let mut call = self.hub.user_states().list();
        for parg in opt.values_of("v").map(|i|i.collect()).unwrap_or(Vec::new()).iter() {
            let (key, value) = parse_kv_arg(&*parg, err, false);
            match key {
                "request-metadata-user-overrides-user-id" => {
                    call = call.request_metadata_user_overrides_user_id(value.unwrap_or(""));
                },
                "request-metadata-user-overrides-ip-address" => {
                    call = call.request_metadata_user_overrides_ip_address(value.unwrap_or(""));
                },
                "request-metadata-traffic-source-traffic-sub-id" => {
                    call = call.request_metadata_traffic_source_traffic_sub_id(value.unwrap_or(""));
                },
                "request-metadata-traffic-source-traffic-source-id" => {
                    call = call.request_metadata_traffic_source_traffic_source_id(value.unwrap_or(""));
                },
                "request-metadata-partners-session-id" => {
                    call = call.request_metadata_partners_session_id(value.unwrap_or(""));
                },
                "request-metadata-locale" => {
                    call = call.request_metadata_locale(value.unwrap_or(""));
                },
                "request-metadata-experiment-ids" => {
                    call = call.add_request_metadata_experiment_ids(value.unwrap_or(""));
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
                                                                           v.extend(["request-metadata-experiment-ids", "request-metadata-locale", "request-metadata-partners-session-id", "request-metadata-traffic-source-traffic-source-id", "request-metadata-traffic-source-traffic-sub-id", "request-metadata-user-overrides-ip-address", "request-metadata-user-overrides-user-id"].iter().map(|v|*v));
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

    async fn _users_create_company_relation(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
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
                    "company-admin" => Some(("companyAdmin", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "address" => Some(("address", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "is-pending" => Some(("isPending", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "creation-time" => Some(("creationTime", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "primary-address.administrative-area" => Some(("primaryAddress.administrativeArea", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "primary-address.locality" => Some(("primaryAddress.locality", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "primary-address.lat-lng.latitude" => Some(("primaryAddress.latLng.latitude", JsonTypeInfo { jtype: JsonType::Float, ctype: ComplexType::Pod })),
                    "primary-address.lat-lng.longitude" => Some(("primaryAddress.latLng.longitude", JsonTypeInfo { jtype: JsonType::Float, ctype: ComplexType::Pod })),
                    "primary-address.region-code" => Some(("primaryAddress.regionCode", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "primary-address.dependent-locality" => Some(("primaryAddress.dependentLocality", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "primary-address.address" => Some(("primaryAddress.address", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "primary-address.postal-code" => Some(("primaryAddress.postalCode", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "primary-address.sorting-code" => Some(("primaryAddress.sortingCode", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "primary-address.language-code" => Some(("primaryAddress.languageCode", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "primary-address.address-line" => Some(("primaryAddress.addressLine", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Vec })),
                    "state" => Some(("state", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "name" => Some(("name", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "manager-account" => Some(("managerAccount", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "segment" => Some(("segment", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Vec })),
                    "internal-company-id" => Some(("internalCompanyId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "badge-tier" => Some(("badgeTier", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "phone-number" => Some(("phoneNumber", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "website" => Some(("website", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "primary-country-code" => Some(("primaryCountryCode", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "company-id" => Some(("companyId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "primary-language-code" => Some(("primaryLanguageCode", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "logo-url" => Some(("logoUrl", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "resolved-timestamp" => Some(("resolvedTimestamp", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    _ => {
                        let suggestion = FieldCursor::did_you_mean(key, &vec!["address", "address-line", "administrative-area", "badge-tier", "company-admin", "company-id", "creation-time", "dependent-locality", "internal-company-id", "is-pending", "language-code", "lat-lng", "latitude", "locality", "logo-url", "longitude", "manager-account", "name", "phone-number", "postal-code", "primary-address", "primary-country-code", "primary-language-code", "region-code", "resolved-timestamp", "segment", "sorting-code", "state", "website"]);
                        err.issues.push(CLIError::Field(FieldError::Unknown(temp_cursor.to_string(), suggestion, value.map(|v| v.to_string()))));
                        None
                    }
                };
            if let Some((field_cursor_str, type_info)) = type_info {
                FieldCursor::from(field_cursor_str).set_json_value(&mut object, value.unwrap(), type_info, err, &temp_cursor);
            }
        }
        let mut request: api::CompanyRelation = json::value::from_value(object).unwrap();
        let mut call = self.hub.users().create_company_relation(request, opt.value_of("user-id").unwrap_or(""));
        for parg in opt.values_of("v").map(|i|i.collect()).unwrap_or(Vec::new()).iter() {
            let (key, value) = parse_kv_arg(&*parg, err, false);
            match key {
                "request-metadata-user-overrides-user-id" => {
                    call = call.request_metadata_user_overrides_user_id(value.unwrap_or(""));
                },
                "request-metadata-user-overrides-ip-address" => {
                    call = call.request_metadata_user_overrides_ip_address(value.unwrap_or(""));
                },
                "request-metadata-traffic-source-traffic-sub-id" => {
                    call = call.request_metadata_traffic_source_traffic_sub_id(value.unwrap_or(""));
                },
                "request-metadata-traffic-source-traffic-source-id" => {
                    call = call.request_metadata_traffic_source_traffic_source_id(value.unwrap_or(""));
                },
                "request-metadata-partners-session-id" => {
                    call = call.request_metadata_partners_session_id(value.unwrap_or(""));
                },
                "request-metadata-locale" => {
                    call = call.request_metadata_locale(value.unwrap_or(""));
                },
                "request-metadata-experiment-ids" => {
                    call = call.add_request_metadata_experiment_ids(value.unwrap_or(""));
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
                                                                           v.extend(["request-metadata-experiment-ids", "request-metadata-locale", "request-metadata-partners-session-id", "request-metadata-traffic-source-traffic-source-id", "request-metadata-traffic-source-traffic-sub-id", "request-metadata-user-overrides-ip-address", "request-metadata-user-overrides-user-id"].iter().map(|v|*v));
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

    async fn _users_delete_company_relation(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        let mut call = self.hub.users().delete_company_relation(opt.value_of("user-id").unwrap_or(""));
        for parg in opt.values_of("v").map(|i|i.collect()).unwrap_or(Vec::new()).iter() {
            let (key, value) = parse_kv_arg(&*parg, err, false);
            match key {
                "request-metadata-user-overrides-user-id" => {
                    call = call.request_metadata_user_overrides_user_id(value.unwrap_or(""));
                },
                "request-metadata-user-overrides-ip-address" => {
                    call = call.request_metadata_user_overrides_ip_address(value.unwrap_or(""));
                },
                "request-metadata-traffic-source-traffic-sub-id" => {
                    call = call.request_metadata_traffic_source_traffic_sub_id(value.unwrap_or(""));
                },
                "request-metadata-traffic-source-traffic-source-id" => {
                    call = call.request_metadata_traffic_source_traffic_source_id(value.unwrap_or(""));
                },
                "request-metadata-partners-session-id" => {
                    call = call.request_metadata_partners_session_id(value.unwrap_or(""));
                },
                "request-metadata-locale" => {
                    call = call.request_metadata_locale(value.unwrap_or(""));
                },
                "request-metadata-experiment-ids" => {
                    call = call.add_request_metadata_experiment_ids(value.unwrap_or(""));
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
                                                                           v.extend(["request-metadata-experiment-ids", "request-metadata-locale", "request-metadata-partners-session-id", "request-metadata-traffic-source-traffic-source-id", "request-metadata-traffic-source-traffic-sub-id", "request-metadata-user-overrides-ip-address", "request-metadata-user-overrides-user-id"].iter().map(|v|*v));
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

    async fn _users_get(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        let mut call = self.hub.users().get(opt.value_of("user-id").unwrap_or(""));
        for parg in opt.values_of("v").map(|i|i.collect()).unwrap_or(Vec::new()).iter() {
            let (key, value) = parse_kv_arg(&*parg, err, false);
            match key {
                "user-view" => {
                    call = call.user_view(value.unwrap_or(""));
                },
                "request-metadata-user-overrides-user-id" => {
                    call = call.request_metadata_user_overrides_user_id(value.unwrap_or(""));
                },
                "request-metadata-user-overrides-ip-address" => {
                    call = call.request_metadata_user_overrides_ip_address(value.unwrap_or(""));
                },
                "request-metadata-traffic-source-traffic-sub-id" => {
                    call = call.request_metadata_traffic_source_traffic_sub_id(value.unwrap_or(""));
                },
                "request-metadata-traffic-source-traffic-source-id" => {
                    call = call.request_metadata_traffic_source_traffic_source_id(value.unwrap_or(""));
                },
                "request-metadata-partners-session-id" => {
                    call = call.request_metadata_partners_session_id(value.unwrap_or(""));
                },
                "request-metadata-locale" => {
                    call = call.request_metadata_locale(value.unwrap_or(""));
                },
                "request-metadata-experiment-ids" => {
                    call = call.add_request_metadata_experiment_ids(value.unwrap_or(""));
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
                                                                           v.extend(["request-metadata-experiment-ids", "request-metadata-locale", "request-metadata-partners-session-id", "request-metadata-traffic-source-traffic-source-id", "request-metadata-traffic-source-traffic-sub-id", "request-metadata-user-overrides-ip-address", "request-metadata-user-overrides-user-id", "user-view"].iter().map(|v|*v));
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

    async fn _users_update_profile(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
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
                    "channels" => Some(("channels", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Vec })),
                    "profile-public" => Some(("profilePublic", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "job-functions" => Some(("jobFunctions", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Vec })),
                    "given-name" => Some(("givenName", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "address.administrative-area" => Some(("address.administrativeArea", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "address.locality" => Some(("address.locality", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "address.lat-lng.latitude" => Some(("address.latLng.latitude", JsonTypeInfo { jtype: JsonType::Float, ctype: ComplexType::Pod })),
                    "address.lat-lng.longitude" => Some(("address.latLng.longitude", JsonTypeInfo { jtype: JsonType::Float, ctype: ComplexType::Pod })),
                    "address.region-code" => Some(("address.regionCode", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "address.dependent-locality" => Some(("address.dependentLocality", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "address.address" => Some(("address.address", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "address.postal-code" => Some(("address.postalCode", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "address.sorting-code" => Some(("address.sortingCode", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "address.language-code" => Some(("address.languageCode", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "address.address-line" => Some(("address.addressLine", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Vec })),
                    "industries" => Some(("industries", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Vec })),
                    "email-opt-ins.market-comm" => Some(("emailOptIns.marketComm", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "email-opt-ins.special-offers" => Some(("emailOptIns.specialOffers", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "email-opt-ins.performance-suggestions" => Some(("emailOptIns.performanceSuggestions", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "email-opt-ins.physical-mail" => Some(("emailOptIns.physicalMail", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "email-opt-ins.phone-contact" => Some(("emailOptIns.phoneContact", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "family-name" => Some(("familyName", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "languages" => Some(("languages", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Vec })),
                    "markets" => Some(("markets", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Vec })),
                    "migrate-to-afa" => Some(("migrateToAfa", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "adwords-manager-account" => Some(("adwordsManagerAccount", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "phone-number" => Some(("phoneNumber", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "primary-country-code" => Some(("primaryCountryCode", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "email-address" => Some(("emailAddress", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    _ => {
                        let suggestion = FieldCursor::did_you_mean(key, &vec!["address", "address-line", "administrative-area", "adwords-manager-account", "channels", "dependent-locality", "email-address", "email-opt-ins", "family-name", "given-name", "industries", "job-functions", "language-code", "languages", "lat-lng", "latitude", "locality", "longitude", "market-comm", "markets", "migrate-to-afa", "performance-suggestions", "phone-contact", "phone-number", "physical-mail", "postal-code", "primary-country-code", "profile-public", "region-code", "sorting-code", "special-offers"]);
                        err.issues.push(CLIError::Field(FieldError::Unknown(temp_cursor.to_string(), suggestion, value.map(|v| v.to_string()))));
                        None
                    }
                };
            if let Some((field_cursor_str, type_info)) = type_info {
                FieldCursor::from(field_cursor_str).set_json_value(&mut object, value.unwrap(), type_info, err, &temp_cursor);
            }
        }
        let mut request: api::UserProfile = json::value::from_value(object).unwrap();
        let mut call = self.hub.users().update_profile(request);
        for parg in opt.values_of("v").map(|i|i.collect()).unwrap_or(Vec::new()).iter() {
            let (key, value) = parse_kv_arg(&*parg, err, false);
            match key {
                "request-metadata-user-overrides-user-id" => {
                    call = call.request_metadata_user_overrides_user_id(value.unwrap_or(""));
                },
                "request-metadata-user-overrides-ip-address" => {
                    call = call.request_metadata_user_overrides_ip_address(value.unwrap_or(""));
                },
                "request-metadata-traffic-source-traffic-sub-id" => {
                    call = call.request_metadata_traffic_source_traffic_sub_id(value.unwrap_or(""));
                },
                "request-metadata-traffic-source-traffic-source-id" => {
                    call = call.request_metadata_traffic_source_traffic_source_id(value.unwrap_or(""));
                },
                "request-metadata-partners-session-id" => {
                    call = call.request_metadata_partners_session_id(value.unwrap_or(""));
                },
                "request-metadata-locale" => {
                    call = call.request_metadata_locale(value.unwrap_or(""));
                },
                "request-metadata-experiment-ids" => {
                    call = call.add_request_metadata_experiment_ids(value.unwrap_or(""));
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
                                                                           v.extend(["request-metadata-experiment-ids", "request-metadata-locale", "request-metadata-partners-session-id", "request-metadata-traffic-source-traffic-source-id", "request-metadata-traffic-source-traffic-sub-id", "request-metadata-user-overrides-ip-address", "request-metadata-user-overrides-user-id"].iter().map(|v|*v));
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
            ("analytics", Some(opt)) => {
                match opt.subcommand() {
                    ("list", Some(opt)) => {
                        call_result = self._analytics_list(opt, dry_run, &mut err).await;
                    },
                    _ => {
                        err.issues.push(CLIError::MissingMethodError("analytics".to_string()));
                        writeln!(io::stderr(), "{}\n", opt.usage()).ok();
                    }
                }
            },
            ("client-messages", Some(opt)) => {
                match opt.subcommand() {
                    ("log", Some(opt)) => {
                        call_result = self._client_messages_log(opt, dry_run, &mut err).await;
                    },
                    _ => {
                        err.issues.push(CLIError::MissingMethodError("client-messages".to_string()));
                        writeln!(io::stderr(), "{}\n", opt.usage()).ok();
                    }
                }
            },
            ("companies", Some(opt)) => {
                match opt.subcommand() {
                    ("get", Some(opt)) => {
                        call_result = self._companies_get(opt, dry_run, &mut err).await;
                    },
                    ("leads-create", Some(opt)) => {
                        call_result = self._companies_leads_create(opt, dry_run, &mut err).await;
                    },
                    ("list", Some(opt)) => {
                        call_result = self._companies_list(opt, dry_run, &mut err).await;
                    },
                    _ => {
                        err.issues.push(CLIError::MissingMethodError("companies".to_string()));
                        writeln!(io::stderr(), "{}\n", opt.usage()).ok();
                    }
                }
            },
            ("leads", Some(opt)) => {
                match opt.subcommand() {
                    ("list", Some(opt)) => {
                        call_result = self._leads_list(opt, dry_run, &mut err).await;
                    },
                    _ => {
                        err.issues.push(CLIError::MissingMethodError("leads".to_string()));
                        writeln!(io::stderr(), "{}\n", opt.usage()).ok();
                    }
                }
            },
            ("methods", Some(opt)) => {
                match opt.subcommand() {
                    ("get-partnersstatus", Some(opt)) => {
                        call_result = self._methods_get_partnersstatus(opt, dry_run, &mut err).await;
                    },
                    ("update-companies", Some(opt)) => {
                        call_result = self._methods_update_companies(opt, dry_run, &mut err).await;
                    },
                    ("update-leads", Some(opt)) => {
                        call_result = self._methods_update_leads(opt, dry_run, &mut err).await;
                    },
                    _ => {
                        err.issues.push(CLIError::MissingMethodError("methods".to_string()));
                        writeln!(io::stderr(), "{}\n", opt.usage()).ok();
                    }
                }
            },
            ("offers", Some(opt)) => {
                match opt.subcommand() {
                    ("history-list", Some(opt)) => {
                        call_result = self._offers_history_list(opt, dry_run, &mut err).await;
                    },
                    ("list", Some(opt)) => {
                        call_result = self._offers_list(opt, dry_run, &mut err).await;
                    },
                    _ => {
                        err.issues.push(CLIError::MissingMethodError("offers".to_string()));
                        writeln!(io::stderr(), "{}\n", opt.usage()).ok();
                    }
                }
            },
            ("user-events", Some(opt)) => {
                match opt.subcommand() {
                    ("log", Some(opt)) => {
                        call_result = self._user_events_log(opt, dry_run, &mut err).await;
                    },
                    _ => {
                        err.issues.push(CLIError::MissingMethodError("user-events".to_string()));
                        writeln!(io::stderr(), "{}\n", opt.usage()).ok();
                    }
                }
            },
            ("user-states", Some(opt)) => {
                match opt.subcommand() {
                    ("list", Some(opt)) => {
                        call_result = self._user_states_list(opt, dry_run, &mut err).await;
                    },
                    _ => {
                        err.issues.push(CLIError::MissingMethodError("user-states".to_string()));
                        writeln!(io::stderr(), "{}\n", opt.usage()).ok();
                    }
                }
            },
            ("users", Some(opt)) => {
                match opt.subcommand() {
                    ("create-company-relation", Some(opt)) => {
                        call_result = self._users_create_company_relation(opt, dry_run, &mut err).await;
                    },
                    ("delete-company-relation", Some(opt)) => {
                        call_result = self._users_delete_company_relation(opt, dry_run, &mut err).await;
                    },
                    ("get", Some(opt)) => {
                        call_result = self._users_get(opt, dry_run, &mut err).await;
                    },
                    ("update-profile", Some(opt)) => {
                        call_result = self._users_update_profile(opt, dry_run, &mut err).await;
                    },
                    _ => {
                        err.issues.push(CLIError::MissingMethodError("users".to_string()));
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

            match client::application_secret_from_directory(&config_dir, "partners2-secret.json",
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
        ).persist_tokens_to_disk(format!("{}/partners2", config_dir)).build().await.unwrap();

        let engine = Engine {
            opt: opt,
            hub: api::Partners::new(client, auth),
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
        ("analytics", "methods: 'list'", vec![
            ("list",
                    Some(r##"Lists analytics data for a user's associated company.
        Should only be called within the context of an authorized logged in user."##),
                    "Details at http://byron.github.io/google-apis-rs/google_partners2_cli/analytics_list",
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
        
        ("client-messages", "methods: 'log'", vec![
            ("log",
                    Some(r##"Logs a generic message from the client, such as
        `Failed to render component`, `Profile page is running slow`,
        `More than 500 users have accessed this result.`, etc."##),
                    "Details at http://byron.github.io/google-apis-rs/google_partners2_cli/client-messages_log",
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
        
        ("companies", "methods: 'get', 'leads-create' and 'list'", vec![
            ("get",
                    Some(r##"Gets a company."##),
                    "Details at http://byron.github.io/google-apis-rs/google_partners2_cli/companies_get",
                  vec![
                    (Some(r##"company-id"##),
                     None,
                     Some(r##"The ID of the company to retrieve."##),
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
            ("leads-create",
                    Some(r##"Creates an advertiser lead for the given company ID."##),
                    "Details at http://byron.github.io/google-apis-rs/google_partners2_cli/companies_leads-create",
                  vec![
                    (Some(r##"company-id"##),
                     None,
                     Some(r##"The ID of the company to contact."##),
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
                    Some(r##"Lists companies."##),
                    "Details at http://byron.github.io/google-apis-rs/google_partners2_cli/companies_list",
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
        
        ("leads", "methods: 'list'", vec![
            ("list",
                    Some(r##"Lists advertiser leads for a user's associated company.
        Should only be called within the context of an authorized logged in user."##),
                    "Details at http://byron.github.io/google-apis-rs/google_partners2_cli/leads_list",
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
        
        ("methods", "methods: 'get-partnersstatus', 'update-companies' and 'update-leads'", vec![
            ("get-partnersstatus",
                    Some(r##"Gets Partners Status of the logged in user's agency.
        Should only be called if the logged in user is the admin of the agency."##),
                    "Details at http://byron.github.io/google-apis-rs/google_partners2_cli/methods_get-partnersstatus",
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
            ("update-companies",
                    Some(r##"Update company.
        Should only be called within the context of an authorized logged in user."##),
                    "Details at http://byron.github.io/google-apis-rs/google_partners2_cli/methods_update-companies",
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
            ("update-leads",
                    Some(r##"Updates the specified lead."##),
                    "Details at http://byron.github.io/google-apis-rs/google_partners2_cli/methods_update-leads",
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
        
        ("offers", "methods: 'history-list' and 'list'", vec![
            ("history-list",
                    Some(r##"Lists the Historical Offers for the current user (or user's entire company)"##),
                    "Details at http://byron.github.io/google-apis-rs/google_partners2_cli/offers_history-list",
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
                    Some(r##"Lists the Offers available for the current user"##),
                    "Details at http://byron.github.io/google-apis-rs/google_partners2_cli/offers_list",
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
        
        ("user-events", "methods: 'log'", vec![
            ("log",
                    Some(r##"Logs a user event."##),
                    "Details at http://byron.github.io/google-apis-rs/google_partners2_cli/user-events_log",
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
        
        ("user-states", "methods: 'list'", vec![
            ("list",
                    Some(r##"Lists states for current user."##),
                    "Details at http://byron.github.io/google-apis-rs/google_partners2_cli/user-states_list",
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
        
        ("users", "methods: 'create-company-relation', 'delete-company-relation', 'get' and 'update-profile'", vec![
            ("create-company-relation",
                    Some(r##"Creates a user's company relation. Affiliates the user to a company."##),
                    "Details at http://byron.github.io/google-apis-rs/google_partners2_cli/users_create-company-relation",
                  vec![
                    (Some(r##"user-id"##),
                     None,
                     Some(r##"The ID of the user. Can be set to <code>me</code> to mean
        the currently authenticated user."##),
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
            ("delete-company-relation",
                    Some(r##"Deletes a user's company relation. Unaffiliaites the user from a company."##),
                    "Details at http://byron.github.io/google-apis-rs/google_partners2_cli/users_delete-company-relation",
                  vec![
                    (Some(r##"user-id"##),
                     None,
                     Some(r##"The ID of the user. Can be set to <code>me</code> to mean
        the currently authenticated user."##),
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
                    Some(r##"Gets a user."##),
                    "Details at http://byron.github.io/google-apis-rs/google_partners2_cli/users_get",
                  vec![
                    (Some(r##"user-id"##),
                     None,
                     Some(r##"Identifier of the user. Can be set to <code>me</code> to mean the currently
        authenticated user."##),
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
            ("update-profile",
                    Some(r##"Updates a user's profile. A user can only update their own profile and
        should only be called within the context of a logged in user."##),
                    "Details at http://byron.github.io/google-apis-rs/google_partners2_cli/users_update-profile",
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
    
    let mut app = App::new("partners2")
           .author("Sebastian Thiel <byronimo@gmail.com>")
           .version("5.0.5+20180925")
           .about("Searches certified companies and creates contact leads with them, and also audits the usage of clients.")
           .after_help("All documentation details can be found at http://byron.github.io/google-apis-rs/google_partners2_cli")
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
