// DO NOT EDIT !
// This file was generated automatically from 'src/mako/cli/main.rs.mako'
// DO NOT EDIT !
#![allow(unused_variables, unused_imports, dead_code, unused_mut)]

#[macro_use]
extern crate clap;
extern crate yup_oauth2 as oauth2;
extern crate yup_hyper_mock as mock;
extern crate hyper_rustls;
extern crate serde;
extern crate serde_json;
extern crate hyper;
extern crate mime;
extern crate strsim;
extern crate google_manufacturers1 as api;

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
    hub: api::ManufacturerCenter<hyper::Client, Authenticator<DefaultAuthenticatorDelegate, JsonTokenStorage, hyper::Client>>,
    gp: Vec<&'static str>,
    gpm: Vec<(&'static str, &'static str)>,
}


impl<'n> Engine<'n> {
    fn _accounts_products_delete(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
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
                CallType::Standard => call.doit(),
                _ => unreachable!()
            } {
                Err(api_err) => Err(DoitError::ApiError(api_err)),
                Ok((mut response, output_schema)) => {
                    let mut value = json::value::to_value(&output_schema).expect("serde to work");
                    remove_json_null_values(&mut value);
                    json::to_writer_pretty(&mut ostream, &value).unwrap();
                    ostream.flush().unwrap();
                    Ok(())
                }
            }
        }
    }

    fn _accounts_products_get(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        let mut call = self.hub.accounts().products_get(opt.value_of("parent").unwrap_or(""), opt.value_of("name").unwrap_or(""));
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
                CallType::Standard => call.doit(),
                _ => unreachable!()
            } {
                Err(api_err) => Err(DoitError::ApiError(api_err)),
                Ok((mut response, output_schema)) => {
                    let mut value = json::value::to_value(&output_schema).expect("serde to work");
                    remove_json_null_values(&mut value);
                    json::to_writer_pretty(&mut ostream, &value).unwrap();
                    ostream.flush().unwrap();
                    Ok(())
                }
            }
        }
    }

    fn _accounts_products_list(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        let mut call = self.hub.accounts().products_list(opt.value_of("parent").unwrap_or(""));
        for parg in opt.values_of("v").map(|i|i.collect()).unwrap_or(Vec::new()).iter() {
            let (key, value) = parse_kv_arg(&*parg, err, false);
            match key {
                "page-token" => {
                    call = call.page_token(value.unwrap_or(""));
                },
                "page-size" => {
                    call = call.page_size(arg_from_str(value.unwrap_or("-0"), err, "page-size", "integer"));
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
                                                                           v.extend(["page-token", "page-size"].iter().map(|v|*v));
                                                                           v } ));
                    }
                }
            }
        }
        let protocol = CallType::Standard;
        if dry_run {
            Ok(())
        } else {
            assert!(err.issues.len() == 0);
            for scope in self.opt.values_of("url").map(|i|i.collect()).unwrap_or(Vec::new()).iter() {
                call = call.add_scope(scope);
            }
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
                    let mut value = json::value::to_value(&output_schema).expect("serde to work");
                    remove_json_null_values(&mut value);
                    json::to_writer_pretty(&mut ostream, &value).unwrap();
                    ostream.flush().unwrap();
                    Ok(())
                }
            }
        }
    }

    fn _accounts_products_update(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
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
                    "target-country" => Some(("targetCountry", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "manually-deleted-attributes" => Some(("manuallyDeletedAttributes", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Vec })),
                    "name" => Some(("name", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "parent" => Some(("parent", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "manually-provided-attributes.product-line" => Some(("manuallyProvidedAttributes.productLine", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "manually-provided-attributes.color" => Some(("manuallyProvidedAttributes.color", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "manually-provided-attributes.release-date" => Some(("manuallyProvidedAttributes.releaseDate", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "manually-provided-attributes.item-group-id" => Some(("manuallyProvidedAttributes.itemGroupId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "manually-provided-attributes.video-link" => Some(("manuallyProvidedAttributes.videoLink", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Vec })),
                    "manually-provided-attributes.flavor" => Some(("manuallyProvidedAttributes.flavor", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "manually-provided-attributes.scent" => Some(("manuallyProvidedAttributes.scent", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "manually-provided-attributes.size" => Some(("manuallyProvidedAttributes.size", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "manually-provided-attributes.capacity.value" => Some(("manuallyProvidedAttributes.capacity.value", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "manually-provided-attributes.capacity.unit" => Some(("manuallyProvidedAttributes.capacity.unit", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "manually-provided-attributes.title" => Some(("manuallyProvidedAttributes.title", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "manually-provided-attributes.pattern" => Some(("manuallyProvidedAttributes.pattern", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "manually-provided-attributes.disclosure-date" => Some(("manuallyProvidedAttributes.disclosureDate", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "manually-provided-attributes.theme" => Some(("manuallyProvidedAttributes.theme", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "manually-provided-attributes.suggested-retail-price.currency" => Some(("manuallyProvidedAttributes.suggestedRetailPrice.currency", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "manually-provided-attributes.suggested-retail-price.amount" => Some(("manuallyProvidedAttributes.suggestedRetailPrice.amount", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "manually-provided-attributes.description" => Some(("manuallyProvidedAttributes.description", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "manually-provided-attributes.format" => Some(("manuallyProvidedAttributes.format", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "manually-provided-attributes.mpn" => Some(("manuallyProvidedAttributes.mpn", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "manually-provided-attributes.brand" => Some(("manuallyProvidedAttributes.brand", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "manually-provided-attributes.material" => Some(("manuallyProvidedAttributes.material", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "manually-provided-attributes.product-name" => Some(("manuallyProvidedAttributes.productName", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "manually-provided-attributes.size-system" => Some(("manuallyProvidedAttributes.sizeSystem", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "manually-provided-attributes.size-type" => Some(("manuallyProvidedAttributes.sizeType", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "manually-provided-attributes.count.value" => Some(("manuallyProvidedAttributes.count.value", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "manually-provided-attributes.count.unit" => Some(("manuallyProvidedAttributes.count.unit", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "manually-provided-attributes.gender" => Some(("manuallyProvidedAttributes.gender", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "manually-provided-attributes.product-page-url" => Some(("manuallyProvidedAttributes.productPageUrl", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "manually-provided-attributes.image-link.status" => Some(("manuallyProvidedAttributes.imageLink.status", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "manually-provided-attributes.image-link.image-url" => Some(("manuallyProvidedAttributes.imageLink.imageUrl", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "manually-provided-attributes.image-link.type" => Some(("manuallyProvidedAttributes.imageLink.type", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "manually-provided-attributes.product-type" => Some(("manuallyProvidedAttributes.productType", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Vec })),
                    "manually-provided-attributes.target-account-id" => Some(("manuallyProvidedAttributes.targetAccountId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "manually-provided-attributes.gtin" => Some(("manuallyProvidedAttributes.gtin", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Vec })),
                    "manually-provided-attributes.age-group" => Some(("manuallyProvidedAttributes.ageGroup", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "final-attributes.product-line" => Some(("finalAttributes.productLine", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "final-attributes.color" => Some(("finalAttributes.color", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "final-attributes.release-date" => Some(("finalAttributes.releaseDate", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "final-attributes.item-group-id" => Some(("finalAttributes.itemGroupId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "final-attributes.video-link" => Some(("finalAttributes.videoLink", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Vec })),
                    "final-attributes.flavor" => Some(("finalAttributes.flavor", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "final-attributes.scent" => Some(("finalAttributes.scent", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "final-attributes.size" => Some(("finalAttributes.size", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "final-attributes.capacity.value" => Some(("finalAttributes.capacity.value", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "final-attributes.capacity.unit" => Some(("finalAttributes.capacity.unit", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "final-attributes.title" => Some(("finalAttributes.title", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "final-attributes.pattern" => Some(("finalAttributes.pattern", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "final-attributes.disclosure-date" => Some(("finalAttributes.disclosureDate", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "final-attributes.theme" => Some(("finalAttributes.theme", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "final-attributes.suggested-retail-price.currency" => Some(("finalAttributes.suggestedRetailPrice.currency", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "final-attributes.suggested-retail-price.amount" => Some(("finalAttributes.suggestedRetailPrice.amount", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "final-attributes.description" => Some(("finalAttributes.description", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "final-attributes.format" => Some(("finalAttributes.format", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "final-attributes.mpn" => Some(("finalAttributes.mpn", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "final-attributes.brand" => Some(("finalAttributes.brand", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "final-attributes.material" => Some(("finalAttributes.material", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "final-attributes.product-name" => Some(("finalAttributes.productName", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "final-attributes.size-system" => Some(("finalAttributes.sizeSystem", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "final-attributes.size-type" => Some(("finalAttributes.sizeType", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "final-attributes.count.value" => Some(("finalAttributes.count.value", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "final-attributes.count.unit" => Some(("finalAttributes.count.unit", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "final-attributes.gender" => Some(("finalAttributes.gender", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "final-attributes.product-page-url" => Some(("finalAttributes.productPageUrl", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "final-attributes.image-link.status" => Some(("finalAttributes.imageLink.status", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "final-attributes.image-link.image-url" => Some(("finalAttributes.imageLink.imageUrl", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "final-attributes.image-link.type" => Some(("finalAttributes.imageLink.type", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "final-attributes.product-type" => Some(("finalAttributes.productType", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Vec })),
                    "final-attributes.target-account-id" => Some(("finalAttributes.targetAccountId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "final-attributes.gtin" => Some(("finalAttributes.gtin", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Vec })),
                    "final-attributes.age-group" => Some(("finalAttributes.ageGroup", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "uploaded-attributes.product-line" => Some(("uploadedAttributes.productLine", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "uploaded-attributes.color" => Some(("uploadedAttributes.color", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "uploaded-attributes.release-date" => Some(("uploadedAttributes.releaseDate", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "uploaded-attributes.item-group-id" => Some(("uploadedAttributes.itemGroupId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "uploaded-attributes.video-link" => Some(("uploadedAttributes.videoLink", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Vec })),
                    "uploaded-attributes.flavor" => Some(("uploadedAttributes.flavor", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "uploaded-attributes.scent" => Some(("uploadedAttributes.scent", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "uploaded-attributes.size" => Some(("uploadedAttributes.size", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "uploaded-attributes.capacity.value" => Some(("uploadedAttributes.capacity.value", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "uploaded-attributes.capacity.unit" => Some(("uploadedAttributes.capacity.unit", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "uploaded-attributes.title" => Some(("uploadedAttributes.title", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "uploaded-attributes.pattern" => Some(("uploadedAttributes.pattern", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "uploaded-attributes.disclosure-date" => Some(("uploadedAttributes.disclosureDate", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "uploaded-attributes.theme" => Some(("uploadedAttributes.theme", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "uploaded-attributes.suggested-retail-price.currency" => Some(("uploadedAttributes.suggestedRetailPrice.currency", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "uploaded-attributes.suggested-retail-price.amount" => Some(("uploadedAttributes.suggestedRetailPrice.amount", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "uploaded-attributes.description" => Some(("uploadedAttributes.description", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "uploaded-attributes.format" => Some(("uploadedAttributes.format", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "uploaded-attributes.mpn" => Some(("uploadedAttributes.mpn", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "uploaded-attributes.brand" => Some(("uploadedAttributes.brand", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "uploaded-attributes.material" => Some(("uploadedAttributes.material", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "uploaded-attributes.product-name" => Some(("uploadedAttributes.productName", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "uploaded-attributes.size-system" => Some(("uploadedAttributes.sizeSystem", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "uploaded-attributes.size-type" => Some(("uploadedAttributes.sizeType", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "uploaded-attributes.count.value" => Some(("uploadedAttributes.count.value", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "uploaded-attributes.count.unit" => Some(("uploadedAttributes.count.unit", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "uploaded-attributes.gender" => Some(("uploadedAttributes.gender", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "uploaded-attributes.product-page-url" => Some(("uploadedAttributes.productPageUrl", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "uploaded-attributes.image-link.status" => Some(("uploadedAttributes.imageLink.status", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "uploaded-attributes.image-link.image-url" => Some(("uploadedAttributes.imageLink.imageUrl", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "uploaded-attributes.image-link.type" => Some(("uploadedAttributes.imageLink.type", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "uploaded-attributes.product-type" => Some(("uploadedAttributes.productType", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Vec })),
                    "uploaded-attributes.target-account-id" => Some(("uploadedAttributes.targetAccountId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "uploaded-attributes.gtin" => Some(("uploadedAttributes.gtin", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Vec })),
                    "uploaded-attributes.age-group" => Some(("uploadedAttributes.ageGroup", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "content-language" => Some(("contentLanguage", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "product-id" => Some(("productId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    _ => {
                        let suggestion = FieldCursor::did_you_mean(key, &vec!["age-group", "amount", "brand", "capacity", "color", "content-language", "count", "currency", "description", "disclosure-date", "final-attributes", "flavor", "format", "gender", "gtin", "image-link", "image-url", "item-group-id", "manually-deleted-attributes", "manually-provided-attributes", "material", "mpn", "name", "parent", "pattern", "product-id", "product-line", "product-name", "product-page-url", "product-type", "release-date", "scent", "size", "size-system", "size-type", "status", "suggested-retail-price", "target-account-id", "target-country", "theme", "title", "type", "unit", "uploaded-attributes", "value", "video-link"]);
                        err.issues.push(CLIError::Field(FieldError::Unknown(temp_cursor.to_string(), suggestion, value.map(|v| v.to_string()))));
                        None
                    }
                };
            if let Some((field_cursor_str, type_info)) = type_info {
                FieldCursor::from(field_cursor_str).set_json_value(&mut object, value.unwrap(), type_info, err, &temp_cursor);
            }
        }
        let mut request: api::Product = json::value::from_value(object).unwrap();
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
                CallType::Standard => call.doit(),
                _ => unreachable!()
            } {
                Err(api_err) => Err(DoitError::ApiError(api_err)),
                Ok((mut response, output_schema)) => {
                    let mut value = json::value::to_value(&output_schema).expect("serde to work");
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
            ("accounts", Some(opt)) => {
                match opt.subcommand() {
                    ("products-delete", Some(opt)) => {
                        call_result = self._accounts_products_delete(opt, dry_run, &mut err);
                    },
                    ("products-get", Some(opt)) => {
                        call_result = self._accounts_products_get(opt, dry_run, &mut err);
                    },
                    ("products-list", Some(opt)) => {
                        call_result = self._accounts_products_list(opt, dry_run, &mut err);
                    },
                    ("products-update", Some(opt)) => {
                        call_result = self._accounts_products_update(opt, dry_run, &mut err);
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
    fn new(opt: ArgMatches<'n>) -> Result<Engine<'n>, InvalidOptionsError> {
        let (config_dir, secret) = {
            let config_dir = match cmn::assure_config_dir_exists(opt.value_of("folder").unwrap_or("~/.google-service-cli")) {
                Err(e) => return Err(InvalidOptionsError::single(e, 3)),
                Ok(p) => p,
            };

            match cmn::application_secret_from_directory(&config_dir, "manufacturers1-secret.json",
                                                         "{\"installed\":{\"auth_uri\":\"https://accounts.google.com/o/oauth2/auth\",\"client_secret\":\"hCsslbCUyfehWMmbkG8vTYxG\",\"token_uri\":\"https://accounts.google.com/o/oauth2/token\",\"client_email\":\"\",\"redirect_uris\":[\"urn:ietf:wg:oauth:2.0:oob\",\"oob\"],\"client_x509_cert_url\":\"\",\"client_id\":\"620010449518-9ngf7o4dhs0dka470npqvor6dc5lqb9b.apps.googleusercontent.com\",\"auth_provider_x509_cert_url\":\"https://www.googleapis.com/oauth2/v1/certs\"}}") {
                Ok(secret) => (config_dir, secret),
                Err(e) => return Err(InvalidOptionsError::single(e, 4))
            }
        };

        let auth = Authenticator::new(  &secret, DefaultAuthenticatorDelegate,
                                        if opt.is_present("debug-auth") {
                                            hyper::Client::with_connector(mock::TeeConnector {
                                                    connector: hyper::net::HttpsConnector::new(hyper_rustls::TlsClient::new())
                                                })
                                        } else {
                                            hyper::Client::with_connector(hyper::net::HttpsConnector::new(hyper_rustls::TlsClient::new()))
                                        },
                                        JsonTokenStorage {
                                          program_name: "manufacturers1",
                                          db_dir: config_dir.clone(),
                                        }, Some(FlowType::InstalledRedirect(54324)));

        let client =
            if opt.is_present("debug") {
                hyper::Client::with_connector(mock::TeeConnector {
                        connector: hyper::net::HttpsConnector::new(hyper_rustls::TlsClient::new())
                    })
            } else {
                hyper::Client::with_connector(hyper::net::HttpsConnector::new(hyper_rustls::TlsClient::new()))
            };
        let engine = Engine {
            opt: opt,
            hub: api::ManufacturerCenter::new(client, auth),
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
        ("accounts", "methods: 'products-delete', 'products-get', 'products-list' and 'products-update'", vec![
            ("products-delete",
                    Some(r##"Deletes the product from a Manufacturer Center account."##),
                    "Details at http://byron.github.io/google-apis-rs/google_manufacturers1_cli/accounts_products-delete",
                  vec![
                    (Some(r##"parent"##),
                     None,
                     Some(r##"Parent ID in the format `accounts/{account_id}`.
        
        `account_id` - The ID of the Manufacturer Center account."##),
                     Some(true),
                     Some(false)),
        
                    (Some(r##"name"##),
                     None,
                     Some(r##"Name in the format `{target_country}:{content_language}:{product_id}`.
        
        `target_country`   - The target country of the product as a CLDR territory
                             code (for example, US).
        
        `content_language` - The content language of the product as a two-letter
                             ISO 639-1 language code (for example, en).
        
        `product_id`     -   The ID of the product. For more information, see
                             https://support.google.com/manufacturers/answer/6124116#id."##),
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
                    Some(r##"Gets the product from a Manufacturer Center account, including product
        issues.
        
        A recently updated product takes around 15 minutes to process. Changes are
        only visible after it has been processed. While some issues may be
        available once the product has been processed, other issues may take days
        to appear."##),
                    "Details at http://byron.github.io/google-apis-rs/google_manufacturers1_cli/accounts_products-get",
                  vec![
                    (Some(r##"parent"##),
                     None,
                     Some(r##"Parent ID in the format `accounts/{account_id}`.
        
        `account_id` - The ID of the Manufacturer Center account."##),
                     Some(true),
                     Some(false)),
        
                    (Some(r##"name"##),
                     None,
                     Some(r##"Name in the format `{target_country}:{content_language}:{product_id}`.
        
        `target_country`   - The target country of the product as a CLDR territory
                             code (for example, US).
        
        `content_language` - The content language of the product as a two-letter
                             ISO 639-1 language code (for example, en).
        
        `product_id`     -   The ID of the product. For more information, see
                             https://support.google.com/manufacturers/answer/6124116#id."##),
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
                     Some(r##"Parent ID in the format `accounts/{account_id}`.
        
        `account_id` - The ID of the Manufacturer Center account."##),
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
                    Some(r##"Inserts or updates the product in a Manufacturer Center account.
        
        The checks at upload time are minimal. All required attributes need to be
        present for a product to be valid. Issues may show up later
        after the API has accepted an update for a product and it is possible to
        overwrite an existing valid product with an invalid product. To detect
        this, you should retrieve the product and check it for issues once the
        updated version is available.
        
        Inserted or updated products first need to be processed before they can be
        retrieved. Until then, new products will be unavailable, and retrieval
        of updated products will return the original state of the product."##),
                    "Details at http://byron.github.io/google-apis-rs/google_manufacturers1_cli/accounts_products-update",
                  vec![
                    (Some(r##"parent"##),
                     None,
                     Some(r##"Parent ID in the format `accounts/{account_id}`.
        
        `account_id` - The ID of the Manufacturer Center account."##),
                     Some(true),
                     Some(false)),
        
                    (Some(r##"name"##),
                     None,
                     Some(r##"Name in the format `{target_country}:{content_language}:{product_id}`.
        
        `target_country`   - The target country of the product as a CLDR territory
                             code (for example, US).
        
        `content_language` - The content language of the product as a two-letter
                             ISO 639-1 language code (for example, en).
        
        `product_id`     -   The ID of the product. For more information, see
                             https://support.google.com/manufacturers/answer/6124116#id."##),
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
           .version("1.0.6+20170808")
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