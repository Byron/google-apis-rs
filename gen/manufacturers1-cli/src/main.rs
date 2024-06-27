// DO NOT EDIT !
// This file was generated automatically from 'src/generator/templates/cli/main.rs.mako'
// DO NOT EDIT !
#![allow(unused_variables, unused_imports, dead_code, unused_mut)]

#[macro_use]
extern crate clap;

use std::env;
use std::io::{self, Write};
use clap::{App, SubCommand, Arg};

use google_manufacturers1::{api, Error, oauth2, client::chrono, FieldMask};


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
    hub: api::ManufacturerCenter<S>,
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
    async fn _accounts_languages_product_certifications_delete(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        let mut call = self.hub.accounts().languages_product_certifications_delete(opt.value_of("name").unwrap_or(""));
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

    async fn _accounts_languages_product_certifications_get(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        let mut call = self.hub.accounts().languages_product_certifications_get(opt.value_of("name").unwrap_or(""));
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

    async fn _accounts_languages_product_certifications_list(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        let mut call = self.hub.accounts().languages_product_certifications_list(opt.value_of("parent").unwrap_or(""));
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

    async fn _accounts_languages_product_certifications_patch(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
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
                    "brand" => Some(("brand", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "country-code" => Some(("countryCode", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Vec })),
                    "mpn" => Some(("mpn", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Vec })),
                    "name" => Some(("name", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "product-code" => Some(("productCode", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Vec })),
                    "product-type" => Some(("productType", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Vec })),
                    "title" => Some(("title", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    _ => {
                        let suggestion = FieldCursor::did_you_mean(key, &vec!["brand", "country-code", "mpn", "name", "product-code", "product-type", "title"]);
                        err.issues.push(CLIError::Field(FieldError::Unknown(temp_cursor.to_string(), suggestion, value.map(|v| v.to_string()))));
                        None
                    }
                };
            if let Some((field_cursor_str, type_info)) = type_info {
                FieldCursor::from(field_cursor_str).set_json_value(&mut object, value.unwrap(), type_info, err, &temp_cursor);
            }
        }
        let mut request: api::ProductCertification = json::value::from_value(object).unwrap();
        let mut call = self.hub.accounts().languages_product_certifications_patch(request, opt.value_of("name").unwrap_or(""));
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

    async fn _accounts_products_delete(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        let mut call = self.hub.accounts().products_delete(opt.value_of("parent").unwrap_or(""), opt.value_of("name").unwrap_or(""));
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

    async fn _accounts_products_get(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        let mut call = self.hub.accounts().products_get(opt.value_of("parent").unwrap_or(""), opt.value_of("name").unwrap_or(""));
        for parg in opt.values_of("v").map(|i|i.collect()).unwrap_or(Vec::new()).iter() {
            let (key, value) = parse_kv_arg(&*parg, err, false);
            match key {
                "include" => {
                    call = call.add_include(value.unwrap_or(""));
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
                                                                           v.extend(["include"].iter().map(|v|*v));
                                                                           v } ));
                    }
                }
            }
        }
        let protocol = CallType::Standard;
        if dry_run {
            Ok(())
        } else {
            assert!(err.issues.len() == 0);
            for scope in self.opt.values_of("url").map(|i|i.collect()).unwrap_or(Vec::new()).iter() {
                call = call.add_scope(scope);
            }
            let mut ostream = match writer_from_opts(opt.value_of("out")) {
                Ok(mut f) => f,
                Err(io_err) => return Err(DoitError::IoError(opt.value_of("out").unwrap_or("-").to_string(), io_err)),
            };
            match match protocol {
                CallType::Standard => call.doit().await,
                _ => unreachable!()
            } {
                Err(api_err) => Err(DoitError::ApiError(api_err)),
                Ok((mut response, output_schema)) => {
                    let mut value = json::value::to_value(&output_schema).expect("serde to work");
                    remove_json_null_values(&mut value);
                    json::to_writer_pretty(&mut ostream, &value).unwrap();
                    ostream.flush().unwrap();
                    Ok(())
                }
            }
        }
    }

    async fn _accounts_products_list(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        let mut call = self.hub.accounts().products_list(opt.value_of("parent").unwrap_or(""));
        for parg in opt.values_of("v").map(|i|i.collect()).unwrap_or(Vec::new()).iter() {
            let (key, value) = parse_kv_arg(&*parg, err, false);
            match key {
                "page-token" => {
                    call = call.page_token(value.unwrap_or(""));
                },
                "page-size" => {
                    call = call.page_size(        value.map(|v| arg_from_str(v, err, "page-size", "int32")).unwrap_or(-0));
                },
                "include" => {
                    call = call.add_include(value.unwrap_or(""));
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
                                                                           v.extend(["include", "page-size", "page-token"].iter().map(|v|*v));
                                                                           v } ));
                    }
                }
            }
        }
        let protocol = CallType::Standard;
        if dry_run {
            Ok(())
        } else {
            assert!(err.issues.len() == 0);
            for scope in self.opt.values_of("url").map(|i|i.collect()).unwrap_or(Vec::new()).iter() {
                call = call.add_scope(scope);
            }
            let mut ostream = match writer_from_opts(opt.value_of("out")) {
                Ok(mut f) => f,
                Err(io_err) => return Err(DoitError::IoError(opt.value_of("out").unwrap_or("-").to_string(), io_err)),
            };
            match match protocol {
                CallType::Standard => call.doit().await,
                _ => unreachable!()
            } {
                Err(api_err) => Err(DoitError::ApiError(api_err)),
                Ok((mut response, output_schema)) => {
                    let mut value = json::value::to_value(&output_schema).expect("serde to work");
                    remove_json_null_values(&mut value);
                    json::to_writer_pretty(&mut ostream, &value).unwrap();
                    ostream.flush().unwrap();
                    Ok(())
                }
            }
        }
    }

    async fn _accounts_products_update(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
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
                    "age-group" => Some(("ageGroup", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "brand" => Some(("brand", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "capacity.unit" => Some(("capacity.unit", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "capacity.value" => Some(("capacity.value", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "color" => Some(("color", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "count.unit" => Some(("count.unit", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "count.value" => Some(("count.value", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "description" => Some(("description", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "disclosure-date" => Some(("disclosureDate", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "excluded-destination" => Some(("excludedDestination", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Vec })),
                    "flavor" => Some(("flavor", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "format" => Some(("format", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "gender" => Some(("gender", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "grocery.active-ingredients" => Some(("grocery.activeIngredients", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "grocery.alcohol-by-volume" => Some(("grocery.alcoholByVolume", JsonTypeInfo { jtype: JsonType::Float, ctype: ComplexType::Pod })),
                    "grocery.allergens" => Some(("grocery.allergens", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "grocery.derived-nutrition-claim" => Some(("grocery.derivedNutritionClaim", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Vec })),
                    "grocery.directions" => Some(("grocery.directions", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "grocery.indications" => Some(("grocery.indications", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "grocery.ingredients" => Some(("grocery.ingredients", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "grocery.nutrition-claim" => Some(("grocery.nutritionClaim", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Vec })),
                    "grocery.storage-instructions" => Some(("grocery.storageInstructions", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "gtin" => Some(("gtin", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Vec })),
                    "image-link.image-url" => Some(("imageLink.imageUrl", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "image-link.status" => Some(("imageLink.status", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "image-link.type" => Some(("imageLink.type", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "included-destination" => Some(("includedDestination", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Vec })),
                    "item-group-id" => Some(("itemGroupId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "material" => Some(("material", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "mpn" => Some(("mpn", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "nutrition.added-sugars.amount" => Some(("nutrition.addedSugars.amount", JsonTypeInfo { jtype: JsonType::Float, ctype: ComplexType::Pod })),
                    "nutrition.added-sugars.unit" => Some(("nutrition.addedSugars.unit", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "nutrition.added-sugars-daily-percentage" => Some(("nutrition.addedSugarsDailyPercentage", JsonTypeInfo { jtype: JsonType::Float, ctype: ComplexType::Pod })),
                    "nutrition.calcium.amount" => Some(("nutrition.calcium.amount", JsonTypeInfo { jtype: JsonType::Float, ctype: ComplexType::Pod })),
                    "nutrition.calcium.unit" => Some(("nutrition.calcium.unit", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "nutrition.calcium-daily-percentage" => Some(("nutrition.calciumDailyPercentage", JsonTypeInfo { jtype: JsonType::Float, ctype: ComplexType::Pod })),
                    "nutrition.cholesterol.amount" => Some(("nutrition.cholesterol.amount", JsonTypeInfo { jtype: JsonType::Float, ctype: ComplexType::Pod })),
                    "nutrition.cholesterol.unit" => Some(("nutrition.cholesterol.unit", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "nutrition.cholesterol-daily-percentage" => Some(("nutrition.cholesterolDailyPercentage", JsonTypeInfo { jtype: JsonType::Float, ctype: ComplexType::Pod })),
                    "nutrition.dietary-fiber.amount" => Some(("nutrition.dietaryFiber.amount", JsonTypeInfo { jtype: JsonType::Float, ctype: ComplexType::Pod })),
                    "nutrition.dietary-fiber.unit" => Some(("nutrition.dietaryFiber.unit", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "nutrition.dietary-fiber-daily-percentage" => Some(("nutrition.dietaryFiberDailyPercentage", JsonTypeInfo { jtype: JsonType::Float, ctype: ComplexType::Pod })),
                    "nutrition.energy.amount" => Some(("nutrition.energy.amount", JsonTypeInfo { jtype: JsonType::Float, ctype: ComplexType::Pod })),
                    "nutrition.energy.unit" => Some(("nutrition.energy.unit", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "nutrition.energy-from-fat.amount" => Some(("nutrition.energyFromFat.amount", JsonTypeInfo { jtype: JsonType::Float, ctype: ComplexType::Pod })),
                    "nutrition.energy-from-fat.unit" => Some(("nutrition.energyFromFat.unit", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "nutrition.folate-daily-percentage" => Some(("nutrition.folateDailyPercentage", JsonTypeInfo { jtype: JsonType::Float, ctype: ComplexType::Pod })),
                    "nutrition.folate-folic-acid.amount" => Some(("nutrition.folateFolicAcid.amount", JsonTypeInfo { jtype: JsonType::Float, ctype: ComplexType::Pod })),
                    "nutrition.folate-folic-acid.unit" => Some(("nutrition.folateFolicAcid.unit", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "nutrition.folate-mcg-dfe" => Some(("nutrition.folateMcgDfe", JsonTypeInfo { jtype: JsonType::Float, ctype: ComplexType::Pod })),
                    "nutrition.iron.amount" => Some(("nutrition.iron.amount", JsonTypeInfo { jtype: JsonType::Float, ctype: ComplexType::Pod })),
                    "nutrition.iron.unit" => Some(("nutrition.iron.unit", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "nutrition.iron-daily-percentage" => Some(("nutrition.ironDailyPercentage", JsonTypeInfo { jtype: JsonType::Float, ctype: ComplexType::Pod })),
                    "nutrition.monounsaturated-fat.amount" => Some(("nutrition.monounsaturatedFat.amount", JsonTypeInfo { jtype: JsonType::Float, ctype: ComplexType::Pod })),
                    "nutrition.monounsaturated-fat.unit" => Some(("nutrition.monounsaturatedFat.unit", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "nutrition.nutrition-fact-measure" => Some(("nutrition.nutritionFactMeasure", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "nutrition.polyols.amount" => Some(("nutrition.polyols.amount", JsonTypeInfo { jtype: JsonType::Float, ctype: ComplexType::Pod })),
                    "nutrition.polyols.unit" => Some(("nutrition.polyols.unit", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "nutrition.polyunsaturated-fat.amount" => Some(("nutrition.polyunsaturatedFat.amount", JsonTypeInfo { jtype: JsonType::Float, ctype: ComplexType::Pod })),
                    "nutrition.polyunsaturated-fat.unit" => Some(("nutrition.polyunsaturatedFat.unit", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "nutrition.potassium.amount" => Some(("nutrition.potassium.amount", JsonTypeInfo { jtype: JsonType::Float, ctype: ComplexType::Pod })),
                    "nutrition.potassium.unit" => Some(("nutrition.potassium.unit", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "nutrition.potassium-daily-percentage" => Some(("nutrition.potassiumDailyPercentage", JsonTypeInfo { jtype: JsonType::Float, ctype: ComplexType::Pod })),
                    "nutrition.prepared-size-description" => Some(("nutrition.preparedSizeDescription", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "nutrition.protein.amount" => Some(("nutrition.protein.amount", JsonTypeInfo { jtype: JsonType::Float, ctype: ComplexType::Pod })),
                    "nutrition.protein.unit" => Some(("nutrition.protein.unit", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "nutrition.protein-daily-percentage" => Some(("nutrition.proteinDailyPercentage", JsonTypeInfo { jtype: JsonType::Float, ctype: ComplexType::Pod })),
                    "nutrition.saturated-fat.amount" => Some(("nutrition.saturatedFat.amount", JsonTypeInfo { jtype: JsonType::Float, ctype: ComplexType::Pod })),
                    "nutrition.saturated-fat.unit" => Some(("nutrition.saturatedFat.unit", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "nutrition.saturated-fat-daily-percentage" => Some(("nutrition.saturatedFatDailyPercentage", JsonTypeInfo { jtype: JsonType::Float, ctype: ComplexType::Pod })),
                    "nutrition.serving-size-description" => Some(("nutrition.servingSizeDescription", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "nutrition.serving-size-measure.amount" => Some(("nutrition.servingSizeMeasure.amount", JsonTypeInfo { jtype: JsonType::Float, ctype: ComplexType::Pod })),
                    "nutrition.serving-size-measure.unit" => Some(("nutrition.servingSizeMeasure.unit", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "nutrition.servings-per-container" => Some(("nutrition.servingsPerContainer", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "nutrition.sodium.amount" => Some(("nutrition.sodium.amount", JsonTypeInfo { jtype: JsonType::Float, ctype: ComplexType::Pod })),
                    "nutrition.sodium.unit" => Some(("nutrition.sodium.unit", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "nutrition.sodium-daily-percentage" => Some(("nutrition.sodiumDailyPercentage", JsonTypeInfo { jtype: JsonType::Float, ctype: ComplexType::Pod })),
                    "nutrition.starch.amount" => Some(("nutrition.starch.amount", JsonTypeInfo { jtype: JsonType::Float, ctype: ComplexType::Pod })),
                    "nutrition.starch.unit" => Some(("nutrition.starch.unit", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "nutrition.total-carbohydrate.amount" => Some(("nutrition.totalCarbohydrate.amount", JsonTypeInfo { jtype: JsonType::Float, ctype: ComplexType::Pod })),
                    "nutrition.total-carbohydrate.unit" => Some(("nutrition.totalCarbohydrate.unit", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "nutrition.total-carbohydrate-daily-percentage" => Some(("nutrition.totalCarbohydrateDailyPercentage", JsonTypeInfo { jtype: JsonType::Float, ctype: ComplexType::Pod })),
                    "nutrition.total-fat.amount" => Some(("nutrition.totalFat.amount", JsonTypeInfo { jtype: JsonType::Float, ctype: ComplexType::Pod })),
                    "nutrition.total-fat.unit" => Some(("nutrition.totalFat.unit", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "nutrition.total-fat-daily-percentage" => Some(("nutrition.totalFatDailyPercentage", JsonTypeInfo { jtype: JsonType::Float, ctype: ComplexType::Pod })),
                    "nutrition.total-sugars.amount" => Some(("nutrition.totalSugars.amount", JsonTypeInfo { jtype: JsonType::Float, ctype: ComplexType::Pod })),
                    "nutrition.total-sugars.unit" => Some(("nutrition.totalSugars.unit", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "nutrition.total-sugars-daily-percentage" => Some(("nutrition.totalSugarsDailyPercentage", JsonTypeInfo { jtype: JsonType::Float, ctype: ComplexType::Pod })),
                    "nutrition.trans-fat.amount" => Some(("nutrition.transFat.amount", JsonTypeInfo { jtype: JsonType::Float, ctype: ComplexType::Pod })),
                    "nutrition.trans-fat.unit" => Some(("nutrition.transFat.unit", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "nutrition.trans-fat-daily-percentage" => Some(("nutrition.transFatDailyPercentage", JsonTypeInfo { jtype: JsonType::Float, ctype: ComplexType::Pod })),
                    "nutrition.vitamin-d.amount" => Some(("nutrition.vitaminD.amount", JsonTypeInfo { jtype: JsonType::Float, ctype: ComplexType::Pod })),
                    "nutrition.vitamin-d.unit" => Some(("nutrition.vitaminD.unit", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "nutrition.vitamin-d-daily-percentage" => Some(("nutrition.vitaminDDailyPercentage", JsonTypeInfo { jtype: JsonType::Float, ctype: ComplexType::Pod })),
                    "pattern" => Some(("pattern", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "product-highlight" => Some(("productHighlight", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Vec })),
                    "product-line" => Some(("productLine", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "product-name" => Some(("productName", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "product-page-url" => Some(("productPageUrl", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "product-type" => Some(("productType", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Vec })),
                    "release-date" => Some(("releaseDate", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "rich-product-content" => Some(("richProductContent", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Vec })),
                    "scent" => Some(("scent", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "size" => Some(("size", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "size-system" => Some(("sizeSystem", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "size-type" => Some(("sizeType", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Vec })),
                    "suggested-retail-price.amount" => Some(("suggestedRetailPrice.amount", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "suggested-retail-price.currency" => Some(("suggestedRetailPrice.currency", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "target-client-id" => Some(("targetClientId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "theme" => Some(("theme", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "title" => Some(("title", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "video-link" => Some(("videoLink", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Vec })),
                    "virtual-model-link" => Some(("virtualModelLink", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    _ => {
                        let suggestion = FieldCursor::did_you_mean(key, &vec!["active-ingredients", "added-sugars", "added-sugars-daily-percentage", "age-group", "alcohol-by-volume", "allergens", "amount", "brand", "calcium", "calcium-daily-percentage", "capacity", "cholesterol", "cholesterol-daily-percentage", "color", "count", "currency", "derived-nutrition-claim", "description", "dietary-fiber", "dietary-fiber-daily-percentage", "directions", "disclosure-date", "energy", "energy-from-fat", "excluded-destination", "flavor", "folate-daily-percentage", "folate-folic-acid", "folate-mcg-dfe", "format", "gender", "grocery", "gtin", "image-link", "image-url", "included-destination", "indications", "ingredients", "iron", "iron-daily-percentage", "item-group-id", "material", "monounsaturated-fat", "mpn", "nutrition", "nutrition-claim", "nutrition-fact-measure", "pattern", "polyols", "polyunsaturated-fat", "potassium", "potassium-daily-percentage", "prepared-size-description", "product-highlight", "product-line", "product-name", "product-page-url", "product-type", "protein", "protein-daily-percentage", "release-date", "rich-product-content", "saturated-fat", "saturated-fat-daily-percentage", "scent", "serving-size-description", "serving-size-measure", "servings-per-container", "size", "size-system", "size-type", "sodium", "sodium-daily-percentage", "starch", "status", "storage-instructions", "suggested-retail-price", "target-client-id", "theme", "title", "total-carbohydrate", "total-carbohydrate-daily-percentage", "total-fat", "total-fat-daily-percentage", "total-sugars", "total-sugars-daily-percentage", "trans-fat", "trans-fat-daily-percentage", "type", "unit", "value", "video-link", "virtual-model-link", "vitamin-d", "vitamin-d-daily-percentage"]);
                        err.issues.push(CLIError::Field(FieldError::Unknown(temp_cursor.to_string(), suggestion, value.map(|v| v.to_string()))));
                        None
                    }
                };
            if let Some((field_cursor_str, type_info)) = type_info {
                FieldCursor::from(field_cursor_str).set_json_value(&mut object, value.unwrap(), type_info, err, &temp_cursor);
            }
        }
        let mut request: api::Attributes = json::value::from_value(object).unwrap();
        let mut call = self.hub.accounts().products_update(request, opt.value_of("parent").unwrap_or(""), opt.value_of("name").unwrap_or(""));
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
                    ("languages-product-certifications-delete", Some(opt)) => {
                        call_result = self._accounts_languages_product_certifications_delete(opt, dry_run, &mut err).await;
                    },
                    ("languages-product-certifications-get", Some(opt)) => {
                        call_result = self._accounts_languages_product_certifications_get(opt, dry_run, &mut err).await;
                    },
                    ("languages-product-certifications-list", Some(opt)) => {
                        call_result = self._accounts_languages_product_certifications_list(opt, dry_run, &mut err).await;
                    },
                    ("languages-product-certifications-patch", Some(opt)) => {
                        call_result = self._accounts_languages_product_certifications_patch(opt, dry_run, &mut err).await;
                    },
                    ("products-delete", Some(opt)) => {
                        call_result = self._accounts_products_delete(opt, dry_run, &mut err).await;
                    },
                    ("products-get", Some(opt)) => {
                        call_result = self._accounts_products_get(opt, dry_run, &mut err).await;
                    },
                    ("products-list", Some(opt)) => {
                        call_result = self._accounts_products_list(opt, dry_run, &mut err).await;
                    },
                    ("products-update", Some(opt)) => {
                        call_result = self._accounts_products_update(opt, dry_run, &mut err).await;
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

            match client::application_secret_from_directory(&config_dir, "manufacturers1-secret.json",
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
        ).persist_tokens_to_disk(format!("{}/manufacturers1", config_dir)).build().await.unwrap();

        let engine = Engine {
            opt: opt,
            hub: api::ManufacturerCenter::new(client, auth),
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
        ("accounts", "methods: 'languages-product-certifications-delete', 'languages-product-certifications-get', 'languages-product-certifications-list', 'languages-product-certifications-patch', 'products-delete', 'products-get', 'products-list' and 'products-update'", vec![
            ("languages-product-certifications-delete",
                    Some(r##"Deletes a product certification by its name. This method can only be called by certification bodies."##),
                    "Details at http://byron.github.io/google-apis-rs/google_manufacturers1_cli/accounts_languages-product-certifications-delete",
                  vec![
                    (Some(r##"name"##),
                     None,
                     Some(r##"Required. The name of the product certification to delete. Format: accounts/{account}/languages/{language_code}/productCertifications/{id}"##),
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
            ("languages-product-certifications-get",
                    Some(r##"Gets a product certification by its name. This method can only be called by certification bodies."##),
                    "Details at http://byron.github.io/google-apis-rs/google_manufacturers1_cli/accounts_languages-product-certifications-get",
                  vec![
                    (Some(r##"name"##),
                     None,
                     Some(r##"Required. The name of the product certification to get. Format: accounts/{account}/languages/{language_code}/productCertifications/{id}"##),
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
            ("languages-product-certifications-list",
                    Some(r##"Lists product certifications from a specified certification body. This method can only be called by certification bodies."##),
                    "Details at http://byron.github.io/google-apis-rs/google_manufacturers1_cli/accounts_languages-product-certifications-list",
                  vec![
                    (Some(r##"parent"##),
                     None,
                     Some(r##"Required. The parent, which owns this collection of product certifications. Format: accounts/{account}/languages/{language_code}"##),
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
            ("languages-product-certifications-patch",
                    Some(r##"Updates (or creates if allow_missing = true) a product certification which links certifications with products. This method can only be called by certification bodies."##),
                    "Details at http://byron.github.io/google-apis-rs/google_manufacturers1_cli/accounts_languages-product-certifications-patch",
                  vec![
                    (Some(r##"name"##),
                     None,
                     Some(r##"Required. The unique name identifier of a product certification Format: accounts/{account}/languages/{language_code}/productCertifications/{id} Where `id` is a some unique identifier and `language_code` is a 2-letter ISO 639-1 code of a Shopping supported language according to https://support.google.com/merchants/answer/160637."##),
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
            ("products-delete",
                    Some(r##"Deletes the product from a Manufacturer Center account."##),
                    "Details at http://byron.github.io/google-apis-rs/google_manufacturers1_cli/accounts_products-delete",
                  vec![
                    (Some(r##"parent"##),
                     None,
                     Some(r##"Parent ID in the format `accounts/{account_id}`. `account_id` - The ID of the Manufacturer Center account."##),
                     Some(true),
                     Some(false)),
        
                    (Some(r##"name"##),
                     None,
                     Some(r##"Name in the format `{target_country}:{content_language}:{product_id}`. `target_country` - The target country of the product as a CLDR territory code (for example, US). `content_language` - The content language of the product as a two-letter ISO 639-1 language code (for example, en). `product_id` - The ID of the product. For more information, see https://support.google.com/manufacturers/answer/6124116#id."##),
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
            ("products-get",
                    Some(r##"Gets the product from a Manufacturer Center account, including product issues. A recently updated product takes around 15 minutes to process. Changes are only visible after it has been processed. While some issues may be available once the product has been processed, other issues may take days to appear."##),
                    "Details at http://byron.github.io/google-apis-rs/google_manufacturers1_cli/accounts_products-get",
                  vec![
                    (Some(r##"parent"##),
                     None,
                     Some(r##"Parent ID in the format `accounts/{account_id}`. `account_id` - The ID of the Manufacturer Center account."##),
                     Some(true),
                     Some(false)),
        
                    (Some(r##"name"##),
                     None,
                     Some(r##"Name in the format `{target_country}:{content_language}:{product_id}`. `target_country` - The target country of the product as a CLDR territory code (for example, US). `content_language` - The content language of the product as a two-letter ISO 639-1 language code (for example, en). `product_id` - The ID of the product. For more information, see https://support.google.com/manufacturers/answer/6124116#id."##),
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
            ("products-list",
                    Some(r##"Lists all the products in a Manufacturer Center account."##),
                    "Details at http://byron.github.io/google-apis-rs/google_manufacturers1_cli/accounts_products-list",
                  vec![
                    (Some(r##"parent"##),
                     None,
                     Some(r##"Parent ID in the format `accounts/{account_id}`. `account_id` - The ID of the Manufacturer Center account."##),
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
            ("products-update",
                    Some(r##"Inserts or updates the attributes of the product in a Manufacturer Center account. Creates a product with the provided attributes. If the product already exists, then all attributes are replaced with the new ones. The checks at upload time are minimal. All required attributes need to be present for a product to be valid. Issues may show up later after the API has accepted a new upload for a product and it is possible to overwrite an existing valid product with an invalid product. To detect this, you should retrieve the product and check it for issues once the new version is available. Uploaded attributes first need to be processed before they can be retrieved. Until then, new products will be unavailable, and retrieval of previously uploaded products will return the original state of the product."##),
                    "Details at http://byron.github.io/google-apis-rs/google_manufacturers1_cli/accounts_products-update",
                  vec![
                    (Some(r##"parent"##),
                     None,
                     Some(r##"Parent ID in the format `accounts/{account_id}`. `account_id` - The ID of the Manufacturer Center account."##),
                     Some(true),
                     Some(false)),
        
                    (Some(r##"name"##),
                     None,
                     Some(r##"Name in the format `{target_country}:{content_language}:{product_id}`. `target_country` - The target country of the product as a CLDR territory code (for example, US). `content_language` - The content language of the product as a two-letter ISO 639-1 language code (for example, en). `product_id` - The ID of the product. For more information, see https://support.google.com/manufacturers/answer/6124116#id."##),
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
    
    let mut app = App::new("manufacturers1")
           .author("Sebastian Thiel <byronimo@gmail.com>")
           .version("5.0.5+20240524")
           .about("Public API for managing Manufacturer Center related data.")
           .after_help("All documentation details can be found at http://byron.github.io/google-apis-rs/google_manufacturers1_cli")
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
