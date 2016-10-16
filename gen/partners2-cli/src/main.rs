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
extern crate google_partners2 as api;

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
    hub: api::Partners<hyper::Client, Authenticator<DefaultAuthenticatorDelegate, JsonTokenStorage, hyper::Client>>,
    gp: Vec<&'static str>,
    gpm: Vec<(&'static str, &'static str)>,
}


impl<'n> Engine<'n> {
    fn _client_messages_log(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
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
                    "request-metadata.partners-session-id" => Some(("requestMetadata.partnersSessionId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "request-metadata.traffic-source.traffic-sub-id" => Some(("requestMetadata.trafficSource.trafficSubId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "request-metadata.traffic-source.traffic-source-id" => Some(("requestMetadata.trafficSource.trafficSourceId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "request-metadata.user-overrides.user-id" => Some(("requestMetadata.userOverrides.userId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "request-metadata.user-overrides.ip-address" => Some(("requestMetadata.userOverrides.ipAddress", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "request-metadata.experiment-ids" => Some(("requestMetadata.experimentIds", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Vec })),
                    "details" => Some(("details", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "level" => Some(("level", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
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

    fn _companies_get(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
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
                                                                           v.extend(["order-by", "request-metadata-locale", "request-metadata-user-overrides-ip-address", "request-metadata-partners-session-id", "request-metadata-traffic-source-traffic-sub-id", "request-metadata-user-overrides-user-id", "address", "request-metadata-experiment-ids", "currency-code", "request-metadata-traffic-source-traffic-source-id", "view"].iter().map(|v|*v));
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

    fn _companies_leads_create(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
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
                    "request-metadata.partners-session-id" => Some(("requestMetadata.partnersSessionId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "request-metadata.traffic-source.traffic-sub-id" => Some(("requestMetadata.trafficSource.trafficSubId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "request-metadata.traffic-source.traffic-source-id" => Some(("requestMetadata.trafficSource.trafficSourceId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "request-metadata.user-overrides.user-id" => Some(("requestMetadata.userOverrides.userId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "request-metadata.user-overrides.ip-address" => Some(("requestMetadata.userOverrides.ipAddress", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "request-metadata.experiment-ids" => Some(("requestMetadata.experimentIds", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Vec })),
                    "recaptcha-challenge.id" => Some(("recaptchaChallenge.id", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "recaptcha-challenge.response" => Some(("recaptchaChallenge.response", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "lead.gps-motivations" => Some(("lead.gpsMotivations", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Vec })),
                    "lead.family-name" => Some(("lead.familyName", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "lead.min-monthly-budget.units" => Some(("lead.minMonthlyBudget.units", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "lead.min-monthly-budget.nanos" => Some(("lead.minMonthlyBudget.nanos", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "lead.min-monthly-budget.currency-code" => Some(("lead.minMonthlyBudget.currencyCode", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "lead.comments" => Some(("lead.comments", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "lead.email" => Some(("lead.email", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "lead.website-url" => Some(("lead.websiteUrl", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "lead.phone-number" => Some(("lead.phoneNumber", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "lead.given-name" => Some(("lead.givenName", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "lead.type" => Some(("lead.type", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "lead.id" => Some(("lead.id", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    _ => {
                        let suggestion = FieldCursor::did_you_mean(key, &vec!["comments", "currency-code", "email", "experiment-ids", "family-name", "given-name", "gps-motivations", "id", "ip-address", "lead", "locale", "min-monthly-budget", "nanos", "partners-session-id", "phone-number", "recaptcha-challenge", "request-metadata", "response", "traffic-source", "traffic-source-id", "traffic-sub-id", "type", "units", "user-id", "user-overrides", "website-url"]);
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

    fn _companies_list(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
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
                    call = call.page_size(arg_from_str(value.unwrap_or("-0"), err, "page-size", "integer"));
                },
                "order-by" => {
                    call = call.order_by(value.unwrap_or(""));
                },
                "min-monthly-budget-units" => {
                    call = call.min_monthly_budget_units(value.unwrap_or(""));
                },
                "min-monthly-budget-nanos" => {
                    call = call.min_monthly_budget_nanos(arg_from_str(value.unwrap_or("-0"), err, "min-monthly-budget-nanos", "integer"));
                },
                "min-monthly-budget-currency-code" => {
                    call = call.min_monthly_budget_currency_code(value.unwrap_or(""));
                },
                "max-monthly-budget-units" => {
                    call = call.max_monthly_budget_units(value.unwrap_or(""));
                },
                "max-monthly-budget-nanos" => {
                    call = call.max_monthly_budget_nanos(arg_from_str(value.unwrap_or("-0"), err, "max-monthly-budget-nanos", "integer"));
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
                                                                           v.extend(["order-by", "page-size", "request-metadata-partners-session-id", "max-monthly-budget-currency-code", "max-monthly-budget-nanos", "language-codes", "min-monthly-budget-nanos", "request-metadata-traffic-source-traffic-sub-id", "industries", "min-monthly-budget-currency-code", "min-monthly-budget-units", "page-token", "request-metadata-locale", "request-metadata-traffic-source-traffic-source-id", "company-name", "address", "services", "request-metadata-experiment-ids", "gps-motivations", "request-metadata-user-overrides-ip-address", "website-url", "request-metadata-user-overrides-user-id", "view", "max-monthly-budget-units"].iter().map(|v|*v));
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

    fn _user_events_log(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
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
                    "lead.gps-motivations" => Some(("lead.gpsMotivations", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Vec })),
                    "lead.family-name" => Some(("lead.familyName", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "lead.min-monthly-budget.units" => Some(("lead.minMonthlyBudget.units", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "lead.min-monthly-budget.nanos" => Some(("lead.minMonthlyBudget.nanos", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "lead.min-monthly-budget.currency-code" => Some(("lead.minMonthlyBudget.currencyCode", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "lead.comments" => Some(("lead.comments", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "lead.email" => Some(("lead.email", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "lead.website-url" => Some(("lead.websiteUrl", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "lead.phone-number" => Some(("lead.phoneNumber", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "lead.given-name" => Some(("lead.givenName", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "lead.type" => Some(("lead.type", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "lead.id" => Some(("lead.id", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "url" => Some(("url", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "event-action" => Some(("eventAction", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "event-category" => Some(("eventCategory", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "request-metadata.locale" => Some(("requestMetadata.locale", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "request-metadata.partners-session-id" => Some(("requestMetadata.partnersSessionId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "request-metadata.traffic-source.traffic-sub-id" => Some(("requestMetadata.trafficSource.trafficSubId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "request-metadata.traffic-source.traffic-source-id" => Some(("requestMetadata.trafficSource.trafficSourceId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "request-metadata.user-overrides.user-id" => Some(("requestMetadata.userOverrides.userId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "request-metadata.user-overrides.ip-address" => Some(("requestMetadata.userOverrides.ipAddress", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "request-metadata.experiment-ids" => Some(("requestMetadata.experimentIds", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Vec })),
                    "event-scope" => Some(("eventScope", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    _ => {
                        let suggestion = FieldCursor::did_you_mean(key, &vec!["comments", "currency-code", "email", "event-action", "event-category", "event-scope", "experiment-ids", "family-name", "given-name", "gps-motivations", "id", "ip-address", "lead", "locale", "min-monthly-budget", "nanos", "partners-session-id", "phone-number", "request-metadata", "traffic-source", "traffic-source-id", "traffic-sub-id", "type", "units", "url", "user-id", "user-overrides", "website-url"]);
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

    fn _user_states_list(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
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
                                                                           v.extend(["request-metadata-user-overrides-user-id", "request-metadata-user-overrides-ip-address", "request-metadata-partners-session-id", "request-metadata-traffic-source-traffic-sub-id", "request-metadata-locale", "request-metadata-experiment-ids", "request-metadata-traffic-source-traffic-source-id"].iter().map(|v|*v));
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
            ("client-messages", Some(opt)) => {
                match opt.subcommand() {
                    ("log", Some(opt)) => {
                        call_result = self._client_messages_log(opt, dry_run, &mut err);
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
                        call_result = self._companies_get(opt, dry_run, &mut err);
                    },
                    ("leads-create", Some(opt)) => {
                        call_result = self._companies_leads_create(opt, dry_run, &mut err);
                    },
                    ("list", Some(opt)) => {
                        call_result = self._companies_list(opt, dry_run, &mut err);
                    },
                    _ => {
                        err.issues.push(CLIError::MissingMethodError("companies".to_string()));
                        writeln!(io::stderr(), "{}\n", opt.usage()).ok();
                    }
                }
            },
            ("user-events", Some(opt)) => {
                match opt.subcommand() {
                    ("log", Some(opt)) => {
                        call_result = self._user_events_log(opt, dry_run, &mut err);
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
                        call_result = self._user_states_list(opt, dry_run, &mut err);
                    },
                    _ => {
                        err.issues.push(CLIError::MissingMethodError("user-states".to_string()));
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

            match cmn::application_secret_from_directory(&config_dir, "partners2-secret.json",
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
                                          program_name: "partners2",
                                          db_dir: config_dir.clone(),
                                        }, Some(FlowType::InstalledRedirect(54324)));

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
            hub: api::Partners::new(client, auth),
            gp: vec!["$-xgafv", "access-token", "alt", "bearer-token", "callback", "fields", "key", "oauth-token", "pp", "pretty-print", "quota-user", "upload-type", "upload-protocol"],
            gpm: vec![
                    ("$-xgafv", "$.xgafv"),
                    ("access-token", "access_token"),
                    ("bearer-token", "bearer_token"),
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
        ("client-messages", "methods: 'log'", vec![
            ("log",
                    Some(r##"Logs a generic message from the client, such as `Failed to render component`, `Profile page is running slow`, `More than 500 users have accessed this result.`, etc."##),
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
        
    ];
    
    let mut app = App::new("partners2")
           .author("Sebastian Thiel <byronimo@gmail.com>")
           .version("1.0.0+20151009")
           .about("Lets advertisers search certified companies and create contact leads with them, and also audits the usage of clients.")
           .after_help("All documentation details can be found at http://byron.github.io/google-apis-rs/google_partners2_cli")
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