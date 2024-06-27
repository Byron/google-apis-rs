// DO NOT EDIT !
// This file was generated automatically from 'src/generator/templates/cli/main.rs.mako'
// DO NOT EDIT !
#![allow(unused_variables, unused_imports, dead_code, unused_mut)]

#[macro_use]
extern crate clap;

use std::env;
use std::io::{self, Write};
use clap::{App, SubCommand, Arg};

use google_dialogflow3::{api, Error, oauth2, client::chrono, FieldMask};


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
    hub: api::Dialogflow<S>,
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
    async fn _projects_locations_agents_changelogs_get(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        let mut call = self.hub.projects().locations_agents_changelogs_get(opt.value_of("name").unwrap_or(""));
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

    async fn _projects_locations_agents_changelogs_list(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        let mut call = self.hub.projects().locations_agents_changelogs_list(opt.value_of("parent").unwrap_or(""));
        for parg in opt.values_of("v").map(|i|i.collect()).unwrap_or(Vec::new()).iter() {
            let (key, value) = parse_kv_arg(&*parg, err, false);
            match key {
                "page-token" => {
                    call = call.page_token(value.unwrap_or(""));
                },
                "page-size" => {
                    call = call.page_size(        value.map(|v| arg_from_str(v, err, "page-size", "int32")).unwrap_or(-0));
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
                                                                           v.extend(["filter", "page-size", "page-token"].iter().map(|v|*v));
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

    async fn _projects_locations_agents_create(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
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
                    "advanced-settings.audio-export-gcs-destination.uri" => Some(("advancedSettings.audioExportGcsDestination.uri", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "advanced-settings.dtmf-settings.enabled" => Some(("advancedSettings.dtmfSettings.enabled", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "advanced-settings.dtmf-settings.endpointing-timeout-duration" => Some(("advancedSettings.dtmfSettings.endpointingTimeoutDuration", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "advanced-settings.dtmf-settings.finish-digit" => Some(("advancedSettings.dtmfSettings.finishDigit", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "advanced-settings.dtmf-settings.interdigit-timeout-duration" => Some(("advancedSettings.dtmfSettings.interdigitTimeoutDuration", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "advanced-settings.dtmf-settings.max-digits" => Some(("advancedSettings.dtmfSettings.maxDigits", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "advanced-settings.logging-settings.enable-interaction-logging" => Some(("advancedSettings.loggingSettings.enableInteractionLogging", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "advanced-settings.logging-settings.enable-stackdriver-logging" => Some(("advancedSettings.loggingSettings.enableStackdriverLogging", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "advanced-settings.speech-settings.endpointer-sensitivity" => Some(("advancedSettings.speechSettings.endpointerSensitivity", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "advanced-settings.speech-settings.models" => Some(("advancedSettings.speechSettings.models", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Map })),
                    "advanced-settings.speech-settings.no-speech-timeout" => Some(("advancedSettings.speechSettings.noSpeechTimeout", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "advanced-settings.speech-settings.use-timeout-based-endpointing" => Some(("advancedSettings.speechSettings.useTimeoutBasedEndpointing", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "answer-feedback-settings.enable-answer-feedback" => Some(("answerFeedbackSettings.enableAnswerFeedback", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "avatar-uri" => Some(("avatarUri", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "default-language-code" => Some(("defaultLanguageCode", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "description" => Some(("description", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "display-name" => Some(("displayName", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "enable-multi-language-training" => Some(("enableMultiLanguageTraining", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "enable-spell-correction" => Some(("enableSpellCorrection", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "enable-stackdriver-logging" => Some(("enableStackdriverLogging", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "gen-app-builder-settings.engine" => Some(("genAppBuilderSettings.engine", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "git-integration-settings.github-settings.access-token" => Some(("gitIntegrationSettings.githubSettings.accessToken", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "git-integration-settings.github-settings.branches" => Some(("gitIntegrationSettings.githubSettings.branches", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Vec })),
                    "git-integration-settings.github-settings.display-name" => Some(("gitIntegrationSettings.githubSettings.displayName", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "git-integration-settings.github-settings.repository-uri" => Some(("gitIntegrationSettings.githubSettings.repositoryUri", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "git-integration-settings.github-settings.tracking-branch" => Some(("gitIntegrationSettings.githubSettings.trackingBranch", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "locked" => Some(("locked", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "name" => Some(("name", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "security-settings" => Some(("securitySettings", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "speech-to-text-settings.enable-speech-adaptation" => Some(("speechToTextSettings.enableSpeechAdaptation", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "start-flow" => Some(("startFlow", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "supported-language-codes" => Some(("supportedLanguageCodes", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Vec })),
                    "time-zone" => Some(("timeZone", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    _ => {
                        let suggestion = FieldCursor::did_you_mean(key, &vec!["access-token", "advanced-settings", "answer-feedback-settings", "audio-export-gcs-destination", "avatar-uri", "branches", "default-language-code", "description", "display-name", "dtmf-settings", "enable-answer-feedback", "enable-interaction-logging", "enable-multi-language-training", "enable-speech-adaptation", "enable-spell-correction", "enable-stackdriver-logging", "enabled", "endpointer-sensitivity", "endpointing-timeout-duration", "engine", "finish-digit", "gen-app-builder-settings", "git-integration-settings", "github-settings", "interdigit-timeout-duration", "locked", "logging-settings", "max-digits", "models", "name", "no-speech-timeout", "repository-uri", "security-settings", "speech-settings", "speech-to-text-settings", "start-flow", "supported-language-codes", "time-zone", "tracking-branch", "uri", "use-timeout-based-endpointing"]);
                        err.issues.push(CLIError::Field(FieldError::Unknown(temp_cursor.to_string(), suggestion, value.map(|v| v.to_string()))));
                        None
                    }
                };
            if let Some((field_cursor_str, type_info)) = type_info {
                FieldCursor::from(field_cursor_str).set_json_value(&mut object, value.unwrap(), type_info, err, &temp_cursor);
            }
        }
        let mut request: api::GoogleCloudDialogflowCxV3Agent = json::value::from_value(object).unwrap();
        let mut call = self.hub.projects().locations_agents_create(request, opt.value_of("parent").unwrap_or(""));
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

    async fn _projects_locations_agents_delete(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        let mut call = self.hub.projects().locations_agents_delete(opt.value_of("name").unwrap_or(""));
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

    async fn _projects_locations_agents_entity_types_create(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
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
                    "auto-expansion-mode" => Some(("autoExpansionMode", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "display-name" => Some(("displayName", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "enable-fuzzy-extraction" => Some(("enableFuzzyExtraction", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "kind" => Some(("kind", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "name" => Some(("name", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "redact" => Some(("redact", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    _ => {
                        let suggestion = FieldCursor::did_you_mean(key, &vec!["auto-expansion-mode", "display-name", "enable-fuzzy-extraction", "kind", "name", "redact"]);
                        err.issues.push(CLIError::Field(FieldError::Unknown(temp_cursor.to_string(), suggestion, value.map(|v| v.to_string()))));
                        None
                    }
                };
            if let Some((field_cursor_str, type_info)) = type_info {
                FieldCursor::from(field_cursor_str).set_json_value(&mut object, value.unwrap(), type_info, err, &temp_cursor);
            }
        }
        let mut request: api::GoogleCloudDialogflowCxV3EntityType = json::value::from_value(object).unwrap();
        let mut call = self.hub.projects().locations_agents_entity_types_create(request, opt.value_of("parent").unwrap_or(""));
        for parg in opt.values_of("v").map(|i|i.collect()).unwrap_or(Vec::new()).iter() {
            let (key, value) = parse_kv_arg(&*parg, err, false);
            match key {
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
                                                                           v.extend(["language-code"].iter().map(|v|*v));
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

    async fn _projects_locations_agents_entity_types_delete(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        let mut call = self.hub.projects().locations_agents_entity_types_delete(opt.value_of("name").unwrap_or(""));
        for parg in opt.values_of("v").map(|i|i.collect()).unwrap_or(Vec::new()).iter() {
            let (key, value) = parse_kv_arg(&*parg, err, false);
            match key {
                "force" => {
                    call = call.force(        value.map(|v| arg_from_str(v, err, "force", "boolean")).unwrap_or(false));
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
                                                                           v.extend(["force"].iter().map(|v|*v));
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

    async fn _projects_locations_agents_entity_types_export(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
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
                    "data-format" => Some(("dataFormat", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "entity-types" => Some(("entityTypes", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Vec })),
                    "entity-types-content-inline" => Some(("entityTypesContentInline", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "entity-types-uri" => Some(("entityTypesUri", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "language-code" => Some(("languageCode", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    _ => {
                        let suggestion = FieldCursor::did_you_mean(key, &vec!["data-format", "entity-types", "entity-types-content-inline", "entity-types-uri", "language-code"]);
                        err.issues.push(CLIError::Field(FieldError::Unknown(temp_cursor.to_string(), suggestion, value.map(|v| v.to_string()))));
                        None
                    }
                };
            if let Some((field_cursor_str, type_info)) = type_info {
                FieldCursor::from(field_cursor_str).set_json_value(&mut object, value.unwrap(), type_info, err, &temp_cursor);
            }
        }
        let mut request: api::GoogleCloudDialogflowCxV3ExportEntityTypesRequest = json::value::from_value(object).unwrap();
        let mut call = self.hub.projects().locations_agents_entity_types_export(request, opt.value_of("parent").unwrap_or(""));
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

    async fn _projects_locations_agents_entity_types_get(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        let mut call = self.hub.projects().locations_agents_entity_types_get(opt.value_of("name").unwrap_or(""));
        for parg in opt.values_of("v").map(|i|i.collect()).unwrap_or(Vec::new()).iter() {
            let (key, value) = parse_kv_arg(&*parg, err, false);
            match key {
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
                                                                           v.extend(["language-code"].iter().map(|v|*v));
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

    async fn _projects_locations_agents_entity_types_import(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
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
                    "entity-types-content.content" => Some(("entityTypesContent.content", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "entity-types-uri" => Some(("entityTypesUri", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "merge-option" => Some(("mergeOption", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "target-entity-type" => Some(("targetEntityType", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    _ => {
                        let suggestion = FieldCursor::did_you_mean(key, &vec!["content", "entity-types-content", "entity-types-uri", "merge-option", "target-entity-type"]);
                        err.issues.push(CLIError::Field(FieldError::Unknown(temp_cursor.to_string(), suggestion, value.map(|v| v.to_string()))));
                        None
                    }
                };
            if let Some((field_cursor_str, type_info)) = type_info {
                FieldCursor::from(field_cursor_str).set_json_value(&mut object, value.unwrap(), type_info, err, &temp_cursor);
            }
        }
        let mut request: api::GoogleCloudDialogflowCxV3ImportEntityTypesRequest = json::value::from_value(object).unwrap();
        let mut call = self.hub.projects().locations_agents_entity_types_import(request, opt.value_of("parent").unwrap_or(""));
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

    async fn _projects_locations_agents_entity_types_list(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        let mut call = self.hub.projects().locations_agents_entity_types_list(opt.value_of("parent").unwrap_or(""));
        for parg in opt.values_of("v").map(|i|i.collect()).unwrap_or(Vec::new()).iter() {
            let (key, value) = parse_kv_arg(&*parg, err, false);
            match key {
                "page-token" => {
                    call = call.page_token(value.unwrap_or(""));
                },
                "page-size" => {
                    call = call.page_size(        value.map(|v| arg_from_str(v, err, "page-size", "int32")).unwrap_or(-0));
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
                                                                           v.extend(["language-code", "page-size", "page-token"].iter().map(|v|*v));
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

    async fn _projects_locations_agents_entity_types_patch(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
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
                    "auto-expansion-mode" => Some(("autoExpansionMode", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "display-name" => Some(("displayName", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "enable-fuzzy-extraction" => Some(("enableFuzzyExtraction", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "kind" => Some(("kind", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "name" => Some(("name", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "redact" => Some(("redact", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    _ => {
                        let suggestion = FieldCursor::did_you_mean(key, &vec!["auto-expansion-mode", "display-name", "enable-fuzzy-extraction", "kind", "name", "redact"]);
                        err.issues.push(CLIError::Field(FieldError::Unknown(temp_cursor.to_string(), suggestion, value.map(|v| v.to_string()))));
                        None
                    }
                };
            if let Some((field_cursor_str, type_info)) = type_info {
                FieldCursor::from(field_cursor_str).set_json_value(&mut object, value.unwrap(), type_info, err, &temp_cursor);
            }
        }
        let mut request: api::GoogleCloudDialogflowCxV3EntityType = json::value::from_value(object).unwrap();
        let mut call = self.hub.projects().locations_agents_entity_types_patch(request, opt.value_of("name").unwrap_or(""));
        for parg in opt.values_of("v").map(|i|i.collect()).unwrap_or(Vec::new()).iter() {
            let (key, value) = parse_kv_arg(&*parg, err, false);
            match key {
                "update-mask" => {
                    call = call.update_mask(        value.map(|v| arg_from_str(v, err, "update-mask", "google-fieldmask")).unwrap_or(FieldMask::default()));
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
                                                                           v.extend(["language-code", "update-mask"].iter().map(|v|*v));
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

    async fn _projects_locations_agents_environments_continuous_test_results_list(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        let mut call = self.hub.projects().locations_agents_environments_continuous_test_results_list(opt.value_of("parent").unwrap_or(""));
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

    async fn _projects_locations_agents_environments_create(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
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
                    "description" => Some(("description", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "display-name" => Some(("displayName", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "name" => Some(("name", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "test-cases-config.enable-continuous-run" => Some(("testCasesConfig.enableContinuousRun", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "test-cases-config.enable-predeployment-run" => Some(("testCasesConfig.enablePredeploymentRun", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "test-cases-config.test-cases" => Some(("testCasesConfig.testCases", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Vec })),
                    "update-time" => Some(("updateTime", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    _ => {
                        let suggestion = FieldCursor::did_you_mean(key, &vec!["description", "display-name", "enable-continuous-run", "enable-predeployment-run", "name", "test-cases", "test-cases-config", "update-time"]);
                        err.issues.push(CLIError::Field(FieldError::Unknown(temp_cursor.to_string(), suggestion, value.map(|v| v.to_string()))));
                        None
                    }
                };
            if let Some((field_cursor_str, type_info)) = type_info {
                FieldCursor::from(field_cursor_str).set_json_value(&mut object, value.unwrap(), type_info, err, &temp_cursor);
            }
        }
        let mut request: api::GoogleCloudDialogflowCxV3Environment = json::value::from_value(object).unwrap();
        let mut call = self.hub.projects().locations_agents_environments_create(request, opt.value_of("parent").unwrap_or(""));
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

    async fn _projects_locations_agents_environments_delete(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        let mut call = self.hub.projects().locations_agents_environments_delete(opt.value_of("name").unwrap_or(""));
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

    async fn _projects_locations_agents_environments_deploy_flow(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
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
                    "flow-version" => Some(("flowVersion", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    _ => {
                        let suggestion = FieldCursor::did_you_mean(key, &vec!["flow-version"]);
                        err.issues.push(CLIError::Field(FieldError::Unknown(temp_cursor.to_string(), suggestion, value.map(|v| v.to_string()))));
                        None
                    }
                };
            if let Some((field_cursor_str, type_info)) = type_info {
                FieldCursor::from(field_cursor_str).set_json_value(&mut object, value.unwrap(), type_info, err, &temp_cursor);
            }
        }
        let mut request: api::GoogleCloudDialogflowCxV3DeployFlowRequest = json::value::from_value(object).unwrap();
        let mut call = self.hub.projects().locations_agents_environments_deploy_flow(request, opt.value_of("environment").unwrap_or(""));
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

    async fn _projects_locations_agents_environments_deployments_get(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        let mut call = self.hub.projects().locations_agents_environments_deployments_get(opt.value_of("name").unwrap_or(""));
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

    async fn _projects_locations_agents_environments_deployments_list(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        let mut call = self.hub.projects().locations_agents_environments_deployments_list(opt.value_of("parent").unwrap_or(""));
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

    async fn _projects_locations_agents_environments_experiments_create(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
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
                    "create-time" => Some(("createTime", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "definition.condition" => Some(("definition.condition", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "description" => Some(("description", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "display-name" => Some(("displayName", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "end-time" => Some(("endTime", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "experiment-length" => Some(("experimentLength", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "last-update-time" => Some(("lastUpdateTime", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "name" => Some(("name", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "result.last-update-time" => Some(("result.lastUpdateTime", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "rollout-config.failure-condition" => Some(("rolloutConfig.failureCondition", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "rollout-config.rollout-condition" => Some(("rolloutConfig.rolloutCondition", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "rollout-failure-reason" => Some(("rolloutFailureReason", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "rollout-state.start-time" => Some(("rolloutState.startTime", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "rollout-state.step" => Some(("rolloutState.step", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "rollout-state.step-index" => Some(("rolloutState.stepIndex", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "start-time" => Some(("startTime", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "state" => Some(("state", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    _ => {
                        let suggestion = FieldCursor::did_you_mean(key, &vec!["condition", "create-time", "definition", "description", "display-name", "end-time", "experiment-length", "failure-condition", "last-update-time", "name", "result", "rollout-condition", "rollout-config", "rollout-failure-reason", "rollout-state", "start-time", "state", "step", "step-index"]);
                        err.issues.push(CLIError::Field(FieldError::Unknown(temp_cursor.to_string(), suggestion, value.map(|v| v.to_string()))));
                        None
                    }
                };
            if let Some((field_cursor_str, type_info)) = type_info {
                FieldCursor::from(field_cursor_str).set_json_value(&mut object, value.unwrap(), type_info, err, &temp_cursor);
            }
        }
        let mut request: api::GoogleCloudDialogflowCxV3Experiment = json::value::from_value(object).unwrap();
        let mut call = self.hub.projects().locations_agents_environments_experiments_create(request, opt.value_of("parent").unwrap_or(""));
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

    async fn _projects_locations_agents_environments_experiments_delete(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        let mut call = self.hub.projects().locations_agents_environments_experiments_delete(opt.value_of("name").unwrap_or(""));
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

    async fn _projects_locations_agents_environments_experiments_get(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        let mut call = self.hub.projects().locations_agents_environments_experiments_get(opt.value_of("name").unwrap_or(""));
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

    async fn _projects_locations_agents_environments_experiments_list(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        let mut call = self.hub.projects().locations_agents_environments_experiments_list(opt.value_of("parent").unwrap_or(""));
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

    async fn _projects_locations_agents_environments_experiments_patch(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
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
                    "create-time" => Some(("createTime", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "definition.condition" => Some(("definition.condition", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "description" => Some(("description", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "display-name" => Some(("displayName", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "end-time" => Some(("endTime", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "experiment-length" => Some(("experimentLength", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "last-update-time" => Some(("lastUpdateTime", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "name" => Some(("name", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "result.last-update-time" => Some(("result.lastUpdateTime", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "rollout-config.failure-condition" => Some(("rolloutConfig.failureCondition", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "rollout-config.rollout-condition" => Some(("rolloutConfig.rolloutCondition", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "rollout-failure-reason" => Some(("rolloutFailureReason", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "rollout-state.start-time" => Some(("rolloutState.startTime", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "rollout-state.step" => Some(("rolloutState.step", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "rollout-state.step-index" => Some(("rolloutState.stepIndex", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "start-time" => Some(("startTime", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "state" => Some(("state", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    _ => {
                        let suggestion = FieldCursor::did_you_mean(key, &vec!["condition", "create-time", "definition", "description", "display-name", "end-time", "experiment-length", "failure-condition", "last-update-time", "name", "result", "rollout-condition", "rollout-config", "rollout-failure-reason", "rollout-state", "start-time", "state", "step", "step-index"]);
                        err.issues.push(CLIError::Field(FieldError::Unknown(temp_cursor.to_string(), suggestion, value.map(|v| v.to_string()))));
                        None
                    }
                };
            if let Some((field_cursor_str, type_info)) = type_info {
                FieldCursor::from(field_cursor_str).set_json_value(&mut object, value.unwrap(), type_info, err, &temp_cursor);
            }
        }
        let mut request: api::GoogleCloudDialogflowCxV3Experiment = json::value::from_value(object).unwrap();
        let mut call = self.hub.projects().locations_agents_environments_experiments_patch(request, opt.value_of("name").unwrap_or(""));
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

    async fn _projects_locations_agents_environments_experiments_start(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
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
        let mut request: api::GoogleCloudDialogflowCxV3StartExperimentRequest = json::value::from_value(object).unwrap();
        let mut call = self.hub.projects().locations_agents_environments_experiments_start(request, opt.value_of("name").unwrap_or(""));
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

    async fn _projects_locations_agents_environments_experiments_stop(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
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
        let mut request: api::GoogleCloudDialogflowCxV3StopExperimentRequest = json::value::from_value(object).unwrap();
        let mut call = self.hub.projects().locations_agents_environments_experiments_stop(request, opt.value_of("name").unwrap_or(""));
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

    async fn _projects_locations_agents_environments_get(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        let mut call = self.hub.projects().locations_agents_environments_get(opt.value_of("name").unwrap_or(""));
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

    async fn _projects_locations_agents_environments_list(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        let mut call = self.hub.projects().locations_agents_environments_list(opt.value_of("parent").unwrap_or(""));
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

    async fn _projects_locations_agents_environments_lookup_environment_history(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        let mut call = self.hub.projects().locations_agents_environments_lookup_environment_history(opt.value_of("name").unwrap_or(""));
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

    async fn _projects_locations_agents_environments_patch(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
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
                    "description" => Some(("description", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "display-name" => Some(("displayName", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "name" => Some(("name", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "test-cases-config.enable-continuous-run" => Some(("testCasesConfig.enableContinuousRun", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "test-cases-config.enable-predeployment-run" => Some(("testCasesConfig.enablePredeploymentRun", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "test-cases-config.test-cases" => Some(("testCasesConfig.testCases", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Vec })),
                    "update-time" => Some(("updateTime", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    _ => {
                        let suggestion = FieldCursor::did_you_mean(key, &vec!["description", "display-name", "enable-continuous-run", "enable-predeployment-run", "name", "test-cases", "test-cases-config", "update-time"]);
                        err.issues.push(CLIError::Field(FieldError::Unknown(temp_cursor.to_string(), suggestion, value.map(|v| v.to_string()))));
                        None
                    }
                };
            if let Some((field_cursor_str, type_info)) = type_info {
                FieldCursor::from(field_cursor_str).set_json_value(&mut object, value.unwrap(), type_info, err, &temp_cursor);
            }
        }
        let mut request: api::GoogleCloudDialogflowCxV3Environment = json::value::from_value(object).unwrap();
        let mut call = self.hub.projects().locations_agents_environments_patch(request, opt.value_of("name").unwrap_or(""));
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

    async fn _projects_locations_agents_environments_run_continuous_test(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
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
        let mut request: api::GoogleCloudDialogflowCxV3RunContinuousTestRequest = json::value::from_value(object).unwrap();
        let mut call = self.hub.projects().locations_agents_environments_run_continuous_test(request, opt.value_of("environment").unwrap_or(""));
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

    async fn _projects_locations_agents_environments_sessions_detect_intent(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
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
                    "output-audio-config.audio-encoding" => Some(("outputAudioConfig.audioEncoding", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "output-audio-config.sample-rate-hertz" => Some(("outputAudioConfig.sampleRateHertz", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "output-audio-config.synthesize-speech-config.effects-profile-id" => Some(("outputAudioConfig.synthesizeSpeechConfig.effectsProfileId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Vec })),
                    "output-audio-config.synthesize-speech-config.pitch" => Some(("outputAudioConfig.synthesizeSpeechConfig.pitch", JsonTypeInfo { jtype: JsonType::Float, ctype: ComplexType::Pod })),
                    "output-audio-config.synthesize-speech-config.speaking-rate" => Some(("outputAudioConfig.synthesizeSpeechConfig.speakingRate", JsonTypeInfo { jtype: JsonType::Float, ctype: ComplexType::Pod })),
                    "output-audio-config.synthesize-speech-config.voice.name" => Some(("outputAudioConfig.synthesizeSpeechConfig.voice.name", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "output-audio-config.synthesize-speech-config.voice.ssml-gender" => Some(("outputAudioConfig.synthesizeSpeechConfig.voice.ssmlGender", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "output-audio-config.synthesize-speech-config.volume-gain-db" => Some(("outputAudioConfig.synthesizeSpeechConfig.volumeGainDb", JsonTypeInfo { jtype: JsonType::Float, ctype: ComplexType::Pod })),
                    "query-input.audio.audio" => Some(("queryInput.audio.audio", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "query-input.audio.config.audio-encoding" => Some(("queryInput.audio.config.audioEncoding", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "query-input.audio.config.barge-in-config.no-barge-in-duration" => Some(("queryInput.audio.config.bargeInConfig.noBargeInDuration", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "query-input.audio.config.barge-in-config.total-duration" => Some(("queryInput.audio.config.bargeInConfig.totalDuration", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "query-input.audio.config.enable-word-info" => Some(("queryInput.audio.config.enableWordInfo", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "query-input.audio.config.model" => Some(("queryInput.audio.config.model", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "query-input.audio.config.model-variant" => Some(("queryInput.audio.config.modelVariant", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "query-input.audio.config.opt-out-conformer-model-migration" => Some(("queryInput.audio.config.optOutConformerModelMigration", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "query-input.audio.config.phrase-hints" => Some(("queryInput.audio.config.phraseHints", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Vec })),
                    "query-input.audio.config.sample-rate-hertz" => Some(("queryInput.audio.config.sampleRateHertz", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "query-input.audio.config.single-utterance" => Some(("queryInput.audio.config.singleUtterance", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "query-input.dtmf.digits" => Some(("queryInput.dtmf.digits", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "query-input.dtmf.finish-digit" => Some(("queryInput.dtmf.finishDigit", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "query-input.event.event" => Some(("queryInput.event.event", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "query-input.intent.intent" => Some(("queryInput.intent.intent", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "query-input.language-code" => Some(("queryInput.languageCode", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "query-input.text.text" => Some(("queryInput.text.text", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "query-params.analyze-query-text-sentiment" => Some(("queryParams.analyzeQueryTextSentiment", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "query-params.channel" => Some(("queryParams.channel", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "query-params.current-page" => Some(("queryParams.currentPage", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "query-params.disable-webhook" => Some(("queryParams.disableWebhook", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "query-params.flow-versions" => Some(("queryParams.flowVersions", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Vec })),
                    "query-params.geo-location.latitude" => Some(("queryParams.geoLocation.latitude", JsonTypeInfo { jtype: JsonType::Float, ctype: ComplexType::Pod })),
                    "query-params.geo-location.longitude" => Some(("queryParams.geoLocation.longitude", JsonTypeInfo { jtype: JsonType::Float, ctype: ComplexType::Pod })),
                    "query-params.populate-data-store-connection-signals" => Some(("queryParams.populateDataStoreConnectionSignals", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "query-params.session-ttl" => Some(("queryParams.sessionTtl", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "query-params.time-zone" => Some(("queryParams.timeZone", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "query-params.webhook-headers" => Some(("queryParams.webhookHeaders", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Map })),
                    _ => {
                        let suggestion = FieldCursor::did_you_mean(key, &vec!["analyze-query-text-sentiment", "audio", "audio-encoding", "barge-in-config", "channel", "config", "current-page", "digits", "disable-webhook", "dtmf", "effects-profile-id", "enable-word-info", "event", "finish-digit", "flow-versions", "geo-location", "intent", "language-code", "latitude", "longitude", "model", "model-variant", "name", "no-barge-in-duration", "opt-out-conformer-model-migration", "output-audio-config", "phrase-hints", "pitch", "populate-data-store-connection-signals", "query-input", "query-params", "sample-rate-hertz", "session-ttl", "single-utterance", "speaking-rate", "ssml-gender", "synthesize-speech-config", "text", "time-zone", "total-duration", "voice", "volume-gain-db", "webhook-headers"]);
                        err.issues.push(CLIError::Field(FieldError::Unknown(temp_cursor.to_string(), suggestion, value.map(|v| v.to_string()))));
                        None
                    }
                };
            if let Some((field_cursor_str, type_info)) = type_info {
                FieldCursor::from(field_cursor_str).set_json_value(&mut object, value.unwrap(), type_info, err, &temp_cursor);
            }
        }
        let mut request: api::GoogleCloudDialogflowCxV3DetectIntentRequest = json::value::from_value(object).unwrap();
        let mut call = self.hub.projects().locations_agents_environments_sessions_detect_intent(request, opt.value_of("session").unwrap_or(""));
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

    async fn _projects_locations_agents_environments_sessions_entity_types_create(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
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
                    "entity-override-mode" => Some(("entityOverrideMode", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "name" => Some(("name", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    _ => {
                        let suggestion = FieldCursor::did_you_mean(key, &vec!["entity-override-mode", "name"]);
                        err.issues.push(CLIError::Field(FieldError::Unknown(temp_cursor.to_string(), suggestion, value.map(|v| v.to_string()))));
                        None
                    }
                };
            if let Some((field_cursor_str, type_info)) = type_info {
                FieldCursor::from(field_cursor_str).set_json_value(&mut object, value.unwrap(), type_info, err, &temp_cursor);
            }
        }
        let mut request: api::GoogleCloudDialogflowCxV3SessionEntityType = json::value::from_value(object).unwrap();
        let mut call = self.hub.projects().locations_agents_environments_sessions_entity_types_create(request, opt.value_of("parent").unwrap_or(""));
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

    async fn _projects_locations_agents_environments_sessions_entity_types_delete(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        let mut call = self.hub.projects().locations_agents_environments_sessions_entity_types_delete(opt.value_of("name").unwrap_or(""));
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

    async fn _projects_locations_agents_environments_sessions_entity_types_get(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        let mut call = self.hub.projects().locations_agents_environments_sessions_entity_types_get(opt.value_of("name").unwrap_or(""));
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

    async fn _projects_locations_agents_environments_sessions_entity_types_list(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        let mut call = self.hub.projects().locations_agents_environments_sessions_entity_types_list(opt.value_of("parent").unwrap_or(""));
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

    async fn _projects_locations_agents_environments_sessions_entity_types_patch(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
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
                    "entity-override-mode" => Some(("entityOverrideMode", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "name" => Some(("name", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    _ => {
                        let suggestion = FieldCursor::did_you_mean(key, &vec!["entity-override-mode", "name"]);
                        err.issues.push(CLIError::Field(FieldError::Unknown(temp_cursor.to_string(), suggestion, value.map(|v| v.to_string()))));
                        None
                    }
                };
            if let Some((field_cursor_str, type_info)) = type_info {
                FieldCursor::from(field_cursor_str).set_json_value(&mut object, value.unwrap(), type_info, err, &temp_cursor);
            }
        }
        let mut request: api::GoogleCloudDialogflowCxV3SessionEntityType = json::value::from_value(object).unwrap();
        let mut call = self.hub.projects().locations_agents_environments_sessions_entity_types_patch(request, opt.value_of("name").unwrap_or(""));
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

    async fn _projects_locations_agents_environments_sessions_fulfill_intent(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
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
                    "match.confidence" => Some(("match.confidence", JsonTypeInfo { jtype: JsonType::Float, ctype: ComplexType::Pod })),
                    "match.event" => Some(("match.event", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "match.intent.description" => Some(("match.intent.description", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "match.intent.display-name" => Some(("match.intent.displayName", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "match.intent.is-fallback" => Some(("match.intent.isFallback", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "match.intent.labels" => Some(("match.intent.labels", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Map })),
                    "match.intent.name" => Some(("match.intent.name", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "match.intent.priority" => Some(("match.intent.priority", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "match.match-type" => Some(("match.matchType", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "match.resolved-input" => Some(("match.resolvedInput", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "match-intent-request.persist-parameter-changes" => Some(("matchIntentRequest.persistParameterChanges", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "match-intent-request.query-input.audio.audio" => Some(("matchIntentRequest.queryInput.audio.audio", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "match-intent-request.query-input.audio.config.audio-encoding" => Some(("matchIntentRequest.queryInput.audio.config.audioEncoding", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "match-intent-request.query-input.audio.config.barge-in-config.no-barge-in-duration" => Some(("matchIntentRequest.queryInput.audio.config.bargeInConfig.noBargeInDuration", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "match-intent-request.query-input.audio.config.barge-in-config.total-duration" => Some(("matchIntentRequest.queryInput.audio.config.bargeInConfig.totalDuration", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "match-intent-request.query-input.audio.config.enable-word-info" => Some(("matchIntentRequest.queryInput.audio.config.enableWordInfo", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "match-intent-request.query-input.audio.config.model" => Some(("matchIntentRequest.queryInput.audio.config.model", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "match-intent-request.query-input.audio.config.model-variant" => Some(("matchIntentRequest.queryInput.audio.config.modelVariant", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "match-intent-request.query-input.audio.config.opt-out-conformer-model-migration" => Some(("matchIntentRequest.queryInput.audio.config.optOutConformerModelMigration", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "match-intent-request.query-input.audio.config.phrase-hints" => Some(("matchIntentRequest.queryInput.audio.config.phraseHints", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Vec })),
                    "match-intent-request.query-input.audio.config.sample-rate-hertz" => Some(("matchIntentRequest.queryInput.audio.config.sampleRateHertz", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "match-intent-request.query-input.audio.config.single-utterance" => Some(("matchIntentRequest.queryInput.audio.config.singleUtterance", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "match-intent-request.query-input.dtmf.digits" => Some(("matchIntentRequest.queryInput.dtmf.digits", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "match-intent-request.query-input.dtmf.finish-digit" => Some(("matchIntentRequest.queryInput.dtmf.finishDigit", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "match-intent-request.query-input.event.event" => Some(("matchIntentRequest.queryInput.event.event", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "match-intent-request.query-input.intent.intent" => Some(("matchIntentRequest.queryInput.intent.intent", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "match-intent-request.query-input.language-code" => Some(("matchIntentRequest.queryInput.languageCode", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "match-intent-request.query-input.text.text" => Some(("matchIntentRequest.queryInput.text.text", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "match-intent-request.query-params.analyze-query-text-sentiment" => Some(("matchIntentRequest.queryParams.analyzeQueryTextSentiment", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "match-intent-request.query-params.channel" => Some(("matchIntentRequest.queryParams.channel", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "match-intent-request.query-params.current-page" => Some(("matchIntentRequest.queryParams.currentPage", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "match-intent-request.query-params.disable-webhook" => Some(("matchIntentRequest.queryParams.disableWebhook", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "match-intent-request.query-params.flow-versions" => Some(("matchIntentRequest.queryParams.flowVersions", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Vec })),
                    "match-intent-request.query-params.geo-location.latitude" => Some(("matchIntentRequest.queryParams.geoLocation.latitude", JsonTypeInfo { jtype: JsonType::Float, ctype: ComplexType::Pod })),
                    "match-intent-request.query-params.geo-location.longitude" => Some(("matchIntentRequest.queryParams.geoLocation.longitude", JsonTypeInfo { jtype: JsonType::Float, ctype: ComplexType::Pod })),
                    "match-intent-request.query-params.populate-data-store-connection-signals" => Some(("matchIntentRequest.queryParams.populateDataStoreConnectionSignals", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "match-intent-request.query-params.session-ttl" => Some(("matchIntentRequest.queryParams.sessionTtl", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "match-intent-request.query-params.time-zone" => Some(("matchIntentRequest.queryParams.timeZone", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "match-intent-request.query-params.webhook-headers" => Some(("matchIntentRequest.queryParams.webhookHeaders", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Map })),
                    "output-audio-config.audio-encoding" => Some(("outputAudioConfig.audioEncoding", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "output-audio-config.sample-rate-hertz" => Some(("outputAudioConfig.sampleRateHertz", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "output-audio-config.synthesize-speech-config.effects-profile-id" => Some(("outputAudioConfig.synthesizeSpeechConfig.effectsProfileId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Vec })),
                    "output-audio-config.synthesize-speech-config.pitch" => Some(("outputAudioConfig.synthesizeSpeechConfig.pitch", JsonTypeInfo { jtype: JsonType::Float, ctype: ComplexType::Pod })),
                    "output-audio-config.synthesize-speech-config.speaking-rate" => Some(("outputAudioConfig.synthesizeSpeechConfig.speakingRate", JsonTypeInfo { jtype: JsonType::Float, ctype: ComplexType::Pod })),
                    "output-audio-config.synthesize-speech-config.voice.name" => Some(("outputAudioConfig.synthesizeSpeechConfig.voice.name", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "output-audio-config.synthesize-speech-config.voice.ssml-gender" => Some(("outputAudioConfig.synthesizeSpeechConfig.voice.ssmlGender", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "output-audio-config.synthesize-speech-config.volume-gain-db" => Some(("outputAudioConfig.synthesizeSpeechConfig.volumeGainDb", JsonTypeInfo { jtype: JsonType::Float, ctype: ComplexType::Pod })),
                    _ => {
                        let suggestion = FieldCursor::did_you_mean(key, &vec!["analyze-query-text-sentiment", "audio", "audio-encoding", "barge-in-config", "channel", "confidence", "config", "current-page", "description", "digits", "disable-webhook", "display-name", "dtmf", "effects-profile-id", "enable-word-info", "event", "finish-digit", "flow-versions", "geo-location", "intent", "is-fallback", "labels", "language-code", "latitude", "longitude", "match", "match-intent-request", "match-type", "model", "model-variant", "name", "no-barge-in-duration", "opt-out-conformer-model-migration", "output-audio-config", "persist-parameter-changes", "phrase-hints", "pitch", "populate-data-store-connection-signals", "priority", "query-input", "query-params", "resolved-input", "sample-rate-hertz", "session-ttl", "single-utterance", "speaking-rate", "ssml-gender", "synthesize-speech-config", "text", "time-zone", "total-duration", "voice", "volume-gain-db", "webhook-headers"]);
                        err.issues.push(CLIError::Field(FieldError::Unknown(temp_cursor.to_string(), suggestion, value.map(|v| v.to_string()))));
                        None
                    }
                };
            if let Some((field_cursor_str, type_info)) = type_info {
                FieldCursor::from(field_cursor_str).set_json_value(&mut object, value.unwrap(), type_info, err, &temp_cursor);
            }
        }
        let mut request: api::GoogleCloudDialogflowCxV3FulfillIntentRequest = json::value::from_value(object).unwrap();
        let mut call = self.hub.projects().locations_agents_environments_sessions_fulfill_intent(request, opt.value_of("session").unwrap_or(""));
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

    async fn _projects_locations_agents_environments_sessions_match_intent(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
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
                    "persist-parameter-changes" => Some(("persistParameterChanges", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "query-input.audio.audio" => Some(("queryInput.audio.audio", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "query-input.audio.config.audio-encoding" => Some(("queryInput.audio.config.audioEncoding", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "query-input.audio.config.barge-in-config.no-barge-in-duration" => Some(("queryInput.audio.config.bargeInConfig.noBargeInDuration", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "query-input.audio.config.barge-in-config.total-duration" => Some(("queryInput.audio.config.bargeInConfig.totalDuration", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "query-input.audio.config.enable-word-info" => Some(("queryInput.audio.config.enableWordInfo", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "query-input.audio.config.model" => Some(("queryInput.audio.config.model", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "query-input.audio.config.model-variant" => Some(("queryInput.audio.config.modelVariant", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "query-input.audio.config.opt-out-conformer-model-migration" => Some(("queryInput.audio.config.optOutConformerModelMigration", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "query-input.audio.config.phrase-hints" => Some(("queryInput.audio.config.phraseHints", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Vec })),
                    "query-input.audio.config.sample-rate-hertz" => Some(("queryInput.audio.config.sampleRateHertz", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "query-input.audio.config.single-utterance" => Some(("queryInput.audio.config.singleUtterance", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "query-input.dtmf.digits" => Some(("queryInput.dtmf.digits", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "query-input.dtmf.finish-digit" => Some(("queryInput.dtmf.finishDigit", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "query-input.event.event" => Some(("queryInput.event.event", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "query-input.intent.intent" => Some(("queryInput.intent.intent", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "query-input.language-code" => Some(("queryInput.languageCode", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "query-input.text.text" => Some(("queryInput.text.text", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "query-params.analyze-query-text-sentiment" => Some(("queryParams.analyzeQueryTextSentiment", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "query-params.channel" => Some(("queryParams.channel", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "query-params.current-page" => Some(("queryParams.currentPage", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "query-params.disable-webhook" => Some(("queryParams.disableWebhook", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "query-params.flow-versions" => Some(("queryParams.flowVersions", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Vec })),
                    "query-params.geo-location.latitude" => Some(("queryParams.geoLocation.latitude", JsonTypeInfo { jtype: JsonType::Float, ctype: ComplexType::Pod })),
                    "query-params.geo-location.longitude" => Some(("queryParams.geoLocation.longitude", JsonTypeInfo { jtype: JsonType::Float, ctype: ComplexType::Pod })),
                    "query-params.populate-data-store-connection-signals" => Some(("queryParams.populateDataStoreConnectionSignals", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "query-params.session-ttl" => Some(("queryParams.sessionTtl", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "query-params.time-zone" => Some(("queryParams.timeZone", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "query-params.webhook-headers" => Some(("queryParams.webhookHeaders", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Map })),
                    _ => {
                        let suggestion = FieldCursor::did_you_mean(key, &vec!["analyze-query-text-sentiment", "audio", "audio-encoding", "barge-in-config", "channel", "config", "current-page", "digits", "disable-webhook", "dtmf", "enable-word-info", "event", "finish-digit", "flow-versions", "geo-location", "intent", "language-code", "latitude", "longitude", "model", "model-variant", "no-barge-in-duration", "opt-out-conformer-model-migration", "persist-parameter-changes", "phrase-hints", "populate-data-store-connection-signals", "query-input", "query-params", "sample-rate-hertz", "session-ttl", "single-utterance", "text", "time-zone", "total-duration", "webhook-headers"]);
                        err.issues.push(CLIError::Field(FieldError::Unknown(temp_cursor.to_string(), suggestion, value.map(|v| v.to_string()))));
                        None
                    }
                };
            if let Some((field_cursor_str, type_info)) = type_info {
                FieldCursor::from(field_cursor_str).set_json_value(&mut object, value.unwrap(), type_info, err, &temp_cursor);
            }
        }
        let mut request: api::GoogleCloudDialogflowCxV3MatchIntentRequest = json::value::from_value(object).unwrap();
        let mut call = self.hub.projects().locations_agents_environments_sessions_match_intent(request, opt.value_of("session").unwrap_or(""));
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

    async fn _projects_locations_agents_environments_sessions_server_streaming_detect_intent(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
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
                    "output-audio-config.audio-encoding" => Some(("outputAudioConfig.audioEncoding", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "output-audio-config.sample-rate-hertz" => Some(("outputAudioConfig.sampleRateHertz", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "output-audio-config.synthesize-speech-config.effects-profile-id" => Some(("outputAudioConfig.synthesizeSpeechConfig.effectsProfileId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Vec })),
                    "output-audio-config.synthesize-speech-config.pitch" => Some(("outputAudioConfig.synthesizeSpeechConfig.pitch", JsonTypeInfo { jtype: JsonType::Float, ctype: ComplexType::Pod })),
                    "output-audio-config.synthesize-speech-config.speaking-rate" => Some(("outputAudioConfig.synthesizeSpeechConfig.speakingRate", JsonTypeInfo { jtype: JsonType::Float, ctype: ComplexType::Pod })),
                    "output-audio-config.synthesize-speech-config.voice.name" => Some(("outputAudioConfig.synthesizeSpeechConfig.voice.name", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "output-audio-config.synthesize-speech-config.voice.ssml-gender" => Some(("outputAudioConfig.synthesizeSpeechConfig.voice.ssmlGender", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "output-audio-config.synthesize-speech-config.volume-gain-db" => Some(("outputAudioConfig.synthesizeSpeechConfig.volumeGainDb", JsonTypeInfo { jtype: JsonType::Float, ctype: ComplexType::Pod })),
                    "query-input.audio.audio" => Some(("queryInput.audio.audio", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "query-input.audio.config.audio-encoding" => Some(("queryInput.audio.config.audioEncoding", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "query-input.audio.config.barge-in-config.no-barge-in-duration" => Some(("queryInput.audio.config.bargeInConfig.noBargeInDuration", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "query-input.audio.config.barge-in-config.total-duration" => Some(("queryInput.audio.config.bargeInConfig.totalDuration", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "query-input.audio.config.enable-word-info" => Some(("queryInput.audio.config.enableWordInfo", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "query-input.audio.config.model" => Some(("queryInput.audio.config.model", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "query-input.audio.config.model-variant" => Some(("queryInput.audio.config.modelVariant", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "query-input.audio.config.opt-out-conformer-model-migration" => Some(("queryInput.audio.config.optOutConformerModelMigration", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "query-input.audio.config.phrase-hints" => Some(("queryInput.audio.config.phraseHints", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Vec })),
                    "query-input.audio.config.sample-rate-hertz" => Some(("queryInput.audio.config.sampleRateHertz", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "query-input.audio.config.single-utterance" => Some(("queryInput.audio.config.singleUtterance", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "query-input.dtmf.digits" => Some(("queryInput.dtmf.digits", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "query-input.dtmf.finish-digit" => Some(("queryInput.dtmf.finishDigit", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "query-input.event.event" => Some(("queryInput.event.event", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "query-input.intent.intent" => Some(("queryInput.intent.intent", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "query-input.language-code" => Some(("queryInput.languageCode", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "query-input.text.text" => Some(("queryInput.text.text", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "query-params.analyze-query-text-sentiment" => Some(("queryParams.analyzeQueryTextSentiment", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "query-params.channel" => Some(("queryParams.channel", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "query-params.current-page" => Some(("queryParams.currentPage", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "query-params.disable-webhook" => Some(("queryParams.disableWebhook", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "query-params.flow-versions" => Some(("queryParams.flowVersions", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Vec })),
                    "query-params.geo-location.latitude" => Some(("queryParams.geoLocation.latitude", JsonTypeInfo { jtype: JsonType::Float, ctype: ComplexType::Pod })),
                    "query-params.geo-location.longitude" => Some(("queryParams.geoLocation.longitude", JsonTypeInfo { jtype: JsonType::Float, ctype: ComplexType::Pod })),
                    "query-params.populate-data-store-connection-signals" => Some(("queryParams.populateDataStoreConnectionSignals", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "query-params.session-ttl" => Some(("queryParams.sessionTtl", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "query-params.time-zone" => Some(("queryParams.timeZone", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "query-params.webhook-headers" => Some(("queryParams.webhookHeaders", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Map })),
                    _ => {
                        let suggestion = FieldCursor::did_you_mean(key, &vec!["analyze-query-text-sentiment", "audio", "audio-encoding", "barge-in-config", "channel", "config", "current-page", "digits", "disable-webhook", "dtmf", "effects-profile-id", "enable-word-info", "event", "finish-digit", "flow-versions", "geo-location", "intent", "language-code", "latitude", "longitude", "model", "model-variant", "name", "no-barge-in-duration", "opt-out-conformer-model-migration", "output-audio-config", "phrase-hints", "pitch", "populate-data-store-connection-signals", "query-input", "query-params", "sample-rate-hertz", "session-ttl", "single-utterance", "speaking-rate", "ssml-gender", "synthesize-speech-config", "text", "time-zone", "total-duration", "voice", "volume-gain-db", "webhook-headers"]);
                        err.issues.push(CLIError::Field(FieldError::Unknown(temp_cursor.to_string(), suggestion, value.map(|v| v.to_string()))));
                        None
                    }
                };
            if let Some((field_cursor_str, type_info)) = type_info {
                FieldCursor::from(field_cursor_str).set_json_value(&mut object, value.unwrap(), type_info, err, &temp_cursor);
            }
        }
        let mut request: api::GoogleCloudDialogflowCxV3DetectIntentRequest = json::value::from_value(object).unwrap();
        let mut call = self.hub.projects().locations_agents_environments_sessions_server_streaming_detect_intent(request, opt.value_of("session").unwrap_or(""));
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

    async fn _projects_locations_agents_export(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
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
                    "agent-uri" => Some(("agentUri", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "data-format" => Some(("dataFormat", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "environment" => Some(("environment", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "git-destination.commit-message" => Some(("gitDestination.commitMessage", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "git-destination.tracking-branch" => Some(("gitDestination.trackingBranch", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "include-bigquery-export-settings" => Some(("includeBigqueryExportSettings", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    _ => {
                        let suggestion = FieldCursor::did_you_mean(key, &vec!["agent-uri", "commit-message", "data-format", "environment", "git-destination", "include-bigquery-export-settings", "tracking-branch"]);
                        err.issues.push(CLIError::Field(FieldError::Unknown(temp_cursor.to_string(), suggestion, value.map(|v| v.to_string()))));
                        None
                    }
                };
            if let Some((field_cursor_str, type_info)) = type_info {
                FieldCursor::from(field_cursor_str).set_json_value(&mut object, value.unwrap(), type_info, err, &temp_cursor);
            }
        }
        let mut request: api::GoogleCloudDialogflowCxV3ExportAgentRequest = json::value::from_value(object).unwrap();
        let mut call = self.hub.projects().locations_agents_export(request, opt.value_of("name").unwrap_or(""));
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

    async fn _projects_locations_agents_flows_create(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
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
                    "advanced-settings.audio-export-gcs-destination.uri" => Some(("advancedSettings.audioExportGcsDestination.uri", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "advanced-settings.dtmf-settings.enabled" => Some(("advancedSettings.dtmfSettings.enabled", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "advanced-settings.dtmf-settings.endpointing-timeout-duration" => Some(("advancedSettings.dtmfSettings.endpointingTimeoutDuration", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "advanced-settings.dtmf-settings.finish-digit" => Some(("advancedSettings.dtmfSettings.finishDigit", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "advanced-settings.dtmf-settings.interdigit-timeout-duration" => Some(("advancedSettings.dtmfSettings.interdigitTimeoutDuration", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "advanced-settings.dtmf-settings.max-digits" => Some(("advancedSettings.dtmfSettings.maxDigits", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "advanced-settings.logging-settings.enable-interaction-logging" => Some(("advancedSettings.loggingSettings.enableInteractionLogging", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "advanced-settings.logging-settings.enable-stackdriver-logging" => Some(("advancedSettings.loggingSettings.enableStackdriverLogging", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "advanced-settings.speech-settings.endpointer-sensitivity" => Some(("advancedSettings.speechSettings.endpointerSensitivity", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "advanced-settings.speech-settings.models" => Some(("advancedSettings.speechSettings.models", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Map })),
                    "advanced-settings.speech-settings.no-speech-timeout" => Some(("advancedSettings.speechSettings.noSpeechTimeout", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "advanced-settings.speech-settings.use-timeout-based-endpointing" => Some(("advancedSettings.speechSettings.useTimeoutBasedEndpointing", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "description" => Some(("description", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "display-name" => Some(("displayName", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "knowledge-connector-settings.enabled" => Some(("knowledgeConnectorSettings.enabled", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "knowledge-connector-settings.target-flow" => Some(("knowledgeConnectorSettings.targetFlow", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "knowledge-connector-settings.target-page" => Some(("knowledgeConnectorSettings.targetPage", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "knowledge-connector-settings.trigger-fulfillment.advanced-settings.audio-export-gcs-destination.uri" => Some(("knowledgeConnectorSettings.triggerFulfillment.advancedSettings.audioExportGcsDestination.uri", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "knowledge-connector-settings.trigger-fulfillment.advanced-settings.dtmf-settings.enabled" => Some(("knowledgeConnectorSettings.triggerFulfillment.advancedSettings.dtmfSettings.enabled", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "knowledge-connector-settings.trigger-fulfillment.advanced-settings.dtmf-settings.endpointing-timeout-duration" => Some(("knowledgeConnectorSettings.triggerFulfillment.advancedSettings.dtmfSettings.endpointingTimeoutDuration", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "knowledge-connector-settings.trigger-fulfillment.advanced-settings.dtmf-settings.finish-digit" => Some(("knowledgeConnectorSettings.triggerFulfillment.advancedSettings.dtmfSettings.finishDigit", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "knowledge-connector-settings.trigger-fulfillment.advanced-settings.dtmf-settings.interdigit-timeout-duration" => Some(("knowledgeConnectorSettings.triggerFulfillment.advancedSettings.dtmfSettings.interdigitTimeoutDuration", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "knowledge-connector-settings.trigger-fulfillment.advanced-settings.dtmf-settings.max-digits" => Some(("knowledgeConnectorSettings.triggerFulfillment.advancedSettings.dtmfSettings.maxDigits", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "knowledge-connector-settings.trigger-fulfillment.advanced-settings.logging-settings.enable-interaction-logging" => Some(("knowledgeConnectorSettings.triggerFulfillment.advancedSettings.loggingSettings.enableInteractionLogging", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "knowledge-connector-settings.trigger-fulfillment.advanced-settings.logging-settings.enable-stackdriver-logging" => Some(("knowledgeConnectorSettings.triggerFulfillment.advancedSettings.loggingSettings.enableStackdriverLogging", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "knowledge-connector-settings.trigger-fulfillment.advanced-settings.speech-settings.endpointer-sensitivity" => Some(("knowledgeConnectorSettings.triggerFulfillment.advancedSettings.speechSettings.endpointerSensitivity", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "knowledge-connector-settings.trigger-fulfillment.advanced-settings.speech-settings.models" => Some(("knowledgeConnectorSettings.triggerFulfillment.advancedSettings.speechSettings.models", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Map })),
                    "knowledge-connector-settings.trigger-fulfillment.advanced-settings.speech-settings.no-speech-timeout" => Some(("knowledgeConnectorSettings.triggerFulfillment.advancedSettings.speechSettings.noSpeechTimeout", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "knowledge-connector-settings.trigger-fulfillment.advanced-settings.speech-settings.use-timeout-based-endpointing" => Some(("knowledgeConnectorSettings.triggerFulfillment.advancedSettings.speechSettings.useTimeoutBasedEndpointing", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "knowledge-connector-settings.trigger-fulfillment.enable-generative-fallback" => Some(("knowledgeConnectorSettings.triggerFulfillment.enableGenerativeFallback", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "knowledge-connector-settings.trigger-fulfillment.return-partial-responses" => Some(("knowledgeConnectorSettings.triggerFulfillment.returnPartialResponses", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "knowledge-connector-settings.trigger-fulfillment.tag" => Some(("knowledgeConnectorSettings.triggerFulfillment.tag", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "knowledge-connector-settings.trigger-fulfillment.webhook" => Some(("knowledgeConnectorSettings.triggerFulfillment.webhook", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "multi-language-settings.enable-multi-language-detection" => Some(("multiLanguageSettings.enableMultiLanguageDetection", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "multi-language-settings.supported-response-language-codes" => Some(("multiLanguageSettings.supportedResponseLanguageCodes", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Vec })),
                    "name" => Some(("name", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "nlu-settings.classification-threshold" => Some(("nluSettings.classificationThreshold", JsonTypeInfo { jtype: JsonType::Float, ctype: ComplexType::Pod })),
                    "nlu-settings.model-training-mode" => Some(("nluSettings.modelTrainingMode", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "nlu-settings.model-type" => Some(("nluSettings.modelType", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "transition-route-groups" => Some(("transitionRouteGroups", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Vec })),
                    _ => {
                        let suggestion = FieldCursor::did_you_mean(key, &vec!["advanced-settings", "audio-export-gcs-destination", "classification-threshold", "description", "display-name", "dtmf-settings", "enable-generative-fallback", "enable-interaction-logging", "enable-multi-language-detection", "enable-stackdriver-logging", "enabled", "endpointer-sensitivity", "endpointing-timeout-duration", "finish-digit", "interdigit-timeout-duration", "knowledge-connector-settings", "logging-settings", "max-digits", "model-training-mode", "model-type", "models", "multi-language-settings", "name", "nlu-settings", "no-speech-timeout", "return-partial-responses", "speech-settings", "supported-response-language-codes", "tag", "target-flow", "target-page", "transition-route-groups", "trigger-fulfillment", "uri", "use-timeout-based-endpointing", "webhook"]);
                        err.issues.push(CLIError::Field(FieldError::Unknown(temp_cursor.to_string(), suggestion, value.map(|v| v.to_string()))));
                        None
                    }
                };
            if let Some((field_cursor_str, type_info)) = type_info {
                FieldCursor::from(field_cursor_str).set_json_value(&mut object, value.unwrap(), type_info, err, &temp_cursor);
            }
        }
        let mut request: api::GoogleCloudDialogflowCxV3Flow = json::value::from_value(object).unwrap();
        let mut call = self.hub.projects().locations_agents_flows_create(request, opt.value_of("parent").unwrap_or(""));
        for parg in opt.values_of("v").map(|i|i.collect()).unwrap_or(Vec::new()).iter() {
            let (key, value) = parse_kv_arg(&*parg, err, false);
            match key {
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
                                                                           v.extend(["language-code"].iter().map(|v|*v));
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

    async fn _projects_locations_agents_flows_delete(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        let mut call = self.hub.projects().locations_agents_flows_delete(opt.value_of("name").unwrap_or(""));
        for parg in opt.values_of("v").map(|i|i.collect()).unwrap_or(Vec::new()).iter() {
            let (key, value) = parse_kv_arg(&*parg, err, false);
            match key {
                "force" => {
                    call = call.force(        value.map(|v| arg_from_str(v, err, "force", "boolean")).unwrap_or(false));
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
                                                                           v.extend(["force"].iter().map(|v|*v));
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

    async fn _projects_locations_agents_flows_export(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
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
                    "flow-uri" => Some(("flowUri", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "include-referenced-flows" => Some(("includeReferencedFlows", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    _ => {
                        let suggestion = FieldCursor::did_you_mean(key, &vec!["flow-uri", "include-referenced-flows"]);
                        err.issues.push(CLIError::Field(FieldError::Unknown(temp_cursor.to_string(), suggestion, value.map(|v| v.to_string()))));
                        None
                    }
                };
            if let Some((field_cursor_str, type_info)) = type_info {
                FieldCursor::from(field_cursor_str).set_json_value(&mut object, value.unwrap(), type_info, err, &temp_cursor);
            }
        }
        let mut request: api::GoogleCloudDialogflowCxV3ExportFlowRequest = json::value::from_value(object).unwrap();
        let mut call = self.hub.projects().locations_agents_flows_export(request, opt.value_of("name").unwrap_or(""));
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

    async fn _projects_locations_agents_flows_get(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        let mut call = self.hub.projects().locations_agents_flows_get(opt.value_of("name").unwrap_or(""));
        for parg in opt.values_of("v").map(|i|i.collect()).unwrap_or(Vec::new()).iter() {
            let (key, value) = parse_kv_arg(&*parg, err, false);
            match key {
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
                                                                           v.extend(["language-code"].iter().map(|v|*v));
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

    async fn _projects_locations_agents_flows_get_validation_result(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        let mut call = self.hub.projects().locations_agents_flows_get_validation_result(opt.value_of("name").unwrap_or(""));
        for parg in opt.values_of("v").map(|i|i.collect()).unwrap_or(Vec::new()).iter() {
            let (key, value) = parse_kv_arg(&*parg, err, false);
            match key {
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
                                                                           v.extend(["language-code"].iter().map(|v|*v));
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

    async fn _projects_locations_agents_flows_import(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
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
                    "flow-content" => Some(("flowContent", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "flow-import-strategy.global-import-strategy" => Some(("flowImportStrategy.globalImportStrategy", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "flow-uri" => Some(("flowUri", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "import-option" => Some(("importOption", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    _ => {
                        let suggestion = FieldCursor::did_you_mean(key, &vec!["flow-content", "flow-import-strategy", "flow-uri", "global-import-strategy", "import-option"]);
                        err.issues.push(CLIError::Field(FieldError::Unknown(temp_cursor.to_string(), suggestion, value.map(|v| v.to_string()))));
                        None
                    }
                };
            if let Some((field_cursor_str, type_info)) = type_info {
                FieldCursor::from(field_cursor_str).set_json_value(&mut object, value.unwrap(), type_info, err, &temp_cursor);
            }
        }
        let mut request: api::GoogleCloudDialogflowCxV3ImportFlowRequest = json::value::from_value(object).unwrap();
        let mut call = self.hub.projects().locations_agents_flows_import(request, opt.value_of("parent").unwrap_or(""));
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

    async fn _projects_locations_agents_flows_list(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        let mut call = self.hub.projects().locations_agents_flows_list(opt.value_of("parent").unwrap_or(""));
        for parg in opt.values_of("v").map(|i|i.collect()).unwrap_or(Vec::new()).iter() {
            let (key, value) = parse_kv_arg(&*parg, err, false);
            match key {
                "page-token" => {
                    call = call.page_token(value.unwrap_or(""));
                },
                "page-size" => {
                    call = call.page_size(        value.map(|v| arg_from_str(v, err, "page-size", "int32")).unwrap_or(-0));
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
                                                                           v.extend(["language-code", "page-size", "page-token"].iter().map(|v|*v));
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

    async fn _projects_locations_agents_flows_pages_create(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
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
                    "advanced-settings.audio-export-gcs-destination.uri" => Some(("advancedSettings.audioExportGcsDestination.uri", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "advanced-settings.dtmf-settings.enabled" => Some(("advancedSettings.dtmfSettings.enabled", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "advanced-settings.dtmf-settings.endpointing-timeout-duration" => Some(("advancedSettings.dtmfSettings.endpointingTimeoutDuration", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "advanced-settings.dtmf-settings.finish-digit" => Some(("advancedSettings.dtmfSettings.finishDigit", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "advanced-settings.dtmf-settings.interdigit-timeout-duration" => Some(("advancedSettings.dtmfSettings.interdigitTimeoutDuration", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "advanced-settings.dtmf-settings.max-digits" => Some(("advancedSettings.dtmfSettings.maxDigits", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "advanced-settings.logging-settings.enable-interaction-logging" => Some(("advancedSettings.loggingSettings.enableInteractionLogging", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "advanced-settings.logging-settings.enable-stackdriver-logging" => Some(("advancedSettings.loggingSettings.enableStackdriverLogging", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "advanced-settings.speech-settings.endpointer-sensitivity" => Some(("advancedSettings.speechSettings.endpointerSensitivity", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "advanced-settings.speech-settings.models" => Some(("advancedSettings.speechSettings.models", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Map })),
                    "advanced-settings.speech-settings.no-speech-timeout" => Some(("advancedSettings.speechSettings.noSpeechTimeout", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "advanced-settings.speech-settings.use-timeout-based-endpointing" => Some(("advancedSettings.speechSettings.useTimeoutBasedEndpointing", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "description" => Some(("description", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "display-name" => Some(("displayName", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "entry-fulfillment.advanced-settings.audio-export-gcs-destination.uri" => Some(("entryFulfillment.advancedSettings.audioExportGcsDestination.uri", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "entry-fulfillment.advanced-settings.dtmf-settings.enabled" => Some(("entryFulfillment.advancedSettings.dtmfSettings.enabled", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "entry-fulfillment.advanced-settings.dtmf-settings.endpointing-timeout-duration" => Some(("entryFulfillment.advancedSettings.dtmfSettings.endpointingTimeoutDuration", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "entry-fulfillment.advanced-settings.dtmf-settings.finish-digit" => Some(("entryFulfillment.advancedSettings.dtmfSettings.finishDigit", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "entry-fulfillment.advanced-settings.dtmf-settings.interdigit-timeout-duration" => Some(("entryFulfillment.advancedSettings.dtmfSettings.interdigitTimeoutDuration", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "entry-fulfillment.advanced-settings.dtmf-settings.max-digits" => Some(("entryFulfillment.advancedSettings.dtmfSettings.maxDigits", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "entry-fulfillment.advanced-settings.logging-settings.enable-interaction-logging" => Some(("entryFulfillment.advancedSettings.loggingSettings.enableInteractionLogging", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "entry-fulfillment.advanced-settings.logging-settings.enable-stackdriver-logging" => Some(("entryFulfillment.advancedSettings.loggingSettings.enableStackdriverLogging", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "entry-fulfillment.advanced-settings.speech-settings.endpointer-sensitivity" => Some(("entryFulfillment.advancedSettings.speechSettings.endpointerSensitivity", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "entry-fulfillment.advanced-settings.speech-settings.models" => Some(("entryFulfillment.advancedSettings.speechSettings.models", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Map })),
                    "entry-fulfillment.advanced-settings.speech-settings.no-speech-timeout" => Some(("entryFulfillment.advancedSettings.speechSettings.noSpeechTimeout", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "entry-fulfillment.advanced-settings.speech-settings.use-timeout-based-endpointing" => Some(("entryFulfillment.advancedSettings.speechSettings.useTimeoutBasedEndpointing", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "entry-fulfillment.enable-generative-fallback" => Some(("entryFulfillment.enableGenerativeFallback", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "entry-fulfillment.return-partial-responses" => Some(("entryFulfillment.returnPartialResponses", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "entry-fulfillment.tag" => Some(("entryFulfillment.tag", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "entry-fulfillment.webhook" => Some(("entryFulfillment.webhook", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "knowledge-connector-settings.enabled" => Some(("knowledgeConnectorSettings.enabled", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "knowledge-connector-settings.target-flow" => Some(("knowledgeConnectorSettings.targetFlow", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "knowledge-connector-settings.target-page" => Some(("knowledgeConnectorSettings.targetPage", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "knowledge-connector-settings.trigger-fulfillment.advanced-settings.audio-export-gcs-destination.uri" => Some(("knowledgeConnectorSettings.triggerFulfillment.advancedSettings.audioExportGcsDestination.uri", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "knowledge-connector-settings.trigger-fulfillment.advanced-settings.dtmf-settings.enabled" => Some(("knowledgeConnectorSettings.triggerFulfillment.advancedSettings.dtmfSettings.enabled", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "knowledge-connector-settings.trigger-fulfillment.advanced-settings.dtmf-settings.endpointing-timeout-duration" => Some(("knowledgeConnectorSettings.triggerFulfillment.advancedSettings.dtmfSettings.endpointingTimeoutDuration", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "knowledge-connector-settings.trigger-fulfillment.advanced-settings.dtmf-settings.finish-digit" => Some(("knowledgeConnectorSettings.triggerFulfillment.advancedSettings.dtmfSettings.finishDigit", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "knowledge-connector-settings.trigger-fulfillment.advanced-settings.dtmf-settings.interdigit-timeout-duration" => Some(("knowledgeConnectorSettings.triggerFulfillment.advancedSettings.dtmfSettings.interdigitTimeoutDuration", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "knowledge-connector-settings.trigger-fulfillment.advanced-settings.dtmf-settings.max-digits" => Some(("knowledgeConnectorSettings.triggerFulfillment.advancedSettings.dtmfSettings.maxDigits", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "knowledge-connector-settings.trigger-fulfillment.advanced-settings.logging-settings.enable-interaction-logging" => Some(("knowledgeConnectorSettings.triggerFulfillment.advancedSettings.loggingSettings.enableInteractionLogging", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "knowledge-connector-settings.trigger-fulfillment.advanced-settings.logging-settings.enable-stackdriver-logging" => Some(("knowledgeConnectorSettings.triggerFulfillment.advancedSettings.loggingSettings.enableStackdriverLogging", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "knowledge-connector-settings.trigger-fulfillment.advanced-settings.speech-settings.endpointer-sensitivity" => Some(("knowledgeConnectorSettings.triggerFulfillment.advancedSettings.speechSettings.endpointerSensitivity", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "knowledge-connector-settings.trigger-fulfillment.advanced-settings.speech-settings.models" => Some(("knowledgeConnectorSettings.triggerFulfillment.advancedSettings.speechSettings.models", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Map })),
                    "knowledge-connector-settings.trigger-fulfillment.advanced-settings.speech-settings.no-speech-timeout" => Some(("knowledgeConnectorSettings.triggerFulfillment.advancedSettings.speechSettings.noSpeechTimeout", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "knowledge-connector-settings.trigger-fulfillment.advanced-settings.speech-settings.use-timeout-based-endpointing" => Some(("knowledgeConnectorSettings.triggerFulfillment.advancedSettings.speechSettings.useTimeoutBasedEndpointing", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "knowledge-connector-settings.trigger-fulfillment.enable-generative-fallback" => Some(("knowledgeConnectorSettings.triggerFulfillment.enableGenerativeFallback", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "knowledge-connector-settings.trigger-fulfillment.return-partial-responses" => Some(("knowledgeConnectorSettings.triggerFulfillment.returnPartialResponses", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "knowledge-connector-settings.trigger-fulfillment.tag" => Some(("knowledgeConnectorSettings.triggerFulfillment.tag", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "knowledge-connector-settings.trigger-fulfillment.webhook" => Some(("knowledgeConnectorSettings.triggerFulfillment.webhook", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "name" => Some(("name", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "transition-route-groups" => Some(("transitionRouteGroups", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Vec })),
                    _ => {
                        let suggestion = FieldCursor::did_you_mean(key, &vec!["advanced-settings", "audio-export-gcs-destination", "description", "display-name", "dtmf-settings", "enable-generative-fallback", "enable-interaction-logging", "enable-stackdriver-logging", "enabled", "endpointer-sensitivity", "endpointing-timeout-duration", "entry-fulfillment", "finish-digit", "interdigit-timeout-duration", "knowledge-connector-settings", "logging-settings", "max-digits", "models", "name", "no-speech-timeout", "return-partial-responses", "speech-settings", "tag", "target-flow", "target-page", "transition-route-groups", "trigger-fulfillment", "uri", "use-timeout-based-endpointing", "webhook"]);
                        err.issues.push(CLIError::Field(FieldError::Unknown(temp_cursor.to_string(), suggestion, value.map(|v| v.to_string()))));
                        None
                    }
                };
            if let Some((field_cursor_str, type_info)) = type_info {
                FieldCursor::from(field_cursor_str).set_json_value(&mut object, value.unwrap(), type_info, err, &temp_cursor);
            }
        }
        let mut request: api::GoogleCloudDialogflowCxV3Page = json::value::from_value(object).unwrap();
        let mut call = self.hub.projects().locations_agents_flows_pages_create(request, opt.value_of("parent").unwrap_or(""));
        for parg in opt.values_of("v").map(|i|i.collect()).unwrap_or(Vec::new()).iter() {
            let (key, value) = parse_kv_arg(&*parg, err, false);
            match key {
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
                                                                           v.extend(["language-code"].iter().map(|v|*v));
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

    async fn _projects_locations_agents_flows_pages_delete(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        let mut call = self.hub.projects().locations_agents_flows_pages_delete(opt.value_of("name").unwrap_or(""));
        for parg in opt.values_of("v").map(|i|i.collect()).unwrap_or(Vec::new()).iter() {
            let (key, value) = parse_kv_arg(&*parg, err, false);
            match key {
                "force" => {
                    call = call.force(        value.map(|v| arg_from_str(v, err, "force", "boolean")).unwrap_or(false));
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
                                                                           v.extend(["force"].iter().map(|v|*v));
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

    async fn _projects_locations_agents_flows_pages_get(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        let mut call = self.hub.projects().locations_agents_flows_pages_get(opt.value_of("name").unwrap_or(""));
        for parg in opt.values_of("v").map(|i|i.collect()).unwrap_or(Vec::new()).iter() {
            let (key, value) = parse_kv_arg(&*parg, err, false);
            match key {
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
                                                                           v.extend(["language-code"].iter().map(|v|*v));
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

    async fn _projects_locations_agents_flows_pages_list(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        let mut call = self.hub.projects().locations_agents_flows_pages_list(opt.value_of("parent").unwrap_or(""));
        for parg in opt.values_of("v").map(|i|i.collect()).unwrap_or(Vec::new()).iter() {
            let (key, value) = parse_kv_arg(&*parg, err, false);
            match key {
                "page-token" => {
                    call = call.page_token(value.unwrap_or(""));
                },
                "page-size" => {
                    call = call.page_size(        value.map(|v| arg_from_str(v, err, "page-size", "int32")).unwrap_or(-0));
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
                                                                           v.extend(["language-code", "page-size", "page-token"].iter().map(|v|*v));
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

    async fn _projects_locations_agents_flows_pages_patch(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
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
                    "advanced-settings.audio-export-gcs-destination.uri" => Some(("advancedSettings.audioExportGcsDestination.uri", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "advanced-settings.dtmf-settings.enabled" => Some(("advancedSettings.dtmfSettings.enabled", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "advanced-settings.dtmf-settings.endpointing-timeout-duration" => Some(("advancedSettings.dtmfSettings.endpointingTimeoutDuration", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "advanced-settings.dtmf-settings.finish-digit" => Some(("advancedSettings.dtmfSettings.finishDigit", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "advanced-settings.dtmf-settings.interdigit-timeout-duration" => Some(("advancedSettings.dtmfSettings.interdigitTimeoutDuration", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "advanced-settings.dtmf-settings.max-digits" => Some(("advancedSettings.dtmfSettings.maxDigits", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "advanced-settings.logging-settings.enable-interaction-logging" => Some(("advancedSettings.loggingSettings.enableInteractionLogging", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "advanced-settings.logging-settings.enable-stackdriver-logging" => Some(("advancedSettings.loggingSettings.enableStackdriverLogging", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "advanced-settings.speech-settings.endpointer-sensitivity" => Some(("advancedSettings.speechSettings.endpointerSensitivity", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "advanced-settings.speech-settings.models" => Some(("advancedSettings.speechSettings.models", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Map })),
                    "advanced-settings.speech-settings.no-speech-timeout" => Some(("advancedSettings.speechSettings.noSpeechTimeout", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "advanced-settings.speech-settings.use-timeout-based-endpointing" => Some(("advancedSettings.speechSettings.useTimeoutBasedEndpointing", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "description" => Some(("description", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "display-name" => Some(("displayName", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "entry-fulfillment.advanced-settings.audio-export-gcs-destination.uri" => Some(("entryFulfillment.advancedSettings.audioExportGcsDestination.uri", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "entry-fulfillment.advanced-settings.dtmf-settings.enabled" => Some(("entryFulfillment.advancedSettings.dtmfSettings.enabled", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "entry-fulfillment.advanced-settings.dtmf-settings.endpointing-timeout-duration" => Some(("entryFulfillment.advancedSettings.dtmfSettings.endpointingTimeoutDuration", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "entry-fulfillment.advanced-settings.dtmf-settings.finish-digit" => Some(("entryFulfillment.advancedSettings.dtmfSettings.finishDigit", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "entry-fulfillment.advanced-settings.dtmf-settings.interdigit-timeout-duration" => Some(("entryFulfillment.advancedSettings.dtmfSettings.interdigitTimeoutDuration", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "entry-fulfillment.advanced-settings.dtmf-settings.max-digits" => Some(("entryFulfillment.advancedSettings.dtmfSettings.maxDigits", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "entry-fulfillment.advanced-settings.logging-settings.enable-interaction-logging" => Some(("entryFulfillment.advancedSettings.loggingSettings.enableInteractionLogging", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "entry-fulfillment.advanced-settings.logging-settings.enable-stackdriver-logging" => Some(("entryFulfillment.advancedSettings.loggingSettings.enableStackdriverLogging", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "entry-fulfillment.advanced-settings.speech-settings.endpointer-sensitivity" => Some(("entryFulfillment.advancedSettings.speechSettings.endpointerSensitivity", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "entry-fulfillment.advanced-settings.speech-settings.models" => Some(("entryFulfillment.advancedSettings.speechSettings.models", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Map })),
                    "entry-fulfillment.advanced-settings.speech-settings.no-speech-timeout" => Some(("entryFulfillment.advancedSettings.speechSettings.noSpeechTimeout", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "entry-fulfillment.advanced-settings.speech-settings.use-timeout-based-endpointing" => Some(("entryFulfillment.advancedSettings.speechSettings.useTimeoutBasedEndpointing", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "entry-fulfillment.enable-generative-fallback" => Some(("entryFulfillment.enableGenerativeFallback", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "entry-fulfillment.return-partial-responses" => Some(("entryFulfillment.returnPartialResponses", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "entry-fulfillment.tag" => Some(("entryFulfillment.tag", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "entry-fulfillment.webhook" => Some(("entryFulfillment.webhook", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "knowledge-connector-settings.enabled" => Some(("knowledgeConnectorSettings.enabled", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "knowledge-connector-settings.target-flow" => Some(("knowledgeConnectorSettings.targetFlow", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "knowledge-connector-settings.target-page" => Some(("knowledgeConnectorSettings.targetPage", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "knowledge-connector-settings.trigger-fulfillment.advanced-settings.audio-export-gcs-destination.uri" => Some(("knowledgeConnectorSettings.triggerFulfillment.advancedSettings.audioExportGcsDestination.uri", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "knowledge-connector-settings.trigger-fulfillment.advanced-settings.dtmf-settings.enabled" => Some(("knowledgeConnectorSettings.triggerFulfillment.advancedSettings.dtmfSettings.enabled", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "knowledge-connector-settings.trigger-fulfillment.advanced-settings.dtmf-settings.endpointing-timeout-duration" => Some(("knowledgeConnectorSettings.triggerFulfillment.advancedSettings.dtmfSettings.endpointingTimeoutDuration", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "knowledge-connector-settings.trigger-fulfillment.advanced-settings.dtmf-settings.finish-digit" => Some(("knowledgeConnectorSettings.triggerFulfillment.advancedSettings.dtmfSettings.finishDigit", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "knowledge-connector-settings.trigger-fulfillment.advanced-settings.dtmf-settings.interdigit-timeout-duration" => Some(("knowledgeConnectorSettings.triggerFulfillment.advancedSettings.dtmfSettings.interdigitTimeoutDuration", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "knowledge-connector-settings.trigger-fulfillment.advanced-settings.dtmf-settings.max-digits" => Some(("knowledgeConnectorSettings.triggerFulfillment.advancedSettings.dtmfSettings.maxDigits", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "knowledge-connector-settings.trigger-fulfillment.advanced-settings.logging-settings.enable-interaction-logging" => Some(("knowledgeConnectorSettings.triggerFulfillment.advancedSettings.loggingSettings.enableInteractionLogging", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "knowledge-connector-settings.trigger-fulfillment.advanced-settings.logging-settings.enable-stackdriver-logging" => Some(("knowledgeConnectorSettings.triggerFulfillment.advancedSettings.loggingSettings.enableStackdriverLogging", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "knowledge-connector-settings.trigger-fulfillment.advanced-settings.speech-settings.endpointer-sensitivity" => Some(("knowledgeConnectorSettings.triggerFulfillment.advancedSettings.speechSettings.endpointerSensitivity", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "knowledge-connector-settings.trigger-fulfillment.advanced-settings.speech-settings.models" => Some(("knowledgeConnectorSettings.triggerFulfillment.advancedSettings.speechSettings.models", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Map })),
                    "knowledge-connector-settings.trigger-fulfillment.advanced-settings.speech-settings.no-speech-timeout" => Some(("knowledgeConnectorSettings.triggerFulfillment.advancedSettings.speechSettings.noSpeechTimeout", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "knowledge-connector-settings.trigger-fulfillment.advanced-settings.speech-settings.use-timeout-based-endpointing" => Some(("knowledgeConnectorSettings.triggerFulfillment.advancedSettings.speechSettings.useTimeoutBasedEndpointing", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "knowledge-connector-settings.trigger-fulfillment.enable-generative-fallback" => Some(("knowledgeConnectorSettings.triggerFulfillment.enableGenerativeFallback", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "knowledge-connector-settings.trigger-fulfillment.return-partial-responses" => Some(("knowledgeConnectorSettings.triggerFulfillment.returnPartialResponses", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "knowledge-connector-settings.trigger-fulfillment.tag" => Some(("knowledgeConnectorSettings.triggerFulfillment.tag", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "knowledge-connector-settings.trigger-fulfillment.webhook" => Some(("knowledgeConnectorSettings.triggerFulfillment.webhook", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "name" => Some(("name", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "transition-route-groups" => Some(("transitionRouteGroups", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Vec })),
                    _ => {
                        let suggestion = FieldCursor::did_you_mean(key, &vec!["advanced-settings", "audio-export-gcs-destination", "description", "display-name", "dtmf-settings", "enable-generative-fallback", "enable-interaction-logging", "enable-stackdriver-logging", "enabled", "endpointer-sensitivity", "endpointing-timeout-duration", "entry-fulfillment", "finish-digit", "interdigit-timeout-duration", "knowledge-connector-settings", "logging-settings", "max-digits", "models", "name", "no-speech-timeout", "return-partial-responses", "speech-settings", "tag", "target-flow", "target-page", "transition-route-groups", "trigger-fulfillment", "uri", "use-timeout-based-endpointing", "webhook"]);
                        err.issues.push(CLIError::Field(FieldError::Unknown(temp_cursor.to_string(), suggestion, value.map(|v| v.to_string()))));
                        None
                    }
                };
            if let Some((field_cursor_str, type_info)) = type_info {
                FieldCursor::from(field_cursor_str).set_json_value(&mut object, value.unwrap(), type_info, err, &temp_cursor);
            }
        }
        let mut request: api::GoogleCloudDialogflowCxV3Page = json::value::from_value(object).unwrap();
        let mut call = self.hub.projects().locations_agents_flows_pages_patch(request, opt.value_of("name").unwrap_or(""));
        for parg in opt.values_of("v").map(|i|i.collect()).unwrap_or(Vec::new()).iter() {
            let (key, value) = parse_kv_arg(&*parg, err, false);
            match key {
                "update-mask" => {
                    call = call.update_mask(        value.map(|v| arg_from_str(v, err, "update-mask", "google-fieldmask")).unwrap_or(FieldMask::default()));
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
                                                                           v.extend(["language-code", "update-mask"].iter().map(|v|*v));
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

    async fn _projects_locations_agents_flows_patch(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
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
                    "advanced-settings.audio-export-gcs-destination.uri" => Some(("advancedSettings.audioExportGcsDestination.uri", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "advanced-settings.dtmf-settings.enabled" => Some(("advancedSettings.dtmfSettings.enabled", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "advanced-settings.dtmf-settings.endpointing-timeout-duration" => Some(("advancedSettings.dtmfSettings.endpointingTimeoutDuration", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "advanced-settings.dtmf-settings.finish-digit" => Some(("advancedSettings.dtmfSettings.finishDigit", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "advanced-settings.dtmf-settings.interdigit-timeout-duration" => Some(("advancedSettings.dtmfSettings.interdigitTimeoutDuration", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "advanced-settings.dtmf-settings.max-digits" => Some(("advancedSettings.dtmfSettings.maxDigits", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "advanced-settings.logging-settings.enable-interaction-logging" => Some(("advancedSettings.loggingSettings.enableInteractionLogging", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "advanced-settings.logging-settings.enable-stackdriver-logging" => Some(("advancedSettings.loggingSettings.enableStackdriverLogging", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "advanced-settings.speech-settings.endpointer-sensitivity" => Some(("advancedSettings.speechSettings.endpointerSensitivity", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "advanced-settings.speech-settings.models" => Some(("advancedSettings.speechSettings.models", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Map })),
                    "advanced-settings.speech-settings.no-speech-timeout" => Some(("advancedSettings.speechSettings.noSpeechTimeout", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "advanced-settings.speech-settings.use-timeout-based-endpointing" => Some(("advancedSettings.speechSettings.useTimeoutBasedEndpointing", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "description" => Some(("description", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "display-name" => Some(("displayName", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "knowledge-connector-settings.enabled" => Some(("knowledgeConnectorSettings.enabled", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "knowledge-connector-settings.target-flow" => Some(("knowledgeConnectorSettings.targetFlow", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "knowledge-connector-settings.target-page" => Some(("knowledgeConnectorSettings.targetPage", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "knowledge-connector-settings.trigger-fulfillment.advanced-settings.audio-export-gcs-destination.uri" => Some(("knowledgeConnectorSettings.triggerFulfillment.advancedSettings.audioExportGcsDestination.uri", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "knowledge-connector-settings.trigger-fulfillment.advanced-settings.dtmf-settings.enabled" => Some(("knowledgeConnectorSettings.triggerFulfillment.advancedSettings.dtmfSettings.enabled", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "knowledge-connector-settings.trigger-fulfillment.advanced-settings.dtmf-settings.endpointing-timeout-duration" => Some(("knowledgeConnectorSettings.triggerFulfillment.advancedSettings.dtmfSettings.endpointingTimeoutDuration", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "knowledge-connector-settings.trigger-fulfillment.advanced-settings.dtmf-settings.finish-digit" => Some(("knowledgeConnectorSettings.triggerFulfillment.advancedSettings.dtmfSettings.finishDigit", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "knowledge-connector-settings.trigger-fulfillment.advanced-settings.dtmf-settings.interdigit-timeout-duration" => Some(("knowledgeConnectorSettings.triggerFulfillment.advancedSettings.dtmfSettings.interdigitTimeoutDuration", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "knowledge-connector-settings.trigger-fulfillment.advanced-settings.dtmf-settings.max-digits" => Some(("knowledgeConnectorSettings.triggerFulfillment.advancedSettings.dtmfSettings.maxDigits", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "knowledge-connector-settings.trigger-fulfillment.advanced-settings.logging-settings.enable-interaction-logging" => Some(("knowledgeConnectorSettings.triggerFulfillment.advancedSettings.loggingSettings.enableInteractionLogging", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "knowledge-connector-settings.trigger-fulfillment.advanced-settings.logging-settings.enable-stackdriver-logging" => Some(("knowledgeConnectorSettings.triggerFulfillment.advancedSettings.loggingSettings.enableStackdriverLogging", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "knowledge-connector-settings.trigger-fulfillment.advanced-settings.speech-settings.endpointer-sensitivity" => Some(("knowledgeConnectorSettings.triggerFulfillment.advancedSettings.speechSettings.endpointerSensitivity", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "knowledge-connector-settings.trigger-fulfillment.advanced-settings.speech-settings.models" => Some(("knowledgeConnectorSettings.triggerFulfillment.advancedSettings.speechSettings.models", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Map })),
                    "knowledge-connector-settings.trigger-fulfillment.advanced-settings.speech-settings.no-speech-timeout" => Some(("knowledgeConnectorSettings.triggerFulfillment.advancedSettings.speechSettings.noSpeechTimeout", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "knowledge-connector-settings.trigger-fulfillment.advanced-settings.speech-settings.use-timeout-based-endpointing" => Some(("knowledgeConnectorSettings.triggerFulfillment.advancedSettings.speechSettings.useTimeoutBasedEndpointing", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "knowledge-connector-settings.trigger-fulfillment.enable-generative-fallback" => Some(("knowledgeConnectorSettings.triggerFulfillment.enableGenerativeFallback", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "knowledge-connector-settings.trigger-fulfillment.return-partial-responses" => Some(("knowledgeConnectorSettings.triggerFulfillment.returnPartialResponses", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "knowledge-connector-settings.trigger-fulfillment.tag" => Some(("knowledgeConnectorSettings.triggerFulfillment.tag", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "knowledge-connector-settings.trigger-fulfillment.webhook" => Some(("knowledgeConnectorSettings.triggerFulfillment.webhook", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "multi-language-settings.enable-multi-language-detection" => Some(("multiLanguageSettings.enableMultiLanguageDetection", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "multi-language-settings.supported-response-language-codes" => Some(("multiLanguageSettings.supportedResponseLanguageCodes", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Vec })),
                    "name" => Some(("name", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "nlu-settings.classification-threshold" => Some(("nluSettings.classificationThreshold", JsonTypeInfo { jtype: JsonType::Float, ctype: ComplexType::Pod })),
                    "nlu-settings.model-training-mode" => Some(("nluSettings.modelTrainingMode", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "nlu-settings.model-type" => Some(("nluSettings.modelType", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "transition-route-groups" => Some(("transitionRouteGroups", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Vec })),
                    _ => {
                        let suggestion = FieldCursor::did_you_mean(key, &vec!["advanced-settings", "audio-export-gcs-destination", "classification-threshold", "description", "display-name", "dtmf-settings", "enable-generative-fallback", "enable-interaction-logging", "enable-multi-language-detection", "enable-stackdriver-logging", "enabled", "endpointer-sensitivity", "endpointing-timeout-duration", "finish-digit", "interdigit-timeout-duration", "knowledge-connector-settings", "logging-settings", "max-digits", "model-training-mode", "model-type", "models", "multi-language-settings", "name", "nlu-settings", "no-speech-timeout", "return-partial-responses", "speech-settings", "supported-response-language-codes", "tag", "target-flow", "target-page", "transition-route-groups", "trigger-fulfillment", "uri", "use-timeout-based-endpointing", "webhook"]);
                        err.issues.push(CLIError::Field(FieldError::Unknown(temp_cursor.to_string(), suggestion, value.map(|v| v.to_string()))));
                        None
                    }
                };
            if let Some((field_cursor_str, type_info)) = type_info {
                FieldCursor::from(field_cursor_str).set_json_value(&mut object, value.unwrap(), type_info, err, &temp_cursor);
            }
        }
        let mut request: api::GoogleCloudDialogflowCxV3Flow = json::value::from_value(object).unwrap();
        let mut call = self.hub.projects().locations_agents_flows_patch(request, opt.value_of("name").unwrap_or(""));
        for parg in opt.values_of("v").map(|i|i.collect()).unwrap_or(Vec::new()).iter() {
            let (key, value) = parse_kv_arg(&*parg, err, false);
            match key {
                "update-mask" => {
                    call = call.update_mask(        value.map(|v| arg_from_str(v, err, "update-mask", "google-fieldmask")).unwrap_or(FieldMask::default()));
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
                                                                           v.extend(["language-code", "update-mask"].iter().map(|v|*v));
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

    async fn _projects_locations_agents_flows_train(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
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
        let mut request: api::GoogleCloudDialogflowCxV3TrainFlowRequest = json::value::from_value(object).unwrap();
        let mut call = self.hub.projects().locations_agents_flows_train(request, opt.value_of("name").unwrap_or(""));
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

    async fn _projects_locations_agents_flows_transition_route_groups_create(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
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
                    "display-name" => Some(("displayName", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "name" => Some(("name", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    _ => {
                        let suggestion = FieldCursor::did_you_mean(key, &vec!["display-name", "name"]);
                        err.issues.push(CLIError::Field(FieldError::Unknown(temp_cursor.to_string(), suggestion, value.map(|v| v.to_string()))));
                        None
                    }
                };
            if let Some((field_cursor_str, type_info)) = type_info {
                FieldCursor::from(field_cursor_str).set_json_value(&mut object, value.unwrap(), type_info, err, &temp_cursor);
            }
        }
        let mut request: api::GoogleCloudDialogflowCxV3TransitionRouteGroup = json::value::from_value(object).unwrap();
        let mut call = self.hub.projects().locations_agents_flows_transition_route_groups_create(request, opt.value_of("parent").unwrap_or(""));
        for parg in opt.values_of("v").map(|i|i.collect()).unwrap_or(Vec::new()).iter() {
            let (key, value) = parse_kv_arg(&*parg, err, false);
            match key {
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
                                                                           v.extend(["language-code"].iter().map(|v|*v));
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

    async fn _projects_locations_agents_flows_transition_route_groups_delete(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        let mut call = self.hub.projects().locations_agents_flows_transition_route_groups_delete(opt.value_of("name").unwrap_or(""));
        for parg in opt.values_of("v").map(|i|i.collect()).unwrap_or(Vec::new()).iter() {
            let (key, value) = parse_kv_arg(&*parg, err, false);
            match key {
                "force" => {
                    call = call.force(        value.map(|v| arg_from_str(v, err, "force", "boolean")).unwrap_or(false));
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
                                                                           v.extend(["force"].iter().map(|v|*v));
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

    async fn _projects_locations_agents_flows_transition_route_groups_get(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        let mut call = self.hub.projects().locations_agents_flows_transition_route_groups_get(opt.value_of("name").unwrap_or(""));
        for parg in opt.values_of("v").map(|i|i.collect()).unwrap_or(Vec::new()).iter() {
            let (key, value) = parse_kv_arg(&*parg, err, false);
            match key {
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
                                                                           v.extend(["language-code"].iter().map(|v|*v));
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

    async fn _projects_locations_agents_flows_transition_route_groups_list(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        let mut call = self.hub.projects().locations_agents_flows_transition_route_groups_list(opt.value_of("parent").unwrap_or(""));
        for parg in opt.values_of("v").map(|i|i.collect()).unwrap_or(Vec::new()).iter() {
            let (key, value) = parse_kv_arg(&*parg, err, false);
            match key {
                "page-token" => {
                    call = call.page_token(value.unwrap_or(""));
                },
                "page-size" => {
                    call = call.page_size(        value.map(|v| arg_from_str(v, err, "page-size", "int32")).unwrap_or(-0));
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
                                                                           v.extend(["language-code", "page-size", "page-token"].iter().map(|v|*v));
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

    async fn _projects_locations_agents_flows_transition_route_groups_patch(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
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
                    "display-name" => Some(("displayName", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "name" => Some(("name", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    _ => {
                        let suggestion = FieldCursor::did_you_mean(key, &vec!["display-name", "name"]);
                        err.issues.push(CLIError::Field(FieldError::Unknown(temp_cursor.to_string(), suggestion, value.map(|v| v.to_string()))));
                        None
                    }
                };
            if let Some((field_cursor_str, type_info)) = type_info {
                FieldCursor::from(field_cursor_str).set_json_value(&mut object, value.unwrap(), type_info, err, &temp_cursor);
            }
        }
        let mut request: api::GoogleCloudDialogflowCxV3TransitionRouteGroup = json::value::from_value(object).unwrap();
        let mut call = self.hub.projects().locations_agents_flows_transition_route_groups_patch(request, opt.value_of("name").unwrap_or(""));
        for parg in opt.values_of("v").map(|i|i.collect()).unwrap_or(Vec::new()).iter() {
            let (key, value) = parse_kv_arg(&*parg, err, false);
            match key {
                "update-mask" => {
                    call = call.update_mask(        value.map(|v| arg_from_str(v, err, "update-mask", "google-fieldmask")).unwrap_or(FieldMask::default()));
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
                                                                           v.extend(["language-code", "update-mask"].iter().map(|v|*v));
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

    async fn _projects_locations_agents_flows_validate(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
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
                    "language-code" => Some(("languageCode", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    _ => {
                        let suggestion = FieldCursor::did_you_mean(key, &vec!["language-code"]);
                        err.issues.push(CLIError::Field(FieldError::Unknown(temp_cursor.to_string(), suggestion, value.map(|v| v.to_string()))));
                        None
                    }
                };
            if let Some((field_cursor_str, type_info)) = type_info {
                FieldCursor::from(field_cursor_str).set_json_value(&mut object, value.unwrap(), type_info, err, &temp_cursor);
            }
        }
        let mut request: api::GoogleCloudDialogflowCxV3ValidateFlowRequest = json::value::from_value(object).unwrap();
        let mut call = self.hub.projects().locations_agents_flows_validate(request, opt.value_of("name").unwrap_or(""));
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

    async fn _projects_locations_agents_flows_versions_compare_versions(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
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
                    "language-code" => Some(("languageCode", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "target-version" => Some(("targetVersion", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    _ => {
                        let suggestion = FieldCursor::did_you_mean(key, &vec!["language-code", "target-version"]);
                        err.issues.push(CLIError::Field(FieldError::Unknown(temp_cursor.to_string(), suggestion, value.map(|v| v.to_string()))));
                        None
                    }
                };
            if let Some((field_cursor_str, type_info)) = type_info {
                FieldCursor::from(field_cursor_str).set_json_value(&mut object, value.unwrap(), type_info, err, &temp_cursor);
            }
        }
        let mut request: api::GoogleCloudDialogflowCxV3CompareVersionsRequest = json::value::from_value(object).unwrap();
        let mut call = self.hub.projects().locations_agents_flows_versions_compare_versions(request, opt.value_of("base-version").unwrap_or(""));
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

    async fn _projects_locations_agents_flows_versions_create(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
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
                    "create-time" => Some(("createTime", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "description" => Some(("description", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "display-name" => Some(("displayName", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "name" => Some(("name", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "nlu-settings.classification-threshold" => Some(("nluSettings.classificationThreshold", JsonTypeInfo { jtype: JsonType::Float, ctype: ComplexType::Pod })),
                    "nlu-settings.model-training-mode" => Some(("nluSettings.modelTrainingMode", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "nlu-settings.model-type" => Some(("nluSettings.modelType", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "state" => Some(("state", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    _ => {
                        let suggestion = FieldCursor::did_you_mean(key, &vec!["classification-threshold", "create-time", "description", "display-name", "model-training-mode", "model-type", "name", "nlu-settings", "state"]);
                        err.issues.push(CLIError::Field(FieldError::Unknown(temp_cursor.to_string(), suggestion, value.map(|v| v.to_string()))));
                        None
                    }
                };
            if let Some((field_cursor_str, type_info)) = type_info {
                FieldCursor::from(field_cursor_str).set_json_value(&mut object, value.unwrap(), type_info, err, &temp_cursor);
            }
        }
        let mut request: api::GoogleCloudDialogflowCxV3Version = json::value::from_value(object).unwrap();
        let mut call = self.hub.projects().locations_agents_flows_versions_create(request, opt.value_of("parent").unwrap_or(""));
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

    async fn _projects_locations_agents_flows_versions_delete(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        let mut call = self.hub.projects().locations_agents_flows_versions_delete(opt.value_of("name").unwrap_or(""));
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

    async fn _projects_locations_agents_flows_versions_get(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        let mut call = self.hub.projects().locations_agents_flows_versions_get(opt.value_of("name").unwrap_or(""));
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

    async fn _projects_locations_agents_flows_versions_list(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        let mut call = self.hub.projects().locations_agents_flows_versions_list(opt.value_of("parent").unwrap_or(""));
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

    async fn _projects_locations_agents_flows_versions_load(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
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
                    "allow-override-agent-resources" => Some(("allowOverrideAgentResources", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    _ => {
                        let suggestion = FieldCursor::did_you_mean(key, &vec!["allow-override-agent-resources"]);
                        err.issues.push(CLIError::Field(FieldError::Unknown(temp_cursor.to_string(), suggestion, value.map(|v| v.to_string()))));
                        None
                    }
                };
            if let Some((field_cursor_str, type_info)) = type_info {
                FieldCursor::from(field_cursor_str).set_json_value(&mut object, value.unwrap(), type_info, err, &temp_cursor);
            }
        }
        let mut request: api::GoogleCloudDialogflowCxV3LoadVersionRequest = json::value::from_value(object).unwrap();
        let mut call = self.hub.projects().locations_agents_flows_versions_load(request, opt.value_of("name").unwrap_or(""));
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

    async fn _projects_locations_agents_flows_versions_patch(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
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
                    "create-time" => Some(("createTime", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "description" => Some(("description", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "display-name" => Some(("displayName", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "name" => Some(("name", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "nlu-settings.classification-threshold" => Some(("nluSettings.classificationThreshold", JsonTypeInfo { jtype: JsonType::Float, ctype: ComplexType::Pod })),
                    "nlu-settings.model-training-mode" => Some(("nluSettings.modelTrainingMode", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "nlu-settings.model-type" => Some(("nluSettings.modelType", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "state" => Some(("state", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    _ => {
                        let suggestion = FieldCursor::did_you_mean(key, &vec!["classification-threshold", "create-time", "description", "display-name", "model-training-mode", "model-type", "name", "nlu-settings", "state"]);
                        err.issues.push(CLIError::Field(FieldError::Unknown(temp_cursor.to_string(), suggestion, value.map(|v| v.to_string()))));
                        None
                    }
                };
            if let Some((field_cursor_str, type_info)) = type_info {
                FieldCursor::from(field_cursor_str).set_json_value(&mut object, value.unwrap(), type_info, err, &temp_cursor);
            }
        }
        let mut request: api::GoogleCloudDialogflowCxV3Version = json::value::from_value(object).unwrap();
        let mut call = self.hub.projects().locations_agents_flows_versions_patch(request, opt.value_of("name").unwrap_or(""));
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

    async fn _projects_locations_agents_generators_create(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
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
                    "display-name" => Some(("displayName", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "name" => Some(("name", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "prompt-text.text" => Some(("promptText.text", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    _ => {
                        let suggestion = FieldCursor::did_you_mean(key, &vec!["display-name", "name", "prompt-text", "text"]);
                        err.issues.push(CLIError::Field(FieldError::Unknown(temp_cursor.to_string(), suggestion, value.map(|v| v.to_string()))));
                        None
                    }
                };
            if let Some((field_cursor_str, type_info)) = type_info {
                FieldCursor::from(field_cursor_str).set_json_value(&mut object, value.unwrap(), type_info, err, &temp_cursor);
            }
        }
        let mut request: api::GoogleCloudDialogflowCxV3Generator = json::value::from_value(object).unwrap();
        let mut call = self.hub.projects().locations_agents_generators_create(request, opt.value_of("parent").unwrap_or(""));
        for parg in opt.values_of("v").map(|i|i.collect()).unwrap_or(Vec::new()).iter() {
            let (key, value) = parse_kv_arg(&*parg, err, false);
            match key {
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
                                                                           v.extend(["language-code"].iter().map(|v|*v));
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

    async fn _projects_locations_agents_generators_delete(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        let mut call = self.hub.projects().locations_agents_generators_delete(opt.value_of("name").unwrap_or(""));
        for parg in opt.values_of("v").map(|i|i.collect()).unwrap_or(Vec::new()).iter() {
            let (key, value) = parse_kv_arg(&*parg, err, false);
            match key {
                "force" => {
                    call = call.force(        value.map(|v| arg_from_str(v, err, "force", "boolean")).unwrap_or(false));
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
                                                                           v.extend(["force"].iter().map(|v|*v));
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

    async fn _projects_locations_agents_generators_get(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        let mut call = self.hub.projects().locations_agents_generators_get(opt.value_of("name").unwrap_or(""));
        for parg in opt.values_of("v").map(|i|i.collect()).unwrap_or(Vec::new()).iter() {
            let (key, value) = parse_kv_arg(&*parg, err, false);
            match key {
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
                                                                           v.extend(["language-code"].iter().map(|v|*v));
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

    async fn _projects_locations_agents_generators_list(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        let mut call = self.hub.projects().locations_agents_generators_list(opt.value_of("parent").unwrap_or(""));
        for parg in opt.values_of("v").map(|i|i.collect()).unwrap_or(Vec::new()).iter() {
            let (key, value) = parse_kv_arg(&*parg, err, false);
            match key {
                "page-token" => {
                    call = call.page_token(value.unwrap_or(""));
                },
                "page-size" => {
                    call = call.page_size(        value.map(|v| arg_from_str(v, err, "page-size", "int32")).unwrap_or(-0));
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
                                                                           v.extend(["language-code", "page-size", "page-token"].iter().map(|v|*v));
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

    async fn _projects_locations_agents_generators_patch(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
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
                    "display-name" => Some(("displayName", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "name" => Some(("name", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "prompt-text.text" => Some(("promptText.text", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    _ => {
                        let suggestion = FieldCursor::did_you_mean(key, &vec!["display-name", "name", "prompt-text", "text"]);
                        err.issues.push(CLIError::Field(FieldError::Unknown(temp_cursor.to_string(), suggestion, value.map(|v| v.to_string()))));
                        None
                    }
                };
            if let Some((field_cursor_str, type_info)) = type_info {
                FieldCursor::from(field_cursor_str).set_json_value(&mut object, value.unwrap(), type_info, err, &temp_cursor);
            }
        }
        let mut request: api::GoogleCloudDialogflowCxV3Generator = json::value::from_value(object).unwrap();
        let mut call = self.hub.projects().locations_agents_generators_patch(request, opt.value_of("name").unwrap_or(""));
        for parg in opt.values_of("v").map(|i|i.collect()).unwrap_or(Vec::new()).iter() {
            let (key, value) = parse_kv_arg(&*parg, err, false);
            match key {
                "update-mask" => {
                    call = call.update_mask(        value.map(|v| arg_from_str(v, err, "update-mask", "google-fieldmask")).unwrap_or(FieldMask::default()));
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
                                                                           v.extend(["language-code", "update-mask"].iter().map(|v|*v));
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

    async fn _projects_locations_agents_get(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        let mut call = self.hub.projects().locations_agents_get(opt.value_of("name").unwrap_or(""));
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

    async fn _projects_locations_agents_get_generative_settings(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        let mut call = self.hub.projects().locations_agents_get_generative_settings(opt.value_of("name").unwrap_or(""));
        for parg in opt.values_of("v").map(|i|i.collect()).unwrap_or(Vec::new()).iter() {
            let (key, value) = parse_kv_arg(&*parg, err, false);
            match key {
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
                                                                           v.extend(["language-code"].iter().map(|v|*v));
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

    async fn _projects_locations_agents_get_validation_result(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        let mut call = self.hub.projects().locations_agents_get_validation_result(opt.value_of("name").unwrap_or(""));
        for parg in opt.values_of("v").map(|i|i.collect()).unwrap_or(Vec::new()).iter() {
            let (key, value) = parse_kv_arg(&*parg, err, false);
            match key {
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
                                                                           v.extend(["language-code"].iter().map(|v|*v));
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

    async fn _projects_locations_agents_intents_create(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
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
                    "description" => Some(("description", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "display-name" => Some(("displayName", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "is-fallback" => Some(("isFallback", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "labels" => Some(("labels", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Map })),
                    "name" => Some(("name", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "priority" => Some(("priority", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    _ => {
                        let suggestion = FieldCursor::did_you_mean(key, &vec!["description", "display-name", "is-fallback", "labels", "name", "priority"]);
                        err.issues.push(CLIError::Field(FieldError::Unknown(temp_cursor.to_string(), suggestion, value.map(|v| v.to_string()))));
                        None
                    }
                };
            if let Some((field_cursor_str, type_info)) = type_info {
                FieldCursor::from(field_cursor_str).set_json_value(&mut object, value.unwrap(), type_info, err, &temp_cursor);
            }
        }
        let mut request: api::GoogleCloudDialogflowCxV3Intent = json::value::from_value(object).unwrap();
        let mut call = self.hub.projects().locations_agents_intents_create(request, opt.value_of("parent").unwrap_or(""));
        for parg in opt.values_of("v").map(|i|i.collect()).unwrap_or(Vec::new()).iter() {
            let (key, value) = parse_kv_arg(&*parg, err, false);
            match key {
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
                                                                           v.extend(["language-code"].iter().map(|v|*v));
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

    async fn _projects_locations_agents_intents_delete(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        let mut call = self.hub.projects().locations_agents_intents_delete(opt.value_of("name").unwrap_or(""));
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

    async fn _projects_locations_agents_intents_export(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
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
                    "data-format" => Some(("dataFormat", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "intents" => Some(("intents", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Vec })),
                    "intents-content-inline" => Some(("intentsContentInline", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "intents-uri" => Some(("intentsUri", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    _ => {
                        let suggestion = FieldCursor::did_you_mean(key, &vec!["data-format", "intents", "intents-content-inline", "intents-uri"]);
                        err.issues.push(CLIError::Field(FieldError::Unknown(temp_cursor.to_string(), suggestion, value.map(|v| v.to_string()))));
                        None
                    }
                };
            if let Some((field_cursor_str, type_info)) = type_info {
                FieldCursor::from(field_cursor_str).set_json_value(&mut object, value.unwrap(), type_info, err, &temp_cursor);
            }
        }
        let mut request: api::GoogleCloudDialogflowCxV3ExportIntentsRequest = json::value::from_value(object).unwrap();
        let mut call = self.hub.projects().locations_agents_intents_export(request, opt.value_of("parent").unwrap_or(""));
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

    async fn _projects_locations_agents_intents_get(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        let mut call = self.hub.projects().locations_agents_intents_get(opt.value_of("name").unwrap_or(""));
        for parg in opt.values_of("v").map(|i|i.collect()).unwrap_or(Vec::new()).iter() {
            let (key, value) = parse_kv_arg(&*parg, err, false);
            match key {
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
                                                                           v.extend(["language-code"].iter().map(|v|*v));
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

    async fn _projects_locations_agents_intents_import(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
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
                    "intents-content.content" => Some(("intentsContent.content", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "intents-uri" => Some(("intentsUri", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "merge-option" => Some(("mergeOption", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    _ => {
                        let suggestion = FieldCursor::did_you_mean(key, &vec!["content", "intents-content", "intents-uri", "merge-option"]);
                        err.issues.push(CLIError::Field(FieldError::Unknown(temp_cursor.to_string(), suggestion, value.map(|v| v.to_string()))));
                        None
                    }
                };
            if let Some((field_cursor_str, type_info)) = type_info {
                FieldCursor::from(field_cursor_str).set_json_value(&mut object, value.unwrap(), type_info, err, &temp_cursor);
            }
        }
        let mut request: api::GoogleCloudDialogflowCxV3ImportIntentsRequest = json::value::from_value(object).unwrap();
        let mut call = self.hub.projects().locations_agents_intents_import(request, opt.value_of("parent").unwrap_or(""));
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

    async fn _projects_locations_agents_intents_list(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        let mut call = self.hub.projects().locations_agents_intents_list(opt.value_of("parent").unwrap_or(""));
        for parg in opt.values_of("v").map(|i|i.collect()).unwrap_or(Vec::new()).iter() {
            let (key, value) = parse_kv_arg(&*parg, err, false);
            match key {
                "page-token" => {
                    call = call.page_token(value.unwrap_or(""));
                },
                "page-size" => {
                    call = call.page_size(        value.map(|v| arg_from_str(v, err, "page-size", "int32")).unwrap_or(-0));
                },
                "language-code" => {
                    call = call.language_code(value.unwrap_or(""));
                },
                "intent-view" => {
                    call = call.intent_view(value.unwrap_or(""));
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
                                                                           v.extend(["intent-view", "language-code", "page-size", "page-token"].iter().map(|v|*v));
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

    async fn _projects_locations_agents_intents_patch(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
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
                    "description" => Some(("description", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "display-name" => Some(("displayName", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "is-fallback" => Some(("isFallback", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "labels" => Some(("labels", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Map })),
                    "name" => Some(("name", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "priority" => Some(("priority", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    _ => {
                        let suggestion = FieldCursor::did_you_mean(key, &vec!["description", "display-name", "is-fallback", "labels", "name", "priority"]);
                        err.issues.push(CLIError::Field(FieldError::Unknown(temp_cursor.to_string(), suggestion, value.map(|v| v.to_string()))));
                        None
                    }
                };
            if let Some((field_cursor_str, type_info)) = type_info {
                FieldCursor::from(field_cursor_str).set_json_value(&mut object, value.unwrap(), type_info, err, &temp_cursor);
            }
        }
        let mut request: api::GoogleCloudDialogflowCxV3Intent = json::value::from_value(object).unwrap();
        let mut call = self.hub.projects().locations_agents_intents_patch(request, opt.value_of("name").unwrap_or(""));
        for parg in opt.values_of("v").map(|i|i.collect()).unwrap_or(Vec::new()).iter() {
            let (key, value) = parse_kv_arg(&*parg, err, false);
            match key {
                "update-mask" => {
                    call = call.update_mask(        value.map(|v| arg_from_str(v, err, "update-mask", "google-fieldmask")).unwrap_or(FieldMask::default()));
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
                                                                           v.extend(["language-code", "update-mask"].iter().map(|v|*v));
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

    async fn _projects_locations_agents_list(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        let mut call = self.hub.projects().locations_agents_list(opt.value_of("parent").unwrap_or(""));
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

    async fn _projects_locations_agents_patch(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
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
                    "advanced-settings.audio-export-gcs-destination.uri" => Some(("advancedSettings.audioExportGcsDestination.uri", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "advanced-settings.dtmf-settings.enabled" => Some(("advancedSettings.dtmfSettings.enabled", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "advanced-settings.dtmf-settings.endpointing-timeout-duration" => Some(("advancedSettings.dtmfSettings.endpointingTimeoutDuration", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "advanced-settings.dtmf-settings.finish-digit" => Some(("advancedSettings.dtmfSettings.finishDigit", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "advanced-settings.dtmf-settings.interdigit-timeout-duration" => Some(("advancedSettings.dtmfSettings.interdigitTimeoutDuration", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "advanced-settings.dtmf-settings.max-digits" => Some(("advancedSettings.dtmfSettings.maxDigits", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "advanced-settings.logging-settings.enable-interaction-logging" => Some(("advancedSettings.loggingSettings.enableInteractionLogging", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "advanced-settings.logging-settings.enable-stackdriver-logging" => Some(("advancedSettings.loggingSettings.enableStackdriverLogging", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "advanced-settings.speech-settings.endpointer-sensitivity" => Some(("advancedSettings.speechSettings.endpointerSensitivity", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "advanced-settings.speech-settings.models" => Some(("advancedSettings.speechSettings.models", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Map })),
                    "advanced-settings.speech-settings.no-speech-timeout" => Some(("advancedSettings.speechSettings.noSpeechTimeout", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "advanced-settings.speech-settings.use-timeout-based-endpointing" => Some(("advancedSettings.speechSettings.useTimeoutBasedEndpointing", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "answer-feedback-settings.enable-answer-feedback" => Some(("answerFeedbackSettings.enableAnswerFeedback", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "avatar-uri" => Some(("avatarUri", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "default-language-code" => Some(("defaultLanguageCode", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "description" => Some(("description", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "display-name" => Some(("displayName", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "enable-multi-language-training" => Some(("enableMultiLanguageTraining", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "enable-spell-correction" => Some(("enableSpellCorrection", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "enable-stackdriver-logging" => Some(("enableStackdriverLogging", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "gen-app-builder-settings.engine" => Some(("genAppBuilderSettings.engine", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "git-integration-settings.github-settings.access-token" => Some(("gitIntegrationSettings.githubSettings.accessToken", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "git-integration-settings.github-settings.branches" => Some(("gitIntegrationSettings.githubSettings.branches", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Vec })),
                    "git-integration-settings.github-settings.display-name" => Some(("gitIntegrationSettings.githubSettings.displayName", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "git-integration-settings.github-settings.repository-uri" => Some(("gitIntegrationSettings.githubSettings.repositoryUri", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "git-integration-settings.github-settings.tracking-branch" => Some(("gitIntegrationSettings.githubSettings.trackingBranch", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "locked" => Some(("locked", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "name" => Some(("name", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "security-settings" => Some(("securitySettings", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "speech-to-text-settings.enable-speech-adaptation" => Some(("speechToTextSettings.enableSpeechAdaptation", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "start-flow" => Some(("startFlow", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "supported-language-codes" => Some(("supportedLanguageCodes", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Vec })),
                    "time-zone" => Some(("timeZone", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    _ => {
                        let suggestion = FieldCursor::did_you_mean(key, &vec!["access-token", "advanced-settings", "answer-feedback-settings", "audio-export-gcs-destination", "avatar-uri", "branches", "default-language-code", "description", "display-name", "dtmf-settings", "enable-answer-feedback", "enable-interaction-logging", "enable-multi-language-training", "enable-speech-adaptation", "enable-spell-correction", "enable-stackdriver-logging", "enabled", "endpointer-sensitivity", "endpointing-timeout-duration", "engine", "finish-digit", "gen-app-builder-settings", "git-integration-settings", "github-settings", "interdigit-timeout-duration", "locked", "logging-settings", "max-digits", "models", "name", "no-speech-timeout", "repository-uri", "security-settings", "speech-settings", "speech-to-text-settings", "start-flow", "supported-language-codes", "time-zone", "tracking-branch", "uri", "use-timeout-based-endpointing"]);
                        err.issues.push(CLIError::Field(FieldError::Unknown(temp_cursor.to_string(), suggestion, value.map(|v| v.to_string()))));
                        None
                    }
                };
            if let Some((field_cursor_str, type_info)) = type_info {
                FieldCursor::from(field_cursor_str).set_json_value(&mut object, value.unwrap(), type_info, err, &temp_cursor);
            }
        }
        let mut request: api::GoogleCloudDialogflowCxV3Agent = json::value::from_value(object).unwrap();
        let mut call = self.hub.projects().locations_agents_patch(request, opt.value_of("name").unwrap_or(""));
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

    async fn _projects_locations_agents_restore(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
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
                    "agent-content" => Some(("agentContent", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "agent-uri" => Some(("agentUri", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "git-source.tracking-branch" => Some(("gitSource.trackingBranch", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "restore-option" => Some(("restoreOption", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    _ => {
                        let suggestion = FieldCursor::did_you_mean(key, &vec!["agent-content", "agent-uri", "git-source", "restore-option", "tracking-branch"]);
                        err.issues.push(CLIError::Field(FieldError::Unknown(temp_cursor.to_string(), suggestion, value.map(|v| v.to_string()))));
                        None
                    }
                };
            if let Some((field_cursor_str, type_info)) = type_info {
                FieldCursor::from(field_cursor_str).set_json_value(&mut object, value.unwrap(), type_info, err, &temp_cursor);
            }
        }
        let mut request: api::GoogleCloudDialogflowCxV3RestoreAgentRequest = json::value::from_value(object).unwrap();
        let mut call = self.hub.projects().locations_agents_restore(request, opt.value_of("name").unwrap_or(""));
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

    async fn _projects_locations_agents_sessions_detect_intent(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
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
                    "output-audio-config.audio-encoding" => Some(("outputAudioConfig.audioEncoding", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "output-audio-config.sample-rate-hertz" => Some(("outputAudioConfig.sampleRateHertz", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "output-audio-config.synthesize-speech-config.effects-profile-id" => Some(("outputAudioConfig.synthesizeSpeechConfig.effectsProfileId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Vec })),
                    "output-audio-config.synthesize-speech-config.pitch" => Some(("outputAudioConfig.synthesizeSpeechConfig.pitch", JsonTypeInfo { jtype: JsonType::Float, ctype: ComplexType::Pod })),
                    "output-audio-config.synthesize-speech-config.speaking-rate" => Some(("outputAudioConfig.synthesizeSpeechConfig.speakingRate", JsonTypeInfo { jtype: JsonType::Float, ctype: ComplexType::Pod })),
                    "output-audio-config.synthesize-speech-config.voice.name" => Some(("outputAudioConfig.synthesizeSpeechConfig.voice.name", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "output-audio-config.synthesize-speech-config.voice.ssml-gender" => Some(("outputAudioConfig.synthesizeSpeechConfig.voice.ssmlGender", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "output-audio-config.synthesize-speech-config.volume-gain-db" => Some(("outputAudioConfig.synthesizeSpeechConfig.volumeGainDb", JsonTypeInfo { jtype: JsonType::Float, ctype: ComplexType::Pod })),
                    "query-input.audio.audio" => Some(("queryInput.audio.audio", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "query-input.audio.config.audio-encoding" => Some(("queryInput.audio.config.audioEncoding", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "query-input.audio.config.barge-in-config.no-barge-in-duration" => Some(("queryInput.audio.config.bargeInConfig.noBargeInDuration", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "query-input.audio.config.barge-in-config.total-duration" => Some(("queryInput.audio.config.bargeInConfig.totalDuration", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "query-input.audio.config.enable-word-info" => Some(("queryInput.audio.config.enableWordInfo", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "query-input.audio.config.model" => Some(("queryInput.audio.config.model", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "query-input.audio.config.model-variant" => Some(("queryInput.audio.config.modelVariant", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "query-input.audio.config.opt-out-conformer-model-migration" => Some(("queryInput.audio.config.optOutConformerModelMigration", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "query-input.audio.config.phrase-hints" => Some(("queryInput.audio.config.phraseHints", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Vec })),
                    "query-input.audio.config.sample-rate-hertz" => Some(("queryInput.audio.config.sampleRateHertz", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "query-input.audio.config.single-utterance" => Some(("queryInput.audio.config.singleUtterance", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "query-input.dtmf.digits" => Some(("queryInput.dtmf.digits", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "query-input.dtmf.finish-digit" => Some(("queryInput.dtmf.finishDigit", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "query-input.event.event" => Some(("queryInput.event.event", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "query-input.intent.intent" => Some(("queryInput.intent.intent", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "query-input.language-code" => Some(("queryInput.languageCode", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "query-input.text.text" => Some(("queryInput.text.text", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "query-params.analyze-query-text-sentiment" => Some(("queryParams.analyzeQueryTextSentiment", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "query-params.channel" => Some(("queryParams.channel", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "query-params.current-page" => Some(("queryParams.currentPage", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "query-params.disable-webhook" => Some(("queryParams.disableWebhook", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "query-params.flow-versions" => Some(("queryParams.flowVersions", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Vec })),
                    "query-params.geo-location.latitude" => Some(("queryParams.geoLocation.latitude", JsonTypeInfo { jtype: JsonType::Float, ctype: ComplexType::Pod })),
                    "query-params.geo-location.longitude" => Some(("queryParams.geoLocation.longitude", JsonTypeInfo { jtype: JsonType::Float, ctype: ComplexType::Pod })),
                    "query-params.populate-data-store-connection-signals" => Some(("queryParams.populateDataStoreConnectionSignals", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "query-params.session-ttl" => Some(("queryParams.sessionTtl", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "query-params.time-zone" => Some(("queryParams.timeZone", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "query-params.webhook-headers" => Some(("queryParams.webhookHeaders", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Map })),
                    _ => {
                        let suggestion = FieldCursor::did_you_mean(key, &vec!["analyze-query-text-sentiment", "audio", "audio-encoding", "barge-in-config", "channel", "config", "current-page", "digits", "disable-webhook", "dtmf", "effects-profile-id", "enable-word-info", "event", "finish-digit", "flow-versions", "geo-location", "intent", "language-code", "latitude", "longitude", "model", "model-variant", "name", "no-barge-in-duration", "opt-out-conformer-model-migration", "output-audio-config", "phrase-hints", "pitch", "populate-data-store-connection-signals", "query-input", "query-params", "sample-rate-hertz", "session-ttl", "single-utterance", "speaking-rate", "ssml-gender", "synthesize-speech-config", "text", "time-zone", "total-duration", "voice", "volume-gain-db", "webhook-headers"]);
                        err.issues.push(CLIError::Field(FieldError::Unknown(temp_cursor.to_string(), suggestion, value.map(|v| v.to_string()))));
                        None
                    }
                };
            if let Some((field_cursor_str, type_info)) = type_info {
                FieldCursor::from(field_cursor_str).set_json_value(&mut object, value.unwrap(), type_info, err, &temp_cursor);
            }
        }
        let mut request: api::GoogleCloudDialogflowCxV3DetectIntentRequest = json::value::from_value(object).unwrap();
        let mut call = self.hub.projects().locations_agents_sessions_detect_intent(request, opt.value_of("session").unwrap_or(""));
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

    async fn _projects_locations_agents_sessions_entity_types_create(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
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
                    "entity-override-mode" => Some(("entityOverrideMode", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "name" => Some(("name", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    _ => {
                        let suggestion = FieldCursor::did_you_mean(key, &vec!["entity-override-mode", "name"]);
                        err.issues.push(CLIError::Field(FieldError::Unknown(temp_cursor.to_string(), suggestion, value.map(|v| v.to_string()))));
                        None
                    }
                };
            if let Some((field_cursor_str, type_info)) = type_info {
                FieldCursor::from(field_cursor_str).set_json_value(&mut object, value.unwrap(), type_info, err, &temp_cursor);
            }
        }
        let mut request: api::GoogleCloudDialogflowCxV3SessionEntityType = json::value::from_value(object).unwrap();
        let mut call = self.hub.projects().locations_agents_sessions_entity_types_create(request, opt.value_of("parent").unwrap_or(""));
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

    async fn _projects_locations_agents_sessions_entity_types_delete(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        let mut call = self.hub.projects().locations_agents_sessions_entity_types_delete(opt.value_of("name").unwrap_or(""));
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

    async fn _projects_locations_agents_sessions_entity_types_get(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        let mut call = self.hub.projects().locations_agents_sessions_entity_types_get(opt.value_of("name").unwrap_or(""));
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

    async fn _projects_locations_agents_sessions_entity_types_list(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        let mut call = self.hub.projects().locations_agents_sessions_entity_types_list(opt.value_of("parent").unwrap_or(""));
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

    async fn _projects_locations_agents_sessions_entity_types_patch(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
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
                    "entity-override-mode" => Some(("entityOverrideMode", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "name" => Some(("name", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    _ => {
                        let suggestion = FieldCursor::did_you_mean(key, &vec!["entity-override-mode", "name"]);
                        err.issues.push(CLIError::Field(FieldError::Unknown(temp_cursor.to_string(), suggestion, value.map(|v| v.to_string()))));
                        None
                    }
                };
            if let Some((field_cursor_str, type_info)) = type_info {
                FieldCursor::from(field_cursor_str).set_json_value(&mut object, value.unwrap(), type_info, err, &temp_cursor);
            }
        }
        let mut request: api::GoogleCloudDialogflowCxV3SessionEntityType = json::value::from_value(object).unwrap();
        let mut call = self.hub.projects().locations_agents_sessions_entity_types_patch(request, opt.value_of("name").unwrap_or(""));
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

    async fn _projects_locations_agents_sessions_fulfill_intent(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
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
                    "match.confidence" => Some(("match.confidence", JsonTypeInfo { jtype: JsonType::Float, ctype: ComplexType::Pod })),
                    "match.event" => Some(("match.event", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "match.intent.description" => Some(("match.intent.description", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "match.intent.display-name" => Some(("match.intent.displayName", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "match.intent.is-fallback" => Some(("match.intent.isFallback", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "match.intent.labels" => Some(("match.intent.labels", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Map })),
                    "match.intent.name" => Some(("match.intent.name", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "match.intent.priority" => Some(("match.intent.priority", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "match.match-type" => Some(("match.matchType", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "match.resolved-input" => Some(("match.resolvedInput", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "match-intent-request.persist-parameter-changes" => Some(("matchIntentRequest.persistParameterChanges", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "match-intent-request.query-input.audio.audio" => Some(("matchIntentRequest.queryInput.audio.audio", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "match-intent-request.query-input.audio.config.audio-encoding" => Some(("matchIntentRequest.queryInput.audio.config.audioEncoding", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "match-intent-request.query-input.audio.config.barge-in-config.no-barge-in-duration" => Some(("matchIntentRequest.queryInput.audio.config.bargeInConfig.noBargeInDuration", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "match-intent-request.query-input.audio.config.barge-in-config.total-duration" => Some(("matchIntentRequest.queryInput.audio.config.bargeInConfig.totalDuration", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "match-intent-request.query-input.audio.config.enable-word-info" => Some(("matchIntentRequest.queryInput.audio.config.enableWordInfo", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "match-intent-request.query-input.audio.config.model" => Some(("matchIntentRequest.queryInput.audio.config.model", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "match-intent-request.query-input.audio.config.model-variant" => Some(("matchIntentRequest.queryInput.audio.config.modelVariant", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "match-intent-request.query-input.audio.config.opt-out-conformer-model-migration" => Some(("matchIntentRequest.queryInput.audio.config.optOutConformerModelMigration", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "match-intent-request.query-input.audio.config.phrase-hints" => Some(("matchIntentRequest.queryInput.audio.config.phraseHints", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Vec })),
                    "match-intent-request.query-input.audio.config.sample-rate-hertz" => Some(("matchIntentRequest.queryInput.audio.config.sampleRateHertz", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "match-intent-request.query-input.audio.config.single-utterance" => Some(("matchIntentRequest.queryInput.audio.config.singleUtterance", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "match-intent-request.query-input.dtmf.digits" => Some(("matchIntentRequest.queryInput.dtmf.digits", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "match-intent-request.query-input.dtmf.finish-digit" => Some(("matchIntentRequest.queryInput.dtmf.finishDigit", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "match-intent-request.query-input.event.event" => Some(("matchIntentRequest.queryInput.event.event", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "match-intent-request.query-input.intent.intent" => Some(("matchIntentRequest.queryInput.intent.intent", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "match-intent-request.query-input.language-code" => Some(("matchIntentRequest.queryInput.languageCode", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "match-intent-request.query-input.text.text" => Some(("matchIntentRequest.queryInput.text.text", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "match-intent-request.query-params.analyze-query-text-sentiment" => Some(("matchIntentRequest.queryParams.analyzeQueryTextSentiment", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "match-intent-request.query-params.channel" => Some(("matchIntentRequest.queryParams.channel", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "match-intent-request.query-params.current-page" => Some(("matchIntentRequest.queryParams.currentPage", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "match-intent-request.query-params.disable-webhook" => Some(("matchIntentRequest.queryParams.disableWebhook", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "match-intent-request.query-params.flow-versions" => Some(("matchIntentRequest.queryParams.flowVersions", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Vec })),
                    "match-intent-request.query-params.geo-location.latitude" => Some(("matchIntentRequest.queryParams.geoLocation.latitude", JsonTypeInfo { jtype: JsonType::Float, ctype: ComplexType::Pod })),
                    "match-intent-request.query-params.geo-location.longitude" => Some(("matchIntentRequest.queryParams.geoLocation.longitude", JsonTypeInfo { jtype: JsonType::Float, ctype: ComplexType::Pod })),
                    "match-intent-request.query-params.populate-data-store-connection-signals" => Some(("matchIntentRequest.queryParams.populateDataStoreConnectionSignals", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "match-intent-request.query-params.session-ttl" => Some(("matchIntentRequest.queryParams.sessionTtl", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "match-intent-request.query-params.time-zone" => Some(("matchIntentRequest.queryParams.timeZone", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "match-intent-request.query-params.webhook-headers" => Some(("matchIntentRequest.queryParams.webhookHeaders", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Map })),
                    "output-audio-config.audio-encoding" => Some(("outputAudioConfig.audioEncoding", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "output-audio-config.sample-rate-hertz" => Some(("outputAudioConfig.sampleRateHertz", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "output-audio-config.synthesize-speech-config.effects-profile-id" => Some(("outputAudioConfig.synthesizeSpeechConfig.effectsProfileId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Vec })),
                    "output-audio-config.synthesize-speech-config.pitch" => Some(("outputAudioConfig.synthesizeSpeechConfig.pitch", JsonTypeInfo { jtype: JsonType::Float, ctype: ComplexType::Pod })),
                    "output-audio-config.synthesize-speech-config.speaking-rate" => Some(("outputAudioConfig.synthesizeSpeechConfig.speakingRate", JsonTypeInfo { jtype: JsonType::Float, ctype: ComplexType::Pod })),
                    "output-audio-config.synthesize-speech-config.voice.name" => Some(("outputAudioConfig.synthesizeSpeechConfig.voice.name", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "output-audio-config.synthesize-speech-config.voice.ssml-gender" => Some(("outputAudioConfig.synthesizeSpeechConfig.voice.ssmlGender", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "output-audio-config.synthesize-speech-config.volume-gain-db" => Some(("outputAudioConfig.synthesizeSpeechConfig.volumeGainDb", JsonTypeInfo { jtype: JsonType::Float, ctype: ComplexType::Pod })),
                    _ => {
                        let suggestion = FieldCursor::did_you_mean(key, &vec!["analyze-query-text-sentiment", "audio", "audio-encoding", "barge-in-config", "channel", "confidence", "config", "current-page", "description", "digits", "disable-webhook", "display-name", "dtmf", "effects-profile-id", "enable-word-info", "event", "finish-digit", "flow-versions", "geo-location", "intent", "is-fallback", "labels", "language-code", "latitude", "longitude", "match", "match-intent-request", "match-type", "model", "model-variant", "name", "no-barge-in-duration", "opt-out-conformer-model-migration", "output-audio-config", "persist-parameter-changes", "phrase-hints", "pitch", "populate-data-store-connection-signals", "priority", "query-input", "query-params", "resolved-input", "sample-rate-hertz", "session-ttl", "single-utterance", "speaking-rate", "ssml-gender", "synthesize-speech-config", "text", "time-zone", "total-duration", "voice", "volume-gain-db", "webhook-headers"]);
                        err.issues.push(CLIError::Field(FieldError::Unknown(temp_cursor.to_string(), suggestion, value.map(|v| v.to_string()))));
                        None
                    }
                };
            if let Some((field_cursor_str, type_info)) = type_info {
                FieldCursor::from(field_cursor_str).set_json_value(&mut object, value.unwrap(), type_info, err, &temp_cursor);
            }
        }
        let mut request: api::GoogleCloudDialogflowCxV3FulfillIntentRequest = json::value::from_value(object).unwrap();
        let mut call = self.hub.projects().locations_agents_sessions_fulfill_intent(request, opt.value_of("session").unwrap_or(""));
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

    async fn _projects_locations_agents_sessions_match_intent(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
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
                    "persist-parameter-changes" => Some(("persistParameterChanges", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "query-input.audio.audio" => Some(("queryInput.audio.audio", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "query-input.audio.config.audio-encoding" => Some(("queryInput.audio.config.audioEncoding", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "query-input.audio.config.barge-in-config.no-barge-in-duration" => Some(("queryInput.audio.config.bargeInConfig.noBargeInDuration", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "query-input.audio.config.barge-in-config.total-duration" => Some(("queryInput.audio.config.bargeInConfig.totalDuration", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "query-input.audio.config.enable-word-info" => Some(("queryInput.audio.config.enableWordInfo", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "query-input.audio.config.model" => Some(("queryInput.audio.config.model", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "query-input.audio.config.model-variant" => Some(("queryInput.audio.config.modelVariant", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "query-input.audio.config.opt-out-conformer-model-migration" => Some(("queryInput.audio.config.optOutConformerModelMigration", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "query-input.audio.config.phrase-hints" => Some(("queryInput.audio.config.phraseHints", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Vec })),
                    "query-input.audio.config.sample-rate-hertz" => Some(("queryInput.audio.config.sampleRateHertz", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "query-input.audio.config.single-utterance" => Some(("queryInput.audio.config.singleUtterance", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "query-input.dtmf.digits" => Some(("queryInput.dtmf.digits", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "query-input.dtmf.finish-digit" => Some(("queryInput.dtmf.finishDigit", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "query-input.event.event" => Some(("queryInput.event.event", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "query-input.intent.intent" => Some(("queryInput.intent.intent", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "query-input.language-code" => Some(("queryInput.languageCode", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "query-input.text.text" => Some(("queryInput.text.text", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "query-params.analyze-query-text-sentiment" => Some(("queryParams.analyzeQueryTextSentiment", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "query-params.channel" => Some(("queryParams.channel", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "query-params.current-page" => Some(("queryParams.currentPage", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "query-params.disable-webhook" => Some(("queryParams.disableWebhook", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "query-params.flow-versions" => Some(("queryParams.flowVersions", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Vec })),
                    "query-params.geo-location.latitude" => Some(("queryParams.geoLocation.latitude", JsonTypeInfo { jtype: JsonType::Float, ctype: ComplexType::Pod })),
                    "query-params.geo-location.longitude" => Some(("queryParams.geoLocation.longitude", JsonTypeInfo { jtype: JsonType::Float, ctype: ComplexType::Pod })),
                    "query-params.populate-data-store-connection-signals" => Some(("queryParams.populateDataStoreConnectionSignals", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "query-params.session-ttl" => Some(("queryParams.sessionTtl", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "query-params.time-zone" => Some(("queryParams.timeZone", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "query-params.webhook-headers" => Some(("queryParams.webhookHeaders", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Map })),
                    _ => {
                        let suggestion = FieldCursor::did_you_mean(key, &vec!["analyze-query-text-sentiment", "audio", "audio-encoding", "barge-in-config", "channel", "config", "current-page", "digits", "disable-webhook", "dtmf", "enable-word-info", "event", "finish-digit", "flow-versions", "geo-location", "intent", "language-code", "latitude", "longitude", "model", "model-variant", "no-barge-in-duration", "opt-out-conformer-model-migration", "persist-parameter-changes", "phrase-hints", "populate-data-store-connection-signals", "query-input", "query-params", "sample-rate-hertz", "session-ttl", "single-utterance", "text", "time-zone", "total-duration", "webhook-headers"]);
                        err.issues.push(CLIError::Field(FieldError::Unknown(temp_cursor.to_string(), suggestion, value.map(|v| v.to_string()))));
                        None
                    }
                };
            if let Some((field_cursor_str, type_info)) = type_info {
                FieldCursor::from(field_cursor_str).set_json_value(&mut object, value.unwrap(), type_info, err, &temp_cursor);
            }
        }
        let mut request: api::GoogleCloudDialogflowCxV3MatchIntentRequest = json::value::from_value(object).unwrap();
        let mut call = self.hub.projects().locations_agents_sessions_match_intent(request, opt.value_of("session").unwrap_or(""));
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

    async fn _projects_locations_agents_sessions_server_streaming_detect_intent(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
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
                    "output-audio-config.audio-encoding" => Some(("outputAudioConfig.audioEncoding", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "output-audio-config.sample-rate-hertz" => Some(("outputAudioConfig.sampleRateHertz", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "output-audio-config.synthesize-speech-config.effects-profile-id" => Some(("outputAudioConfig.synthesizeSpeechConfig.effectsProfileId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Vec })),
                    "output-audio-config.synthesize-speech-config.pitch" => Some(("outputAudioConfig.synthesizeSpeechConfig.pitch", JsonTypeInfo { jtype: JsonType::Float, ctype: ComplexType::Pod })),
                    "output-audio-config.synthesize-speech-config.speaking-rate" => Some(("outputAudioConfig.synthesizeSpeechConfig.speakingRate", JsonTypeInfo { jtype: JsonType::Float, ctype: ComplexType::Pod })),
                    "output-audio-config.synthesize-speech-config.voice.name" => Some(("outputAudioConfig.synthesizeSpeechConfig.voice.name", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "output-audio-config.synthesize-speech-config.voice.ssml-gender" => Some(("outputAudioConfig.synthesizeSpeechConfig.voice.ssmlGender", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "output-audio-config.synthesize-speech-config.volume-gain-db" => Some(("outputAudioConfig.synthesizeSpeechConfig.volumeGainDb", JsonTypeInfo { jtype: JsonType::Float, ctype: ComplexType::Pod })),
                    "query-input.audio.audio" => Some(("queryInput.audio.audio", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "query-input.audio.config.audio-encoding" => Some(("queryInput.audio.config.audioEncoding", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "query-input.audio.config.barge-in-config.no-barge-in-duration" => Some(("queryInput.audio.config.bargeInConfig.noBargeInDuration", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "query-input.audio.config.barge-in-config.total-duration" => Some(("queryInput.audio.config.bargeInConfig.totalDuration", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "query-input.audio.config.enable-word-info" => Some(("queryInput.audio.config.enableWordInfo", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "query-input.audio.config.model" => Some(("queryInput.audio.config.model", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "query-input.audio.config.model-variant" => Some(("queryInput.audio.config.modelVariant", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "query-input.audio.config.opt-out-conformer-model-migration" => Some(("queryInput.audio.config.optOutConformerModelMigration", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "query-input.audio.config.phrase-hints" => Some(("queryInput.audio.config.phraseHints", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Vec })),
                    "query-input.audio.config.sample-rate-hertz" => Some(("queryInput.audio.config.sampleRateHertz", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "query-input.audio.config.single-utterance" => Some(("queryInput.audio.config.singleUtterance", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "query-input.dtmf.digits" => Some(("queryInput.dtmf.digits", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "query-input.dtmf.finish-digit" => Some(("queryInput.dtmf.finishDigit", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "query-input.event.event" => Some(("queryInput.event.event", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "query-input.intent.intent" => Some(("queryInput.intent.intent", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "query-input.language-code" => Some(("queryInput.languageCode", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "query-input.text.text" => Some(("queryInput.text.text", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "query-params.analyze-query-text-sentiment" => Some(("queryParams.analyzeQueryTextSentiment", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "query-params.channel" => Some(("queryParams.channel", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "query-params.current-page" => Some(("queryParams.currentPage", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "query-params.disable-webhook" => Some(("queryParams.disableWebhook", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "query-params.flow-versions" => Some(("queryParams.flowVersions", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Vec })),
                    "query-params.geo-location.latitude" => Some(("queryParams.geoLocation.latitude", JsonTypeInfo { jtype: JsonType::Float, ctype: ComplexType::Pod })),
                    "query-params.geo-location.longitude" => Some(("queryParams.geoLocation.longitude", JsonTypeInfo { jtype: JsonType::Float, ctype: ComplexType::Pod })),
                    "query-params.populate-data-store-connection-signals" => Some(("queryParams.populateDataStoreConnectionSignals", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "query-params.session-ttl" => Some(("queryParams.sessionTtl", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "query-params.time-zone" => Some(("queryParams.timeZone", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "query-params.webhook-headers" => Some(("queryParams.webhookHeaders", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Map })),
                    _ => {
                        let suggestion = FieldCursor::did_you_mean(key, &vec!["analyze-query-text-sentiment", "audio", "audio-encoding", "barge-in-config", "channel", "config", "current-page", "digits", "disable-webhook", "dtmf", "effects-profile-id", "enable-word-info", "event", "finish-digit", "flow-versions", "geo-location", "intent", "language-code", "latitude", "longitude", "model", "model-variant", "name", "no-barge-in-duration", "opt-out-conformer-model-migration", "output-audio-config", "phrase-hints", "pitch", "populate-data-store-connection-signals", "query-input", "query-params", "sample-rate-hertz", "session-ttl", "single-utterance", "speaking-rate", "ssml-gender", "synthesize-speech-config", "text", "time-zone", "total-duration", "voice", "volume-gain-db", "webhook-headers"]);
                        err.issues.push(CLIError::Field(FieldError::Unknown(temp_cursor.to_string(), suggestion, value.map(|v| v.to_string()))));
                        None
                    }
                };
            if let Some((field_cursor_str, type_info)) = type_info {
                FieldCursor::from(field_cursor_str).set_json_value(&mut object, value.unwrap(), type_info, err, &temp_cursor);
            }
        }
        let mut request: api::GoogleCloudDialogflowCxV3DetectIntentRequest = json::value::from_value(object).unwrap();
        let mut call = self.hub.projects().locations_agents_sessions_server_streaming_detect_intent(request, opt.value_of("session").unwrap_or(""));
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

    async fn _projects_locations_agents_sessions_submit_answer_feedback(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
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
                    "answer-feedback.custom-rating" => Some(("answerFeedback.customRating", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "answer-feedback.rating" => Some(("answerFeedback.rating", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "answer-feedback.rating-reason.feedback" => Some(("answerFeedback.ratingReason.feedback", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "answer-feedback.rating-reason.reason-labels" => Some(("answerFeedback.ratingReason.reasonLabels", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Vec })),
                    "response-id" => Some(("responseId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "update-mask" => Some(("updateMask", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    _ => {
                        let suggestion = FieldCursor::did_you_mean(key, &vec!["answer-feedback", "custom-rating", "feedback", "rating", "rating-reason", "reason-labels", "response-id", "update-mask"]);
                        err.issues.push(CLIError::Field(FieldError::Unknown(temp_cursor.to_string(), suggestion, value.map(|v| v.to_string()))));
                        None
                    }
                };
            if let Some((field_cursor_str, type_info)) = type_info {
                FieldCursor::from(field_cursor_str).set_json_value(&mut object, value.unwrap(), type_info, err, &temp_cursor);
            }
        }
        let mut request: api::GoogleCloudDialogflowCxV3SubmitAnswerFeedbackRequest = json::value::from_value(object).unwrap();
        let mut call = self.hub.projects().locations_agents_sessions_submit_answer_feedback(request, opt.value_of("session").unwrap_or(""));
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

    async fn _projects_locations_agents_test_cases_batch_delete(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
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
        let mut request: api::GoogleCloudDialogflowCxV3BatchDeleteTestCasesRequest = json::value::from_value(object).unwrap();
        let mut call = self.hub.projects().locations_agents_test_cases_batch_delete(request, opt.value_of("parent").unwrap_or(""));
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

    async fn _projects_locations_agents_test_cases_batch_run(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
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
                    "environment" => Some(("environment", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "test-cases" => Some(("testCases", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Vec })),
                    _ => {
                        let suggestion = FieldCursor::did_you_mean(key, &vec!["environment", "test-cases"]);
                        err.issues.push(CLIError::Field(FieldError::Unknown(temp_cursor.to_string(), suggestion, value.map(|v| v.to_string()))));
                        None
                    }
                };
            if let Some((field_cursor_str, type_info)) = type_info {
                FieldCursor::from(field_cursor_str).set_json_value(&mut object, value.unwrap(), type_info, err, &temp_cursor);
            }
        }
        let mut request: api::GoogleCloudDialogflowCxV3BatchRunTestCasesRequest = json::value::from_value(object).unwrap();
        let mut call = self.hub.projects().locations_agents_test_cases_batch_run(request, opt.value_of("parent").unwrap_or(""));
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

    async fn _projects_locations_agents_test_cases_calculate_coverage(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        let mut call = self.hub.projects().locations_agents_test_cases_calculate_coverage(opt.value_of("agent").unwrap_or(""));
        for parg in opt.values_of("v").map(|i|i.collect()).unwrap_or(Vec::new()).iter() {
            let (key, value) = parse_kv_arg(&*parg, err, false);
            match key {
                "type" => {
                    call = call.type_(value.unwrap_or(""));
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
                                                                           v.extend(["type"].iter().map(|v|*v));
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

    async fn _projects_locations_agents_test_cases_create(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
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
                    "creation-time" => Some(("creationTime", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "display-name" => Some(("displayName", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "last-test-result.environment" => Some(("lastTestResult.environment", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "last-test-result.name" => Some(("lastTestResult.name", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "last-test-result.test-result" => Some(("lastTestResult.testResult", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "last-test-result.test-time" => Some(("lastTestResult.testTime", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "name" => Some(("name", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "notes" => Some(("notes", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "tags" => Some(("tags", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Vec })),
                    "test-config.flow" => Some(("testConfig.flow", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "test-config.page" => Some(("testConfig.page", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "test-config.tracking-parameters" => Some(("testConfig.trackingParameters", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Vec })),
                    _ => {
                        let suggestion = FieldCursor::did_you_mean(key, &vec!["creation-time", "display-name", "environment", "flow", "last-test-result", "name", "notes", "page", "tags", "test-config", "test-result", "test-time", "tracking-parameters"]);
                        err.issues.push(CLIError::Field(FieldError::Unknown(temp_cursor.to_string(), suggestion, value.map(|v| v.to_string()))));
                        None
                    }
                };
            if let Some((field_cursor_str, type_info)) = type_info {
                FieldCursor::from(field_cursor_str).set_json_value(&mut object, value.unwrap(), type_info, err, &temp_cursor);
            }
        }
        let mut request: api::GoogleCloudDialogflowCxV3TestCase = json::value::from_value(object).unwrap();
        let mut call = self.hub.projects().locations_agents_test_cases_create(request, opt.value_of("parent").unwrap_or(""));
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

    async fn _projects_locations_agents_test_cases_export(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
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
                    "data-format" => Some(("dataFormat", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "filter" => Some(("filter", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "gcs-uri" => Some(("gcsUri", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    _ => {
                        let suggestion = FieldCursor::did_you_mean(key, &vec!["data-format", "filter", "gcs-uri"]);
                        err.issues.push(CLIError::Field(FieldError::Unknown(temp_cursor.to_string(), suggestion, value.map(|v| v.to_string()))));
                        None
                    }
                };
            if let Some((field_cursor_str, type_info)) = type_info {
                FieldCursor::from(field_cursor_str).set_json_value(&mut object, value.unwrap(), type_info, err, &temp_cursor);
            }
        }
        let mut request: api::GoogleCloudDialogflowCxV3ExportTestCasesRequest = json::value::from_value(object).unwrap();
        let mut call = self.hub.projects().locations_agents_test_cases_export(request, opt.value_of("parent").unwrap_or(""));
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

    async fn _projects_locations_agents_test_cases_get(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        let mut call = self.hub.projects().locations_agents_test_cases_get(opt.value_of("name").unwrap_or(""));
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

    async fn _projects_locations_agents_test_cases_import(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
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
                    "content" => Some(("content", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "gcs-uri" => Some(("gcsUri", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    _ => {
                        let suggestion = FieldCursor::did_you_mean(key, &vec!["content", "gcs-uri"]);
                        err.issues.push(CLIError::Field(FieldError::Unknown(temp_cursor.to_string(), suggestion, value.map(|v| v.to_string()))));
                        None
                    }
                };
            if let Some((field_cursor_str, type_info)) = type_info {
                FieldCursor::from(field_cursor_str).set_json_value(&mut object, value.unwrap(), type_info, err, &temp_cursor);
            }
        }
        let mut request: api::GoogleCloudDialogflowCxV3ImportTestCasesRequest = json::value::from_value(object).unwrap();
        let mut call = self.hub.projects().locations_agents_test_cases_import(request, opt.value_of("parent").unwrap_or(""));
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

    async fn _projects_locations_agents_test_cases_list(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        let mut call = self.hub.projects().locations_agents_test_cases_list(opt.value_of("parent").unwrap_or(""));
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
                                                                           v.extend(["page-size", "page-token", "view"].iter().map(|v|*v));
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

    async fn _projects_locations_agents_test_cases_patch(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
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
                    "creation-time" => Some(("creationTime", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "display-name" => Some(("displayName", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "last-test-result.environment" => Some(("lastTestResult.environment", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "last-test-result.name" => Some(("lastTestResult.name", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "last-test-result.test-result" => Some(("lastTestResult.testResult", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "last-test-result.test-time" => Some(("lastTestResult.testTime", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "name" => Some(("name", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "notes" => Some(("notes", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "tags" => Some(("tags", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Vec })),
                    "test-config.flow" => Some(("testConfig.flow", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "test-config.page" => Some(("testConfig.page", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "test-config.tracking-parameters" => Some(("testConfig.trackingParameters", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Vec })),
                    _ => {
                        let suggestion = FieldCursor::did_you_mean(key, &vec!["creation-time", "display-name", "environment", "flow", "last-test-result", "name", "notes", "page", "tags", "test-config", "test-result", "test-time", "tracking-parameters"]);
                        err.issues.push(CLIError::Field(FieldError::Unknown(temp_cursor.to_string(), suggestion, value.map(|v| v.to_string()))));
                        None
                    }
                };
            if let Some((field_cursor_str, type_info)) = type_info {
                FieldCursor::from(field_cursor_str).set_json_value(&mut object, value.unwrap(), type_info, err, &temp_cursor);
            }
        }
        let mut request: api::GoogleCloudDialogflowCxV3TestCase = json::value::from_value(object).unwrap();
        let mut call = self.hub.projects().locations_agents_test_cases_patch(request, opt.value_of("name").unwrap_or(""));
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

    async fn _projects_locations_agents_test_cases_results_get(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        let mut call = self.hub.projects().locations_agents_test_cases_results_get(opt.value_of("name").unwrap_or(""));
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

    async fn _projects_locations_agents_test_cases_results_list(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        let mut call = self.hub.projects().locations_agents_test_cases_results_list(opt.value_of("parent").unwrap_or(""));
        for parg in opt.values_of("v").map(|i|i.collect()).unwrap_or(Vec::new()).iter() {
            let (key, value) = parse_kv_arg(&*parg, err, false);
            match key {
                "page-token" => {
                    call = call.page_token(value.unwrap_or(""));
                },
                "page-size" => {
                    call = call.page_size(        value.map(|v| arg_from_str(v, err, "page-size", "int32")).unwrap_or(-0));
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
                                                                           v.extend(["filter", "page-size", "page-token"].iter().map(|v|*v));
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

    async fn _projects_locations_agents_test_cases_run(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
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
                    "environment" => Some(("environment", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    _ => {
                        let suggestion = FieldCursor::did_you_mean(key, &vec!["environment"]);
                        err.issues.push(CLIError::Field(FieldError::Unknown(temp_cursor.to_string(), suggestion, value.map(|v| v.to_string()))));
                        None
                    }
                };
            if let Some((field_cursor_str, type_info)) = type_info {
                FieldCursor::from(field_cursor_str).set_json_value(&mut object, value.unwrap(), type_info, err, &temp_cursor);
            }
        }
        let mut request: api::GoogleCloudDialogflowCxV3RunTestCaseRequest = json::value::from_value(object).unwrap();
        let mut call = self.hub.projects().locations_agents_test_cases_run(request, opt.value_of("name").unwrap_or(""));
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

    async fn _projects_locations_agents_transition_route_groups_create(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
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
                    "display-name" => Some(("displayName", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "name" => Some(("name", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    _ => {
                        let suggestion = FieldCursor::did_you_mean(key, &vec!["display-name", "name"]);
                        err.issues.push(CLIError::Field(FieldError::Unknown(temp_cursor.to_string(), suggestion, value.map(|v| v.to_string()))));
                        None
                    }
                };
            if let Some((field_cursor_str, type_info)) = type_info {
                FieldCursor::from(field_cursor_str).set_json_value(&mut object, value.unwrap(), type_info, err, &temp_cursor);
            }
        }
        let mut request: api::GoogleCloudDialogflowCxV3TransitionRouteGroup = json::value::from_value(object).unwrap();
        let mut call = self.hub.projects().locations_agents_transition_route_groups_create(request, opt.value_of("parent").unwrap_or(""));
        for parg in opt.values_of("v").map(|i|i.collect()).unwrap_or(Vec::new()).iter() {
            let (key, value) = parse_kv_arg(&*parg, err, false);
            match key {
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
                                                                           v.extend(["language-code"].iter().map(|v|*v));
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

    async fn _projects_locations_agents_transition_route_groups_delete(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        let mut call = self.hub.projects().locations_agents_transition_route_groups_delete(opt.value_of("name").unwrap_or(""));
        for parg in opt.values_of("v").map(|i|i.collect()).unwrap_or(Vec::new()).iter() {
            let (key, value) = parse_kv_arg(&*parg, err, false);
            match key {
                "force" => {
                    call = call.force(        value.map(|v| arg_from_str(v, err, "force", "boolean")).unwrap_or(false));
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
                                                                           v.extend(["force"].iter().map(|v|*v));
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

    async fn _projects_locations_agents_transition_route_groups_get(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        let mut call = self.hub.projects().locations_agents_transition_route_groups_get(opt.value_of("name").unwrap_or(""));
        for parg in opt.values_of("v").map(|i|i.collect()).unwrap_or(Vec::new()).iter() {
            let (key, value) = parse_kv_arg(&*parg, err, false);
            match key {
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
                                                                           v.extend(["language-code"].iter().map(|v|*v));
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

    async fn _projects_locations_agents_transition_route_groups_list(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        let mut call = self.hub.projects().locations_agents_transition_route_groups_list(opt.value_of("parent").unwrap_or(""));
        for parg in opt.values_of("v").map(|i|i.collect()).unwrap_or(Vec::new()).iter() {
            let (key, value) = parse_kv_arg(&*parg, err, false);
            match key {
                "page-token" => {
                    call = call.page_token(value.unwrap_or(""));
                },
                "page-size" => {
                    call = call.page_size(        value.map(|v| arg_from_str(v, err, "page-size", "int32")).unwrap_or(-0));
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
                                                                           v.extend(["language-code", "page-size", "page-token"].iter().map(|v|*v));
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

    async fn _projects_locations_agents_transition_route_groups_patch(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
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
                    "display-name" => Some(("displayName", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "name" => Some(("name", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    _ => {
                        let suggestion = FieldCursor::did_you_mean(key, &vec!["display-name", "name"]);
                        err.issues.push(CLIError::Field(FieldError::Unknown(temp_cursor.to_string(), suggestion, value.map(|v| v.to_string()))));
                        None
                    }
                };
            if let Some((field_cursor_str, type_info)) = type_info {
                FieldCursor::from(field_cursor_str).set_json_value(&mut object, value.unwrap(), type_info, err, &temp_cursor);
            }
        }
        let mut request: api::GoogleCloudDialogflowCxV3TransitionRouteGroup = json::value::from_value(object).unwrap();
        let mut call = self.hub.projects().locations_agents_transition_route_groups_patch(request, opt.value_of("name").unwrap_or(""));
        for parg in opt.values_of("v").map(|i|i.collect()).unwrap_or(Vec::new()).iter() {
            let (key, value) = parse_kv_arg(&*parg, err, false);
            match key {
                "update-mask" => {
                    call = call.update_mask(        value.map(|v| arg_from_str(v, err, "update-mask", "google-fieldmask")).unwrap_or(FieldMask::default()));
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
                                                                           v.extend(["language-code", "update-mask"].iter().map(|v|*v));
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

    async fn _projects_locations_agents_update_generative_settings(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
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
                    "fallback-settings.selected-prompt" => Some(("fallbackSettings.selectedPrompt", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "knowledge-connector-settings.agent" => Some(("knowledgeConnectorSettings.agent", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "knowledge-connector-settings.agent-identity" => Some(("knowledgeConnectorSettings.agentIdentity", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "knowledge-connector-settings.agent-scope" => Some(("knowledgeConnectorSettings.agentScope", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "knowledge-connector-settings.business" => Some(("knowledgeConnectorSettings.business", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "knowledge-connector-settings.business-description" => Some(("knowledgeConnectorSettings.businessDescription", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "knowledge-connector-settings.disable-data-store-fallback" => Some(("knowledgeConnectorSettings.disableDataStoreFallback", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "language-code" => Some(("languageCode", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "name" => Some(("name", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    _ => {
                        let suggestion = FieldCursor::did_you_mean(key, &vec!["agent", "agent-identity", "agent-scope", "business", "business-description", "disable-data-store-fallback", "fallback-settings", "knowledge-connector-settings", "language-code", "name", "selected-prompt"]);
                        err.issues.push(CLIError::Field(FieldError::Unknown(temp_cursor.to_string(), suggestion, value.map(|v| v.to_string()))));
                        None
                    }
                };
            if let Some((field_cursor_str, type_info)) = type_info {
                FieldCursor::from(field_cursor_str).set_json_value(&mut object, value.unwrap(), type_info, err, &temp_cursor);
            }
        }
        let mut request: api::GoogleCloudDialogflowCxV3GenerativeSettings = json::value::from_value(object).unwrap();
        let mut call = self.hub.projects().locations_agents_update_generative_settings(request, opt.value_of("name").unwrap_or(""));
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

    async fn _projects_locations_agents_validate(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
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
                    "language-code" => Some(("languageCode", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    _ => {
                        let suggestion = FieldCursor::did_you_mean(key, &vec!["language-code"]);
                        err.issues.push(CLIError::Field(FieldError::Unknown(temp_cursor.to_string(), suggestion, value.map(|v| v.to_string()))));
                        None
                    }
                };
            if let Some((field_cursor_str, type_info)) = type_info {
                FieldCursor::from(field_cursor_str).set_json_value(&mut object, value.unwrap(), type_info, err, &temp_cursor);
            }
        }
        let mut request: api::GoogleCloudDialogflowCxV3ValidateAgentRequest = json::value::from_value(object).unwrap();
        let mut call = self.hub.projects().locations_agents_validate(request, opt.value_of("name").unwrap_or(""));
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

    async fn _projects_locations_agents_webhooks_create(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
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
                    "disabled" => Some(("disabled", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "display-name" => Some(("displayName", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "generic-web-service.allowed-ca-certs" => Some(("genericWebService.allowedCaCerts", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Vec })),
                    "generic-web-service.http-method" => Some(("genericWebService.httpMethod", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "generic-web-service.oauth-config.client-id" => Some(("genericWebService.oauthConfig.clientId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "generic-web-service.oauth-config.client-secret" => Some(("genericWebService.oauthConfig.clientSecret", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "generic-web-service.oauth-config.scopes" => Some(("genericWebService.oauthConfig.scopes", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Vec })),
                    "generic-web-service.oauth-config.token-endpoint" => Some(("genericWebService.oauthConfig.tokenEndpoint", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "generic-web-service.parameter-mapping" => Some(("genericWebService.parameterMapping", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Map })),
                    "generic-web-service.password" => Some(("genericWebService.password", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "generic-web-service.request-body" => Some(("genericWebService.requestBody", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "generic-web-service.request-headers" => Some(("genericWebService.requestHeaders", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Map })),
                    "generic-web-service.service-agent-auth" => Some(("genericWebService.serviceAgentAuth", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "generic-web-service.uri" => Some(("genericWebService.uri", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "generic-web-service.username" => Some(("genericWebService.username", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "generic-web-service.webhook-type" => Some(("genericWebService.webhookType", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "name" => Some(("name", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "service-directory.generic-web-service.allowed-ca-certs" => Some(("serviceDirectory.genericWebService.allowedCaCerts", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Vec })),
                    "service-directory.generic-web-service.http-method" => Some(("serviceDirectory.genericWebService.httpMethod", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "service-directory.generic-web-service.oauth-config.client-id" => Some(("serviceDirectory.genericWebService.oauthConfig.clientId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "service-directory.generic-web-service.oauth-config.client-secret" => Some(("serviceDirectory.genericWebService.oauthConfig.clientSecret", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "service-directory.generic-web-service.oauth-config.scopes" => Some(("serviceDirectory.genericWebService.oauthConfig.scopes", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Vec })),
                    "service-directory.generic-web-service.oauth-config.token-endpoint" => Some(("serviceDirectory.genericWebService.oauthConfig.tokenEndpoint", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "service-directory.generic-web-service.parameter-mapping" => Some(("serviceDirectory.genericWebService.parameterMapping", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Map })),
                    "service-directory.generic-web-service.password" => Some(("serviceDirectory.genericWebService.password", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "service-directory.generic-web-service.request-body" => Some(("serviceDirectory.genericWebService.requestBody", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "service-directory.generic-web-service.request-headers" => Some(("serviceDirectory.genericWebService.requestHeaders", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Map })),
                    "service-directory.generic-web-service.service-agent-auth" => Some(("serviceDirectory.genericWebService.serviceAgentAuth", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "service-directory.generic-web-service.uri" => Some(("serviceDirectory.genericWebService.uri", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "service-directory.generic-web-service.username" => Some(("serviceDirectory.genericWebService.username", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "service-directory.generic-web-service.webhook-type" => Some(("serviceDirectory.genericWebService.webhookType", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "service-directory.service" => Some(("serviceDirectory.service", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "timeout" => Some(("timeout", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    _ => {
                        let suggestion = FieldCursor::did_you_mean(key, &vec!["allowed-ca-certs", "client-id", "client-secret", "disabled", "display-name", "generic-web-service", "http-method", "name", "oauth-config", "parameter-mapping", "password", "request-body", "request-headers", "scopes", "service", "service-agent-auth", "service-directory", "timeout", "token-endpoint", "uri", "username", "webhook-type"]);
                        err.issues.push(CLIError::Field(FieldError::Unknown(temp_cursor.to_string(), suggestion, value.map(|v| v.to_string()))));
                        None
                    }
                };
            if let Some((field_cursor_str, type_info)) = type_info {
                FieldCursor::from(field_cursor_str).set_json_value(&mut object, value.unwrap(), type_info, err, &temp_cursor);
            }
        }
        let mut request: api::GoogleCloudDialogflowCxV3Webhook = json::value::from_value(object).unwrap();
        let mut call = self.hub.projects().locations_agents_webhooks_create(request, opt.value_of("parent").unwrap_or(""));
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

    async fn _projects_locations_agents_webhooks_delete(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        let mut call = self.hub.projects().locations_agents_webhooks_delete(opt.value_of("name").unwrap_or(""));
        for parg in opt.values_of("v").map(|i|i.collect()).unwrap_or(Vec::new()).iter() {
            let (key, value) = parse_kv_arg(&*parg, err, false);
            match key {
                "force" => {
                    call = call.force(        value.map(|v| arg_from_str(v, err, "force", "boolean")).unwrap_or(false));
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
                                                                           v.extend(["force"].iter().map(|v|*v));
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

    async fn _projects_locations_agents_webhooks_get(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        let mut call = self.hub.projects().locations_agents_webhooks_get(opt.value_of("name").unwrap_or(""));
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

    async fn _projects_locations_agents_webhooks_list(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        let mut call = self.hub.projects().locations_agents_webhooks_list(opt.value_of("parent").unwrap_or(""));
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

    async fn _projects_locations_agents_webhooks_patch(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
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
                    "disabled" => Some(("disabled", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "display-name" => Some(("displayName", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "generic-web-service.allowed-ca-certs" => Some(("genericWebService.allowedCaCerts", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Vec })),
                    "generic-web-service.http-method" => Some(("genericWebService.httpMethod", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "generic-web-service.oauth-config.client-id" => Some(("genericWebService.oauthConfig.clientId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "generic-web-service.oauth-config.client-secret" => Some(("genericWebService.oauthConfig.clientSecret", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "generic-web-service.oauth-config.scopes" => Some(("genericWebService.oauthConfig.scopes", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Vec })),
                    "generic-web-service.oauth-config.token-endpoint" => Some(("genericWebService.oauthConfig.tokenEndpoint", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "generic-web-service.parameter-mapping" => Some(("genericWebService.parameterMapping", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Map })),
                    "generic-web-service.password" => Some(("genericWebService.password", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "generic-web-service.request-body" => Some(("genericWebService.requestBody", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "generic-web-service.request-headers" => Some(("genericWebService.requestHeaders", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Map })),
                    "generic-web-service.service-agent-auth" => Some(("genericWebService.serviceAgentAuth", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "generic-web-service.uri" => Some(("genericWebService.uri", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "generic-web-service.username" => Some(("genericWebService.username", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "generic-web-service.webhook-type" => Some(("genericWebService.webhookType", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "name" => Some(("name", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "service-directory.generic-web-service.allowed-ca-certs" => Some(("serviceDirectory.genericWebService.allowedCaCerts", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Vec })),
                    "service-directory.generic-web-service.http-method" => Some(("serviceDirectory.genericWebService.httpMethod", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "service-directory.generic-web-service.oauth-config.client-id" => Some(("serviceDirectory.genericWebService.oauthConfig.clientId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "service-directory.generic-web-service.oauth-config.client-secret" => Some(("serviceDirectory.genericWebService.oauthConfig.clientSecret", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "service-directory.generic-web-service.oauth-config.scopes" => Some(("serviceDirectory.genericWebService.oauthConfig.scopes", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Vec })),
                    "service-directory.generic-web-service.oauth-config.token-endpoint" => Some(("serviceDirectory.genericWebService.oauthConfig.tokenEndpoint", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "service-directory.generic-web-service.parameter-mapping" => Some(("serviceDirectory.genericWebService.parameterMapping", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Map })),
                    "service-directory.generic-web-service.password" => Some(("serviceDirectory.genericWebService.password", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "service-directory.generic-web-service.request-body" => Some(("serviceDirectory.genericWebService.requestBody", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "service-directory.generic-web-service.request-headers" => Some(("serviceDirectory.genericWebService.requestHeaders", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Map })),
                    "service-directory.generic-web-service.service-agent-auth" => Some(("serviceDirectory.genericWebService.serviceAgentAuth", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "service-directory.generic-web-service.uri" => Some(("serviceDirectory.genericWebService.uri", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "service-directory.generic-web-service.username" => Some(("serviceDirectory.genericWebService.username", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "service-directory.generic-web-service.webhook-type" => Some(("serviceDirectory.genericWebService.webhookType", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "service-directory.service" => Some(("serviceDirectory.service", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "timeout" => Some(("timeout", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    _ => {
                        let suggestion = FieldCursor::did_you_mean(key, &vec!["allowed-ca-certs", "client-id", "client-secret", "disabled", "display-name", "generic-web-service", "http-method", "name", "oauth-config", "parameter-mapping", "password", "request-body", "request-headers", "scopes", "service", "service-agent-auth", "service-directory", "timeout", "token-endpoint", "uri", "username", "webhook-type"]);
                        err.issues.push(CLIError::Field(FieldError::Unknown(temp_cursor.to_string(), suggestion, value.map(|v| v.to_string()))));
                        None
                    }
                };
            if let Some((field_cursor_str, type_info)) = type_info {
                FieldCursor::from(field_cursor_str).set_json_value(&mut object, value.unwrap(), type_info, err, &temp_cursor);
            }
        }
        let mut request: api::GoogleCloudDialogflowCxV3Webhook = json::value::from_value(object).unwrap();
        let mut call = self.hub.projects().locations_agents_webhooks_patch(request, opt.value_of("name").unwrap_or(""));
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

    async fn _projects_locations_get(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        let mut call = self.hub.projects().locations_get(opt.value_of("name").unwrap_or(""));
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

    async fn _projects_locations_list(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        let mut call = self.hub.projects().locations_list(opt.value_of("name").unwrap_or(""));
        for parg in opt.values_of("v").map(|i|i.collect()).unwrap_or(Vec::new()).iter() {
            let (key, value) = parse_kv_arg(&*parg, err, false);
            match key {
                "page-token" => {
                    call = call.page_token(value.unwrap_or(""));
                },
                "page-size" => {
                    call = call.page_size(        value.map(|v| arg_from_str(v, err, "page-size", "int32")).unwrap_or(-0));
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
                                                                           v.extend(["filter", "page-size", "page-token"].iter().map(|v|*v));
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

    async fn _projects_locations_operations_cancel(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        let mut call = self.hub.projects().locations_operations_cancel(opt.value_of("name").unwrap_or(""));
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

    async fn _projects_locations_operations_get(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        let mut call = self.hub.projects().locations_operations_get(opt.value_of("name").unwrap_or(""));
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

    async fn _projects_locations_operations_list(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        let mut call = self.hub.projects().locations_operations_list(opt.value_of("name").unwrap_or(""));
        for parg in opt.values_of("v").map(|i|i.collect()).unwrap_or(Vec::new()).iter() {
            let (key, value) = parse_kv_arg(&*parg, err, false);
            match key {
                "page-token" => {
                    call = call.page_token(value.unwrap_or(""));
                },
                "page-size" => {
                    call = call.page_size(        value.map(|v| arg_from_str(v, err, "page-size", "int32")).unwrap_or(-0));
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
                                                                           v.extend(["filter", "page-size", "page-token"].iter().map(|v|*v));
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

    async fn _projects_locations_security_settings_create(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
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
                    "audio-export-settings.audio-export-pattern" => Some(("audioExportSettings.audioExportPattern", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "audio-export-settings.audio-format" => Some(("audioExportSettings.audioFormat", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "audio-export-settings.enable-audio-redaction" => Some(("audioExportSettings.enableAudioRedaction", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "audio-export-settings.gcs-bucket" => Some(("audioExportSettings.gcsBucket", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "audio-export-settings.store-tts-audio" => Some(("audioExportSettings.storeTtsAudio", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "deidentify-template" => Some(("deidentifyTemplate", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "display-name" => Some(("displayName", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "insights-export-settings.enable-insights-export" => Some(("insightsExportSettings.enableInsightsExport", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "inspect-template" => Some(("inspectTemplate", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "name" => Some(("name", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "purge-data-types" => Some(("purgeDataTypes", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Vec })),
                    "redaction-scope" => Some(("redactionScope", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "redaction-strategy" => Some(("redactionStrategy", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "retention-strategy" => Some(("retentionStrategy", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "retention-window-days" => Some(("retentionWindowDays", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    _ => {
                        let suggestion = FieldCursor::did_you_mean(key, &vec!["audio-export-pattern", "audio-export-settings", "audio-format", "deidentify-template", "display-name", "enable-audio-redaction", "enable-insights-export", "gcs-bucket", "insights-export-settings", "inspect-template", "name", "purge-data-types", "redaction-scope", "redaction-strategy", "retention-strategy", "retention-window-days", "store-tts-audio"]);
                        err.issues.push(CLIError::Field(FieldError::Unknown(temp_cursor.to_string(), suggestion, value.map(|v| v.to_string()))));
                        None
                    }
                };
            if let Some((field_cursor_str, type_info)) = type_info {
                FieldCursor::from(field_cursor_str).set_json_value(&mut object, value.unwrap(), type_info, err, &temp_cursor);
            }
        }
        let mut request: api::GoogleCloudDialogflowCxV3SecuritySettings = json::value::from_value(object).unwrap();
        let mut call = self.hub.projects().locations_security_settings_create(request, opt.value_of("parent").unwrap_or(""));
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

    async fn _projects_locations_security_settings_delete(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        let mut call = self.hub.projects().locations_security_settings_delete(opt.value_of("name").unwrap_or(""));
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

    async fn _projects_locations_security_settings_get(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        let mut call = self.hub.projects().locations_security_settings_get(opt.value_of("name").unwrap_or(""));
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

    async fn _projects_locations_security_settings_list(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        let mut call = self.hub.projects().locations_security_settings_list(opt.value_of("parent").unwrap_or(""));
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

    async fn _projects_locations_security_settings_patch(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
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
                    "audio-export-settings.audio-export-pattern" => Some(("audioExportSettings.audioExportPattern", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "audio-export-settings.audio-format" => Some(("audioExportSettings.audioFormat", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "audio-export-settings.enable-audio-redaction" => Some(("audioExportSettings.enableAudioRedaction", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "audio-export-settings.gcs-bucket" => Some(("audioExportSettings.gcsBucket", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "audio-export-settings.store-tts-audio" => Some(("audioExportSettings.storeTtsAudio", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "deidentify-template" => Some(("deidentifyTemplate", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "display-name" => Some(("displayName", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "insights-export-settings.enable-insights-export" => Some(("insightsExportSettings.enableInsightsExport", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "inspect-template" => Some(("inspectTemplate", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "name" => Some(("name", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "purge-data-types" => Some(("purgeDataTypes", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Vec })),
                    "redaction-scope" => Some(("redactionScope", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "redaction-strategy" => Some(("redactionStrategy", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "retention-strategy" => Some(("retentionStrategy", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "retention-window-days" => Some(("retentionWindowDays", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    _ => {
                        let suggestion = FieldCursor::did_you_mean(key, &vec!["audio-export-pattern", "audio-export-settings", "audio-format", "deidentify-template", "display-name", "enable-audio-redaction", "enable-insights-export", "gcs-bucket", "insights-export-settings", "inspect-template", "name", "purge-data-types", "redaction-scope", "redaction-strategy", "retention-strategy", "retention-window-days", "store-tts-audio"]);
                        err.issues.push(CLIError::Field(FieldError::Unknown(temp_cursor.to_string(), suggestion, value.map(|v| v.to_string()))));
                        None
                    }
                };
            if let Some((field_cursor_str, type_info)) = type_info {
                FieldCursor::from(field_cursor_str).set_json_value(&mut object, value.unwrap(), type_info, err, &temp_cursor);
            }
        }
        let mut request: api::GoogleCloudDialogflowCxV3SecuritySettings = json::value::from_value(object).unwrap();
        let mut call = self.hub.projects().locations_security_settings_patch(request, opt.value_of("name").unwrap_or(""));
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

    async fn _projects_operations_cancel(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        let mut call = self.hub.projects().operations_cancel(opt.value_of("name").unwrap_or(""));
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

    async fn _projects_operations_get(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        let mut call = self.hub.projects().operations_get(opt.value_of("name").unwrap_or(""));
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

    async fn _projects_operations_list(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        let mut call = self.hub.projects().operations_list(opt.value_of("name").unwrap_or(""));
        for parg in opt.values_of("v").map(|i|i.collect()).unwrap_or(Vec::new()).iter() {
            let (key, value) = parse_kv_arg(&*parg, err, false);
            match key {
                "page-token" => {
                    call = call.page_token(value.unwrap_or(""));
                },
                "page-size" => {
                    call = call.page_size(        value.map(|v| arg_from_str(v, err, "page-size", "int32")).unwrap_or(-0));
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
                                                                           v.extend(["filter", "page-size", "page-token"].iter().map(|v|*v));
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
                    ("locations-agents-changelogs-get", Some(opt)) => {
                        call_result = self._projects_locations_agents_changelogs_get(opt, dry_run, &mut err).await;
                    },
                    ("locations-agents-changelogs-list", Some(opt)) => {
                        call_result = self._projects_locations_agents_changelogs_list(opt, dry_run, &mut err).await;
                    },
                    ("locations-agents-create", Some(opt)) => {
                        call_result = self._projects_locations_agents_create(opt, dry_run, &mut err).await;
                    },
                    ("locations-agents-delete", Some(opt)) => {
                        call_result = self._projects_locations_agents_delete(opt, dry_run, &mut err).await;
                    },
                    ("locations-agents-entity-types-create", Some(opt)) => {
                        call_result = self._projects_locations_agents_entity_types_create(opt, dry_run, &mut err).await;
                    },
                    ("locations-agents-entity-types-delete", Some(opt)) => {
                        call_result = self._projects_locations_agents_entity_types_delete(opt, dry_run, &mut err).await;
                    },
                    ("locations-agents-entity-types-export", Some(opt)) => {
                        call_result = self._projects_locations_agents_entity_types_export(opt, dry_run, &mut err).await;
                    },
                    ("locations-agents-entity-types-get", Some(opt)) => {
                        call_result = self._projects_locations_agents_entity_types_get(opt, dry_run, &mut err).await;
                    },
                    ("locations-agents-entity-types-import", Some(opt)) => {
                        call_result = self._projects_locations_agents_entity_types_import(opt, dry_run, &mut err).await;
                    },
                    ("locations-agents-entity-types-list", Some(opt)) => {
                        call_result = self._projects_locations_agents_entity_types_list(opt, dry_run, &mut err).await;
                    },
                    ("locations-agents-entity-types-patch", Some(opt)) => {
                        call_result = self._projects_locations_agents_entity_types_patch(opt, dry_run, &mut err).await;
                    },
                    ("locations-agents-environments-continuous-test-results-list", Some(opt)) => {
                        call_result = self._projects_locations_agents_environments_continuous_test_results_list(opt, dry_run, &mut err).await;
                    },
                    ("locations-agents-environments-create", Some(opt)) => {
                        call_result = self._projects_locations_agents_environments_create(opt, dry_run, &mut err).await;
                    },
                    ("locations-agents-environments-delete", Some(opt)) => {
                        call_result = self._projects_locations_agents_environments_delete(opt, dry_run, &mut err).await;
                    },
                    ("locations-agents-environments-deploy-flow", Some(opt)) => {
                        call_result = self._projects_locations_agents_environments_deploy_flow(opt, dry_run, &mut err).await;
                    },
                    ("locations-agents-environments-deployments-get", Some(opt)) => {
                        call_result = self._projects_locations_agents_environments_deployments_get(opt, dry_run, &mut err).await;
                    },
                    ("locations-agents-environments-deployments-list", Some(opt)) => {
                        call_result = self._projects_locations_agents_environments_deployments_list(opt, dry_run, &mut err).await;
                    },
                    ("locations-agents-environments-experiments-create", Some(opt)) => {
                        call_result = self._projects_locations_agents_environments_experiments_create(opt, dry_run, &mut err).await;
                    },
                    ("locations-agents-environments-experiments-delete", Some(opt)) => {
                        call_result = self._projects_locations_agents_environments_experiments_delete(opt, dry_run, &mut err).await;
                    },
                    ("locations-agents-environments-experiments-get", Some(opt)) => {
                        call_result = self._projects_locations_agents_environments_experiments_get(opt, dry_run, &mut err).await;
                    },
                    ("locations-agents-environments-experiments-list", Some(opt)) => {
                        call_result = self._projects_locations_agents_environments_experiments_list(opt, dry_run, &mut err).await;
                    },
                    ("locations-agents-environments-experiments-patch", Some(opt)) => {
                        call_result = self._projects_locations_agents_environments_experiments_patch(opt, dry_run, &mut err).await;
                    },
                    ("locations-agents-environments-experiments-start", Some(opt)) => {
                        call_result = self._projects_locations_agents_environments_experiments_start(opt, dry_run, &mut err).await;
                    },
                    ("locations-agents-environments-experiments-stop", Some(opt)) => {
                        call_result = self._projects_locations_agents_environments_experiments_stop(opt, dry_run, &mut err).await;
                    },
                    ("locations-agents-environments-get", Some(opt)) => {
                        call_result = self._projects_locations_agents_environments_get(opt, dry_run, &mut err).await;
                    },
                    ("locations-agents-environments-list", Some(opt)) => {
                        call_result = self._projects_locations_agents_environments_list(opt, dry_run, &mut err).await;
                    },
                    ("locations-agents-environments-lookup-environment-history", Some(opt)) => {
                        call_result = self._projects_locations_agents_environments_lookup_environment_history(opt, dry_run, &mut err).await;
                    },
                    ("locations-agents-environments-patch", Some(opt)) => {
                        call_result = self._projects_locations_agents_environments_patch(opt, dry_run, &mut err).await;
                    },
                    ("locations-agents-environments-run-continuous-test", Some(opt)) => {
                        call_result = self._projects_locations_agents_environments_run_continuous_test(opt, dry_run, &mut err).await;
                    },
                    ("locations-agents-environments-sessions-detect-intent", Some(opt)) => {
                        call_result = self._projects_locations_agents_environments_sessions_detect_intent(opt, dry_run, &mut err).await;
                    },
                    ("locations-agents-environments-sessions-entity-types-create", Some(opt)) => {
                        call_result = self._projects_locations_agents_environments_sessions_entity_types_create(opt, dry_run, &mut err).await;
                    },
                    ("locations-agents-environments-sessions-entity-types-delete", Some(opt)) => {
                        call_result = self._projects_locations_agents_environments_sessions_entity_types_delete(opt, dry_run, &mut err).await;
                    },
                    ("locations-agents-environments-sessions-entity-types-get", Some(opt)) => {
                        call_result = self._projects_locations_agents_environments_sessions_entity_types_get(opt, dry_run, &mut err).await;
                    },
                    ("locations-agents-environments-sessions-entity-types-list", Some(opt)) => {
                        call_result = self._projects_locations_agents_environments_sessions_entity_types_list(opt, dry_run, &mut err).await;
                    },
                    ("locations-agents-environments-sessions-entity-types-patch", Some(opt)) => {
                        call_result = self._projects_locations_agents_environments_sessions_entity_types_patch(opt, dry_run, &mut err).await;
                    },
                    ("locations-agents-environments-sessions-fulfill-intent", Some(opt)) => {
                        call_result = self._projects_locations_agents_environments_sessions_fulfill_intent(opt, dry_run, &mut err).await;
                    },
                    ("locations-agents-environments-sessions-match-intent", Some(opt)) => {
                        call_result = self._projects_locations_agents_environments_sessions_match_intent(opt, dry_run, &mut err).await;
                    },
                    ("locations-agents-environments-sessions-server-streaming-detect-intent", Some(opt)) => {
                        call_result = self._projects_locations_agents_environments_sessions_server_streaming_detect_intent(opt, dry_run, &mut err).await;
                    },
                    ("locations-agents-export", Some(opt)) => {
                        call_result = self._projects_locations_agents_export(opt, dry_run, &mut err).await;
                    },
                    ("locations-agents-flows-create", Some(opt)) => {
                        call_result = self._projects_locations_agents_flows_create(opt, dry_run, &mut err).await;
                    },
                    ("locations-agents-flows-delete", Some(opt)) => {
                        call_result = self._projects_locations_agents_flows_delete(opt, dry_run, &mut err).await;
                    },
                    ("locations-agents-flows-export", Some(opt)) => {
                        call_result = self._projects_locations_agents_flows_export(opt, dry_run, &mut err).await;
                    },
                    ("locations-agents-flows-get", Some(opt)) => {
                        call_result = self._projects_locations_agents_flows_get(opt, dry_run, &mut err).await;
                    },
                    ("locations-agents-flows-get-validation-result", Some(opt)) => {
                        call_result = self._projects_locations_agents_flows_get_validation_result(opt, dry_run, &mut err).await;
                    },
                    ("locations-agents-flows-import", Some(opt)) => {
                        call_result = self._projects_locations_agents_flows_import(opt, dry_run, &mut err).await;
                    },
                    ("locations-agents-flows-list", Some(opt)) => {
                        call_result = self._projects_locations_agents_flows_list(opt, dry_run, &mut err).await;
                    },
                    ("locations-agents-flows-pages-create", Some(opt)) => {
                        call_result = self._projects_locations_agents_flows_pages_create(opt, dry_run, &mut err).await;
                    },
                    ("locations-agents-flows-pages-delete", Some(opt)) => {
                        call_result = self._projects_locations_agents_flows_pages_delete(opt, dry_run, &mut err).await;
                    },
                    ("locations-agents-flows-pages-get", Some(opt)) => {
                        call_result = self._projects_locations_agents_flows_pages_get(opt, dry_run, &mut err).await;
                    },
                    ("locations-agents-flows-pages-list", Some(opt)) => {
                        call_result = self._projects_locations_agents_flows_pages_list(opt, dry_run, &mut err).await;
                    },
                    ("locations-agents-flows-pages-patch", Some(opt)) => {
                        call_result = self._projects_locations_agents_flows_pages_patch(opt, dry_run, &mut err).await;
                    },
                    ("locations-agents-flows-patch", Some(opt)) => {
                        call_result = self._projects_locations_agents_flows_patch(opt, dry_run, &mut err).await;
                    },
                    ("locations-agents-flows-train", Some(opt)) => {
                        call_result = self._projects_locations_agents_flows_train(opt, dry_run, &mut err).await;
                    },
                    ("locations-agents-flows-transition-route-groups-create", Some(opt)) => {
                        call_result = self._projects_locations_agents_flows_transition_route_groups_create(opt, dry_run, &mut err).await;
                    },
                    ("locations-agents-flows-transition-route-groups-delete", Some(opt)) => {
                        call_result = self._projects_locations_agents_flows_transition_route_groups_delete(opt, dry_run, &mut err).await;
                    },
                    ("locations-agents-flows-transition-route-groups-get", Some(opt)) => {
                        call_result = self._projects_locations_agents_flows_transition_route_groups_get(opt, dry_run, &mut err).await;
                    },
                    ("locations-agents-flows-transition-route-groups-list", Some(opt)) => {
                        call_result = self._projects_locations_agents_flows_transition_route_groups_list(opt, dry_run, &mut err).await;
                    },
                    ("locations-agents-flows-transition-route-groups-patch", Some(opt)) => {
                        call_result = self._projects_locations_agents_flows_transition_route_groups_patch(opt, dry_run, &mut err).await;
                    },
                    ("locations-agents-flows-validate", Some(opt)) => {
                        call_result = self._projects_locations_agents_flows_validate(opt, dry_run, &mut err).await;
                    },
                    ("locations-agents-flows-versions-compare-versions", Some(opt)) => {
                        call_result = self._projects_locations_agents_flows_versions_compare_versions(opt, dry_run, &mut err).await;
                    },
                    ("locations-agents-flows-versions-create", Some(opt)) => {
                        call_result = self._projects_locations_agents_flows_versions_create(opt, dry_run, &mut err).await;
                    },
                    ("locations-agents-flows-versions-delete", Some(opt)) => {
                        call_result = self._projects_locations_agents_flows_versions_delete(opt, dry_run, &mut err).await;
                    },
                    ("locations-agents-flows-versions-get", Some(opt)) => {
                        call_result = self._projects_locations_agents_flows_versions_get(opt, dry_run, &mut err).await;
                    },
                    ("locations-agents-flows-versions-list", Some(opt)) => {
                        call_result = self._projects_locations_agents_flows_versions_list(opt, dry_run, &mut err).await;
                    },
                    ("locations-agents-flows-versions-load", Some(opt)) => {
                        call_result = self._projects_locations_agents_flows_versions_load(opt, dry_run, &mut err).await;
                    },
                    ("locations-agents-flows-versions-patch", Some(opt)) => {
                        call_result = self._projects_locations_agents_flows_versions_patch(opt, dry_run, &mut err).await;
                    },
                    ("locations-agents-generators-create", Some(opt)) => {
                        call_result = self._projects_locations_agents_generators_create(opt, dry_run, &mut err).await;
                    },
                    ("locations-agents-generators-delete", Some(opt)) => {
                        call_result = self._projects_locations_agents_generators_delete(opt, dry_run, &mut err).await;
                    },
                    ("locations-agents-generators-get", Some(opt)) => {
                        call_result = self._projects_locations_agents_generators_get(opt, dry_run, &mut err).await;
                    },
                    ("locations-agents-generators-list", Some(opt)) => {
                        call_result = self._projects_locations_agents_generators_list(opt, dry_run, &mut err).await;
                    },
                    ("locations-agents-generators-patch", Some(opt)) => {
                        call_result = self._projects_locations_agents_generators_patch(opt, dry_run, &mut err).await;
                    },
                    ("locations-agents-get", Some(opt)) => {
                        call_result = self._projects_locations_agents_get(opt, dry_run, &mut err).await;
                    },
                    ("locations-agents-get-generative-settings", Some(opt)) => {
                        call_result = self._projects_locations_agents_get_generative_settings(opt, dry_run, &mut err).await;
                    },
                    ("locations-agents-get-validation-result", Some(opt)) => {
                        call_result = self._projects_locations_agents_get_validation_result(opt, dry_run, &mut err).await;
                    },
                    ("locations-agents-intents-create", Some(opt)) => {
                        call_result = self._projects_locations_agents_intents_create(opt, dry_run, &mut err).await;
                    },
                    ("locations-agents-intents-delete", Some(opt)) => {
                        call_result = self._projects_locations_agents_intents_delete(opt, dry_run, &mut err).await;
                    },
                    ("locations-agents-intents-export", Some(opt)) => {
                        call_result = self._projects_locations_agents_intents_export(opt, dry_run, &mut err).await;
                    },
                    ("locations-agents-intents-get", Some(opt)) => {
                        call_result = self._projects_locations_agents_intents_get(opt, dry_run, &mut err).await;
                    },
                    ("locations-agents-intents-import", Some(opt)) => {
                        call_result = self._projects_locations_agents_intents_import(opt, dry_run, &mut err).await;
                    },
                    ("locations-agents-intents-list", Some(opt)) => {
                        call_result = self._projects_locations_agents_intents_list(opt, dry_run, &mut err).await;
                    },
                    ("locations-agents-intents-patch", Some(opt)) => {
                        call_result = self._projects_locations_agents_intents_patch(opt, dry_run, &mut err).await;
                    },
                    ("locations-agents-list", Some(opt)) => {
                        call_result = self._projects_locations_agents_list(opt, dry_run, &mut err).await;
                    },
                    ("locations-agents-patch", Some(opt)) => {
                        call_result = self._projects_locations_agents_patch(opt, dry_run, &mut err).await;
                    },
                    ("locations-agents-restore", Some(opt)) => {
                        call_result = self._projects_locations_agents_restore(opt, dry_run, &mut err).await;
                    },
                    ("locations-agents-sessions-detect-intent", Some(opt)) => {
                        call_result = self._projects_locations_agents_sessions_detect_intent(opt, dry_run, &mut err).await;
                    },
                    ("locations-agents-sessions-entity-types-create", Some(opt)) => {
                        call_result = self._projects_locations_agents_sessions_entity_types_create(opt, dry_run, &mut err).await;
                    },
                    ("locations-agents-sessions-entity-types-delete", Some(opt)) => {
                        call_result = self._projects_locations_agents_sessions_entity_types_delete(opt, dry_run, &mut err).await;
                    },
                    ("locations-agents-sessions-entity-types-get", Some(opt)) => {
                        call_result = self._projects_locations_agents_sessions_entity_types_get(opt, dry_run, &mut err).await;
                    },
                    ("locations-agents-sessions-entity-types-list", Some(opt)) => {
                        call_result = self._projects_locations_agents_sessions_entity_types_list(opt, dry_run, &mut err).await;
                    },
                    ("locations-agents-sessions-entity-types-patch", Some(opt)) => {
                        call_result = self._projects_locations_agents_sessions_entity_types_patch(opt, dry_run, &mut err).await;
                    },
                    ("locations-agents-sessions-fulfill-intent", Some(opt)) => {
                        call_result = self._projects_locations_agents_sessions_fulfill_intent(opt, dry_run, &mut err).await;
                    },
                    ("locations-agents-sessions-match-intent", Some(opt)) => {
                        call_result = self._projects_locations_agents_sessions_match_intent(opt, dry_run, &mut err).await;
                    },
                    ("locations-agents-sessions-server-streaming-detect-intent", Some(opt)) => {
                        call_result = self._projects_locations_agents_sessions_server_streaming_detect_intent(opt, dry_run, &mut err).await;
                    },
                    ("locations-agents-sessions-submit-answer-feedback", Some(opt)) => {
                        call_result = self._projects_locations_agents_sessions_submit_answer_feedback(opt, dry_run, &mut err).await;
                    },
                    ("locations-agents-test-cases-batch-delete", Some(opt)) => {
                        call_result = self._projects_locations_agents_test_cases_batch_delete(opt, dry_run, &mut err).await;
                    },
                    ("locations-agents-test-cases-batch-run", Some(opt)) => {
                        call_result = self._projects_locations_agents_test_cases_batch_run(opt, dry_run, &mut err).await;
                    },
                    ("locations-agents-test-cases-calculate-coverage", Some(opt)) => {
                        call_result = self._projects_locations_agents_test_cases_calculate_coverage(opt, dry_run, &mut err).await;
                    },
                    ("locations-agents-test-cases-create", Some(opt)) => {
                        call_result = self._projects_locations_agents_test_cases_create(opt, dry_run, &mut err).await;
                    },
                    ("locations-agents-test-cases-export", Some(opt)) => {
                        call_result = self._projects_locations_agents_test_cases_export(opt, dry_run, &mut err).await;
                    },
                    ("locations-agents-test-cases-get", Some(opt)) => {
                        call_result = self._projects_locations_agents_test_cases_get(opt, dry_run, &mut err).await;
                    },
                    ("locations-agents-test-cases-import", Some(opt)) => {
                        call_result = self._projects_locations_agents_test_cases_import(opt, dry_run, &mut err).await;
                    },
                    ("locations-agents-test-cases-list", Some(opt)) => {
                        call_result = self._projects_locations_agents_test_cases_list(opt, dry_run, &mut err).await;
                    },
                    ("locations-agents-test-cases-patch", Some(opt)) => {
                        call_result = self._projects_locations_agents_test_cases_patch(opt, dry_run, &mut err).await;
                    },
                    ("locations-agents-test-cases-results-get", Some(opt)) => {
                        call_result = self._projects_locations_agents_test_cases_results_get(opt, dry_run, &mut err).await;
                    },
                    ("locations-agents-test-cases-results-list", Some(opt)) => {
                        call_result = self._projects_locations_agents_test_cases_results_list(opt, dry_run, &mut err).await;
                    },
                    ("locations-agents-test-cases-run", Some(opt)) => {
                        call_result = self._projects_locations_agents_test_cases_run(opt, dry_run, &mut err).await;
                    },
                    ("locations-agents-transition-route-groups-create", Some(opt)) => {
                        call_result = self._projects_locations_agents_transition_route_groups_create(opt, dry_run, &mut err).await;
                    },
                    ("locations-agents-transition-route-groups-delete", Some(opt)) => {
                        call_result = self._projects_locations_agents_transition_route_groups_delete(opt, dry_run, &mut err).await;
                    },
                    ("locations-agents-transition-route-groups-get", Some(opt)) => {
                        call_result = self._projects_locations_agents_transition_route_groups_get(opt, dry_run, &mut err).await;
                    },
                    ("locations-agents-transition-route-groups-list", Some(opt)) => {
                        call_result = self._projects_locations_agents_transition_route_groups_list(opt, dry_run, &mut err).await;
                    },
                    ("locations-agents-transition-route-groups-patch", Some(opt)) => {
                        call_result = self._projects_locations_agents_transition_route_groups_patch(opt, dry_run, &mut err).await;
                    },
                    ("locations-agents-update-generative-settings", Some(opt)) => {
                        call_result = self._projects_locations_agents_update_generative_settings(opt, dry_run, &mut err).await;
                    },
                    ("locations-agents-validate", Some(opt)) => {
                        call_result = self._projects_locations_agents_validate(opt, dry_run, &mut err).await;
                    },
                    ("locations-agents-webhooks-create", Some(opt)) => {
                        call_result = self._projects_locations_agents_webhooks_create(opt, dry_run, &mut err).await;
                    },
                    ("locations-agents-webhooks-delete", Some(opt)) => {
                        call_result = self._projects_locations_agents_webhooks_delete(opt, dry_run, &mut err).await;
                    },
                    ("locations-agents-webhooks-get", Some(opt)) => {
                        call_result = self._projects_locations_agents_webhooks_get(opt, dry_run, &mut err).await;
                    },
                    ("locations-agents-webhooks-list", Some(opt)) => {
                        call_result = self._projects_locations_agents_webhooks_list(opt, dry_run, &mut err).await;
                    },
                    ("locations-agents-webhooks-patch", Some(opt)) => {
                        call_result = self._projects_locations_agents_webhooks_patch(opt, dry_run, &mut err).await;
                    },
                    ("locations-get", Some(opt)) => {
                        call_result = self._projects_locations_get(opt, dry_run, &mut err).await;
                    },
                    ("locations-list", Some(opt)) => {
                        call_result = self._projects_locations_list(opt, dry_run, &mut err).await;
                    },
                    ("locations-operations-cancel", Some(opt)) => {
                        call_result = self._projects_locations_operations_cancel(opt, dry_run, &mut err).await;
                    },
                    ("locations-operations-get", Some(opt)) => {
                        call_result = self._projects_locations_operations_get(opt, dry_run, &mut err).await;
                    },
                    ("locations-operations-list", Some(opt)) => {
                        call_result = self._projects_locations_operations_list(opt, dry_run, &mut err).await;
                    },
                    ("locations-security-settings-create", Some(opt)) => {
                        call_result = self._projects_locations_security_settings_create(opt, dry_run, &mut err).await;
                    },
                    ("locations-security-settings-delete", Some(opt)) => {
                        call_result = self._projects_locations_security_settings_delete(opt, dry_run, &mut err).await;
                    },
                    ("locations-security-settings-get", Some(opt)) => {
                        call_result = self._projects_locations_security_settings_get(opt, dry_run, &mut err).await;
                    },
                    ("locations-security-settings-list", Some(opt)) => {
                        call_result = self._projects_locations_security_settings_list(opt, dry_run, &mut err).await;
                    },
                    ("locations-security-settings-patch", Some(opt)) => {
                        call_result = self._projects_locations_security_settings_patch(opt, dry_run, &mut err).await;
                    },
                    ("operations-cancel", Some(opt)) => {
                        call_result = self._projects_operations_cancel(opt, dry_run, &mut err).await;
                    },
                    ("operations-get", Some(opt)) => {
                        call_result = self._projects_operations_get(opt, dry_run, &mut err).await;
                    },
                    ("operations-list", Some(opt)) => {
                        call_result = self._projects_operations_list(opt, dry_run, &mut err).await;
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

            match client::application_secret_from_directory(&config_dir, "dialogflow3-secret.json",
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
        ).persist_tokens_to_disk(format!("{}/dialogflow3", config_dir)).build().await.unwrap();

        let engine = Engine {
            opt: opt,
            hub: api::Dialogflow::new(client, auth),
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
        ("projects", "methods: 'locations-agents-changelogs-get', 'locations-agents-changelogs-list', 'locations-agents-create', 'locations-agents-delete', 'locations-agents-entity-types-create', 'locations-agents-entity-types-delete', 'locations-agents-entity-types-export', 'locations-agents-entity-types-get', 'locations-agents-entity-types-import', 'locations-agents-entity-types-list', 'locations-agents-entity-types-patch', 'locations-agents-environments-continuous-test-results-list', 'locations-agents-environments-create', 'locations-agents-environments-delete', 'locations-agents-environments-deploy-flow', 'locations-agents-environments-deployments-get', 'locations-agents-environments-deployments-list', 'locations-agents-environments-experiments-create', 'locations-agents-environments-experiments-delete', 'locations-agents-environments-experiments-get', 'locations-agents-environments-experiments-list', 'locations-agents-environments-experiments-patch', 'locations-agents-environments-experiments-start', 'locations-agents-environments-experiments-stop', 'locations-agents-environments-get', 'locations-agents-environments-list', 'locations-agents-environments-lookup-environment-history', 'locations-agents-environments-patch', 'locations-agents-environments-run-continuous-test', 'locations-agents-environments-sessions-detect-intent', 'locations-agents-environments-sessions-entity-types-create', 'locations-agents-environments-sessions-entity-types-delete', 'locations-agents-environments-sessions-entity-types-get', 'locations-agents-environments-sessions-entity-types-list', 'locations-agents-environments-sessions-entity-types-patch', 'locations-agents-environments-sessions-fulfill-intent', 'locations-agents-environments-sessions-match-intent', 'locations-agents-environments-sessions-server-streaming-detect-intent', 'locations-agents-export', 'locations-agents-flows-create', 'locations-agents-flows-delete', 'locations-agents-flows-export', 'locations-agents-flows-get', 'locations-agents-flows-get-validation-result', 'locations-agents-flows-import', 'locations-agents-flows-list', 'locations-agents-flows-pages-create', 'locations-agents-flows-pages-delete', 'locations-agents-flows-pages-get', 'locations-agents-flows-pages-list', 'locations-agents-flows-pages-patch', 'locations-agents-flows-patch', 'locations-agents-flows-train', 'locations-agents-flows-transition-route-groups-create', 'locations-agents-flows-transition-route-groups-delete', 'locations-agents-flows-transition-route-groups-get', 'locations-agents-flows-transition-route-groups-list', 'locations-agents-flows-transition-route-groups-patch', 'locations-agents-flows-validate', 'locations-agents-flows-versions-compare-versions', 'locations-agents-flows-versions-create', 'locations-agents-flows-versions-delete', 'locations-agents-flows-versions-get', 'locations-agents-flows-versions-list', 'locations-agents-flows-versions-load', 'locations-agents-flows-versions-patch', 'locations-agents-generators-create', 'locations-agents-generators-delete', 'locations-agents-generators-get', 'locations-agents-generators-list', 'locations-agents-generators-patch', 'locations-agents-get', 'locations-agents-get-generative-settings', 'locations-agents-get-validation-result', 'locations-agents-intents-create', 'locations-agents-intents-delete', 'locations-agents-intents-export', 'locations-agents-intents-get', 'locations-agents-intents-import', 'locations-agents-intents-list', 'locations-agents-intents-patch', 'locations-agents-list', 'locations-agents-patch', 'locations-agents-restore', 'locations-agents-sessions-detect-intent', 'locations-agents-sessions-entity-types-create', 'locations-agents-sessions-entity-types-delete', 'locations-agents-sessions-entity-types-get', 'locations-agents-sessions-entity-types-list', 'locations-agents-sessions-entity-types-patch', 'locations-agents-sessions-fulfill-intent', 'locations-agents-sessions-match-intent', 'locations-agents-sessions-server-streaming-detect-intent', 'locations-agents-sessions-submit-answer-feedback', 'locations-agents-test-cases-batch-delete', 'locations-agents-test-cases-batch-run', 'locations-agents-test-cases-calculate-coverage', 'locations-agents-test-cases-create', 'locations-agents-test-cases-export', 'locations-agents-test-cases-get', 'locations-agents-test-cases-import', 'locations-agents-test-cases-list', 'locations-agents-test-cases-patch', 'locations-agents-test-cases-results-get', 'locations-agents-test-cases-results-list', 'locations-agents-test-cases-run', 'locations-agents-transition-route-groups-create', 'locations-agents-transition-route-groups-delete', 'locations-agents-transition-route-groups-get', 'locations-agents-transition-route-groups-list', 'locations-agents-transition-route-groups-patch', 'locations-agents-update-generative-settings', 'locations-agents-validate', 'locations-agents-webhooks-create', 'locations-agents-webhooks-delete', 'locations-agents-webhooks-get', 'locations-agents-webhooks-list', 'locations-agents-webhooks-patch', 'locations-get', 'locations-list', 'locations-operations-cancel', 'locations-operations-get', 'locations-operations-list', 'locations-security-settings-create', 'locations-security-settings-delete', 'locations-security-settings-get', 'locations-security-settings-list', 'locations-security-settings-patch', 'operations-cancel', 'operations-get' and 'operations-list'", vec![
            ("locations-agents-changelogs-get",
                    Some(r##"Retrieves the specified Changelog."##),
                    "Details at http://byron.github.io/google-apis-rs/google_dialogflow3_cli/projects_locations-agents-changelogs-get",
                  vec![
                    (Some(r##"name"##),
                     None,
                     Some(r##"Required. The name of the changelog to get. Format: `projects//locations//agents//changelogs/`."##),
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
            ("locations-agents-changelogs-list",
                    Some(r##"Returns the list of Changelogs."##),
                    "Details at http://byron.github.io/google-apis-rs/google_dialogflow3_cli/projects_locations-agents-changelogs-list",
                  vec![
                    (Some(r##"parent"##),
                     None,
                     Some(r##"Required. The agent containing the changelogs. Format: `projects//locations//agents/`."##),
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
            ("locations-agents-create",
                    Some(r##"Creates an agent in the specified location. Note: You should always train flows prior to sending them queries. See the [training documentation](https://cloud.google.com/dialogflow/cx/docs/concept/training)."##),
                    "Details at http://byron.github.io/google-apis-rs/google_dialogflow3_cli/projects_locations-agents-create",
                  vec![
                    (Some(r##"parent"##),
                     None,
                     Some(r##"Required. The location to create a agent for. Format: `projects//locations/`."##),
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
            ("locations-agents-delete",
                    Some(r##"Deletes the specified agent."##),
                    "Details at http://byron.github.io/google-apis-rs/google_dialogflow3_cli/projects_locations-agents-delete",
                  vec![
                    (Some(r##"name"##),
                     None,
                     Some(r##"Required. The name of the agent to delete. Format: `projects//locations//agents/`."##),
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
            ("locations-agents-entity-types-create",
                    Some(r##"Creates an entity type in the specified agent. Note: You should always train a flow prior to sending it queries. See the [training documentation](https://cloud.google.com/dialogflow/cx/docs/concept/training)."##),
                    "Details at http://byron.github.io/google-apis-rs/google_dialogflow3_cli/projects_locations-agents-entity-types-create",
                  vec![
                    (Some(r##"parent"##),
                     None,
                     Some(r##"Required. The agent to create a entity type for. Format: `projects//locations//agents/`."##),
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
            ("locations-agents-entity-types-delete",
                    Some(r##"Deletes the specified entity type. Note: You should always train a flow prior to sending it queries. See the [training documentation](https://cloud.google.com/dialogflow/cx/docs/concept/training)."##),
                    "Details at http://byron.github.io/google-apis-rs/google_dialogflow3_cli/projects_locations-agents-entity-types-delete",
                  vec![
                    (Some(r##"name"##),
                     None,
                     Some(r##"Required. The name of the entity type to delete. Format: `projects//locations//agents//entityTypes/`."##),
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
            ("locations-agents-entity-types-export",
                    Some(r##"Exports the selected entity types."##),
                    "Details at http://byron.github.io/google-apis-rs/google_dialogflow3_cli/projects_locations-agents-entity-types-export",
                  vec![
                    (Some(r##"parent"##),
                     None,
                     Some(r##"Required. The name of the parent agent to export entity types. Format: `projects//locations//agents/`."##),
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
            ("locations-agents-entity-types-get",
                    Some(r##"Retrieves the specified entity type."##),
                    "Details at http://byron.github.io/google-apis-rs/google_dialogflow3_cli/projects_locations-agents-entity-types-get",
                  vec![
                    (Some(r##"name"##),
                     None,
                     Some(r##"Required. The name of the entity type. Format: `projects//locations//agents//entityTypes/`."##),
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
            ("locations-agents-entity-types-import",
                    Some(r##"Imports the specified entitytypes into the agent."##),
                    "Details at http://byron.github.io/google-apis-rs/google_dialogflow3_cli/projects_locations-agents-entity-types-import",
                  vec![
                    (Some(r##"parent"##),
                     None,
                     Some(r##"Required. The agent to import the entity types into. Format: `projects//locations//agents/`."##),
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
            ("locations-agents-entity-types-list",
                    Some(r##"Returns the list of all entity types in the specified agent."##),
                    "Details at http://byron.github.io/google-apis-rs/google_dialogflow3_cli/projects_locations-agents-entity-types-list",
                  vec![
                    (Some(r##"parent"##),
                     None,
                     Some(r##"Required. The agent to list all entity types for. Format: `projects//locations//agents/`."##),
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
            ("locations-agents-entity-types-patch",
                    Some(r##"Updates the specified entity type. Note: You should always train a flow prior to sending it queries. See the [training documentation](https://cloud.google.com/dialogflow/cx/docs/concept/training)."##),
                    "Details at http://byron.github.io/google-apis-rs/google_dialogflow3_cli/projects_locations-agents-entity-types-patch",
                  vec![
                    (Some(r##"name"##),
                     None,
                     Some(r##"The unique identifier of the entity type. Required for EntityTypes.UpdateEntityType. Format: `projects//locations//agents//entityTypes/`."##),
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
            ("locations-agents-environments-continuous-test-results-list",
                    Some(r##"Fetches a list of continuous test results for a given environment."##),
                    "Details at http://byron.github.io/google-apis-rs/google_dialogflow3_cli/projects_locations-agents-environments-continuous-test-results-list",
                  vec![
                    (Some(r##"parent"##),
                     None,
                     Some(r##"Required. The environment to list results for. Format: `projects//locations//agents// environments/`."##),
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
            ("locations-agents-environments-create",
                    Some(r##"Creates an Environment in the specified Agent. This method is a [long-running operation](https://cloud.google.com/dialogflow/cx/docs/how/long-running-operation). The returned `Operation` type has the following method-specific fields: - `metadata`: An empty [Struct message](https://developers.google.com/protocol-buffers/docs/reference/google.protobuf#struct) - `response`: Environment"##),
                    "Details at http://byron.github.io/google-apis-rs/google_dialogflow3_cli/projects_locations-agents-environments-create",
                  vec![
                    (Some(r##"parent"##),
                     None,
                     Some(r##"Required. The Agent to create an Environment for. Format: `projects//locations//agents/`."##),
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
            ("locations-agents-environments-delete",
                    Some(r##"Deletes the specified Environment."##),
                    "Details at http://byron.github.io/google-apis-rs/google_dialogflow3_cli/projects_locations-agents-environments-delete",
                  vec![
                    (Some(r##"name"##),
                     None,
                     Some(r##"Required. The name of the Environment to delete. Format: `projects//locations//agents//environments/`."##),
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
            ("locations-agents-environments-deploy-flow",
                    Some(r##"Deploys a flow to the specified Environment. This method is a [long-running operation](https://cloud.google.com/dialogflow/cx/docs/how/long-running-operation). The returned `Operation` type has the following method-specific fields: - `metadata`: DeployFlowMetadata - `response`: DeployFlowResponse"##),
                    "Details at http://byron.github.io/google-apis-rs/google_dialogflow3_cli/projects_locations-agents-environments-deploy-flow",
                  vec![
                    (Some(r##"environment"##),
                     None,
                     Some(r##"Required. The environment to deploy the flow to. Format: `projects//locations//agents// environments/`."##),
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
            ("locations-agents-environments-deployments-get",
                    Some(r##"Retrieves the specified Deployment."##),
                    "Details at http://byron.github.io/google-apis-rs/google_dialogflow3_cli/projects_locations-agents-environments-deployments-get",
                  vec![
                    (Some(r##"name"##),
                     None,
                     Some(r##"Required. The name of the Deployment. Format: `projects//locations//agents//environments//deployments/`."##),
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
            ("locations-agents-environments-deployments-list",
                    Some(r##"Returns the list of all deployments in the specified Environment."##),
                    "Details at http://byron.github.io/google-apis-rs/google_dialogflow3_cli/projects_locations-agents-environments-deployments-list",
                  vec![
                    (Some(r##"parent"##),
                     None,
                     Some(r##"Required. The Environment to list all environments for. Format: `projects//locations//agents//environments/`."##),
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
            ("locations-agents-environments-experiments-create",
                    Some(r##"Creates an Experiment in the specified Environment."##),
                    "Details at http://byron.github.io/google-apis-rs/google_dialogflow3_cli/projects_locations-agents-environments-experiments-create",
                  vec![
                    (Some(r##"parent"##),
                     None,
                     Some(r##"Required. The Agent to create an Environment for. Format: `projects//locations//agents//environments/`."##),
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
            ("locations-agents-environments-experiments-delete",
                    Some(r##"Deletes the specified Experiment."##),
                    "Details at http://byron.github.io/google-apis-rs/google_dialogflow3_cli/projects_locations-agents-environments-experiments-delete",
                  vec![
                    (Some(r##"name"##),
                     None,
                     Some(r##"Required. The name of the Environment to delete. Format: `projects//locations//agents//environments//experiments/`."##),
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
            ("locations-agents-environments-experiments-get",
                    Some(r##"Retrieves the specified Experiment."##),
                    "Details at http://byron.github.io/google-apis-rs/google_dialogflow3_cli/projects_locations-agents-environments-experiments-get",
                  vec![
                    (Some(r##"name"##),
                     None,
                     Some(r##"Required. The name of the Environment. Format: `projects//locations//agents//environments//experiments/`."##),
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
            ("locations-agents-environments-experiments-list",
                    Some(r##"Returns the list of all experiments in the specified Environment."##),
                    "Details at http://byron.github.io/google-apis-rs/google_dialogflow3_cli/projects_locations-agents-environments-experiments-list",
                  vec![
                    (Some(r##"parent"##),
                     None,
                     Some(r##"Required. The Environment to list all environments for. Format: `projects//locations//agents//environments/`."##),
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
            ("locations-agents-environments-experiments-patch",
                    Some(r##"Updates the specified Experiment."##),
                    "Details at http://byron.github.io/google-apis-rs/google_dialogflow3_cli/projects_locations-agents-environments-experiments-patch",
                  vec![
                    (Some(r##"name"##),
                     None,
                     Some(r##"The name of the experiment. Format: projects//locations//agents//environments//experiments/.."##),
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
            ("locations-agents-environments-experiments-start",
                    Some(r##"Starts the specified Experiment. This rpc only changes the state of experiment from PENDING to RUNNING."##),
                    "Details at http://byron.github.io/google-apis-rs/google_dialogflow3_cli/projects_locations-agents-environments-experiments-start",
                  vec![
                    (Some(r##"name"##),
                     None,
                     Some(r##"Required. Resource name of the experiment to start. Format: `projects//locations//agents//environments//experiments/`."##),
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
            ("locations-agents-environments-experiments-stop",
                    Some(r##"Stops the specified Experiment. This rpc only changes the state of experiment from RUNNING to DONE."##),
                    "Details at http://byron.github.io/google-apis-rs/google_dialogflow3_cli/projects_locations-agents-environments-experiments-stop",
                  vec![
                    (Some(r##"name"##),
                     None,
                     Some(r##"Required. Resource name of the experiment to stop. Format: `projects//locations//agents//environments//experiments/`."##),
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
            ("locations-agents-environments-get",
                    Some(r##"Retrieves the specified Environment."##),
                    "Details at http://byron.github.io/google-apis-rs/google_dialogflow3_cli/projects_locations-agents-environments-get",
                  vec![
                    (Some(r##"name"##),
                     None,
                     Some(r##"Required. The name of the Environment. Format: `projects//locations//agents//environments/`."##),
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
            ("locations-agents-environments-list",
                    Some(r##"Returns the list of all environments in the specified Agent."##),
                    "Details at http://byron.github.io/google-apis-rs/google_dialogflow3_cli/projects_locations-agents-environments-list",
                  vec![
                    (Some(r##"parent"##),
                     None,
                     Some(r##"Required. The Agent to list all environments for. Format: `projects//locations//agents/`."##),
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
            ("locations-agents-environments-lookup-environment-history",
                    Some(r##"Looks up the history of the specified Environment."##),
                    "Details at http://byron.github.io/google-apis-rs/google_dialogflow3_cli/projects_locations-agents-environments-lookup-environment-history",
                  vec![
                    (Some(r##"name"##),
                     None,
                     Some(r##"Required. Resource name of the environment to look up the history for. Format: `projects//locations//agents//environments/`."##),
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
            ("locations-agents-environments-patch",
                    Some(r##"Updates the specified Environment. This method is a [long-running operation](https://cloud.google.com/dialogflow/cx/docs/how/long-running-operation). The returned `Operation` type has the following method-specific fields: - `metadata`: An empty [Struct message](https://developers.google.com/protocol-buffers/docs/reference/google.protobuf#struct) - `response`: Environment"##),
                    "Details at http://byron.github.io/google-apis-rs/google_dialogflow3_cli/projects_locations-agents-environments-patch",
                  vec![
                    (Some(r##"name"##),
                     None,
                     Some(r##"The name of the environment. Format: `projects//locations//agents//environments/`."##),
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
            ("locations-agents-environments-run-continuous-test",
                    Some(r##"Kicks off a continuous test under the specified Environment. This method is a [long-running operation](https://cloud.google.com/dialogflow/cx/docs/how/long-running-operation). The returned `Operation` type has the following method-specific fields: - `metadata`: RunContinuousTestMetadata - `response`: RunContinuousTestResponse"##),
                    "Details at http://byron.github.io/google-apis-rs/google_dialogflow3_cli/projects_locations-agents-environments-run-continuous-test",
                  vec![
                    (Some(r##"environment"##),
                     None,
                     Some(r##"Required. Format: `projects//locations//agents//environments/`."##),
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
            ("locations-agents-environments-sessions-detect-intent",
                    Some(r##"Processes a natural language query and returns structured, actionable data as a result. This method is not idempotent, because it may cause session entity types to be updated, which in turn might affect results of future queries. Note: Always use agent versions for production traffic. See [Versions and environments](https://cloud.google.com/dialogflow/cx/docs/concept/version)."##),
                    "Details at http://byron.github.io/google-apis-rs/google_dialogflow3_cli/projects_locations-agents-environments-sessions-detect-intent",
                  vec![
                    (Some(r##"session"##),
                     None,
                     Some(r##"Required. The name of the session this query is sent to. Format: `projects//locations//agents//sessions/` or `projects//locations//agents//environments//sessions/`. If `Environment ID` is not specified, we assume default 'draft' environment. It's up to the API caller to choose an appropriate `Session ID`. It can be a random number or some type of session identifiers (preferably hashed). The length of the `Session ID` must not exceed 36 characters. For more information, see the [sessions guide](https://cloud.google.com/dialogflow/cx/docs/concept/session). Note: Always use agent versions for production traffic. See [Versions and environments](https://cloud.google.com/dialogflow/cx/docs/concept/version)."##),
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
            ("locations-agents-environments-sessions-entity-types-create",
                    Some(r##"Creates a session entity type."##),
                    "Details at http://byron.github.io/google-apis-rs/google_dialogflow3_cli/projects_locations-agents-environments-sessions-entity-types-create",
                  vec![
                    (Some(r##"parent"##),
                     None,
                     Some(r##"Required. The session to create a session entity type for. Format: `projects//locations//agents//sessions/` or `projects//locations//agents//environments//sessions/`. If `Environment ID` is not specified, we assume default 'draft' environment."##),
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
            ("locations-agents-environments-sessions-entity-types-delete",
                    Some(r##"Deletes the specified session entity type."##),
                    "Details at http://byron.github.io/google-apis-rs/google_dialogflow3_cli/projects_locations-agents-environments-sessions-entity-types-delete",
                  vec![
                    (Some(r##"name"##),
                     None,
                     Some(r##"Required. The name of the session entity type to delete. Format: `projects//locations//agents//sessions//entityTypes/` or `projects//locations//agents//environments//sessions//entityTypes/`. If `Environment ID` is not specified, we assume default 'draft' environment."##),
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
            ("locations-agents-environments-sessions-entity-types-get",
                    Some(r##"Retrieves the specified session entity type."##),
                    "Details at http://byron.github.io/google-apis-rs/google_dialogflow3_cli/projects_locations-agents-environments-sessions-entity-types-get",
                  vec![
                    (Some(r##"name"##),
                     None,
                     Some(r##"Required. The name of the session entity type. Format: `projects//locations//agents//sessions//entityTypes/` or `projects//locations//agents//environments//sessions//entityTypes/`. If `Environment ID` is not specified, we assume default 'draft' environment."##),
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
            ("locations-agents-environments-sessions-entity-types-list",
                    Some(r##"Returns the list of all session entity types in the specified session."##),
                    "Details at http://byron.github.io/google-apis-rs/google_dialogflow3_cli/projects_locations-agents-environments-sessions-entity-types-list",
                  vec![
                    (Some(r##"parent"##),
                     None,
                     Some(r##"Required. The session to list all session entity types from. Format: `projects//locations//agents//sessions/` or `projects//locations//agents//environments//sessions/`. If `Environment ID` is not specified, we assume default 'draft' environment."##),
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
            ("locations-agents-environments-sessions-entity-types-patch",
                    Some(r##"Updates the specified session entity type."##),
                    "Details at http://byron.github.io/google-apis-rs/google_dialogflow3_cli/projects_locations-agents-environments-sessions-entity-types-patch",
                  vec![
                    (Some(r##"name"##),
                     None,
                     Some(r##"Required. The unique identifier of the session entity type. Format: `projects//locations//agents//sessions//entityTypes/` or `projects//locations//agents//environments//sessions//entityTypes/`. If `Environment ID` is not specified, we assume default 'draft' environment."##),
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
            ("locations-agents-environments-sessions-fulfill-intent",
                    Some(r##"Fulfills a matched intent returned by MatchIntent. Must be called after MatchIntent, with input from MatchIntentResponse. Otherwise, the behavior is undefined."##),
                    "Details at http://byron.github.io/google-apis-rs/google_dialogflow3_cli/projects_locations-agents-environments-sessions-fulfill-intent",
                  vec![
                    (Some(r##"session"##),
                     None,
                     Some(r##"Required. The name of the session this query is sent to. Format: `projects//locations//agents//sessions/` or `projects//locations//agents//environments//sessions/`. If `Environment ID` is not specified, we assume default 'draft' environment. It's up to the API caller to choose an appropriate `Session ID`. It can be a random number or some type of session identifiers (preferably hashed). The length of the `Session ID` must not exceed 36 characters. For more information, see the [sessions guide](https://cloud.google.com/dialogflow/cx/docs/concept/session)."##),
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
            ("locations-agents-environments-sessions-match-intent",
                    Some(r##"Returns preliminary intent match results, doesn't change the session status."##),
                    "Details at http://byron.github.io/google-apis-rs/google_dialogflow3_cli/projects_locations-agents-environments-sessions-match-intent",
                  vec![
                    (Some(r##"session"##),
                     None,
                     Some(r##"Required. The name of the session this query is sent to. Format: `projects//locations//agents//sessions/` or `projects//locations//agents//environments//sessions/`. If `Environment ID` is not specified, we assume default 'draft' environment. It's up to the API caller to choose an appropriate `Session ID`. It can be a random number or some type of session identifiers (preferably hashed). The length of the `Session ID` must not exceed 36 characters. For more information, see the [sessions guide](https://cloud.google.com/dialogflow/cx/docs/concept/session)."##),
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
            ("locations-agents-environments-sessions-server-streaming-detect-intent",
                    Some(r##"Processes a natural language query and returns structured, actionable data as a result through server-side streaming. Server-side streaming allows Dialogflow to send [partial responses](https://cloud.google.com/dialogflow/cx/docs/concept/fulfillment#partial-response) earlier in a single request."##),
                    "Details at http://byron.github.io/google-apis-rs/google_dialogflow3_cli/projects_locations-agents-environments-sessions-server-streaming-detect-intent",
                  vec![
                    (Some(r##"session"##),
                     None,
                     Some(r##"Required. The name of the session this query is sent to. Format: `projects//locations//agents//sessions/` or `projects//locations//agents//environments//sessions/`. If `Environment ID` is not specified, we assume default 'draft' environment. It's up to the API caller to choose an appropriate `Session ID`. It can be a random number or some type of session identifiers (preferably hashed). The length of the `Session ID` must not exceed 36 characters. For more information, see the [sessions guide](https://cloud.google.com/dialogflow/cx/docs/concept/session). Note: Always use agent versions for production traffic. See [Versions and environments](https://cloud.google.com/dialogflow/cx/docs/concept/version)."##),
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
            ("locations-agents-export",
                    Some(r##"Exports the specified agent to a binary file. This method is a [long-running operation](https://cloud.google.com/dialogflow/cx/docs/how/long-running-operation). The returned `Operation` type has the following method-specific fields: - `metadata`: An empty [Struct message](https://developers.google.com/protocol-buffers/docs/reference/google.protobuf#struct) - `response`: ExportAgentResponse"##),
                    "Details at http://byron.github.io/google-apis-rs/google_dialogflow3_cli/projects_locations-agents-export",
                  vec![
                    (Some(r##"name"##),
                     None,
                     Some(r##"Required. The name of the agent to export. Format: `projects//locations//agents/`."##),
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
            ("locations-agents-flows-create",
                    Some(r##"Creates a flow in the specified agent. Note: You should always train a flow prior to sending it queries. See the [training documentation](https://cloud.google.com/dialogflow/cx/docs/concept/training)."##),
                    "Details at http://byron.github.io/google-apis-rs/google_dialogflow3_cli/projects_locations-agents-flows-create",
                  vec![
                    (Some(r##"parent"##),
                     None,
                     Some(r##"Required. The agent to create a flow for. Format: `projects//locations//agents/`."##),
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
            ("locations-agents-flows-delete",
                    Some(r##"Deletes a specified flow."##),
                    "Details at http://byron.github.io/google-apis-rs/google_dialogflow3_cli/projects_locations-agents-flows-delete",
                  vec![
                    (Some(r##"name"##),
                     None,
                     Some(r##"Required. The name of the flow to delete. Format: `projects//locations//agents//flows/`."##),
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
            ("locations-agents-flows-export",
                    Some(r##"Exports the specified flow to a binary file. This method is a [long-running operation](https://cloud.google.com/dialogflow/cx/docs/how/long-running-operation). The returned `Operation` type has the following method-specific fields: - `metadata`: An empty [Struct message](https://developers.google.com/protocol-buffers/docs/reference/google.protobuf#struct) - `response`: ExportFlowResponse Note that resources (e.g. intents, entities, webhooks) that the flow references will also be exported."##),
                    "Details at http://byron.github.io/google-apis-rs/google_dialogflow3_cli/projects_locations-agents-flows-export",
                  vec![
                    (Some(r##"name"##),
                     None,
                     Some(r##"Required. The name of the flow to export. Format: `projects//locations//agents//flows/`."##),
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
            ("locations-agents-flows-get",
                    Some(r##"Retrieves the specified flow."##),
                    "Details at http://byron.github.io/google-apis-rs/google_dialogflow3_cli/projects_locations-agents-flows-get",
                  vec![
                    (Some(r##"name"##),
                     None,
                     Some(r##"Required. The name of the flow to get. Format: `projects//locations//agents//flows/`."##),
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
            ("locations-agents-flows-get-validation-result",
                    Some(r##"Gets the latest flow validation result. Flow validation is performed when ValidateFlow is called."##),
                    "Details at http://byron.github.io/google-apis-rs/google_dialogflow3_cli/projects_locations-agents-flows-get-validation-result",
                  vec![
                    (Some(r##"name"##),
                     None,
                     Some(r##"Required. The flow name. Format: `projects//locations//agents//flows//validationResult`."##),
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
            ("locations-agents-flows-import",
                    Some(r##"Imports the specified flow to the specified agent from a binary file. This method is a [long-running operation](https://cloud.google.com/dialogflow/cx/docs/how/long-running-operation). The returned `Operation` type has the following method-specific fields: - `metadata`: An empty [Struct message](https://developers.google.com/protocol-buffers/docs/reference/google.protobuf#struct) - `response`: ImportFlowResponse Note: You should always train a flow prior to sending it queries. See the [training documentation](https://cloud.google.com/dialogflow/cx/docs/concept/training)."##),
                    "Details at http://byron.github.io/google-apis-rs/google_dialogflow3_cli/projects_locations-agents-flows-import",
                  vec![
                    (Some(r##"parent"##),
                     None,
                     Some(r##"Required. The agent to import the flow into. Format: `projects//locations//agents/`."##),
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
            ("locations-agents-flows-list",
                    Some(r##"Returns the list of all flows in the specified agent."##),
                    "Details at http://byron.github.io/google-apis-rs/google_dialogflow3_cli/projects_locations-agents-flows-list",
                  vec![
                    (Some(r##"parent"##),
                     None,
                     Some(r##"Required. The agent containing the flows. Format: `projects//locations//agents/`."##),
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
            ("locations-agents-flows-pages-create",
                    Some(r##"Creates a page in the specified flow. Note: You should always train a flow prior to sending it queries. See the [training documentation](https://cloud.google.com/dialogflow/cx/docs/concept/training)."##),
                    "Details at http://byron.github.io/google-apis-rs/google_dialogflow3_cli/projects_locations-agents-flows-pages-create",
                  vec![
                    (Some(r##"parent"##),
                     None,
                     Some(r##"Required. The flow to create a page for. Format: `projects//locations//agents//flows/`."##),
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
            ("locations-agents-flows-pages-delete",
                    Some(r##"Deletes the specified page. Note: You should always train a flow prior to sending it queries. See the [training documentation](https://cloud.google.com/dialogflow/cx/docs/concept/training)."##),
                    "Details at http://byron.github.io/google-apis-rs/google_dialogflow3_cli/projects_locations-agents-flows-pages-delete",
                  vec![
                    (Some(r##"name"##),
                     None,
                     Some(r##"Required. The name of the page to delete. Format: `projects//locations//agents//Flows//pages/`."##),
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
            ("locations-agents-flows-pages-get",
                    Some(r##"Retrieves the specified page."##),
                    "Details at http://byron.github.io/google-apis-rs/google_dialogflow3_cli/projects_locations-agents-flows-pages-get",
                  vec![
                    (Some(r##"name"##),
                     None,
                     Some(r##"Required. The name of the page. Format: `projects//locations//agents//flows//pages/`."##),
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
            ("locations-agents-flows-pages-list",
                    Some(r##"Returns the list of all pages in the specified flow."##),
                    "Details at http://byron.github.io/google-apis-rs/google_dialogflow3_cli/projects_locations-agents-flows-pages-list",
                  vec![
                    (Some(r##"parent"##),
                     None,
                     Some(r##"Required. The flow to list all pages for. Format: `projects//locations//agents//flows/`."##),
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
            ("locations-agents-flows-pages-patch",
                    Some(r##"Updates the specified page. Note: You should always train a flow prior to sending it queries. See the [training documentation](https://cloud.google.com/dialogflow/cx/docs/concept/training)."##),
                    "Details at http://byron.github.io/google-apis-rs/google_dialogflow3_cli/projects_locations-agents-flows-pages-patch",
                  vec![
                    (Some(r##"name"##),
                     None,
                     Some(r##"The unique identifier of the page. Required for the Pages.UpdatePage method. Pages.CreatePage populates the name automatically. Format: `projects//locations//agents//flows//pages/`."##),
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
            ("locations-agents-flows-patch",
                    Some(r##"Updates the specified flow. Note: You should always train a flow prior to sending it queries. See the [training documentation](https://cloud.google.com/dialogflow/cx/docs/concept/training)."##),
                    "Details at http://byron.github.io/google-apis-rs/google_dialogflow3_cli/projects_locations-agents-flows-patch",
                  vec![
                    (Some(r##"name"##),
                     None,
                     Some(r##"The unique identifier of the flow. Format: `projects//locations//agents//flows/`."##),
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
            ("locations-agents-flows-train",
                    Some(r##"Trains the specified flow. Note that only the flow in 'draft' environment is trained. This method is a [long-running operation](https://cloud.google.com/dialogflow/cx/docs/how/long-running-operation). The returned `Operation` type has the following method-specific fields: - `metadata`: An empty [Struct message](https://developers.google.com/protocol-buffers/docs/reference/google.protobuf#struct) - `response`: An [Empty message](https://developers.google.com/protocol-buffers/docs/reference/google.protobuf#empty) Note: You should always train a flow prior to sending it queries. See the [training documentation](https://cloud.google.com/dialogflow/cx/docs/concept/training)."##),
                    "Details at http://byron.github.io/google-apis-rs/google_dialogflow3_cli/projects_locations-agents-flows-train",
                  vec![
                    (Some(r##"name"##),
                     None,
                     Some(r##"Required. The flow to train. Format: `projects//locations//agents//flows/`."##),
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
            ("locations-agents-flows-transition-route-groups-create",
                    Some(r##"Creates an TransitionRouteGroup in the specified flow. Note: You should always train a flow prior to sending it queries. See the [training documentation](https://cloud.google.com/dialogflow/cx/docs/concept/training)."##),
                    "Details at http://byron.github.io/google-apis-rs/google_dialogflow3_cli/projects_locations-agents-flows-transition-route-groups-create",
                  vec![
                    (Some(r##"parent"##),
                     None,
                     Some(r##"Required. The flow to create an TransitionRouteGroup for. Format: `projects//locations//agents//flows/` or `projects//locations//agents/` for agent-level groups."##),
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
            ("locations-agents-flows-transition-route-groups-delete",
                    Some(r##"Deletes the specified TransitionRouteGroup. Note: You should always train a flow prior to sending it queries. See the [training documentation](https://cloud.google.com/dialogflow/cx/docs/concept/training)."##),
                    "Details at http://byron.github.io/google-apis-rs/google_dialogflow3_cli/projects_locations-agents-flows-transition-route-groups-delete",
                  vec![
                    (Some(r##"name"##),
                     None,
                     Some(r##"Required. The name of the TransitionRouteGroup to delete. Format: `projects//locations//agents//flows//transitionRouteGroups/` or `projects//locations//agents//transitionRouteGroups/`."##),
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
            ("locations-agents-flows-transition-route-groups-get",
                    Some(r##"Retrieves the specified TransitionRouteGroup."##),
                    "Details at http://byron.github.io/google-apis-rs/google_dialogflow3_cli/projects_locations-agents-flows-transition-route-groups-get",
                  vec![
                    (Some(r##"name"##),
                     None,
                     Some(r##"Required. The name of the TransitionRouteGroup. Format: `projects//locations//agents//flows//transitionRouteGroups/` or `projects//locations//agents//transitionRouteGroups/`."##),
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
            ("locations-agents-flows-transition-route-groups-list",
                    Some(r##"Returns the list of all transition route groups in the specified flow."##),
                    "Details at http://byron.github.io/google-apis-rs/google_dialogflow3_cli/projects_locations-agents-flows-transition-route-groups-list",
                  vec![
                    (Some(r##"parent"##),
                     None,
                     Some(r##"Required. The flow to list all transition route groups for. Format: `projects//locations//agents//flows/` or `projects//locations//agents/."##),
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
            ("locations-agents-flows-transition-route-groups-patch",
                    Some(r##"Updates the specified TransitionRouteGroup. Note: You should always train a flow prior to sending it queries. See the [training documentation](https://cloud.google.com/dialogflow/cx/docs/concept/training)."##),
                    "Details at http://byron.github.io/google-apis-rs/google_dialogflow3_cli/projects_locations-agents-flows-transition-route-groups-patch",
                  vec![
                    (Some(r##"name"##),
                     None,
                     Some(r##"The unique identifier of the transition route group. TransitionRouteGroups.CreateTransitionRouteGroup populates the name automatically. Format: `projects//locations//agents//flows//transitionRouteGroups/` ."##),
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
            ("locations-agents-flows-validate",
                    Some(r##"Validates the specified flow and creates or updates validation results. Please call this API after the training is completed to get the complete validation results."##),
                    "Details at http://byron.github.io/google-apis-rs/google_dialogflow3_cli/projects_locations-agents-flows-validate",
                  vec![
                    (Some(r##"name"##),
                     None,
                     Some(r##"Required. The flow to validate. Format: `projects//locations//agents//flows/`."##),
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
            ("locations-agents-flows-versions-compare-versions",
                    Some(r##"Compares the specified base version with target version."##),
                    "Details at http://byron.github.io/google-apis-rs/google_dialogflow3_cli/projects_locations-agents-flows-versions-compare-versions",
                  vec![
                    (Some(r##"base-version"##),
                     None,
                     Some(r##"Required. Name of the base flow version to compare with the target version. Use version ID `0` to indicate the draft version of the specified flow. Format: `projects//locations//agents/ /flows//versions/`."##),
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
            ("locations-agents-flows-versions-create",
                    Some(r##"Creates a Version in the specified Flow. This method is a [long-running operation](https://cloud.google.com/dialogflow/cx/docs/how/long-running-operation). The returned `Operation` type has the following method-specific fields: - `metadata`: CreateVersionOperationMetadata - `response`: Version"##),
                    "Details at http://byron.github.io/google-apis-rs/google_dialogflow3_cli/projects_locations-agents-flows-versions-create",
                  vec![
                    (Some(r##"parent"##),
                     None,
                     Some(r##"Required. The Flow to create an Version for. Format: `projects//locations//agents//flows/`."##),
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
            ("locations-agents-flows-versions-delete",
                    Some(r##"Deletes the specified Version."##),
                    "Details at http://byron.github.io/google-apis-rs/google_dialogflow3_cli/projects_locations-agents-flows-versions-delete",
                  vec![
                    (Some(r##"name"##),
                     None,
                     Some(r##"Required. The name of the Version to delete. Format: `projects//locations//agents//flows//versions/`."##),
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
            ("locations-agents-flows-versions-get",
                    Some(r##"Retrieves the specified Version."##),
                    "Details at http://byron.github.io/google-apis-rs/google_dialogflow3_cli/projects_locations-agents-flows-versions-get",
                  vec![
                    (Some(r##"name"##),
                     None,
                     Some(r##"Required. The name of the Version. Format: `projects//locations//agents//flows//versions/`."##),
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
            ("locations-agents-flows-versions-list",
                    Some(r##"Returns the list of all versions in the specified Flow."##),
                    "Details at http://byron.github.io/google-apis-rs/google_dialogflow3_cli/projects_locations-agents-flows-versions-list",
                  vec![
                    (Some(r##"parent"##),
                     None,
                     Some(r##"Required. The Flow to list all versions for. Format: `projects//locations//agents//flows/`."##),
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
            ("locations-agents-flows-versions-load",
                    Some(r##"Loads resources in the specified version to the draft flow. This method is a [long-running operation](https://cloud.google.com/dialogflow/cx/docs/how/long-running-operation). The returned `Operation` type has the following method-specific fields: - `metadata`: An empty [Struct message](https://developers.google.com/protocol-buffers/docs/reference/google.protobuf#struct) - `response`: An [Empty message](https://developers.google.com/protocol-buffers/docs/reference/google.protobuf#empty)"##),
                    "Details at http://byron.github.io/google-apis-rs/google_dialogflow3_cli/projects_locations-agents-flows-versions-load",
                  vec![
                    (Some(r##"name"##),
                     None,
                     Some(r##"Required. The Version to be loaded to draft flow. Format: `projects//locations//agents//flows//versions/`."##),
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
            ("locations-agents-flows-versions-patch",
                    Some(r##"Updates the specified Version."##),
                    "Details at http://byron.github.io/google-apis-rs/google_dialogflow3_cli/projects_locations-agents-flows-versions-patch",
                  vec![
                    (Some(r##"name"##),
                     None,
                     Some(r##"Format: projects//locations//agents//flows//versions/. Version ID is a self-increasing number generated by Dialogflow upon version creation."##),
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
            ("locations-agents-generators-create",
                    Some(r##"Creates a generator in the specified agent."##),
                    "Details at http://byron.github.io/google-apis-rs/google_dialogflow3_cli/projects_locations-agents-generators-create",
                  vec![
                    (Some(r##"parent"##),
                     None,
                     Some(r##"Required. The agent to create a generator for. Format: `projects//locations//agents/`."##),
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
            ("locations-agents-generators-delete",
                    Some(r##"Deletes the specified generators."##),
                    "Details at http://byron.github.io/google-apis-rs/google_dialogflow3_cli/projects_locations-agents-generators-delete",
                  vec![
                    (Some(r##"name"##),
                     None,
                     Some(r##"Required. The name of the generator to delete. Format: `projects//locations//agents//generators/`."##),
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
            ("locations-agents-generators-get",
                    Some(r##"Retrieves the specified generator."##),
                    "Details at http://byron.github.io/google-apis-rs/google_dialogflow3_cli/projects_locations-agents-generators-get",
                  vec![
                    (Some(r##"name"##),
                     None,
                     Some(r##"Required. The name of the generator. Format: `projects//locations//agents//generators/`."##),
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
            ("locations-agents-generators-list",
                    Some(r##"Returns the list of all generators in the specified agent."##),
                    "Details at http://byron.github.io/google-apis-rs/google_dialogflow3_cli/projects_locations-agents-generators-list",
                  vec![
                    (Some(r##"parent"##),
                     None,
                     Some(r##"Required. The agent to list all generators for. Format: `projects//locations//agents/`."##),
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
            ("locations-agents-generators-patch",
                    Some(r##"Update the specified generator."##),
                    "Details at http://byron.github.io/google-apis-rs/google_dialogflow3_cli/projects_locations-agents-generators-patch",
                  vec![
                    (Some(r##"name"##),
                     None,
                     Some(r##"The unique identifier of the generator. Must be set for the Generators.UpdateGenerator method. Generators.CreateGenerate populates the name automatically. Format: `projects//locations//agents//generators/`."##),
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
            ("locations-agents-get",
                    Some(r##"Retrieves the specified agent."##),
                    "Details at http://byron.github.io/google-apis-rs/google_dialogflow3_cli/projects_locations-agents-get",
                  vec![
                    (Some(r##"name"##),
                     None,
                     Some(r##"Required. The name of the agent. Format: `projects//locations//agents/`."##),
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
            ("locations-agents-get-generative-settings",
                    Some(r##"Gets the generative settings for the agent."##),
                    "Details at http://byron.github.io/google-apis-rs/google_dialogflow3_cli/projects_locations-agents-get-generative-settings",
                  vec![
                    (Some(r##"name"##),
                     None,
                     Some(r##"Required. Format: `projects//locations//agents//generativeSettings`."##),
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
            ("locations-agents-get-validation-result",
                    Some(r##"Gets the latest agent validation result. Agent validation is performed when ValidateAgent is called."##),
                    "Details at http://byron.github.io/google-apis-rs/google_dialogflow3_cli/projects_locations-agents-get-validation-result",
                  vec![
                    (Some(r##"name"##),
                     None,
                     Some(r##"Required. The agent name. Format: `projects//locations//agents//validationResult`."##),
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
            ("locations-agents-intents-create",
                    Some(r##"Creates an intent in the specified agent. Note: You should always train a flow prior to sending it queries. See the [training documentation](https://cloud.google.com/dialogflow/cx/docs/concept/training)."##),
                    "Details at http://byron.github.io/google-apis-rs/google_dialogflow3_cli/projects_locations-agents-intents-create",
                  vec![
                    (Some(r##"parent"##),
                     None,
                     Some(r##"Required. The agent to create an intent for. Format: `projects//locations//agents/`."##),
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
            ("locations-agents-intents-delete",
                    Some(r##"Deletes the specified intent. Note: You should always train a flow prior to sending it queries. See the [training documentation](https://cloud.google.com/dialogflow/cx/docs/concept/training)."##),
                    "Details at http://byron.github.io/google-apis-rs/google_dialogflow3_cli/projects_locations-agents-intents-delete",
                  vec![
                    (Some(r##"name"##),
                     None,
                     Some(r##"Required. The name of the intent to delete. Format: `projects//locations//agents//intents/`."##),
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
            ("locations-agents-intents-export",
                    Some(r##"Exports the selected intents. This method is a [long-running operation](https://cloud.google.com/dialogflow/cx/docs/how/long-running-operation). The returned `Operation` type has the following method-specific fields: - `metadata`: ExportIntentsMetadata - `response`: ExportIntentsResponse"##),
                    "Details at http://byron.github.io/google-apis-rs/google_dialogflow3_cli/projects_locations-agents-intents-export",
                  vec![
                    (Some(r##"parent"##),
                     None,
                     Some(r##"Required. The name of the parent agent to export intents. Format: `projects//locations//agents/`."##),
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
            ("locations-agents-intents-get",
                    Some(r##"Retrieves the specified intent."##),
                    "Details at http://byron.github.io/google-apis-rs/google_dialogflow3_cli/projects_locations-agents-intents-get",
                  vec![
                    (Some(r##"name"##),
                     None,
                     Some(r##"Required. The name of the intent. Format: `projects//locations//agents//intents/`."##),
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
            ("locations-agents-intents-import",
                    Some(r##"Imports the specified intents into the agent. This method is a [long-running operation](https://cloud.google.com/dialogflow/cx/docs/how/long-running-operation). The returned `Operation` type has the following method-specific fields: - `metadata`: ImportIntentsMetadata - `response`: ImportIntentsResponse"##),
                    "Details at http://byron.github.io/google-apis-rs/google_dialogflow3_cli/projects_locations-agents-intents-import",
                  vec![
                    (Some(r##"parent"##),
                     None,
                     Some(r##"Required. The agent to import the intents into. Format: `projects//locations//agents/`."##),
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
            ("locations-agents-intents-list",
                    Some(r##"Returns the list of all intents in the specified agent."##),
                    "Details at http://byron.github.io/google-apis-rs/google_dialogflow3_cli/projects_locations-agents-intents-list",
                  vec![
                    (Some(r##"parent"##),
                     None,
                     Some(r##"Required. The agent to list all intents for. Format: `projects//locations//agents/`."##),
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
            ("locations-agents-intents-patch",
                    Some(r##"Updates the specified intent. Note: You should always train a flow prior to sending it queries. See the [training documentation](https://cloud.google.com/dialogflow/cx/docs/concept/training)."##),
                    "Details at http://byron.github.io/google-apis-rs/google_dialogflow3_cli/projects_locations-agents-intents-patch",
                  vec![
                    (Some(r##"name"##),
                     None,
                     Some(r##"The unique identifier of the intent. Required for the Intents.UpdateIntent method. Intents.CreateIntent populates the name automatically. Format: `projects//locations//agents//intents/`."##),
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
            ("locations-agents-list",
                    Some(r##"Returns the list of all agents in the specified location."##),
                    "Details at http://byron.github.io/google-apis-rs/google_dialogflow3_cli/projects_locations-agents-list",
                  vec![
                    (Some(r##"parent"##),
                     None,
                     Some(r##"Required. The location to list all agents for. Format: `projects//locations/`."##),
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
            ("locations-agents-patch",
                    Some(r##"Updates the specified agent. Note: You should always train flows prior to sending them queries. See the [training documentation](https://cloud.google.com/dialogflow/cx/docs/concept/training)."##),
                    "Details at http://byron.github.io/google-apis-rs/google_dialogflow3_cli/projects_locations-agents-patch",
                  vec![
                    (Some(r##"name"##),
                     None,
                     Some(r##"The unique identifier of the agent. Required for the Agents.UpdateAgent method. Agents.CreateAgent populates the name automatically. Format: `projects//locations//agents/`."##),
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
            ("locations-agents-restore",
                    Some(r##"Restores the specified agent from a binary file. Replaces the current agent with a new one. Note that all existing resources in agent (e.g. intents, entity types, flows) will be removed. This method is a [long-running operation](https://cloud.google.com/dialogflow/cx/docs/how/long-running-operation). The returned `Operation` type has the following method-specific fields: - `metadata`: An empty [Struct message](https://developers.google.com/protocol-buffers/docs/reference/google.protobuf#struct) - `response`: An [Empty message](https://developers.google.com/protocol-buffers/docs/reference/google.protobuf#empty) Note: You should always train flows prior to sending them queries. See the [training documentation](https://cloud.google.com/dialogflow/cx/docs/concept/training)."##),
                    "Details at http://byron.github.io/google-apis-rs/google_dialogflow3_cli/projects_locations-agents-restore",
                  vec![
                    (Some(r##"name"##),
                     None,
                     Some(r##"Required. The name of the agent to restore into. Format: `projects//locations//agents/`."##),
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
            ("locations-agents-sessions-detect-intent",
                    Some(r##"Processes a natural language query and returns structured, actionable data as a result. This method is not idempotent, because it may cause session entity types to be updated, which in turn might affect results of future queries. Note: Always use agent versions for production traffic. See [Versions and environments](https://cloud.google.com/dialogflow/cx/docs/concept/version)."##),
                    "Details at http://byron.github.io/google-apis-rs/google_dialogflow3_cli/projects_locations-agents-sessions-detect-intent",
                  vec![
                    (Some(r##"session"##),
                     None,
                     Some(r##"Required. The name of the session this query is sent to. Format: `projects//locations//agents//sessions/` or `projects//locations//agents//environments//sessions/`. If `Environment ID` is not specified, we assume default 'draft' environment. It's up to the API caller to choose an appropriate `Session ID`. It can be a random number or some type of session identifiers (preferably hashed). The length of the `Session ID` must not exceed 36 characters. For more information, see the [sessions guide](https://cloud.google.com/dialogflow/cx/docs/concept/session). Note: Always use agent versions for production traffic. See [Versions and environments](https://cloud.google.com/dialogflow/cx/docs/concept/version)."##),
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
            ("locations-agents-sessions-entity-types-create",
                    Some(r##"Creates a session entity type."##),
                    "Details at http://byron.github.io/google-apis-rs/google_dialogflow3_cli/projects_locations-agents-sessions-entity-types-create",
                  vec![
                    (Some(r##"parent"##),
                     None,
                     Some(r##"Required. The session to create a session entity type for. Format: `projects//locations//agents//sessions/` or `projects//locations//agents//environments//sessions/`. If `Environment ID` is not specified, we assume default 'draft' environment."##),
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
            ("locations-agents-sessions-entity-types-delete",
                    Some(r##"Deletes the specified session entity type."##),
                    "Details at http://byron.github.io/google-apis-rs/google_dialogflow3_cli/projects_locations-agents-sessions-entity-types-delete",
                  vec![
                    (Some(r##"name"##),
                     None,
                     Some(r##"Required. The name of the session entity type to delete. Format: `projects//locations//agents//sessions//entityTypes/` or `projects//locations//agents//environments//sessions//entityTypes/`. If `Environment ID` is not specified, we assume default 'draft' environment."##),
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
            ("locations-agents-sessions-entity-types-get",
                    Some(r##"Retrieves the specified session entity type."##),
                    "Details at http://byron.github.io/google-apis-rs/google_dialogflow3_cli/projects_locations-agents-sessions-entity-types-get",
                  vec![
                    (Some(r##"name"##),
                     None,
                     Some(r##"Required. The name of the session entity type. Format: `projects//locations//agents//sessions//entityTypes/` or `projects//locations//agents//environments//sessions//entityTypes/`. If `Environment ID` is not specified, we assume default 'draft' environment."##),
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
            ("locations-agents-sessions-entity-types-list",
                    Some(r##"Returns the list of all session entity types in the specified session."##),
                    "Details at http://byron.github.io/google-apis-rs/google_dialogflow3_cli/projects_locations-agents-sessions-entity-types-list",
                  vec![
                    (Some(r##"parent"##),
                     None,
                     Some(r##"Required. The session to list all session entity types from. Format: `projects//locations//agents//sessions/` or `projects//locations//agents//environments//sessions/`. If `Environment ID` is not specified, we assume default 'draft' environment."##),
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
            ("locations-agents-sessions-entity-types-patch",
                    Some(r##"Updates the specified session entity type."##),
                    "Details at http://byron.github.io/google-apis-rs/google_dialogflow3_cli/projects_locations-agents-sessions-entity-types-patch",
                  vec![
                    (Some(r##"name"##),
                     None,
                     Some(r##"Required. The unique identifier of the session entity type. Format: `projects//locations//agents//sessions//entityTypes/` or `projects//locations//agents//environments//sessions//entityTypes/`. If `Environment ID` is not specified, we assume default 'draft' environment."##),
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
            ("locations-agents-sessions-fulfill-intent",
                    Some(r##"Fulfills a matched intent returned by MatchIntent. Must be called after MatchIntent, with input from MatchIntentResponse. Otherwise, the behavior is undefined."##),
                    "Details at http://byron.github.io/google-apis-rs/google_dialogflow3_cli/projects_locations-agents-sessions-fulfill-intent",
                  vec![
                    (Some(r##"session"##),
                     None,
                     Some(r##"Required. The name of the session this query is sent to. Format: `projects//locations//agents//sessions/` or `projects//locations//agents//environments//sessions/`. If `Environment ID` is not specified, we assume default 'draft' environment. It's up to the API caller to choose an appropriate `Session ID`. It can be a random number or some type of session identifiers (preferably hashed). The length of the `Session ID` must not exceed 36 characters. For more information, see the [sessions guide](https://cloud.google.com/dialogflow/cx/docs/concept/session)."##),
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
            ("locations-agents-sessions-match-intent",
                    Some(r##"Returns preliminary intent match results, doesn't change the session status."##),
                    "Details at http://byron.github.io/google-apis-rs/google_dialogflow3_cli/projects_locations-agents-sessions-match-intent",
                  vec![
                    (Some(r##"session"##),
                     None,
                     Some(r##"Required. The name of the session this query is sent to. Format: `projects//locations//agents//sessions/` or `projects//locations//agents//environments//sessions/`. If `Environment ID` is not specified, we assume default 'draft' environment. It's up to the API caller to choose an appropriate `Session ID`. It can be a random number or some type of session identifiers (preferably hashed). The length of the `Session ID` must not exceed 36 characters. For more information, see the [sessions guide](https://cloud.google.com/dialogflow/cx/docs/concept/session)."##),
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
            ("locations-agents-sessions-server-streaming-detect-intent",
                    Some(r##"Processes a natural language query and returns structured, actionable data as a result through server-side streaming. Server-side streaming allows Dialogflow to send [partial responses](https://cloud.google.com/dialogflow/cx/docs/concept/fulfillment#partial-response) earlier in a single request."##),
                    "Details at http://byron.github.io/google-apis-rs/google_dialogflow3_cli/projects_locations-agents-sessions-server-streaming-detect-intent",
                  vec![
                    (Some(r##"session"##),
                     None,
                     Some(r##"Required. The name of the session this query is sent to. Format: `projects//locations//agents//sessions/` or `projects//locations//agents//environments//sessions/`. If `Environment ID` is not specified, we assume default 'draft' environment. It's up to the API caller to choose an appropriate `Session ID`. It can be a random number or some type of session identifiers (preferably hashed). The length of the `Session ID` must not exceed 36 characters. For more information, see the [sessions guide](https://cloud.google.com/dialogflow/cx/docs/concept/session). Note: Always use agent versions for production traffic. See [Versions and environments](https://cloud.google.com/dialogflow/cx/docs/concept/version)."##),
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
            ("locations-agents-sessions-submit-answer-feedback",
                    Some(r##"Updates the feedback received from the user for a single turn of the bot response."##),
                    "Details at http://byron.github.io/google-apis-rs/google_dialogflow3_cli/projects_locations-agents-sessions-submit-answer-feedback",
                  vec![
                    (Some(r##"session"##),
                     None,
                     Some(r##"Required. The name of the session the feedback was sent to."##),
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
            ("locations-agents-test-cases-batch-delete",
                    Some(r##"Batch deletes test cases."##),
                    "Details at http://byron.github.io/google-apis-rs/google_dialogflow3_cli/projects_locations-agents-test-cases-batch-delete",
                  vec![
                    (Some(r##"parent"##),
                     None,
                     Some(r##"Required. The agent to delete test cases from. Format: `projects//locations//agents/`."##),
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
            ("locations-agents-test-cases-batch-run",
                    Some(r##"Kicks off a batch run of test cases. This method is a [long-running operation](https://cloud.google.com/dialogflow/cx/docs/how/long-running-operation). The returned `Operation` type has the following method-specific fields: - `metadata`: BatchRunTestCasesMetadata - `response`: BatchRunTestCasesResponse"##),
                    "Details at http://byron.github.io/google-apis-rs/google_dialogflow3_cli/projects_locations-agents-test-cases-batch-run",
                  vec![
                    (Some(r##"parent"##),
                     None,
                     Some(r##"Required. Agent name. Format: `projects//locations//agents/ `."##),
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
            ("locations-agents-test-cases-calculate-coverage",
                    Some(r##"Calculates the test coverage for an agent."##),
                    "Details at http://byron.github.io/google-apis-rs/google_dialogflow3_cli/projects_locations-agents-test-cases-calculate-coverage",
                  vec![
                    (Some(r##"agent"##),
                     None,
                     Some(r##"Required. The agent to calculate coverage for. Format: `projects//locations//agents/`."##),
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
            ("locations-agents-test-cases-create",
                    Some(r##"Creates a test case for the given agent."##),
                    "Details at http://byron.github.io/google-apis-rs/google_dialogflow3_cli/projects_locations-agents-test-cases-create",
                  vec![
                    (Some(r##"parent"##),
                     None,
                     Some(r##"Required. The agent to create the test case for. Format: `projects//locations//agents/`."##),
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
            ("locations-agents-test-cases-export",
                    Some(r##"Exports the test cases under the agent to a Cloud Storage bucket or a local file. Filter can be applied to export a subset of test cases. This method is a [long-running operation](https://cloud.google.com/dialogflow/cx/docs/how/long-running-operation). The returned `Operation` type has the following method-specific fields: - `metadata`: ExportTestCasesMetadata - `response`: ExportTestCasesResponse"##),
                    "Details at http://byron.github.io/google-apis-rs/google_dialogflow3_cli/projects_locations-agents-test-cases-export",
                  vec![
                    (Some(r##"parent"##),
                     None,
                     Some(r##"Required. The agent where to export test cases from. Format: `projects//locations//agents/`."##),
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
            ("locations-agents-test-cases-get",
                    Some(r##"Gets a test case."##),
                    "Details at http://byron.github.io/google-apis-rs/google_dialogflow3_cli/projects_locations-agents-test-cases-get",
                  vec![
                    (Some(r##"name"##),
                     None,
                     Some(r##"Required. The name of the testcase. Format: `projects//locations//agents//testCases/`."##),
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
            ("locations-agents-test-cases-import",
                    Some(r##"Imports the test cases from a Cloud Storage bucket or a local file. It always creates new test cases and won't overwrite any existing ones. The provided ID in the imported test case is neglected. This method is a [long-running operation](https://cloud.google.com/dialogflow/cx/docs/how/long-running-operation). The returned `Operation` type has the following method-specific fields: - `metadata`: ImportTestCasesMetadata - `response`: ImportTestCasesResponse"##),
                    "Details at http://byron.github.io/google-apis-rs/google_dialogflow3_cli/projects_locations-agents-test-cases-import",
                  vec![
                    (Some(r##"parent"##),
                     None,
                     Some(r##"Required. The agent to import test cases to. Format: `projects//locations//agents/`."##),
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
            ("locations-agents-test-cases-list",
                    Some(r##"Fetches a list of test cases for a given agent."##),
                    "Details at http://byron.github.io/google-apis-rs/google_dialogflow3_cli/projects_locations-agents-test-cases-list",
                  vec![
                    (Some(r##"parent"##),
                     None,
                     Some(r##"Required. The agent to list all pages for. Format: `projects//locations//agents/`."##),
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
            ("locations-agents-test-cases-patch",
                    Some(r##"Updates the specified test case."##),
                    "Details at http://byron.github.io/google-apis-rs/google_dialogflow3_cli/projects_locations-agents-test-cases-patch",
                  vec![
                    (Some(r##"name"##),
                     None,
                     Some(r##"The unique identifier of the test case. TestCases.CreateTestCase will populate the name automatically. Otherwise use format: `projects//locations//agents/ /testCases/`."##),
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
            ("locations-agents-test-cases-results-get",
                    Some(r##"Gets a test case result."##),
                    "Details at http://byron.github.io/google-apis-rs/google_dialogflow3_cli/projects_locations-agents-test-cases-results-get",
                  vec![
                    (Some(r##"name"##),
                     None,
                     Some(r##"Required. The name of the testcase. Format: `projects//locations//agents//testCases//results/`."##),
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
            ("locations-agents-test-cases-results-list",
                    Some(r##"Fetches the list of run results for the given test case. A maximum of 100 results are kept for each test case."##),
                    "Details at http://byron.github.io/google-apis-rs/google_dialogflow3_cli/projects_locations-agents-test-cases-results-list",
                  vec![
                    (Some(r##"parent"##),
                     None,
                     Some(r##"Required. The test case to list results for. Format: `projects//locations//agents// testCases/`. Specify a `-` as a wildcard for TestCase ID to list results across multiple test cases."##),
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
            ("locations-agents-test-cases-run",
                    Some(r##"Kicks off a test case run. This method is a [long-running operation](https://cloud.google.com/dialogflow/cx/docs/how/long-running-operation). The returned `Operation` type has the following method-specific fields: - `metadata`: RunTestCaseMetadata - `response`: RunTestCaseResponse"##),
                    "Details at http://byron.github.io/google-apis-rs/google_dialogflow3_cli/projects_locations-agents-test-cases-run",
                  vec![
                    (Some(r##"name"##),
                     None,
                     Some(r##"Required. Format of test case name to run: `projects//locations/ /agents//testCases/`."##),
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
            ("locations-agents-transition-route-groups-create",
                    Some(r##"Creates an TransitionRouteGroup in the specified flow. Note: You should always train a flow prior to sending it queries. See the [training documentation](https://cloud.google.com/dialogflow/cx/docs/concept/training)."##),
                    "Details at http://byron.github.io/google-apis-rs/google_dialogflow3_cli/projects_locations-agents-transition-route-groups-create",
                  vec![
                    (Some(r##"parent"##),
                     None,
                     Some(r##"Required. The flow to create an TransitionRouteGroup for. Format: `projects//locations//agents//flows/` or `projects//locations//agents/` for agent-level groups."##),
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
            ("locations-agents-transition-route-groups-delete",
                    Some(r##"Deletes the specified TransitionRouteGroup. Note: You should always train a flow prior to sending it queries. See the [training documentation](https://cloud.google.com/dialogflow/cx/docs/concept/training)."##),
                    "Details at http://byron.github.io/google-apis-rs/google_dialogflow3_cli/projects_locations-agents-transition-route-groups-delete",
                  vec![
                    (Some(r##"name"##),
                     None,
                     Some(r##"Required. The name of the TransitionRouteGroup to delete. Format: `projects//locations//agents//flows//transitionRouteGroups/` or `projects//locations//agents//transitionRouteGroups/`."##),
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
            ("locations-agents-transition-route-groups-get",
                    Some(r##"Retrieves the specified TransitionRouteGroup."##),
                    "Details at http://byron.github.io/google-apis-rs/google_dialogflow3_cli/projects_locations-agents-transition-route-groups-get",
                  vec![
                    (Some(r##"name"##),
                     None,
                     Some(r##"Required. The name of the TransitionRouteGroup. Format: `projects//locations//agents//flows//transitionRouteGroups/` or `projects//locations//agents//transitionRouteGroups/`."##),
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
            ("locations-agents-transition-route-groups-list",
                    Some(r##"Returns the list of all transition route groups in the specified flow."##),
                    "Details at http://byron.github.io/google-apis-rs/google_dialogflow3_cli/projects_locations-agents-transition-route-groups-list",
                  vec![
                    (Some(r##"parent"##),
                     None,
                     Some(r##"Required. The flow to list all transition route groups for. Format: `projects//locations//agents//flows/` or `projects//locations//agents/."##),
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
            ("locations-agents-transition-route-groups-patch",
                    Some(r##"Updates the specified TransitionRouteGroup. Note: You should always train a flow prior to sending it queries. See the [training documentation](https://cloud.google.com/dialogflow/cx/docs/concept/training)."##),
                    "Details at http://byron.github.io/google-apis-rs/google_dialogflow3_cli/projects_locations-agents-transition-route-groups-patch",
                  vec![
                    (Some(r##"name"##),
                     None,
                     Some(r##"The unique identifier of the transition route group. TransitionRouteGroups.CreateTransitionRouteGroup populates the name automatically. Format: `projects//locations//agents//flows//transitionRouteGroups/` ."##),
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
            ("locations-agents-update-generative-settings",
                    Some(r##"Updates the generative settings for the agent."##),
                    "Details at http://byron.github.io/google-apis-rs/google_dialogflow3_cli/projects_locations-agents-update-generative-settings",
                  vec![
                    (Some(r##"name"##),
                     None,
                     Some(r##"Format: `projects//locations//agents//generativeSettings`."##),
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
            ("locations-agents-validate",
                    Some(r##"Validates the specified agent and creates or updates validation results. The agent in draft version is validated. Please call this API after the training is completed to get the complete validation results."##),
                    "Details at http://byron.github.io/google-apis-rs/google_dialogflow3_cli/projects_locations-agents-validate",
                  vec![
                    (Some(r##"name"##),
                     None,
                     Some(r##"Required. The agent to validate. Format: `projects//locations//agents/`."##),
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
            ("locations-agents-webhooks-create",
                    Some(r##"Creates a webhook in the specified agent."##),
                    "Details at http://byron.github.io/google-apis-rs/google_dialogflow3_cli/projects_locations-agents-webhooks-create",
                  vec![
                    (Some(r##"parent"##),
                     None,
                     Some(r##"Required. The agent to create a webhook for. Format: `projects//locations//agents/`."##),
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
            ("locations-agents-webhooks-delete",
                    Some(r##"Deletes the specified webhook."##),
                    "Details at http://byron.github.io/google-apis-rs/google_dialogflow3_cli/projects_locations-agents-webhooks-delete",
                  vec![
                    (Some(r##"name"##),
                     None,
                     Some(r##"Required. The name of the webhook to delete. Format: `projects//locations//agents//webhooks/`."##),
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
            ("locations-agents-webhooks-get",
                    Some(r##"Retrieves the specified webhook."##),
                    "Details at http://byron.github.io/google-apis-rs/google_dialogflow3_cli/projects_locations-agents-webhooks-get",
                  vec![
                    (Some(r##"name"##),
                     None,
                     Some(r##"Required. The name of the webhook. Format: `projects//locations//agents//webhooks/`."##),
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
            ("locations-agents-webhooks-list",
                    Some(r##"Returns the list of all webhooks in the specified agent."##),
                    "Details at http://byron.github.io/google-apis-rs/google_dialogflow3_cli/projects_locations-agents-webhooks-list",
                  vec![
                    (Some(r##"parent"##),
                     None,
                     Some(r##"Required. The agent to list all webhooks for. Format: `projects//locations//agents/`."##),
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
            ("locations-agents-webhooks-patch",
                    Some(r##"Updates the specified webhook."##),
                    "Details at http://byron.github.io/google-apis-rs/google_dialogflow3_cli/projects_locations-agents-webhooks-patch",
                  vec![
                    (Some(r##"name"##),
                     None,
                     Some(r##"The unique identifier of the webhook. Required for the Webhooks.UpdateWebhook method. Webhooks.CreateWebhook populates the name automatically. Format: `projects//locations//agents//webhooks/`."##),
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
            ("locations-get",
                    Some(r##"Gets information about a location."##),
                    "Details at http://byron.github.io/google-apis-rs/google_dialogflow3_cli/projects_locations-get",
                  vec![
                    (Some(r##"name"##),
                     None,
                     Some(r##"Resource name for the location."##),
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
            ("locations-list",
                    Some(r##"Lists information about the supported locations for this service."##),
                    "Details at http://byron.github.io/google-apis-rs/google_dialogflow3_cli/projects_locations-list",
                  vec![
                    (Some(r##"name"##),
                     None,
                     Some(r##"The resource that owns the locations collection, if applicable."##),
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
            ("locations-operations-cancel",
                    Some(r##"Starts asynchronous cancellation on a long-running operation. The server makes a best effort to cancel the operation, but success is not guaranteed. If the server doesn't support this method, it returns `google.rpc.Code.UNIMPLEMENTED`. Clients can use Operations.GetOperation or other methods to check whether the cancellation succeeded or whether the operation completed despite cancellation. On successful cancellation, the operation is not deleted; instead, it becomes an operation with an Operation.error value with a google.rpc.Status.code of 1, corresponding to `Code.CANCELLED`."##),
                    "Details at http://byron.github.io/google-apis-rs/google_dialogflow3_cli/projects_locations-operations-cancel",
                  vec![
                    (Some(r##"name"##),
                     None,
                     Some(r##"The name of the operation resource to be cancelled."##),
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
            ("locations-operations-get",
                    Some(r##"Gets the latest state of a long-running operation. Clients can use this method to poll the operation result at intervals as recommended by the API service."##),
                    "Details at http://byron.github.io/google-apis-rs/google_dialogflow3_cli/projects_locations-operations-get",
                  vec![
                    (Some(r##"name"##),
                     None,
                     Some(r##"The name of the operation resource."##),
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
            ("locations-operations-list",
                    Some(r##"Lists operations that match the specified filter in the request. If the server doesn't support this method, it returns `UNIMPLEMENTED`."##),
                    "Details at http://byron.github.io/google-apis-rs/google_dialogflow3_cli/projects_locations-operations-list",
                  vec![
                    (Some(r##"name"##),
                     None,
                     Some(r##"The name of the operation's parent resource."##),
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
            ("locations-security-settings-create",
                    Some(r##"Create security settings in the specified location."##),
                    "Details at http://byron.github.io/google-apis-rs/google_dialogflow3_cli/projects_locations-security-settings-create",
                  vec![
                    (Some(r##"parent"##),
                     None,
                     Some(r##"Required. The location to create an SecuritySettings for. Format: `projects//locations/`."##),
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
            ("locations-security-settings-delete",
                    Some(r##"Deletes the specified SecuritySettings."##),
                    "Details at http://byron.github.io/google-apis-rs/google_dialogflow3_cli/projects_locations-security-settings-delete",
                  vec![
                    (Some(r##"name"##),
                     None,
                     Some(r##"Required. The name of the SecuritySettings to delete. Format: `projects//locations//securitySettings/`."##),
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
            ("locations-security-settings-get",
                    Some(r##"Retrieves the specified SecuritySettings. The returned settings may be stale by up to 1 minute."##),
                    "Details at http://byron.github.io/google-apis-rs/google_dialogflow3_cli/projects_locations-security-settings-get",
                  vec![
                    (Some(r##"name"##),
                     None,
                     Some(r##"Required. Resource name of the settings. Format: `projects//locations//securitySettings/`."##),
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
            ("locations-security-settings-list",
                    Some(r##"Returns the list of all security settings in the specified location."##),
                    "Details at http://byron.github.io/google-apis-rs/google_dialogflow3_cli/projects_locations-security-settings-list",
                  vec![
                    (Some(r##"parent"##),
                     None,
                     Some(r##"Required. The location to list all security settings for. Format: `projects//locations/`."##),
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
            ("locations-security-settings-patch",
                    Some(r##"Updates the specified SecuritySettings."##),
                    "Details at http://byron.github.io/google-apis-rs/google_dialogflow3_cli/projects_locations-security-settings-patch",
                  vec![
                    (Some(r##"name"##),
                     None,
                     Some(r##"Resource name of the settings. Required for the SecuritySettingsService.UpdateSecuritySettings method. SecuritySettingsService.CreateSecuritySettings populates the name automatically. Format: `projects//locations//securitySettings/`."##),
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
            ("operations-cancel",
                    Some(r##"Starts asynchronous cancellation on a long-running operation. The server makes a best effort to cancel the operation, but success is not guaranteed. If the server doesn't support this method, it returns `google.rpc.Code.UNIMPLEMENTED`. Clients can use Operations.GetOperation or other methods to check whether the cancellation succeeded or whether the operation completed despite cancellation. On successful cancellation, the operation is not deleted; instead, it becomes an operation with an Operation.error value with a google.rpc.Status.code of 1, corresponding to `Code.CANCELLED`."##),
                    "Details at http://byron.github.io/google-apis-rs/google_dialogflow3_cli/projects_operations-cancel",
                  vec![
                    (Some(r##"name"##),
                     None,
                     Some(r##"The name of the operation resource to be cancelled."##),
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
            ("operations-get",
                    Some(r##"Gets the latest state of a long-running operation. Clients can use this method to poll the operation result at intervals as recommended by the API service."##),
                    "Details at http://byron.github.io/google-apis-rs/google_dialogflow3_cli/projects_operations-get",
                  vec![
                    (Some(r##"name"##),
                     None,
                     Some(r##"The name of the operation resource."##),
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
            ("operations-list",
                    Some(r##"Lists operations that match the specified filter in the request. If the server doesn't support this method, it returns `UNIMPLEMENTED`."##),
                    "Details at http://byron.github.io/google-apis-rs/google_dialogflow3_cli/projects_operations-list",
                  vec![
                    (Some(r##"name"##),
                     None,
                     Some(r##"The name of the operation's parent resource."##),
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
    
    let mut app = App::new("dialogflow3")
           .author("Sebastian Thiel <byronimo@gmail.com>")
           .version("5.0.5+20240614")
           .about("Builds conversational interfaces (for example, chatbots, and voice-powered apps and devices).")
           .after_help("All documentation details can be found at http://byron.github.io/google-apis-rs/google_dialogflow3_cli")
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
