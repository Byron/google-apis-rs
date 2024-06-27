// DO NOT EDIT !
// This file was generated automatically from 'src/generator/templates/cli/main.rs.mako'
// DO NOT EDIT !
#![allow(unused_variables, unused_imports, dead_code, unused_mut)]

#[macro_use]
extern crate clap;

use std::env;
use std::io::{self, Write};
use clap::{App, SubCommand, Arg};

use google_youtube3::{api, Error, oauth2, client::chrono, FieldMask};


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
    hub: api::YouTube<S>,
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
    async fn _abuse_reports_insert(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
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
                    "subject.id" => Some(("subject.id", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "subject.type-id" => Some(("subject.typeId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "subject.url" => Some(("subject.url", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    _ => {
                        let suggestion = FieldCursor::did_you_mean(key, &vec!["description", "id", "subject", "type-id", "url"]);
                        err.issues.push(CLIError::Field(FieldError::Unknown(temp_cursor.to_string(), suggestion, value.map(|v| v.to_string()))));
                        None
                    }
                };
            if let Some((field_cursor_str, type_info)) = type_info {
                FieldCursor::from(field_cursor_str).set_json_value(&mut object, value.unwrap(), type_info, err, &temp_cursor);
            }
        }
        let mut request: api::AbuseReport = json::value::from_value(object).unwrap();
        let mut call = self.hub.abuse_reports().insert(request);
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

    async fn _activities_list(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        let mut call = self.hub.activities().list(&opt.values_of("part").map(|i|i.collect()).unwrap_or(Vec::new()).iter().map(|&v| v.to_string()).collect::<Vec<String>>());
        for parg in opt.values_of("v").map(|i|i.collect()).unwrap_or(Vec::new()).iter() {
            let (key, value) = parse_kv_arg(&*parg, err, false);
            match key {
                "region-code" => {
                    call = call.region_code(value.unwrap_or(""));
                },
                "published-before" => {
                    call = call.published_before(        value.map(|v| arg_from_str(v, err, "published-before", "google-datetime")).unwrap_or(chrono::Utc::now()));
                },
                "published-after" => {
                    call = call.published_after(        value.map(|v| arg_from_str(v, err, "published-after", "google-datetime")).unwrap_or(chrono::Utc::now()));
                },
                "page-token" => {
                    call = call.page_token(value.unwrap_or(""));
                },
                "mine" => {
                    call = call.mine(        value.map(|v| arg_from_str(v, err, "mine", "boolean")).unwrap_or(false));
                },
                "max-results" => {
                    call = call.max_results(        value.map(|v| arg_from_str(v, err, "max-results", "uint32")).unwrap_or(0));
                },
                "home" => {
                    call = call.home(        value.map(|v| arg_from_str(v, err, "home", "boolean")).unwrap_or(false));
                },
                "channel-id" => {
                    call = call.channel_id(value.unwrap_or(""));
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
                                                                           v.extend(["channel-id", "home", "max-results", "mine", "page-token", "published-after", "published-before", "region-code"].iter().map(|v|*v));
                                                                           v } ));
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

    async fn _captions_delete(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        let mut call = self.hub.captions().delete(opt.value_of("id").unwrap_or(""));
        for parg in opt.values_of("v").map(|i|i.collect()).unwrap_or(Vec::new()).iter() {
            let (key, value) = parse_kv_arg(&*parg, err, false);
            match key {
                "on-behalf-of-content-owner" => {
                    call = call.on_behalf_of_content_owner(value.unwrap_or(""));
                },
                "on-behalf-of" => {
                    call = call.on_behalf_of(value.unwrap_or(""));
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
                                                                           v.extend(["on-behalf-of", "on-behalf-of-content-owner"].iter().map(|v|*v));
                                                                           v } ));
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
            match match protocol {
                CallType::Standard => call.doit().await,
                _ => unreachable!()
            } {
                Err(api_err) => Err(DoitError::ApiError(api_err)),
                Ok(mut response) => {
                    Ok(())
                }
            }
        }
    }

    async fn _captions_download(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        let mut download_mode = false;
        let mut call = self.hub.captions().download(opt.value_of("id").unwrap_or(""));
        for parg in opt.values_of("v").map(|i|i.collect()).unwrap_or(Vec::new()).iter() {
            let (key, value) = parse_kv_arg(&*parg, err, false);
            match key {
                "tlang" => {
                    call = call.tlang(value.unwrap_or(""));
                },
                "tfmt" => {
                    call = call.tfmt(value.unwrap_or(""));
                },
                "on-behalf-of-content-owner" => {
                    call = call.on_behalf_of_content_owner(value.unwrap_or(""));
                },
                "on-behalf-of" => {
                    call = call.on_behalf_of(value.unwrap_or(""));
                },
                _ => {
                    let mut found = false;
                    for param in &self.gp {
                        if key == *param {
                            if key == "alt" && value.unwrap_or("unset") == "media" {
                                download_mode = true;
                            }
                            found = true;
                            call = call.param(self.gpm.iter().find(|t| t.0 == key).unwrap_or(&("", key)).1, value.unwrap_or("unset"));
                            break;
                        }
                    }
                    if !found {
                        err.issues.push(CLIError::UnknownParameter(key.to_string(),
                                                                  {let mut v = Vec::new();
                                                                           v.extend(self.gp.iter().map(|v|*v));
                                                                           v.extend(["on-behalf-of", "on-behalf-of-content-owner", "tfmt", "tlang"].iter().map(|v|*v));
                                                                           v } ));
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
                Ok(mut response) => {
                    if !download_mode {
                    } else {
                    let bytes = hyper::body::to_bytes(response.into_body()).await.expect("a string as API currently is inefficient").to_vec();
                    ostream.write_all(&bytes).expect("write to be complete");
                    ostream.flush().expect("io to never fail which should really be fixed one day");
                    }
                    Ok(())
                }
            }
        }
    }

    async fn _captions_insert(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
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
                    "etag" => Some(("etag", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "id" => Some(("id", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "kind" => Some(("kind", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "snippet.audio-track-type" => Some(("snippet.audioTrackType", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "snippet.failure-reason" => Some(("snippet.failureReason", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "snippet.is-auto-synced" => Some(("snippet.isAutoSynced", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "snippet.is-cc" => Some(("snippet.isCC", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "snippet.is-draft" => Some(("snippet.isDraft", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "snippet.is-easy-reader" => Some(("snippet.isEasyReader", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "snippet.is-large" => Some(("snippet.isLarge", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "snippet.language" => Some(("snippet.language", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "snippet.last-updated" => Some(("snippet.lastUpdated", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "snippet.name" => Some(("snippet.name", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "snippet.status" => Some(("snippet.status", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "snippet.track-kind" => Some(("snippet.trackKind", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "snippet.video-id" => Some(("snippet.videoId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    _ => {
                        let suggestion = FieldCursor::did_you_mean(key, &vec!["audio-track-type", "etag", "failure-reason", "id", "is-auto-synced", "is-cc", "is-draft", "is-easy-reader", "is-large", "kind", "language", "last-updated", "name", "snippet", "status", "track-kind", "video-id"]);
                        err.issues.push(CLIError::Field(FieldError::Unknown(temp_cursor.to_string(), suggestion, value.map(|v| v.to_string()))));
                        None
                    }
                };
            if let Some((field_cursor_str, type_info)) = type_info {
                FieldCursor::from(field_cursor_str).set_json_value(&mut object, value.unwrap(), type_info, err, &temp_cursor);
            }
        }
        let mut request: api::Caption = json::value::from_value(object).unwrap();
        let mut call = self.hub.captions().insert(request);
        for parg in opt.values_of("v").map(|i|i.collect()).unwrap_or(Vec::new()).iter() {
            let (key, value) = parse_kv_arg(&*parg, err, false);
            match key {
                "sync" => {
                    call = call.sync(        value.map(|v| arg_from_str(v, err, "sync", "boolean")).unwrap_or(false));
                },
                "on-behalf-of-content-owner" => {
                    call = call.on_behalf_of_content_owner(value.unwrap_or(""));
                },
                "on-behalf-of" => {
                    call = call.on_behalf_of(value.unwrap_or(""));
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
                                                                           v.extend(["on-behalf-of", "on-behalf-of-content-owner", "sync"].iter().map(|v|*v));
                                                                           v } ));
                    }
                }
            }
        }
        let vals = opt.values_of("mode").unwrap().collect::<Vec<&str>>();
        let protocol = calltype_from_str(vals[0], ["simple"].iter().map(|&v| v.to_string()).collect(), err);
        let mut input_file = input_file_from_opts(vals[1], err);
        let mime_type = input_mime_from_opts(opt.value_of("mime").unwrap_or("application/octet-stream"), err);
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
                CallType::Upload(UploadProtocol::Simple) => call.upload(input_file.unwrap(), mime_type.unwrap()).await,
                CallType::Standard => unreachable!()
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

    async fn _captions_list(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        let mut call = self.hub.captions().list(&opt.values_of("part").map(|i|i.collect()).unwrap_or(Vec::new()).iter().map(|&v| v.to_string()).collect::<Vec<String>>(), opt.value_of("video-id").unwrap_or(""));
        for parg in opt.values_of("v").map(|i|i.collect()).unwrap_or(Vec::new()).iter() {
            let (key, value) = parse_kv_arg(&*parg, err, false);
            match key {
                "on-behalf-of-content-owner" => {
                    call = call.on_behalf_of_content_owner(value.unwrap_or(""));
                },
                "on-behalf-of" => {
                    call = call.on_behalf_of(value.unwrap_or(""));
                },
                "id" => {
                    call = call.add_id(value.unwrap_or(""));
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
                                                                           v.extend(["id", "on-behalf-of", "on-behalf-of-content-owner"].iter().map(|v|*v));
                                                                           v } ));
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

    async fn _captions_update(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
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
                    "etag" => Some(("etag", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "id" => Some(("id", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "kind" => Some(("kind", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "snippet.audio-track-type" => Some(("snippet.audioTrackType", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "snippet.failure-reason" => Some(("snippet.failureReason", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "snippet.is-auto-synced" => Some(("snippet.isAutoSynced", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "snippet.is-cc" => Some(("snippet.isCC", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "snippet.is-draft" => Some(("snippet.isDraft", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "snippet.is-easy-reader" => Some(("snippet.isEasyReader", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "snippet.is-large" => Some(("snippet.isLarge", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "snippet.language" => Some(("snippet.language", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "snippet.last-updated" => Some(("snippet.lastUpdated", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "snippet.name" => Some(("snippet.name", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "snippet.status" => Some(("snippet.status", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "snippet.track-kind" => Some(("snippet.trackKind", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "snippet.video-id" => Some(("snippet.videoId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    _ => {
                        let suggestion = FieldCursor::did_you_mean(key, &vec!["audio-track-type", "etag", "failure-reason", "id", "is-auto-synced", "is-cc", "is-draft", "is-easy-reader", "is-large", "kind", "language", "last-updated", "name", "snippet", "status", "track-kind", "video-id"]);
                        err.issues.push(CLIError::Field(FieldError::Unknown(temp_cursor.to_string(), suggestion, value.map(|v| v.to_string()))));
                        None
                    }
                };
            if let Some((field_cursor_str, type_info)) = type_info {
                FieldCursor::from(field_cursor_str).set_json_value(&mut object, value.unwrap(), type_info, err, &temp_cursor);
            }
        }
        let mut request: api::Caption = json::value::from_value(object).unwrap();
        let mut call = self.hub.captions().update(request);
        for parg in opt.values_of("v").map(|i|i.collect()).unwrap_or(Vec::new()).iter() {
            let (key, value) = parse_kv_arg(&*parg, err, false);
            match key {
                "sync" => {
                    call = call.sync(        value.map(|v| arg_from_str(v, err, "sync", "boolean")).unwrap_or(false));
                },
                "on-behalf-of-content-owner" => {
                    call = call.on_behalf_of_content_owner(value.unwrap_or(""));
                },
                "on-behalf-of" => {
                    call = call.on_behalf_of(value.unwrap_or(""));
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
                                                                           v.extend(["on-behalf-of", "on-behalf-of-content-owner", "sync"].iter().map(|v|*v));
                                                                           v } ));
                    }
                }
            }
        }
        let vals = opt.values_of("mode").unwrap().collect::<Vec<&str>>();
        let protocol = calltype_from_str(vals[0], ["simple"].iter().map(|&v| v.to_string()).collect(), err);
        let mut input_file = input_file_from_opts(vals[1], err);
        let mime_type = input_mime_from_opts(opt.value_of("mime").unwrap_or("application/octet-stream"), err);
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
                CallType::Upload(UploadProtocol::Simple) => call.upload(input_file.unwrap(), mime_type.unwrap()).await,
                CallType::Standard => unreachable!()
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

    async fn _channel_banners_insert(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
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
                    "etag" => Some(("etag", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "kind" => Some(("kind", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "url" => Some(("url", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    _ => {
                        let suggestion = FieldCursor::did_you_mean(key, &vec!["etag", "kind", "url"]);
                        err.issues.push(CLIError::Field(FieldError::Unknown(temp_cursor.to_string(), suggestion, value.map(|v| v.to_string()))));
                        None
                    }
                };
            if let Some((field_cursor_str, type_info)) = type_info {
                FieldCursor::from(field_cursor_str).set_json_value(&mut object, value.unwrap(), type_info, err, &temp_cursor);
            }
        }
        let mut request: api::ChannelBannerResource = json::value::from_value(object).unwrap();
        let mut call = self.hub.channel_banners().insert(request);
        for parg in opt.values_of("v").map(|i|i.collect()).unwrap_or(Vec::new()).iter() {
            let (key, value) = parse_kv_arg(&*parg, err, false);
            match key {
                "on-behalf-of-content-owner-channel" => {
                    call = call.on_behalf_of_content_owner_channel(value.unwrap_or(""));
                },
                "on-behalf-of-content-owner" => {
                    call = call.on_behalf_of_content_owner(value.unwrap_or(""));
                },
                "channel-id" => {
                    call = call.channel_id(value.unwrap_or(""));
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
                                                                           v.extend(["channel-id", "on-behalf-of-content-owner", "on-behalf-of-content-owner-channel"].iter().map(|v|*v));
                                                                           v } ));
                    }
                }
            }
        }
        let vals = opt.values_of("mode").unwrap().collect::<Vec<&str>>();
        let protocol = calltype_from_str(vals[0], ["simple"].iter().map(|&v| v.to_string()).collect(), err);
        let mut input_file = input_file_from_opts(vals[1], err);
        let mime_type = input_mime_from_opts(opt.value_of("mime").unwrap_or("application/octet-stream"), err);
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
                CallType::Upload(UploadProtocol::Simple) => call.upload(input_file.unwrap(), mime_type.unwrap()).await,
                CallType::Standard => unreachable!()
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

    async fn _channel_sections_delete(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        let mut call = self.hub.channel_sections().delete(opt.value_of("id").unwrap_or(""));
        for parg in opt.values_of("v").map(|i|i.collect()).unwrap_or(Vec::new()).iter() {
            let (key, value) = parse_kv_arg(&*parg, err, false);
            match key {
                "on-behalf-of-content-owner" => {
                    call = call.on_behalf_of_content_owner(value.unwrap_or(""));
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
                                                                           v.extend(["on-behalf-of-content-owner"].iter().map(|v|*v));
                                                                           v } ));
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
            match match protocol {
                CallType::Standard => call.doit().await,
                _ => unreachable!()
            } {
                Err(api_err) => Err(DoitError::ApiError(api_err)),
                Ok(mut response) => {
                    Ok(())
                }
            }
        }
    }

    async fn _channel_sections_insert(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
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
                    "content-details.channels" => Some(("contentDetails.channels", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Vec })),
                    "content-details.playlists" => Some(("contentDetails.playlists", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Vec })),
                    "etag" => Some(("etag", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "id" => Some(("id", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "kind" => Some(("kind", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "snippet.channel-id" => Some(("snippet.channelId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "snippet.default-language" => Some(("snippet.defaultLanguage", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "snippet.localized.title" => Some(("snippet.localized.title", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "snippet.position" => Some(("snippet.position", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "snippet.style" => Some(("snippet.style", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "snippet.title" => Some(("snippet.title", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "snippet.type" => Some(("snippet.type", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "targeting.countries" => Some(("targeting.countries", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Vec })),
                    "targeting.languages" => Some(("targeting.languages", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Vec })),
                    "targeting.regions" => Some(("targeting.regions", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Vec })),
                    _ => {
                        let suggestion = FieldCursor::did_you_mean(key, &vec!["channel-id", "channels", "content-details", "countries", "default-language", "etag", "id", "kind", "languages", "localized", "playlists", "position", "regions", "snippet", "style", "targeting", "title", "type"]);
                        err.issues.push(CLIError::Field(FieldError::Unknown(temp_cursor.to_string(), suggestion, value.map(|v| v.to_string()))));
                        None
                    }
                };
            if let Some((field_cursor_str, type_info)) = type_info {
                FieldCursor::from(field_cursor_str).set_json_value(&mut object, value.unwrap(), type_info, err, &temp_cursor);
            }
        }
        let mut request: api::ChannelSection = json::value::from_value(object).unwrap();
        let mut call = self.hub.channel_sections().insert(request);
        for parg in opt.values_of("v").map(|i|i.collect()).unwrap_or(Vec::new()).iter() {
            let (key, value) = parse_kv_arg(&*parg, err, false);
            match key {
                "on-behalf-of-content-owner-channel" => {
                    call = call.on_behalf_of_content_owner_channel(value.unwrap_or(""));
                },
                "on-behalf-of-content-owner" => {
                    call = call.on_behalf_of_content_owner(value.unwrap_or(""));
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
                                                                           v.extend(["on-behalf-of-content-owner", "on-behalf-of-content-owner-channel"].iter().map(|v|*v));
                                                                           v } ));
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

    async fn _channel_sections_list(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        let mut call = self.hub.channel_sections().list(&opt.values_of("part").map(|i|i.collect()).unwrap_or(Vec::new()).iter().map(|&v| v.to_string()).collect::<Vec<String>>());
        for parg in opt.values_of("v").map(|i|i.collect()).unwrap_or(Vec::new()).iter() {
            let (key, value) = parse_kv_arg(&*parg, err, false);
            match key {
                "on-behalf-of-content-owner" => {
                    call = call.on_behalf_of_content_owner(value.unwrap_or(""));
                },
                "mine" => {
                    call = call.mine(        value.map(|v| arg_from_str(v, err, "mine", "boolean")).unwrap_or(false));
                },
                "id" => {
                    call = call.add_id(value.unwrap_or(""));
                },
                "hl" => {
                    call = call.hl(value.unwrap_or(""));
                },
                "channel-id" => {
                    call = call.channel_id(value.unwrap_or(""));
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
                                                                           v.extend(["channel-id", "hl", "id", "mine", "on-behalf-of-content-owner"].iter().map(|v|*v));
                                                                           v } ));
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

    async fn _channel_sections_update(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
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
                    "content-details.channels" => Some(("contentDetails.channels", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Vec })),
                    "content-details.playlists" => Some(("contentDetails.playlists", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Vec })),
                    "etag" => Some(("etag", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "id" => Some(("id", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "kind" => Some(("kind", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "snippet.channel-id" => Some(("snippet.channelId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "snippet.default-language" => Some(("snippet.defaultLanguage", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "snippet.localized.title" => Some(("snippet.localized.title", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "snippet.position" => Some(("snippet.position", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "snippet.style" => Some(("snippet.style", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "snippet.title" => Some(("snippet.title", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "snippet.type" => Some(("snippet.type", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "targeting.countries" => Some(("targeting.countries", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Vec })),
                    "targeting.languages" => Some(("targeting.languages", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Vec })),
                    "targeting.regions" => Some(("targeting.regions", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Vec })),
                    _ => {
                        let suggestion = FieldCursor::did_you_mean(key, &vec!["channel-id", "channels", "content-details", "countries", "default-language", "etag", "id", "kind", "languages", "localized", "playlists", "position", "regions", "snippet", "style", "targeting", "title", "type"]);
                        err.issues.push(CLIError::Field(FieldError::Unknown(temp_cursor.to_string(), suggestion, value.map(|v| v.to_string()))));
                        None
                    }
                };
            if let Some((field_cursor_str, type_info)) = type_info {
                FieldCursor::from(field_cursor_str).set_json_value(&mut object, value.unwrap(), type_info, err, &temp_cursor);
            }
        }
        let mut request: api::ChannelSection = json::value::from_value(object).unwrap();
        let mut call = self.hub.channel_sections().update(request);
        for parg in opt.values_of("v").map(|i|i.collect()).unwrap_or(Vec::new()).iter() {
            let (key, value) = parse_kv_arg(&*parg, err, false);
            match key {
                "on-behalf-of-content-owner" => {
                    call = call.on_behalf_of_content_owner(value.unwrap_or(""));
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
                                                                           v.extend(["on-behalf-of-content-owner"].iter().map(|v|*v));
                                                                           v } ));
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

    async fn _channels_list(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        let mut call = self.hub.channels().list(&opt.values_of("part").map(|i|i.collect()).unwrap_or(Vec::new()).iter().map(|&v| v.to_string()).collect::<Vec<String>>());
        for parg in opt.values_of("v").map(|i|i.collect()).unwrap_or(Vec::new()).iter() {
            let (key, value) = parse_kv_arg(&*parg, err, false);
            match key {
                "page-token" => {
                    call = call.page_token(value.unwrap_or(""));
                },
                "on-behalf-of-content-owner" => {
                    call = call.on_behalf_of_content_owner(value.unwrap_or(""));
                },
                "my-subscribers" => {
                    call = call.my_subscribers(        value.map(|v| arg_from_str(v, err, "my-subscribers", "boolean")).unwrap_or(false));
                },
                "mine" => {
                    call = call.mine(        value.map(|v| arg_from_str(v, err, "mine", "boolean")).unwrap_or(false));
                },
                "max-results" => {
                    call = call.max_results(        value.map(|v| arg_from_str(v, err, "max-results", "uint32")).unwrap_or(0));
                },
                "managed-by-me" => {
                    call = call.managed_by_me(        value.map(|v| arg_from_str(v, err, "managed-by-me", "boolean")).unwrap_or(false));
                },
                "id" => {
                    call = call.add_id(value.unwrap_or(""));
                },
                "hl" => {
                    call = call.hl(value.unwrap_or(""));
                },
                "for-username" => {
                    call = call.for_username(value.unwrap_or(""));
                },
                "for-handle" => {
                    call = call.for_handle(value.unwrap_or(""));
                },
                "category-id" => {
                    call = call.category_id(value.unwrap_or(""));
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
                                                                           v.extend(["category-id", "for-handle", "for-username", "hl", "id", "managed-by-me", "max-results", "mine", "my-subscribers", "on-behalf-of-content-owner", "page-token"].iter().map(|v|*v));
                                                                           v } ));
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

    async fn _channels_update(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
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
                    "audit-details.community-guidelines-good-standing" => Some(("auditDetails.communityGuidelinesGoodStanding", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "audit-details.content-id-claims-good-standing" => Some(("auditDetails.contentIdClaimsGoodStanding", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "audit-details.copyright-strikes-good-standing" => Some(("auditDetails.copyrightStrikesGoodStanding", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "branding-settings.channel.country" => Some(("brandingSettings.channel.country", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "branding-settings.channel.default-language" => Some(("brandingSettings.channel.defaultLanguage", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "branding-settings.channel.default-tab" => Some(("brandingSettings.channel.defaultTab", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "branding-settings.channel.description" => Some(("brandingSettings.channel.description", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "branding-settings.channel.featured-channels-title" => Some(("brandingSettings.channel.featuredChannelsTitle", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "branding-settings.channel.featured-channels-urls" => Some(("brandingSettings.channel.featuredChannelsUrls", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Vec })),
                    "branding-settings.channel.keywords" => Some(("brandingSettings.channel.keywords", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "branding-settings.channel.moderate-comments" => Some(("brandingSettings.channel.moderateComments", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "branding-settings.channel.profile-color" => Some(("brandingSettings.channel.profileColor", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "branding-settings.channel.show-browse-view" => Some(("brandingSettings.channel.showBrowseView", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "branding-settings.channel.show-related-channels" => Some(("brandingSettings.channel.showRelatedChannels", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "branding-settings.channel.title" => Some(("brandingSettings.channel.title", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "branding-settings.channel.tracking-analytics-account-id" => Some(("brandingSettings.channel.trackingAnalyticsAccountId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "branding-settings.channel.unsubscribed-trailer" => Some(("brandingSettings.channel.unsubscribedTrailer", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "branding-settings.image.background-image-url.default" => Some(("brandingSettings.image.backgroundImageUrl.default", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "branding-settings.image.background-image-url.default-language.value" => Some(("brandingSettings.image.backgroundImageUrl.defaultLanguage.value", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "branding-settings.image.banner-external-url" => Some(("brandingSettings.image.bannerExternalUrl", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "branding-settings.image.banner-image-url" => Some(("brandingSettings.image.bannerImageUrl", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "branding-settings.image.banner-mobile-extra-hd-image-url" => Some(("brandingSettings.image.bannerMobileExtraHdImageUrl", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "branding-settings.image.banner-mobile-hd-image-url" => Some(("brandingSettings.image.bannerMobileHdImageUrl", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "branding-settings.image.banner-mobile-image-url" => Some(("brandingSettings.image.bannerMobileImageUrl", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "branding-settings.image.banner-mobile-low-image-url" => Some(("brandingSettings.image.bannerMobileLowImageUrl", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "branding-settings.image.banner-mobile-medium-hd-image-url" => Some(("brandingSettings.image.bannerMobileMediumHdImageUrl", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "branding-settings.image.banner-tablet-extra-hd-image-url" => Some(("brandingSettings.image.bannerTabletExtraHdImageUrl", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "branding-settings.image.banner-tablet-hd-image-url" => Some(("brandingSettings.image.bannerTabletHdImageUrl", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "branding-settings.image.banner-tablet-image-url" => Some(("brandingSettings.image.bannerTabletImageUrl", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "branding-settings.image.banner-tablet-low-image-url" => Some(("brandingSettings.image.bannerTabletLowImageUrl", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "branding-settings.image.banner-tv-high-image-url" => Some(("brandingSettings.image.bannerTvHighImageUrl", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "branding-settings.image.banner-tv-image-url" => Some(("brandingSettings.image.bannerTvImageUrl", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "branding-settings.image.banner-tv-low-image-url" => Some(("brandingSettings.image.bannerTvLowImageUrl", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "branding-settings.image.banner-tv-medium-image-url" => Some(("brandingSettings.image.bannerTvMediumImageUrl", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "branding-settings.image.large-branded-banner-image-imap-script.default" => Some(("brandingSettings.image.largeBrandedBannerImageImapScript.default", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "branding-settings.image.large-branded-banner-image-imap-script.default-language.value" => Some(("brandingSettings.image.largeBrandedBannerImageImapScript.defaultLanguage.value", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "branding-settings.image.large-branded-banner-image-url.default" => Some(("brandingSettings.image.largeBrandedBannerImageUrl.default", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "branding-settings.image.large-branded-banner-image-url.default-language.value" => Some(("brandingSettings.image.largeBrandedBannerImageUrl.defaultLanguage.value", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "branding-settings.image.small-branded-banner-image-imap-script.default" => Some(("brandingSettings.image.smallBrandedBannerImageImapScript.default", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "branding-settings.image.small-branded-banner-image-imap-script.default-language.value" => Some(("brandingSettings.image.smallBrandedBannerImageImapScript.defaultLanguage.value", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "branding-settings.image.small-branded-banner-image-url.default" => Some(("brandingSettings.image.smallBrandedBannerImageUrl.default", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "branding-settings.image.small-branded-banner-image-url.default-language.value" => Some(("brandingSettings.image.smallBrandedBannerImageUrl.defaultLanguage.value", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "branding-settings.image.tracking-image-url" => Some(("brandingSettings.image.trackingImageUrl", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "branding-settings.image.watch-icon-image-url" => Some(("brandingSettings.image.watchIconImageUrl", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "branding-settings.watch.background-color" => Some(("brandingSettings.watch.backgroundColor", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "branding-settings.watch.featured-playlist-id" => Some(("brandingSettings.watch.featuredPlaylistId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "branding-settings.watch.text-color" => Some(("brandingSettings.watch.textColor", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "content-details.related-playlists.favorites" => Some(("contentDetails.relatedPlaylists.favorites", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "content-details.related-playlists.likes" => Some(("contentDetails.relatedPlaylists.likes", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "content-details.related-playlists.uploads" => Some(("contentDetails.relatedPlaylists.uploads", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "content-details.related-playlists.watch-history" => Some(("contentDetails.relatedPlaylists.watchHistory", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "content-details.related-playlists.watch-later" => Some(("contentDetails.relatedPlaylists.watchLater", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "content-owner-details.content-owner" => Some(("contentOwnerDetails.contentOwner", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "content-owner-details.time-linked" => Some(("contentOwnerDetails.timeLinked", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "etag" => Some(("etag", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "id" => Some(("id", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "kind" => Some(("kind", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "snippet.country" => Some(("snippet.country", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "snippet.custom-url" => Some(("snippet.customUrl", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "snippet.default-language" => Some(("snippet.defaultLanguage", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "snippet.description" => Some(("snippet.description", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "snippet.localized.description" => Some(("snippet.localized.description", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "snippet.localized.title" => Some(("snippet.localized.title", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "snippet.published-at" => Some(("snippet.publishedAt", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "snippet.thumbnails.default.height" => Some(("snippet.thumbnails.default.height", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "snippet.thumbnails.default.url" => Some(("snippet.thumbnails.default.url", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "snippet.thumbnails.default.width" => Some(("snippet.thumbnails.default.width", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "snippet.thumbnails.high.height" => Some(("snippet.thumbnails.high.height", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "snippet.thumbnails.high.url" => Some(("snippet.thumbnails.high.url", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "snippet.thumbnails.high.width" => Some(("snippet.thumbnails.high.width", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "snippet.thumbnails.maxres.height" => Some(("snippet.thumbnails.maxres.height", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "snippet.thumbnails.maxres.url" => Some(("snippet.thumbnails.maxres.url", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "snippet.thumbnails.maxres.width" => Some(("snippet.thumbnails.maxres.width", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "snippet.thumbnails.medium.height" => Some(("snippet.thumbnails.medium.height", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "snippet.thumbnails.medium.url" => Some(("snippet.thumbnails.medium.url", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "snippet.thumbnails.medium.width" => Some(("snippet.thumbnails.medium.width", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "snippet.thumbnails.standard.height" => Some(("snippet.thumbnails.standard.height", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "snippet.thumbnails.standard.url" => Some(("snippet.thumbnails.standard.url", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "snippet.thumbnails.standard.width" => Some(("snippet.thumbnails.standard.width", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "snippet.title" => Some(("snippet.title", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "statistics.comment-count" => Some(("statistics.commentCount", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "statistics.hidden-subscriber-count" => Some(("statistics.hiddenSubscriberCount", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "statistics.subscriber-count" => Some(("statistics.subscriberCount", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "statistics.video-count" => Some(("statistics.videoCount", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "statistics.view-count" => Some(("statistics.viewCount", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "status.is-linked" => Some(("status.isLinked", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "status.long-uploads-status" => Some(("status.longUploadsStatus", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "status.made-for-kids" => Some(("status.madeForKids", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "status.privacy-status" => Some(("status.privacyStatus", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "status.self-declared-made-for-kids" => Some(("status.selfDeclaredMadeForKids", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "topic-details.topic-categories" => Some(("topicDetails.topicCategories", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Vec })),
                    "topic-details.topic-ids" => Some(("topicDetails.topicIds", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Vec })),
                    _ => {
                        let suggestion = FieldCursor::did_you_mean(key, &vec!["audit-details", "background-color", "background-image-url", "banner-external-url", "banner-image-url", "banner-mobile-extra-hd-image-url", "banner-mobile-hd-image-url", "banner-mobile-image-url", "banner-mobile-low-image-url", "banner-mobile-medium-hd-image-url", "banner-tablet-extra-hd-image-url", "banner-tablet-hd-image-url", "banner-tablet-image-url", "banner-tablet-low-image-url", "banner-tv-high-image-url", "banner-tv-image-url", "banner-tv-low-image-url", "banner-tv-medium-image-url", "branding-settings", "channel", "comment-count", "community-guidelines-good-standing", "content-details", "content-id-claims-good-standing", "content-owner", "content-owner-details", "copyright-strikes-good-standing", "country", "custom-url", "default", "default-language", "default-tab", "description", "etag", "favorites", "featured-channels-title", "featured-channels-urls", "featured-playlist-id", "height", "hidden-subscriber-count", "high", "id", "image", "is-linked", "keywords", "kind", "large-branded-banner-image-imap-script", "large-branded-banner-image-url", "likes", "localized", "long-uploads-status", "made-for-kids", "maxres", "medium", "moderate-comments", "privacy-status", "profile-color", "published-at", "related-playlists", "self-declared-made-for-kids", "show-browse-view", "show-related-channels", "small-branded-banner-image-imap-script", "small-branded-banner-image-url", "snippet", "standard", "statistics", "status", "subscriber-count", "text-color", "thumbnails", "time-linked", "title", "topic-categories", "topic-details", "topic-ids", "tracking-analytics-account-id", "tracking-image-url", "unsubscribed-trailer", "uploads", "url", "value", "video-count", "view-count", "watch", "watch-history", "watch-icon-image-url", "watch-later", "width"]);
                        err.issues.push(CLIError::Field(FieldError::Unknown(temp_cursor.to_string(), suggestion, value.map(|v| v.to_string()))));
                        None
                    }
                };
            if let Some((field_cursor_str, type_info)) = type_info {
                FieldCursor::from(field_cursor_str).set_json_value(&mut object, value.unwrap(), type_info, err, &temp_cursor);
            }
        }
        let mut request: api::Channel = json::value::from_value(object).unwrap();
        let mut call = self.hub.channels().update(request);
        for parg in opt.values_of("v").map(|i|i.collect()).unwrap_or(Vec::new()).iter() {
            let (key, value) = parse_kv_arg(&*parg, err, false);
            match key {
                "on-behalf-of-content-owner" => {
                    call = call.on_behalf_of_content_owner(value.unwrap_or(""));
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
                                                                           v.extend(["on-behalf-of-content-owner"].iter().map(|v|*v));
                                                                           v } ));
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

    async fn _comment_threads_insert(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
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
                    "etag" => Some(("etag", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "id" => Some(("id", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "kind" => Some(("kind", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "snippet.can-reply" => Some(("snippet.canReply", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "snippet.channel-id" => Some(("snippet.channelId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "snippet.is-public" => Some(("snippet.isPublic", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "snippet.top-level-comment.etag" => Some(("snippet.topLevelComment.etag", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "snippet.top-level-comment.id" => Some(("snippet.topLevelComment.id", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "snippet.top-level-comment.kind" => Some(("snippet.topLevelComment.kind", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "snippet.top-level-comment.snippet.author-channel-id.value" => Some(("snippet.topLevelComment.snippet.authorChannelId.value", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "snippet.top-level-comment.snippet.author-channel-url" => Some(("snippet.topLevelComment.snippet.authorChannelUrl", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "snippet.top-level-comment.snippet.author-display-name" => Some(("snippet.topLevelComment.snippet.authorDisplayName", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "snippet.top-level-comment.snippet.author-profile-image-url" => Some(("snippet.topLevelComment.snippet.authorProfileImageUrl", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "snippet.top-level-comment.snippet.can-rate" => Some(("snippet.topLevelComment.snippet.canRate", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "snippet.top-level-comment.snippet.channel-id" => Some(("snippet.topLevelComment.snippet.channelId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "snippet.top-level-comment.snippet.like-count" => Some(("snippet.topLevelComment.snippet.likeCount", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "snippet.top-level-comment.snippet.moderation-status" => Some(("snippet.topLevelComment.snippet.moderationStatus", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "snippet.top-level-comment.snippet.parent-id" => Some(("snippet.topLevelComment.snippet.parentId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "snippet.top-level-comment.snippet.published-at" => Some(("snippet.topLevelComment.snippet.publishedAt", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "snippet.top-level-comment.snippet.text-display" => Some(("snippet.topLevelComment.snippet.textDisplay", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "snippet.top-level-comment.snippet.text-original" => Some(("snippet.topLevelComment.snippet.textOriginal", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "snippet.top-level-comment.snippet.updated-at" => Some(("snippet.topLevelComment.snippet.updatedAt", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "snippet.top-level-comment.snippet.video-id" => Some(("snippet.topLevelComment.snippet.videoId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "snippet.top-level-comment.snippet.viewer-rating" => Some(("snippet.topLevelComment.snippet.viewerRating", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "snippet.total-reply-count" => Some(("snippet.totalReplyCount", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "snippet.video-id" => Some(("snippet.videoId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    _ => {
                        let suggestion = FieldCursor::did_you_mean(key, &vec!["author-channel-id", "author-channel-url", "author-display-name", "author-profile-image-url", "can-rate", "can-reply", "channel-id", "etag", "id", "is-public", "kind", "like-count", "moderation-status", "parent-id", "published-at", "snippet", "text-display", "text-original", "top-level-comment", "total-reply-count", "updated-at", "value", "video-id", "viewer-rating"]);
                        err.issues.push(CLIError::Field(FieldError::Unknown(temp_cursor.to_string(), suggestion, value.map(|v| v.to_string()))));
                        None
                    }
                };
            if let Some((field_cursor_str, type_info)) = type_info {
                FieldCursor::from(field_cursor_str).set_json_value(&mut object, value.unwrap(), type_info, err, &temp_cursor);
            }
        }
        let mut request: api::CommentThread = json::value::from_value(object).unwrap();
        let mut call = self.hub.comment_threads().insert(request);
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

    async fn _comment_threads_list(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        let mut call = self.hub.comment_threads().list(&opt.values_of("part").map(|i|i.collect()).unwrap_or(Vec::new()).iter().map(|&v| v.to_string()).collect::<Vec<String>>());
        for parg in opt.values_of("v").map(|i|i.collect()).unwrap_or(Vec::new()).iter() {
            let (key, value) = parse_kv_arg(&*parg, err, false);
            match key {
                "video-id" => {
                    call = call.video_id(value.unwrap_or(""));
                },
                "text-format" => {
                    call = call.text_format(value.unwrap_or(""));
                },
                "search-terms" => {
                    call = call.search_terms(value.unwrap_or(""));
                },
                "page-token" => {
                    call = call.page_token(value.unwrap_or(""));
                },
                "order" => {
                    call = call.order(value.unwrap_or(""));
                },
                "moderation-status" => {
                    call = call.moderation_status(value.unwrap_or(""));
                },
                "max-results" => {
                    call = call.max_results(        value.map(|v| arg_from_str(v, err, "max-results", "uint32")).unwrap_or(0));
                },
                "id" => {
                    call = call.add_id(value.unwrap_or(""));
                },
                "channel-id" => {
                    call = call.channel_id(value.unwrap_or(""));
                },
                "all-threads-related-to-channel-id" => {
                    call = call.all_threads_related_to_channel_id(value.unwrap_or(""));
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
                                                                           v.extend(["all-threads-related-to-channel-id", "channel-id", "id", "max-results", "moderation-status", "order", "page-token", "search-terms", "text-format", "video-id"].iter().map(|v|*v));
                                                                           v } ));
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

    async fn _comments_delete(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        let mut call = self.hub.comments().delete(opt.value_of("id").unwrap_or(""));
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
            match match protocol {
                CallType::Standard => call.doit().await,
                _ => unreachable!()
            } {
                Err(api_err) => Err(DoitError::ApiError(api_err)),
                Ok(mut response) => {
                    Ok(())
                }
            }
        }
    }

    async fn _comments_insert(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
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
                    "etag" => Some(("etag", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "id" => Some(("id", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "kind" => Some(("kind", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "snippet.author-channel-id.value" => Some(("snippet.authorChannelId.value", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "snippet.author-channel-url" => Some(("snippet.authorChannelUrl", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "snippet.author-display-name" => Some(("snippet.authorDisplayName", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "snippet.author-profile-image-url" => Some(("snippet.authorProfileImageUrl", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "snippet.can-rate" => Some(("snippet.canRate", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "snippet.channel-id" => Some(("snippet.channelId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "snippet.like-count" => Some(("snippet.likeCount", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "snippet.moderation-status" => Some(("snippet.moderationStatus", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "snippet.parent-id" => Some(("snippet.parentId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "snippet.published-at" => Some(("snippet.publishedAt", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "snippet.text-display" => Some(("snippet.textDisplay", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "snippet.text-original" => Some(("snippet.textOriginal", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "snippet.updated-at" => Some(("snippet.updatedAt", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "snippet.video-id" => Some(("snippet.videoId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "snippet.viewer-rating" => Some(("snippet.viewerRating", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    _ => {
                        let suggestion = FieldCursor::did_you_mean(key, &vec!["author-channel-id", "author-channel-url", "author-display-name", "author-profile-image-url", "can-rate", "channel-id", "etag", "id", "kind", "like-count", "moderation-status", "parent-id", "published-at", "snippet", "text-display", "text-original", "updated-at", "value", "video-id", "viewer-rating"]);
                        err.issues.push(CLIError::Field(FieldError::Unknown(temp_cursor.to_string(), suggestion, value.map(|v| v.to_string()))));
                        None
                    }
                };
            if let Some((field_cursor_str, type_info)) = type_info {
                FieldCursor::from(field_cursor_str).set_json_value(&mut object, value.unwrap(), type_info, err, &temp_cursor);
            }
        }
        let mut request: api::Comment = json::value::from_value(object).unwrap();
        let mut call = self.hub.comments().insert(request);
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

    async fn _comments_list(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        let mut call = self.hub.comments().list(&opt.values_of("part").map(|i|i.collect()).unwrap_or(Vec::new()).iter().map(|&v| v.to_string()).collect::<Vec<String>>());
        for parg in opt.values_of("v").map(|i|i.collect()).unwrap_or(Vec::new()).iter() {
            let (key, value) = parse_kv_arg(&*parg, err, false);
            match key {
                "text-format" => {
                    call = call.text_format(value.unwrap_or(""));
                },
                "parent-id" => {
                    call = call.parent_id(value.unwrap_or(""));
                },
                "page-token" => {
                    call = call.page_token(value.unwrap_or(""));
                },
                "max-results" => {
                    call = call.max_results(        value.map(|v| arg_from_str(v, err, "max-results", "uint32")).unwrap_or(0));
                },
                "id" => {
                    call = call.add_id(value.unwrap_or(""));
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
                                                                           v.extend(["id", "max-results", "page-token", "parent-id", "text-format"].iter().map(|v|*v));
                                                                           v } ));
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

    async fn _comments_mark_as_spam(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        let mut call = self.hub.comments().mark_as_spam(&opt.values_of("id").map(|i|i.collect()).unwrap_or(Vec::new()).iter().map(|&v| v.to_string()).collect::<Vec<String>>());
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
            match match protocol {
                CallType::Standard => call.doit().await,
                _ => unreachable!()
            } {
                Err(api_err) => Err(DoitError::ApiError(api_err)),
                Ok(mut response) => {
                    Ok(())
                }
            }
        }
    }

    async fn _comments_set_moderation_status(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        let mut call = self.hub.comments().set_moderation_status(&opt.values_of("id").map(|i|i.collect()).unwrap_or(Vec::new()).iter().map(|&v| v.to_string()).collect::<Vec<String>>(), opt.value_of("moderation-status").unwrap_or(""));
        for parg in opt.values_of("v").map(|i|i.collect()).unwrap_or(Vec::new()).iter() {
            let (key, value) = parse_kv_arg(&*parg, err, false);
            match key {
                "ban-author" => {
                    call = call.ban_author(        value.map(|v| arg_from_str(v, err, "ban-author", "boolean")).unwrap_or(false));
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
                                                                           v.extend(["ban-author"].iter().map(|v|*v));
                                                                           v } ));
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
            match match protocol {
                CallType::Standard => call.doit().await,
                _ => unreachable!()
            } {
                Err(api_err) => Err(DoitError::ApiError(api_err)),
                Ok(mut response) => {
                    Ok(())
                }
            }
        }
    }

    async fn _comments_update(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
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
                    "etag" => Some(("etag", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "id" => Some(("id", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "kind" => Some(("kind", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "snippet.author-channel-id.value" => Some(("snippet.authorChannelId.value", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "snippet.author-channel-url" => Some(("snippet.authorChannelUrl", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "snippet.author-display-name" => Some(("snippet.authorDisplayName", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "snippet.author-profile-image-url" => Some(("snippet.authorProfileImageUrl", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "snippet.can-rate" => Some(("snippet.canRate", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "snippet.channel-id" => Some(("snippet.channelId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "snippet.like-count" => Some(("snippet.likeCount", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "snippet.moderation-status" => Some(("snippet.moderationStatus", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "snippet.parent-id" => Some(("snippet.parentId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "snippet.published-at" => Some(("snippet.publishedAt", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "snippet.text-display" => Some(("snippet.textDisplay", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "snippet.text-original" => Some(("snippet.textOriginal", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "snippet.updated-at" => Some(("snippet.updatedAt", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "snippet.video-id" => Some(("snippet.videoId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "snippet.viewer-rating" => Some(("snippet.viewerRating", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    _ => {
                        let suggestion = FieldCursor::did_you_mean(key, &vec!["author-channel-id", "author-channel-url", "author-display-name", "author-profile-image-url", "can-rate", "channel-id", "etag", "id", "kind", "like-count", "moderation-status", "parent-id", "published-at", "snippet", "text-display", "text-original", "updated-at", "value", "video-id", "viewer-rating"]);
                        err.issues.push(CLIError::Field(FieldError::Unknown(temp_cursor.to_string(), suggestion, value.map(|v| v.to_string()))));
                        None
                    }
                };
            if let Some((field_cursor_str, type_info)) = type_info {
                FieldCursor::from(field_cursor_str).set_json_value(&mut object, value.unwrap(), type_info, err, &temp_cursor);
            }
        }
        let mut request: api::Comment = json::value::from_value(object).unwrap();
        let mut call = self.hub.comments().update(request);
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

    async fn _i18n_languages_list(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        let mut call = self.hub.i18n_languages().list(&opt.values_of("part").map(|i|i.collect()).unwrap_or(Vec::new()).iter().map(|&v| v.to_string()).collect::<Vec<String>>());
        for parg in opt.values_of("v").map(|i|i.collect()).unwrap_or(Vec::new()).iter() {
            let (key, value) = parse_kv_arg(&*parg, err, false);
            match key {
                "hl" => {
                    call = call.hl(value.unwrap_or(""));
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
                                                                           v.extend(["hl"].iter().map(|v|*v));
                                                                           v } ));
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

    async fn _i18n_regions_list(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        let mut call = self.hub.i18n_regions().list(&opt.values_of("part").map(|i|i.collect()).unwrap_or(Vec::new()).iter().map(|&v| v.to_string()).collect::<Vec<String>>());
        for parg in opt.values_of("v").map(|i|i.collect()).unwrap_or(Vec::new()).iter() {
            let (key, value) = parse_kv_arg(&*parg, err, false);
            match key {
                "hl" => {
                    call = call.hl(value.unwrap_or(""));
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
                                                                           v.extend(["hl"].iter().map(|v|*v));
                                                                           v } ));
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

    async fn _live_broadcasts_bind(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        let mut call = self.hub.live_broadcasts().bind(opt.value_of("id").unwrap_or(""), &opt.values_of("part").map(|i|i.collect()).unwrap_or(Vec::new()).iter().map(|&v| v.to_string()).collect::<Vec<String>>());
        for parg in opt.values_of("v").map(|i|i.collect()).unwrap_or(Vec::new()).iter() {
            let (key, value) = parse_kv_arg(&*parg, err, false);
            match key {
                "stream-id" => {
                    call = call.stream_id(value.unwrap_or(""));
                },
                "on-behalf-of-content-owner-channel" => {
                    call = call.on_behalf_of_content_owner_channel(value.unwrap_or(""));
                },
                "on-behalf-of-content-owner" => {
                    call = call.on_behalf_of_content_owner(value.unwrap_or(""));
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
                                                                           v.extend(["on-behalf-of-content-owner", "on-behalf-of-content-owner-channel", "stream-id"].iter().map(|v|*v));
                                                                           v } ));
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

    async fn _live_broadcasts_delete(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        let mut call = self.hub.live_broadcasts().delete(opt.value_of("id").unwrap_or(""));
        for parg in opt.values_of("v").map(|i|i.collect()).unwrap_or(Vec::new()).iter() {
            let (key, value) = parse_kv_arg(&*parg, err, false);
            match key {
                "on-behalf-of-content-owner-channel" => {
                    call = call.on_behalf_of_content_owner_channel(value.unwrap_or(""));
                },
                "on-behalf-of-content-owner" => {
                    call = call.on_behalf_of_content_owner(value.unwrap_or(""));
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
                                                                           v.extend(["on-behalf-of-content-owner", "on-behalf-of-content-owner-channel"].iter().map(|v|*v));
                                                                           v } ));
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
            match match protocol {
                CallType::Standard => call.doit().await,
                _ => unreachable!()
            } {
                Err(api_err) => Err(DoitError::ApiError(api_err)),
                Ok(mut response) => {
                    Ok(())
                }
            }
        }
    }

    async fn _live_broadcasts_insert(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
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
                    "content-details.bound-stream-id" => Some(("contentDetails.boundStreamId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "content-details.bound-stream-last-update-time-ms" => Some(("contentDetails.boundStreamLastUpdateTimeMs", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "content-details.closed-captions-type" => Some(("contentDetails.closedCaptionsType", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "content-details.enable-auto-start" => Some(("contentDetails.enableAutoStart", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "content-details.enable-auto-stop" => Some(("contentDetails.enableAutoStop", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "content-details.enable-closed-captions" => Some(("contentDetails.enableClosedCaptions", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "content-details.enable-content-encryption" => Some(("contentDetails.enableContentEncryption", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "content-details.enable-dvr" => Some(("contentDetails.enableDvr", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "content-details.enable-embed" => Some(("contentDetails.enableEmbed", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "content-details.enable-low-latency" => Some(("contentDetails.enableLowLatency", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "content-details.latency-preference" => Some(("contentDetails.latencyPreference", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "content-details.mesh" => Some(("contentDetails.mesh", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "content-details.monitor-stream.broadcast-stream-delay-ms" => Some(("contentDetails.monitorStream.broadcastStreamDelayMs", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "content-details.monitor-stream.embed-html" => Some(("contentDetails.monitorStream.embedHtml", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "content-details.monitor-stream.enable-monitor-stream" => Some(("contentDetails.monitorStream.enableMonitorStream", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "content-details.projection" => Some(("contentDetails.projection", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "content-details.record-from-start" => Some(("contentDetails.recordFromStart", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "content-details.start-with-slate" => Some(("contentDetails.startWithSlate", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "content-details.stereo-layout" => Some(("contentDetails.stereoLayout", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "etag" => Some(("etag", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "id" => Some(("id", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "kind" => Some(("kind", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "monetization-details.cuepoint-schedule.enabled" => Some(("monetizationDetails.cuepointSchedule.enabled", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "monetization-details.cuepoint-schedule.pause-ads-until" => Some(("monetizationDetails.cuepointSchedule.pauseAdsUntil", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "monetization-details.cuepoint-schedule.repeat-interval-secs" => Some(("monetizationDetails.cuepointSchedule.repeatIntervalSecs", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "monetization-details.cuepoint-schedule.schedule-strategy" => Some(("monetizationDetails.cuepointSchedule.scheduleStrategy", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "snippet.actual-end-time" => Some(("snippet.actualEndTime", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "snippet.actual-start-time" => Some(("snippet.actualStartTime", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "snippet.channel-id" => Some(("snippet.channelId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "snippet.description" => Some(("snippet.description", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "snippet.is-default-broadcast" => Some(("snippet.isDefaultBroadcast", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "snippet.live-chat-id" => Some(("snippet.liveChatId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "snippet.published-at" => Some(("snippet.publishedAt", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "snippet.scheduled-end-time" => Some(("snippet.scheduledEndTime", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "snippet.scheduled-start-time" => Some(("snippet.scheduledStartTime", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "snippet.thumbnails.default.height" => Some(("snippet.thumbnails.default.height", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "snippet.thumbnails.default.url" => Some(("snippet.thumbnails.default.url", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "snippet.thumbnails.default.width" => Some(("snippet.thumbnails.default.width", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "snippet.thumbnails.high.height" => Some(("snippet.thumbnails.high.height", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "snippet.thumbnails.high.url" => Some(("snippet.thumbnails.high.url", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "snippet.thumbnails.high.width" => Some(("snippet.thumbnails.high.width", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "snippet.thumbnails.maxres.height" => Some(("snippet.thumbnails.maxres.height", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "snippet.thumbnails.maxres.url" => Some(("snippet.thumbnails.maxres.url", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "snippet.thumbnails.maxres.width" => Some(("snippet.thumbnails.maxres.width", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "snippet.thumbnails.medium.height" => Some(("snippet.thumbnails.medium.height", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "snippet.thumbnails.medium.url" => Some(("snippet.thumbnails.medium.url", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "snippet.thumbnails.medium.width" => Some(("snippet.thumbnails.medium.width", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "snippet.thumbnails.standard.height" => Some(("snippet.thumbnails.standard.height", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "snippet.thumbnails.standard.url" => Some(("snippet.thumbnails.standard.url", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "snippet.thumbnails.standard.width" => Some(("snippet.thumbnails.standard.width", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "snippet.title" => Some(("snippet.title", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "statistics.concurrent-viewers" => Some(("statistics.concurrentViewers", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "status.life-cycle-status" => Some(("status.lifeCycleStatus", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "status.live-broadcast-priority" => Some(("status.liveBroadcastPriority", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "status.made-for-kids" => Some(("status.madeForKids", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "status.privacy-status" => Some(("status.privacyStatus", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "status.recording-status" => Some(("status.recordingStatus", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "status.self-declared-made-for-kids" => Some(("status.selfDeclaredMadeForKids", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    _ => {
                        let suggestion = FieldCursor::did_you_mean(key, &vec!["actual-end-time", "actual-start-time", "bound-stream-id", "bound-stream-last-update-time-ms", "broadcast-stream-delay-ms", "channel-id", "closed-captions-type", "concurrent-viewers", "content-details", "cuepoint-schedule", "default", "description", "embed-html", "enable-auto-start", "enable-auto-stop", "enable-closed-captions", "enable-content-encryption", "enable-dvr", "enable-embed", "enable-low-latency", "enable-monitor-stream", "enabled", "etag", "height", "high", "id", "is-default-broadcast", "kind", "latency-preference", "life-cycle-status", "live-broadcast-priority", "live-chat-id", "made-for-kids", "maxres", "medium", "mesh", "monetization-details", "monitor-stream", "pause-ads-until", "privacy-status", "projection", "published-at", "record-from-start", "recording-status", "repeat-interval-secs", "schedule-strategy", "scheduled-end-time", "scheduled-start-time", "self-declared-made-for-kids", "snippet", "standard", "start-with-slate", "statistics", "status", "stereo-layout", "thumbnails", "title", "url", "width"]);
                        err.issues.push(CLIError::Field(FieldError::Unknown(temp_cursor.to_string(), suggestion, value.map(|v| v.to_string()))));
                        None
                    }
                };
            if let Some((field_cursor_str, type_info)) = type_info {
                FieldCursor::from(field_cursor_str).set_json_value(&mut object, value.unwrap(), type_info, err, &temp_cursor);
            }
        }
        let mut request: api::LiveBroadcast = json::value::from_value(object).unwrap();
        let mut call = self.hub.live_broadcasts().insert(request);
        for parg in opt.values_of("v").map(|i|i.collect()).unwrap_or(Vec::new()).iter() {
            let (key, value) = parse_kv_arg(&*parg, err, false);
            match key {
                "on-behalf-of-content-owner-channel" => {
                    call = call.on_behalf_of_content_owner_channel(value.unwrap_or(""));
                },
                "on-behalf-of-content-owner" => {
                    call = call.on_behalf_of_content_owner(value.unwrap_or(""));
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
                                                                           v.extend(["on-behalf-of-content-owner", "on-behalf-of-content-owner-channel"].iter().map(|v|*v));
                                                                           v } ));
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

    async fn _live_broadcasts_insert_cuepoint(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
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
                    "cue-type" => Some(("cueType", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "duration-secs" => Some(("durationSecs", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "etag" => Some(("etag", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "id" => Some(("id", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "insertion-offset-time-ms" => Some(("insertionOffsetTimeMs", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "walltime-ms" => Some(("walltimeMs", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    _ => {
                        let suggestion = FieldCursor::did_you_mean(key, &vec!["cue-type", "duration-secs", "etag", "id", "insertion-offset-time-ms", "walltime-ms"]);
                        err.issues.push(CLIError::Field(FieldError::Unknown(temp_cursor.to_string(), suggestion, value.map(|v| v.to_string()))));
                        None
                    }
                };
            if let Some((field_cursor_str, type_info)) = type_info {
                FieldCursor::from(field_cursor_str).set_json_value(&mut object, value.unwrap(), type_info, err, &temp_cursor);
            }
        }
        let mut request: api::Cuepoint = json::value::from_value(object).unwrap();
        let mut call = self.hub.live_broadcasts().insert_cuepoint(request);
        for parg in opt.values_of("v").map(|i|i.collect()).unwrap_or(Vec::new()).iter() {
            let (key, value) = parse_kv_arg(&*parg, err, false);
            match key {
                "part" => {
                    call = call.add_part(value.unwrap_or(""));
                },
                "on-behalf-of-content-owner-channel" => {
                    call = call.on_behalf_of_content_owner_channel(value.unwrap_or(""));
                },
                "on-behalf-of-content-owner" => {
                    call = call.on_behalf_of_content_owner(value.unwrap_or(""));
                },
                "id" => {
                    call = call.id(value.unwrap_or(""));
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
                                                                           v.extend(["id", "on-behalf-of-content-owner", "on-behalf-of-content-owner-channel", "part"].iter().map(|v|*v));
                                                                           v } ));
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

    async fn _live_broadcasts_list(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        let mut call = self.hub.live_broadcasts().list(&opt.values_of("part").map(|i|i.collect()).unwrap_or(Vec::new()).iter().map(|&v| v.to_string()).collect::<Vec<String>>());
        for parg in opt.values_of("v").map(|i|i.collect()).unwrap_or(Vec::new()).iter() {
            let (key, value) = parse_kv_arg(&*parg, err, false);
            match key {
                "page-token" => {
                    call = call.page_token(value.unwrap_or(""));
                },
                "on-behalf-of-content-owner-channel" => {
                    call = call.on_behalf_of_content_owner_channel(value.unwrap_or(""));
                },
                "on-behalf-of-content-owner" => {
                    call = call.on_behalf_of_content_owner(value.unwrap_or(""));
                },
                "mine" => {
                    call = call.mine(        value.map(|v| arg_from_str(v, err, "mine", "boolean")).unwrap_or(false));
                },
                "max-results" => {
                    call = call.max_results(        value.map(|v| arg_from_str(v, err, "max-results", "uint32")).unwrap_or(0));
                },
                "id" => {
                    call = call.add_id(value.unwrap_or(""));
                },
                "broadcast-type" => {
                    call = call.broadcast_type(value.unwrap_or(""));
                },
                "broadcast-status" => {
                    call = call.broadcast_status(value.unwrap_or(""));
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
                                                                           v.extend(["broadcast-status", "broadcast-type", "id", "max-results", "mine", "on-behalf-of-content-owner", "on-behalf-of-content-owner-channel", "page-token"].iter().map(|v|*v));
                                                                           v } ));
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

    async fn _live_broadcasts_transition(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        let mut call = self.hub.live_broadcasts().transition(opt.value_of("broadcast-status").unwrap_or(""), opt.value_of("id").unwrap_or(""), &opt.values_of("part").map(|i|i.collect()).unwrap_or(Vec::new()).iter().map(|&v| v.to_string()).collect::<Vec<String>>());
        for parg in opt.values_of("v").map(|i|i.collect()).unwrap_or(Vec::new()).iter() {
            let (key, value) = parse_kv_arg(&*parg, err, false);
            match key {
                "on-behalf-of-content-owner-channel" => {
                    call = call.on_behalf_of_content_owner_channel(value.unwrap_or(""));
                },
                "on-behalf-of-content-owner" => {
                    call = call.on_behalf_of_content_owner(value.unwrap_or(""));
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
                                                                           v.extend(["on-behalf-of-content-owner", "on-behalf-of-content-owner-channel"].iter().map(|v|*v));
                                                                           v } ));
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

    async fn _live_broadcasts_update(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
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
                    "content-details.bound-stream-id" => Some(("contentDetails.boundStreamId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "content-details.bound-stream-last-update-time-ms" => Some(("contentDetails.boundStreamLastUpdateTimeMs", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "content-details.closed-captions-type" => Some(("contentDetails.closedCaptionsType", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "content-details.enable-auto-start" => Some(("contentDetails.enableAutoStart", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "content-details.enable-auto-stop" => Some(("contentDetails.enableAutoStop", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "content-details.enable-closed-captions" => Some(("contentDetails.enableClosedCaptions", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "content-details.enable-content-encryption" => Some(("contentDetails.enableContentEncryption", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "content-details.enable-dvr" => Some(("contentDetails.enableDvr", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "content-details.enable-embed" => Some(("contentDetails.enableEmbed", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "content-details.enable-low-latency" => Some(("contentDetails.enableLowLatency", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "content-details.latency-preference" => Some(("contentDetails.latencyPreference", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "content-details.mesh" => Some(("contentDetails.mesh", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "content-details.monitor-stream.broadcast-stream-delay-ms" => Some(("contentDetails.monitorStream.broadcastStreamDelayMs", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "content-details.monitor-stream.embed-html" => Some(("contentDetails.monitorStream.embedHtml", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "content-details.monitor-stream.enable-monitor-stream" => Some(("contentDetails.monitorStream.enableMonitorStream", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "content-details.projection" => Some(("contentDetails.projection", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "content-details.record-from-start" => Some(("contentDetails.recordFromStart", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "content-details.start-with-slate" => Some(("contentDetails.startWithSlate", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "content-details.stereo-layout" => Some(("contentDetails.stereoLayout", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "etag" => Some(("etag", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "id" => Some(("id", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "kind" => Some(("kind", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "monetization-details.cuepoint-schedule.enabled" => Some(("monetizationDetails.cuepointSchedule.enabled", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "monetization-details.cuepoint-schedule.pause-ads-until" => Some(("monetizationDetails.cuepointSchedule.pauseAdsUntil", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "monetization-details.cuepoint-schedule.repeat-interval-secs" => Some(("monetizationDetails.cuepointSchedule.repeatIntervalSecs", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "monetization-details.cuepoint-schedule.schedule-strategy" => Some(("monetizationDetails.cuepointSchedule.scheduleStrategy", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "snippet.actual-end-time" => Some(("snippet.actualEndTime", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "snippet.actual-start-time" => Some(("snippet.actualStartTime", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "snippet.channel-id" => Some(("snippet.channelId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "snippet.description" => Some(("snippet.description", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "snippet.is-default-broadcast" => Some(("snippet.isDefaultBroadcast", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "snippet.live-chat-id" => Some(("snippet.liveChatId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "snippet.published-at" => Some(("snippet.publishedAt", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "snippet.scheduled-end-time" => Some(("snippet.scheduledEndTime", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "snippet.scheduled-start-time" => Some(("snippet.scheduledStartTime", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "snippet.thumbnails.default.height" => Some(("snippet.thumbnails.default.height", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "snippet.thumbnails.default.url" => Some(("snippet.thumbnails.default.url", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "snippet.thumbnails.default.width" => Some(("snippet.thumbnails.default.width", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "snippet.thumbnails.high.height" => Some(("snippet.thumbnails.high.height", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "snippet.thumbnails.high.url" => Some(("snippet.thumbnails.high.url", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "snippet.thumbnails.high.width" => Some(("snippet.thumbnails.high.width", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "snippet.thumbnails.maxres.height" => Some(("snippet.thumbnails.maxres.height", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "snippet.thumbnails.maxres.url" => Some(("snippet.thumbnails.maxres.url", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "snippet.thumbnails.maxres.width" => Some(("snippet.thumbnails.maxres.width", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "snippet.thumbnails.medium.height" => Some(("snippet.thumbnails.medium.height", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "snippet.thumbnails.medium.url" => Some(("snippet.thumbnails.medium.url", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "snippet.thumbnails.medium.width" => Some(("snippet.thumbnails.medium.width", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "snippet.thumbnails.standard.height" => Some(("snippet.thumbnails.standard.height", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "snippet.thumbnails.standard.url" => Some(("snippet.thumbnails.standard.url", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "snippet.thumbnails.standard.width" => Some(("snippet.thumbnails.standard.width", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "snippet.title" => Some(("snippet.title", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "statistics.concurrent-viewers" => Some(("statistics.concurrentViewers", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "status.life-cycle-status" => Some(("status.lifeCycleStatus", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "status.live-broadcast-priority" => Some(("status.liveBroadcastPriority", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "status.made-for-kids" => Some(("status.madeForKids", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "status.privacy-status" => Some(("status.privacyStatus", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "status.recording-status" => Some(("status.recordingStatus", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "status.self-declared-made-for-kids" => Some(("status.selfDeclaredMadeForKids", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    _ => {
                        let suggestion = FieldCursor::did_you_mean(key, &vec!["actual-end-time", "actual-start-time", "bound-stream-id", "bound-stream-last-update-time-ms", "broadcast-stream-delay-ms", "channel-id", "closed-captions-type", "concurrent-viewers", "content-details", "cuepoint-schedule", "default", "description", "embed-html", "enable-auto-start", "enable-auto-stop", "enable-closed-captions", "enable-content-encryption", "enable-dvr", "enable-embed", "enable-low-latency", "enable-monitor-stream", "enabled", "etag", "height", "high", "id", "is-default-broadcast", "kind", "latency-preference", "life-cycle-status", "live-broadcast-priority", "live-chat-id", "made-for-kids", "maxres", "medium", "mesh", "monetization-details", "monitor-stream", "pause-ads-until", "privacy-status", "projection", "published-at", "record-from-start", "recording-status", "repeat-interval-secs", "schedule-strategy", "scheduled-end-time", "scheduled-start-time", "self-declared-made-for-kids", "snippet", "standard", "start-with-slate", "statistics", "status", "stereo-layout", "thumbnails", "title", "url", "width"]);
                        err.issues.push(CLIError::Field(FieldError::Unknown(temp_cursor.to_string(), suggestion, value.map(|v| v.to_string()))));
                        None
                    }
                };
            if let Some((field_cursor_str, type_info)) = type_info {
                FieldCursor::from(field_cursor_str).set_json_value(&mut object, value.unwrap(), type_info, err, &temp_cursor);
            }
        }
        let mut request: api::LiveBroadcast = json::value::from_value(object).unwrap();
        let mut call = self.hub.live_broadcasts().update(request);
        for parg in opt.values_of("v").map(|i|i.collect()).unwrap_or(Vec::new()).iter() {
            let (key, value) = parse_kv_arg(&*parg, err, false);
            match key {
                "on-behalf-of-content-owner-channel" => {
                    call = call.on_behalf_of_content_owner_channel(value.unwrap_or(""));
                },
                "on-behalf-of-content-owner" => {
                    call = call.on_behalf_of_content_owner(value.unwrap_or(""));
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
                                                                           v.extend(["on-behalf-of-content-owner", "on-behalf-of-content-owner-channel"].iter().map(|v|*v));
                                                                           v } ));
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

    async fn _live_chat_bans_delete(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        let mut call = self.hub.live_chat_bans().delete(opt.value_of("id").unwrap_or(""));
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
            match match protocol {
                CallType::Standard => call.doit().await,
                _ => unreachable!()
            } {
                Err(api_err) => Err(DoitError::ApiError(api_err)),
                Ok(mut response) => {
                    Ok(())
                }
            }
        }
    }

    async fn _live_chat_bans_insert(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
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
                    "etag" => Some(("etag", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "id" => Some(("id", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "kind" => Some(("kind", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "snippet.ban-duration-seconds" => Some(("snippet.banDurationSeconds", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "snippet.banned-user-details.channel-id" => Some(("snippet.bannedUserDetails.channelId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "snippet.banned-user-details.channel-url" => Some(("snippet.bannedUserDetails.channelUrl", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "snippet.banned-user-details.display-name" => Some(("snippet.bannedUserDetails.displayName", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "snippet.banned-user-details.profile-image-url" => Some(("snippet.bannedUserDetails.profileImageUrl", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "snippet.live-chat-id" => Some(("snippet.liveChatId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "snippet.type" => Some(("snippet.type", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    _ => {
                        let suggestion = FieldCursor::did_you_mean(key, &vec!["ban-duration-seconds", "banned-user-details", "channel-id", "channel-url", "display-name", "etag", "id", "kind", "live-chat-id", "profile-image-url", "snippet", "type"]);
                        err.issues.push(CLIError::Field(FieldError::Unknown(temp_cursor.to_string(), suggestion, value.map(|v| v.to_string()))));
                        None
                    }
                };
            if let Some((field_cursor_str, type_info)) = type_info {
                FieldCursor::from(field_cursor_str).set_json_value(&mut object, value.unwrap(), type_info, err, &temp_cursor);
            }
        }
        let mut request: api::LiveChatBan = json::value::from_value(object).unwrap();
        let mut call = self.hub.live_chat_bans().insert(request);
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

    async fn _live_chat_messages_delete(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        let mut call = self.hub.live_chat_messages().delete(opt.value_of("id").unwrap_or(""));
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
            match match protocol {
                CallType::Standard => call.doit().await,
                _ => unreachable!()
            } {
                Err(api_err) => Err(DoitError::ApiError(api_err)),
                Ok(mut response) => {
                    Ok(())
                }
            }
        }
    }

    async fn _live_chat_messages_insert(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
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
                    "author-details.channel-id" => Some(("authorDetails.channelId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "author-details.channel-url" => Some(("authorDetails.channelUrl", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "author-details.display-name" => Some(("authorDetails.displayName", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "author-details.is-chat-moderator" => Some(("authorDetails.isChatModerator", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "author-details.is-chat-owner" => Some(("authorDetails.isChatOwner", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "author-details.is-chat-sponsor" => Some(("authorDetails.isChatSponsor", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "author-details.is-verified" => Some(("authorDetails.isVerified", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "author-details.profile-image-url" => Some(("authorDetails.profileImageUrl", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "etag" => Some(("etag", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "id" => Some(("id", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "kind" => Some(("kind", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "snippet.author-channel-id" => Some(("snippet.authorChannelId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "snippet.display-message" => Some(("snippet.displayMessage", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "snippet.fan-funding-event-details.amount-display-string" => Some(("snippet.fanFundingEventDetails.amountDisplayString", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "snippet.fan-funding-event-details.amount-micros" => Some(("snippet.fanFundingEventDetails.amountMicros", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "snippet.fan-funding-event-details.currency" => Some(("snippet.fanFundingEventDetails.currency", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "snippet.fan-funding-event-details.user-comment" => Some(("snippet.fanFundingEventDetails.userComment", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "snippet.gift-membership-received-details.associated-membership-gifting-message-id" => Some(("snippet.giftMembershipReceivedDetails.associatedMembershipGiftingMessageId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "snippet.gift-membership-received-details.gifter-channel-id" => Some(("snippet.giftMembershipReceivedDetails.gifterChannelId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "snippet.gift-membership-received-details.member-level-name" => Some(("snippet.giftMembershipReceivedDetails.memberLevelName", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "snippet.has-display-content" => Some(("snippet.hasDisplayContent", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "snippet.live-chat-id" => Some(("snippet.liveChatId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "snippet.member-milestone-chat-details.member-level-name" => Some(("snippet.memberMilestoneChatDetails.memberLevelName", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "snippet.member-milestone-chat-details.member-month" => Some(("snippet.memberMilestoneChatDetails.memberMonth", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "snippet.member-milestone-chat-details.user-comment" => Some(("snippet.memberMilestoneChatDetails.userComment", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "snippet.membership-gifting-details.gift-memberships-count" => Some(("snippet.membershipGiftingDetails.giftMembershipsCount", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "snippet.membership-gifting-details.gift-memberships-level-name" => Some(("snippet.membershipGiftingDetails.giftMembershipsLevelName", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "snippet.message-deleted-details.deleted-message-id" => Some(("snippet.messageDeletedDetails.deletedMessageId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "snippet.message-retracted-details.retracted-message-id" => Some(("snippet.messageRetractedDetails.retractedMessageId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "snippet.new-sponsor-details.is-upgrade" => Some(("snippet.newSponsorDetails.isUpgrade", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "snippet.new-sponsor-details.member-level-name" => Some(("snippet.newSponsorDetails.memberLevelName", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "snippet.poll-details.metadata.question-text" => Some(("snippet.pollDetails.metadata.questionText", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "snippet.poll-details.status" => Some(("snippet.pollDetails.status", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "snippet.published-at" => Some(("snippet.publishedAt", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "snippet.super-chat-details.amount-display-string" => Some(("snippet.superChatDetails.amountDisplayString", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "snippet.super-chat-details.amount-micros" => Some(("snippet.superChatDetails.amountMicros", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "snippet.super-chat-details.currency" => Some(("snippet.superChatDetails.currency", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "snippet.super-chat-details.tier" => Some(("snippet.superChatDetails.tier", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "snippet.super-chat-details.user-comment" => Some(("snippet.superChatDetails.userComment", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "snippet.super-sticker-details.amount-display-string" => Some(("snippet.superStickerDetails.amountDisplayString", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "snippet.super-sticker-details.amount-micros" => Some(("snippet.superStickerDetails.amountMicros", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "snippet.super-sticker-details.currency" => Some(("snippet.superStickerDetails.currency", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "snippet.super-sticker-details.super-sticker-metadata.alt-text" => Some(("snippet.superStickerDetails.superStickerMetadata.altText", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "snippet.super-sticker-details.super-sticker-metadata.alt-text-language" => Some(("snippet.superStickerDetails.superStickerMetadata.altTextLanguage", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "snippet.super-sticker-details.super-sticker-metadata.sticker-id" => Some(("snippet.superStickerDetails.superStickerMetadata.stickerId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "snippet.super-sticker-details.tier" => Some(("snippet.superStickerDetails.tier", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "snippet.text-message-details.message-text" => Some(("snippet.textMessageDetails.messageText", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "snippet.type" => Some(("snippet.type", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "snippet.user-banned-details.ban-duration-seconds" => Some(("snippet.userBannedDetails.banDurationSeconds", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "snippet.user-banned-details.ban-type" => Some(("snippet.userBannedDetails.banType", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "snippet.user-banned-details.banned-user-details.channel-id" => Some(("snippet.userBannedDetails.bannedUserDetails.channelId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "snippet.user-banned-details.banned-user-details.channel-url" => Some(("snippet.userBannedDetails.bannedUserDetails.channelUrl", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "snippet.user-banned-details.banned-user-details.display-name" => Some(("snippet.userBannedDetails.bannedUserDetails.displayName", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "snippet.user-banned-details.banned-user-details.profile-image-url" => Some(("snippet.userBannedDetails.bannedUserDetails.profileImageUrl", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    _ => {
                        let suggestion = FieldCursor::did_you_mean(key, &vec!["alt-text", "alt-text-language", "amount-display-string", "amount-micros", "associated-membership-gifting-message-id", "author-channel-id", "author-details", "ban-duration-seconds", "ban-type", "banned-user-details", "channel-id", "channel-url", "currency", "deleted-message-id", "display-message", "display-name", "etag", "fan-funding-event-details", "gift-membership-received-details", "gift-memberships-count", "gift-memberships-level-name", "gifter-channel-id", "has-display-content", "id", "is-chat-moderator", "is-chat-owner", "is-chat-sponsor", "is-upgrade", "is-verified", "kind", "live-chat-id", "member-level-name", "member-milestone-chat-details", "member-month", "membership-gifting-details", "message-deleted-details", "message-retracted-details", "message-text", "metadata", "new-sponsor-details", "poll-details", "profile-image-url", "published-at", "question-text", "retracted-message-id", "snippet", "status", "sticker-id", "super-chat-details", "super-sticker-details", "super-sticker-metadata", "text-message-details", "tier", "type", "user-banned-details", "user-comment"]);
                        err.issues.push(CLIError::Field(FieldError::Unknown(temp_cursor.to_string(), suggestion, value.map(|v| v.to_string()))));
                        None
                    }
                };
            if let Some((field_cursor_str, type_info)) = type_info {
                FieldCursor::from(field_cursor_str).set_json_value(&mut object, value.unwrap(), type_info, err, &temp_cursor);
            }
        }
        let mut request: api::LiveChatMessage = json::value::from_value(object).unwrap();
        let mut call = self.hub.live_chat_messages().insert(request);
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

    async fn _live_chat_messages_list(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        let mut call = self.hub.live_chat_messages().list(opt.value_of("live-chat-id").unwrap_or(""), &opt.values_of("part").map(|i|i.collect()).unwrap_or(Vec::new()).iter().map(|&v| v.to_string()).collect::<Vec<String>>());
        for parg in opt.values_of("v").map(|i|i.collect()).unwrap_or(Vec::new()).iter() {
            let (key, value) = parse_kv_arg(&*parg, err, false);
            match key {
                "profile-image-size" => {
                    call = call.profile_image_size(        value.map(|v| arg_from_str(v, err, "profile-image-size", "uint32")).unwrap_or(0));
                },
                "page-token" => {
                    call = call.page_token(value.unwrap_or(""));
                },
                "max-results" => {
                    call = call.max_results(        value.map(|v| arg_from_str(v, err, "max-results", "uint32")).unwrap_or(0));
                },
                "hl" => {
                    call = call.hl(value.unwrap_or(""));
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
                                                                           v.extend(["hl", "max-results", "page-token", "profile-image-size"].iter().map(|v|*v));
                                                                           v } ));
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

    async fn _live_chat_messages_transition(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        let mut call = self.hub.live_chat_messages().transition();
        for parg in opt.values_of("v").map(|i|i.collect()).unwrap_or(Vec::new()).iter() {
            let (key, value) = parse_kv_arg(&*parg, err, false);
            match key {
                "status" => {
                    call = call.status(value.unwrap_or(""));
                },
                "id" => {
                    call = call.id(value.unwrap_or(""));
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
                                                                           v.extend(["id", "status"].iter().map(|v|*v));
                                                                           v } ));
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

    async fn _live_chat_moderators_delete(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        let mut call = self.hub.live_chat_moderators().delete(opt.value_of("id").unwrap_or(""));
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
            match match protocol {
                CallType::Standard => call.doit().await,
                _ => unreachable!()
            } {
                Err(api_err) => Err(DoitError::ApiError(api_err)),
                Ok(mut response) => {
                    Ok(())
                }
            }
        }
    }

    async fn _live_chat_moderators_insert(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
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
                    "etag" => Some(("etag", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "id" => Some(("id", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "kind" => Some(("kind", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "snippet.live-chat-id" => Some(("snippet.liveChatId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "snippet.moderator-details.channel-id" => Some(("snippet.moderatorDetails.channelId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "snippet.moderator-details.channel-url" => Some(("snippet.moderatorDetails.channelUrl", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "snippet.moderator-details.display-name" => Some(("snippet.moderatorDetails.displayName", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "snippet.moderator-details.profile-image-url" => Some(("snippet.moderatorDetails.profileImageUrl", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    _ => {
                        let suggestion = FieldCursor::did_you_mean(key, &vec!["channel-id", "channel-url", "display-name", "etag", "id", "kind", "live-chat-id", "moderator-details", "profile-image-url", "snippet"]);
                        err.issues.push(CLIError::Field(FieldError::Unknown(temp_cursor.to_string(), suggestion, value.map(|v| v.to_string()))));
                        None
                    }
                };
            if let Some((field_cursor_str, type_info)) = type_info {
                FieldCursor::from(field_cursor_str).set_json_value(&mut object, value.unwrap(), type_info, err, &temp_cursor);
            }
        }
        let mut request: api::LiveChatModerator = json::value::from_value(object).unwrap();
        let mut call = self.hub.live_chat_moderators().insert(request);
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

    async fn _live_chat_moderators_list(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        let mut call = self.hub.live_chat_moderators().list(opt.value_of("live-chat-id").unwrap_or(""), &opt.values_of("part").map(|i|i.collect()).unwrap_or(Vec::new()).iter().map(|&v| v.to_string()).collect::<Vec<String>>());
        for parg in opt.values_of("v").map(|i|i.collect()).unwrap_or(Vec::new()).iter() {
            let (key, value) = parse_kv_arg(&*parg, err, false);
            match key {
                "page-token" => {
                    call = call.page_token(value.unwrap_or(""));
                },
                "max-results" => {
                    call = call.max_results(        value.map(|v| arg_from_str(v, err, "max-results", "uint32")).unwrap_or(0));
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
                                                                           v.extend(["max-results", "page-token"].iter().map(|v|*v));
                                                                           v } ));
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

    async fn _live_streams_delete(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        let mut call = self.hub.live_streams().delete(opt.value_of("id").unwrap_or(""));
        for parg in opt.values_of("v").map(|i|i.collect()).unwrap_or(Vec::new()).iter() {
            let (key, value) = parse_kv_arg(&*parg, err, false);
            match key {
                "on-behalf-of-content-owner-channel" => {
                    call = call.on_behalf_of_content_owner_channel(value.unwrap_or(""));
                },
                "on-behalf-of-content-owner" => {
                    call = call.on_behalf_of_content_owner(value.unwrap_or(""));
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
                                                                           v.extend(["on-behalf-of-content-owner", "on-behalf-of-content-owner-channel"].iter().map(|v|*v));
                                                                           v } ));
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
            match match protocol {
                CallType::Standard => call.doit().await,
                _ => unreachable!()
            } {
                Err(api_err) => Err(DoitError::ApiError(api_err)),
                Ok(mut response) => {
                    Ok(())
                }
            }
        }
    }

    async fn _live_streams_insert(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
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
                    "cdn.format" => Some(("cdn.format", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "cdn.frame-rate" => Some(("cdn.frameRate", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "cdn.ingestion-info.backup-ingestion-address" => Some(("cdn.ingestionInfo.backupIngestionAddress", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "cdn.ingestion-info.ingestion-address" => Some(("cdn.ingestionInfo.ingestionAddress", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "cdn.ingestion-info.rtmps-backup-ingestion-address" => Some(("cdn.ingestionInfo.rtmpsBackupIngestionAddress", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "cdn.ingestion-info.rtmps-ingestion-address" => Some(("cdn.ingestionInfo.rtmpsIngestionAddress", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "cdn.ingestion-info.stream-name" => Some(("cdn.ingestionInfo.streamName", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "cdn.ingestion-type" => Some(("cdn.ingestionType", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "cdn.resolution" => Some(("cdn.resolution", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "content-details.closed-captions-ingestion-url" => Some(("contentDetails.closedCaptionsIngestionUrl", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "content-details.is-reusable" => Some(("contentDetails.isReusable", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "etag" => Some(("etag", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "id" => Some(("id", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "kind" => Some(("kind", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "snippet.channel-id" => Some(("snippet.channelId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "snippet.description" => Some(("snippet.description", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "snippet.is-default-stream" => Some(("snippet.isDefaultStream", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "snippet.published-at" => Some(("snippet.publishedAt", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "snippet.title" => Some(("snippet.title", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "status.health-status.last-update-time-seconds" => Some(("status.healthStatus.lastUpdateTimeSeconds", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "status.health-status.status" => Some(("status.healthStatus.status", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "status.stream-status" => Some(("status.streamStatus", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    _ => {
                        let suggestion = FieldCursor::did_you_mean(key, &vec!["backup-ingestion-address", "cdn", "channel-id", "closed-captions-ingestion-url", "content-details", "description", "etag", "format", "frame-rate", "health-status", "id", "ingestion-address", "ingestion-info", "ingestion-type", "is-default-stream", "is-reusable", "kind", "last-update-time-seconds", "published-at", "resolution", "rtmps-backup-ingestion-address", "rtmps-ingestion-address", "snippet", "status", "stream-name", "stream-status", "title"]);
                        err.issues.push(CLIError::Field(FieldError::Unknown(temp_cursor.to_string(), suggestion, value.map(|v| v.to_string()))));
                        None
                    }
                };
            if let Some((field_cursor_str, type_info)) = type_info {
                FieldCursor::from(field_cursor_str).set_json_value(&mut object, value.unwrap(), type_info, err, &temp_cursor);
            }
        }
        let mut request: api::LiveStream = json::value::from_value(object).unwrap();
        let mut call = self.hub.live_streams().insert(request);
        for parg in opt.values_of("v").map(|i|i.collect()).unwrap_or(Vec::new()).iter() {
            let (key, value) = parse_kv_arg(&*parg, err, false);
            match key {
                "on-behalf-of-content-owner-channel" => {
                    call = call.on_behalf_of_content_owner_channel(value.unwrap_or(""));
                },
                "on-behalf-of-content-owner" => {
                    call = call.on_behalf_of_content_owner(value.unwrap_or(""));
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
                                                                           v.extend(["on-behalf-of-content-owner", "on-behalf-of-content-owner-channel"].iter().map(|v|*v));
                                                                           v } ));
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

    async fn _live_streams_list(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        let mut call = self.hub.live_streams().list(&opt.values_of("part").map(|i|i.collect()).unwrap_or(Vec::new()).iter().map(|&v| v.to_string()).collect::<Vec<String>>());
        for parg in opt.values_of("v").map(|i|i.collect()).unwrap_or(Vec::new()).iter() {
            let (key, value) = parse_kv_arg(&*parg, err, false);
            match key {
                "page-token" => {
                    call = call.page_token(value.unwrap_or(""));
                },
                "on-behalf-of-content-owner-channel" => {
                    call = call.on_behalf_of_content_owner_channel(value.unwrap_or(""));
                },
                "on-behalf-of-content-owner" => {
                    call = call.on_behalf_of_content_owner(value.unwrap_or(""));
                },
                "mine" => {
                    call = call.mine(        value.map(|v| arg_from_str(v, err, "mine", "boolean")).unwrap_or(false));
                },
                "max-results" => {
                    call = call.max_results(        value.map(|v| arg_from_str(v, err, "max-results", "uint32")).unwrap_or(0));
                },
                "id" => {
                    call = call.add_id(value.unwrap_or(""));
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
                                                                           v.extend(["id", "max-results", "mine", "on-behalf-of-content-owner", "on-behalf-of-content-owner-channel", "page-token"].iter().map(|v|*v));
                                                                           v } ));
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

    async fn _live_streams_update(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
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
                    "cdn.format" => Some(("cdn.format", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "cdn.frame-rate" => Some(("cdn.frameRate", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "cdn.ingestion-info.backup-ingestion-address" => Some(("cdn.ingestionInfo.backupIngestionAddress", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "cdn.ingestion-info.ingestion-address" => Some(("cdn.ingestionInfo.ingestionAddress", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "cdn.ingestion-info.rtmps-backup-ingestion-address" => Some(("cdn.ingestionInfo.rtmpsBackupIngestionAddress", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "cdn.ingestion-info.rtmps-ingestion-address" => Some(("cdn.ingestionInfo.rtmpsIngestionAddress", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "cdn.ingestion-info.stream-name" => Some(("cdn.ingestionInfo.streamName", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "cdn.ingestion-type" => Some(("cdn.ingestionType", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "cdn.resolution" => Some(("cdn.resolution", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "content-details.closed-captions-ingestion-url" => Some(("contentDetails.closedCaptionsIngestionUrl", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "content-details.is-reusable" => Some(("contentDetails.isReusable", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "etag" => Some(("etag", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "id" => Some(("id", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "kind" => Some(("kind", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "snippet.channel-id" => Some(("snippet.channelId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "snippet.description" => Some(("snippet.description", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "snippet.is-default-stream" => Some(("snippet.isDefaultStream", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "snippet.published-at" => Some(("snippet.publishedAt", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "snippet.title" => Some(("snippet.title", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "status.health-status.last-update-time-seconds" => Some(("status.healthStatus.lastUpdateTimeSeconds", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "status.health-status.status" => Some(("status.healthStatus.status", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "status.stream-status" => Some(("status.streamStatus", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    _ => {
                        let suggestion = FieldCursor::did_you_mean(key, &vec!["backup-ingestion-address", "cdn", "channel-id", "closed-captions-ingestion-url", "content-details", "description", "etag", "format", "frame-rate", "health-status", "id", "ingestion-address", "ingestion-info", "ingestion-type", "is-default-stream", "is-reusable", "kind", "last-update-time-seconds", "published-at", "resolution", "rtmps-backup-ingestion-address", "rtmps-ingestion-address", "snippet", "status", "stream-name", "stream-status", "title"]);
                        err.issues.push(CLIError::Field(FieldError::Unknown(temp_cursor.to_string(), suggestion, value.map(|v| v.to_string()))));
                        None
                    }
                };
            if let Some((field_cursor_str, type_info)) = type_info {
                FieldCursor::from(field_cursor_str).set_json_value(&mut object, value.unwrap(), type_info, err, &temp_cursor);
            }
        }
        let mut request: api::LiveStream = json::value::from_value(object).unwrap();
        let mut call = self.hub.live_streams().update(request);
        for parg in opt.values_of("v").map(|i|i.collect()).unwrap_or(Vec::new()).iter() {
            let (key, value) = parse_kv_arg(&*parg, err, false);
            match key {
                "on-behalf-of-content-owner-channel" => {
                    call = call.on_behalf_of_content_owner_channel(value.unwrap_or(""));
                },
                "on-behalf-of-content-owner" => {
                    call = call.on_behalf_of_content_owner(value.unwrap_or(""));
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
                                                                           v.extend(["on-behalf-of-content-owner", "on-behalf-of-content-owner-channel"].iter().map(|v|*v));
                                                                           v } ));
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

    async fn _members_list(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        let mut call = self.hub.members().list(&opt.values_of("part").map(|i|i.collect()).unwrap_or(Vec::new()).iter().map(|&v| v.to_string()).collect::<Vec<String>>());
        for parg in opt.values_of("v").map(|i|i.collect()).unwrap_or(Vec::new()).iter() {
            let (key, value) = parse_kv_arg(&*parg, err, false);
            match key {
                "page-token" => {
                    call = call.page_token(value.unwrap_or(""));
                },
                "mode" => {
                    call = call.mode(value.unwrap_or(""));
                },
                "max-results" => {
                    call = call.max_results(        value.map(|v| arg_from_str(v, err, "max-results", "uint32")).unwrap_or(0));
                },
                "has-access-to-level" => {
                    call = call.has_access_to_level(value.unwrap_or(""));
                },
                "filter-by-member-channel-id" => {
                    call = call.filter_by_member_channel_id(value.unwrap_or(""));
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
                                                                           v.extend(["filter-by-member-channel-id", "has-access-to-level", "max-results", "mode", "page-token"].iter().map(|v|*v));
                                                                           v } ));
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

    async fn _memberships_levels_list(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        let mut call = self.hub.memberships_levels().list(&opt.values_of("part").map(|i|i.collect()).unwrap_or(Vec::new()).iter().map(|&v| v.to_string()).collect::<Vec<String>>());
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

    async fn _playlist_images_delete(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        let mut call = self.hub.playlist_images().delete();
        for parg in opt.values_of("v").map(|i|i.collect()).unwrap_or(Vec::new()).iter() {
            let (key, value) = parse_kv_arg(&*parg, err, false);
            match key {
                "on-behalf-of-content-owner" => {
                    call = call.on_behalf_of_content_owner(value.unwrap_or(""));
                },
                "id" => {
                    call = call.id(value.unwrap_or(""));
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
                                                                           v.extend(["id", "on-behalf-of-content-owner"].iter().map(|v|*v));
                                                                           v } ));
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
            match match protocol {
                CallType::Standard => call.doit().await,
                _ => unreachable!()
            } {
                Err(api_err) => Err(DoitError::ApiError(api_err)),
                Ok(mut response) => {
                    Ok(())
                }
            }
        }
    }

    async fn _playlist_images_insert(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
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
                    "id" => Some(("id", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "kind" => Some(("kind", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "snippet.height" => Some(("snippet.height", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "snippet.playlist-id" => Some(("snippet.playlistId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "snippet.type" => Some(("snippet.type", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "snippet.width" => Some(("snippet.width", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    _ => {
                        let suggestion = FieldCursor::did_you_mean(key, &vec!["height", "id", "kind", "playlist-id", "snippet", "type", "width"]);
                        err.issues.push(CLIError::Field(FieldError::Unknown(temp_cursor.to_string(), suggestion, value.map(|v| v.to_string()))));
                        None
                    }
                };
            if let Some((field_cursor_str, type_info)) = type_info {
                FieldCursor::from(field_cursor_str).set_json_value(&mut object, value.unwrap(), type_info, err, &temp_cursor);
            }
        }
        let mut request: api::PlaylistImage = json::value::from_value(object).unwrap();
        let mut call = self.hub.playlist_images().insert(request);
        for parg in opt.values_of("v").map(|i|i.collect()).unwrap_or(Vec::new()).iter() {
            let (key, value) = parse_kv_arg(&*parg, err, false);
            match key {
                "part" => {
                    call = call.add_part(value.unwrap_or(""));
                },
                "on-behalf-of-content-owner-channel" => {
                    call = call.on_behalf_of_content_owner_channel(value.unwrap_or(""));
                },
                "on-behalf-of-content-owner" => {
                    call = call.on_behalf_of_content_owner(value.unwrap_or(""));
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
                                                                           v.extend(["on-behalf-of-content-owner", "on-behalf-of-content-owner-channel", "part"].iter().map(|v|*v));
                                                                           v } ));
                    }
                }
            }
        }
        let vals = opt.values_of("mode").unwrap().collect::<Vec<&str>>();
        let protocol = calltype_from_str(vals[0], ["simple"].iter().map(|&v| v.to_string()).collect(), err);
        let mut input_file = input_file_from_opts(vals[1], err);
        let mime_type = input_mime_from_opts(opt.value_of("mime").unwrap_or("application/octet-stream"), err);
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
                CallType::Upload(UploadProtocol::Simple) => call.upload(input_file.unwrap(), mime_type.unwrap()).await,
                CallType::Standard => unreachable!()
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

    async fn _playlist_images_list(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        let mut call = self.hub.playlist_images().list();
        for parg in opt.values_of("v").map(|i|i.collect()).unwrap_or(Vec::new()).iter() {
            let (key, value) = parse_kv_arg(&*parg, err, false);
            match key {
                "part" => {
                    call = call.add_part(value.unwrap_or(""));
                },
                "parent" => {
                    call = call.parent(value.unwrap_or(""));
                },
                "page-token" => {
                    call = call.page_token(value.unwrap_or(""));
                },
                "on-behalf-of-content-owner-channel" => {
                    call = call.on_behalf_of_content_owner_channel(value.unwrap_or(""));
                },
                "on-behalf-of-content-owner" => {
                    call = call.on_behalf_of_content_owner(value.unwrap_or(""));
                },
                "max-results" => {
                    call = call.max_results(        value.map(|v| arg_from_str(v, err, "max-results", "uint32")).unwrap_or(0));
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
                                                                           v.extend(["max-results", "on-behalf-of-content-owner", "on-behalf-of-content-owner-channel", "page-token", "parent", "part"].iter().map(|v|*v));
                                                                           v } ));
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

    async fn _playlist_images_update(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
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
                    "id" => Some(("id", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "kind" => Some(("kind", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "snippet.height" => Some(("snippet.height", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "snippet.playlist-id" => Some(("snippet.playlistId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "snippet.type" => Some(("snippet.type", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "snippet.width" => Some(("snippet.width", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    _ => {
                        let suggestion = FieldCursor::did_you_mean(key, &vec!["height", "id", "kind", "playlist-id", "snippet", "type", "width"]);
                        err.issues.push(CLIError::Field(FieldError::Unknown(temp_cursor.to_string(), suggestion, value.map(|v| v.to_string()))));
                        None
                    }
                };
            if let Some((field_cursor_str, type_info)) = type_info {
                FieldCursor::from(field_cursor_str).set_json_value(&mut object, value.unwrap(), type_info, err, &temp_cursor);
            }
        }
        let mut request: api::PlaylistImage = json::value::from_value(object).unwrap();
        let mut call = self.hub.playlist_images().update(request);
        for parg in opt.values_of("v").map(|i|i.collect()).unwrap_or(Vec::new()).iter() {
            let (key, value) = parse_kv_arg(&*parg, err, false);
            match key {
                "part" => {
                    call = call.add_part(value.unwrap_or(""));
                },
                "on-behalf-of-content-owner" => {
                    call = call.on_behalf_of_content_owner(value.unwrap_or(""));
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
                                                                           v.extend(["on-behalf-of-content-owner", "part"].iter().map(|v|*v));
                                                                           v } ));
                    }
                }
            }
        }
        let vals = opt.values_of("mode").unwrap().collect::<Vec<&str>>();
        let protocol = calltype_from_str(vals[0], ["simple"].iter().map(|&v| v.to_string()).collect(), err);
        let mut input_file = input_file_from_opts(vals[1], err);
        let mime_type = input_mime_from_opts(opt.value_of("mime").unwrap_or("application/octet-stream"), err);
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
                CallType::Upload(UploadProtocol::Simple) => call.upload(input_file.unwrap(), mime_type.unwrap()).await,
                CallType::Standard => unreachable!()
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

    async fn _playlist_items_delete(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        let mut call = self.hub.playlist_items().delete(opt.value_of("id").unwrap_or(""));
        for parg in opt.values_of("v").map(|i|i.collect()).unwrap_or(Vec::new()).iter() {
            let (key, value) = parse_kv_arg(&*parg, err, false);
            match key {
                "on-behalf-of-content-owner" => {
                    call = call.on_behalf_of_content_owner(value.unwrap_or(""));
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
                                                                           v.extend(["on-behalf-of-content-owner"].iter().map(|v|*v));
                                                                           v } ));
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
            match match protocol {
                CallType::Standard => call.doit().await,
                _ => unreachable!()
            } {
                Err(api_err) => Err(DoitError::ApiError(api_err)),
                Ok(mut response) => {
                    Ok(())
                }
            }
        }
    }

    async fn _playlist_items_insert(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
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
                    "content-details.end-at" => Some(("contentDetails.endAt", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "content-details.note" => Some(("contentDetails.note", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "content-details.start-at" => Some(("contentDetails.startAt", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "content-details.video-id" => Some(("contentDetails.videoId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "content-details.video-published-at" => Some(("contentDetails.videoPublishedAt", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "etag" => Some(("etag", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "id" => Some(("id", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "kind" => Some(("kind", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "snippet.channel-id" => Some(("snippet.channelId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "snippet.channel-title" => Some(("snippet.channelTitle", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "snippet.description" => Some(("snippet.description", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "snippet.playlist-id" => Some(("snippet.playlistId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "snippet.position" => Some(("snippet.position", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "snippet.published-at" => Some(("snippet.publishedAt", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "snippet.resource-id.channel-id" => Some(("snippet.resourceId.channelId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "snippet.resource-id.kind" => Some(("snippet.resourceId.kind", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "snippet.resource-id.playlist-id" => Some(("snippet.resourceId.playlistId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "snippet.resource-id.video-id" => Some(("snippet.resourceId.videoId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "snippet.thumbnails.default.height" => Some(("snippet.thumbnails.default.height", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "snippet.thumbnails.default.url" => Some(("snippet.thumbnails.default.url", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "snippet.thumbnails.default.width" => Some(("snippet.thumbnails.default.width", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "snippet.thumbnails.high.height" => Some(("snippet.thumbnails.high.height", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "snippet.thumbnails.high.url" => Some(("snippet.thumbnails.high.url", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "snippet.thumbnails.high.width" => Some(("snippet.thumbnails.high.width", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "snippet.thumbnails.maxres.height" => Some(("snippet.thumbnails.maxres.height", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "snippet.thumbnails.maxres.url" => Some(("snippet.thumbnails.maxres.url", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "snippet.thumbnails.maxres.width" => Some(("snippet.thumbnails.maxres.width", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "snippet.thumbnails.medium.height" => Some(("snippet.thumbnails.medium.height", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "snippet.thumbnails.medium.url" => Some(("snippet.thumbnails.medium.url", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "snippet.thumbnails.medium.width" => Some(("snippet.thumbnails.medium.width", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "snippet.thumbnails.standard.height" => Some(("snippet.thumbnails.standard.height", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "snippet.thumbnails.standard.url" => Some(("snippet.thumbnails.standard.url", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "snippet.thumbnails.standard.width" => Some(("snippet.thumbnails.standard.width", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "snippet.title" => Some(("snippet.title", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "snippet.video-owner-channel-id" => Some(("snippet.videoOwnerChannelId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "snippet.video-owner-channel-title" => Some(("snippet.videoOwnerChannelTitle", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "status.privacy-status" => Some(("status.privacyStatus", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    _ => {
                        let suggestion = FieldCursor::did_you_mean(key, &vec!["channel-id", "channel-title", "content-details", "default", "description", "end-at", "etag", "height", "high", "id", "kind", "maxres", "medium", "note", "playlist-id", "position", "privacy-status", "published-at", "resource-id", "snippet", "standard", "start-at", "status", "thumbnails", "title", "url", "video-id", "video-owner-channel-id", "video-owner-channel-title", "video-published-at", "width"]);
                        err.issues.push(CLIError::Field(FieldError::Unknown(temp_cursor.to_string(), suggestion, value.map(|v| v.to_string()))));
                        None
                    }
                };
            if let Some((field_cursor_str, type_info)) = type_info {
                FieldCursor::from(field_cursor_str).set_json_value(&mut object, value.unwrap(), type_info, err, &temp_cursor);
            }
        }
        let mut request: api::PlaylistItem = json::value::from_value(object).unwrap();
        let mut call = self.hub.playlist_items().insert(request);
        for parg in opt.values_of("v").map(|i|i.collect()).unwrap_or(Vec::new()).iter() {
            let (key, value) = parse_kv_arg(&*parg, err, false);
            match key {
                "on-behalf-of-content-owner" => {
                    call = call.on_behalf_of_content_owner(value.unwrap_or(""));
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
                                                                           v.extend(["on-behalf-of-content-owner"].iter().map(|v|*v));
                                                                           v } ));
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

    async fn _playlist_items_list(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        let mut call = self.hub.playlist_items().list(&opt.values_of("part").map(|i|i.collect()).unwrap_or(Vec::new()).iter().map(|&v| v.to_string()).collect::<Vec<String>>());
        for parg in opt.values_of("v").map(|i|i.collect()).unwrap_or(Vec::new()).iter() {
            let (key, value) = parse_kv_arg(&*parg, err, false);
            match key {
                "video-id" => {
                    call = call.video_id(value.unwrap_or(""));
                },
                "playlist-id" => {
                    call = call.playlist_id(value.unwrap_or(""));
                },
                "page-token" => {
                    call = call.page_token(value.unwrap_or(""));
                },
                "on-behalf-of-content-owner" => {
                    call = call.on_behalf_of_content_owner(value.unwrap_or(""));
                },
                "max-results" => {
                    call = call.max_results(        value.map(|v| arg_from_str(v, err, "max-results", "uint32")).unwrap_or(0));
                },
                "id" => {
                    call = call.add_id(value.unwrap_or(""));
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
                                                                           v.extend(["id", "max-results", "on-behalf-of-content-owner", "page-token", "playlist-id", "video-id"].iter().map(|v|*v));
                                                                           v } ));
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

    async fn _playlist_items_update(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
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
                    "content-details.end-at" => Some(("contentDetails.endAt", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "content-details.note" => Some(("contentDetails.note", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "content-details.start-at" => Some(("contentDetails.startAt", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "content-details.video-id" => Some(("contentDetails.videoId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "content-details.video-published-at" => Some(("contentDetails.videoPublishedAt", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "etag" => Some(("etag", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "id" => Some(("id", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "kind" => Some(("kind", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "snippet.channel-id" => Some(("snippet.channelId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "snippet.channel-title" => Some(("snippet.channelTitle", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "snippet.description" => Some(("snippet.description", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "snippet.playlist-id" => Some(("snippet.playlistId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "snippet.position" => Some(("snippet.position", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "snippet.published-at" => Some(("snippet.publishedAt", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "snippet.resource-id.channel-id" => Some(("snippet.resourceId.channelId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "snippet.resource-id.kind" => Some(("snippet.resourceId.kind", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "snippet.resource-id.playlist-id" => Some(("snippet.resourceId.playlistId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "snippet.resource-id.video-id" => Some(("snippet.resourceId.videoId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "snippet.thumbnails.default.height" => Some(("snippet.thumbnails.default.height", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "snippet.thumbnails.default.url" => Some(("snippet.thumbnails.default.url", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "snippet.thumbnails.default.width" => Some(("snippet.thumbnails.default.width", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "snippet.thumbnails.high.height" => Some(("snippet.thumbnails.high.height", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "snippet.thumbnails.high.url" => Some(("snippet.thumbnails.high.url", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "snippet.thumbnails.high.width" => Some(("snippet.thumbnails.high.width", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "snippet.thumbnails.maxres.height" => Some(("snippet.thumbnails.maxres.height", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "snippet.thumbnails.maxres.url" => Some(("snippet.thumbnails.maxres.url", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "snippet.thumbnails.maxres.width" => Some(("snippet.thumbnails.maxres.width", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "snippet.thumbnails.medium.height" => Some(("snippet.thumbnails.medium.height", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "snippet.thumbnails.medium.url" => Some(("snippet.thumbnails.medium.url", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "snippet.thumbnails.medium.width" => Some(("snippet.thumbnails.medium.width", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "snippet.thumbnails.standard.height" => Some(("snippet.thumbnails.standard.height", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "snippet.thumbnails.standard.url" => Some(("snippet.thumbnails.standard.url", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "snippet.thumbnails.standard.width" => Some(("snippet.thumbnails.standard.width", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "snippet.title" => Some(("snippet.title", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "snippet.video-owner-channel-id" => Some(("snippet.videoOwnerChannelId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "snippet.video-owner-channel-title" => Some(("snippet.videoOwnerChannelTitle", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "status.privacy-status" => Some(("status.privacyStatus", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    _ => {
                        let suggestion = FieldCursor::did_you_mean(key, &vec!["channel-id", "channel-title", "content-details", "default", "description", "end-at", "etag", "height", "high", "id", "kind", "maxres", "medium", "note", "playlist-id", "position", "privacy-status", "published-at", "resource-id", "snippet", "standard", "start-at", "status", "thumbnails", "title", "url", "video-id", "video-owner-channel-id", "video-owner-channel-title", "video-published-at", "width"]);
                        err.issues.push(CLIError::Field(FieldError::Unknown(temp_cursor.to_string(), suggestion, value.map(|v| v.to_string()))));
                        None
                    }
                };
            if let Some((field_cursor_str, type_info)) = type_info {
                FieldCursor::from(field_cursor_str).set_json_value(&mut object, value.unwrap(), type_info, err, &temp_cursor);
            }
        }
        let mut request: api::PlaylistItem = json::value::from_value(object).unwrap();
        let mut call = self.hub.playlist_items().update(request);
        for parg in opt.values_of("v").map(|i|i.collect()).unwrap_or(Vec::new()).iter() {
            let (key, value) = parse_kv_arg(&*parg, err, false);
            match key {
                "on-behalf-of-content-owner" => {
                    call = call.on_behalf_of_content_owner(value.unwrap_or(""));
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
                                                                           v.extend(["on-behalf-of-content-owner"].iter().map(|v|*v));
                                                                           v } ));
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

    async fn _playlists_delete(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        let mut call = self.hub.playlists().delete(opt.value_of("id").unwrap_or(""));
        for parg in opt.values_of("v").map(|i|i.collect()).unwrap_or(Vec::new()).iter() {
            let (key, value) = parse_kv_arg(&*parg, err, false);
            match key {
                "on-behalf-of-content-owner" => {
                    call = call.on_behalf_of_content_owner(value.unwrap_or(""));
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
                                                                           v.extend(["on-behalf-of-content-owner"].iter().map(|v|*v));
                                                                           v } ));
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
            match match protocol {
                CallType::Standard => call.doit().await,
                _ => unreachable!()
            } {
                Err(api_err) => Err(DoitError::ApiError(api_err)),
                Ok(mut response) => {
                    Ok(())
                }
            }
        }
    }

    async fn _playlists_insert(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
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
                    "content-details.item-count" => Some(("contentDetails.itemCount", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "etag" => Some(("etag", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "id" => Some(("id", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "kind" => Some(("kind", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "player.embed-html" => Some(("player.embedHtml", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "snippet.channel-id" => Some(("snippet.channelId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "snippet.channel-title" => Some(("snippet.channelTitle", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "snippet.default-language" => Some(("snippet.defaultLanguage", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "snippet.description" => Some(("snippet.description", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "snippet.localized.description" => Some(("snippet.localized.description", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "snippet.localized.title" => Some(("snippet.localized.title", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "snippet.published-at" => Some(("snippet.publishedAt", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "snippet.tags" => Some(("snippet.tags", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Vec })),
                    "snippet.thumbnail-video-id" => Some(("snippet.thumbnailVideoId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "snippet.thumbnails.default.height" => Some(("snippet.thumbnails.default.height", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "snippet.thumbnails.default.url" => Some(("snippet.thumbnails.default.url", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "snippet.thumbnails.default.width" => Some(("snippet.thumbnails.default.width", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "snippet.thumbnails.high.height" => Some(("snippet.thumbnails.high.height", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "snippet.thumbnails.high.url" => Some(("snippet.thumbnails.high.url", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "snippet.thumbnails.high.width" => Some(("snippet.thumbnails.high.width", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "snippet.thumbnails.maxres.height" => Some(("snippet.thumbnails.maxres.height", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "snippet.thumbnails.maxres.url" => Some(("snippet.thumbnails.maxres.url", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "snippet.thumbnails.maxres.width" => Some(("snippet.thumbnails.maxres.width", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "snippet.thumbnails.medium.height" => Some(("snippet.thumbnails.medium.height", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "snippet.thumbnails.medium.url" => Some(("snippet.thumbnails.medium.url", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "snippet.thumbnails.medium.width" => Some(("snippet.thumbnails.medium.width", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "snippet.thumbnails.standard.height" => Some(("snippet.thumbnails.standard.height", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "snippet.thumbnails.standard.url" => Some(("snippet.thumbnails.standard.url", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "snippet.thumbnails.standard.width" => Some(("snippet.thumbnails.standard.width", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "snippet.title" => Some(("snippet.title", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "status.privacy-status" => Some(("status.privacyStatus", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    _ => {
                        let suggestion = FieldCursor::did_you_mean(key, &vec!["channel-id", "channel-title", "content-details", "default", "default-language", "description", "embed-html", "etag", "height", "high", "id", "item-count", "kind", "localized", "maxres", "medium", "player", "privacy-status", "published-at", "snippet", "standard", "status", "tags", "thumbnail-video-id", "thumbnails", "title", "url", "width"]);
                        err.issues.push(CLIError::Field(FieldError::Unknown(temp_cursor.to_string(), suggestion, value.map(|v| v.to_string()))));
                        None
                    }
                };
            if let Some((field_cursor_str, type_info)) = type_info {
                FieldCursor::from(field_cursor_str).set_json_value(&mut object, value.unwrap(), type_info, err, &temp_cursor);
            }
        }
        let mut request: api::Playlist = json::value::from_value(object).unwrap();
        let mut call = self.hub.playlists().insert(request);
        for parg in opt.values_of("v").map(|i|i.collect()).unwrap_or(Vec::new()).iter() {
            let (key, value) = parse_kv_arg(&*parg, err, false);
            match key {
                "on-behalf-of-content-owner-channel" => {
                    call = call.on_behalf_of_content_owner_channel(value.unwrap_or(""));
                },
                "on-behalf-of-content-owner" => {
                    call = call.on_behalf_of_content_owner(value.unwrap_or(""));
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
                                                                           v.extend(["on-behalf-of-content-owner", "on-behalf-of-content-owner-channel"].iter().map(|v|*v));
                                                                           v } ));
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

    async fn _playlists_list(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        let mut call = self.hub.playlists().list(&opt.values_of("part").map(|i|i.collect()).unwrap_or(Vec::new()).iter().map(|&v| v.to_string()).collect::<Vec<String>>());
        for parg in opt.values_of("v").map(|i|i.collect()).unwrap_or(Vec::new()).iter() {
            let (key, value) = parse_kv_arg(&*parg, err, false);
            match key {
                "page-token" => {
                    call = call.page_token(value.unwrap_or(""));
                },
                "on-behalf-of-content-owner-channel" => {
                    call = call.on_behalf_of_content_owner_channel(value.unwrap_or(""));
                },
                "on-behalf-of-content-owner" => {
                    call = call.on_behalf_of_content_owner(value.unwrap_or(""));
                },
                "mine" => {
                    call = call.mine(        value.map(|v| arg_from_str(v, err, "mine", "boolean")).unwrap_or(false));
                },
                "max-results" => {
                    call = call.max_results(        value.map(|v| arg_from_str(v, err, "max-results", "uint32")).unwrap_or(0));
                },
                "id" => {
                    call = call.add_id(value.unwrap_or(""));
                },
                "hl" => {
                    call = call.hl(value.unwrap_or(""));
                },
                "channel-id" => {
                    call = call.channel_id(value.unwrap_or(""));
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
                                                                           v.extend(["channel-id", "hl", "id", "max-results", "mine", "on-behalf-of-content-owner", "on-behalf-of-content-owner-channel", "page-token"].iter().map(|v|*v));
                                                                           v } ));
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

    async fn _playlists_update(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
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
                    "content-details.item-count" => Some(("contentDetails.itemCount", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "etag" => Some(("etag", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "id" => Some(("id", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "kind" => Some(("kind", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "player.embed-html" => Some(("player.embedHtml", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "snippet.channel-id" => Some(("snippet.channelId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "snippet.channel-title" => Some(("snippet.channelTitle", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "snippet.default-language" => Some(("snippet.defaultLanguage", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "snippet.description" => Some(("snippet.description", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "snippet.localized.description" => Some(("snippet.localized.description", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "snippet.localized.title" => Some(("snippet.localized.title", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "snippet.published-at" => Some(("snippet.publishedAt", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "snippet.tags" => Some(("snippet.tags", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Vec })),
                    "snippet.thumbnail-video-id" => Some(("snippet.thumbnailVideoId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "snippet.thumbnails.default.height" => Some(("snippet.thumbnails.default.height", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "snippet.thumbnails.default.url" => Some(("snippet.thumbnails.default.url", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "snippet.thumbnails.default.width" => Some(("snippet.thumbnails.default.width", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "snippet.thumbnails.high.height" => Some(("snippet.thumbnails.high.height", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "snippet.thumbnails.high.url" => Some(("snippet.thumbnails.high.url", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "snippet.thumbnails.high.width" => Some(("snippet.thumbnails.high.width", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "snippet.thumbnails.maxres.height" => Some(("snippet.thumbnails.maxres.height", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "snippet.thumbnails.maxres.url" => Some(("snippet.thumbnails.maxres.url", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "snippet.thumbnails.maxres.width" => Some(("snippet.thumbnails.maxres.width", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "snippet.thumbnails.medium.height" => Some(("snippet.thumbnails.medium.height", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "snippet.thumbnails.medium.url" => Some(("snippet.thumbnails.medium.url", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "snippet.thumbnails.medium.width" => Some(("snippet.thumbnails.medium.width", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "snippet.thumbnails.standard.height" => Some(("snippet.thumbnails.standard.height", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "snippet.thumbnails.standard.url" => Some(("snippet.thumbnails.standard.url", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "snippet.thumbnails.standard.width" => Some(("snippet.thumbnails.standard.width", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "snippet.title" => Some(("snippet.title", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "status.privacy-status" => Some(("status.privacyStatus", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    _ => {
                        let suggestion = FieldCursor::did_you_mean(key, &vec!["channel-id", "channel-title", "content-details", "default", "default-language", "description", "embed-html", "etag", "height", "high", "id", "item-count", "kind", "localized", "maxres", "medium", "player", "privacy-status", "published-at", "snippet", "standard", "status", "tags", "thumbnail-video-id", "thumbnails", "title", "url", "width"]);
                        err.issues.push(CLIError::Field(FieldError::Unknown(temp_cursor.to_string(), suggestion, value.map(|v| v.to_string()))));
                        None
                    }
                };
            if let Some((field_cursor_str, type_info)) = type_info {
                FieldCursor::from(field_cursor_str).set_json_value(&mut object, value.unwrap(), type_info, err, &temp_cursor);
            }
        }
        let mut request: api::Playlist = json::value::from_value(object).unwrap();
        let mut call = self.hub.playlists().update(request);
        for parg in opt.values_of("v").map(|i|i.collect()).unwrap_or(Vec::new()).iter() {
            let (key, value) = parse_kv_arg(&*parg, err, false);
            match key {
                "on-behalf-of-content-owner" => {
                    call = call.on_behalf_of_content_owner(value.unwrap_or(""));
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
                                                                           v.extend(["on-behalf-of-content-owner"].iter().map(|v|*v));
                                                                           v } ));
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

    async fn _search_list(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        let mut call = self.hub.search().list(&opt.values_of("part").map(|i|i.collect()).unwrap_or(Vec::new()).iter().map(|&v| v.to_string()).collect::<Vec<String>>());
        for parg in opt.values_of("v").map(|i|i.collect()).unwrap_or(Vec::new()).iter() {
            let (key, value) = parse_kv_arg(&*parg, err, false);
            match key {
                "video-type" => {
                    call = call.video_type(value.unwrap_or(""));
                },
                "video-syndicated" => {
                    call = call.video_syndicated(value.unwrap_or(""));
                },
                "video-paid-product-placement" => {
                    call = call.video_paid_product_placement(value.unwrap_or(""));
                },
                "video-license" => {
                    call = call.video_license(value.unwrap_or(""));
                },
                "video-embeddable" => {
                    call = call.video_embeddable(value.unwrap_or(""));
                },
                "video-duration" => {
                    call = call.video_duration(value.unwrap_or(""));
                },
                "video-dimension" => {
                    call = call.video_dimension(value.unwrap_or(""));
                },
                "video-definition" => {
                    call = call.video_definition(value.unwrap_or(""));
                },
                "video-category-id" => {
                    call = call.video_category_id(value.unwrap_or(""));
                },
                "video-caption" => {
                    call = call.video_caption(value.unwrap_or(""));
                },
                "type" => {
                    call = call.add_type(value.unwrap_or(""));
                },
                "topic-id" => {
                    call = call.topic_id(value.unwrap_or(""));
                },
                "safe-search" => {
                    call = call.safe_search(value.unwrap_or(""));
                },
                "relevance-language" => {
                    call = call.relevance_language(value.unwrap_or(""));
                },
                "region-code" => {
                    call = call.region_code(value.unwrap_or(""));
                },
                "q" => {
                    call = call.q(value.unwrap_or(""));
                },
                "published-before" => {
                    call = call.published_before(        value.map(|v| arg_from_str(v, err, "published-before", "google-datetime")).unwrap_or(chrono::Utc::now()));
                },
                "published-after" => {
                    call = call.published_after(        value.map(|v| arg_from_str(v, err, "published-after", "google-datetime")).unwrap_or(chrono::Utc::now()));
                },
                "page-token" => {
                    call = call.page_token(value.unwrap_or(""));
                },
                "order" => {
                    call = call.order(value.unwrap_or(""));
                },
                "on-behalf-of-content-owner" => {
                    call = call.on_behalf_of_content_owner(value.unwrap_or(""));
                },
                "max-results" => {
                    call = call.max_results(        value.map(|v| arg_from_str(v, err, "max-results", "uint32")).unwrap_or(0));
                },
                "location-radius" => {
                    call = call.location_radius(value.unwrap_or(""));
                },
                "location" => {
                    call = call.location(value.unwrap_or(""));
                },
                "for-mine" => {
                    call = call.for_mine(        value.map(|v| arg_from_str(v, err, "for-mine", "boolean")).unwrap_or(false));
                },
                "for-developer" => {
                    call = call.for_developer(        value.map(|v| arg_from_str(v, err, "for-developer", "boolean")).unwrap_or(false));
                },
                "for-content-owner" => {
                    call = call.for_content_owner(        value.map(|v| arg_from_str(v, err, "for-content-owner", "boolean")).unwrap_or(false));
                },
                "event-type" => {
                    call = call.event_type(value.unwrap_or(""));
                },
                "channel-type" => {
                    call = call.channel_type(value.unwrap_or(""));
                },
                "channel-id" => {
                    call = call.channel_id(value.unwrap_or(""));
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
                                                                           v.extend(["channel-id", "channel-type", "event-type", "for-content-owner", "for-developer", "for-mine", "location", "location-radius", "max-results", "on-behalf-of-content-owner", "order", "page-token", "published-after", "published-before", "q", "region-code", "relevance-language", "safe-search", "topic-id", "type", "video-caption", "video-category-id", "video-definition", "video-dimension", "video-duration", "video-embeddable", "video-license", "video-paid-product-placement", "video-syndicated", "video-type"].iter().map(|v|*v));
                                                                           v } ));
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

    async fn _subscriptions_delete(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        let mut call = self.hub.subscriptions().delete(opt.value_of("id").unwrap_or(""));
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
            match match protocol {
                CallType::Standard => call.doit().await,
                _ => unreachable!()
            } {
                Err(api_err) => Err(DoitError::ApiError(api_err)),
                Ok(mut response) => {
                    Ok(())
                }
            }
        }
    }

    async fn _subscriptions_insert(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
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
                    "content-details.activity-type" => Some(("contentDetails.activityType", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "content-details.new-item-count" => Some(("contentDetails.newItemCount", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "content-details.total-item-count" => Some(("contentDetails.totalItemCount", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "etag" => Some(("etag", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "id" => Some(("id", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "kind" => Some(("kind", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "snippet.channel-id" => Some(("snippet.channelId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "snippet.channel-title" => Some(("snippet.channelTitle", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "snippet.description" => Some(("snippet.description", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "snippet.published-at" => Some(("snippet.publishedAt", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "snippet.resource-id.channel-id" => Some(("snippet.resourceId.channelId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "snippet.resource-id.kind" => Some(("snippet.resourceId.kind", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "snippet.resource-id.playlist-id" => Some(("snippet.resourceId.playlistId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "snippet.resource-id.video-id" => Some(("snippet.resourceId.videoId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "snippet.thumbnails.default.height" => Some(("snippet.thumbnails.default.height", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "snippet.thumbnails.default.url" => Some(("snippet.thumbnails.default.url", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "snippet.thumbnails.default.width" => Some(("snippet.thumbnails.default.width", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "snippet.thumbnails.high.height" => Some(("snippet.thumbnails.high.height", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "snippet.thumbnails.high.url" => Some(("snippet.thumbnails.high.url", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "snippet.thumbnails.high.width" => Some(("snippet.thumbnails.high.width", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "snippet.thumbnails.maxres.height" => Some(("snippet.thumbnails.maxres.height", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "snippet.thumbnails.maxres.url" => Some(("snippet.thumbnails.maxres.url", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "snippet.thumbnails.maxres.width" => Some(("snippet.thumbnails.maxres.width", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "snippet.thumbnails.medium.height" => Some(("snippet.thumbnails.medium.height", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "snippet.thumbnails.medium.url" => Some(("snippet.thumbnails.medium.url", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "snippet.thumbnails.medium.width" => Some(("snippet.thumbnails.medium.width", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "snippet.thumbnails.standard.height" => Some(("snippet.thumbnails.standard.height", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "snippet.thumbnails.standard.url" => Some(("snippet.thumbnails.standard.url", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "snippet.thumbnails.standard.width" => Some(("snippet.thumbnails.standard.width", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "snippet.title" => Some(("snippet.title", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "subscriber-snippet.channel-id" => Some(("subscriberSnippet.channelId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "subscriber-snippet.description" => Some(("subscriberSnippet.description", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "subscriber-snippet.thumbnails.default.height" => Some(("subscriberSnippet.thumbnails.default.height", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "subscriber-snippet.thumbnails.default.url" => Some(("subscriberSnippet.thumbnails.default.url", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "subscriber-snippet.thumbnails.default.width" => Some(("subscriberSnippet.thumbnails.default.width", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "subscriber-snippet.thumbnails.high.height" => Some(("subscriberSnippet.thumbnails.high.height", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "subscriber-snippet.thumbnails.high.url" => Some(("subscriberSnippet.thumbnails.high.url", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "subscriber-snippet.thumbnails.high.width" => Some(("subscriberSnippet.thumbnails.high.width", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "subscriber-snippet.thumbnails.maxres.height" => Some(("subscriberSnippet.thumbnails.maxres.height", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "subscriber-snippet.thumbnails.maxres.url" => Some(("subscriberSnippet.thumbnails.maxres.url", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "subscriber-snippet.thumbnails.maxres.width" => Some(("subscriberSnippet.thumbnails.maxres.width", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "subscriber-snippet.thumbnails.medium.height" => Some(("subscriberSnippet.thumbnails.medium.height", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "subscriber-snippet.thumbnails.medium.url" => Some(("subscriberSnippet.thumbnails.medium.url", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "subscriber-snippet.thumbnails.medium.width" => Some(("subscriberSnippet.thumbnails.medium.width", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "subscriber-snippet.thumbnails.standard.height" => Some(("subscriberSnippet.thumbnails.standard.height", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "subscriber-snippet.thumbnails.standard.url" => Some(("subscriberSnippet.thumbnails.standard.url", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "subscriber-snippet.thumbnails.standard.width" => Some(("subscriberSnippet.thumbnails.standard.width", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "subscriber-snippet.title" => Some(("subscriberSnippet.title", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    _ => {
                        let suggestion = FieldCursor::did_you_mean(key, &vec!["activity-type", "channel-id", "channel-title", "content-details", "default", "description", "etag", "height", "high", "id", "kind", "maxres", "medium", "new-item-count", "playlist-id", "published-at", "resource-id", "snippet", "standard", "subscriber-snippet", "thumbnails", "title", "total-item-count", "url", "video-id", "width"]);
                        err.issues.push(CLIError::Field(FieldError::Unknown(temp_cursor.to_string(), suggestion, value.map(|v| v.to_string()))));
                        None
                    }
                };
            if let Some((field_cursor_str, type_info)) = type_info {
                FieldCursor::from(field_cursor_str).set_json_value(&mut object, value.unwrap(), type_info, err, &temp_cursor);
            }
        }
        let mut request: api::Subscription = json::value::from_value(object).unwrap();
        let mut call = self.hub.subscriptions().insert(request);
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

    async fn _subscriptions_list(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        let mut call = self.hub.subscriptions().list(&opt.values_of("part").map(|i|i.collect()).unwrap_or(Vec::new()).iter().map(|&v| v.to_string()).collect::<Vec<String>>());
        for parg in opt.values_of("v").map(|i|i.collect()).unwrap_or(Vec::new()).iter() {
            let (key, value) = parse_kv_arg(&*parg, err, false);
            match key {
                "page-token" => {
                    call = call.page_token(value.unwrap_or(""));
                },
                "order" => {
                    call = call.order(value.unwrap_or(""));
                },
                "on-behalf-of-content-owner-channel" => {
                    call = call.on_behalf_of_content_owner_channel(value.unwrap_or(""));
                },
                "on-behalf-of-content-owner" => {
                    call = call.on_behalf_of_content_owner(value.unwrap_or(""));
                },
                "my-subscribers" => {
                    call = call.my_subscribers(        value.map(|v| arg_from_str(v, err, "my-subscribers", "boolean")).unwrap_or(false));
                },
                "my-recent-subscribers" => {
                    call = call.my_recent_subscribers(        value.map(|v| arg_from_str(v, err, "my-recent-subscribers", "boolean")).unwrap_or(false));
                },
                "mine" => {
                    call = call.mine(        value.map(|v| arg_from_str(v, err, "mine", "boolean")).unwrap_or(false));
                },
                "max-results" => {
                    call = call.max_results(        value.map(|v| arg_from_str(v, err, "max-results", "uint32")).unwrap_or(0));
                },
                "id" => {
                    call = call.add_id(value.unwrap_or(""));
                },
                "for-channel-id" => {
                    call = call.for_channel_id(value.unwrap_or(""));
                },
                "channel-id" => {
                    call = call.channel_id(value.unwrap_or(""));
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
                                                                           v.extend(["channel-id", "for-channel-id", "id", "max-results", "mine", "my-recent-subscribers", "my-subscribers", "on-behalf-of-content-owner", "on-behalf-of-content-owner-channel", "order", "page-token"].iter().map(|v|*v));
                                                                           v } ));
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

    async fn _super_chat_events_list(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        let mut call = self.hub.super_chat_events().list(&opt.values_of("part").map(|i|i.collect()).unwrap_or(Vec::new()).iter().map(|&v| v.to_string()).collect::<Vec<String>>());
        for parg in opt.values_of("v").map(|i|i.collect()).unwrap_or(Vec::new()).iter() {
            let (key, value) = parse_kv_arg(&*parg, err, false);
            match key {
                "page-token" => {
                    call = call.page_token(value.unwrap_or(""));
                },
                "max-results" => {
                    call = call.max_results(        value.map(|v| arg_from_str(v, err, "max-results", "uint32")).unwrap_or(0));
                },
                "hl" => {
                    call = call.hl(value.unwrap_or(""));
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
                                                                           v.extend(["hl", "max-results", "page-token"].iter().map(|v|*v));
                                                                           v } ));
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

    async fn _tests_insert(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
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
                    "featured-part" => Some(("featuredPart", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "gaia" => Some(("gaia", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "id" => Some(("id", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    _ => {
                        let suggestion = FieldCursor::did_you_mean(key, &vec!["featured-part", "gaia", "id"]);
                        err.issues.push(CLIError::Field(FieldError::Unknown(temp_cursor.to_string(), suggestion, value.map(|v| v.to_string()))));
                        None
                    }
                };
            if let Some((field_cursor_str, type_info)) = type_info {
                FieldCursor::from(field_cursor_str).set_json_value(&mut object, value.unwrap(), type_info, err, &temp_cursor);
            }
        }
        let mut request: api::TestItem = json::value::from_value(object).unwrap();
        let mut call = self.hub.tests().insert(request);
        for parg in opt.values_of("v").map(|i|i.collect()).unwrap_or(Vec::new()).iter() {
            let (key, value) = parse_kv_arg(&*parg, err, false);
            match key {
                "external-channel-id" => {
                    call = call.external_channel_id(value.unwrap_or(""));
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
                                                                           v.extend(["external-channel-id"].iter().map(|v|*v));
                                                                           v } ));
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

    async fn _third_party_links_delete(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        let mut call = self.hub.third_party_links().delete(opt.value_of("linking-token").unwrap_or(""), opt.value_of("type").unwrap_or(""));
        for parg in opt.values_of("v").map(|i|i.collect()).unwrap_or(Vec::new()).iter() {
            let (key, value) = parse_kv_arg(&*parg, err, false);
            match key {
                "part" => {
                    call = call.add_part(value.unwrap_or(""));
                },
                "external-channel-id" => {
                    call = call.external_channel_id(value.unwrap_or(""));
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
                                                                           v.extend(["external-channel-id", "part"].iter().map(|v|*v));
                                                                           v } ));
                    }
                }
            }
        }
        let protocol = CallType::Standard;
        if dry_run {
            Ok(())
        } else {
            assert!(err.issues.len() == 0);
            match match protocol {
                CallType::Standard => call.doit().await,
                _ => unreachable!()
            } {
                Err(api_err) => Err(DoitError::ApiError(api_err)),
                Ok(mut response) => {
                    Ok(())
                }
            }
        }
    }

    async fn _third_party_links_insert(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
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
                    "etag" => Some(("etag", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "kind" => Some(("kind", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "linking-token" => Some(("linkingToken", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "snippet.channel-to-store-link.billing-details.billing-status" => Some(("snippet.channelToStoreLink.billingDetails.billingStatus", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "snippet.channel-to-store-link.merchant-id" => Some(("snippet.channelToStoreLink.merchantId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "snippet.channel-to-store-link.store-name" => Some(("snippet.channelToStoreLink.storeName", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "snippet.channel-to-store-link.store-url" => Some(("snippet.channelToStoreLink.storeUrl", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "snippet.type" => Some(("snippet.type", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "status.link-status" => Some(("status.linkStatus", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    _ => {
                        let suggestion = FieldCursor::did_you_mean(key, &vec!["billing-details", "billing-status", "channel-to-store-link", "etag", "kind", "link-status", "linking-token", "merchant-id", "snippet", "status", "store-name", "store-url", "type"]);
                        err.issues.push(CLIError::Field(FieldError::Unknown(temp_cursor.to_string(), suggestion, value.map(|v| v.to_string()))));
                        None
                    }
                };
            if let Some((field_cursor_str, type_info)) = type_info {
                FieldCursor::from(field_cursor_str).set_json_value(&mut object, value.unwrap(), type_info, err, &temp_cursor);
            }
        }
        let mut request: api::ThirdPartyLink = json::value::from_value(object).unwrap();
        let mut call = self.hub.third_party_links().insert(request);
        for parg in opt.values_of("v").map(|i|i.collect()).unwrap_or(Vec::new()).iter() {
            let (key, value) = parse_kv_arg(&*parg, err, false);
            match key {
                "external-channel-id" => {
                    call = call.external_channel_id(value.unwrap_or(""));
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
                                                                           v.extend(["external-channel-id"].iter().map(|v|*v));
                                                                           v } ));
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

    async fn _third_party_links_list(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        let mut call = self.hub.third_party_links().list(&opt.values_of("part").map(|i|i.collect()).unwrap_or(Vec::new()).iter().map(|&v| v.to_string()).collect::<Vec<String>>());
        for parg in opt.values_of("v").map(|i|i.collect()).unwrap_or(Vec::new()).iter() {
            let (key, value) = parse_kv_arg(&*parg, err, false);
            match key {
                "type" => {
                    call = call.type_(value.unwrap_or(""));
                },
                "linking-token" => {
                    call = call.linking_token(value.unwrap_or(""));
                },
                "external-channel-id" => {
                    call = call.external_channel_id(value.unwrap_or(""));
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
                                                                           v.extend(["external-channel-id", "linking-token", "type"].iter().map(|v|*v));
                                                                           v } ));
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

    async fn _third_party_links_update(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
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
                    "etag" => Some(("etag", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "kind" => Some(("kind", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "linking-token" => Some(("linkingToken", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "snippet.channel-to-store-link.billing-details.billing-status" => Some(("snippet.channelToStoreLink.billingDetails.billingStatus", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "snippet.channel-to-store-link.merchant-id" => Some(("snippet.channelToStoreLink.merchantId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "snippet.channel-to-store-link.store-name" => Some(("snippet.channelToStoreLink.storeName", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "snippet.channel-to-store-link.store-url" => Some(("snippet.channelToStoreLink.storeUrl", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "snippet.type" => Some(("snippet.type", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "status.link-status" => Some(("status.linkStatus", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    _ => {
                        let suggestion = FieldCursor::did_you_mean(key, &vec!["billing-details", "billing-status", "channel-to-store-link", "etag", "kind", "link-status", "linking-token", "merchant-id", "snippet", "status", "store-name", "store-url", "type"]);
                        err.issues.push(CLIError::Field(FieldError::Unknown(temp_cursor.to_string(), suggestion, value.map(|v| v.to_string()))));
                        None
                    }
                };
            if let Some((field_cursor_str, type_info)) = type_info {
                FieldCursor::from(field_cursor_str).set_json_value(&mut object, value.unwrap(), type_info, err, &temp_cursor);
            }
        }
        let mut request: api::ThirdPartyLink = json::value::from_value(object).unwrap();
        let mut call = self.hub.third_party_links().update(request);
        for parg in opt.values_of("v").map(|i|i.collect()).unwrap_or(Vec::new()).iter() {
            let (key, value) = parse_kv_arg(&*parg, err, false);
            match key {
                "external-channel-id" => {
                    call = call.external_channel_id(value.unwrap_or(""));
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
                                                                           v.extend(["external-channel-id"].iter().map(|v|*v));
                                                                           v } ));
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

    async fn _thumbnails_set(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        let mut call = self.hub.thumbnails().set(opt.value_of("video-id").unwrap_or(""));
        for parg in opt.values_of("v").map(|i|i.collect()).unwrap_or(Vec::new()).iter() {
            let (key, value) = parse_kv_arg(&*parg, err, false);
            match key {
                "on-behalf-of-content-owner" => {
                    call = call.on_behalf_of_content_owner(value.unwrap_or(""));
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
                                                                           v.extend(["on-behalf-of-content-owner"].iter().map(|v|*v));
                                                                           v } ));
                    }
                }
            }
        }
        let vals = opt.values_of("mode").unwrap().collect::<Vec<&str>>();
        let protocol = calltype_from_str(vals[0], ["simple"].iter().map(|&v| v.to_string()).collect(), err);
        let mut input_file = input_file_from_opts(vals[1], err);
        let mime_type = input_mime_from_opts(opt.value_of("mime").unwrap_or("application/octet-stream"), err);
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
                CallType::Upload(UploadProtocol::Simple) => call.upload(input_file.unwrap(), mime_type.unwrap()).await,
                CallType::Standard => unreachable!()
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

    async fn _video_abuse_report_reasons_list(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        let mut call = self.hub.video_abuse_report_reasons().list(&opt.values_of("part").map(|i|i.collect()).unwrap_or(Vec::new()).iter().map(|&v| v.to_string()).collect::<Vec<String>>());
        for parg in opt.values_of("v").map(|i|i.collect()).unwrap_or(Vec::new()).iter() {
            let (key, value) = parse_kv_arg(&*parg, err, false);
            match key {
                "hl" => {
                    call = call.hl(value.unwrap_or(""));
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
                                                                           v.extend(["hl"].iter().map(|v|*v));
                                                                           v } ));
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

    async fn _video_categories_list(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        let mut call = self.hub.video_categories().list(&opt.values_of("part").map(|i|i.collect()).unwrap_or(Vec::new()).iter().map(|&v| v.to_string()).collect::<Vec<String>>());
        for parg in opt.values_of("v").map(|i|i.collect()).unwrap_or(Vec::new()).iter() {
            let (key, value) = parse_kv_arg(&*parg, err, false);
            match key {
                "region-code" => {
                    call = call.region_code(value.unwrap_or(""));
                },
                "id" => {
                    call = call.add_id(value.unwrap_or(""));
                },
                "hl" => {
                    call = call.hl(value.unwrap_or(""));
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
                                                                           v.extend(["hl", "id", "region-code"].iter().map(|v|*v));
                                                                           v } ));
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

    async fn _videos_delete(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        let mut call = self.hub.videos().delete(opt.value_of("id").unwrap_or(""));
        for parg in opt.values_of("v").map(|i|i.collect()).unwrap_or(Vec::new()).iter() {
            let (key, value) = parse_kv_arg(&*parg, err, false);
            match key {
                "on-behalf-of-content-owner" => {
                    call = call.on_behalf_of_content_owner(value.unwrap_or(""));
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
                                                                           v.extend(["on-behalf-of-content-owner"].iter().map(|v|*v));
                                                                           v } ));
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
            match match protocol {
                CallType::Standard => call.doit().await,
                _ => unreachable!()
            } {
                Err(api_err) => Err(DoitError::ApiError(api_err)),
                Ok(mut response) => {
                    Ok(())
                }
            }
        }
    }

    async fn _videos_get_rating(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        let mut call = self.hub.videos().get_rating(&opt.values_of("id").map(|i|i.collect()).unwrap_or(Vec::new()).iter().map(|&v| v.to_string()).collect::<Vec<String>>());
        for parg in opt.values_of("v").map(|i|i.collect()).unwrap_or(Vec::new()).iter() {
            let (key, value) = parse_kv_arg(&*parg, err, false);
            match key {
                "on-behalf-of-content-owner" => {
                    call = call.on_behalf_of_content_owner(value.unwrap_or(""));
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
                                                                           v.extend(["on-behalf-of-content-owner"].iter().map(|v|*v));
                                                                           v } ));
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

    async fn _videos_insert(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
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
                    "age-gating.alcohol-content" => Some(("ageGating.alcoholContent", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "age-gating.restricted" => Some(("ageGating.restricted", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "age-gating.video-game-rating" => Some(("ageGating.videoGameRating", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "content-details.caption" => Some(("contentDetails.caption", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "content-details.content-rating.acb-rating" => Some(("contentDetails.contentRating.acbRating", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "content-details.content-rating.agcom-rating" => Some(("contentDetails.contentRating.agcomRating", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "content-details.content-rating.anatel-rating" => Some(("contentDetails.contentRating.anatelRating", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "content-details.content-rating.bbfc-rating" => Some(("contentDetails.contentRating.bbfcRating", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "content-details.content-rating.bfvc-rating" => Some(("contentDetails.contentRating.bfvcRating", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "content-details.content-rating.bmukk-rating" => Some(("contentDetails.contentRating.bmukkRating", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "content-details.content-rating.catv-rating" => Some(("contentDetails.contentRating.catvRating", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "content-details.content-rating.catvfr-rating" => Some(("contentDetails.contentRating.catvfrRating", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "content-details.content-rating.cbfc-rating" => Some(("contentDetails.contentRating.cbfcRating", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "content-details.content-rating.ccc-rating" => Some(("contentDetails.contentRating.cccRating", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "content-details.content-rating.cce-rating" => Some(("contentDetails.contentRating.cceRating", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "content-details.content-rating.chfilm-rating" => Some(("contentDetails.contentRating.chfilmRating", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "content-details.content-rating.chvrs-rating" => Some(("contentDetails.contentRating.chvrsRating", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "content-details.content-rating.cicf-rating" => Some(("contentDetails.contentRating.cicfRating", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "content-details.content-rating.cna-rating" => Some(("contentDetails.contentRating.cnaRating", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "content-details.content-rating.cnc-rating" => Some(("contentDetails.contentRating.cncRating", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "content-details.content-rating.csa-rating" => Some(("contentDetails.contentRating.csaRating", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "content-details.content-rating.cscf-rating" => Some(("contentDetails.contentRating.cscfRating", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "content-details.content-rating.czfilm-rating" => Some(("contentDetails.contentRating.czfilmRating", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "content-details.content-rating.djctq-rating" => Some(("contentDetails.contentRating.djctqRating", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "content-details.content-rating.djctq-rating-reasons" => Some(("contentDetails.contentRating.djctqRatingReasons", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Vec })),
                    "content-details.content-rating.ecbmct-rating" => Some(("contentDetails.contentRating.ecbmctRating", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "content-details.content-rating.eefilm-rating" => Some(("contentDetails.contentRating.eefilmRating", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "content-details.content-rating.egfilm-rating" => Some(("contentDetails.contentRating.egfilmRating", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "content-details.content-rating.eirin-rating" => Some(("contentDetails.contentRating.eirinRating", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "content-details.content-rating.fcbm-rating" => Some(("contentDetails.contentRating.fcbmRating", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "content-details.content-rating.fco-rating" => Some(("contentDetails.contentRating.fcoRating", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "content-details.content-rating.fmoc-rating" => Some(("contentDetails.contentRating.fmocRating", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "content-details.content-rating.fpb-rating" => Some(("contentDetails.contentRating.fpbRating", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "content-details.content-rating.fpb-rating-reasons" => Some(("contentDetails.contentRating.fpbRatingReasons", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Vec })),
                    "content-details.content-rating.fsk-rating" => Some(("contentDetails.contentRating.fskRating", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "content-details.content-rating.grfilm-rating" => Some(("contentDetails.contentRating.grfilmRating", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "content-details.content-rating.icaa-rating" => Some(("contentDetails.contentRating.icaaRating", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "content-details.content-rating.ifco-rating" => Some(("contentDetails.contentRating.ifcoRating", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "content-details.content-rating.ilfilm-rating" => Some(("contentDetails.contentRating.ilfilmRating", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "content-details.content-rating.incaa-rating" => Some(("contentDetails.contentRating.incaaRating", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "content-details.content-rating.kfcb-rating" => Some(("contentDetails.contentRating.kfcbRating", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "content-details.content-rating.kijkwijzer-rating" => Some(("contentDetails.contentRating.kijkwijzerRating", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "content-details.content-rating.kmrb-rating" => Some(("contentDetails.contentRating.kmrbRating", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "content-details.content-rating.lsf-rating" => Some(("contentDetails.contentRating.lsfRating", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "content-details.content-rating.mccaa-rating" => Some(("contentDetails.contentRating.mccaaRating", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "content-details.content-rating.mccyp-rating" => Some(("contentDetails.contentRating.mccypRating", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "content-details.content-rating.mcst-rating" => Some(("contentDetails.contentRating.mcstRating", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "content-details.content-rating.mda-rating" => Some(("contentDetails.contentRating.mdaRating", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "content-details.content-rating.medietilsynet-rating" => Some(("contentDetails.contentRating.medietilsynetRating", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "content-details.content-rating.meku-rating" => Some(("contentDetails.contentRating.mekuRating", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "content-details.content-rating.mena-mpaa-rating" => Some(("contentDetails.contentRating.menaMpaaRating", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "content-details.content-rating.mibac-rating" => Some(("contentDetails.contentRating.mibacRating", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "content-details.content-rating.moc-rating" => Some(("contentDetails.contentRating.mocRating", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "content-details.content-rating.moctw-rating" => Some(("contentDetails.contentRating.moctwRating", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "content-details.content-rating.mpaa-rating" => Some(("contentDetails.contentRating.mpaaRating", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "content-details.content-rating.mpaat-rating" => Some(("contentDetails.contentRating.mpaatRating", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "content-details.content-rating.mtrcb-rating" => Some(("contentDetails.contentRating.mtrcbRating", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "content-details.content-rating.nbc-rating" => Some(("contentDetails.contentRating.nbcRating", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "content-details.content-rating.nbcpl-rating" => Some(("contentDetails.contentRating.nbcplRating", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "content-details.content-rating.nfrc-rating" => Some(("contentDetails.contentRating.nfrcRating", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "content-details.content-rating.nfvcb-rating" => Some(("contentDetails.contentRating.nfvcbRating", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "content-details.content-rating.nkclv-rating" => Some(("contentDetails.contentRating.nkclvRating", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "content-details.content-rating.nmc-rating" => Some(("contentDetails.contentRating.nmcRating", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "content-details.content-rating.oflc-rating" => Some(("contentDetails.contentRating.oflcRating", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "content-details.content-rating.pefilm-rating" => Some(("contentDetails.contentRating.pefilmRating", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "content-details.content-rating.rcnof-rating" => Some(("contentDetails.contentRating.rcnofRating", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "content-details.content-rating.resorteviolencia-rating" => Some(("contentDetails.contentRating.resorteviolenciaRating", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "content-details.content-rating.rtc-rating" => Some(("contentDetails.contentRating.rtcRating", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "content-details.content-rating.rte-rating" => Some(("contentDetails.contentRating.rteRating", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "content-details.content-rating.russia-rating" => Some(("contentDetails.contentRating.russiaRating", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "content-details.content-rating.skfilm-rating" => Some(("contentDetails.contentRating.skfilmRating", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "content-details.content-rating.smais-rating" => Some(("contentDetails.contentRating.smaisRating", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "content-details.content-rating.smsa-rating" => Some(("contentDetails.contentRating.smsaRating", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "content-details.content-rating.tvpg-rating" => Some(("contentDetails.contentRating.tvpgRating", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "content-details.content-rating.yt-rating" => Some(("contentDetails.contentRating.ytRating", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "content-details.country-restriction.allowed" => Some(("contentDetails.countryRestriction.allowed", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "content-details.country-restriction.exception" => Some(("contentDetails.countryRestriction.exception", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Vec })),
                    "content-details.definition" => Some(("contentDetails.definition", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "content-details.dimension" => Some(("contentDetails.dimension", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "content-details.duration" => Some(("contentDetails.duration", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "content-details.has-custom-thumbnail" => Some(("contentDetails.hasCustomThumbnail", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "content-details.licensed-content" => Some(("contentDetails.licensedContent", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "content-details.projection" => Some(("contentDetails.projection", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "content-details.region-restriction.allowed" => Some(("contentDetails.regionRestriction.allowed", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Vec })),
                    "content-details.region-restriction.blocked" => Some(("contentDetails.regionRestriction.blocked", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Vec })),
                    "etag" => Some(("etag", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "file-details.bitrate-bps" => Some(("fileDetails.bitrateBps", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "file-details.container" => Some(("fileDetails.container", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "file-details.creation-time" => Some(("fileDetails.creationTime", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "file-details.duration-ms" => Some(("fileDetails.durationMs", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "file-details.file-name" => Some(("fileDetails.fileName", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "file-details.file-size" => Some(("fileDetails.fileSize", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "file-details.file-type" => Some(("fileDetails.fileType", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "id" => Some(("id", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "kind" => Some(("kind", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "live-streaming-details.active-live-chat-id" => Some(("liveStreamingDetails.activeLiveChatId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "live-streaming-details.actual-end-time" => Some(("liveStreamingDetails.actualEndTime", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "live-streaming-details.actual-start-time" => Some(("liveStreamingDetails.actualStartTime", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "live-streaming-details.concurrent-viewers" => Some(("liveStreamingDetails.concurrentViewers", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "live-streaming-details.scheduled-end-time" => Some(("liveStreamingDetails.scheduledEndTime", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "live-streaming-details.scheduled-start-time" => Some(("liveStreamingDetails.scheduledStartTime", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "monetization-details.access.allowed" => Some(("monetizationDetails.access.allowed", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "monetization-details.access.exception" => Some(("monetizationDetails.access.exception", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Vec })),
                    "player.embed-height" => Some(("player.embedHeight", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "player.embed-html" => Some(("player.embedHtml", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "player.embed-width" => Some(("player.embedWidth", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "processing-details.editor-suggestions-availability" => Some(("processingDetails.editorSuggestionsAvailability", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "processing-details.file-details-availability" => Some(("processingDetails.fileDetailsAvailability", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "processing-details.processing-failure-reason" => Some(("processingDetails.processingFailureReason", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "processing-details.processing-issues-availability" => Some(("processingDetails.processingIssuesAvailability", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "processing-details.processing-progress.parts-processed" => Some(("processingDetails.processingProgress.partsProcessed", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "processing-details.processing-progress.parts-total" => Some(("processingDetails.processingProgress.partsTotal", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "processing-details.processing-progress.time-left-ms" => Some(("processingDetails.processingProgress.timeLeftMs", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "processing-details.processing-status" => Some(("processingDetails.processingStatus", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "processing-details.tag-suggestions-availability" => Some(("processingDetails.tagSuggestionsAvailability", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "processing-details.thumbnails-availability" => Some(("processingDetails.thumbnailsAvailability", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "recording-details.location.altitude" => Some(("recordingDetails.location.altitude", JsonTypeInfo { jtype: JsonType::Float, ctype: ComplexType::Pod })),
                    "recording-details.location.latitude" => Some(("recordingDetails.location.latitude", JsonTypeInfo { jtype: JsonType::Float, ctype: ComplexType::Pod })),
                    "recording-details.location.longitude" => Some(("recordingDetails.location.longitude", JsonTypeInfo { jtype: JsonType::Float, ctype: ComplexType::Pod })),
                    "recording-details.location-description" => Some(("recordingDetails.locationDescription", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "recording-details.recording-date" => Some(("recordingDetails.recordingDate", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "snippet.category-id" => Some(("snippet.categoryId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "snippet.channel-id" => Some(("snippet.channelId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "snippet.channel-title" => Some(("snippet.channelTitle", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "snippet.default-audio-language" => Some(("snippet.defaultAudioLanguage", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "snippet.default-language" => Some(("snippet.defaultLanguage", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "snippet.description" => Some(("snippet.description", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "snippet.live-broadcast-content" => Some(("snippet.liveBroadcastContent", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "snippet.localized.description" => Some(("snippet.localized.description", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "snippet.localized.title" => Some(("snippet.localized.title", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "snippet.published-at" => Some(("snippet.publishedAt", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "snippet.tags" => Some(("snippet.tags", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Vec })),
                    "snippet.thumbnails.default.height" => Some(("snippet.thumbnails.default.height", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "snippet.thumbnails.default.url" => Some(("snippet.thumbnails.default.url", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "snippet.thumbnails.default.width" => Some(("snippet.thumbnails.default.width", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "snippet.thumbnails.high.height" => Some(("snippet.thumbnails.high.height", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "snippet.thumbnails.high.url" => Some(("snippet.thumbnails.high.url", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "snippet.thumbnails.high.width" => Some(("snippet.thumbnails.high.width", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "snippet.thumbnails.maxres.height" => Some(("snippet.thumbnails.maxres.height", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "snippet.thumbnails.maxres.url" => Some(("snippet.thumbnails.maxres.url", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "snippet.thumbnails.maxres.width" => Some(("snippet.thumbnails.maxres.width", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "snippet.thumbnails.medium.height" => Some(("snippet.thumbnails.medium.height", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "snippet.thumbnails.medium.url" => Some(("snippet.thumbnails.medium.url", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "snippet.thumbnails.medium.width" => Some(("snippet.thumbnails.medium.width", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "snippet.thumbnails.standard.height" => Some(("snippet.thumbnails.standard.height", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "snippet.thumbnails.standard.url" => Some(("snippet.thumbnails.standard.url", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "snippet.thumbnails.standard.width" => Some(("snippet.thumbnails.standard.width", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "snippet.title" => Some(("snippet.title", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "statistics.comment-count" => Some(("statistics.commentCount", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "statistics.dislike-count" => Some(("statistics.dislikeCount", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "statistics.favorite-count" => Some(("statistics.favoriteCount", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "statistics.like-count" => Some(("statistics.likeCount", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "statistics.view-count" => Some(("statistics.viewCount", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "status.embeddable" => Some(("status.embeddable", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "status.failure-reason" => Some(("status.failureReason", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "status.license" => Some(("status.license", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "status.made-for-kids" => Some(("status.madeForKids", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "status.privacy-status" => Some(("status.privacyStatus", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "status.public-stats-viewable" => Some(("status.publicStatsViewable", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "status.publish-at" => Some(("status.publishAt", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "status.rejection-reason" => Some(("status.rejectionReason", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "status.self-declared-made-for-kids" => Some(("status.selfDeclaredMadeForKids", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "status.upload-status" => Some(("status.uploadStatus", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "suggestions.editor-suggestions" => Some(("suggestions.editorSuggestions", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Vec })),
                    "suggestions.processing-errors" => Some(("suggestions.processingErrors", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Vec })),
                    "suggestions.processing-hints" => Some(("suggestions.processingHints", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Vec })),
                    "suggestions.processing-warnings" => Some(("suggestions.processingWarnings", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Vec })),
                    "topic-details.relevant-topic-ids" => Some(("topicDetails.relevantTopicIds", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Vec })),
                    "topic-details.topic-categories" => Some(("topicDetails.topicCategories", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Vec })),
                    "topic-details.topic-ids" => Some(("topicDetails.topicIds", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Vec })),
                    _ => {
                        let suggestion = FieldCursor::did_you_mean(key, &vec!["acb-rating", "access", "active-live-chat-id", "actual-end-time", "actual-start-time", "agcom-rating", "age-gating", "alcohol-content", "allowed", "altitude", "anatel-rating", "bbfc-rating", "bfvc-rating", "bitrate-bps", "blocked", "bmukk-rating", "caption", "category-id", "catv-rating", "catvfr-rating", "cbfc-rating", "ccc-rating", "cce-rating", "channel-id", "channel-title", "chfilm-rating", "chvrs-rating", "cicf-rating", "cna-rating", "cnc-rating", "comment-count", "concurrent-viewers", "container", "content-details", "content-rating", "country-restriction", "creation-time", "csa-rating", "cscf-rating", "czfilm-rating", "default", "default-audio-language", "default-language", "definition", "description", "dimension", "dislike-count", "djctq-rating", "djctq-rating-reasons", "duration", "duration-ms", "ecbmct-rating", "editor-suggestions", "editor-suggestions-availability", "eefilm-rating", "egfilm-rating", "eirin-rating", "embed-height", "embed-html", "embed-width", "embeddable", "etag", "exception", "failure-reason", "favorite-count", "fcbm-rating", "fco-rating", "file-details", "file-details-availability", "file-name", "file-size", "file-type", "fmoc-rating", "fpb-rating", "fpb-rating-reasons", "fsk-rating", "grfilm-rating", "has-custom-thumbnail", "height", "high", "icaa-rating", "id", "ifco-rating", "ilfilm-rating", "incaa-rating", "kfcb-rating", "kijkwijzer-rating", "kind", "kmrb-rating", "latitude", "license", "licensed-content", "like-count", "live-broadcast-content", "live-streaming-details", "localized", "location", "location-description", "longitude", "lsf-rating", "made-for-kids", "maxres", "mccaa-rating", "mccyp-rating", "mcst-rating", "mda-rating", "medietilsynet-rating", "medium", "meku-rating", "mena-mpaa-rating", "mibac-rating", "moc-rating", "moctw-rating", "monetization-details", "mpaa-rating", "mpaat-rating", "mtrcb-rating", "nbc-rating", "nbcpl-rating", "nfrc-rating", "nfvcb-rating", "nkclv-rating", "nmc-rating", "oflc-rating", "parts-processed", "parts-total", "pefilm-rating", "player", "privacy-status", "processing-details", "processing-errors", "processing-failure-reason", "processing-hints", "processing-issues-availability", "processing-progress", "processing-status", "processing-warnings", "projection", "public-stats-viewable", "publish-at", "published-at", "rcnof-rating", "recording-date", "recording-details", "region-restriction", "rejection-reason", "relevant-topic-ids", "resorteviolencia-rating", "restricted", "rtc-rating", "rte-rating", "russia-rating", "scheduled-end-time", "scheduled-start-time", "self-declared-made-for-kids", "skfilm-rating", "smais-rating", "smsa-rating", "snippet", "standard", "statistics", "status", "suggestions", "tag-suggestions-availability", "tags", "thumbnails", "thumbnails-availability", "time-left-ms", "title", "topic-categories", "topic-details", "topic-ids", "tvpg-rating", "upload-status", "url", "video-game-rating", "view-count", "width", "yt-rating"]);
                        err.issues.push(CLIError::Field(FieldError::Unknown(temp_cursor.to_string(), suggestion, value.map(|v| v.to_string()))));
                        None
                    }
                };
            if let Some((field_cursor_str, type_info)) = type_info {
                FieldCursor::from(field_cursor_str).set_json_value(&mut object, value.unwrap(), type_info, err, &temp_cursor);
            }
        }
        let mut request: api::Video = json::value::from_value(object).unwrap();
        let mut call = self.hub.videos().insert(request);
        for parg in opt.values_of("v").map(|i|i.collect()).unwrap_or(Vec::new()).iter() {
            let (key, value) = parse_kv_arg(&*parg, err, false);
            match key {
                "stabilize" => {
                    call = call.stabilize(        value.map(|v| arg_from_str(v, err, "stabilize", "boolean")).unwrap_or(false));
                },
                "on-behalf-of-content-owner-channel" => {
                    call = call.on_behalf_of_content_owner_channel(value.unwrap_or(""));
                },
                "on-behalf-of-content-owner" => {
                    call = call.on_behalf_of_content_owner(value.unwrap_or(""));
                },
                "notify-subscribers" => {
                    call = call.notify_subscribers(        value.map(|v| arg_from_str(v, err, "notify-subscribers", "boolean")).unwrap_or(false));
                },
                "auto-levels" => {
                    call = call.auto_levels(        value.map(|v| arg_from_str(v, err, "auto-levels", "boolean")).unwrap_or(false));
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
                                                                           v.extend(["auto-levels", "notify-subscribers", "on-behalf-of-content-owner", "on-behalf-of-content-owner-channel", "stabilize"].iter().map(|v|*v));
                                                                           v } ));
                    }
                }
            }
        }
        let vals = opt.values_of("mode").unwrap().collect::<Vec<&str>>();
        let protocol = calltype_from_str(vals[0], ["simple"].iter().map(|&v| v.to_string()).collect(), err);
        let mut input_file = input_file_from_opts(vals[1], err);
        let mime_type = input_mime_from_opts(opt.value_of("mime").unwrap_or("application/octet-stream"), err);
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
                CallType::Upload(UploadProtocol::Simple) => call.upload(input_file.unwrap(), mime_type.unwrap()).await,
                CallType::Standard => unreachable!()
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

    async fn _videos_list(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        let mut call = self.hub.videos().list(&opt.values_of("part").map(|i|i.collect()).unwrap_or(Vec::new()).iter().map(|&v| v.to_string()).collect::<Vec<String>>());
        for parg in opt.values_of("v").map(|i|i.collect()).unwrap_or(Vec::new()).iter() {
            let (key, value) = parse_kv_arg(&*parg, err, false);
            match key {
                "video-category-id" => {
                    call = call.video_category_id(value.unwrap_or(""));
                },
                "region-code" => {
                    call = call.region_code(value.unwrap_or(""));
                },
                "page-token" => {
                    call = call.page_token(value.unwrap_or(""));
                },
                "on-behalf-of-content-owner" => {
                    call = call.on_behalf_of_content_owner(value.unwrap_or(""));
                },
                "my-rating" => {
                    call = call.my_rating(value.unwrap_or(""));
                },
                "max-width" => {
                    call = call.max_width(        value.map(|v| arg_from_str(v, err, "max-width", "int32")).unwrap_or(-0));
                },
                "max-results" => {
                    call = call.max_results(        value.map(|v| arg_from_str(v, err, "max-results", "uint32")).unwrap_or(0));
                },
                "max-height" => {
                    call = call.max_height(        value.map(|v| arg_from_str(v, err, "max-height", "int32")).unwrap_or(-0));
                },
                "locale" => {
                    call = call.locale(value.unwrap_or(""));
                },
                "id" => {
                    call = call.add_id(value.unwrap_or(""));
                },
                "hl" => {
                    call = call.hl(value.unwrap_or(""));
                },
                "chart" => {
                    call = call.chart(value.unwrap_or(""));
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
                                                                           v.extend(["chart", "hl", "id", "locale", "max-height", "max-results", "max-width", "my-rating", "on-behalf-of-content-owner", "page-token", "region-code", "video-category-id"].iter().map(|v|*v));
                                                                           v } ));
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

    async fn _videos_rate(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        let mut call = self.hub.videos().rate(opt.value_of("id").unwrap_or(""), opt.value_of("rating").unwrap_or(""));
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
            match match protocol {
                CallType::Standard => call.doit().await,
                _ => unreachable!()
            } {
                Err(api_err) => Err(DoitError::ApiError(api_err)),
                Ok(mut response) => {
                    Ok(())
                }
            }
        }
    }

    async fn _videos_report_abuse(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
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
                    "comments" => Some(("comments", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "language" => Some(("language", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "reason-id" => Some(("reasonId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "secondary-reason-id" => Some(("secondaryReasonId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "video-id" => Some(("videoId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    _ => {
                        let suggestion = FieldCursor::did_you_mean(key, &vec!["comments", "language", "reason-id", "secondary-reason-id", "video-id"]);
                        err.issues.push(CLIError::Field(FieldError::Unknown(temp_cursor.to_string(), suggestion, value.map(|v| v.to_string()))));
                        None
                    }
                };
            if let Some((field_cursor_str, type_info)) = type_info {
                FieldCursor::from(field_cursor_str).set_json_value(&mut object, value.unwrap(), type_info, err, &temp_cursor);
            }
        }
        let mut request: api::VideoAbuseReport = json::value::from_value(object).unwrap();
        let mut call = self.hub.videos().report_abuse(request);
        for parg in opt.values_of("v").map(|i|i.collect()).unwrap_or(Vec::new()).iter() {
            let (key, value) = parse_kv_arg(&*parg, err, false);
            match key {
                "on-behalf-of-content-owner" => {
                    call = call.on_behalf_of_content_owner(value.unwrap_or(""));
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
                                                                           v.extend(["on-behalf-of-content-owner"].iter().map(|v|*v));
                                                                           v } ));
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
            match match protocol {
                CallType::Standard => call.doit().await,
                _ => unreachable!()
            } {
                Err(api_err) => Err(DoitError::ApiError(api_err)),
                Ok(mut response) => {
                    Ok(())
                }
            }
        }
    }

    async fn _videos_update(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
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
                    "age-gating.alcohol-content" => Some(("ageGating.alcoholContent", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "age-gating.restricted" => Some(("ageGating.restricted", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "age-gating.video-game-rating" => Some(("ageGating.videoGameRating", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "content-details.caption" => Some(("contentDetails.caption", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "content-details.content-rating.acb-rating" => Some(("contentDetails.contentRating.acbRating", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "content-details.content-rating.agcom-rating" => Some(("contentDetails.contentRating.agcomRating", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "content-details.content-rating.anatel-rating" => Some(("contentDetails.contentRating.anatelRating", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "content-details.content-rating.bbfc-rating" => Some(("contentDetails.contentRating.bbfcRating", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "content-details.content-rating.bfvc-rating" => Some(("contentDetails.contentRating.bfvcRating", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "content-details.content-rating.bmukk-rating" => Some(("contentDetails.contentRating.bmukkRating", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "content-details.content-rating.catv-rating" => Some(("contentDetails.contentRating.catvRating", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "content-details.content-rating.catvfr-rating" => Some(("contentDetails.contentRating.catvfrRating", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "content-details.content-rating.cbfc-rating" => Some(("contentDetails.contentRating.cbfcRating", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "content-details.content-rating.ccc-rating" => Some(("contentDetails.contentRating.cccRating", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "content-details.content-rating.cce-rating" => Some(("contentDetails.contentRating.cceRating", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "content-details.content-rating.chfilm-rating" => Some(("contentDetails.contentRating.chfilmRating", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "content-details.content-rating.chvrs-rating" => Some(("contentDetails.contentRating.chvrsRating", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "content-details.content-rating.cicf-rating" => Some(("contentDetails.contentRating.cicfRating", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "content-details.content-rating.cna-rating" => Some(("contentDetails.contentRating.cnaRating", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "content-details.content-rating.cnc-rating" => Some(("contentDetails.contentRating.cncRating", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "content-details.content-rating.csa-rating" => Some(("contentDetails.contentRating.csaRating", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "content-details.content-rating.cscf-rating" => Some(("contentDetails.contentRating.cscfRating", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "content-details.content-rating.czfilm-rating" => Some(("contentDetails.contentRating.czfilmRating", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "content-details.content-rating.djctq-rating" => Some(("contentDetails.contentRating.djctqRating", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "content-details.content-rating.djctq-rating-reasons" => Some(("contentDetails.contentRating.djctqRatingReasons", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Vec })),
                    "content-details.content-rating.ecbmct-rating" => Some(("contentDetails.contentRating.ecbmctRating", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "content-details.content-rating.eefilm-rating" => Some(("contentDetails.contentRating.eefilmRating", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "content-details.content-rating.egfilm-rating" => Some(("contentDetails.contentRating.egfilmRating", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "content-details.content-rating.eirin-rating" => Some(("contentDetails.contentRating.eirinRating", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "content-details.content-rating.fcbm-rating" => Some(("contentDetails.contentRating.fcbmRating", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "content-details.content-rating.fco-rating" => Some(("contentDetails.contentRating.fcoRating", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "content-details.content-rating.fmoc-rating" => Some(("contentDetails.contentRating.fmocRating", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "content-details.content-rating.fpb-rating" => Some(("contentDetails.contentRating.fpbRating", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "content-details.content-rating.fpb-rating-reasons" => Some(("contentDetails.contentRating.fpbRatingReasons", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Vec })),
                    "content-details.content-rating.fsk-rating" => Some(("contentDetails.contentRating.fskRating", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "content-details.content-rating.grfilm-rating" => Some(("contentDetails.contentRating.grfilmRating", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "content-details.content-rating.icaa-rating" => Some(("contentDetails.contentRating.icaaRating", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "content-details.content-rating.ifco-rating" => Some(("contentDetails.contentRating.ifcoRating", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "content-details.content-rating.ilfilm-rating" => Some(("contentDetails.contentRating.ilfilmRating", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "content-details.content-rating.incaa-rating" => Some(("contentDetails.contentRating.incaaRating", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "content-details.content-rating.kfcb-rating" => Some(("contentDetails.contentRating.kfcbRating", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "content-details.content-rating.kijkwijzer-rating" => Some(("contentDetails.contentRating.kijkwijzerRating", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "content-details.content-rating.kmrb-rating" => Some(("contentDetails.contentRating.kmrbRating", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "content-details.content-rating.lsf-rating" => Some(("contentDetails.contentRating.lsfRating", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "content-details.content-rating.mccaa-rating" => Some(("contentDetails.contentRating.mccaaRating", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "content-details.content-rating.mccyp-rating" => Some(("contentDetails.contentRating.mccypRating", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "content-details.content-rating.mcst-rating" => Some(("contentDetails.contentRating.mcstRating", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "content-details.content-rating.mda-rating" => Some(("contentDetails.contentRating.mdaRating", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "content-details.content-rating.medietilsynet-rating" => Some(("contentDetails.contentRating.medietilsynetRating", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "content-details.content-rating.meku-rating" => Some(("contentDetails.contentRating.mekuRating", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "content-details.content-rating.mena-mpaa-rating" => Some(("contentDetails.contentRating.menaMpaaRating", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "content-details.content-rating.mibac-rating" => Some(("contentDetails.contentRating.mibacRating", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "content-details.content-rating.moc-rating" => Some(("contentDetails.contentRating.mocRating", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "content-details.content-rating.moctw-rating" => Some(("contentDetails.contentRating.moctwRating", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "content-details.content-rating.mpaa-rating" => Some(("contentDetails.contentRating.mpaaRating", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "content-details.content-rating.mpaat-rating" => Some(("contentDetails.contentRating.mpaatRating", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "content-details.content-rating.mtrcb-rating" => Some(("contentDetails.contentRating.mtrcbRating", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "content-details.content-rating.nbc-rating" => Some(("contentDetails.contentRating.nbcRating", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "content-details.content-rating.nbcpl-rating" => Some(("contentDetails.contentRating.nbcplRating", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "content-details.content-rating.nfrc-rating" => Some(("contentDetails.contentRating.nfrcRating", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "content-details.content-rating.nfvcb-rating" => Some(("contentDetails.contentRating.nfvcbRating", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "content-details.content-rating.nkclv-rating" => Some(("contentDetails.contentRating.nkclvRating", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "content-details.content-rating.nmc-rating" => Some(("contentDetails.contentRating.nmcRating", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "content-details.content-rating.oflc-rating" => Some(("contentDetails.contentRating.oflcRating", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "content-details.content-rating.pefilm-rating" => Some(("contentDetails.contentRating.pefilmRating", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "content-details.content-rating.rcnof-rating" => Some(("contentDetails.contentRating.rcnofRating", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "content-details.content-rating.resorteviolencia-rating" => Some(("contentDetails.contentRating.resorteviolenciaRating", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "content-details.content-rating.rtc-rating" => Some(("contentDetails.contentRating.rtcRating", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "content-details.content-rating.rte-rating" => Some(("contentDetails.contentRating.rteRating", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "content-details.content-rating.russia-rating" => Some(("contentDetails.contentRating.russiaRating", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "content-details.content-rating.skfilm-rating" => Some(("contentDetails.contentRating.skfilmRating", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "content-details.content-rating.smais-rating" => Some(("contentDetails.contentRating.smaisRating", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "content-details.content-rating.smsa-rating" => Some(("contentDetails.contentRating.smsaRating", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "content-details.content-rating.tvpg-rating" => Some(("contentDetails.contentRating.tvpgRating", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "content-details.content-rating.yt-rating" => Some(("contentDetails.contentRating.ytRating", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "content-details.country-restriction.allowed" => Some(("contentDetails.countryRestriction.allowed", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "content-details.country-restriction.exception" => Some(("contentDetails.countryRestriction.exception", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Vec })),
                    "content-details.definition" => Some(("contentDetails.definition", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "content-details.dimension" => Some(("contentDetails.dimension", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "content-details.duration" => Some(("contentDetails.duration", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "content-details.has-custom-thumbnail" => Some(("contentDetails.hasCustomThumbnail", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "content-details.licensed-content" => Some(("contentDetails.licensedContent", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "content-details.projection" => Some(("contentDetails.projection", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "content-details.region-restriction.allowed" => Some(("contentDetails.regionRestriction.allowed", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Vec })),
                    "content-details.region-restriction.blocked" => Some(("contentDetails.regionRestriction.blocked", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Vec })),
                    "etag" => Some(("etag", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "file-details.bitrate-bps" => Some(("fileDetails.bitrateBps", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "file-details.container" => Some(("fileDetails.container", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "file-details.creation-time" => Some(("fileDetails.creationTime", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "file-details.duration-ms" => Some(("fileDetails.durationMs", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "file-details.file-name" => Some(("fileDetails.fileName", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "file-details.file-size" => Some(("fileDetails.fileSize", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "file-details.file-type" => Some(("fileDetails.fileType", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "id" => Some(("id", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "kind" => Some(("kind", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "live-streaming-details.active-live-chat-id" => Some(("liveStreamingDetails.activeLiveChatId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "live-streaming-details.actual-end-time" => Some(("liveStreamingDetails.actualEndTime", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "live-streaming-details.actual-start-time" => Some(("liveStreamingDetails.actualStartTime", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "live-streaming-details.concurrent-viewers" => Some(("liveStreamingDetails.concurrentViewers", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "live-streaming-details.scheduled-end-time" => Some(("liveStreamingDetails.scheduledEndTime", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "live-streaming-details.scheduled-start-time" => Some(("liveStreamingDetails.scheduledStartTime", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "monetization-details.access.allowed" => Some(("monetizationDetails.access.allowed", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "monetization-details.access.exception" => Some(("monetizationDetails.access.exception", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Vec })),
                    "player.embed-height" => Some(("player.embedHeight", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "player.embed-html" => Some(("player.embedHtml", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "player.embed-width" => Some(("player.embedWidth", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "processing-details.editor-suggestions-availability" => Some(("processingDetails.editorSuggestionsAvailability", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "processing-details.file-details-availability" => Some(("processingDetails.fileDetailsAvailability", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "processing-details.processing-failure-reason" => Some(("processingDetails.processingFailureReason", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "processing-details.processing-issues-availability" => Some(("processingDetails.processingIssuesAvailability", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "processing-details.processing-progress.parts-processed" => Some(("processingDetails.processingProgress.partsProcessed", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "processing-details.processing-progress.parts-total" => Some(("processingDetails.processingProgress.partsTotal", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "processing-details.processing-progress.time-left-ms" => Some(("processingDetails.processingProgress.timeLeftMs", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "processing-details.processing-status" => Some(("processingDetails.processingStatus", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "processing-details.tag-suggestions-availability" => Some(("processingDetails.tagSuggestionsAvailability", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "processing-details.thumbnails-availability" => Some(("processingDetails.thumbnailsAvailability", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "recording-details.location.altitude" => Some(("recordingDetails.location.altitude", JsonTypeInfo { jtype: JsonType::Float, ctype: ComplexType::Pod })),
                    "recording-details.location.latitude" => Some(("recordingDetails.location.latitude", JsonTypeInfo { jtype: JsonType::Float, ctype: ComplexType::Pod })),
                    "recording-details.location.longitude" => Some(("recordingDetails.location.longitude", JsonTypeInfo { jtype: JsonType::Float, ctype: ComplexType::Pod })),
                    "recording-details.location-description" => Some(("recordingDetails.locationDescription", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "recording-details.recording-date" => Some(("recordingDetails.recordingDate", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "snippet.category-id" => Some(("snippet.categoryId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "snippet.channel-id" => Some(("snippet.channelId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "snippet.channel-title" => Some(("snippet.channelTitle", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "snippet.default-audio-language" => Some(("snippet.defaultAudioLanguage", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "snippet.default-language" => Some(("snippet.defaultLanguage", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "snippet.description" => Some(("snippet.description", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "snippet.live-broadcast-content" => Some(("snippet.liveBroadcastContent", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "snippet.localized.description" => Some(("snippet.localized.description", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "snippet.localized.title" => Some(("snippet.localized.title", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "snippet.published-at" => Some(("snippet.publishedAt", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "snippet.tags" => Some(("snippet.tags", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Vec })),
                    "snippet.thumbnails.default.height" => Some(("snippet.thumbnails.default.height", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "snippet.thumbnails.default.url" => Some(("snippet.thumbnails.default.url", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "snippet.thumbnails.default.width" => Some(("snippet.thumbnails.default.width", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "snippet.thumbnails.high.height" => Some(("snippet.thumbnails.high.height", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "snippet.thumbnails.high.url" => Some(("snippet.thumbnails.high.url", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "snippet.thumbnails.high.width" => Some(("snippet.thumbnails.high.width", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "snippet.thumbnails.maxres.height" => Some(("snippet.thumbnails.maxres.height", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "snippet.thumbnails.maxres.url" => Some(("snippet.thumbnails.maxres.url", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "snippet.thumbnails.maxres.width" => Some(("snippet.thumbnails.maxres.width", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "snippet.thumbnails.medium.height" => Some(("snippet.thumbnails.medium.height", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "snippet.thumbnails.medium.url" => Some(("snippet.thumbnails.medium.url", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "snippet.thumbnails.medium.width" => Some(("snippet.thumbnails.medium.width", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "snippet.thumbnails.standard.height" => Some(("snippet.thumbnails.standard.height", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "snippet.thumbnails.standard.url" => Some(("snippet.thumbnails.standard.url", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "snippet.thumbnails.standard.width" => Some(("snippet.thumbnails.standard.width", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "snippet.title" => Some(("snippet.title", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "statistics.comment-count" => Some(("statistics.commentCount", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "statistics.dislike-count" => Some(("statistics.dislikeCount", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "statistics.favorite-count" => Some(("statistics.favoriteCount", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "statistics.like-count" => Some(("statistics.likeCount", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "statistics.view-count" => Some(("statistics.viewCount", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "status.embeddable" => Some(("status.embeddable", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "status.failure-reason" => Some(("status.failureReason", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "status.license" => Some(("status.license", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "status.made-for-kids" => Some(("status.madeForKids", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "status.privacy-status" => Some(("status.privacyStatus", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "status.public-stats-viewable" => Some(("status.publicStatsViewable", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "status.publish-at" => Some(("status.publishAt", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "status.rejection-reason" => Some(("status.rejectionReason", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "status.self-declared-made-for-kids" => Some(("status.selfDeclaredMadeForKids", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "status.upload-status" => Some(("status.uploadStatus", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "suggestions.editor-suggestions" => Some(("suggestions.editorSuggestions", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Vec })),
                    "suggestions.processing-errors" => Some(("suggestions.processingErrors", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Vec })),
                    "suggestions.processing-hints" => Some(("suggestions.processingHints", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Vec })),
                    "suggestions.processing-warnings" => Some(("suggestions.processingWarnings", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Vec })),
                    "topic-details.relevant-topic-ids" => Some(("topicDetails.relevantTopicIds", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Vec })),
                    "topic-details.topic-categories" => Some(("topicDetails.topicCategories", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Vec })),
                    "topic-details.topic-ids" => Some(("topicDetails.topicIds", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Vec })),
                    _ => {
                        let suggestion = FieldCursor::did_you_mean(key, &vec!["acb-rating", "access", "active-live-chat-id", "actual-end-time", "actual-start-time", "agcom-rating", "age-gating", "alcohol-content", "allowed", "altitude", "anatel-rating", "bbfc-rating", "bfvc-rating", "bitrate-bps", "blocked", "bmukk-rating", "caption", "category-id", "catv-rating", "catvfr-rating", "cbfc-rating", "ccc-rating", "cce-rating", "channel-id", "channel-title", "chfilm-rating", "chvrs-rating", "cicf-rating", "cna-rating", "cnc-rating", "comment-count", "concurrent-viewers", "container", "content-details", "content-rating", "country-restriction", "creation-time", "csa-rating", "cscf-rating", "czfilm-rating", "default", "default-audio-language", "default-language", "definition", "description", "dimension", "dislike-count", "djctq-rating", "djctq-rating-reasons", "duration", "duration-ms", "ecbmct-rating", "editor-suggestions", "editor-suggestions-availability", "eefilm-rating", "egfilm-rating", "eirin-rating", "embed-height", "embed-html", "embed-width", "embeddable", "etag", "exception", "failure-reason", "favorite-count", "fcbm-rating", "fco-rating", "file-details", "file-details-availability", "file-name", "file-size", "file-type", "fmoc-rating", "fpb-rating", "fpb-rating-reasons", "fsk-rating", "grfilm-rating", "has-custom-thumbnail", "height", "high", "icaa-rating", "id", "ifco-rating", "ilfilm-rating", "incaa-rating", "kfcb-rating", "kijkwijzer-rating", "kind", "kmrb-rating", "latitude", "license", "licensed-content", "like-count", "live-broadcast-content", "live-streaming-details", "localized", "location", "location-description", "longitude", "lsf-rating", "made-for-kids", "maxres", "mccaa-rating", "mccyp-rating", "mcst-rating", "mda-rating", "medietilsynet-rating", "medium", "meku-rating", "mena-mpaa-rating", "mibac-rating", "moc-rating", "moctw-rating", "monetization-details", "mpaa-rating", "mpaat-rating", "mtrcb-rating", "nbc-rating", "nbcpl-rating", "nfrc-rating", "nfvcb-rating", "nkclv-rating", "nmc-rating", "oflc-rating", "parts-processed", "parts-total", "pefilm-rating", "player", "privacy-status", "processing-details", "processing-errors", "processing-failure-reason", "processing-hints", "processing-issues-availability", "processing-progress", "processing-status", "processing-warnings", "projection", "public-stats-viewable", "publish-at", "published-at", "rcnof-rating", "recording-date", "recording-details", "region-restriction", "rejection-reason", "relevant-topic-ids", "resorteviolencia-rating", "restricted", "rtc-rating", "rte-rating", "russia-rating", "scheduled-end-time", "scheduled-start-time", "self-declared-made-for-kids", "skfilm-rating", "smais-rating", "smsa-rating", "snippet", "standard", "statistics", "status", "suggestions", "tag-suggestions-availability", "tags", "thumbnails", "thumbnails-availability", "time-left-ms", "title", "topic-categories", "topic-details", "topic-ids", "tvpg-rating", "upload-status", "url", "video-game-rating", "view-count", "width", "yt-rating"]);
                        err.issues.push(CLIError::Field(FieldError::Unknown(temp_cursor.to_string(), suggestion, value.map(|v| v.to_string()))));
                        None
                    }
                };
            if let Some((field_cursor_str, type_info)) = type_info {
                FieldCursor::from(field_cursor_str).set_json_value(&mut object, value.unwrap(), type_info, err, &temp_cursor);
            }
        }
        let mut request: api::Video = json::value::from_value(object).unwrap();
        let mut call = self.hub.videos().update(request);
        for parg in opt.values_of("v").map(|i|i.collect()).unwrap_or(Vec::new()).iter() {
            let (key, value) = parse_kv_arg(&*parg, err, false);
            match key {
                "on-behalf-of-content-owner" => {
                    call = call.on_behalf_of_content_owner(value.unwrap_or(""));
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
                                                                           v.extend(["on-behalf-of-content-owner"].iter().map(|v|*v));
                                                                           v } ));
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

    async fn _watermarks_set(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
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
                    "image-bytes" => Some(("imageBytes", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "image-url" => Some(("imageUrl", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "position.corner-position" => Some(("position.cornerPosition", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "position.type" => Some(("position.type", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "target-channel-id" => Some(("targetChannelId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "timing.duration-ms" => Some(("timing.durationMs", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "timing.offset-ms" => Some(("timing.offsetMs", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "timing.type" => Some(("timing.type", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    _ => {
                        let suggestion = FieldCursor::did_you_mean(key, &vec!["corner-position", "duration-ms", "image-bytes", "image-url", "offset-ms", "position", "target-channel-id", "timing", "type"]);
                        err.issues.push(CLIError::Field(FieldError::Unknown(temp_cursor.to_string(), suggestion, value.map(|v| v.to_string()))));
                        None
                    }
                };
            if let Some((field_cursor_str, type_info)) = type_info {
                FieldCursor::from(field_cursor_str).set_json_value(&mut object, value.unwrap(), type_info, err, &temp_cursor);
            }
        }
        let mut request: api::InvideoBranding = json::value::from_value(object).unwrap();
        let mut call = self.hub.watermarks().set(request, opt.value_of("channel-id").unwrap_or(""));
        for parg in opt.values_of("v").map(|i|i.collect()).unwrap_or(Vec::new()).iter() {
            let (key, value) = parse_kv_arg(&*parg, err, false);
            match key {
                "on-behalf-of-content-owner" => {
                    call = call.on_behalf_of_content_owner(value.unwrap_or(""));
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
                                                                           v.extend(["on-behalf-of-content-owner"].iter().map(|v|*v));
                                                                           v } ));
                    }
                }
            }
        }
        let vals = opt.values_of("mode").unwrap().collect::<Vec<&str>>();
        let protocol = calltype_from_str(vals[0], ["simple"].iter().map(|&v| v.to_string()).collect(), err);
        let mut input_file = input_file_from_opts(vals[1], err);
        let mime_type = input_mime_from_opts(opt.value_of("mime").unwrap_or("application/octet-stream"), err);
        if dry_run {
            Ok(())
        } else {
            assert!(err.issues.len() == 0);
            for scope in self.opt.values_of("url").map(|i|i.collect()).unwrap_or(Vec::new()).iter() {
                call = call.add_scope(scope);
            }
            match match protocol {
                CallType::Upload(UploadProtocol::Simple) => call.upload(input_file.unwrap(), mime_type.unwrap()).await,
                CallType::Standard => unreachable!()
            } {
                Err(api_err) => Err(DoitError::ApiError(api_err)),
                Ok(mut response) => {
                    Ok(())
                }
            }
        }
    }

    async fn _watermarks_unset(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        let mut call = self.hub.watermarks().unset(opt.value_of("channel-id").unwrap_or(""));
        for parg in opt.values_of("v").map(|i|i.collect()).unwrap_or(Vec::new()).iter() {
            let (key, value) = parse_kv_arg(&*parg, err, false);
            match key {
                "on-behalf-of-content-owner" => {
                    call = call.on_behalf_of_content_owner(value.unwrap_or(""));
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
                                                                           v.extend(["on-behalf-of-content-owner"].iter().map(|v|*v));
                                                                           v } ));
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
            match match protocol {
                CallType::Standard => call.doit().await,
                _ => unreachable!()
            } {
                Err(api_err) => Err(DoitError::ApiError(api_err)),
                Ok(mut response) => {
                    Ok(())
                }
            }
        }
    }

    async fn _youtube_v3_update_comment_threads(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
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
                    "etag" => Some(("etag", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "id" => Some(("id", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "kind" => Some(("kind", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "snippet.can-reply" => Some(("snippet.canReply", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "snippet.channel-id" => Some(("snippet.channelId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "snippet.is-public" => Some(("snippet.isPublic", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "snippet.top-level-comment.etag" => Some(("snippet.topLevelComment.etag", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "snippet.top-level-comment.id" => Some(("snippet.topLevelComment.id", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "snippet.top-level-comment.kind" => Some(("snippet.topLevelComment.kind", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "snippet.top-level-comment.snippet.author-channel-id.value" => Some(("snippet.topLevelComment.snippet.authorChannelId.value", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "snippet.top-level-comment.snippet.author-channel-url" => Some(("snippet.topLevelComment.snippet.authorChannelUrl", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "snippet.top-level-comment.snippet.author-display-name" => Some(("snippet.topLevelComment.snippet.authorDisplayName", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "snippet.top-level-comment.snippet.author-profile-image-url" => Some(("snippet.topLevelComment.snippet.authorProfileImageUrl", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "snippet.top-level-comment.snippet.can-rate" => Some(("snippet.topLevelComment.snippet.canRate", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "snippet.top-level-comment.snippet.channel-id" => Some(("snippet.topLevelComment.snippet.channelId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "snippet.top-level-comment.snippet.like-count" => Some(("snippet.topLevelComment.snippet.likeCount", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "snippet.top-level-comment.snippet.moderation-status" => Some(("snippet.topLevelComment.snippet.moderationStatus", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "snippet.top-level-comment.snippet.parent-id" => Some(("snippet.topLevelComment.snippet.parentId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "snippet.top-level-comment.snippet.published-at" => Some(("snippet.topLevelComment.snippet.publishedAt", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "snippet.top-level-comment.snippet.text-display" => Some(("snippet.topLevelComment.snippet.textDisplay", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "snippet.top-level-comment.snippet.text-original" => Some(("snippet.topLevelComment.snippet.textOriginal", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "snippet.top-level-comment.snippet.updated-at" => Some(("snippet.topLevelComment.snippet.updatedAt", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "snippet.top-level-comment.snippet.video-id" => Some(("snippet.topLevelComment.snippet.videoId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "snippet.top-level-comment.snippet.viewer-rating" => Some(("snippet.topLevelComment.snippet.viewerRating", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "snippet.total-reply-count" => Some(("snippet.totalReplyCount", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "snippet.video-id" => Some(("snippet.videoId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    _ => {
                        let suggestion = FieldCursor::did_you_mean(key, &vec!["author-channel-id", "author-channel-url", "author-display-name", "author-profile-image-url", "can-rate", "can-reply", "channel-id", "etag", "id", "is-public", "kind", "like-count", "moderation-status", "parent-id", "published-at", "snippet", "text-display", "text-original", "top-level-comment", "total-reply-count", "updated-at", "value", "video-id", "viewer-rating"]);
                        err.issues.push(CLIError::Field(FieldError::Unknown(temp_cursor.to_string(), suggestion, value.map(|v| v.to_string()))));
                        None
                    }
                };
            if let Some((field_cursor_str, type_info)) = type_info {
                FieldCursor::from(field_cursor_str).set_json_value(&mut object, value.unwrap(), type_info, err, &temp_cursor);
            }
        }
        let mut request: api::CommentThread = json::value::from_value(object).unwrap();
        let mut call = self.hub.youtube().v3_update_comment_threads(request);
        for parg in opt.values_of("v").map(|i|i.collect()).unwrap_or(Vec::new()).iter() {
            let (key, value) = parse_kv_arg(&*parg, err, false);
            match key {
                "part" => {
                    call = call.add_part(value.unwrap_or(""));
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
                                                                           v.extend(["part"].iter().map(|v|*v));
                                                                           v } ));
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
            ("abuse-reports", Some(opt)) => {
                match opt.subcommand() {
                    ("insert", Some(opt)) => {
                        call_result = self._abuse_reports_insert(opt, dry_run, &mut err).await;
                    },
                    _ => {
                        err.issues.push(CLIError::MissingMethodError("abuse-reports".to_string()));
                        writeln!(io::stderr(), "{}\n", opt.usage()).ok();
                    }
                }
            },
            ("activities", Some(opt)) => {
                match opt.subcommand() {
                    ("list", Some(opt)) => {
                        call_result = self._activities_list(opt, dry_run, &mut err).await;
                    },
                    _ => {
                        err.issues.push(CLIError::MissingMethodError("activities".to_string()));
                        writeln!(io::stderr(), "{}\n", opt.usage()).ok();
                    }
                }
            },
            ("captions", Some(opt)) => {
                match opt.subcommand() {
                    ("delete", Some(opt)) => {
                        call_result = self._captions_delete(opt, dry_run, &mut err).await;
                    },
                    ("download", Some(opt)) => {
                        call_result = self._captions_download(opt, dry_run, &mut err).await;
                    },
                    ("insert", Some(opt)) => {
                        call_result = self._captions_insert(opt, dry_run, &mut err).await;
                    },
                    ("list", Some(opt)) => {
                        call_result = self._captions_list(opt, dry_run, &mut err).await;
                    },
                    ("update", Some(opt)) => {
                        call_result = self._captions_update(opt, dry_run, &mut err).await;
                    },
                    _ => {
                        err.issues.push(CLIError::MissingMethodError("captions".to_string()));
                        writeln!(io::stderr(), "{}\n", opt.usage()).ok();
                    }
                }
            },
            ("channel-banners", Some(opt)) => {
                match opt.subcommand() {
                    ("insert", Some(opt)) => {
                        call_result = self._channel_banners_insert(opt, dry_run, &mut err).await;
                    },
                    _ => {
                        err.issues.push(CLIError::MissingMethodError("channel-banners".to_string()));
                        writeln!(io::stderr(), "{}\n", opt.usage()).ok();
                    }
                }
            },
            ("channel-sections", Some(opt)) => {
                match opt.subcommand() {
                    ("delete", Some(opt)) => {
                        call_result = self._channel_sections_delete(opt, dry_run, &mut err).await;
                    },
                    ("insert", Some(opt)) => {
                        call_result = self._channel_sections_insert(opt, dry_run, &mut err).await;
                    },
                    ("list", Some(opt)) => {
                        call_result = self._channel_sections_list(opt, dry_run, &mut err).await;
                    },
                    ("update", Some(opt)) => {
                        call_result = self._channel_sections_update(opt, dry_run, &mut err).await;
                    },
                    _ => {
                        err.issues.push(CLIError::MissingMethodError("channel-sections".to_string()));
                        writeln!(io::stderr(), "{}\n", opt.usage()).ok();
                    }
                }
            },
            ("channels", Some(opt)) => {
                match opt.subcommand() {
                    ("list", Some(opt)) => {
                        call_result = self._channels_list(opt, dry_run, &mut err).await;
                    },
                    ("update", Some(opt)) => {
                        call_result = self._channels_update(opt, dry_run, &mut err).await;
                    },
                    _ => {
                        err.issues.push(CLIError::MissingMethodError("channels".to_string()));
                        writeln!(io::stderr(), "{}\n", opt.usage()).ok();
                    }
                }
            },
            ("comment-threads", Some(opt)) => {
                match opt.subcommand() {
                    ("insert", Some(opt)) => {
                        call_result = self._comment_threads_insert(opt, dry_run, &mut err).await;
                    },
                    ("list", Some(opt)) => {
                        call_result = self._comment_threads_list(opt, dry_run, &mut err).await;
                    },
                    _ => {
                        err.issues.push(CLIError::MissingMethodError("comment-threads".to_string()));
                        writeln!(io::stderr(), "{}\n", opt.usage()).ok();
                    }
                }
            },
            ("comments", Some(opt)) => {
                match opt.subcommand() {
                    ("delete", Some(opt)) => {
                        call_result = self._comments_delete(opt, dry_run, &mut err).await;
                    },
                    ("insert", Some(opt)) => {
                        call_result = self._comments_insert(opt, dry_run, &mut err).await;
                    },
                    ("list", Some(opt)) => {
                        call_result = self._comments_list(opt, dry_run, &mut err).await;
                    },
                    ("mark-as-spam", Some(opt)) => {
                        call_result = self._comments_mark_as_spam(opt, dry_run, &mut err).await;
                    },
                    ("set-moderation-status", Some(opt)) => {
                        call_result = self._comments_set_moderation_status(opt, dry_run, &mut err).await;
                    },
                    ("update", Some(opt)) => {
                        call_result = self._comments_update(opt, dry_run, &mut err).await;
                    },
                    _ => {
                        err.issues.push(CLIError::MissingMethodError("comments".to_string()));
                        writeln!(io::stderr(), "{}\n", opt.usage()).ok();
                    }
                }
            },
            ("i18n-languages", Some(opt)) => {
                match opt.subcommand() {
                    ("list", Some(opt)) => {
                        call_result = self._i18n_languages_list(opt, dry_run, &mut err).await;
                    },
                    _ => {
                        err.issues.push(CLIError::MissingMethodError("i18n-languages".to_string()));
                        writeln!(io::stderr(), "{}\n", opt.usage()).ok();
                    }
                }
            },
            ("i18n-regions", Some(opt)) => {
                match opt.subcommand() {
                    ("list", Some(opt)) => {
                        call_result = self._i18n_regions_list(opt, dry_run, &mut err).await;
                    },
                    _ => {
                        err.issues.push(CLIError::MissingMethodError("i18n-regions".to_string()));
                        writeln!(io::stderr(), "{}\n", opt.usage()).ok();
                    }
                }
            },
            ("live-broadcasts", Some(opt)) => {
                match opt.subcommand() {
                    ("bind", Some(opt)) => {
                        call_result = self._live_broadcasts_bind(opt, dry_run, &mut err).await;
                    },
                    ("delete", Some(opt)) => {
                        call_result = self._live_broadcasts_delete(opt, dry_run, &mut err).await;
                    },
                    ("insert", Some(opt)) => {
                        call_result = self._live_broadcasts_insert(opt, dry_run, &mut err).await;
                    },
                    ("insert-cuepoint", Some(opt)) => {
                        call_result = self._live_broadcasts_insert_cuepoint(opt, dry_run, &mut err).await;
                    },
                    ("list", Some(opt)) => {
                        call_result = self._live_broadcasts_list(opt, dry_run, &mut err).await;
                    },
                    ("transition", Some(opt)) => {
                        call_result = self._live_broadcasts_transition(opt, dry_run, &mut err).await;
                    },
                    ("update", Some(opt)) => {
                        call_result = self._live_broadcasts_update(opt, dry_run, &mut err).await;
                    },
                    _ => {
                        err.issues.push(CLIError::MissingMethodError("live-broadcasts".to_string()));
                        writeln!(io::stderr(), "{}\n", opt.usage()).ok();
                    }
                }
            },
            ("live-chat-bans", Some(opt)) => {
                match opt.subcommand() {
                    ("delete", Some(opt)) => {
                        call_result = self._live_chat_bans_delete(opt, dry_run, &mut err).await;
                    },
                    ("insert", Some(opt)) => {
                        call_result = self._live_chat_bans_insert(opt, dry_run, &mut err).await;
                    },
                    _ => {
                        err.issues.push(CLIError::MissingMethodError("live-chat-bans".to_string()));
                        writeln!(io::stderr(), "{}\n", opt.usage()).ok();
                    }
                }
            },
            ("live-chat-messages", Some(opt)) => {
                match opt.subcommand() {
                    ("delete", Some(opt)) => {
                        call_result = self._live_chat_messages_delete(opt, dry_run, &mut err).await;
                    },
                    ("insert", Some(opt)) => {
                        call_result = self._live_chat_messages_insert(opt, dry_run, &mut err).await;
                    },
                    ("list", Some(opt)) => {
                        call_result = self._live_chat_messages_list(opt, dry_run, &mut err).await;
                    },
                    ("transition", Some(opt)) => {
                        call_result = self._live_chat_messages_transition(opt, dry_run, &mut err).await;
                    },
                    _ => {
                        err.issues.push(CLIError::MissingMethodError("live-chat-messages".to_string()));
                        writeln!(io::stderr(), "{}\n", opt.usage()).ok();
                    }
                }
            },
            ("live-chat-moderators", Some(opt)) => {
                match opt.subcommand() {
                    ("delete", Some(opt)) => {
                        call_result = self._live_chat_moderators_delete(opt, dry_run, &mut err).await;
                    },
                    ("insert", Some(opt)) => {
                        call_result = self._live_chat_moderators_insert(opt, dry_run, &mut err).await;
                    },
                    ("list", Some(opt)) => {
                        call_result = self._live_chat_moderators_list(opt, dry_run, &mut err).await;
                    },
                    _ => {
                        err.issues.push(CLIError::MissingMethodError("live-chat-moderators".to_string()));
                        writeln!(io::stderr(), "{}\n", opt.usage()).ok();
                    }
                }
            },
            ("live-streams", Some(opt)) => {
                match opt.subcommand() {
                    ("delete", Some(opt)) => {
                        call_result = self._live_streams_delete(opt, dry_run, &mut err).await;
                    },
                    ("insert", Some(opt)) => {
                        call_result = self._live_streams_insert(opt, dry_run, &mut err).await;
                    },
                    ("list", Some(opt)) => {
                        call_result = self._live_streams_list(opt, dry_run, &mut err).await;
                    },
                    ("update", Some(opt)) => {
                        call_result = self._live_streams_update(opt, dry_run, &mut err).await;
                    },
                    _ => {
                        err.issues.push(CLIError::MissingMethodError("live-streams".to_string()));
                        writeln!(io::stderr(), "{}\n", opt.usage()).ok();
                    }
                }
            },
            ("members", Some(opt)) => {
                match opt.subcommand() {
                    ("list", Some(opt)) => {
                        call_result = self._members_list(opt, dry_run, &mut err).await;
                    },
                    _ => {
                        err.issues.push(CLIError::MissingMethodError("members".to_string()));
                        writeln!(io::stderr(), "{}\n", opt.usage()).ok();
                    }
                }
            },
            ("memberships-levels", Some(opt)) => {
                match opt.subcommand() {
                    ("list", Some(opt)) => {
                        call_result = self._memberships_levels_list(opt, dry_run, &mut err).await;
                    },
                    _ => {
                        err.issues.push(CLIError::MissingMethodError("memberships-levels".to_string()));
                        writeln!(io::stderr(), "{}\n", opt.usage()).ok();
                    }
                }
            },
            ("playlist-images", Some(opt)) => {
                match opt.subcommand() {
                    ("delete", Some(opt)) => {
                        call_result = self._playlist_images_delete(opt, dry_run, &mut err).await;
                    },
                    ("insert", Some(opt)) => {
                        call_result = self._playlist_images_insert(opt, dry_run, &mut err).await;
                    },
                    ("list", Some(opt)) => {
                        call_result = self._playlist_images_list(opt, dry_run, &mut err).await;
                    },
                    ("update", Some(opt)) => {
                        call_result = self._playlist_images_update(opt, dry_run, &mut err).await;
                    },
                    _ => {
                        err.issues.push(CLIError::MissingMethodError("playlist-images".to_string()));
                        writeln!(io::stderr(), "{}\n", opt.usage()).ok();
                    }
                }
            },
            ("playlist-items", Some(opt)) => {
                match opt.subcommand() {
                    ("delete", Some(opt)) => {
                        call_result = self._playlist_items_delete(opt, dry_run, &mut err).await;
                    },
                    ("insert", Some(opt)) => {
                        call_result = self._playlist_items_insert(opt, dry_run, &mut err).await;
                    },
                    ("list", Some(opt)) => {
                        call_result = self._playlist_items_list(opt, dry_run, &mut err).await;
                    },
                    ("update", Some(opt)) => {
                        call_result = self._playlist_items_update(opt, dry_run, &mut err).await;
                    },
                    _ => {
                        err.issues.push(CLIError::MissingMethodError("playlist-items".to_string()));
                        writeln!(io::stderr(), "{}\n", opt.usage()).ok();
                    }
                }
            },
            ("playlists", Some(opt)) => {
                match opt.subcommand() {
                    ("delete", Some(opt)) => {
                        call_result = self._playlists_delete(opt, dry_run, &mut err).await;
                    },
                    ("insert", Some(opt)) => {
                        call_result = self._playlists_insert(opt, dry_run, &mut err).await;
                    },
                    ("list", Some(opt)) => {
                        call_result = self._playlists_list(opt, dry_run, &mut err).await;
                    },
                    ("update", Some(opt)) => {
                        call_result = self._playlists_update(opt, dry_run, &mut err).await;
                    },
                    _ => {
                        err.issues.push(CLIError::MissingMethodError("playlists".to_string()));
                        writeln!(io::stderr(), "{}\n", opt.usage()).ok();
                    }
                }
            },
            ("search", Some(opt)) => {
                match opt.subcommand() {
                    ("list", Some(opt)) => {
                        call_result = self._search_list(opt, dry_run, &mut err).await;
                    },
                    _ => {
                        err.issues.push(CLIError::MissingMethodError("search".to_string()));
                        writeln!(io::stderr(), "{}\n", opt.usage()).ok();
                    }
                }
            },
            ("subscriptions", Some(opt)) => {
                match opt.subcommand() {
                    ("delete", Some(opt)) => {
                        call_result = self._subscriptions_delete(opt, dry_run, &mut err).await;
                    },
                    ("insert", Some(opt)) => {
                        call_result = self._subscriptions_insert(opt, dry_run, &mut err).await;
                    },
                    ("list", Some(opt)) => {
                        call_result = self._subscriptions_list(opt, dry_run, &mut err).await;
                    },
                    _ => {
                        err.issues.push(CLIError::MissingMethodError("subscriptions".to_string()));
                        writeln!(io::stderr(), "{}\n", opt.usage()).ok();
                    }
                }
            },
            ("super-chat-events", Some(opt)) => {
                match opt.subcommand() {
                    ("list", Some(opt)) => {
                        call_result = self._super_chat_events_list(opt, dry_run, &mut err).await;
                    },
                    _ => {
                        err.issues.push(CLIError::MissingMethodError("super-chat-events".to_string()));
                        writeln!(io::stderr(), "{}\n", opt.usage()).ok();
                    }
                }
            },
            ("tests", Some(opt)) => {
                match opt.subcommand() {
                    ("insert", Some(opt)) => {
                        call_result = self._tests_insert(opt, dry_run, &mut err).await;
                    },
                    _ => {
                        err.issues.push(CLIError::MissingMethodError("tests".to_string()));
                        writeln!(io::stderr(), "{}\n", opt.usage()).ok();
                    }
                }
            },
            ("third-party-links", Some(opt)) => {
                match opt.subcommand() {
                    ("delete", Some(opt)) => {
                        call_result = self._third_party_links_delete(opt, dry_run, &mut err).await;
                    },
                    ("insert", Some(opt)) => {
                        call_result = self._third_party_links_insert(opt, dry_run, &mut err).await;
                    },
                    ("list", Some(opt)) => {
                        call_result = self._third_party_links_list(opt, dry_run, &mut err).await;
                    },
                    ("update", Some(opt)) => {
                        call_result = self._third_party_links_update(opt, dry_run, &mut err).await;
                    },
                    _ => {
                        err.issues.push(CLIError::MissingMethodError("third-party-links".to_string()));
                        writeln!(io::stderr(), "{}\n", opt.usage()).ok();
                    }
                }
            },
            ("thumbnails", Some(opt)) => {
                match opt.subcommand() {
                    ("set", Some(opt)) => {
                        call_result = self._thumbnails_set(opt, dry_run, &mut err).await;
                    },
                    _ => {
                        err.issues.push(CLIError::MissingMethodError("thumbnails".to_string()));
                        writeln!(io::stderr(), "{}\n", opt.usage()).ok();
                    }
                }
            },
            ("video-abuse-report-reasons", Some(opt)) => {
                match opt.subcommand() {
                    ("list", Some(opt)) => {
                        call_result = self._video_abuse_report_reasons_list(opt, dry_run, &mut err).await;
                    },
                    _ => {
                        err.issues.push(CLIError::MissingMethodError("video-abuse-report-reasons".to_string()));
                        writeln!(io::stderr(), "{}\n", opt.usage()).ok();
                    }
                }
            },
            ("video-categories", Some(opt)) => {
                match opt.subcommand() {
                    ("list", Some(opt)) => {
                        call_result = self._video_categories_list(opt, dry_run, &mut err).await;
                    },
                    _ => {
                        err.issues.push(CLIError::MissingMethodError("video-categories".to_string()));
                        writeln!(io::stderr(), "{}\n", opt.usage()).ok();
                    }
                }
            },
            ("videos", Some(opt)) => {
                match opt.subcommand() {
                    ("delete", Some(opt)) => {
                        call_result = self._videos_delete(opt, dry_run, &mut err).await;
                    },
                    ("get-rating", Some(opt)) => {
                        call_result = self._videos_get_rating(opt, dry_run, &mut err).await;
                    },
                    ("insert", Some(opt)) => {
                        call_result = self._videos_insert(opt, dry_run, &mut err).await;
                    },
                    ("list", Some(opt)) => {
                        call_result = self._videos_list(opt, dry_run, &mut err).await;
                    },
                    ("rate", Some(opt)) => {
                        call_result = self._videos_rate(opt, dry_run, &mut err).await;
                    },
                    ("report-abuse", Some(opt)) => {
                        call_result = self._videos_report_abuse(opt, dry_run, &mut err).await;
                    },
                    ("update", Some(opt)) => {
                        call_result = self._videos_update(opt, dry_run, &mut err).await;
                    },
                    _ => {
                        err.issues.push(CLIError::MissingMethodError("videos".to_string()));
                        writeln!(io::stderr(), "{}\n", opt.usage()).ok();
                    }
                }
            },
            ("watermarks", Some(opt)) => {
                match opt.subcommand() {
                    ("set", Some(opt)) => {
                        call_result = self._watermarks_set(opt, dry_run, &mut err).await;
                    },
                    ("unset", Some(opt)) => {
                        call_result = self._watermarks_unset(opt, dry_run, &mut err).await;
                    },
                    _ => {
                        err.issues.push(CLIError::MissingMethodError("watermarks".to_string()));
                        writeln!(io::stderr(), "{}\n", opt.usage()).ok();
                    }
                }
            },
            ("youtube", Some(opt)) => {
                match opt.subcommand() {
                    ("v3-update-comment-threads", Some(opt)) => {
                        call_result = self._youtube_v3_update_comment_threads(opt, dry_run, &mut err).await;
                    },
                    _ => {
                        err.issues.push(CLIError::MissingMethodError("youtube".to_string()));
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

            match client::application_secret_from_directory(&config_dir, "youtube3-secret.json",
                                                         "{
  \"installed\": {
    \"auth_uri\": \"https://accounts.google.com/o/oauth2/auth\",
    \"client_secret\": \"UqkDJd5RFwnHoiG5x5Rub8SI\",
    \"token_uri\": \"https://accounts.google.com/o/oauth2/token\",
    \"client_email\": \"\",
    \"redirect_uris\": [
      \"urn:ietf:wg:oauth:2.0:oob\",
      \"oob\"
    ],
    \"client_x509_cert_url\": \"\",
    \"client_id\": \"14070749909-vgip2f1okm7bkvajhi9jugan6126io9v.apps.googleusercontent.com\",
    \"auth_provider_x509_cert_url\": \"https://www.googleapis.com/oauth2/v1/certs\"
  }
}") {
                Ok(secret) => (config_dir, secret),
                Err(e) => return Err(InvalidOptionsError::single(e, 4))
            }
        };

        let client = hyper::Client::builder().build(connector);

        let auth = oauth2::InstalledFlowAuthenticator::with_client(
            secret,
            oauth2::InstalledFlowReturnMethod::HTTPRedirect,
            client.clone(),
        ).persist_tokens_to_disk(format!("{}/youtube3", config_dir)).build().await.unwrap();

        let engine = Engine {
            opt: opt,
            hub: api::YouTube::new(client, auth),
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
    let upload_value_names = ["mode", "file"];
    let arg_data = [
        ("abuse-reports", "methods: 'insert'", vec![
            ("insert",
                    Some(r##"Inserts a new resource into this collection."##),
                    "Details at http://byron.github.io/google-apis-rs/google_youtube3_cli/abuse-reports_insert",
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
        
        ("activities", "methods: 'list'", vec![
            ("list",
                    Some(r##"Retrieves a list of resources, possibly filtered."##),
                    "Details at http://byron.github.io/google-apis-rs/google_youtube3_cli/activities_list",
                  vec![
                    (Some(r##"part"##),
                     None,
                     Some(r##"The *part* parameter specifies a comma-separated list of one or more activity resource properties that the API response will include. If the parameter identifies a property that contains child properties, the child properties will be included in the response. For example, in an activity resource, the snippet property contains other properties that identify the type of activity, a display title for the activity, and so forth. If you set *part=snippet*, the API response will also contain all of those nested properties."##),
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
        
        ("captions", "methods: 'delete', 'download', 'insert', 'list' and 'update'", vec![
            ("delete",
                    Some(r##"Deletes a resource."##),
                    "Details at http://byron.github.io/google-apis-rs/google_youtube3_cli/captions_delete",
                  vec![
                    (Some(r##"id"##),
                     None,
                     None,
                     Some(true),
                     Some(false)),
        
                    (Some(r##"v"##),
                     Some(r##"p"##),
                     Some(r##"Set various optional parameters, matching the key=value form"##),
                     Some(false),
                     Some(true)),
                  ]),
            ("download",
                    Some(r##"Downloads a caption track."##),
                    "Details at http://byron.github.io/google-apis-rs/google_youtube3_cli/captions_download",
                  vec![
                    (Some(r##"id"##),
                     None,
                     Some(r##"The ID of the caption track to download, required for One Platform."##),
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
                    Some(r##"Inserts a new resource into this collection."##),
                    "Details at http://byron.github.io/google-apis-rs/google_youtube3_cli/captions_insert",
                  vec![
                    (Some(r##"kv"##),
                     Some(r##"r"##),
                     Some(r##"Set various fields of the request structure, matching the key=value form"##),
                     Some(true),
                     Some(true)),
        
                    (Some(r##"mode"##),
                     Some(r##"u"##),
                     Some(r##"Specify the upload protocol (simple) and the file to upload"##),
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
                    Some(r##"Retrieves a list of resources, possibly filtered."##),
                    "Details at http://byron.github.io/google-apis-rs/google_youtube3_cli/captions_list",
                  vec![
                    (Some(r##"part"##),
                     None,
                     Some(r##"The *part* parameter specifies a comma-separated list of one or more caption resource parts that the API response will include. The part names that you can include in the parameter value are id and snippet."##),
                     Some(true),
                     Some(false)),
        
                    (Some(r##"video-id"##),
                     None,
                     Some(r##"Returns the captions for the specified video."##),
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
            ("update",
                    Some(r##"Updates an existing resource."##),
                    "Details at http://byron.github.io/google-apis-rs/google_youtube3_cli/captions_update",
                  vec![
                    (Some(r##"kv"##),
                     Some(r##"r"##),
                     Some(r##"Set various fields of the request structure, matching the key=value form"##),
                     Some(true),
                     Some(true)),
        
                    (Some(r##"mode"##),
                     Some(r##"u"##),
                     Some(r##"Specify the upload protocol (simple) and the file to upload"##),
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
        
        ("channel-banners", "methods: 'insert'", vec![
            ("insert",
                    Some(r##"Inserts a new resource into this collection."##),
                    "Details at http://byron.github.io/google-apis-rs/google_youtube3_cli/channel-banners_insert",
                  vec![
                    (Some(r##"kv"##),
                     Some(r##"r"##),
                     Some(r##"Set various fields of the request structure, matching the key=value form"##),
                     Some(true),
                     Some(true)),
        
                    (Some(r##"mode"##),
                     Some(r##"u"##),
                     Some(r##"Specify the upload protocol (simple) and the file to upload"##),
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
        
        ("channel-sections", "methods: 'delete', 'insert', 'list' and 'update'", vec![
            ("delete",
                    Some(r##"Deletes a resource."##),
                    "Details at http://byron.github.io/google-apis-rs/google_youtube3_cli/channel-sections_delete",
                  vec![
                    (Some(r##"id"##),
                     None,
                     None,
                     Some(true),
                     Some(false)),
        
                    (Some(r##"v"##),
                     Some(r##"p"##),
                     Some(r##"Set various optional parameters, matching the key=value form"##),
                     Some(false),
                     Some(true)),
                  ]),
            ("insert",
                    Some(r##"Inserts a new resource into this collection."##),
                    "Details at http://byron.github.io/google-apis-rs/google_youtube3_cli/channel-sections_insert",
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
            ("list",
                    Some(r##"Retrieves a list of resources, possibly filtered."##),
                    "Details at http://byron.github.io/google-apis-rs/google_youtube3_cli/channel-sections_list",
                  vec![
                    (Some(r##"part"##),
                     None,
                     Some(r##"The *part* parameter specifies a comma-separated list of one or more channelSection resource properties that the API response will include. The part names that you can include in the parameter value are id, snippet, and contentDetails. If the parameter identifies a property that contains child properties, the child properties will be included in the response. For example, in a channelSection resource, the snippet property contains other properties, such as a display title for the channelSection. If you set *part=snippet*, the API response will also contain all of those nested properties."##),
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
            ("update",
                    Some(r##"Updates an existing resource."##),
                    "Details at http://byron.github.io/google-apis-rs/google_youtube3_cli/channel-sections_update",
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
        
        ("channels", "methods: 'list' and 'update'", vec![
            ("list",
                    Some(r##"Retrieves a list of resources, possibly filtered."##),
                    "Details at http://byron.github.io/google-apis-rs/google_youtube3_cli/channels_list",
                  vec![
                    (Some(r##"part"##),
                     None,
                     Some(r##"The *part* parameter specifies a comma-separated list of one or more channel resource properties that the API response will include. If the parameter identifies a property that contains child properties, the child properties will be included in the response. For example, in a channel resource, the contentDetails property contains other properties, such as the uploads properties. As such, if you set *part=contentDetails*, the API response will also contain all of those nested properties."##),
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
            ("update",
                    Some(r##"Updates an existing resource."##),
                    "Details at http://byron.github.io/google-apis-rs/google_youtube3_cli/channels_update",
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
        
        ("comment-threads", "methods: 'insert' and 'list'", vec![
            ("insert",
                    Some(r##"Inserts a new resource into this collection."##),
                    "Details at http://byron.github.io/google-apis-rs/google_youtube3_cli/comment-threads_insert",
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
            ("list",
                    Some(r##"Retrieves a list of resources, possibly filtered."##),
                    "Details at http://byron.github.io/google-apis-rs/google_youtube3_cli/comment-threads_list",
                  vec![
                    (Some(r##"part"##),
                     None,
                     Some(r##"The *part* parameter specifies a comma-separated list of one or more commentThread resource properties that the API response will include."##),
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
        
        ("comments", "methods: 'delete', 'insert', 'list', 'mark-as-spam', 'set-moderation-status' and 'update'", vec![
            ("delete",
                    Some(r##"Deletes a resource."##),
                    "Details at http://byron.github.io/google-apis-rs/google_youtube3_cli/comments_delete",
                  vec![
                    (Some(r##"id"##),
                     None,
                     None,
                     Some(true),
                     Some(false)),
        
                    (Some(r##"v"##),
                     Some(r##"p"##),
                     Some(r##"Set various optional parameters, matching the key=value form"##),
                     Some(false),
                     Some(true)),
                  ]),
            ("insert",
                    Some(r##"Inserts a new resource into this collection."##),
                    "Details at http://byron.github.io/google-apis-rs/google_youtube3_cli/comments_insert",
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
            ("list",
                    Some(r##"Retrieves a list of resources, possibly filtered."##),
                    "Details at http://byron.github.io/google-apis-rs/google_youtube3_cli/comments_list",
                  vec![
                    (Some(r##"part"##),
                     None,
                     Some(r##"The *part* parameter specifies a comma-separated list of one or more comment resource properties that the API response will include."##),
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
            ("mark-as-spam",
                    Some(r##"Expresses the caller's opinion that one or more comments should be flagged as spam."##),
                    "Details at http://byron.github.io/google-apis-rs/google_youtube3_cli/comments_mark-as-spam",
                  vec![
                    (Some(r##"id"##),
                     None,
                     Some(r##"Flags the comments with the given IDs as spam in the caller's opinion."##),
                     Some(true),
                     Some(false)),
        
                    (Some(r##"v"##),
                     Some(r##"p"##),
                     Some(r##"Set various optional parameters, matching the key=value form"##),
                     Some(false),
                     Some(true)),
                  ]),
            ("set-moderation-status",
                    Some(r##"Sets the moderation status of one or more comments."##),
                    "Details at http://byron.github.io/google-apis-rs/google_youtube3_cli/comments_set-moderation-status",
                  vec![
                    (Some(r##"id"##),
                     None,
                     Some(r##"Modifies the moderation status of the comments with the given IDs"##),
                     Some(true),
                     Some(false)),
        
                    (Some(r##"moderation-status"##),
                     None,
                     Some(r##"Specifies the requested moderation status. Note, comments can be in statuses, which are not available through this call. For example, this call does not allow to mark a comment as 'likely spam'. Valid values: 'heldForReview', 'published' or 'rejected'."##),
                     Some(true),
                     Some(false)),
        
                    (Some(r##"v"##),
                     Some(r##"p"##),
                     Some(r##"Set various optional parameters, matching the key=value form"##),
                     Some(false),
                     Some(true)),
                  ]),
            ("update",
                    Some(r##"Updates an existing resource."##),
                    "Details at http://byron.github.io/google-apis-rs/google_youtube3_cli/comments_update",
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
        
        ("i18n-languages", "methods: 'list'", vec![
            ("list",
                    Some(r##"Retrieves a list of resources, possibly filtered."##),
                    "Details at http://byron.github.io/google-apis-rs/google_youtube3_cli/i18n-languages_list",
                  vec![
                    (Some(r##"part"##),
                     None,
                     Some(r##"The *part* parameter specifies the i18nLanguage resource properties that the API response will include. Set the parameter value to snippet."##),
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
        
        ("i18n-regions", "methods: 'list'", vec![
            ("list",
                    Some(r##"Retrieves a list of resources, possibly filtered."##),
                    "Details at http://byron.github.io/google-apis-rs/google_youtube3_cli/i18n-regions_list",
                  vec![
                    (Some(r##"part"##),
                     None,
                     Some(r##"The *part* parameter specifies the i18nRegion resource properties that the API response will include. Set the parameter value to snippet."##),
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
        
        ("live-broadcasts", "methods: 'bind', 'delete', 'insert', 'insert-cuepoint', 'list', 'transition' and 'update'", vec![
            ("bind",
                    Some(r##"Bind a broadcast to a stream."##),
                    "Details at http://byron.github.io/google-apis-rs/google_youtube3_cli/live-broadcasts_bind",
                  vec![
                    (Some(r##"id"##),
                     None,
                     Some(r##"Broadcast to bind to the stream"##),
                     Some(true),
                     Some(false)),
        
                    (Some(r##"part"##),
                     None,
                     Some(r##"The *part* parameter specifies a comma-separated list of one or more liveBroadcast resource properties that the API response will include. The part names that you can include in the parameter value are id, snippet, contentDetails, and status."##),
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
                    Some(r##"Delete a given broadcast."##),
                    "Details at http://byron.github.io/google-apis-rs/google_youtube3_cli/live-broadcasts_delete",
                  vec![
                    (Some(r##"id"##),
                     None,
                     Some(r##"Broadcast to delete."##),
                     Some(true),
                     Some(false)),
        
                    (Some(r##"v"##),
                     Some(r##"p"##),
                     Some(r##"Set various optional parameters, matching the key=value form"##),
                     Some(false),
                     Some(true)),
                  ]),
            ("insert",
                    Some(r##"Inserts a new stream for the authenticated user."##),
                    "Details at http://byron.github.io/google-apis-rs/google_youtube3_cli/live-broadcasts_insert",
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
            ("insert-cuepoint",
                    Some(r##"Insert cuepoints in a broadcast"##),
                    "Details at http://byron.github.io/google-apis-rs/google_youtube3_cli/live-broadcasts_insert-cuepoint",
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
            ("list",
                    Some(r##"Retrieve the list of broadcasts associated with the given channel."##),
                    "Details at http://byron.github.io/google-apis-rs/google_youtube3_cli/live-broadcasts_list",
                  vec![
                    (Some(r##"part"##),
                     None,
                     Some(r##"The *part* parameter specifies a comma-separated list of one or more liveBroadcast resource properties that the API response will include. The part names that you can include in the parameter value are id, snippet, contentDetails, status and statistics."##),
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
            ("transition",
                    Some(r##"Transition a broadcast to a given status."##),
                    "Details at http://byron.github.io/google-apis-rs/google_youtube3_cli/live-broadcasts_transition",
                  vec![
                    (Some(r##"broadcast-status"##),
                     None,
                     Some(r##"The status to which the broadcast is going to transition."##),
                     Some(true),
                     Some(false)),
        
                    (Some(r##"id"##),
                     None,
                     Some(r##"Broadcast to transition."##),
                     Some(true),
                     Some(false)),
        
                    (Some(r##"part"##),
                     None,
                     Some(r##"The *part* parameter specifies a comma-separated list of one or more liveBroadcast resource properties that the API response will include. The part names that you can include in the parameter value are id, snippet, contentDetails, and status."##),
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
            ("update",
                    Some(r##"Updates an existing broadcast for the authenticated user."##),
                    "Details at http://byron.github.io/google-apis-rs/google_youtube3_cli/live-broadcasts_update",
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
        
        ("live-chat-bans", "methods: 'delete' and 'insert'", vec![
            ("delete",
                    Some(r##"Deletes a chat ban."##),
                    "Details at http://byron.github.io/google-apis-rs/google_youtube3_cli/live-chat-bans_delete",
                  vec![
                    (Some(r##"id"##),
                     None,
                     None,
                     Some(true),
                     Some(false)),
        
                    (Some(r##"v"##),
                     Some(r##"p"##),
                     Some(r##"Set various optional parameters, matching the key=value form"##),
                     Some(false),
                     Some(true)),
                  ]),
            ("insert",
                    Some(r##"Inserts a new resource into this collection."##),
                    "Details at http://byron.github.io/google-apis-rs/google_youtube3_cli/live-chat-bans_insert",
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
        
        ("live-chat-messages", "methods: 'delete', 'insert', 'list' and 'transition'", vec![
            ("delete",
                    Some(r##"Deletes a chat message."##),
                    "Details at http://byron.github.io/google-apis-rs/google_youtube3_cli/live-chat-messages_delete",
                  vec![
                    (Some(r##"id"##),
                     None,
                     None,
                     Some(true),
                     Some(false)),
        
                    (Some(r##"v"##),
                     Some(r##"p"##),
                     Some(r##"Set various optional parameters, matching the key=value form"##),
                     Some(false),
                     Some(true)),
                  ]),
            ("insert",
                    Some(r##"Inserts a new resource into this collection."##),
                    "Details at http://byron.github.io/google-apis-rs/google_youtube3_cli/live-chat-messages_insert",
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
            ("list",
                    Some(r##"Retrieves a list of resources, possibly filtered."##),
                    "Details at http://byron.github.io/google-apis-rs/google_youtube3_cli/live-chat-messages_list",
                  vec![
                    (Some(r##"live-chat-id"##),
                     None,
                     Some(r##"The id of the live chat for which comments should be returned."##),
                     Some(true),
                     Some(false)),
        
                    (Some(r##"part"##),
                     None,
                     Some(r##"The *part* parameter specifies the liveChatComment resource parts that the API response will include. Supported values are id and snippet."##),
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
            ("transition",
                    Some(r##"Transition a durable chat event."##),
                    "Details at http://byron.github.io/google-apis-rs/google_youtube3_cli/live-chat-messages_transition",
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
        
        ("live-chat-moderators", "methods: 'delete', 'insert' and 'list'", vec![
            ("delete",
                    Some(r##"Deletes a chat moderator."##),
                    "Details at http://byron.github.io/google-apis-rs/google_youtube3_cli/live-chat-moderators_delete",
                  vec![
                    (Some(r##"id"##),
                     None,
                     None,
                     Some(true),
                     Some(false)),
        
                    (Some(r##"v"##),
                     Some(r##"p"##),
                     Some(r##"Set various optional parameters, matching the key=value form"##),
                     Some(false),
                     Some(true)),
                  ]),
            ("insert",
                    Some(r##"Inserts a new resource into this collection."##),
                    "Details at http://byron.github.io/google-apis-rs/google_youtube3_cli/live-chat-moderators_insert",
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
            ("list",
                    Some(r##"Retrieves a list of resources, possibly filtered."##),
                    "Details at http://byron.github.io/google-apis-rs/google_youtube3_cli/live-chat-moderators_list",
                  vec![
                    (Some(r##"live-chat-id"##),
                     None,
                     Some(r##"The id of the live chat for which moderators should be returned."##),
                     Some(true),
                     Some(false)),
        
                    (Some(r##"part"##),
                     None,
                     Some(r##"The *part* parameter specifies the liveChatModerator resource parts that the API response will include. Supported values are id and snippet."##),
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
        
        ("live-streams", "methods: 'delete', 'insert', 'list' and 'update'", vec![
            ("delete",
                    Some(r##"Deletes an existing stream for the authenticated user."##),
                    "Details at http://byron.github.io/google-apis-rs/google_youtube3_cli/live-streams_delete",
                  vec![
                    (Some(r##"id"##),
                     None,
                     None,
                     Some(true),
                     Some(false)),
        
                    (Some(r##"v"##),
                     Some(r##"p"##),
                     Some(r##"Set various optional parameters, matching the key=value form"##),
                     Some(false),
                     Some(true)),
                  ]),
            ("insert",
                    Some(r##"Inserts a new stream for the authenticated user."##),
                    "Details at http://byron.github.io/google-apis-rs/google_youtube3_cli/live-streams_insert",
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
            ("list",
                    Some(r##"Retrieve the list of streams associated with the given channel. --"##),
                    "Details at http://byron.github.io/google-apis-rs/google_youtube3_cli/live-streams_list",
                  vec![
                    (Some(r##"part"##),
                     None,
                     Some(r##"The *part* parameter specifies a comma-separated list of one or more liveStream resource properties that the API response will include. The part names that you can include in the parameter value are id, snippet, cdn, and status."##),
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
            ("update",
                    Some(r##"Updates an existing stream for the authenticated user."##),
                    "Details at http://byron.github.io/google-apis-rs/google_youtube3_cli/live-streams_update",
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
        
        ("members", "methods: 'list'", vec![
            ("list",
                    Some(r##"Retrieves a list of members that match the request criteria for a channel."##),
                    "Details at http://byron.github.io/google-apis-rs/google_youtube3_cli/members_list",
                  vec![
                    (Some(r##"part"##),
                     None,
                     Some(r##"The *part* parameter specifies the member resource parts that the API response will include. Set the parameter value to snippet."##),
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
        
        ("memberships-levels", "methods: 'list'", vec![
            ("list",
                    Some(r##"Retrieves a list of all pricing levels offered by a creator to the fans."##),
                    "Details at http://byron.github.io/google-apis-rs/google_youtube3_cli/memberships-levels_list",
                  vec![
                    (Some(r##"part"##),
                     None,
                     Some(r##"The *part* parameter specifies the membershipsLevel resource parts that the API response will include. Supported values are id and snippet."##),
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
        
        ("playlist-images", "methods: 'delete', 'insert', 'list' and 'update'", vec![
            ("delete",
                    Some(r##"Deletes a resource."##),
                    "Details at http://byron.github.io/google-apis-rs/google_youtube3_cli/playlist-images_delete",
                  vec![
                    (Some(r##"v"##),
                     Some(r##"p"##),
                     Some(r##"Set various optional parameters, matching the key=value form"##),
                     Some(false),
                     Some(true)),
                  ]),
            ("insert",
                    Some(r##"Inserts a new resource into this collection."##),
                    "Details at http://byron.github.io/google-apis-rs/google_youtube3_cli/playlist-images_insert",
                  vec![
                    (Some(r##"kv"##),
                     Some(r##"r"##),
                     Some(r##"Set various fields of the request structure, matching the key=value form"##),
                     Some(true),
                     Some(true)),
        
                    (Some(r##"mode"##),
                     Some(r##"u"##),
                     Some(r##"Specify the upload protocol (simple) and the file to upload"##),
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
                    Some(r##"Retrieves a list of resources, possibly filtered."##),
                    "Details at http://byron.github.io/google-apis-rs/google_youtube3_cli/playlist-images_list",
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
            ("update",
                    Some(r##"Updates an existing resource."##),
                    "Details at http://byron.github.io/google-apis-rs/google_youtube3_cli/playlist-images_update",
                  vec![
                    (Some(r##"kv"##),
                     Some(r##"r"##),
                     Some(r##"Set various fields of the request structure, matching the key=value form"##),
                     Some(true),
                     Some(true)),
        
                    (Some(r##"mode"##),
                     Some(r##"u"##),
                     Some(r##"Specify the upload protocol (simple) and the file to upload"##),
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
        
        ("playlist-items", "methods: 'delete', 'insert', 'list' and 'update'", vec![
            ("delete",
                    Some(r##"Deletes a resource."##),
                    "Details at http://byron.github.io/google-apis-rs/google_youtube3_cli/playlist-items_delete",
                  vec![
                    (Some(r##"id"##),
                     None,
                     None,
                     Some(true),
                     Some(false)),
        
                    (Some(r##"v"##),
                     Some(r##"p"##),
                     Some(r##"Set various optional parameters, matching the key=value form"##),
                     Some(false),
                     Some(true)),
                  ]),
            ("insert",
                    Some(r##"Inserts a new resource into this collection."##),
                    "Details at http://byron.github.io/google-apis-rs/google_youtube3_cli/playlist-items_insert",
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
            ("list",
                    Some(r##"Retrieves a list of resources, possibly filtered."##),
                    "Details at http://byron.github.io/google-apis-rs/google_youtube3_cli/playlist-items_list",
                  vec![
                    (Some(r##"part"##),
                     None,
                     Some(r##"The *part* parameter specifies a comma-separated list of one or more playlistItem resource properties that the API response will include. If the parameter identifies a property that contains child properties, the child properties will be included in the response. For example, in a playlistItem resource, the snippet property contains numerous fields, including the title, description, position, and resourceId properties. As such, if you set *part=snippet*, the API response will contain all of those properties."##),
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
            ("update",
                    Some(r##"Updates an existing resource."##),
                    "Details at http://byron.github.io/google-apis-rs/google_youtube3_cli/playlist-items_update",
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
        
        ("playlists", "methods: 'delete', 'insert', 'list' and 'update'", vec![
            ("delete",
                    Some(r##"Deletes a resource."##),
                    "Details at http://byron.github.io/google-apis-rs/google_youtube3_cli/playlists_delete",
                  vec![
                    (Some(r##"id"##),
                     None,
                     None,
                     Some(true),
                     Some(false)),
        
                    (Some(r##"v"##),
                     Some(r##"p"##),
                     Some(r##"Set various optional parameters, matching the key=value form"##),
                     Some(false),
                     Some(true)),
                  ]),
            ("insert",
                    Some(r##"Inserts a new resource into this collection."##),
                    "Details at http://byron.github.io/google-apis-rs/google_youtube3_cli/playlists_insert",
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
            ("list",
                    Some(r##"Retrieves a list of resources, possibly filtered."##),
                    "Details at http://byron.github.io/google-apis-rs/google_youtube3_cli/playlists_list",
                  vec![
                    (Some(r##"part"##),
                     None,
                     Some(r##"The *part* parameter specifies a comma-separated list of one or more playlist resource properties that the API response will include. If the parameter identifies a property that contains child properties, the child properties will be included in the response. For example, in a playlist resource, the snippet property contains properties like author, title, description, tags, and timeCreated. As such, if you set *part=snippet*, the API response will contain all of those properties."##),
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
            ("update",
                    Some(r##"Updates an existing resource."##),
                    "Details at http://byron.github.io/google-apis-rs/google_youtube3_cli/playlists_update",
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
        
        ("search", "methods: 'list'", vec![
            ("list",
                    Some(r##"Retrieves a list of search resources"##),
                    "Details at http://byron.github.io/google-apis-rs/google_youtube3_cli/search_list",
                  vec![
                    (Some(r##"part"##),
                     None,
                     Some(r##"The *part* parameter specifies a comma-separated list of one or more search resource properties that the API response will include. Set the parameter value to snippet."##),
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
        
        ("subscriptions", "methods: 'delete', 'insert' and 'list'", vec![
            ("delete",
                    Some(r##"Deletes a resource."##),
                    "Details at http://byron.github.io/google-apis-rs/google_youtube3_cli/subscriptions_delete",
                  vec![
                    (Some(r##"id"##),
                     None,
                     None,
                     Some(true),
                     Some(false)),
        
                    (Some(r##"v"##),
                     Some(r##"p"##),
                     Some(r##"Set various optional parameters, matching the key=value form"##),
                     Some(false),
                     Some(true)),
                  ]),
            ("insert",
                    Some(r##"Inserts a new resource into this collection."##),
                    "Details at http://byron.github.io/google-apis-rs/google_youtube3_cli/subscriptions_insert",
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
            ("list",
                    Some(r##"Retrieves a list of resources, possibly filtered."##),
                    "Details at http://byron.github.io/google-apis-rs/google_youtube3_cli/subscriptions_list",
                  vec![
                    (Some(r##"part"##),
                     None,
                     Some(r##"The *part* parameter specifies a comma-separated list of one or more subscription resource properties that the API response will include. If the parameter identifies a property that contains child properties, the child properties will be included in the response. For example, in a subscription resource, the snippet property contains other properties, such as a display title for the subscription. If you set *part=snippet*, the API response will also contain all of those nested properties."##),
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
        
        ("super-chat-events", "methods: 'list'", vec![
            ("list",
                    Some(r##"Retrieves a list of resources, possibly filtered."##),
                    "Details at http://byron.github.io/google-apis-rs/google_youtube3_cli/super-chat-events_list",
                  vec![
                    (Some(r##"part"##),
                     None,
                     Some(r##"The *part* parameter specifies the superChatEvent resource parts that the API response will include. This parameter is currently not supported."##),
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
        
        ("tests", "methods: 'insert'", vec![
            ("insert",
                    Some(r##"POST method."##),
                    "Details at http://byron.github.io/google-apis-rs/google_youtube3_cli/tests_insert",
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
        
        ("third-party-links", "methods: 'delete', 'insert', 'list' and 'update'", vec![
            ("delete",
                    Some(r##"Deletes a resource."##),
                    "Details at http://byron.github.io/google-apis-rs/google_youtube3_cli/third-party-links_delete",
                  vec![
                    (Some(r##"linking-token"##),
                     None,
                     Some(r##"Delete the partner links with the given linking token."##),
                     Some(true),
                     Some(false)),
        
                    (Some(r##"type"##),
                     None,
                     Some(r##"Type of the link to be deleted."##),
                     Some(true),
                     Some(false)),
        
                    (Some(r##"v"##),
                     Some(r##"p"##),
                     Some(r##"Set various optional parameters, matching the key=value form"##),
                     Some(false),
                     Some(true)),
                  ]),
            ("insert",
                    Some(r##"Inserts a new resource into this collection."##),
                    "Details at http://byron.github.io/google-apis-rs/google_youtube3_cli/third-party-links_insert",
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
            ("list",
                    Some(r##"Retrieves a list of resources, possibly filtered."##),
                    "Details at http://byron.github.io/google-apis-rs/google_youtube3_cli/third-party-links_list",
                  vec![
                    (Some(r##"part"##),
                     None,
                     Some(r##"The *part* parameter specifies the thirdPartyLink resource parts that the API response will include. Supported values are linkingToken, status, and snippet."##),
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
            ("update",
                    Some(r##"Updates an existing resource."##),
                    "Details at http://byron.github.io/google-apis-rs/google_youtube3_cli/third-party-links_update",
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
        
        ("thumbnails", "methods: 'set'", vec![
            ("set",
                    Some(r##"As this is not an insert in a strict sense (it supports uploading/setting of a thumbnail for multiple videos, which doesn't result in creation of a single resource), I use a custom verb here."##),
                    "Details at http://byron.github.io/google-apis-rs/google_youtube3_cli/thumbnails_set",
                  vec![
                    (Some(r##"video-id"##),
                     None,
                     Some(r##"Returns the Thumbnail with the given video IDs for Stubby or Apiary."##),
                     Some(true),
                     Some(false)),
        
                    (Some(r##"mode"##),
                     Some(r##"u"##),
                     Some(r##"Specify the upload protocol (simple) and the file to upload"##),
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
        
        ("video-abuse-report-reasons", "methods: 'list'", vec![
            ("list",
                    Some(r##"Retrieves a list of resources, possibly filtered."##),
                    "Details at http://byron.github.io/google-apis-rs/google_youtube3_cli/video-abuse-report-reasons_list",
                  vec![
                    (Some(r##"part"##),
                     None,
                     Some(r##"The *part* parameter specifies the videoCategory resource parts that the API response will include. Supported values are id and snippet."##),
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
        
        ("video-categories", "methods: 'list'", vec![
            ("list",
                    Some(r##"Retrieves a list of resources, possibly filtered."##),
                    "Details at http://byron.github.io/google-apis-rs/google_youtube3_cli/video-categories_list",
                  vec![
                    (Some(r##"part"##),
                     None,
                     Some(r##"The *part* parameter specifies the videoCategory resource properties that the API response will include. Set the parameter value to snippet."##),
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
        
        ("videos", "methods: 'delete', 'get-rating', 'insert', 'list', 'rate', 'report-abuse' and 'update'", vec![
            ("delete",
                    Some(r##"Deletes a resource."##),
                    "Details at http://byron.github.io/google-apis-rs/google_youtube3_cli/videos_delete",
                  vec![
                    (Some(r##"id"##),
                     None,
                     None,
                     Some(true),
                     Some(false)),
        
                    (Some(r##"v"##),
                     Some(r##"p"##),
                     Some(r##"Set various optional parameters, matching the key=value form"##),
                     Some(false),
                     Some(true)),
                  ]),
            ("get-rating",
                    Some(r##"Retrieves the ratings that the authorized user gave to a list of specified videos."##),
                    "Details at http://byron.github.io/google-apis-rs/google_youtube3_cli/videos_get-rating",
                  vec![
                    (Some(r##"id"##),
                     None,
                     None,
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
                    Some(r##"Inserts a new resource into this collection."##),
                    "Details at http://byron.github.io/google-apis-rs/google_youtube3_cli/videos_insert",
                  vec![
                    (Some(r##"kv"##),
                     Some(r##"r"##),
                     Some(r##"Set various fields of the request structure, matching the key=value form"##),
                     Some(true),
                     Some(true)),
        
                    (Some(r##"mode"##),
                     Some(r##"u"##),
                     Some(r##"Specify the upload protocol (simple) and the file to upload"##),
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
                    Some(r##"Retrieves a list of resources, possibly filtered."##),
                    "Details at http://byron.github.io/google-apis-rs/google_youtube3_cli/videos_list",
                  vec![
                    (Some(r##"part"##),
                     None,
                     Some(r##"The *part* parameter specifies a comma-separated list of one or more video resource properties that the API response will include. If the parameter identifies a property that contains child properties, the child properties will be included in the response. For example, in a video resource, the snippet property contains the channelId, title, description, tags, and categoryId properties. As such, if you set *part=snippet*, the API response will contain all of those properties."##),
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
            ("rate",
                    Some(r##"Adds a like or dislike rating to a video or removes a rating from a video."##),
                    "Details at http://byron.github.io/google-apis-rs/google_youtube3_cli/videos_rate",
                  vec![
                    (Some(r##"id"##),
                     None,
                     None,
                     Some(true),
                     Some(false)),
        
                    (Some(r##"rating"##),
                     None,
                     None,
                     Some(true),
                     Some(false)),
        
                    (Some(r##"v"##),
                     Some(r##"p"##),
                     Some(r##"Set various optional parameters, matching the key=value form"##),
                     Some(false),
                     Some(true)),
                  ]),
            ("report-abuse",
                    Some(r##"Report abuse for a video."##),
                    "Details at http://byron.github.io/google-apis-rs/google_youtube3_cli/videos_report-abuse",
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
                  ]),
            ("update",
                    Some(r##"Updates an existing resource."##),
                    "Details at http://byron.github.io/google-apis-rs/google_youtube3_cli/videos_update",
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
        
        ("watermarks", "methods: 'set' and 'unset'", vec![
            ("set",
                    Some(r##"Allows upload of watermark image and setting it for a channel."##),
                    "Details at http://byron.github.io/google-apis-rs/google_youtube3_cli/watermarks_set",
                  vec![
                    (Some(r##"channel-id"##),
                     None,
                     None,
                     Some(true),
                     Some(false)),
        
                    (Some(r##"kv"##),
                     Some(r##"r"##),
                     Some(r##"Set various fields of the request structure, matching the key=value form"##),
                     Some(true),
                     Some(true)),
        
                    (Some(r##"mode"##),
                     Some(r##"u"##),
                     Some(r##"Specify the upload protocol (simple) and the file to upload"##),
                     Some(true),
                     Some(true)),
        
                    (Some(r##"v"##),
                     Some(r##"p"##),
                     Some(r##"Set various optional parameters, matching the key=value form"##),
                     Some(false),
                     Some(true)),
                  ]),
            ("unset",
                    Some(r##"Allows removal of channel watermark."##),
                    "Details at http://byron.github.io/google-apis-rs/google_youtube3_cli/watermarks_unset",
                  vec![
                    (Some(r##"channel-id"##),
                     None,
                     None,
                     Some(true),
                     Some(false)),
        
                    (Some(r##"v"##),
                     Some(r##"p"##),
                     Some(r##"Set various optional parameters, matching the key=value form"##),
                     Some(false),
                     Some(true)),
                  ]),
            ]),
        
        ("youtube", "methods: 'v3-update-comment-threads'", vec![
            ("v3-update-comment-threads",
                    Some(r##"Updates an existing resource."##),
                    "Details at http://byron.github.io/google-apis-rs/google_youtube3_cli/youtube_v3-update-comment-threads",
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
    
    let mut app = App::new("youtube3")
           .author("Sebastian Thiel <byronimo@gmail.com>")
           .version("5.0.5+20240626")
           .about("The YouTube Data API v3 is an API that provides access to YouTube data, such as videos, playlists, and channels.")
           .after_help("All documentation details can be found at http://byron.github.io/google-apis-rs/google_youtube3_cli")
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
                       if arg_name_str == "mode" {
                           arg = arg.number_of_values(2);
                           arg = arg.value_names(&upload_value_names);
           
                           scmd = scmd.arg(Arg::with_name("mime")
                                               .short("m")
                                               .requires("mode")
                                               .required(false)
                                               .help("The file's mime time, like 'application/octet-stream'")
                                               .takes_value(true));
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
