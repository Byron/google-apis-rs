// DO NOT EDIT !
// This file was generated automatically from 'src/generator/templates/cli/main.rs.mako'
// DO NOT EDIT !
#![allow(unused_variables, unused_imports, dead_code, unused_mut)]

#[macro_use]
extern crate clap;

use std::env;
use std::io::{self, Write};
use clap::{App, SubCommand, Arg};

use google_tagmanager2::{api, Error, oauth2, client::chrono, FieldMask};


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
    hub: api::TagManager<S>,
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
    async fn _accounts_containers_combine(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        let mut call = self.hub.accounts().containers_combine(opt.value_of("path").unwrap_or(""));
        for parg in opt.values_of("v").map(|i|i.collect()).unwrap_or(Vec::new()).iter() {
            let (key, value) = parse_kv_arg(&*parg, err, false);
            match key {
                "setting-source" => {
                    call = call.setting_source(value.unwrap_or(""));
                },
                "container-id" => {
                    call = call.container_id(value.unwrap_or(""));
                },
                "allow-user-permission-feature-update" => {
                    call = call.allow_user_permission_feature_update(        value.map(|v| arg_from_str(v, err, "allow-user-permission-feature-update", "boolean")).unwrap_or(false));
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
                                                                           v.extend(["allow-user-permission-feature-update", "container-id", "setting-source"].iter().map(|v|*v));
                                                                           v } ));
                    }
                }
            }
        }
        let protocol = CallType::Standard;
        if dry_run {
            Ok(())
        } else {
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

    async fn _accounts_containers_create(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
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
                    "container-id" => Some(("containerId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "domain-name" => Some(("domainName", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Vec })),
                    "features.support-built-in-variables" => Some(("features.supportBuiltInVariables", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "features.support-clients" => Some(("features.supportClients", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "features.support-environments" => Some(("features.supportEnvironments", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "features.support-folders" => Some(("features.supportFolders", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "features.support-gtag-configs" => Some(("features.supportGtagConfigs", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "features.support-tags" => Some(("features.supportTags", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "features.support-templates" => Some(("features.supportTemplates", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "features.support-transformations" => Some(("features.supportTransformations", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "features.support-triggers" => Some(("features.supportTriggers", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "features.support-user-permissions" => Some(("features.supportUserPermissions", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "features.support-variables" => Some(("features.supportVariables", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "features.support-versions" => Some(("features.supportVersions", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "features.support-workspaces" => Some(("features.supportWorkspaces", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "features.support-zones" => Some(("features.supportZones", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "fingerprint" => Some(("fingerprint", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "name" => Some(("name", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "notes" => Some(("notes", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "path" => Some(("path", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "public-id" => Some(("publicId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "tag-ids" => Some(("tagIds", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Vec })),
                    "tag-manager-url" => Some(("tagManagerUrl", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "tagging-server-urls" => Some(("taggingServerUrls", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Vec })),
                    "usage-context" => Some(("usageContext", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Vec })),
                    _ => {
                        let suggestion = FieldCursor::did_you_mean(key, &vec!["account-id", "container-id", "domain-name", "features", "fingerprint", "name", "notes", "path", "public-id", "support-built-in-variables", "support-clients", "support-environments", "support-folders", "support-gtag-configs", "support-tags", "support-templates", "support-transformations", "support-triggers", "support-user-permissions", "support-variables", "support-versions", "support-workspaces", "support-zones", "tag-ids", "tag-manager-url", "tagging-server-urls", "usage-context"]);
                        err.issues.push(CLIError::Field(FieldError::Unknown(temp_cursor.to_string(), suggestion, value.map(|v| v.to_string()))));
                        None
                    }
                };
            if let Some((field_cursor_str, type_info)) = type_info {
                FieldCursor::from(field_cursor_str).set_json_value(&mut object, value.unwrap(), type_info, err, &temp_cursor);
            }
        }
        let mut request: api::Container = json::value::from_value(object).unwrap();
        let mut call = self.hub.accounts().containers_create(request, opt.value_of("parent").unwrap_or(""));
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

    async fn _accounts_containers_delete(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        let mut call = self.hub.accounts().containers_delete(opt.value_of("path").unwrap_or(""));
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

    async fn _accounts_containers_destinations_get(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        let mut call = self.hub.accounts().containers_destinations_get(opt.value_of("path").unwrap_or(""));
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

    async fn _accounts_containers_destinations_link(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        let mut call = self.hub.accounts().containers_destinations_link(opt.value_of("parent").unwrap_or(""));
        for parg in opt.values_of("v").map(|i|i.collect()).unwrap_or(Vec::new()).iter() {
            let (key, value) = parse_kv_arg(&*parg, err, false);
            match key {
                "destination-id" => {
                    call = call.destination_id(value.unwrap_or(""));
                },
                "allow-user-permission-feature-update" => {
                    call = call.allow_user_permission_feature_update(        value.map(|v| arg_from_str(v, err, "allow-user-permission-feature-update", "boolean")).unwrap_or(false));
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
                                                                           v.extend(["allow-user-permission-feature-update", "destination-id"].iter().map(|v|*v));
                                                                           v } ));
                    }
                }
            }
        }
        let protocol = CallType::Standard;
        if dry_run {
            Ok(())
        } else {
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

    async fn _accounts_containers_destinations_list(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        let mut call = self.hub.accounts().containers_destinations_list(opt.value_of("parent").unwrap_or(""));
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

    async fn _accounts_containers_environments_create(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
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
                    "authorization-code" => Some(("authorizationCode", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "authorization-timestamp" => Some(("authorizationTimestamp", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "container-id" => Some(("containerId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "container-version-id" => Some(("containerVersionId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "description" => Some(("description", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "enable-debug" => Some(("enableDebug", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "environment-id" => Some(("environmentId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "fingerprint" => Some(("fingerprint", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "name" => Some(("name", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "path" => Some(("path", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "tag-manager-url" => Some(("tagManagerUrl", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "type" => Some(("type", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "url" => Some(("url", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "workspace-id" => Some(("workspaceId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    _ => {
                        let suggestion = FieldCursor::did_you_mean(key, &vec!["account-id", "authorization-code", "authorization-timestamp", "container-id", "container-version-id", "description", "enable-debug", "environment-id", "fingerprint", "name", "path", "tag-manager-url", "type", "url", "workspace-id"]);
                        err.issues.push(CLIError::Field(FieldError::Unknown(temp_cursor.to_string(), suggestion, value.map(|v| v.to_string()))));
                        None
                    }
                };
            if let Some((field_cursor_str, type_info)) = type_info {
                FieldCursor::from(field_cursor_str).set_json_value(&mut object, value.unwrap(), type_info, err, &temp_cursor);
            }
        }
        let mut request: api::Environment = json::value::from_value(object).unwrap();
        let mut call = self.hub.accounts().containers_environments_create(request, opt.value_of("parent").unwrap_or(""));
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

    async fn _accounts_containers_environments_delete(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        let mut call = self.hub.accounts().containers_environments_delete(opt.value_of("path").unwrap_or(""));
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

    async fn _accounts_containers_environments_get(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        let mut call = self.hub.accounts().containers_environments_get(opt.value_of("path").unwrap_or(""));
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

    async fn _accounts_containers_environments_list(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        let mut call = self.hub.accounts().containers_environments_list(opt.value_of("parent").unwrap_or(""));
        for parg in opt.values_of("v").map(|i|i.collect()).unwrap_or(Vec::new()).iter() {
            let (key, value) = parse_kv_arg(&*parg, err, false);
            match key {
                "page-token" => {
                    call = call.page_token(value.unwrap_or(""));
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
                                                                           v.extend(["page-token"].iter().map(|v|*v));
                                                                           v } ));
                    }
                }
            }
        }
        let protocol = CallType::Standard;
        if dry_run {
            Ok(())
        } else {
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

    async fn _accounts_containers_environments_reauthorize(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
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
                    "authorization-code" => Some(("authorizationCode", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "authorization-timestamp" => Some(("authorizationTimestamp", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "container-id" => Some(("containerId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "container-version-id" => Some(("containerVersionId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "description" => Some(("description", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "enable-debug" => Some(("enableDebug", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "environment-id" => Some(("environmentId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "fingerprint" => Some(("fingerprint", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "name" => Some(("name", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "path" => Some(("path", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "tag-manager-url" => Some(("tagManagerUrl", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "type" => Some(("type", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "url" => Some(("url", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "workspace-id" => Some(("workspaceId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    _ => {
                        let suggestion = FieldCursor::did_you_mean(key, &vec!["account-id", "authorization-code", "authorization-timestamp", "container-id", "container-version-id", "description", "enable-debug", "environment-id", "fingerprint", "name", "path", "tag-manager-url", "type", "url", "workspace-id"]);
                        err.issues.push(CLIError::Field(FieldError::Unknown(temp_cursor.to_string(), suggestion, value.map(|v| v.to_string()))));
                        None
                    }
                };
            if let Some((field_cursor_str, type_info)) = type_info {
                FieldCursor::from(field_cursor_str).set_json_value(&mut object, value.unwrap(), type_info, err, &temp_cursor);
            }
        }
        let mut request: api::Environment = json::value::from_value(object).unwrap();
        let mut call = self.hub.accounts().containers_environments_reauthorize(request, opt.value_of("path").unwrap_or(""));
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

    async fn _accounts_containers_environments_update(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
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
                    "authorization-code" => Some(("authorizationCode", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "authorization-timestamp" => Some(("authorizationTimestamp", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "container-id" => Some(("containerId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "container-version-id" => Some(("containerVersionId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "description" => Some(("description", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "enable-debug" => Some(("enableDebug", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "environment-id" => Some(("environmentId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "fingerprint" => Some(("fingerprint", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "name" => Some(("name", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "path" => Some(("path", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "tag-manager-url" => Some(("tagManagerUrl", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "type" => Some(("type", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "url" => Some(("url", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "workspace-id" => Some(("workspaceId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    _ => {
                        let suggestion = FieldCursor::did_you_mean(key, &vec!["account-id", "authorization-code", "authorization-timestamp", "container-id", "container-version-id", "description", "enable-debug", "environment-id", "fingerprint", "name", "path", "tag-manager-url", "type", "url", "workspace-id"]);
                        err.issues.push(CLIError::Field(FieldError::Unknown(temp_cursor.to_string(), suggestion, value.map(|v| v.to_string()))));
                        None
                    }
                };
            if let Some((field_cursor_str, type_info)) = type_info {
                FieldCursor::from(field_cursor_str).set_json_value(&mut object, value.unwrap(), type_info, err, &temp_cursor);
            }
        }
        let mut request: api::Environment = json::value::from_value(object).unwrap();
        let mut call = self.hub.accounts().containers_environments_update(request, opt.value_of("path").unwrap_or(""));
        for parg in opt.values_of("v").map(|i|i.collect()).unwrap_or(Vec::new()).iter() {
            let (key, value) = parse_kv_arg(&*parg, err, false);
            match key {
                "fingerprint" => {
                    call = call.fingerprint(value.unwrap_or(""));
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
                                                                           v.extend(["fingerprint"].iter().map(|v|*v));
                                                                           v } ));
                    }
                }
            }
        }
        let protocol = CallType::Standard;
        if dry_run {
            Ok(())
        } else {
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

    async fn _accounts_containers_get(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        let mut call = self.hub.accounts().containers_get(opt.value_of("path").unwrap_or(""));
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

    async fn _accounts_containers_list(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        let mut call = self.hub.accounts().containers_list(opt.value_of("parent").unwrap_or(""));
        for parg in opt.values_of("v").map(|i|i.collect()).unwrap_or(Vec::new()).iter() {
            let (key, value) = parse_kv_arg(&*parg, err, false);
            match key {
                "page-token" => {
                    call = call.page_token(value.unwrap_or(""));
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
                                                                           v.extend(["page-token"].iter().map(|v|*v));
                                                                           v } ));
                    }
                }
            }
        }
        let protocol = CallType::Standard;
        if dry_run {
            Ok(())
        } else {
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

    async fn _accounts_containers_lookup(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        let mut call = self.hub.accounts().containers_lookup();
        for parg in opt.values_of("v").map(|i|i.collect()).unwrap_or(Vec::new()).iter() {
            let (key, value) = parse_kv_arg(&*parg, err, false);
            match key {
                "destination-id" => {
                    call = call.destination_id(value.unwrap_or(""));
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
                                                                           v.extend(["destination-id"].iter().map(|v|*v));
                                                                           v } ));
                    }
                }
            }
        }
        let protocol = CallType::Standard;
        if dry_run {
            Ok(())
        } else {
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

    async fn _accounts_containers_move_tag_id(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        let mut call = self.hub.accounts().containers_move_tag_id(opt.value_of("path").unwrap_or(""));
        for parg in opt.values_of("v").map(|i|i.collect()).unwrap_or(Vec::new()).iter() {
            let (key, value) = parse_kv_arg(&*parg, err, false);
            match key {
                "tag-name" => {
                    call = call.tag_name(value.unwrap_or(""));
                },
                "tag-id" => {
                    call = call.tag_id(value.unwrap_or(""));
                },
                "copy-users" => {
                    call = call.copy_users(        value.map(|v| arg_from_str(v, err, "copy-users", "boolean")).unwrap_or(false));
                },
                "copy-terms-of-service" => {
                    call = call.copy_terms_of_service(        value.map(|v| arg_from_str(v, err, "copy-terms-of-service", "boolean")).unwrap_or(false));
                },
                "copy-settings" => {
                    call = call.copy_settings(        value.map(|v| arg_from_str(v, err, "copy-settings", "boolean")).unwrap_or(false));
                },
                "allow-user-permission-feature-update" => {
                    call = call.allow_user_permission_feature_update(        value.map(|v| arg_from_str(v, err, "allow-user-permission-feature-update", "boolean")).unwrap_or(false));
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
                                                                           v.extend(["allow-user-permission-feature-update", "copy-settings", "copy-terms-of-service", "copy-users", "tag-id", "tag-name"].iter().map(|v|*v));
                                                                           v } ));
                    }
                }
            }
        }
        let protocol = CallType::Standard;
        if dry_run {
            Ok(())
        } else {
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

    async fn _accounts_containers_snippet(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        let mut call = self.hub.accounts().containers_snippet(opt.value_of("path").unwrap_or(""));
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

    async fn _accounts_containers_update(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
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
                    "container-id" => Some(("containerId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "domain-name" => Some(("domainName", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Vec })),
                    "features.support-built-in-variables" => Some(("features.supportBuiltInVariables", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "features.support-clients" => Some(("features.supportClients", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "features.support-environments" => Some(("features.supportEnvironments", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "features.support-folders" => Some(("features.supportFolders", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "features.support-gtag-configs" => Some(("features.supportGtagConfigs", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "features.support-tags" => Some(("features.supportTags", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "features.support-templates" => Some(("features.supportTemplates", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "features.support-transformations" => Some(("features.supportTransformations", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "features.support-triggers" => Some(("features.supportTriggers", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "features.support-user-permissions" => Some(("features.supportUserPermissions", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "features.support-variables" => Some(("features.supportVariables", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "features.support-versions" => Some(("features.supportVersions", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "features.support-workspaces" => Some(("features.supportWorkspaces", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "features.support-zones" => Some(("features.supportZones", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "fingerprint" => Some(("fingerprint", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "name" => Some(("name", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "notes" => Some(("notes", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "path" => Some(("path", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "public-id" => Some(("publicId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "tag-ids" => Some(("tagIds", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Vec })),
                    "tag-manager-url" => Some(("tagManagerUrl", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "tagging-server-urls" => Some(("taggingServerUrls", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Vec })),
                    "usage-context" => Some(("usageContext", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Vec })),
                    _ => {
                        let suggestion = FieldCursor::did_you_mean(key, &vec!["account-id", "container-id", "domain-name", "features", "fingerprint", "name", "notes", "path", "public-id", "support-built-in-variables", "support-clients", "support-environments", "support-folders", "support-gtag-configs", "support-tags", "support-templates", "support-transformations", "support-triggers", "support-user-permissions", "support-variables", "support-versions", "support-workspaces", "support-zones", "tag-ids", "tag-manager-url", "tagging-server-urls", "usage-context"]);
                        err.issues.push(CLIError::Field(FieldError::Unknown(temp_cursor.to_string(), suggestion, value.map(|v| v.to_string()))));
                        None
                    }
                };
            if let Some((field_cursor_str, type_info)) = type_info {
                FieldCursor::from(field_cursor_str).set_json_value(&mut object, value.unwrap(), type_info, err, &temp_cursor);
            }
        }
        let mut request: api::Container = json::value::from_value(object).unwrap();
        let mut call = self.hub.accounts().containers_update(request, opt.value_of("path").unwrap_or(""));
        for parg in opt.values_of("v").map(|i|i.collect()).unwrap_or(Vec::new()).iter() {
            let (key, value) = parse_kv_arg(&*parg, err, false);
            match key {
                "fingerprint" => {
                    call = call.fingerprint(value.unwrap_or(""));
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
                                                                           v.extend(["fingerprint"].iter().map(|v|*v));
                                                                           v } ));
                    }
                }
            }
        }
        let protocol = CallType::Standard;
        if dry_run {
            Ok(())
        } else {
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

    async fn _accounts_containers_version_headers_latest(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        let mut call = self.hub.accounts().containers_version_headers_latest(opt.value_of("parent").unwrap_or(""));
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

    async fn _accounts_containers_version_headers_list(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        let mut call = self.hub.accounts().containers_version_headers_list(opt.value_of("parent").unwrap_or(""));
        for parg in opt.values_of("v").map(|i|i.collect()).unwrap_or(Vec::new()).iter() {
            let (key, value) = parse_kv_arg(&*parg, err, false);
            match key {
                "page-token" => {
                    call = call.page_token(value.unwrap_or(""));
                },
                "include-deleted" => {
                    call = call.include_deleted(        value.map(|v| arg_from_str(v, err, "include-deleted", "boolean")).unwrap_or(false));
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
                                                                           v.extend(["include-deleted", "page-token"].iter().map(|v|*v));
                                                                           v } ));
                    }
                }
            }
        }
        let protocol = CallType::Standard;
        if dry_run {
            Ok(())
        } else {
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

    async fn _accounts_containers_versions_delete(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        let mut call = self.hub.accounts().containers_versions_delete(opt.value_of("path").unwrap_or(""));
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

    async fn _accounts_containers_versions_get(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        let mut call = self.hub.accounts().containers_versions_get(opt.value_of("path").unwrap_or(""));
        for parg in opt.values_of("v").map(|i|i.collect()).unwrap_or(Vec::new()).iter() {
            let (key, value) = parse_kv_arg(&*parg, err, false);
            match key {
                "container-version-id" => {
                    call = call.container_version_id(value.unwrap_or(""));
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
                                                                           v.extend(["container-version-id"].iter().map(|v|*v));
                                                                           v } ));
                    }
                }
            }
        }
        let protocol = CallType::Standard;
        if dry_run {
            Ok(())
        } else {
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

    async fn _accounts_containers_versions_live(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        let mut call = self.hub.accounts().containers_versions_live(opt.value_of("parent").unwrap_or(""));
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

    async fn _accounts_containers_versions_publish(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        let mut call = self.hub.accounts().containers_versions_publish(opt.value_of("path").unwrap_or(""));
        for parg in opt.values_of("v").map(|i|i.collect()).unwrap_or(Vec::new()).iter() {
            let (key, value) = parse_kv_arg(&*parg, err, false);
            match key {
                "fingerprint" => {
                    call = call.fingerprint(value.unwrap_or(""));
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
                                                                           v.extend(["fingerprint"].iter().map(|v|*v));
                                                                           v } ));
                    }
                }
            }
        }
        let protocol = CallType::Standard;
        if dry_run {
            Ok(())
        } else {
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

    async fn _accounts_containers_versions_set_latest(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        let mut call = self.hub.accounts().containers_versions_set_latest(opt.value_of("path").unwrap_or(""));
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

    async fn _accounts_containers_versions_undelete(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        let mut call = self.hub.accounts().containers_versions_undelete(opt.value_of("path").unwrap_or(""));
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

    async fn _accounts_containers_versions_update(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
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
                    "container.account-id" => Some(("container.accountId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "container.container-id" => Some(("container.containerId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "container.domain-name" => Some(("container.domainName", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Vec })),
                    "container.features.support-built-in-variables" => Some(("container.features.supportBuiltInVariables", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "container.features.support-clients" => Some(("container.features.supportClients", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "container.features.support-environments" => Some(("container.features.supportEnvironments", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "container.features.support-folders" => Some(("container.features.supportFolders", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "container.features.support-gtag-configs" => Some(("container.features.supportGtagConfigs", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "container.features.support-tags" => Some(("container.features.supportTags", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "container.features.support-templates" => Some(("container.features.supportTemplates", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "container.features.support-transformations" => Some(("container.features.supportTransformations", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "container.features.support-triggers" => Some(("container.features.supportTriggers", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "container.features.support-user-permissions" => Some(("container.features.supportUserPermissions", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "container.features.support-variables" => Some(("container.features.supportVariables", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "container.features.support-versions" => Some(("container.features.supportVersions", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "container.features.support-workspaces" => Some(("container.features.supportWorkspaces", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "container.features.support-zones" => Some(("container.features.supportZones", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "container.fingerprint" => Some(("container.fingerprint", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "container.name" => Some(("container.name", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "container.notes" => Some(("container.notes", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "container.path" => Some(("container.path", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "container.public-id" => Some(("container.publicId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "container.tag-ids" => Some(("container.tagIds", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Vec })),
                    "container.tag-manager-url" => Some(("container.tagManagerUrl", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "container.tagging-server-urls" => Some(("container.taggingServerUrls", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Vec })),
                    "container.usage-context" => Some(("container.usageContext", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Vec })),
                    "container-id" => Some(("containerId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "container-version-id" => Some(("containerVersionId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "deleted" => Some(("deleted", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "description" => Some(("description", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "fingerprint" => Some(("fingerprint", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "name" => Some(("name", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "path" => Some(("path", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "tag-manager-url" => Some(("tagManagerUrl", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    _ => {
                        let suggestion = FieldCursor::did_you_mean(key, &vec!["account-id", "container", "container-id", "container-version-id", "deleted", "description", "domain-name", "features", "fingerprint", "name", "notes", "path", "public-id", "support-built-in-variables", "support-clients", "support-environments", "support-folders", "support-gtag-configs", "support-tags", "support-templates", "support-transformations", "support-triggers", "support-user-permissions", "support-variables", "support-versions", "support-workspaces", "support-zones", "tag-ids", "tag-manager-url", "tagging-server-urls", "usage-context"]);
                        err.issues.push(CLIError::Field(FieldError::Unknown(temp_cursor.to_string(), suggestion, value.map(|v| v.to_string()))));
                        None
                    }
                };
            if let Some((field_cursor_str, type_info)) = type_info {
                FieldCursor::from(field_cursor_str).set_json_value(&mut object, value.unwrap(), type_info, err, &temp_cursor);
            }
        }
        let mut request: api::ContainerVersion = json::value::from_value(object).unwrap();
        let mut call = self.hub.accounts().containers_versions_update(request, opt.value_of("path").unwrap_or(""));
        for parg in opt.values_of("v").map(|i|i.collect()).unwrap_or(Vec::new()).iter() {
            let (key, value) = parse_kv_arg(&*parg, err, false);
            match key {
                "fingerprint" => {
                    call = call.fingerprint(value.unwrap_or(""));
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
                                                                           v.extend(["fingerprint"].iter().map(|v|*v));
                                                                           v } ));
                    }
                }
            }
        }
        let protocol = CallType::Standard;
        if dry_run {
            Ok(())
        } else {
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

    async fn _accounts_containers_workspaces_built_in_variables_create(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        let mut call = self.hub.accounts().containers_workspaces_built_in_variables_create(opt.value_of("parent").unwrap_or(""));
        for parg in opt.values_of("v").map(|i|i.collect()).unwrap_or(Vec::new()).iter() {
            let (key, value) = parse_kv_arg(&*parg, err, false);
            match key {
                "type" => {
                    call = call.add_type(value.unwrap_or(""));
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

    async fn _accounts_containers_workspaces_built_in_variables_delete(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        let mut call = self.hub.accounts().containers_workspaces_built_in_variables_delete(opt.value_of("path").unwrap_or(""));
        for parg in opt.values_of("v").map(|i|i.collect()).unwrap_or(Vec::new()).iter() {
            let (key, value) = parse_kv_arg(&*parg, err, false);
            match key {
                "type" => {
                    call = call.add_type(value.unwrap_or(""));
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

    async fn _accounts_containers_workspaces_built_in_variables_list(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        let mut call = self.hub.accounts().containers_workspaces_built_in_variables_list(opt.value_of("parent").unwrap_or(""));
        for parg in opt.values_of("v").map(|i|i.collect()).unwrap_or(Vec::new()).iter() {
            let (key, value) = parse_kv_arg(&*parg, err, false);
            match key {
                "page-token" => {
                    call = call.page_token(value.unwrap_or(""));
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
                                                                           v.extend(["page-token"].iter().map(|v|*v));
                                                                           v } ));
                    }
                }
            }
        }
        let protocol = CallType::Standard;
        if dry_run {
            Ok(())
        } else {
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

    async fn _accounts_containers_workspaces_built_in_variables_revert(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        let mut call = self.hub.accounts().containers_workspaces_built_in_variables_revert(opt.value_of("path").unwrap_or(""));
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

    async fn _accounts_containers_workspaces_clients_create(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
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
                    "client-id" => Some(("clientId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "container-id" => Some(("containerId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "fingerprint" => Some(("fingerprint", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "name" => Some(("name", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "notes" => Some(("notes", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "parent-folder-id" => Some(("parentFolderId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "path" => Some(("path", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "priority" => Some(("priority", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "tag-manager-url" => Some(("tagManagerUrl", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "type" => Some(("type", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "workspace-id" => Some(("workspaceId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    _ => {
                        let suggestion = FieldCursor::did_you_mean(key, &vec!["account-id", "client-id", "container-id", "fingerprint", "name", "notes", "parent-folder-id", "path", "priority", "tag-manager-url", "type", "workspace-id"]);
                        err.issues.push(CLIError::Field(FieldError::Unknown(temp_cursor.to_string(), suggestion, value.map(|v| v.to_string()))));
                        None
                    }
                };
            if let Some((field_cursor_str, type_info)) = type_info {
                FieldCursor::from(field_cursor_str).set_json_value(&mut object, value.unwrap(), type_info, err, &temp_cursor);
            }
        }
        let mut request: api::Client = json::value::from_value(object).unwrap();
        let mut call = self.hub.accounts().containers_workspaces_clients_create(request, opt.value_of("parent").unwrap_or(""));
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

    async fn _accounts_containers_workspaces_clients_delete(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        let mut call = self.hub.accounts().containers_workspaces_clients_delete(opt.value_of("path").unwrap_or(""));
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

    async fn _accounts_containers_workspaces_clients_get(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        let mut call = self.hub.accounts().containers_workspaces_clients_get(opt.value_of("path").unwrap_or(""));
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

    async fn _accounts_containers_workspaces_clients_list(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        let mut call = self.hub.accounts().containers_workspaces_clients_list(opt.value_of("parent").unwrap_or(""));
        for parg in opt.values_of("v").map(|i|i.collect()).unwrap_or(Vec::new()).iter() {
            let (key, value) = parse_kv_arg(&*parg, err, false);
            match key {
                "page-token" => {
                    call = call.page_token(value.unwrap_or(""));
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
                                                                           v.extend(["page-token"].iter().map(|v|*v));
                                                                           v } ));
                    }
                }
            }
        }
        let protocol = CallType::Standard;
        if dry_run {
            Ok(())
        } else {
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

    async fn _accounts_containers_workspaces_clients_revert(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        let mut call = self.hub.accounts().containers_workspaces_clients_revert(opt.value_of("path").unwrap_or(""));
        for parg in opt.values_of("v").map(|i|i.collect()).unwrap_or(Vec::new()).iter() {
            let (key, value) = parse_kv_arg(&*parg, err, false);
            match key {
                "fingerprint" => {
                    call = call.fingerprint(value.unwrap_or(""));
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
                                                                           v.extend(["fingerprint"].iter().map(|v|*v));
                                                                           v } ));
                    }
                }
            }
        }
        let protocol = CallType::Standard;
        if dry_run {
            Ok(())
        } else {
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

    async fn _accounts_containers_workspaces_clients_update(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
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
                    "client-id" => Some(("clientId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "container-id" => Some(("containerId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "fingerprint" => Some(("fingerprint", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "name" => Some(("name", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "notes" => Some(("notes", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "parent-folder-id" => Some(("parentFolderId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "path" => Some(("path", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "priority" => Some(("priority", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "tag-manager-url" => Some(("tagManagerUrl", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "type" => Some(("type", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "workspace-id" => Some(("workspaceId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    _ => {
                        let suggestion = FieldCursor::did_you_mean(key, &vec!["account-id", "client-id", "container-id", "fingerprint", "name", "notes", "parent-folder-id", "path", "priority", "tag-manager-url", "type", "workspace-id"]);
                        err.issues.push(CLIError::Field(FieldError::Unknown(temp_cursor.to_string(), suggestion, value.map(|v| v.to_string()))));
                        None
                    }
                };
            if let Some((field_cursor_str, type_info)) = type_info {
                FieldCursor::from(field_cursor_str).set_json_value(&mut object, value.unwrap(), type_info, err, &temp_cursor);
            }
        }
        let mut request: api::Client = json::value::from_value(object).unwrap();
        let mut call = self.hub.accounts().containers_workspaces_clients_update(request, opt.value_of("path").unwrap_or(""));
        for parg in opt.values_of("v").map(|i|i.collect()).unwrap_or(Vec::new()).iter() {
            let (key, value) = parse_kv_arg(&*parg, err, false);
            match key {
                "fingerprint" => {
                    call = call.fingerprint(value.unwrap_or(""));
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
                                                                           v.extend(["fingerprint"].iter().map(|v|*v));
                                                                           v } ));
                    }
                }
            }
        }
        let protocol = CallType::Standard;
        if dry_run {
            Ok(())
        } else {
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

    async fn _accounts_containers_workspaces_create(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
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
                    "container-id" => Some(("containerId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "description" => Some(("description", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "fingerprint" => Some(("fingerprint", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "name" => Some(("name", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "path" => Some(("path", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "tag-manager-url" => Some(("tagManagerUrl", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "workspace-id" => Some(("workspaceId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    _ => {
                        let suggestion = FieldCursor::did_you_mean(key, &vec!["account-id", "container-id", "description", "fingerprint", "name", "path", "tag-manager-url", "workspace-id"]);
                        err.issues.push(CLIError::Field(FieldError::Unknown(temp_cursor.to_string(), suggestion, value.map(|v| v.to_string()))));
                        None
                    }
                };
            if let Some((field_cursor_str, type_info)) = type_info {
                FieldCursor::from(field_cursor_str).set_json_value(&mut object, value.unwrap(), type_info, err, &temp_cursor);
            }
        }
        let mut request: api::Workspace = json::value::from_value(object).unwrap();
        let mut call = self.hub.accounts().containers_workspaces_create(request, opt.value_of("parent").unwrap_or(""));
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

    async fn _accounts_containers_workspaces_create_version(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
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
                    "notes" => Some(("notes", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    _ => {
                        let suggestion = FieldCursor::did_you_mean(key, &vec!["name", "notes"]);
                        err.issues.push(CLIError::Field(FieldError::Unknown(temp_cursor.to_string(), suggestion, value.map(|v| v.to_string()))));
                        None
                    }
                };
            if let Some((field_cursor_str, type_info)) = type_info {
                FieldCursor::from(field_cursor_str).set_json_value(&mut object, value.unwrap(), type_info, err, &temp_cursor);
            }
        }
        let mut request: api::CreateContainerVersionRequestVersionOptions = json::value::from_value(object).unwrap();
        let mut call = self.hub.accounts().containers_workspaces_create_version(request, opt.value_of("path").unwrap_or(""));
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

    async fn _accounts_containers_workspaces_delete(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        let mut call = self.hub.accounts().containers_workspaces_delete(opt.value_of("path").unwrap_or(""));
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

    async fn _accounts_containers_workspaces_folders_create(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
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
                    "container-id" => Some(("containerId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "fingerprint" => Some(("fingerprint", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "folder-id" => Some(("folderId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "name" => Some(("name", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "notes" => Some(("notes", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "path" => Some(("path", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "tag-manager-url" => Some(("tagManagerUrl", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "workspace-id" => Some(("workspaceId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    _ => {
                        let suggestion = FieldCursor::did_you_mean(key, &vec!["account-id", "container-id", "fingerprint", "folder-id", "name", "notes", "path", "tag-manager-url", "workspace-id"]);
                        err.issues.push(CLIError::Field(FieldError::Unknown(temp_cursor.to_string(), suggestion, value.map(|v| v.to_string()))));
                        None
                    }
                };
            if let Some((field_cursor_str, type_info)) = type_info {
                FieldCursor::from(field_cursor_str).set_json_value(&mut object, value.unwrap(), type_info, err, &temp_cursor);
            }
        }
        let mut request: api::Folder = json::value::from_value(object).unwrap();
        let mut call = self.hub.accounts().containers_workspaces_folders_create(request, opt.value_of("parent").unwrap_or(""));
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

    async fn _accounts_containers_workspaces_folders_delete(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        let mut call = self.hub.accounts().containers_workspaces_folders_delete(opt.value_of("path").unwrap_or(""));
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

    async fn _accounts_containers_workspaces_folders_entities(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        let mut call = self.hub.accounts().containers_workspaces_folders_entities(opt.value_of("path").unwrap_or(""));
        for parg in opt.values_of("v").map(|i|i.collect()).unwrap_or(Vec::new()).iter() {
            let (key, value) = parse_kv_arg(&*parg, err, false);
            match key {
                "page-token" => {
                    call = call.page_token(value.unwrap_or(""));
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
                                                                           v.extend(["page-token"].iter().map(|v|*v));
                                                                           v } ));
                    }
                }
            }
        }
        let protocol = CallType::Standard;
        if dry_run {
            Ok(())
        } else {
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

    async fn _accounts_containers_workspaces_folders_get(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        let mut call = self.hub.accounts().containers_workspaces_folders_get(opt.value_of("path").unwrap_or(""));
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

    async fn _accounts_containers_workspaces_folders_list(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        let mut call = self.hub.accounts().containers_workspaces_folders_list(opt.value_of("parent").unwrap_or(""));
        for parg in opt.values_of("v").map(|i|i.collect()).unwrap_or(Vec::new()).iter() {
            let (key, value) = parse_kv_arg(&*parg, err, false);
            match key {
                "page-token" => {
                    call = call.page_token(value.unwrap_or(""));
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
                                                                           v.extend(["page-token"].iter().map(|v|*v));
                                                                           v } ));
                    }
                }
            }
        }
        let protocol = CallType::Standard;
        if dry_run {
            Ok(())
        } else {
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

    async fn _accounts_containers_workspaces_folders_move_entities_to_folder(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
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
                    "container-id" => Some(("containerId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "fingerprint" => Some(("fingerprint", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "folder-id" => Some(("folderId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "name" => Some(("name", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "notes" => Some(("notes", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "path" => Some(("path", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "tag-manager-url" => Some(("tagManagerUrl", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "workspace-id" => Some(("workspaceId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    _ => {
                        let suggestion = FieldCursor::did_you_mean(key, &vec!["account-id", "container-id", "fingerprint", "folder-id", "name", "notes", "path", "tag-manager-url", "workspace-id"]);
                        err.issues.push(CLIError::Field(FieldError::Unknown(temp_cursor.to_string(), suggestion, value.map(|v| v.to_string()))));
                        None
                    }
                };
            if let Some((field_cursor_str, type_info)) = type_info {
                FieldCursor::from(field_cursor_str).set_json_value(&mut object, value.unwrap(), type_info, err, &temp_cursor);
            }
        }
        let mut request: api::Folder = json::value::from_value(object).unwrap();
        let mut call = self.hub.accounts().containers_workspaces_folders_move_entities_to_folder(request, opt.value_of("path").unwrap_or(""));
        for parg in opt.values_of("v").map(|i|i.collect()).unwrap_or(Vec::new()).iter() {
            let (key, value) = parse_kv_arg(&*parg, err, false);
            match key {
                "variable-id" => {
                    call = call.add_variable_id(value.unwrap_or(""));
                },
                "trigger-id" => {
                    call = call.add_trigger_id(value.unwrap_or(""));
                },
                "tag-id" => {
                    call = call.add_tag_id(value.unwrap_or(""));
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
                                                                           v.extend(["tag-id", "trigger-id", "variable-id"].iter().map(|v|*v));
                                                                           v } ));
                    }
                }
            }
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

    async fn _accounts_containers_workspaces_folders_revert(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        let mut call = self.hub.accounts().containers_workspaces_folders_revert(opt.value_of("path").unwrap_or(""));
        for parg in opt.values_of("v").map(|i|i.collect()).unwrap_or(Vec::new()).iter() {
            let (key, value) = parse_kv_arg(&*parg, err, false);
            match key {
                "fingerprint" => {
                    call = call.fingerprint(value.unwrap_or(""));
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
                                                                           v.extend(["fingerprint"].iter().map(|v|*v));
                                                                           v } ));
                    }
                }
            }
        }
        let protocol = CallType::Standard;
        if dry_run {
            Ok(())
        } else {
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

    async fn _accounts_containers_workspaces_folders_update(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
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
                    "container-id" => Some(("containerId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "fingerprint" => Some(("fingerprint", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "folder-id" => Some(("folderId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "name" => Some(("name", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "notes" => Some(("notes", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "path" => Some(("path", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "tag-manager-url" => Some(("tagManagerUrl", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "workspace-id" => Some(("workspaceId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    _ => {
                        let suggestion = FieldCursor::did_you_mean(key, &vec!["account-id", "container-id", "fingerprint", "folder-id", "name", "notes", "path", "tag-manager-url", "workspace-id"]);
                        err.issues.push(CLIError::Field(FieldError::Unknown(temp_cursor.to_string(), suggestion, value.map(|v| v.to_string()))));
                        None
                    }
                };
            if let Some((field_cursor_str, type_info)) = type_info {
                FieldCursor::from(field_cursor_str).set_json_value(&mut object, value.unwrap(), type_info, err, &temp_cursor);
            }
        }
        let mut request: api::Folder = json::value::from_value(object).unwrap();
        let mut call = self.hub.accounts().containers_workspaces_folders_update(request, opt.value_of("path").unwrap_or(""));
        for parg in opt.values_of("v").map(|i|i.collect()).unwrap_or(Vec::new()).iter() {
            let (key, value) = parse_kv_arg(&*parg, err, false);
            match key {
                "fingerprint" => {
                    call = call.fingerprint(value.unwrap_or(""));
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
                                                                           v.extend(["fingerprint"].iter().map(|v|*v));
                                                                           v } ));
                    }
                }
            }
        }
        let protocol = CallType::Standard;
        if dry_run {
            Ok(())
        } else {
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

    async fn _accounts_containers_workspaces_get(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        let mut call = self.hub.accounts().containers_workspaces_get(opt.value_of("path").unwrap_or(""));
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

    async fn _accounts_containers_workspaces_get_status(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        let mut call = self.hub.accounts().containers_workspaces_get_status(opt.value_of("path").unwrap_or(""));
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

    async fn _accounts_containers_workspaces_gtag_config_create(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
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
                    "container-id" => Some(("containerId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "fingerprint" => Some(("fingerprint", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "gtag-config-id" => Some(("gtagConfigId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "path" => Some(("path", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "tag-manager-url" => Some(("tagManagerUrl", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "type" => Some(("type", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "workspace-id" => Some(("workspaceId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    _ => {
                        let suggestion = FieldCursor::did_you_mean(key, &vec!["account-id", "container-id", "fingerprint", "gtag-config-id", "path", "tag-manager-url", "type", "workspace-id"]);
                        err.issues.push(CLIError::Field(FieldError::Unknown(temp_cursor.to_string(), suggestion, value.map(|v| v.to_string()))));
                        None
                    }
                };
            if let Some((field_cursor_str, type_info)) = type_info {
                FieldCursor::from(field_cursor_str).set_json_value(&mut object, value.unwrap(), type_info, err, &temp_cursor);
            }
        }
        let mut request: api::GtagConfig = json::value::from_value(object).unwrap();
        let mut call = self.hub.accounts().containers_workspaces_gtag_config_create(request, opt.value_of("parent").unwrap_or(""));
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

    async fn _accounts_containers_workspaces_gtag_config_delete(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        let mut call = self.hub.accounts().containers_workspaces_gtag_config_delete(opt.value_of("path").unwrap_or(""));
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

    async fn _accounts_containers_workspaces_gtag_config_get(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        let mut call = self.hub.accounts().containers_workspaces_gtag_config_get(opt.value_of("path").unwrap_or(""));
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

    async fn _accounts_containers_workspaces_gtag_config_list(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        let mut call = self.hub.accounts().containers_workspaces_gtag_config_list(opt.value_of("parent").unwrap_or(""));
        for parg in opt.values_of("v").map(|i|i.collect()).unwrap_or(Vec::new()).iter() {
            let (key, value) = parse_kv_arg(&*parg, err, false);
            match key {
                "page-token" => {
                    call = call.page_token(value.unwrap_or(""));
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
                                                                           v.extend(["page-token"].iter().map(|v|*v));
                                                                           v } ));
                    }
                }
            }
        }
        let protocol = CallType::Standard;
        if dry_run {
            Ok(())
        } else {
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

    async fn _accounts_containers_workspaces_gtag_config_update(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
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
                    "container-id" => Some(("containerId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "fingerprint" => Some(("fingerprint", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "gtag-config-id" => Some(("gtagConfigId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "path" => Some(("path", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "tag-manager-url" => Some(("tagManagerUrl", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "type" => Some(("type", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "workspace-id" => Some(("workspaceId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    _ => {
                        let suggestion = FieldCursor::did_you_mean(key, &vec!["account-id", "container-id", "fingerprint", "gtag-config-id", "path", "tag-manager-url", "type", "workspace-id"]);
                        err.issues.push(CLIError::Field(FieldError::Unknown(temp_cursor.to_string(), suggestion, value.map(|v| v.to_string()))));
                        None
                    }
                };
            if let Some((field_cursor_str, type_info)) = type_info {
                FieldCursor::from(field_cursor_str).set_json_value(&mut object, value.unwrap(), type_info, err, &temp_cursor);
            }
        }
        let mut request: api::GtagConfig = json::value::from_value(object).unwrap();
        let mut call = self.hub.accounts().containers_workspaces_gtag_config_update(request, opt.value_of("path").unwrap_or(""));
        for parg in opt.values_of("v").map(|i|i.collect()).unwrap_or(Vec::new()).iter() {
            let (key, value) = parse_kv_arg(&*parg, err, false);
            match key {
                "fingerprint" => {
                    call = call.fingerprint(value.unwrap_or(""));
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
                                                                           v.extend(["fingerprint"].iter().map(|v|*v));
                                                                           v } ));
                    }
                }
            }
        }
        let protocol = CallType::Standard;
        if dry_run {
            Ok(())
        } else {
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

    async fn _accounts_containers_workspaces_list(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        let mut call = self.hub.accounts().containers_workspaces_list(opt.value_of("parent").unwrap_or(""));
        for parg in opt.values_of("v").map(|i|i.collect()).unwrap_or(Vec::new()).iter() {
            let (key, value) = parse_kv_arg(&*parg, err, false);
            match key {
                "page-token" => {
                    call = call.page_token(value.unwrap_or(""));
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
                                                                           v.extend(["page-token"].iter().map(|v|*v));
                                                                           v } ));
                    }
                }
            }
        }
        let protocol = CallType::Standard;
        if dry_run {
            Ok(())
        } else {
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

    async fn _accounts_containers_workspaces_quick_preview(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        let mut call = self.hub.accounts().containers_workspaces_quick_preview(opt.value_of("path").unwrap_or(""));
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

    async fn _accounts_containers_workspaces_resolve_conflict(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
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
                    "built-in-variable.account-id" => Some(("builtInVariable.accountId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "built-in-variable.container-id" => Some(("builtInVariable.containerId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "built-in-variable.name" => Some(("builtInVariable.name", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "built-in-variable.path" => Some(("builtInVariable.path", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "built-in-variable.type" => Some(("builtInVariable.type", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "built-in-variable.workspace-id" => Some(("builtInVariable.workspaceId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "change-status" => Some(("changeStatus", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "client.account-id" => Some(("client.accountId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "client.client-id" => Some(("client.clientId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "client.container-id" => Some(("client.containerId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "client.fingerprint" => Some(("client.fingerprint", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "client.name" => Some(("client.name", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "client.notes" => Some(("client.notes", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "client.parent-folder-id" => Some(("client.parentFolderId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "client.path" => Some(("client.path", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "client.priority" => Some(("client.priority", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "client.tag-manager-url" => Some(("client.tagManagerUrl", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "client.type" => Some(("client.type", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "client.workspace-id" => Some(("client.workspaceId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "custom-template.account-id" => Some(("customTemplate.accountId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "custom-template.container-id" => Some(("customTemplate.containerId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "custom-template.fingerprint" => Some(("customTemplate.fingerprint", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "custom-template.gallery-reference.host" => Some(("customTemplate.galleryReference.host", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "custom-template.gallery-reference.is-modified" => Some(("customTemplate.galleryReference.isModified", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "custom-template.gallery-reference.owner" => Some(("customTemplate.galleryReference.owner", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "custom-template.gallery-reference.repository" => Some(("customTemplate.galleryReference.repository", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "custom-template.gallery-reference.signature" => Some(("customTemplate.galleryReference.signature", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "custom-template.gallery-reference.version" => Some(("customTemplate.galleryReference.version", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "custom-template.name" => Some(("customTemplate.name", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "custom-template.path" => Some(("customTemplate.path", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "custom-template.tag-manager-url" => Some(("customTemplate.tagManagerUrl", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "custom-template.template-data" => Some(("customTemplate.templateData", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "custom-template.template-id" => Some(("customTemplate.templateId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "custom-template.workspace-id" => Some(("customTemplate.workspaceId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "folder.account-id" => Some(("folder.accountId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "folder.container-id" => Some(("folder.containerId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "folder.fingerprint" => Some(("folder.fingerprint", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "folder.folder-id" => Some(("folder.folderId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "folder.name" => Some(("folder.name", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "folder.notes" => Some(("folder.notes", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "folder.path" => Some(("folder.path", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "folder.tag-manager-url" => Some(("folder.tagManagerUrl", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "folder.workspace-id" => Some(("folder.workspaceId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "gtag-config.account-id" => Some(("gtagConfig.accountId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "gtag-config.container-id" => Some(("gtagConfig.containerId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "gtag-config.fingerprint" => Some(("gtagConfig.fingerprint", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "gtag-config.gtag-config-id" => Some(("gtagConfig.gtagConfigId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "gtag-config.path" => Some(("gtagConfig.path", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "gtag-config.tag-manager-url" => Some(("gtagConfig.tagManagerUrl", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "gtag-config.type" => Some(("gtagConfig.type", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "gtag-config.workspace-id" => Some(("gtagConfig.workspaceId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "tag.account-id" => Some(("tag.accountId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "tag.blocking-rule-id" => Some(("tag.blockingRuleId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Vec })),
                    "tag.blocking-trigger-id" => Some(("tag.blockingTriggerId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Vec })),
                    "tag.consent-settings.consent-status" => Some(("tag.consentSettings.consentStatus", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "tag.consent-settings.consent-type.is-weak-reference" => Some(("tag.consentSettings.consentType.isWeakReference", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "tag.consent-settings.consent-type.key" => Some(("tag.consentSettings.consentType.key", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "tag.consent-settings.consent-type.type" => Some(("tag.consentSettings.consentType.type", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "tag.consent-settings.consent-type.value" => Some(("tag.consentSettings.consentType.value", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "tag.container-id" => Some(("tag.containerId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "tag.fingerprint" => Some(("tag.fingerprint", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "tag.firing-rule-id" => Some(("tag.firingRuleId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Vec })),
                    "tag.firing-trigger-id" => Some(("tag.firingTriggerId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Vec })),
                    "tag.live-only" => Some(("tag.liveOnly", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "tag.monitoring-metadata.is-weak-reference" => Some(("tag.monitoringMetadata.isWeakReference", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "tag.monitoring-metadata.key" => Some(("tag.monitoringMetadata.key", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "tag.monitoring-metadata.type" => Some(("tag.monitoringMetadata.type", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "tag.monitoring-metadata.value" => Some(("tag.monitoringMetadata.value", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "tag.monitoring-metadata-tag-name-key" => Some(("tag.monitoringMetadataTagNameKey", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "tag.name" => Some(("tag.name", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "tag.notes" => Some(("tag.notes", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "tag.parent-folder-id" => Some(("tag.parentFolderId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "tag.path" => Some(("tag.path", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "tag.paused" => Some(("tag.paused", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "tag.priority.is-weak-reference" => Some(("tag.priority.isWeakReference", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "tag.priority.key" => Some(("tag.priority.key", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "tag.priority.type" => Some(("tag.priority.type", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "tag.priority.value" => Some(("tag.priority.value", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "tag.schedule-end-ms" => Some(("tag.scheduleEndMs", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "tag.schedule-start-ms" => Some(("tag.scheduleStartMs", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "tag.tag-firing-option" => Some(("tag.tagFiringOption", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "tag.tag-id" => Some(("tag.tagId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "tag.tag-manager-url" => Some(("tag.tagManagerUrl", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "tag.type" => Some(("tag.type", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "tag.workspace-id" => Some(("tag.workspaceId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "transformation.account-id" => Some(("transformation.accountId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "transformation.container-id" => Some(("transformation.containerId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "transformation.fingerprint" => Some(("transformation.fingerprint", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "transformation.name" => Some(("transformation.name", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "transformation.notes" => Some(("transformation.notes", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "transformation.parent-folder-id" => Some(("transformation.parentFolderId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "transformation.path" => Some(("transformation.path", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "transformation.tag-manager-url" => Some(("transformation.tagManagerUrl", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "transformation.transformation-id" => Some(("transformation.transformationId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "transformation.type" => Some(("transformation.type", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "transformation.workspace-id" => Some(("transformation.workspaceId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "trigger.account-id" => Some(("trigger.accountId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "trigger.check-validation.is-weak-reference" => Some(("trigger.checkValidation.isWeakReference", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "trigger.check-validation.key" => Some(("trigger.checkValidation.key", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "trigger.check-validation.type" => Some(("trigger.checkValidation.type", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "trigger.check-validation.value" => Some(("trigger.checkValidation.value", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "trigger.container-id" => Some(("trigger.containerId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "trigger.continuous-time-min-milliseconds.is-weak-reference" => Some(("trigger.continuousTimeMinMilliseconds.isWeakReference", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "trigger.continuous-time-min-milliseconds.key" => Some(("trigger.continuousTimeMinMilliseconds.key", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "trigger.continuous-time-min-milliseconds.type" => Some(("trigger.continuousTimeMinMilliseconds.type", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "trigger.continuous-time-min-milliseconds.value" => Some(("trigger.continuousTimeMinMilliseconds.value", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "trigger.event-name.is-weak-reference" => Some(("trigger.eventName.isWeakReference", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "trigger.event-name.key" => Some(("trigger.eventName.key", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "trigger.event-name.type" => Some(("trigger.eventName.type", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "trigger.event-name.value" => Some(("trigger.eventName.value", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "trigger.fingerprint" => Some(("trigger.fingerprint", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "trigger.horizontal-scroll-percentage-list.is-weak-reference" => Some(("trigger.horizontalScrollPercentageList.isWeakReference", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "trigger.horizontal-scroll-percentage-list.key" => Some(("trigger.horizontalScrollPercentageList.key", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "trigger.horizontal-scroll-percentage-list.type" => Some(("trigger.horizontalScrollPercentageList.type", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "trigger.horizontal-scroll-percentage-list.value" => Some(("trigger.horizontalScrollPercentageList.value", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "trigger.interval.is-weak-reference" => Some(("trigger.interval.isWeakReference", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "trigger.interval.key" => Some(("trigger.interval.key", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "trigger.interval.type" => Some(("trigger.interval.type", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "trigger.interval.value" => Some(("trigger.interval.value", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "trigger.interval-seconds.is-weak-reference" => Some(("trigger.intervalSeconds.isWeakReference", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "trigger.interval-seconds.key" => Some(("trigger.intervalSeconds.key", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "trigger.interval-seconds.type" => Some(("trigger.intervalSeconds.type", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "trigger.interval-seconds.value" => Some(("trigger.intervalSeconds.value", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "trigger.limit.is-weak-reference" => Some(("trigger.limit.isWeakReference", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "trigger.limit.key" => Some(("trigger.limit.key", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "trigger.limit.type" => Some(("trigger.limit.type", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "trigger.limit.value" => Some(("trigger.limit.value", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "trigger.max-timer-length-seconds.is-weak-reference" => Some(("trigger.maxTimerLengthSeconds.isWeakReference", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "trigger.max-timer-length-seconds.key" => Some(("trigger.maxTimerLengthSeconds.key", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "trigger.max-timer-length-seconds.type" => Some(("trigger.maxTimerLengthSeconds.type", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "trigger.max-timer-length-seconds.value" => Some(("trigger.maxTimerLengthSeconds.value", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "trigger.name" => Some(("trigger.name", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "trigger.notes" => Some(("trigger.notes", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "trigger.parent-folder-id" => Some(("trigger.parentFolderId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "trigger.path" => Some(("trigger.path", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "trigger.selector.is-weak-reference" => Some(("trigger.selector.isWeakReference", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "trigger.selector.key" => Some(("trigger.selector.key", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "trigger.selector.type" => Some(("trigger.selector.type", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "trigger.selector.value" => Some(("trigger.selector.value", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "trigger.tag-manager-url" => Some(("trigger.tagManagerUrl", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "trigger.total-time-min-milliseconds.is-weak-reference" => Some(("trigger.totalTimeMinMilliseconds.isWeakReference", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "trigger.total-time-min-milliseconds.key" => Some(("trigger.totalTimeMinMilliseconds.key", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "trigger.total-time-min-milliseconds.type" => Some(("trigger.totalTimeMinMilliseconds.type", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "trigger.total-time-min-milliseconds.value" => Some(("trigger.totalTimeMinMilliseconds.value", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "trigger.trigger-id" => Some(("trigger.triggerId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "trigger.type" => Some(("trigger.type", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "trigger.unique-trigger-id.is-weak-reference" => Some(("trigger.uniqueTriggerId.isWeakReference", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "trigger.unique-trigger-id.key" => Some(("trigger.uniqueTriggerId.key", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "trigger.unique-trigger-id.type" => Some(("trigger.uniqueTriggerId.type", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "trigger.unique-trigger-id.value" => Some(("trigger.uniqueTriggerId.value", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "trigger.vertical-scroll-percentage-list.is-weak-reference" => Some(("trigger.verticalScrollPercentageList.isWeakReference", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "trigger.vertical-scroll-percentage-list.key" => Some(("trigger.verticalScrollPercentageList.key", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "trigger.vertical-scroll-percentage-list.type" => Some(("trigger.verticalScrollPercentageList.type", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "trigger.vertical-scroll-percentage-list.value" => Some(("trigger.verticalScrollPercentageList.value", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "trigger.visibility-selector.is-weak-reference" => Some(("trigger.visibilitySelector.isWeakReference", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "trigger.visibility-selector.key" => Some(("trigger.visibilitySelector.key", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "trigger.visibility-selector.type" => Some(("trigger.visibilitySelector.type", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "trigger.visibility-selector.value" => Some(("trigger.visibilitySelector.value", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "trigger.visible-percentage-max.is-weak-reference" => Some(("trigger.visiblePercentageMax.isWeakReference", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "trigger.visible-percentage-max.key" => Some(("trigger.visiblePercentageMax.key", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "trigger.visible-percentage-max.type" => Some(("trigger.visiblePercentageMax.type", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "trigger.visible-percentage-max.value" => Some(("trigger.visiblePercentageMax.value", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "trigger.visible-percentage-min.is-weak-reference" => Some(("trigger.visiblePercentageMin.isWeakReference", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "trigger.visible-percentage-min.key" => Some(("trigger.visiblePercentageMin.key", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "trigger.visible-percentage-min.type" => Some(("trigger.visiblePercentageMin.type", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "trigger.visible-percentage-min.value" => Some(("trigger.visiblePercentageMin.value", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "trigger.wait-for-tags.is-weak-reference" => Some(("trigger.waitForTags.isWeakReference", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "trigger.wait-for-tags.key" => Some(("trigger.waitForTags.key", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "trigger.wait-for-tags.type" => Some(("trigger.waitForTags.type", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "trigger.wait-for-tags.value" => Some(("trigger.waitForTags.value", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "trigger.wait-for-tags-timeout.is-weak-reference" => Some(("trigger.waitForTagsTimeout.isWeakReference", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "trigger.wait-for-tags-timeout.key" => Some(("trigger.waitForTagsTimeout.key", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "trigger.wait-for-tags-timeout.type" => Some(("trigger.waitForTagsTimeout.type", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "trigger.wait-for-tags-timeout.value" => Some(("trigger.waitForTagsTimeout.value", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "trigger.workspace-id" => Some(("trigger.workspaceId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "variable.account-id" => Some(("variable.accountId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "variable.container-id" => Some(("variable.containerId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "variable.disabling-trigger-id" => Some(("variable.disablingTriggerId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Vec })),
                    "variable.enabling-trigger-id" => Some(("variable.enablingTriggerId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Vec })),
                    "variable.fingerprint" => Some(("variable.fingerprint", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "variable.format-value.case-conversion-type" => Some(("variable.formatValue.caseConversionType", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "variable.format-value.convert-false-to-value.is-weak-reference" => Some(("variable.formatValue.convertFalseToValue.isWeakReference", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "variable.format-value.convert-false-to-value.key" => Some(("variable.formatValue.convertFalseToValue.key", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "variable.format-value.convert-false-to-value.type" => Some(("variable.formatValue.convertFalseToValue.type", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "variable.format-value.convert-false-to-value.value" => Some(("variable.formatValue.convertFalseToValue.value", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "variable.format-value.convert-null-to-value.is-weak-reference" => Some(("variable.formatValue.convertNullToValue.isWeakReference", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "variable.format-value.convert-null-to-value.key" => Some(("variable.formatValue.convertNullToValue.key", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "variable.format-value.convert-null-to-value.type" => Some(("variable.formatValue.convertNullToValue.type", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "variable.format-value.convert-null-to-value.value" => Some(("variable.formatValue.convertNullToValue.value", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "variable.format-value.convert-true-to-value.is-weak-reference" => Some(("variable.formatValue.convertTrueToValue.isWeakReference", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "variable.format-value.convert-true-to-value.key" => Some(("variable.formatValue.convertTrueToValue.key", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "variable.format-value.convert-true-to-value.type" => Some(("variable.formatValue.convertTrueToValue.type", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "variable.format-value.convert-true-to-value.value" => Some(("variable.formatValue.convertTrueToValue.value", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "variable.format-value.convert-undefined-to-value.is-weak-reference" => Some(("variable.formatValue.convertUndefinedToValue.isWeakReference", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "variable.format-value.convert-undefined-to-value.key" => Some(("variable.formatValue.convertUndefinedToValue.key", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "variable.format-value.convert-undefined-to-value.type" => Some(("variable.formatValue.convertUndefinedToValue.type", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "variable.format-value.convert-undefined-to-value.value" => Some(("variable.formatValue.convertUndefinedToValue.value", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "variable.name" => Some(("variable.name", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "variable.notes" => Some(("variable.notes", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "variable.parent-folder-id" => Some(("variable.parentFolderId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "variable.path" => Some(("variable.path", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "variable.schedule-end-ms" => Some(("variable.scheduleEndMs", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "variable.schedule-start-ms" => Some(("variable.scheduleStartMs", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "variable.tag-manager-url" => Some(("variable.tagManagerUrl", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "variable.type" => Some(("variable.type", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "variable.variable-id" => Some(("variable.variableId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "variable.workspace-id" => Some(("variable.workspaceId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "zone.account-id" => Some(("zone.accountId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "zone.boundary.custom-evaluation-trigger-id" => Some(("zone.boundary.customEvaluationTriggerId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Vec })),
                    "zone.container-id" => Some(("zone.containerId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "zone.fingerprint" => Some(("zone.fingerprint", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "zone.name" => Some(("zone.name", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "zone.notes" => Some(("zone.notes", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "zone.path" => Some(("zone.path", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "zone.tag-manager-url" => Some(("zone.tagManagerUrl", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "zone.type-restriction.enable" => Some(("zone.typeRestriction.enable", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "zone.type-restriction.whitelisted-type-id" => Some(("zone.typeRestriction.whitelistedTypeId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Vec })),
                    "zone.workspace-id" => Some(("zone.workspaceId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "zone.zone-id" => Some(("zone.zoneId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    _ => {
                        let suggestion = FieldCursor::did_you_mean(key, &vec!["account-id", "blocking-rule-id", "blocking-trigger-id", "boundary", "built-in-variable", "case-conversion-type", "change-status", "check-validation", "client", "client-id", "consent-settings", "consent-status", "consent-type", "container-id", "continuous-time-min-milliseconds", "convert-false-to-value", "convert-null-to-value", "convert-true-to-value", "convert-undefined-to-value", "custom-evaluation-trigger-id", "custom-template", "disabling-trigger-id", "enable", "enabling-trigger-id", "event-name", "fingerprint", "firing-rule-id", "firing-trigger-id", "folder", "folder-id", "format-value", "gallery-reference", "gtag-config", "gtag-config-id", "horizontal-scroll-percentage-list", "host", "interval", "interval-seconds", "is-modified", "is-weak-reference", "key", "limit", "live-only", "max-timer-length-seconds", "monitoring-metadata", "monitoring-metadata-tag-name-key", "name", "notes", "owner", "parent-folder-id", "path", "paused", "priority", "repository", "schedule-end-ms", "schedule-start-ms", "selector", "signature", "tag", "tag-firing-option", "tag-id", "tag-manager-url", "template-data", "template-id", "total-time-min-milliseconds", "transformation", "transformation-id", "trigger", "trigger-id", "type", "type-restriction", "unique-trigger-id", "value", "variable", "variable-id", "version", "vertical-scroll-percentage-list", "visibility-selector", "visible-percentage-max", "visible-percentage-min", "wait-for-tags", "wait-for-tags-timeout", "whitelisted-type-id", "workspace-id", "zone", "zone-id"]);
                        err.issues.push(CLIError::Field(FieldError::Unknown(temp_cursor.to_string(), suggestion, value.map(|v| v.to_string()))));
                        None
                    }
                };
            if let Some((field_cursor_str, type_info)) = type_info {
                FieldCursor::from(field_cursor_str).set_json_value(&mut object, value.unwrap(), type_info, err, &temp_cursor);
            }
        }
        let mut request: api::Entity = json::value::from_value(object).unwrap();
        let mut call = self.hub.accounts().containers_workspaces_resolve_conflict(request, opt.value_of("path").unwrap_or(""));
        for parg in opt.values_of("v").map(|i|i.collect()).unwrap_or(Vec::new()).iter() {
            let (key, value) = parse_kv_arg(&*parg, err, false);
            match key {
                "fingerprint" => {
                    call = call.fingerprint(value.unwrap_or(""));
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
                                                                           v.extend(["fingerprint"].iter().map(|v|*v));
                                                                           v } ));
                    }
                }
            }
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

    async fn _accounts_containers_workspaces_sync(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        let mut call = self.hub.accounts().containers_workspaces_sync(opt.value_of("path").unwrap_or(""));
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

    async fn _accounts_containers_workspaces_tags_create(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
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
                    "blocking-rule-id" => Some(("blockingRuleId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Vec })),
                    "blocking-trigger-id" => Some(("blockingTriggerId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Vec })),
                    "consent-settings.consent-status" => Some(("consentSettings.consentStatus", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "consent-settings.consent-type.is-weak-reference" => Some(("consentSettings.consentType.isWeakReference", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "consent-settings.consent-type.key" => Some(("consentSettings.consentType.key", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "consent-settings.consent-type.type" => Some(("consentSettings.consentType.type", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "consent-settings.consent-type.value" => Some(("consentSettings.consentType.value", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "container-id" => Some(("containerId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "fingerprint" => Some(("fingerprint", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "firing-rule-id" => Some(("firingRuleId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Vec })),
                    "firing-trigger-id" => Some(("firingTriggerId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Vec })),
                    "live-only" => Some(("liveOnly", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "monitoring-metadata.is-weak-reference" => Some(("monitoringMetadata.isWeakReference", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "monitoring-metadata.key" => Some(("monitoringMetadata.key", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "monitoring-metadata.type" => Some(("monitoringMetadata.type", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "monitoring-metadata.value" => Some(("monitoringMetadata.value", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "monitoring-metadata-tag-name-key" => Some(("monitoringMetadataTagNameKey", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "name" => Some(("name", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "notes" => Some(("notes", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "parent-folder-id" => Some(("parentFolderId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "path" => Some(("path", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "paused" => Some(("paused", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "priority.is-weak-reference" => Some(("priority.isWeakReference", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "priority.key" => Some(("priority.key", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "priority.type" => Some(("priority.type", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "priority.value" => Some(("priority.value", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "schedule-end-ms" => Some(("scheduleEndMs", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "schedule-start-ms" => Some(("scheduleStartMs", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "tag-firing-option" => Some(("tagFiringOption", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "tag-id" => Some(("tagId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "tag-manager-url" => Some(("tagManagerUrl", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "type" => Some(("type", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "workspace-id" => Some(("workspaceId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    _ => {
                        let suggestion = FieldCursor::did_you_mean(key, &vec!["account-id", "blocking-rule-id", "blocking-trigger-id", "consent-settings", "consent-status", "consent-type", "container-id", "fingerprint", "firing-rule-id", "firing-trigger-id", "is-weak-reference", "key", "live-only", "monitoring-metadata", "monitoring-metadata-tag-name-key", "name", "notes", "parent-folder-id", "path", "paused", "priority", "schedule-end-ms", "schedule-start-ms", "tag-firing-option", "tag-id", "tag-manager-url", "type", "value", "workspace-id"]);
                        err.issues.push(CLIError::Field(FieldError::Unknown(temp_cursor.to_string(), suggestion, value.map(|v| v.to_string()))));
                        None
                    }
                };
            if let Some((field_cursor_str, type_info)) = type_info {
                FieldCursor::from(field_cursor_str).set_json_value(&mut object, value.unwrap(), type_info, err, &temp_cursor);
            }
        }
        let mut request: api::Tag = json::value::from_value(object).unwrap();
        let mut call = self.hub.accounts().containers_workspaces_tags_create(request, opt.value_of("parent").unwrap_or(""));
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

    async fn _accounts_containers_workspaces_tags_delete(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        let mut call = self.hub.accounts().containers_workspaces_tags_delete(opt.value_of("path").unwrap_or(""));
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

    async fn _accounts_containers_workspaces_tags_get(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        let mut call = self.hub.accounts().containers_workspaces_tags_get(opt.value_of("path").unwrap_or(""));
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

    async fn _accounts_containers_workspaces_tags_list(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        let mut call = self.hub.accounts().containers_workspaces_tags_list(opt.value_of("parent").unwrap_or(""));
        for parg in opt.values_of("v").map(|i|i.collect()).unwrap_or(Vec::new()).iter() {
            let (key, value) = parse_kv_arg(&*parg, err, false);
            match key {
                "page-token" => {
                    call = call.page_token(value.unwrap_or(""));
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
                                                                           v.extend(["page-token"].iter().map(|v|*v));
                                                                           v } ));
                    }
                }
            }
        }
        let protocol = CallType::Standard;
        if dry_run {
            Ok(())
        } else {
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

    async fn _accounts_containers_workspaces_tags_revert(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        let mut call = self.hub.accounts().containers_workspaces_tags_revert(opt.value_of("path").unwrap_or(""));
        for parg in opt.values_of("v").map(|i|i.collect()).unwrap_or(Vec::new()).iter() {
            let (key, value) = parse_kv_arg(&*parg, err, false);
            match key {
                "fingerprint" => {
                    call = call.fingerprint(value.unwrap_or(""));
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
                                                                           v.extend(["fingerprint"].iter().map(|v|*v));
                                                                           v } ));
                    }
                }
            }
        }
        let protocol = CallType::Standard;
        if dry_run {
            Ok(())
        } else {
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

    async fn _accounts_containers_workspaces_tags_update(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
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
                    "blocking-rule-id" => Some(("blockingRuleId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Vec })),
                    "blocking-trigger-id" => Some(("blockingTriggerId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Vec })),
                    "consent-settings.consent-status" => Some(("consentSettings.consentStatus", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "consent-settings.consent-type.is-weak-reference" => Some(("consentSettings.consentType.isWeakReference", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "consent-settings.consent-type.key" => Some(("consentSettings.consentType.key", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "consent-settings.consent-type.type" => Some(("consentSettings.consentType.type", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "consent-settings.consent-type.value" => Some(("consentSettings.consentType.value", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "container-id" => Some(("containerId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "fingerprint" => Some(("fingerprint", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "firing-rule-id" => Some(("firingRuleId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Vec })),
                    "firing-trigger-id" => Some(("firingTriggerId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Vec })),
                    "live-only" => Some(("liveOnly", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "monitoring-metadata.is-weak-reference" => Some(("monitoringMetadata.isWeakReference", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "monitoring-metadata.key" => Some(("monitoringMetadata.key", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "monitoring-metadata.type" => Some(("monitoringMetadata.type", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "monitoring-metadata.value" => Some(("monitoringMetadata.value", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "monitoring-metadata-tag-name-key" => Some(("monitoringMetadataTagNameKey", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "name" => Some(("name", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "notes" => Some(("notes", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "parent-folder-id" => Some(("parentFolderId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "path" => Some(("path", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "paused" => Some(("paused", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "priority.is-weak-reference" => Some(("priority.isWeakReference", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "priority.key" => Some(("priority.key", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "priority.type" => Some(("priority.type", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "priority.value" => Some(("priority.value", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "schedule-end-ms" => Some(("scheduleEndMs", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "schedule-start-ms" => Some(("scheduleStartMs", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "tag-firing-option" => Some(("tagFiringOption", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "tag-id" => Some(("tagId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "tag-manager-url" => Some(("tagManagerUrl", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "type" => Some(("type", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "workspace-id" => Some(("workspaceId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    _ => {
                        let suggestion = FieldCursor::did_you_mean(key, &vec!["account-id", "blocking-rule-id", "blocking-trigger-id", "consent-settings", "consent-status", "consent-type", "container-id", "fingerprint", "firing-rule-id", "firing-trigger-id", "is-weak-reference", "key", "live-only", "monitoring-metadata", "monitoring-metadata-tag-name-key", "name", "notes", "parent-folder-id", "path", "paused", "priority", "schedule-end-ms", "schedule-start-ms", "tag-firing-option", "tag-id", "tag-manager-url", "type", "value", "workspace-id"]);
                        err.issues.push(CLIError::Field(FieldError::Unknown(temp_cursor.to_string(), suggestion, value.map(|v| v.to_string()))));
                        None
                    }
                };
            if let Some((field_cursor_str, type_info)) = type_info {
                FieldCursor::from(field_cursor_str).set_json_value(&mut object, value.unwrap(), type_info, err, &temp_cursor);
            }
        }
        let mut request: api::Tag = json::value::from_value(object).unwrap();
        let mut call = self.hub.accounts().containers_workspaces_tags_update(request, opt.value_of("path").unwrap_or(""));
        for parg in opt.values_of("v").map(|i|i.collect()).unwrap_or(Vec::new()).iter() {
            let (key, value) = parse_kv_arg(&*parg, err, false);
            match key {
                "fingerprint" => {
                    call = call.fingerprint(value.unwrap_or(""));
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
                                                                           v.extend(["fingerprint"].iter().map(|v|*v));
                                                                           v } ));
                    }
                }
            }
        }
        let protocol = CallType::Standard;
        if dry_run {
            Ok(())
        } else {
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

    async fn _accounts_containers_workspaces_templates_create(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
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
                    "container-id" => Some(("containerId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "fingerprint" => Some(("fingerprint", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "gallery-reference.host" => Some(("galleryReference.host", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "gallery-reference.is-modified" => Some(("galleryReference.isModified", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "gallery-reference.owner" => Some(("galleryReference.owner", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "gallery-reference.repository" => Some(("galleryReference.repository", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "gallery-reference.signature" => Some(("galleryReference.signature", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "gallery-reference.version" => Some(("galleryReference.version", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "name" => Some(("name", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "path" => Some(("path", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "tag-manager-url" => Some(("tagManagerUrl", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "template-data" => Some(("templateData", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "template-id" => Some(("templateId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "workspace-id" => Some(("workspaceId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    _ => {
                        let suggestion = FieldCursor::did_you_mean(key, &vec!["account-id", "container-id", "fingerprint", "gallery-reference", "host", "is-modified", "name", "owner", "path", "repository", "signature", "tag-manager-url", "template-data", "template-id", "version", "workspace-id"]);
                        err.issues.push(CLIError::Field(FieldError::Unknown(temp_cursor.to_string(), suggestion, value.map(|v| v.to_string()))));
                        None
                    }
                };
            if let Some((field_cursor_str, type_info)) = type_info {
                FieldCursor::from(field_cursor_str).set_json_value(&mut object, value.unwrap(), type_info, err, &temp_cursor);
            }
        }
        let mut request: api::CustomTemplate = json::value::from_value(object).unwrap();
        let mut call = self.hub.accounts().containers_workspaces_templates_create(request, opt.value_of("parent").unwrap_or(""));
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

    async fn _accounts_containers_workspaces_templates_delete(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        let mut call = self.hub.accounts().containers_workspaces_templates_delete(opt.value_of("path").unwrap_or(""));
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

    async fn _accounts_containers_workspaces_templates_get(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        let mut call = self.hub.accounts().containers_workspaces_templates_get(opt.value_of("path").unwrap_or(""));
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

    async fn _accounts_containers_workspaces_templates_list(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        let mut call = self.hub.accounts().containers_workspaces_templates_list(opt.value_of("parent").unwrap_or(""));
        for parg in opt.values_of("v").map(|i|i.collect()).unwrap_or(Vec::new()).iter() {
            let (key, value) = parse_kv_arg(&*parg, err, false);
            match key {
                "page-token" => {
                    call = call.page_token(value.unwrap_or(""));
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
                                                                           v.extend(["page-token"].iter().map(|v|*v));
                                                                           v } ));
                    }
                }
            }
        }
        let protocol = CallType::Standard;
        if dry_run {
            Ok(())
        } else {
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

    async fn _accounts_containers_workspaces_templates_revert(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        let mut call = self.hub.accounts().containers_workspaces_templates_revert(opt.value_of("path").unwrap_or(""));
        for parg in opt.values_of("v").map(|i|i.collect()).unwrap_or(Vec::new()).iter() {
            let (key, value) = parse_kv_arg(&*parg, err, false);
            match key {
                "fingerprint" => {
                    call = call.fingerprint(value.unwrap_or(""));
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
                                                                           v.extend(["fingerprint"].iter().map(|v|*v));
                                                                           v } ));
                    }
                }
            }
        }
        let protocol = CallType::Standard;
        if dry_run {
            Ok(())
        } else {
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

    async fn _accounts_containers_workspaces_templates_update(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
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
                    "container-id" => Some(("containerId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "fingerprint" => Some(("fingerprint", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "gallery-reference.host" => Some(("galleryReference.host", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "gallery-reference.is-modified" => Some(("galleryReference.isModified", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "gallery-reference.owner" => Some(("galleryReference.owner", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "gallery-reference.repository" => Some(("galleryReference.repository", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "gallery-reference.signature" => Some(("galleryReference.signature", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "gallery-reference.version" => Some(("galleryReference.version", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "name" => Some(("name", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "path" => Some(("path", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "tag-manager-url" => Some(("tagManagerUrl", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "template-data" => Some(("templateData", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "template-id" => Some(("templateId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "workspace-id" => Some(("workspaceId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    _ => {
                        let suggestion = FieldCursor::did_you_mean(key, &vec!["account-id", "container-id", "fingerprint", "gallery-reference", "host", "is-modified", "name", "owner", "path", "repository", "signature", "tag-manager-url", "template-data", "template-id", "version", "workspace-id"]);
                        err.issues.push(CLIError::Field(FieldError::Unknown(temp_cursor.to_string(), suggestion, value.map(|v| v.to_string()))));
                        None
                    }
                };
            if let Some((field_cursor_str, type_info)) = type_info {
                FieldCursor::from(field_cursor_str).set_json_value(&mut object, value.unwrap(), type_info, err, &temp_cursor);
            }
        }
        let mut request: api::CustomTemplate = json::value::from_value(object).unwrap();
        let mut call = self.hub.accounts().containers_workspaces_templates_update(request, opt.value_of("path").unwrap_or(""));
        for parg in opt.values_of("v").map(|i|i.collect()).unwrap_or(Vec::new()).iter() {
            let (key, value) = parse_kv_arg(&*parg, err, false);
            match key {
                "fingerprint" => {
                    call = call.fingerprint(value.unwrap_or(""));
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
                                                                           v.extend(["fingerprint"].iter().map(|v|*v));
                                                                           v } ));
                    }
                }
            }
        }
        let protocol = CallType::Standard;
        if dry_run {
            Ok(())
        } else {
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

    async fn _accounts_containers_workspaces_transformations_create(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
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
                    "container-id" => Some(("containerId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "fingerprint" => Some(("fingerprint", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "name" => Some(("name", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "notes" => Some(("notes", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "parent-folder-id" => Some(("parentFolderId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "path" => Some(("path", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "tag-manager-url" => Some(("tagManagerUrl", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "transformation-id" => Some(("transformationId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "type" => Some(("type", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "workspace-id" => Some(("workspaceId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    _ => {
                        let suggestion = FieldCursor::did_you_mean(key, &vec!["account-id", "container-id", "fingerprint", "name", "notes", "parent-folder-id", "path", "tag-manager-url", "transformation-id", "type", "workspace-id"]);
                        err.issues.push(CLIError::Field(FieldError::Unknown(temp_cursor.to_string(), suggestion, value.map(|v| v.to_string()))));
                        None
                    }
                };
            if let Some((field_cursor_str, type_info)) = type_info {
                FieldCursor::from(field_cursor_str).set_json_value(&mut object, value.unwrap(), type_info, err, &temp_cursor);
            }
        }
        let mut request: api::Transformation = json::value::from_value(object).unwrap();
        let mut call = self.hub.accounts().containers_workspaces_transformations_create(request, opt.value_of("parent").unwrap_or(""));
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

    async fn _accounts_containers_workspaces_transformations_delete(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        let mut call = self.hub.accounts().containers_workspaces_transformations_delete(opt.value_of("path").unwrap_or(""));
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

    async fn _accounts_containers_workspaces_transformations_get(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        let mut call = self.hub.accounts().containers_workspaces_transformations_get(opt.value_of("path").unwrap_or(""));
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

    async fn _accounts_containers_workspaces_transformations_list(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        let mut call = self.hub.accounts().containers_workspaces_transformations_list(opt.value_of("parent").unwrap_or(""));
        for parg in opt.values_of("v").map(|i|i.collect()).unwrap_or(Vec::new()).iter() {
            let (key, value) = parse_kv_arg(&*parg, err, false);
            match key {
                "page-token" => {
                    call = call.page_token(value.unwrap_or(""));
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
                                                                           v.extend(["page-token"].iter().map(|v|*v));
                                                                           v } ));
                    }
                }
            }
        }
        let protocol = CallType::Standard;
        if dry_run {
            Ok(())
        } else {
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

    async fn _accounts_containers_workspaces_transformations_revert(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        let mut call = self.hub.accounts().containers_workspaces_transformations_revert(opt.value_of("path").unwrap_or(""));
        for parg in opt.values_of("v").map(|i|i.collect()).unwrap_or(Vec::new()).iter() {
            let (key, value) = parse_kv_arg(&*parg, err, false);
            match key {
                "fingerprint" => {
                    call = call.fingerprint(value.unwrap_or(""));
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
                                                                           v.extend(["fingerprint"].iter().map(|v|*v));
                                                                           v } ));
                    }
                }
            }
        }
        let protocol = CallType::Standard;
        if dry_run {
            Ok(())
        } else {
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

    async fn _accounts_containers_workspaces_transformations_update(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
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
                    "container-id" => Some(("containerId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "fingerprint" => Some(("fingerprint", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "name" => Some(("name", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "notes" => Some(("notes", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "parent-folder-id" => Some(("parentFolderId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "path" => Some(("path", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "tag-manager-url" => Some(("tagManagerUrl", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "transformation-id" => Some(("transformationId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "type" => Some(("type", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "workspace-id" => Some(("workspaceId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    _ => {
                        let suggestion = FieldCursor::did_you_mean(key, &vec!["account-id", "container-id", "fingerprint", "name", "notes", "parent-folder-id", "path", "tag-manager-url", "transformation-id", "type", "workspace-id"]);
                        err.issues.push(CLIError::Field(FieldError::Unknown(temp_cursor.to_string(), suggestion, value.map(|v| v.to_string()))));
                        None
                    }
                };
            if let Some((field_cursor_str, type_info)) = type_info {
                FieldCursor::from(field_cursor_str).set_json_value(&mut object, value.unwrap(), type_info, err, &temp_cursor);
            }
        }
        let mut request: api::Transformation = json::value::from_value(object).unwrap();
        let mut call = self.hub.accounts().containers_workspaces_transformations_update(request, opt.value_of("path").unwrap_or(""));
        for parg in opt.values_of("v").map(|i|i.collect()).unwrap_or(Vec::new()).iter() {
            let (key, value) = parse_kv_arg(&*parg, err, false);
            match key {
                "fingerprint" => {
                    call = call.fingerprint(value.unwrap_or(""));
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
                                                                           v.extend(["fingerprint"].iter().map(|v|*v));
                                                                           v } ));
                    }
                }
            }
        }
        let protocol = CallType::Standard;
        if dry_run {
            Ok(())
        } else {
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

    async fn _accounts_containers_workspaces_triggers_create(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
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
                    "check-validation.is-weak-reference" => Some(("checkValidation.isWeakReference", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "check-validation.key" => Some(("checkValidation.key", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "check-validation.type" => Some(("checkValidation.type", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "check-validation.value" => Some(("checkValidation.value", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "container-id" => Some(("containerId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "continuous-time-min-milliseconds.is-weak-reference" => Some(("continuousTimeMinMilliseconds.isWeakReference", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "continuous-time-min-milliseconds.key" => Some(("continuousTimeMinMilliseconds.key", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "continuous-time-min-milliseconds.type" => Some(("continuousTimeMinMilliseconds.type", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "continuous-time-min-milliseconds.value" => Some(("continuousTimeMinMilliseconds.value", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "event-name.is-weak-reference" => Some(("eventName.isWeakReference", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "event-name.key" => Some(("eventName.key", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "event-name.type" => Some(("eventName.type", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "event-name.value" => Some(("eventName.value", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "fingerprint" => Some(("fingerprint", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "horizontal-scroll-percentage-list.is-weak-reference" => Some(("horizontalScrollPercentageList.isWeakReference", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "horizontal-scroll-percentage-list.key" => Some(("horizontalScrollPercentageList.key", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "horizontal-scroll-percentage-list.type" => Some(("horizontalScrollPercentageList.type", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "horizontal-scroll-percentage-list.value" => Some(("horizontalScrollPercentageList.value", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "interval.is-weak-reference" => Some(("interval.isWeakReference", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "interval.key" => Some(("interval.key", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "interval.type" => Some(("interval.type", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "interval.value" => Some(("interval.value", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "interval-seconds.is-weak-reference" => Some(("intervalSeconds.isWeakReference", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "interval-seconds.key" => Some(("intervalSeconds.key", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "interval-seconds.type" => Some(("intervalSeconds.type", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "interval-seconds.value" => Some(("intervalSeconds.value", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "limit.is-weak-reference" => Some(("limit.isWeakReference", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "limit.key" => Some(("limit.key", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "limit.type" => Some(("limit.type", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "limit.value" => Some(("limit.value", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "max-timer-length-seconds.is-weak-reference" => Some(("maxTimerLengthSeconds.isWeakReference", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "max-timer-length-seconds.key" => Some(("maxTimerLengthSeconds.key", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "max-timer-length-seconds.type" => Some(("maxTimerLengthSeconds.type", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "max-timer-length-seconds.value" => Some(("maxTimerLengthSeconds.value", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "name" => Some(("name", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "notes" => Some(("notes", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "parent-folder-id" => Some(("parentFolderId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "path" => Some(("path", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "selector.is-weak-reference" => Some(("selector.isWeakReference", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "selector.key" => Some(("selector.key", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "selector.type" => Some(("selector.type", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "selector.value" => Some(("selector.value", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "tag-manager-url" => Some(("tagManagerUrl", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "total-time-min-milliseconds.is-weak-reference" => Some(("totalTimeMinMilliseconds.isWeakReference", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "total-time-min-milliseconds.key" => Some(("totalTimeMinMilliseconds.key", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "total-time-min-milliseconds.type" => Some(("totalTimeMinMilliseconds.type", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "total-time-min-milliseconds.value" => Some(("totalTimeMinMilliseconds.value", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "trigger-id" => Some(("triggerId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "type" => Some(("type", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "unique-trigger-id.is-weak-reference" => Some(("uniqueTriggerId.isWeakReference", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "unique-trigger-id.key" => Some(("uniqueTriggerId.key", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "unique-trigger-id.type" => Some(("uniqueTriggerId.type", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "unique-trigger-id.value" => Some(("uniqueTriggerId.value", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "vertical-scroll-percentage-list.is-weak-reference" => Some(("verticalScrollPercentageList.isWeakReference", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "vertical-scroll-percentage-list.key" => Some(("verticalScrollPercentageList.key", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "vertical-scroll-percentage-list.type" => Some(("verticalScrollPercentageList.type", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "vertical-scroll-percentage-list.value" => Some(("verticalScrollPercentageList.value", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "visibility-selector.is-weak-reference" => Some(("visibilitySelector.isWeakReference", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "visibility-selector.key" => Some(("visibilitySelector.key", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "visibility-selector.type" => Some(("visibilitySelector.type", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "visibility-selector.value" => Some(("visibilitySelector.value", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "visible-percentage-max.is-weak-reference" => Some(("visiblePercentageMax.isWeakReference", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "visible-percentage-max.key" => Some(("visiblePercentageMax.key", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "visible-percentage-max.type" => Some(("visiblePercentageMax.type", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "visible-percentage-max.value" => Some(("visiblePercentageMax.value", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "visible-percentage-min.is-weak-reference" => Some(("visiblePercentageMin.isWeakReference", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "visible-percentage-min.key" => Some(("visiblePercentageMin.key", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "visible-percentage-min.type" => Some(("visiblePercentageMin.type", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "visible-percentage-min.value" => Some(("visiblePercentageMin.value", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "wait-for-tags.is-weak-reference" => Some(("waitForTags.isWeakReference", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "wait-for-tags.key" => Some(("waitForTags.key", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "wait-for-tags.type" => Some(("waitForTags.type", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "wait-for-tags.value" => Some(("waitForTags.value", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "wait-for-tags-timeout.is-weak-reference" => Some(("waitForTagsTimeout.isWeakReference", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "wait-for-tags-timeout.key" => Some(("waitForTagsTimeout.key", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "wait-for-tags-timeout.type" => Some(("waitForTagsTimeout.type", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "wait-for-tags-timeout.value" => Some(("waitForTagsTimeout.value", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "workspace-id" => Some(("workspaceId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    _ => {
                        let suggestion = FieldCursor::did_you_mean(key, &vec!["account-id", "check-validation", "container-id", "continuous-time-min-milliseconds", "event-name", "fingerprint", "horizontal-scroll-percentage-list", "interval", "interval-seconds", "is-weak-reference", "key", "limit", "max-timer-length-seconds", "name", "notes", "parent-folder-id", "path", "selector", "tag-manager-url", "total-time-min-milliseconds", "trigger-id", "type", "unique-trigger-id", "value", "vertical-scroll-percentage-list", "visibility-selector", "visible-percentage-max", "visible-percentage-min", "wait-for-tags", "wait-for-tags-timeout", "workspace-id"]);
                        err.issues.push(CLIError::Field(FieldError::Unknown(temp_cursor.to_string(), suggestion, value.map(|v| v.to_string()))));
                        None
                    }
                };
            if let Some((field_cursor_str, type_info)) = type_info {
                FieldCursor::from(field_cursor_str).set_json_value(&mut object, value.unwrap(), type_info, err, &temp_cursor);
            }
        }
        let mut request: api::Trigger = json::value::from_value(object).unwrap();
        let mut call = self.hub.accounts().containers_workspaces_triggers_create(request, opt.value_of("parent").unwrap_or(""));
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

    async fn _accounts_containers_workspaces_triggers_delete(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        let mut call = self.hub.accounts().containers_workspaces_triggers_delete(opt.value_of("path").unwrap_or(""));
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

    async fn _accounts_containers_workspaces_triggers_get(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        let mut call = self.hub.accounts().containers_workspaces_triggers_get(opt.value_of("path").unwrap_or(""));
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

    async fn _accounts_containers_workspaces_triggers_list(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        let mut call = self.hub.accounts().containers_workspaces_triggers_list(opt.value_of("parent").unwrap_or(""));
        for parg in opt.values_of("v").map(|i|i.collect()).unwrap_or(Vec::new()).iter() {
            let (key, value) = parse_kv_arg(&*parg, err, false);
            match key {
                "page-token" => {
                    call = call.page_token(value.unwrap_or(""));
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
                                                                           v.extend(["page-token"].iter().map(|v|*v));
                                                                           v } ));
                    }
                }
            }
        }
        let protocol = CallType::Standard;
        if dry_run {
            Ok(())
        } else {
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

    async fn _accounts_containers_workspaces_triggers_revert(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        let mut call = self.hub.accounts().containers_workspaces_triggers_revert(opt.value_of("path").unwrap_or(""));
        for parg in opt.values_of("v").map(|i|i.collect()).unwrap_or(Vec::new()).iter() {
            let (key, value) = parse_kv_arg(&*parg, err, false);
            match key {
                "fingerprint" => {
                    call = call.fingerprint(value.unwrap_or(""));
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
                                                                           v.extend(["fingerprint"].iter().map(|v|*v));
                                                                           v } ));
                    }
                }
            }
        }
        let protocol = CallType::Standard;
        if dry_run {
            Ok(())
        } else {
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

    async fn _accounts_containers_workspaces_triggers_update(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
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
                    "check-validation.is-weak-reference" => Some(("checkValidation.isWeakReference", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "check-validation.key" => Some(("checkValidation.key", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "check-validation.type" => Some(("checkValidation.type", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "check-validation.value" => Some(("checkValidation.value", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "container-id" => Some(("containerId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "continuous-time-min-milliseconds.is-weak-reference" => Some(("continuousTimeMinMilliseconds.isWeakReference", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "continuous-time-min-milliseconds.key" => Some(("continuousTimeMinMilliseconds.key", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "continuous-time-min-milliseconds.type" => Some(("continuousTimeMinMilliseconds.type", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "continuous-time-min-milliseconds.value" => Some(("continuousTimeMinMilliseconds.value", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "event-name.is-weak-reference" => Some(("eventName.isWeakReference", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "event-name.key" => Some(("eventName.key", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "event-name.type" => Some(("eventName.type", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "event-name.value" => Some(("eventName.value", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "fingerprint" => Some(("fingerprint", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "horizontal-scroll-percentage-list.is-weak-reference" => Some(("horizontalScrollPercentageList.isWeakReference", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "horizontal-scroll-percentage-list.key" => Some(("horizontalScrollPercentageList.key", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "horizontal-scroll-percentage-list.type" => Some(("horizontalScrollPercentageList.type", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "horizontal-scroll-percentage-list.value" => Some(("horizontalScrollPercentageList.value", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "interval.is-weak-reference" => Some(("interval.isWeakReference", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "interval.key" => Some(("interval.key", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "interval.type" => Some(("interval.type", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "interval.value" => Some(("interval.value", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "interval-seconds.is-weak-reference" => Some(("intervalSeconds.isWeakReference", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "interval-seconds.key" => Some(("intervalSeconds.key", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "interval-seconds.type" => Some(("intervalSeconds.type", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "interval-seconds.value" => Some(("intervalSeconds.value", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "limit.is-weak-reference" => Some(("limit.isWeakReference", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "limit.key" => Some(("limit.key", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "limit.type" => Some(("limit.type", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "limit.value" => Some(("limit.value", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "max-timer-length-seconds.is-weak-reference" => Some(("maxTimerLengthSeconds.isWeakReference", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "max-timer-length-seconds.key" => Some(("maxTimerLengthSeconds.key", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "max-timer-length-seconds.type" => Some(("maxTimerLengthSeconds.type", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "max-timer-length-seconds.value" => Some(("maxTimerLengthSeconds.value", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "name" => Some(("name", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "notes" => Some(("notes", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "parent-folder-id" => Some(("parentFolderId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "path" => Some(("path", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "selector.is-weak-reference" => Some(("selector.isWeakReference", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "selector.key" => Some(("selector.key", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "selector.type" => Some(("selector.type", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "selector.value" => Some(("selector.value", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "tag-manager-url" => Some(("tagManagerUrl", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "total-time-min-milliseconds.is-weak-reference" => Some(("totalTimeMinMilliseconds.isWeakReference", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "total-time-min-milliseconds.key" => Some(("totalTimeMinMilliseconds.key", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "total-time-min-milliseconds.type" => Some(("totalTimeMinMilliseconds.type", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "total-time-min-milliseconds.value" => Some(("totalTimeMinMilliseconds.value", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "trigger-id" => Some(("triggerId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "type" => Some(("type", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "unique-trigger-id.is-weak-reference" => Some(("uniqueTriggerId.isWeakReference", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "unique-trigger-id.key" => Some(("uniqueTriggerId.key", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "unique-trigger-id.type" => Some(("uniqueTriggerId.type", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "unique-trigger-id.value" => Some(("uniqueTriggerId.value", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "vertical-scroll-percentage-list.is-weak-reference" => Some(("verticalScrollPercentageList.isWeakReference", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "vertical-scroll-percentage-list.key" => Some(("verticalScrollPercentageList.key", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "vertical-scroll-percentage-list.type" => Some(("verticalScrollPercentageList.type", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "vertical-scroll-percentage-list.value" => Some(("verticalScrollPercentageList.value", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "visibility-selector.is-weak-reference" => Some(("visibilitySelector.isWeakReference", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "visibility-selector.key" => Some(("visibilitySelector.key", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "visibility-selector.type" => Some(("visibilitySelector.type", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "visibility-selector.value" => Some(("visibilitySelector.value", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "visible-percentage-max.is-weak-reference" => Some(("visiblePercentageMax.isWeakReference", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "visible-percentage-max.key" => Some(("visiblePercentageMax.key", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "visible-percentage-max.type" => Some(("visiblePercentageMax.type", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "visible-percentage-max.value" => Some(("visiblePercentageMax.value", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "visible-percentage-min.is-weak-reference" => Some(("visiblePercentageMin.isWeakReference", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "visible-percentage-min.key" => Some(("visiblePercentageMin.key", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "visible-percentage-min.type" => Some(("visiblePercentageMin.type", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "visible-percentage-min.value" => Some(("visiblePercentageMin.value", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "wait-for-tags.is-weak-reference" => Some(("waitForTags.isWeakReference", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "wait-for-tags.key" => Some(("waitForTags.key", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "wait-for-tags.type" => Some(("waitForTags.type", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "wait-for-tags.value" => Some(("waitForTags.value", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "wait-for-tags-timeout.is-weak-reference" => Some(("waitForTagsTimeout.isWeakReference", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "wait-for-tags-timeout.key" => Some(("waitForTagsTimeout.key", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "wait-for-tags-timeout.type" => Some(("waitForTagsTimeout.type", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "wait-for-tags-timeout.value" => Some(("waitForTagsTimeout.value", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "workspace-id" => Some(("workspaceId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    _ => {
                        let suggestion = FieldCursor::did_you_mean(key, &vec!["account-id", "check-validation", "container-id", "continuous-time-min-milliseconds", "event-name", "fingerprint", "horizontal-scroll-percentage-list", "interval", "interval-seconds", "is-weak-reference", "key", "limit", "max-timer-length-seconds", "name", "notes", "parent-folder-id", "path", "selector", "tag-manager-url", "total-time-min-milliseconds", "trigger-id", "type", "unique-trigger-id", "value", "vertical-scroll-percentage-list", "visibility-selector", "visible-percentage-max", "visible-percentage-min", "wait-for-tags", "wait-for-tags-timeout", "workspace-id"]);
                        err.issues.push(CLIError::Field(FieldError::Unknown(temp_cursor.to_string(), suggestion, value.map(|v| v.to_string()))));
                        None
                    }
                };
            if let Some((field_cursor_str, type_info)) = type_info {
                FieldCursor::from(field_cursor_str).set_json_value(&mut object, value.unwrap(), type_info, err, &temp_cursor);
            }
        }
        let mut request: api::Trigger = json::value::from_value(object).unwrap();
        let mut call = self.hub.accounts().containers_workspaces_triggers_update(request, opt.value_of("path").unwrap_or(""));
        for parg in opt.values_of("v").map(|i|i.collect()).unwrap_or(Vec::new()).iter() {
            let (key, value) = parse_kv_arg(&*parg, err, false);
            match key {
                "fingerprint" => {
                    call = call.fingerprint(value.unwrap_or(""));
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
                                                                           v.extend(["fingerprint"].iter().map(|v|*v));
                                                                           v } ));
                    }
                }
            }
        }
        let protocol = CallType::Standard;
        if dry_run {
            Ok(())
        } else {
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

    async fn _accounts_containers_workspaces_update(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
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
                    "container-id" => Some(("containerId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "description" => Some(("description", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "fingerprint" => Some(("fingerprint", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "name" => Some(("name", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "path" => Some(("path", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "tag-manager-url" => Some(("tagManagerUrl", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "workspace-id" => Some(("workspaceId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    _ => {
                        let suggestion = FieldCursor::did_you_mean(key, &vec!["account-id", "container-id", "description", "fingerprint", "name", "path", "tag-manager-url", "workspace-id"]);
                        err.issues.push(CLIError::Field(FieldError::Unknown(temp_cursor.to_string(), suggestion, value.map(|v| v.to_string()))));
                        None
                    }
                };
            if let Some((field_cursor_str, type_info)) = type_info {
                FieldCursor::from(field_cursor_str).set_json_value(&mut object, value.unwrap(), type_info, err, &temp_cursor);
            }
        }
        let mut request: api::Workspace = json::value::from_value(object).unwrap();
        let mut call = self.hub.accounts().containers_workspaces_update(request, opt.value_of("path").unwrap_or(""));
        for parg in opt.values_of("v").map(|i|i.collect()).unwrap_or(Vec::new()).iter() {
            let (key, value) = parse_kv_arg(&*parg, err, false);
            match key {
                "fingerprint" => {
                    call = call.fingerprint(value.unwrap_or(""));
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
                                                                           v.extend(["fingerprint"].iter().map(|v|*v));
                                                                           v } ));
                    }
                }
            }
        }
        let protocol = CallType::Standard;
        if dry_run {
            Ok(())
        } else {
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

    async fn _accounts_containers_workspaces_variables_create(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
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
                    "container-id" => Some(("containerId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "disabling-trigger-id" => Some(("disablingTriggerId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Vec })),
                    "enabling-trigger-id" => Some(("enablingTriggerId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Vec })),
                    "fingerprint" => Some(("fingerprint", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "format-value.case-conversion-type" => Some(("formatValue.caseConversionType", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "format-value.convert-false-to-value.is-weak-reference" => Some(("formatValue.convertFalseToValue.isWeakReference", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "format-value.convert-false-to-value.key" => Some(("formatValue.convertFalseToValue.key", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "format-value.convert-false-to-value.type" => Some(("formatValue.convertFalseToValue.type", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "format-value.convert-false-to-value.value" => Some(("formatValue.convertFalseToValue.value", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "format-value.convert-null-to-value.is-weak-reference" => Some(("formatValue.convertNullToValue.isWeakReference", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "format-value.convert-null-to-value.key" => Some(("formatValue.convertNullToValue.key", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "format-value.convert-null-to-value.type" => Some(("formatValue.convertNullToValue.type", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "format-value.convert-null-to-value.value" => Some(("formatValue.convertNullToValue.value", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "format-value.convert-true-to-value.is-weak-reference" => Some(("formatValue.convertTrueToValue.isWeakReference", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "format-value.convert-true-to-value.key" => Some(("formatValue.convertTrueToValue.key", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "format-value.convert-true-to-value.type" => Some(("formatValue.convertTrueToValue.type", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "format-value.convert-true-to-value.value" => Some(("formatValue.convertTrueToValue.value", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "format-value.convert-undefined-to-value.is-weak-reference" => Some(("formatValue.convertUndefinedToValue.isWeakReference", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "format-value.convert-undefined-to-value.key" => Some(("formatValue.convertUndefinedToValue.key", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "format-value.convert-undefined-to-value.type" => Some(("formatValue.convertUndefinedToValue.type", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "format-value.convert-undefined-to-value.value" => Some(("formatValue.convertUndefinedToValue.value", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "name" => Some(("name", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "notes" => Some(("notes", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "parent-folder-id" => Some(("parentFolderId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "path" => Some(("path", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "schedule-end-ms" => Some(("scheduleEndMs", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "schedule-start-ms" => Some(("scheduleStartMs", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "tag-manager-url" => Some(("tagManagerUrl", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "type" => Some(("type", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "variable-id" => Some(("variableId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "workspace-id" => Some(("workspaceId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    _ => {
                        let suggestion = FieldCursor::did_you_mean(key, &vec!["account-id", "case-conversion-type", "container-id", "convert-false-to-value", "convert-null-to-value", "convert-true-to-value", "convert-undefined-to-value", "disabling-trigger-id", "enabling-trigger-id", "fingerprint", "format-value", "is-weak-reference", "key", "name", "notes", "parent-folder-id", "path", "schedule-end-ms", "schedule-start-ms", "tag-manager-url", "type", "value", "variable-id", "workspace-id"]);
                        err.issues.push(CLIError::Field(FieldError::Unknown(temp_cursor.to_string(), suggestion, value.map(|v| v.to_string()))));
                        None
                    }
                };
            if let Some((field_cursor_str, type_info)) = type_info {
                FieldCursor::from(field_cursor_str).set_json_value(&mut object, value.unwrap(), type_info, err, &temp_cursor);
            }
        }
        let mut request: api::Variable = json::value::from_value(object).unwrap();
        let mut call = self.hub.accounts().containers_workspaces_variables_create(request, opt.value_of("parent").unwrap_or(""));
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

    async fn _accounts_containers_workspaces_variables_delete(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        let mut call = self.hub.accounts().containers_workspaces_variables_delete(opt.value_of("path").unwrap_or(""));
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

    async fn _accounts_containers_workspaces_variables_get(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        let mut call = self.hub.accounts().containers_workspaces_variables_get(opt.value_of("path").unwrap_or(""));
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

    async fn _accounts_containers_workspaces_variables_list(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        let mut call = self.hub.accounts().containers_workspaces_variables_list(opt.value_of("parent").unwrap_or(""));
        for parg in opt.values_of("v").map(|i|i.collect()).unwrap_or(Vec::new()).iter() {
            let (key, value) = parse_kv_arg(&*parg, err, false);
            match key {
                "page-token" => {
                    call = call.page_token(value.unwrap_or(""));
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
                                                                           v.extend(["page-token"].iter().map(|v|*v));
                                                                           v } ));
                    }
                }
            }
        }
        let protocol = CallType::Standard;
        if dry_run {
            Ok(())
        } else {
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

    async fn _accounts_containers_workspaces_variables_revert(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        let mut call = self.hub.accounts().containers_workspaces_variables_revert(opt.value_of("path").unwrap_or(""));
        for parg in opt.values_of("v").map(|i|i.collect()).unwrap_or(Vec::new()).iter() {
            let (key, value) = parse_kv_arg(&*parg, err, false);
            match key {
                "fingerprint" => {
                    call = call.fingerprint(value.unwrap_or(""));
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
                                                                           v.extend(["fingerprint"].iter().map(|v|*v));
                                                                           v } ));
                    }
                }
            }
        }
        let protocol = CallType::Standard;
        if dry_run {
            Ok(())
        } else {
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

    async fn _accounts_containers_workspaces_variables_update(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
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
                    "container-id" => Some(("containerId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "disabling-trigger-id" => Some(("disablingTriggerId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Vec })),
                    "enabling-trigger-id" => Some(("enablingTriggerId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Vec })),
                    "fingerprint" => Some(("fingerprint", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "format-value.case-conversion-type" => Some(("formatValue.caseConversionType", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "format-value.convert-false-to-value.is-weak-reference" => Some(("formatValue.convertFalseToValue.isWeakReference", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "format-value.convert-false-to-value.key" => Some(("formatValue.convertFalseToValue.key", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "format-value.convert-false-to-value.type" => Some(("formatValue.convertFalseToValue.type", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "format-value.convert-false-to-value.value" => Some(("formatValue.convertFalseToValue.value", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "format-value.convert-null-to-value.is-weak-reference" => Some(("formatValue.convertNullToValue.isWeakReference", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "format-value.convert-null-to-value.key" => Some(("formatValue.convertNullToValue.key", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "format-value.convert-null-to-value.type" => Some(("formatValue.convertNullToValue.type", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "format-value.convert-null-to-value.value" => Some(("formatValue.convertNullToValue.value", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "format-value.convert-true-to-value.is-weak-reference" => Some(("formatValue.convertTrueToValue.isWeakReference", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "format-value.convert-true-to-value.key" => Some(("formatValue.convertTrueToValue.key", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "format-value.convert-true-to-value.type" => Some(("formatValue.convertTrueToValue.type", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "format-value.convert-true-to-value.value" => Some(("formatValue.convertTrueToValue.value", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "format-value.convert-undefined-to-value.is-weak-reference" => Some(("formatValue.convertUndefinedToValue.isWeakReference", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "format-value.convert-undefined-to-value.key" => Some(("formatValue.convertUndefinedToValue.key", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "format-value.convert-undefined-to-value.type" => Some(("formatValue.convertUndefinedToValue.type", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "format-value.convert-undefined-to-value.value" => Some(("formatValue.convertUndefinedToValue.value", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "name" => Some(("name", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "notes" => Some(("notes", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "parent-folder-id" => Some(("parentFolderId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "path" => Some(("path", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "schedule-end-ms" => Some(("scheduleEndMs", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "schedule-start-ms" => Some(("scheduleStartMs", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "tag-manager-url" => Some(("tagManagerUrl", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "type" => Some(("type", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "variable-id" => Some(("variableId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "workspace-id" => Some(("workspaceId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    _ => {
                        let suggestion = FieldCursor::did_you_mean(key, &vec!["account-id", "case-conversion-type", "container-id", "convert-false-to-value", "convert-null-to-value", "convert-true-to-value", "convert-undefined-to-value", "disabling-trigger-id", "enabling-trigger-id", "fingerprint", "format-value", "is-weak-reference", "key", "name", "notes", "parent-folder-id", "path", "schedule-end-ms", "schedule-start-ms", "tag-manager-url", "type", "value", "variable-id", "workspace-id"]);
                        err.issues.push(CLIError::Field(FieldError::Unknown(temp_cursor.to_string(), suggestion, value.map(|v| v.to_string()))));
                        None
                    }
                };
            if let Some((field_cursor_str, type_info)) = type_info {
                FieldCursor::from(field_cursor_str).set_json_value(&mut object, value.unwrap(), type_info, err, &temp_cursor);
            }
        }
        let mut request: api::Variable = json::value::from_value(object).unwrap();
        let mut call = self.hub.accounts().containers_workspaces_variables_update(request, opt.value_of("path").unwrap_or(""));
        for parg in opt.values_of("v").map(|i|i.collect()).unwrap_or(Vec::new()).iter() {
            let (key, value) = parse_kv_arg(&*parg, err, false);
            match key {
                "fingerprint" => {
                    call = call.fingerprint(value.unwrap_or(""));
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
                                                                           v.extend(["fingerprint"].iter().map(|v|*v));
                                                                           v } ));
                    }
                }
            }
        }
        let protocol = CallType::Standard;
        if dry_run {
            Ok(())
        } else {
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

    async fn _accounts_containers_workspaces_zones_create(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
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
                    "boundary.custom-evaluation-trigger-id" => Some(("boundary.customEvaluationTriggerId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Vec })),
                    "container-id" => Some(("containerId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "fingerprint" => Some(("fingerprint", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "name" => Some(("name", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "notes" => Some(("notes", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "path" => Some(("path", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "tag-manager-url" => Some(("tagManagerUrl", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "type-restriction.enable" => Some(("typeRestriction.enable", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "type-restriction.whitelisted-type-id" => Some(("typeRestriction.whitelistedTypeId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Vec })),
                    "workspace-id" => Some(("workspaceId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "zone-id" => Some(("zoneId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    _ => {
                        let suggestion = FieldCursor::did_you_mean(key, &vec!["account-id", "boundary", "container-id", "custom-evaluation-trigger-id", "enable", "fingerprint", "name", "notes", "path", "tag-manager-url", "type-restriction", "whitelisted-type-id", "workspace-id", "zone-id"]);
                        err.issues.push(CLIError::Field(FieldError::Unknown(temp_cursor.to_string(), suggestion, value.map(|v| v.to_string()))));
                        None
                    }
                };
            if let Some((field_cursor_str, type_info)) = type_info {
                FieldCursor::from(field_cursor_str).set_json_value(&mut object, value.unwrap(), type_info, err, &temp_cursor);
            }
        }
        let mut request: api::Zone = json::value::from_value(object).unwrap();
        let mut call = self.hub.accounts().containers_workspaces_zones_create(request, opt.value_of("parent").unwrap_or(""));
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

    async fn _accounts_containers_workspaces_zones_delete(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        let mut call = self.hub.accounts().containers_workspaces_zones_delete(opt.value_of("path").unwrap_or(""));
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

    async fn _accounts_containers_workspaces_zones_get(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        let mut call = self.hub.accounts().containers_workspaces_zones_get(opt.value_of("path").unwrap_or(""));
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

    async fn _accounts_containers_workspaces_zones_list(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        let mut call = self.hub.accounts().containers_workspaces_zones_list(opt.value_of("parent").unwrap_or(""));
        for parg in opt.values_of("v").map(|i|i.collect()).unwrap_or(Vec::new()).iter() {
            let (key, value) = parse_kv_arg(&*parg, err, false);
            match key {
                "page-token" => {
                    call = call.page_token(value.unwrap_or(""));
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
                                                                           v.extend(["page-token"].iter().map(|v|*v));
                                                                           v } ));
                    }
                }
            }
        }
        let protocol = CallType::Standard;
        if dry_run {
            Ok(())
        } else {
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

    async fn _accounts_containers_workspaces_zones_revert(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        let mut call = self.hub.accounts().containers_workspaces_zones_revert(opt.value_of("path").unwrap_or(""));
        for parg in opt.values_of("v").map(|i|i.collect()).unwrap_or(Vec::new()).iter() {
            let (key, value) = parse_kv_arg(&*parg, err, false);
            match key {
                "fingerprint" => {
                    call = call.fingerprint(value.unwrap_or(""));
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
                                                                           v.extend(["fingerprint"].iter().map(|v|*v));
                                                                           v } ));
                    }
                }
            }
        }
        let protocol = CallType::Standard;
        if dry_run {
            Ok(())
        } else {
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

    async fn _accounts_containers_workspaces_zones_update(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
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
                    "boundary.custom-evaluation-trigger-id" => Some(("boundary.customEvaluationTriggerId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Vec })),
                    "container-id" => Some(("containerId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "fingerprint" => Some(("fingerprint", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "name" => Some(("name", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "notes" => Some(("notes", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "path" => Some(("path", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "tag-manager-url" => Some(("tagManagerUrl", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "type-restriction.enable" => Some(("typeRestriction.enable", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "type-restriction.whitelisted-type-id" => Some(("typeRestriction.whitelistedTypeId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Vec })),
                    "workspace-id" => Some(("workspaceId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "zone-id" => Some(("zoneId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    _ => {
                        let suggestion = FieldCursor::did_you_mean(key, &vec!["account-id", "boundary", "container-id", "custom-evaluation-trigger-id", "enable", "fingerprint", "name", "notes", "path", "tag-manager-url", "type-restriction", "whitelisted-type-id", "workspace-id", "zone-id"]);
                        err.issues.push(CLIError::Field(FieldError::Unknown(temp_cursor.to_string(), suggestion, value.map(|v| v.to_string()))));
                        None
                    }
                };
            if let Some((field_cursor_str, type_info)) = type_info {
                FieldCursor::from(field_cursor_str).set_json_value(&mut object, value.unwrap(), type_info, err, &temp_cursor);
            }
        }
        let mut request: api::Zone = json::value::from_value(object).unwrap();
        let mut call = self.hub.accounts().containers_workspaces_zones_update(request, opt.value_of("path").unwrap_or(""));
        for parg in opt.values_of("v").map(|i|i.collect()).unwrap_or(Vec::new()).iter() {
            let (key, value) = parse_kv_arg(&*parg, err, false);
            match key {
                "fingerprint" => {
                    call = call.fingerprint(value.unwrap_or(""));
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
                                                                           v.extend(["fingerprint"].iter().map(|v|*v));
                                                                           v } ));
                    }
                }
            }
        }
        let protocol = CallType::Standard;
        if dry_run {
            Ok(())
        } else {
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

    async fn _accounts_get(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        let mut call = self.hub.accounts().get(opt.value_of("path").unwrap_or(""));
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

    async fn _accounts_list(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        let mut call = self.hub.accounts().list();
        for parg in opt.values_of("v").map(|i|i.collect()).unwrap_or(Vec::new()).iter() {
            let (key, value) = parse_kv_arg(&*parg, err, false);
            match key {
                "page-token" => {
                    call = call.page_token(value.unwrap_or(""));
                },
                "include-google-tags" => {
                    call = call.include_google_tags(        value.map(|v| arg_from_str(v, err, "include-google-tags", "boolean")).unwrap_or(false));
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
                                                                           v.extend(["include-google-tags", "page-token"].iter().map(|v|*v));
                                                                           v } ));
                    }
                }
            }
        }
        let protocol = CallType::Standard;
        if dry_run {
            Ok(())
        } else {
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

    async fn _accounts_update(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
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
                    "features.support-multiple-containers" => Some(("features.supportMultipleContainers", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "features.support-user-permissions" => Some(("features.supportUserPermissions", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "fingerprint" => Some(("fingerprint", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "name" => Some(("name", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "path" => Some(("path", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "share-data" => Some(("shareData", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "tag-manager-url" => Some(("tagManagerUrl", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    _ => {
                        let suggestion = FieldCursor::did_you_mean(key, &vec!["account-id", "features", "fingerprint", "name", "path", "share-data", "support-multiple-containers", "support-user-permissions", "tag-manager-url"]);
                        err.issues.push(CLIError::Field(FieldError::Unknown(temp_cursor.to_string(), suggestion, value.map(|v| v.to_string()))));
                        None
                    }
                };
            if let Some((field_cursor_str, type_info)) = type_info {
                FieldCursor::from(field_cursor_str).set_json_value(&mut object, value.unwrap(), type_info, err, &temp_cursor);
            }
        }
        let mut request: api::Account = json::value::from_value(object).unwrap();
        let mut call = self.hub.accounts().update(request, opt.value_of("path").unwrap_or(""));
        for parg in opt.values_of("v").map(|i|i.collect()).unwrap_or(Vec::new()).iter() {
            let (key, value) = parse_kv_arg(&*parg, err, false);
            match key {
                "fingerprint" => {
                    call = call.fingerprint(value.unwrap_or(""));
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
                                                                           v.extend(["fingerprint"].iter().map(|v|*v));
                                                                           v } ));
                    }
                }
            }
        }
        let protocol = CallType::Standard;
        if dry_run {
            Ok(())
        } else {
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

    async fn _accounts_user_permissions_create(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
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
                    "account-access.permission" => Some(("accountAccess.permission", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "account-id" => Some(("accountId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "email-address" => Some(("emailAddress", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "path" => Some(("path", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    _ => {
                        let suggestion = FieldCursor::did_you_mean(key, &vec!["account-access", "account-id", "email-address", "path", "permission"]);
                        err.issues.push(CLIError::Field(FieldError::Unknown(temp_cursor.to_string(), suggestion, value.map(|v| v.to_string()))));
                        None
                    }
                };
            if let Some((field_cursor_str, type_info)) = type_info {
                FieldCursor::from(field_cursor_str).set_json_value(&mut object, value.unwrap(), type_info, err, &temp_cursor);
            }
        }
        let mut request: api::UserPermission = json::value::from_value(object).unwrap();
        let mut call = self.hub.accounts().user_permissions_create(request, opt.value_of("parent").unwrap_or(""));
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

    async fn _accounts_user_permissions_delete(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        let mut call = self.hub.accounts().user_permissions_delete(opt.value_of("path").unwrap_or(""));
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

    async fn _accounts_user_permissions_get(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        let mut call = self.hub.accounts().user_permissions_get(opt.value_of("path").unwrap_or(""));
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

    async fn _accounts_user_permissions_list(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        let mut call = self.hub.accounts().user_permissions_list(opt.value_of("parent").unwrap_or(""));
        for parg in opt.values_of("v").map(|i|i.collect()).unwrap_or(Vec::new()).iter() {
            let (key, value) = parse_kv_arg(&*parg, err, false);
            match key {
                "page-token" => {
                    call = call.page_token(value.unwrap_or(""));
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
                                                                           v.extend(["page-token"].iter().map(|v|*v));
                                                                           v } ));
                    }
                }
            }
        }
        let protocol = CallType::Standard;
        if dry_run {
            Ok(())
        } else {
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

    async fn _accounts_user_permissions_update(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
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
                    "account-access.permission" => Some(("accountAccess.permission", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "account-id" => Some(("accountId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "email-address" => Some(("emailAddress", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "path" => Some(("path", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    _ => {
                        let suggestion = FieldCursor::did_you_mean(key, &vec!["account-access", "account-id", "email-address", "path", "permission"]);
                        err.issues.push(CLIError::Field(FieldError::Unknown(temp_cursor.to_string(), suggestion, value.map(|v| v.to_string()))));
                        None
                    }
                };
            if let Some((field_cursor_str, type_info)) = type_info {
                FieldCursor::from(field_cursor_str).set_json_value(&mut object, value.unwrap(), type_info, err, &temp_cursor);
            }
        }
        let mut request: api::UserPermission = json::value::from_value(object).unwrap();
        let mut call = self.hub.accounts().user_permissions_update(request, opt.value_of("path").unwrap_or(""));
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
            ("accounts", Some(opt)) => {
                match opt.subcommand() {
                    ("containers-combine", Some(opt)) => {
                        call_result = self._accounts_containers_combine(opt, dry_run, &mut err).await;
                    },
                    ("containers-create", Some(opt)) => {
                        call_result = self._accounts_containers_create(opt, dry_run, &mut err).await;
                    },
                    ("containers-delete", Some(opt)) => {
                        call_result = self._accounts_containers_delete(opt, dry_run, &mut err).await;
                    },
                    ("containers-destinations-get", Some(opt)) => {
                        call_result = self._accounts_containers_destinations_get(opt, dry_run, &mut err).await;
                    },
                    ("containers-destinations-link", Some(opt)) => {
                        call_result = self._accounts_containers_destinations_link(opt, dry_run, &mut err).await;
                    },
                    ("containers-destinations-list", Some(opt)) => {
                        call_result = self._accounts_containers_destinations_list(opt, dry_run, &mut err).await;
                    },
                    ("containers-environments-create", Some(opt)) => {
                        call_result = self._accounts_containers_environments_create(opt, dry_run, &mut err).await;
                    },
                    ("containers-environments-delete", Some(opt)) => {
                        call_result = self._accounts_containers_environments_delete(opt, dry_run, &mut err).await;
                    },
                    ("containers-environments-get", Some(opt)) => {
                        call_result = self._accounts_containers_environments_get(opt, dry_run, &mut err).await;
                    },
                    ("containers-environments-list", Some(opt)) => {
                        call_result = self._accounts_containers_environments_list(opt, dry_run, &mut err).await;
                    },
                    ("containers-environments-reauthorize", Some(opt)) => {
                        call_result = self._accounts_containers_environments_reauthorize(opt, dry_run, &mut err).await;
                    },
                    ("containers-environments-update", Some(opt)) => {
                        call_result = self._accounts_containers_environments_update(opt, dry_run, &mut err).await;
                    },
                    ("containers-get", Some(opt)) => {
                        call_result = self._accounts_containers_get(opt, dry_run, &mut err).await;
                    },
                    ("containers-list", Some(opt)) => {
                        call_result = self._accounts_containers_list(opt, dry_run, &mut err).await;
                    },
                    ("containers-lookup", Some(opt)) => {
                        call_result = self._accounts_containers_lookup(opt, dry_run, &mut err).await;
                    },
                    ("containers-move-tag-id", Some(opt)) => {
                        call_result = self._accounts_containers_move_tag_id(opt, dry_run, &mut err).await;
                    },
                    ("containers-snippet", Some(opt)) => {
                        call_result = self._accounts_containers_snippet(opt, dry_run, &mut err).await;
                    },
                    ("containers-update", Some(opt)) => {
                        call_result = self._accounts_containers_update(opt, dry_run, &mut err).await;
                    },
                    ("containers-version-headers-latest", Some(opt)) => {
                        call_result = self._accounts_containers_version_headers_latest(opt, dry_run, &mut err).await;
                    },
                    ("containers-version-headers-list", Some(opt)) => {
                        call_result = self._accounts_containers_version_headers_list(opt, dry_run, &mut err).await;
                    },
                    ("containers-versions-delete", Some(opt)) => {
                        call_result = self._accounts_containers_versions_delete(opt, dry_run, &mut err).await;
                    },
                    ("containers-versions-get", Some(opt)) => {
                        call_result = self._accounts_containers_versions_get(opt, dry_run, &mut err).await;
                    },
                    ("containers-versions-live", Some(opt)) => {
                        call_result = self._accounts_containers_versions_live(opt, dry_run, &mut err).await;
                    },
                    ("containers-versions-publish", Some(opt)) => {
                        call_result = self._accounts_containers_versions_publish(opt, dry_run, &mut err).await;
                    },
                    ("containers-versions-set-latest", Some(opt)) => {
                        call_result = self._accounts_containers_versions_set_latest(opt, dry_run, &mut err).await;
                    },
                    ("containers-versions-undelete", Some(opt)) => {
                        call_result = self._accounts_containers_versions_undelete(opt, dry_run, &mut err).await;
                    },
                    ("containers-versions-update", Some(opt)) => {
                        call_result = self._accounts_containers_versions_update(opt, dry_run, &mut err).await;
                    },
                    ("containers-workspaces-built-in-variables-create", Some(opt)) => {
                        call_result = self._accounts_containers_workspaces_built_in_variables_create(opt, dry_run, &mut err).await;
                    },
                    ("containers-workspaces-built-in-variables-delete", Some(opt)) => {
                        call_result = self._accounts_containers_workspaces_built_in_variables_delete(opt, dry_run, &mut err).await;
                    },
                    ("containers-workspaces-built-in-variables-list", Some(opt)) => {
                        call_result = self._accounts_containers_workspaces_built_in_variables_list(opt, dry_run, &mut err).await;
                    },
                    ("containers-workspaces-built-in-variables-revert", Some(opt)) => {
                        call_result = self._accounts_containers_workspaces_built_in_variables_revert(opt, dry_run, &mut err).await;
                    },
                    ("containers-workspaces-clients-create", Some(opt)) => {
                        call_result = self._accounts_containers_workspaces_clients_create(opt, dry_run, &mut err).await;
                    },
                    ("containers-workspaces-clients-delete", Some(opt)) => {
                        call_result = self._accounts_containers_workspaces_clients_delete(opt, dry_run, &mut err).await;
                    },
                    ("containers-workspaces-clients-get", Some(opt)) => {
                        call_result = self._accounts_containers_workspaces_clients_get(opt, dry_run, &mut err).await;
                    },
                    ("containers-workspaces-clients-list", Some(opt)) => {
                        call_result = self._accounts_containers_workspaces_clients_list(opt, dry_run, &mut err).await;
                    },
                    ("containers-workspaces-clients-revert", Some(opt)) => {
                        call_result = self._accounts_containers_workspaces_clients_revert(opt, dry_run, &mut err).await;
                    },
                    ("containers-workspaces-clients-update", Some(opt)) => {
                        call_result = self._accounts_containers_workspaces_clients_update(opt, dry_run, &mut err).await;
                    },
                    ("containers-workspaces-create", Some(opt)) => {
                        call_result = self._accounts_containers_workspaces_create(opt, dry_run, &mut err).await;
                    },
                    ("containers-workspaces-create-version", Some(opt)) => {
                        call_result = self._accounts_containers_workspaces_create_version(opt, dry_run, &mut err).await;
                    },
                    ("containers-workspaces-delete", Some(opt)) => {
                        call_result = self._accounts_containers_workspaces_delete(opt, dry_run, &mut err).await;
                    },
                    ("containers-workspaces-folders-create", Some(opt)) => {
                        call_result = self._accounts_containers_workspaces_folders_create(opt, dry_run, &mut err).await;
                    },
                    ("containers-workspaces-folders-delete", Some(opt)) => {
                        call_result = self._accounts_containers_workspaces_folders_delete(opt, dry_run, &mut err).await;
                    },
                    ("containers-workspaces-folders-entities", Some(opt)) => {
                        call_result = self._accounts_containers_workspaces_folders_entities(opt, dry_run, &mut err).await;
                    },
                    ("containers-workspaces-folders-get", Some(opt)) => {
                        call_result = self._accounts_containers_workspaces_folders_get(opt, dry_run, &mut err).await;
                    },
                    ("containers-workspaces-folders-list", Some(opt)) => {
                        call_result = self._accounts_containers_workspaces_folders_list(opt, dry_run, &mut err).await;
                    },
                    ("containers-workspaces-folders-move-entities-to-folder", Some(opt)) => {
                        call_result = self._accounts_containers_workspaces_folders_move_entities_to_folder(opt, dry_run, &mut err).await;
                    },
                    ("containers-workspaces-folders-revert", Some(opt)) => {
                        call_result = self._accounts_containers_workspaces_folders_revert(opt, dry_run, &mut err).await;
                    },
                    ("containers-workspaces-folders-update", Some(opt)) => {
                        call_result = self._accounts_containers_workspaces_folders_update(opt, dry_run, &mut err).await;
                    },
                    ("containers-workspaces-get", Some(opt)) => {
                        call_result = self._accounts_containers_workspaces_get(opt, dry_run, &mut err).await;
                    },
                    ("containers-workspaces-get-status", Some(opt)) => {
                        call_result = self._accounts_containers_workspaces_get_status(opt, dry_run, &mut err).await;
                    },
                    ("containers-workspaces-gtag-config-create", Some(opt)) => {
                        call_result = self._accounts_containers_workspaces_gtag_config_create(opt, dry_run, &mut err).await;
                    },
                    ("containers-workspaces-gtag-config-delete", Some(opt)) => {
                        call_result = self._accounts_containers_workspaces_gtag_config_delete(opt, dry_run, &mut err).await;
                    },
                    ("containers-workspaces-gtag-config-get", Some(opt)) => {
                        call_result = self._accounts_containers_workspaces_gtag_config_get(opt, dry_run, &mut err).await;
                    },
                    ("containers-workspaces-gtag-config-list", Some(opt)) => {
                        call_result = self._accounts_containers_workspaces_gtag_config_list(opt, dry_run, &mut err).await;
                    },
                    ("containers-workspaces-gtag-config-update", Some(opt)) => {
                        call_result = self._accounts_containers_workspaces_gtag_config_update(opt, dry_run, &mut err).await;
                    },
                    ("containers-workspaces-list", Some(opt)) => {
                        call_result = self._accounts_containers_workspaces_list(opt, dry_run, &mut err).await;
                    },
                    ("containers-workspaces-quick-preview", Some(opt)) => {
                        call_result = self._accounts_containers_workspaces_quick_preview(opt, dry_run, &mut err).await;
                    },
                    ("containers-workspaces-resolve-conflict", Some(opt)) => {
                        call_result = self._accounts_containers_workspaces_resolve_conflict(opt, dry_run, &mut err).await;
                    },
                    ("containers-workspaces-sync", Some(opt)) => {
                        call_result = self._accounts_containers_workspaces_sync(opt, dry_run, &mut err).await;
                    },
                    ("containers-workspaces-tags-create", Some(opt)) => {
                        call_result = self._accounts_containers_workspaces_tags_create(opt, dry_run, &mut err).await;
                    },
                    ("containers-workspaces-tags-delete", Some(opt)) => {
                        call_result = self._accounts_containers_workspaces_tags_delete(opt, dry_run, &mut err).await;
                    },
                    ("containers-workspaces-tags-get", Some(opt)) => {
                        call_result = self._accounts_containers_workspaces_tags_get(opt, dry_run, &mut err).await;
                    },
                    ("containers-workspaces-tags-list", Some(opt)) => {
                        call_result = self._accounts_containers_workspaces_tags_list(opt, dry_run, &mut err).await;
                    },
                    ("containers-workspaces-tags-revert", Some(opt)) => {
                        call_result = self._accounts_containers_workspaces_tags_revert(opt, dry_run, &mut err).await;
                    },
                    ("containers-workspaces-tags-update", Some(opt)) => {
                        call_result = self._accounts_containers_workspaces_tags_update(opt, dry_run, &mut err).await;
                    },
                    ("containers-workspaces-templates-create", Some(opt)) => {
                        call_result = self._accounts_containers_workspaces_templates_create(opt, dry_run, &mut err).await;
                    },
                    ("containers-workspaces-templates-delete", Some(opt)) => {
                        call_result = self._accounts_containers_workspaces_templates_delete(opt, dry_run, &mut err).await;
                    },
                    ("containers-workspaces-templates-get", Some(opt)) => {
                        call_result = self._accounts_containers_workspaces_templates_get(opt, dry_run, &mut err).await;
                    },
                    ("containers-workspaces-templates-list", Some(opt)) => {
                        call_result = self._accounts_containers_workspaces_templates_list(opt, dry_run, &mut err).await;
                    },
                    ("containers-workspaces-templates-revert", Some(opt)) => {
                        call_result = self._accounts_containers_workspaces_templates_revert(opt, dry_run, &mut err).await;
                    },
                    ("containers-workspaces-templates-update", Some(opt)) => {
                        call_result = self._accounts_containers_workspaces_templates_update(opt, dry_run, &mut err).await;
                    },
                    ("containers-workspaces-transformations-create", Some(opt)) => {
                        call_result = self._accounts_containers_workspaces_transformations_create(opt, dry_run, &mut err).await;
                    },
                    ("containers-workspaces-transformations-delete", Some(opt)) => {
                        call_result = self._accounts_containers_workspaces_transformations_delete(opt, dry_run, &mut err).await;
                    },
                    ("containers-workspaces-transformations-get", Some(opt)) => {
                        call_result = self._accounts_containers_workspaces_transformations_get(opt, dry_run, &mut err).await;
                    },
                    ("containers-workspaces-transformations-list", Some(opt)) => {
                        call_result = self._accounts_containers_workspaces_transformations_list(opt, dry_run, &mut err).await;
                    },
                    ("containers-workspaces-transformations-revert", Some(opt)) => {
                        call_result = self._accounts_containers_workspaces_transformations_revert(opt, dry_run, &mut err).await;
                    },
                    ("containers-workspaces-transformations-update", Some(opt)) => {
                        call_result = self._accounts_containers_workspaces_transformations_update(opt, dry_run, &mut err).await;
                    },
                    ("containers-workspaces-triggers-create", Some(opt)) => {
                        call_result = self._accounts_containers_workspaces_triggers_create(opt, dry_run, &mut err).await;
                    },
                    ("containers-workspaces-triggers-delete", Some(opt)) => {
                        call_result = self._accounts_containers_workspaces_triggers_delete(opt, dry_run, &mut err).await;
                    },
                    ("containers-workspaces-triggers-get", Some(opt)) => {
                        call_result = self._accounts_containers_workspaces_triggers_get(opt, dry_run, &mut err).await;
                    },
                    ("containers-workspaces-triggers-list", Some(opt)) => {
                        call_result = self._accounts_containers_workspaces_triggers_list(opt, dry_run, &mut err).await;
                    },
                    ("containers-workspaces-triggers-revert", Some(opt)) => {
                        call_result = self._accounts_containers_workspaces_triggers_revert(opt, dry_run, &mut err).await;
                    },
                    ("containers-workspaces-triggers-update", Some(opt)) => {
                        call_result = self._accounts_containers_workspaces_triggers_update(opt, dry_run, &mut err).await;
                    },
                    ("containers-workspaces-update", Some(opt)) => {
                        call_result = self._accounts_containers_workspaces_update(opt, dry_run, &mut err).await;
                    },
                    ("containers-workspaces-variables-create", Some(opt)) => {
                        call_result = self._accounts_containers_workspaces_variables_create(opt, dry_run, &mut err).await;
                    },
                    ("containers-workspaces-variables-delete", Some(opt)) => {
                        call_result = self._accounts_containers_workspaces_variables_delete(opt, dry_run, &mut err).await;
                    },
                    ("containers-workspaces-variables-get", Some(opt)) => {
                        call_result = self._accounts_containers_workspaces_variables_get(opt, dry_run, &mut err).await;
                    },
                    ("containers-workspaces-variables-list", Some(opt)) => {
                        call_result = self._accounts_containers_workspaces_variables_list(opt, dry_run, &mut err).await;
                    },
                    ("containers-workspaces-variables-revert", Some(opt)) => {
                        call_result = self._accounts_containers_workspaces_variables_revert(opt, dry_run, &mut err).await;
                    },
                    ("containers-workspaces-variables-update", Some(opt)) => {
                        call_result = self._accounts_containers_workspaces_variables_update(opt, dry_run, &mut err).await;
                    },
                    ("containers-workspaces-zones-create", Some(opt)) => {
                        call_result = self._accounts_containers_workspaces_zones_create(opt, dry_run, &mut err).await;
                    },
                    ("containers-workspaces-zones-delete", Some(opt)) => {
                        call_result = self._accounts_containers_workspaces_zones_delete(opt, dry_run, &mut err).await;
                    },
                    ("containers-workspaces-zones-get", Some(opt)) => {
                        call_result = self._accounts_containers_workspaces_zones_get(opt, dry_run, &mut err).await;
                    },
                    ("containers-workspaces-zones-list", Some(opt)) => {
                        call_result = self._accounts_containers_workspaces_zones_list(opt, dry_run, &mut err).await;
                    },
                    ("containers-workspaces-zones-revert", Some(opt)) => {
                        call_result = self._accounts_containers_workspaces_zones_revert(opt, dry_run, &mut err).await;
                    },
                    ("containers-workspaces-zones-update", Some(opt)) => {
                        call_result = self._accounts_containers_workspaces_zones_update(opt, dry_run, &mut err).await;
                    },
                    ("get", Some(opt)) => {
                        call_result = self._accounts_get(opt, dry_run, &mut err).await;
                    },
                    ("list", Some(opt)) => {
                        call_result = self._accounts_list(opt, dry_run, &mut err).await;
                    },
                    ("update", Some(opt)) => {
                        call_result = self._accounts_update(opt, dry_run, &mut err).await;
                    },
                    ("user-permissions-create", Some(opt)) => {
                        call_result = self._accounts_user_permissions_create(opt, dry_run, &mut err).await;
                    },
                    ("user-permissions-delete", Some(opt)) => {
                        call_result = self._accounts_user_permissions_delete(opt, dry_run, &mut err).await;
                    },
                    ("user-permissions-get", Some(opt)) => {
                        call_result = self._accounts_user_permissions_get(opt, dry_run, &mut err).await;
                    },
                    ("user-permissions-list", Some(opt)) => {
                        call_result = self._accounts_user_permissions_list(opt, dry_run, &mut err).await;
                    },
                    ("user-permissions-update", Some(opt)) => {
                        call_result = self._accounts_user_permissions_update(opt, dry_run, &mut err).await;
                    },
                    _ => {
                        err.issues.push(CLIError::MissingMethodError("accounts".to_string()));
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

            match client::application_secret_from_directory(&config_dir, "tagmanager2-secret.json",
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
        ).persist_tokens_to_disk(format!("{}/tagmanager2", config_dir)).build().await.unwrap();

        let engine = Engine {
            opt: opt,
            hub: api::TagManager::new(client, auth),
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
        ("accounts", "methods: 'containers-combine', 'containers-create', 'containers-delete', 'containers-destinations-get', 'containers-destinations-link', 'containers-destinations-list', 'containers-environments-create', 'containers-environments-delete', 'containers-environments-get', 'containers-environments-list', 'containers-environments-reauthorize', 'containers-environments-update', 'containers-get', 'containers-list', 'containers-lookup', 'containers-move-tag-id', 'containers-snippet', 'containers-update', 'containers-version-headers-latest', 'containers-version-headers-list', 'containers-versions-delete', 'containers-versions-get', 'containers-versions-live', 'containers-versions-publish', 'containers-versions-set-latest', 'containers-versions-undelete', 'containers-versions-update', 'containers-workspaces-built-in-variables-create', 'containers-workspaces-built-in-variables-delete', 'containers-workspaces-built-in-variables-list', 'containers-workspaces-built-in-variables-revert', 'containers-workspaces-clients-create', 'containers-workspaces-clients-delete', 'containers-workspaces-clients-get', 'containers-workspaces-clients-list', 'containers-workspaces-clients-revert', 'containers-workspaces-clients-update', 'containers-workspaces-create', 'containers-workspaces-create-version', 'containers-workspaces-delete', 'containers-workspaces-folders-create', 'containers-workspaces-folders-delete', 'containers-workspaces-folders-entities', 'containers-workspaces-folders-get', 'containers-workspaces-folders-list', 'containers-workspaces-folders-move-entities-to-folder', 'containers-workspaces-folders-revert', 'containers-workspaces-folders-update', 'containers-workspaces-get', 'containers-workspaces-get-status', 'containers-workspaces-gtag-config-create', 'containers-workspaces-gtag-config-delete', 'containers-workspaces-gtag-config-get', 'containers-workspaces-gtag-config-list', 'containers-workspaces-gtag-config-update', 'containers-workspaces-list', 'containers-workspaces-quick-preview', 'containers-workspaces-resolve-conflict', 'containers-workspaces-sync', 'containers-workspaces-tags-create', 'containers-workspaces-tags-delete', 'containers-workspaces-tags-get', 'containers-workspaces-tags-list', 'containers-workspaces-tags-revert', 'containers-workspaces-tags-update', 'containers-workspaces-templates-create', 'containers-workspaces-templates-delete', 'containers-workspaces-templates-get', 'containers-workspaces-templates-list', 'containers-workspaces-templates-revert', 'containers-workspaces-templates-update', 'containers-workspaces-transformations-create', 'containers-workspaces-transformations-delete', 'containers-workspaces-transformations-get', 'containers-workspaces-transformations-list', 'containers-workspaces-transformations-revert', 'containers-workspaces-transformations-update', 'containers-workspaces-triggers-create', 'containers-workspaces-triggers-delete', 'containers-workspaces-triggers-get', 'containers-workspaces-triggers-list', 'containers-workspaces-triggers-revert', 'containers-workspaces-triggers-update', 'containers-workspaces-update', 'containers-workspaces-variables-create', 'containers-workspaces-variables-delete', 'containers-workspaces-variables-get', 'containers-workspaces-variables-list', 'containers-workspaces-variables-revert', 'containers-workspaces-variables-update', 'containers-workspaces-zones-create', 'containers-workspaces-zones-delete', 'containers-workspaces-zones-get', 'containers-workspaces-zones-list', 'containers-workspaces-zones-revert', 'containers-workspaces-zones-update', 'get', 'list', 'update', 'user-permissions-create', 'user-permissions-delete', 'user-permissions-get', 'user-permissions-list' and 'user-permissions-update'", vec![
            ("containers-combine",
                    Some(r##"Combines Containers."##),
                    "Details at http://byron.github.io/google-apis-rs/google_tagmanager2_cli/accounts_containers-combine",
                  vec![
                    (Some(r##"path"##),
                     None,
                     Some(r##"GTM Container's API relative path. Example: accounts/{account_id}/containers/{container_id}"##),
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
            ("containers-create",
                    Some(r##"Creates a Container."##),
                    "Details at http://byron.github.io/google-apis-rs/google_tagmanager2_cli/accounts_containers-create",
                  vec![
                    (Some(r##"parent"##),
                     None,
                     Some(r##"GTM Account's API relative path. Example: accounts/{account_id}."##),
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
            ("containers-delete",
                    Some(r##"Deletes a Container."##),
                    "Details at http://byron.github.io/google-apis-rs/google_tagmanager2_cli/accounts_containers-delete",
                  vec![
                    (Some(r##"path"##),
                     None,
                     Some(r##"GTM Container's API relative path. Example: accounts/{account_id}/containers/{container_id}"##),
                     Some(true),
                     Some(false)),
        
                    (Some(r##"v"##),
                     Some(r##"p"##),
                     Some(r##"Set various optional parameters, matching the key=value form"##),
                     Some(false),
                     Some(true)),
                  ]),
            ("containers-destinations-get",
                    Some(r##"Gets a Destination."##),
                    "Details at http://byron.github.io/google-apis-rs/google_tagmanager2_cli/accounts_containers-destinations-get",
                  vec![
                    (Some(r##"path"##),
                     None,
                     Some(r##"Google Tag Destination's API relative path. Example: accounts/{account_id}/containers/{container_id}/destinations/{destination_link_id}"##),
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
            ("containers-destinations-link",
                    Some(r##"Adds a Destination to this Container and removes it from the Container to which it is currently linked."##),
                    "Details at http://byron.github.io/google-apis-rs/google_tagmanager2_cli/accounts_containers-destinations-link",
                  vec![
                    (Some(r##"parent"##),
                     None,
                     Some(r##"GTM parent Container's API relative path. Example: accounts/{account_id}/containers/{container_id}"##),
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
            ("containers-destinations-list",
                    Some(r##"Lists all Destinations linked to a GTM Container."##),
                    "Details at http://byron.github.io/google-apis-rs/google_tagmanager2_cli/accounts_containers-destinations-list",
                  vec![
                    (Some(r##"parent"##),
                     None,
                     Some(r##"GTM parent Container's API relative path. Example: accounts/{account_id}/containers/{container_id}"##),
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
            ("containers-environments-create",
                    Some(r##"Creates a GTM Environment."##),
                    "Details at http://byron.github.io/google-apis-rs/google_tagmanager2_cli/accounts_containers-environments-create",
                  vec![
                    (Some(r##"parent"##),
                     None,
                     Some(r##"GTM Container's API relative path. Example: accounts/{account_id}/containers/{container_id}"##),
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
            ("containers-environments-delete",
                    Some(r##"Deletes a GTM Environment."##),
                    "Details at http://byron.github.io/google-apis-rs/google_tagmanager2_cli/accounts_containers-environments-delete",
                  vec![
                    (Some(r##"path"##),
                     None,
                     Some(r##"GTM Environment's API relative path. Example: accounts/{account_id}/containers/{container_id}/environments/{environment_id}"##),
                     Some(true),
                     Some(false)),
        
                    (Some(r##"v"##),
                     Some(r##"p"##),
                     Some(r##"Set various optional parameters, matching the key=value form"##),
                     Some(false),
                     Some(true)),
                  ]),
            ("containers-environments-get",
                    Some(r##"Gets a GTM Environment."##),
                    "Details at http://byron.github.io/google-apis-rs/google_tagmanager2_cli/accounts_containers-environments-get",
                  vec![
                    (Some(r##"path"##),
                     None,
                     Some(r##"GTM Environment's API relative path. Example: accounts/{account_id}/containers/{container_id}/environments/{environment_id}"##),
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
            ("containers-environments-list",
                    Some(r##"Lists all GTM Environments of a GTM Container."##),
                    "Details at http://byron.github.io/google-apis-rs/google_tagmanager2_cli/accounts_containers-environments-list",
                  vec![
                    (Some(r##"parent"##),
                     None,
                     Some(r##"GTM Container's API relative path. Example: accounts/{account_id}/containers/{container_id}"##),
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
            ("containers-environments-reauthorize",
                    Some(r##"Re-generates the authorization code for a GTM Environment."##),
                    "Details at http://byron.github.io/google-apis-rs/google_tagmanager2_cli/accounts_containers-environments-reauthorize",
                  vec![
                    (Some(r##"path"##),
                     None,
                     Some(r##"GTM Environment's API relative path. Example: accounts/{account_id}/containers/{container_id}/environments/{environment_id}"##),
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
            ("containers-environments-update",
                    Some(r##"Updates a GTM Environment."##),
                    "Details at http://byron.github.io/google-apis-rs/google_tagmanager2_cli/accounts_containers-environments-update",
                  vec![
                    (Some(r##"path"##),
                     None,
                     Some(r##"GTM Environment's API relative path. Example: accounts/{account_id}/containers/{container_id}/environments/{environment_id}"##),
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
            ("containers-get",
                    Some(r##"Gets a Container."##),
                    "Details at http://byron.github.io/google-apis-rs/google_tagmanager2_cli/accounts_containers-get",
                  vec![
                    (Some(r##"path"##),
                     None,
                     Some(r##"GTM Container's API relative path. Example: accounts/{account_id}/containers/{container_id}"##),
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
            ("containers-list",
                    Some(r##"Lists all Containers that belongs to a GTM Account."##),
                    "Details at http://byron.github.io/google-apis-rs/google_tagmanager2_cli/accounts_containers-list",
                  vec![
                    (Some(r##"parent"##),
                     None,
                     Some(r##"GTM Account's API relative path. Example: accounts/{account_id}."##),
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
            ("containers-lookup",
                    Some(r##"Looks up a Container by destination ID."##),
                    "Details at http://byron.github.io/google-apis-rs/google_tagmanager2_cli/accounts_containers-lookup",
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
            ("containers-move-tag-id",
                    Some(r##"Move Tag ID out of a Container."##),
                    "Details at http://byron.github.io/google-apis-rs/google_tagmanager2_cli/accounts_containers-move-tag-id",
                  vec![
                    (Some(r##"path"##),
                     None,
                     Some(r##"GTM Container's API relative path. Example: accounts/{account_id}/containers/{container_id}"##),
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
            ("containers-snippet",
                    Some(r##"Gets the tagging snippet for a Container."##),
                    "Details at http://byron.github.io/google-apis-rs/google_tagmanager2_cli/accounts_containers-snippet",
                  vec![
                    (Some(r##"path"##),
                     None,
                     Some(r##"Container snippet's API relative path. Example: accounts/{account_id}/containers/{container_id}:snippet"##),
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
            ("containers-update",
                    Some(r##"Updates a Container."##),
                    "Details at http://byron.github.io/google-apis-rs/google_tagmanager2_cli/accounts_containers-update",
                  vec![
                    (Some(r##"path"##),
                     None,
                     Some(r##"GTM Container's API relative path. Example: accounts/{account_id}/containers/{container_id}"##),
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
            ("containers-version-headers-latest",
                    Some(r##"Gets the latest container version header"##),
                    "Details at http://byron.github.io/google-apis-rs/google_tagmanager2_cli/accounts_containers-version-headers-latest",
                  vec![
                    (Some(r##"parent"##),
                     None,
                     Some(r##"GTM Container's API relative path. Example: accounts/{account_id}/containers/{container_id}"##),
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
            ("containers-version-headers-list",
                    Some(r##"Lists all Container Versions of a GTM Container."##),
                    "Details at http://byron.github.io/google-apis-rs/google_tagmanager2_cli/accounts_containers-version-headers-list",
                  vec![
                    (Some(r##"parent"##),
                     None,
                     Some(r##"GTM Container's API relative path. Example: accounts/{account_id}/containers/{container_id}"##),
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
            ("containers-versions-delete",
                    Some(r##"Deletes a Container Version."##),
                    "Details at http://byron.github.io/google-apis-rs/google_tagmanager2_cli/accounts_containers-versions-delete",
                  vec![
                    (Some(r##"path"##),
                     None,
                     Some(r##"GTM ContainerVersion's API relative path. Example: accounts/{account_id}/containers/{container_id}/versions/{version_id}"##),
                     Some(true),
                     Some(false)),
        
                    (Some(r##"v"##),
                     Some(r##"p"##),
                     Some(r##"Set various optional parameters, matching the key=value form"##),
                     Some(false),
                     Some(true)),
                  ]),
            ("containers-versions-get",
                    Some(r##"Gets a Container Version."##),
                    "Details at http://byron.github.io/google-apis-rs/google_tagmanager2_cli/accounts_containers-versions-get",
                  vec![
                    (Some(r##"path"##),
                     None,
                     Some(r##"GTM ContainerVersion's API relative path. Example: accounts/{account_id}/containers/{container_id}/versions/{version_id}"##),
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
            ("containers-versions-live",
                    Some(r##"Gets the live (i.e. published) container version"##),
                    "Details at http://byron.github.io/google-apis-rs/google_tagmanager2_cli/accounts_containers-versions-live",
                  vec![
                    (Some(r##"parent"##),
                     None,
                     Some(r##"GTM Container's API relative path. Example: accounts/{account_id}/containers/{container_id}"##),
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
            ("containers-versions-publish",
                    Some(r##"Publishes a Container Version."##),
                    "Details at http://byron.github.io/google-apis-rs/google_tagmanager2_cli/accounts_containers-versions-publish",
                  vec![
                    (Some(r##"path"##),
                     None,
                     Some(r##"GTM ContainerVersion's API relative path. Example: accounts/{account_id}/containers/{container_id}/versions/{version_id}"##),
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
            ("containers-versions-set-latest",
                    Some(r##"Sets the latest version used for synchronization of workspaces when detecting conflicts and errors."##),
                    "Details at http://byron.github.io/google-apis-rs/google_tagmanager2_cli/accounts_containers-versions-set-latest",
                  vec![
                    (Some(r##"path"##),
                     None,
                     Some(r##"GTM ContainerVersion's API relative path. Example: accounts/{account_id}/containers/{container_id}/versions/{version_id}"##),
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
            ("containers-versions-undelete",
                    Some(r##"Undeletes a Container Version."##),
                    "Details at http://byron.github.io/google-apis-rs/google_tagmanager2_cli/accounts_containers-versions-undelete",
                  vec![
                    (Some(r##"path"##),
                     None,
                     Some(r##"GTM ContainerVersion's API relative path. Example: accounts/{account_id}/containers/{container_id}/versions/{version_id}"##),
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
            ("containers-versions-update",
                    Some(r##"Updates a Container Version."##),
                    "Details at http://byron.github.io/google-apis-rs/google_tagmanager2_cli/accounts_containers-versions-update",
                  vec![
                    (Some(r##"path"##),
                     None,
                     Some(r##"GTM ContainerVersion's API relative path. Example: accounts/{account_id}/containers/{container_id}/versions/{version_id}"##),
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
            ("containers-workspaces-built-in-variables-create",
                    Some(r##"Creates one or more GTM Built-In Variables."##),
                    "Details at http://byron.github.io/google-apis-rs/google_tagmanager2_cli/accounts_containers-workspaces-built-in-variables-create",
                  vec![
                    (Some(r##"parent"##),
                     None,
                     Some(r##"GTM Workspace's API relative path. Example: accounts/{account_id}/containers/{container_id}/workspaces/{workspace_id}"##),
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
            ("containers-workspaces-built-in-variables-delete",
                    Some(r##"Deletes one or more GTM Built-In Variables."##),
                    "Details at http://byron.github.io/google-apis-rs/google_tagmanager2_cli/accounts_containers-workspaces-built-in-variables-delete",
                  vec![
                    (Some(r##"path"##),
                     None,
                     Some(r##"GTM BuiltInVariable's API relative path. Example: accounts/{account_id}/containers/{container_id}/workspaces/{workspace_id}/built_in_variables"##),
                     Some(true),
                     Some(false)),
        
                    (Some(r##"v"##),
                     Some(r##"p"##),
                     Some(r##"Set various optional parameters, matching the key=value form"##),
                     Some(false),
                     Some(true)),
                  ]),
            ("containers-workspaces-built-in-variables-list",
                    Some(r##"Lists all the enabled Built-In Variables of a GTM Container."##),
                    "Details at http://byron.github.io/google-apis-rs/google_tagmanager2_cli/accounts_containers-workspaces-built-in-variables-list",
                  vec![
                    (Some(r##"parent"##),
                     None,
                     Some(r##"GTM Workspace's API relative path. Example: accounts/{account_id}/containers/{container_id}/workspaces/{workspace_id}"##),
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
            ("containers-workspaces-built-in-variables-revert",
                    Some(r##"Reverts changes to a GTM Built-In Variables in a GTM Workspace."##),
                    "Details at http://byron.github.io/google-apis-rs/google_tagmanager2_cli/accounts_containers-workspaces-built-in-variables-revert",
                  vec![
                    (Some(r##"path"##),
                     None,
                     Some(r##"GTM BuiltInVariable's API relative path. Example: accounts/{account_id}/containers/{container_id}/workspaces/{workspace_id}/built_in_variables"##),
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
            ("containers-workspaces-clients-create",
                    Some(r##"Creates a GTM Client."##),
                    "Details at http://byron.github.io/google-apis-rs/google_tagmanager2_cli/accounts_containers-workspaces-clients-create",
                  vec![
                    (Some(r##"parent"##),
                     None,
                     Some(r##"GTM Workspace's API relative path. Example: accounts/{account_id}/containers/{container_id}/workspaces/{workspace_id}"##),
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
            ("containers-workspaces-clients-delete",
                    Some(r##"Deletes a GTM Client."##),
                    "Details at http://byron.github.io/google-apis-rs/google_tagmanager2_cli/accounts_containers-workspaces-clients-delete",
                  vec![
                    (Some(r##"path"##),
                     None,
                     Some(r##"GTM Client's API relative path. Example: accounts/{account_id}/containers/{container_id}/workspaces/{workspace_id}/clients/{client_id}"##),
                     Some(true),
                     Some(false)),
        
                    (Some(r##"v"##),
                     Some(r##"p"##),
                     Some(r##"Set various optional parameters, matching the key=value form"##),
                     Some(false),
                     Some(true)),
                  ]),
            ("containers-workspaces-clients-get",
                    Some(r##"Gets a GTM Client."##),
                    "Details at http://byron.github.io/google-apis-rs/google_tagmanager2_cli/accounts_containers-workspaces-clients-get",
                  vec![
                    (Some(r##"path"##),
                     None,
                     Some(r##"GTM Client's API relative path. Example: accounts/{account_id}/containers/{container_id}/workspaces/{workspace_id}/clients/{client_id}"##),
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
            ("containers-workspaces-clients-list",
                    Some(r##"Lists all GTM Clients of a GTM container workspace."##),
                    "Details at http://byron.github.io/google-apis-rs/google_tagmanager2_cli/accounts_containers-workspaces-clients-list",
                  vec![
                    (Some(r##"parent"##),
                     None,
                     Some(r##"GTM Workspace's API relative path. Example: accounts/{account_id}/containers/{container_id}/workspaces/{workspace_id}"##),
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
            ("containers-workspaces-clients-revert",
                    Some(r##"Reverts changes to a GTM Client in a GTM Workspace."##),
                    "Details at http://byron.github.io/google-apis-rs/google_tagmanager2_cli/accounts_containers-workspaces-clients-revert",
                  vec![
                    (Some(r##"path"##),
                     None,
                     Some(r##"GTM Client's API relative path. Example: accounts/{account_id}/containers/{container_id}/workspaces/{workspace_id}/clients/{client_id}"##),
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
            ("containers-workspaces-clients-update",
                    Some(r##"Updates a GTM Client."##),
                    "Details at http://byron.github.io/google-apis-rs/google_tagmanager2_cli/accounts_containers-workspaces-clients-update",
                  vec![
                    (Some(r##"path"##),
                     None,
                     Some(r##"GTM Client's API relative path. Example: accounts/{account_id}/containers/{container_id}/workspaces/{workspace_id}/clients/{client_id}"##),
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
            ("containers-workspaces-create",
                    Some(r##"Creates a Workspace."##),
                    "Details at http://byron.github.io/google-apis-rs/google_tagmanager2_cli/accounts_containers-workspaces-create",
                  vec![
                    (Some(r##"parent"##),
                     None,
                     Some(r##"GTM parent Container's API relative path. Example: accounts/{account_id}/containers/{container_id}"##),
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
            ("containers-workspaces-create-version",
                    Some(r##"Creates a Container Version from the entities present in the workspace, deletes the workspace, and sets the base container version to the newly created version."##),
                    "Details at http://byron.github.io/google-apis-rs/google_tagmanager2_cli/accounts_containers-workspaces-create-version",
                  vec![
                    (Some(r##"path"##),
                     None,
                     Some(r##"GTM Workspace's API relative path. Example: accounts/{account_id}/containers/{container_id}/workspaces/{workspace_id}"##),
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
            ("containers-workspaces-delete",
                    Some(r##"Deletes a Workspace."##),
                    "Details at http://byron.github.io/google-apis-rs/google_tagmanager2_cli/accounts_containers-workspaces-delete",
                  vec![
                    (Some(r##"path"##),
                     None,
                     Some(r##"GTM Workspace's API relative path. Example: accounts/{account_id}/containers/{container_id}/workspaces/{workspace_id}"##),
                     Some(true),
                     Some(false)),
        
                    (Some(r##"v"##),
                     Some(r##"p"##),
                     Some(r##"Set various optional parameters, matching the key=value form"##),
                     Some(false),
                     Some(true)),
                  ]),
            ("containers-workspaces-folders-create",
                    Some(r##"Creates a GTM Folder."##),
                    "Details at http://byron.github.io/google-apis-rs/google_tagmanager2_cli/accounts_containers-workspaces-folders-create",
                  vec![
                    (Some(r##"parent"##),
                     None,
                     Some(r##"GTM Workspace's API relative path. Example: accounts/{account_id}/containers/{container_id}/workspaces/{workspace_id}"##),
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
            ("containers-workspaces-folders-delete",
                    Some(r##"Deletes a GTM Folder."##),
                    "Details at http://byron.github.io/google-apis-rs/google_tagmanager2_cli/accounts_containers-workspaces-folders-delete",
                  vec![
                    (Some(r##"path"##),
                     None,
                     Some(r##"GTM Folder's API relative path. Example: accounts/{account_id}/containers/{container_id}/workspaces/{workspace_id}/folders/{folder_id}"##),
                     Some(true),
                     Some(false)),
        
                    (Some(r##"v"##),
                     Some(r##"p"##),
                     Some(r##"Set various optional parameters, matching the key=value form"##),
                     Some(false),
                     Some(true)),
                  ]),
            ("containers-workspaces-folders-entities",
                    Some(r##"List all entities in a GTM Folder."##),
                    "Details at http://byron.github.io/google-apis-rs/google_tagmanager2_cli/accounts_containers-workspaces-folders-entities",
                  vec![
                    (Some(r##"path"##),
                     None,
                     Some(r##"GTM Folder's API relative path. Example: accounts/{account_id}/containers/{container_id}/workspaces/{workspace_id}/folders/{folder_id}"##),
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
            ("containers-workspaces-folders-get",
                    Some(r##"Gets a GTM Folder."##),
                    "Details at http://byron.github.io/google-apis-rs/google_tagmanager2_cli/accounts_containers-workspaces-folders-get",
                  vec![
                    (Some(r##"path"##),
                     None,
                     Some(r##"GTM Folder's API relative path. Example: accounts/{account_id}/containers/{container_id}/workspaces/{workspace_id}/folders/{folder_id}"##),
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
            ("containers-workspaces-folders-list",
                    Some(r##"Lists all GTM Folders of a Container."##),
                    "Details at http://byron.github.io/google-apis-rs/google_tagmanager2_cli/accounts_containers-workspaces-folders-list",
                  vec![
                    (Some(r##"parent"##),
                     None,
                     Some(r##"GTM Workspace's API relative path. Example: accounts/{account_id}/containers/{container_id}/workspaces/{workspace_id}"##),
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
            ("containers-workspaces-folders-move-entities-to-folder",
                    Some(r##"Moves entities to a GTM Folder. If {folder_id} in the request path equals 0, this will instead move entities out of the folder they currently belong to."##),
                    "Details at http://byron.github.io/google-apis-rs/google_tagmanager2_cli/accounts_containers-workspaces-folders-move-entities-to-folder",
                  vec![
                    (Some(r##"path"##),
                     None,
                     Some(r##"GTM Folder's API relative path. Example: accounts/{account_id}/containers/{container_id}/workspaces/{workspace_id}/folders/{folder_id}"##),
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
                  ]),
            ("containers-workspaces-folders-revert",
                    Some(r##"Reverts changes to a GTM Folder in a GTM Workspace."##),
                    "Details at http://byron.github.io/google-apis-rs/google_tagmanager2_cli/accounts_containers-workspaces-folders-revert",
                  vec![
                    (Some(r##"path"##),
                     None,
                     Some(r##"GTM Folder's API relative path. Example: accounts/{account_id}/containers/{container_id}/workspaces/{workspace_id}/folders/{folder_id}"##),
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
            ("containers-workspaces-folders-update",
                    Some(r##"Updates a GTM Folder."##),
                    "Details at http://byron.github.io/google-apis-rs/google_tagmanager2_cli/accounts_containers-workspaces-folders-update",
                  vec![
                    (Some(r##"path"##),
                     None,
                     Some(r##"GTM Folder's API relative path. Example: accounts/{account_id}/containers/{container_id}/workspaces/{workspace_id}/folders/{folder_id}"##),
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
            ("containers-workspaces-get",
                    Some(r##"Gets a Workspace."##),
                    "Details at http://byron.github.io/google-apis-rs/google_tagmanager2_cli/accounts_containers-workspaces-get",
                  vec![
                    (Some(r##"path"##),
                     None,
                     Some(r##"GTM Workspace's API relative path. Example: accounts/{account_id}/containers/{container_id}/workspaces/{workspace_id}"##),
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
            ("containers-workspaces-get-status",
                    Some(r##"Finds conflicting and modified entities in the workspace."##),
                    "Details at http://byron.github.io/google-apis-rs/google_tagmanager2_cli/accounts_containers-workspaces-get-status",
                  vec![
                    (Some(r##"path"##),
                     None,
                     Some(r##"GTM Workspace's API relative path. Example: accounts/{account_id}/containers/{container_id}/workspaces/{workspace_id}"##),
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
            ("containers-workspaces-gtag-config-create",
                    Some(r##"Creates a Google tag config."##),
                    "Details at http://byron.github.io/google-apis-rs/google_tagmanager2_cli/accounts_containers-workspaces-gtag-config-create",
                  vec![
                    (Some(r##"parent"##),
                     None,
                     Some(r##"Workspace's API relative path. Example: accounts/{account_id}/containers/{container_id}/workspaces/{workspace_id}"##),
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
            ("containers-workspaces-gtag-config-delete",
                    Some(r##"Deletes a Google tag config."##),
                    "Details at http://byron.github.io/google-apis-rs/google_tagmanager2_cli/accounts_containers-workspaces-gtag-config-delete",
                  vec![
                    (Some(r##"path"##),
                     None,
                     Some(r##"Google tag config's API relative path. Example: accounts/{account_id}/containers/{container_id}/workspaces/{workspace_id}/gtag_config/{gtag_config_id}"##),
                     Some(true),
                     Some(false)),
        
                    (Some(r##"v"##),
                     Some(r##"p"##),
                     Some(r##"Set various optional parameters, matching the key=value form"##),
                     Some(false),
                     Some(true)),
                  ]),
            ("containers-workspaces-gtag-config-get",
                    Some(r##"Gets a Google tag config."##),
                    "Details at http://byron.github.io/google-apis-rs/google_tagmanager2_cli/accounts_containers-workspaces-gtag-config-get",
                  vec![
                    (Some(r##"path"##),
                     None,
                     Some(r##"Google tag config's API relative path. Example: accounts/{account_id}/containers/{container_id}/workspaces/{workspace_id}/gtag_config/{gtag_config_id}"##),
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
            ("containers-workspaces-gtag-config-list",
                    Some(r##"Lists all Google tag configs in a Container."##),
                    "Details at http://byron.github.io/google-apis-rs/google_tagmanager2_cli/accounts_containers-workspaces-gtag-config-list",
                  vec![
                    (Some(r##"parent"##),
                     None,
                     Some(r##"Workspace's API relative path. Example: accounts/{account_id}/containers/{container_id}/workspaces/{workspace_id}"##),
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
            ("containers-workspaces-gtag-config-update",
                    Some(r##"Updates a Google tag config."##),
                    "Details at http://byron.github.io/google-apis-rs/google_tagmanager2_cli/accounts_containers-workspaces-gtag-config-update",
                  vec![
                    (Some(r##"path"##),
                     None,
                     Some(r##"Google tag config's API relative path. Example: accounts/{account_id}/containers/{container_id}/workspaces/{workspace_id}/gtag_config/{gtag_config_id}"##),
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
            ("containers-workspaces-list",
                    Some(r##"Lists all Workspaces that belong to a GTM Container."##),
                    "Details at http://byron.github.io/google-apis-rs/google_tagmanager2_cli/accounts_containers-workspaces-list",
                  vec![
                    (Some(r##"parent"##),
                     None,
                     Some(r##"GTM parent Container's API relative path. Example: accounts/{account_id}/containers/{container_id}"##),
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
            ("containers-workspaces-quick-preview",
                    Some(r##"Quick previews a workspace by creating a fake container version from all entities in the provided workspace."##),
                    "Details at http://byron.github.io/google-apis-rs/google_tagmanager2_cli/accounts_containers-workspaces-quick-preview",
                  vec![
                    (Some(r##"path"##),
                     None,
                     Some(r##"GTM Workspace's API relative path. Example: accounts/{account_id}/containers/{container_id}/workspaces/{workspace_id}"##),
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
            ("containers-workspaces-resolve-conflict",
                    Some(r##"Resolves a merge conflict for a workspace entity by updating it to the resolved entity passed in the request."##),
                    "Details at http://byron.github.io/google-apis-rs/google_tagmanager2_cli/accounts_containers-workspaces-resolve-conflict",
                  vec![
                    (Some(r##"path"##),
                     None,
                     Some(r##"GTM Workspace's API relative path. Example: accounts/{account_id}/containers/{container_id}/workspaces/{workspace_id}"##),
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
                  ]),
            ("containers-workspaces-sync",
                    Some(r##"Syncs a workspace to the latest container version by updating all unmodified workspace entities and displaying conflicts for modified entities."##),
                    "Details at http://byron.github.io/google-apis-rs/google_tagmanager2_cli/accounts_containers-workspaces-sync",
                  vec![
                    (Some(r##"path"##),
                     None,
                     Some(r##"GTM Workspace's API relative path. Example: accounts/{account_id}/containers/{container_id}/workspaces/{workspace_id}"##),
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
            ("containers-workspaces-tags-create",
                    Some(r##"Creates a GTM Tag."##),
                    "Details at http://byron.github.io/google-apis-rs/google_tagmanager2_cli/accounts_containers-workspaces-tags-create",
                  vec![
                    (Some(r##"parent"##),
                     None,
                     Some(r##"GTM Workspace's API relative path. Example: accounts/{account_id}/containers/{container_id}/workspaces/{workspace_id}"##),
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
            ("containers-workspaces-tags-delete",
                    Some(r##"Deletes a GTM Tag."##),
                    "Details at http://byron.github.io/google-apis-rs/google_tagmanager2_cli/accounts_containers-workspaces-tags-delete",
                  vec![
                    (Some(r##"path"##),
                     None,
                     Some(r##"GTM Tag's API relative path. Example: accounts/{account_id}/containers/{container_id}/workspaces/{workspace_id}/tags/{tag_id}"##),
                     Some(true),
                     Some(false)),
        
                    (Some(r##"v"##),
                     Some(r##"p"##),
                     Some(r##"Set various optional parameters, matching the key=value form"##),
                     Some(false),
                     Some(true)),
                  ]),
            ("containers-workspaces-tags-get",
                    Some(r##"Gets a GTM Tag."##),
                    "Details at http://byron.github.io/google-apis-rs/google_tagmanager2_cli/accounts_containers-workspaces-tags-get",
                  vec![
                    (Some(r##"path"##),
                     None,
                     Some(r##"GTM Tag's API relative path. Example: accounts/{account_id}/containers/{container_id}/workspaces/{workspace_id}/tags/{tag_id}"##),
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
            ("containers-workspaces-tags-list",
                    Some(r##"Lists all GTM Tags of a Container."##),
                    "Details at http://byron.github.io/google-apis-rs/google_tagmanager2_cli/accounts_containers-workspaces-tags-list",
                  vec![
                    (Some(r##"parent"##),
                     None,
                     Some(r##"GTM Workspace's API relative path. Example: accounts/{account_id}/containers/{container_id}/workspaces/{workspace_id}"##),
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
            ("containers-workspaces-tags-revert",
                    Some(r##"Reverts changes to a GTM Tag in a GTM Workspace."##),
                    "Details at http://byron.github.io/google-apis-rs/google_tagmanager2_cli/accounts_containers-workspaces-tags-revert",
                  vec![
                    (Some(r##"path"##),
                     None,
                     Some(r##"GTM Tag's API relative path. Example: accounts/{account_id}/containers/{container_id}/workspaces/{workspace_id}/tags/{tag_id}"##),
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
            ("containers-workspaces-tags-update",
                    Some(r##"Updates a GTM Tag."##),
                    "Details at http://byron.github.io/google-apis-rs/google_tagmanager2_cli/accounts_containers-workspaces-tags-update",
                  vec![
                    (Some(r##"path"##),
                     None,
                     Some(r##"GTM Tag's API relative path. Example: accounts/{account_id}/containers/{container_id}/workspaces/{workspace_id}/tags/{tag_id}"##),
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
            ("containers-workspaces-templates-create",
                    Some(r##"Creates a GTM Custom Template."##),
                    "Details at http://byron.github.io/google-apis-rs/google_tagmanager2_cli/accounts_containers-workspaces-templates-create",
                  vec![
                    (Some(r##"parent"##),
                     None,
                     Some(r##"GTM Workspace's API relative path. Example: accounts/{account_id}/containers/{container_id}/workspaces/{workspace_id}"##),
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
            ("containers-workspaces-templates-delete",
                    Some(r##"Deletes a GTM Template."##),
                    "Details at http://byron.github.io/google-apis-rs/google_tagmanager2_cli/accounts_containers-workspaces-templates-delete",
                  vec![
                    (Some(r##"path"##),
                     None,
                     Some(r##"GTM Custom Template's API relative path. Example: accounts/{account_id}/containers/{container_id}/workspaces/{workspace_id}/templates/{template_id}"##),
                     Some(true),
                     Some(false)),
        
                    (Some(r##"v"##),
                     Some(r##"p"##),
                     Some(r##"Set various optional parameters, matching the key=value form"##),
                     Some(false),
                     Some(true)),
                  ]),
            ("containers-workspaces-templates-get",
                    Some(r##"Gets a GTM Template."##),
                    "Details at http://byron.github.io/google-apis-rs/google_tagmanager2_cli/accounts_containers-workspaces-templates-get",
                  vec![
                    (Some(r##"path"##),
                     None,
                     Some(r##"GTM Custom Template's API relative path. Example: accounts/{account_id}/containers/{container_id}/workspaces/{workspace_id}/templates/{template_id}"##),
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
            ("containers-workspaces-templates-list",
                    Some(r##"Lists all GTM Templates of a GTM container workspace."##),
                    "Details at http://byron.github.io/google-apis-rs/google_tagmanager2_cli/accounts_containers-workspaces-templates-list",
                  vec![
                    (Some(r##"parent"##),
                     None,
                     Some(r##"GTM Workspace's API relative path. Example: accounts/{account_id}/containers/{container_id}/workspaces/{workspace_id}"##),
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
            ("containers-workspaces-templates-revert",
                    Some(r##"Reverts changes to a GTM Template in a GTM Workspace."##),
                    "Details at http://byron.github.io/google-apis-rs/google_tagmanager2_cli/accounts_containers-workspaces-templates-revert",
                  vec![
                    (Some(r##"path"##),
                     None,
                     Some(r##"GTM Custom Template's API relative path. Example: accounts/{account_id}/containers/{container_id}/workspaces/{workspace_id}/templates/{template_id}"##),
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
            ("containers-workspaces-templates-update",
                    Some(r##"Updates a GTM Template."##),
                    "Details at http://byron.github.io/google-apis-rs/google_tagmanager2_cli/accounts_containers-workspaces-templates-update",
                  vec![
                    (Some(r##"path"##),
                     None,
                     Some(r##"GTM Custom Template's API relative path. Example: accounts/{account_id}/containers/{container_id}/workspaces/{workspace_id}/templates/{template_id}"##),
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
            ("containers-workspaces-transformations-create",
                    Some(r##"Creates a GTM Transformation."##),
                    "Details at http://byron.github.io/google-apis-rs/google_tagmanager2_cli/accounts_containers-workspaces-transformations-create",
                  vec![
                    (Some(r##"parent"##),
                     None,
                     Some(r##"GTM Workspace's API relative path. Example: accounts/{account_id}/containers/{container_id}/workspaces/{workspace_id}"##),
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
            ("containers-workspaces-transformations-delete",
                    Some(r##"Deletes a GTM Transformation."##),
                    "Details at http://byron.github.io/google-apis-rs/google_tagmanager2_cli/accounts_containers-workspaces-transformations-delete",
                  vec![
                    (Some(r##"path"##),
                     None,
                     Some(r##"GTM Transformation's API relative path. Example: accounts/{account_id}/containers/{container_id}/workspaces/{workspace_id}/transformations/{transformation_id}"##),
                     Some(true),
                     Some(false)),
        
                    (Some(r##"v"##),
                     Some(r##"p"##),
                     Some(r##"Set various optional parameters, matching the key=value form"##),
                     Some(false),
                     Some(true)),
                  ]),
            ("containers-workspaces-transformations-get",
                    Some(r##"Gets a GTM Transformation."##),
                    "Details at http://byron.github.io/google-apis-rs/google_tagmanager2_cli/accounts_containers-workspaces-transformations-get",
                  vec![
                    (Some(r##"path"##),
                     None,
                     Some(r##"GTM Transformation's API relative path. Example: accounts/{account_id}/containers/{container_id}/workspaces/{workspace_id}/transformations/{transformation_id}"##),
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
            ("containers-workspaces-transformations-list",
                    Some(r##"Lists all GTM Transformations of a GTM container workspace."##),
                    "Details at http://byron.github.io/google-apis-rs/google_tagmanager2_cli/accounts_containers-workspaces-transformations-list",
                  vec![
                    (Some(r##"parent"##),
                     None,
                     Some(r##"GTM Workspace's API relative path. Example: accounts/{account_id}/containers/{container_id}/workspaces/{workspace_id}"##),
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
            ("containers-workspaces-transformations-revert",
                    Some(r##"Reverts changes to a GTM Transformation in a GTM Workspace."##),
                    "Details at http://byron.github.io/google-apis-rs/google_tagmanager2_cli/accounts_containers-workspaces-transformations-revert",
                  vec![
                    (Some(r##"path"##),
                     None,
                     Some(r##"GTM Transformation's API relative path. Example: accounts/{account_id}/containers/{container_id}/workspaces/{workspace_id}/transformations/{transformation_id}"##),
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
            ("containers-workspaces-transformations-update",
                    Some(r##"Updates a GTM Transformation."##),
                    "Details at http://byron.github.io/google-apis-rs/google_tagmanager2_cli/accounts_containers-workspaces-transformations-update",
                  vec![
                    (Some(r##"path"##),
                     None,
                     Some(r##"GTM Transformation's API relative path. Example: accounts/{account_id}/containers/{container_id}/workspaces/{workspace_id}/transformations/{transformation_id}"##),
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
            ("containers-workspaces-triggers-create",
                    Some(r##"Creates a GTM Trigger."##),
                    "Details at http://byron.github.io/google-apis-rs/google_tagmanager2_cli/accounts_containers-workspaces-triggers-create",
                  vec![
                    (Some(r##"parent"##),
                     None,
                     Some(r##"GTM Workspace's API relative path. Example: accounts/{account_id}/containers/{container_id}/workspaces/{workspace_id}"##),
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
            ("containers-workspaces-triggers-delete",
                    Some(r##"Deletes a GTM Trigger."##),
                    "Details at http://byron.github.io/google-apis-rs/google_tagmanager2_cli/accounts_containers-workspaces-triggers-delete",
                  vec![
                    (Some(r##"path"##),
                     None,
                     Some(r##"GTM Trigger's API relative path. Example: accounts/{account_id}/containers/{container_id}/workspaces/{workspace_id}/triggers/{trigger_id}"##),
                     Some(true),
                     Some(false)),
        
                    (Some(r##"v"##),
                     Some(r##"p"##),
                     Some(r##"Set various optional parameters, matching the key=value form"##),
                     Some(false),
                     Some(true)),
                  ]),
            ("containers-workspaces-triggers-get",
                    Some(r##"Gets a GTM Trigger."##),
                    "Details at http://byron.github.io/google-apis-rs/google_tagmanager2_cli/accounts_containers-workspaces-triggers-get",
                  vec![
                    (Some(r##"path"##),
                     None,
                     Some(r##"GTM Trigger's API relative path. Example: accounts/{account_id}/containers/{container_id}/workspaces/{workspace_id}/triggers/{trigger_id}"##),
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
            ("containers-workspaces-triggers-list",
                    Some(r##"Lists all GTM Triggers of a Container."##),
                    "Details at http://byron.github.io/google-apis-rs/google_tagmanager2_cli/accounts_containers-workspaces-triggers-list",
                  vec![
                    (Some(r##"parent"##),
                     None,
                     Some(r##"GTM Workspace's API relative path. Example: accounts/{account_id}/containers/{container_id}/workspaces/{workspace_id}"##),
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
            ("containers-workspaces-triggers-revert",
                    Some(r##"Reverts changes to a GTM Trigger in a GTM Workspace."##),
                    "Details at http://byron.github.io/google-apis-rs/google_tagmanager2_cli/accounts_containers-workspaces-triggers-revert",
                  vec![
                    (Some(r##"path"##),
                     None,
                     Some(r##"GTM Trigger's API relative path. Example: accounts/{account_id}/containers/{container_id}/workspaces/{workspace_id}/triggers/{trigger_id}"##),
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
            ("containers-workspaces-triggers-update",
                    Some(r##"Updates a GTM Trigger."##),
                    "Details at http://byron.github.io/google-apis-rs/google_tagmanager2_cli/accounts_containers-workspaces-triggers-update",
                  vec![
                    (Some(r##"path"##),
                     None,
                     Some(r##"GTM Trigger's API relative path. Example: accounts/{account_id}/containers/{container_id}/workspaces/{workspace_id}/triggers/{trigger_id}"##),
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
            ("containers-workspaces-update",
                    Some(r##"Updates a Workspace."##),
                    "Details at http://byron.github.io/google-apis-rs/google_tagmanager2_cli/accounts_containers-workspaces-update",
                  vec![
                    (Some(r##"path"##),
                     None,
                     Some(r##"GTM Workspace's API relative path. Example: accounts/{account_id}/containers/{container_id}/workspaces/{workspace_id}"##),
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
            ("containers-workspaces-variables-create",
                    Some(r##"Creates a GTM Variable."##),
                    "Details at http://byron.github.io/google-apis-rs/google_tagmanager2_cli/accounts_containers-workspaces-variables-create",
                  vec![
                    (Some(r##"parent"##),
                     None,
                     Some(r##"GTM Workspace's API relative path. Example: accounts/{account_id}/containers/{container_id}/workspaces/{workspace_id}"##),
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
            ("containers-workspaces-variables-delete",
                    Some(r##"Deletes a GTM Variable."##),
                    "Details at http://byron.github.io/google-apis-rs/google_tagmanager2_cli/accounts_containers-workspaces-variables-delete",
                  vec![
                    (Some(r##"path"##),
                     None,
                     Some(r##"GTM Variable's API relative path. Example: accounts/{account_id}/containers/{container_id}/workspaces/{workspace_id}/variables/{variable_id}"##),
                     Some(true),
                     Some(false)),
        
                    (Some(r##"v"##),
                     Some(r##"p"##),
                     Some(r##"Set various optional parameters, matching the key=value form"##),
                     Some(false),
                     Some(true)),
                  ]),
            ("containers-workspaces-variables-get",
                    Some(r##"Gets a GTM Variable."##),
                    "Details at http://byron.github.io/google-apis-rs/google_tagmanager2_cli/accounts_containers-workspaces-variables-get",
                  vec![
                    (Some(r##"path"##),
                     None,
                     Some(r##"GTM Variable's API relative path. Example: accounts/{account_id}/containers/{container_id}/workspaces/{workspace_id}/variables/{variable_id}"##),
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
            ("containers-workspaces-variables-list",
                    Some(r##"Lists all GTM Variables of a Container."##),
                    "Details at http://byron.github.io/google-apis-rs/google_tagmanager2_cli/accounts_containers-workspaces-variables-list",
                  vec![
                    (Some(r##"parent"##),
                     None,
                     Some(r##"GTM Workspace's API relative path. Example: accounts/{account_id}/containers/{container_id}/workspaces/{workspace_id}"##),
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
            ("containers-workspaces-variables-revert",
                    Some(r##"Reverts changes to a GTM Variable in a GTM Workspace."##),
                    "Details at http://byron.github.io/google-apis-rs/google_tagmanager2_cli/accounts_containers-workspaces-variables-revert",
                  vec![
                    (Some(r##"path"##),
                     None,
                     Some(r##"GTM Variable's API relative path. Example: accounts/{account_id}/containers/{container_id}/workspaces/{workspace_id}/variables/{variable_id}"##),
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
            ("containers-workspaces-variables-update",
                    Some(r##"Updates a GTM Variable."##),
                    "Details at http://byron.github.io/google-apis-rs/google_tagmanager2_cli/accounts_containers-workspaces-variables-update",
                  vec![
                    (Some(r##"path"##),
                     None,
                     Some(r##"GTM Variable's API relative path. Example: accounts/{account_id}/containers/{container_id}/workspaces/{workspace_id}/variables/{variable_id}"##),
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
            ("containers-workspaces-zones-create",
                    Some(r##"Creates a GTM Zone."##),
                    "Details at http://byron.github.io/google-apis-rs/google_tagmanager2_cli/accounts_containers-workspaces-zones-create",
                  vec![
                    (Some(r##"parent"##),
                     None,
                     Some(r##"GTM Workspace's API relative path. Example: accounts/{account_id}/containers/{container_id}/workspaces/{workspace_id}"##),
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
            ("containers-workspaces-zones-delete",
                    Some(r##"Deletes a GTM Zone."##),
                    "Details at http://byron.github.io/google-apis-rs/google_tagmanager2_cli/accounts_containers-workspaces-zones-delete",
                  vec![
                    (Some(r##"path"##),
                     None,
                     Some(r##"GTM Zone's API relative path. Example: accounts/{account_id}/containers/{container_id}/workspaces/{workspace_id}/zones/{zone_id}"##),
                     Some(true),
                     Some(false)),
        
                    (Some(r##"v"##),
                     Some(r##"p"##),
                     Some(r##"Set various optional parameters, matching the key=value form"##),
                     Some(false),
                     Some(true)),
                  ]),
            ("containers-workspaces-zones-get",
                    Some(r##"Gets a GTM Zone."##),
                    "Details at http://byron.github.io/google-apis-rs/google_tagmanager2_cli/accounts_containers-workspaces-zones-get",
                  vec![
                    (Some(r##"path"##),
                     None,
                     Some(r##"GTM Zone's API relative path. Example: accounts/{account_id}/containers/{container_id}/workspaces/{workspace_id}/zones/{zone_id}"##),
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
            ("containers-workspaces-zones-list",
                    Some(r##"Lists all GTM Zones of a GTM container workspace."##),
                    "Details at http://byron.github.io/google-apis-rs/google_tagmanager2_cli/accounts_containers-workspaces-zones-list",
                  vec![
                    (Some(r##"parent"##),
                     None,
                     Some(r##"GTM Workspace's API relative path. Example: accounts/{account_id}/containers/{container_id}/workspaces/{workspace_id}"##),
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
            ("containers-workspaces-zones-revert",
                    Some(r##"Reverts changes to a GTM Zone in a GTM Workspace."##),
                    "Details at http://byron.github.io/google-apis-rs/google_tagmanager2_cli/accounts_containers-workspaces-zones-revert",
                  vec![
                    (Some(r##"path"##),
                     None,
                     Some(r##"GTM Zone's API relative path. Example: accounts/{account_id}/containers/{container_id}/workspaces/{workspace_id}/zones/{zone_id}"##),
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
            ("containers-workspaces-zones-update",
                    Some(r##"Updates a GTM Zone."##),
                    "Details at http://byron.github.io/google-apis-rs/google_tagmanager2_cli/accounts_containers-workspaces-zones-update",
                  vec![
                    (Some(r##"path"##),
                     None,
                     Some(r##"GTM Zone's API relative path. Example: accounts/{account_id}/containers/{container_id}/workspaces/{workspace_id}/zones/{zone_id}"##),
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
            ("get",
                    Some(r##"Gets a GTM Account."##),
                    "Details at http://byron.github.io/google-apis-rs/google_tagmanager2_cli/accounts_get",
                  vec![
                    (Some(r##"path"##),
                     None,
                     Some(r##"GTM Account's API relative path. Example: accounts/{account_id}"##),
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
                    Some(r##"Lists all GTM Accounts that a user has access to."##),
                    "Details at http://byron.github.io/google-apis-rs/google_tagmanager2_cli/accounts_list",
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
                    Some(r##"Updates a GTM Account."##),
                    "Details at http://byron.github.io/google-apis-rs/google_tagmanager2_cli/accounts_update",
                  vec![
                    (Some(r##"path"##),
                     None,
                     Some(r##"GTM Account's API relative path. Example: accounts/{account_id}"##),
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
            ("user-permissions-create",
                    Some(r##"Creates a user's Account & Container access."##),
                    "Details at http://byron.github.io/google-apis-rs/google_tagmanager2_cli/accounts_user-permissions-create",
                  vec![
                    (Some(r##"parent"##),
                     None,
                     Some(r##"GTM Account's API relative path. Example: accounts/{account_id}"##),
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
            ("user-permissions-delete",
                    Some(r##"Removes a user from the account, revoking access to it and all of its containers."##),
                    "Details at http://byron.github.io/google-apis-rs/google_tagmanager2_cli/accounts_user-permissions-delete",
                  vec![
                    (Some(r##"path"##),
                     None,
                     Some(r##"GTM UserPermission's API relative path. Example: accounts/{account_id}/user_permissions/{user_permission_id}"##),
                     Some(true),
                     Some(false)),
        
                    (Some(r##"v"##),
                     Some(r##"p"##),
                     Some(r##"Set various optional parameters, matching the key=value form"##),
                     Some(false),
                     Some(true)),
                  ]),
            ("user-permissions-get",
                    Some(r##"Gets a user's Account & Container access."##),
                    "Details at http://byron.github.io/google-apis-rs/google_tagmanager2_cli/accounts_user-permissions-get",
                  vec![
                    (Some(r##"path"##),
                     None,
                     Some(r##"GTM UserPermission's API relative path. Example: accounts/{account_id}/user_permissions/{user_permission_id}"##),
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
            ("user-permissions-list",
                    Some(r##"List all users that have access to the account along with Account and Container user access granted to each of them."##),
                    "Details at http://byron.github.io/google-apis-rs/google_tagmanager2_cli/accounts_user-permissions-list",
                  vec![
                    (Some(r##"parent"##),
                     None,
                     Some(r##"GTM Account's API relative path. Example: accounts/{account_id}"##),
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
            ("user-permissions-update",
                    Some(r##"Updates a user's Account & Container access."##),
                    "Details at http://byron.github.io/google-apis-rs/google_tagmanager2_cli/accounts_user-permissions-update",
                  vec![
                    (Some(r##"path"##),
                     None,
                     Some(r##"GTM UserPermission's API relative path. Example: accounts/{account_id}/user_permissions/{user_permission_id}"##),
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
    
    let mut app = App::new("tagmanager2")
           .author("Sebastian Thiel <byronimo@gmail.com>")
           .version("5.0.5+20240619")
           .about("This API allows clients to access and modify container and tag configuration.")
           .after_help("All documentation details can be found at http://byron.github.io/google-apis-rs/google_tagmanager2_cli")
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
