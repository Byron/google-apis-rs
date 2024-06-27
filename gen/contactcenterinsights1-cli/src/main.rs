// DO NOT EDIT !
// This file was generated automatically from 'src/generator/templates/cli/main.rs.mako'
// DO NOT EDIT !
#![allow(unused_variables, unused_imports, dead_code, unused_mut)]

#[macro_use]
extern crate clap;

use std::env;
use std::io::{self, Write};
use clap::{App, SubCommand, Arg};

use google_contactcenterinsights1::{api, Error, oauth2, client::chrono, FieldMask};


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
    hub: api::Contactcenterinsights<S>,
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
    async fn _projects_locations_conversations_analyses_create(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
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
                    "analysis-result.call-analysis-metadata.issue-model-result.issue-model" => Some(("analysisResult.callAnalysisMetadata.issueModelResult.issueModel", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "analysis-result.call-analysis-metadata.silence.silence-duration" => Some(("analysisResult.callAnalysisMetadata.silence.silenceDuration", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "analysis-result.call-analysis-metadata.silence.silence-percentage" => Some(("analysisResult.callAnalysisMetadata.silence.silencePercentage", JsonTypeInfo { jtype: JsonType::Float, ctype: ComplexType::Pod })),
                    "analysis-result.end-time" => Some(("analysisResult.endTime", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "annotator-selector.issue-models" => Some(("annotatorSelector.issueModels", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Vec })),
                    "annotator-selector.phrase-matchers" => Some(("annotatorSelector.phraseMatchers", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Vec })),
                    "annotator-selector.run-entity-annotator" => Some(("annotatorSelector.runEntityAnnotator", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "annotator-selector.run-intent-annotator" => Some(("annotatorSelector.runIntentAnnotator", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "annotator-selector.run-interruption-annotator" => Some(("annotatorSelector.runInterruptionAnnotator", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "annotator-selector.run-issue-model-annotator" => Some(("annotatorSelector.runIssueModelAnnotator", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "annotator-selector.run-phrase-matcher-annotator" => Some(("annotatorSelector.runPhraseMatcherAnnotator", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "annotator-selector.run-sentiment-annotator" => Some(("annotatorSelector.runSentimentAnnotator", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "annotator-selector.run-silence-annotator" => Some(("annotatorSelector.runSilenceAnnotator", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "annotator-selector.run-summarization-annotator" => Some(("annotatorSelector.runSummarizationAnnotator", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "annotator-selector.summarization-config.conversation-profile" => Some(("annotatorSelector.summarizationConfig.conversationProfile", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "annotator-selector.summarization-config.summarization-model" => Some(("annotatorSelector.summarizationConfig.summarizationModel", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "create-time" => Some(("createTime", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "name" => Some(("name", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "request-time" => Some(("requestTime", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    _ => {
                        let suggestion = FieldCursor::did_you_mean(key, &vec!["analysis-result", "annotator-selector", "call-analysis-metadata", "conversation-profile", "create-time", "end-time", "issue-model", "issue-model-result", "issue-models", "name", "phrase-matchers", "request-time", "run-entity-annotator", "run-intent-annotator", "run-interruption-annotator", "run-issue-model-annotator", "run-phrase-matcher-annotator", "run-sentiment-annotator", "run-silence-annotator", "run-summarization-annotator", "silence", "silence-duration", "silence-percentage", "summarization-config", "summarization-model"]);
                        err.issues.push(CLIError::Field(FieldError::Unknown(temp_cursor.to_string(), suggestion, value.map(|v| v.to_string()))));
                        None
                    }
                };
            if let Some((field_cursor_str, type_info)) = type_info {
                FieldCursor::from(field_cursor_str).set_json_value(&mut object, value.unwrap(), type_info, err, &temp_cursor);
            }
        }
        let mut request: api::GoogleCloudContactcenterinsightsV1Analysis = json::value::from_value(object).unwrap();
        let mut call = self.hub.projects().locations_conversations_analyses_create(request, opt.value_of("parent").unwrap_or(""));
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

    async fn _projects_locations_conversations_analyses_delete(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        let mut call = self.hub.projects().locations_conversations_analyses_delete(opt.value_of("name").unwrap_or(""));
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

    async fn _projects_locations_conversations_analyses_get(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        let mut call = self.hub.projects().locations_conversations_analyses_get(opt.value_of("name").unwrap_or(""));
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

    async fn _projects_locations_conversations_analyses_list(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        let mut call = self.hub.projects().locations_conversations_analyses_list(opt.value_of("parent").unwrap_or(""));
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

    async fn _projects_locations_conversations_bulk_analyze(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
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
                    "analysis-percentage" => Some(("analysisPercentage", JsonTypeInfo { jtype: JsonType::Float, ctype: ComplexType::Pod })),
                    "annotator-selector.issue-models" => Some(("annotatorSelector.issueModels", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Vec })),
                    "annotator-selector.phrase-matchers" => Some(("annotatorSelector.phraseMatchers", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Vec })),
                    "annotator-selector.run-entity-annotator" => Some(("annotatorSelector.runEntityAnnotator", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "annotator-selector.run-intent-annotator" => Some(("annotatorSelector.runIntentAnnotator", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "annotator-selector.run-interruption-annotator" => Some(("annotatorSelector.runInterruptionAnnotator", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "annotator-selector.run-issue-model-annotator" => Some(("annotatorSelector.runIssueModelAnnotator", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "annotator-selector.run-phrase-matcher-annotator" => Some(("annotatorSelector.runPhraseMatcherAnnotator", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "annotator-selector.run-sentiment-annotator" => Some(("annotatorSelector.runSentimentAnnotator", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "annotator-selector.run-silence-annotator" => Some(("annotatorSelector.runSilenceAnnotator", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "annotator-selector.run-summarization-annotator" => Some(("annotatorSelector.runSummarizationAnnotator", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "annotator-selector.summarization-config.conversation-profile" => Some(("annotatorSelector.summarizationConfig.conversationProfile", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "annotator-selector.summarization-config.summarization-model" => Some(("annotatorSelector.summarizationConfig.summarizationModel", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "filter" => Some(("filter", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "parent" => Some(("parent", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    _ => {
                        let suggestion = FieldCursor::did_you_mean(key, &vec!["analysis-percentage", "annotator-selector", "conversation-profile", "filter", "issue-models", "parent", "phrase-matchers", "run-entity-annotator", "run-intent-annotator", "run-interruption-annotator", "run-issue-model-annotator", "run-phrase-matcher-annotator", "run-sentiment-annotator", "run-silence-annotator", "run-summarization-annotator", "summarization-config", "summarization-model"]);
                        err.issues.push(CLIError::Field(FieldError::Unknown(temp_cursor.to_string(), suggestion, value.map(|v| v.to_string()))));
                        None
                    }
                };
            if let Some((field_cursor_str, type_info)) = type_info {
                FieldCursor::from(field_cursor_str).set_json_value(&mut object, value.unwrap(), type_info, err, &temp_cursor);
            }
        }
        let mut request: api::GoogleCloudContactcenterinsightsV1BulkAnalyzeConversationsRequest = json::value::from_value(object).unwrap();
        let mut call = self.hub.projects().locations_conversations_bulk_analyze(request, opt.value_of("parent").unwrap_or(""));
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

    async fn _projects_locations_conversations_bulk_delete(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
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
                    "filter" => Some(("filter", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "force" => Some(("force", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "max-delete-count" => Some(("maxDeleteCount", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "parent" => Some(("parent", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    _ => {
                        let suggestion = FieldCursor::did_you_mean(key, &vec!["filter", "force", "max-delete-count", "parent"]);
                        err.issues.push(CLIError::Field(FieldError::Unknown(temp_cursor.to_string(), suggestion, value.map(|v| v.to_string()))));
                        None
                    }
                };
            if let Some((field_cursor_str, type_info)) = type_info {
                FieldCursor::from(field_cursor_str).set_json_value(&mut object, value.unwrap(), type_info, err, &temp_cursor);
            }
        }
        let mut request: api::GoogleCloudContactcenterinsightsV1BulkDeleteConversationsRequest = json::value::from_value(object).unwrap();
        let mut call = self.hub.projects().locations_conversations_bulk_delete(request, opt.value_of("parent").unwrap_or(""));
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

    async fn _projects_locations_conversations_calculate_stats(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        let mut call = self.hub.projects().locations_conversations_calculate_stats(opt.value_of("location").unwrap_or(""));
        for parg in opt.values_of("v").map(|i|i.collect()).unwrap_or(Vec::new()).iter() {
            let (key, value) = parse_kv_arg(&*parg, err, false);
            match key {
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
                                                                           v.extend(["filter"].iter().map(|v|*v));
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

    async fn _projects_locations_conversations_create(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
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
                    "agent-id" => Some(("agentId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "call-metadata.agent-channel" => Some(("callMetadata.agentChannel", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "call-metadata.customer-channel" => Some(("callMetadata.customerChannel", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "create-time" => Some(("createTime", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "data-source.dialogflow-source.audio-uri" => Some(("dataSource.dialogflowSource.audioUri", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "data-source.dialogflow-source.dialogflow-conversation" => Some(("dataSource.dialogflowSource.dialogflowConversation", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "data-source.gcs-source.audio-uri" => Some(("dataSource.gcsSource.audioUri", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "data-source.gcs-source.transcript-uri" => Some(("dataSource.gcsSource.transcriptUri", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "duration" => Some(("duration", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "expire-time" => Some(("expireTime", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "labels" => Some(("labels", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Map })),
                    "language-code" => Some(("languageCode", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "latest-analysis.analysis-result.call-analysis-metadata.issue-model-result.issue-model" => Some(("latestAnalysis.analysisResult.callAnalysisMetadata.issueModelResult.issueModel", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "latest-analysis.analysis-result.call-analysis-metadata.silence.silence-duration" => Some(("latestAnalysis.analysisResult.callAnalysisMetadata.silence.silenceDuration", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "latest-analysis.analysis-result.call-analysis-metadata.silence.silence-percentage" => Some(("latestAnalysis.analysisResult.callAnalysisMetadata.silence.silencePercentage", JsonTypeInfo { jtype: JsonType::Float, ctype: ComplexType::Pod })),
                    "latest-analysis.analysis-result.end-time" => Some(("latestAnalysis.analysisResult.endTime", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "latest-analysis.annotator-selector.issue-models" => Some(("latestAnalysis.annotatorSelector.issueModels", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Vec })),
                    "latest-analysis.annotator-selector.phrase-matchers" => Some(("latestAnalysis.annotatorSelector.phraseMatchers", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Vec })),
                    "latest-analysis.annotator-selector.run-entity-annotator" => Some(("latestAnalysis.annotatorSelector.runEntityAnnotator", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "latest-analysis.annotator-selector.run-intent-annotator" => Some(("latestAnalysis.annotatorSelector.runIntentAnnotator", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "latest-analysis.annotator-selector.run-interruption-annotator" => Some(("latestAnalysis.annotatorSelector.runInterruptionAnnotator", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "latest-analysis.annotator-selector.run-issue-model-annotator" => Some(("latestAnalysis.annotatorSelector.runIssueModelAnnotator", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "latest-analysis.annotator-selector.run-phrase-matcher-annotator" => Some(("latestAnalysis.annotatorSelector.runPhraseMatcherAnnotator", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "latest-analysis.annotator-selector.run-sentiment-annotator" => Some(("latestAnalysis.annotatorSelector.runSentimentAnnotator", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "latest-analysis.annotator-selector.run-silence-annotator" => Some(("latestAnalysis.annotatorSelector.runSilenceAnnotator", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "latest-analysis.annotator-selector.run-summarization-annotator" => Some(("latestAnalysis.annotatorSelector.runSummarizationAnnotator", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "latest-analysis.annotator-selector.summarization-config.conversation-profile" => Some(("latestAnalysis.annotatorSelector.summarizationConfig.conversationProfile", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "latest-analysis.annotator-selector.summarization-config.summarization-model" => Some(("latestAnalysis.annotatorSelector.summarizationConfig.summarizationModel", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "latest-analysis.create-time" => Some(("latestAnalysis.createTime", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "latest-analysis.name" => Some(("latestAnalysis.name", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "latest-analysis.request-time" => Some(("latestAnalysis.requestTime", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "latest-summary.answer-record" => Some(("latestSummary.answerRecord", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "latest-summary.confidence" => Some(("latestSummary.confidence", JsonTypeInfo { jtype: JsonType::Float, ctype: ComplexType::Pod })),
                    "latest-summary.conversation-model" => Some(("latestSummary.conversationModel", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "latest-summary.metadata" => Some(("latestSummary.metadata", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Map })),
                    "latest-summary.text" => Some(("latestSummary.text", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "latest-summary.text-sections" => Some(("latestSummary.textSections", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Map })),
                    "medium" => Some(("medium", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "name" => Some(("name", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "obfuscated-user-id" => Some(("obfuscatedUserId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "quality-metadata.customer-satisfaction-rating" => Some(("qualityMetadata.customerSatisfactionRating", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "quality-metadata.menu-path" => Some(("qualityMetadata.menuPath", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "quality-metadata.wait-duration" => Some(("qualityMetadata.waitDuration", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "start-time" => Some(("startTime", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "ttl" => Some(("ttl", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "turn-count" => Some(("turnCount", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "update-time" => Some(("updateTime", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    _ => {
                        let suggestion = FieldCursor::did_you_mean(key, &vec!["agent-channel", "agent-id", "analysis-result", "annotator-selector", "answer-record", "audio-uri", "call-analysis-metadata", "call-metadata", "confidence", "conversation-model", "conversation-profile", "create-time", "customer-channel", "customer-satisfaction-rating", "data-source", "dialogflow-conversation", "dialogflow-source", "duration", "end-time", "expire-time", "gcs-source", "issue-model", "issue-model-result", "issue-models", "labels", "language-code", "latest-analysis", "latest-summary", "medium", "menu-path", "metadata", "name", "obfuscated-user-id", "phrase-matchers", "quality-metadata", "request-time", "run-entity-annotator", "run-intent-annotator", "run-interruption-annotator", "run-issue-model-annotator", "run-phrase-matcher-annotator", "run-sentiment-annotator", "run-silence-annotator", "run-summarization-annotator", "silence", "silence-duration", "silence-percentage", "start-time", "summarization-config", "summarization-model", "text", "text-sections", "transcript-uri", "ttl", "turn-count", "update-time", "wait-duration"]);
                        err.issues.push(CLIError::Field(FieldError::Unknown(temp_cursor.to_string(), suggestion, value.map(|v| v.to_string()))));
                        None
                    }
                };
            if let Some((field_cursor_str, type_info)) = type_info {
                FieldCursor::from(field_cursor_str).set_json_value(&mut object, value.unwrap(), type_info, err, &temp_cursor);
            }
        }
        let mut request: api::GoogleCloudContactcenterinsightsV1Conversation = json::value::from_value(object).unwrap();
        let mut call = self.hub.projects().locations_conversations_create(request, opt.value_of("parent").unwrap_or(""));
        for parg in opt.values_of("v").map(|i|i.collect()).unwrap_or(Vec::new()).iter() {
            let (key, value) = parse_kv_arg(&*parg, err, false);
            match key {
                "conversation-id" => {
                    call = call.conversation_id(value.unwrap_or(""));
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
                                                                           v.extend(["conversation-id"].iter().map(|v|*v));
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

    async fn _projects_locations_conversations_delete(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        let mut call = self.hub.projects().locations_conversations_delete(opt.value_of("name").unwrap_or(""));
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

    async fn _projects_locations_conversations_get(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        let mut call = self.hub.projects().locations_conversations_get(opt.value_of("name").unwrap_or(""));
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

    async fn _projects_locations_conversations_ingest(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
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
                    "conversation-config.agent-channel" => Some(("conversationConfig.agentChannel", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "conversation-config.agent-id" => Some(("conversationConfig.agentId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "conversation-config.customer-channel" => Some(("conversationConfig.customerChannel", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "gcs-source.bucket-object-type" => Some(("gcsSource.bucketObjectType", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "gcs-source.bucket-uri" => Some(("gcsSource.bucketUri", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "gcs-source.custom-metadata-keys" => Some(("gcsSource.customMetadataKeys", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Vec })),
                    "gcs-source.metadata-bucket-uri" => Some(("gcsSource.metadataBucketUri", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "parent" => Some(("parent", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "redaction-config.deidentify-template" => Some(("redactionConfig.deidentifyTemplate", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "redaction-config.inspect-template" => Some(("redactionConfig.inspectTemplate", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "sample-size" => Some(("sampleSize", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "speech-config.speech-recognizer" => Some(("speechConfig.speechRecognizer", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "transcript-object-config.medium" => Some(("transcriptObjectConfig.medium", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    _ => {
                        let suggestion = FieldCursor::did_you_mean(key, &vec!["agent-channel", "agent-id", "bucket-object-type", "bucket-uri", "conversation-config", "custom-metadata-keys", "customer-channel", "deidentify-template", "gcs-source", "inspect-template", "medium", "metadata-bucket-uri", "parent", "redaction-config", "sample-size", "speech-config", "speech-recognizer", "transcript-object-config"]);
                        err.issues.push(CLIError::Field(FieldError::Unknown(temp_cursor.to_string(), suggestion, value.map(|v| v.to_string()))));
                        None
                    }
                };
            if let Some((field_cursor_str, type_info)) = type_info {
                FieldCursor::from(field_cursor_str).set_json_value(&mut object, value.unwrap(), type_info, err, &temp_cursor);
            }
        }
        let mut request: api::GoogleCloudContactcenterinsightsV1IngestConversationsRequest = json::value::from_value(object).unwrap();
        let mut call = self.hub.projects().locations_conversations_ingest(request, opt.value_of("parent").unwrap_or(""));
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

    async fn _projects_locations_conversations_list(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        let mut call = self.hub.projects().locations_conversations_list(opt.value_of("parent").unwrap_or(""));
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
                                                                           v.extend(["filter", "order-by", "page-size", "page-token", "view"].iter().map(|v|*v));
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

    async fn _projects_locations_conversations_patch(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
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
                    "agent-id" => Some(("agentId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "call-metadata.agent-channel" => Some(("callMetadata.agentChannel", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "call-metadata.customer-channel" => Some(("callMetadata.customerChannel", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "create-time" => Some(("createTime", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "data-source.dialogflow-source.audio-uri" => Some(("dataSource.dialogflowSource.audioUri", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "data-source.dialogflow-source.dialogflow-conversation" => Some(("dataSource.dialogflowSource.dialogflowConversation", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "data-source.gcs-source.audio-uri" => Some(("dataSource.gcsSource.audioUri", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "data-source.gcs-source.transcript-uri" => Some(("dataSource.gcsSource.transcriptUri", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "duration" => Some(("duration", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "expire-time" => Some(("expireTime", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "labels" => Some(("labels", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Map })),
                    "language-code" => Some(("languageCode", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "latest-analysis.analysis-result.call-analysis-metadata.issue-model-result.issue-model" => Some(("latestAnalysis.analysisResult.callAnalysisMetadata.issueModelResult.issueModel", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "latest-analysis.analysis-result.call-analysis-metadata.silence.silence-duration" => Some(("latestAnalysis.analysisResult.callAnalysisMetadata.silence.silenceDuration", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "latest-analysis.analysis-result.call-analysis-metadata.silence.silence-percentage" => Some(("latestAnalysis.analysisResult.callAnalysisMetadata.silence.silencePercentage", JsonTypeInfo { jtype: JsonType::Float, ctype: ComplexType::Pod })),
                    "latest-analysis.analysis-result.end-time" => Some(("latestAnalysis.analysisResult.endTime", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "latest-analysis.annotator-selector.issue-models" => Some(("latestAnalysis.annotatorSelector.issueModels", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Vec })),
                    "latest-analysis.annotator-selector.phrase-matchers" => Some(("latestAnalysis.annotatorSelector.phraseMatchers", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Vec })),
                    "latest-analysis.annotator-selector.run-entity-annotator" => Some(("latestAnalysis.annotatorSelector.runEntityAnnotator", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "latest-analysis.annotator-selector.run-intent-annotator" => Some(("latestAnalysis.annotatorSelector.runIntentAnnotator", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "latest-analysis.annotator-selector.run-interruption-annotator" => Some(("latestAnalysis.annotatorSelector.runInterruptionAnnotator", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "latest-analysis.annotator-selector.run-issue-model-annotator" => Some(("latestAnalysis.annotatorSelector.runIssueModelAnnotator", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "latest-analysis.annotator-selector.run-phrase-matcher-annotator" => Some(("latestAnalysis.annotatorSelector.runPhraseMatcherAnnotator", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "latest-analysis.annotator-selector.run-sentiment-annotator" => Some(("latestAnalysis.annotatorSelector.runSentimentAnnotator", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "latest-analysis.annotator-selector.run-silence-annotator" => Some(("latestAnalysis.annotatorSelector.runSilenceAnnotator", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "latest-analysis.annotator-selector.run-summarization-annotator" => Some(("latestAnalysis.annotatorSelector.runSummarizationAnnotator", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "latest-analysis.annotator-selector.summarization-config.conversation-profile" => Some(("latestAnalysis.annotatorSelector.summarizationConfig.conversationProfile", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "latest-analysis.annotator-selector.summarization-config.summarization-model" => Some(("latestAnalysis.annotatorSelector.summarizationConfig.summarizationModel", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "latest-analysis.create-time" => Some(("latestAnalysis.createTime", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "latest-analysis.name" => Some(("latestAnalysis.name", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "latest-analysis.request-time" => Some(("latestAnalysis.requestTime", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "latest-summary.answer-record" => Some(("latestSummary.answerRecord", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "latest-summary.confidence" => Some(("latestSummary.confidence", JsonTypeInfo { jtype: JsonType::Float, ctype: ComplexType::Pod })),
                    "latest-summary.conversation-model" => Some(("latestSummary.conversationModel", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "latest-summary.metadata" => Some(("latestSummary.metadata", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Map })),
                    "latest-summary.text" => Some(("latestSummary.text", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "latest-summary.text-sections" => Some(("latestSummary.textSections", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Map })),
                    "medium" => Some(("medium", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "name" => Some(("name", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "obfuscated-user-id" => Some(("obfuscatedUserId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "quality-metadata.customer-satisfaction-rating" => Some(("qualityMetadata.customerSatisfactionRating", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "quality-metadata.menu-path" => Some(("qualityMetadata.menuPath", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "quality-metadata.wait-duration" => Some(("qualityMetadata.waitDuration", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "start-time" => Some(("startTime", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "ttl" => Some(("ttl", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "turn-count" => Some(("turnCount", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "update-time" => Some(("updateTime", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    _ => {
                        let suggestion = FieldCursor::did_you_mean(key, &vec!["agent-channel", "agent-id", "analysis-result", "annotator-selector", "answer-record", "audio-uri", "call-analysis-metadata", "call-metadata", "confidence", "conversation-model", "conversation-profile", "create-time", "customer-channel", "customer-satisfaction-rating", "data-source", "dialogflow-conversation", "dialogflow-source", "duration", "end-time", "expire-time", "gcs-source", "issue-model", "issue-model-result", "issue-models", "labels", "language-code", "latest-analysis", "latest-summary", "medium", "menu-path", "metadata", "name", "obfuscated-user-id", "phrase-matchers", "quality-metadata", "request-time", "run-entity-annotator", "run-intent-annotator", "run-interruption-annotator", "run-issue-model-annotator", "run-phrase-matcher-annotator", "run-sentiment-annotator", "run-silence-annotator", "run-summarization-annotator", "silence", "silence-duration", "silence-percentage", "start-time", "summarization-config", "summarization-model", "text", "text-sections", "transcript-uri", "ttl", "turn-count", "update-time", "wait-duration"]);
                        err.issues.push(CLIError::Field(FieldError::Unknown(temp_cursor.to_string(), suggestion, value.map(|v| v.to_string()))));
                        None
                    }
                };
            if let Some((field_cursor_str, type_info)) = type_info {
                FieldCursor::from(field_cursor_str).set_json_value(&mut object, value.unwrap(), type_info, err, &temp_cursor);
            }
        }
        let mut request: api::GoogleCloudContactcenterinsightsV1Conversation = json::value::from_value(object).unwrap();
        let mut call = self.hub.projects().locations_conversations_patch(request, opt.value_of("name").unwrap_or(""));
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

    async fn _projects_locations_conversations_upload(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
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
                    "conversation.agent-id" => Some(("conversation.agentId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "conversation.call-metadata.agent-channel" => Some(("conversation.callMetadata.agentChannel", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "conversation.call-metadata.customer-channel" => Some(("conversation.callMetadata.customerChannel", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "conversation.create-time" => Some(("conversation.createTime", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "conversation.data-source.dialogflow-source.audio-uri" => Some(("conversation.dataSource.dialogflowSource.audioUri", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "conversation.data-source.dialogflow-source.dialogflow-conversation" => Some(("conversation.dataSource.dialogflowSource.dialogflowConversation", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "conversation.data-source.gcs-source.audio-uri" => Some(("conversation.dataSource.gcsSource.audioUri", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "conversation.data-source.gcs-source.transcript-uri" => Some(("conversation.dataSource.gcsSource.transcriptUri", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "conversation.duration" => Some(("conversation.duration", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "conversation.expire-time" => Some(("conversation.expireTime", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "conversation.labels" => Some(("conversation.labels", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Map })),
                    "conversation.language-code" => Some(("conversation.languageCode", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "conversation.latest-analysis.analysis-result.call-analysis-metadata.issue-model-result.issue-model" => Some(("conversation.latestAnalysis.analysisResult.callAnalysisMetadata.issueModelResult.issueModel", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "conversation.latest-analysis.analysis-result.call-analysis-metadata.silence.silence-duration" => Some(("conversation.latestAnalysis.analysisResult.callAnalysisMetadata.silence.silenceDuration", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "conversation.latest-analysis.analysis-result.call-analysis-metadata.silence.silence-percentage" => Some(("conversation.latestAnalysis.analysisResult.callAnalysisMetadata.silence.silencePercentage", JsonTypeInfo { jtype: JsonType::Float, ctype: ComplexType::Pod })),
                    "conversation.latest-analysis.analysis-result.end-time" => Some(("conversation.latestAnalysis.analysisResult.endTime", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "conversation.latest-analysis.annotator-selector.issue-models" => Some(("conversation.latestAnalysis.annotatorSelector.issueModels", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Vec })),
                    "conversation.latest-analysis.annotator-selector.phrase-matchers" => Some(("conversation.latestAnalysis.annotatorSelector.phraseMatchers", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Vec })),
                    "conversation.latest-analysis.annotator-selector.run-entity-annotator" => Some(("conversation.latestAnalysis.annotatorSelector.runEntityAnnotator", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "conversation.latest-analysis.annotator-selector.run-intent-annotator" => Some(("conversation.latestAnalysis.annotatorSelector.runIntentAnnotator", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "conversation.latest-analysis.annotator-selector.run-interruption-annotator" => Some(("conversation.latestAnalysis.annotatorSelector.runInterruptionAnnotator", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "conversation.latest-analysis.annotator-selector.run-issue-model-annotator" => Some(("conversation.latestAnalysis.annotatorSelector.runIssueModelAnnotator", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "conversation.latest-analysis.annotator-selector.run-phrase-matcher-annotator" => Some(("conversation.latestAnalysis.annotatorSelector.runPhraseMatcherAnnotator", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "conversation.latest-analysis.annotator-selector.run-sentiment-annotator" => Some(("conversation.latestAnalysis.annotatorSelector.runSentimentAnnotator", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "conversation.latest-analysis.annotator-selector.run-silence-annotator" => Some(("conversation.latestAnalysis.annotatorSelector.runSilenceAnnotator", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "conversation.latest-analysis.annotator-selector.run-summarization-annotator" => Some(("conversation.latestAnalysis.annotatorSelector.runSummarizationAnnotator", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "conversation.latest-analysis.annotator-selector.summarization-config.conversation-profile" => Some(("conversation.latestAnalysis.annotatorSelector.summarizationConfig.conversationProfile", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "conversation.latest-analysis.annotator-selector.summarization-config.summarization-model" => Some(("conversation.latestAnalysis.annotatorSelector.summarizationConfig.summarizationModel", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "conversation.latest-analysis.create-time" => Some(("conversation.latestAnalysis.createTime", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "conversation.latest-analysis.name" => Some(("conversation.latestAnalysis.name", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "conversation.latest-analysis.request-time" => Some(("conversation.latestAnalysis.requestTime", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "conversation.latest-summary.answer-record" => Some(("conversation.latestSummary.answerRecord", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "conversation.latest-summary.confidence" => Some(("conversation.latestSummary.confidence", JsonTypeInfo { jtype: JsonType::Float, ctype: ComplexType::Pod })),
                    "conversation.latest-summary.conversation-model" => Some(("conversation.latestSummary.conversationModel", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "conversation.latest-summary.metadata" => Some(("conversation.latestSummary.metadata", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Map })),
                    "conversation.latest-summary.text" => Some(("conversation.latestSummary.text", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "conversation.latest-summary.text-sections" => Some(("conversation.latestSummary.textSections", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Map })),
                    "conversation.medium" => Some(("conversation.medium", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "conversation.name" => Some(("conversation.name", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "conversation.obfuscated-user-id" => Some(("conversation.obfuscatedUserId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "conversation.quality-metadata.customer-satisfaction-rating" => Some(("conversation.qualityMetadata.customerSatisfactionRating", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "conversation.quality-metadata.menu-path" => Some(("conversation.qualityMetadata.menuPath", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "conversation.quality-metadata.wait-duration" => Some(("conversation.qualityMetadata.waitDuration", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "conversation.start-time" => Some(("conversation.startTime", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "conversation.ttl" => Some(("conversation.ttl", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "conversation.turn-count" => Some(("conversation.turnCount", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "conversation.update-time" => Some(("conversation.updateTime", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "conversation-id" => Some(("conversationId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "parent" => Some(("parent", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "redaction-config.deidentify-template" => Some(("redactionConfig.deidentifyTemplate", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "redaction-config.inspect-template" => Some(("redactionConfig.inspectTemplate", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "speech-config.speech-recognizer" => Some(("speechConfig.speechRecognizer", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    _ => {
                        let suggestion = FieldCursor::did_you_mean(key, &vec!["agent-channel", "agent-id", "analysis-result", "annotator-selector", "answer-record", "audio-uri", "call-analysis-metadata", "call-metadata", "confidence", "conversation", "conversation-id", "conversation-model", "conversation-profile", "create-time", "customer-channel", "customer-satisfaction-rating", "data-source", "deidentify-template", "dialogflow-conversation", "dialogflow-source", "duration", "end-time", "expire-time", "gcs-source", "inspect-template", "issue-model", "issue-model-result", "issue-models", "labels", "language-code", "latest-analysis", "latest-summary", "medium", "menu-path", "metadata", "name", "obfuscated-user-id", "parent", "phrase-matchers", "quality-metadata", "redaction-config", "request-time", "run-entity-annotator", "run-intent-annotator", "run-interruption-annotator", "run-issue-model-annotator", "run-phrase-matcher-annotator", "run-sentiment-annotator", "run-silence-annotator", "run-summarization-annotator", "silence", "silence-duration", "silence-percentage", "speech-config", "speech-recognizer", "start-time", "summarization-config", "summarization-model", "text", "text-sections", "transcript-uri", "ttl", "turn-count", "update-time", "wait-duration"]);
                        err.issues.push(CLIError::Field(FieldError::Unknown(temp_cursor.to_string(), suggestion, value.map(|v| v.to_string()))));
                        None
                    }
                };
            if let Some((field_cursor_str, type_info)) = type_info {
                FieldCursor::from(field_cursor_str).set_json_value(&mut object, value.unwrap(), type_info, err, &temp_cursor);
            }
        }
        let mut request: api::GoogleCloudContactcenterinsightsV1UploadConversationRequest = json::value::from_value(object).unwrap();
        let mut call = self.hub.projects().locations_conversations_upload(request, opt.value_of("parent").unwrap_or(""));
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

    async fn _projects_locations_get_settings(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        let mut call = self.hub.projects().locations_get_settings(opt.value_of("name").unwrap_or(""));
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

    async fn _projects_locations_insightsdata_export(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
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
                    "big-query-destination.dataset" => Some(("bigQueryDestination.dataset", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "big-query-destination.project-id" => Some(("bigQueryDestination.projectId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "big-query-destination.table" => Some(("bigQueryDestination.table", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "filter" => Some(("filter", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "kms-key" => Some(("kmsKey", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "parent" => Some(("parent", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "write-disposition" => Some(("writeDisposition", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    _ => {
                        let suggestion = FieldCursor::did_you_mean(key, &vec!["big-query-destination", "dataset", "filter", "kms-key", "parent", "project-id", "table", "write-disposition"]);
                        err.issues.push(CLIError::Field(FieldError::Unknown(temp_cursor.to_string(), suggestion, value.map(|v| v.to_string()))));
                        None
                    }
                };
            if let Some((field_cursor_str, type_info)) = type_info {
                FieldCursor::from(field_cursor_str).set_json_value(&mut object, value.unwrap(), type_info, err, &temp_cursor);
            }
        }
        let mut request: api::GoogleCloudContactcenterinsightsV1ExportInsightsDataRequest = json::value::from_value(object).unwrap();
        let mut call = self.hub.projects().locations_insightsdata_export(request, opt.value_of("parent").unwrap_or(""));
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

    async fn _projects_locations_issue_models_calculate_issue_model_stats(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        let mut call = self.hub.projects().locations_issue_models_calculate_issue_model_stats(opt.value_of("issue-model").unwrap_or(""));
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

    async fn _projects_locations_issue_models_create(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
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
                    "display-name" => Some(("displayName", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "input-data-config.filter" => Some(("inputDataConfig.filter", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "input-data-config.medium" => Some(("inputDataConfig.medium", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "input-data-config.training-conversations-count" => Some(("inputDataConfig.trainingConversationsCount", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "issue-count" => Some(("issueCount", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "language-code" => Some(("languageCode", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "model-type" => Some(("modelType", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "name" => Some(("name", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "state" => Some(("state", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "training-stats.analyzed-conversations-count" => Some(("trainingStats.analyzedConversationsCount", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "training-stats.unclassified-conversations-count" => Some(("trainingStats.unclassifiedConversationsCount", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "update-time" => Some(("updateTime", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    _ => {
                        let suggestion = FieldCursor::did_you_mean(key, &vec!["analyzed-conversations-count", "create-time", "display-name", "filter", "input-data-config", "issue-count", "language-code", "medium", "model-type", "name", "state", "training-conversations-count", "training-stats", "unclassified-conversations-count", "update-time"]);
                        err.issues.push(CLIError::Field(FieldError::Unknown(temp_cursor.to_string(), suggestion, value.map(|v| v.to_string()))));
                        None
                    }
                };
            if let Some((field_cursor_str, type_info)) = type_info {
                FieldCursor::from(field_cursor_str).set_json_value(&mut object, value.unwrap(), type_info, err, &temp_cursor);
            }
        }
        let mut request: api::GoogleCloudContactcenterinsightsV1IssueModel = json::value::from_value(object).unwrap();
        let mut call = self.hub.projects().locations_issue_models_create(request, opt.value_of("parent").unwrap_or(""));
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

    async fn _projects_locations_issue_models_delete(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        let mut call = self.hub.projects().locations_issue_models_delete(opt.value_of("name").unwrap_or(""));
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

    async fn _projects_locations_issue_models_deploy(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
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
        let mut request: api::GoogleCloudContactcenterinsightsV1DeployIssueModelRequest = json::value::from_value(object).unwrap();
        let mut call = self.hub.projects().locations_issue_models_deploy(request, opt.value_of("name").unwrap_or(""));
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

    async fn _projects_locations_issue_models_export(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
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
                    "gcs-destination.object-uri" => Some(("gcsDestination.objectUri", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "name" => Some(("name", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    _ => {
                        let suggestion = FieldCursor::did_you_mean(key, &vec!["gcs-destination", "name", "object-uri"]);
                        err.issues.push(CLIError::Field(FieldError::Unknown(temp_cursor.to_string(), suggestion, value.map(|v| v.to_string()))));
                        None
                    }
                };
            if let Some((field_cursor_str, type_info)) = type_info {
                FieldCursor::from(field_cursor_str).set_json_value(&mut object, value.unwrap(), type_info, err, &temp_cursor);
            }
        }
        let mut request: api::GoogleCloudContactcenterinsightsV1ExportIssueModelRequest = json::value::from_value(object).unwrap();
        let mut call = self.hub.projects().locations_issue_models_export(request, opt.value_of("name").unwrap_or(""));
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

    async fn _projects_locations_issue_models_get(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        let mut call = self.hub.projects().locations_issue_models_get(opt.value_of("name").unwrap_or(""));
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

    async fn _projects_locations_issue_models_import(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
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
                    "create-new-model" => Some(("createNewModel", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "gcs-source.object-uri" => Some(("gcsSource.objectUri", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "parent" => Some(("parent", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    _ => {
                        let suggestion = FieldCursor::did_you_mean(key, &vec!["create-new-model", "gcs-source", "object-uri", "parent"]);
                        err.issues.push(CLIError::Field(FieldError::Unknown(temp_cursor.to_string(), suggestion, value.map(|v| v.to_string()))));
                        None
                    }
                };
            if let Some((field_cursor_str, type_info)) = type_info {
                FieldCursor::from(field_cursor_str).set_json_value(&mut object, value.unwrap(), type_info, err, &temp_cursor);
            }
        }
        let mut request: api::GoogleCloudContactcenterinsightsV1ImportIssueModelRequest = json::value::from_value(object).unwrap();
        let mut call = self.hub.projects().locations_issue_models_import(request, opt.value_of("parent").unwrap_or(""));
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

    async fn _projects_locations_issue_models_issues_delete(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        let mut call = self.hub.projects().locations_issue_models_issues_delete(opt.value_of("name").unwrap_or(""));
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

    async fn _projects_locations_issue_models_issues_get(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        let mut call = self.hub.projects().locations_issue_models_issues_get(opt.value_of("name").unwrap_or(""));
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

    async fn _projects_locations_issue_models_issues_list(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        let mut call = self.hub.projects().locations_issue_models_issues_list(opt.value_of("parent").unwrap_or(""));
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

    async fn _projects_locations_issue_models_issues_patch(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
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
                    "display-name" => Some(("displayName", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "name" => Some(("name", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "sample-utterances" => Some(("sampleUtterances", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Vec })),
                    "update-time" => Some(("updateTime", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    _ => {
                        let suggestion = FieldCursor::did_you_mean(key, &vec!["create-time", "display-name", "name", "sample-utterances", "update-time"]);
                        err.issues.push(CLIError::Field(FieldError::Unknown(temp_cursor.to_string(), suggestion, value.map(|v| v.to_string()))));
                        None
                    }
                };
            if let Some((field_cursor_str, type_info)) = type_info {
                FieldCursor::from(field_cursor_str).set_json_value(&mut object, value.unwrap(), type_info, err, &temp_cursor);
            }
        }
        let mut request: api::GoogleCloudContactcenterinsightsV1Issue = json::value::from_value(object).unwrap();
        let mut call = self.hub.projects().locations_issue_models_issues_patch(request, opt.value_of("name").unwrap_or(""));
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

    async fn _projects_locations_issue_models_list(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        let mut call = self.hub.projects().locations_issue_models_list(opt.value_of("parent").unwrap_or(""));
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

    async fn _projects_locations_issue_models_patch(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
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
                    "display-name" => Some(("displayName", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "input-data-config.filter" => Some(("inputDataConfig.filter", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "input-data-config.medium" => Some(("inputDataConfig.medium", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "input-data-config.training-conversations-count" => Some(("inputDataConfig.trainingConversationsCount", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "issue-count" => Some(("issueCount", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "language-code" => Some(("languageCode", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "model-type" => Some(("modelType", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "name" => Some(("name", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "state" => Some(("state", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "training-stats.analyzed-conversations-count" => Some(("trainingStats.analyzedConversationsCount", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "training-stats.unclassified-conversations-count" => Some(("trainingStats.unclassifiedConversationsCount", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "update-time" => Some(("updateTime", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    _ => {
                        let suggestion = FieldCursor::did_you_mean(key, &vec!["analyzed-conversations-count", "create-time", "display-name", "filter", "input-data-config", "issue-count", "language-code", "medium", "model-type", "name", "state", "training-conversations-count", "training-stats", "unclassified-conversations-count", "update-time"]);
                        err.issues.push(CLIError::Field(FieldError::Unknown(temp_cursor.to_string(), suggestion, value.map(|v| v.to_string()))));
                        None
                    }
                };
            if let Some((field_cursor_str, type_info)) = type_info {
                FieldCursor::from(field_cursor_str).set_json_value(&mut object, value.unwrap(), type_info, err, &temp_cursor);
            }
        }
        let mut request: api::GoogleCloudContactcenterinsightsV1IssueModel = json::value::from_value(object).unwrap();
        let mut call = self.hub.projects().locations_issue_models_patch(request, opt.value_of("name").unwrap_or(""));
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

    async fn _projects_locations_issue_models_undeploy(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
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
        let mut request: api::GoogleCloudContactcenterinsightsV1UndeployIssueModelRequest = json::value::from_value(object).unwrap();
        let mut call = self.hub.projects().locations_issue_models_undeploy(request, opt.value_of("name").unwrap_or(""));
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

    async fn _projects_locations_phrase_matchers_create(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
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
                    "activation-update-time" => Some(("activationUpdateTime", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "active" => Some(("active", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "display-name" => Some(("displayName", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "name" => Some(("name", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "revision-create-time" => Some(("revisionCreateTime", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "revision-id" => Some(("revisionId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "role-match" => Some(("roleMatch", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "type" => Some(("type", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "update-time" => Some(("updateTime", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "version-tag" => Some(("versionTag", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    _ => {
                        let suggestion = FieldCursor::did_you_mean(key, &vec!["activation-update-time", "active", "display-name", "name", "revision-create-time", "revision-id", "role-match", "type", "update-time", "version-tag"]);
                        err.issues.push(CLIError::Field(FieldError::Unknown(temp_cursor.to_string(), suggestion, value.map(|v| v.to_string()))));
                        None
                    }
                };
            if let Some((field_cursor_str, type_info)) = type_info {
                FieldCursor::from(field_cursor_str).set_json_value(&mut object, value.unwrap(), type_info, err, &temp_cursor);
            }
        }
        let mut request: api::GoogleCloudContactcenterinsightsV1PhraseMatcher = json::value::from_value(object).unwrap();
        let mut call = self.hub.projects().locations_phrase_matchers_create(request, opt.value_of("parent").unwrap_or(""));
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

    async fn _projects_locations_phrase_matchers_delete(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        let mut call = self.hub.projects().locations_phrase_matchers_delete(opt.value_of("name").unwrap_or(""));
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

    async fn _projects_locations_phrase_matchers_get(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        let mut call = self.hub.projects().locations_phrase_matchers_get(opt.value_of("name").unwrap_or(""));
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

    async fn _projects_locations_phrase_matchers_list(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        let mut call = self.hub.projects().locations_phrase_matchers_list(opt.value_of("parent").unwrap_or(""));
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

    async fn _projects_locations_phrase_matchers_patch(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
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
                    "activation-update-time" => Some(("activationUpdateTime", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "active" => Some(("active", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "display-name" => Some(("displayName", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "name" => Some(("name", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "revision-create-time" => Some(("revisionCreateTime", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "revision-id" => Some(("revisionId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "role-match" => Some(("roleMatch", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "type" => Some(("type", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "update-time" => Some(("updateTime", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "version-tag" => Some(("versionTag", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    _ => {
                        let suggestion = FieldCursor::did_you_mean(key, &vec!["activation-update-time", "active", "display-name", "name", "revision-create-time", "revision-id", "role-match", "type", "update-time", "version-tag"]);
                        err.issues.push(CLIError::Field(FieldError::Unknown(temp_cursor.to_string(), suggestion, value.map(|v| v.to_string()))));
                        None
                    }
                };
            if let Some((field_cursor_str, type_info)) = type_info {
                FieldCursor::from(field_cursor_str).set_json_value(&mut object, value.unwrap(), type_info, err, &temp_cursor);
            }
        }
        let mut request: api::GoogleCloudContactcenterinsightsV1PhraseMatcher = json::value::from_value(object).unwrap();
        let mut call = self.hub.projects().locations_phrase_matchers_patch(request, opt.value_of("name").unwrap_or(""));
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

    async fn _projects_locations_update_settings(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
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
                    "analysis-config.annotator-selector.issue-models" => Some(("analysisConfig.annotatorSelector.issueModels", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Vec })),
                    "analysis-config.annotator-selector.phrase-matchers" => Some(("analysisConfig.annotatorSelector.phraseMatchers", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Vec })),
                    "analysis-config.annotator-selector.run-entity-annotator" => Some(("analysisConfig.annotatorSelector.runEntityAnnotator", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "analysis-config.annotator-selector.run-intent-annotator" => Some(("analysisConfig.annotatorSelector.runIntentAnnotator", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "analysis-config.annotator-selector.run-interruption-annotator" => Some(("analysisConfig.annotatorSelector.runInterruptionAnnotator", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "analysis-config.annotator-selector.run-issue-model-annotator" => Some(("analysisConfig.annotatorSelector.runIssueModelAnnotator", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "analysis-config.annotator-selector.run-phrase-matcher-annotator" => Some(("analysisConfig.annotatorSelector.runPhraseMatcherAnnotator", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "analysis-config.annotator-selector.run-sentiment-annotator" => Some(("analysisConfig.annotatorSelector.runSentimentAnnotator", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "analysis-config.annotator-selector.run-silence-annotator" => Some(("analysisConfig.annotatorSelector.runSilenceAnnotator", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "analysis-config.annotator-selector.run-summarization-annotator" => Some(("analysisConfig.annotatorSelector.runSummarizationAnnotator", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "analysis-config.annotator-selector.summarization-config.conversation-profile" => Some(("analysisConfig.annotatorSelector.summarizationConfig.conversationProfile", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "analysis-config.annotator-selector.summarization-config.summarization-model" => Some(("analysisConfig.annotatorSelector.summarizationConfig.summarizationModel", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "analysis-config.runtime-integration-analysis-percentage" => Some(("analysisConfig.runtimeIntegrationAnalysisPercentage", JsonTypeInfo { jtype: JsonType::Float, ctype: ComplexType::Pod })),
                    "analysis-config.upload-conversation-analysis-percentage" => Some(("analysisConfig.uploadConversationAnalysisPercentage", JsonTypeInfo { jtype: JsonType::Float, ctype: ComplexType::Pod })),
                    "conversation-ttl" => Some(("conversationTtl", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "create-time" => Some(("createTime", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "language-code" => Some(("languageCode", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "name" => Some(("name", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "pubsub-notification-settings" => Some(("pubsubNotificationSettings", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Map })),
                    "redaction-config.deidentify-template" => Some(("redactionConfig.deidentifyTemplate", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "redaction-config.inspect-template" => Some(("redactionConfig.inspectTemplate", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "speech-config.speech-recognizer" => Some(("speechConfig.speechRecognizer", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "update-time" => Some(("updateTime", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    _ => {
                        let suggestion = FieldCursor::did_you_mean(key, &vec!["analysis-config", "annotator-selector", "conversation-profile", "conversation-ttl", "create-time", "deidentify-template", "inspect-template", "issue-models", "language-code", "name", "phrase-matchers", "pubsub-notification-settings", "redaction-config", "run-entity-annotator", "run-intent-annotator", "run-interruption-annotator", "run-issue-model-annotator", "run-phrase-matcher-annotator", "run-sentiment-annotator", "run-silence-annotator", "run-summarization-annotator", "runtime-integration-analysis-percentage", "speech-config", "speech-recognizer", "summarization-config", "summarization-model", "update-time", "upload-conversation-analysis-percentage"]);
                        err.issues.push(CLIError::Field(FieldError::Unknown(temp_cursor.to_string(), suggestion, value.map(|v| v.to_string()))));
                        None
                    }
                };
            if let Some((field_cursor_str, type_info)) = type_info {
                FieldCursor::from(field_cursor_str).set_json_value(&mut object, value.unwrap(), type_info, err, &temp_cursor);
            }
        }
        let mut request: api::GoogleCloudContactcenterinsightsV1Settings = json::value::from_value(object).unwrap();
        let mut call = self.hub.projects().locations_update_settings(request, opt.value_of("name").unwrap_or(""));
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

    async fn _projects_locations_views_create(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
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
                    "display-name" => Some(("displayName", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "name" => Some(("name", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "update-time" => Some(("updateTime", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "value" => Some(("value", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    _ => {
                        let suggestion = FieldCursor::did_you_mean(key, &vec!["create-time", "display-name", "name", "update-time", "value"]);
                        err.issues.push(CLIError::Field(FieldError::Unknown(temp_cursor.to_string(), suggestion, value.map(|v| v.to_string()))));
                        None
                    }
                };
            if let Some((field_cursor_str, type_info)) = type_info {
                FieldCursor::from(field_cursor_str).set_json_value(&mut object, value.unwrap(), type_info, err, &temp_cursor);
            }
        }
        let mut request: api::GoogleCloudContactcenterinsightsV1View = json::value::from_value(object).unwrap();
        let mut call = self.hub.projects().locations_views_create(request, opt.value_of("parent").unwrap_or(""));
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

    async fn _projects_locations_views_delete(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        let mut call = self.hub.projects().locations_views_delete(opt.value_of("name").unwrap_or(""));
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

    async fn _projects_locations_views_get(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        let mut call = self.hub.projects().locations_views_get(opt.value_of("name").unwrap_or(""));
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

    async fn _projects_locations_views_list(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        let mut call = self.hub.projects().locations_views_list(opt.value_of("parent").unwrap_or(""));
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

    async fn _projects_locations_views_patch(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
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
                    "display-name" => Some(("displayName", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "name" => Some(("name", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "update-time" => Some(("updateTime", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "value" => Some(("value", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    _ => {
                        let suggestion = FieldCursor::did_you_mean(key, &vec!["create-time", "display-name", "name", "update-time", "value"]);
                        err.issues.push(CLIError::Field(FieldError::Unknown(temp_cursor.to_string(), suggestion, value.map(|v| v.to_string()))));
                        None
                    }
                };
            if let Some((field_cursor_str, type_info)) = type_info {
                FieldCursor::from(field_cursor_str).set_json_value(&mut object, value.unwrap(), type_info, err, &temp_cursor);
            }
        }
        let mut request: api::GoogleCloudContactcenterinsightsV1View = json::value::from_value(object).unwrap();
        let mut call = self.hub.projects().locations_views_patch(request, opt.value_of("name").unwrap_or(""));
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

    async fn _doit(&self, dry_run: bool) -> Result<Result<(), DoitError>, Option<InvalidOptionsError>> {
        let mut err = InvalidOptionsError::new();
        let mut call_result: Result<(), DoitError> = Ok(());
        let mut err_opt: Option<InvalidOptionsError> = None;
        match self.opt.subcommand() {
            ("projects", Some(opt)) => {
                match opt.subcommand() {
                    ("locations-conversations-analyses-create", Some(opt)) => {
                        call_result = self._projects_locations_conversations_analyses_create(opt, dry_run, &mut err).await;
                    },
                    ("locations-conversations-analyses-delete", Some(opt)) => {
                        call_result = self._projects_locations_conversations_analyses_delete(opt, dry_run, &mut err).await;
                    },
                    ("locations-conversations-analyses-get", Some(opt)) => {
                        call_result = self._projects_locations_conversations_analyses_get(opt, dry_run, &mut err).await;
                    },
                    ("locations-conversations-analyses-list", Some(opt)) => {
                        call_result = self._projects_locations_conversations_analyses_list(opt, dry_run, &mut err).await;
                    },
                    ("locations-conversations-bulk-analyze", Some(opt)) => {
                        call_result = self._projects_locations_conversations_bulk_analyze(opt, dry_run, &mut err).await;
                    },
                    ("locations-conversations-bulk-delete", Some(opt)) => {
                        call_result = self._projects_locations_conversations_bulk_delete(opt, dry_run, &mut err).await;
                    },
                    ("locations-conversations-calculate-stats", Some(opt)) => {
                        call_result = self._projects_locations_conversations_calculate_stats(opt, dry_run, &mut err).await;
                    },
                    ("locations-conversations-create", Some(opt)) => {
                        call_result = self._projects_locations_conversations_create(opt, dry_run, &mut err).await;
                    },
                    ("locations-conversations-delete", Some(opt)) => {
                        call_result = self._projects_locations_conversations_delete(opt, dry_run, &mut err).await;
                    },
                    ("locations-conversations-get", Some(opt)) => {
                        call_result = self._projects_locations_conversations_get(opt, dry_run, &mut err).await;
                    },
                    ("locations-conversations-ingest", Some(opt)) => {
                        call_result = self._projects_locations_conversations_ingest(opt, dry_run, &mut err).await;
                    },
                    ("locations-conversations-list", Some(opt)) => {
                        call_result = self._projects_locations_conversations_list(opt, dry_run, &mut err).await;
                    },
                    ("locations-conversations-patch", Some(opt)) => {
                        call_result = self._projects_locations_conversations_patch(opt, dry_run, &mut err).await;
                    },
                    ("locations-conversations-upload", Some(opt)) => {
                        call_result = self._projects_locations_conversations_upload(opt, dry_run, &mut err).await;
                    },
                    ("locations-get-settings", Some(opt)) => {
                        call_result = self._projects_locations_get_settings(opt, dry_run, &mut err).await;
                    },
                    ("locations-insightsdata-export", Some(opt)) => {
                        call_result = self._projects_locations_insightsdata_export(opt, dry_run, &mut err).await;
                    },
                    ("locations-issue-models-calculate-issue-model-stats", Some(opt)) => {
                        call_result = self._projects_locations_issue_models_calculate_issue_model_stats(opt, dry_run, &mut err).await;
                    },
                    ("locations-issue-models-create", Some(opt)) => {
                        call_result = self._projects_locations_issue_models_create(opt, dry_run, &mut err).await;
                    },
                    ("locations-issue-models-delete", Some(opt)) => {
                        call_result = self._projects_locations_issue_models_delete(opt, dry_run, &mut err).await;
                    },
                    ("locations-issue-models-deploy", Some(opt)) => {
                        call_result = self._projects_locations_issue_models_deploy(opt, dry_run, &mut err).await;
                    },
                    ("locations-issue-models-export", Some(opt)) => {
                        call_result = self._projects_locations_issue_models_export(opt, dry_run, &mut err).await;
                    },
                    ("locations-issue-models-get", Some(opt)) => {
                        call_result = self._projects_locations_issue_models_get(opt, dry_run, &mut err).await;
                    },
                    ("locations-issue-models-import", Some(opt)) => {
                        call_result = self._projects_locations_issue_models_import(opt, dry_run, &mut err).await;
                    },
                    ("locations-issue-models-issues-delete", Some(opt)) => {
                        call_result = self._projects_locations_issue_models_issues_delete(opt, dry_run, &mut err).await;
                    },
                    ("locations-issue-models-issues-get", Some(opt)) => {
                        call_result = self._projects_locations_issue_models_issues_get(opt, dry_run, &mut err).await;
                    },
                    ("locations-issue-models-issues-list", Some(opt)) => {
                        call_result = self._projects_locations_issue_models_issues_list(opt, dry_run, &mut err).await;
                    },
                    ("locations-issue-models-issues-patch", Some(opt)) => {
                        call_result = self._projects_locations_issue_models_issues_patch(opt, dry_run, &mut err).await;
                    },
                    ("locations-issue-models-list", Some(opt)) => {
                        call_result = self._projects_locations_issue_models_list(opt, dry_run, &mut err).await;
                    },
                    ("locations-issue-models-patch", Some(opt)) => {
                        call_result = self._projects_locations_issue_models_patch(opt, dry_run, &mut err).await;
                    },
                    ("locations-issue-models-undeploy", Some(opt)) => {
                        call_result = self._projects_locations_issue_models_undeploy(opt, dry_run, &mut err).await;
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
                    ("locations-phrase-matchers-create", Some(opt)) => {
                        call_result = self._projects_locations_phrase_matchers_create(opt, dry_run, &mut err).await;
                    },
                    ("locations-phrase-matchers-delete", Some(opt)) => {
                        call_result = self._projects_locations_phrase_matchers_delete(opt, dry_run, &mut err).await;
                    },
                    ("locations-phrase-matchers-get", Some(opt)) => {
                        call_result = self._projects_locations_phrase_matchers_get(opt, dry_run, &mut err).await;
                    },
                    ("locations-phrase-matchers-list", Some(opt)) => {
                        call_result = self._projects_locations_phrase_matchers_list(opt, dry_run, &mut err).await;
                    },
                    ("locations-phrase-matchers-patch", Some(opt)) => {
                        call_result = self._projects_locations_phrase_matchers_patch(opt, dry_run, &mut err).await;
                    },
                    ("locations-update-settings", Some(opt)) => {
                        call_result = self._projects_locations_update_settings(opt, dry_run, &mut err).await;
                    },
                    ("locations-views-create", Some(opt)) => {
                        call_result = self._projects_locations_views_create(opt, dry_run, &mut err).await;
                    },
                    ("locations-views-delete", Some(opt)) => {
                        call_result = self._projects_locations_views_delete(opt, dry_run, &mut err).await;
                    },
                    ("locations-views-get", Some(opt)) => {
                        call_result = self._projects_locations_views_get(opt, dry_run, &mut err).await;
                    },
                    ("locations-views-list", Some(opt)) => {
                        call_result = self._projects_locations_views_list(opt, dry_run, &mut err).await;
                    },
                    ("locations-views-patch", Some(opt)) => {
                        call_result = self._projects_locations_views_patch(opt, dry_run, &mut err).await;
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

            match client::application_secret_from_directory(&config_dir, "contactcenterinsights1-secret.json",
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
        ).persist_tokens_to_disk(format!("{}/contactcenterinsights1", config_dir)).build().await.unwrap();

        let engine = Engine {
            opt: opt,
            hub: api::Contactcenterinsights::new(client, auth),
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
        ("projects", "methods: 'locations-conversations-analyses-create', 'locations-conversations-analyses-delete', 'locations-conversations-analyses-get', 'locations-conversations-analyses-list', 'locations-conversations-bulk-analyze', 'locations-conversations-bulk-delete', 'locations-conversations-calculate-stats', 'locations-conversations-create', 'locations-conversations-delete', 'locations-conversations-get', 'locations-conversations-ingest', 'locations-conversations-list', 'locations-conversations-patch', 'locations-conversations-upload', 'locations-get-settings', 'locations-insightsdata-export', 'locations-issue-models-calculate-issue-model-stats', 'locations-issue-models-create', 'locations-issue-models-delete', 'locations-issue-models-deploy', 'locations-issue-models-export', 'locations-issue-models-get', 'locations-issue-models-import', 'locations-issue-models-issues-delete', 'locations-issue-models-issues-get', 'locations-issue-models-issues-list', 'locations-issue-models-issues-patch', 'locations-issue-models-list', 'locations-issue-models-patch', 'locations-issue-models-undeploy', 'locations-operations-cancel', 'locations-operations-get', 'locations-operations-list', 'locations-phrase-matchers-create', 'locations-phrase-matchers-delete', 'locations-phrase-matchers-get', 'locations-phrase-matchers-list', 'locations-phrase-matchers-patch', 'locations-update-settings', 'locations-views-create', 'locations-views-delete', 'locations-views-get', 'locations-views-list' and 'locations-views-patch'", vec![
            ("locations-conversations-analyses-create",
                    Some(r##"Creates an analysis. The long running operation is done when the analysis has completed."##),
                    "Details at http://byron.github.io/google-apis-rs/google_contactcenterinsights1_cli/projects_locations-conversations-analyses-create",
                  vec![
                    (Some(r##"parent"##),
                     None,
                     Some(r##"Required. The parent resource of the analysis."##),
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
            ("locations-conversations-analyses-delete",
                    Some(r##"Deletes an analysis."##),
                    "Details at http://byron.github.io/google-apis-rs/google_contactcenterinsights1_cli/projects_locations-conversations-analyses-delete",
                  vec![
                    (Some(r##"name"##),
                     None,
                     Some(r##"Required. The name of the analysis to delete."##),
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
            ("locations-conversations-analyses-get",
                    Some(r##"Gets an analysis."##),
                    "Details at http://byron.github.io/google-apis-rs/google_contactcenterinsights1_cli/projects_locations-conversations-analyses-get",
                  vec![
                    (Some(r##"name"##),
                     None,
                     Some(r##"Required. The name of the analysis to get."##),
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
            ("locations-conversations-analyses-list",
                    Some(r##"Lists analyses."##),
                    "Details at http://byron.github.io/google-apis-rs/google_contactcenterinsights1_cli/projects_locations-conversations-analyses-list",
                  vec![
                    (Some(r##"parent"##),
                     None,
                     Some(r##"Required. The parent resource of the analyses."##),
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
            ("locations-conversations-bulk-analyze",
                    Some(r##"Analyzes multiple conversations in a single request."##),
                    "Details at http://byron.github.io/google-apis-rs/google_contactcenterinsights1_cli/projects_locations-conversations-bulk-analyze",
                  vec![
                    (Some(r##"parent"##),
                     None,
                     Some(r##"Required. The parent resource to create analyses in."##),
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
            ("locations-conversations-bulk-delete",
                    Some(r##"Deletes multiple conversations in a single request."##),
                    "Details at http://byron.github.io/google-apis-rs/google_contactcenterinsights1_cli/projects_locations-conversations-bulk-delete",
                  vec![
                    (Some(r##"parent"##),
                     None,
                     Some(r##"Required. The parent resource to delete conversations from. Format: projects/{project}/locations/{location}"##),
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
            ("locations-conversations-calculate-stats",
                    Some(r##"Gets conversation statistics."##),
                    "Details at http://byron.github.io/google-apis-rs/google_contactcenterinsights1_cli/projects_locations-conversations-calculate-stats",
                  vec![
                    (Some(r##"location"##),
                     None,
                     Some(r##"Required. The location of the conversations."##),
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
            ("locations-conversations-create",
                    Some(r##"Creates a conversation. Does not support audio transcription or DLP redaction. Use `conversations.upload` instead."##),
                    "Details at http://byron.github.io/google-apis-rs/google_contactcenterinsights1_cli/projects_locations-conversations-create",
                  vec![
                    (Some(r##"parent"##),
                     None,
                     Some(r##"Required. The parent resource of the conversation."##),
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
            ("locations-conversations-delete",
                    Some(r##"Deletes a conversation."##),
                    "Details at http://byron.github.io/google-apis-rs/google_contactcenterinsights1_cli/projects_locations-conversations-delete",
                  vec![
                    (Some(r##"name"##),
                     None,
                     Some(r##"Required. The name of the conversation to delete."##),
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
            ("locations-conversations-get",
                    Some(r##"Gets a conversation."##),
                    "Details at http://byron.github.io/google-apis-rs/google_contactcenterinsights1_cli/projects_locations-conversations-get",
                  vec![
                    (Some(r##"name"##),
                     None,
                     Some(r##"Required. The name of the conversation to get."##),
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
            ("locations-conversations-ingest",
                    Some(r##"Imports conversations and processes them according to the user's configuration."##),
                    "Details at http://byron.github.io/google-apis-rs/google_contactcenterinsights1_cli/projects_locations-conversations-ingest",
                  vec![
                    (Some(r##"parent"##),
                     None,
                     Some(r##"Required. The parent resource for new conversations."##),
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
            ("locations-conversations-list",
                    Some(r##"Lists conversations."##),
                    "Details at http://byron.github.io/google-apis-rs/google_contactcenterinsights1_cli/projects_locations-conversations-list",
                  vec![
                    (Some(r##"parent"##),
                     None,
                     Some(r##"Required. The parent resource of the conversation."##),
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
            ("locations-conversations-patch",
                    Some(r##"Updates a conversation."##),
                    "Details at http://byron.github.io/google-apis-rs/google_contactcenterinsights1_cli/projects_locations-conversations-patch",
                  vec![
                    (Some(r##"name"##),
                     None,
                     Some(r##"Immutable. The resource name of the conversation. Format: projects/{project}/locations/{location}/conversations/{conversation}"##),
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
            ("locations-conversations-upload",
                    Some(r##"Create a long-running conversation upload operation. This method differs from `CreateConversation` by allowing audio transcription and optional DLP redaction."##),
                    "Details at http://byron.github.io/google-apis-rs/google_contactcenterinsights1_cli/projects_locations-conversations-upload",
                  vec![
                    (Some(r##"parent"##),
                     None,
                     Some(r##"Required. The parent resource of the conversation."##),
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
            ("locations-get-settings",
                    Some(r##"Gets project-level settings."##),
                    "Details at http://byron.github.io/google-apis-rs/google_contactcenterinsights1_cli/projects_locations-get-settings",
                  vec![
                    (Some(r##"name"##),
                     None,
                     Some(r##"Required. The name of the settings resource to get."##),
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
            ("locations-insightsdata-export",
                    Some(r##"Export insights data to a destination defined in the request body."##),
                    "Details at http://byron.github.io/google-apis-rs/google_contactcenterinsights1_cli/projects_locations-insightsdata-export",
                  vec![
                    (Some(r##"parent"##),
                     None,
                     Some(r##"Required. The parent resource to export data from."##),
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
            ("locations-issue-models-calculate-issue-model-stats",
                    Some(r##"Gets an issue model's statistics."##),
                    "Details at http://byron.github.io/google-apis-rs/google_contactcenterinsights1_cli/projects_locations-issue-models-calculate-issue-model-stats",
                  vec![
                    (Some(r##"issue-model"##),
                     None,
                     Some(r##"Required. The resource name of the issue model to query against."##),
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
            ("locations-issue-models-create",
                    Some(r##"Creates an issue model."##),
                    "Details at http://byron.github.io/google-apis-rs/google_contactcenterinsights1_cli/projects_locations-issue-models-create",
                  vec![
                    (Some(r##"parent"##),
                     None,
                     Some(r##"Required. The parent resource of the issue model."##),
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
            ("locations-issue-models-delete",
                    Some(r##"Deletes an issue model."##),
                    "Details at http://byron.github.io/google-apis-rs/google_contactcenterinsights1_cli/projects_locations-issue-models-delete",
                  vec![
                    (Some(r##"name"##),
                     None,
                     Some(r##"Required. The name of the issue model to delete."##),
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
            ("locations-issue-models-deploy",
                    Some(r##"Deploys an issue model. Returns an error if a model is already deployed. An issue model can only be used in analysis after it has been deployed."##),
                    "Details at http://byron.github.io/google-apis-rs/google_contactcenterinsights1_cli/projects_locations-issue-models-deploy",
                  vec![
                    (Some(r##"name"##),
                     None,
                     Some(r##"Required. The issue model to deploy."##),
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
            ("locations-issue-models-export",
                    Some(r##"Exports an issue model to the provided destination."##),
                    "Details at http://byron.github.io/google-apis-rs/google_contactcenterinsights1_cli/projects_locations-issue-models-export",
                  vec![
                    (Some(r##"name"##),
                     None,
                     Some(r##"Required. The issue model to export."##),
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
            ("locations-issue-models-get",
                    Some(r##"Gets an issue model."##),
                    "Details at http://byron.github.io/google-apis-rs/google_contactcenterinsights1_cli/projects_locations-issue-models-get",
                  vec![
                    (Some(r##"name"##),
                     None,
                     Some(r##"Required. The name of the issue model to get."##),
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
            ("locations-issue-models-import",
                    Some(r##"Imports an issue model from a Cloud Storage bucket."##),
                    "Details at http://byron.github.io/google-apis-rs/google_contactcenterinsights1_cli/projects_locations-issue-models-import",
                  vec![
                    (Some(r##"parent"##),
                     None,
                     Some(r##"Required. The parent resource of the issue model."##),
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
            ("locations-issue-models-issues-delete",
                    Some(r##"Deletes an issue."##),
                    "Details at http://byron.github.io/google-apis-rs/google_contactcenterinsights1_cli/projects_locations-issue-models-issues-delete",
                  vec![
                    (Some(r##"name"##),
                     None,
                     Some(r##"Required. The name of the issue to delete."##),
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
            ("locations-issue-models-issues-get",
                    Some(r##"Gets an issue."##),
                    "Details at http://byron.github.io/google-apis-rs/google_contactcenterinsights1_cli/projects_locations-issue-models-issues-get",
                  vec![
                    (Some(r##"name"##),
                     None,
                     Some(r##"Required. The name of the issue to get."##),
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
            ("locations-issue-models-issues-list",
                    Some(r##"Lists issues."##),
                    "Details at http://byron.github.io/google-apis-rs/google_contactcenterinsights1_cli/projects_locations-issue-models-issues-list",
                  vec![
                    (Some(r##"parent"##),
                     None,
                     Some(r##"Required. The parent resource of the issue."##),
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
            ("locations-issue-models-issues-patch",
                    Some(r##"Updates an issue."##),
                    "Details at http://byron.github.io/google-apis-rs/google_contactcenterinsights1_cli/projects_locations-issue-models-issues-patch",
                  vec![
                    (Some(r##"name"##),
                     None,
                     Some(r##"Immutable. The resource name of the issue. Format: projects/{project}/locations/{location}/issueModels/{issue_model}/issues/{issue}"##),
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
            ("locations-issue-models-list",
                    Some(r##"Lists issue models."##),
                    "Details at http://byron.github.io/google-apis-rs/google_contactcenterinsights1_cli/projects_locations-issue-models-list",
                  vec![
                    (Some(r##"parent"##),
                     None,
                     Some(r##"Required. The parent resource of the issue model."##),
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
            ("locations-issue-models-patch",
                    Some(r##"Updates an issue model."##),
                    "Details at http://byron.github.io/google-apis-rs/google_contactcenterinsights1_cli/projects_locations-issue-models-patch",
                  vec![
                    (Some(r##"name"##),
                     None,
                     Some(r##"Immutable. The resource name of the issue model. Format: projects/{project}/locations/{location}/issueModels/{issue_model}"##),
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
            ("locations-issue-models-undeploy",
                    Some(r##"Undeploys an issue model. An issue model can not be used in analysis after it has been undeployed."##),
                    "Details at http://byron.github.io/google-apis-rs/google_contactcenterinsights1_cli/projects_locations-issue-models-undeploy",
                  vec![
                    (Some(r##"name"##),
                     None,
                     Some(r##"Required. The issue model to undeploy."##),
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
            ("locations-operations-cancel",
                    Some(r##"Starts asynchronous cancellation on a long-running operation. The server makes a best effort to cancel the operation, but success is not guaranteed. If the server doesn't support this method, it returns `google.rpc.Code.UNIMPLEMENTED`. Clients can use Operations.GetOperation or other methods to check whether the cancellation succeeded or whether the operation completed despite cancellation. On successful cancellation, the operation is not deleted; instead, it becomes an operation with an Operation.error value with a google.rpc.Status.code of 1, corresponding to `Code.CANCELLED`."##),
                    "Details at http://byron.github.io/google-apis-rs/google_contactcenterinsights1_cli/projects_locations-operations-cancel",
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
                    "Details at http://byron.github.io/google-apis-rs/google_contactcenterinsights1_cli/projects_locations-operations-get",
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
                    "Details at http://byron.github.io/google-apis-rs/google_contactcenterinsights1_cli/projects_locations-operations-list",
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
            ("locations-phrase-matchers-create",
                    Some(r##"Creates a phrase matcher."##),
                    "Details at http://byron.github.io/google-apis-rs/google_contactcenterinsights1_cli/projects_locations-phrase-matchers-create",
                  vec![
                    (Some(r##"parent"##),
                     None,
                     Some(r##"Required. The parent resource of the phrase matcher. Required. The location to create a phrase matcher for. Format: `projects//locations/` or `projects//locations/`"##),
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
            ("locations-phrase-matchers-delete",
                    Some(r##"Deletes a phrase matcher."##),
                    "Details at http://byron.github.io/google-apis-rs/google_contactcenterinsights1_cli/projects_locations-phrase-matchers-delete",
                  vec![
                    (Some(r##"name"##),
                     None,
                     Some(r##"Required. The name of the phrase matcher to delete."##),
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
            ("locations-phrase-matchers-get",
                    Some(r##"Gets a phrase matcher."##),
                    "Details at http://byron.github.io/google-apis-rs/google_contactcenterinsights1_cli/projects_locations-phrase-matchers-get",
                  vec![
                    (Some(r##"name"##),
                     None,
                     Some(r##"Required. The name of the phrase matcher to get."##),
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
            ("locations-phrase-matchers-list",
                    Some(r##"Lists phrase matchers."##),
                    "Details at http://byron.github.io/google-apis-rs/google_contactcenterinsights1_cli/projects_locations-phrase-matchers-list",
                  vec![
                    (Some(r##"parent"##),
                     None,
                     Some(r##"Required. The parent resource of the phrase matcher."##),
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
            ("locations-phrase-matchers-patch",
                    Some(r##"Updates a phrase matcher."##),
                    "Details at http://byron.github.io/google-apis-rs/google_contactcenterinsights1_cli/projects_locations-phrase-matchers-patch",
                  vec![
                    (Some(r##"name"##),
                     None,
                     Some(r##"The resource name of the phrase matcher. Format: projects/{project}/locations/{location}/phraseMatchers/{phrase_matcher}"##),
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
            ("locations-update-settings",
                    Some(r##"Updates project-level settings."##),
                    "Details at http://byron.github.io/google-apis-rs/google_contactcenterinsights1_cli/projects_locations-update-settings",
                  vec![
                    (Some(r##"name"##),
                     None,
                     Some(r##"Immutable. The resource name of the settings resource. Format: projects/{project}/locations/{location}/settings"##),
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
            ("locations-views-create",
                    Some(r##"Creates a view."##),
                    "Details at http://byron.github.io/google-apis-rs/google_contactcenterinsights1_cli/projects_locations-views-create",
                  vec![
                    (Some(r##"parent"##),
                     None,
                     Some(r##"Required. The parent resource of the view. Required. The location to create a view for. Format: `projects//locations/` or `projects//locations/`"##),
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
            ("locations-views-delete",
                    Some(r##"Deletes a view."##),
                    "Details at http://byron.github.io/google-apis-rs/google_contactcenterinsights1_cli/projects_locations-views-delete",
                  vec![
                    (Some(r##"name"##),
                     None,
                     Some(r##"Required. The name of the view to delete."##),
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
            ("locations-views-get",
                    Some(r##"Gets a view."##),
                    "Details at http://byron.github.io/google-apis-rs/google_contactcenterinsights1_cli/projects_locations-views-get",
                  vec![
                    (Some(r##"name"##),
                     None,
                     Some(r##"Required. The name of the view to get."##),
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
            ("locations-views-list",
                    Some(r##"Lists views."##),
                    "Details at http://byron.github.io/google-apis-rs/google_contactcenterinsights1_cli/projects_locations-views-list",
                  vec![
                    (Some(r##"parent"##),
                     None,
                     Some(r##"Required. The parent resource of the views."##),
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
            ("locations-views-patch",
                    Some(r##"Updates a view."##),
                    "Details at http://byron.github.io/google-apis-rs/google_contactcenterinsights1_cli/projects_locations-views-patch",
                  vec![
                    (Some(r##"name"##),
                     None,
                     Some(r##"Immutable. The resource name of the view. Format: projects/{project}/locations/{location}/views/{view}"##),
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
    
    let mut app = App::new("contactcenterinsights1")
           .author("Sebastian Thiel <byronimo@gmail.com>")
           .version("5.0.5+20240624")
           .about("")
           .after_help("All documentation details can be found at http://byron.github.io/google-apis-rs/google_contactcenterinsights1_cli")
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
