// DO NOT EDIT !
// This file was generated automatically from 'src/generator/templates/cli/main.rs.mako'
// DO NOT EDIT !
#![allow(unused_variables, unused_imports, dead_code, unused_mut)]

#[macro_use]
extern crate clap;

use std::env;
use std::io::{self, Write};
use clap::{App, SubCommand, Arg};

use google_sheets4::{api, Error, oauth2, client::chrono, FieldMask};


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
    hub: api::Sheets<S>,
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
    async fn _spreadsheets_batch_update(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
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
                    "include-spreadsheet-in-response" => Some(("includeSpreadsheetInResponse", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "response-include-grid-data" => Some(("responseIncludeGridData", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "response-ranges" => Some(("responseRanges", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Vec })),
                    _ => {
                        let suggestion = FieldCursor::did_you_mean(key, &vec!["include-spreadsheet-in-response", "response-include-grid-data", "response-ranges"]);
                        err.issues.push(CLIError::Field(FieldError::Unknown(temp_cursor.to_string(), suggestion, value.map(|v| v.to_string()))));
                        None
                    }
                };
            if let Some((field_cursor_str, type_info)) = type_info {
                FieldCursor::from(field_cursor_str).set_json_value(&mut object, value.unwrap(), type_info, err, &temp_cursor);
            }
        }
        let mut request: api::BatchUpdateSpreadsheetRequest = json::value::from_value(object).unwrap();
        let mut call = self.hub.spreadsheets().batch_update(request, opt.value_of("spreadsheet-id").unwrap_or(""));
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

    async fn _spreadsheets_create(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
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
                    "properties.auto-recalc" => Some(("properties.autoRecalc", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "properties.default-format.background-color.alpha" => Some(("properties.defaultFormat.backgroundColor.alpha", JsonTypeInfo { jtype: JsonType::Float, ctype: ComplexType::Pod })),
                    "properties.default-format.background-color.blue" => Some(("properties.defaultFormat.backgroundColor.blue", JsonTypeInfo { jtype: JsonType::Float, ctype: ComplexType::Pod })),
                    "properties.default-format.background-color.green" => Some(("properties.defaultFormat.backgroundColor.green", JsonTypeInfo { jtype: JsonType::Float, ctype: ComplexType::Pod })),
                    "properties.default-format.background-color.red" => Some(("properties.defaultFormat.backgroundColor.red", JsonTypeInfo { jtype: JsonType::Float, ctype: ComplexType::Pod })),
                    "properties.default-format.background-color-style.rgb-color.alpha" => Some(("properties.defaultFormat.backgroundColorStyle.rgbColor.alpha", JsonTypeInfo { jtype: JsonType::Float, ctype: ComplexType::Pod })),
                    "properties.default-format.background-color-style.rgb-color.blue" => Some(("properties.defaultFormat.backgroundColorStyle.rgbColor.blue", JsonTypeInfo { jtype: JsonType::Float, ctype: ComplexType::Pod })),
                    "properties.default-format.background-color-style.rgb-color.green" => Some(("properties.defaultFormat.backgroundColorStyle.rgbColor.green", JsonTypeInfo { jtype: JsonType::Float, ctype: ComplexType::Pod })),
                    "properties.default-format.background-color-style.rgb-color.red" => Some(("properties.defaultFormat.backgroundColorStyle.rgbColor.red", JsonTypeInfo { jtype: JsonType::Float, ctype: ComplexType::Pod })),
                    "properties.default-format.background-color-style.theme-color" => Some(("properties.defaultFormat.backgroundColorStyle.themeColor", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "properties.default-format.borders.bottom.color.alpha" => Some(("properties.defaultFormat.borders.bottom.color.alpha", JsonTypeInfo { jtype: JsonType::Float, ctype: ComplexType::Pod })),
                    "properties.default-format.borders.bottom.color.blue" => Some(("properties.defaultFormat.borders.bottom.color.blue", JsonTypeInfo { jtype: JsonType::Float, ctype: ComplexType::Pod })),
                    "properties.default-format.borders.bottom.color.green" => Some(("properties.defaultFormat.borders.bottom.color.green", JsonTypeInfo { jtype: JsonType::Float, ctype: ComplexType::Pod })),
                    "properties.default-format.borders.bottom.color.red" => Some(("properties.defaultFormat.borders.bottom.color.red", JsonTypeInfo { jtype: JsonType::Float, ctype: ComplexType::Pod })),
                    "properties.default-format.borders.bottom.color-style.rgb-color.alpha" => Some(("properties.defaultFormat.borders.bottom.colorStyle.rgbColor.alpha", JsonTypeInfo { jtype: JsonType::Float, ctype: ComplexType::Pod })),
                    "properties.default-format.borders.bottom.color-style.rgb-color.blue" => Some(("properties.defaultFormat.borders.bottom.colorStyle.rgbColor.blue", JsonTypeInfo { jtype: JsonType::Float, ctype: ComplexType::Pod })),
                    "properties.default-format.borders.bottom.color-style.rgb-color.green" => Some(("properties.defaultFormat.borders.bottom.colorStyle.rgbColor.green", JsonTypeInfo { jtype: JsonType::Float, ctype: ComplexType::Pod })),
                    "properties.default-format.borders.bottom.color-style.rgb-color.red" => Some(("properties.defaultFormat.borders.bottom.colorStyle.rgbColor.red", JsonTypeInfo { jtype: JsonType::Float, ctype: ComplexType::Pod })),
                    "properties.default-format.borders.bottom.color-style.theme-color" => Some(("properties.defaultFormat.borders.bottom.colorStyle.themeColor", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "properties.default-format.borders.bottom.style" => Some(("properties.defaultFormat.borders.bottom.style", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "properties.default-format.borders.bottom.width" => Some(("properties.defaultFormat.borders.bottom.width", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "properties.default-format.borders.left.color.alpha" => Some(("properties.defaultFormat.borders.left.color.alpha", JsonTypeInfo { jtype: JsonType::Float, ctype: ComplexType::Pod })),
                    "properties.default-format.borders.left.color.blue" => Some(("properties.defaultFormat.borders.left.color.blue", JsonTypeInfo { jtype: JsonType::Float, ctype: ComplexType::Pod })),
                    "properties.default-format.borders.left.color.green" => Some(("properties.defaultFormat.borders.left.color.green", JsonTypeInfo { jtype: JsonType::Float, ctype: ComplexType::Pod })),
                    "properties.default-format.borders.left.color.red" => Some(("properties.defaultFormat.borders.left.color.red", JsonTypeInfo { jtype: JsonType::Float, ctype: ComplexType::Pod })),
                    "properties.default-format.borders.left.color-style.rgb-color.alpha" => Some(("properties.defaultFormat.borders.left.colorStyle.rgbColor.alpha", JsonTypeInfo { jtype: JsonType::Float, ctype: ComplexType::Pod })),
                    "properties.default-format.borders.left.color-style.rgb-color.blue" => Some(("properties.defaultFormat.borders.left.colorStyle.rgbColor.blue", JsonTypeInfo { jtype: JsonType::Float, ctype: ComplexType::Pod })),
                    "properties.default-format.borders.left.color-style.rgb-color.green" => Some(("properties.defaultFormat.borders.left.colorStyle.rgbColor.green", JsonTypeInfo { jtype: JsonType::Float, ctype: ComplexType::Pod })),
                    "properties.default-format.borders.left.color-style.rgb-color.red" => Some(("properties.defaultFormat.borders.left.colorStyle.rgbColor.red", JsonTypeInfo { jtype: JsonType::Float, ctype: ComplexType::Pod })),
                    "properties.default-format.borders.left.color-style.theme-color" => Some(("properties.defaultFormat.borders.left.colorStyle.themeColor", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "properties.default-format.borders.left.style" => Some(("properties.defaultFormat.borders.left.style", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "properties.default-format.borders.left.width" => Some(("properties.defaultFormat.borders.left.width", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "properties.default-format.borders.right.color.alpha" => Some(("properties.defaultFormat.borders.right.color.alpha", JsonTypeInfo { jtype: JsonType::Float, ctype: ComplexType::Pod })),
                    "properties.default-format.borders.right.color.blue" => Some(("properties.defaultFormat.borders.right.color.blue", JsonTypeInfo { jtype: JsonType::Float, ctype: ComplexType::Pod })),
                    "properties.default-format.borders.right.color.green" => Some(("properties.defaultFormat.borders.right.color.green", JsonTypeInfo { jtype: JsonType::Float, ctype: ComplexType::Pod })),
                    "properties.default-format.borders.right.color.red" => Some(("properties.defaultFormat.borders.right.color.red", JsonTypeInfo { jtype: JsonType::Float, ctype: ComplexType::Pod })),
                    "properties.default-format.borders.right.color-style.rgb-color.alpha" => Some(("properties.defaultFormat.borders.right.colorStyle.rgbColor.alpha", JsonTypeInfo { jtype: JsonType::Float, ctype: ComplexType::Pod })),
                    "properties.default-format.borders.right.color-style.rgb-color.blue" => Some(("properties.defaultFormat.borders.right.colorStyle.rgbColor.blue", JsonTypeInfo { jtype: JsonType::Float, ctype: ComplexType::Pod })),
                    "properties.default-format.borders.right.color-style.rgb-color.green" => Some(("properties.defaultFormat.borders.right.colorStyle.rgbColor.green", JsonTypeInfo { jtype: JsonType::Float, ctype: ComplexType::Pod })),
                    "properties.default-format.borders.right.color-style.rgb-color.red" => Some(("properties.defaultFormat.borders.right.colorStyle.rgbColor.red", JsonTypeInfo { jtype: JsonType::Float, ctype: ComplexType::Pod })),
                    "properties.default-format.borders.right.color-style.theme-color" => Some(("properties.defaultFormat.borders.right.colorStyle.themeColor", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "properties.default-format.borders.right.style" => Some(("properties.defaultFormat.borders.right.style", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "properties.default-format.borders.right.width" => Some(("properties.defaultFormat.borders.right.width", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "properties.default-format.borders.top.color.alpha" => Some(("properties.defaultFormat.borders.top.color.alpha", JsonTypeInfo { jtype: JsonType::Float, ctype: ComplexType::Pod })),
                    "properties.default-format.borders.top.color.blue" => Some(("properties.defaultFormat.borders.top.color.blue", JsonTypeInfo { jtype: JsonType::Float, ctype: ComplexType::Pod })),
                    "properties.default-format.borders.top.color.green" => Some(("properties.defaultFormat.borders.top.color.green", JsonTypeInfo { jtype: JsonType::Float, ctype: ComplexType::Pod })),
                    "properties.default-format.borders.top.color.red" => Some(("properties.defaultFormat.borders.top.color.red", JsonTypeInfo { jtype: JsonType::Float, ctype: ComplexType::Pod })),
                    "properties.default-format.borders.top.color-style.rgb-color.alpha" => Some(("properties.defaultFormat.borders.top.colorStyle.rgbColor.alpha", JsonTypeInfo { jtype: JsonType::Float, ctype: ComplexType::Pod })),
                    "properties.default-format.borders.top.color-style.rgb-color.blue" => Some(("properties.defaultFormat.borders.top.colorStyle.rgbColor.blue", JsonTypeInfo { jtype: JsonType::Float, ctype: ComplexType::Pod })),
                    "properties.default-format.borders.top.color-style.rgb-color.green" => Some(("properties.defaultFormat.borders.top.colorStyle.rgbColor.green", JsonTypeInfo { jtype: JsonType::Float, ctype: ComplexType::Pod })),
                    "properties.default-format.borders.top.color-style.rgb-color.red" => Some(("properties.defaultFormat.borders.top.colorStyle.rgbColor.red", JsonTypeInfo { jtype: JsonType::Float, ctype: ComplexType::Pod })),
                    "properties.default-format.borders.top.color-style.theme-color" => Some(("properties.defaultFormat.borders.top.colorStyle.themeColor", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "properties.default-format.borders.top.style" => Some(("properties.defaultFormat.borders.top.style", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "properties.default-format.borders.top.width" => Some(("properties.defaultFormat.borders.top.width", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "properties.default-format.horizontal-alignment" => Some(("properties.defaultFormat.horizontalAlignment", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "properties.default-format.hyperlink-display-type" => Some(("properties.defaultFormat.hyperlinkDisplayType", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "properties.default-format.number-format.pattern" => Some(("properties.defaultFormat.numberFormat.pattern", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "properties.default-format.number-format.type" => Some(("properties.defaultFormat.numberFormat.type", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "properties.default-format.padding.bottom" => Some(("properties.defaultFormat.padding.bottom", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "properties.default-format.padding.left" => Some(("properties.defaultFormat.padding.left", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "properties.default-format.padding.right" => Some(("properties.defaultFormat.padding.right", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "properties.default-format.padding.top" => Some(("properties.defaultFormat.padding.top", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "properties.default-format.text-direction" => Some(("properties.defaultFormat.textDirection", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "properties.default-format.text-format.bold" => Some(("properties.defaultFormat.textFormat.bold", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "properties.default-format.text-format.font-family" => Some(("properties.defaultFormat.textFormat.fontFamily", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "properties.default-format.text-format.font-size" => Some(("properties.defaultFormat.textFormat.fontSize", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "properties.default-format.text-format.foreground-color.alpha" => Some(("properties.defaultFormat.textFormat.foregroundColor.alpha", JsonTypeInfo { jtype: JsonType::Float, ctype: ComplexType::Pod })),
                    "properties.default-format.text-format.foreground-color.blue" => Some(("properties.defaultFormat.textFormat.foregroundColor.blue", JsonTypeInfo { jtype: JsonType::Float, ctype: ComplexType::Pod })),
                    "properties.default-format.text-format.foreground-color.green" => Some(("properties.defaultFormat.textFormat.foregroundColor.green", JsonTypeInfo { jtype: JsonType::Float, ctype: ComplexType::Pod })),
                    "properties.default-format.text-format.foreground-color.red" => Some(("properties.defaultFormat.textFormat.foregroundColor.red", JsonTypeInfo { jtype: JsonType::Float, ctype: ComplexType::Pod })),
                    "properties.default-format.text-format.foreground-color-style.rgb-color.alpha" => Some(("properties.defaultFormat.textFormat.foregroundColorStyle.rgbColor.alpha", JsonTypeInfo { jtype: JsonType::Float, ctype: ComplexType::Pod })),
                    "properties.default-format.text-format.foreground-color-style.rgb-color.blue" => Some(("properties.defaultFormat.textFormat.foregroundColorStyle.rgbColor.blue", JsonTypeInfo { jtype: JsonType::Float, ctype: ComplexType::Pod })),
                    "properties.default-format.text-format.foreground-color-style.rgb-color.green" => Some(("properties.defaultFormat.textFormat.foregroundColorStyle.rgbColor.green", JsonTypeInfo { jtype: JsonType::Float, ctype: ComplexType::Pod })),
                    "properties.default-format.text-format.foreground-color-style.rgb-color.red" => Some(("properties.defaultFormat.textFormat.foregroundColorStyle.rgbColor.red", JsonTypeInfo { jtype: JsonType::Float, ctype: ComplexType::Pod })),
                    "properties.default-format.text-format.foreground-color-style.theme-color" => Some(("properties.defaultFormat.textFormat.foregroundColorStyle.themeColor", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "properties.default-format.text-format.italic" => Some(("properties.defaultFormat.textFormat.italic", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "properties.default-format.text-format.link.uri" => Some(("properties.defaultFormat.textFormat.link.uri", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "properties.default-format.text-format.strikethrough" => Some(("properties.defaultFormat.textFormat.strikethrough", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "properties.default-format.text-format.underline" => Some(("properties.defaultFormat.textFormat.underline", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "properties.default-format.text-rotation.angle" => Some(("properties.defaultFormat.textRotation.angle", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "properties.default-format.text-rotation.vertical" => Some(("properties.defaultFormat.textRotation.vertical", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "properties.default-format.vertical-alignment" => Some(("properties.defaultFormat.verticalAlignment", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "properties.default-format.wrap-strategy" => Some(("properties.defaultFormat.wrapStrategy", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "properties.import-functions-external-url-access-allowed" => Some(("properties.importFunctionsExternalUrlAccessAllowed", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "properties.iterative-calculation-settings.convergence-threshold" => Some(("properties.iterativeCalculationSettings.convergenceThreshold", JsonTypeInfo { jtype: JsonType::Float, ctype: ComplexType::Pod })),
                    "properties.iterative-calculation-settings.max-iterations" => Some(("properties.iterativeCalculationSettings.maxIterations", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "properties.locale" => Some(("properties.locale", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "properties.spreadsheet-theme.primary-font-family" => Some(("properties.spreadsheetTheme.primaryFontFamily", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "properties.time-zone" => Some(("properties.timeZone", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "properties.title" => Some(("properties.title", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "spreadsheet-id" => Some(("spreadsheetId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "spreadsheet-url" => Some(("spreadsheetUrl", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    _ => {
                        let suggestion = FieldCursor::did_you_mean(key, &vec!["alpha", "angle", "auto-recalc", "background-color", "background-color-style", "blue", "bold", "borders", "bottom", "color", "color-style", "convergence-threshold", "default-format", "font-family", "font-size", "foreground-color", "foreground-color-style", "green", "horizontal-alignment", "hyperlink-display-type", "import-functions-external-url-access-allowed", "italic", "iterative-calculation-settings", "left", "link", "locale", "max-iterations", "number-format", "padding", "pattern", "primary-font-family", "properties", "red", "rgb-color", "right", "spreadsheet-id", "spreadsheet-theme", "spreadsheet-url", "strikethrough", "style", "text-direction", "text-format", "text-rotation", "theme-color", "time-zone", "title", "top", "type", "underline", "uri", "vertical", "vertical-alignment", "width", "wrap-strategy"]);
                        err.issues.push(CLIError::Field(FieldError::Unknown(temp_cursor.to_string(), suggestion, value.map(|v| v.to_string()))));
                        None
                    }
                };
            if let Some((field_cursor_str, type_info)) = type_info {
                FieldCursor::from(field_cursor_str).set_json_value(&mut object, value.unwrap(), type_info, err, &temp_cursor);
            }
        }
        let mut request: api::Spreadsheet = json::value::from_value(object).unwrap();
        let mut call = self.hub.spreadsheets().create(request);
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

    async fn _spreadsheets_developer_metadata_get(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        let metadata_id: i32 = arg_from_str(&opt.value_of("metadata-id").unwrap_or(""), err, "<metadata-id>", "integer");
        let mut call = self.hub.spreadsheets().developer_metadata_get(opt.value_of("spreadsheet-id").unwrap_or(""), metadata_id);
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

    async fn _spreadsheets_developer_metadata_search(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
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
        let mut request: api::SearchDeveloperMetadataRequest = json::value::from_value(object).unwrap();
        let mut call = self.hub.spreadsheets().developer_metadata_search(request, opt.value_of("spreadsheet-id").unwrap_or(""));
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

    async fn _spreadsheets_get(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        let mut call = self.hub.spreadsheets().get(opt.value_of("spreadsheet-id").unwrap_or(""));
        for parg in opt.values_of("v").map(|i|i.collect()).unwrap_or(Vec::new()).iter() {
            let (key, value) = parse_kv_arg(&*parg, err, false);
            match key {
                "ranges" => {
                    call = call.add_ranges(value.unwrap_or(""));
                },
                "include-grid-data" => {
                    call = call.include_grid_data(        value.map(|v| arg_from_str(v, err, "include-grid-data", "boolean")).unwrap_or(false));
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
                                                                           v.extend(["include-grid-data", "ranges"].iter().map(|v|*v));
                                                                           v } ));
                    }
                }
            }
        }
        let protocol = CallType::Standard;
        if dry_run {
            Ok(())
        } else {
            assert!(err.issues.len() == 0);
            for scope in self.opt.values_of("url").map(|i|i.collect()).unwrap_or(Vec::new()).iter() {
                call = call.add_scope(scope);
            }
            let mut ostream = match writer_from_opts(opt.value_of("out")) {
                Ok(mut f) => f,
                Err(io_err) => return Err(DoitError::IoError(opt.value_of("out").unwrap_or("-").to_string(), io_err)),
            };
            match match protocol {
                CallType::Standard => call.doit().await,
                _ => unreachable!()
            } {
                Err(api_err) => Err(DoitError::ApiError(api_err)),
                Ok((mut response, output_schema)) => {
                    let mut value = json::value::to_value(&output_schema).expect("serde to work");
                    remove_json_null_values(&mut value);
                    json::to_writer_pretty(&mut ostream, &value).unwrap();
                    ostream.flush().unwrap();
                    Ok(())
                }
            }
        }
    }

    async fn _spreadsheets_get_by_data_filter(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
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
                    "include-grid-data" => Some(("includeGridData", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    _ => {
                        let suggestion = FieldCursor::did_you_mean(key, &vec!["include-grid-data"]);
                        err.issues.push(CLIError::Field(FieldError::Unknown(temp_cursor.to_string(), suggestion, value.map(|v| v.to_string()))));
                        None
                    }
                };
            if let Some((field_cursor_str, type_info)) = type_info {
                FieldCursor::from(field_cursor_str).set_json_value(&mut object, value.unwrap(), type_info, err, &temp_cursor);
            }
        }
        let mut request: api::GetSpreadsheetByDataFilterRequest = json::value::from_value(object).unwrap();
        let mut call = self.hub.spreadsheets().get_by_data_filter(request, opt.value_of("spreadsheet-id").unwrap_or(""));
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

    async fn _spreadsheets_sheets_copy_to(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
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
                    "destination-spreadsheet-id" => Some(("destinationSpreadsheetId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    _ => {
                        let suggestion = FieldCursor::did_you_mean(key, &vec!["destination-spreadsheet-id"]);
                        err.issues.push(CLIError::Field(FieldError::Unknown(temp_cursor.to_string(), suggestion, value.map(|v| v.to_string()))));
                        None
                    }
                };
            if let Some((field_cursor_str, type_info)) = type_info {
                FieldCursor::from(field_cursor_str).set_json_value(&mut object, value.unwrap(), type_info, err, &temp_cursor);
            }
        }
        let mut request: api::CopySheetToAnotherSpreadsheetRequest = json::value::from_value(object).unwrap();
        let sheet_id: i32 = arg_from_str(&opt.value_of("sheet-id").unwrap_or(""), err, "<sheet-id>", "integer");
        let mut call = self.hub.spreadsheets().sheets_copy_to(request, opt.value_of("spreadsheet-id").unwrap_or(""), sheet_id);
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

    async fn _spreadsheets_values_append(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
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
                    "major-dimension" => Some(("majorDimension", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "range" => Some(("range", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    _ => {
                        let suggestion = FieldCursor::did_you_mean(key, &vec!["major-dimension", "range"]);
                        err.issues.push(CLIError::Field(FieldError::Unknown(temp_cursor.to_string(), suggestion, value.map(|v| v.to_string()))));
                        None
                    }
                };
            if let Some((field_cursor_str, type_info)) = type_info {
                FieldCursor::from(field_cursor_str).set_json_value(&mut object, value.unwrap(), type_info, err, &temp_cursor);
            }
        }
        let mut request: api::ValueRange = json::value::from_value(object).unwrap();
        let mut call = self.hub.spreadsheets().values_append(request, opt.value_of("spreadsheet-id").unwrap_or(""), opt.value_of("range").unwrap_or(""));
        for parg in opt.values_of("v").map(|i|i.collect()).unwrap_or(Vec::new()).iter() {
            let (key, value) = parse_kv_arg(&*parg, err, false);
            match key {
                "value-input-option" => {
                    call = call.value_input_option(value.unwrap_or(""));
                },
                "response-value-render-option" => {
                    call = call.response_value_render_option(value.unwrap_or(""));
                },
                "response-date-time-render-option" => {
                    call = call.response_date_time_render_option(value.unwrap_or(""));
                },
                "insert-data-option" => {
                    call = call.insert_data_option(value.unwrap_or(""));
                },
                "include-values-in-response" => {
                    call = call.include_values_in_response(        value.map(|v| arg_from_str(v, err, "include-values-in-response", "boolean")).unwrap_or(false));
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
                                                                           v.extend(["include-values-in-response", "insert-data-option", "response-date-time-render-option", "response-value-render-option", "value-input-option"].iter().map(|v|*v));
                                                                           v } ));
                    }
                }
            }
        }
        let protocol = CallType::Standard;
        if dry_run {
            Ok(())
        } else {
            assert!(err.issues.len() == 0);
            for scope in self.opt.values_of("url").map(|i|i.collect()).unwrap_or(Vec::new()).iter() {
                call = call.add_scope(scope);
            }
            let mut ostream = match writer_from_opts(opt.value_of("out")) {
                Ok(mut f) => f,
                Err(io_err) => return Err(DoitError::IoError(opt.value_of("out").unwrap_or("-").to_string(), io_err)),
            };
            match match protocol {
                CallType::Standard => call.doit().await,
                _ => unreachable!()
            } {
                Err(api_err) => Err(DoitError::ApiError(api_err)),
                Ok((mut response, output_schema)) => {
                    let mut value = json::value::to_value(&output_schema).expect("serde to work");
                    remove_json_null_values(&mut value);
                    json::to_writer_pretty(&mut ostream, &value).unwrap();
                    ostream.flush().unwrap();
                    Ok(())
                }
            }
        }
    }

    async fn _spreadsheets_values_batch_clear(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
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
                    "ranges" => Some(("ranges", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Vec })),
                    _ => {
                        let suggestion = FieldCursor::did_you_mean(key, &vec!["ranges"]);
                        err.issues.push(CLIError::Field(FieldError::Unknown(temp_cursor.to_string(), suggestion, value.map(|v| v.to_string()))));
                        None
                    }
                };
            if let Some((field_cursor_str, type_info)) = type_info {
                FieldCursor::from(field_cursor_str).set_json_value(&mut object, value.unwrap(), type_info, err, &temp_cursor);
            }
        }
        let mut request: api::BatchClearValuesRequest = json::value::from_value(object).unwrap();
        let mut call = self.hub.spreadsheets().values_batch_clear(request, opt.value_of("spreadsheet-id").unwrap_or(""));
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

    async fn _spreadsheets_values_batch_clear_by_data_filter(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
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
        let mut request: api::BatchClearValuesByDataFilterRequest = json::value::from_value(object).unwrap();
        let mut call = self.hub.spreadsheets().values_batch_clear_by_data_filter(request, opt.value_of("spreadsheet-id").unwrap_or(""));
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

    async fn _spreadsheets_values_batch_get(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        let mut call = self.hub.spreadsheets().values_batch_get(opt.value_of("spreadsheet-id").unwrap_or(""));
        for parg in opt.values_of("v").map(|i|i.collect()).unwrap_or(Vec::new()).iter() {
            let (key, value) = parse_kv_arg(&*parg, err, false);
            match key {
                "value-render-option" => {
                    call = call.value_render_option(value.unwrap_or(""));
                },
                "ranges" => {
                    call = call.add_ranges(value.unwrap_or(""));
                },
                "major-dimension" => {
                    call = call.major_dimension(value.unwrap_or(""));
                },
                "date-time-render-option" => {
                    call = call.date_time_render_option(value.unwrap_or(""));
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
                                                                           v.extend(["date-time-render-option", "major-dimension", "ranges", "value-render-option"].iter().map(|v|*v));
                                                                           v } ));
                    }
                }
            }
        }
        let protocol = CallType::Standard;
        if dry_run {
            Ok(())
        } else {
            assert!(err.issues.len() == 0);
            for scope in self.opt.values_of("url").map(|i|i.collect()).unwrap_or(Vec::new()).iter() {
                call = call.add_scope(scope);
            }
            let mut ostream = match writer_from_opts(opt.value_of("out")) {
                Ok(mut f) => f,
                Err(io_err) => return Err(DoitError::IoError(opt.value_of("out").unwrap_or("-").to_string(), io_err)),
            };
            match match protocol {
                CallType::Standard => call.doit().await,
                _ => unreachable!()
            } {
                Err(api_err) => Err(DoitError::ApiError(api_err)),
                Ok((mut response, output_schema)) => {
                    let mut value = json::value::to_value(&output_schema).expect("serde to work");
                    remove_json_null_values(&mut value);
                    json::to_writer_pretty(&mut ostream, &value).unwrap();
                    ostream.flush().unwrap();
                    Ok(())
                }
            }
        }
    }

    async fn _spreadsheets_values_batch_get_by_data_filter(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
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
                    "date-time-render-option" => Some(("dateTimeRenderOption", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "major-dimension" => Some(("majorDimension", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "value-render-option" => Some(("valueRenderOption", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    _ => {
                        let suggestion = FieldCursor::did_you_mean(key, &vec!["date-time-render-option", "major-dimension", "value-render-option"]);
                        err.issues.push(CLIError::Field(FieldError::Unknown(temp_cursor.to_string(), suggestion, value.map(|v| v.to_string()))));
                        None
                    }
                };
            if let Some((field_cursor_str, type_info)) = type_info {
                FieldCursor::from(field_cursor_str).set_json_value(&mut object, value.unwrap(), type_info, err, &temp_cursor);
            }
        }
        let mut request: api::BatchGetValuesByDataFilterRequest = json::value::from_value(object).unwrap();
        let mut call = self.hub.spreadsheets().values_batch_get_by_data_filter(request, opt.value_of("spreadsheet-id").unwrap_or(""));
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

    async fn _spreadsheets_values_batch_update(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
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
                    "include-values-in-response" => Some(("includeValuesInResponse", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "response-date-time-render-option" => Some(("responseDateTimeRenderOption", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "response-value-render-option" => Some(("responseValueRenderOption", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "value-input-option" => Some(("valueInputOption", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    _ => {
                        let suggestion = FieldCursor::did_you_mean(key, &vec!["include-values-in-response", "response-date-time-render-option", "response-value-render-option", "value-input-option"]);
                        err.issues.push(CLIError::Field(FieldError::Unknown(temp_cursor.to_string(), suggestion, value.map(|v| v.to_string()))));
                        None
                    }
                };
            if let Some((field_cursor_str, type_info)) = type_info {
                FieldCursor::from(field_cursor_str).set_json_value(&mut object, value.unwrap(), type_info, err, &temp_cursor);
            }
        }
        let mut request: api::BatchUpdateValuesRequest = json::value::from_value(object).unwrap();
        let mut call = self.hub.spreadsheets().values_batch_update(request, opt.value_of("spreadsheet-id").unwrap_or(""));
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

    async fn _spreadsheets_values_batch_update_by_data_filter(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
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
                    "include-values-in-response" => Some(("includeValuesInResponse", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "response-date-time-render-option" => Some(("responseDateTimeRenderOption", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "response-value-render-option" => Some(("responseValueRenderOption", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "value-input-option" => Some(("valueInputOption", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    _ => {
                        let suggestion = FieldCursor::did_you_mean(key, &vec!["include-values-in-response", "response-date-time-render-option", "response-value-render-option", "value-input-option"]);
                        err.issues.push(CLIError::Field(FieldError::Unknown(temp_cursor.to_string(), suggestion, value.map(|v| v.to_string()))));
                        None
                    }
                };
            if let Some((field_cursor_str, type_info)) = type_info {
                FieldCursor::from(field_cursor_str).set_json_value(&mut object, value.unwrap(), type_info, err, &temp_cursor);
            }
        }
        let mut request: api::BatchUpdateValuesByDataFilterRequest = json::value::from_value(object).unwrap();
        let mut call = self.hub.spreadsheets().values_batch_update_by_data_filter(request, opt.value_of("spreadsheet-id").unwrap_or(""));
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

    async fn _spreadsheets_values_clear(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
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
        let mut request: api::ClearValuesRequest = json::value::from_value(object).unwrap();
        let mut call = self.hub.spreadsheets().values_clear(request, opt.value_of("spreadsheet-id").unwrap_or(""), opt.value_of("range").unwrap_or(""));
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

    async fn _spreadsheets_values_get(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        let mut call = self.hub.spreadsheets().values_get(opt.value_of("spreadsheet-id").unwrap_or(""), opt.value_of("range").unwrap_or(""));
        for parg in opt.values_of("v").map(|i|i.collect()).unwrap_or(Vec::new()).iter() {
            let (key, value) = parse_kv_arg(&*parg, err, false);
            match key {
                "value-render-option" => {
                    call = call.value_render_option(value.unwrap_or(""));
                },
                "major-dimension" => {
                    call = call.major_dimension(value.unwrap_or(""));
                },
                "date-time-render-option" => {
                    call = call.date_time_render_option(value.unwrap_or(""));
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
                                                                           v.extend(["date-time-render-option", "major-dimension", "value-render-option"].iter().map(|v|*v));
                                                                           v } ));
                    }
                }
            }
        }
        let protocol = CallType::Standard;
        if dry_run {
            Ok(())
        } else {
            assert!(err.issues.len() == 0);
            for scope in self.opt.values_of("url").map(|i|i.collect()).unwrap_or(Vec::new()).iter() {
                call = call.add_scope(scope);
            }
            let mut ostream = match writer_from_opts(opt.value_of("out")) {
                Ok(mut f) => f,
                Err(io_err) => return Err(DoitError::IoError(opt.value_of("out").unwrap_or("-").to_string(), io_err)),
            };
            match match protocol {
                CallType::Standard => call.doit().await,
                _ => unreachable!()
            } {
                Err(api_err) => Err(DoitError::ApiError(api_err)),
                Ok((mut response, output_schema)) => {
                    let mut value = json::value::to_value(&output_schema).expect("serde to work");
                    remove_json_null_values(&mut value);
                    json::to_writer_pretty(&mut ostream, &value).unwrap();
                    ostream.flush().unwrap();
                    Ok(())
                }
            }
        }
    }

    async fn _spreadsheets_values_update(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
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
                    "major-dimension" => Some(("majorDimension", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "range" => Some(("range", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    _ => {
                        let suggestion = FieldCursor::did_you_mean(key, &vec!["major-dimension", "range"]);
                        err.issues.push(CLIError::Field(FieldError::Unknown(temp_cursor.to_string(), suggestion, value.map(|v| v.to_string()))));
                        None
                    }
                };
            if let Some((field_cursor_str, type_info)) = type_info {
                FieldCursor::from(field_cursor_str).set_json_value(&mut object, value.unwrap(), type_info, err, &temp_cursor);
            }
        }
        let mut request: api::ValueRange = json::value::from_value(object).unwrap();
        let mut call = self.hub.spreadsheets().values_update(request, opt.value_of("spreadsheet-id").unwrap_or(""), opt.value_of("range").unwrap_or(""));
        for parg in opt.values_of("v").map(|i|i.collect()).unwrap_or(Vec::new()).iter() {
            let (key, value) = parse_kv_arg(&*parg, err, false);
            match key {
                "value-input-option" => {
                    call = call.value_input_option(value.unwrap_or(""));
                },
                "response-value-render-option" => {
                    call = call.response_value_render_option(value.unwrap_or(""));
                },
                "response-date-time-render-option" => {
                    call = call.response_date_time_render_option(value.unwrap_or(""));
                },
                "include-values-in-response" => {
                    call = call.include_values_in_response(        value.map(|v| arg_from_str(v, err, "include-values-in-response", "boolean")).unwrap_or(false));
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
                                                                           v.extend(["include-values-in-response", "response-date-time-render-option", "response-value-render-option", "value-input-option"].iter().map(|v|*v));
                                                                           v } ));
                    }
                }
            }
        }
        let protocol = CallType::Standard;
        if dry_run {
            Ok(())
        } else {
            assert!(err.issues.len() == 0);
            for scope in self.opt.values_of("url").map(|i|i.collect()).unwrap_or(Vec::new()).iter() {
                call = call.add_scope(scope);
            }
            let mut ostream = match writer_from_opts(opt.value_of("out")) {
                Ok(mut f) => f,
                Err(io_err) => return Err(DoitError::IoError(opt.value_of("out").unwrap_or("-").to_string(), io_err)),
            };
            match match protocol {
                CallType::Standard => call.doit().await,
                _ => unreachable!()
            } {
                Err(api_err) => Err(DoitError::ApiError(api_err)),
                Ok((mut response, output_schema)) => {
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
            ("spreadsheets", Some(opt)) => {
                match opt.subcommand() {
                    ("batch-update", Some(opt)) => {
                        call_result = self._spreadsheets_batch_update(opt, dry_run, &mut err).await;
                    },
                    ("create", Some(opt)) => {
                        call_result = self._spreadsheets_create(opt, dry_run, &mut err).await;
                    },
                    ("developer-metadata-get", Some(opt)) => {
                        call_result = self._spreadsheets_developer_metadata_get(opt, dry_run, &mut err).await;
                    },
                    ("developer-metadata-search", Some(opt)) => {
                        call_result = self._spreadsheets_developer_metadata_search(opt, dry_run, &mut err).await;
                    },
                    ("get", Some(opt)) => {
                        call_result = self._spreadsheets_get(opt, dry_run, &mut err).await;
                    },
                    ("get-by-data-filter", Some(opt)) => {
                        call_result = self._spreadsheets_get_by_data_filter(opt, dry_run, &mut err).await;
                    },
                    ("sheets-copy-to", Some(opt)) => {
                        call_result = self._spreadsheets_sheets_copy_to(opt, dry_run, &mut err).await;
                    },
                    ("values-append", Some(opt)) => {
                        call_result = self._spreadsheets_values_append(opt, dry_run, &mut err).await;
                    },
                    ("values-batch-clear", Some(opt)) => {
                        call_result = self._spreadsheets_values_batch_clear(opt, dry_run, &mut err).await;
                    },
                    ("values-batch-clear-by-data-filter", Some(opt)) => {
                        call_result = self._spreadsheets_values_batch_clear_by_data_filter(opt, dry_run, &mut err).await;
                    },
                    ("values-batch-get", Some(opt)) => {
                        call_result = self._spreadsheets_values_batch_get(opt, dry_run, &mut err).await;
                    },
                    ("values-batch-get-by-data-filter", Some(opt)) => {
                        call_result = self._spreadsheets_values_batch_get_by_data_filter(opt, dry_run, &mut err).await;
                    },
                    ("values-batch-update", Some(opt)) => {
                        call_result = self._spreadsheets_values_batch_update(opt, dry_run, &mut err).await;
                    },
                    ("values-batch-update-by-data-filter", Some(opt)) => {
                        call_result = self._spreadsheets_values_batch_update_by_data_filter(opt, dry_run, &mut err).await;
                    },
                    ("values-clear", Some(opt)) => {
                        call_result = self._spreadsheets_values_clear(opt, dry_run, &mut err).await;
                    },
                    ("values-get", Some(opt)) => {
                        call_result = self._spreadsheets_values_get(opt, dry_run, &mut err).await;
                    },
                    ("values-update", Some(opt)) => {
                        call_result = self._spreadsheets_values_update(opt, dry_run, &mut err).await;
                    },
                    _ => {
                        err.issues.push(CLIError::MissingMethodError("spreadsheets".to_string()));
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

            match client::application_secret_from_directory(&config_dir, "sheets4-secret.json",
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
        ).persist_tokens_to_disk(format!("{}/sheets4", config_dir)).build().await.unwrap();

        let engine = Engine {
            opt: opt,
            hub: api::Sheets::new(client, auth),
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
        ("spreadsheets", "methods: 'batch-update', 'create', 'developer-metadata-get', 'developer-metadata-search', 'get', 'get-by-data-filter', 'sheets-copy-to', 'values-append', 'values-batch-clear', 'values-batch-clear-by-data-filter', 'values-batch-get', 'values-batch-get-by-data-filter', 'values-batch-update', 'values-batch-update-by-data-filter', 'values-clear', 'values-get' and 'values-update'", vec![
            ("batch-update",
                    Some(r##"Applies one or more updates to the spreadsheet. Each request is validated before being applied. If any request is not valid then the entire request will fail and nothing will be applied. Some requests have replies to give you some information about how they are applied. The replies will mirror the requests. For example, if you applied 4 updates and the 3rd one had a reply, then the response will have 2 empty replies, the actual reply, and another empty reply, in that order. Due to the collaborative nature of spreadsheets, it is not guaranteed that the spreadsheet will reflect exactly your changes after this completes, however it is guaranteed that the updates in the request will be applied together atomically. Your changes may be altered with respect to collaborator changes. If there are no collaborators, the spreadsheet should reflect your changes."##),
                    "Details at http://byron.github.io/google-apis-rs/google_sheets4_cli/spreadsheets_batch-update",
                  vec![
                    (Some(r##"spreadsheet-id"##),
                     None,
                     Some(r##"The spreadsheet to apply the updates to."##),
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
            ("create",
                    Some(r##"Creates a spreadsheet, returning the newly created spreadsheet."##),
                    "Details at http://byron.github.io/google-apis-rs/google_sheets4_cli/spreadsheets_create",
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
            ("developer-metadata-get",
                    Some(r##"Returns the developer metadata with the specified ID. The caller must specify the spreadsheet ID and the developer metadata's unique metadataId."##),
                    "Details at http://byron.github.io/google-apis-rs/google_sheets4_cli/spreadsheets_developer-metadata-get",
                  vec![
                    (Some(r##"spreadsheet-id"##),
                     None,
                     Some(r##"The ID of the spreadsheet to retrieve metadata from."##),
                     Some(true),
                     Some(false)),
        
                    (Some(r##"metadata-id"##),
                     None,
                     Some(r##"The ID of the developer metadata to retrieve."##),
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
            ("developer-metadata-search",
                    Some(r##"Returns all developer metadata matching the specified DataFilter. If the provided DataFilter represents a DeveloperMetadataLookup object, this will return all DeveloperMetadata entries selected by it. If the DataFilter represents a location in a spreadsheet, this will return all developer metadata associated with locations intersecting that region."##),
                    "Details at http://byron.github.io/google-apis-rs/google_sheets4_cli/spreadsheets_developer-metadata-search",
                  vec![
                    (Some(r##"spreadsheet-id"##),
                     None,
                     Some(r##"The ID of the spreadsheet to retrieve metadata from."##),
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
                    Some(r##"Returns the spreadsheet at the given ID. The caller must specify the spreadsheet ID. By default, data within grids is not returned. You can include grid data in one of 2 ways: * Specify a [field mask](https://developers.google.com/sheets/api/guides/field-masks) listing your desired fields using the `fields` URL parameter in HTTP * Set the includeGridData URL parameter to true. If a field mask is set, the `includeGridData` parameter is ignored For large spreadsheets, as a best practice, retrieve only the specific spreadsheet fields that you want. To retrieve only subsets of spreadsheet data, use the ranges URL parameter. Ranges are specified using [A1 notation](/sheets/api/guides/concepts#cell). You can define a single cell (for example, `A1`) or multiple cells (for example, `A1:D5`). You can also get cells from other sheets within the same spreadsheet (for example, `Sheet2!A1:C4`) or retrieve multiple ranges at once (for example, `?ranges=A1:D5&ranges=Sheet2!A1:C4`). Limiting the range returns only the portions of the spreadsheet that intersect the requested ranges."##),
                    "Details at http://byron.github.io/google-apis-rs/google_sheets4_cli/spreadsheets_get",
                  vec![
                    (Some(r##"spreadsheet-id"##),
                     None,
                     Some(r##"The spreadsheet to request."##),
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
            ("get-by-data-filter",
                    Some(r##"Returns the spreadsheet at the given ID. The caller must specify the spreadsheet ID. This method differs from GetSpreadsheet in that it allows selecting which subsets of spreadsheet data to return by specifying a dataFilters parameter. Multiple DataFilters can be specified. Specifying one or more data filters returns the portions of the spreadsheet that intersect ranges matched by any of the filters. By default, data within grids is not returned. You can include grid data one of 2 ways: * Specify a [field mask](https://developers.google.com/sheets/api/guides/field-masks) listing your desired fields using the `fields` URL parameter in HTTP * Set the includeGridData parameter to true. If a field mask is set, the `includeGridData` parameter is ignored For large spreadsheets, as a best practice, retrieve only the specific spreadsheet fields that you want."##),
                    "Details at http://byron.github.io/google-apis-rs/google_sheets4_cli/spreadsheets_get-by-data-filter",
                  vec![
                    (Some(r##"spreadsheet-id"##),
                     None,
                     Some(r##"The spreadsheet to request."##),
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
            ("sheets-copy-to",
                    Some(r##"Copies a single sheet from a spreadsheet to another spreadsheet. Returns the properties of the newly created sheet."##),
                    "Details at http://byron.github.io/google-apis-rs/google_sheets4_cli/spreadsheets_sheets-copy-to",
                  vec![
                    (Some(r##"spreadsheet-id"##),
                     None,
                     Some(r##"The ID of the spreadsheet containing the sheet to copy."##),
                     Some(true),
                     Some(false)),
        
                    (Some(r##"sheet-id"##),
                     None,
                     Some(r##"The ID of the sheet to copy."##),
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
            ("values-append",
                    Some(r##"Appends values to a spreadsheet. The input range is used to search for existing data and find a "table" within that range. Values will be appended to the next row of the table, starting with the first column of the table. See the [guide](/sheets/api/guides/values#appending_values) and [sample code](/sheets/api/samples/writing#append_values) for specific details of how tables are detected and data is appended. The caller must specify the spreadsheet ID, range, and a valueInputOption. The `valueInputOption` only controls how the input data will be added to the sheet (column-wise or row-wise), it does not influence what cell the data starts being written to."##),
                    "Details at http://byron.github.io/google-apis-rs/google_sheets4_cli/spreadsheets_values-append",
                  vec![
                    (Some(r##"spreadsheet-id"##),
                     None,
                     Some(r##"The ID of the spreadsheet to update."##),
                     Some(true),
                     Some(false)),
        
                    (Some(r##"range"##),
                     None,
                     Some(r##"The [A1 notation](/sheets/api/guides/concepts#cell) of a range to search for a logical table of data. Values are appended after the last row of the table."##),
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
            ("values-batch-clear",
                    Some(r##"Clears one or more ranges of values from a spreadsheet. The caller must specify the spreadsheet ID and one or more ranges. Only values are cleared -- all other properties of the cell (such as formatting and data validation) are kept."##),
                    "Details at http://byron.github.io/google-apis-rs/google_sheets4_cli/spreadsheets_values-batch-clear",
                  vec![
                    (Some(r##"spreadsheet-id"##),
                     None,
                     Some(r##"The ID of the spreadsheet to update."##),
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
            ("values-batch-clear-by-data-filter",
                    Some(r##"Clears one or more ranges of values from a spreadsheet. The caller must specify the spreadsheet ID and one or more DataFilters. Ranges matching any of the specified data filters will be cleared. Only values are cleared -- all other properties of the cell (such as formatting, data validation, etc..) are kept."##),
                    "Details at http://byron.github.io/google-apis-rs/google_sheets4_cli/spreadsheets_values-batch-clear-by-data-filter",
                  vec![
                    (Some(r##"spreadsheet-id"##),
                     None,
                     Some(r##"The ID of the spreadsheet to update."##),
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
            ("values-batch-get",
                    Some(r##"Returns one or more ranges of values from a spreadsheet. The caller must specify the spreadsheet ID and one or more ranges."##),
                    "Details at http://byron.github.io/google-apis-rs/google_sheets4_cli/spreadsheets_values-batch-get",
                  vec![
                    (Some(r##"spreadsheet-id"##),
                     None,
                     Some(r##"The ID of the spreadsheet to retrieve data from."##),
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
            ("values-batch-get-by-data-filter",
                    Some(r##"Returns one or more ranges of values that match the specified data filters. The caller must specify the spreadsheet ID and one or more DataFilters. Ranges that match any of the data filters in the request will be returned."##),
                    "Details at http://byron.github.io/google-apis-rs/google_sheets4_cli/spreadsheets_values-batch-get-by-data-filter",
                  vec![
                    (Some(r##"spreadsheet-id"##),
                     None,
                     Some(r##"The ID of the spreadsheet to retrieve data from."##),
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
            ("values-batch-update",
                    Some(r##"Sets values in one or more ranges of a spreadsheet. The caller must specify the spreadsheet ID, a valueInputOption, and one or more ValueRanges."##),
                    "Details at http://byron.github.io/google-apis-rs/google_sheets4_cli/spreadsheets_values-batch-update",
                  vec![
                    (Some(r##"spreadsheet-id"##),
                     None,
                     Some(r##"The ID of the spreadsheet to update."##),
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
            ("values-batch-update-by-data-filter",
                    Some(r##"Sets values in one or more ranges of a spreadsheet. The caller must specify the spreadsheet ID, a valueInputOption, and one or more DataFilterValueRanges."##),
                    "Details at http://byron.github.io/google-apis-rs/google_sheets4_cli/spreadsheets_values-batch-update-by-data-filter",
                  vec![
                    (Some(r##"spreadsheet-id"##),
                     None,
                     Some(r##"The ID of the spreadsheet to update."##),
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
            ("values-clear",
                    Some(r##"Clears values from a spreadsheet. The caller must specify the spreadsheet ID and range. Only values are cleared -- all other properties of the cell (such as formatting, data validation, etc..) are kept."##),
                    "Details at http://byron.github.io/google-apis-rs/google_sheets4_cli/spreadsheets_values-clear",
                  vec![
                    (Some(r##"spreadsheet-id"##),
                     None,
                     Some(r##"The ID of the spreadsheet to update."##),
                     Some(true),
                     Some(false)),
        
                    (Some(r##"range"##),
                     None,
                     Some(r##"The [A1 notation or R1C1 notation](/sheets/api/guides/concepts#cell) of the values to clear."##),
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
            ("values-get",
                    Some(r##"Returns a range of values from a spreadsheet. The caller must specify the spreadsheet ID and a range."##),
                    "Details at http://byron.github.io/google-apis-rs/google_sheets4_cli/spreadsheets_values-get",
                  vec![
                    (Some(r##"spreadsheet-id"##),
                     None,
                     Some(r##"The ID of the spreadsheet to retrieve data from."##),
                     Some(true),
                     Some(false)),
        
                    (Some(r##"range"##),
                     None,
                     Some(r##"The [A1 notation or R1C1 notation](/sheets/api/guides/concepts#cell) of the range to retrieve values from."##),
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
            ("values-update",
                    Some(r##"Sets values in a range of a spreadsheet. The caller must specify the spreadsheet ID, range, and a valueInputOption."##),
                    "Details at http://byron.github.io/google-apis-rs/google_sheets4_cli/spreadsheets_values-update",
                  vec![
                    (Some(r##"spreadsheet-id"##),
                     None,
                     Some(r##"The ID of the spreadsheet to update."##),
                     Some(true),
                     Some(false)),
        
                    (Some(r##"range"##),
                     None,
                     Some(r##"The [A1 notation](/sheets/api/guides/concepts#cell) of the values to update."##),
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
    
    let mut app = App::new("sheets4")
           .author("Sebastian Thiel <byronimo@gmail.com>")
           .version("5.0.5+20240621")
           .about("Reads and writes Google Sheets.")
           .after_help("All documentation details can be found at http://byron.github.io/google-apis-rs/google_sheets4_cli")
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
