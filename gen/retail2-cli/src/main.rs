// DO NOT EDIT !
// This file was generated automatically from 'src/generator/templates/cli/main.rs.mako'
// DO NOT EDIT !
#![allow(unused_variables, unused_imports, dead_code, unused_mut)]

#[macro_use]
extern crate clap;

use std::env;
use std::io::{self, Write};
use clap::{App, SubCommand, Arg};

use google_retail2::{api, Error, oauth2, client::chrono, FieldMask};


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
    hub: api::CloudRetail<S>,
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
    async fn _projects_locations_catalogs_attributes_config_add_catalog_attribute(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
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
                    "catalog-attribute.dynamic-facetable-option" => Some(("catalogAttribute.dynamicFacetableOption", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "catalog-attribute.exact-searchable-option" => Some(("catalogAttribute.exactSearchableOption", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "catalog-attribute.facet-config.merged-facet.merged-facet-key" => Some(("catalogAttribute.facetConfig.mergedFacet.mergedFacetKey", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "catalog-attribute.facet-config.rerank-config.facet-values" => Some(("catalogAttribute.facetConfig.rerankConfig.facetValues", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Vec })),
                    "catalog-attribute.facet-config.rerank-config.rerank-facet" => Some(("catalogAttribute.facetConfig.rerankConfig.rerankFacet", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "catalog-attribute.in-use" => Some(("catalogAttribute.inUse", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "catalog-attribute.indexable-option" => Some(("catalogAttribute.indexableOption", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "catalog-attribute.key" => Some(("catalogAttribute.key", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "catalog-attribute.retrievable-option" => Some(("catalogAttribute.retrievableOption", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "catalog-attribute.searchable-option" => Some(("catalogAttribute.searchableOption", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "catalog-attribute.type" => Some(("catalogAttribute.type", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    _ => {
                        let suggestion = FieldCursor::did_you_mean(key, &vec!["catalog-attribute", "dynamic-facetable-option", "exact-searchable-option", "facet-config", "facet-values", "in-use", "indexable-option", "key", "merged-facet", "merged-facet-key", "rerank-config", "rerank-facet", "retrievable-option", "searchable-option", "type"]);
                        err.issues.push(CLIError::Field(FieldError::Unknown(temp_cursor.to_string(), suggestion, value.map(|v| v.to_string()))));
                        None
                    }
                };
            if let Some((field_cursor_str, type_info)) = type_info {
                FieldCursor::from(field_cursor_str).set_json_value(&mut object, value.unwrap(), type_info, err, &temp_cursor);
            }
        }
        let mut request: api::GoogleCloudRetailV2AddCatalogAttributeRequest = json::value::from_value(object).unwrap();
        let mut call = self.hub.projects().locations_catalogs_attributes_config_add_catalog_attribute(request, opt.value_of("attributes-config").unwrap_or(""));
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

    async fn _projects_locations_catalogs_attributes_config_remove_catalog_attribute(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
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
                    "key" => Some(("key", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    _ => {
                        let suggestion = FieldCursor::did_you_mean(key, &vec!["key"]);
                        err.issues.push(CLIError::Field(FieldError::Unknown(temp_cursor.to_string(), suggestion, value.map(|v| v.to_string()))));
                        None
                    }
                };
            if let Some((field_cursor_str, type_info)) = type_info {
                FieldCursor::from(field_cursor_str).set_json_value(&mut object, value.unwrap(), type_info, err, &temp_cursor);
            }
        }
        let mut request: api::GoogleCloudRetailV2RemoveCatalogAttributeRequest = json::value::from_value(object).unwrap();
        let mut call = self.hub.projects().locations_catalogs_attributes_config_remove_catalog_attribute(request, opt.value_of("attributes-config").unwrap_or(""));
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

    async fn _projects_locations_catalogs_attributes_config_replace_catalog_attribute(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
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
                    "catalog-attribute.dynamic-facetable-option" => Some(("catalogAttribute.dynamicFacetableOption", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "catalog-attribute.exact-searchable-option" => Some(("catalogAttribute.exactSearchableOption", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "catalog-attribute.facet-config.merged-facet.merged-facet-key" => Some(("catalogAttribute.facetConfig.mergedFacet.mergedFacetKey", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "catalog-attribute.facet-config.rerank-config.facet-values" => Some(("catalogAttribute.facetConfig.rerankConfig.facetValues", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Vec })),
                    "catalog-attribute.facet-config.rerank-config.rerank-facet" => Some(("catalogAttribute.facetConfig.rerankConfig.rerankFacet", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "catalog-attribute.in-use" => Some(("catalogAttribute.inUse", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "catalog-attribute.indexable-option" => Some(("catalogAttribute.indexableOption", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "catalog-attribute.key" => Some(("catalogAttribute.key", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "catalog-attribute.retrievable-option" => Some(("catalogAttribute.retrievableOption", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "catalog-attribute.searchable-option" => Some(("catalogAttribute.searchableOption", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "catalog-attribute.type" => Some(("catalogAttribute.type", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "update-mask" => Some(("updateMask", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    _ => {
                        let suggestion = FieldCursor::did_you_mean(key, &vec!["catalog-attribute", "dynamic-facetable-option", "exact-searchable-option", "facet-config", "facet-values", "in-use", "indexable-option", "key", "merged-facet", "merged-facet-key", "rerank-config", "rerank-facet", "retrievable-option", "searchable-option", "type", "update-mask"]);
                        err.issues.push(CLIError::Field(FieldError::Unknown(temp_cursor.to_string(), suggestion, value.map(|v| v.to_string()))));
                        None
                    }
                };
            if let Some((field_cursor_str, type_info)) = type_info {
                FieldCursor::from(field_cursor_str).set_json_value(&mut object, value.unwrap(), type_info, err, &temp_cursor);
            }
        }
        let mut request: api::GoogleCloudRetailV2ReplaceCatalogAttributeRequest = json::value::from_value(object).unwrap();
        let mut call = self.hub.projects().locations_catalogs_attributes_config_replace_catalog_attribute(request, opt.value_of("attributes-config").unwrap_or(""));
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

    async fn _projects_locations_catalogs_branches_operations_get(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        let mut call = self.hub.projects().locations_catalogs_branches_operations_get(opt.value_of("name").unwrap_or(""));
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

    async fn _projects_locations_catalogs_branches_products_add_fulfillment_places(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
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
                    "add-time" => Some(("addTime", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "allow-missing" => Some(("allowMissing", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "place-ids" => Some(("placeIds", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Vec })),
                    "type" => Some(("type", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    _ => {
                        let suggestion = FieldCursor::did_you_mean(key, &vec!["add-time", "allow-missing", "place-ids", "type"]);
                        err.issues.push(CLIError::Field(FieldError::Unknown(temp_cursor.to_string(), suggestion, value.map(|v| v.to_string()))));
                        None
                    }
                };
            if let Some((field_cursor_str, type_info)) = type_info {
                FieldCursor::from(field_cursor_str).set_json_value(&mut object, value.unwrap(), type_info, err, &temp_cursor);
            }
        }
        let mut request: api::GoogleCloudRetailV2AddFulfillmentPlacesRequest = json::value::from_value(object).unwrap();
        let mut call = self.hub.projects().locations_catalogs_branches_products_add_fulfillment_places(request, opt.value_of("product").unwrap_or(""));
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

    async fn _projects_locations_catalogs_branches_products_add_local_inventories(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
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
                    "add-mask" => Some(("addMask", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "add-time" => Some(("addTime", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "allow-missing" => Some(("allowMissing", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    _ => {
                        let suggestion = FieldCursor::did_you_mean(key, &vec!["add-mask", "add-time", "allow-missing"]);
                        err.issues.push(CLIError::Field(FieldError::Unknown(temp_cursor.to_string(), suggestion, value.map(|v| v.to_string()))));
                        None
                    }
                };
            if let Some((field_cursor_str, type_info)) = type_info {
                FieldCursor::from(field_cursor_str).set_json_value(&mut object, value.unwrap(), type_info, err, &temp_cursor);
            }
        }
        let mut request: api::GoogleCloudRetailV2AddLocalInventoriesRequest = json::value::from_value(object).unwrap();
        let mut call = self.hub.projects().locations_catalogs_branches_products_add_local_inventories(request, opt.value_of("product").unwrap_or(""));
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

    async fn _projects_locations_catalogs_branches_products_create(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
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
                    "audience.age-groups" => Some(("audience.ageGroups", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Vec })),
                    "audience.genders" => Some(("audience.genders", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Vec })),
                    "availability" => Some(("availability", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "available-quantity" => Some(("availableQuantity", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "available-time" => Some(("availableTime", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "brands" => Some(("brands", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Vec })),
                    "categories" => Some(("categories", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Vec })),
                    "collection-member-ids" => Some(("collectionMemberIds", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Vec })),
                    "color-info.color-families" => Some(("colorInfo.colorFamilies", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Vec })),
                    "color-info.colors" => Some(("colorInfo.colors", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Vec })),
                    "conditions" => Some(("conditions", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Vec })),
                    "description" => Some(("description", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "expire-time" => Some(("expireTime", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "gtin" => Some(("gtin", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "id" => Some(("id", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "language-code" => Some(("languageCode", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "materials" => Some(("materials", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Vec })),
                    "name" => Some(("name", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "patterns" => Some(("patterns", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Vec })),
                    "price-info.cost" => Some(("priceInfo.cost", JsonTypeInfo { jtype: JsonType::Float, ctype: ComplexType::Pod })),
                    "price-info.currency-code" => Some(("priceInfo.currencyCode", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "price-info.original-price" => Some(("priceInfo.originalPrice", JsonTypeInfo { jtype: JsonType::Float, ctype: ComplexType::Pod })),
                    "price-info.price" => Some(("priceInfo.price", JsonTypeInfo { jtype: JsonType::Float, ctype: ComplexType::Pod })),
                    "price-info.price-effective-time" => Some(("priceInfo.priceEffectiveTime", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "price-info.price-expire-time" => Some(("priceInfo.priceExpireTime", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "price-info.price-range.original-price.exclusive-maximum" => Some(("priceInfo.priceRange.originalPrice.exclusiveMaximum", JsonTypeInfo { jtype: JsonType::Float, ctype: ComplexType::Pod })),
                    "price-info.price-range.original-price.exclusive-minimum" => Some(("priceInfo.priceRange.originalPrice.exclusiveMinimum", JsonTypeInfo { jtype: JsonType::Float, ctype: ComplexType::Pod })),
                    "price-info.price-range.original-price.maximum" => Some(("priceInfo.priceRange.originalPrice.maximum", JsonTypeInfo { jtype: JsonType::Float, ctype: ComplexType::Pod })),
                    "price-info.price-range.original-price.minimum" => Some(("priceInfo.priceRange.originalPrice.minimum", JsonTypeInfo { jtype: JsonType::Float, ctype: ComplexType::Pod })),
                    "price-info.price-range.price.exclusive-maximum" => Some(("priceInfo.priceRange.price.exclusiveMaximum", JsonTypeInfo { jtype: JsonType::Float, ctype: ComplexType::Pod })),
                    "price-info.price-range.price.exclusive-minimum" => Some(("priceInfo.priceRange.price.exclusiveMinimum", JsonTypeInfo { jtype: JsonType::Float, ctype: ComplexType::Pod })),
                    "price-info.price-range.price.maximum" => Some(("priceInfo.priceRange.price.maximum", JsonTypeInfo { jtype: JsonType::Float, ctype: ComplexType::Pod })),
                    "price-info.price-range.price.minimum" => Some(("priceInfo.priceRange.price.minimum", JsonTypeInfo { jtype: JsonType::Float, ctype: ComplexType::Pod })),
                    "primary-product-id" => Some(("primaryProductId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "publish-time" => Some(("publishTime", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "rating.average-rating" => Some(("rating.averageRating", JsonTypeInfo { jtype: JsonType::Float, ctype: ComplexType::Pod })),
                    "rating.rating-count" => Some(("rating.ratingCount", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "rating.rating-histogram" => Some(("rating.ratingHistogram", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Vec })),
                    "retrievable-fields" => Some(("retrievableFields", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "sizes" => Some(("sizes", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Vec })),
                    "tags" => Some(("tags", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Vec })),
                    "title" => Some(("title", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "ttl" => Some(("ttl", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "type" => Some(("type", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "uri" => Some(("uri", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    _ => {
                        let suggestion = FieldCursor::did_you_mean(key, &vec!["age-groups", "audience", "availability", "available-quantity", "available-time", "average-rating", "brands", "categories", "collection-member-ids", "color-families", "color-info", "colors", "conditions", "cost", "currency-code", "description", "exclusive-maximum", "exclusive-minimum", "expire-time", "genders", "gtin", "id", "language-code", "materials", "maximum", "minimum", "name", "original-price", "patterns", "price", "price-effective-time", "price-expire-time", "price-info", "price-range", "primary-product-id", "publish-time", "rating", "rating-count", "rating-histogram", "retrievable-fields", "sizes", "tags", "title", "ttl", "type", "uri"]);
                        err.issues.push(CLIError::Field(FieldError::Unknown(temp_cursor.to_string(), suggestion, value.map(|v| v.to_string()))));
                        None
                    }
                };
            if let Some((field_cursor_str, type_info)) = type_info {
                FieldCursor::from(field_cursor_str).set_json_value(&mut object, value.unwrap(), type_info, err, &temp_cursor);
            }
        }
        let mut request: api::GoogleCloudRetailV2Product = json::value::from_value(object).unwrap();
        let mut call = self.hub.projects().locations_catalogs_branches_products_create(request, opt.value_of("parent").unwrap_or(""));
        for parg in opt.values_of("v").map(|i|i.collect()).unwrap_or(Vec::new()).iter() {
            let (key, value) = parse_kv_arg(&*parg, err, false);
            match key {
                "product-id" => {
                    call = call.product_id(value.unwrap_or(""));
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
                                                                           v.extend(["product-id"].iter().map(|v|*v));
                                                                           v } ));
                    }
                }
            }
        }
        let protocol = CallType::Standard;
        if dry_run {
            Ok(())
        } else {
            assert!(err.issues.len() == 0);
            for scope in self.opt.values_of("url").map(|i|i.collect()).unwrap_or(Vec::new()).iter() {
                call = call.add_scope(scope);
            }
            let mut ostream = match writer_from_opts(opt.value_of("out")) {
                Ok(mut f) => f,
                Err(io_err) => return Err(DoitError::IoError(opt.value_of("out").unwrap_or("-").to_string(), io_err)),
            };
            match match protocol {
                CallType::Standard => call.doit().await,
                _ => unreachable!()
            } {
                Err(api_err) => Err(DoitError::ApiError(api_err)),
                Ok((mut response, output_schema)) => {
                    let mut value = json::value::to_value(&output_schema).expect("serde to work");
                    remove_json_null_values(&mut value);
                    json::to_writer_pretty(&mut ostream, &value).unwrap();
                    ostream.flush().unwrap();
                    Ok(())
                }
            }
        }
    }

    async fn _projects_locations_catalogs_branches_products_delete(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        let mut call = self.hub.projects().locations_catalogs_branches_products_delete(opt.value_of("name").unwrap_or(""));
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

    async fn _projects_locations_catalogs_branches_products_get(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        let mut call = self.hub.projects().locations_catalogs_branches_products_get(opt.value_of("name").unwrap_or(""));
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

    async fn _projects_locations_catalogs_branches_products_import(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
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
                    "errors-config.gcs-prefix" => Some(("errorsConfig.gcsPrefix", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "input-config.big-query-source.data-schema" => Some(("inputConfig.bigQuerySource.dataSchema", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "input-config.big-query-source.dataset-id" => Some(("inputConfig.bigQuerySource.datasetId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "input-config.big-query-source.gcs-staging-dir" => Some(("inputConfig.bigQuerySource.gcsStagingDir", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "input-config.big-query-source.partition-date.day" => Some(("inputConfig.bigQuerySource.partitionDate.day", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "input-config.big-query-source.partition-date.month" => Some(("inputConfig.bigQuerySource.partitionDate.month", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "input-config.big-query-source.partition-date.year" => Some(("inputConfig.bigQuerySource.partitionDate.year", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "input-config.big-query-source.project-id" => Some(("inputConfig.bigQuerySource.projectId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "input-config.big-query-source.table-id" => Some(("inputConfig.bigQuerySource.tableId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "input-config.gcs-source.data-schema" => Some(("inputConfig.gcsSource.dataSchema", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "input-config.gcs-source.input-uris" => Some(("inputConfig.gcsSource.inputUris", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Vec })),
                    "notification-pubsub-topic" => Some(("notificationPubsubTopic", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "reconciliation-mode" => Some(("reconciliationMode", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "request-id" => Some(("requestId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "update-mask" => Some(("updateMask", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    _ => {
                        let suggestion = FieldCursor::did_you_mean(key, &vec!["big-query-source", "data-schema", "dataset-id", "day", "errors-config", "gcs-prefix", "gcs-source", "gcs-staging-dir", "input-config", "input-uris", "month", "notification-pubsub-topic", "partition-date", "project-id", "reconciliation-mode", "request-id", "table-id", "update-mask", "year"]);
                        err.issues.push(CLIError::Field(FieldError::Unknown(temp_cursor.to_string(), suggestion, value.map(|v| v.to_string()))));
                        None
                    }
                };
            if let Some((field_cursor_str, type_info)) = type_info {
                FieldCursor::from(field_cursor_str).set_json_value(&mut object, value.unwrap(), type_info, err, &temp_cursor);
            }
        }
        let mut request: api::GoogleCloudRetailV2ImportProductsRequest = json::value::from_value(object).unwrap();
        let mut call = self.hub.projects().locations_catalogs_branches_products_import(request, opt.value_of("parent").unwrap_or(""));
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

    async fn _projects_locations_catalogs_branches_products_list(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        let mut call = self.hub.projects().locations_catalogs_branches_products_list(opt.value_of("parent").unwrap_or(""));
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
                                                                           v.extend(["filter", "page-size", "page-token", "read-mask"].iter().map(|v|*v));
                                                                           v } ));
                    }
                }
            }
        }
        let protocol = CallType::Standard;
        if dry_run {
            Ok(())
        } else {
            assert!(err.issues.len() == 0);
            for scope in self.opt.values_of("url").map(|i|i.collect()).unwrap_or(Vec::new()).iter() {
                call = call.add_scope(scope);
            }
            let mut ostream = match writer_from_opts(opt.value_of("out")) {
                Ok(mut f) => f,
                Err(io_err) => return Err(DoitError::IoError(opt.value_of("out").unwrap_or("-").to_string(), io_err)),
            };
            match match protocol {
                CallType::Standard => call.doit().await,
                _ => unreachable!()
            } {
                Err(api_err) => Err(DoitError::ApiError(api_err)),
                Ok((mut response, output_schema)) => {
                    let mut value = json::value::to_value(&output_schema).expect("serde to work");
                    remove_json_null_values(&mut value);
                    json::to_writer_pretty(&mut ostream, &value).unwrap();
                    ostream.flush().unwrap();
                    Ok(())
                }
            }
        }
    }

    async fn _projects_locations_catalogs_branches_products_patch(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
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
                    "audience.age-groups" => Some(("audience.ageGroups", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Vec })),
                    "audience.genders" => Some(("audience.genders", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Vec })),
                    "availability" => Some(("availability", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "available-quantity" => Some(("availableQuantity", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "available-time" => Some(("availableTime", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "brands" => Some(("brands", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Vec })),
                    "categories" => Some(("categories", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Vec })),
                    "collection-member-ids" => Some(("collectionMemberIds", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Vec })),
                    "color-info.color-families" => Some(("colorInfo.colorFamilies", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Vec })),
                    "color-info.colors" => Some(("colorInfo.colors", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Vec })),
                    "conditions" => Some(("conditions", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Vec })),
                    "description" => Some(("description", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "expire-time" => Some(("expireTime", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "gtin" => Some(("gtin", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "id" => Some(("id", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "language-code" => Some(("languageCode", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "materials" => Some(("materials", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Vec })),
                    "name" => Some(("name", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "patterns" => Some(("patterns", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Vec })),
                    "price-info.cost" => Some(("priceInfo.cost", JsonTypeInfo { jtype: JsonType::Float, ctype: ComplexType::Pod })),
                    "price-info.currency-code" => Some(("priceInfo.currencyCode", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "price-info.original-price" => Some(("priceInfo.originalPrice", JsonTypeInfo { jtype: JsonType::Float, ctype: ComplexType::Pod })),
                    "price-info.price" => Some(("priceInfo.price", JsonTypeInfo { jtype: JsonType::Float, ctype: ComplexType::Pod })),
                    "price-info.price-effective-time" => Some(("priceInfo.priceEffectiveTime", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "price-info.price-expire-time" => Some(("priceInfo.priceExpireTime", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "price-info.price-range.original-price.exclusive-maximum" => Some(("priceInfo.priceRange.originalPrice.exclusiveMaximum", JsonTypeInfo { jtype: JsonType::Float, ctype: ComplexType::Pod })),
                    "price-info.price-range.original-price.exclusive-minimum" => Some(("priceInfo.priceRange.originalPrice.exclusiveMinimum", JsonTypeInfo { jtype: JsonType::Float, ctype: ComplexType::Pod })),
                    "price-info.price-range.original-price.maximum" => Some(("priceInfo.priceRange.originalPrice.maximum", JsonTypeInfo { jtype: JsonType::Float, ctype: ComplexType::Pod })),
                    "price-info.price-range.original-price.minimum" => Some(("priceInfo.priceRange.originalPrice.minimum", JsonTypeInfo { jtype: JsonType::Float, ctype: ComplexType::Pod })),
                    "price-info.price-range.price.exclusive-maximum" => Some(("priceInfo.priceRange.price.exclusiveMaximum", JsonTypeInfo { jtype: JsonType::Float, ctype: ComplexType::Pod })),
                    "price-info.price-range.price.exclusive-minimum" => Some(("priceInfo.priceRange.price.exclusiveMinimum", JsonTypeInfo { jtype: JsonType::Float, ctype: ComplexType::Pod })),
                    "price-info.price-range.price.maximum" => Some(("priceInfo.priceRange.price.maximum", JsonTypeInfo { jtype: JsonType::Float, ctype: ComplexType::Pod })),
                    "price-info.price-range.price.minimum" => Some(("priceInfo.priceRange.price.minimum", JsonTypeInfo { jtype: JsonType::Float, ctype: ComplexType::Pod })),
                    "primary-product-id" => Some(("primaryProductId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "publish-time" => Some(("publishTime", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "rating.average-rating" => Some(("rating.averageRating", JsonTypeInfo { jtype: JsonType::Float, ctype: ComplexType::Pod })),
                    "rating.rating-count" => Some(("rating.ratingCount", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "rating.rating-histogram" => Some(("rating.ratingHistogram", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Vec })),
                    "retrievable-fields" => Some(("retrievableFields", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "sizes" => Some(("sizes", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Vec })),
                    "tags" => Some(("tags", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Vec })),
                    "title" => Some(("title", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "ttl" => Some(("ttl", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "type" => Some(("type", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "uri" => Some(("uri", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    _ => {
                        let suggestion = FieldCursor::did_you_mean(key, &vec!["age-groups", "audience", "availability", "available-quantity", "available-time", "average-rating", "brands", "categories", "collection-member-ids", "color-families", "color-info", "colors", "conditions", "cost", "currency-code", "description", "exclusive-maximum", "exclusive-minimum", "expire-time", "genders", "gtin", "id", "language-code", "materials", "maximum", "minimum", "name", "original-price", "patterns", "price", "price-effective-time", "price-expire-time", "price-info", "price-range", "primary-product-id", "publish-time", "rating", "rating-count", "rating-histogram", "retrievable-fields", "sizes", "tags", "title", "ttl", "type", "uri"]);
                        err.issues.push(CLIError::Field(FieldError::Unknown(temp_cursor.to_string(), suggestion, value.map(|v| v.to_string()))));
                        None
                    }
                };
            if let Some((field_cursor_str, type_info)) = type_info {
                FieldCursor::from(field_cursor_str).set_json_value(&mut object, value.unwrap(), type_info, err, &temp_cursor);
            }
        }
        let mut request: api::GoogleCloudRetailV2Product = json::value::from_value(object).unwrap();
        let mut call = self.hub.projects().locations_catalogs_branches_products_patch(request, opt.value_of("name").unwrap_or(""));
        for parg in opt.values_of("v").map(|i|i.collect()).unwrap_or(Vec::new()).iter() {
            let (key, value) = parse_kv_arg(&*parg, err, false);
            match key {
                "update-mask" => {
                    call = call.update_mask(        value.map(|v| arg_from_str(v, err, "update-mask", "google-fieldmask")).unwrap_or(FieldMask::default()));
                },
                "allow-missing" => {
                    call = call.allow_missing(        value.map(|v| arg_from_str(v, err, "allow-missing", "boolean")).unwrap_or(false));
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
                                                                           v.extend(["allow-missing", "update-mask"].iter().map(|v|*v));
                                                                           v } ));
                    }
                }
            }
        }
        let protocol = CallType::Standard;
        if dry_run {
            Ok(())
        } else {
            assert!(err.issues.len() == 0);
            for scope in self.opt.values_of("url").map(|i|i.collect()).unwrap_or(Vec::new()).iter() {
                call = call.add_scope(scope);
            }
            let mut ostream = match writer_from_opts(opt.value_of("out")) {
                Ok(mut f) => f,
                Err(io_err) => return Err(DoitError::IoError(opt.value_of("out").unwrap_or("-").to_string(), io_err)),
            };
            match match protocol {
                CallType::Standard => call.doit().await,
                _ => unreachable!()
            } {
                Err(api_err) => Err(DoitError::ApiError(api_err)),
                Ok((mut response, output_schema)) => {
                    let mut value = json::value::to_value(&output_schema).expect("serde to work");
                    remove_json_null_values(&mut value);
                    json::to_writer_pretty(&mut ostream, &value).unwrap();
                    ostream.flush().unwrap();
                    Ok(())
                }
            }
        }
    }

    async fn _projects_locations_catalogs_branches_products_purge(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
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
                    _ => {
                        let suggestion = FieldCursor::did_you_mean(key, &vec!["filter", "force"]);
                        err.issues.push(CLIError::Field(FieldError::Unknown(temp_cursor.to_string(), suggestion, value.map(|v| v.to_string()))));
                        None
                    }
                };
            if let Some((field_cursor_str, type_info)) = type_info {
                FieldCursor::from(field_cursor_str).set_json_value(&mut object, value.unwrap(), type_info, err, &temp_cursor);
            }
        }
        let mut request: api::GoogleCloudRetailV2PurgeProductsRequest = json::value::from_value(object).unwrap();
        let mut call = self.hub.projects().locations_catalogs_branches_products_purge(request, opt.value_of("parent").unwrap_or(""));
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

    async fn _projects_locations_catalogs_branches_products_remove_fulfillment_places(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
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
                    "allow-missing" => Some(("allowMissing", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "place-ids" => Some(("placeIds", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Vec })),
                    "remove-time" => Some(("removeTime", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "type" => Some(("type", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    _ => {
                        let suggestion = FieldCursor::did_you_mean(key, &vec!["allow-missing", "place-ids", "remove-time", "type"]);
                        err.issues.push(CLIError::Field(FieldError::Unknown(temp_cursor.to_string(), suggestion, value.map(|v| v.to_string()))));
                        None
                    }
                };
            if let Some((field_cursor_str, type_info)) = type_info {
                FieldCursor::from(field_cursor_str).set_json_value(&mut object, value.unwrap(), type_info, err, &temp_cursor);
            }
        }
        let mut request: api::GoogleCloudRetailV2RemoveFulfillmentPlacesRequest = json::value::from_value(object).unwrap();
        let mut call = self.hub.projects().locations_catalogs_branches_products_remove_fulfillment_places(request, opt.value_of("product").unwrap_or(""));
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

    async fn _projects_locations_catalogs_branches_products_remove_local_inventories(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
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
                    "allow-missing" => Some(("allowMissing", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "place-ids" => Some(("placeIds", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Vec })),
                    "remove-time" => Some(("removeTime", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    _ => {
                        let suggestion = FieldCursor::did_you_mean(key, &vec!["allow-missing", "place-ids", "remove-time"]);
                        err.issues.push(CLIError::Field(FieldError::Unknown(temp_cursor.to_string(), suggestion, value.map(|v| v.to_string()))));
                        None
                    }
                };
            if let Some((field_cursor_str, type_info)) = type_info {
                FieldCursor::from(field_cursor_str).set_json_value(&mut object, value.unwrap(), type_info, err, &temp_cursor);
            }
        }
        let mut request: api::GoogleCloudRetailV2RemoveLocalInventoriesRequest = json::value::from_value(object).unwrap();
        let mut call = self.hub.projects().locations_catalogs_branches_products_remove_local_inventories(request, opt.value_of("product").unwrap_or(""));
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

    async fn _projects_locations_catalogs_branches_products_set_inventory(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
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
                    "allow-missing" => Some(("allowMissing", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "inventory.audience.age-groups" => Some(("inventory.audience.ageGroups", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Vec })),
                    "inventory.audience.genders" => Some(("inventory.audience.genders", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Vec })),
                    "inventory.availability" => Some(("inventory.availability", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "inventory.available-quantity" => Some(("inventory.availableQuantity", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "inventory.available-time" => Some(("inventory.availableTime", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "inventory.brands" => Some(("inventory.brands", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Vec })),
                    "inventory.categories" => Some(("inventory.categories", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Vec })),
                    "inventory.collection-member-ids" => Some(("inventory.collectionMemberIds", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Vec })),
                    "inventory.color-info.color-families" => Some(("inventory.colorInfo.colorFamilies", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Vec })),
                    "inventory.color-info.colors" => Some(("inventory.colorInfo.colors", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Vec })),
                    "inventory.conditions" => Some(("inventory.conditions", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Vec })),
                    "inventory.description" => Some(("inventory.description", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "inventory.expire-time" => Some(("inventory.expireTime", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "inventory.gtin" => Some(("inventory.gtin", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "inventory.id" => Some(("inventory.id", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "inventory.language-code" => Some(("inventory.languageCode", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "inventory.materials" => Some(("inventory.materials", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Vec })),
                    "inventory.name" => Some(("inventory.name", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "inventory.patterns" => Some(("inventory.patterns", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Vec })),
                    "inventory.price-info.cost" => Some(("inventory.priceInfo.cost", JsonTypeInfo { jtype: JsonType::Float, ctype: ComplexType::Pod })),
                    "inventory.price-info.currency-code" => Some(("inventory.priceInfo.currencyCode", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "inventory.price-info.original-price" => Some(("inventory.priceInfo.originalPrice", JsonTypeInfo { jtype: JsonType::Float, ctype: ComplexType::Pod })),
                    "inventory.price-info.price" => Some(("inventory.priceInfo.price", JsonTypeInfo { jtype: JsonType::Float, ctype: ComplexType::Pod })),
                    "inventory.price-info.price-effective-time" => Some(("inventory.priceInfo.priceEffectiveTime", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "inventory.price-info.price-expire-time" => Some(("inventory.priceInfo.priceExpireTime", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "inventory.price-info.price-range.original-price.exclusive-maximum" => Some(("inventory.priceInfo.priceRange.originalPrice.exclusiveMaximum", JsonTypeInfo { jtype: JsonType::Float, ctype: ComplexType::Pod })),
                    "inventory.price-info.price-range.original-price.exclusive-minimum" => Some(("inventory.priceInfo.priceRange.originalPrice.exclusiveMinimum", JsonTypeInfo { jtype: JsonType::Float, ctype: ComplexType::Pod })),
                    "inventory.price-info.price-range.original-price.maximum" => Some(("inventory.priceInfo.priceRange.originalPrice.maximum", JsonTypeInfo { jtype: JsonType::Float, ctype: ComplexType::Pod })),
                    "inventory.price-info.price-range.original-price.minimum" => Some(("inventory.priceInfo.priceRange.originalPrice.minimum", JsonTypeInfo { jtype: JsonType::Float, ctype: ComplexType::Pod })),
                    "inventory.price-info.price-range.price.exclusive-maximum" => Some(("inventory.priceInfo.priceRange.price.exclusiveMaximum", JsonTypeInfo { jtype: JsonType::Float, ctype: ComplexType::Pod })),
                    "inventory.price-info.price-range.price.exclusive-minimum" => Some(("inventory.priceInfo.priceRange.price.exclusiveMinimum", JsonTypeInfo { jtype: JsonType::Float, ctype: ComplexType::Pod })),
                    "inventory.price-info.price-range.price.maximum" => Some(("inventory.priceInfo.priceRange.price.maximum", JsonTypeInfo { jtype: JsonType::Float, ctype: ComplexType::Pod })),
                    "inventory.price-info.price-range.price.minimum" => Some(("inventory.priceInfo.priceRange.price.minimum", JsonTypeInfo { jtype: JsonType::Float, ctype: ComplexType::Pod })),
                    "inventory.primary-product-id" => Some(("inventory.primaryProductId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "inventory.publish-time" => Some(("inventory.publishTime", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "inventory.rating.average-rating" => Some(("inventory.rating.averageRating", JsonTypeInfo { jtype: JsonType::Float, ctype: ComplexType::Pod })),
                    "inventory.rating.rating-count" => Some(("inventory.rating.ratingCount", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "inventory.rating.rating-histogram" => Some(("inventory.rating.ratingHistogram", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Vec })),
                    "inventory.retrievable-fields" => Some(("inventory.retrievableFields", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "inventory.sizes" => Some(("inventory.sizes", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Vec })),
                    "inventory.tags" => Some(("inventory.tags", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Vec })),
                    "inventory.title" => Some(("inventory.title", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "inventory.ttl" => Some(("inventory.ttl", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "inventory.type" => Some(("inventory.type", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "inventory.uri" => Some(("inventory.uri", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "set-mask" => Some(("setMask", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "set-time" => Some(("setTime", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    _ => {
                        let suggestion = FieldCursor::did_you_mean(key, &vec!["age-groups", "allow-missing", "audience", "availability", "available-quantity", "available-time", "average-rating", "brands", "categories", "collection-member-ids", "color-families", "color-info", "colors", "conditions", "cost", "currency-code", "description", "exclusive-maximum", "exclusive-minimum", "expire-time", "genders", "gtin", "id", "inventory", "language-code", "materials", "maximum", "minimum", "name", "original-price", "patterns", "price", "price-effective-time", "price-expire-time", "price-info", "price-range", "primary-product-id", "publish-time", "rating", "rating-count", "rating-histogram", "retrievable-fields", "set-mask", "set-time", "sizes", "tags", "title", "ttl", "type", "uri"]);
                        err.issues.push(CLIError::Field(FieldError::Unknown(temp_cursor.to_string(), suggestion, value.map(|v| v.to_string()))));
                        None
                    }
                };
            if let Some((field_cursor_str, type_info)) = type_info {
                FieldCursor::from(field_cursor_str).set_json_value(&mut object, value.unwrap(), type_info, err, &temp_cursor);
            }
        }
        let mut request: api::GoogleCloudRetailV2SetInventoryRequest = json::value::from_value(object).unwrap();
        let mut call = self.hub.projects().locations_catalogs_branches_products_set_inventory(request, opt.value_of("name").unwrap_or(""));
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

    async fn _projects_locations_catalogs_complete_query(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        let mut call = self.hub.projects().locations_catalogs_complete_query(opt.value_of("catalog").unwrap_or(""));
        for parg in opt.values_of("v").map(|i|i.collect()).unwrap_or(Vec::new()).iter() {
            let (key, value) = parse_kv_arg(&*parg, err, false);
            match key {
                "visitor-id" => {
                    call = call.visitor_id(value.unwrap_or(""));
                },
                "query" => {
                    call = call.query(value.unwrap_or(""));
                },
                "max-suggestions" => {
                    call = call.max_suggestions(        value.map(|v| arg_from_str(v, err, "max-suggestions", "int32")).unwrap_or(-0));
                },
                "language-codes" => {
                    call = call.add_language_codes(value.unwrap_or(""));
                },
                "entity" => {
                    call = call.entity(value.unwrap_or(""));
                },
                "enable-attribute-suggestions" => {
                    call = call.enable_attribute_suggestions(        value.map(|v| arg_from_str(v, err, "enable-attribute-suggestions", "boolean")).unwrap_or(false));
                },
                "device-type" => {
                    call = call.device_type(value.unwrap_or(""));
                },
                "dataset" => {
                    call = call.dataset(value.unwrap_or(""));
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
                                                                           v.extend(["dataset", "device-type", "enable-attribute-suggestions", "entity", "language-codes", "max-suggestions", "query", "visitor-id"].iter().map(|v|*v));
                                                                           v } ));
                    }
                }
            }
        }
        let protocol = CallType::Standard;
        if dry_run {
            Ok(())
        } else {
            assert!(err.issues.len() == 0);
            for scope in self.opt.values_of("url").map(|i|i.collect()).unwrap_or(Vec::new()).iter() {
                call = call.add_scope(scope);
            }
            let mut ostream = match writer_from_opts(opt.value_of("out")) {
                Ok(mut f) => f,
                Err(io_err) => return Err(DoitError::IoError(opt.value_of("out").unwrap_or("-").to_string(), io_err)),
            };
            match match protocol {
                CallType::Standard => call.doit().await,
                _ => unreachable!()
            } {
                Err(api_err) => Err(DoitError::ApiError(api_err)),
                Ok((mut response, output_schema)) => {
                    let mut value = json::value::to_value(&output_schema).expect("serde to work");
                    remove_json_null_values(&mut value);
                    json::to_writer_pretty(&mut ostream, &value).unwrap();
                    ostream.flush().unwrap();
                    Ok(())
                }
            }
        }
    }

    async fn _projects_locations_catalogs_completion_data_import(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
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
                    "input-config.big-query-source.data-schema" => Some(("inputConfig.bigQuerySource.dataSchema", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "input-config.big-query-source.dataset-id" => Some(("inputConfig.bigQuerySource.datasetId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "input-config.big-query-source.gcs-staging-dir" => Some(("inputConfig.bigQuerySource.gcsStagingDir", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "input-config.big-query-source.partition-date.day" => Some(("inputConfig.bigQuerySource.partitionDate.day", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "input-config.big-query-source.partition-date.month" => Some(("inputConfig.bigQuerySource.partitionDate.month", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "input-config.big-query-source.partition-date.year" => Some(("inputConfig.bigQuerySource.partitionDate.year", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "input-config.big-query-source.project-id" => Some(("inputConfig.bigQuerySource.projectId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "input-config.big-query-source.table-id" => Some(("inputConfig.bigQuerySource.tableId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "notification-pubsub-topic" => Some(("notificationPubsubTopic", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    _ => {
                        let suggestion = FieldCursor::did_you_mean(key, &vec!["big-query-source", "data-schema", "dataset-id", "day", "gcs-staging-dir", "input-config", "month", "notification-pubsub-topic", "partition-date", "project-id", "table-id", "year"]);
                        err.issues.push(CLIError::Field(FieldError::Unknown(temp_cursor.to_string(), suggestion, value.map(|v| v.to_string()))));
                        None
                    }
                };
            if let Some((field_cursor_str, type_info)) = type_info {
                FieldCursor::from(field_cursor_str).set_json_value(&mut object, value.unwrap(), type_info, err, &temp_cursor);
            }
        }
        let mut request: api::GoogleCloudRetailV2ImportCompletionDataRequest = json::value::from_value(object).unwrap();
        let mut call = self.hub.projects().locations_catalogs_completion_data_import(request, opt.value_of("parent").unwrap_or(""));
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

    async fn _projects_locations_catalogs_controls_create(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
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
                    "associated-serving-config-ids" => Some(("associatedServingConfigIds", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Vec })),
                    "display-name" => Some(("displayName", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "name" => Some(("name", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "rule.boost-action.boost" => Some(("rule.boostAction.boost", JsonTypeInfo { jtype: JsonType::Float, ctype: ComplexType::Pod })),
                    "rule.boost-action.products-filter" => Some(("rule.boostAction.productsFilter", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "rule.condition.page-categories" => Some(("rule.condition.pageCategories", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Vec })),
                    "rule.do-not-associate-action.do-not-associate-terms" => Some(("rule.doNotAssociateAction.doNotAssociateTerms", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Vec })),
                    "rule.do-not-associate-action.query-terms" => Some(("rule.doNotAssociateAction.queryTerms", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Vec })),
                    "rule.do-not-associate-action.terms" => Some(("rule.doNotAssociateAction.terms", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Vec })),
                    "rule.filter-action.filter" => Some(("rule.filterAction.filter", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "rule.ignore-action.ignore-terms" => Some(("rule.ignoreAction.ignoreTerms", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Vec })),
                    "rule.oneway-synonyms-action.oneway-terms" => Some(("rule.onewaySynonymsAction.onewayTerms", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Vec })),
                    "rule.oneway-synonyms-action.query-terms" => Some(("rule.onewaySynonymsAction.queryTerms", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Vec })),
                    "rule.oneway-synonyms-action.synonyms" => Some(("rule.onewaySynonymsAction.synonyms", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Vec })),
                    "rule.redirect-action.redirect-uri" => Some(("rule.redirectAction.redirectUri", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "rule.remove-facet-action.attribute-names" => Some(("rule.removeFacetAction.attributeNames", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Vec })),
                    "rule.replacement-action.query-terms" => Some(("rule.replacementAction.queryTerms", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Vec })),
                    "rule.replacement-action.replacement-term" => Some(("rule.replacementAction.replacementTerm", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "rule.replacement-action.term" => Some(("rule.replacementAction.term", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "rule.twoway-synonyms-action.synonyms" => Some(("rule.twowaySynonymsAction.synonyms", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Vec })),
                    "search-solution-use-case" => Some(("searchSolutionUseCase", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Vec })),
                    "solution-types" => Some(("solutionTypes", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Vec })),
                    _ => {
                        let suggestion = FieldCursor::did_you_mean(key, &vec!["associated-serving-config-ids", "attribute-names", "boost", "boost-action", "condition", "display-name", "do-not-associate-action", "do-not-associate-terms", "filter", "filter-action", "ignore-action", "ignore-terms", "name", "oneway-synonyms-action", "oneway-terms", "page-categories", "products-filter", "query-terms", "redirect-action", "redirect-uri", "remove-facet-action", "replacement-action", "replacement-term", "rule", "search-solution-use-case", "solution-types", "synonyms", "term", "terms", "twoway-synonyms-action"]);
                        err.issues.push(CLIError::Field(FieldError::Unknown(temp_cursor.to_string(), suggestion, value.map(|v| v.to_string()))));
                        None
                    }
                };
            if let Some((field_cursor_str, type_info)) = type_info {
                FieldCursor::from(field_cursor_str).set_json_value(&mut object, value.unwrap(), type_info, err, &temp_cursor);
            }
        }
        let mut request: api::GoogleCloudRetailV2Control = json::value::from_value(object).unwrap();
        let mut call = self.hub.projects().locations_catalogs_controls_create(request, opt.value_of("parent").unwrap_or(""));
        for parg in opt.values_of("v").map(|i|i.collect()).unwrap_or(Vec::new()).iter() {
            let (key, value) = parse_kv_arg(&*parg, err, false);
            match key {
                "control-id" => {
                    call = call.control_id(value.unwrap_or(""));
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
                                                                           v.extend(["control-id"].iter().map(|v|*v));
                                                                           v } ));
                    }
                }
            }
        }
        let protocol = CallType::Standard;
        if dry_run {
            Ok(())
        } else {
            assert!(err.issues.len() == 0);
            for scope in self.opt.values_of("url").map(|i|i.collect()).unwrap_or(Vec::new()).iter() {
                call = call.add_scope(scope);
            }
            let mut ostream = match writer_from_opts(opt.value_of("out")) {
                Ok(mut f) => f,
                Err(io_err) => return Err(DoitError::IoError(opt.value_of("out").unwrap_or("-").to_string(), io_err)),
            };
            match match protocol {
                CallType::Standard => call.doit().await,
                _ => unreachable!()
            } {
                Err(api_err) => Err(DoitError::ApiError(api_err)),
                Ok((mut response, output_schema)) => {
                    let mut value = json::value::to_value(&output_schema).expect("serde to work");
                    remove_json_null_values(&mut value);
                    json::to_writer_pretty(&mut ostream, &value).unwrap();
                    ostream.flush().unwrap();
                    Ok(())
                }
            }
        }
    }

    async fn _projects_locations_catalogs_controls_delete(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        let mut call = self.hub.projects().locations_catalogs_controls_delete(opt.value_of("name").unwrap_or(""));
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

    async fn _projects_locations_catalogs_controls_get(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        let mut call = self.hub.projects().locations_catalogs_controls_get(opt.value_of("name").unwrap_or(""));
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

    async fn _projects_locations_catalogs_controls_list(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        let mut call = self.hub.projects().locations_catalogs_controls_list(opt.value_of("parent").unwrap_or(""));
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

    async fn _projects_locations_catalogs_controls_patch(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
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
                    "associated-serving-config-ids" => Some(("associatedServingConfigIds", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Vec })),
                    "display-name" => Some(("displayName", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "name" => Some(("name", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "rule.boost-action.boost" => Some(("rule.boostAction.boost", JsonTypeInfo { jtype: JsonType::Float, ctype: ComplexType::Pod })),
                    "rule.boost-action.products-filter" => Some(("rule.boostAction.productsFilter", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "rule.condition.page-categories" => Some(("rule.condition.pageCategories", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Vec })),
                    "rule.do-not-associate-action.do-not-associate-terms" => Some(("rule.doNotAssociateAction.doNotAssociateTerms", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Vec })),
                    "rule.do-not-associate-action.query-terms" => Some(("rule.doNotAssociateAction.queryTerms", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Vec })),
                    "rule.do-not-associate-action.terms" => Some(("rule.doNotAssociateAction.terms", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Vec })),
                    "rule.filter-action.filter" => Some(("rule.filterAction.filter", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "rule.ignore-action.ignore-terms" => Some(("rule.ignoreAction.ignoreTerms", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Vec })),
                    "rule.oneway-synonyms-action.oneway-terms" => Some(("rule.onewaySynonymsAction.onewayTerms", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Vec })),
                    "rule.oneway-synonyms-action.query-terms" => Some(("rule.onewaySynonymsAction.queryTerms", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Vec })),
                    "rule.oneway-synonyms-action.synonyms" => Some(("rule.onewaySynonymsAction.synonyms", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Vec })),
                    "rule.redirect-action.redirect-uri" => Some(("rule.redirectAction.redirectUri", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "rule.remove-facet-action.attribute-names" => Some(("rule.removeFacetAction.attributeNames", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Vec })),
                    "rule.replacement-action.query-terms" => Some(("rule.replacementAction.queryTerms", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Vec })),
                    "rule.replacement-action.replacement-term" => Some(("rule.replacementAction.replacementTerm", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "rule.replacement-action.term" => Some(("rule.replacementAction.term", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "rule.twoway-synonyms-action.synonyms" => Some(("rule.twowaySynonymsAction.synonyms", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Vec })),
                    "search-solution-use-case" => Some(("searchSolutionUseCase", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Vec })),
                    "solution-types" => Some(("solutionTypes", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Vec })),
                    _ => {
                        let suggestion = FieldCursor::did_you_mean(key, &vec!["associated-serving-config-ids", "attribute-names", "boost", "boost-action", "condition", "display-name", "do-not-associate-action", "do-not-associate-terms", "filter", "filter-action", "ignore-action", "ignore-terms", "name", "oneway-synonyms-action", "oneway-terms", "page-categories", "products-filter", "query-terms", "redirect-action", "redirect-uri", "remove-facet-action", "replacement-action", "replacement-term", "rule", "search-solution-use-case", "solution-types", "synonyms", "term", "terms", "twoway-synonyms-action"]);
                        err.issues.push(CLIError::Field(FieldError::Unknown(temp_cursor.to_string(), suggestion, value.map(|v| v.to_string()))));
                        None
                    }
                };
            if let Some((field_cursor_str, type_info)) = type_info {
                FieldCursor::from(field_cursor_str).set_json_value(&mut object, value.unwrap(), type_info, err, &temp_cursor);
            }
        }
        let mut request: api::GoogleCloudRetailV2Control = json::value::from_value(object).unwrap();
        let mut call = self.hub.projects().locations_catalogs_controls_patch(request, opt.value_of("name").unwrap_or(""));
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

    async fn _projects_locations_catalogs_export_analytics_metrics(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
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
                    "output-config.bigquery-destination.dataset-id" => Some(("outputConfig.bigqueryDestination.datasetId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "output-config.bigquery-destination.table-id-prefix" => Some(("outputConfig.bigqueryDestination.tableIdPrefix", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "output-config.bigquery-destination.table-type" => Some(("outputConfig.bigqueryDestination.tableType", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "output-config.gcs-destination.output-uri-prefix" => Some(("outputConfig.gcsDestination.outputUriPrefix", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    _ => {
                        let suggestion = FieldCursor::did_you_mean(key, &vec!["bigquery-destination", "dataset-id", "filter", "gcs-destination", "output-config", "output-uri-prefix", "table-id-prefix", "table-type"]);
                        err.issues.push(CLIError::Field(FieldError::Unknown(temp_cursor.to_string(), suggestion, value.map(|v| v.to_string()))));
                        None
                    }
                };
            if let Some((field_cursor_str, type_info)) = type_info {
                FieldCursor::from(field_cursor_str).set_json_value(&mut object, value.unwrap(), type_info, err, &temp_cursor);
            }
        }
        let mut request: api::GoogleCloudRetailV2ExportAnalyticsMetricsRequest = json::value::from_value(object).unwrap();
        let mut call = self.hub.projects().locations_catalogs_export_analytics_metrics(request, opt.value_of("catalog").unwrap_or(""));
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

    async fn _projects_locations_catalogs_get_attributes_config(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        let mut call = self.hub.projects().locations_catalogs_get_attributes_config(opt.value_of("name").unwrap_or(""));
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

    async fn _projects_locations_catalogs_get_completion_config(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        let mut call = self.hub.projects().locations_catalogs_get_completion_config(opt.value_of("name").unwrap_or(""));
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

    async fn _projects_locations_catalogs_get_default_branch(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        let mut call = self.hub.projects().locations_catalogs_get_default_branch(opt.value_of("catalog").unwrap_or(""));
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

    async fn _projects_locations_catalogs_list(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        let mut call = self.hub.projects().locations_catalogs_list(opt.value_of("parent").unwrap_or(""));
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

    async fn _projects_locations_catalogs_models_create(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
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
                    "data-state" => Some(("dataState", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "display-name" => Some(("displayName", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "filtering-option" => Some(("filteringOption", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "last-tune-time" => Some(("lastTuneTime", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "model-features-config.frequently-bought-together-config.context-products-type" => Some(("modelFeaturesConfig.frequentlyBoughtTogetherConfig.contextProductsType", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "name" => Some(("name", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "optimization-objective" => Some(("optimizationObjective", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "periodic-tuning-state" => Some(("periodicTuningState", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "serving-state" => Some(("servingState", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "training-state" => Some(("trainingState", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "tuning-operation" => Some(("tuningOperation", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "type" => Some(("type", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "update-time" => Some(("updateTime", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    _ => {
                        let suggestion = FieldCursor::did_you_mean(key, &vec!["context-products-type", "create-time", "data-state", "display-name", "filtering-option", "frequently-bought-together-config", "last-tune-time", "model-features-config", "name", "optimization-objective", "periodic-tuning-state", "serving-state", "training-state", "tuning-operation", "type", "update-time"]);
                        err.issues.push(CLIError::Field(FieldError::Unknown(temp_cursor.to_string(), suggestion, value.map(|v| v.to_string()))));
                        None
                    }
                };
            if let Some((field_cursor_str, type_info)) = type_info {
                FieldCursor::from(field_cursor_str).set_json_value(&mut object, value.unwrap(), type_info, err, &temp_cursor);
            }
        }
        let mut request: api::GoogleCloudRetailV2Model = json::value::from_value(object).unwrap();
        let mut call = self.hub.projects().locations_catalogs_models_create(request, opt.value_of("parent").unwrap_or(""));
        for parg in opt.values_of("v").map(|i|i.collect()).unwrap_or(Vec::new()).iter() {
            let (key, value) = parse_kv_arg(&*parg, err, false);
            match key {
                "dry-run" => {
                    call = call.dry_run(        value.map(|v| arg_from_str(v, err, "dry-run", "boolean")).unwrap_or(false));
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
                                                                           v.extend(["dry-run"].iter().map(|v|*v));
                                                                           v } ));
                    }
                }
            }
        }
        let protocol = CallType::Standard;
        if dry_run {
            Ok(())
        } else {
            assert!(err.issues.len() == 0);
            for scope in self.opt.values_of("url").map(|i|i.collect()).unwrap_or(Vec::new()).iter() {
                call = call.add_scope(scope);
            }
            let mut ostream = match writer_from_opts(opt.value_of("out")) {
                Ok(mut f) => f,
                Err(io_err) => return Err(DoitError::IoError(opt.value_of("out").unwrap_or("-").to_string(), io_err)),
            };
            match match protocol {
                CallType::Standard => call.doit().await,
                _ => unreachable!()
            } {
                Err(api_err) => Err(DoitError::ApiError(api_err)),
                Ok((mut response, output_schema)) => {
                    let mut value = json::value::to_value(&output_schema).expect("serde to work");
                    remove_json_null_values(&mut value);
                    json::to_writer_pretty(&mut ostream, &value).unwrap();
                    ostream.flush().unwrap();
                    Ok(())
                }
            }
        }
    }

    async fn _projects_locations_catalogs_models_delete(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        let mut call = self.hub.projects().locations_catalogs_models_delete(opt.value_of("name").unwrap_or(""));
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

    async fn _projects_locations_catalogs_models_get(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        let mut call = self.hub.projects().locations_catalogs_models_get(opt.value_of("name").unwrap_or(""));
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

    async fn _projects_locations_catalogs_models_list(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        let mut call = self.hub.projects().locations_catalogs_models_list(opt.value_of("parent").unwrap_or(""));
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

    async fn _projects_locations_catalogs_models_patch(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
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
                    "data-state" => Some(("dataState", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "display-name" => Some(("displayName", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "filtering-option" => Some(("filteringOption", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "last-tune-time" => Some(("lastTuneTime", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "model-features-config.frequently-bought-together-config.context-products-type" => Some(("modelFeaturesConfig.frequentlyBoughtTogetherConfig.contextProductsType", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "name" => Some(("name", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "optimization-objective" => Some(("optimizationObjective", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "periodic-tuning-state" => Some(("periodicTuningState", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "serving-state" => Some(("servingState", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "training-state" => Some(("trainingState", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "tuning-operation" => Some(("tuningOperation", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "type" => Some(("type", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "update-time" => Some(("updateTime", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    _ => {
                        let suggestion = FieldCursor::did_you_mean(key, &vec!["context-products-type", "create-time", "data-state", "display-name", "filtering-option", "frequently-bought-together-config", "last-tune-time", "model-features-config", "name", "optimization-objective", "periodic-tuning-state", "serving-state", "training-state", "tuning-operation", "type", "update-time"]);
                        err.issues.push(CLIError::Field(FieldError::Unknown(temp_cursor.to_string(), suggestion, value.map(|v| v.to_string()))));
                        None
                    }
                };
            if let Some((field_cursor_str, type_info)) = type_info {
                FieldCursor::from(field_cursor_str).set_json_value(&mut object, value.unwrap(), type_info, err, &temp_cursor);
            }
        }
        let mut request: api::GoogleCloudRetailV2Model = json::value::from_value(object).unwrap();
        let mut call = self.hub.projects().locations_catalogs_models_patch(request, opt.value_of("name").unwrap_or(""));
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

    async fn _projects_locations_catalogs_models_pause(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
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
        let mut request: api::GoogleCloudRetailV2PauseModelRequest = json::value::from_value(object).unwrap();
        let mut call = self.hub.projects().locations_catalogs_models_pause(request, opt.value_of("name").unwrap_or(""));
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

    async fn _projects_locations_catalogs_models_resume(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
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
        let mut request: api::GoogleCloudRetailV2ResumeModelRequest = json::value::from_value(object).unwrap();
        let mut call = self.hub.projects().locations_catalogs_models_resume(request, opt.value_of("name").unwrap_or(""));
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

    async fn _projects_locations_catalogs_models_tune(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
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
        let mut request: api::GoogleCloudRetailV2TuneModelRequest = json::value::from_value(object).unwrap();
        let mut call = self.hub.projects().locations_catalogs_models_tune(request, opt.value_of("name").unwrap_or(""));
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

    async fn _projects_locations_catalogs_operations_get(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        let mut call = self.hub.projects().locations_catalogs_operations_get(opt.value_of("name").unwrap_or(""));
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

    async fn _projects_locations_catalogs_operations_list(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        let mut call = self.hub.projects().locations_catalogs_operations_list(opt.value_of("name").unwrap_or(""));
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

    async fn _projects_locations_catalogs_patch(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
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
                    "product-level-config.ingestion-product-type" => Some(("productLevelConfig.ingestionProductType", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "product-level-config.merchant-center-product-id-field" => Some(("productLevelConfig.merchantCenterProductIdField", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    _ => {
                        let suggestion = FieldCursor::did_you_mean(key, &vec!["display-name", "ingestion-product-type", "merchant-center-product-id-field", "name", "product-level-config"]);
                        err.issues.push(CLIError::Field(FieldError::Unknown(temp_cursor.to_string(), suggestion, value.map(|v| v.to_string()))));
                        None
                    }
                };
            if let Some((field_cursor_str, type_info)) = type_info {
                FieldCursor::from(field_cursor_str).set_json_value(&mut object, value.unwrap(), type_info, err, &temp_cursor);
            }
        }
        let mut request: api::GoogleCloudRetailV2Catalog = json::value::from_value(object).unwrap();
        let mut call = self.hub.projects().locations_catalogs_patch(request, opt.value_of("name").unwrap_or(""));
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

    async fn _projects_locations_catalogs_placements_predict(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
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
                    "labels" => Some(("labels", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Map })),
                    "page-size" => Some(("pageSize", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "page-token" => Some(("pageToken", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "user-event.attribution-token" => Some(("userEvent.attributionToken", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "user-event.cart-id" => Some(("userEvent.cartId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "user-event.completion-detail.completion-attribution-token" => Some(("userEvent.completionDetail.completionAttributionToken", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "user-event.completion-detail.selected-position" => Some(("userEvent.completionDetail.selectedPosition", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "user-event.completion-detail.selected-suggestion" => Some(("userEvent.completionDetail.selectedSuggestion", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "user-event.entity" => Some(("userEvent.entity", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "user-event.event-time" => Some(("userEvent.eventTime", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "user-event.event-type" => Some(("userEvent.eventType", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "user-event.experiment-ids" => Some(("userEvent.experimentIds", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Vec })),
                    "user-event.filter" => Some(("userEvent.filter", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "user-event.offset" => Some(("userEvent.offset", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "user-event.order-by" => Some(("userEvent.orderBy", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "user-event.page-categories" => Some(("userEvent.pageCategories", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Vec })),
                    "user-event.page-view-id" => Some(("userEvent.pageViewId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "user-event.purchase-transaction.cost" => Some(("userEvent.purchaseTransaction.cost", JsonTypeInfo { jtype: JsonType::Float, ctype: ComplexType::Pod })),
                    "user-event.purchase-transaction.currency-code" => Some(("userEvent.purchaseTransaction.currencyCode", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "user-event.purchase-transaction.id" => Some(("userEvent.purchaseTransaction.id", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "user-event.purchase-transaction.revenue" => Some(("userEvent.purchaseTransaction.revenue", JsonTypeInfo { jtype: JsonType::Float, ctype: ComplexType::Pod })),
                    "user-event.purchase-transaction.tax" => Some(("userEvent.purchaseTransaction.tax", JsonTypeInfo { jtype: JsonType::Float, ctype: ComplexType::Pod })),
                    "user-event.referrer-uri" => Some(("userEvent.referrerUri", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "user-event.search-query" => Some(("userEvent.searchQuery", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "user-event.session-id" => Some(("userEvent.sessionId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "user-event.uri" => Some(("userEvent.uri", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "user-event.user-info.direct-user-request" => Some(("userEvent.userInfo.directUserRequest", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "user-event.user-info.ip-address" => Some(("userEvent.userInfo.ipAddress", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "user-event.user-info.user-agent" => Some(("userEvent.userInfo.userAgent", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "user-event.user-info.user-id" => Some(("userEvent.userInfo.userId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "user-event.visitor-id" => Some(("userEvent.visitorId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "validate-only" => Some(("validateOnly", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    _ => {
                        let suggestion = FieldCursor::did_you_mean(key, &vec!["attribution-token", "cart-id", "completion-attribution-token", "completion-detail", "cost", "currency-code", "direct-user-request", "entity", "event-time", "event-type", "experiment-ids", "filter", "id", "ip-address", "labels", "offset", "order-by", "page-categories", "page-size", "page-token", "page-view-id", "purchase-transaction", "referrer-uri", "revenue", "search-query", "selected-position", "selected-suggestion", "session-id", "tax", "uri", "user-agent", "user-event", "user-id", "user-info", "validate-only", "visitor-id"]);
                        err.issues.push(CLIError::Field(FieldError::Unknown(temp_cursor.to_string(), suggestion, value.map(|v| v.to_string()))));
                        None
                    }
                };
            if let Some((field_cursor_str, type_info)) = type_info {
                FieldCursor::from(field_cursor_str).set_json_value(&mut object, value.unwrap(), type_info, err, &temp_cursor);
            }
        }
        let mut request: api::GoogleCloudRetailV2PredictRequest = json::value::from_value(object).unwrap();
        let mut call = self.hub.projects().locations_catalogs_placements_predict(request, opt.value_of("placement").unwrap_or(""));
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

    async fn _projects_locations_catalogs_placements_search(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
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
                    "boost-spec.skip-boost-spec-validation" => Some(("boostSpec.skipBoostSpecValidation", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "branch" => Some(("branch", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "canonical-filter" => Some(("canonicalFilter", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "dynamic-facet-spec.mode" => Some(("dynamicFacetSpec.mode", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "entity" => Some(("entity", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "filter" => Some(("filter", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "labels" => Some(("labels", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Map })),
                    "offset" => Some(("offset", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "order-by" => Some(("orderBy", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "page-categories" => Some(("pageCategories", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Vec })),
                    "page-size" => Some(("pageSize", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "page-token" => Some(("pageToken", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "personalization-spec.mode" => Some(("personalizationSpec.mode", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "query" => Some(("query", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "query-expansion-spec.condition" => Some(("queryExpansionSpec.condition", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "query-expansion-spec.pin-unexpanded-results" => Some(("queryExpansionSpec.pinUnexpandedResults", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "search-mode" => Some(("searchMode", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "spell-correction-spec.mode" => Some(("spellCorrectionSpec.mode", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "user-info.direct-user-request" => Some(("userInfo.directUserRequest", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "user-info.ip-address" => Some(("userInfo.ipAddress", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "user-info.user-agent" => Some(("userInfo.userAgent", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "user-info.user-id" => Some(("userInfo.userId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "variant-rollup-keys" => Some(("variantRollupKeys", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Vec })),
                    "visitor-id" => Some(("visitorId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    _ => {
                        let suggestion = FieldCursor::did_you_mean(key, &vec!["boost-spec", "branch", "canonical-filter", "condition", "direct-user-request", "dynamic-facet-spec", "entity", "filter", "ip-address", "labels", "mode", "offset", "order-by", "page-categories", "page-size", "page-token", "personalization-spec", "pin-unexpanded-results", "query", "query-expansion-spec", "search-mode", "skip-boost-spec-validation", "spell-correction-spec", "user-agent", "user-id", "user-info", "variant-rollup-keys", "visitor-id"]);
                        err.issues.push(CLIError::Field(FieldError::Unknown(temp_cursor.to_string(), suggestion, value.map(|v| v.to_string()))));
                        None
                    }
                };
            if let Some((field_cursor_str, type_info)) = type_info {
                FieldCursor::from(field_cursor_str).set_json_value(&mut object, value.unwrap(), type_info, err, &temp_cursor);
            }
        }
        let mut request: api::GoogleCloudRetailV2SearchRequest = json::value::from_value(object).unwrap();
        let mut call = self.hub.projects().locations_catalogs_placements_search(request, opt.value_of("placement").unwrap_or(""));
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

    async fn _projects_locations_catalogs_serving_configs_add_control(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
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
                    "control-id" => Some(("controlId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    _ => {
                        let suggestion = FieldCursor::did_you_mean(key, &vec!["control-id"]);
                        err.issues.push(CLIError::Field(FieldError::Unknown(temp_cursor.to_string(), suggestion, value.map(|v| v.to_string()))));
                        None
                    }
                };
            if let Some((field_cursor_str, type_info)) = type_info {
                FieldCursor::from(field_cursor_str).set_json_value(&mut object, value.unwrap(), type_info, err, &temp_cursor);
            }
        }
        let mut request: api::GoogleCloudRetailV2AddControlRequest = json::value::from_value(object).unwrap();
        let mut call = self.hub.projects().locations_catalogs_serving_configs_add_control(request, opt.value_of("serving-config").unwrap_or(""));
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

    async fn _projects_locations_catalogs_serving_configs_create(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
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
                    "boost-control-ids" => Some(("boostControlIds", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Vec })),
                    "display-name" => Some(("displayName", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "diversity-level" => Some(("diversityLevel", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "diversity-type" => Some(("diversityType", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "do-not-associate-control-ids" => Some(("doNotAssociateControlIds", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Vec })),
                    "dynamic-facet-spec.mode" => Some(("dynamicFacetSpec.mode", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "enable-category-filter-level" => Some(("enableCategoryFilterLevel", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "facet-control-ids" => Some(("facetControlIds", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Vec })),
                    "filter-control-ids" => Some(("filterControlIds", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Vec })),
                    "ignore-control-ids" => Some(("ignoreControlIds", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Vec })),
                    "ignore-recs-denylist" => Some(("ignoreRecsDenylist", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "model-id" => Some(("modelId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "name" => Some(("name", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "oneway-synonyms-control-ids" => Some(("onewaySynonymsControlIds", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Vec })),
                    "personalization-spec.mode" => Some(("personalizationSpec.mode", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "price-reranking-level" => Some(("priceRerankingLevel", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "redirect-control-ids" => Some(("redirectControlIds", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Vec })),
                    "replacement-control-ids" => Some(("replacementControlIds", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Vec })),
                    "solution-types" => Some(("solutionTypes", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Vec })),
                    "twoway-synonyms-control-ids" => Some(("twowaySynonymsControlIds", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Vec })),
                    _ => {
                        let suggestion = FieldCursor::did_you_mean(key, &vec!["boost-control-ids", "display-name", "diversity-level", "diversity-type", "do-not-associate-control-ids", "dynamic-facet-spec", "enable-category-filter-level", "facet-control-ids", "filter-control-ids", "ignore-control-ids", "ignore-recs-denylist", "mode", "model-id", "name", "oneway-synonyms-control-ids", "personalization-spec", "price-reranking-level", "redirect-control-ids", "replacement-control-ids", "solution-types", "twoway-synonyms-control-ids"]);
                        err.issues.push(CLIError::Field(FieldError::Unknown(temp_cursor.to_string(), suggestion, value.map(|v| v.to_string()))));
                        None
                    }
                };
            if let Some((field_cursor_str, type_info)) = type_info {
                FieldCursor::from(field_cursor_str).set_json_value(&mut object, value.unwrap(), type_info, err, &temp_cursor);
            }
        }
        let mut request: api::GoogleCloudRetailV2ServingConfig = json::value::from_value(object).unwrap();
        let mut call = self.hub.projects().locations_catalogs_serving_configs_create(request, opt.value_of("parent").unwrap_or(""));
        for parg in opt.values_of("v").map(|i|i.collect()).unwrap_or(Vec::new()).iter() {
            let (key, value) = parse_kv_arg(&*parg, err, false);
            match key {
                "serving-config-id" => {
                    call = call.serving_config_id(value.unwrap_or(""));
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
                                                                           v.extend(["serving-config-id"].iter().map(|v|*v));
                                                                           v } ));
                    }
                }
            }
        }
        let protocol = CallType::Standard;
        if dry_run {
            Ok(())
        } else {
            assert!(err.issues.len() == 0);
            for scope in self.opt.values_of("url").map(|i|i.collect()).unwrap_or(Vec::new()).iter() {
                call = call.add_scope(scope);
            }
            let mut ostream = match writer_from_opts(opt.value_of("out")) {
                Ok(mut f) => f,
                Err(io_err) => return Err(DoitError::IoError(opt.value_of("out").unwrap_or("-").to_string(), io_err)),
            };
            match match protocol {
                CallType::Standard => call.doit().await,
                _ => unreachable!()
            } {
                Err(api_err) => Err(DoitError::ApiError(api_err)),
                Ok((mut response, output_schema)) => {
                    let mut value = json::value::to_value(&output_schema).expect("serde to work");
                    remove_json_null_values(&mut value);
                    json::to_writer_pretty(&mut ostream, &value).unwrap();
                    ostream.flush().unwrap();
                    Ok(())
                }
            }
        }
    }

    async fn _projects_locations_catalogs_serving_configs_delete(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        let mut call = self.hub.projects().locations_catalogs_serving_configs_delete(opt.value_of("name").unwrap_or(""));
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

    async fn _projects_locations_catalogs_serving_configs_get(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        let mut call = self.hub.projects().locations_catalogs_serving_configs_get(opt.value_of("name").unwrap_or(""));
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

    async fn _projects_locations_catalogs_serving_configs_list(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        let mut call = self.hub.projects().locations_catalogs_serving_configs_list(opt.value_of("parent").unwrap_or(""));
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

    async fn _projects_locations_catalogs_serving_configs_patch(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
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
                    "boost-control-ids" => Some(("boostControlIds", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Vec })),
                    "display-name" => Some(("displayName", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "diversity-level" => Some(("diversityLevel", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "diversity-type" => Some(("diversityType", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "do-not-associate-control-ids" => Some(("doNotAssociateControlIds", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Vec })),
                    "dynamic-facet-spec.mode" => Some(("dynamicFacetSpec.mode", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "enable-category-filter-level" => Some(("enableCategoryFilterLevel", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "facet-control-ids" => Some(("facetControlIds", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Vec })),
                    "filter-control-ids" => Some(("filterControlIds", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Vec })),
                    "ignore-control-ids" => Some(("ignoreControlIds", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Vec })),
                    "ignore-recs-denylist" => Some(("ignoreRecsDenylist", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "model-id" => Some(("modelId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "name" => Some(("name", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "oneway-synonyms-control-ids" => Some(("onewaySynonymsControlIds", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Vec })),
                    "personalization-spec.mode" => Some(("personalizationSpec.mode", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "price-reranking-level" => Some(("priceRerankingLevel", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "redirect-control-ids" => Some(("redirectControlIds", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Vec })),
                    "replacement-control-ids" => Some(("replacementControlIds", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Vec })),
                    "solution-types" => Some(("solutionTypes", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Vec })),
                    "twoway-synonyms-control-ids" => Some(("twowaySynonymsControlIds", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Vec })),
                    _ => {
                        let suggestion = FieldCursor::did_you_mean(key, &vec!["boost-control-ids", "display-name", "diversity-level", "diversity-type", "do-not-associate-control-ids", "dynamic-facet-spec", "enable-category-filter-level", "facet-control-ids", "filter-control-ids", "ignore-control-ids", "ignore-recs-denylist", "mode", "model-id", "name", "oneway-synonyms-control-ids", "personalization-spec", "price-reranking-level", "redirect-control-ids", "replacement-control-ids", "solution-types", "twoway-synonyms-control-ids"]);
                        err.issues.push(CLIError::Field(FieldError::Unknown(temp_cursor.to_string(), suggestion, value.map(|v| v.to_string()))));
                        None
                    }
                };
            if let Some((field_cursor_str, type_info)) = type_info {
                FieldCursor::from(field_cursor_str).set_json_value(&mut object, value.unwrap(), type_info, err, &temp_cursor);
            }
        }
        let mut request: api::GoogleCloudRetailV2ServingConfig = json::value::from_value(object).unwrap();
        let mut call = self.hub.projects().locations_catalogs_serving_configs_patch(request, opt.value_of("name").unwrap_or(""));
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

    async fn _projects_locations_catalogs_serving_configs_predict(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
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
                    "labels" => Some(("labels", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Map })),
                    "page-size" => Some(("pageSize", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "page-token" => Some(("pageToken", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "user-event.attribution-token" => Some(("userEvent.attributionToken", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "user-event.cart-id" => Some(("userEvent.cartId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "user-event.completion-detail.completion-attribution-token" => Some(("userEvent.completionDetail.completionAttributionToken", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "user-event.completion-detail.selected-position" => Some(("userEvent.completionDetail.selectedPosition", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "user-event.completion-detail.selected-suggestion" => Some(("userEvent.completionDetail.selectedSuggestion", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "user-event.entity" => Some(("userEvent.entity", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "user-event.event-time" => Some(("userEvent.eventTime", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "user-event.event-type" => Some(("userEvent.eventType", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "user-event.experiment-ids" => Some(("userEvent.experimentIds", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Vec })),
                    "user-event.filter" => Some(("userEvent.filter", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "user-event.offset" => Some(("userEvent.offset", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "user-event.order-by" => Some(("userEvent.orderBy", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "user-event.page-categories" => Some(("userEvent.pageCategories", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Vec })),
                    "user-event.page-view-id" => Some(("userEvent.pageViewId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "user-event.purchase-transaction.cost" => Some(("userEvent.purchaseTransaction.cost", JsonTypeInfo { jtype: JsonType::Float, ctype: ComplexType::Pod })),
                    "user-event.purchase-transaction.currency-code" => Some(("userEvent.purchaseTransaction.currencyCode", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "user-event.purchase-transaction.id" => Some(("userEvent.purchaseTransaction.id", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "user-event.purchase-transaction.revenue" => Some(("userEvent.purchaseTransaction.revenue", JsonTypeInfo { jtype: JsonType::Float, ctype: ComplexType::Pod })),
                    "user-event.purchase-transaction.tax" => Some(("userEvent.purchaseTransaction.tax", JsonTypeInfo { jtype: JsonType::Float, ctype: ComplexType::Pod })),
                    "user-event.referrer-uri" => Some(("userEvent.referrerUri", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "user-event.search-query" => Some(("userEvent.searchQuery", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "user-event.session-id" => Some(("userEvent.sessionId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "user-event.uri" => Some(("userEvent.uri", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "user-event.user-info.direct-user-request" => Some(("userEvent.userInfo.directUserRequest", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "user-event.user-info.ip-address" => Some(("userEvent.userInfo.ipAddress", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "user-event.user-info.user-agent" => Some(("userEvent.userInfo.userAgent", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "user-event.user-info.user-id" => Some(("userEvent.userInfo.userId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "user-event.visitor-id" => Some(("userEvent.visitorId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "validate-only" => Some(("validateOnly", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    _ => {
                        let suggestion = FieldCursor::did_you_mean(key, &vec!["attribution-token", "cart-id", "completion-attribution-token", "completion-detail", "cost", "currency-code", "direct-user-request", "entity", "event-time", "event-type", "experiment-ids", "filter", "id", "ip-address", "labels", "offset", "order-by", "page-categories", "page-size", "page-token", "page-view-id", "purchase-transaction", "referrer-uri", "revenue", "search-query", "selected-position", "selected-suggestion", "session-id", "tax", "uri", "user-agent", "user-event", "user-id", "user-info", "validate-only", "visitor-id"]);
                        err.issues.push(CLIError::Field(FieldError::Unknown(temp_cursor.to_string(), suggestion, value.map(|v| v.to_string()))));
                        None
                    }
                };
            if let Some((field_cursor_str, type_info)) = type_info {
                FieldCursor::from(field_cursor_str).set_json_value(&mut object, value.unwrap(), type_info, err, &temp_cursor);
            }
        }
        let mut request: api::GoogleCloudRetailV2PredictRequest = json::value::from_value(object).unwrap();
        let mut call = self.hub.projects().locations_catalogs_serving_configs_predict(request, opt.value_of("placement").unwrap_or(""));
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

    async fn _projects_locations_catalogs_serving_configs_remove_control(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
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
                    "control-id" => Some(("controlId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    _ => {
                        let suggestion = FieldCursor::did_you_mean(key, &vec!["control-id"]);
                        err.issues.push(CLIError::Field(FieldError::Unknown(temp_cursor.to_string(), suggestion, value.map(|v| v.to_string()))));
                        None
                    }
                };
            if let Some((field_cursor_str, type_info)) = type_info {
                FieldCursor::from(field_cursor_str).set_json_value(&mut object, value.unwrap(), type_info, err, &temp_cursor);
            }
        }
        let mut request: api::GoogleCloudRetailV2RemoveControlRequest = json::value::from_value(object).unwrap();
        let mut call = self.hub.projects().locations_catalogs_serving_configs_remove_control(request, opt.value_of("serving-config").unwrap_or(""));
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

    async fn _projects_locations_catalogs_serving_configs_search(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
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
                    "boost-spec.skip-boost-spec-validation" => Some(("boostSpec.skipBoostSpecValidation", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "branch" => Some(("branch", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "canonical-filter" => Some(("canonicalFilter", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "dynamic-facet-spec.mode" => Some(("dynamicFacetSpec.mode", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "entity" => Some(("entity", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "filter" => Some(("filter", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "labels" => Some(("labels", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Map })),
                    "offset" => Some(("offset", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "order-by" => Some(("orderBy", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "page-categories" => Some(("pageCategories", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Vec })),
                    "page-size" => Some(("pageSize", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "page-token" => Some(("pageToken", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "personalization-spec.mode" => Some(("personalizationSpec.mode", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "query" => Some(("query", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "query-expansion-spec.condition" => Some(("queryExpansionSpec.condition", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "query-expansion-spec.pin-unexpanded-results" => Some(("queryExpansionSpec.pinUnexpandedResults", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "search-mode" => Some(("searchMode", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "spell-correction-spec.mode" => Some(("spellCorrectionSpec.mode", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "user-info.direct-user-request" => Some(("userInfo.directUserRequest", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "user-info.ip-address" => Some(("userInfo.ipAddress", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "user-info.user-agent" => Some(("userInfo.userAgent", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "user-info.user-id" => Some(("userInfo.userId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "variant-rollup-keys" => Some(("variantRollupKeys", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Vec })),
                    "visitor-id" => Some(("visitorId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    _ => {
                        let suggestion = FieldCursor::did_you_mean(key, &vec!["boost-spec", "branch", "canonical-filter", "condition", "direct-user-request", "dynamic-facet-spec", "entity", "filter", "ip-address", "labels", "mode", "offset", "order-by", "page-categories", "page-size", "page-token", "personalization-spec", "pin-unexpanded-results", "query", "query-expansion-spec", "search-mode", "skip-boost-spec-validation", "spell-correction-spec", "user-agent", "user-id", "user-info", "variant-rollup-keys", "visitor-id"]);
                        err.issues.push(CLIError::Field(FieldError::Unknown(temp_cursor.to_string(), suggestion, value.map(|v| v.to_string()))));
                        None
                    }
                };
            if let Some((field_cursor_str, type_info)) = type_info {
                FieldCursor::from(field_cursor_str).set_json_value(&mut object, value.unwrap(), type_info, err, &temp_cursor);
            }
        }
        let mut request: api::GoogleCloudRetailV2SearchRequest = json::value::from_value(object).unwrap();
        let mut call = self.hub.projects().locations_catalogs_serving_configs_search(request, opt.value_of("placement").unwrap_or(""));
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

    async fn _projects_locations_catalogs_set_default_branch(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
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
                    "branch-id" => Some(("branchId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "force" => Some(("force", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "note" => Some(("note", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    _ => {
                        let suggestion = FieldCursor::did_you_mean(key, &vec!["branch-id", "force", "note"]);
                        err.issues.push(CLIError::Field(FieldError::Unknown(temp_cursor.to_string(), suggestion, value.map(|v| v.to_string()))));
                        None
                    }
                };
            if let Some((field_cursor_str, type_info)) = type_info {
                FieldCursor::from(field_cursor_str).set_json_value(&mut object, value.unwrap(), type_info, err, &temp_cursor);
            }
        }
        let mut request: api::GoogleCloudRetailV2SetDefaultBranchRequest = json::value::from_value(object).unwrap();
        let mut call = self.hub.projects().locations_catalogs_set_default_branch(request, opt.value_of("catalog").unwrap_or(""));
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

    async fn _projects_locations_catalogs_update_attributes_config(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
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
                    "attribute-config-level" => Some(("attributeConfigLevel", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "name" => Some(("name", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    _ => {
                        let suggestion = FieldCursor::did_you_mean(key, &vec!["attribute-config-level", "name"]);
                        err.issues.push(CLIError::Field(FieldError::Unknown(temp_cursor.to_string(), suggestion, value.map(|v| v.to_string()))));
                        None
                    }
                };
            if let Some((field_cursor_str, type_info)) = type_info {
                FieldCursor::from(field_cursor_str).set_json_value(&mut object, value.unwrap(), type_info, err, &temp_cursor);
            }
        }
        let mut request: api::GoogleCloudRetailV2AttributesConfig = json::value::from_value(object).unwrap();
        let mut call = self.hub.projects().locations_catalogs_update_attributes_config(request, opt.value_of("name").unwrap_or(""));
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

    async fn _projects_locations_catalogs_update_completion_config(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
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
                    "allowlist-input-config.big-query-source.data-schema" => Some(("allowlistInputConfig.bigQuerySource.dataSchema", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "allowlist-input-config.big-query-source.dataset-id" => Some(("allowlistInputConfig.bigQuerySource.datasetId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "allowlist-input-config.big-query-source.gcs-staging-dir" => Some(("allowlistInputConfig.bigQuerySource.gcsStagingDir", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "allowlist-input-config.big-query-source.partition-date.day" => Some(("allowlistInputConfig.bigQuerySource.partitionDate.day", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "allowlist-input-config.big-query-source.partition-date.month" => Some(("allowlistInputConfig.bigQuerySource.partitionDate.month", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "allowlist-input-config.big-query-source.partition-date.year" => Some(("allowlistInputConfig.bigQuerySource.partitionDate.year", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "allowlist-input-config.big-query-source.project-id" => Some(("allowlistInputConfig.bigQuerySource.projectId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "allowlist-input-config.big-query-source.table-id" => Some(("allowlistInputConfig.bigQuerySource.tableId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "auto-learning" => Some(("autoLearning", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "denylist-input-config.big-query-source.data-schema" => Some(("denylistInputConfig.bigQuerySource.dataSchema", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "denylist-input-config.big-query-source.dataset-id" => Some(("denylistInputConfig.bigQuerySource.datasetId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "denylist-input-config.big-query-source.gcs-staging-dir" => Some(("denylistInputConfig.bigQuerySource.gcsStagingDir", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "denylist-input-config.big-query-source.partition-date.day" => Some(("denylistInputConfig.bigQuerySource.partitionDate.day", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "denylist-input-config.big-query-source.partition-date.month" => Some(("denylistInputConfig.bigQuerySource.partitionDate.month", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "denylist-input-config.big-query-source.partition-date.year" => Some(("denylistInputConfig.bigQuerySource.partitionDate.year", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "denylist-input-config.big-query-source.project-id" => Some(("denylistInputConfig.bigQuerySource.projectId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "denylist-input-config.big-query-source.table-id" => Some(("denylistInputConfig.bigQuerySource.tableId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "last-allowlist-import-operation" => Some(("lastAllowlistImportOperation", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "last-denylist-import-operation" => Some(("lastDenylistImportOperation", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "last-suggestions-import-operation" => Some(("lastSuggestionsImportOperation", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "matching-order" => Some(("matchingOrder", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "max-suggestions" => Some(("maxSuggestions", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "min-prefix-length" => Some(("minPrefixLength", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "name" => Some(("name", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "suggestions-input-config.big-query-source.data-schema" => Some(("suggestionsInputConfig.bigQuerySource.dataSchema", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "suggestions-input-config.big-query-source.dataset-id" => Some(("suggestionsInputConfig.bigQuerySource.datasetId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "suggestions-input-config.big-query-source.gcs-staging-dir" => Some(("suggestionsInputConfig.bigQuerySource.gcsStagingDir", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "suggestions-input-config.big-query-source.partition-date.day" => Some(("suggestionsInputConfig.bigQuerySource.partitionDate.day", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "suggestions-input-config.big-query-source.partition-date.month" => Some(("suggestionsInputConfig.bigQuerySource.partitionDate.month", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "suggestions-input-config.big-query-source.partition-date.year" => Some(("suggestionsInputConfig.bigQuerySource.partitionDate.year", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "suggestions-input-config.big-query-source.project-id" => Some(("suggestionsInputConfig.bigQuerySource.projectId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "suggestions-input-config.big-query-source.table-id" => Some(("suggestionsInputConfig.bigQuerySource.tableId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    _ => {
                        let suggestion = FieldCursor::did_you_mean(key, &vec!["allowlist-input-config", "auto-learning", "big-query-source", "data-schema", "dataset-id", "day", "denylist-input-config", "gcs-staging-dir", "last-allowlist-import-operation", "last-denylist-import-operation", "last-suggestions-import-operation", "matching-order", "max-suggestions", "min-prefix-length", "month", "name", "partition-date", "project-id", "suggestions-input-config", "table-id", "year"]);
                        err.issues.push(CLIError::Field(FieldError::Unknown(temp_cursor.to_string(), suggestion, value.map(|v| v.to_string()))));
                        None
                    }
                };
            if let Some((field_cursor_str, type_info)) = type_info {
                FieldCursor::from(field_cursor_str).set_json_value(&mut object, value.unwrap(), type_info, err, &temp_cursor);
            }
        }
        let mut request: api::GoogleCloudRetailV2CompletionConfig = json::value::from_value(object).unwrap();
        let mut call = self.hub.projects().locations_catalogs_update_completion_config(request, opt.value_of("name").unwrap_or(""));
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

    async fn _projects_locations_catalogs_user_events_collect(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        let mut call = self.hub.projects().locations_catalogs_user_events_collect(opt.value_of("parent").unwrap_or(""));
        for parg in opt.values_of("v").map(|i|i.collect()).unwrap_or(Vec::new()).iter() {
            let (key, value) = parse_kv_arg(&*parg, err, false);
            match key {
                "user-event" => {
                    call = call.user_event(value.unwrap_or(""));
                },
                "uri" => {
                    call = call.uri(value.unwrap_or(""));
                },
                "raw-json" => {
                    call = call.raw_json(value.unwrap_or(""));
                },
                "prebuilt-rule" => {
                    call = call.prebuilt_rule(value.unwrap_or(""));
                },
                "ets" => {
                    call = call.ets(        value.map(|v| arg_from_str(v, err, "ets", "int64")).unwrap_or(-0));
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
                                                                           v.extend(["ets", "prebuilt-rule", "raw-json", "uri", "user-event"].iter().map(|v|*v));
                                                                           v } ));
                    }
                }
            }
        }
        let protocol = CallType::Standard;
        if dry_run {
            Ok(())
        } else {
            assert!(err.issues.len() == 0);
            for scope in self.opt.values_of("url").map(|i|i.collect()).unwrap_or(Vec::new()).iter() {
                call = call.add_scope(scope);
            }
            let mut ostream = match writer_from_opts(opt.value_of("out")) {
                Ok(mut f) => f,
                Err(io_err) => return Err(DoitError::IoError(opt.value_of("out").unwrap_or("-").to_string(), io_err)),
            };
            match match protocol {
                CallType::Standard => call.doit().await,
                _ => unreachable!()
            } {
                Err(api_err) => Err(DoitError::ApiError(api_err)),
                Ok((mut response, output_schema)) => {
                    let mut value = json::value::to_value(&output_schema).expect("serde to work");
                    remove_json_null_values(&mut value);
                    json::to_writer_pretty(&mut ostream, &value).unwrap();
                    ostream.flush().unwrap();
                    Ok(())
                }
            }
        }
    }

    async fn _projects_locations_catalogs_user_events_import(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
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
                    "errors-config.gcs-prefix" => Some(("errorsConfig.gcsPrefix", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "input-config.big-query-source.data-schema" => Some(("inputConfig.bigQuerySource.dataSchema", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "input-config.big-query-source.dataset-id" => Some(("inputConfig.bigQuerySource.datasetId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "input-config.big-query-source.gcs-staging-dir" => Some(("inputConfig.bigQuerySource.gcsStagingDir", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "input-config.big-query-source.partition-date.day" => Some(("inputConfig.bigQuerySource.partitionDate.day", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "input-config.big-query-source.partition-date.month" => Some(("inputConfig.bigQuerySource.partitionDate.month", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "input-config.big-query-source.partition-date.year" => Some(("inputConfig.bigQuerySource.partitionDate.year", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "input-config.big-query-source.project-id" => Some(("inputConfig.bigQuerySource.projectId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "input-config.big-query-source.table-id" => Some(("inputConfig.bigQuerySource.tableId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "input-config.gcs-source.data-schema" => Some(("inputConfig.gcsSource.dataSchema", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "input-config.gcs-source.input-uris" => Some(("inputConfig.gcsSource.inputUris", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Vec })),
                    _ => {
                        let suggestion = FieldCursor::did_you_mean(key, &vec!["big-query-source", "data-schema", "dataset-id", "day", "errors-config", "gcs-prefix", "gcs-source", "gcs-staging-dir", "input-config", "input-uris", "month", "partition-date", "project-id", "table-id", "year"]);
                        err.issues.push(CLIError::Field(FieldError::Unknown(temp_cursor.to_string(), suggestion, value.map(|v| v.to_string()))));
                        None
                    }
                };
            if let Some((field_cursor_str, type_info)) = type_info {
                FieldCursor::from(field_cursor_str).set_json_value(&mut object, value.unwrap(), type_info, err, &temp_cursor);
            }
        }
        let mut request: api::GoogleCloudRetailV2ImportUserEventsRequest = json::value::from_value(object).unwrap();
        let mut call = self.hub.projects().locations_catalogs_user_events_import(request, opt.value_of("parent").unwrap_or(""));
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

    async fn _projects_locations_catalogs_user_events_purge(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
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
                    _ => {
                        let suggestion = FieldCursor::did_you_mean(key, &vec!["filter", "force"]);
                        err.issues.push(CLIError::Field(FieldError::Unknown(temp_cursor.to_string(), suggestion, value.map(|v| v.to_string()))));
                        None
                    }
                };
            if let Some((field_cursor_str, type_info)) = type_info {
                FieldCursor::from(field_cursor_str).set_json_value(&mut object, value.unwrap(), type_info, err, &temp_cursor);
            }
        }
        let mut request: api::GoogleCloudRetailV2PurgeUserEventsRequest = json::value::from_value(object).unwrap();
        let mut call = self.hub.projects().locations_catalogs_user_events_purge(request, opt.value_of("parent").unwrap_or(""));
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

    async fn _projects_locations_catalogs_user_events_rejoin(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
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
                    "user-event-rejoin-scope" => Some(("userEventRejoinScope", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    _ => {
                        let suggestion = FieldCursor::did_you_mean(key, &vec!["user-event-rejoin-scope"]);
                        err.issues.push(CLIError::Field(FieldError::Unknown(temp_cursor.to_string(), suggestion, value.map(|v| v.to_string()))));
                        None
                    }
                };
            if let Some((field_cursor_str, type_info)) = type_info {
                FieldCursor::from(field_cursor_str).set_json_value(&mut object, value.unwrap(), type_info, err, &temp_cursor);
            }
        }
        let mut request: api::GoogleCloudRetailV2RejoinUserEventsRequest = json::value::from_value(object).unwrap();
        let mut call = self.hub.projects().locations_catalogs_user_events_rejoin(request, opt.value_of("parent").unwrap_or(""));
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

    async fn _projects_locations_catalogs_user_events_write(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
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
                    "attribution-token" => Some(("attributionToken", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "cart-id" => Some(("cartId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "completion-detail.completion-attribution-token" => Some(("completionDetail.completionAttributionToken", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "completion-detail.selected-position" => Some(("completionDetail.selectedPosition", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "completion-detail.selected-suggestion" => Some(("completionDetail.selectedSuggestion", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "entity" => Some(("entity", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "event-time" => Some(("eventTime", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "event-type" => Some(("eventType", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "experiment-ids" => Some(("experimentIds", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Vec })),
                    "filter" => Some(("filter", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "offset" => Some(("offset", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "order-by" => Some(("orderBy", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "page-categories" => Some(("pageCategories", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Vec })),
                    "page-view-id" => Some(("pageViewId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "purchase-transaction.cost" => Some(("purchaseTransaction.cost", JsonTypeInfo { jtype: JsonType::Float, ctype: ComplexType::Pod })),
                    "purchase-transaction.currency-code" => Some(("purchaseTransaction.currencyCode", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "purchase-transaction.id" => Some(("purchaseTransaction.id", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "purchase-transaction.revenue" => Some(("purchaseTransaction.revenue", JsonTypeInfo { jtype: JsonType::Float, ctype: ComplexType::Pod })),
                    "purchase-transaction.tax" => Some(("purchaseTransaction.tax", JsonTypeInfo { jtype: JsonType::Float, ctype: ComplexType::Pod })),
                    "referrer-uri" => Some(("referrerUri", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "search-query" => Some(("searchQuery", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "session-id" => Some(("sessionId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "uri" => Some(("uri", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "user-info.direct-user-request" => Some(("userInfo.directUserRequest", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "user-info.ip-address" => Some(("userInfo.ipAddress", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "user-info.user-agent" => Some(("userInfo.userAgent", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "user-info.user-id" => Some(("userInfo.userId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "visitor-id" => Some(("visitorId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    _ => {
                        let suggestion = FieldCursor::did_you_mean(key, &vec!["attribution-token", "cart-id", "completion-attribution-token", "completion-detail", "cost", "currency-code", "direct-user-request", "entity", "event-time", "event-type", "experiment-ids", "filter", "id", "ip-address", "offset", "order-by", "page-categories", "page-view-id", "purchase-transaction", "referrer-uri", "revenue", "search-query", "selected-position", "selected-suggestion", "session-id", "tax", "uri", "user-agent", "user-id", "user-info", "visitor-id"]);
                        err.issues.push(CLIError::Field(FieldError::Unknown(temp_cursor.to_string(), suggestion, value.map(|v| v.to_string()))));
                        None
                    }
                };
            if let Some((field_cursor_str, type_info)) = type_info {
                FieldCursor::from(field_cursor_str).set_json_value(&mut object, value.unwrap(), type_info, err, &temp_cursor);
            }
        }
        let mut request: api::GoogleCloudRetailV2UserEvent = json::value::from_value(object).unwrap();
        let mut call = self.hub.projects().locations_catalogs_user_events_write(request, opt.value_of("parent").unwrap_or(""));
        for parg in opt.values_of("v").map(|i|i.collect()).unwrap_or(Vec::new()).iter() {
            let (key, value) = parse_kv_arg(&*parg, err, false);
            match key {
                "write-async" => {
                    call = call.write_async(        value.map(|v| arg_from_str(v, err, "write-async", "boolean")).unwrap_or(false));
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
                                                                           v.extend(["write-async"].iter().map(|v|*v));
                                                                           v } ));
                    }
                }
            }
        }
        let protocol = CallType::Standard;
        if dry_run {
            Ok(())
        } else {
            assert!(err.issues.len() == 0);
            for scope in self.opt.values_of("url").map(|i|i.collect()).unwrap_or(Vec::new()).iter() {
                call = call.add_scope(scope);
            }
            let mut ostream = match writer_from_opts(opt.value_of("out")) {
                Ok(mut f) => f,
                Err(io_err) => return Err(DoitError::IoError(opt.value_of("out").unwrap_or("-").to_string(), io_err)),
            };
            match match protocol {
                CallType::Standard => call.doit().await,
                _ => unreachable!()
            } {
                Err(api_err) => Err(DoitError::ApiError(api_err)),
                Ok((mut response, output_schema)) => {
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
                    ("locations-catalogs-attributes-config-add-catalog-attribute", Some(opt)) => {
                        call_result = self._projects_locations_catalogs_attributes_config_add_catalog_attribute(opt, dry_run, &mut err).await;
                    },
                    ("locations-catalogs-attributes-config-remove-catalog-attribute", Some(opt)) => {
                        call_result = self._projects_locations_catalogs_attributes_config_remove_catalog_attribute(opt, dry_run, &mut err).await;
                    },
                    ("locations-catalogs-attributes-config-replace-catalog-attribute", Some(opt)) => {
                        call_result = self._projects_locations_catalogs_attributes_config_replace_catalog_attribute(opt, dry_run, &mut err).await;
                    },
                    ("locations-catalogs-branches-operations-get", Some(opt)) => {
                        call_result = self._projects_locations_catalogs_branches_operations_get(opt, dry_run, &mut err).await;
                    },
                    ("locations-catalogs-branches-products-add-fulfillment-places", Some(opt)) => {
                        call_result = self._projects_locations_catalogs_branches_products_add_fulfillment_places(opt, dry_run, &mut err).await;
                    },
                    ("locations-catalogs-branches-products-add-local-inventories", Some(opt)) => {
                        call_result = self._projects_locations_catalogs_branches_products_add_local_inventories(opt, dry_run, &mut err).await;
                    },
                    ("locations-catalogs-branches-products-create", Some(opt)) => {
                        call_result = self._projects_locations_catalogs_branches_products_create(opt, dry_run, &mut err).await;
                    },
                    ("locations-catalogs-branches-products-delete", Some(opt)) => {
                        call_result = self._projects_locations_catalogs_branches_products_delete(opt, dry_run, &mut err).await;
                    },
                    ("locations-catalogs-branches-products-get", Some(opt)) => {
                        call_result = self._projects_locations_catalogs_branches_products_get(opt, dry_run, &mut err).await;
                    },
                    ("locations-catalogs-branches-products-import", Some(opt)) => {
                        call_result = self._projects_locations_catalogs_branches_products_import(opt, dry_run, &mut err).await;
                    },
                    ("locations-catalogs-branches-products-list", Some(opt)) => {
                        call_result = self._projects_locations_catalogs_branches_products_list(opt, dry_run, &mut err).await;
                    },
                    ("locations-catalogs-branches-products-patch", Some(opt)) => {
                        call_result = self._projects_locations_catalogs_branches_products_patch(opt, dry_run, &mut err).await;
                    },
                    ("locations-catalogs-branches-products-purge", Some(opt)) => {
                        call_result = self._projects_locations_catalogs_branches_products_purge(opt, dry_run, &mut err).await;
                    },
                    ("locations-catalogs-branches-products-remove-fulfillment-places", Some(opt)) => {
                        call_result = self._projects_locations_catalogs_branches_products_remove_fulfillment_places(opt, dry_run, &mut err).await;
                    },
                    ("locations-catalogs-branches-products-remove-local-inventories", Some(opt)) => {
                        call_result = self._projects_locations_catalogs_branches_products_remove_local_inventories(opt, dry_run, &mut err).await;
                    },
                    ("locations-catalogs-branches-products-set-inventory", Some(opt)) => {
                        call_result = self._projects_locations_catalogs_branches_products_set_inventory(opt, dry_run, &mut err).await;
                    },
                    ("locations-catalogs-complete-query", Some(opt)) => {
                        call_result = self._projects_locations_catalogs_complete_query(opt, dry_run, &mut err).await;
                    },
                    ("locations-catalogs-completion-data-import", Some(opt)) => {
                        call_result = self._projects_locations_catalogs_completion_data_import(opt, dry_run, &mut err).await;
                    },
                    ("locations-catalogs-controls-create", Some(opt)) => {
                        call_result = self._projects_locations_catalogs_controls_create(opt, dry_run, &mut err).await;
                    },
                    ("locations-catalogs-controls-delete", Some(opt)) => {
                        call_result = self._projects_locations_catalogs_controls_delete(opt, dry_run, &mut err).await;
                    },
                    ("locations-catalogs-controls-get", Some(opt)) => {
                        call_result = self._projects_locations_catalogs_controls_get(opt, dry_run, &mut err).await;
                    },
                    ("locations-catalogs-controls-list", Some(opt)) => {
                        call_result = self._projects_locations_catalogs_controls_list(opt, dry_run, &mut err).await;
                    },
                    ("locations-catalogs-controls-patch", Some(opt)) => {
                        call_result = self._projects_locations_catalogs_controls_patch(opt, dry_run, &mut err).await;
                    },
                    ("locations-catalogs-export-analytics-metrics", Some(opt)) => {
                        call_result = self._projects_locations_catalogs_export_analytics_metrics(opt, dry_run, &mut err).await;
                    },
                    ("locations-catalogs-get-attributes-config", Some(opt)) => {
                        call_result = self._projects_locations_catalogs_get_attributes_config(opt, dry_run, &mut err).await;
                    },
                    ("locations-catalogs-get-completion-config", Some(opt)) => {
                        call_result = self._projects_locations_catalogs_get_completion_config(opt, dry_run, &mut err).await;
                    },
                    ("locations-catalogs-get-default-branch", Some(opt)) => {
                        call_result = self._projects_locations_catalogs_get_default_branch(opt, dry_run, &mut err).await;
                    },
                    ("locations-catalogs-list", Some(opt)) => {
                        call_result = self._projects_locations_catalogs_list(opt, dry_run, &mut err).await;
                    },
                    ("locations-catalogs-models-create", Some(opt)) => {
                        call_result = self._projects_locations_catalogs_models_create(opt, dry_run, &mut err).await;
                    },
                    ("locations-catalogs-models-delete", Some(opt)) => {
                        call_result = self._projects_locations_catalogs_models_delete(opt, dry_run, &mut err).await;
                    },
                    ("locations-catalogs-models-get", Some(opt)) => {
                        call_result = self._projects_locations_catalogs_models_get(opt, dry_run, &mut err).await;
                    },
                    ("locations-catalogs-models-list", Some(opt)) => {
                        call_result = self._projects_locations_catalogs_models_list(opt, dry_run, &mut err).await;
                    },
                    ("locations-catalogs-models-patch", Some(opt)) => {
                        call_result = self._projects_locations_catalogs_models_patch(opt, dry_run, &mut err).await;
                    },
                    ("locations-catalogs-models-pause", Some(opt)) => {
                        call_result = self._projects_locations_catalogs_models_pause(opt, dry_run, &mut err).await;
                    },
                    ("locations-catalogs-models-resume", Some(opt)) => {
                        call_result = self._projects_locations_catalogs_models_resume(opt, dry_run, &mut err).await;
                    },
                    ("locations-catalogs-models-tune", Some(opt)) => {
                        call_result = self._projects_locations_catalogs_models_tune(opt, dry_run, &mut err).await;
                    },
                    ("locations-catalogs-operations-get", Some(opt)) => {
                        call_result = self._projects_locations_catalogs_operations_get(opt, dry_run, &mut err).await;
                    },
                    ("locations-catalogs-operations-list", Some(opt)) => {
                        call_result = self._projects_locations_catalogs_operations_list(opt, dry_run, &mut err).await;
                    },
                    ("locations-catalogs-patch", Some(opt)) => {
                        call_result = self._projects_locations_catalogs_patch(opt, dry_run, &mut err).await;
                    },
                    ("locations-catalogs-placements-predict", Some(opt)) => {
                        call_result = self._projects_locations_catalogs_placements_predict(opt, dry_run, &mut err).await;
                    },
                    ("locations-catalogs-placements-search", Some(opt)) => {
                        call_result = self._projects_locations_catalogs_placements_search(opt, dry_run, &mut err).await;
                    },
                    ("locations-catalogs-serving-configs-add-control", Some(opt)) => {
                        call_result = self._projects_locations_catalogs_serving_configs_add_control(opt, dry_run, &mut err).await;
                    },
                    ("locations-catalogs-serving-configs-create", Some(opt)) => {
                        call_result = self._projects_locations_catalogs_serving_configs_create(opt, dry_run, &mut err).await;
                    },
                    ("locations-catalogs-serving-configs-delete", Some(opt)) => {
                        call_result = self._projects_locations_catalogs_serving_configs_delete(opt, dry_run, &mut err).await;
                    },
                    ("locations-catalogs-serving-configs-get", Some(opt)) => {
                        call_result = self._projects_locations_catalogs_serving_configs_get(opt, dry_run, &mut err).await;
                    },
                    ("locations-catalogs-serving-configs-list", Some(opt)) => {
                        call_result = self._projects_locations_catalogs_serving_configs_list(opt, dry_run, &mut err).await;
                    },
                    ("locations-catalogs-serving-configs-patch", Some(opt)) => {
                        call_result = self._projects_locations_catalogs_serving_configs_patch(opt, dry_run, &mut err).await;
                    },
                    ("locations-catalogs-serving-configs-predict", Some(opt)) => {
                        call_result = self._projects_locations_catalogs_serving_configs_predict(opt, dry_run, &mut err).await;
                    },
                    ("locations-catalogs-serving-configs-remove-control", Some(opt)) => {
                        call_result = self._projects_locations_catalogs_serving_configs_remove_control(opt, dry_run, &mut err).await;
                    },
                    ("locations-catalogs-serving-configs-search", Some(opt)) => {
                        call_result = self._projects_locations_catalogs_serving_configs_search(opt, dry_run, &mut err).await;
                    },
                    ("locations-catalogs-set-default-branch", Some(opt)) => {
                        call_result = self._projects_locations_catalogs_set_default_branch(opt, dry_run, &mut err).await;
                    },
                    ("locations-catalogs-update-attributes-config", Some(opt)) => {
                        call_result = self._projects_locations_catalogs_update_attributes_config(opt, dry_run, &mut err).await;
                    },
                    ("locations-catalogs-update-completion-config", Some(opt)) => {
                        call_result = self._projects_locations_catalogs_update_completion_config(opt, dry_run, &mut err).await;
                    },
                    ("locations-catalogs-user-events-collect", Some(opt)) => {
                        call_result = self._projects_locations_catalogs_user_events_collect(opt, dry_run, &mut err).await;
                    },
                    ("locations-catalogs-user-events-import", Some(opt)) => {
                        call_result = self._projects_locations_catalogs_user_events_import(opt, dry_run, &mut err).await;
                    },
                    ("locations-catalogs-user-events-purge", Some(opt)) => {
                        call_result = self._projects_locations_catalogs_user_events_purge(opt, dry_run, &mut err).await;
                    },
                    ("locations-catalogs-user-events-rejoin", Some(opt)) => {
                        call_result = self._projects_locations_catalogs_user_events_rejoin(opt, dry_run, &mut err).await;
                    },
                    ("locations-catalogs-user-events-write", Some(opt)) => {
                        call_result = self._projects_locations_catalogs_user_events_write(opt, dry_run, &mut err).await;
                    },
                    ("locations-operations-get", Some(opt)) => {
                        call_result = self._projects_locations_operations_get(opt, dry_run, &mut err).await;
                    },
                    ("locations-operations-list", Some(opt)) => {
                        call_result = self._projects_locations_operations_list(opt, dry_run, &mut err).await;
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

            match client::application_secret_from_directory(&config_dir, "retail2-secret.json",
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
        ).persist_tokens_to_disk(format!("{}/retail2", config_dir)).build().await.unwrap();

        let engine = Engine {
            opt: opt,
            hub: api::CloudRetail::new(client, auth),
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
        ("projects", "methods: 'locations-catalogs-attributes-config-add-catalog-attribute', 'locations-catalogs-attributes-config-remove-catalog-attribute', 'locations-catalogs-attributes-config-replace-catalog-attribute', 'locations-catalogs-branches-operations-get', 'locations-catalogs-branches-products-add-fulfillment-places', 'locations-catalogs-branches-products-add-local-inventories', 'locations-catalogs-branches-products-create', 'locations-catalogs-branches-products-delete', 'locations-catalogs-branches-products-get', 'locations-catalogs-branches-products-import', 'locations-catalogs-branches-products-list', 'locations-catalogs-branches-products-patch', 'locations-catalogs-branches-products-purge', 'locations-catalogs-branches-products-remove-fulfillment-places', 'locations-catalogs-branches-products-remove-local-inventories', 'locations-catalogs-branches-products-set-inventory', 'locations-catalogs-complete-query', 'locations-catalogs-completion-data-import', 'locations-catalogs-controls-create', 'locations-catalogs-controls-delete', 'locations-catalogs-controls-get', 'locations-catalogs-controls-list', 'locations-catalogs-controls-patch', 'locations-catalogs-export-analytics-metrics', 'locations-catalogs-get-attributes-config', 'locations-catalogs-get-completion-config', 'locations-catalogs-get-default-branch', 'locations-catalogs-list', 'locations-catalogs-models-create', 'locations-catalogs-models-delete', 'locations-catalogs-models-get', 'locations-catalogs-models-list', 'locations-catalogs-models-patch', 'locations-catalogs-models-pause', 'locations-catalogs-models-resume', 'locations-catalogs-models-tune', 'locations-catalogs-operations-get', 'locations-catalogs-operations-list', 'locations-catalogs-patch', 'locations-catalogs-placements-predict', 'locations-catalogs-placements-search', 'locations-catalogs-serving-configs-add-control', 'locations-catalogs-serving-configs-create', 'locations-catalogs-serving-configs-delete', 'locations-catalogs-serving-configs-get', 'locations-catalogs-serving-configs-list', 'locations-catalogs-serving-configs-patch', 'locations-catalogs-serving-configs-predict', 'locations-catalogs-serving-configs-remove-control', 'locations-catalogs-serving-configs-search', 'locations-catalogs-set-default-branch', 'locations-catalogs-update-attributes-config', 'locations-catalogs-update-completion-config', 'locations-catalogs-user-events-collect', 'locations-catalogs-user-events-import', 'locations-catalogs-user-events-purge', 'locations-catalogs-user-events-rejoin', 'locations-catalogs-user-events-write', 'locations-operations-get', 'locations-operations-list', 'operations-get' and 'operations-list'", vec![
            ("locations-catalogs-attributes-config-add-catalog-attribute",
                    Some(r##"Adds the specified CatalogAttribute to the AttributesConfig. If the CatalogAttribute to add already exists, an ALREADY_EXISTS error is returned."##),
                    "Details at http://byron.github.io/google-apis-rs/google_retail2_cli/projects_locations-catalogs-attributes-config-add-catalog-attribute",
                  vec![
                    (Some(r##"attributes-config"##),
                     None,
                     Some(r##"Required. Full AttributesConfig resource name. Format: `projects/{project_number}/locations/{location_id}/catalogs/{catalog_id}/attributesConfig`"##),
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
            ("locations-catalogs-attributes-config-remove-catalog-attribute",
                    Some(r##"Removes the specified CatalogAttribute from the AttributesConfig. If the CatalogAttribute to remove does not exist, a NOT_FOUND error is returned."##),
                    "Details at http://byron.github.io/google-apis-rs/google_retail2_cli/projects_locations-catalogs-attributes-config-remove-catalog-attribute",
                  vec![
                    (Some(r##"attributes-config"##),
                     None,
                     Some(r##"Required. Full AttributesConfig resource name. Format: `projects/{project_number}/locations/{location_id}/catalogs/{catalog_id}/attributesConfig`"##),
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
            ("locations-catalogs-attributes-config-replace-catalog-attribute",
                    Some(r##"Replaces the specified CatalogAttribute in the AttributesConfig by updating the catalog attribute with the same CatalogAttribute.key. If the CatalogAttribute to replace does not exist, a NOT_FOUND error is returned."##),
                    "Details at http://byron.github.io/google-apis-rs/google_retail2_cli/projects_locations-catalogs-attributes-config-replace-catalog-attribute",
                  vec![
                    (Some(r##"attributes-config"##),
                     None,
                     Some(r##"Required. Full AttributesConfig resource name. Format: `projects/{project_number}/locations/{location_id}/catalogs/{catalog_id}/attributesConfig`"##),
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
            ("locations-catalogs-branches-operations-get",
                    Some(r##"Gets the latest state of a long-running operation. Clients can use this method to poll the operation result at intervals as recommended by the API service."##),
                    "Details at http://byron.github.io/google-apis-rs/google_retail2_cli/projects_locations-catalogs-branches-operations-get",
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
            ("locations-catalogs-branches-products-add-fulfillment-places",
                    Some(r##"We recommend that you use the ProductService.AddLocalInventories method instead of the ProductService.AddFulfillmentPlaces method. ProductService.AddLocalInventories achieves the same results but provides more fine-grained control over ingesting local inventory data. Incrementally adds place IDs to Product.fulfillment_info.place_ids. This process is asynchronous and does not require the Product to exist before updating fulfillment information. If the request is valid, the update will be enqueued and processed downstream. As a consequence, when a response is returned, the added place IDs are not immediately manifested in the Product queried by ProductService.GetProduct or ProductService.ListProducts. The returned Operations will be obsolete after 1 day, and GetOperation API will return NOT_FOUND afterwards. If conflicting updates are issued, the Operations associated with the stale updates will not be marked as done until being obsolete."##),
                    "Details at http://byron.github.io/google-apis-rs/google_retail2_cli/projects_locations-catalogs-branches-products-add-fulfillment-places",
                  vec![
                    (Some(r##"product"##),
                     None,
                     Some(r##"Required. Full resource name of Product, such as `projects/*/locations/global/catalogs/default_catalog/branches/default_branch/products/some_product_id`. If the caller does not have permission to access the Product, regardless of whether or not it exists, a PERMISSION_DENIED error is returned."##),
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
            ("locations-catalogs-branches-products-add-local-inventories",
                    Some(r##"Updates local inventory information for a Product at a list of places, while respecting the last update timestamps of each inventory field. This process is asynchronous and does not require the Product to exist before updating inventory information. If the request is valid, the update will be enqueued and processed downstream. As a consequence, when a response is returned, updates are not immediately manifested in the Product queried by ProductService.GetProduct or ProductService.ListProducts. Local inventory information can only be modified using this method. ProductService.CreateProduct and ProductService.UpdateProduct has no effect on local inventories. The returned Operations will be obsolete after 1 day, and GetOperation API will return NOT_FOUND afterwards. If conflicting updates are issued, the Operations associated with the stale updates will not be marked as done until being obsolete."##),
                    "Details at http://byron.github.io/google-apis-rs/google_retail2_cli/projects_locations-catalogs-branches-products-add-local-inventories",
                  vec![
                    (Some(r##"product"##),
                     None,
                     Some(r##"Required. Full resource name of Product, such as `projects/*/locations/global/catalogs/default_catalog/branches/default_branch/products/some_product_id`. If the caller does not have permission to access the Product, regardless of whether or not it exists, a PERMISSION_DENIED error is returned."##),
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
            ("locations-catalogs-branches-products-create",
                    Some(r##"Creates a Product."##),
                    "Details at http://byron.github.io/google-apis-rs/google_retail2_cli/projects_locations-catalogs-branches-products-create",
                  vec![
                    (Some(r##"parent"##),
                     None,
                     Some(r##"Required. The parent catalog resource name, such as `projects/*/locations/global/catalogs/default_catalog/branches/default_branch`."##),
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
            ("locations-catalogs-branches-products-delete",
                    Some(r##"Deletes a Product."##),
                    "Details at http://byron.github.io/google-apis-rs/google_retail2_cli/projects_locations-catalogs-branches-products-delete",
                  vec![
                    (Some(r##"name"##),
                     None,
                     Some(r##"Required. Full resource name of Product, such as `projects/*/locations/global/catalogs/default_catalog/branches/default_branch/products/some_product_id`. If the caller does not have permission to delete the Product, regardless of whether or not it exists, a PERMISSION_DENIED error is returned. If the Product to delete does not exist, a NOT_FOUND error is returned. The Product to delete can neither be a Product.Type.COLLECTION Product member nor a Product.Type.PRIMARY Product with more than one variants. Otherwise, an INVALID_ARGUMENT error is returned. All inventory information for the named Product will be deleted."##),
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
            ("locations-catalogs-branches-products-get",
                    Some(r##"Gets a Product."##),
                    "Details at http://byron.github.io/google-apis-rs/google_retail2_cli/projects_locations-catalogs-branches-products-get",
                  vec![
                    (Some(r##"name"##),
                     None,
                     Some(r##"Required. Full resource name of Product, such as `projects/*/locations/global/catalogs/default_catalog/branches/default_branch/products/some_product_id`. If the caller does not have permission to access the Product, regardless of whether or not it exists, a PERMISSION_DENIED error is returned. If the requested Product does not exist, a NOT_FOUND error is returned."##),
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
            ("locations-catalogs-branches-products-import",
                    Some(r##"Bulk import of multiple Products. Request processing may be synchronous. Non-existing items are created. Note that it is possible for a subset of the Products to be successfully updated."##),
                    "Details at http://byron.github.io/google-apis-rs/google_retail2_cli/projects_locations-catalogs-branches-products-import",
                  vec![
                    (Some(r##"parent"##),
                     None,
                     Some(r##"Required. `projects/1234/locations/global/catalogs/default_catalog/branches/default_branch` If no updateMask is specified, requires products.create permission. If updateMask is specified, requires products.update permission."##),
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
            ("locations-catalogs-branches-products-list",
                    Some(r##"Gets a list of Products."##),
                    "Details at http://byron.github.io/google-apis-rs/google_retail2_cli/projects_locations-catalogs-branches-products-list",
                  vec![
                    (Some(r##"parent"##),
                     None,
                     Some(r##"Required. The parent branch resource name, such as `projects/*/locations/global/catalogs/default_catalog/branches/0`. Use `default_branch` as the branch ID, to list products under the default branch. If the caller does not have permission to list Products under this branch, regardless of whether or not this branch exists, a PERMISSION_DENIED error is returned."##),
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
            ("locations-catalogs-branches-products-patch",
                    Some(r##"Updates a Product."##),
                    "Details at http://byron.github.io/google-apis-rs/google_retail2_cli/projects_locations-catalogs-branches-products-patch",
                  vec![
                    (Some(r##"name"##),
                     None,
                     Some(r##"Immutable. Full resource name of the product, such as `projects/*/locations/global/catalogs/default_catalog/branches/default_branch/products/product_id`."##),
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
            ("locations-catalogs-branches-products-purge",
                    Some(r##"Permanently deletes all selected Products under a branch. This process is asynchronous. If the request is valid, the removal will be enqueued and processed offline. Depending on the number of Products, this operation could take hours to complete. Before the operation completes, some Products may still be returned by ProductService.GetProduct or ProductService.ListProducts. Depending on the number of Products, this operation could take hours to complete. To get a sample of Products that would be deleted, set PurgeProductsRequest.force to false."##),
                    "Details at http://byron.github.io/google-apis-rs/google_retail2_cli/projects_locations-catalogs-branches-products-purge",
                  vec![
                    (Some(r##"parent"##),
                     None,
                     Some(r##"Required. The resource name of the branch under which the products are created. The format is `projects/${projectId}/locations/global/catalogs/${catalogId}/branches/${branchId}`"##),
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
            ("locations-catalogs-branches-products-remove-fulfillment-places",
                    Some(r##"We recommend that you use the ProductService.RemoveLocalInventories method instead of the ProductService.RemoveFulfillmentPlaces method. ProductService.RemoveLocalInventories achieves the same results but provides more fine-grained control over ingesting local inventory data. Incrementally removes place IDs from a Product.fulfillment_info.place_ids. This process is asynchronous and does not require the Product to exist before updating fulfillment information. If the request is valid, the update will be enqueued and processed downstream. As a consequence, when a response is returned, the removed place IDs are not immediately manifested in the Product queried by ProductService.GetProduct or ProductService.ListProducts. The returned Operations will be obsolete after 1 day, and GetOperation API will return NOT_FOUND afterwards. If conflicting updates are issued, the Operations associated with the stale updates will not be marked as done until being obsolete."##),
                    "Details at http://byron.github.io/google-apis-rs/google_retail2_cli/projects_locations-catalogs-branches-products-remove-fulfillment-places",
                  vec![
                    (Some(r##"product"##),
                     None,
                     Some(r##"Required. Full resource name of Product, such as `projects/*/locations/global/catalogs/default_catalog/branches/default_branch/products/some_product_id`. If the caller does not have permission to access the Product, regardless of whether or not it exists, a PERMISSION_DENIED error is returned."##),
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
            ("locations-catalogs-branches-products-remove-local-inventories",
                    Some(r##"Remove local inventory information for a Product at a list of places at a removal timestamp. This process is asynchronous. If the request is valid, the removal will be enqueued and processed downstream. As a consequence, when a response is returned, removals are not immediately manifested in the Product queried by ProductService.GetProduct or ProductService.ListProducts. Local inventory information can only be removed using this method. ProductService.CreateProduct and ProductService.UpdateProduct has no effect on local inventories. The returned Operations will be obsolete after 1 day, and GetOperation API will return NOT_FOUND afterwards. If conflicting updates are issued, the Operations associated with the stale updates will not be marked as done until being obsolete."##),
                    "Details at http://byron.github.io/google-apis-rs/google_retail2_cli/projects_locations-catalogs-branches-products-remove-local-inventories",
                  vec![
                    (Some(r##"product"##),
                     None,
                     Some(r##"Required. Full resource name of Product, such as `projects/*/locations/global/catalogs/default_catalog/branches/default_branch/products/some_product_id`. If the caller does not have permission to access the Product, regardless of whether or not it exists, a PERMISSION_DENIED error is returned."##),
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
            ("locations-catalogs-branches-products-set-inventory",
                    Some(r##"Updates inventory information for a Product while respecting the last update timestamps of each inventory field. This process is asynchronous and does not require the Product to exist before updating fulfillment information. If the request is valid, the update is enqueued and processed downstream. As a consequence, when a response is returned, updates are not immediately manifested in the Product queried by ProductService.GetProduct or ProductService.ListProducts. When inventory is updated with ProductService.CreateProduct and ProductService.UpdateProduct, the specified inventory field value(s) overwrite any existing value(s) while ignoring the last update time for this field. Furthermore, the last update times for the specified inventory fields are overwritten by the times of the ProductService.CreateProduct or ProductService.UpdateProduct request. If no inventory fields are set in CreateProductRequest.product, then any pre-existing inventory information for this product is used. If no inventory fields are set in SetInventoryRequest.set_mask, then any existing inventory information is preserved. Pre-existing inventory information can only be updated with ProductService.SetInventory, ProductService.AddFulfillmentPlaces, and ProductService.RemoveFulfillmentPlaces. The returned Operations is obsolete after one day, and the GetOperation API returns `NOT_FOUND` afterwards. If conflicting updates are issued, the Operations associated with the stale updates are not marked as done until they are obsolete."##),
                    "Details at http://byron.github.io/google-apis-rs/google_retail2_cli/projects_locations-catalogs-branches-products-set-inventory",
                  vec![
                    (Some(r##"name"##),
                     None,
                     Some(r##"Immutable. Full resource name of the product, such as `projects/*/locations/global/catalogs/default_catalog/branches/default_branch/products/product_id`."##),
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
            ("locations-catalogs-complete-query",
                    Some(r##"Completes the specified prefix with keyword suggestions. This feature is only available for users who have Retail Search enabled. Enable Retail Search on Cloud Console before using this feature."##),
                    "Details at http://byron.github.io/google-apis-rs/google_retail2_cli/projects_locations-catalogs-complete-query",
                  vec![
                    (Some(r##"catalog"##),
                     None,
                     Some(r##"Required. Catalog for which the completion is performed. Full resource name of catalog, such as `projects/*/locations/global/catalogs/default_catalog`."##),
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
            ("locations-catalogs-completion-data-import",
                    Some(r##"Bulk import of processed completion dataset. Request processing is asynchronous. Partial updating is not supported. The operation is successfully finished only after the imported suggestions are indexed successfully and ready for serving. The process takes hours. This feature is only available for users who have Retail Search enabled. Enable Retail Search on Cloud Console before using this feature."##),
                    "Details at http://byron.github.io/google-apis-rs/google_retail2_cli/projects_locations-catalogs-completion-data-import",
                  vec![
                    (Some(r##"parent"##),
                     None,
                     Some(r##"Required. The catalog which the suggestions dataset belongs to. Format: `projects/1234/locations/global/catalogs/default_catalog`."##),
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
            ("locations-catalogs-controls-create",
                    Some(r##"Creates a Control. If the Control to create already exists, an ALREADY_EXISTS error is returned."##),
                    "Details at http://byron.github.io/google-apis-rs/google_retail2_cli/projects_locations-catalogs-controls-create",
                  vec![
                    (Some(r##"parent"##),
                     None,
                     Some(r##"Required. Full resource name of parent catalog. Format: `projects/{project_number}/locations/{location_id}/catalogs/{catalog_id}`"##),
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
            ("locations-catalogs-controls-delete",
                    Some(r##"Deletes a Control. If the Control to delete does not exist, a NOT_FOUND error is returned."##),
                    "Details at http://byron.github.io/google-apis-rs/google_retail2_cli/projects_locations-catalogs-controls-delete",
                  vec![
                    (Some(r##"name"##),
                     None,
                     Some(r##"Required. The resource name of the Control to delete. Format: `projects/{project_number}/locations/{location_id}/catalogs/{catalog_id}/controls/{control_id}`"##),
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
            ("locations-catalogs-controls-get",
                    Some(r##"Gets a Control."##),
                    "Details at http://byron.github.io/google-apis-rs/google_retail2_cli/projects_locations-catalogs-controls-get",
                  vec![
                    (Some(r##"name"##),
                     None,
                     Some(r##"Required. The resource name of the Control to get. Format: `projects/{project_number}/locations/{location_id}/catalogs/{catalog_id}/controls/{control_id}`"##),
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
            ("locations-catalogs-controls-list",
                    Some(r##"Lists all Controls by their parent Catalog."##),
                    "Details at http://byron.github.io/google-apis-rs/google_retail2_cli/projects_locations-catalogs-controls-list",
                  vec![
                    (Some(r##"parent"##),
                     None,
                     Some(r##"Required. The catalog resource name. Format: `projects/{project_number}/locations/{location_id}/catalogs/{catalog_id}`"##),
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
            ("locations-catalogs-controls-patch",
                    Some(r##"Updates a Control. Control cannot be set to a different oneof field, if so an INVALID_ARGUMENT is returned. If the Control to update does not exist, a NOT_FOUND error is returned."##),
                    "Details at http://byron.github.io/google-apis-rs/google_retail2_cli/projects_locations-catalogs-controls-patch",
                  vec![
                    (Some(r##"name"##),
                     None,
                     Some(r##"Immutable. Fully qualified name `projects/*/locations/global/catalogs/*/controls/*`"##),
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
            ("locations-catalogs-export-analytics-metrics",
                    Some(r##"Exports analytics metrics. `Operation.response` is of type `ExportAnalyticsMetricsResponse`. `Operation.metadata` is of type `ExportMetadata`."##),
                    "Details at http://byron.github.io/google-apis-rs/google_retail2_cli/projects_locations-catalogs-export-analytics-metrics",
                  vec![
                    (Some(r##"catalog"##),
                     None,
                     Some(r##"Required. Full resource name of the parent catalog. Expected format: `projects/*/locations/*/catalogs/*`"##),
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
            ("locations-catalogs-get-attributes-config",
                    Some(r##"Gets an AttributesConfig."##),
                    "Details at http://byron.github.io/google-apis-rs/google_retail2_cli/projects_locations-catalogs-get-attributes-config",
                  vec![
                    (Some(r##"name"##),
                     None,
                     Some(r##"Required. Full AttributesConfig resource name. Format: `projects/{project_number}/locations/{location_id}/catalogs/{catalog_id}/attributesConfig`"##),
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
            ("locations-catalogs-get-completion-config",
                    Some(r##"Gets a CompletionConfig."##),
                    "Details at http://byron.github.io/google-apis-rs/google_retail2_cli/projects_locations-catalogs-get-completion-config",
                  vec![
                    (Some(r##"name"##),
                     None,
                     Some(r##"Required. Full CompletionConfig resource name. Format: `projects/{project_number}/locations/{location_id}/catalogs/{catalog_id}/completionConfig`"##),
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
            ("locations-catalogs-get-default-branch",
                    Some(r##"Get which branch is currently default branch set by CatalogService.SetDefaultBranch method under a specified parent catalog."##),
                    "Details at http://byron.github.io/google-apis-rs/google_retail2_cli/projects_locations-catalogs-get-default-branch",
                  vec![
                    (Some(r##"catalog"##),
                     None,
                     Some(r##"The parent catalog resource name, such as `projects/*/locations/global/catalogs/default_catalog`."##),
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
            ("locations-catalogs-list",
                    Some(r##"Lists all the Catalogs associated with the project."##),
                    "Details at http://byron.github.io/google-apis-rs/google_retail2_cli/projects_locations-catalogs-list",
                  vec![
                    (Some(r##"parent"##),
                     None,
                     Some(r##"Required. The account resource name with an associated location. If the caller does not have permission to list Catalogs under this location, regardless of whether or not this location exists, a PERMISSION_DENIED error is returned."##),
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
            ("locations-catalogs-models-create",
                    Some(r##"Creates a new model."##),
                    "Details at http://byron.github.io/google-apis-rs/google_retail2_cli/projects_locations-catalogs-models-create",
                  vec![
                    (Some(r##"parent"##),
                     None,
                     Some(r##"Required. The parent resource under which to create the model. Format: `projects/{project_number}/locations/{location_id}/catalogs/{catalog_id}`"##),
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
            ("locations-catalogs-models-delete",
                    Some(r##"Deletes an existing model."##),
                    "Details at http://byron.github.io/google-apis-rs/google_retail2_cli/projects_locations-catalogs-models-delete",
                  vec![
                    (Some(r##"name"##),
                     None,
                     Some(r##"Required. The resource name of the Model to delete. Format: `projects/{project_number}/locations/{location_id}/catalogs/{catalog_id}/models/{model_id}`"##),
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
            ("locations-catalogs-models-get",
                    Some(r##"Gets a model."##),
                    "Details at http://byron.github.io/google-apis-rs/google_retail2_cli/projects_locations-catalogs-models-get",
                  vec![
                    (Some(r##"name"##),
                     None,
                     Some(r##"Required. The resource name of the Model to get. Format: `projects/{project_number}/locations/{location_id}/catalogs/{catalog}/models/{model_id}`"##),
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
            ("locations-catalogs-models-list",
                    Some(r##"Lists all the models linked to this event store."##),
                    "Details at http://byron.github.io/google-apis-rs/google_retail2_cli/projects_locations-catalogs-models-list",
                  vec![
                    (Some(r##"parent"##),
                     None,
                     Some(r##"Required. The parent for which to list models. Format: `projects/{project_number}/locations/{location_id}/catalogs/{catalog_id}`"##),
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
            ("locations-catalogs-models-patch",
                    Some(r##"Update of model metadata. Only fields that currently can be updated are: `filtering_option` and `periodic_tuning_state`. If other values are provided, this API method ignores them."##),
                    "Details at http://byron.github.io/google-apis-rs/google_retail2_cli/projects_locations-catalogs-models-patch",
                  vec![
                    (Some(r##"name"##),
                     None,
                     Some(r##"Required. The fully qualified resource name of the model. Format: `projects/{project_number}/locations/{location_id}/catalogs/{catalog_id}/models/{model_id}` catalog_id has char limit of 50. recommendation_model_id has char limit of 40."##),
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
            ("locations-catalogs-models-pause",
                    Some(r##"Pauses the training of an existing model."##),
                    "Details at http://byron.github.io/google-apis-rs/google_retail2_cli/projects_locations-catalogs-models-pause",
                  vec![
                    (Some(r##"name"##),
                     None,
                     Some(r##"Required. The name of the model to pause. Format: `projects/{project_number}/locations/{location_id}/catalogs/{catalog_id}/models/{model_id}`"##),
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
            ("locations-catalogs-models-resume",
                    Some(r##"Resumes the training of an existing model."##),
                    "Details at http://byron.github.io/google-apis-rs/google_retail2_cli/projects_locations-catalogs-models-resume",
                  vec![
                    (Some(r##"name"##),
                     None,
                     Some(r##"Required. The name of the model to resume. Format: `projects/{project_number}/locations/{location_id}/catalogs/{catalog_id}/models/{model_id}`"##),
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
            ("locations-catalogs-models-tune",
                    Some(r##"Tunes an existing model."##),
                    "Details at http://byron.github.io/google-apis-rs/google_retail2_cli/projects_locations-catalogs-models-tune",
                  vec![
                    (Some(r##"name"##),
                     None,
                     Some(r##"Required. The resource name of the model to tune. Format: `projects/{project_number}/locations/{location_id}/catalogs/{catalog_id}/models/{model_id}`"##),
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
            ("locations-catalogs-operations-get",
                    Some(r##"Gets the latest state of a long-running operation. Clients can use this method to poll the operation result at intervals as recommended by the API service."##),
                    "Details at http://byron.github.io/google-apis-rs/google_retail2_cli/projects_locations-catalogs-operations-get",
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
            ("locations-catalogs-operations-list",
                    Some(r##"Lists operations that match the specified filter in the request. If the server doesn't support this method, it returns `UNIMPLEMENTED`."##),
                    "Details at http://byron.github.io/google-apis-rs/google_retail2_cli/projects_locations-catalogs-operations-list",
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
            ("locations-catalogs-patch",
                    Some(r##"Updates the Catalogs."##),
                    "Details at http://byron.github.io/google-apis-rs/google_retail2_cli/projects_locations-catalogs-patch",
                  vec![
                    (Some(r##"name"##),
                     None,
                     Some(r##"Required. Immutable. The fully qualified resource name of the catalog."##),
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
            ("locations-catalogs-placements-predict",
                    Some(r##"Makes a recommendation prediction."##),
                    "Details at http://byron.github.io/google-apis-rs/google_retail2_cli/projects_locations-catalogs-placements-predict",
                  vec![
                    (Some(r##"placement"##),
                     None,
                     Some(r##"Required. Full resource name of the format: `{placement=projects/*/locations/global/catalogs/default_catalog/servingConfigs/*}` or `{placement=projects/*/locations/global/catalogs/default_catalog/placements/*}`. We recommend using the `servingConfigs` resource. `placements` is a legacy resource. The ID of the Recommendations AI serving config or placement. Before you can request predictions from your model, you must create at least one serving config or placement for it. For more information, see [Manage serving configs] (https://cloud.google.com/retail/docs/manage-configs). The full list of available serving configs can be seen at https://console.cloud.google.com/ai/retail/catalogs/default_catalog/configs"##),
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
            ("locations-catalogs-placements-search",
                    Some(r##"Performs a search. This feature is only available for users who have Retail Search enabled. Enable Retail Search on Cloud Console before using this feature."##),
                    "Details at http://byron.github.io/google-apis-rs/google_retail2_cli/projects_locations-catalogs-placements-search",
                  vec![
                    (Some(r##"placement"##),
                     None,
                     Some(r##"Required. The resource name of the Retail Search serving config, such as `projects/*/locations/global/catalogs/default_catalog/servingConfigs/default_serving_config` or the name of the legacy placement resource, such as `projects/*/locations/global/catalogs/default_catalog/placements/default_search`. This field is used to identify the serving config name and the set of models that are used to make the search."##),
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
            ("locations-catalogs-serving-configs-add-control",
                    Some(r##"Enables a Control on the specified ServingConfig. The control is added in the last position of the list of controls it belongs to (e.g. if it's a facet spec control it will be applied in the last position of servingConfig.facetSpecIds) Returns a ALREADY_EXISTS error if the control has already been applied. Returns a FAILED_PRECONDITION error if the addition could exceed maximum number of control allowed for that type of control."##),
                    "Details at http://byron.github.io/google-apis-rs/google_retail2_cli/projects_locations-catalogs-serving-configs-add-control",
                  vec![
                    (Some(r##"serving-config"##),
                     None,
                     Some(r##"Required. The source ServingConfig resource name . Format: `projects/{project_number}/locations/{location_id}/catalogs/{catalog_id}/servingConfigs/{serving_config_id}`"##),
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
            ("locations-catalogs-serving-configs-create",
                    Some(r##"Creates a ServingConfig. A maximum of 100 ServingConfigs are allowed in a Catalog, otherwise a FAILED_PRECONDITION error is returned."##),
                    "Details at http://byron.github.io/google-apis-rs/google_retail2_cli/projects_locations-catalogs-serving-configs-create",
                  vec![
                    (Some(r##"parent"##),
                     None,
                     Some(r##"Required. Full resource name of parent. Format: `projects/{project_number}/locations/{location_id}/catalogs/{catalog_id}`"##),
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
            ("locations-catalogs-serving-configs-delete",
                    Some(r##"Deletes a ServingConfig. Returns a NotFound error if the ServingConfig does not exist."##),
                    "Details at http://byron.github.io/google-apis-rs/google_retail2_cli/projects_locations-catalogs-serving-configs-delete",
                  vec![
                    (Some(r##"name"##),
                     None,
                     Some(r##"Required. The resource name of the ServingConfig to delete. Format: `projects/{project_number}/locations/{location_id}/catalogs/{catalog_id}/servingConfigs/{serving_config_id}`"##),
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
            ("locations-catalogs-serving-configs-get",
                    Some(r##"Gets a ServingConfig. Returns a NotFound error if the ServingConfig does not exist."##),
                    "Details at http://byron.github.io/google-apis-rs/google_retail2_cli/projects_locations-catalogs-serving-configs-get",
                  vec![
                    (Some(r##"name"##),
                     None,
                     Some(r##"Required. The resource name of the ServingConfig to get. Format: `projects/{project_number}/locations/{location_id}/catalogs/{catalog_id}/servingConfigs/{serving_config_id}`"##),
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
            ("locations-catalogs-serving-configs-list",
                    Some(r##"Lists all ServingConfigs linked to this catalog."##),
                    "Details at http://byron.github.io/google-apis-rs/google_retail2_cli/projects_locations-catalogs-serving-configs-list",
                  vec![
                    (Some(r##"parent"##),
                     None,
                     Some(r##"Required. The catalog resource name. Format: `projects/{project_number}/locations/{location_id}/catalogs/{catalog_id}`"##),
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
            ("locations-catalogs-serving-configs-patch",
                    Some(r##"Updates a ServingConfig."##),
                    "Details at http://byron.github.io/google-apis-rs/google_retail2_cli/projects_locations-catalogs-serving-configs-patch",
                  vec![
                    (Some(r##"name"##),
                     None,
                     Some(r##"Immutable. Fully qualified name `projects/*/locations/global/catalogs/*/servingConfig/*`"##),
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
            ("locations-catalogs-serving-configs-predict",
                    Some(r##"Makes a recommendation prediction."##),
                    "Details at http://byron.github.io/google-apis-rs/google_retail2_cli/projects_locations-catalogs-serving-configs-predict",
                  vec![
                    (Some(r##"placement"##),
                     None,
                     Some(r##"Required. Full resource name of the format: `{placement=projects/*/locations/global/catalogs/default_catalog/servingConfigs/*}` or `{placement=projects/*/locations/global/catalogs/default_catalog/placements/*}`. We recommend using the `servingConfigs` resource. `placements` is a legacy resource. The ID of the Recommendations AI serving config or placement. Before you can request predictions from your model, you must create at least one serving config or placement for it. For more information, see [Manage serving configs] (https://cloud.google.com/retail/docs/manage-configs). The full list of available serving configs can be seen at https://console.cloud.google.com/ai/retail/catalogs/default_catalog/configs"##),
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
            ("locations-catalogs-serving-configs-remove-control",
                    Some(r##"Disables a Control on the specified ServingConfig. The control is removed from the ServingConfig. Returns a NOT_FOUND error if the Control is not enabled for the ServingConfig."##),
                    "Details at http://byron.github.io/google-apis-rs/google_retail2_cli/projects_locations-catalogs-serving-configs-remove-control",
                  vec![
                    (Some(r##"serving-config"##),
                     None,
                     Some(r##"Required. The source ServingConfig resource name . Format: `projects/{project_number}/locations/{location_id}/catalogs/{catalog_id}/servingConfigs/{serving_config_id}`"##),
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
            ("locations-catalogs-serving-configs-search",
                    Some(r##"Performs a search. This feature is only available for users who have Retail Search enabled. Enable Retail Search on Cloud Console before using this feature."##),
                    "Details at http://byron.github.io/google-apis-rs/google_retail2_cli/projects_locations-catalogs-serving-configs-search",
                  vec![
                    (Some(r##"placement"##),
                     None,
                     Some(r##"Required. The resource name of the Retail Search serving config, such as `projects/*/locations/global/catalogs/default_catalog/servingConfigs/default_serving_config` or the name of the legacy placement resource, such as `projects/*/locations/global/catalogs/default_catalog/placements/default_search`. This field is used to identify the serving config name and the set of models that are used to make the search."##),
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
            ("locations-catalogs-set-default-branch",
                    Some(r##"Set a specified branch id as default branch. API methods such as SearchService.Search, ProductService.GetProduct, ProductService.ListProducts will treat requests using "default_branch" to the actual branch id set as default. For example, if `projects/*/locations/*/catalogs/*/branches/1` is set as default, setting SearchRequest.branch to `projects/*/locations/*/catalogs/*/branches/default_branch` is equivalent to setting SearchRequest.branch to `projects/*/locations/*/catalogs/*/branches/1`. Using multiple branches can be useful when developers would like to have a staging branch to test and verify for future usage. When it becomes ready, developers switch on the staging branch using this API while keeping using `projects/*/locations/*/catalogs/*/branches/default_branch` as SearchRequest.branch to route the traffic to this staging branch. CAUTION: If you have live predict/search traffic, switching the default branch could potentially cause outages if the ID space of the new branch is very different from the old one. More specifically: * PredictionService will only return product IDs from branch {newBranch}. * SearchService will only return product IDs from branch {newBranch} (if branch is not explicitly set). * UserEventService will only join events with products from branch {newBranch}."##),
                    "Details at http://byron.github.io/google-apis-rs/google_retail2_cli/projects_locations-catalogs-set-default-branch",
                  vec![
                    (Some(r##"catalog"##),
                     None,
                     Some(r##"Full resource name of the catalog, such as `projects/*/locations/global/catalogs/default_catalog`."##),
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
            ("locations-catalogs-update-attributes-config",
                    Some(r##"Updates the AttributesConfig. The catalog attributes in the request will be updated in the catalog, or inserted if they do not exist. Existing catalog attributes not included in the request will remain unchanged. Attributes that are assigned to products, but do not exist at the catalog level, are always included in the response. The product attribute is assigned default values for missing catalog attribute fields, e.g., searchable and dynamic facetable options."##),
                    "Details at http://byron.github.io/google-apis-rs/google_retail2_cli/projects_locations-catalogs-update-attributes-config",
                  vec![
                    (Some(r##"name"##),
                     None,
                     Some(r##"Required. Immutable. The fully qualified resource name of the attribute config. Format: `projects/*/locations/*/catalogs/*/attributesConfig`"##),
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
            ("locations-catalogs-update-completion-config",
                    Some(r##"Updates the CompletionConfigs."##),
                    "Details at http://byron.github.io/google-apis-rs/google_retail2_cli/projects_locations-catalogs-update-completion-config",
                  vec![
                    (Some(r##"name"##),
                     None,
                     Some(r##"Required. Immutable. Fully qualified name `projects/*/locations/*/catalogs/*/completionConfig`"##),
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
            ("locations-catalogs-user-events-collect",
                    Some(r##"Writes a single user event from the browser. This uses a GET request to due to browser restriction of POST-ing to a 3rd party domain. This method is used only by the Retail API JavaScript pixel and Google Tag Manager. Users should not call this method directly."##),
                    "Details at http://byron.github.io/google-apis-rs/google_retail2_cli/projects_locations-catalogs-user-events-collect",
                  vec![
                    (Some(r##"parent"##),
                     None,
                     Some(r##"Required. The parent catalog name, such as `projects/1234/locations/global/catalogs/default_catalog`."##),
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
            ("locations-catalogs-user-events-import",
                    Some(r##"Bulk import of User events. Request processing might be synchronous. Events that already exist are skipped. Use this method for backfilling historical user events. `Operation.response` is of type `ImportResponse`. Note that it is possible for a subset of the items to be successfully inserted. `Operation.metadata` is of type `ImportMetadata`."##),
                    "Details at http://byron.github.io/google-apis-rs/google_retail2_cli/projects_locations-catalogs-user-events-import",
                  vec![
                    (Some(r##"parent"##),
                     None,
                     Some(r##"Required. `projects/1234/locations/global/catalogs/default_catalog`"##),
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
            ("locations-catalogs-user-events-purge",
                    Some(r##"Deletes permanently all user events specified by the filter provided. Depending on the number of events specified by the filter, this operation could take hours or days to complete. To test a filter, use the list command first."##),
                    "Details at http://byron.github.io/google-apis-rs/google_retail2_cli/projects_locations-catalogs-user-events-purge",
                  vec![
                    (Some(r##"parent"##),
                     None,
                     Some(r##"Required. The resource name of the catalog under which the events are created. The format is `projects/${projectId}/locations/global/catalogs/${catalogId}`"##),
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
            ("locations-catalogs-user-events-rejoin",
                    Some(r##"Starts a user-event rejoin operation with latest product catalog. Events are not annotated with detailed product information for products that are missing from the catalog when the user event is ingested. These events are stored as unjoined events with limited usage on training and serving. You can use this method to start a join operation on specified events with the latest version of product catalog. You can also use this method to correct events joined with the wrong product catalog. A rejoin operation can take hours or days to complete."##),
                    "Details at http://byron.github.io/google-apis-rs/google_retail2_cli/projects_locations-catalogs-user-events-rejoin",
                  vec![
                    (Some(r##"parent"##),
                     None,
                     Some(r##"Required. The parent catalog resource name, such as `projects/1234/locations/global/catalogs/default_catalog`."##),
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
            ("locations-catalogs-user-events-write",
                    Some(r##"Writes a single user event."##),
                    "Details at http://byron.github.io/google-apis-rs/google_retail2_cli/projects_locations-catalogs-user-events-write",
                  vec![
                    (Some(r##"parent"##),
                     None,
                     Some(r##"Required. The parent catalog resource name, such as `projects/1234/locations/global/catalogs/default_catalog`."##),
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
            ("locations-operations-get",
                    Some(r##"Gets the latest state of a long-running operation. Clients can use this method to poll the operation result at intervals as recommended by the API service."##),
                    "Details at http://byron.github.io/google-apis-rs/google_retail2_cli/projects_locations-operations-get",
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
                    "Details at http://byron.github.io/google-apis-rs/google_retail2_cli/projects_locations-operations-list",
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
            ("operations-get",
                    Some(r##"Gets the latest state of a long-running operation. Clients can use this method to poll the operation result at intervals as recommended by the API service."##),
                    "Details at http://byron.github.io/google-apis-rs/google_retail2_cli/projects_operations-get",
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
                    "Details at http://byron.github.io/google-apis-rs/google_retail2_cli/projects_operations-list",
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
    
    let mut app = App::new("retail2")
           .author("Sebastian Thiel <byronimo@gmail.com>")
           .version("5.0.5+20240614")
           .about("Vertex AI Search for Retail API is made up of Retail Search, Browse and Recommendations. These discovery AI solutions help you implement personalized search, browse and recommendations, based on machine learning models, across your websites and mobile applications.")
           .after_help("All documentation details can be found at http://byron.github.io/google-apis-rs/google_retail2_cli")
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
