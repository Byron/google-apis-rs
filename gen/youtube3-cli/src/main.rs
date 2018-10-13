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
extern crate google_youtube3 as api;

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
    hub: api::YouTube<hyper::Client, Authenticator<DefaultAuthenticatorDelegate, JsonTokenStorage, hyper::Client>>,
    gp: Vec<&'static str>,
    gpm: Vec<(&'static str, &'static str)>,
}


impl<'n> Engine<'n> {
    fn _activities_insert(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
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
                    "snippet.thumbnails.default.url" => Some(("snippet.thumbnails.default.url", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "snippet.thumbnails.default.width" => Some(("snippet.thumbnails.default.width", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "snippet.thumbnails.default.height" => Some(("snippet.thumbnails.default.height", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "snippet.thumbnails.high.url" => Some(("snippet.thumbnails.high.url", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "snippet.thumbnails.high.width" => Some(("snippet.thumbnails.high.width", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "snippet.thumbnails.high.height" => Some(("snippet.thumbnails.high.height", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "snippet.thumbnails.medium.url" => Some(("snippet.thumbnails.medium.url", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "snippet.thumbnails.medium.width" => Some(("snippet.thumbnails.medium.width", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "snippet.thumbnails.medium.height" => Some(("snippet.thumbnails.medium.height", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "snippet.thumbnails.maxres.url" => Some(("snippet.thumbnails.maxres.url", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "snippet.thumbnails.maxres.width" => Some(("snippet.thumbnails.maxres.width", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "snippet.thumbnails.maxres.height" => Some(("snippet.thumbnails.maxres.height", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "snippet.thumbnails.standard.url" => Some(("snippet.thumbnails.standard.url", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "snippet.thumbnails.standard.width" => Some(("snippet.thumbnails.standard.width", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "snippet.thumbnails.standard.height" => Some(("snippet.thumbnails.standard.height", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "snippet.title" => Some(("snippet.title", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "snippet.channel-id" => Some(("snippet.channelId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "snippet.published-at" => Some(("snippet.publishedAt", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "snippet.channel-title" => Some(("snippet.channelTitle", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "snippet.type" => Some(("snippet.type", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "snippet.group-id" => Some(("snippet.groupId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "snippet.description" => Some(("snippet.description", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "content-details.comment.resource-id.kind" => Some(("contentDetails.comment.resourceId.kind", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "content-details.comment.resource-id.channel-id" => Some(("contentDetails.comment.resourceId.channelId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "content-details.comment.resource-id.playlist-id" => Some(("contentDetails.comment.resourceId.playlistId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "content-details.comment.resource-id.video-id" => Some(("contentDetails.comment.resourceId.videoId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "content-details.playlist-item.resource-id.kind" => Some(("contentDetails.playlistItem.resourceId.kind", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "content-details.playlist-item.resource-id.channel-id" => Some(("contentDetails.playlistItem.resourceId.channelId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "content-details.playlist-item.resource-id.playlist-id" => Some(("contentDetails.playlistItem.resourceId.playlistId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "content-details.playlist-item.resource-id.video-id" => Some(("contentDetails.playlistItem.resourceId.videoId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "content-details.playlist-item.playlist-id" => Some(("contentDetails.playlistItem.playlistId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "content-details.playlist-item.playlist-item-id" => Some(("contentDetails.playlistItem.playlistItemId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "content-details.like.resource-id.kind" => Some(("contentDetails.like.resourceId.kind", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "content-details.like.resource-id.channel-id" => Some(("contentDetails.like.resourceId.channelId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "content-details.like.resource-id.playlist-id" => Some(("contentDetails.like.resourceId.playlistId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "content-details.like.resource-id.video-id" => Some(("contentDetails.like.resourceId.videoId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "content-details.promoted-item.cta-type" => Some(("contentDetails.promotedItem.ctaType", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "content-details.promoted-item.ad-tag" => Some(("contentDetails.promotedItem.adTag", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "content-details.promoted-item.destination-url" => Some(("contentDetails.promotedItem.destinationUrl", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "content-details.promoted-item.forecasting-url" => Some(("contentDetails.promotedItem.forecastingUrl", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Vec })),
                    "content-details.promoted-item.impression-url" => Some(("contentDetails.promotedItem.impressionUrl", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Vec })),
                    "content-details.promoted-item.creative-view-url" => Some(("contentDetails.promotedItem.creativeViewUrl", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "content-details.promoted-item.video-id" => Some(("contentDetails.promotedItem.videoId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "content-details.promoted-item.description-text" => Some(("contentDetails.promotedItem.descriptionText", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "content-details.promoted-item.custom-cta-button-text" => Some(("contentDetails.promotedItem.customCtaButtonText", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "content-details.promoted-item.click-tracking-url" => Some(("contentDetails.promotedItem.clickTrackingUrl", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "content-details.social.resource-id.kind" => Some(("contentDetails.social.resourceId.kind", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "content-details.social.resource-id.channel-id" => Some(("contentDetails.social.resourceId.channelId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "content-details.social.resource-id.playlist-id" => Some(("contentDetails.social.resourceId.playlistId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "content-details.social.resource-id.video-id" => Some(("contentDetails.social.resourceId.videoId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "content-details.social.image-url" => Some(("contentDetails.social.imageUrl", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "content-details.social.type" => Some(("contentDetails.social.type", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "content-details.social.reference-url" => Some(("contentDetails.social.referenceUrl", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "content-details.social.author" => Some(("contentDetails.social.author", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "content-details.favorite.resource-id.kind" => Some(("contentDetails.favorite.resourceId.kind", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "content-details.favorite.resource-id.channel-id" => Some(("contentDetails.favorite.resourceId.channelId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "content-details.favorite.resource-id.playlist-id" => Some(("contentDetails.favorite.resourceId.playlistId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "content-details.favorite.resource-id.video-id" => Some(("contentDetails.favorite.resourceId.videoId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "content-details.upload.video-id" => Some(("contentDetails.upload.videoId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "content-details.recommendation.resource-id.kind" => Some(("contentDetails.recommendation.resourceId.kind", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "content-details.recommendation.resource-id.channel-id" => Some(("contentDetails.recommendation.resourceId.channelId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "content-details.recommendation.resource-id.playlist-id" => Some(("contentDetails.recommendation.resourceId.playlistId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "content-details.recommendation.resource-id.video-id" => Some(("contentDetails.recommendation.resourceId.videoId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "content-details.recommendation.reason" => Some(("contentDetails.recommendation.reason", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "content-details.recommendation.seed-resource-id.kind" => Some(("contentDetails.recommendation.seedResourceId.kind", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "content-details.recommendation.seed-resource-id.channel-id" => Some(("contentDetails.recommendation.seedResourceId.channelId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "content-details.recommendation.seed-resource-id.playlist-id" => Some(("contentDetails.recommendation.seedResourceId.playlistId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "content-details.recommendation.seed-resource-id.video-id" => Some(("contentDetails.recommendation.seedResourceId.videoId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "content-details.subscription.resource-id.kind" => Some(("contentDetails.subscription.resourceId.kind", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "content-details.subscription.resource-id.channel-id" => Some(("contentDetails.subscription.resourceId.channelId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "content-details.subscription.resource-id.playlist-id" => Some(("contentDetails.subscription.resourceId.playlistId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "content-details.subscription.resource-id.video-id" => Some(("contentDetails.subscription.resourceId.videoId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "content-details.bulletin.resource-id.kind" => Some(("contentDetails.bulletin.resourceId.kind", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "content-details.bulletin.resource-id.channel-id" => Some(("contentDetails.bulletin.resourceId.channelId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "content-details.bulletin.resource-id.playlist-id" => Some(("contentDetails.bulletin.resourceId.playlistId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "content-details.bulletin.resource-id.video-id" => Some(("contentDetails.bulletin.resourceId.videoId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "content-details.channel-item.resource-id.kind" => Some(("contentDetails.channelItem.resourceId.kind", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "content-details.channel-item.resource-id.channel-id" => Some(("contentDetails.channelItem.resourceId.channelId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "content-details.channel-item.resource-id.playlist-id" => Some(("contentDetails.channelItem.resourceId.playlistId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "content-details.channel-item.resource-id.video-id" => Some(("contentDetails.channelItem.resourceId.videoId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "kind" => Some(("kind", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "etag" => Some(("etag", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "id" => Some(("id", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    _ => {
                        let suggestion = FieldCursor::did_you_mean(key, &vec!["ad-tag", "author", "bulletin", "channel-id", "channel-item", "channel-title", "click-tracking-url", "comment", "content-details", "creative-view-url", "cta-type", "custom-cta-button-text", "default", "description", "description-text", "destination-url", "etag", "favorite", "forecasting-url", "group-id", "height", "high", "id", "image-url", "impression-url", "kind", "like", "maxres", "medium", "playlist-id", "playlist-item", "playlist-item-id", "promoted-item", "published-at", "reason", "recommendation", "reference-url", "resource-id", "seed-resource-id", "snippet", "social", "standard", "subscription", "thumbnails", "title", "type", "upload", "url", "video-id", "width"]);
                        err.issues.push(CLIError::Field(FieldError::Unknown(temp_cursor.to_string(), suggestion, value.map(|v| v.to_string()))));
                        None
                    }
                };
            if let Some((field_cursor_str, type_info)) = type_info {
                FieldCursor::from(field_cursor_str).set_json_value(&mut object, value.unwrap(), type_info, err, &temp_cursor);
            }
        }
        let mut request: api::Activity = json::value::from_value(object).unwrap();
        let mut call = self.hub.activities().insert(request);
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

    fn _activities_list(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        let mut call = self.hub.activities().list(opt.value_of("part").unwrap_or(""));
        for parg in opt.values_of("v").map(|i|i.collect()).unwrap_or(Vec::new()).iter() {
            let (key, value) = parse_kv_arg(&*parg, err, false);
            match key {
                "region-code" => {
                    call = call.region_code(value.unwrap_or(""));
                },
                "published-before" => {
                    call = call.published_before(value.unwrap_or(""));
                },
                "published-after" => {
                    call = call.published_after(value.unwrap_or(""));
                },
                "page-token" => {
                    call = call.page_token(value.unwrap_or(""));
                },
                "mine" => {
                    call = call.mine(arg_from_str(value.unwrap_or("false"), err, "mine", "boolean"));
                },
                "max-results" => {
                    call = call.max_results(arg_from_str(value.unwrap_or("-0"), err, "max-results", "integer"));
                },
                "home" => {
                    call = call.home(arg_from_str(value.unwrap_or("false"), err, "home", "boolean"));
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
                                                                           v.extend(["page-token", "published-before", "channel-id", "mine", "max-results", "region-code", "home", "published-after"].iter().map(|v|*v));
                                                                           v } ));
                    }
                }
            }
        }
        let protocol = CallType::Standard;
        if dry_run {
            Ok(())
        } else {
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

    fn _captions_delete(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
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
                                                                           v.extend(["on-behalf-of-content-owner", "on-behalf-of"].iter().map(|v|*v));
                                                                           v } ));
                    }
                }
            }
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
                CallType::Standard => call.doit(),
                _ => unreachable!()
            } {
                Err(api_err) => Err(DoitError::ApiError(api_err)),
                Ok(mut response) => {
                    Ok(())
                }
            }
        }
    }

    fn _captions_download(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
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
                                                                           v.extend(["tfmt", "on-behalf-of-content-owner", "on-behalf-of", "tlang"].iter().map(|v|*v));
                                                                           v } ));
                    }
                }
            }
        }
        let protocol = CallType::Standard;
        if dry_run {
            Ok(())
        } else {
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
                Ok(mut response) => {
                    if !download_mode {
                    } else {
                    io::copy(&mut response, &mut ostream).unwrap();
                    ostream.flush().unwrap();
                    }
                    Ok(())
                }
            }
        }
    }

    fn _captions_insert(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
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
                    "snippet.status" => Some(("snippet.status", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "snippet.audio-track-type" => Some(("snippet.audioTrackType", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "snippet.language" => Some(("snippet.language", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "snippet.name" => Some(("snippet.name", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "snippet.video-id" => Some(("snippet.videoId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "snippet.is-draft" => Some(("snippet.isDraft", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "snippet.is-auto-synced" => Some(("snippet.isAutoSynced", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "snippet.track-kind" => Some(("snippet.trackKind", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "snippet.last-updated" => Some(("snippet.lastUpdated", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "snippet.is-cc" => Some(("snippet.isCC", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "snippet.is-easy-reader" => Some(("snippet.isEasyReader", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "snippet.is-large" => Some(("snippet.isLarge", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "snippet.failure-reason" => Some(("snippet.failureReason", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "kind" => Some(("kind", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "etag" => Some(("etag", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "id" => Some(("id", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
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
                    call = call.sync(arg_from_str(value.unwrap_or("false"), err, "sync", "boolean"));
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
                                                                           v.extend(["on-behalf-of-content-owner", "on-behalf-of", "sync"].iter().map(|v|*v));
                                                                           v } ));
                    }
                }
            }
        }
        let vals = opt.values_of("mode").unwrap().collect::<Vec<&str>>();
        let protocol = calltype_from_str(vals[0], ["simple", "resumable"].iter().map(|&v| v.to_string()).collect(), err);
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
                CallType::Upload(UploadProtocol::Simple) => call.upload(input_file.unwrap(), mime_type.unwrap()),
                CallType::Upload(UploadProtocol::Resumable) => call.upload_resumable(input_file.unwrap(), mime_type.unwrap()),
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

    fn _captions_list(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        let mut call = self.hub.captions().list(opt.value_of("part").unwrap_or(""), opt.value_of("video-id").unwrap_or(""));
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
                                                                           v.extend(["on-behalf-of-content-owner", "on-behalf-of", "id"].iter().map(|v|*v));
                                                                           v } ));
                    }
                }
            }
        }
        let protocol = CallType::Standard;
        if dry_run {
            Ok(())
        } else {
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

    fn _captions_update(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
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
                    "snippet.status" => Some(("snippet.status", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "snippet.audio-track-type" => Some(("snippet.audioTrackType", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "snippet.language" => Some(("snippet.language", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "snippet.name" => Some(("snippet.name", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "snippet.video-id" => Some(("snippet.videoId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "snippet.is-draft" => Some(("snippet.isDraft", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "snippet.is-auto-synced" => Some(("snippet.isAutoSynced", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "snippet.track-kind" => Some(("snippet.trackKind", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "snippet.last-updated" => Some(("snippet.lastUpdated", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "snippet.is-cc" => Some(("snippet.isCC", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "snippet.is-easy-reader" => Some(("snippet.isEasyReader", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "snippet.is-large" => Some(("snippet.isLarge", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "snippet.failure-reason" => Some(("snippet.failureReason", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "kind" => Some(("kind", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "etag" => Some(("etag", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "id" => Some(("id", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
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
                    call = call.sync(arg_from_str(value.unwrap_or("false"), err, "sync", "boolean"));
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
                                                                           v.extend(["on-behalf-of-content-owner", "on-behalf-of", "sync"].iter().map(|v|*v));
                                                                           v } ));
                    }
                }
            }
        }
        let vals = opt.values_of("mode").unwrap().collect::<Vec<&str>>();
        let protocol = calltype_from_str(vals[0], ["simple", "resumable"].iter().map(|&v| v.to_string()).collect(), err);
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
                CallType::Upload(UploadProtocol::Simple) => call.upload(input_file.unwrap(), mime_type.unwrap()),
                CallType::Upload(UploadProtocol::Resumable) => call.upload_resumable(input_file.unwrap(), mime_type.unwrap()),
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

    fn _channel_banners_insert(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
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
                    "kind" => Some(("kind", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "etag" => Some(("etag", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
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
                                                                           v.extend(["on-behalf-of-content-owner", "channel-id"].iter().map(|v|*v));
                                                                           v } ));
                    }
                }
            }
        }
        let vals = opt.values_of("mode").unwrap().collect::<Vec<&str>>();
        let protocol = calltype_from_str(vals[0], ["simple", "resumable"].iter().map(|&v| v.to_string()).collect(), err);
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
                CallType::Upload(UploadProtocol::Simple) => call.upload(input_file.unwrap(), mime_type.unwrap()),
                CallType::Upload(UploadProtocol::Resumable) => call.upload_resumable(input_file.unwrap(), mime_type.unwrap()),
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

    fn _channel_sections_delete(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
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
                CallType::Standard => call.doit(),
                _ => unreachable!()
            } {
                Err(api_err) => Err(DoitError::ApiError(api_err)),
                Ok(mut response) => {
                    Ok(())
                }
            }
        }
    }

    fn _channel_sections_insert(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
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
                    "kind" => Some(("kind", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "targeting.languages" => Some(("targeting.languages", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Vec })),
                    "targeting.regions" => Some(("targeting.regions", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Vec })),
                    "targeting.countries" => Some(("targeting.countries", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Vec })),
                    "content-details.channels" => Some(("contentDetails.channels", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Vec })),
                    "content-details.playlists" => Some(("contentDetails.playlists", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Vec })),
                    "snippet.style" => Some(("snippet.style", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "snippet.title" => Some(("snippet.title", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "snippet.channel-id" => Some(("snippet.channelId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "snippet.default-language" => Some(("snippet.defaultLanguage", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "snippet.position" => Some(("snippet.position", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "snippet.type" => Some(("snippet.type", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "snippet.localized.title" => Some(("snippet.localized.title", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "etag" => Some(("etag", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "id" => Some(("id", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
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
                                                                           v.extend(["on-behalf-of-content-owner-channel", "on-behalf-of-content-owner"].iter().map(|v|*v));
                                                                           v } ));
                    }
                }
            }
        }
        let protocol = CallType::Standard;
        if dry_run {
            Ok(())
        } else {
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

    fn _channel_sections_list(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        let mut call = self.hub.channel_sections().list(opt.value_of("part").unwrap_or(""));
        for parg in opt.values_of("v").map(|i|i.collect()).unwrap_or(Vec::new()).iter() {
            let (key, value) = parse_kv_arg(&*parg, err, false);
            match key {
                "on-behalf-of-content-owner" => {
                    call = call.on_behalf_of_content_owner(value.unwrap_or(""));
                },
                "mine" => {
                    call = call.mine(arg_from_str(value.unwrap_or("false"), err, "mine", "boolean"));
                },
                "id" => {
                    call = call.id(value.unwrap_or(""));
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
                                                                           v.extend(["on-behalf-of-content-owner", "channel-id", "mine", "hl", "id"].iter().map(|v|*v));
                                                                           v } ));
                    }
                }
            }
        }
        let protocol = CallType::Standard;
        if dry_run {
            Ok(())
        } else {
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

    fn _channel_sections_update(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
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
                    "kind" => Some(("kind", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "targeting.languages" => Some(("targeting.languages", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Vec })),
                    "targeting.regions" => Some(("targeting.regions", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Vec })),
                    "targeting.countries" => Some(("targeting.countries", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Vec })),
                    "content-details.channels" => Some(("contentDetails.channels", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Vec })),
                    "content-details.playlists" => Some(("contentDetails.playlists", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Vec })),
                    "snippet.style" => Some(("snippet.style", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "snippet.title" => Some(("snippet.title", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "snippet.channel-id" => Some(("snippet.channelId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "snippet.default-language" => Some(("snippet.defaultLanguage", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "snippet.position" => Some(("snippet.position", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "snippet.type" => Some(("snippet.type", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "snippet.localized.title" => Some(("snippet.localized.title", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "etag" => Some(("etag", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "id" => Some(("id", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
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

    fn _channels_list(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        let mut call = self.hub.channels().list(opt.value_of("part").unwrap_or(""));
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
                    call = call.my_subscribers(arg_from_str(value.unwrap_or("false"), err, "my-subscribers", "boolean"));
                },
                "mine" => {
                    call = call.mine(arg_from_str(value.unwrap_or("false"), err, "mine", "boolean"));
                },
                "max-results" => {
                    call = call.max_results(arg_from_str(value.unwrap_or("-0"), err, "max-results", "integer"));
                },
                "managed-by-me" => {
                    call = call.managed_by_me(arg_from_str(value.unwrap_or("false"), err, "managed-by-me", "boolean"));
                },
                "id" => {
                    call = call.id(value.unwrap_or(""));
                },
                "hl" => {
                    call = call.hl(value.unwrap_or(""));
                },
                "for-username" => {
                    call = call.for_username(value.unwrap_or(""));
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
                                                                           v.extend(["managed-by-me", "on-behalf-of-content-owner", "for-username", "mine", "max-results", "category-id", "page-token", "my-subscribers", "hl", "id"].iter().map(|v|*v));
                                                                           v } ));
                    }
                }
            }
        }
        let protocol = CallType::Standard;
        if dry_run {
            Ok(())
        } else {
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

    fn _channels_update(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
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
                    "status.privacy-status" => Some(("status.privacyStatus", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "status.is-linked" => Some(("status.isLinked", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "status.long-uploads-status" => Some(("status.longUploadsStatus", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "invideo-promotion.default-timing.offset-ms" => Some(("invideoPromotion.defaultTiming.offsetMs", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "invideo-promotion.default-timing.type" => Some(("invideoPromotion.defaultTiming.type", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "invideo-promotion.default-timing.duration-ms" => Some(("invideoPromotion.defaultTiming.durationMs", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "invideo-promotion.position.corner-position" => Some(("invideoPromotion.position.cornerPosition", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "invideo-promotion.position.type" => Some(("invideoPromotion.position.type", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "invideo-promotion.use-smart-timing" => Some(("invideoPromotion.useSmartTiming", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "kind" => Some(("kind", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "statistics.comment-count" => Some(("statistics.commentCount", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "statistics.subscriber-count" => Some(("statistics.subscriberCount", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "statistics.video-count" => Some(("statistics.videoCount", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "statistics.hidden-subscriber-count" => Some(("statistics.hiddenSubscriberCount", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "statistics.view-count" => Some(("statistics.viewCount", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "content-owner-details.content-owner" => Some(("contentOwnerDetails.contentOwner", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "content-owner-details.time-linked" => Some(("contentOwnerDetails.timeLinked", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "topic-details.topic-ids" => Some(("topicDetails.topicIds", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Vec })),
                    "topic-details.topic-categories" => Some(("topicDetails.topicCategories", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Vec })),
                    "content-details.related-playlists.watch-later" => Some(("contentDetails.relatedPlaylists.watchLater", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "content-details.related-playlists.watch-history" => Some(("contentDetails.relatedPlaylists.watchHistory", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "content-details.related-playlists.uploads" => Some(("contentDetails.relatedPlaylists.uploads", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "content-details.related-playlists.favorites" => Some(("contentDetails.relatedPlaylists.favorites", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "content-details.related-playlists.likes" => Some(("contentDetails.relatedPlaylists.likes", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "branding-settings.image.large-branded-banner-image-imap-script.default" => Some(("brandingSettings.image.largeBrandedBannerImageImapScript.default", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "branding-settings.image.large-branded-banner-image-imap-script.default-language.value" => Some(("brandingSettings.image.largeBrandedBannerImageImapScript.defaultLanguage.value", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "branding-settings.image.small-branded-banner-image-url.default" => Some(("brandingSettings.image.smallBrandedBannerImageUrl.default", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "branding-settings.image.small-branded-banner-image-url.default-language.value" => Some(("brandingSettings.image.smallBrandedBannerImageUrl.defaultLanguage.value", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "branding-settings.image.banner-tv-image-url" => Some(("brandingSettings.image.bannerTvImageUrl", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "branding-settings.image.banner-tv-low-image-url" => Some(("brandingSettings.image.bannerTvLowImageUrl", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "branding-settings.image.large-branded-banner-image-url.default" => Some(("brandingSettings.image.largeBrandedBannerImageUrl.default", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "branding-settings.image.large-branded-banner-image-url.default-language.value" => Some(("brandingSettings.image.largeBrandedBannerImageUrl.defaultLanguage.value", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "branding-settings.image.banner-tv-high-image-url" => Some(("brandingSettings.image.bannerTvHighImageUrl", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "branding-settings.image.background-image-url.default" => Some(("brandingSettings.image.backgroundImageUrl.default", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "branding-settings.image.background-image-url.default-language.value" => Some(("brandingSettings.image.backgroundImageUrl.defaultLanguage.value", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "branding-settings.image.small-branded-banner-image-imap-script.default" => Some(("brandingSettings.image.smallBrandedBannerImageImapScript.default", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "branding-settings.image.small-branded-banner-image-imap-script.default-language.value" => Some(("brandingSettings.image.smallBrandedBannerImageImapScript.defaultLanguage.value", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "branding-settings.image.banner-external-url" => Some(("brandingSettings.image.bannerExternalUrl", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "branding-settings.image.watch-icon-image-url" => Some(("brandingSettings.image.watchIconImageUrl", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "branding-settings.image.banner-tv-medium-image-url" => Some(("brandingSettings.image.bannerTvMediumImageUrl", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "branding-settings.image.banner-mobile-image-url" => Some(("brandingSettings.image.bannerMobileImageUrl", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "branding-settings.image.banner-tablet-hd-image-url" => Some(("brandingSettings.image.bannerTabletHdImageUrl", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "branding-settings.image.banner-tablet-low-image-url" => Some(("brandingSettings.image.bannerTabletLowImageUrl", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "branding-settings.image.tracking-image-url" => Some(("brandingSettings.image.trackingImageUrl", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "branding-settings.image.banner-mobile-extra-hd-image-url" => Some(("brandingSettings.image.bannerMobileExtraHdImageUrl", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "branding-settings.image.banner-tablet-image-url" => Some(("brandingSettings.image.bannerTabletImageUrl", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "branding-settings.image.banner-mobile-low-image-url" => Some(("brandingSettings.image.bannerMobileLowImageUrl", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "branding-settings.image.banner-mobile-medium-hd-image-url" => Some(("brandingSettings.image.bannerMobileMediumHdImageUrl", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "branding-settings.image.banner-tablet-extra-hd-image-url" => Some(("brandingSettings.image.bannerTabletExtraHdImageUrl", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "branding-settings.image.banner-image-url" => Some(("brandingSettings.image.bannerImageUrl", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "branding-settings.image.banner-mobile-hd-image-url" => Some(("brandingSettings.image.bannerMobileHdImageUrl", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "branding-settings.watch.text-color" => Some(("brandingSettings.watch.textColor", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "branding-settings.watch.featured-playlist-id" => Some(("brandingSettings.watch.featuredPlaylistId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "branding-settings.watch.background-color" => Some(("brandingSettings.watch.backgroundColor", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "branding-settings.channel.description" => Some(("brandingSettings.channel.description", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "branding-settings.channel.title" => Some(("brandingSettings.channel.title", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "branding-settings.channel.country" => Some(("brandingSettings.channel.country", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "branding-settings.channel.show-browse-view" => Some(("brandingSettings.channel.showBrowseView", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "branding-settings.channel.featured-channels-title" => Some(("brandingSettings.channel.featuredChannelsTitle", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "branding-settings.channel.default-language" => Some(("brandingSettings.channel.defaultLanguage", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "branding-settings.channel.unsubscribed-trailer" => Some(("brandingSettings.channel.unsubscribedTrailer", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "branding-settings.channel.keywords" => Some(("brandingSettings.channel.keywords", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "branding-settings.channel.profile-color" => Some(("brandingSettings.channel.profileColor", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "branding-settings.channel.default-tab" => Some(("brandingSettings.channel.defaultTab", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "branding-settings.channel.moderate-comments" => Some(("brandingSettings.channel.moderateComments", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "branding-settings.channel.featured-channels-urls" => Some(("brandingSettings.channel.featuredChannelsUrls", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Vec })),
                    "branding-settings.channel.tracking-analytics-account-id" => Some(("brandingSettings.channel.trackingAnalyticsAccountId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "branding-settings.channel.show-related-channels" => Some(("brandingSettings.channel.showRelatedChannels", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "snippet.description" => Some(("snippet.description", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "snippet.title" => Some(("snippet.title", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "snippet.country" => Some(("snippet.country", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "snippet.custom-url" => Some(("snippet.customUrl", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "snippet.published-at" => Some(("snippet.publishedAt", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "snippet.default-language" => Some(("snippet.defaultLanguage", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "snippet.localized.description" => Some(("snippet.localized.description", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "snippet.localized.title" => Some(("snippet.localized.title", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "snippet.thumbnails.default.url" => Some(("snippet.thumbnails.default.url", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "snippet.thumbnails.default.width" => Some(("snippet.thumbnails.default.width", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "snippet.thumbnails.default.height" => Some(("snippet.thumbnails.default.height", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "snippet.thumbnails.high.url" => Some(("snippet.thumbnails.high.url", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "snippet.thumbnails.high.width" => Some(("snippet.thumbnails.high.width", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "snippet.thumbnails.high.height" => Some(("snippet.thumbnails.high.height", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "snippet.thumbnails.medium.url" => Some(("snippet.thumbnails.medium.url", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "snippet.thumbnails.medium.width" => Some(("snippet.thumbnails.medium.width", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "snippet.thumbnails.medium.height" => Some(("snippet.thumbnails.medium.height", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "snippet.thumbnails.maxres.url" => Some(("snippet.thumbnails.maxres.url", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "snippet.thumbnails.maxres.width" => Some(("snippet.thumbnails.maxres.width", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "snippet.thumbnails.maxres.height" => Some(("snippet.thumbnails.maxres.height", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "snippet.thumbnails.standard.url" => Some(("snippet.thumbnails.standard.url", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "snippet.thumbnails.standard.width" => Some(("snippet.thumbnails.standard.width", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "snippet.thumbnails.standard.height" => Some(("snippet.thumbnails.standard.height", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "audit-details.community-guidelines-good-standing" => Some(("auditDetails.communityGuidelinesGoodStanding", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "audit-details.content-id-claims-good-standing" => Some(("auditDetails.contentIdClaimsGoodStanding", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "audit-details.overall-good-standing" => Some(("auditDetails.overallGoodStanding", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "audit-details.copyright-strikes-good-standing" => Some(("auditDetails.copyrightStrikesGoodStanding", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "etag" => Some(("etag", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "id" => Some(("id", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    _ => {
                        let suggestion = FieldCursor::did_you_mean(key, &vec!["audit-details", "background-color", "background-image-url", "banner-external-url", "banner-image-url", "banner-mobile-extra-hd-image-url", "banner-mobile-hd-image-url", "banner-mobile-image-url", "banner-mobile-low-image-url", "banner-mobile-medium-hd-image-url", "banner-tablet-extra-hd-image-url", "banner-tablet-hd-image-url", "banner-tablet-image-url", "banner-tablet-low-image-url", "banner-tv-high-image-url", "banner-tv-image-url", "banner-tv-low-image-url", "banner-tv-medium-image-url", "branding-settings", "channel", "comment-count", "community-guidelines-good-standing", "content-details", "content-id-claims-good-standing", "content-owner", "content-owner-details", "copyright-strikes-good-standing", "corner-position", "country", "custom-url", "default", "default-language", "default-tab", "default-timing", "description", "duration-ms", "etag", "favorites", "featured-channels-title", "featured-channels-urls", "featured-playlist-id", "height", "hidden-subscriber-count", "high", "id", "image", "invideo-promotion", "is-linked", "keywords", "kind", "large-branded-banner-image-imap-script", "large-branded-banner-image-url", "likes", "localized", "long-uploads-status", "maxres", "medium", "moderate-comments", "offset-ms", "overall-good-standing", "position", "privacy-status", "profile-color", "published-at", "related-playlists", "show-browse-view", "show-related-channels", "small-branded-banner-image-imap-script", "small-branded-banner-image-url", "snippet", "standard", "statistics", "status", "subscriber-count", "text-color", "thumbnails", "time-linked", "title", "topic-categories", "topic-details", "topic-ids", "tracking-analytics-account-id", "tracking-image-url", "type", "unsubscribed-trailer", "uploads", "url", "use-smart-timing", "value", "video-count", "view-count", "watch", "watch-history", "watch-icon-image-url", "watch-later", "width"]);
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

    fn _comment_threads_insert(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
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
                    "snippet.is-public" => Some(("snippet.isPublic", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "snippet.channel-id" => Some(("snippet.channelId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "snippet.video-id" => Some(("snippet.videoId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "snippet.can-reply" => Some(("snippet.canReply", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "snippet.total-reply-count" => Some(("snippet.totalReplyCount", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "snippet.top-level-comment.snippet.author-channel-url" => Some(("snippet.topLevelComment.snippet.authorChannelUrl", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "snippet.top-level-comment.snippet.author-display-name" => Some(("snippet.topLevelComment.snippet.authorDisplayName", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "snippet.top-level-comment.snippet.channel-id" => Some(("snippet.topLevelComment.snippet.channelId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "snippet.top-level-comment.snippet.viewer-rating" => Some(("snippet.topLevelComment.snippet.viewerRating", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "snippet.top-level-comment.snippet.moderation-status" => Some(("snippet.topLevelComment.snippet.moderationStatus", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "snippet.top-level-comment.snippet.video-id" => Some(("snippet.topLevelComment.snippet.videoId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "snippet.top-level-comment.snippet.text-original" => Some(("snippet.topLevelComment.snippet.textOriginal", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "snippet.top-level-comment.snippet.published-at" => Some(("snippet.topLevelComment.snippet.publishedAt", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "snippet.top-level-comment.snippet.parent-id" => Some(("snippet.topLevelComment.snippet.parentId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "snippet.top-level-comment.snippet.can-rate" => Some(("snippet.topLevelComment.snippet.canRate", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "snippet.top-level-comment.snippet.like-count" => Some(("snippet.topLevelComment.snippet.likeCount", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "snippet.top-level-comment.snippet.updated-at" => Some(("snippet.topLevelComment.snippet.updatedAt", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "snippet.top-level-comment.snippet.author-profile-image-url" => Some(("snippet.topLevelComment.snippet.authorProfileImageUrl", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "snippet.top-level-comment.snippet.text-display" => Some(("snippet.topLevelComment.snippet.textDisplay", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "snippet.top-level-comment.kind" => Some(("snippet.topLevelComment.kind", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "snippet.top-level-comment.etag" => Some(("snippet.topLevelComment.etag", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "snippet.top-level-comment.id" => Some(("snippet.topLevelComment.id", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "kind" => Some(("kind", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "etag" => Some(("etag", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "id" => Some(("id", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    _ => {
                        let suggestion = FieldCursor::did_you_mean(key, &vec!["author-channel-url", "author-display-name", "author-profile-image-url", "can-rate", "can-reply", "channel-id", "etag", "id", "is-public", "kind", "like-count", "moderation-status", "parent-id", "published-at", "snippet", "text-display", "text-original", "top-level-comment", "total-reply-count", "updated-at", "video-id", "viewer-rating"]);
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

    fn _comment_threads_list(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        let mut call = self.hub.comment_threads().list(opt.value_of("part").unwrap_or(""));
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
                    call = call.max_results(arg_from_str(value.unwrap_or("-0"), err, "max-results", "integer"));
                },
                "id" => {
                    call = call.id(value.unwrap_or(""));
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
                                                                           v.extend(["max-results", "all-threads-related-to-channel-id", "channel-id", "video-id", "moderation-status", "id", "page-token", "search-terms", "text-format", "order"].iter().map(|v|*v));
                                                                           v } ));
                    }
                }
            }
        }
        let protocol = CallType::Standard;
        if dry_run {
            Ok(())
        } else {
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

    fn _comment_threads_update(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
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
                    "snippet.is-public" => Some(("snippet.isPublic", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "snippet.channel-id" => Some(("snippet.channelId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "snippet.video-id" => Some(("snippet.videoId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "snippet.can-reply" => Some(("snippet.canReply", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "snippet.total-reply-count" => Some(("snippet.totalReplyCount", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "snippet.top-level-comment.snippet.author-channel-url" => Some(("snippet.topLevelComment.snippet.authorChannelUrl", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "snippet.top-level-comment.snippet.author-display-name" => Some(("snippet.topLevelComment.snippet.authorDisplayName", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "snippet.top-level-comment.snippet.channel-id" => Some(("snippet.topLevelComment.snippet.channelId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "snippet.top-level-comment.snippet.viewer-rating" => Some(("snippet.topLevelComment.snippet.viewerRating", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "snippet.top-level-comment.snippet.moderation-status" => Some(("snippet.topLevelComment.snippet.moderationStatus", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "snippet.top-level-comment.snippet.video-id" => Some(("snippet.topLevelComment.snippet.videoId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "snippet.top-level-comment.snippet.text-original" => Some(("snippet.topLevelComment.snippet.textOriginal", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "snippet.top-level-comment.snippet.published-at" => Some(("snippet.topLevelComment.snippet.publishedAt", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "snippet.top-level-comment.snippet.parent-id" => Some(("snippet.topLevelComment.snippet.parentId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "snippet.top-level-comment.snippet.can-rate" => Some(("snippet.topLevelComment.snippet.canRate", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "snippet.top-level-comment.snippet.like-count" => Some(("snippet.topLevelComment.snippet.likeCount", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "snippet.top-level-comment.snippet.updated-at" => Some(("snippet.topLevelComment.snippet.updatedAt", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "snippet.top-level-comment.snippet.author-profile-image-url" => Some(("snippet.topLevelComment.snippet.authorProfileImageUrl", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "snippet.top-level-comment.snippet.text-display" => Some(("snippet.topLevelComment.snippet.textDisplay", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "snippet.top-level-comment.kind" => Some(("snippet.topLevelComment.kind", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "snippet.top-level-comment.etag" => Some(("snippet.topLevelComment.etag", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "snippet.top-level-comment.id" => Some(("snippet.topLevelComment.id", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "kind" => Some(("kind", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "etag" => Some(("etag", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "id" => Some(("id", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    _ => {
                        let suggestion = FieldCursor::did_you_mean(key, &vec!["author-channel-url", "author-display-name", "author-profile-image-url", "can-rate", "can-reply", "channel-id", "etag", "id", "is-public", "kind", "like-count", "moderation-status", "parent-id", "published-at", "snippet", "text-display", "text-original", "top-level-comment", "total-reply-count", "updated-at", "video-id", "viewer-rating"]);
                        err.issues.push(CLIError::Field(FieldError::Unknown(temp_cursor.to_string(), suggestion, value.map(|v| v.to_string()))));
                        None
                    }
                };
            if let Some((field_cursor_str, type_info)) = type_info {
                FieldCursor::from(field_cursor_str).set_json_value(&mut object, value.unwrap(), type_info, err, &temp_cursor);
            }
        }
        let mut request: api::CommentThread = json::value::from_value(object).unwrap();
        let mut call = self.hub.comment_threads().update(request);
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

    fn _comments_delete(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
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
                CallType::Standard => call.doit(),
                _ => unreachable!()
            } {
                Err(api_err) => Err(DoitError::ApiError(api_err)),
                Ok(mut response) => {
                    Ok(())
                }
            }
        }
    }

    fn _comments_insert(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
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
                    "snippet.author-channel-url" => Some(("snippet.authorChannelUrl", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "snippet.author-display-name" => Some(("snippet.authorDisplayName", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "snippet.channel-id" => Some(("snippet.channelId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "snippet.viewer-rating" => Some(("snippet.viewerRating", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "snippet.moderation-status" => Some(("snippet.moderationStatus", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "snippet.video-id" => Some(("snippet.videoId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "snippet.text-original" => Some(("snippet.textOriginal", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "snippet.published-at" => Some(("snippet.publishedAt", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "snippet.parent-id" => Some(("snippet.parentId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "snippet.can-rate" => Some(("snippet.canRate", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "snippet.like-count" => Some(("snippet.likeCount", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "snippet.updated-at" => Some(("snippet.updatedAt", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "snippet.author-profile-image-url" => Some(("snippet.authorProfileImageUrl", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "snippet.text-display" => Some(("snippet.textDisplay", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "kind" => Some(("kind", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "etag" => Some(("etag", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "id" => Some(("id", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    _ => {
                        let suggestion = FieldCursor::did_you_mean(key, &vec!["author-channel-url", "author-display-name", "author-profile-image-url", "can-rate", "channel-id", "etag", "id", "kind", "like-count", "moderation-status", "parent-id", "published-at", "snippet", "text-display", "text-original", "updated-at", "video-id", "viewer-rating"]);
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

    fn _comments_list(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        let mut call = self.hub.comments().list(opt.value_of("part").unwrap_or(""));
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
                    call = call.max_results(arg_from_str(value.unwrap_or("-0"), err, "max-results", "integer"));
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
                                                                           v.extend(["page-token", "text-format", "id", "max-results", "parent-id"].iter().map(|v|*v));
                                                                           v } ));
                    }
                }
            }
        }
        let protocol = CallType::Standard;
        if dry_run {
            Ok(())
        } else {
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

    fn _comments_mark_as_spam(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        let mut call = self.hub.comments().mark_as_spam(opt.value_of("id").unwrap_or(""));
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
                CallType::Standard => call.doit(),
                _ => unreachable!()
            } {
                Err(api_err) => Err(DoitError::ApiError(api_err)),
                Ok(mut response) => {
                    Ok(())
                }
            }
        }
    }

    fn _comments_set_moderation_status(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        let mut call = self.hub.comments().set_moderation_status(opt.value_of("id").unwrap_or(""), opt.value_of("moderation-status").unwrap_or(""));
        for parg in opt.values_of("v").map(|i|i.collect()).unwrap_or(Vec::new()).iter() {
            let (key, value) = parse_kv_arg(&*parg, err, false);
            match key {
                "ban-author" => {
                    call = call.ban_author(arg_from_str(value.unwrap_or("false"), err, "ban-author", "boolean"));
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
                CallType::Standard => call.doit(),
                _ => unreachable!()
            } {
                Err(api_err) => Err(DoitError::ApiError(api_err)),
                Ok(mut response) => {
                    Ok(())
                }
            }
        }
    }

    fn _comments_update(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
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
                    "snippet.author-channel-url" => Some(("snippet.authorChannelUrl", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "snippet.author-display-name" => Some(("snippet.authorDisplayName", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "snippet.channel-id" => Some(("snippet.channelId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "snippet.viewer-rating" => Some(("snippet.viewerRating", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "snippet.moderation-status" => Some(("snippet.moderationStatus", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "snippet.video-id" => Some(("snippet.videoId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "snippet.text-original" => Some(("snippet.textOriginal", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "snippet.published-at" => Some(("snippet.publishedAt", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "snippet.parent-id" => Some(("snippet.parentId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "snippet.can-rate" => Some(("snippet.canRate", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "snippet.like-count" => Some(("snippet.likeCount", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "snippet.updated-at" => Some(("snippet.updatedAt", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "snippet.author-profile-image-url" => Some(("snippet.authorProfileImageUrl", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "snippet.text-display" => Some(("snippet.textDisplay", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "kind" => Some(("kind", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "etag" => Some(("etag", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "id" => Some(("id", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    _ => {
                        let suggestion = FieldCursor::did_you_mean(key, &vec!["author-channel-url", "author-display-name", "author-profile-image-url", "can-rate", "channel-id", "etag", "id", "kind", "like-count", "moderation-status", "parent-id", "published-at", "snippet", "text-display", "text-original", "updated-at", "video-id", "viewer-rating"]);
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

    fn _guide_categories_list(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        let mut call = self.hub.guide_categories().list(opt.value_of("part").unwrap_or(""));
        for parg in opt.values_of("v").map(|i|i.collect()).unwrap_or(Vec::new()).iter() {
            let (key, value) = parse_kv_arg(&*parg, err, false);
            match key {
                "region-code" => {
                    call = call.region_code(value.unwrap_or(""));
                },
                "id" => {
                    call = call.id(value.unwrap_or(""));
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
                                                                           v.extend(["region-code", "id", "hl"].iter().map(|v|*v));
                                                                           v } ));
                    }
                }
            }
        }
        let protocol = CallType::Standard;
        if dry_run {
            Ok(())
        } else {
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

    fn _i18n_languages_list(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        let mut call = self.hub.i18n_languages().list(opt.value_of("part").unwrap_or(""));
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

    fn _i18n_regions_list(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        let mut call = self.hub.i18n_regions().list(opt.value_of("part").unwrap_or(""));
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

    fn _live_broadcasts_bind(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        let mut call = self.hub.live_broadcasts().bind(opt.value_of("id").unwrap_or(""), opt.value_of("part").unwrap_or(""));
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
                                                                           v.extend(["on-behalf-of-content-owner-channel", "on-behalf-of-content-owner", "stream-id"].iter().map(|v|*v));
                                                                           v } ));
                    }
                }
            }
        }
        let protocol = CallType::Standard;
        if dry_run {
            Ok(())
        } else {
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

    fn _live_broadcasts_control(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        let mut call = self.hub.live_broadcasts().control(opt.value_of("id").unwrap_or(""), opt.value_of("part").unwrap_or(""));
        for parg in opt.values_of("v").map(|i|i.collect()).unwrap_or(Vec::new()).iter() {
            let (key, value) = parse_kv_arg(&*parg, err, false);
            match key {
                "walltime" => {
                    call = call.walltime(value.unwrap_or(""));
                },
                "on-behalf-of-content-owner-channel" => {
                    call = call.on_behalf_of_content_owner_channel(value.unwrap_or(""));
                },
                "on-behalf-of-content-owner" => {
                    call = call.on_behalf_of_content_owner(value.unwrap_or(""));
                },
                "offset-time-ms" => {
                    call = call.offset_time_ms(value.unwrap_or(""));
                },
                "display-slate" => {
                    call = call.display_slate(arg_from_str(value.unwrap_or("false"), err, "display-slate", "boolean"));
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
                                                                           v.extend(["on-behalf-of-content-owner-channel", "on-behalf-of-content-owner", "display-slate", "offset-time-ms", "walltime"].iter().map(|v|*v));
                                                                           v } ));
                    }
                }
            }
        }
        let protocol = CallType::Standard;
        if dry_run {
            Ok(())
        } else {
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

    fn _live_broadcasts_delete(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
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
                                                                           v.extend(["on-behalf-of-content-owner-channel", "on-behalf-of-content-owner"].iter().map(|v|*v));
                                                                           v } ));
                    }
                }
            }
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
                CallType::Standard => call.doit(),
                _ => unreachable!()
            } {
                Err(api_err) => Err(DoitError::ApiError(api_err)),
                Ok(mut response) => {
                    Ok(())
                }
            }
        }
    }

    fn _live_broadcasts_insert(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
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
                    "status.recording-status" => Some(("status.recordingStatus", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "status.privacy-status" => Some(("status.privacyStatus", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "status.life-cycle-status" => Some(("status.lifeCycleStatus", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "status.live-broadcast-priority" => Some(("status.liveBroadcastPriority", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "kind" => Some(("kind", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "statistics.concurrent-viewers" => Some(("statistics.concurrentViewers", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "statistics.total-chat-count" => Some(("statistics.totalChatCount", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "content-details.closed-captions-type" => Some(("contentDetails.closedCaptionsType", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "content-details.projection" => Some(("contentDetails.projection", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "content-details.start-with-slate" => Some(("contentDetails.startWithSlate", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "content-details.record-from-start" => Some(("contentDetails.recordFromStart", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "content-details.enable-auto-start" => Some(("contentDetails.enableAutoStart", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "content-details.mesh" => Some(("contentDetails.mesh", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "content-details.latency-preference" => Some(("contentDetails.latencyPreference", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "content-details.enable-embed" => Some(("contentDetails.enableEmbed", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "content-details.enable-closed-captions" => Some(("contentDetails.enableClosedCaptions", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "content-details.enable-low-latency" => Some(("contentDetails.enableLowLatency", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "content-details.stereo-layout" => Some(("contentDetails.stereoLayout", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "content-details.bound-stream-last-update-time-ms" => Some(("contentDetails.boundStreamLastUpdateTimeMs", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "content-details.enable-content-encryption" => Some(("contentDetails.enableContentEncryption", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "content-details.bound-stream-id" => Some(("contentDetails.boundStreamId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "content-details.enable-dvr" => Some(("contentDetails.enableDvr", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "content-details.monitor-stream.broadcast-stream-delay-ms" => Some(("contentDetails.monitorStream.broadcastStreamDelayMs", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "content-details.monitor-stream.embed-html" => Some(("contentDetails.monitorStream.embedHtml", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "content-details.monitor-stream.enable-monitor-stream" => Some(("contentDetails.monitorStream.enableMonitorStream", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "snippet.actual-end-time" => Some(("snippet.actualEndTime", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "snippet.description" => Some(("snippet.description", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "snippet.title" => Some(("snippet.title", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "snippet.channel-id" => Some(("snippet.channelId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "snippet.published-at" => Some(("snippet.publishedAt", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "snippet.live-chat-id" => Some(("snippet.liveChatId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "snippet.scheduled-start-time" => Some(("snippet.scheduledStartTime", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "snippet.actual-start-time" => Some(("snippet.actualStartTime", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "snippet.scheduled-end-time" => Some(("snippet.scheduledEndTime", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "snippet.is-default-broadcast" => Some(("snippet.isDefaultBroadcast", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "snippet.thumbnails.default.url" => Some(("snippet.thumbnails.default.url", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "snippet.thumbnails.default.width" => Some(("snippet.thumbnails.default.width", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "snippet.thumbnails.default.height" => Some(("snippet.thumbnails.default.height", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "snippet.thumbnails.high.url" => Some(("snippet.thumbnails.high.url", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "snippet.thumbnails.high.width" => Some(("snippet.thumbnails.high.width", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "snippet.thumbnails.high.height" => Some(("snippet.thumbnails.high.height", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "snippet.thumbnails.medium.url" => Some(("snippet.thumbnails.medium.url", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "snippet.thumbnails.medium.width" => Some(("snippet.thumbnails.medium.width", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "snippet.thumbnails.medium.height" => Some(("snippet.thumbnails.medium.height", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "snippet.thumbnails.maxres.url" => Some(("snippet.thumbnails.maxres.url", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "snippet.thumbnails.maxres.width" => Some(("snippet.thumbnails.maxres.width", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "snippet.thumbnails.maxres.height" => Some(("snippet.thumbnails.maxres.height", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "snippet.thumbnails.standard.url" => Some(("snippet.thumbnails.standard.url", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "snippet.thumbnails.standard.width" => Some(("snippet.thumbnails.standard.width", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "snippet.thumbnails.standard.height" => Some(("snippet.thumbnails.standard.height", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "etag" => Some(("etag", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "id" => Some(("id", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    _ => {
                        let suggestion = FieldCursor::did_you_mean(key, &vec!["actual-end-time", "actual-start-time", "bound-stream-id", "bound-stream-last-update-time-ms", "broadcast-stream-delay-ms", "channel-id", "closed-captions-type", "concurrent-viewers", "content-details", "default", "description", "embed-html", "enable-auto-start", "enable-closed-captions", "enable-content-encryption", "enable-dvr", "enable-embed", "enable-low-latency", "enable-monitor-stream", "etag", "height", "high", "id", "is-default-broadcast", "kind", "latency-preference", "life-cycle-status", "live-broadcast-priority", "live-chat-id", "maxres", "medium", "mesh", "monitor-stream", "privacy-status", "projection", "published-at", "record-from-start", "recording-status", "scheduled-end-time", "scheduled-start-time", "snippet", "standard", "start-with-slate", "statistics", "status", "stereo-layout", "thumbnails", "title", "total-chat-count", "url", "width"]);
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
                                                                           v.extend(["on-behalf-of-content-owner-channel", "on-behalf-of-content-owner"].iter().map(|v|*v));
                                                                           v } ));
                    }
                }
            }
        }
        let protocol = CallType::Standard;
        if dry_run {
            Ok(())
        } else {
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

    fn _live_broadcasts_list(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        let mut call = self.hub.live_broadcasts().list(opt.value_of("part").unwrap_or(""));
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
                    call = call.mine(arg_from_str(value.unwrap_or("false"), err, "mine", "boolean"));
                },
                "max-results" => {
                    call = call.max_results(arg_from_str(value.unwrap_or("-0"), err, "max-results", "integer"));
                },
                "id" => {
                    call = call.id(value.unwrap_or(""));
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
                                                                           v.extend(["broadcast-status", "on-behalf-of-content-owner", "on-behalf-of-content-owner-channel", "mine", "max-results", "page-token", "broadcast-type", "id"].iter().map(|v|*v));
                                                                           v } ));
                    }
                }
            }
        }
        let protocol = CallType::Standard;
        if dry_run {
            Ok(())
        } else {
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

    fn _live_broadcasts_transition(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        let mut call = self.hub.live_broadcasts().transition(opt.value_of("broadcast-status").unwrap_or(""), opt.value_of("id").unwrap_or(""), opt.value_of("part").unwrap_or(""));
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
                                                                           v.extend(["on-behalf-of-content-owner-channel", "on-behalf-of-content-owner"].iter().map(|v|*v));
                                                                           v } ));
                    }
                }
            }
        }
        let protocol = CallType::Standard;
        if dry_run {
            Ok(())
        } else {
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

    fn _live_broadcasts_update(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
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
                    "status.recording-status" => Some(("status.recordingStatus", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "status.privacy-status" => Some(("status.privacyStatus", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "status.life-cycle-status" => Some(("status.lifeCycleStatus", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "status.live-broadcast-priority" => Some(("status.liveBroadcastPriority", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "kind" => Some(("kind", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "statistics.concurrent-viewers" => Some(("statistics.concurrentViewers", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "statistics.total-chat-count" => Some(("statistics.totalChatCount", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "content-details.closed-captions-type" => Some(("contentDetails.closedCaptionsType", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "content-details.projection" => Some(("contentDetails.projection", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "content-details.start-with-slate" => Some(("contentDetails.startWithSlate", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "content-details.record-from-start" => Some(("contentDetails.recordFromStart", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "content-details.enable-auto-start" => Some(("contentDetails.enableAutoStart", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "content-details.mesh" => Some(("contentDetails.mesh", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "content-details.latency-preference" => Some(("contentDetails.latencyPreference", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "content-details.enable-embed" => Some(("contentDetails.enableEmbed", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "content-details.enable-closed-captions" => Some(("contentDetails.enableClosedCaptions", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "content-details.enable-low-latency" => Some(("contentDetails.enableLowLatency", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "content-details.stereo-layout" => Some(("contentDetails.stereoLayout", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "content-details.bound-stream-last-update-time-ms" => Some(("contentDetails.boundStreamLastUpdateTimeMs", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "content-details.enable-content-encryption" => Some(("contentDetails.enableContentEncryption", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "content-details.bound-stream-id" => Some(("contentDetails.boundStreamId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "content-details.enable-dvr" => Some(("contentDetails.enableDvr", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "content-details.monitor-stream.broadcast-stream-delay-ms" => Some(("contentDetails.monitorStream.broadcastStreamDelayMs", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "content-details.monitor-stream.embed-html" => Some(("contentDetails.monitorStream.embedHtml", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "content-details.monitor-stream.enable-monitor-stream" => Some(("contentDetails.monitorStream.enableMonitorStream", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "snippet.actual-end-time" => Some(("snippet.actualEndTime", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "snippet.description" => Some(("snippet.description", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "snippet.title" => Some(("snippet.title", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "snippet.channel-id" => Some(("snippet.channelId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "snippet.published-at" => Some(("snippet.publishedAt", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "snippet.live-chat-id" => Some(("snippet.liveChatId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "snippet.scheduled-start-time" => Some(("snippet.scheduledStartTime", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "snippet.actual-start-time" => Some(("snippet.actualStartTime", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "snippet.scheduled-end-time" => Some(("snippet.scheduledEndTime", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "snippet.is-default-broadcast" => Some(("snippet.isDefaultBroadcast", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "snippet.thumbnails.default.url" => Some(("snippet.thumbnails.default.url", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "snippet.thumbnails.default.width" => Some(("snippet.thumbnails.default.width", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "snippet.thumbnails.default.height" => Some(("snippet.thumbnails.default.height", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "snippet.thumbnails.high.url" => Some(("snippet.thumbnails.high.url", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "snippet.thumbnails.high.width" => Some(("snippet.thumbnails.high.width", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "snippet.thumbnails.high.height" => Some(("snippet.thumbnails.high.height", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "snippet.thumbnails.medium.url" => Some(("snippet.thumbnails.medium.url", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "snippet.thumbnails.medium.width" => Some(("snippet.thumbnails.medium.width", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "snippet.thumbnails.medium.height" => Some(("snippet.thumbnails.medium.height", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "snippet.thumbnails.maxres.url" => Some(("snippet.thumbnails.maxres.url", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "snippet.thumbnails.maxres.width" => Some(("snippet.thumbnails.maxres.width", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "snippet.thumbnails.maxres.height" => Some(("snippet.thumbnails.maxres.height", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "snippet.thumbnails.standard.url" => Some(("snippet.thumbnails.standard.url", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "snippet.thumbnails.standard.width" => Some(("snippet.thumbnails.standard.width", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "snippet.thumbnails.standard.height" => Some(("snippet.thumbnails.standard.height", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "etag" => Some(("etag", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "id" => Some(("id", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    _ => {
                        let suggestion = FieldCursor::did_you_mean(key, &vec!["actual-end-time", "actual-start-time", "bound-stream-id", "bound-stream-last-update-time-ms", "broadcast-stream-delay-ms", "channel-id", "closed-captions-type", "concurrent-viewers", "content-details", "default", "description", "embed-html", "enable-auto-start", "enable-closed-captions", "enable-content-encryption", "enable-dvr", "enable-embed", "enable-low-latency", "enable-monitor-stream", "etag", "height", "high", "id", "is-default-broadcast", "kind", "latency-preference", "life-cycle-status", "live-broadcast-priority", "live-chat-id", "maxres", "medium", "mesh", "monitor-stream", "privacy-status", "projection", "published-at", "record-from-start", "recording-status", "scheduled-end-time", "scheduled-start-time", "snippet", "standard", "start-with-slate", "statistics", "status", "stereo-layout", "thumbnails", "title", "total-chat-count", "url", "width"]);
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
                                                                           v.extend(["on-behalf-of-content-owner-channel", "on-behalf-of-content-owner"].iter().map(|v|*v));
                                                                           v } ));
                    }
                }
            }
        }
        let protocol = CallType::Standard;
        if dry_run {
            Ok(())
        } else {
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

    fn _live_chat_bans_delete(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
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
                CallType::Standard => call.doit(),
                _ => unreachable!()
            } {
                Err(api_err) => Err(DoitError::ApiError(api_err)),
                Ok(mut response) => {
                    Ok(())
                }
            }
        }
    }

    fn _live_chat_bans_insert(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
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
                    "snippet.ban-duration-seconds" => Some(("snippet.banDurationSeconds", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "snippet.live-chat-id" => Some(("snippet.liveChatId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "snippet.banned-user-details.channel-id" => Some(("snippet.bannedUserDetails.channelId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "snippet.banned-user-details.display-name" => Some(("snippet.bannedUserDetails.displayName", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "snippet.banned-user-details.profile-image-url" => Some(("snippet.bannedUserDetails.profileImageUrl", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "snippet.banned-user-details.channel-url" => Some(("snippet.bannedUserDetails.channelUrl", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "snippet.type" => Some(("snippet.type", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "kind" => Some(("kind", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "etag" => Some(("etag", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "id" => Some(("id", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
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

    fn _live_chat_messages_delete(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
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
                CallType::Standard => call.doit(),
                _ => unreachable!()
            } {
                Err(api_err) => Err(DoitError::ApiError(api_err)),
                Ok(mut response) => {
                    Ok(())
                }
            }
        }
    }

    fn _live_chat_messages_insert(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
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
                    "snippet.display-message" => Some(("snippet.displayMessage", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "snippet.message-retracted-details.retracted-message-id" => Some(("snippet.messageRetractedDetails.retractedMessageId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "snippet.super-chat-details.tier" => Some(("snippet.superChatDetails.tier", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "snippet.super-chat-details.currency" => Some(("snippet.superChatDetails.currency", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "snippet.super-chat-details.amount-display-string" => Some(("snippet.superChatDetails.amountDisplayString", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "snippet.super-chat-details.user-comment" => Some(("snippet.superChatDetails.userComment", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "snippet.super-chat-details.amount-micros" => Some(("snippet.superChatDetails.amountMicros", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "snippet.fan-funding-event-details.currency" => Some(("snippet.fanFundingEventDetails.currency", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "snippet.fan-funding-event-details.amount-display-string" => Some(("snippet.fanFundingEventDetails.amountDisplayString", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "snippet.fan-funding-event-details.user-comment" => Some(("snippet.fanFundingEventDetails.userComment", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "snippet.fan-funding-event-details.amount-micros" => Some(("snippet.fanFundingEventDetails.amountMicros", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "snippet.has-display-content" => Some(("snippet.hasDisplayContent", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "snippet.published-at" => Some(("snippet.publishedAt", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "snippet.user-banned-details.ban-duration-seconds" => Some(("snippet.userBannedDetails.banDurationSeconds", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "snippet.user-banned-details.banned-user-details.channel-id" => Some(("snippet.userBannedDetails.bannedUserDetails.channelId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "snippet.user-banned-details.banned-user-details.display-name" => Some(("snippet.userBannedDetails.bannedUserDetails.displayName", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "snippet.user-banned-details.banned-user-details.profile-image-url" => Some(("snippet.userBannedDetails.bannedUserDetails.profileImageUrl", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "snippet.user-banned-details.banned-user-details.channel-url" => Some(("snippet.userBannedDetails.bannedUserDetails.channelUrl", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "snippet.user-banned-details.ban-type" => Some(("snippet.userBannedDetails.banType", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "snippet.live-chat-id" => Some(("snippet.liveChatId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "snippet.author-channel-id" => Some(("snippet.authorChannelId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "snippet.poll-edited-details.prompt" => Some(("snippet.pollEditedDetails.prompt", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "snippet.poll-edited-details.id" => Some(("snippet.pollEditedDetails.id", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "snippet.text-message-details.message-text" => Some(("snippet.textMessageDetails.messageText", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "snippet.message-deleted-details.deleted-message-id" => Some(("snippet.messageDeletedDetails.deletedMessageId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "snippet.poll-voted-details.item-id" => Some(("snippet.pollVotedDetails.itemId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "snippet.poll-voted-details.poll-id" => Some(("snippet.pollVotedDetails.pollId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "snippet.poll-opened-details.prompt" => Some(("snippet.pollOpenedDetails.prompt", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "snippet.poll-opened-details.id" => Some(("snippet.pollOpenedDetails.id", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "snippet.type" => Some(("snippet.type", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "snippet.poll-closed-details.poll-id" => Some(("snippet.pollClosedDetails.pollId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "kind" => Some(("kind", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "etag" => Some(("etag", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "author-details.display-name" => Some(("authorDetails.displayName", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "author-details.is-chat-moderator" => Some(("authorDetails.isChatModerator", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "author-details.channel-id" => Some(("authorDetails.channelId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "author-details.is-chat-sponsor" => Some(("authorDetails.isChatSponsor", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "author-details.profile-image-url" => Some(("authorDetails.profileImageUrl", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "author-details.is-chat-owner" => Some(("authorDetails.isChatOwner", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "author-details.is-verified" => Some(("authorDetails.isVerified", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "author-details.channel-url" => Some(("authorDetails.channelUrl", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "id" => Some(("id", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    _ => {
                        let suggestion = FieldCursor::did_you_mean(key, &vec!["amount-display-string", "amount-micros", "author-channel-id", "author-details", "ban-duration-seconds", "ban-type", "banned-user-details", "channel-id", "channel-url", "currency", "deleted-message-id", "display-message", "display-name", "etag", "fan-funding-event-details", "has-display-content", "id", "is-chat-moderator", "is-chat-owner", "is-chat-sponsor", "is-verified", "item-id", "kind", "live-chat-id", "message-deleted-details", "message-retracted-details", "message-text", "poll-closed-details", "poll-edited-details", "poll-id", "poll-opened-details", "poll-voted-details", "profile-image-url", "prompt", "published-at", "retracted-message-id", "snippet", "super-chat-details", "text-message-details", "tier", "type", "user-banned-details", "user-comment"]);
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

    fn _live_chat_messages_list(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        let mut call = self.hub.live_chat_messages().list(opt.value_of("live-chat-id").unwrap_or(""), opt.value_of("part").unwrap_or(""));
        for parg in opt.values_of("v").map(|i|i.collect()).unwrap_or(Vec::new()).iter() {
            let (key, value) = parse_kv_arg(&*parg, err, false);
            match key {
                "profile-image-size" => {
                    call = call.profile_image_size(arg_from_str(value.unwrap_or("-0"), err, "profile-image-size", "integer"));
                },
                "page-token" => {
                    call = call.page_token(value.unwrap_or(""));
                },
                "max-results" => {
                    call = call.max_results(arg_from_str(value.unwrap_or("-0"), err, "max-results", "integer"));
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
                                                                           v.extend(["profile-image-size", "hl", "max-results", "page-token"].iter().map(|v|*v));
                                                                           v } ));
                    }
                }
            }
        }
        let protocol = CallType::Standard;
        if dry_run {
            Ok(())
        } else {
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

    fn _live_chat_moderators_delete(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
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
                CallType::Standard => call.doit(),
                _ => unreachable!()
            } {
                Err(api_err) => Err(DoitError::ApiError(api_err)),
                Ok(mut response) => {
                    Ok(())
                }
            }
        }
    }

    fn _live_chat_moderators_insert(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
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
                    "snippet.live-chat-id" => Some(("snippet.liveChatId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "snippet.moderator-details.channel-id" => Some(("snippet.moderatorDetails.channelId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "snippet.moderator-details.display-name" => Some(("snippet.moderatorDetails.displayName", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "snippet.moderator-details.profile-image-url" => Some(("snippet.moderatorDetails.profileImageUrl", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "snippet.moderator-details.channel-url" => Some(("snippet.moderatorDetails.channelUrl", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "kind" => Some(("kind", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "etag" => Some(("etag", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "id" => Some(("id", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
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

    fn _live_chat_moderators_list(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        let mut call = self.hub.live_chat_moderators().list(opt.value_of("live-chat-id").unwrap_or(""), opt.value_of("part").unwrap_or(""));
        for parg in opt.values_of("v").map(|i|i.collect()).unwrap_or(Vec::new()).iter() {
            let (key, value) = parse_kv_arg(&*parg, err, false);
            match key {
                "page-token" => {
                    call = call.page_token(value.unwrap_or(""));
                },
                "max-results" => {
                    call = call.max_results(arg_from_str(value.unwrap_or("-0"), err, "max-results", "integer"));
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
                                                                           v.extend(["page-token", "max-results"].iter().map(|v|*v));
                                                                           v } ));
                    }
                }
            }
        }
        let protocol = CallType::Standard;
        if dry_run {
            Ok(())
        } else {
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

    fn _live_streams_delete(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
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
                                                                           v.extend(["on-behalf-of-content-owner-channel", "on-behalf-of-content-owner"].iter().map(|v|*v));
                                                                           v } ));
                    }
                }
            }
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
                CallType::Standard => call.doit(),
                _ => unreachable!()
            } {
                Err(api_err) => Err(DoitError::ApiError(api_err)),
                Ok(mut response) => {
                    Ok(())
                }
            }
        }
    }

    fn _live_streams_insert(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
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
                    "status.stream-status" => Some(("status.streamStatus", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "status.health-status.status" => Some(("status.healthStatus.status", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "status.health-status.last-update-time-seconds" => Some(("status.healthStatus.lastUpdateTimeSeconds", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "kind" => Some(("kind", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "content-details.is-reusable" => Some(("contentDetails.isReusable", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "content-details.closed-captions-ingestion-url" => Some(("contentDetails.closedCaptionsIngestionUrl", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "cdn.frame-rate" => Some(("cdn.frameRate", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "cdn.ingestion-type" => Some(("cdn.ingestionType", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "cdn.ingestion-info.backup-ingestion-address" => Some(("cdn.ingestionInfo.backupIngestionAddress", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "cdn.ingestion-info.stream-name" => Some(("cdn.ingestionInfo.streamName", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "cdn.ingestion-info.ingestion-address" => Some(("cdn.ingestionInfo.ingestionAddress", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "cdn.resolution" => Some(("cdn.resolution", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "cdn.format" => Some(("cdn.format", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "snippet.is-default-stream" => Some(("snippet.isDefaultStream", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "snippet.channel-id" => Some(("snippet.channelId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "snippet.description" => Some(("snippet.description", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "snippet.published-at" => Some(("snippet.publishedAt", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "snippet.title" => Some(("snippet.title", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "etag" => Some(("etag", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "id" => Some(("id", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    _ => {
                        let suggestion = FieldCursor::did_you_mean(key, &vec!["backup-ingestion-address", "cdn", "channel-id", "closed-captions-ingestion-url", "content-details", "description", "etag", "format", "frame-rate", "health-status", "id", "ingestion-address", "ingestion-info", "ingestion-type", "is-default-stream", "is-reusable", "kind", "last-update-time-seconds", "published-at", "resolution", "snippet", "status", "stream-name", "stream-status", "title"]);
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
                                                                           v.extend(["on-behalf-of-content-owner-channel", "on-behalf-of-content-owner"].iter().map(|v|*v));
                                                                           v } ));
                    }
                }
            }
        }
        let protocol = CallType::Standard;
        if dry_run {
            Ok(())
        } else {
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

    fn _live_streams_list(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        let mut call = self.hub.live_streams().list(opt.value_of("part").unwrap_or(""));
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
                    call = call.mine(arg_from_str(value.unwrap_or("false"), err, "mine", "boolean"));
                },
                "max-results" => {
                    call = call.max_results(arg_from_str(value.unwrap_or("-0"), err, "max-results", "integer"));
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
                                                                           v.extend(["on-behalf-of-content-owner", "on-behalf-of-content-owner-channel", "mine", "max-results", "page-token", "id"].iter().map(|v|*v));
                                                                           v } ));
                    }
                }
            }
        }
        let protocol = CallType::Standard;
        if dry_run {
            Ok(())
        } else {
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

    fn _live_streams_update(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
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
                    "status.stream-status" => Some(("status.streamStatus", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "status.health-status.status" => Some(("status.healthStatus.status", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "status.health-status.last-update-time-seconds" => Some(("status.healthStatus.lastUpdateTimeSeconds", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "kind" => Some(("kind", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "content-details.is-reusable" => Some(("contentDetails.isReusable", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "content-details.closed-captions-ingestion-url" => Some(("contentDetails.closedCaptionsIngestionUrl", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "cdn.frame-rate" => Some(("cdn.frameRate", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "cdn.ingestion-type" => Some(("cdn.ingestionType", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "cdn.ingestion-info.backup-ingestion-address" => Some(("cdn.ingestionInfo.backupIngestionAddress", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "cdn.ingestion-info.stream-name" => Some(("cdn.ingestionInfo.streamName", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "cdn.ingestion-info.ingestion-address" => Some(("cdn.ingestionInfo.ingestionAddress", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "cdn.resolution" => Some(("cdn.resolution", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "cdn.format" => Some(("cdn.format", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "snippet.is-default-stream" => Some(("snippet.isDefaultStream", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "snippet.channel-id" => Some(("snippet.channelId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "snippet.description" => Some(("snippet.description", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "snippet.published-at" => Some(("snippet.publishedAt", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "snippet.title" => Some(("snippet.title", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "etag" => Some(("etag", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "id" => Some(("id", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    _ => {
                        let suggestion = FieldCursor::did_you_mean(key, &vec!["backup-ingestion-address", "cdn", "channel-id", "closed-captions-ingestion-url", "content-details", "description", "etag", "format", "frame-rate", "health-status", "id", "ingestion-address", "ingestion-info", "ingestion-type", "is-default-stream", "is-reusable", "kind", "last-update-time-seconds", "published-at", "resolution", "snippet", "status", "stream-name", "stream-status", "title"]);
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
                                                                           v.extend(["on-behalf-of-content-owner-channel", "on-behalf-of-content-owner"].iter().map(|v|*v));
                                                                           v } ));
                    }
                }
            }
        }
        let protocol = CallType::Standard;
        if dry_run {
            Ok(())
        } else {
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

    fn _playlist_items_delete(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
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
                CallType::Standard => call.doit(),
                _ => unreachable!()
            } {
                Err(api_err) => Err(DoitError::ApiError(api_err)),
                Ok(mut response) => {
                    Ok(())
                }
            }
        }
    }

    fn _playlist_items_insert(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
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
                    "status.privacy-status" => Some(("status.privacyStatus", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "kind" => Some(("kind", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "content-details.note" => Some(("contentDetails.note", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "content-details.video-published-at" => Some(("contentDetails.videoPublishedAt", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "content-details.start-at" => Some(("contentDetails.startAt", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "content-details.end-at" => Some(("contentDetails.endAt", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "content-details.video-id" => Some(("contentDetails.videoId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "snippet.playlist-id" => Some(("snippet.playlistId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "snippet.description" => Some(("snippet.description", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "snippet.title" => Some(("snippet.title", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "snippet.resource-id.kind" => Some(("snippet.resourceId.kind", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "snippet.resource-id.channel-id" => Some(("snippet.resourceId.channelId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "snippet.resource-id.playlist-id" => Some(("snippet.resourceId.playlistId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "snippet.resource-id.video-id" => Some(("snippet.resourceId.videoId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "snippet.channel-id" => Some(("snippet.channelId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "snippet.published-at" => Some(("snippet.publishedAt", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "snippet.channel-title" => Some(("snippet.channelTitle", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "snippet.position" => Some(("snippet.position", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "snippet.thumbnails.default.url" => Some(("snippet.thumbnails.default.url", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "snippet.thumbnails.default.width" => Some(("snippet.thumbnails.default.width", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "snippet.thumbnails.default.height" => Some(("snippet.thumbnails.default.height", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "snippet.thumbnails.high.url" => Some(("snippet.thumbnails.high.url", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "snippet.thumbnails.high.width" => Some(("snippet.thumbnails.high.width", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "snippet.thumbnails.high.height" => Some(("snippet.thumbnails.high.height", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "snippet.thumbnails.medium.url" => Some(("snippet.thumbnails.medium.url", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "snippet.thumbnails.medium.width" => Some(("snippet.thumbnails.medium.width", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "snippet.thumbnails.medium.height" => Some(("snippet.thumbnails.medium.height", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "snippet.thumbnails.maxres.url" => Some(("snippet.thumbnails.maxres.url", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "snippet.thumbnails.maxres.width" => Some(("snippet.thumbnails.maxres.width", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "snippet.thumbnails.maxres.height" => Some(("snippet.thumbnails.maxres.height", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "snippet.thumbnails.standard.url" => Some(("snippet.thumbnails.standard.url", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "snippet.thumbnails.standard.width" => Some(("snippet.thumbnails.standard.width", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "snippet.thumbnails.standard.height" => Some(("snippet.thumbnails.standard.height", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "etag" => Some(("etag", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "id" => Some(("id", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    _ => {
                        let suggestion = FieldCursor::did_you_mean(key, &vec!["channel-id", "channel-title", "content-details", "default", "description", "end-at", "etag", "height", "high", "id", "kind", "maxres", "medium", "note", "playlist-id", "position", "privacy-status", "published-at", "resource-id", "snippet", "standard", "start-at", "status", "thumbnails", "title", "url", "video-id", "video-published-at", "width"]);
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

    fn _playlist_items_list(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        let mut call = self.hub.playlist_items().list(opt.value_of("part").unwrap_or(""));
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
                    call = call.max_results(arg_from_str(value.unwrap_or("-0"), err, "max-results", "integer"));
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
                                                                           v.extend(["on-behalf-of-content-owner", "playlist-id", "video-id", "max-results", "page-token", "id"].iter().map(|v|*v));
                                                                           v } ));
                    }
                }
            }
        }
        let protocol = CallType::Standard;
        if dry_run {
            Ok(())
        } else {
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

    fn _playlist_items_update(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
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
                    "status.privacy-status" => Some(("status.privacyStatus", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "kind" => Some(("kind", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "content-details.note" => Some(("contentDetails.note", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "content-details.video-published-at" => Some(("contentDetails.videoPublishedAt", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "content-details.start-at" => Some(("contentDetails.startAt", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "content-details.end-at" => Some(("contentDetails.endAt", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "content-details.video-id" => Some(("contentDetails.videoId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "snippet.playlist-id" => Some(("snippet.playlistId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "snippet.description" => Some(("snippet.description", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "snippet.title" => Some(("snippet.title", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "snippet.resource-id.kind" => Some(("snippet.resourceId.kind", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "snippet.resource-id.channel-id" => Some(("snippet.resourceId.channelId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "snippet.resource-id.playlist-id" => Some(("snippet.resourceId.playlistId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "snippet.resource-id.video-id" => Some(("snippet.resourceId.videoId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "snippet.channel-id" => Some(("snippet.channelId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "snippet.published-at" => Some(("snippet.publishedAt", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "snippet.channel-title" => Some(("snippet.channelTitle", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "snippet.position" => Some(("snippet.position", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "snippet.thumbnails.default.url" => Some(("snippet.thumbnails.default.url", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "snippet.thumbnails.default.width" => Some(("snippet.thumbnails.default.width", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "snippet.thumbnails.default.height" => Some(("snippet.thumbnails.default.height", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "snippet.thumbnails.high.url" => Some(("snippet.thumbnails.high.url", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "snippet.thumbnails.high.width" => Some(("snippet.thumbnails.high.width", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "snippet.thumbnails.high.height" => Some(("snippet.thumbnails.high.height", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "snippet.thumbnails.medium.url" => Some(("snippet.thumbnails.medium.url", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "snippet.thumbnails.medium.width" => Some(("snippet.thumbnails.medium.width", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "snippet.thumbnails.medium.height" => Some(("snippet.thumbnails.medium.height", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "snippet.thumbnails.maxres.url" => Some(("snippet.thumbnails.maxres.url", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "snippet.thumbnails.maxres.width" => Some(("snippet.thumbnails.maxres.width", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "snippet.thumbnails.maxres.height" => Some(("snippet.thumbnails.maxres.height", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "snippet.thumbnails.standard.url" => Some(("snippet.thumbnails.standard.url", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "snippet.thumbnails.standard.width" => Some(("snippet.thumbnails.standard.width", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "snippet.thumbnails.standard.height" => Some(("snippet.thumbnails.standard.height", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "etag" => Some(("etag", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "id" => Some(("id", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    _ => {
                        let suggestion = FieldCursor::did_you_mean(key, &vec!["channel-id", "channel-title", "content-details", "default", "description", "end-at", "etag", "height", "high", "id", "kind", "maxres", "medium", "note", "playlist-id", "position", "privacy-status", "published-at", "resource-id", "snippet", "standard", "start-at", "status", "thumbnails", "title", "url", "video-id", "video-published-at", "width"]);
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

    fn _playlists_delete(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
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
                CallType::Standard => call.doit(),
                _ => unreachable!()
            } {
                Err(api_err) => Err(DoitError::ApiError(api_err)),
                Ok(mut response) => {
                    Ok(())
                }
            }
        }
    }

    fn _playlists_insert(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
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
                    "status.privacy-status" => Some(("status.privacyStatus", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "kind" => Some(("kind", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "content-details.item-count" => Some(("contentDetails.itemCount", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "snippet.description" => Some(("snippet.description", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "snippet.tags" => Some(("snippet.tags", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Vec })),
                    "snippet.channel-id" => Some(("snippet.channelId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "snippet.published-at" => Some(("snippet.publishedAt", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "snippet.channel-title" => Some(("snippet.channelTitle", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "snippet.title" => Some(("snippet.title", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "snippet.default-language" => Some(("snippet.defaultLanguage", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "snippet.localized.description" => Some(("snippet.localized.description", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "snippet.localized.title" => Some(("snippet.localized.title", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "snippet.thumbnails.default.url" => Some(("snippet.thumbnails.default.url", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "snippet.thumbnails.default.width" => Some(("snippet.thumbnails.default.width", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "snippet.thumbnails.default.height" => Some(("snippet.thumbnails.default.height", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "snippet.thumbnails.high.url" => Some(("snippet.thumbnails.high.url", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "snippet.thumbnails.high.width" => Some(("snippet.thumbnails.high.width", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "snippet.thumbnails.high.height" => Some(("snippet.thumbnails.high.height", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "snippet.thumbnails.medium.url" => Some(("snippet.thumbnails.medium.url", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "snippet.thumbnails.medium.width" => Some(("snippet.thumbnails.medium.width", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "snippet.thumbnails.medium.height" => Some(("snippet.thumbnails.medium.height", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "snippet.thumbnails.maxres.url" => Some(("snippet.thumbnails.maxres.url", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "snippet.thumbnails.maxres.width" => Some(("snippet.thumbnails.maxres.width", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "snippet.thumbnails.maxres.height" => Some(("snippet.thumbnails.maxres.height", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "snippet.thumbnails.standard.url" => Some(("snippet.thumbnails.standard.url", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "snippet.thumbnails.standard.width" => Some(("snippet.thumbnails.standard.width", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "snippet.thumbnails.standard.height" => Some(("snippet.thumbnails.standard.height", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "player.embed-html" => Some(("player.embedHtml", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "etag" => Some(("etag", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "id" => Some(("id", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    _ => {
                        let suggestion = FieldCursor::did_you_mean(key, &vec!["channel-id", "channel-title", "content-details", "default", "default-language", "description", "embed-html", "etag", "height", "high", "id", "item-count", "kind", "localized", "maxres", "medium", "player", "privacy-status", "published-at", "snippet", "standard", "status", "tags", "thumbnails", "title", "url", "width"]);
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
                                                                           v.extend(["on-behalf-of-content-owner-channel", "on-behalf-of-content-owner"].iter().map(|v|*v));
                                                                           v } ));
                    }
                }
            }
        }
        let protocol = CallType::Standard;
        if dry_run {
            Ok(())
        } else {
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

    fn _playlists_list(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        let mut call = self.hub.playlists().list(opt.value_of("part").unwrap_or(""));
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
                    call = call.mine(arg_from_str(value.unwrap_or("false"), err, "mine", "boolean"));
                },
                "max-results" => {
                    call = call.max_results(arg_from_str(value.unwrap_or("-0"), err, "max-results", "integer"));
                },
                "id" => {
                    call = call.id(value.unwrap_or(""));
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
                                                                           v.extend(["on-behalf-of-content-owner", "on-behalf-of-content-owner-channel", "channel-id", "mine", "max-results", "page-token", "hl", "id"].iter().map(|v|*v));
                                                                           v } ));
                    }
                }
            }
        }
        let protocol = CallType::Standard;
        if dry_run {
            Ok(())
        } else {
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

    fn _playlists_update(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
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
                    "status.privacy-status" => Some(("status.privacyStatus", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "kind" => Some(("kind", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "content-details.item-count" => Some(("contentDetails.itemCount", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "snippet.description" => Some(("snippet.description", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "snippet.tags" => Some(("snippet.tags", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Vec })),
                    "snippet.channel-id" => Some(("snippet.channelId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "snippet.published-at" => Some(("snippet.publishedAt", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "snippet.channel-title" => Some(("snippet.channelTitle", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "snippet.title" => Some(("snippet.title", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "snippet.default-language" => Some(("snippet.defaultLanguage", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "snippet.localized.description" => Some(("snippet.localized.description", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "snippet.localized.title" => Some(("snippet.localized.title", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "snippet.thumbnails.default.url" => Some(("snippet.thumbnails.default.url", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "snippet.thumbnails.default.width" => Some(("snippet.thumbnails.default.width", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "snippet.thumbnails.default.height" => Some(("snippet.thumbnails.default.height", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "snippet.thumbnails.high.url" => Some(("snippet.thumbnails.high.url", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "snippet.thumbnails.high.width" => Some(("snippet.thumbnails.high.width", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "snippet.thumbnails.high.height" => Some(("snippet.thumbnails.high.height", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "snippet.thumbnails.medium.url" => Some(("snippet.thumbnails.medium.url", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "snippet.thumbnails.medium.width" => Some(("snippet.thumbnails.medium.width", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "snippet.thumbnails.medium.height" => Some(("snippet.thumbnails.medium.height", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "snippet.thumbnails.maxres.url" => Some(("snippet.thumbnails.maxres.url", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "snippet.thumbnails.maxres.width" => Some(("snippet.thumbnails.maxres.width", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "snippet.thumbnails.maxres.height" => Some(("snippet.thumbnails.maxres.height", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "snippet.thumbnails.standard.url" => Some(("snippet.thumbnails.standard.url", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "snippet.thumbnails.standard.width" => Some(("snippet.thumbnails.standard.width", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "snippet.thumbnails.standard.height" => Some(("snippet.thumbnails.standard.height", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "player.embed-html" => Some(("player.embedHtml", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "etag" => Some(("etag", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "id" => Some(("id", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    _ => {
                        let suggestion = FieldCursor::did_you_mean(key, &vec!["channel-id", "channel-title", "content-details", "default", "default-language", "description", "embed-html", "etag", "height", "high", "id", "item-count", "kind", "localized", "maxres", "medium", "player", "privacy-status", "published-at", "snippet", "standard", "status", "tags", "thumbnails", "title", "url", "width"]);
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

    fn _search_list(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        let mut call = self.hub.search().list(opt.value_of("part").unwrap_or(""));
        for parg in opt.values_of("v").map(|i|i.collect()).unwrap_or(Vec::new()).iter() {
            let (key, value) = parse_kv_arg(&*parg, err, false);
            match key {
                "video-type" => {
                    call = call.video_type(value.unwrap_or(""));
                },
                "video-syndicated" => {
                    call = call.video_syndicated(value.unwrap_or(""));
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
                    call = call.type_(value.unwrap_or(""));
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
                "related-to-video-id" => {
                    call = call.related_to_video_id(value.unwrap_or(""));
                },
                "region-code" => {
                    call = call.region_code(value.unwrap_or(""));
                },
                "q" => {
                    call = call.q(value.unwrap_or(""));
                },
                "published-before" => {
                    call = call.published_before(value.unwrap_or(""));
                },
                "published-after" => {
                    call = call.published_after(value.unwrap_or(""));
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
                    call = call.max_results(arg_from_str(value.unwrap_or("-0"), err, "max-results", "integer"));
                },
                "location-radius" => {
                    call = call.location_radius(value.unwrap_or(""));
                },
                "location" => {
                    call = call.location(value.unwrap_or(""));
                },
                "for-mine" => {
                    call = call.for_mine(arg_from_str(value.unwrap_or("false"), err, "for-mine", "boolean"));
                },
                "for-developer" => {
                    call = call.for_developer(arg_from_str(value.unwrap_or("false"), err, "for-developer", "boolean"));
                },
                "for-content-owner" => {
                    call = call.for_content_owner(arg_from_str(value.unwrap_or("false"), err, "for-content-owner", "boolean"));
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
                                                                           v.extend(["location-radius", "channel-id", "video-syndicated", "event-type", "channel-type", "video-caption", "published-after", "on-behalf-of-content-owner", "video-category-id", "for-content-owner", "region-code", "location", "for-developer", "video-type", "type", "topic-id", "published-before", "video-dimension", "video-license", "max-results", "related-to-video-id", "video-definition", "page-token", "video-duration", "relevance-language", "for-mine", "q", "safe-search", "video-embeddable", "order"].iter().map(|v|*v));
                                                                           v } ));
                    }
                }
            }
        }
        let protocol = CallType::Standard;
        if dry_run {
            Ok(())
        } else {
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

    fn _sponsors_list(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        let mut call = self.hub.sponsors().list(opt.value_of("part").unwrap_or(""));
        for parg in opt.values_of("v").map(|i|i.collect()).unwrap_or(Vec::new()).iter() {
            let (key, value) = parse_kv_arg(&*parg, err, false);
            match key {
                "page-token" => {
                    call = call.page_token(value.unwrap_or(""));
                },
                "max-results" => {
                    call = call.max_results(arg_from_str(value.unwrap_or("-0"), err, "max-results", "integer"));
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
                                                                           v.extend(["filter", "page-token", "max-results"].iter().map(|v|*v));
                                                                           v } ));
                    }
                }
            }
        }
        let protocol = CallType::Standard;
        if dry_run {
            Ok(())
        } else {
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

    fn _subscriptions_delete(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
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
                CallType::Standard => call.doit(),
                _ => unreachable!()
            } {
                Err(api_err) => Err(DoitError::ApiError(api_err)),
                Ok(mut response) => {
                    Ok(())
                }
            }
        }
    }

    fn _subscriptions_insert(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
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
                    "kind" => Some(("kind", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "content-details.new-item-count" => Some(("contentDetails.newItemCount", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "content-details.activity-type" => Some(("contentDetails.activityType", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "content-details.total-item-count" => Some(("contentDetails.totalItemCount", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "snippet.description" => Some(("snippet.description", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "snippet.title" => Some(("snippet.title", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "snippet.resource-id.kind" => Some(("snippet.resourceId.kind", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "snippet.resource-id.channel-id" => Some(("snippet.resourceId.channelId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "snippet.resource-id.playlist-id" => Some(("snippet.resourceId.playlistId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "snippet.resource-id.video-id" => Some(("snippet.resourceId.videoId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "snippet.channel-id" => Some(("snippet.channelId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "snippet.published-at" => Some(("snippet.publishedAt", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "snippet.channel-title" => Some(("snippet.channelTitle", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "snippet.thumbnails.default.url" => Some(("snippet.thumbnails.default.url", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "snippet.thumbnails.default.width" => Some(("snippet.thumbnails.default.width", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "snippet.thumbnails.default.height" => Some(("snippet.thumbnails.default.height", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "snippet.thumbnails.high.url" => Some(("snippet.thumbnails.high.url", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "snippet.thumbnails.high.width" => Some(("snippet.thumbnails.high.width", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "snippet.thumbnails.high.height" => Some(("snippet.thumbnails.high.height", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "snippet.thumbnails.medium.url" => Some(("snippet.thumbnails.medium.url", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "snippet.thumbnails.medium.width" => Some(("snippet.thumbnails.medium.width", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "snippet.thumbnails.medium.height" => Some(("snippet.thumbnails.medium.height", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "snippet.thumbnails.maxres.url" => Some(("snippet.thumbnails.maxres.url", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "snippet.thumbnails.maxres.width" => Some(("snippet.thumbnails.maxres.width", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "snippet.thumbnails.maxres.height" => Some(("snippet.thumbnails.maxres.height", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "snippet.thumbnails.standard.url" => Some(("snippet.thumbnails.standard.url", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "snippet.thumbnails.standard.width" => Some(("snippet.thumbnails.standard.width", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "snippet.thumbnails.standard.height" => Some(("snippet.thumbnails.standard.height", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "etag" => Some(("etag", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "subscriber-snippet.title" => Some(("subscriberSnippet.title", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "subscriber-snippet.channel-id" => Some(("subscriberSnippet.channelId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "subscriber-snippet.description" => Some(("subscriberSnippet.description", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "subscriber-snippet.thumbnails.default.url" => Some(("subscriberSnippet.thumbnails.default.url", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "subscriber-snippet.thumbnails.default.width" => Some(("subscriberSnippet.thumbnails.default.width", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "subscriber-snippet.thumbnails.default.height" => Some(("subscriberSnippet.thumbnails.default.height", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "subscriber-snippet.thumbnails.high.url" => Some(("subscriberSnippet.thumbnails.high.url", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "subscriber-snippet.thumbnails.high.width" => Some(("subscriberSnippet.thumbnails.high.width", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "subscriber-snippet.thumbnails.high.height" => Some(("subscriberSnippet.thumbnails.high.height", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "subscriber-snippet.thumbnails.medium.url" => Some(("subscriberSnippet.thumbnails.medium.url", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "subscriber-snippet.thumbnails.medium.width" => Some(("subscriberSnippet.thumbnails.medium.width", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "subscriber-snippet.thumbnails.medium.height" => Some(("subscriberSnippet.thumbnails.medium.height", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "subscriber-snippet.thumbnails.maxres.url" => Some(("subscriberSnippet.thumbnails.maxres.url", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "subscriber-snippet.thumbnails.maxres.width" => Some(("subscriberSnippet.thumbnails.maxres.width", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "subscriber-snippet.thumbnails.maxres.height" => Some(("subscriberSnippet.thumbnails.maxres.height", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "subscriber-snippet.thumbnails.standard.url" => Some(("subscriberSnippet.thumbnails.standard.url", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "subscriber-snippet.thumbnails.standard.width" => Some(("subscriberSnippet.thumbnails.standard.width", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "subscriber-snippet.thumbnails.standard.height" => Some(("subscriberSnippet.thumbnails.standard.height", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "id" => Some(("id", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
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

    fn _subscriptions_list(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        let mut call = self.hub.subscriptions().list(opt.value_of("part").unwrap_or(""));
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
                    call = call.my_subscribers(arg_from_str(value.unwrap_or("false"), err, "my-subscribers", "boolean"));
                },
                "my-recent-subscribers" => {
                    call = call.my_recent_subscribers(arg_from_str(value.unwrap_or("false"), err, "my-recent-subscribers", "boolean"));
                },
                "mine" => {
                    call = call.mine(arg_from_str(value.unwrap_or("false"), err, "mine", "boolean"));
                },
                "max-results" => {
                    call = call.max_results(arg_from_str(value.unwrap_or("-0"), err, "max-results", "integer"));
                },
                "id" => {
                    call = call.id(value.unwrap_or(""));
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
                                                                           v.extend(["on-behalf-of-content-owner", "on-behalf-of-content-owner-channel", "channel-id", "mine", "max-results", "id", "page-token", "my-subscribers", "for-channel-id", "order", "my-recent-subscribers"].iter().map(|v|*v));
                                                                           v } ));
                    }
                }
            }
        }
        let protocol = CallType::Standard;
        if dry_run {
            Ok(())
        } else {
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

    fn _super_chat_events_list(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        let mut call = self.hub.super_chat_events().list(opt.value_of("part").unwrap_or(""));
        for parg in opt.values_of("v").map(|i|i.collect()).unwrap_or(Vec::new()).iter() {
            let (key, value) = parse_kv_arg(&*parg, err, false);
            match key {
                "page-token" => {
                    call = call.page_token(value.unwrap_or(""));
                },
                "max-results" => {
                    call = call.max_results(arg_from_str(value.unwrap_or("-0"), err, "max-results", "integer"));
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
                                                                           v.extend(["page-token", "hl", "max-results"].iter().map(|v|*v));
                                                                           v } ));
                    }
                }
            }
        }
        let protocol = CallType::Standard;
        if dry_run {
            Ok(())
        } else {
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

    fn _thumbnails_set(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
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
        let protocol = calltype_from_str(vals[0], ["simple", "resumable"].iter().map(|&v| v.to_string()).collect(), err);
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
                CallType::Upload(UploadProtocol::Simple) => call.upload(input_file.unwrap(), mime_type.unwrap()),
                CallType::Upload(UploadProtocol::Resumable) => call.upload_resumable(input_file.unwrap(), mime_type.unwrap()),
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

    fn _video_abuse_report_reasons_list(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        let mut call = self.hub.video_abuse_report_reasons().list(opt.value_of("part").unwrap_or(""));
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

    fn _video_categories_list(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        let mut call = self.hub.video_categories().list(opt.value_of("part").unwrap_or(""));
        for parg in opt.values_of("v").map(|i|i.collect()).unwrap_or(Vec::new()).iter() {
            let (key, value) = parse_kv_arg(&*parg, err, false);
            match key {
                "region-code" => {
                    call = call.region_code(value.unwrap_or(""));
                },
                "id" => {
                    call = call.id(value.unwrap_or(""));
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
                                                                           v.extend(["region-code", "id", "hl"].iter().map(|v|*v));
                                                                           v } ));
                    }
                }
            }
        }
        let protocol = CallType::Standard;
        if dry_run {
            Ok(())
        } else {
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

    fn _videos_delete(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
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
                CallType::Standard => call.doit(),
                _ => unreachable!()
            } {
                Err(api_err) => Err(DoitError::ApiError(api_err)),
                Ok(mut response) => {
                    Ok(())
                }
            }
        }
    }

    fn _videos_get_rating(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        let mut call = self.hub.videos().get_rating(opt.value_of("id").unwrap_or(""));
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

    fn _videos_insert(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
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
                    "status.license" => Some(("status.license", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "status.embeddable" => Some(("status.embeddable", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "status.privacy-status" => Some(("status.privacyStatus", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "status.publish-at" => Some(("status.publishAt", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "status.public-stats-viewable" => Some(("status.publicStatsViewable", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "status.upload-status" => Some(("status.uploadStatus", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "status.rejection-reason" => Some(("status.rejectionReason", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "status.failure-reason" => Some(("status.failureReason", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "topic-details.topic-ids" => Some(("topicDetails.topicIds", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Vec })),
                    "topic-details.topic-categories" => Some(("topicDetails.topicCategories", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Vec })),
                    "topic-details.relevant-topic-ids" => Some(("topicDetails.relevantTopicIds", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Vec })),
                    "kind" => Some(("kind", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "statistics.comment-count" => Some(("statistics.commentCount", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "statistics.view-count" => Some(("statistics.viewCount", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "statistics.favorite-count" => Some(("statistics.favoriteCount", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "statistics.dislike-count" => Some(("statistics.dislikeCount", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "statistics.like-count" => Some(("statistics.likeCount", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "content-details.definition" => Some(("contentDetails.definition", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "content-details.projection" => Some(("contentDetails.projection", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "content-details.country-restriction.exception" => Some(("contentDetails.countryRestriction.exception", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Vec })),
                    "content-details.country-restriction.allowed" => Some(("contentDetails.countryRestriction.allowed", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "content-details.has-custom-thumbnail" => Some(("contentDetails.hasCustomThumbnail", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "content-details.content-rating.yt-rating" => Some(("contentDetails.contentRating.ytRating", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "content-details.content-rating.catvfr-rating" => Some(("contentDetails.contentRating.catvfrRating", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "content-details.content-rating.cbfc-rating" => Some(("contentDetails.contentRating.cbfcRating", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "content-details.content-rating.bfvc-rating" => Some(("contentDetails.contentRating.bfvcRating", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "content-details.content-rating.mda-rating" => Some(("contentDetails.contentRating.mdaRating", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "content-details.content-rating.acb-rating" => Some(("contentDetails.contentRating.acbRating", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "content-details.content-rating.nfvcb-rating" => Some(("contentDetails.contentRating.nfvcbRating", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "content-details.content-rating.bmukk-rating" => Some(("contentDetails.contentRating.bmukkRating", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "content-details.content-rating.chfilm-rating" => Some(("contentDetails.contentRating.chfilmRating", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "content-details.content-rating.mena-mpaa-rating" => Some(("contentDetails.contentRating.menaMpaaRating", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "content-details.content-rating.csa-rating" => Some(("contentDetails.contentRating.csaRating", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "content-details.content-rating.moctw-rating" => Some(("contentDetails.contentRating.moctwRating", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "content-details.content-rating.anatel-rating" => Some(("contentDetails.contentRating.anatelRating", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "content-details.content-rating.catv-rating" => Some(("contentDetails.contentRating.catvRating", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "content-details.content-rating.pefilm-rating" => Some(("contentDetails.contentRating.pefilmRating", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "content-details.content-rating.djctq-rating-reasons" => Some(("contentDetails.contentRating.djctqRatingReasons", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Vec })),
                    "content-details.content-rating.incaa-rating" => Some(("contentDetails.contentRating.incaaRating", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "content-details.content-rating.cnc-rating" => Some(("contentDetails.contentRating.cncRating", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "content-details.content-rating.oflc-rating" => Some(("contentDetails.contentRating.oflcRating", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "content-details.content-rating.fpb-rating" => Some(("contentDetails.contentRating.fpbRating", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "content-details.content-rating.mccaa-rating" => Some(("contentDetails.contentRating.mccaaRating", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "content-details.content-rating.tvpg-rating" => Some(("contentDetails.contentRating.tvpgRating", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "content-details.content-rating.rtc-rating" => Some(("contentDetails.contentRating.rtcRating", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "content-details.content-rating.cscf-rating" => Some(("contentDetails.contentRating.cscfRating", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "content-details.content-rating.fsk-rating" => Some(("contentDetails.contentRating.fskRating", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "content-details.content-rating.bbfc-rating" => Some(("contentDetails.contentRating.bbfcRating", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "content-details.content-rating.kmrb-rating" => Some(("contentDetails.contentRating.kmrbRating", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "content-details.content-rating.smsa-rating" => Some(("contentDetails.contentRating.smsaRating", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "content-details.content-rating.egfilm-rating" => Some(("contentDetails.contentRating.egfilmRating", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "content-details.content-rating.cicf-rating" => Some(("contentDetails.contentRating.cicfRating", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "content-details.content-rating.nbcpl-rating" => Some(("contentDetails.contentRating.nbcplRating", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "content-details.content-rating.nbc-rating" => Some(("contentDetails.contentRating.nbcRating", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "content-details.content-rating.djctq-rating" => Some(("contentDetails.contentRating.djctqRating", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "content-details.content-rating.ifco-rating" => Some(("contentDetails.contentRating.ifcoRating", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "content-details.content-rating.mpaat-rating" => Some(("contentDetails.contentRating.mpaatRating", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "content-details.content-rating.fco-rating" => Some(("contentDetails.contentRating.fcoRating", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "content-details.content-rating.eefilm-rating" => Some(("contentDetails.contentRating.eefilmRating", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "content-details.content-rating.medietilsynet-rating" => Some(("contentDetails.contentRating.medietilsynetRating", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "content-details.content-rating.grfilm-rating" => Some(("contentDetails.contentRating.grfilmRating", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "content-details.content-rating.ccc-rating" => Some(("contentDetails.contentRating.cccRating", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "content-details.content-rating.rte-rating" => Some(("contentDetails.contentRating.rteRating", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "content-details.content-rating.czfilm-rating" => Some(("contentDetails.contentRating.czfilmRating", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "content-details.content-rating.ecbmct-rating" => Some(("contentDetails.contentRating.ecbmctRating", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "content-details.content-rating.fmoc-rating" => Some(("contentDetails.contentRating.fmocRating", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "content-details.content-rating.eirin-rating" => Some(("contentDetails.contentRating.eirinRating", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "content-details.content-rating.lsf-rating" => Some(("contentDetails.contentRating.lsfRating", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "content-details.content-rating.cce-rating" => Some(("contentDetails.contentRating.cceRating", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "content-details.content-rating.nkclv-rating" => Some(("contentDetails.contentRating.nkclvRating", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "content-details.content-rating.mtrcb-rating" => Some(("contentDetails.contentRating.mtrcbRating", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "content-details.content-rating.mibac-rating" => Some(("contentDetails.contentRating.mibacRating", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "content-details.content-rating.ilfilm-rating" => Some(("contentDetails.contentRating.ilfilmRating", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "content-details.content-rating.mcst-rating" => Some(("contentDetails.contentRating.mcstRating", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "content-details.content-rating.smais-rating" => Some(("contentDetails.contentRating.smaisRating", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "content-details.content-rating.russia-rating" => Some(("contentDetails.contentRating.russiaRating", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "content-details.content-rating.mpaa-rating" => Some(("contentDetails.contentRating.mpaaRating", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "content-details.content-rating.kfcb-rating" => Some(("contentDetails.contentRating.kfcbRating", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "content-details.content-rating.fpb-rating-reasons" => Some(("contentDetails.contentRating.fpbRatingReasons", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Vec })),
                    "content-details.content-rating.agcom-rating" => Some(("contentDetails.contentRating.agcomRating", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "content-details.content-rating.chvrs-rating" => Some(("contentDetails.contentRating.chvrsRating", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "content-details.content-rating.cna-rating" => Some(("contentDetails.contentRating.cnaRating", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "content-details.content-rating.icaa-rating" => Some(("contentDetails.contentRating.icaaRating", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "content-details.content-rating.mccyp-rating" => Some(("contentDetails.contentRating.mccypRating", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "content-details.content-rating.nfrc-rating" => Some(("contentDetails.contentRating.nfrcRating", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "content-details.content-rating.skfilm-rating" => Some(("contentDetails.contentRating.skfilmRating", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "content-details.content-rating.moc-rating" => Some(("contentDetails.contentRating.mocRating", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "content-details.content-rating.rcnof-rating" => Some(("contentDetails.contentRating.rcnofRating", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "content-details.content-rating.meku-rating" => Some(("contentDetails.contentRating.mekuRating", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "content-details.content-rating.resorteviolencia-rating" => Some(("contentDetails.contentRating.resorteviolenciaRating", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "content-details.content-rating.fcbm-rating" => Some(("contentDetails.contentRating.fcbmRating", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "content-details.content-rating.kijkwijzer-rating" => Some(("contentDetails.contentRating.kijkwijzerRating", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "content-details.caption" => Some(("contentDetails.caption", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "content-details.region-restriction.blocked" => Some(("contentDetails.regionRestriction.blocked", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Vec })),
                    "content-details.region-restriction.allowed" => Some(("contentDetails.regionRestriction.allowed", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Vec })),
                    "content-details.duration" => Some(("contentDetails.duration", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "content-details.licensed-content" => Some(("contentDetails.licensedContent", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "content-details.dimension" => Some(("contentDetails.dimension", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "monetization-details.access.exception" => Some(("monetizationDetails.access.exception", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Vec })),
                    "monetization-details.access.allowed" => Some(("monetizationDetails.access.allowed", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "age-gating.restricted" => Some(("ageGating.restricted", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "age-gating.alcohol-content" => Some(("ageGating.alcoholContent", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "age-gating.video-game-rating" => Some(("ageGating.videoGameRating", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "suggestions.processing-errors" => Some(("suggestions.processingErrors", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Vec })),
                    "suggestions.editor-suggestions" => Some(("suggestions.editorSuggestions", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Vec })),
                    "suggestions.processing-warnings" => Some(("suggestions.processingWarnings", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Vec })),
                    "suggestions.processing-hints" => Some(("suggestions.processingHints", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Vec })),
                    "live-streaming-details.actual-end-time" => Some(("liveStreamingDetails.actualEndTime", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "live-streaming-details.active-live-chat-id" => Some(("liveStreamingDetails.activeLiveChatId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "live-streaming-details.scheduled-start-time" => Some(("liveStreamingDetails.scheduledStartTime", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "live-streaming-details.concurrent-viewers" => Some(("liveStreamingDetails.concurrentViewers", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "live-streaming-details.actual-start-time" => Some(("liveStreamingDetails.actualStartTime", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "live-streaming-details.scheduled-end-time" => Some(("liveStreamingDetails.scheduledEndTime", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "file-details.bitrate-bps" => Some(("fileDetails.bitrateBps", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "file-details.container" => Some(("fileDetails.container", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "file-details.file-type" => Some(("fileDetails.fileType", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "file-details.creation-time" => Some(("fileDetails.creationTime", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "file-details.duration-ms" => Some(("fileDetails.durationMs", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "file-details.file-name" => Some(("fileDetails.fileName", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "file-details.file-size" => Some(("fileDetails.fileSize", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "snippet.description" => Some(("snippet.description", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "snippet.tags" => Some(("snippet.tags", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Vec })),
                    "snippet.default-audio-language" => Some(("snippet.defaultAudioLanguage", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "snippet.channel-id" => Some(("snippet.channelId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "snippet.published-at" => Some(("snippet.publishedAt", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "snippet.live-broadcast-content" => Some(("snippet.liveBroadcastContent", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "snippet.default-language" => Some(("snippet.defaultLanguage", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "snippet.channel-title" => Some(("snippet.channelTitle", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "snippet.title" => Some(("snippet.title", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "snippet.category-id" => Some(("snippet.categoryId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "snippet.localized.description" => Some(("snippet.localized.description", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "snippet.localized.title" => Some(("snippet.localized.title", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "snippet.thumbnails.default.url" => Some(("snippet.thumbnails.default.url", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "snippet.thumbnails.default.width" => Some(("snippet.thumbnails.default.width", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "snippet.thumbnails.default.height" => Some(("snippet.thumbnails.default.height", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "snippet.thumbnails.high.url" => Some(("snippet.thumbnails.high.url", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "snippet.thumbnails.high.width" => Some(("snippet.thumbnails.high.width", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "snippet.thumbnails.high.height" => Some(("snippet.thumbnails.high.height", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "snippet.thumbnails.medium.url" => Some(("snippet.thumbnails.medium.url", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "snippet.thumbnails.medium.width" => Some(("snippet.thumbnails.medium.width", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "snippet.thumbnails.medium.height" => Some(("snippet.thumbnails.medium.height", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "snippet.thumbnails.maxres.url" => Some(("snippet.thumbnails.maxres.url", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "snippet.thumbnails.maxres.width" => Some(("snippet.thumbnails.maxres.width", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "snippet.thumbnails.maxres.height" => Some(("snippet.thumbnails.maxres.height", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "snippet.thumbnails.standard.url" => Some(("snippet.thumbnails.standard.url", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "snippet.thumbnails.standard.width" => Some(("snippet.thumbnails.standard.width", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "snippet.thumbnails.standard.height" => Some(("snippet.thumbnails.standard.height", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "player.embed-html" => Some(("player.embedHtml", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "player.embed-height" => Some(("player.embedHeight", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "player.embed-width" => Some(("player.embedWidth", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "processing-details.file-details-availability" => Some(("processingDetails.fileDetailsAvailability", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "processing-details.editor-suggestions-availability" => Some(("processingDetails.editorSuggestionsAvailability", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "processing-details.processing-status" => Some(("processingDetails.processingStatus", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "processing-details.processing-issues-availability" => Some(("processingDetails.processingIssuesAvailability", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "processing-details.processing-failure-reason" => Some(("processingDetails.processingFailureReason", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "processing-details.thumbnails-availability" => Some(("processingDetails.thumbnailsAvailability", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "processing-details.processing-progress.time-left-ms" => Some(("processingDetails.processingProgress.timeLeftMs", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "processing-details.processing-progress.parts-processed" => Some(("processingDetails.processingProgress.partsProcessed", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "processing-details.processing-progress.parts-total" => Some(("processingDetails.processingProgress.partsTotal", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "processing-details.tag-suggestions-availability" => Some(("processingDetails.tagSuggestionsAvailability", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "etag" => Some(("etag", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "project-details.tags" => Some(("projectDetails.tags", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Vec })),
                    "recording-details.recording-date" => Some(("recordingDetails.recordingDate", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "recording-details.location-description" => Some(("recordingDetails.locationDescription", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "recording-details.location.latitude" => Some(("recordingDetails.location.latitude", JsonTypeInfo { jtype: JsonType::Float, ctype: ComplexType::Pod })),
                    "recording-details.location.altitude" => Some(("recordingDetails.location.altitude", JsonTypeInfo { jtype: JsonType::Float, ctype: ComplexType::Pod })),
                    "recording-details.location.longitude" => Some(("recordingDetails.location.longitude", JsonTypeInfo { jtype: JsonType::Float, ctype: ComplexType::Pod })),
                    "id" => Some(("id", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    _ => {
                        let suggestion = FieldCursor::did_you_mean(key, &vec!["acb-rating", "access", "active-live-chat-id", "actual-end-time", "actual-start-time", "agcom-rating", "age-gating", "alcohol-content", "allowed", "altitude", "anatel-rating", "bbfc-rating", "bfvc-rating", "bitrate-bps", "blocked", "bmukk-rating", "caption", "category-id", "catv-rating", "catvfr-rating", "cbfc-rating", "ccc-rating", "cce-rating", "channel-id", "channel-title", "chfilm-rating", "chvrs-rating", "cicf-rating", "cna-rating", "cnc-rating", "comment-count", "concurrent-viewers", "container", "content-details", "content-rating", "country-restriction", "creation-time", "csa-rating", "cscf-rating", "czfilm-rating", "default", "default-audio-language", "default-language", "definition", "description", "dimension", "dislike-count", "djctq-rating", "djctq-rating-reasons", "duration", "duration-ms", "ecbmct-rating", "editor-suggestions", "editor-suggestions-availability", "eefilm-rating", "egfilm-rating", "eirin-rating", "embed-height", "embed-html", "embed-width", "embeddable", "etag", "exception", "failure-reason", "favorite-count", "fcbm-rating", "fco-rating", "file-details", "file-details-availability", "file-name", "file-size", "file-type", "fmoc-rating", "fpb-rating", "fpb-rating-reasons", "fsk-rating", "grfilm-rating", "has-custom-thumbnail", "height", "high", "icaa-rating", "id", "ifco-rating", "ilfilm-rating", "incaa-rating", "kfcb-rating", "kijkwijzer-rating", "kind", "kmrb-rating", "latitude", "license", "licensed-content", "like-count", "live-broadcast-content", "live-streaming-details", "localized", "location", "location-description", "longitude", "lsf-rating", "maxres", "mccaa-rating", "mccyp-rating", "mcst-rating", "mda-rating", "medietilsynet-rating", "medium", "meku-rating", "mena-mpaa-rating", "mibac-rating", "moc-rating", "moctw-rating", "monetization-details", "mpaa-rating", "mpaat-rating", "mtrcb-rating", "nbc-rating", "nbcpl-rating", "nfrc-rating", "nfvcb-rating", "nkclv-rating", "oflc-rating", "parts-processed", "parts-total", "pefilm-rating", "player", "privacy-status", "processing-details", "processing-errors", "processing-failure-reason", "processing-hints", "processing-issues-availability", "processing-progress", "processing-status", "processing-warnings", "project-details", "projection", "public-stats-viewable", "publish-at", "published-at", "rcnof-rating", "recording-date", "recording-details", "region-restriction", "rejection-reason", "relevant-topic-ids", "resorteviolencia-rating", "restricted", "rtc-rating", "rte-rating", "russia-rating", "scheduled-end-time", "scheduled-start-time", "skfilm-rating", "smais-rating", "smsa-rating", "snippet", "standard", "statistics", "status", "suggestions", "tag-suggestions-availability", "tags", "thumbnails", "thumbnails-availability", "time-left-ms", "title", "topic-categories", "topic-details", "topic-ids", "tvpg-rating", "upload-status", "url", "video-game-rating", "view-count", "width", "yt-rating"]);
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
                    call = call.stabilize(arg_from_str(value.unwrap_or("false"), err, "stabilize", "boolean"));
                },
                "on-behalf-of-content-owner-channel" => {
                    call = call.on_behalf_of_content_owner_channel(value.unwrap_or(""));
                },
                "on-behalf-of-content-owner" => {
                    call = call.on_behalf_of_content_owner(value.unwrap_or(""));
                },
                "notify-subscribers" => {
                    call = call.notify_subscribers(arg_from_str(value.unwrap_or("false"), err, "notify-subscribers", "boolean"));
                },
                "auto-levels" => {
                    call = call.auto_levels(arg_from_str(value.unwrap_or("false"), err, "auto-levels", "boolean"));
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
                                                                           v.extend(["on-behalf-of-content-owner-channel", "on-behalf-of-content-owner", "auto-levels", "notify-subscribers", "stabilize"].iter().map(|v|*v));
                                                                           v } ));
                    }
                }
            }
        }
        let vals = opt.values_of("mode").unwrap().collect::<Vec<&str>>();
        let protocol = calltype_from_str(vals[0], ["simple", "resumable"].iter().map(|&v| v.to_string()).collect(), err);
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
                CallType::Upload(UploadProtocol::Simple) => call.upload(input_file.unwrap(), mime_type.unwrap()),
                CallType::Upload(UploadProtocol::Resumable) => call.upload_resumable(input_file.unwrap(), mime_type.unwrap()),
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

    fn _videos_list(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        let mut call = self.hub.videos().list(opt.value_of("part").unwrap_or(""));
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
                    call = call.max_width(arg_from_str(value.unwrap_or("-0"), err, "max-width", "integer"));
                },
                "max-results" => {
                    call = call.max_results(arg_from_str(value.unwrap_or("-0"), err, "max-results", "integer"));
                },
                "max-height" => {
                    call = call.max_height(arg_from_str(value.unwrap_or("-0"), err, "max-height", "integer"));
                },
                "locale" => {
                    call = call.locale(value.unwrap_or(""));
                },
                "id" => {
                    call = call.id(value.unwrap_or(""));
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
                                                                           v.extend(["on-behalf-of-content-owner", "region-code", "max-width", "page-token", "locale", "chart", "max-results", "video-category-id", "max-height", "hl", "my-rating", "id"].iter().map(|v|*v));
                                                                           v } ));
                    }
                }
            }
        }
        let protocol = CallType::Standard;
        if dry_run {
            Ok(())
        } else {
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

    fn _videos_rate(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
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
                CallType::Standard => call.doit(),
                _ => unreachable!()
            } {
                Err(api_err) => Err(DoitError::ApiError(api_err)),
                Ok(mut response) => {
                    Ok(())
                }
            }
        }
    }

    fn _videos_report_abuse(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
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
                    "secondary-reason-id" => Some(("secondaryReasonId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "reason-id" => Some(("reasonId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "language" => Some(("language", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "comments" => Some(("comments", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
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
                CallType::Standard => call.doit(),
                _ => unreachable!()
            } {
                Err(api_err) => Err(DoitError::ApiError(api_err)),
                Ok(mut response) => {
                    Ok(())
                }
            }
        }
    }

    fn _videos_update(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
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
                    "status.license" => Some(("status.license", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "status.embeddable" => Some(("status.embeddable", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "status.privacy-status" => Some(("status.privacyStatus", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "status.publish-at" => Some(("status.publishAt", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "status.public-stats-viewable" => Some(("status.publicStatsViewable", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "status.upload-status" => Some(("status.uploadStatus", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "status.rejection-reason" => Some(("status.rejectionReason", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "status.failure-reason" => Some(("status.failureReason", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "topic-details.topic-ids" => Some(("topicDetails.topicIds", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Vec })),
                    "topic-details.topic-categories" => Some(("topicDetails.topicCategories", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Vec })),
                    "topic-details.relevant-topic-ids" => Some(("topicDetails.relevantTopicIds", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Vec })),
                    "kind" => Some(("kind", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "statistics.comment-count" => Some(("statistics.commentCount", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "statistics.view-count" => Some(("statistics.viewCount", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "statistics.favorite-count" => Some(("statistics.favoriteCount", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "statistics.dislike-count" => Some(("statistics.dislikeCount", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "statistics.like-count" => Some(("statistics.likeCount", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "content-details.definition" => Some(("contentDetails.definition", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "content-details.projection" => Some(("contentDetails.projection", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "content-details.country-restriction.exception" => Some(("contentDetails.countryRestriction.exception", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Vec })),
                    "content-details.country-restriction.allowed" => Some(("contentDetails.countryRestriction.allowed", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "content-details.has-custom-thumbnail" => Some(("contentDetails.hasCustomThumbnail", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "content-details.content-rating.yt-rating" => Some(("contentDetails.contentRating.ytRating", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "content-details.content-rating.catvfr-rating" => Some(("contentDetails.contentRating.catvfrRating", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "content-details.content-rating.cbfc-rating" => Some(("contentDetails.contentRating.cbfcRating", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "content-details.content-rating.bfvc-rating" => Some(("contentDetails.contentRating.bfvcRating", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "content-details.content-rating.mda-rating" => Some(("contentDetails.contentRating.mdaRating", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "content-details.content-rating.acb-rating" => Some(("contentDetails.contentRating.acbRating", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "content-details.content-rating.nfvcb-rating" => Some(("contentDetails.contentRating.nfvcbRating", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "content-details.content-rating.bmukk-rating" => Some(("contentDetails.contentRating.bmukkRating", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "content-details.content-rating.chfilm-rating" => Some(("contentDetails.contentRating.chfilmRating", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "content-details.content-rating.mena-mpaa-rating" => Some(("contentDetails.contentRating.menaMpaaRating", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "content-details.content-rating.csa-rating" => Some(("contentDetails.contentRating.csaRating", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "content-details.content-rating.moctw-rating" => Some(("contentDetails.contentRating.moctwRating", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "content-details.content-rating.anatel-rating" => Some(("contentDetails.contentRating.anatelRating", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "content-details.content-rating.catv-rating" => Some(("contentDetails.contentRating.catvRating", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "content-details.content-rating.pefilm-rating" => Some(("contentDetails.contentRating.pefilmRating", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "content-details.content-rating.djctq-rating-reasons" => Some(("contentDetails.contentRating.djctqRatingReasons", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Vec })),
                    "content-details.content-rating.incaa-rating" => Some(("contentDetails.contentRating.incaaRating", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "content-details.content-rating.cnc-rating" => Some(("contentDetails.contentRating.cncRating", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "content-details.content-rating.oflc-rating" => Some(("contentDetails.contentRating.oflcRating", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "content-details.content-rating.fpb-rating" => Some(("contentDetails.contentRating.fpbRating", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "content-details.content-rating.mccaa-rating" => Some(("contentDetails.contentRating.mccaaRating", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "content-details.content-rating.tvpg-rating" => Some(("contentDetails.contentRating.tvpgRating", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "content-details.content-rating.rtc-rating" => Some(("contentDetails.contentRating.rtcRating", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "content-details.content-rating.cscf-rating" => Some(("contentDetails.contentRating.cscfRating", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "content-details.content-rating.fsk-rating" => Some(("contentDetails.contentRating.fskRating", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "content-details.content-rating.bbfc-rating" => Some(("contentDetails.contentRating.bbfcRating", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "content-details.content-rating.kmrb-rating" => Some(("contentDetails.contentRating.kmrbRating", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "content-details.content-rating.smsa-rating" => Some(("contentDetails.contentRating.smsaRating", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "content-details.content-rating.egfilm-rating" => Some(("contentDetails.contentRating.egfilmRating", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "content-details.content-rating.cicf-rating" => Some(("contentDetails.contentRating.cicfRating", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "content-details.content-rating.nbcpl-rating" => Some(("contentDetails.contentRating.nbcplRating", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "content-details.content-rating.nbc-rating" => Some(("contentDetails.contentRating.nbcRating", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "content-details.content-rating.djctq-rating" => Some(("contentDetails.contentRating.djctqRating", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "content-details.content-rating.ifco-rating" => Some(("contentDetails.contentRating.ifcoRating", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "content-details.content-rating.mpaat-rating" => Some(("contentDetails.contentRating.mpaatRating", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "content-details.content-rating.fco-rating" => Some(("contentDetails.contentRating.fcoRating", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "content-details.content-rating.eefilm-rating" => Some(("contentDetails.contentRating.eefilmRating", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "content-details.content-rating.medietilsynet-rating" => Some(("contentDetails.contentRating.medietilsynetRating", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "content-details.content-rating.grfilm-rating" => Some(("contentDetails.contentRating.grfilmRating", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "content-details.content-rating.ccc-rating" => Some(("contentDetails.contentRating.cccRating", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "content-details.content-rating.rte-rating" => Some(("contentDetails.contentRating.rteRating", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "content-details.content-rating.czfilm-rating" => Some(("contentDetails.contentRating.czfilmRating", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "content-details.content-rating.ecbmct-rating" => Some(("contentDetails.contentRating.ecbmctRating", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "content-details.content-rating.fmoc-rating" => Some(("contentDetails.contentRating.fmocRating", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "content-details.content-rating.eirin-rating" => Some(("contentDetails.contentRating.eirinRating", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "content-details.content-rating.lsf-rating" => Some(("contentDetails.contentRating.lsfRating", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "content-details.content-rating.cce-rating" => Some(("contentDetails.contentRating.cceRating", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "content-details.content-rating.nkclv-rating" => Some(("contentDetails.contentRating.nkclvRating", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "content-details.content-rating.mtrcb-rating" => Some(("contentDetails.contentRating.mtrcbRating", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "content-details.content-rating.mibac-rating" => Some(("contentDetails.contentRating.mibacRating", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "content-details.content-rating.ilfilm-rating" => Some(("contentDetails.contentRating.ilfilmRating", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "content-details.content-rating.mcst-rating" => Some(("contentDetails.contentRating.mcstRating", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "content-details.content-rating.smais-rating" => Some(("contentDetails.contentRating.smaisRating", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "content-details.content-rating.russia-rating" => Some(("contentDetails.contentRating.russiaRating", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "content-details.content-rating.mpaa-rating" => Some(("contentDetails.contentRating.mpaaRating", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "content-details.content-rating.kfcb-rating" => Some(("contentDetails.contentRating.kfcbRating", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "content-details.content-rating.fpb-rating-reasons" => Some(("contentDetails.contentRating.fpbRatingReasons", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Vec })),
                    "content-details.content-rating.agcom-rating" => Some(("contentDetails.contentRating.agcomRating", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "content-details.content-rating.chvrs-rating" => Some(("contentDetails.contentRating.chvrsRating", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "content-details.content-rating.cna-rating" => Some(("contentDetails.contentRating.cnaRating", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "content-details.content-rating.icaa-rating" => Some(("contentDetails.contentRating.icaaRating", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "content-details.content-rating.mccyp-rating" => Some(("contentDetails.contentRating.mccypRating", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "content-details.content-rating.nfrc-rating" => Some(("contentDetails.contentRating.nfrcRating", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "content-details.content-rating.skfilm-rating" => Some(("contentDetails.contentRating.skfilmRating", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "content-details.content-rating.moc-rating" => Some(("contentDetails.contentRating.mocRating", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "content-details.content-rating.rcnof-rating" => Some(("contentDetails.contentRating.rcnofRating", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "content-details.content-rating.meku-rating" => Some(("contentDetails.contentRating.mekuRating", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "content-details.content-rating.resorteviolencia-rating" => Some(("contentDetails.contentRating.resorteviolenciaRating", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "content-details.content-rating.fcbm-rating" => Some(("contentDetails.contentRating.fcbmRating", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "content-details.content-rating.kijkwijzer-rating" => Some(("contentDetails.contentRating.kijkwijzerRating", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "content-details.caption" => Some(("contentDetails.caption", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "content-details.region-restriction.blocked" => Some(("contentDetails.regionRestriction.blocked", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Vec })),
                    "content-details.region-restriction.allowed" => Some(("contentDetails.regionRestriction.allowed", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Vec })),
                    "content-details.duration" => Some(("contentDetails.duration", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "content-details.licensed-content" => Some(("contentDetails.licensedContent", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "content-details.dimension" => Some(("contentDetails.dimension", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "monetization-details.access.exception" => Some(("monetizationDetails.access.exception", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Vec })),
                    "monetization-details.access.allowed" => Some(("monetizationDetails.access.allowed", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "age-gating.restricted" => Some(("ageGating.restricted", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "age-gating.alcohol-content" => Some(("ageGating.alcoholContent", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "age-gating.video-game-rating" => Some(("ageGating.videoGameRating", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "suggestions.processing-errors" => Some(("suggestions.processingErrors", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Vec })),
                    "suggestions.editor-suggestions" => Some(("suggestions.editorSuggestions", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Vec })),
                    "suggestions.processing-warnings" => Some(("suggestions.processingWarnings", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Vec })),
                    "suggestions.processing-hints" => Some(("suggestions.processingHints", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Vec })),
                    "live-streaming-details.actual-end-time" => Some(("liveStreamingDetails.actualEndTime", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "live-streaming-details.active-live-chat-id" => Some(("liveStreamingDetails.activeLiveChatId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "live-streaming-details.scheduled-start-time" => Some(("liveStreamingDetails.scheduledStartTime", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "live-streaming-details.concurrent-viewers" => Some(("liveStreamingDetails.concurrentViewers", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "live-streaming-details.actual-start-time" => Some(("liveStreamingDetails.actualStartTime", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "live-streaming-details.scheduled-end-time" => Some(("liveStreamingDetails.scheduledEndTime", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "file-details.bitrate-bps" => Some(("fileDetails.bitrateBps", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "file-details.container" => Some(("fileDetails.container", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "file-details.file-type" => Some(("fileDetails.fileType", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "file-details.creation-time" => Some(("fileDetails.creationTime", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "file-details.duration-ms" => Some(("fileDetails.durationMs", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "file-details.file-name" => Some(("fileDetails.fileName", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "file-details.file-size" => Some(("fileDetails.fileSize", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "snippet.description" => Some(("snippet.description", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "snippet.tags" => Some(("snippet.tags", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Vec })),
                    "snippet.default-audio-language" => Some(("snippet.defaultAudioLanguage", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "snippet.channel-id" => Some(("snippet.channelId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "snippet.published-at" => Some(("snippet.publishedAt", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "snippet.live-broadcast-content" => Some(("snippet.liveBroadcastContent", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "snippet.default-language" => Some(("snippet.defaultLanguage", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "snippet.channel-title" => Some(("snippet.channelTitle", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "snippet.title" => Some(("snippet.title", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "snippet.category-id" => Some(("snippet.categoryId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "snippet.localized.description" => Some(("snippet.localized.description", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "snippet.localized.title" => Some(("snippet.localized.title", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "snippet.thumbnails.default.url" => Some(("snippet.thumbnails.default.url", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "snippet.thumbnails.default.width" => Some(("snippet.thumbnails.default.width", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "snippet.thumbnails.default.height" => Some(("snippet.thumbnails.default.height", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "snippet.thumbnails.high.url" => Some(("snippet.thumbnails.high.url", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "snippet.thumbnails.high.width" => Some(("snippet.thumbnails.high.width", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "snippet.thumbnails.high.height" => Some(("snippet.thumbnails.high.height", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "snippet.thumbnails.medium.url" => Some(("snippet.thumbnails.medium.url", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "snippet.thumbnails.medium.width" => Some(("snippet.thumbnails.medium.width", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "snippet.thumbnails.medium.height" => Some(("snippet.thumbnails.medium.height", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "snippet.thumbnails.maxres.url" => Some(("snippet.thumbnails.maxres.url", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "snippet.thumbnails.maxres.width" => Some(("snippet.thumbnails.maxres.width", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "snippet.thumbnails.maxres.height" => Some(("snippet.thumbnails.maxres.height", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "snippet.thumbnails.standard.url" => Some(("snippet.thumbnails.standard.url", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "snippet.thumbnails.standard.width" => Some(("snippet.thumbnails.standard.width", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "snippet.thumbnails.standard.height" => Some(("snippet.thumbnails.standard.height", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "player.embed-html" => Some(("player.embedHtml", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "player.embed-height" => Some(("player.embedHeight", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "player.embed-width" => Some(("player.embedWidth", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "processing-details.file-details-availability" => Some(("processingDetails.fileDetailsAvailability", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "processing-details.editor-suggestions-availability" => Some(("processingDetails.editorSuggestionsAvailability", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "processing-details.processing-status" => Some(("processingDetails.processingStatus", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "processing-details.processing-issues-availability" => Some(("processingDetails.processingIssuesAvailability", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "processing-details.processing-failure-reason" => Some(("processingDetails.processingFailureReason", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "processing-details.thumbnails-availability" => Some(("processingDetails.thumbnailsAvailability", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "processing-details.processing-progress.time-left-ms" => Some(("processingDetails.processingProgress.timeLeftMs", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "processing-details.processing-progress.parts-processed" => Some(("processingDetails.processingProgress.partsProcessed", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "processing-details.processing-progress.parts-total" => Some(("processingDetails.processingProgress.partsTotal", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "processing-details.tag-suggestions-availability" => Some(("processingDetails.tagSuggestionsAvailability", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "etag" => Some(("etag", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "project-details.tags" => Some(("projectDetails.tags", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Vec })),
                    "recording-details.recording-date" => Some(("recordingDetails.recordingDate", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "recording-details.location-description" => Some(("recordingDetails.locationDescription", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "recording-details.location.latitude" => Some(("recordingDetails.location.latitude", JsonTypeInfo { jtype: JsonType::Float, ctype: ComplexType::Pod })),
                    "recording-details.location.altitude" => Some(("recordingDetails.location.altitude", JsonTypeInfo { jtype: JsonType::Float, ctype: ComplexType::Pod })),
                    "recording-details.location.longitude" => Some(("recordingDetails.location.longitude", JsonTypeInfo { jtype: JsonType::Float, ctype: ComplexType::Pod })),
                    "id" => Some(("id", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    _ => {
                        let suggestion = FieldCursor::did_you_mean(key, &vec!["acb-rating", "access", "active-live-chat-id", "actual-end-time", "actual-start-time", "agcom-rating", "age-gating", "alcohol-content", "allowed", "altitude", "anatel-rating", "bbfc-rating", "bfvc-rating", "bitrate-bps", "blocked", "bmukk-rating", "caption", "category-id", "catv-rating", "catvfr-rating", "cbfc-rating", "ccc-rating", "cce-rating", "channel-id", "channel-title", "chfilm-rating", "chvrs-rating", "cicf-rating", "cna-rating", "cnc-rating", "comment-count", "concurrent-viewers", "container", "content-details", "content-rating", "country-restriction", "creation-time", "csa-rating", "cscf-rating", "czfilm-rating", "default", "default-audio-language", "default-language", "definition", "description", "dimension", "dislike-count", "djctq-rating", "djctq-rating-reasons", "duration", "duration-ms", "ecbmct-rating", "editor-suggestions", "editor-suggestions-availability", "eefilm-rating", "egfilm-rating", "eirin-rating", "embed-height", "embed-html", "embed-width", "embeddable", "etag", "exception", "failure-reason", "favorite-count", "fcbm-rating", "fco-rating", "file-details", "file-details-availability", "file-name", "file-size", "file-type", "fmoc-rating", "fpb-rating", "fpb-rating-reasons", "fsk-rating", "grfilm-rating", "has-custom-thumbnail", "height", "high", "icaa-rating", "id", "ifco-rating", "ilfilm-rating", "incaa-rating", "kfcb-rating", "kijkwijzer-rating", "kind", "kmrb-rating", "latitude", "license", "licensed-content", "like-count", "live-broadcast-content", "live-streaming-details", "localized", "location", "location-description", "longitude", "lsf-rating", "maxres", "mccaa-rating", "mccyp-rating", "mcst-rating", "mda-rating", "medietilsynet-rating", "medium", "meku-rating", "mena-mpaa-rating", "mibac-rating", "moc-rating", "moctw-rating", "monetization-details", "mpaa-rating", "mpaat-rating", "mtrcb-rating", "nbc-rating", "nbcpl-rating", "nfrc-rating", "nfvcb-rating", "nkclv-rating", "oflc-rating", "parts-processed", "parts-total", "pefilm-rating", "player", "privacy-status", "processing-details", "processing-errors", "processing-failure-reason", "processing-hints", "processing-issues-availability", "processing-progress", "processing-status", "processing-warnings", "project-details", "projection", "public-stats-viewable", "publish-at", "published-at", "rcnof-rating", "recording-date", "recording-details", "region-restriction", "rejection-reason", "relevant-topic-ids", "resorteviolencia-rating", "restricted", "rtc-rating", "rte-rating", "russia-rating", "scheduled-end-time", "scheduled-start-time", "skfilm-rating", "smais-rating", "smsa-rating", "snippet", "standard", "statistics", "status", "suggestions", "tag-suggestions-availability", "tags", "thumbnails", "thumbnails-availability", "time-left-ms", "title", "topic-categories", "topic-details", "topic-ids", "tvpg-rating", "upload-status", "url", "video-game-rating", "view-count", "width", "yt-rating"]);
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

    fn _watermarks_set(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
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
                    "target-channel-id" => Some(("targetChannelId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "position.corner-position" => Some(("position.cornerPosition", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "position.type" => Some(("position.type", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "image-url" => Some(("imageUrl", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "timing.offset-ms" => Some(("timing.offsetMs", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "timing.type" => Some(("timing.type", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "timing.duration-ms" => Some(("timing.durationMs", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "image-bytes" => Some(("imageBytes", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
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
        let protocol = calltype_from_str(vals[0], ["simple", "resumable"].iter().map(|&v| v.to_string()).collect(), err);
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
                CallType::Upload(UploadProtocol::Simple) => call.upload(input_file.unwrap(), mime_type.unwrap()),
                CallType::Upload(UploadProtocol::Resumable) => call.upload_resumable(input_file.unwrap(), mime_type.unwrap()),
                CallType::Standard => unreachable!()
            } {
                Err(api_err) => Err(DoitError::ApiError(api_err)),
                Ok(mut response) => {
                    Ok(())
                }
            }
        }
    }

    fn _watermarks_unset(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
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
                CallType::Standard => call.doit(),
                _ => unreachable!()
            } {
                Err(api_err) => Err(DoitError::ApiError(api_err)),
                Ok(mut response) => {
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
            ("activities", Some(opt)) => {
                match opt.subcommand() {
                    ("insert", Some(opt)) => {
                        call_result = self._activities_insert(opt, dry_run, &mut err);
                    },
                    ("list", Some(opt)) => {
                        call_result = self._activities_list(opt, dry_run, &mut err);
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
                        call_result = self._captions_delete(opt, dry_run, &mut err);
                    },
                    ("download", Some(opt)) => {
                        call_result = self._captions_download(opt, dry_run, &mut err);
                    },
                    ("insert", Some(opt)) => {
                        call_result = self._captions_insert(opt, dry_run, &mut err);
                    },
                    ("list", Some(opt)) => {
                        call_result = self._captions_list(opt, dry_run, &mut err);
                    },
                    ("update", Some(opt)) => {
                        call_result = self._captions_update(opt, dry_run, &mut err);
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
                        call_result = self._channel_banners_insert(opt, dry_run, &mut err);
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
                        call_result = self._channel_sections_delete(opt, dry_run, &mut err);
                    },
                    ("insert", Some(opt)) => {
                        call_result = self._channel_sections_insert(opt, dry_run, &mut err);
                    },
                    ("list", Some(opt)) => {
                        call_result = self._channel_sections_list(opt, dry_run, &mut err);
                    },
                    ("update", Some(opt)) => {
                        call_result = self._channel_sections_update(opt, dry_run, &mut err);
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
                        call_result = self._channels_list(opt, dry_run, &mut err);
                    },
                    ("update", Some(opt)) => {
                        call_result = self._channels_update(opt, dry_run, &mut err);
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
                        call_result = self._comment_threads_insert(opt, dry_run, &mut err);
                    },
                    ("list", Some(opt)) => {
                        call_result = self._comment_threads_list(opt, dry_run, &mut err);
                    },
                    ("update", Some(opt)) => {
                        call_result = self._comment_threads_update(opt, dry_run, &mut err);
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
                        call_result = self._comments_delete(opt, dry_run, &mut err);
                    },
                    ("insert", Some(opt)) => {
                        call_result = self._comments_insert(opt, dry_run, &mut err);
                    },
                    ("list", Some(opt)) => {
                        call_result = self._comments_list(opt, dry_run, &mut err);
                    },
                    ("mark-as-spam", Some(opt)) => {
                        call_result = self._comments_mark_as_spam(opt, dry_run, &mut err);
                    },
                    ("set-moderation-status", Some(opt)) => {
                        call_result = self._comments_set_moderation_status(opt, dry_run, &mut err);
                    },
                    ("update", Some(opt)) => {
                        call_result = self._comments_update(opt, dry_run, &mut err);
                    },
                    _ => {
                        err.issues.push(CLIError::MissingMethodError("comments".to_string()));
                        writeln!(io::stderr(), "{}\n", opt.usage()).ok();
                    }
                }
            },
            ("guide-categories", Some(opt)) => {
                match opt.subcommand() {
                    ("list", Some(opt)) => {
                        call_result = self._guide_categories_list(opt, dry_run, &mut err);
                    },
                    _ => {
                        err.issues.push(CLIError::MissingMethodError("guide-categories".to_string()));
                        writeln!(io::stderr(), "{}\n", opt.usage()).ok();
                    }
                }
            },
            ("i18n-languages", Some(opt)) => {
                match opt.subcommand() {
                    ("list", Some(opt)) => {
                        call_result = self._i18n_languages_list(opt, dry_run, &mut err);
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
                        call_result = self._i18n_regions_list(opt, dry_run, &mut err);
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
                        call_result = self._live_broadcasts_bind(opt, dry_run, &mut err);
                    },
                    ("control", Some(opt)) => {
                        call_result = self._live_broadcasts_control(opt, dry_run, &mut err);
                    },
                    ("delete", Some(opt)) => {
                        call_result = self._live_broadcasts_delete(opt, dry_run, &mut err);
                    },
                    ("insert", Some(opt)) => {
                        call_result = self._live_broadcasts_insert(opt, dry_run, &mut err);
                    },
                    ("list", Some(opt)) => {
                        call_result = self._live_broadcasts_list(opt, dry_run, &mut err);
                    },
                    ("transition", Some(opt)) => {
                        call_result = self._live_broadcasts_transition(opt, dry_run, &mut err);
                    },
                    ("update", Some(opt)) => {
                        call_result = self._live_broadcasts_update(opt, dry_run, &mut err);
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
                        call_result = self._live_chat_bans_delete(opt, dry_run, &mut err);
                    },
                    ("insert", Some(opt)) => {
                        call_result = self._live_chat_bans_insert(opt, dry_run, &mut err);
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
                        call_result = self._live_chat_messages_delete(opt, dry_run, &mut err);
                    },
                    ("insert", Some(opt)) => {
                        call_result = self._live_chat_messages_insert(opt, dry_run, &mut err);
                    },
                    ("list", Some(opt)) => {
                        call_result = self._live_chat_messages_list(opt, dry_run, &mut err);
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
                        call_result = self._live_chat_moderators_delete(opt, dry_run, &mut err);
                    },
                    ("insert", Some(opt)) => {
                        call_result = self._live_chat_moderators_insert(opt, dry_run, &mut err);
                    },
                    ("list", Some(opt)) => {
                        call_result = self._live_chat_moderators_list(opt, dry_run, &mut err);
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
                        call_result = self._live_streams_delete(opt, dry_run, &mut err);
                    },
                    ("insert", Some(opt)) => {
                        call_result = self._live_streams_insert(opt, dry_run, &mut err);
                    },
                    ("list", Some(opt)) => {
                        call_result = self._live_streams_list(opt, dry_run, &mut err);
                    },
                    ("update", Some(opt)) => {
                        call_result = self._live_streams_update(opt, dry_run, &mut err);
                    },
                    _ => {
                        err.issues.push(CLIError::MissingMethodError("live-streams".to_string()));
                        writeln!(io::stderr(), "{}\n", opt.usage()).ok();
                    }
                }
            },
            ("playlist-items", Some(opt)) => {
                match opt.subcommand() {
                    ("delete", Some(opt)) => {
                        call_result = self._playlist_items_delete(opt, dry_run, &mut err);
                    },
                    ("insert", Some(opt)) => {
                        call_result = self._playlist_items_insert(opt, dry_run, &mut err);
                    },
                    ("list", Some(opt)) => {
                        call_result = self._playlist_items_list(opt, dry_run, &mut err);
                    },
                    ("update", Some(opt)) => {
                        call_result = self._playlist_items_update(opt, dry_run, &mut err);
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
                        call_result = self._playlists_delete(opt, dry_run, &mut err);
                    },
                    ("insert", Some(opt)) => {
                        call_result = self._playlists_insert(opt, dry_run, &mut err);
                    },
                    ("list", Some(opt)) => {
                        call_result = self._playlists_list(opt, dry_run, &mut err);
                    },
                    ("update", Some(opt)) => {
                        call_result = self._playlists_update(opt, dry_run, &mut err);
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
                        call_result = self._search_list(opt, dry_run, &mut err);
                    },
                    _ => {
                        err.issues.push(CLIError::MissingMethodError("search".to_string()));
                        writeln!(io::stderr(), "{}\n", opt.usage()).ok();
                    }
                }
            },
            ("sponsors", Some(opt)) => {
                match opt.subcommand() {
                    ("list", Some(opt)) => {
                        call_result = self._sponsors_list(opt, dry_run, &mut err);
                    },
                    _ => {
                        err.issues.push(CLIError::MissingMethodError("sponsors".to_string()));
                        writeln!(io::stderr(), "{}\n", opt.usage()).ok();
                    }
                }
            },
            ("subscriptions", Some(opt)) => {
                match opt.subcommand() {
                    ("delete", Some(opt)) => {
                        call_result = self._subscriptions_delete(opt, dry_run, &mut err);
                    },
                    ("insert", Some(opt)) => {
                        call_result = self._subscriptions_insert(opt, dry_run, &mut err);
                    },
                    ("list", Some(opt)) => {
                        call_result = self._subscriptions_list(opt, dry_run, &mut err);
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
                        call_result = self._super_chat_events_list(opt, dry_run, &mut err);
                    },
                    _ => {
                        err.issues.push(CLIError::MissingMethodError("super-chat-events".to_string()));
                        writeln!(io::stderr(), "{}\n", opt.usage()).ok();
                    }
                }
            },
            ("thumbnails", Some(opt)) => {
                match opt.subcommand() {
                    ("set", Some(opt)) => {
                        call_result = self._thumbnails_set(opt, dry_run, &mut err);
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
                        call_result = self._video_abuse_report_reasons_list(opt, dry_run, &mut err);
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
                        call_result = self._video_categories_list(opt, dry_run, &mut err);
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
                        call_result = self._videos_delete(opt, dry_run, &mut err);
                    },
                    ("get-rating", Some(opt)) => {
                        call_result = self._videos_get_rating(opt, dry_run, &mut err);
                    },
                    ("insert", Some(opt)) => {
                        call_result = self._videos_insert(opt, dry_run, &mut err);
                    },
                    ("list", Some(opt)) => {
                        call_result = self._videos_list(opt, dry_run, &mut err);
                    },
                    ("rate", Some(opt)) => {
                        call_result = self._videos_rate(opt, dry_run, &mut err);
                    },
                    ("report-abuse", Some(opt)) => {
                        call_result = self._videos_report_abuse(opt, dry_run, &mut err);
                    },
                    ("update", Some(opt)) => {
                        call_result = self._videos_update(opt, dry_run, &mut err);
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
                        call_result = self._watermarks_set(opt, dry_run, &mut err);
                    },
                    ("unset", Some(opt)) => {
                        call_result = self._watermarks_unset(opt, dry_run, &mut err);
                    },
                    _ => {
                        err.issues.push(CLIError::MissingMethodError("watermarks".to_string()));
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

            match cmn::application_secret_from_directory(&config_dir, "youtube3-secret.json",
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

        let auth = Authenticator::new(  &secret, DefaultAuthenticatorDelegate,
                                        if opt.is_present("debug-auth") {
                                            hyper::Client::with_connector(mock::TeeConnector {
                                                    connector: hyper::net::HttpsConnector::new(hyper_rustls::TlsClient::new())
                                                })
                                        } else {
                                            hyper::Client::with_connector(hyper::net::HttpsConnector::new(hyper_rustls::TlsClient::new()))
                                        },
                                        JsonTokenStorage {
                                          program_name: "youtube3",
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
            hub: api::YouTube::new(client, auth),
            gp: vec!["alt", "fields", "key", "oauth-token", "pretty-print", "quota-user", "user-ip"],
            gpm: vec![
                    ("oauth-token", "oauth_token"),
                    ("pretty-print", "prettyPrint"),
                    ("quota-user", "quotaUser"),
                    ("user-ip", "userIp"),
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
    let upload_value_names = ["mode", "file"];
    let arg_data = [
        ("activities", "methods: 'insert' and 'list'", vec![
            ("insert",
                    Some(r##"Posts a bulletin for a specific channel. (The user submitting the request must be authorized to act on the channel's behalf.)
        
        Note: Even though an activity resource can contain information about actions like a user rating a video or marking a video as a favorite, you need to use other API methods to generate those activity resources. For example, you would use the API's videos.rate() method to rate a video and the playlistItems.insert() method to mark a video as a favorite."##),
                    "Details at http://byron.github.io/google-apis-rs/google_youtube3_cli/activities_insert",
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
                    Some(r##"Returns a list of channel activity events that match the request criteria. For example, you can retrieve events associated with a particular channel, events associated with the user's subscriptions and Google+ friends, or the YouTube home page feed, which is customized for each user."##),
                    "Details at http://byron.github.io/google-apis-rs/google_youtube3_cli/activities_list",
                  vec![
                    (Some(r##"part"##),
                     None,
                     Some(r##"The part parameter specifies a comma-separated list of one or more activity resource properties that the API response will include.
        
        If the parameter identifies a property that contains child properties, the child properties will be included in the response. For example, in an activity resource, the snippet property contains other properties that identify the type of activity, a display title for the activity, and so forth. If you set part=snippet, the API response will also contain all of those nested properties."##),
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
                    Some(r##"Deletes a specified caption track."##),
                    "Details at http://byron.github.io/google-apis-rs/google_youtube3_cli/captions_delete",
                  vec![
                    (Some(r##"id"##),
                     None,
                     Some(r##"The id parameter identifies the caption track that is being deleted. The value is a caption track ID as identified by the id property in a caption resource."##),
                     Some(true),
                     Some(false)),
        
                    (Some(r##"v"##),
                     Some(r##"p"##),
                     Some(r##"Set various optional parameters, matching the key=value form"##),
                     Some(false),
                     Some(true)),
                  ]),
            ("download",
                    Some(r##"Downloads a caption track. The caption track is returned in its original format unless the request specifies a value for the tfmt parameter and in its original language unless the request specifies a value for the tlang parameter."##),
                    "Details at http://byron.github.io/google-apis-rs/google_youtube3_cli/captions_download",
                  vec![
                    (Some(r##"id"##),
                     None,
                     Some(r##"The id parameter identifies the caption track that is being retrieved. The value is a caption track ID as identified by the id property in a caption resource."##),
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
                    Some(r##"Uploads a caption track."##),
                    "Details at http://byron.github.io/google-apis-rs/google_youtube3_cli/captions_insert",
                  vec![
                    (Some(r##"kv"##),
                     Some(r##"r"##),
                     Some(r##"Set various fields of the request structure, matching the key=value form"##),
                     Some(true),
                     Some(true)),
        
                    (Some(r##"mode"##),
                     Some(r##"u"##),
                     Some(r##"Specify the upload protocol (simple|resumable) and the file to upload"##),
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
                    Some(r##"Returns a list of caption tracks that are associated with a specified video. Note that the API response does not contain the actual captions and that the captions.download method provides the ability to retrieve a caption track."##),
                    "Details at http://byron.github.io/google-apis-rs/google_youtube3_cli/captions_list",
                  vec![
                    (Some(r##"part"##),
                     None,
                     Some(r##"The part parameter specifies a comma-separated list of one or more caption resource parts that the API response will include. The part names that you can include in the parameter value are id and snippet."##),
                     Some(true),
                     Some(false)),
        
                    (Some(r##"video-id"##),
                     None,
                     Some(r##"The videoId parameter specifies the YouTube video ID of the video for which the API should return caption tracks."##),
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
                    Some(r##"Updates a caption track. When updating a caption track, you can change the track's draft status, upload a new caption file for the track, or both."##),
                    "Details at http://byron.github.io/google-apis-rs/google_youtube3_cli/captions_update",
                  vec![
                    (Some(r##"kv"##),
                     Some(r##"r"##),
                     Some(r##"Set various fields of the request structure, matching the key=value form"##),
                     Some(true),
                     Some(true)),
        
                    (Some(r##"mode"##),
                     Some(r##"u"##),
                     Some(r##"Specify the upload protocol (simple|resumable) and the file to upload"##),
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
                    Some(r##"Uploads a channel banner image to YouTube. This method represents the first two steps in a three-step process to update the banner image for a channel:
        
        - Call the channelBanners.insert method to upload the binary image data to YouTube. The image must have a 16:9 aspect ratio and be at least 2120x1192 pixels.
        - Extract the url property's value from the response that the API returns for step 1.
        - Call the channels.update method to update the channel's branding settings. Set the brandingSettings.image.bannerExternalUrl property's value to the URL obtained in step 2."##),
                    "Details at http://byron.github.io/google-apis-rs/google_youtube3_cli/channel-banners_insert",
                  vec![
                    (Some(r##"kv"##),
                     Some(r##"r"##),
                     Some(r##"Set various fields of the request structure, matching the key=value form"##),
                     Some(true),
                     Some(true)),
        
                    (Some(r##"mode"##),
                     Some(r##"u"##),
                     Some(r##"Specify the upload protocol (simple|resumable) and the file to upload"##),
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
                    Some(r##"Deletes a channelSection."##),
                    "Details at http://byron.github.io/google-apis-rs/google_youtube3_cli/channel-sections_delete",
                  vec![
                    (Some(r##"id"##),
                     None,
                     Some(r##"The id parameter specifies the YouTube channelSection ID for the resource that is being deleted. In a channelSection resource, the id property specifies the YouTube channelSection ID."##),
                     Some(true),
                     Some(false)),
        
                    (Some(r##"v"##),
                     Some(r##"p"##),
                     Some(r##"Set various optional parameters, matching the key=value form"##),
                     Some(false),
                     Some(true)),
                  ]),
            ("insert",
                    Some(r##"Adds a channelSection for the authenticated user's channel."##),
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
                    Some(r##"Returns channelSection resources that match the API request criteria."##),
                    "Details at http://byron.github.io/google-apis-rs/google_youtube3_cli/channel-sections_list",
                  vec![
                    (Some(r##"part"##),
                     None,
                     Some(r##"The part parameter specifies a comma-separated list of one or more channelSection resource properties that the API response will include. The part names that you can include in the parameter value are id, snippet, and contentDetails.
        
        If the parameter identifies a property that contains child properties, the child properties will be included in the response. For example, in a channelSection resource, the snippet property contains other properties, such as a display title for the channelSection. If you set part=snippet, the API response will also contain all of those nested properties."##),
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
                    Some(r##"Update a channelSection."##),
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
                    Some(r##"Returns a collection of zero or more channel resources that match the request criteria."##),
                    "Details at http://byron.github.io/google-apis-rs/google_youtube3_cli/channels_list",
                  vec![
                    (Some(r##"part"##),
                     None,
                     Some(r##"The part parameter specifies a comma-separated list of one or more channel resource properties that the API response will include.
        
        If the parameter identifies a property that contains child properties, the child properties will be included in the response. For example, in a channel resource, the contentDetails property contains other properties, such as the uploads properties. As such, if you set part=contentDetails, the API response will also contain all of those nested properties."##),
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
                    Some(r##"Updates a channel's metadata. Note that this method currently only supports updates to the channel resource's brandingSettings and invideoPromotion objects and their child properties."##),
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
        
        ("comment-threads", "methods: 'insert', 'list' and 'update'", vec![
            ("insert",
                    Some(r##"Creates a new top-level comment. To add a reply to an existing comment, use the comments.insert method instead."##),
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
                    Some(r##"Returns a list of comment threads that match the API request parameters."##),
                    "Details at http://byron.github.io/google-apis-rs/google_youtube3_cli/comment-threads_list",
                  vec![
                    (Some(r##"part"##),
                     None,
                     Some(r##"The part parameter specifies a comma-separated list of one or more commentThread resource properties that the API response will include."##),
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
                    Some(r##"Modifies the top-level comment in a comment thread."##),
                    "Details at http://byron.github.io/google-apis-rs/google_youtube3_cli/comment-threads_update",
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
        
        ("comments", "methods: 'delete', 'insert', 'list', 'mark-as-spam', 'set-moderation-status' and 'update'", vec![
            ("delete",
                    Some(r##"Deletes a comment."##),
                    "Details at http://byron.github.io/google-apis-rs/google_youtube3_cli/comments_delete",
                  vec![
                    (Some(r##"id"##),
                     None,
                     Some(r##"The id parameter specifies the comment ID for the resource that is being deleted."##),
                     Some(true),
                     Some(false)),
        
                    (Some(r##"v"##),
                     Some(r##"p"##),
                     Some(r##"Set various optional parameters, matching the key=value form"##),
                     Some(false),
                     Some(true)),
                  ]),
            ("insert",
                    Some(r##"Creates a reply to an existing comment. Note: To create a top-level comment, use the commentThreads.insert method."##),
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
                    Some(r##"Returns a list of comments that match the API request parameters."##),
                    "Details at http://byron.github.io/google-apis-rs/google_youtube3_cli/comments_list",
                  vec![
                    (Some(r##"part"##),
                     None,
                     Some(r##"The part parameter specifies a comma-separated list of one or more comment resource properties that the API response will include."##),
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
                     Some(r##"The id parameter specifies a comma-separated list of IDs of comments that the caller believes should be classified as spam."##),
                     Some(true),
                     Some(false)),
        
                    (Some(r##"v"##),
                     Some(r##"p"##),
                     Some(r##"Set various optional parameters, matching the key=value form"##),
                     Some(false),
                     Some(true)),
                  ]),
            ("set-moderation-status",
                    Some(r##"Sets the moderation status of one or more comments. The API request must be authorized by the owner of the channel or video associated with the comments."##),
                    "Details at http://byron.github.io/google-apis-rs/google_youtube3_cli/comments_set-moderation-status",
                  vec![
                    (Some(r##"id"##),
                     None,
                     Some(r##"The id parameter specifies a comma-separated list of IDs that identify the comments for which you are updating the moderation status."##),
                     Some(true),
                     Some(false)),
        
                    (Some(r##"moderation-status"##),
                     None,
                     Some(r##"Identifies the new moderation status of the specified comments."##),
                     Some(true),
                     Some(false)),
        
                    (Some(r##"v"##),
                     Some(r##"p"##),
                     Some(r##"Set various optional parameters, matching the key=value form"##),
                     Some(false),
                     Some(true)),
                  ]),
            ("update",
                    Some(r##"Modifies a comment."##),
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
        
        ("guide-categories", "methods: 'list'", vec![
            ("list",
                    Some(r##"Returns a list of categories that can be associated with YouTube channels."##),
                    "Details at http://byron.github.io/google-apis-rs/google_youtube3_cli/guide-categories_list",
                  vec![
                    (Some(r##"part"##),
                     None,
                     Some(r##"The part parameter specifies the guideCategory resource properties that the API response will include. Set the parameter value to snippet."##),
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
        
        ("i18n-languages", "methods: 'list'", vec![
            ("list",
                    Some(r##"Returns a list of application languages that the YouTube website supports."##),
                    "Details at http://byron.github.io/google-apis-rs/google_youtube3_cli/i18n-languages_list",
                  vec![
                    (Some(r##"part"##),
                     None,
                     Some(r##"The part parameter specifies the i18nLanguage resource properties that the API response will include. Set the parameter value to snippet."##),
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
                    Some(r##"Returns a list of content regions that the YouTube website supports."##),
                    "Details at http://byron.github.io/google-apis-rs/google_youtube3_cli/i18n-regions_list",
                  vec![
                    (Some(r##"part"##),
                     None,
                     Some(r##"The part parameter specifies the i18nRegion resource properties that the API response will include. Set the parameter value to snippet."##),
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
        
        ("live-broadcasts", "methods: 'bind', 'control', 'delete', 'insert', 'list', 'transition' and 'update'", vec![
            ("bind",
                    Some(r##"Binds a YouTube broadcast to a stream or removes an existing binding between a broadcast and a stream. A broadcast can only be bound to one video stream, though a video stream may be bound to more than one broadcast."##),
                    "Details at http://byron.github.io/google-apis-rs/google_youtube3_cli/live-broadcasts_bind",
                  vec![
                    (Some(r##"id"##),
                     None,
                     Some(r##"The id parameter specifies the unique ID of the broadcast that is being bound to a video stream."##),
                     Some(true),
                     Some(false)),
        
                    (Some(r##"part"##),
                     None,
                     Some(r##"The part parameter specifies a comma-separated list of one or more liveBroadcast resource properties that the API response will include. The part names that you can include in the parameter value are id, snippet, contentDetails, and status."##),
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
            ("control",
                    Some(r##"Controls the settings for a slate that can be displayed in the broadcast stream."##),
                    "Details at http://byron.github.io/google-apis-rs/google_youtube3_cli/live-broadcasts_control",
                  vec![
                    (Some(r##"id"##),
                     None,
                     Some(r##"The id parameter specifies the YouTube live broadcast ID that uniquely identifies the broadcast in which the slate is being updated."##),
                     Some(true),
                     Some(false)),
        
                    (Some(r##"part"##),
                     None,
                     Some(r##"The part parameter specifies a comma-separated list of one or more liveBroadcast resource properties that the API response will include. The part names that you can include in the parameter value are id, snippet, contentDetails, and status."##),
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
                    Some(r##"Deletes a broadcast."##),
                    "Details at http://byron.github.io/google-apis-rs/google_youtube3_cli/live-broadcasts_delete",
                  vec![
                    (Some(r##"id"##),
                     None,
                     Some(r##"The id parameter specifies the YouTube live broadcast ID for the resource that is being deleted."##),
                     Some(true),
                     Some(false)),
        
                    (Some(r##"v"##),
                     Some(r##"p"##),
                     Some(r##"Set various optional parameters, matching the key=value form"##),
                     Some(false),
                     Some(true)),
                  ]),
            ("insert",
                    Some(r##"Creates a broadcast."##),
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
            ("list",
                    Some(r##"Returns a list of YouTube broadcasts that match the API request parameters."##),
                    "Details at http://byron.github.io/google-apis-rs/google_youtube3_cli/live-broadcasts_list",
                  vec![
                    (Some(r##"part"##),
                     None,
                     Some(r##"The part parameter specifies a comma-separated list of one or more liveBroadcast resource properties that the API response will include. The part names that you can include in the parameter value are id, snippet, contentDetails, and status."##),
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
                    Some(r##"Changes the status of a YouTube live broadcast and initiates any processes associated with the new status. For example, when you transition a broadcast's status to testing, YouTube starts to transmit video to that broadcast's monitor stream. Before calling this method, you should confirm that the value of the status.streamStatus property for the stream bound to your broadcast is active."##),
                    "Details at http://byron.github.io/google-apis-rs/google_youtube3_cli/live-broadcasts_transition",
                  vec![
                    (Some(r##"broadcast-status"##),
                     None,
                     Some(r##"The broadcastStatus parameter identifies the state to which the broadcast is changing. Note that to transition a broadcast to either the testing or live state, the status.streamStatus must be active for the stream that the broadcast is bound to."##),
                     Some(true),
                     Some(false)),
        
                    (Some(r##"id"##),
                     None,
                     Some(r##"The id parameter specifies the unique ID of the broadcast that is transitioning to another status."##),
                     Some(true),
                     Some(false)),
        
                    (Some(r##"part"##),
                     None,
                     Some(r##"The part parameter specifies a comma-separated list of one or more liveBroadcast resource properties that the API response will include. The part names that you can include in the parameter value are id, snippet, contentDetails, and status."##),
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
                    Some(r##"Updates a broadcast. For example, you could modify the broadcast settings defined in the liveBroadcast resource's contentDetails object."##),
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
                    Some(r##"Removes a chat ban."##),
                    "Details at http://byron.github.io/google-apis-rs/google_youtube3_cli/live-chat-bans_delete",
                  vec![
                    (Some(r##"id"##),
                     None,
                     Some(r##"The id parameter identifies the chat ban to remove. The value uniquely identifies both the ban and the chat."##),
                     Some(true),
                     Some(false)),
        
                    (Some(r##"v"##),
                     Some(r##"p"##),
                     Some(r##"Set various optional parameters, matching the key=value form"##),
                     Some(false),
                     Some(true)),
                  ]),
            ("insert",
                    Some(r##"Adds a new ban to the chat."##),
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
        
        ("live-chat-messages", "methods: 'delete', 'insert' and 'list'", vec![
            ("delete",
                    Some(r##"Deletes a chat message."##),
                    "Details at http://byron.github.io/google-apis-rs/google_youtube3_cli/live-chat-messages_delete",
                  vec![
                    (Some(r##"id"##),
                     None,
                     Some(r##"The id parameter specifies the YouTube chat message ID of the resource that is being deleted."##),
                     Some(true),
                     Some(false)),
        
                    (Some(r##"v"##),
                     Some(r##"p"##),
                     Some(r##"Set various optional parameters, matching the key=value form"##),
                     Some(false),
                     Some(true)),
                  ]),
            ("insert",
                    Some(r##"Adds a message to a live chat."##),
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
                    Some(r##"Lists live chat messages for a specific chat."##),
                    "Details at http://byron.github.io/google-apis-rs/google_youtube3_cli/live-chat-messages_list",
                  vec![
                    (Some(r##"live-chat-id"##),
                     None,
                     Some(r##"The liveChatId parameter specifies the ID of the chat whose messages will be returned."##),
                     Some(true),
                     Some(false)),
        
                    (Some(r##"part"##),
                     None,
                     Some(r##"The part parameter specifies the liveChatComment resource parts that the API response will include. Supported values are id and snippet."##),
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
        
        ("live-chat-moderators", "methods: 'delete', 'insert' and 'list'", vec![
            ("delete",
                    Some(r##"Removes a chat moderator."##),
                    "Details at http://byron.github.io/google-apis-rs/google_youtube3_cli/live-chat-moderators_delete",
                  vec![
                    (Some(r##"id"##),
                     None,
                     Some(r##"The id parameter identifies the chat moderator to remove. The value uniquely identifies both the moderator and the chat."##),
                     Some(true),
                     Some(false)),
        
                    (Some(r##"v"##),
                     Some(r##"p"##),
                     Some(r##"Set various optional parameters, matching the key=value form"##),
                     Some(false),
                     Some(true)),
                  ]),
            ("insert",
                    Some(r##"Adds a new moderator for the chat."##),
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
                    Some(r##"Lists moderators for a live chat."##),
                    "Details at http://byron.github.io/google-apis-rs/google_youtube3_cli/live-chat-moderators_list",
                  vec![
                    (Some(r##"live-chat-id"##),
                     None,
                     Some(r##"The liveChatId parameter specifies the YouTube live chat for which the API should return moderators."##),
                     Some(true),
                     Some(false)),
        
                    (Some(r##"part"##),
                     None,
                     Some(r##"The part parameter specifies the liveChatModerator resource parts that the API response will include. Supported values are id and snippet."##),
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
                    Some(r##"Deletes a video stream."##),
                    "Details at http://byron.github.io/google-apis-rs/google_youtube3_cli/live-streams_delete",
                  vec![
                    (Some(r##"id"##),
                     None,
                     Some(r##"The id parameter specifies the YouTube live stream ID for the resource that is being deleted."##),
                     Some(true),
                     Some(false)),
        
                    (Some(r##"v"##),
                     Some(r##"p"##),
                     Some(r##"Set various optional parameters, matching the key=value form"##),
                     Some(false),
                     Some(true)),
                  ]),
            ("insert",
                    Some(r##"Creates a video stream. The stream enables you to send your video to YouTube, which can then broadcast the video to your audience."##),
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
                    Some(r##"Returns a list of video streams that match the API request parameters."##),
                    "Details at http://byron.github.io/google-apis-rs/google_youtube3_cli/live-streams_list",
                  vec![
                    (Some(r##"part"##),
                     None,
                     Some(r##"The part parameter specifies a comma-separated list of one or more liveStream resource properties that the API response will include. The part names that you can include in the parameter value are id, snippet, cdn, and status."##),
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
                    Some(r##"Updates a video stream. If the properties that you want to change cannot be updated, then you need to create a new stream with the proper settings."##),
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
        
        ("playlist-items", "methods: 'delete', 'insert', 'list' and 'update'", vec![
            ("delete",
                    Some(r##"Deletes a playlist item."##),
                    "Details at http://byron.github.io/google-apis-rs/google_youtube3_cli/playlist-items_delete",
                  vec![
                    (Some(r##"id"##),
                     None,
                     Some(r##"The id parameter specifies the YouTube playlist item ID for the playlist item that is being deleted. In a playlistItem resource, the id property specifies the playlist item's ID."##),
                     Some(true),
                     Some(false)),
        
                    (Some(r##"v"##),
                     Some(r##"p"##),
                     Some(r##"Set various optional parameters, matching the key=value form"##),
                     Some(false),
                     Some(true)),
                  ]),
            ("insert",
                    Some(r##"Adds a resource to a playlist."##),
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
                    Some(r##"Returns a collection of playlist items that match the API request parameters. You can retrieve all of the playlist items in a specified playlist or retrieve one or more playlist items by their unique IDs."##),
                    "Details at http://byron.github.io/google-apis-rs/google_youtube3_cli/playlist-items_list",
                  vec![
                    (Some(r##"part"##),
                     None,
                     Some(r##"The part parameter specifies a comma-separated list of one or more playlistItem resource properties that the API response will include.
        
        If the parameter identifies a property that contains child properties, the child properties will be included in the response. For example, in a playlistItem resource, the snippet property contains numerous fields, including the title, description, position, and resourceId properties. As such, if you set part=snippet, the API response will contain all of those properties."##),
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
                    Some(r##"Modifies a playlist item. For example, you could update the item's position in the playlist."##),
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
                    Some(r##"Deletes a playlist."##),
                    "Details at http://byron.github.io/google-apis-rs/google_youtube3_cli/playlists_delete",
                  vec![
                    (Some(r##"id"##),
                     None,
                     Some(r##"The id parameter specifies the YouTube playlist ID for the playlist that is being deleted. In a playlist resource, the id property specifies the playlist's ID."##),
                     Some(true),
                     Some(false)),
        
                    (Some(r##"v"##),
                     Some(r##"p"##),
                     Some(r##"Set various optional parameters, matching the key=value form"##),
                     Some(false),
                     Some(true)),
                  ]),
            ("insert",
                    Some(r##"Creates a playlist."##),
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
                    Some(r##"Returns a collection of playlists that match the API request parameters. For example, you can retrieve all playlists that the authenticated user owns, or you can retrieve one or more playlists by their unique IDs."##),
                    "Details at http://byron.github.io/google-apis-rs/google_youtube3_cli/playlists_list",
                  vec![
                    (Some(r##"part"##),
                     None,
                     Some(r##"The part parameter specifies a comma-separated list of one or more playlist resource properties that the API response will include.
        
        If the parameter identifies a property that contains child properties, the child properties will be included in the response. For example, in a playlist resource, the snippet property contains properties like author, title, description, tags, and timeCreated. As such, if you set part=snippet, the API response will contain all of those properties."##),
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
                    Some(r##"Modifies a playlist. For example, you could change a playlist's title, description, or privacy status."##),
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
                    Some(r##"Returns a collection of search results that match the query parameters specified in the API request. By default, a search result set identifies matching video, channel, and playlist resources, but you can also configure queries to only retrieve a specific type of resource."##),
                    "Details at http://byron.github.io/google-apis-rs/google_youtube3_cli/search_list",
                  vec![
                    (Some(r##"part"##),
                     None,
                     Some(r##"The part parameter specifies a comma-separated list of one or more search resource properties that the API response will include. Set the parameter value to snippet."##),
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
        
        ("sponsors", "methods: 'list'", vec![
            ("list",
                    Some(r##"Lists sponsors for a channel."##),
                    "Details at http://byron.github.io/google-apis-rs/google_youtube3_cli/sponsors_list",
                  vec![
                    (Some(r##"part"##),
                     None,
                     Some(r##"The part parameter specifies the sponsor resource parts that the API response will include. Supported values are id and snippet."##),
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
                    Some(r##"Deletes a subscription."##),
                    "Details at http://byron.github.io/google-apis-rs/google_youtube3_cli/subscriptions_delete",
                  vec![
                    (Some(r##"id"##),
                     None,
                     Some(r##"The id parameter specifies the YouTube subscription ID for the resource that is being deleted. In a subscription resource, the id property specifies the YouTube subscription ID."##),
                     Some(true),
                     Some(false)),
        
                    (Some(r##"v"##),
                     Some(r##"p"##),
                     Some(r##"Set various optional parameters, matching the key=value form"##),
                     Some(false),
                     Some(true)),
                  ]),
            ("insert",
                    Some(r##"Adds a subscription for the authenticated user's channel."##),
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
                    Some(r##"Returns subscription resources that match the API request criteria."##),
                    "Details at http://byron.github.io/google-apis-rs/google_youtube3_cli/subscriptions_list",
                  vec![
                    (Some(r##"part"##),
                     None,
                     Some(r##"The part parameter specifies a comma-separated list of one or more subscription resource properties that the API response will include.
        
        If the parameter identifies a property that contains child properties, the child properties will be included in the response. For example, in a subscription resource, the snippet property contains other properties, such as a display title for the subscription. If you set part=snippet, the API response will also contain all of those nested properties."##),
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
                    Some(r##"Lists Super Chat events for a channel."##),
                    "Details at http://byron.github.io/google-apis-rs/google_youtube3_cli/super-chat-events_list",
                  vec![
                    (Some(r##"part"##),
                     None,
                     Some(r##"The part parameter specifies the superChatEvent resource parts that the API response will include. Supported values are id and snippet."##),
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
        
        ("thumbnails", "methods: 'set'", vec![
            ("set",
                    Some(r##"Uploads a custom video thumbnail to YouTube and sets it for a video."##),
                    "Details at http://byron.github.io/google-apis-rs/google_youtube3_cli/thumbnails_set",
                  vec![
                    (Some(r##"video-id"##),
                     None,
                     Some(r##"The videoId parameter specifies a YouTube video ID for which the custom video thumbnail is being provided."##),
                     Some(true),
                     Some(false)),
        
                    (Some(r##"mode"##),
                     Some(r##"u"##),
                     Some(r##"Specify the upload protocol (simple|resumable) and the file to upload"##),
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
                    Some(r##"Returns a list of abuse reasons that can be used for reporting abusive videos."##),
                    "Details at http://byron.github.io/google-apis-rs/google_youtube3_cli/video-abuse-report-reasons_list",
                  vec![
                    (Some(r##"part"##),
                     None,
                     Some(r##"The part parameter specifies the videoCategory resource parts that the API response will include. Supported values are id and snippet."##),
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
                    Some(r##"Returns a list of categories that can be associated with YouTube videos."##),
                    "Details at http://byron.github.io/google-apis-rs/google_youtube3_cli/video-categories_list",
                  vec![
                    (Some(r##"part"##),
                     None,
                     Some(r##"The part parameter specifies the videoCategory resource properties that the API response will include. Set the parameter value to snippet."##),
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
                    Some(r##"Deletes a YouTube video."##),
                    "Details at http://byron.github.io/google-apis-rs/google_youtube3_cli/videos_delete",
                  vec![
                    (Some(r##"id"##),
                     None,
                     Some(r##"The id parameter specifies the YouTube video ID for the resource that is being deleted. In a video resource, the id property specifies the video's ID."##),
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
                     Some(r##"The id parameter specifies a comma-separated list of the YouTube video ID(s) for the resource(s) for which you are retrieving rating data. In a video resource, the id property specifies the video's ID."##),
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
                    Some(r##"Uploads a video to YouTube and optionally sets the video's metadata."##),
                    "Details at http://byron.github.io/google-apis-rs/google_youtube3_cli/videos_insert",
                  vec![
                    (Some(r##"kv"##),
                     Some(r##"r"##),
                     Some(r##"Set various fields of the request structure, matching the key=value form"##),
                     Some(true),
                     Some(true)),
        
                    (Some(r##"mode"##),
                     Some(r##"u"##),
                     Some(r##"Specify the upload protocol (simple|resumable) and the file to upload"##),
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
                    Some(r##"Returns a list of videos that match the API request parameters."##),
                    "Details at http://byron.github.io/google-apis-rs/google_youtube3_cli/videos_list",
                  vec![
                    (Some(r##"part"##),
                     None,
                     Some(r##"The part parameter specifies a comma-separated list of one or more video resource properties that the API response will include.
        
        If the parameter identifies a property that contains child properties, the child properties will be included in the response. For example, in a video resource, the snippet property contains the channelId, title, description, tags, and categoryId properties. As such, if you set part=snippet, the API response will contain all of those properties."##),
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
                    Some(r##"Add a like or dislike rating to a video or remove a rating from a video."##),
                    "Details at http://byron.github.io/google-apis-rs/google_youtube3_cli/videos_rate",
                  vec![
                    (Some(r##"id"##),
                     None,
                     Some(r##"The id parameter specifies the YouTube video ID of the video that is being rated or having its rating removed."##),
                     Some(true),
                     Some(false)),
        
                    (Some(r##"rating"##),
                     None,
                     Some(r##"Specifies the rating to record."##),
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
                    Some(r##"Updates a video's metadata."##),
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
                    Some(r##"Uploads a watermark image to YouTube and sets it for a channel."##),
                    "Details at http://byron.github.io/google-apis-rs/google_youtube3_cli/watermarks_set",
                  vec![
                    (Some(r##"channel-id"##),
                     None,
                     Some(r##"The channelId parameter specifies the YouTube channel ID for which the watermark is being provided."##),
                     Some(true),
                     Some(false)),
        
                    (Some(r##"kv"##),
                     Some(r##"r"##),
                     Some(r##"Set various fields of the request structure, matching the key=value form"##),
                     Some(true),
                     Some(true)),
        
                    (Some(r##"mode"##),
                     Some(r##"u"##),
                     Some(r##"Specify the upload protocol (simple|resumable) and the file to upload"##),
                     Some(true),
                     Some(true)),
        
                    (Some(r##"v"##),
                     Some(r##"p"##),
                     Some(r##"Set various optional parameters, matching the key=value form"##),
                     Some(false),
                     Some(true)),
                  ]),
            ("unset",
                    Some(r##"Deletes a channel's watermark image."##),
                    "Details at http://byron.github.io/google-apis-rs/google_youtube3_cli/watermarks_unset",
                  vec![
                    (Some(r##"channel-id"##),
                     None,
                     Some(r##"The channelId parameter specifies the YouTube channel ID for which the watermark is being unset."##),
                     Some(true),
                     Some(false)),
        
                    (Some(r##"v"##),
                     Some(r##"p"##),
                     Some(r##"Set various optional parameters, matching the key=value form"##),
                     Some(false),
                     Some(true)),
                  ]),
            ]),
        
    ];
    
    let mut app = App::new("youtube3")
           .author("Sebastian Thiel <byronimo@gmail.com>")
           .version("1.0.7+20180511")
           .about("Supports core YouTube features, such as uploading videos, creating and managing playlists, searching for content, and much more.")
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