// DO NOT EDIT !
// This file was generated automatically from 'src/mako/cli/main.rs.mako'
// DO NOT EDIT !
#![feature(plugin, exit_status)]
#![allow(unused_variables, unused_imports, dead_code, unused_mut)]

#[macro_use]
extern crate clap;
extern crate yup_oauth2 as oauth2;
extern crate yup_hyper_mock as mock;
extern crate serde;
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
          calltype_from_str, remove_json_null_values};

use std::default::Default;
use std::str::FromStr;

use oauth2::{Authenticator, DefaultAuthenticatorDelegate};
use serde::json;
use clap::ArgMatches;

enum DoitError {
    IoError(String, io::Error),
    ApiError(api::Error),
}

struct Engine<'n, 'a> {
    opt: ArgMatches<'n, 'a>,
    hub: api::YouTube<hyper::Client, Authenticator<DefaultAuthenticatorDelegate, JsonTokenStorage, hyper::Client>>,
    gp: Vec<&'static str>,
    gpm: Vec<(&'static str, &'static str)>,
}


impl<'n, 'a> Engine<'n, 'a> {
    fn _activities_insert(&self, opt: &ArgMatches<'n, 'a>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        
        let mut request = api::Activity::default();
        let mut field_cursor = FieldCursor::default();
        for kvarg in opt.values_of("kv").unwrap_or(Vec::new()).iter() {
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
            fn request_content_details_bulletin_init(request: &mut api::Activity) {
                request_content_details_init(request);
                if request.content_details.as_mut().unwrap().bulletin.is_none() {
                    request.content_details.as_mut().unwrap().bulletin = Some(Default::default());
                }
            }
            
            fn request_content_details_bulletin_resource_id_init(request: &mut api::Activity) {
                request_content_details_bulletin_init(request);
                if request.content_details.as_mut().unwrap().bulletin.as_mut().unwrap().resource_id.is_none() {
                    request.content_details.as_mut().unwrap().bulletin.as_mut().unwrap().resource_id = Some(Default::default());
                }
            }
            
            fn request_content_details_channel_item_init(request: &mut api::Activity) {
                request_content_details_init(request);
                if request.content_details.as_mut().unwrap().channel_item.is_none() {
                    request.content_details.as_mut().unwrap().channel_item = Some(Default::default());
                }
            }
            
            fn request_content_details_channel_item_resource_id_init(request: &mut api::Activity) {
                request_content_details_channel_item_init(request);
                if request.content_details.as_mut().unwrap().channel_item.as_mut().unwrap().resource_id.is_none() {
                    request.content_details.as_mut().unwrap().channel_item.as_mut().unwrap().resource_id = Some(Default::default());
                }
            }
            
            fn request_content_details_comment_init(request: &mut api::Activity) {
                request_content_details_init(request);
                if request.content_details.as_mut().unwrap().comment.is_none() {
                    request.content_details.as_mut().unwrap().comment = Some(Default::default());
                }
            }
            
            fn request_content_details_comment_resource_id_init(request: &mut api::Activity) {
                request_content_details_comment_init(request);
                if request.content_details.as_mut().unwrap().comment.as_mut().unwrap().resource_id.is_none() {
                    request.content_details.as_mut().unwrap().comment.as_mut().unwrap().resource_id = Some(Default::default());
                }
            }
            
            fn request_content_details_favorite_init(request: &mut api::Activity) {
                request_content_details_init(request);
                if request.content_details.as_mut().unwrap().favorite.is_none() {
                    request.content_details.as_mut().unwrap().favorite = Some(Default::default());
                }
            }
            
            fn request_content_details_favorite_resource_id_init(request: &mut api::Activity) {
                request_content_details_favorite_init(request);
                if request.content_details.as_mut().unwrap().favorite.as_mut().unwrap().resource_id.is_none() {
                    request.content_details.as_mut().unwrap().favorite.as_mut().unwrap().resource_id = Some(Default::default());
                }
            }
            
            fn request_content_details_init(request: &mut api::Activity) {
                if request.content_details.is_none() {
                    request.content_details = Some(Default::default());
                }
            }
            
            fn request_content_details_like_init(request: &mut api::Activity) {
                request_content_details_init(request);
                if request.content_details.as_mut().unwrap().like.is_none() {
                    request.content_details.as_mut().unwrap().like = Some(Default::default());
                }
            }
            
            fn request_content_details_like_resource_id_init(request: &mut api::Activity) {
                request_content_details_like_init(request);
                if request.content_details.as_mut().unwrap().like.as_mut().unwrap().resource_id.is_none() {
                    request.content_details.as_mut().unwrap().like.as_mut().unwrap().resource_id = Some(Default::default());
                }
            }
            
            fn request_content_details_playlist_item_init(request: &mut api::Activity) {
                request_content_details_init(request);
                if request.content_details.as_mut().unwrap().playlist_item.is_none() {
                    request.content_details.as_mut().unwrap().playlist_item = Some(Default::default());
                }
            }
            
            fn request_content_details_playlist_item_resource_id_init(request: &mut api::Activity) {
                request_content_details_playlist_item_init(request);
                if request.content_details.as_mut().unwrap().playlist_item.as_mut().unwrap().resource_id.is_none() {
                    request.content_details.as_mut().unwrap().playlist_item.as_mut().unwrap().resource_id = Some(Default::default());
                }
            }
            
            fn request_content_details_promoted_item_init(request: &mut api::Activity) {
                request_content_details_init(request);
                if request.content_details.as_mut().unwrap().promoted_item.is_none() {
                    request.content_details.as_mut().unwrap().promoted_item = Some(Default::default());
                }
            }
            
            fn request_content_details_recommendation_init(request: &mut api::Activity) {
                request_content_details_init(request);
                if request.content_details.as_mut().unwrap().recommendation.is_none() {
                    request.content_details.as_mut().unwrap().recommendation = Some(Default::default());
                }
            }
            
            fn request_content_details_recommendation_resource_id_init(request: &mut api::Activity) {
                request_content_details_recommendation_init(request);
                if request.content_details.as_mut().unwrap().recommendation.as_mut().unwrap().resource_id.is_none() {
                    request.content_details.as_mut().unwrap().recommendation.as_mut().unwrap().resource_id = Some(Default::default());
                }
            }
            
            fn request_content_details_recommendation_seed_resource_id_init(request: &mut api::Activity) {
                request_content_details_recommendation_init(request);
                if request.content_details.as_mut().unwrap().recommendation.as_mut().unwrap().seed_resource_id.is_none() {
                    request.content_details.as_mut().unwrap().recommendation.as_mut().unwrap().seed_resource_id = Some(Default::default());
                }
            }
            
            fn request_content_details_social_init(request: &mut api::Activity) {
                request_content_details_init(request);
                if request.content_details.as_mut().unwrap().social.is_none() {
                    request.content_details.as_mut().unwrap().social = Some(Default::default());
                }
            }
            
            fn request_content_details_social_resource_id_init(request: &mut api::Activity) {
                request_content_details_social_init(request);
                if request.content_details.as_mut().unwrap().social.as_mut().unwrap().resource_id.is_none() {
                    request.content_details.as_mut().unwrap().social.as_mut().unwrap().resource_id = Some(Default::default());
                }
            }
            
            fn request_content_details_subscription_init(request: &mut api::Activity) {
                request_content_details_init(request);
                if request.content_details.as_mut().unwrap().subscription.is_none() {
                    request.content_details.as_mut().unwrap().subscription = Some(Default::default());
                }
            }
            
            fn request_content_details_subscription_resource_id_init(request: &mut api::Activity) {
                request_content_details_subscription_init(request);
                if request.content_details.as_mut().unwrap().subscription.as_mut().unwrap().resource_id.is_none() {
                    request.content_details.as_mut().unwrap().subscription.as_mut().unwrap().resource_id = Some(Default::default());
                }
            }
            
            fn request_content_details_upload_init(request: &mut api::Activity) {
                request_content_details_init(request);
                if request.content_details.as_mut().unwrap().upload.is_none() {
                    request.content_details.as_mut().unwrap().upload = Some(Default::default());
                }
            }
            
            fn request_snippet_init(request: &mut api::Activity) {
                if request.snippet.is_none() {
                    request.snippet = Some(Default::default());
                }
            }
            
            fn request_snippet_thumbnails_default_init(request: &mut api::Activity) {
                request_snippet_thumbnails_init(request);
                if request.snippet.as_mut().unwrap().thumbnails.as_mut().unwrap().default.is_none() {
                    request.snippet.as_mut().unwrap().thumbnails.as_mut().unwrap().default = Some(Default::default());
                }
            }
            
            fn request_snippet_thumbnails_high_init(request: &mut api::Activity) {
                request_snippet_thumbnails_init(request);
                if request.snippet.as_mut().unwrap().thumbnails.as_mut().unwrap().high.is_none() {
                    request.snippet.as_mut().unwrap().thumbnails.as_mut().unwrap().high = Some(Default::default());
                }
            }
            
            fn request_snippet_thumbnails_init(request: &mut api::Activity) {
                request_snippet_init(request);
                if request.snippet.as_mut().unwrap().thumbnails.is_none() {
                    request.snippet.as_mut().unwrap().thumbnails = Some(Default::default());
                }
            }
            
            fn request_snippet_thumbnails_maxres_init(request: &mut api::Activity) {
                request_snippet_thumbnails_init(request);
                if request.snippet.as_mut().unwrap().thumbnails.as_mut().unwrap().maxres.is_none() {
                    request.snippet.as_mut().unwrap().thumbnails.as_mut().unwrap().maxres = Some(Default::default());
                }
            }
            
            fn request_snippet_thumbnails_medium_init(request: &mut api::Activity) {
                request_snippet_thumbnails_init(request);
                if request.snippet.as_mut().unwrap().thumbnails.as_mut().unwrap().medium.is_none() {
                    request.snippet.as_mut().unwrap().thumbnails.as_mut().unwrap().medium = Some(Default::default());
                }
            }
            
            fn request_snippet_thumbnails_standard_init(request: &mut api::Activity) {
                request_snippet_thumbnails_init(request);
                if request.snippet.as_mut().unwrap().thumbnails.as_mut().unwrap().standard.is_none() {
                    request.snippet.as_mut().unwrap().thumbnails.as_mut().unwrap().standard = Some(Default::default());
                }
            }
            
            match &temp_cursor.to_string()[..] {
                "snippet.thumbnails.default.url" => {
                        request_snippet_thumbnails_default_init(&mut request);
                        request.snippet.as_mut().unwrap().thumbnails.as_mut().unwrap().default.as_mut().unwrap().url = Some(value.unwrap_or("").to_string());
                    },
                "snippet.thumbnails.default.width" => {
                        request_snippet_thumbnails_default_init(&mut request);
                        request.snippet.as_mut().unwrap().thumbnails.as_mut().unwrap().default.as_mut().unwrap().width = Some(arg_from_str(value.unwrap_or("-0"), err, "snippet.thumbnails.default.width", "integer"));
                    },
                "snippet.thumbnails.default.height" => {
                        request_snippet_thumbnails_default_init(&mut request);
                        request.snippet.as_mut().unwrap().thumbnails.as_mut().unwrap().default.as_mut().unwrap().height = Some(arg_from_str(value.unwrap_or("-0"), err, "snippet.thumbnails.default.height", "integer"));
                    },
                "snippet.thumbnails.high.url" => {
                        request_snippet_thumbnails_high_init(&mut request);
                        request.snippet.as_mut().unwrap().thumbnails.as_mut().unwrap().high.as_mut().unwrap().url = Some(value.unwrap_or("").to_string());
                    },
                "snippet.thumbnails.high.width" => {
                        request_snippet_thumbnails_high_init(&mut request);
                        request.snippet.as_mut().unwrap().thumbnails.as_mut().unwrap().high.as_mut().unwrap().width = Some(arg_from_str(value.unwrap_or("-0"), err, "snippet.thumbnails.high.width", "integer"));
                    },
                "snippet.thumbnails.high.height" => {
                        request_snippet_thumbnails_high_init(&mut request);
                        request.snippet.as_mut().unwrap().thumbnails.as_mut().unwrap().high.as_mut().unwrap().height = Some(arg_from_str(value.unwrap_or("-0"), err, "snippet.thumbnails.high.height", "integer"));
                    },
                "snippet.thumbnails.medium.url" => {
                        request_snippet_thumbnails_medium_init(&mut request);
                        request.snippet.as_mut().unwrap().thumbnails.as_mut().unwrap().medium.as_mut().unwrap().url = Some(value.unwrap_or("").to_string());
                    },
                "snippet.thumbnails.medium.width" => {
                        request_snippet_thumbnails_medium_init(&mut request);
                        request.snippet.as_mut().unwrap().thumbnails.as_mut().unwrap().medium.as_mut().unwrap().width = Some(arg_from_str(value.unwrap_or("-0"), err, "snippet.thumbnails.medium.width", "integer"));
                    },
                "snippet.thumbnails.medium.height" => {
                        request_snippet_thumbnails_medium_init(&mut request);
                        request.snippet.as_mut().unwrap().thumbnails.as_mut().unwrap().medium.as_mut().unwrap().height = Some(arg_from_str(value.unwrap_or("-0"), err, "snippet.thumbnails.medium.height", "integer"));
                    },
                "snippet.thumbnails.maxres.url" => {
                        request_snippet_thumbnails_maxres_init(&mut request);
                        request.snippet.as_mut().unwrap().thumbnails.as_mut().unwrap().maxres.as_mut().unwrap().url = Some(value.unwrap_or("").to_string());
                    },
                "snippet.thumbnails.maxres.width" => {
                        request_snippet_thumbnails_maxres_init(&mut request);
                        request.snippet.as_mut().unwrap().thumbnails.as_mut().unwrap().maxres.as_mut().unwrap().width = Some(arg_from_str(value.unwrap_or("-0"), err, "snippet.thumbnails.maxres.width", "integer"));
                    },
                "snippet.thumbnails.maxres.height" => {
                        request_snippet_thumbnails_maxres_init(&mut request);
                        request.snippet.as_mut().unwrap().thumbnails.as_mut().unwrap().maxres.as_mut().unwrap().height = Some(arg_from_str(value.unwrap_or("-0"), err, "snippet.thumbnails.maxres.height", "integer"));
                    },
                "snippet.thumbnails.standard.url" => {
                        request_snippet_thumbnails_standard_init(&mut request);
                        request.snippet.as_mut().unwrap().thumbnails.as_mut().unwrap().standard.as_mut().unwrap().url = Some(value.unwrap_or("").to_string());
                    },
                "snippet.thumbnails.standard.width" => {
                        request_snippet_thumbnails_standard_init(&mut request);
                        request.snippet.as_mut().unwrap().thumbnails.as_mut().unwrap().standard.as_mut().unwrap().width = Some(arg_from_str(value.unwrap_or("-0"), err, "snippet.thumbnails.standard.width", "integer"));
                    },
                "snippet.thumbnails.standard.height" => {
                        request_snippet_thumbnails_standard_init(&mut request);
                        request.snippet.as_mut().unwrap().thumbnails.as_mut().unwrap().standard.as_mut().unwrap().height = Some(arg_from_str(value.unwrap_or("-0"), err, "snippet.thumbnails.standard.height", "integer"));
                    },
                "snippet.title" => {
                        request_snippet_thumbnails_init(&mut request);
                        request.snippet.as_mut().unwrap().title = Some(value.unwrap_or("").to_string());
                    },
                "snippet.channel-id" => {
                        request_snippet_thumbnails_init(&mut request);
                        request.snippet.as_mut().unwrap().channel_id = Some(value.unwrap_or("").to_string());
                    },
                "snippet.published-at" => {
                        request_snippet_thumbnails_init(&mut request);
                        request.snippet.as_mut().unwrap().published_at = Some(value.unwrap_or("").to_string());
                    },
                "snippet.channel-title" => {
                        request_snippet_thumbnails_init(&mut request);
                        request.snippet.as_mut().unwrap().channel_title = Some(value.unwrap_or("").to_string());
                    },
                "snippet.type" => {
                        request_snippet_thumbnails_init(&mut request);
                        request.snippet.as_mut().unwrap().type_ = Some(value.unwrap_or("").to_string());
                    },
                "snippet.group-id" => {
                        request_snippet_thumbnails_init(&mut request);
                        request.snippet.as_mut().unwrap().group_id = Some(value.unwrap_or("").to_string());
                    },
                "snippet.description" => {
                        request_snippet_thumbnails_init(&mut request);
                        request.snippet.as_mut().unwrap().description = Some(value.unwrap_or("").to_string());
                    },
                "content-details.comment.resource-id.kind" => {
                        request_content_details_comment_resource_id_init(&mut request);
                        request.content_details.as_mut().unwrap().comment.as_mut().unwrap().resource_id.as_mut().unwrap().kind = Some(value.unwrap_or("").to_string());
                    },
                "content-details.comment.resource-id.channel-id" => {
                        request_content_details_comment_resource_id_init(&mut request);
                        request.content_details.as_mut().unwrap().comment.as_mut().unwrap().resource_id.as_mut().unwrap().channel_id = Some(value.unwrap_or("").to_string());
                    },
                "content-details.comment.resource-id.playlist-id" => {
                        request_content_details_comment_resource_id_init(&mut request);
                        request.content_details.as_mut().unwrap().comment.as_mut().unwrap().resource_id.as_mut().unwrap().playlist_id = Some(value.unwrap_or("").to_string());
                    },
                "content-details.comment.resource-id.video-id" => {
                        request_content_details_comment_resource_id_init(&mut request);
                        request.content_details.as_mut().unwrap().comment.as_mut().unwrap().resource_id.as_mut().unwrap().video_id = Some(value.unwrap_or("").to_string());
                    },
                "content-details.playlist-item.resource-id.kind" => {
                        request_content_details_playlist_item_resource_id_init(&mut request);
                        request.content_details.as_mut().unwrap().playlist_item.as_mut().unwrap().resource_id.as_mut().unwrap().kind = Some(value.unwrap_or("").to_string());
                    },
                "content-details.playlist-item.resource-id.channel-id" => {
                        request_content_details_playlist_item_resource_id_init(&mut request);
                        request.content_details.as_mut().unwrap().playlist_item.as_mut().unwrap().resource_id.as_mut().unwrap().channel_id = Some(value.unwrap_or("").to_string());
                    },
                "content-details.playlist-item.resource-id.playlist-id" => {
                        request_content_details_playlist_item_resource_id_init(&mut request);
                        request.content_details.as_mut().unwrap().playlist_item.as_mut().unwrap().resource_id.as_mut().unwrap().playlist_id = Some(value.unwrap_or("").to_string());
                    },
                "content-details.playlist-item.resource-id.video-id" => {
                        request_content_details_playlist_item_resource_id_init(&mut request);
                        request.content_details.as_mut().unwrap().playlist_item.as_mut().unwrap().resource_id.as_mut().unwrap().video_id = Some(value.unwrap_or("").to_string());
                    },
                "content-details.playlist-item.playlist-id" => {
                        request_content_details_playlist_item_resource_id_init(&mut request);
                        request.content_details.as_mut().unwrap().playlist_item.as_mut().unwrap().playlist_id = Some(value.unwrap_or("").to_string());
                    },
                "content-details.playlist-item.playlist-item-id" => {
                        request_content_details_playlist_item_resource_id_init(&mut request);
                        request.content_details.as_mut().unwrap().playlist_item.as_mut().unwrap().playlist_item_id = Some(value.unwrap_or("").to_string());
                    },
                "content-details.like.resource-id.kind" => {
                        request_content_details_like_resource_id_init(&mut request);
                        request.content_details.as_mut().unwrap().like.as_mut().unwrap().resource_id.as_mut().unwrap().kind = Some(value.unwrap_or("").to_string());
                    },
                "content-details.like.resource-id.channel-id" => {
                        request_content_details_like_resource_id_init(&mut request);
                        request.content_details.as_mut().unwrap().like.as_mut().unwrap().resource_id.as_mut().unwrap().channel_id = Some(value.unwrap_or("").to_string());
                    },
                "content-details.like.resource-id.playlist-id" => {
                        request_content_details_like_resource_id_init(&mut request);
                        request.content_details.as_mut().unwrap().like.as_mut().unwrap().resource_id.as_mut().unwrap().playlist_id = Some(value.unwrap_or("").to_string());
                    },
                "content-details.like.resource-id.video-id" => {
                        request_content_details_like_resource_id_init(&mut request);
                        request.content_details.as_mut().unwrap().like.as_mut().unwrap().resource_id.as_mut().unwrap().video_id = Some(value.unwrap_or("").to_string());
                    },
                "content-details.promoted-item.cta-type" => {
                        request_content_details_promoted_item_init(&mut request);
                        request.content_details.as_mut().unwrap().promoted_item.as_mut().unwrap().cta_type = Some(value.unwrap_or("").to_string());
                    },
                "content-details.promoted-item.ad-tag" => {
                        request_content_details_promoted_item_init(&mut request);
                        request.content_details.as_mut().unwrap().promoted_item.as_mut().unwrap().ad_tag = Some(value.unwrap_or("").to_string());
                    },
                "content-details.promoted-item.destination-url" => {
                        request_content_details_promoted_item_init(&mut request);
                        request.content_details.as_mut().unwrap().promoted_item.as_mut().unwrap().destination_url = Some(value.unwrap_or("").to_string());
                    },
                "content-details.promoted-item.forecasting-url" => {
                        request_content_details_promoted_item_init(&mut request);
                        if request.content_details.as_mut().unwrap().promoted_item.as_mut().unwrap().forecasting_url.is_none() {
                           request.content_details.as_mut().unwrap().promoted_item.as_mut().unwrap().forecasting_url = Some(Default::default());
                        }
                                        request.content_details.as_mut().unwrap().promoted_item.as_mut().unwrap().forecasting_url.as_mut().unwrap().push(value.unwrap_or("").to_string());
                    },
                "content-details.promoted-item.impression-url" => {
                        request_content_details_promoted_item_init(&mut request);
                        if request.content_details.as_mut().unwrap().promoted_item.as_mut().unwrap().impression_url.is_none() {
                           request.content_details.as_mut().unwrap().promoted_item.as_mut().unwrap().impression_url = Some(Default::default());
                        }
                                        request.content_details.as_mut().unwrap().promoted_item.as_mut().unwrap().impression_url.as_mut().unwrap().push(value.unwrap_or("").to_string());
                    },
                "content-details.promoted-item.creative-view-url" => {
                        request_content_details_promoted_item_init(&mut request);
                        request.content_details.as_mut().unwrap().promoted_item.as_mut().unwrap().creative_view_url = Some(value.unwrap_or("").to_string());
                    },
                "content-details.promoted-item.video-id" => {
                        request_content_details_promoted_item_init(&mut request);
                        request.content_details.as_mut().unwrap().promoted_item.as_mut().unwrap().video_id = Some(value.unwrap_or("").to_string());
                    },
                "content-details.promoted-item.description-text" => {
                        request_content_details_promoted_item_init(&mut request);
                        request.content_details.as_mut().unwrap().promoted_item.as_mut().unwrap().description_text = Some(value.unwrap_or("").to_string());
                    },
                "content-details.promoted-item.custom-cta-button-text" => {
                        request_content_details_promoted_item_init(&mut request);
                        request.content_details.as_mut().unwrap().promoted_item.as_mut().unwrap().custom_cta_button_text = Some(value.unwrap_or("").to_string());
                    },
                "content-details.promoted-item.click-tracking-url" => {
                        request_content_details_promoted_item_init(&mut request);
                        request.content_details.as_mut().unwrap().promoted_item.as_mut().unwrap().click_tracking_url = Some(value.unwrap_or("").to_string());
                    },
                "content-details.social.resource-id.kind" => {
                        request_content_details_social_resource_id_init(&mut request);
                        request.content_details.as_mut().unwrap().social.as_mut().unwrap().resource_id.as_mut().unwrap().kind = Some(value.unwrap_or("").to_string());
                    },
                "content-details.social.resource-id.channel-id" => {
                        request_content_details_social_resource_id_init(&mut request);
                        request.content_details.as_mut().unwrap().social.as_mut().unwrap().resource_id.as_mut().unwrap().channel_id = Some(value.unwrap_or("").to_string());
                    },
                "content-details.social.resource-id.playlist-id" => {
                        request_content_details_social_resource_id_init(&mut request);
                        request.content_details.as_mut().unwrap().social.as_mut().unwrap().resource_id.as_mut().unwrap().playlist_id = Some(value.unwrap_or("").to_string());
                    },
                "content-details.social.resource-id.video-id" => {
                        request_content_details_social_resource_id_init(&mut request);
                        request.content_details.as_mut().unwrap().social.as_mut().unwrap().resource_id.as_mut().unwrap().video_id = Some(value.unwrap_or("").to_string());
                    },
                "content-details.social.image-url" => {
                        request_content_details_social_resource_id_init(&mut request);
                        request.content_details.as_mut().unwrap().social.as_mut().unwrap().image_url = Some(value.unwrap_or("").to_string());
                    },
                "content-details.social.type" => {
                        request_content_details_social_resource_id_init(&mut request);
                        request.content_details.as_mut().unwrap().social.as_mut().unwrap().type_ = Some(value.unwrap_or("").to_string());
                    },
                "content-details.social.reference-url" => {
                        request_content_details_social_resource_id_init(&mut request);
                        request.content_details.as_mut().unwrap().social.as_mut().unwrap().reference_url = Some(value.unwrap_or("").to_string());
                    },
                "content-details.social.author" => {
                        request_content_details_social_resource_id_init(&mut request);
                        request.content_details.as_mut().unwrap().social.as_mut().unwrap().author = Some(value.unwrap_or("").to_string());
                    },
                "content-details.favorite.resource-id.kind" => {
                        request_content_details_favorite_resource_id_init(&mut request);
                        request.content_details.as_mut().unwrap().favorite.as_mut().unwrap().resource_id.as_mut().unwrap().kind = Some(value.unwrap_or("").to_string());
                    },
                "content-details.favorite.resource-id.channel-id" => {
                        request_content_details_favorite_resource_id_init(&mut request);
                        request.content_details.as_mut().unwrap().favorite.as_mut().unwrap().resource_id.as_mut().unwrap().channel_id = Some(value.unwrap_or("").to_string());
                    },
                "content-details.favorite.resource-id.playlist-id" => {
                        request_content_details_favorite_resource_id_init(&mut request);
                        request.content_details.as_mut().unwrap().favorite.as_mut().unwrap().resource_id.as_mut().unwrap().playlist_id = Some(value.unwrap_or("").to_string());
                    },
                "content-details.favorite.resource-id.video-id" => {
                        request_content_details_favorite_resource_id_init(&mut request);
                        request.content_details.as_mut().unwrap().favorite.as_mut().unwrap().resource_id.as_mut().unwrap().video_id = Some(value.unwrap_or("").to_string());
                    },
                "content-details.upload.video-id" => {
                        request_content_details_upload_init(&mut request);
                        request.content_details.as_mut().unwrap().upload.as_mut().unwrap().video_id = Some(value.unwrap_or("").to_string());
                    },
                "content-details.recommendation.resource-id.kind" => {
                        request_content_details_recommendation_resource_id_init(&mut request);
                        request.content_details.as_mut().unwrap().recommendation.as_mut().unwrap().resource_id.as_mut().unwrap().kind = Some(value.unwrap_or("").to_string());
                    },
                "content-details.recommendation.resource-id.channel-id" => {
                        request_content_details_recommendation_resource_id_init(&mut request);
                        request.content_details.as_mut().unwrap().recommendation.as_mut().unwrap().resource_id.as_mut().unwrap().channel_id = Some(value.unwrap_or("").to_string());
                    },
                "content-details.recommendation.resource-id.playlist-id" => {
                        request_content_details_recommendation_resource_id_init(&mut request);
                        request.content_details.as_mut().unwrap().recommendation.as_mut().unwrap().resource_id.as_mut().unwrap().playlist_id = Some(value.unwrap_or("").to_string());
                    },
                "content-details.recommendation.resource-id.video-id" => {
                        request_content_details_recommendation_resource_id_init(&mut request);
                        request.content_details.as_mut().unwrap().recommendation.as_mut().unwrap().resource_id.as_mut().unwrap().video_id = Some(value.unwrap_or("").to_string());
                    },
                "content-details.recommendation.reason" => {
                        request_content_details_recommendation_resource_id_init(&mut request);
                        request.content_details.as_mut().unwrap().recommendation.as_mut().unwrap().reason = Some(value.unwrap_or("").to_string());
                    },
                "content-details.recommendation.seed-resource-id.kind" => {
                        request_content_details_recommendation_seed_resource_id_init(&mut request);
                        request.content_details.as_mut().unwrap().recommendation.as_mut().unwrap().seed_resource_id.as_mut().unwrap().kind = Some(value.unwrap_or("").to_string());
                    },
                "content-details.recommendation.seed-resource-id.channel-id" => {
                        request_content_details_recommendation_seed_resource_id_init(&mut request);
                        request.content_details.as_mut().unwrap().recommendation.as_mut().unwrap().seed_resource_id.as_mut().unwrap().channel_id = Some(value.unwrap_or("").to_string());
                    },
                "content-details.recommendation.seed-resource-id.playlist-id" => {
                        request_content_details_recommendation_seed_resource_id_init(&mut request);
                        request.content_details.as_mut().unwrap().recommendation.as_mut().unwrap().seed_resource_id.as_mut().unwrap().playlist_id = Some(value.unwrap_or("").to_string());
                    },
                "content-details.recommendation.seed-resource-id.video-id" => {
                        request_content_details_recommendation_seed_resource_id_init(&mut request);
                        request.content_details.as_mut().unwrap().recommendation.as_mut().unwrap().seed_resource_id.as_mut().unwrap().video_id = Some(value.unwrap_or("").to_string());
                    },
                "content-details.subscription.resource-id.kind" => {
                        request_content_details_subscription_resource_id_init(&mut request);
                        request.content_details.as_mut().unwrap().subscription.as_mut().unwrap().resource_id.as_mut().unwrap().kind = Some(value.unwrap_or("").to_string());
                    },
                "content-details.subscription.resource-id.channel-id" => {
                        request_content_details_subscription_resource_id_init(&mut request);
                        request.content_details.as_mut().unwrap().subscription.as_mut().unwrap().resource_id.as_mut().unwrap().channel_id = Some(value.unwrap_or("").to_string());
                    },
                "content-details.subscription.resource-id.playlist-id" => {
                        request_content_details_subscription_resource_id_init(&mut request);
                        request.content_details.as_mut().unwrap().subscription.as_mut().unwrap().resource_id.as_mut().unwrap().playlist_id = Some(value.unwrap_or("").to_string());
                    },
                "content-details.subscription.resource-id.video-id" => {
                        request_content_details_subscription_resource_id_init(&mut request);
                        request.content_details.as_mut().unwrap().subscription.as_mut().unwrap().resource_id.as_mut().unwrap().video_id = Some(value.unwrap_or("").to_string());
                    },
                "content-details.bulletin.resource-id.kind" => {
                        request_content_details_bulletin_resource_id_init(&mut request);
                        request.content_details.as_mut().unwrap().bulletin.as_mut().unwrap().resource_id.as_mut().unwrap().kind = Some(value.unwrap_or("").to_string());
                    },
                "content-details.bulletin.resource-id.channel-id" => {
                        request_content_details_bulletin_resource_id_init(&mut request);
                        request.content_details.as_mut().unwrap().bulletin.as_mut().unwrap().resource_id.as_mut().unwrap().channel_id = Some(value.unwrap_or("").to_string());
                    },
                "content-details.bulletin.resource-id.playlist-id" => {
                        request_content_details_bulletin_resource_id_init(&mut request);
                        request.content_details.as_mut().unwrap().bulletin.as_mut().unwrap().resource_id.as_mut().unwrap().playlist_id = Some(value.unwrap_or("").to_string());
                    },
                "content-details.bulletin.resource-id.video-id" => {
                        request_content_details_bulletin_resource_id_init(&mut request);
                        request.content_details.as_mut().unwrap().bulletin.as_mut().unwrap().resource_id.as_mut().unwrap().video_id = Some(value.unwrap_or("").to_string());
                    },
                "content-details.channel-item.resource-id.kind" => {
                        request_content_details_channel_item_resource_id_init(&mut request);
                        request.content_details.as_mut().unwrap().channel_item.as_mut().unwrap().resource_id.as_mut().unwrap().kind = Some(value.unwrap_or("").to_string());
                    },
                "content-details.channel-item.resource-id.channel-id" => {
                        request_content_details_channel_item_resource_id_init(&mut request);
                        request.content_details.as_mut().unwrap().channel_item.as_mut().unwrap().resource_id.as_mut().unwrap().channel_id = Some(value.unwrap_or("").to_string());
                    },
                "content-details.channel-item.resource-id.playlist-id" => {
                        request_content_details_channel_item_resource_id_init(&mut request);
                        request.content_details.as_mut().unwrap().channel_item.as_mut().unwrap().resource_id.as_mut().unwrap().playlist_id = Some(value.unwrap_or("").to_string());
                    },
                "content-details.channel-item.resource-id.video-id" => {
                        request_content_details_channel_item_resource_id_init(&mut request);
                        request.content_details.as_mut().unwrap().channel_item.as_mut().unwrap().resource_id.as_mut().unwrap().video_id = Some(value.unwrap_or("").to_string());
                    },
                "kind" => {
                        request_content_details_init(&mut request);
                        request.kind = Some(value.unwrap_or("").to_string());
                    },
                "etag" => {
                        request_content_details_init(&mut request);
                        request.etag = Some(value.unwrap_or("").to_string());
                    },
                "id" => {
                        request_content_details_init(&mut request);
                        request.id = Some(value.unwrap_or("").to_string());
                    },
                _ => {
                    let suggestion = FieldCursor::did_you_mean(key, &vec!["ad-tag", "author", "bulletin", "channel-id", "channel-item", "channel-title", "click-tracking-url", "comment", "content-details", "creative-view-url", "cta-type", "custom-cta-button-text", "default", "description", "description-text", "destination-url", "etag", "favorite", "forecasting-url", "group-id", "height", "high", "id", "image-url", "impression-url", "kind", "like", "maxres", "medium", "playlist-id", "playlist-item", "playlist-item-id", "promoted-item", "published-at", "reason", "recommendation", "reference-url", "resource-id", "seed-resource-id", "snippet", "social", "standard", "subscription", "thumbnails", "title", "type", "upload", "url", "video-id", "width"]);
                    err.issues.push(CLIError::Field(FieldError::Unknown(temp_cursor.to_string(), suggestion, value.map(|v| v.to_string()))));
                }
            }
        }
        let mut call = self.hub.activities().insert(request);
        for parg in opt.values_of("v").unwrap_or(Vec::new()).iter() {
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
                                                Vec::new() + &self.gp + &[]
                                                            ));
                    }
                }
            }
        }
        let protocol = CallType::Standard;
        if dry_run {
            Ok(())
        } else {
            assert!(err.issues.len() == 0);
            for scope in self.opt.values_of("url").unwrap_or(Vec::new()).iter() {
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
                    let mut value = json::value::to_value(&output_schema);
                    remove_json_null_values(&mut value);
                    serde::json::to_writer_pretty(&mut ostream, &value).unwrap();
                    Ok(())
                }
            }
        }
    }

    fn _activities_list(&self, opt: &ArgMatches<'n, 'a>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        let mut call = self.hub.activities().list(opt.value_of("part").unwrap_or(""));
        for parg in opt.values_of("v").unwrap_or(Vec::new()).iter() {
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
                                                Vec::new() + &self.gp + &["page-token", "published-before", "channel-id", "mine", "max-results", "region-code", "home", "published-after"]
                                                            ));
                    }
                }
            }
        }
        let protocol = CallType::Standard;
        if dry_run {
            Ok(())
        } else {
            assert!(err.issues.len() == 0);
            for scope in self.opt.values_of("url").unwrap_or(Vec::new()).iter() {
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
                    let mut value = json::value::to_value(&output_schema);
                    remove_json_null_values(&mut value);
                    serde::json::to_writer_pretty(&mut ostream, &value).unwrap();
                    Ok(())
                }
            }
        }
    }

    fn _captions_delete(&self, opt: &ArgMatches<'n, 'a>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        let mut call = self.hub.captions().delete(opt.value_of("id").unwrap_or(""));
        for parg in opt.values_of("v").unwrap_or(Vec::new()).iter() {
            let (key, value) = parse_kv_arg(&*parg, err, false);
            match key {
                "on-behalf-of" => {
                    call = call.on_behalf_of(value.unwrap_or(""));
                },
                "debug-project-id-override" => {
                    call = call.debug_project_id_override(value.unwrap_or(""));
                },
                _ => {
                    let mut found = false;
                    for param in &self.gp {
                        if key == *param {
                            found = true;
                            call = call.param(self.gpm.iter().find(|t| t.0 == key).unwrap_or(&("", key)).1, value.unwrap_or("unset"));
                            break;
                        }
                    }
                    if !found {
                        err.issues.push(CLIError::UnknownParameter(key.to_string(),
                                                Vec::new() + &self.gp + &["on-behalf-of", "debug-project-id-override"]
                                                            ));
                    }
                }
            }
        }
        let protocol = CallType::Standard;
        if dry_run {
            Ok(())
        } else {
            assert!(err.issues.len() == 0);
            for scope in self.opt.values_of("url").unwrap_or(Vec::new()).iter() {
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

    fn _captions_download(&self, opt: &ArgMatches<'n, 'a>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        let mut download_mode = false;
        let mut call = self.hub.captions().download(opt.value_of("id").unwrap_or(""));
        for parg in opt.values_of("v").unwrap_or(Vec::new()).iter() {
            let (key, value) = parse_kv_arg(&*parg, err, false);
            match key {
                "tlang" => {
                    call = call.tlang(value.unwrap_or(""));
                },
                "tfmt" => {
                    call = call.tfmt(value.unwrap_or(""));
                },
                "on-behalf-of" => {
                    call = call.on_behalf_of(value.unwrap_or(""));
                },
                "debug-project-id-override" => {
                    call = call.debug_project_id_override(value.unwrap_or(""));
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
                                                Vec::new() + &self.gp + &["tfmt", "on-behalf-of", "tlang", "debug-project-id-override"]
                                                            ));
                    }
                }
            }
        }
        let protocol = CallType::Standard;
        if dry_run {
            Ok(())
        } else {
            assert!(err.issues.len() == 0);
            for scope in self.opt.values_of("url").unwrap_or(Vec::new()).iter() {
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
                    }
                    Ok(())
                }
            }
        }
    }

    fn _captions_insert(&self, opt: &ArgMatches<'n, 'a>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        
        let mut request = api::Caption::default();
        let mut field_cursor = FieldCursor::default();
        for kvarg in opt.values_of("kv").unwrap_or(Vec::new()).iter() {
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
            fn request_snippet_init(request: &mut api::Caption) {
                if request.snippet.is_none() {
                    request.snippet = Some(Default::default());
                }
            }
            
            match &temp_cursor.to_string()[..] {
                "snippet.status" => {
                        request_snippet_init(&mut request);
                        request.snippet.as_mut().unwrap().status = Some(value.unwrap_or("").to_string());
                    },
                "snippet.audio-track-type" => {
                        request_snippet_init(&mut request);
                        request.snippet.as_mut().unwrap().audio_track_type = Some(value.unwrap_or("").to_string());
                    },
                "snippet.language" => {
                        request_snippet_init(&mut request);
                        request.snippet.as_mut().unwrap().language = Some(value.unwrap_or("").to_string());
                    },
                "snippet.name" => {
                        request_snippet_init(&mut request);
                        request.snippet.as_mut().unwrap().name = Some(value.unwrap_or("").to_string());
                    },
                "snippet.video-id" => {
                        request_snippet_init(&mut request);
                        request.snippet.as_mut().unwrap().video_id = Some(value.unwrap_or("").to_string());
                    },
                "snippet.is-draft" => {
                        request_snippet_init(&mut request);
                        request.snippet.as_mut().unwrap().is_draft = Some(arg_from_str(value.unwrap_or("false"), err, "snippet.is-draft", "boolean"));
                    },
                "snippet.is-auto-synced" => {
                        request_snippet_init(&mut request);
                        request.snippet.as_mut().unwrap().is_auto_synced = Some(arg_from_str(value.unwrap_or("false"), err, "snippet.is-auto-synced", "boolean"));
                    },
                "snippet.track-kind" => {
                        request_snippet_init(&mut request);
                        request.snippet.as_mut().unwrap().track_kind = Some(value.unwrap_or("").to_string());
                    },
                "snippet.last-updated" => {
                        request_snippet_init(&mut request);
                        request.snippet.as_mut().unwrap().last_updated = Some(value.unwrap_or("").to_string());
                    },
                "snippet.is-cc" => {
                        request_snippet_init(&mut request);
                        request.snippet.as_mut().unwrap().is_cc = Some(arg_from_str(value.unwrap_or("false"), err, "snippet.is-cc", "boolean"));
                    },
                "snippet.is-easy-reader" => {
                        request_snippet_init(&mut request);
                        request.snippet.as_mut().unwrap().is_easy_reader = Some(arg_from_str(value.unwrap_or("false"), err, "snippet.is-easy-reader", "boolean"));
                    },
                "snippet.is-large" => {
                        request_snippet_init(&mut request);
                        request.snippet.as_mut().unwrap().is_large = Some(arg_from_str(value.unwrap_or("false"), err, "snippet.is-large", "boolean"));
                    },
                "snippet.failure-reason" => {
                        request_snippet_init(&mut request);
                        request.snippet.as_mut().unwrap().failure_reason = Some(value.unwrap_or("").to_string());
                    },
                "kind" => {
                        request_snippet_init(&mut request);
                        request.kind = Some(value.unwrap_or("").to_string());
                    },
                "etag" => {
                        request_snippet_init(&mut request);
                        request.etag = Some(value.unwrap_or("").to_string());
                    },
                "id" => {
                        request_snippet_init(&mut request);
                        request.id = Some(value.unwrap_or("").to_string());
                    },
                _ => {
                    let suggestion = FieldCursor::did_you_mean(key, &vec!["audio-track-type", "etag", "failure-reason", "id", "is-auto-synced", "is-cc", "is-draft", "is-easy-reader", "is-large", "kind", "language", "last-updated", "name", "snippet", "status", "track-kind", "video-id"]);
                    err.issues.push(CLIError::Field(FieldError::Unknown(temp_cursor.to_string(), suggestion, value.map(|v| v.to_string()))));
                }
            }
        }
        let mut call = self.hub.captions().insert(request);
        for parg in opt.values_of("v").unwrap_or(Vec::new()).iter() {
            let (key, value) = parse_kv_arg(&*parg, err, false);
            match key {
                "sync" => {
                    call = call.sync(arg_from_str(value.unwrap_or("false"), err, "sync", "boolean"));
                },
                "on-behalf-of" => {
                    call = call.on_behalf_of(value.unwrap_or(""));
                },
                "debug-project-id-override" => {
                    call = call.debug_project_id_override(value.unwrap_or(""));
                },
                _ => {
                    let mut found = false;
                    for param in &self.gp {
                        if key == *param {
                            found = true;
                            call = call.param(self.gpm.iter().find(|t| t.0 == key).unwrap_or(&("", key)).1, value.unwrap_or("unset"));
                            break;
                        }
                    }
                    if !found {
                        err.issues.push(CLIError::UnknownParameter(key.to_string(),
                                                Vec::new() + &self.gp + &["on-behalf-of", "sync", "debug-project-id-override"]
                                                            ));
                    }
                }
            }
        }
        let vals = opt.values_of("mode").unwrap();
        let protocol = calltype_from_str(vals[0], ["simple", "resumable"].iter().map(|&v| v.to_string()).collect(), err);
        let mut input_file = input_file_from_opts(vals[1], err);
        let mime_type = input_mime_from_opts(opt.value_of("mime").unwrap_or("application/octet-stream"), err);
        if dry_run {
            Ok(())
        } else {
            assert!(err.issues.len() == 0);
            for scope in self.opt.values_of("url").unwrap_or(Vec::new()).iter() {
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
                    let mut value = json::value::to_value(&output_schema);
                    remove_json_null_values(&mut value);
                    serde::json::to_writer_pretty(&mut ostream, &value).unwrap();
                    Ok(())
                }
            }
        }
    }

    fn _captions_list(&self, opt: &ArgMatches<'n, 'a>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        let mut call = self.hub.captions().list(opt.value_of("part").unwrap_or(""), opt.value_of("video-id").unwrap_or(""));
        for parg in opt.values_of("v").unwrap_or(Vec::new()).iter() {
            let (key, value) = parse_kv_arg(&*parg, err, false);
            match key {
                "on-behalf-of" => {
                    call = call.on_behalf_of(value.unwrap_or(""));
                },
                "id" => {
                    call = call.id(value.unwrap_or(""));
                },
                "debug-project-id-override" => {
                    call = call.debug_project_id_override(value.unwrap_or(""));
                },
                _ => {
                    let mut found = false;
                    for param in &self.gp {
                        if key == *param {
                            found = true;
                            call = call.param(self.gpm.iter().find(|t| t.0 == key).unwrap_or(&("", key)).1, value.unwrap_or("unset"));
                            break;
                        }
                    }
                    if !found {
                        err.issues.push(CLIError::UnknownParameter(key.to_string(),
                                                Vec::new() + &self.gp + &["on-behalf-of", "id", "debug-project-id-override"]
                                                            ));
                    }
                }
            }
        }
        let protocol = CallType::Standard;
        if dry_run {
            Ok(())
        } else {
            assert!(err.issues.len() == 0);
            for scope in self.opt.values_of("url").unwrap_or(Vec::new()).iter() {
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
                    let mut value = json::value::to_value(&output_schema);
                    remove_json_null_values(&mut value);
                    serde::json::to_writer_pretty(&mut ostream, &value).unwrap();
                    Ok(())
                }
            }
        }
    }

    fn _captions_update(&self, opt: &ArgMatches<'n, 'a>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        
        let mut request = api::Caption::default();
        let mut field_cursor = FieldCursor::default();
        for kvarg in opt.values_of("kv").unwrap_or(Vec::new()).iter() {
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
            fn request_snippet_init(request: &mut api::Caption) {
                if request.snippet.is_none() {
                    request.snippet = Some(Default::default());
                }
            }
            
            match &temp_cursor.to_string()[..] {
                "snippet.status" => {
                        request_snippet_init(&mut request);
                        request.snippet.as_mut().unwrap().status = Some(value.unwrap_or("").to_string());
                    },
                "snippet.audio-track-type" => {
                        request_snippet_init(&mut request);
                        request.snippet.as_mut().unwrap().audio_track_type = Some(value.unwrap_or("").to_string());
                    },
                "snippet.language" => {
                        request_snippet_init(&mut request);
                        request.snippet.as_mut().unwrap().language = Some(value.unwrap_or("").to_string());
                    },
                "snippet.name" => {
                        request_snippet_init(&mut request);
                        request.snippet.as_mut().unwrap().name = Some(value.unwrap_or("").to_string());
                    },
                "snippet.video-id" => {
                        request_snippet_init(&mut request);
                        request.snippet.as_mut().unwrap().video_id = Some(value.unwrap_or("").to_string());
                    },
                "snippet.is-draft" => {
                        request_snippet_init(&mut request);
                        request.snippet.as_mut().unwrap().is_draft = Some(arg_from_str(value.unwrap_or("false"), err, "snippet.is-draft", "boolean"));
                    },
                "snippet.is-auto-synced" => {
                        request_snippet_init(&mut request);
                        request.snippet.as_mut().unwrap().is_auto_synced = Some(arg_from_str(value.unwrap_or("false"), err, "snippet.is-auto-synced", "boolean"));
                    },
                "snippet.track-kind" => {
                        request_snippet_init(&mut request);
                        request.snippet.as_mut().unwrap().track_kind = Some(value.unwrap_or("").to_string());
                    },
                "snippet.last-updated" => {
                        request_snippet_init(&mut request);
                        request.snippet.as_mut().unwrap().last_updated = Some(value.unwrap_or("").to_string());
                    },
                "snippet.is-cc" => {
                        request_snippet_init(&mut request);
                        request.snippet.as_mut().unwrap().is_cc = Some(arg_from_str(value.unwrap_or("false"), err, "snippet.is-cc", "boolean"));
                    },
                "snippet.is-easy-reader" => {
                        request_snippet_init(&mut request);
                        request.snippet.as_mut().unwrap().is_easy_reader = Some(arg_from_str(value.unwrap_or("false"), err, "snippet.is-easy-reader", "boolean"));
                    },
                "snippet.is-large" => {
                        request_snippet_init(&mut request);
                        request.snippet.as_mut().unwrap().is_large = Some(arg_from_str(value.unwrap_or("false"), err, "snippet.is-large", "boolean"));
                    },
                "snippet.failure-reason" => {
                        request_snippet_init(&mut request);
                        request.snippet.as_mut().unwrap().failure_reason = Some(value.unwrap_or("").to_string());
                    },
                "kind" => {
                        request_snippet_init(&mut request);
                        request.kind = Some(value.unwrap_or("").to_string());
                    },
                "etag" => {
                        request_snippet_init(&mut request);
                        request.etag = Some(value.unwrap_or("").to_string());
                    },
                "id" => {
                        request_snippet_init(&mut request);
                        request.id = Some(value.unwrap_or("").to_string());
                    },
                _ => {
                    let suggestion = FieldCursor::did_you_mean(key, &vec!["audio-track-type", "etag", "failure-reason", "id", "is-auto-synced", "is-cc", "is-draft", "is-easy-reader", "is-large", "kind", "language", "last-updated", "name", "snippet", "status", "track-kind", "video-id"]);
                    err.issues.push(CLIError::Field(FieldError::Unknown(temp_cursor.to_string(), suggestion, value.map(|v| v.to_string()))));
                }
            }
        }
        let mut call = self.hub.captions().update(request);
        for parg in opt.values_of("v").unwrap_or(Vec::new()).iter() {
            let (key, value) = parse_kv_arg(&*parg, err, false);
            match key {
                "sync" => {
                    call = call.sync(arg_from_str(value.unwrap_or("false"), err, "sync", "boolean"));
                },
                "on-behalf-of" => {
                    call = call.on_behalf_of(value.unwrap_or(""));
                },
                "debug-project-id-override" => {
                    call = call.debug_project_id_override(value.unwrap_or(""));
                },
                _ => {
                    let mut found = false;
                    for param in &self.gp {
                        if key == *param {
                            found = true;
                            call = call.param(self.gpm.iter().find(|t| t.0 == key).unwrap_or(&("", key)).1, value.unwrap_or("unset"));
                            break;
                        }
                    }
                    if !found {
                        err.issues.push(CLIError::UnknownParameter(key.to_string(),
                                                Vec::new() + &self.gp + &["on-behalf-of", "sync", "debug-project-id-override"]
                                                            ));
                    }
                }
            }
        }
        let vals = opt.values_of("mode").unwrap();
        let protocol = calltype_from_str(vals[0], ["simple", "resumable"].iter().map(|&v| v.to_string()).collect(), err);
        let mut input_file = input_file_from_opts(vals[1], err);
        let mime_type = input_mime_from_opts(opt.value_of("mime").unwrap_or("application/octet-stream"), err);
        if dry_run {
            Ok(())
        } else {
            assert!(err.issues.len() == 0);
            for scope in self.opt.values_of("url").unwrap_or(Vec::new()).iter() {
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
                    let mut value = json::value::to_value(&output_schema);
                    remove_json_null_values(&mut value);
                    serde::json::to_writer_pretty(&mut ostream, &value).unwrap();
                    Ok(())
                }
            }
        }
    }

    fn _channel_banners_insert(&self, opt: &ArgMatches<'n, 'a>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        
        let mut request = api::ChannelBannerResource::default();
        let mut field_cursor = FieldCursor::default();
        for kvarg in opt.values_of("kv").unwrap_or(Vec::new()).iter() {
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
            match &temp_cursor.to_string()[..] {
                "url" => {
                        request.url = Some(value.unwrap_or("").to_string());
                    },
                "kind" => {
                        request.kind = Some(value.unwrap_or("").to_string());
                    },
                "etag" => {
                        request.etag = Some(value.unwrap_or("").to_string());
                    },
                _ => {
                    let suggestion = FieldCursor::did_you_mean(key, &vec!["etag", "kind", "url"]);
                    err.issues.push(CLIError::Field(FieldError::Unknown(temp_cursor.to_string(), suggestion, value.map(|v| v.to_string()))));
                }
            }
        }
        let mut call = self.hub.channel_banners().insert(request);
        for parg in opt.values_of("v").unwrap_or(Vec::new()).iter() {
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
                                                Vec::new() + &self.gp + &["on-behalf-of-content-owner"]
                                                            ));
                    }
                }
            }
        }
        let vals = opt.values_of("mode").unwrap();
        let protocol = calltype_from_str(vals[0], ["simple", "resumable"].iter().map(|&v| v.to_string()).collect(), err);
        let mut input_file = input_file_from_opts(vals[1], err);
        let mime_type = input_mime_from_opts(opt.value_of("mime").unwrap_or("application/octet-stream"), err);
        if dry_run {
            Ok(())
        } else {
            assert!(err.issues.len() == 0);
            for scope in self.opt.values_of("url").unwrap_or(Vec::new()).iter() {
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
                    let mut value = json::value::to_value(&output_schema);
                    remove_json_null_values(&mut value);
                    serde::json::to_writer_pretty(&mut ostream, &value).unwrap();
                    Ok(())
                }
            }
        }
    }

    fn _channel_sections_delete(&self, opt: &ArgMatches<'n, 'a>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        let mut call = self.hub.channel_sections().delete(opt.value_of("id").unwrap_or(""));
        for parg in opt.values_of("v").unwrap_or(Vec::new()).iter() {
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
                                                Vec::new() + &self.gp + &["on-behalf-of-content-owner"]
                                                            ));
                    }
                }
            }
        }
        let protocol = CallType::Standard;
        if dry_run {
            Ok(())
        } else {
            assert!(err.issues.len() == 0);
            for scope in self.opt.values_of("url").unwrap_or(Vec::new()).iter() {
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

    fn _channel_sections_insert(&self, opt: &ArgMatches<'n, 'a>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        
        let mut request = api::ChannelSection::default();
        let mut field_cursor = FieldCursor::default();
        for kvarg in opt.values_of("kv").unwrap_or(Vec::new()).iter() {
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
            fn request_content_details_init(request: &mut api::ChannelSection) {
                if request.content_details.is_none() {
                    request.content_details = Some(Default::default());
                }
            }
            
            fn request_snippet_init(request: &mut api::ChannelSection) {
                if request.snippet.is_none() {
                    request.snippet = Some(Default::default());
                }
            }
            
            fn request_snippet_localized_init(request: &mut api::ChannelSection) {
                request_snippet_init(request);
                if request.snippet.as_mut().unwrap().localized.is_none() {
                    request.snippet.as_mut().unwrap().localized = Some(Default::default());
                }
            }
            
            fn request_targeting_init(request: &mut api::ChannelSection) {
                if request.targeting.is_none() {
                    request.targeting = Some(Default::default());
                }
            }
            
            match &temp_cursor.to_string()[..] {
                "kind" => {
                        request.kind = Some(value.unwrap_or("").to_string());
                    },
                "targeting.languages" => {
                        request_targeting_init(&mut request);
                        if request.targeting.as_mut().unwrap().languages.is_none() {
                           request.targeting.as_mut().unwrap().languages = Some(Default::default());
                        }
                                        request.targeting.as_mut().unwrap().languages.as_mut().unwrap().push(value.unwrap_or("").to_string());
                    },
                "targeting.regions" => {
                        request_targeting_init(&mut request);
                        if request.targeting.as_mut().unwrap().regions.is_none() {
                           request.targeting.as_mut().unwrap().regions = Some(Default::default());
                        }
                                        request.targeting.as_mut().unwrap().regions.as_mut().unwrap().push(value.unwrap_or("").to_string());
                    },
                "targeting.countries" => {
                        request_targeting_init(&mut request);
                        if request.targeting.as_mut().unwrap().countries.is_none() {
                           request.targeting.as_mut().unwrap().countries = Some(Default::default());
                        }
                                        request.targeting.as_mut().unwrap().countries.as_mut().unwrap().push(value.unwrap_or("").to_string());
                    },
                "content-details.channels" => {
                        request_content_details_init(&mut request);
                        if request.content_details.as_mut().unwrap().channels.is_none() {
                           request.content_details.as_mut().unwrap().channels = Some(Default::default());
                        }
                                        request.content_details.as_mut().unwrap().channels.as_mut().unwrap().push(value.unwrap_or("").to_string());
                    },
                "content-details.playlists" => {
                        request_content_details_init(&mut request);
                        if request.content_details.as_mut().unwrap().playlists.is_none() {
                           request.content_details.as_mut().unwrap().playlists = Some(Default::default());
                        }
                                        request.content_details.as_mut().unwrap().playlists.as_mut().unwrap().push(value.unwrap_or("").to_string());
                    },
                "snippet.style" => {
                        request_snippet_init(&mut request);
                        request.snippet.as_mut().unwrap().style = Some(value.unwrap_or("").to_string());
                    },
                "snippet.title" => {
                        request_snippet_init(&mut request);
                        request.snippet.as_mut().unwrap().title = Some(value.unwrap_or("").to_string());
                    },
                "snippet.channel-id" => {
                        request_snippet_init(&mut request);
                        request.snippet.as_mut().unwrap().channel_id = Some(value.unwrap_or("").to_string());
                    },
                "snippet.default-language" => {
                        request_snippet_init(&mut request);
                        request.snippet.as_mut().unwrap().default_language = Some(value.unwrap_or("").to_string());
                    },
                "snippet.position" => {
                        request_snippet_init(&mut request);
                        request.snippet.as_mut().unwrap().position = Some(arg_from_str(value.unwrap_or("-0"), err, "snippet.position", "integer"));
                    },
                "snippet.type" => {
                        request_snippet_init(&mut request);
                        request.snippet.as_mut().unwrap().type_ = Some(value.unwrap_or("").to_string());
                    },
                "snippet.localized.title" => {
                        request_snippet_localized_init(&mut request);
                        request.snippet.as_mut().unwrap().localized.as_mut().unwrap().title = Some(value.unwrap_or("").to_string());
                    },
                "etag" => {
                        request_snippet_init(&mut request);
                        request.etag = Some(value.unwrap_or("").to_string());
                    },
                "id" => {
                        request_snippet_init(&mut request);
                        request.id = Some(value.unwrap_or("").to_string());
                    },
                _ => {
                    let suggestion = FieldCursor::did_you_mean(key, &vec!["channel-id", "channels", "content-details", "countries", "default-language", "etag", "id", "kind", "languages", "localized", "playlists", "position", "regions", "snippet", "style", "targeting", "title", "type"]);
                    err.issues.push(CLIError::Field(FieldError::Unknown(temp_cursor.to_string(), suggestion, value.map(|v| v.to_string()))));
                }
            }
        }
        let mut call = self.hub.channel_sections().insert(request);
        for parg in opt.values_of("v").unwrap_or(Vec::new()).iter() {
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
                                                Vec::new() + &self.gp + &["on-behalf-of-content-owner-channel", "on-behalf-of-content-owner"]
                                                            ));
                    }
                }
            }
        }
        let protocol = CallType::Standard;
        if dry_run {
            Ok(())
        } else {
            assert!(err.issues.len() == 0);
            for scope in self.opt.values_of("url").unwrap_or(Vec::new()).iter() {
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
                    let mut value = json::value::to_value(&output_schema);
                    remove_json_null_values(&mut value);
                    serde::json::to_writer_pretty(&mut ostream, &value).unwrap();
                    Ok(())
                }
            }
        }
    }

    fn _channel_sections_list(&self, opt: &ArgMatches<'n, 'a>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        let mut call = self.hub.channel_sections().list(opt.value_of("part").unwrap_or(""));
        for parg in opt.values_of("v").unwrap_or(Vec::new()).iter() {
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
                                                Vec::new() + &self.gp + &["on-behalf-of-content-owner", "channel-id", "mine", "hl", "id"]
                                                            ));
                    }
                }
            }
        }
        let protocol = CallType::Standard;
        if dry_run {
            Ok(())
        } else {
            assert!(err.issues.len() == 0);
            for scope in self.opt.values_of("url").unwrap_or(Vec::new()).iter() {
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
                    let mut value = json::value::to_value(&output_schema);
                    remove_json_null_values(&mut value);
                    serde::json::to_writer_pretty(&mut ostream, &value).unwrap();
                    Ok(())
                }
            }
        }
    }

    fn _channel_sections_update(&self, opt: &ArgMatches<'n, 'a>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        
        let mut request = api::ChannelSection::default();
        let mut field_cursor = FieldCursor::default();
        for kvarg in opt.values_of("kv").unwrap_or(Vec::new()).iter() {
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
            fn request_content_details_init(request: &mut api::ChannelSection) {
                if request.content_details.is_none() {
                    request.content_details = Some(Default::default());
                }
            }
            
            fn request_snippet_init(request: &mut api::ChannelSection) {
                if request.snippet.is_none() {
                    request.snippet = Some(Default::default());
                }
            }
            
            fn request_snippet_localized_init(request: &mut api::ChannelSection) {
                request_snippet_init(request);
                if request.snippet.as_mut().unwrap().localized.is_none() {
                    request.snippet.as_mut().unwrap().localized = Some(Default::default());
                }
            }
            
            fn request_targeting_init(request: &mut api::ChannelSection) {
                if request.targeting.is_none() {
                    request.targeting = Some(Default::default());
                }
            }
            
            match &temp_cursor.to_string()[..] {
                "kind" => {
                        request.kind = Some(value.unwrap_or("").to_string());
                    },
                "targeting.languages" => {
                        request_targeting_init(&mut request);
                        if request.targeting.as_mut().unwrap().languages.is_none() {
                           request.targeting.as_mut().unwrap().languages = Some(Default::default());
                        }
                                        request.targeting.as_mut().unwrap().languages.as_mut().unwrap().push(value.unwrap_or("").to_string());
                    },
                "targeting.regions" => {
                        request_targeting_init(&mut request);
                        if request.targeting.as_mut().unwrap().regions.is_none() {
                           request.targeting.as_mut().unwrap().regions = Some(Default::default());
                        }
                                        request.targeting.as_mut().unwrap().regions.as_mut().unwrap().push(value.unwrap_or("").to_string());
                    },
                "targeting.countries" => {
                        request_targeting_init(&mut request);
                        if request.targeting.as_mut().unwrap().countries.is_none() {
                           request.targeting.as_mut().unwrap().countries = Some(Default::default());
                        }
                                        request.targeting.as_mut().unwrap().countries.as_mut().unwrap().push(value.unwrap_or("").to_string());
                    },
                "content-details.channels" => {
                        request_content_details_init(&mut request);
                        if request.content_details.as_mut().unwrap().channels.is_none() {
                           request.content_details.as_mut().unwrap().channels = Some(Default::default());
                        }
                                        request.content_details.as_mut().unwrap().channels.as_mut().unwrap().push(value.unwrap_or("").to_string());
                    },
                "content-details.playlists" => {
                        request_content_details_init(&mut request);
                        if request.content_details.as_mut().unwrap().playlists.is_none() {
                           request.content_details.as_mut().unwrap().playlists = Some(Default::default());
                        }
                                        request.content_details.as_mut().unwrap().playlists.as_mut().unwrap().push(value.unwrap_or("").to_string());
                    },
                "snippet.style" => {
                        request_snippet_init(&mut request);
                        request.snippet.as_mut().unwrap().style = Some(value.unwrap_or("").to_string());
                    },
                "snippet.title" => {
                        request_snippet_init(&mut request);
                        request.snippet.as_mut().unwrap().title = Some(value.unwrap_or("").to_string());
                    },
                "snippet.channel-id" => {
                        request_snippet_init(&mut request);
                        request.snippet.as_mut().unwrap().channel_id = Some(value.unwrap_or("").to_string());
                    },
                "snippet.default-language" => {
                        request_snippet_init(&mut request);
                        request.snippet.as_mut().unwrap().default_language = Some(value.unwrap_or("").to_string());
                    },
                "snippet.position" => {
                        request_snippet_init(&mut request);
                        request.snippet.as_mut().unwrap().position = Some(arg_from_str(value.unwrap_or("-0"), err, "snippet.position", "integer"));
                    },
                "snippet.type" => {
                        request_snippet_init(&mut request);
                        request.snippet.as_mut().unwrap().type_ = Some(value.unwrap_or("").to_string());
                    },
                "snippet.localized.title" => {
                        request_snippet_localized_init(&mut request);
                        request.snippet.as_mut().unwrap().localized.as_mut().unwrap().title = Some(value.unwrap_or("").to_string());
                    },
                "etag" => {
                        request_snippet_init(&mut request);
                        request.etag = Some(value.unwrap_or("").to_string());
                    },
                "id" => {
                        request_snippet_init(&mut request);
                        request.id = Some(value.unwrap_or("").to_string());
                    },
                _ => {
                    let suggestion = FieldCursor::did_you_mean(key, &vec!["channel-id", "channels", "content-details", "countries", "default-language", "etag", "id", "kind", "languages", "localized", "playlists", "position", "regions", "snippet", "style", "targeting", "title", "type"]);
                    err.issues.push(CLIError::Field(FieldError::Unknown(temp_cursor.to_string(), suggestion, value.map(|v| v.to_string()))));
                }
            }
        }
        let mut call = self.hub.channel_sections().update(request);
        for parg in opt.values_of("v").unwrap_or(Vec::new()).iter() {
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
                                                Vec::new() + &self.gp + &["on-behalf-of-content-owner"]
                                                            ));
                    }
                }
            }
        }
        let protocol = CallType::Standard;
        if dry_run {
            Ok(())
        } else {
            assert!(err.issues.len() == 0);
            for scope in self.opt.values_of("url").unwrap_or(Vec::new()).iter() {
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
                    let mut value = json::value::to_value(&output_schema);
                    remove_json_null_values(&mut value);
                    serde::json::to_writer_pretty(&mut ostream, &value).unwrap();
                    Ok(())
                }
            }
        }
    }

    fn _channels_list(&self, opt: &ArgMatches<'n, 'a>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        let mut call = self.hub.channels().list(opt.value_of("part").unwrap_or(""));
        for parg in opt.values_of("v").unwrap_or(Vec::new()).iter() {
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
                                                Vec::new() + &self.gp + &["managed-by-me", "on-behalf-of-content-owner", "for-username", "mine", "max-results", "category-id", "page-token", "my-subscribers", "hl", "id"]
                                                            ));
                    }
                }
            }
        }
        let protocol = CallType::Standard;
        if dry_run {
            Ok(())
        } else {
            assert!(err.issues.len() == 0);
            for scope in self.opt.values_of("url").unwrap_or(Vec::new()).iter() {
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
                    let mut value = json::value::to_value(&output_schema);
                    remove_json_null_values(&mut value);
                    serde::json::to_writer_pretty(&mut ostream, &value).unwrap();
                    Ok(())
                }
            }
        }
    }

    fn _channels_update(&self, opt: &ArgMatches<'n, 'a>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        
        let mut request = api::Channel::default();
        let mut field_cursor = FieldCursor::default();
        for kvarg in opt.values_of("kv").unwrap_or(Vec::new()).iter() {
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
            fn request_audit_details_init(request: &mut api::Channel) {
                if request.audit_details.is_none() {
                    request.audit_details = Some(Default::default());
                }
            }
            
            fn request_branding_settings_channel_init(request: &mut api::Channel) {
                request_branding_settings_init(request);
                if request.branding_settings.as_mut().unwrap().channel.is_none() {
                    request.branding_settings.as_mut().unwrap().channel = Some(Default::default());
                }
            }
            
            fn request_branding_settings_image_background_image_url_default_language_init(request: &mut api::Channel) {
                request_branding_settings_image_background_image_url_init(request);
                if request.branding_settings.as_mut().unwrap().image.as_mut().unwrap().background_image_url.as_mut().unwrap().default_language.is_none() {
                    request.branding_settings.as_mut().unwrap().image.as_mut().unwrap().background_image_url.as_mut().unwrap().default_language = Some(Default::default());
                }
            }
            
            fn request_branding_settings_image_background_image_url_init(request: &mut api::Channel) {
                request_branding_settings_image_init(request);
                if request.branding_settings.as_mut().unwrap().image.as_mut().unwrap().background_image_url.is_none() {
                    request.branding_settings.as_mut().unwrap().image.as_mut().unwrap().background_image_url = Some(Default::default());
                }
            }
            
            fn request_branding_settings_image_init(request: &mut api::Channel) {
                request_branding_settings_init(request);
                if request.branding_settings.as_mut().unwrap().image.is_none() {
                    request.branding_settings.as_mut().unwrap().image = Some(Default::default());
                }
            }
            
            fn request_branding_settings_image_large_branded_banner_image_imap_script_default_language_init(request: &mut api::Channel) {
                request_branding_settings_image_large_branded_banner_image_imap_script_init(request);
                if request.branding_settings.as_mut().unwrap().image.as_mut().unwrap().large_branded_banner_image_imap_script.as_mut().unwrap().default_language.is_none() {
                    request.branding_settings.as_mut().unwrap().image.as_mut().unwrap().large_branded_banner_image_imap_script.as_mut().unwrap().default_language = Some(Default::default());
                }
            }
            
            fn request_branding_settings_image_large_branded_banner_image_imap_script_init(request: &mut api::Channel) {
                request_branding_settings_image_init(request);
                if request.branding_settings.as_mut().unwrap().image.as_mut().unwrap().large_branded_banner_image_imap_script.is_none() {
                    request.branding_settings.as_mut().unwrap().image.as_mut().unwrap().large_branded_banner_image_imap_script = Some(Default::default());
                }
            }
            
            fn request_branding_settings_image_large_branded_banner_image_url_default_language_init(request: &mut api::Channel) {
                request_branding_settings_image_large_branded_banner_image_url_init(request);
                if request.branding_settings.as_mut().unwrap().image.as_mut().unwrap().large_branded_banner_image_url.as_mut().unwrap().default_language.is_none() {
                    request.branding_settings.as_mut().unwrap().image.as_mut().unwrap().large_branded_banner_image_url.as_mut().unwrap().default_language = Some(Default::default());
                }
            }
            
            fn request_branding_settings_image_large_branded_banner_image_url_init(request: &mut api::Channel) {
                request_branding_settings_image_init(request);
                if request.branding_settings.as_mut().unwrap().image.as_mut().unwrap().large_branded_banner_image_url.is_none() {
                    request.branding_settings.as_mut().unwrap().image.as_mut().unwrap().large_branded_banner_image_url = Some(Default::default());
                }
            }
            
            fn request_branding_settings_image_small_branded_banner_image_imap_script_default_language_init(request: &mut api::Channel) {
                request_branding_settings_image_small_branded_banner_image_imap_script_init(request);
                if request.branding_settings.as_mut().unwrap().image.as_mut().unwrap().small_branded_banner_image_imap_script.as_mut().unwrap().default_language.is_none() {
                    request.branding_settings.as_mut().unwrap().image.as_mut().unwrap().small_branded_banner_image_imap_script.as_mut().unwrap().default_language = Some(Default::default());
                }
            }
            
            fn request_branding_settings_image_small_branded_banner_image_imap_script_init(request: &mut api::Channel) {
                request_branding_settings_image_init(request);
                if request.branding_settings.as_mut().unwrap().image.as_mut().unwrap().small_branded_banner_image_imap_script.is_none() {
                    request.branding_settings.as_mut().unwrap().image.as_mut().unwrap().small_branded_banner_image_imap_script = Some(Default::default());
                }
            }
            
            fn request_branding_settings_image_small_branded_banner_image_url_default_language_init(request: &mut api::Channel) {
                request_branding_settings_image_small_branded_banner_image_url_init(request);
                if request.branding_settings.as_mut().unwrap().image.as_mut().unwrap().small_branded_banner_image_url.as_mut().unwrap().default_language.is_none() {
                    request.branding_settings.as_mut().unwrap().image.as_mut().unwrap().small_branded_banner_image_url.as_mut().unwrap().default_language = Some(Default::default());
                }
            }
            
            fn request_branding_settings_image_small_branded_banner_image_url_init(request: &mut api::Channel) {
                request_branding_settings_image_init(request);
                if request.branding_settings.as_mut().unwrap().image.as_mut().unwrap().small_branded_banner_image_url.is_none() {
                    request.branding_settings.as_mut().unwrap().image.as_mut().unwrap().small_branded_banner_image_url = Some(Default::default());
                }
            }
            
            fn request_branding_settings_init(request: &mut api::Channel) {
                if request.branding_settings.is_none() {
                    request.branding_settings = Some(Default::default());
                }
            }
            
            fn request_branding_settings_watch_init(request: &mut api::Channel) {
                request_branding_settings_init(request);
                if request.branding_settings.as_mut().unwrap().watch.is_none() {
                    request.branding_settings.as_mut().unwrap().watch = Some(Default::default());
                }
            }
            
            fn request_content_details_init(request: &mut api::Channel) {
                if request.content_details.is_none() {
                    request.content_details = Some(Default::default());
                }
            }
            
            fn request_content_details_related_playlists_init(request: &mut api::Channel) {
                request_content_details_init(request);
                if request.content_details.as_mut().unwrap().related_playlists.is_none() {
                    request.content_details.as_mut().unwrap().related_playlists = Some(Default::default());
                }
            }
            
            fn request_content_owner_details_init(request: &mut api::Channel) {
                if request.content_owner_details.is_none() {
                    request.content_owner_details = Some(Default::default());
                }
            }
            
            fn request_invideo_promotion_default_timing_init(request: &mut api::Channel) {
                request_invideo_promotion_init(request);
                if request.invideo_promotion.as_mut().unwrap().default_timing.is_none() {
                    request.invideo_promotion.as_mut().unwrap().default_timing = Some(Default::default());
                }
            }
            
            fn request_invideo_promotion_init(request: &mut api::Channel) {
                if request.invideo_promotion.is_none() {
                    request.invideo_promotion = Some(Default::default());
                }
            }
            
            fn request_invideo_promotion_position_init(request: &mut api::Channel) {
                request_invideo_promotion_init(request);
                if request.invideo_promotion.as_mut().unwrap().position.is_none() {
                    request.invideo_promotion.as_mut().unwrap().position = Some(Default::default());
                }
            }
            
            fn request_snippet_init(request: &mut api::Channel) {
                if request.snippet.is_none() {
                    request.snippet = Some(Default::default());
                }
            }
            
            fn request_snippet_localized_init(request: &mut api::Channel) {
                request_snippet_init(request);
                if request.snippet.as_mut().unwrap().localized.is_none() {
                    request.snippet.as_mut().unwrap().localized = Some(Default::default());
                }
            }
            
            fn request_snippet_thumbnails_default_init(request: &mut api::Channel) {
                request_snippet_thumbnails_init(request);
                if request.snippet.as_mut().unwrap().thumbnails.as_mut().unwrap().default.is_none() {
                    request.snippet.as_mut().unwrap().thumbnails.as_mut().unwrap().default = Some(Default::default());
                }
            }
            
            fn request_snippet_thumbnails_high_init(request: &mut api::Channel) {
                request_snippet_thumbnails_init(request);
                if request.snippet.as_mut().unwrap().thumbnails.as_mut().unwrap().high.is_none() {
                    request.snippet.as_mut().unwrap().thumbnails.as_mut().unwrap().high = Some(Default::default());
                }
            }
            
            fn request_snippet_thumbnails_init(request: &mut api::Channel) {
                request_snippet_init(request);
                if request.snippet.as_mut().unwrap().thumbnails.is_none() {
                    request.snippet.as_mut().unwrap().thumbnails = Some(Default::default());
                }
            }
            
            fn request_snippet_thumbnails_maxres_init(request: &mut api::Channel) {
                request_snippet_thumbnails_init(request);
                if request.snippet.as_mut().unwrap().thumbnails.as_mut().unwrap().maxres.is_none() {
                    request.snippet.as_mut().unwrap().thumbnails.as_mut().unwrap().maxres = Some(Default::default());
                }
            }
            
            fn request_snippet_thumbnails_medium_init(request: &mut api::Channel) {
                request_snippet_thumbnails_init(request);
                if request.snippet.as_mut().unwrap().thumbnails.as_mut().unwrap().medium.is_none() {
                    request.snippet.as_mut().unwrap().thumbnails.as_mut().unwrap().medium = Some(Default::default());
                }
            }
            
            fn request_snippet_thumbnails_standard_init(request: &mut api::Channel) {
                request_snippet_thumbnails_init(request);
                if request.snippet.as_mut().unwrap().thumbnails.as_mut().unwrap().standard.is_none() {
                    request.snippet.as_mut().unwrap().thumbnails.as_mut().unwrap().standard = Some(Default::default());
                }
            }
            
            fn request_statistics_init(request: &mut api::Channel) {
                if request.statistics.is_none() {
                    request.statistics = Some(Default::default());
                }
            }
            
            fn request_status_init(request: &mut api::Channel) {
                if request.status.is_none() {
                    request.status = Some(Default::default());
                }
            }
            
            fn request_topic_details_init(request: &mut api::Channel) {
                if request.topic_details.is_none() {
                    request.topic_details = Some(Default::default());
                }
            }
            
            match &temp_cursor.to_string()[..] {
                "status.privacy-status" => {
                        request_status_init(&mut request);
                        request.status.as_mut().unwrap().privacy_status = Some(value.unwrap_or("").to_string());
                    },
                "status.is-linked" => {
                        request_status_init(&mut request);
                        request.status.as_mut().unwrap().is_linked = Some(arg_from_str(value.unwrap_or("false"), err, "status.is-linked", "boolean"));
                    },
                "status.long-uploads-status" => {
                        request_status_init(&mut request);
                        request.status.as_mut().unwrap().long_uploads_status = Some(value.unwrap_or("").to_string());
                    },
                "invideo-promotion.default-timing.offset-ms" => {
                        request_invideo_promotion_default_timing_init(&mut request);
                        request.invideo_promotion.as_mut().unwrap().default_timing.as_mut().unwrap().offset_ms = Some(value.unwrap_or("").to_string());
                    },
                "invideo-promotion.default-timing.type" => {
                        request_invideo_promotion_default_timing_init(&mut request);
                        request.invideo_promotion.as_mut().unwrap().default_timing.as_mut().unwrap().type_ = Some(value.unwrap_or("").to_string());
                    },
                "invideo-promotion.default-timing.duration-ms" => {
                        request_invideo_promotion_default_timing_init(&mut request);
                        request.invideo_promotion.as_mut().unwrap().default_timing.as_mut().unwrap().duration_ms = Some(value.unwrap_or("").to_string());
                    },
                "invideo-promotion.position.corner-position" => {
                        request_invideo_promotion_position_init(&mut request);
                        request.invideo_promotion.as_mut().unwrap().position.as_mut().unwrap().corner_position = Some(value.unwrap_or("").to_string());
                    },
                "invideo-promotion.position.type" => {
                        request_invideo_promotion_position_init(&mut request);
                        request.invideo_promotion.as_mut().unwrap().position.as_mut().unwrap().type_ = Some(value.unwrap_or("").to_string());
                    },
                "invideo-promotion.use-smart-timing" => {
                        request_invideo_promotion_position_init(&mut request);
                        request.invideo_promotion.as_mut().unwrap().use_smart_timing = Some(arg_from_str(value.unwrap_or("false"), err, "invideo-promotion.use-smart-timing", "boolean"));
                    },
                "kind" => {
                        request_invideo_promotion_init(&mut request);
                        request.kind = Some(value.unwrap_or("").to_string());
                    },
                "statistics.comment-count" => {
                        request_statistics_init(&mut request);
                        request.statistics.as_mut().unwrap().comment_count = Some(arg_from_str(value.unwrap_or("-0"), err, "statistics.comment-count", "int64"));
                    },
                "statistics.subscriber-count" => {
                        request_statistics_init(&mut request);
                        request.statistics.as_mut().unwrap().subscriber_count = Some(arg_from_str(value.unwrap_or("-0"), err, "statistics.subscriber-count", "int64"));
                    },
                "statistics.video-count" => {
                        request_statistics_init(&mut request);
                        request.statistics.as_mut().unwrap().video_count = Some(arg_from_str(value.unwrap_or("-0"), err, "statistics.video-count", "int64"));
                    },
                "statistics.hidden-subscriber-count" => {
                        request_statistics_init(&mut request);
                        request.statistics.as_mut().unwrap().hidden_subscriber_count = Some(arg_from_str(value.unwrap_or("false"), err, "statistics.hidden-subscriber-count", "boolean"));
                    },
                "statistics.view-count" => {
                        request_statistics_init(&mut request);
                        request.statistics.as_mut().unwrap().view_count = Some(arg_from_str(value.unwrap_or("-0"), err, "statistics.view-count", "int64"));
                    },
                "content-owner-details.content-owner" => {
                        request_content_owner_details_init(&mut request);
                        request.content_owner_details.as_mut().unwrap().content_owner = Some(value.unwrap_or("").to_string());
                    },
                "content-owner-details.time-linked" => {
                        request_content_owner_details_init(&mut request);
                        request.content_owner_details.as_mut().unwrap().time_linked = Some(value.unwrap_or("").to_string());
                    },
                "topic-details.topic-ids" => {
                        request_topic_details_init(&mut request);
                        if request.topic_details.as_mut().unwrap().topic_ids.is_none() {
                           request.topic_details.as_mut().unwrap().topic_ids = Some(Default::default());
                        }
                                        request.topic_details.as_mut().unwrap().topic_ids.as_mut().unwrap().push(value.unwrap_or("").to_string());
                    },
                "content-details.related-playlists.watch-later" => {
                        request_content_details_related_playlists_init(&mut request);
                        request.content_details.as_mut().unwrap().related_playlists.as_mut().unwrap().watch_later = Some(value.unwrap_or("").to_string());
                    },
                "content-details.related-playlists.watch-history" => {
                        request_content_details_related_playlists_init(&mut request);
                        request.content_details.as_mut().unwrap().related_playlists.as_mut().unwrap().watch_history = Some(value.unwrap_or("").to_string());
                    },
                "content-details.related-playlists.uploads" => {
                        request_content_details_related_playlists_init(&mut request);
                        request.content_details.as_mut().unwrap().related_playlists.as_mut().unwrap().uploads = Some(value.unwrap_or("").to_string());
                    },
                "content-details.related-playlists.favorites" => {
                        request_content_details_related_playlists_init(&mut request);
                        request.content_details.as_mut().unwrap().related_playlists.as_mut().unwrap().favorites = Some(value.unwrap_or("").to_string());
                    },
                "content-details.related-playlists.likes" => {
                        request_content_details_related_playlists_init(&mut request);
                        request.content_details.as_mut().unwrap().related_playlists.as_mut().unwrap().likes = Some(value.unwrap_or("").to_string());
                    },
                "content-details.google-plus-user-id" => {
                        request_content_details_related_playlists_init(&mut request);
                        request.content_details.as_mut().unwrap().google_plus_user_id = Some(value.unwrap_or("").to_string());
                    },
                "branding-settings.image.large-branded-banner-image-imap-script.default" => {
                        request_branding_settings_image_large_branded_banner_image_imap_script_init(&mut request);
                        request.branding_settings.as_mut().unwrap().image.as_mut().unwrap().large_branded_banner_image_imap_script.as_mut().unwrap().default = Some(value.unwrap_or("").to_string());
                    },
                "branding-settings.image.large-branded-banner-image-imap-script.default-language.value" => {
                        request_branding_settings_image_large_branded_banner_image_imap_script_default_language_init(&mut request);
                        request.branding_settings.as_mut().unwrap().image.as_mut().unwrap().large_branded_banner_image_imap_script.as_mut().unwrap().default_language.as_mut().unwrap().value = Some(value.unwrap_or("").to_string());
                    },
                "branding-settings.image.small-branded-banner-image-url.default" => {
                        request_branding_settings_image_small_branded_banner_image_url_init(&mut request);
                        request.branding_settings.as_mut().unwrap().image.as_mut().unwrap().small_branded_banner_image_url.as_mut().unwrap().default = Some(value.unwrap_or("").to_string());
                    },
                "branding-settings.image.small-branded-banner-image-url.default-language.value" => {
                        request_branding_settings_image_small_branded_banner_image_url_default_language_init(&mut request);
                        request.branding_settings.as_mut().unwrap().image.as_mut().unwrap().small_branded_banner_image_url.as_mut().unwrap().default_language.as_mut().unwrap().value = Some(value.unwrap_or("").to_string());
                    },
                "branding-settings.image.banner-tv-image-url" => {
                        request_branding_settings_image_small_branded_banner_image_url_init(&mut request);
                        request.branding_settings.as_mut().unwrap().image.as_mut().unwrap().banner_tv_image_url = Some(value.unwrap_or("").to_string());
                    },
                "branding-settings.image.banner-tv-low-image-url" => {
                        request_branding_settings_image_small_branded_banner_image_url_init(&mut request);
                        request.branding_settings.as_mut().unwrap().image.as_mut().unwrap().banner_tv_low_image_url = Some(value.unwrap_or("").to_string());
                    },
                "branding-settings.image.large-branded-banner-image-url.default" => {
                        request_branding_settings_image_large_branded_banner_image_url_init(&mut request);
                        request.branding_settings.as_mut().unwrap().image.as_mut().unwrap().large_branded_banner_image_url.as_mut().unwrap().default = Some(value.unwrap_or("").to_string());
                    },
                "branding-settings.image.large-branded-banner-image-url.default-language.value" => {
                        request_branding_settings_image_large_branded_banner_image_url_default_language_init(&mut request);
                        request.branding_settings.as_mut().unwrap().image.as_mut().unwrap().large_branded_banner_image_url.as_mut().unwrap().default_language.as_mut().unwrap().value = Some(value.unwrap_or("").to_string());
                    },
                "branding-settings.image.banner-tv-high-image-url" => {
                        request_branding_settings_image_large_branded_banner_image_url_init(&mut request);
                        request.branding_settings.as_mut().unwrap().image.as_mut().unwrap().banner_tv_high_image_url = Some(value.unwrap_or("").to_string());
                    },
                "branding-settings.image.background-image-url.default" => {
                        request_branding_settings_image_background_image_url_init(&mut request);
                        request.branding_settings.as_mut().unwrap().image.as_mut().unwrap().background_image_url.as_mut().unwrap().default = Some(value.unwrap_or("").to_string());
                    },
                "branding-settings.image.background-image-url.default-language.value" => {
                        request_branding_settings_image_background_image_url_default_language_init(&mut request);
                        request.branding_settings.as_mut().unwrap().image.as_mut().unwrap().background_image_url.as_mut().unwrap().default_language.as_mut().unwrap().value = Some(value.unwrap_or("").to_string());
                    },
                "branding-settings.image.small-branded-banner-image-imap-script.default" => {
                        request_branding_settings_image_small_branded_banner_image_imap_script_init(&mut request);
                        request.branding_settings.as_mut().unwrap().image.as_mut().unwrap().small_branded_banner_image_imap_script.as_mut().unwrap().default = Some(value.unwrap_or("").to_string());
                    },
                "branding-settings.image.small-branded-banner-image-imap-script.default-language.value" => {
                        request_branding_settings_image_small_branded_banner_image_imap_script_default_language_init(&mut request);
                        request.branding_settings.as_mut().unwrap().image.as_mut().unwrap().small_branded_banner_image_imap_script.as_mut().unwrap().default_language.as_mut().unwrap().value = Some(value.unwrap_or("").to_string());
                    },
                "branding-settings.image.banner-external-url" => {
                        request_branding_settings_image_small_branded_banner_image_imap_script_init(&mut request);
                        request.branding_settings.as_mut().unwrap().image.as_mut().unwrap().banner_external_url = Some(value.unwrap_or("").to_string());
                    },
                "branding-settings.image.watch-icon-image-url" => {
                        request_branding_settings_image_small_branded_banner_image_imap_script_init(&mut request);
                        request.branding_settings.as_mut().unwrap().image.as_mut().unwrap().watch_icon_image_url = Some(value.unwrap_or("").to_string());
                    },
                "branding-settings.image.banner-tv-medium-image-url" => {
                        request_branding_settings_image_small_branded_banner_image_imap_script_init(&mut request);
                        request.branding_settings.as_mut().unwrap().image.as_mut().unwrap().banner_tv_medium_image_url = Some(value.unwrap_or("").to_string());
                    },
                "branding-settings.image.banner-mobile-image-url" => {
                        request_branding_settings_image_small_branded_banner_image_imap_script_init(&mut request);
                        request.branding_settings.as_mut().unwrap().image.as_mut().unwrap().banner_mobile_image_url = Some(value.unwrap_or("").to_string());
                    },
                "branding-settings.image.banner-tablet-hd-image-url" => {
                        request_branding_settings_image_small_branded_banner_image_imap_script_init(&mut request);
                        request.branding_settings.as_mut().unwrap().image.as_mut().unwrap().banner_tablet_hd_image_url = Some(value.unwrap_or("").to_string());
                    },
                "branding-settings.image.banner-tablet-low-image-url" => {
                        request_branding_settings_image_small_branded_banner_image_imap_script_init(&mut request);
                        request.branding_settings.as_mut().unwrap().image.as_mut().unwrap().banner_tablet_low_image_url = Some(value.unwrap_or("").to_string());
                    },
                "branding-settings.image.tracking-image-url" => {
                        request_branding_settings_image_small_branded_banner_image_imap_script_init(&mut request);
                        request.branding_settings.as_mut().unwrap().image.as_mut().unwrap().tracking_image_url = Some(value.unwrap_or("").to_string());
                    },
                "branding-settings.image.banner-mobile-extra-hd-image-url" => {
                        request_branding_settings_image_small_branded_banner_image_imap_script_init(&mut request);
                        request.branding_settings.as_mut().unwrap().image.as_mut().unwrap().banner_mobile_extra_hd_image_url = Some(value.unwrap_or("").to_string());
                    },
                "branding-settings.image.banner-tablet-image-url" => {
                        request_branding_settings_image_small_branded_banner_image_imap_script_init(&mut request);
                        request.branding_settings.as_mut().unwrap().image.as_mut().unwrap().banner_tablet_image_url = Some(value.unwrap_or("").to_string());
                    },
                "branding-settings.image.banner-mobile-low-image-url" => {
                        request_branding_settings_image_small_branded_banner_image_imap_script_init(&mut request);
                        request.branding_settings.as_mut().unwrap().image.as_mut().unwrap().banner_mobile_low_image_url = Some(value.unwrap_or("").to_string());
                    },
                "branding-settings.image.banner-mobile-medium-hd-image-url" => {
                        request_branding_settings_image_small_branded_banner_image_imap_script_init(&mut request);
                        request.branding_settings.as_mut().unwrap().image.as_mut().unwrap().banner_mobile_medium_hd_image_url = Some(value.unwrap_or("").to_string());
                    },
                "branding-settings.image.banner-tablet-extra-hd-image-url" => {
                        request_branding_settings_image_small_branded_banner_image_imap_script_init(&mut request);
                        request.branding_settings.as_mut().unwrap().image.as_mut().unwrap().banner_tablet_extra_hd_image_url = Some(value.unwrap_or("").to_string());
                    },
                "branding-settings.image.banner-image-url" => {
                        request_branding_settings_image_small_branded_banner_image_imap_script_init(&mut request);
                        request.branding_settings.as_mut().unwrap().image.as_mut().unwrap().banner_image_url = Some(value.unwrap_or("").to_string());
                    },
                "branding-settings.image.banner-mobile-hd-image-url" => {
                        request_branding_settings_image_small_branded_banner_image_imap_script_init(&mut request);
                        request.branding_settings.as_mut().unwrap().image.as_mut().unwrap().banner_mobile_hd_image_url = Some(value.unwrap_or("").to_string());
                    },
                "branding-settings.watch.text-color" => {
                        request_branding_settings_watch_init(&mut request);
                        request.branding_settings.as_mut().unwrap().watch.as_mut().unwrap().text_color = Some(value.unwrap_or("").to_string());
                    },
                "branding-settings.watch.featured-playlist-id" => {
                        request_branding_settings_watch_init(&mut request);
                        request.branding_settings.as_mut().unwrap().watch.as_mut().unwrap().featured_playlist_id = Some(value.unwrap_or("").to_string());
                    },
                "branding-settings.watch.background-color" => {
                        request_branding_settings_watch_init(&mut request);
                        request.branding_settings.as_mut().unwrap().watch.as_mut().unwrap().background_color = Some(value.unwrap_or("").to_string());
                    },
                "branding-settings.channel.description" => {
                        request_branding_settings_channel_init(&mut request);
                        request.branding_settings.as_mut().unwrap().channel.as_mut().unwrap().description = Some(value.unwrap_or("").to_string());
                    },
                "branding-settings.channel.title" => {
                        request_branding_settings_channel_init(&mut request);
                        request.branding_settings.as_mut().unwrap().channel.as_mut().unwrap().title = Some(value.unwrap_or("").to_string());
                    },
                "branding-settings.channel.country" => {
                        request_branding_settings_channel_init(&mut request);
                        request.branding_settings.as_mut().unwrap().channel.as_mut().unwrap().country = Some(value.unwrap_or("").to_string());
                    },
                "branding-settings.channel.show-browse-view" => {
                        request_branding_settings_channel_init(&mut request);
                        request.branding_settings.as_mut().unwrap().channel.as_mut().unwrap().show_browse_view = Some(arg_from_str(value.unwrap_or("false"), err, "branding-settings.channel.show-browse-view", "boolean"));
                    },
                "branding-settings.channel.featured-channels-title" => {
                        request_branding_settings_channel_init(&mut request);
                        request.branding_settings.as_mut().unwrap().channel.as_mut().unwrap().featured_channels_title = Some(value.unwrap_or("").to_string());
                    },
                "branding-settings.channel.default-language" => {
                        request_branding_settings_channel_init(&mut request);
                        request.branding_settings.as_mut().unwrap().channel.as_mut().unwrap().default_language = Some(value.unwrap_or("").to_string());
                    },
                "branding-settings.channel.unsubscribed-trailer" => {
                        request_branding_settings_channel_init(&mut request);
                        request.branding_settings.as_mut().unwrap().channel.as_mut().unwrap().unsubscribed_trailer = Some(value.unwrap_or("").to_string());
                    },
                "branding-settings.channel.keywords" => {
                        request_branding_settings_channel_init(&mut request);
                        request.branding_settings.as_mut().unwrap().channel.as_mut().unwrap().keywords = Some(value.unwrap_or("").to_string());
                    },
                "branding-settings.channel.profile-color" => {
                        request_branding_settings_channel_init(&mut request);
                        request.branding_settings.as_mut().unwrap().channel.as_mut().unwrap().profile_color = Some(value.unwrap_or("").to_string());
                    },
                "branding-settings.channel.default-tab" => {
                        request_branding_settings_channel_init(&mut request);
                        request.branding_settings.as_mut().unwrap().channel.as_mut().unwrap().default_tab = Some(value.unwrap_or("").to_string());
                    },
                "branding-settings.channel.moderate-comments" => {
                        request_branding_settings_channel_init(&mut request);
                        request.branding_settings.as_mut().unwrap().channel.as_mut().unwrap().moderate_comments = Some(arg_from_str(value.unwrap_or("false"), err, "branding-settings.channel.moderate-comments", "boolean"));
                    },
                "branding-settings.channel.featured-channels-urls" => {
                        request_branding_settings_channel_init(&mut request);
                        if request.branding_settings.as_mut().unwrap().channel.as_mut().unwrap().featured_channels_urls.is_none() {
                           request.branding_settings.as_mut().unwrap().channel.as_mut().unwrap().featured_channels_urls = Some(Default::default());
                        }
                                        request.branding_settings.as_mut().unwrap().channel.as_mut().unwrap().featured_channels_urls.as_mut().unwrap().push(value.unwrap_or("").to_string());
                    },
                "branding-settings.channel.tracking-analytics-account-id" => {
                        request_branding_settings_channel_init(&mut request);
                        request.branding_settings.as_mut().unwrap().channel.as_mut().unwrap().tracking_analytics_account_id = Some(value.unwrap_or("").to_string());
                    },
                "branding-settings.channel.show-related-channels" => {
                        request_branding_settings_channel_init(&mut request);
                        request.branding_settings.as_mut().unwrap().channel.as_mut().unwrap().show_related_channels = Some(arg_from_str(value.unwrap_or("false"), err, "branding-settings.channel.show-related-channels", "boolean"));
                    },
                "snippet.thumbnails.default.url" => {
                        request_snippet_thumbnails_default_init(&mut request);
                        request.snippet.as_mut().unwrap().thumbnails.as_mut().unwrap().default.as_mut().unwrap().url = Some(value.unwrap_or("").to_string());
                    },
                "snippet.thumbnails.default.width" => {
                        request_snippet_thumbnails_default_init(&mut request);
                        request.snippet.as_mut().unwrap().thumbnails.as_mut().unwrap().default.as_mut().unwrap().width = Some(arg_from_str(value.unwrap_or("-0"), err, "snippet.thumbnails.default.width", "integer"));
                    },
                "snippet.thumbnails.default.height" => {
                        request_snippet_thumbnails_default_init(&mut request);
                        request.snippet.as_mut().unwrap().thumbnails.as_mut().unwrap().default.as_mut().unwrap().height = Some(arg_from_str(value.unwrap_or("-0"), err, "snippet.thumbnails.default.height", "integer"));
                    },
                "snippet.thumbnails.high.url" => {
                        request_snippet_thumbnails_high_init(&mut request);
                        request.snippet.as_mut().unwrap().thumbnails.as_mut().unwrap().high.as_mut().unwrap().url = Some(value.unwrap_or("").to_string());
                    },
                "snippet.thumbnails.high.width" => {
                        request_snippet_thumbnails_high_init(&mut request);
                        request.snippet.as_mut().unwrap().thumbnails.as_mut().unwrap().high.as_mut().unwrap().width = Some(arg_from_str(value.unwrap_or("-0"), err, "snippet.thumbnails.high.width", "integer"));
                    },
                "snippet.thumbnails.high.height" => {
                        request_snippet_thumbnails_high_init(&mut request);
                        request.snippet.as_mut().unwrap().thumbnails.as_mut().unwrap().high.as_mut().unwrap().height = Some(arg_from_str(value.unwrap_or("-0"), err, "snippet.thumbnails.high.height", "integer"));
                    },
                "snippet.thumbnails.medium.url" => {
                        request_snippet_thumbnails_medium_init(&mut request);
                        request.snippet.as_mut().unwrap().thumbnails.as_mut().unwrap().medium.as_mut().unwrap().url = Some(value.unwrap_or("").to_string());
                    },
                "snippet.thumbnails.medium.width" => {
                        request_snippet_thumbnails_medium_init(&mut request);
                        request.snippet.as_mut().unwrap().thumbnails.as_mut().unwrap().medium.as_mut().unwrap().width = Some(arg_from_str(value.unwrap_or("-0"), err, "snippet.thumbnails.medium.width", "integer"));
                    },
                "snippet.thumbnails.medium.height" => {
                        request_snippet_thumbnails_medium_init(&mut request);
                        request.snippet.as_mut().unwrap().thumbnails.as_mut().unwrap().medium.as_mut().unwrap().height = Some(arg_from_str(value.unwrap_or("-0"), err, "snippet.thumbnails.medium.height", "integer"));
                    },
                "snippet.thumbnails.maxres.url" => {
                        request_snippet_thumbnails_maxres_init(&mut request);
                        request.snippet.as_mut().unwrap().thumbnails.as_mut().unwrap().maxres.as_mut().unwrap().url = Some(value.unwrap_or("").to_string());
                    },
                "snippet.thumbnails.maxres.width" => {
                        request_snippet_thumbnails_maxres_init(&mut request);
                        request.snippet.as_mut().unwrap().thumbnails.as_mut().unwrap().maxres.as_mut().unwrap().width = Some(arg_from_str(value.unwrap_or("-0"), err, "snippet.thumbnails.maxres.width", "integer"));
                    },
                "snippet.thumbnails.maxres.height" => {
                        request_snippet_thumbnails_maxres_init(&mut request);
                        request.snippet.as_mut().unwrap().thumbnails.as_mut().unwrap().maxres.as_mut().unwrap().height = Some(arg_from_str(value.unwrap_or("-0"), err, "snippet.thumbnails.maxres.height", "integer"));
                    },
                "snippet.thumbnails.standard.url" => {
                        request_snippet_thumbnails_standard_init(&mut request);
                        request.snippet.as_mut().unwrap().thumbnails.as_mut().unwrap().standard.as_mut().unwrap().url = Some(value.unwrap_or("").to_string());
                    },
                "snippet.thumbnails.standard.width" => {
                        request_snippet_thumbnails_standard_init(&mut request);
                        request.snippet.as_mut().unwrap().thumbnails.as_mut().unwrap().standard.as_mut().unwrap().width = Some(arg_from_str(value.unwrap_or("-0"), err, "snippet.thumbnails.standard.width", "integer"));
                    },
                "snippet.thumbnails.standard.height" => {
                        request_snippet_thumbnails_standard_init(&mut request);
                        request.snippet.as_mut().unwrap().thumbnails.as_mut().unwrap().standard.as_mut().unwrap().height = Some(arg_from_str(value.unwrap_or("-0"), err, "snippet.thumbnails.standard.height", "integer"));
                    },
                "snippet.title" => {
                        request_snippet_thumbnails_init(&mut request);
                        request.snippet.as_mut().unwrap().title = Some(value.unwrap_or("").to_string());
                    },
                "snippet.country" => {
                        request_snippet_thumbnails_init(&mut request);
                        request.snippet.as_mut().unwrap().country = Some(value.unwrap_or("").to_string());
                    },
                "snippet.default-language" => {
                        request_snippet_thumbnails_init(&mut request);
                        request.snippet.as_mut().unwrap().default_language = Some(value.unwrap_or("").to_string());
                    },
                "snippet.published-at" => {
                        request_snippet_thumbnails_init(&mut request);
                        request.snippet.as_mut().unwrap().published_at = Some(value.unwrap_or("").to_string());
                    },
                "snippet.localized.description" => {
                        request_snippet_localized_init(&mut request);
                        request.snippet.as_mut().unwrap().localized.as_mut().unwrap().description = Some(value.unwrap_or("").to_string());
                    },
                "snippet.localized.title" => {
                        request_snippet_localized_init(&mut request);
                        request.snippet.as_mut().unwrap().localized.as_mut().unwrap().title = Some(value.unwrap_or("").to_string());
                    },
                "snippet.description" => {
                        request_snippet_localized_init(&mut request);
                        request.snippet.as_mut().unwrap().description = Some(value.unwrap_or("").to_string());
                    },
                "audit-details.community-guidelines-good-standing" => {
                        request_audit_details_init(&mut request);
                        request.audit_details.as_mut().unwrap().community_guidelines_good_standing = Some(arg_from_str(value.unwrap_or("false"), err, "audit-details.community-guidelines-good-standing", "boolean"));
                    },
                "audit-details.content-id-claims-good-standing" => {
                        request_audit_details_init(&mut request);
                        request.audit_details.as_mut().unwrap().content_id_claims_good_standing = Some(arg_from_str(value.unwrap_or("false"), err, "audit-details.content-id-claims-good-standing", "boolean"));
                    },
                "audit-details.overall-good-standing" => {
                        request_audit_details_init(&mut request);
                        request.audit_details.as_mut().unwrap().overall_good_standing = Some(arg_from_str(value.unwrap_or("false"), err, "audit-details.overall-good-standing", "boolean"));
                    },
                "audit-details.copyright-strikes-good-standing" => {
                        request_audit_details_init(&mut request);
                        request.audit_details.as_mut().unwrap().copyright_strikes_good_standing = Some(arg_from_str(value.unwrap_or("false"), err, "audit-details.copyright-strikes-good-standing", "boolean"));
                    },
                "etag" => {
                        request_audit_details_init(&mut request);
                        request.etag = Some(value.unwrap_or("").to_string());
                    },
                "id" => {
                        request_audit_details_init(&mut request);
                        request.id = Some(value.unwrap_or("").to_string());
                    },
                _ => {
                    let suggestion = FieldCursor::did_you_mean(key, &vec!["audit-details", "background-color", "background-image-url", "banner-external-url", "banner-image-url", "banner-mobile-extra-hd-image-url", "banner-mobile-hd-image-url", "banner-mobile-image-url", "banner-mobile-low-image-url", "banner-mobile-medium-hd-image-url", "banner-tablet-extra-hd-image-url", "banner-tablet-hd-image-url", "banner-tablet-image-url", "banner-tablet-low-image-url", "banner-tv-high-image-url", "banner-tv-image-url", "banner-tv-low-image-url", "banner-tv-medium-image-url", "branding-settings", "channel", "comment-count", "community-guidelines-good-standing", "content-details", "content-id-claims-good-standing", "content-owner", "content-owner-details", "copyright-strikes-good-standing", "corner-position", "country", "default", "default-language", "default-tab", "default-timing", "description", "duration-ms", "etag", "favorites", "featured-channels-title", "featured-channels-urls", "featured-playlist-id", "google-plus-user-id", "height", "hidden-subscriber-count", "high", "id", "image", "invideo-promotion", "is-linked", "keywords", "kind", "large-branded-banner-image-imap-script", "large-branded-banner-image-url", "likes", "localized", "long-uploads-status", "maxres", "medium", "moderate-comments", "offset-ms", "overall-good-standing", "position", "privacy-status", "profile-color", "published-at", "related-playlists", "show-browse-view", "show-related-channels", "small-branded-banner-image-imap-script", "small-branded-banner-image-url", "snippet", "standard", "statistics", "status", "subscriber-count", "text-color", "thumbnails", "time-linked", "title", "topic-details", "topic-ids", "tracking-analytics-account-id", "tracking-image-url", "type", "unsubscribed-trailer", "uploads", "url", "use-smart-timing", "value", "video-count", "view-count", "watch", "watch-history", "watch-icon-image-url", "watch-later", "width"]);
                    err.issues.push(CLIError::Field(FieldError::Unknown(temp_cursor.to_string(), suggestion, value.map(|v| v.to_string()))));
                }
            }
        }
        let mut call = self.hub.channels().update(request);
        for parg in opt.values_of("v").unwrap_or(Vec::new()).iter() {
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
                                                Vec::new() + &self.gp + &["on-behalf-of-content-owner"]
                                                            ));
                    }
                }
            }
        }
        let protocol = CallType::Standard;
        if dry_run {
            Ok(())
        } else {
            assert!(err.issues.len() == 0);
            for scope in self.opt.values_of("url").unwrap_or(Vec::new()).iter() {
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
                    let mut value = json::value::to_value(&output_schema);
                    remove_json_null_values(&mut value);
                    serde::json::to_writer_pretty(&mut ostream, &value).unwrap();
                    Ok(())
                }
            }
        }
    }

    fn _comment_threads_insert(&self, opt: &ArgMatches<'n, 'a>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        
        let mut request = api::CommentThread::default();
        let mut field_cursor = FieldCursor::default();
        for kvarg in opt.values_of("kv").unwrap_or(Vec::new()).iter() {
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
            fn request_snippet_init(request: &mut api::CommentThread) {
                if request.snippet.is_none() {
                    request.snippet = Some(Default::default());
                }
            }
            
            fn request_snippet_top_level_comment_init(request: &mut api::CommentThread) {
                request_snippet_init(request);
                if request.snippet.as_mut().unwrap().top_level_comment.is_none() {
                    request.snippet.as_mut().unwrap().top_level_comment = Some(Default::default());
                }
            }
            
            fn request_snippet_top_level_comment_snippet_author_channel_id_init(request: &mut api::CommentThread) {
                request_snippet_top_level_comment_snippet_init(request);
                if request.snippet.as_mut().unwrap().top_level_comment.as_mut().unwrap().snippet.as_mut().unwrap().author_channel_id.is_none() {
                    request.snippet.as_mut().unwrap().top_level_comment.as_mut().unwrap().snippet.as_mut().unwrap().author_channel_id = Some(Default::default());
                }
            }
            
            fn request_snippet_top_level_comment_snippet_init(request: &mut api::CommentThread) {
                request_snippet_top_level_comment_init(request);
                if request.snippet.as_mut().unwrap().top_level_comment.as_mut().unwrap().snippet.is_none() {
                    request.snippet.as_mut().unwrap().top_level_comment.as_mut().unwrap().snippet = Some(Default::default());
                }
            }
            
            match &temp_cursor.to_string()[..] {
                "snippet.is-public" => {
                        request_snippet_init(&mut request);
                        request.snippet.as_mut().unwrap().is_public = Some(arg_from_str(value.unwrap_or("false"), err, "snippet.is-public", "boolean"));
                    },
                "snippet.channel-id" => {
                        request_snippet_init(&mut request);
                        request.snippet.as_mut().unwrap().channel_id = Some(value.unwrap_or("").to_string());
                    },
                "snippet.video-id" => {
                        request_snippet_init(&mut request);
                        request.snippet.as_mut().unwrap().video_id = Some(value.unwrap_or("").to_string());
                    },
                "snippet.can-reply" => {
                        request_snippet_init(&mut request);
                        request.snippet.as_mut().unwrap().can_reply = Some(arg_from_str(value.unwrap_or("false"), err, "snippet.can-reply", "boolean"));
                    },
                "snippet.total-reply-count" => {
                        request_snippet_init(&mut request);
                        request.snippet.as_mut().unwrap().total_reply_count = Some(arg_from_str(value.unwrap_or("-0"), err, "snippet.total-reply-count", "integer"));
                    },
                "snippet.top-level-comment.snippet.author-channel-url" => {
                        request_snippet_top_level_comment_snippet_init(&mut request);
                        request.snippet.as_mut().unwrap().top_level_comment.as_mut().unwrap().snippet.as_mut().unwrap().author_channel_url = Some(value.unwrap_or("").to_string());
                    },
                "snippet.top-level-comment.snippet.viewer-rating" => {
                        request_snippet_top_level_comment_snippet_init(&mut request);
                        request.snippet.as_mut().unwrap().top_level_comment.as_mut().unwrap().snippet.as_mut().unwrap().viewer_rating = Some(value.unwrap_or("").to_string());
                    },
                "snippet.top-level-comment.snippet.author-display-name" => {
                        request_snippet_top_level_comment_snippet_init(&mut request);
                        request.snippet.as_mut().unwrap().top_level_comment.as_mut().unwrap().snippet.as_mut().unwrap().author_display_name = Some(value.unwrap_or("").to_string());
                    },
                "snippet.top-level-comment.snippet.channel-id" => {
                        request_snippet_top_level_comment_snippet_init(&mut request);
                        request.snippet.as_mut().unwrap().top_level_comment.as_mut().unwrap().snippet.as_mut().unwrap().channel_id = Some(value.unwrap_or("").to_string());
                    },
                "snippet.top-level-comment.snippet.video-id" => {
                        request_snippet_top_level_comment_snippet_init(&mut request);
                        request.snippet.as_mut().unwrap().top_level_comment.as_mut().unwrap().snippet.as_mut().unwrap().video_id = Some(value.unwrap_or("").to_string());
                    },
                "snippet.top-level-comment.snippet.published-at" => {
                        request_snippet_top_level_comment_snippet_init(&mut request);
                        request.snippet.as_mut().unwrap().top_level_comment.as_mut().unwrap().snippet.as_mut().unwrap().published_at = Some(value.unwrap_or("").to_string());
                    },
                "snippet.top-level-comment.snippet.like-count" => {
                        request_snippet_top_level_comment_snippet_init(&mut request);
                        request.snippet.as_mut().unwrap().top_level_comment.as_mut().unwrap().snippet.as_mut().unwrap().like_count = Some(arg_from_str(value.unwrap_or("-0"), err, "snippet.top-level-comment.snippet.like-count", "integer"));
                    },
                "snippet.top-level-comment.snippet.text-original" => {
                        request_snippet_top_level_comment_snippet_init(&mut request);
                        request.snippet.as_mut().unwrap().top_level_comment.as_mut().unwrap().snippet.as_mut().unwrap().text_original = Some(value.unwrap_or("").to_string());
                    },
                "snippet.top-level-comment.snippet.author-channel-id.value" => {
                        request_snippet_top_level_comment_snippet_author_channel_id_init(&mut request);
                        request.snippet.as_mut().unwrap().top_level_comment.as_mut().unwrap().snippet.as_mut().unwrap().author_channel_id.as_mut().unwrap().value = Some(value.unwrap_or("").to_string());
                    },
                "snippet.top-level-comment.snippet.parent-id" => {
                        request_snippet_top_level_comment_snippet_author_channel_id_init(&mut request);
                        request.snippet.as_mut().unwrap().top_level_comment.as_mut().unwrap().snippet.as_mut().unwrap().parent_id = Some(value.unwrap_or("").to_string());
                    },
                "snippet.top-level-comment.snippet.moderation-status" => {
                        request_snippet_top_level_comment_snippet_author_channel_id_init(&mut request);
                        request.snippet.as_mut().unwrap().top_level_comment.as_mut().unwrap().snippet.as_mut().unwrap().moderation_status = Some(value.unwrap_or("").to_string());
                    },
                "snippet.top-level-comment.snippet.can-rate" => {
                        request_snippet_top_level_comment_snippet_author_channel_id_init(&mut request);
                        request.snippet.as_mut().unwrap().top_level_comment.as_mut().unwrap().snippet.as_mut().unwrap().can_rate = Some(arg_from_str(value.unwrap_or("false"), err, "snippet.top-level-comment.snippet.can-rate", "boolean"));
                    },
                "snippet.top-level-comment.snippet.updated-at" => {
                        request_snippet_top_level_comment_snippet_author_channel_id_init(&mut request);
                        request.snippet.as_mut().unwrap().top_level_comment.as_mut().unwrap().snippet.as_mut().unwrap().updated_at = Some(value.unwrap_or("").to_string());
                    },
                "snippet.top-level-comment.snippet.author-profile-image-url" => {
                        request_snippet_top_level_comment_snippet_author_channel_id_init(&mut request);
                        request.snippet.as_mut().unwrap().top_level_comment.as_mut().unwrap().snippet.as_mut().unwrap().author_profile_image_url = Some(value.unwrap_or("").to_string());
                    },
                "snippet.top-level-comment.snippet.author-googleplus-profile-url" => {
                        request_snippet_top_level_comment_snippet_author_channel_id_init(&mut request);
                        request.snippet.as_mut().unwrap().top_level_comment.as_mut().unwrap().snippet.as_mut().unwrap().author_googleplus_profile_url = Some(value.unwrap_or("").to_string());
                    },
                "snippet.top-level-comment.snippet.text-display" => {
                        request_snippet_top_level_comment_snippet_author_channel_id_init(&mut request);
                        request.snippet.as_mut().unwrap().top_level_comment.as_mut().unwrap().snippet.as_mut().unwrap().text_display = Some(value.unwrap_or("").to_string());
                    },
                "snippet.top-level-comment.kind" => {
                        request_snippet_top_level_comment_snippet_init(&mut request);
                        request.snippet.as_mut().unwrap().top_level_comment.as_mut().unwrap().kind = Some(value.unwrap_or("").to_string());
                    },
                "snippet.top-level-comment.etag" => {
                        request_snippet_top_level_comment_snippet_init(&mut request);
                        request.snippet.as_mut().unwrap().top_level_comment.as_mut().unwrap().etag = Some(value.unwrap_or("").to_string());
                    },
                "snippet.top-level-comment.id" => {
                        request_snippet_top_level_comment_snippet_init(&mut request);
                        request.snippet.as_mut().unwrap().top_level_comment.as_mut().unwrap().id = Some(value.unwrap_or("").to_string());
                    },
                "kind" => {
                        request_snippet_init(&mut request);
                        request.kind = Some(value.unwrap_or("").to_string());
                    },
                "etag" => {
                        request_snippet_init(&mut request);
                        request.etag = Some(value.unwrap_or("").to_string());
                    },
                "id" => {
                        request_snippet_init(&mut request);
                        request.id = Some(value.unwrap_or("").to_string());
                    },
                _ => {
                    let suggestion = FieldCursor::did_you_mean(key, &vec!["author-channel-id", "author-channel-url", "author-display-name", "author-googleplus-profile-url", "author-profile-image-url", "can-rate", "can-reply", "channel-id", "etag", "id", "is-public", "kind", "like-count", "moderation-status", "parent-id", "published-at", "snippet", "text-display", "text-original", "top-level-comment", "total-reply-count", "updated-at", "value", "video-id", "viewer-rating"]);
                    err.issues.push(CLIError::Field(FieldError::Unknown(temp_cursor.to_string(), suggestion, value.map(|v| v.to_string()))));
                }
            }
        }
        let mut call = self.hub.comment_threads().insert(request);
        for parg in opt.values_of("v").unwrap_or(Vec::new()).iter() {
            let (key, value) = parse_kv_arg(&*parg, err, false);
            match key {
                "share-on-google-plus" => {
                    call = call.share_on_google_plus(arg_from_str(value.unwrap_or("false"), err, "share-on-google-plus", "boolean"));
                },
                _ => {
                    let mut found = false;
                    for param in &self.gp {
                        if key == *param {
                            found = true;
                            call = call.param(self.gpm.iter().find(|t| t.0 == key).unwrap_or(&("", key)).1, value.unwrap_or("unset"));
                            break;
                        }
                    }
                    if !found {
                        err.issues.push(CLIError::UnknownParameter(key.to_string(),
                                                Vec::new() + &self.gp + &["share-on-google-plus"]
                                                            ));
                    }
                }
            }
        }
        let protocol = CallType::Standard;
        if dry_run {
            Ok(())
        } else {
            assert!(err.issues.len() == 0);
            for scope in self.opt.values_of("url").unwrap_or(Vec::new()).iter() {
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
                    let mut value = json::value::to_value(&output_schema);
                    remove_json_null_values(&mut value);
                    serde::json::to_writer_pretty(&mut ostream, &value).unwrap();
                    Ok(())
                }
            }
        }
    }

    fn _comment_threads_list(&self, opt: &ArgMatches<'n, 'a>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        let mut call = self.hub.comment_threads().list(opt.value_of("part").unwrap_or(""));
        for parg in opt.values_of("v").unwrap_or(Vec::new()).iter() {
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
                                                Vec::new() + &self.gp + &["all-threads-related-to-channel-id", "channel-id", "video-id", "max-results", "page-token", "search-terms", "text-format", "id", "moderation-status"]
                                                            ));
                    }
                }
            }
        }
        let protocol = CallType::Standard;
        if dry_run {
            Ok(())
        } else {
            assert!(err.issues.len() == 0);
            for scope in self.opt.values_of("url").unwrap_or(Vec::new()).iter() {
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
                    let mut value = json::value::to_value(&output_schema);
                    remove_json_null_values(&mut value);
                    serde::json::to_writer_pretty(&mut ostream, &value).unwrap();
                    Ok(())
                }
            }
        }
    }

    fn _comment_threads_update(&self, opt: &ArgMatches<'n, 'a>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        
        let mut request = api::CommentThread::default();
        let mut field_cursor = FieldCursor::default();
        for kvarg in opt.values_of("kv").unwrap_or(Vec::new()).iter() {
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
            fn request_snippet_init(request: &mut api::CommentThread) {
                if request.snippet.is_none() {
                    request.snippet = Some(Default::default());
                }
            }
            
            fn request_snippet_top_level_comment_init(request: &mut api::CommentThread) {
                request_snippet_init(request);
                if request.snippet.as_mut().unwrap().top_level_comment.is_none() {
                    request.snippet.as_mut().unwrap().top_level_comment = Some(Default::default());
                }
            }
            
            fn request_snippet_top_level_comment_snippet_author_channel_id_init(request: &mut api::CommentThread) {
                request_snippet_top_level_comment_snippet_init(request);
                if request.snippet.as_mut().unwrap().top_level_comment.as_mut().unwrap().snippet.as_mut().unwrap().author_channel_id.is_none() {
                    request.snippet.as_mut().unwrap().top_level_comment.as_mut().unwrap().snippet.as_mut().unwrap().author_channel_id = Some(Default::default());
                }
            }
            
            fn request_snippet_top_level_comment_snippet_init(request: &mut api::CommentThread) {
                request_snippet_top_level_comment_init(request);
                if request.snippet.as_mut().unwrap().top_level_comment.as_mut().unwrap().snippet.is_none() {
                    request.snippet.as_mut().unwrap().top_level_comment.as_mut().unwrap().snippet = Some(Default::default());
                }
            }
            
            match &temp_cursor.to_string()[..] {
                "snippet.is-public" => {
                        request_snippet_init(&mut request);
                        request.snippet.as_mut().unwrap().is_public = Some(arg_from_str(value.unwrap_or("false"), err, "snippet.is-public", "boolean"));
                    },
                "snippet.channel-id" => {
                        request_snippet_init(&mut request);
                        request.snippet.as_mut().unwrap().channel_id = Some(value.unwrap_or("").to_string());
                    },
                "snippet.video-id" => {
                        request_snippet_init(&mut request);
                        request.snippet.as_mut().unwrap().video_id = Some(value.unwrap_or("").to_string());
                    },
                "snippet.can-reply" => {
                        request_snippet_init(&mut request);
                        request.snippet.as_mut().unwrap().can_reply = Some(arg_from_str(value.unwrap_or("false"), err, "snippet.can-reply", "boolean"));
                    },
                "snippet.total-reply-count" => {
                        request_snippet_init(&mut request);
                        request.snippet.as_mut().unwrap().total_reply_count = Some(arg_from_str(value.unwrap_or("-0"), err, "snippet.total-reply-count", "integer"));
                    },
                "snippet.top-level-comment.snippet.author-channel-url" => {
                        request_snippet_top_level_comment_snippet_init(&mut request);
                        request.snippet.as_mut().unwrap().top_level_comment.as_mut().unwrap().snippet.as_mut().unwrap().author_channel_url = Some(value.unwrap_or("").to_string());
                    },
                "snippet.top-level-comment.snippet.viewer-rating" => {
                        request_snippet_top_level_comment_snippet_init(&mut request);
                        request.snippet.as_mut().unwrap().top_level_comment.as_mut().unwrap().snippet.as_mut().unwrap().viewer_rating = Some(value.unwrap_or("").to_string());
                    },
                "snippet.top-level-comment.snippet.author-display-name" => {
                        request_snippet_top_level_comment_snippet_init(&mut request);
                        request.snippet.as_mut().unwrap().top_level_comment.as_mut().unwrap().snippet.as_mut().unwrap().author_display_name = Some(value.unwrap_or("").to_string());
                    },
                "snippet.top-level-comment.snippet.channel-id" => {
                        request_snippet_top_level_comment_snippet_init(&mut request);
                        request.snippet.as_mut().unwrap().top_level_comment.as_mut().unwrap().snippet.as_mut().unwrap().channel_id = Some(value.unwrap_or("").to_string());
                    },
                "snippet.top-level-comment.snippet.video-id" => {
                        request_snippet_top_level_comment_snippet_init(&mut request);
                        request.snippet.as_mut().unwrap().top_level_comment.as_mut().unwrap().snippet.as_mut().unwrap().video_id = Some(value.unwrap_or("").to_string());
                    },
                "snippet.top-level-comment.snippet.published-at" => {
                        request_snippet_top_level_comment_snippet_init(&mut request);
                        request.snippet.as_mut().unwrap().top_level_comment.as_mut().unwrap().snippet.as_mut().unwrap().published_at = Some(value.unwrap_or("").to_string());
                    },
                "snippet.top-level-comment.snippet.like-count" => {
                        request_snippet_top_level_comment_snippet_init(&mut request);
                        request.snippet.as_mut().unwrap().top_level_comment.as_mut().unwrap().snippet.as_mut().unwrap().like_count = Some(arg_from_str(value.unwrap_or("-0"), err, "snippet.top-level-comment.snippet.like-count", "integer"));
                    },
                "snippet.top-level-comment.snippet.text-original" => {
                        request_snippet_top_level_comment_snippet_init(&mut request);
                        request.snippet.as_mut().unwrap().top_level_comment.as_mut().unwrap().snippet.as_mut().unwrap().text_original = Some(value.unwrap_or("").to_string());
                    },
                "snippet.top-level-comment.snippet.author-channel-id.value" => {
                        request_snippet_top_level_comment_snippet_author_channel_id_init(&mut request);
                        request.snippet.as_mut().unwrap().top_level_comment.as_mut().unwrap().snippet.as_mut().unwrap().author_channel_id.as_mut().unwrap().value = Some(value.unwrap_or("").to_string());
                    },
                "snippet.top-level-comment.snippet.parent-id" => {
                        request_snippet_top_level_comment_snippet_author_channel_id_init(&mut request);
                        request.snippet.as_mut().unwrap().top_level_comment.as_mut().unwrap().snippet.as_mut().unwrap().parent_id = Some(value.unwrap_or("").to_string());
                    },
                "snippet.top-level-comment.snippet.moderation-status" => {
                        request_snippet_top_level_comment_snippet_author_channel_id_init(&mut request);
                        request.snippet.as_mut().unwrap().top_level_comment.as_mut().unwrap().snippet.as_mut().unwrap().moderation_status = Some(value.unwrap_or("").to_string());
                    },
                "snippet.top-level-comment.snippet.can-rate" => {
                        request_snippet_top_level_comment_snippet_author_channel_id_init(&mut request);
                        request.snippet.as_mut().unwrap().top_level_comment.as_mut().unwrap().snippet.as_mut().unwrap().can_rate = Some(arg_from_str(value.unwrap_or("false"), err, "snippet.top-level-comment.snippet.can-rate", "boolean"));
                    },
                "snippet.top-level-comment.snippet.updated-at" => {
                        request_snippet_top_level_comment_snippet_author_channel_id_init(&mut request);
                        request.snippet.as_mut().unwrap().top_level_comment.as_mut().unwrap().snippet.as_mut().unwrap().updated_at = Some(value.unwrap_or("").to_string());
                    },
                "snippet.top-level-comment.snippet.author-profile-image-url" => {
                        request_snippet_top_level_comment_snippet_author_channel_id_init(&mut request);
                        request.snippet.as_mut().unwrap().top_level_comment.as_mut().unwrap().snippet.as_mut().unwrap().author_profile_image_url = Some(value.unwrap_or("").to_string());
                    },
                "snippet.top-level-comment.snippet.author-googleplus-profile-url" => {
                        request_snippet_top_level_comment_snippet_author_channel_id_init(&mut request);
                        request.snippet.as_mut().unwrap().top_level_comment.as_mut().unwrap().snippet.as_mut().unwrap().author_googleplus_profile_url = Some(value.unwrap_or("").to_string());
                    },
                "snippet.top-level-comment.snippet.text-display" => {
                        request_snippet_top_level_comment_snippet_author_channel_id_init(&mut request);
                        request.snippet.as_mut().unwrap().top_level_comment.as_mut().unwrap().snippet.as_mut().unwrap().text_display = Some(value.unwrap_or("").to_string());
                    },
                "snippet.top-level-comment.kind" => {
                        request_snippet_top_level_comment_snippet_init(&mut request);
                        request.snippet.as_mut().unwrap().top_level_comment.as_mut().unwrap().kind = Some(value.unwrap_or("").to_string());
                    },
                "snippet.top-level-comment.etag" => {
                        request_snippet_top_level_comment_snippet_init(&mut request);
                        request.snippet.as_mut().unwrap().top_level_comment.as_mut().unwrap().etag = Some(value.unwrap_or("").to_string());
                    },
                "snippet.top-level-comment.id" => {
                        request_snippet_top_level_comment_snippet_init(&mut request);
                        request.snippet.as_mut().unwrap().top_level_comment.as_mut().unwrap().id = Some(value.unwrap_or("").to_string());
                    },
                "kind" => {
                        request_snippet_init(&mut request);
                        request.kind = Some(value.unwrap_or("").to_string());
                    },
                "etag" => {
                        request_snippet_init(&mut request);
                        request.etag = Some(value.unwrap_or("").to_string());
                    },
                "id" => {
                        request_snippet_init(&mut request);
                        request.id = Some(value.unwrap_or("").to_string());
                    },
                _ => {
                    let suggestion = FieldCursor::did_you_mean(key, &vec!["author-channel-id", "author-channel-url", "author-display-name", "author-googleplus-profile-url", "author-profile-image-url", "can-rate", "can-reply", "channel-id", "etag", "id", "is-public", "kind", "like-count", "moderation-status", "parent-id", "published-at", "snippet", "text-display", "text-original", "top-level-comment", "total-reply-count", "updated-at", "value", "video-id", "viewer-rating"]);
                    err.issues.push(CLIError::Field(FieldError::Unknown(temp_cursor.to_string(), suggestion, value.map(|v| v.to_string()))));
                }
            }
        }
        let mut call = self.hub.comment_threads().update(request);
        for parg in opt.values_of("v").unwrap_or(Vec::new()).iter() {
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
                                                Vec::new() + &self.gp + &[]
                                                            ));
                    }
                }
            }
        }
        let protocol = CallType::Standard;
        if dry_run {
            Ok(())
        } else {
            assert!(err.issues.len() == 0);
            for scope in self.opt.values_of("url").unwrap_or(Vec::new()).iter() {
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
                    let mut value = json::value::to_value(&output_schema);
                    remove_json_null_values(&mut value);
                    serde::json::to_writer_pretty(&mut ostream, &value).unwrap();
                    Ok(())
                }
            }
        }
    }

    fn _comments_delete(&self, opt: &ArgMatches<'n, 'a>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        let mut call = self.hub.comments().delete(opt.value_of("id").unwrap_or(""));
        for parg in opt.values_of("v").unwrap_or(Vec::new()).iter() {
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
                                                Vec::new() + &self.gp + &[]
                                                            ));
                    }
                }
            }
        }
        let protocol = CallType::Standard;
        if dry_run {
            Ok(())
        } else {
            assert!(err.issues.len() == 0);
            for scope in self.opt.values_of("url").unwrap_or(Vec::new()).iter() {
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

    fn _comments_insert(&self, opt: &ArgMatches<'n, 'a>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        
        let mut request = api::Comment::default();
        let mut field_cursor = FieldCursor::default();
        for kvarg in opt.values_of("kv").unwrap_or(Vec::new()).iter() {
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
            fn request_snippet_author_channel_id_init(request: &mut api::Comment) {
                request_snippet_init(request);
                if request.snippet.as_mut().unwrap().author_channel_id.is_none() {
                    request.snippet.as_mut().unwrap().author_channel_id = Some(Default::default());
                }
            }
            
            fn request_snippet_init(request: &mut api::Comment) {
                if request.snippet.is_none() {
                    request.snippet = Some(Default::default());
                }
            }
            
            match &temp_cursor.to_string()[..] {
                "snippet.author-channel-url" => {
                        request_snippet_init(&mut request);
                        request.snippet.as_mut().unwrap().author_channel_url = Some(value.unwrap_or("").to_string());
                    },
                "snippet.viewer-rating" => {
                        request_snippet_init(&mut request);
                        request.snippet.as_mut().unwrap().viewer_rating = Some(value.unwrap_or("").to_string());
                    },
                "snippet.author-display-name" => {
                        request_snippet_init(&mut request);
                        request.snippet.as_mut().unwrap().author_display_name = Some(value.unwrap_or("").to_string());
                    },
                "snippet.channel-id" => {
                        request_snippet_init(&mut request);
                        request.snippet.as_mut().unwrap().channel_id = Some(value.unwrap_or("").to_string());
                    },
                "snippet.video-id" => {
                        request_snippet_init(&mut request);
                        request.snippet.as_mut().unwrap().video_id = Some(value.unwrap_or("").to_string());
                    },
                "snippet.published-at" => {
                        request_snippet_init(&mut request);
                        request.snippet.as_mut().unwrap().published_at = Some(value.unwrap_or("").to_string());
                    },
                "snippet.like-count" => {
                        request_snippet_init(&mut request);
                        request.snippet.as_mut().unwrap().like_count = Some(arg_from_str(value.unwrap_or("-0"), err, "snippet.like-count", "integer"));
                    },
                "snippet.text-original" => {
                        request_snippet_init(&mut request);
                        request.snippet.as_mut().unwrap().text_original = Some(value.unwrap_or("").to_string());
                    },
                "snippet.author-channel-id.value" => {
                        request_snippet_author_channel_id_init(&mut request);
                        request.snippet.as_mut().unwrap().author_channel_id.as_mut().unwrap().value = Some(value.unwrap_or("").to_string());
                    },
                "snippet.parent-id" => {
                        request_snippet_author_channel_id_init(&mut request);
                        request.snippet.as_mut().unwrap().parent_id = Some(value.unwrap_or("").to_string());
                    },
                "snippet.moderation-status" => {
                        request_snippet_author_channel_id_init(&mut request);
                        request.snippet.as_mut().unwrap().moderation_status = Some(value.unwrap_or("").to_string());
                    },
                "snippet.can-rate" => {
                        request_snippet_author_channel_id_init(&mut request);
                        request.snippet.as_mut().unwrap().can_rate = Some(arg_from_str(value.unwrap_or("false"), err, "snippet.can-rate", "boolean"));
                    },
                "snippet.updated-at" => {
                        request_snippet_author_channel_id_init(&mut request);
                        request.snippet.as_mut().unwrap().updated_at = Some(value.unwrap_or("").to_string());
                    },
                "snippet.author-profile-image-url" => {
                        request_snippet_author_channel_id_init(&mut request);
                        request.snippet.as_mut().unwrap().author_profile_image_url = Some(value.unwrap_or("").to_string());
                    },
                "snippet.author-googleplus-profile-url" => {
                        request_snippet_author_channel_id_init(&mut request);
                        request.snippet.as_mut().unwrap().author_googleplus_profile_url = Some(value.unwrap_or("").to_string());
                    },
                "snippet.text-display" => {
                        request_snippet_author_channel_id_init(&mut request);
                        request.snippet.as_mut().unwrap().text_display = Some(value.unwrap_or("").to_string());
                    },
                "kind" => {
                        request_snippet_init(&mut request);
                        request.kind = Some(value.unwrap_or("").to_string());
                    },
                "etag" => {
                        request_snippet_init(&mut request);
                        request.etag = Some(value.unwrap_or("").to_string());
                    },
                "id" => {
                        request_snippet_init(&mut request);
                        request.id = Some(value.unwrap_or("").to_string());
                    },
                _ => {
                    let suggestion = FieldCursor::did_you_mean(key, &vec!["author-channel-id", "author-channel-url", "author-display-name", "author-googleplus-profile-url", "author-profile-image-url", "can-rate", "channel-id", "etag", "id", "kind", "like-count", "moderation-status", "parent-id", "published-at", "snippet", "text-display", "text-original", "updated-at", "value", "video-id", "viewer-rating"]);
                    err.issues.push(CLIError::Field(FieldError::Unknown(temp_cursor.to_string(), suggestion, value.map(|v| v.to_string()))));
                }
            }
        }
        let mut call = self.hub.comments().insert(request);
        for parg in opt.values_of("v").unwrap_or(Vec::new()).iter() {
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
                                                Vec::new() + &self.gp + &[]
                                                            ));
                    }
                }
            }
        }
        let protocol = CallType::Standard;
        if dry_run {
            Ok(())
        } else {
            assert!(err.issues.len() == 0);
            for scope in self.opt.values_of("url").unwrap_or(Vec::new()).iter() {
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
                    let mut value = json::value::to_value(&output_schema);
                    remove_json_null_values(&mut value);
                    serde::json::to_writer_pretty(&mut ostream, &value).unwrap();
                    Ok(())
                }
            }
        }
    }

    fn _comments_list(&self, opt: &ArgMatches<'n, 'a>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        let mut call = self.hub.comments().list(opt.value_of("part").unwrap_or(""));
        for parg in opt.values_of("v").unwrap_or(Vec::new()).iter() {
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
                                                Vec::new() + &self.gp + &["page-token", "text-format", "id", "max-results", "parent-id"]
                                                            ));
                    }
                }
            }
        }
        let protocol = CallType::Standard;
        if dry_run {
            Ok(())
        } else {
            assert!(err.issues.len() == 0);
            for scope in self.opt.values_of("url").unwrap_or(Vec::new()).iter() {
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
                    let mut value = json::value::to_value(&output_schema);
                    remove_json_null_values(&mut value);
                    serde::json::to_writer_pretty(&mut ostream, &value).unwrap();
                    Ok(())
                }
            }
        }
    }

    fn _comments_mark_as_spam(&self, opt: &ArgMatches<'n, 'a>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        let mut call = self.hub.comments().mark_as_spam(opt.value_of("id").unwrap_or(""));
        for parg in opt.values_of("v").unwrap_or(Vec::new()).iter() {
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
                                                Vec::new() + &self.gp + &[]
                                                            ));
                    }
                }
            }
        }
        let protocol = CallType::Standard;
        if dry_run {
            Ok(())
        } else {
            assert!(err.issues.len() == 0);
            for scope in self.opt.values_of("url").unwrap_or(Vec::new()).iter() {
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

    fn _comments_set_moderation_status(&self, opt: &ArgMatches<'n, 'a>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        let mut call = self.hub.comments().set_moderation_status(opt.value_of("id").unwrap_or(""), opt.value_of("moderation-status").unwrap_or(""));
        for parg in opt.values_of("v").unwrap_or(Vec::new()).iter() {
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
                                                Vec::new() + &self.gp + &["ban-author"]
                                                            ));
                    }
                }
            }
        }
        let protocol = CallType::Standard;
        if dry_run {
            Ok(())
        } else {
            assert!(err.issues.len() == 0);
            for scope in self.opt.values_of("url").unwrap_or(Vec::new()).iter() {
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

    fn _comments_update(&self, opt: &ArgMatches<'n, 'a>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        
        let mut request = api::Comment::default();
        let mut field_cursor = FieldCursor::default();
        for kvarg in opt.values_of("kv").unwrap_or(Vec::new()).iter() {
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
            fn request_snippet_author_channel_id_init(request: &mut api::Comment) {
                request_snippet_init(request);
                if request.snippet.as_mut().unwrap().author_channel_id.is_none() {
                    request.snippet.as_mut().unwrap().author_channel_id = Some(Default::default());
                }
            }
            
            fn request_snippet_init(request: &mut api::Comment) {
                if request.snippet.is_none() {
                    request.snippet = Some(Default::default());
                }
            }
            
            match &temp_cursor.to_string()[..] {
                "snippet.author-channel-url" => {
                        request_snippet_init(&mut request);
                        request.snippet.as_mut().unwrap().author_channel_url = Some(value.unwrap_or("").to_string());
                    },
                "snippet.viewer-rating" => {
                        request_snippet_init(&mut request);
                        request.snippet.as_mut().unwrap().viewer_rating = Some(value.unwrap_or("").to_string());
                    },
                "snippet.author-display-name" => {
                        request_snippet_init(&mut request);
                        request.snippet.as_mut().unwrap().author_display_name = Some(value.unwrap_or("").to_string());
                    },
                "snippet.channel-id" => {
                        request_snippet_init(&mut request);
                        request.snippet.as_mut().unwrap().channel_id = Some(value.unwrap_or("").to_string());
                    },
                "snippet.video-id" => {
                        request_snippet_init(&mut request);
                        request.snippet.as_mut().unwrap().video_id = Some(value.unwrap_or("").to_string());
                    },
                "snippet.published-at" => {
                        request_snippet_init(&mut request);
                        request.snippet.as_mut().unwrap().published_at = Some(value.unwrap_or("").to_string());
                    },
                "snippet.like-count" => {
                        request_snippet_init(&mut request);
                        request.snippet.as_mut().unwrap().like_count = Some(arg_from_str(value.unwrap_or("-0"), err, "snippet.like-count", "integer"));
                    },
                "snippet.text-original" => {
                        request_snippet_init(&mut request);
                        request.snippet.as_mut().unwrap().text_original = Some(value.unwrap_or("").to_string());
                    },
                "snippet.author-channel-id.value" => {
                        request_snippet_author_channel_id_init(&mut request);
                        request.snippet.as_mut().unwrap().author_channel_id.as_mut().unwrap().value = Some(value.unwrap_or("").to_string());
                    },
                "snippet.parent-id" => {
                        request_snippet_author_channel_id_init(&mut request);
                        request.snippet.as_mut().unwrap().parent_id = Some(value.unwrap_or("").to_string());
                    },
                "snippet.moderation-status" => {
                        request_snippet_author_channel_id_init(&mut request);
                        request.snippet.as_mut().unwrap().moderation_status = Some(value.unwrap_or("").to_string());
                    },
                "snippet.can-rate" => {
                        request_snippet_author_channel_id_init(&mut request);
                        request.snippet.as_mut().unwrap().can_rate = Some(arg_from_str(value.unwrap_or("false"), err, "snippet.can-rate", "boolean"));
                    },
                "snippet.updated-at" => {
                        request_snippet_author_channel_id_init(&mut request);
                        request.snippet.as_mut().unwrap().updated_at = Some(value.unwrap_or("").to_string());
                    },
                "snippet.author-profile-image-url" => {
                        request_snippet_author_channel_id_init(&mut request);
                        request.snippet.as_mut().unwrap().author_profile_image_url = Some(value.unwrap_or("").to_string());
                    },
                "snippet.author-googleplus-profile-url" => {
                        request_snippet_author_channel_id_init(&mut request);
                        request.snippet.as_mut().unwrap().author_googleplus_profile_url = Some(value.unwrap_or("").to_string());
                    },
                "snippet.text-display" => {
                        request_snippet_author_channel_id_init(&mut request);
                        request.snippet.as_mut().unwrap().text_display = Some(value.unwrap_or("").to_string());
                    },
                "kind" => {
                        request_snippet_init(&mut request);
                        request.kind = Some(value.unwrap_or("").to_string());
                    },
                "etag" => {
                        request_snippet_init(&mut request);
                        request.etag = Some(value.unwrap_or("").to_string());
                    },
                "id" => {
                        request_snippet_init(&mut request);
                        request.id = Some(value.unwrap_or("").to_string());
                    },
                _ => {
                    let suggestion = FieldCursor::did_you_mean(key, &vec!["author-channel-id", "author-channel-url", "author-display-name", "author-googleplus-profile-url", "author-profile-image-url", "can-rate", "channel-id", "etag", "id", "kind", "like-count", "moderation-status", "parent-id", "published-at", "snippet", "text-display", "text-original", "updated-at", "value", "video-id", "viewer-rating"]);
                    err.issues.push(CLIError::Field(FieldError::Unknown(temp_cursor.to_string(), suggestion, value.map(|v| v.to_string()))));
                }
            }
        }
        let mut call = self.hub.comments().update(request);
        for parg in opt.values_of("v").unwrap_or(Vec::new()).iter() {
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
                                                Vec::new() + &self.gp + &[]
                                                            ));
                    }
                }
            }
        }
        let protocol = CallType::Standard;
        if dry_run {
            Ok(())
        } else {
            assert!(err.issues.len() == 0);
            for scope in self.opt.values_of("url").unwrap_or(Vec::new()).iter() {
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
                    let mut value = json::value::to_value(&output_schema);
                    remove_json_null_values(&mut value);
                    serde::json::to_writer_pretty(&mut ostream, &value).unwrap();
                    Ok(())
                }
            }
        }
    }

    fn _guide_categories_list(&self, opt: &ArgMatches<'n, 'a>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        let mut call = self.hub.guide_categories().list(opt.value_of("part").unwrap_or(""));
        for parg in opt.values_of("v").unwrap_or(Vec::new()).iter() {
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
                                                Vec::new() + &self.gp + &["region-code", "id", "hl"]
                                                            ));
                    }
                }
            }
        }
        let protocol = CallType::Standard;
        if dry_run {
            Ok(())
        } else {
            assert!(err.issues.len() == 0);
            for scope in self.opt.values_of("url").unwrap_or(Vec::new()).iter() {
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
                    let mut value = json::value::to_value(&output_schema);
                    remove_json_null_values(&mut value);
                    serde::json::to_writer_pretty(&mut ostream, &value).unwrap();
                    Ok(())
                }
            }
        }
    }

    fn _i18n_languages_list(&self, opt: &ArgMatches<'n, 'a>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        let mut call = self.hub.i18n_languages().list(opt.value_of("part").unwrap_or(""));
        for parg in opt.values_of("v").unwrap_or(Vec::new()).iter() {
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
                                                Vec::new() + &self.gp + &["hl"]
                                                            ));
                    }
                }
            }
        }
        let protocol = CallType::Standard;
        if dry_run {
            Ok(())
        } else {
            assert!(err.issues.len() == 0);
            for scope in self.opt.values_of("url").unwrap_or(Vec::new()).iter() {
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
                    let mut value = json::value::to_value(&output_schema);
                    remove_json_null_values(&mut value);
                    serde::json::to_writer_pretty(&mut ostream, &value).unwrap();
                    Ok(())
                }
            }
        }
    }

    fn _i18n_regions_list(&self, opt: &ArgMatches<'n, 'a>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        let mut call = self.hub.i18n_regions().list(opt.value_of("part").unwrap_or(""));
        for parg in opt.values_of("v").unwrap_or(Vec::new()).iter() {
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
                                                Vec::new() + &self.gp + &["hl"]
                                                            ));
                    }
                }
            }
        }
        let protocol = CallType::Standard;
        if dry_run {
            Ok(())
        } else {
            assert!(err.issues.len() == 0);
            for scope in self.opt.values_of("url").unwrap_or(Vec::new()).iter() {
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
                    let mut value = json::value::to_value(&output_schema);
                    remove_json_null_values(&mut value);
                    serde::json::to_writer_pretty(&mut ostream, &value).unwrap();
                    Ok(())
                }
            }
        }
    }

    fn _live_broadcasts_bind(&self, opt: &ArgMatches<'n, 'a>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        let mut call = self.hub.live_broadcasts().bind(opt.value_of("id").unwrap_or(""), opt.value_of("part").unwrap_or(""));
        for parg in opt.values_of("v").unwrap_or(Vec::new()).iter() {
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
                                                Vec::new() + &self.gp + &["on-behalf-of-content-owner-channel", "on-behalf-of-content-owner", "stream-id"]
                                                            ));
                    }
                }
            }
        }
        let protocol = CallType::Standard;
        if dry_run {
            Ok(())
        } else {
            assert!(err.issues.len() == 0);
            for scope in self.opt.values_of("url").unwrap_or(Vec::new()).iter() {
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
                    let mut value = json::value::to_value(&output_schema);
                    remove_json_null_values(&mut value);
                    serde::json::to_writer_pretty(&mut ostream, &value).unwrap();
                    Ok(())
                }
            }
        }
    }

    fn _live_broadcasts_control(&self, opt: &ArgMatches<'n, 'a>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        let mut call = self.hub.live_broadcasts().control(opt.value_of("id").unwrap_or(""), opt.value_of("part").unwrap_or(""));
        for parg in opt.values_of("v").unwrap_or(Vec::new()).iter() {
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
                                                Vec::new() + &self.gp + &["on-behalf-of-content-owner-channel", "on-behalf-of-content-owner", "display-slate", "offset-time-ms", "walltime"]
                                                            ));
                    }
                }
            }
        }
        let protocol = CallType::Standard;
        if dry_run {
            Ok(())
        } else {
            assert!(err.issues.len() == 0);
            for scope in self.opt.values_of("url").unwrap_or(Vec::new()).iter() {
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
                    let mut value = json::value::to_value(&output_schema);
                    remove_json_null_values(&mut value);
                    serde::json::to_writer_pretty(&mut ostream, &value).unwrap();
                    Ok(())
                }
            }
        }
    }

    fn _live_broadcasts_delete(&self, opt: &ArgMatches<'n, 'a>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        let mut call = self.hub.live_broadcasts().delete(opt.value_of("id").unwrap_or(""));
        for parg in opt.values_of("v").unwrap_or(Vec::new()).iter() {
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
                                                Vec::new() + &self.gp + &["on-behalf-of-content-owner-channel", "on-behalf-of-content-owner"]
                                                            ));
                    }
                }
            }
        }
        let protocol = CallType::Standard;
        if dry_run {
            Ok(())
        } else {
            assert!(err.issues.len() == 0);
            for scope in self.opt.values_of("url").unwrap_or(Vec::new()).iter() {
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

    fn _live_broadcasts_insert(&self, opt: &ArgMatches<'n, 'a>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        
        let mut request = api::LiveBroadcast::default();
        let mut field_cursor = FieldCursor::default();
        for kvarg in opt.values_of("kv").unwrap_or(Vec::new()).iter() {
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
            fn request_content_details_init(request: &mut api::LiveBroadcast) {
                if request.content_details.is_none() {
                    request.content_details = Some(Default::default());
                }
            }
            
            fn request_content_details_monitor_stream_init(request: &mut api::LiveBroadcast) {
                request_content_details_init(request);
                if request.content_details.as_mut().unwrap().monitor_stream.is_none() {
                    request.content_details.as_mut().unwrap().monitor_stream = Some(Default::default());
                }
            }
            
            fn request_snippet_init(request: &mut api::LiveBroadcast) {
                if request.snippet.is_none() {
                    request.snippet = Some(Default::default());
                }
            }
            
            fn request_snippet_thumbnails_default_init(request: &mut api::LiveBroadcast) {
                request_snippet_thumbnails_init(request);
                if request.snippet.as_mut().unwrap().thumbnails.as_mut().unwrap().default.is_none() {
                    request.snippet.as_mut().unwrap().thumbnails.as_mut().unwrap().default = Some(Default::default());
                }
            }
            
            fn request_snippet_thumbnails_high_init(request: &mut api::LiveBroadcast) {
                request_snippet_thumbnails_init(request);
                if request.snippet.as_mut().unwrap().thumbnails.as_mut().unwrap().high.is_none() {
                    request.snippet.as_mut().unwrap().thumbnails.as_mut().unwrap().high = Some(Default::default());
                }
            }
            
            fn request_snippet_thumbnails_init(request: &mut api::LiveBroadcast) {
                request_snippet_init(request);
                if request.snippet.as_mut().unwrap().thumbnails.is_none() {
                    request.snippet.as_mut().unwrap().thumbnails = Some(Default::default());
                }
            }
            
            fn request_snippet_thumbnails_maxres_init(request: &mut api::LiveBroadcast) {
                request_snippet_thumbnails_init(request);
                if request.snippet.as_mut().unwrap().thumbnails.as_mut().unwrap().maxres.is_none() {
                    request.snippet.as_mut().unwrap().thumbnails.as_mut().unwrap().maxres = Some(Default::default());
                }
            }
            
            fn request_snippet_thumbnails_medium_init(request: &mut api::LiveBroadcast) {
                request_snippet_thumbnails_init(request);
                if request.snippet.as_mut().unwrap().thumbnails.as_mut().unwrap().medium.is_none() {
                    request.snippet.as_mut().unwrap().thumbnails.as_mut().unwrap().medium = Some(Default::default());
                }
            }
            
            fn request_snippet_thumbnails_standard_init(request: &mut api::LiveBroadcast) {
                request_snippet_thumbnails_init(request);
                if request.snippet.as_mut().unwrap().thumbnails.as_mut().unwrap().standard.is_none() {
                    request.snippet.as_mut().unwrap().thumbnails.as_mut().unwrap().standard = Some(Default::default());
                }
            }
            
            fn request_status_init(request: &mut api::LiveBroadcast) {
                if request.status.is_none() {
                    request.status = Some(Default::default());
                }
            }
            
            match &temp_cursor.to_string()[..] {
                "status.recording-status" => {
                        request_status_init(&mut request);
                        request.status.as_mut().unwrap().recording_status = Some(value.unwrap_or("").to_string());
                    },
                "status.privacy-status" => {
                        request_status_init(&mut request);
                        request.status.as_mut().unwrap().privacy_status = Some(value.unwrap_or("").to_string());
                    },
                "status.life-cycle-status" => {
                        request_status_init(&mut request);
                        request.status.as_mut().unwrap().life_cycle_status = Some(value.unwrap_or("").to_string());
                    },
                "status.is-default-broadcast" => {
                        request_status_init(&mut request);
                        request.status.as_mut().unwrap().is_default_broadcast = Some(arg_from_str(value.unwrap_or("false"), err, "status.is-default-broadcast", "boolean"));
                    },
                "status.live-broadcast-priority" => {
                        request_status_init(&mut request);
                        request.status.as_mut().unwrap().live_broadcast_priority = Some(value.unwrap_or("").to_string());
                    },
                "kind" => {
                        request_status_init(&mut request);
                        request.kind = Some(value.unwrap_or("").to_string());
                    },
                "content-details.start-with-slate" => {
                        request_content_details_init(&mut request);
                        request.content_details.as_mut().unwrap().start_with_slate = Some(arg_from_str(value.unwrap_or("false"), err, "content-details.start-with-slate", "boolean"));
                    },
                "content-details.bound-stream-id" => {
                        request_content_details_init(&mut request);
                        request.content_details.as_mut().unwrap().bound_stream_id = Some(value.unwrap_or("").to_string());
                    },
                "content-details.enable-embed" => {
                        request_content_details_init(&mut request);
                        request.content_details.as_mut().unwrap().enable_embed = Some(arg_from_str(value.unwrap_or("false"), err, "content-details.enable-embed", "boolean"));
                    },
                "content-details.enable-closed-captions" => {
                        request_content_details_init(&mut request);
                        request.content_details.as_mut().unwrap().enable_closed_captions = Some(arg_from_str(value.unwrap_or("false"), err, "content-details.enable-closed-captions", "boolean"));
                    },
                "content-details.enable-content-encryption" => {
                        request_content_details_init(&mut request);
                        request.content_details.as_mut().unwrap().enable_content_encryption = Some(arg_from_str(value.unwrap_or("false"), err, "content-details.enable-content-encryption", "boolean"));
                    },
                "content-details.record-from-start" => {
                        request_content_details_init(&mut request);
                        request.content_details.as_mut().unwrap().record_from_start = Some(arg_from_str(value.unwrap_or("false"), err, "content-details.record-from-start", "boolean"));
                    },
                "content-details.enable-dvr" => {
                        request_content_details_init(&mut request);
                        request.content_details.as_mut().unwrap().enable_dvr = Some(arg_from_str(value.unwrap_or("false"), err, "content-details.enable-dvr", "boolean"));
                    },
                "content-details.monitor-stream.broadcast-stream-delay-ms" => {
                        request_content_details_monitor_stream_init(&mut request);
                        request.content_details.as_mut().unwrap().monitor_stream.as_mut().unwrap().broadcast_stream_delay_ms = Some(arg_from_str(value.unwrap_or("-0"), err, "content-details.monitor-stream.broadcast-stream-delay-ms", "integer"));
                    },
                "content-details.monitor-stream.embed-html" => {
                        request_content_details_monitor_stream_init(&mut request);
                        request.content_details.as_mut().unwrap().monitor_stream.as_mut().unwrap().embed_html = Some(value.unwrap_or("").to_string());
                    },
                "content-details.monitor-stream.enable-monitor-stream" => {
                        request_content_details_monitor_stream_init(&mut request);
                        request.content_details.as_mut().unwrap().monitor_stream.as_mut().unwrap().enable_monitor_stream = Some(arg_from_str(value.unwrap_or("false"), err, "content-details.monitor-stream.enable-monitor-stream", "boolean"));
                    },
                "snippet.actual-end-time" => {
                        request_snippet_init(&mut request);
                        request.snippet.as_mut().unwrap().actual_end_time = Some(value.unwrap_or("").to_string());
                    },
                "snippet.description" => {
                        request_snippet_init(&mut request);
                        request.snippet.as_mut().unwrap().description = Some(value.unwrap_or("").to_string());
                    },
                "snippet.title" => {
                        request_snippet_init(&mut request);
                        request.snippet.as_mut().unwrap().title = Some(value.unwrap_or("").to_string());
                    },
                "snippet.channel-id" => {
                        request_snippet_init(&mut request);
                        request.snippet.as_mut().unwrap().channel_id = Some(value.unwrap_or("").to_string());
                    },
                "snippet.published-at" => {
                        request_snippet_init(&mut request);
                        request.snippet.as_mut().unwrap().published_at = Some(value.unwrap_or("").to_string());
                    },
                "snippet.scheduled-start-time" => {
                        request_snippet_init(&mut request);
                        request.snippet.as_mut().unwrap().scheduled_start_time = Some(value.unwrap_or("").to_string());
                    },
                "snippet.actual-start-time" => {
                        request_snippet_init(&mut request);
                        request.snippet.as_mut().unwrap().actual_start_time = Some(value.unwrap_or("").to_string());
                    },
                "snippet.scheduled-end-time" => {
                        request_snippet_init(&mut request);
                        request.snippet.as_mut().unwrap().scheduled_end_time = Some(value.unwrap_or("").to_string());
                    },
                "snippet.thumbnails.default.url" => {
                        request_snippet_thumbnails_default_init(&mut request);
                        request.snippet.as_mut().unwrap().thumbnails.as_mut().unwrap().default.as_mut().unwrap().url = Some(value.unwrap_or("").to_string());
                    },
                "snippet.thumbnails.default.width" => {
                        request_snippet_thumbnails_default_init(&mut request);
                        request.snippet.as_mut().unwrap().thumbnails.as_mut().unwrap().default.as_mut().unwrap().width = Some(arg_from_str(value.unwrap_or("-0"), err, "snippet.thumbnails.default.width", "integer"));
                    },
                "snippet.thumbnails.default.height" => {
                        request_snippet_thumbnails_default_init(&mut request);
                        request.snippet.as_mut().unwrap().thumbnails.as_mut().unwrap().default.as_mut().unwrap().height = Some(arg_from_str(value.unwrap_or("-0"), err, "snippet.thumbnails.default.height", "integer"));
                    },
                "snippet.thumbnails.high.url" => {
                        request_snippet_thumbnails_high_init(&mut request);
                        request.snippet.as_mut().unwrap().thumbnails.as_mut().unwrap().high.as_mut().unwrap().url = Some(value.unwrap_or("").to_string());
                    },
                "snippet.thumbnails.high.width" => {
                        request_snippet_thumbnails_high_init(&mut request);
                        request.snippet.as_mut().unwrap().thumbnails.as_mut().unwrap().high.as_mut().unwrap().width = Some(arg_from_str(value.unwrap_or("-0"), err, "snippet.thumbnails.high.width", "integer"));
                    },
                "snippet.thumbnails.high.height" => {
                        request_snippet_thumbnails_high_init(&mut request);
                        request.snippet.as_mut().unwrap().thumbnails.as_mut().unwrap().high.as_mut().unwrap().height = Some(arg_from_str(value.unwrap_or("-0"), err, "snippet.thumbnails.high.height", "integer"));
                    },
                "snippet.thumbnails.medium.url" => {
                        request_snippet_thumbnails_medium_init(&mut request);
                        request.snippet.as_mut().unwrap().thumbnails.as_mut().unwrap().medium.as_mut().unwrap().url = Some(value.unwrap_or("").to_string());
                    },
                "snippet.thumbnails.medium.width" => {
                        request_snippet_thumbnails_medium_init(&mut request);
                        request.snippet.as_mut().unwrap().thumbnails.as_mut().unwrap().medium.as_mut().unwrap().width = Some(arg_from_str(value.unwrap_or("-0"), err, "snippet.thumbnails.medium.width", "integer"));
                    },
                "snippet.thumbnails.medium.height" => {
                        request_snippet_thumbnails_medium_init(&mut request);
                        request.snippet.as_mut().unwrap().thumbnails.as_mut().unwrap().medium.as_mut().unwrap().height = Some(arg_from_str(value.unwrap_or("-0"), err, "snippet.thumbnails.medium.height", "integer"));
                    },
                "snippet.thumbnails.maxres.url" => {
                        request_snippet_thumbnails_maxres_init(&mut request);
                        request.snippet.as_mut().unwrap().thumbnails.as_mut().unwrap().maxres.as_mut().unwrap().url = Some(value.unwrap_or("").to_string());
                    },
                "snippet.thumbnails.maxres.width" => {
                        request_snippet_thumbnails_maxres_init(&mut request);
                        request.snippet.as_mut().unwrap().thumbnails.as_mut().unwrap().maxres.as_mut().unwrap().width = Some(arg_from_str(value.unwrap_or("-0"), err, "snippet.thumbnails.maxres.width", "integer"));
                    },
                "snippet.thumbnails.maxres.height" => {
                        request_snippet_thumbnails_maxres_init(&mut request);
                        request.snippet.as_mut().unwrap().thumbnails.as_mut().unwrap().maxres.as_mut().unwrap().height = Some(arg_from_str(value.unwrap_or("-0"), err, "snippet.thumbnails.maxres.height", "integer"));
                    },
                "snippet.thumbnails.standard.url" => {
                        request_snippet_thumbnails_standard_init(&mut request);
                        request.snippet.as_mut().unwrap().thumbnails.as_mut().unwrap().standard.as_mut().unwrap().url = Some(value.unwrap_or("").to_string());
                    },
                "snippet.thumbnails.standard.width" => {
                        request_snippet_thumbnails_standard_init(&mut request);
                        request.snippet.as_mut().unwrap().thumbnails.as_mut().unwrap().standard.as_mut().unwrap().width = Some(arg_from_str(value.unwrap_or("-0"), err, "snippet.thumbnails.standard.width", "integer"));
                    },
                "snippet.thumbnails.standard.height" => {
                        request_snippet_thumbnails_standard_init(&mut request);
                        request.snippet.as_mut().unwrap().thumbnails.as_mut().unwrap().standard.as_mut().unwrap().height = Some(arg_from_str(value.unwrap_or("-0"), err, "snippet.thumbnails.standard.height", "integer"));
                    },
                "etag" => {
                        request_snippet_init(&mut request);
                        request.etag = Some(value.unwrap_or("").to_string());
                    },
                "id" => {
                        request_snippet_init(&mut request);
                        request.id = Some(value.unwrap_or("").to_string());
                    },
                _ => {
                    let suggestion = FieldCursor::did_you_mean(key, &vec!["actual-end-time", "actual-start-time", "bound-stream-id", "broadcast-stream-delay-ms", "channel-id", "content-details", "default", "description", "embed-html", "enable-closed-captions", "enable-content-encryption", "enable-dvr", "enable-embed", "enable-monitor-stream", "etag", "height", "high", "id", "is-default-broadcast", "kind", "life-cycle-status", "live-broadcast-priority", "maxres", "medium", "monitor-stream", "privacy-status", "published-at", "record-from-start", "recording-status", "scheduled-end-time", "scheduled-start-time", "snippet", "standard", "start-with-slate", "status", "thumbnails", "title", "url", "width"]);
                    err.issues.push(CLIError::Field(FieldError::Unknown(temp_cursor.to_string(), suggestion, value.map(|v| v.to_string()))));
                }
            }
        }
        let mut call = self.hub.live_broadcasts().insert(request);
        for parg in opt.values_of("v").unwrap_or(Vec::new()).iter() {
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
                                                Vec::new() + &self.gp + &["on-behalf-of-content-owner-channel", "on-behalf-of-content-owner"]
                                                            ));
                    }
                }
            }
        }
        let protocol = CallType::Standard;
        if dry_run {
            Ok(())
        } else {
            assert!(err.issues.len() == 0);
            for scope in self.opt.values_of("url").unwrap_or(Vec::new()).iter() {
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
                    let mut value = json::value::to_value(&output_schema);
                    remove_json_null_values(&mut value);
                    serde::json::to_writer_pretty(&mut ostream, &value).unwrap();
                    Ok(())
                }
            }
        }
    }

    fn _live_broadcasts_list(&self, opt: &ArgMatches<'n, 'a>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        let mut call = self.hub.live_broadcasts().list(opt.value_of("part").unwrap_or(""));
        for parg in opt.values_of("v").unwrap_or(Vec::new()).iter() {
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
                                                Vec::new() + &self.gp + &["broadcast-status", "on-behalf-of-content-owner", "on-behalf-of-content-owner-channel", "mine", "max-results", "page-token", "id"]
                                                            ));
                    }
                }
            }
        }
        let protocol = CallType::Standard;
        if dry_run {
            Ok(())
        } else {
            assert!(err.issues.len() == 0);
            for scope in self.opt.values_of("url").unwrap_or(Vec::new()).iter() {
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
                    let mut value = json::value::to_value(&output_schema);
                    remove_json_null_values(&mut value);
                    serde::json::to_writer_pretty(&mut ostream, &value).unwrap();
                    Ok(())
                }
            }
        }
    }

    fn _live_broadcasts_transition(&self, opt: &ArgMatches<'n, 'a>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        let mut call = self.hub.live_broadcasts().transition(opt.value_of("broadcast-status").unwrap_or(""), opt.value_of("id").unwrap_or(""), opt.value_of("part").unwrap_or(""));
        for parg in opt.values_of("v").unwrap_or(Vec::new()).iter() {
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
                                                Vec::new() + &self.gp + &["on-behalf-of-content-owner-channel", "on-behalf-of-content-owner"]
                                                            ));
                    }
                }
            }
        }
        let protocol = CallType::Standard;
        if dry_run {
            Ok(())
        } else {
            assert!(err.issues.len() == 0);
            for scope in self.opt.values_of("url").unwrap_or(Vec::new()).iter() {
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
                    let mut value = json::value::to_value(&output_schema);
                    remove_json_null_values(&mut value);
                    serde::json::to_writer_pretty(&mut ostream, &value).unwrap();
                    Ok(())
                }
            }
        }
    }

    fn _live_broadcasts_update(&self, opt: &ArgMatches<'n, 'a>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        
        let mut request = api::LiveBroadcast::default();
        let mut field_cursor = FieldCursor::default();
        for kvarg in opt.values_of("kv").unwrap_or(Vec::new()).iter() {
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
            fn request_content_details_init(request: &mut api::LiveBroadcast) {
                if request.content_details.is_none() {
                    request.content_details = Some(Default::default());
                }
            }
            
            fn request_content_details_monitor_stream_init(request: &mut api::LiveBroadcast) {
                request_content_details_init(request);
                if request.content_details.as_mut().unwrap().monitor_stream.is_none() {
                    request.content_details.as_mut().unwrap().monitor_stream = Some(Default::default());
                }
            }
            
            fn request_snippet_init(request: &mut api::LiveBroadcast) {
                if request.snippet.is_none() {
                    request.snippet = Some(Default::default());
                }
            }
            
            fn request_snippet_thumbnails_default_init(request: &mut api::LiveBroadcast) {
                request_snippet_thumbnails_init(request);
                if request.snippet.as_mut().unwrap().thumbnails.as_mut().unwrap().default.is_none() {
                    request.snippet.as_mut().unwrap().thumbnails.as_mut().unwrap().default = Some(Default::default());
                }
            }
            
            fn request_snippet_thumbnails_high_init(request: &mut api::LiveBroadcast) {
                request_snippet_thumbnails_init(request);
                if request.snippet.as_mut().unwrap().thumbnails.as_mut().unwrap().high.is_none() {
                    request.snippet.as_mut().unwrap().thumbnails.as_mut().unwrap().high = Some(Default::default());
                }
            }
            
            fn request_snippet_thumbnails_init(request: &mut api::LiveBroadcast) {
                request_snippet_init(request);
                if request.snippet.as_mut().unwrap().thumbnails.is_none() {
                    request.snippet.as_mut().unwrap().thumbnails = Some(Default::default());
                }
            }
            
            fn request_snippet_thumbnails_maxres_init(request: &mut api::LiveBroadcast) {
                request_snippet_thumbnails_init(request);
                if request.snippet.as_mut().unwrap().thumbnails.as_mut().unwrap().maxres.is_none() {
                    request.snippet.as_mut().unwrap().thumbnails.as_mut().unwrap().maxres = Some(Default::default());
                }
            }
            
            fn request_snippet_thumbnails_medium_init(request: &mut api::LiveBroadcast) {
                request_snippet_thumbnails_init(request);
                if request.snippet.as_mut().unwrap().thumbnails.as_mut().unwrap().medium.is_none() {
                    request.snippet.as_mut().unwrap().thumbnails.as_mut().unwrap().medium = Some(Default::default());
                }
            }
            
            fn request_snippet_thumbnails_standard_init(request: &mut api::LiveBroadcast) {
                request_snippet_thumbnails_init(request);
                if request.snippet.as_mut().unwrap().thumbnails.as_mut().unwrap().standard.is_none() {
                    request.snippet.as_mut().unwrap().thumbnails.as_mut().unwrap().standard = Some(Default::default());
                }
            }
            
            fn request_status_init(request: &mut api::LiveBroadcast) {
                if request.status.is_none() {
                    request.status = Some(Default::default());
                }
            }
            
            match &temp_cursor.to_string()[..] {
                "status.recording-status" => {
                        request_status_init(&mut request);
                        request.status.as_mut().unwrap().recording_status = Some(value.unwrap_or("").to_string());
                    },
                "status.privacy-status" => {
                        request_status_init(&mut request);
                        request.status.as_mut().unwrap().privacy_status = Some(value.unwrap_or("").to_string());
                    },
                "status.life-cycle-status" => {
                        request_status_init(&mut request);
                        request.status.as_mut().unwrap().life_cycle_status = Some(value.unwrap_or("").to_string());
                    },
                "status.is-default-broadcast" => {
                        request_status_init(&mut request);
                        request.status.as_mut().unwrap().is_default_broadcast = Some(arg_from_str(value.unwrap_or("false"), err, "status.is-default-broadcast", "boolean"));
                    },
                "status.live-broadcast-priority" => {
                        request_status_init(&mut request);
                        request.status.as_mut().unwrap().live_broadcast_priority = Some(value.unwrap_or("").to_string());
                    },
                "kind" => {
                        request_status_init(&mut request);
                        request.kind = Some(value.unwrap_or("").to_string());
                    },
                "content-details.start-with-slate" => {
                        request_content_details_init(&mut request);
                        request.content_details.as_mut().unwrap().start_with_slate = Some(arg_from_str(value.unwrap_or("false"), err, "content-details.start-with-slate", "boolean"));
                    },
                "content-details.bound-stream-id" => {
                        request_content_details_init(&mut request);
                        request.content_details.as_mut().unwrap().bound_stream_id = Some(value.unwrap_or("").to_string());
                    },
                "content-details.enable-embed" => {
                        request_content_details_init(&mut request);
                        request.content_details.as_mut().unwrap().enable_embed = Some(arg_from_str(value.unwrap_or("false"), err, "content-details.enable-embed", "boolean"));
                    },
                "content-details.enable-closed-captions" => {
                        request_content_details_init(&mut request);
                        request.content_details.as_mut().unwrap().enable_closed_captions = Some(arg_from_str(value.unwrap_or("false"), err, "content-details.enable-closed-captions", "boolean"));
                    },
                "content-details.enable-content-encryption" => {
                        request_content_details_init(&mut request);
                        request.content_details.as_mut().unwrap().enable_content_encryption = Some(arg_from_str(value.unwrap_or("false"), err, "content-details.enable-content-encryption", "boolean"));
                    },
                "content-details.record-from-start" => {
                        request_content_details_init(&mut request);
                        request.content_details.as_mut().unwrap().record_from_start = Some(arg_from_str(value.unwrap_or("false"), err, "content-details.record-from-start", "boolean"));
                    },
                "content-details.enable-dvr" => {
                        request_content_details_init(&mut request);
                        request.content_details.as_mut().unwrap().enable_dvr = Some(arg_from_str(value.unwrap_or("false"), err, "content-details.enable-dvr", "boolean"));
                    },
                "content-details.monitor-stream.broadcast-stream-delay-ms" => {
                        request_content_details_monitor_stream_init(&mut request);
                        request.content_details.as_mut().unwrap().monitor_stream.as_mut().unwrap().broadcast_stream_delay_ms = Some(arg_from_str(value.unwrap_or("-0"), err, "content-details.monitor-stream.broadcast-stream-delay-ms", "integer"));
                    },
                "content-details.monitor-stream.embed-html" => {
                        request_content_details_monitor_stream_init(&mut request);
                        request.content_details.as_mut().unwrap().monitor_stream.as_mut().unwrap().embed_html = Some(value.unwrap_or("").to_string());
                    },
                "content-details.monitor-stream.enable-monitor-stream" => {
                        request_content_details_monitor_stream_init(&mut request);
                        request.content_details.as_mut().unwrap().monitor_stream.as_mut().unwrap().enable_monitor_stream = Some(arg_from_str(value.unwrap_or("false"), err, "content-details.monitor-stream.enable-monitor-stream", "boolean"));
                    },
                "snippet.actual-end-time" => {
                        request_snippet_init(&mut request);
                        request.snippet.as_mut().unwrap().actual_end_time = Some(value.unwrap_or("").to_string());
                    },
                "snippet.description" => {
                        request_snippet_init(&mut request);
                        request.snippet.as_mut().unwrap().description = Some(value.unwrap_or("").to_string());
                    },
                "snippet.title" => {
                        request_snippet_init(&mut request);
                        request.snippet.as_mut().unwrap().title = Some(value.unwrap_or("").to_string());
                    },
                "snippet.channel-id" => {
                        request_snippet_init(&mut request);
                        request.snippet.as_mut().unwrap().channel_id = Some(value.unwrap_or("").to_string());
                    },
                "snippet.published-at" => {
                        request_snippet_init(&mut request);
                        request.snippet.as_mut().unwrap().published_at = Some(value.unwrap_or("").to_string());
                    },
                "snippet.scheduled-start-time" => {
                        request_snippet_init(&mut request);
                        request.snippet.as_mut().unwrap().scheduled_start_time = Some(value.unwrap_or("").to_string());
                    },
                "snippet.actual-start-time" => {
                        request_snippet_init(&mut request);
                        request.snippet.as_mut().unwrap().actual_start_time = Some(value.unwrap_or("").to_string());
                    },
                "snippet.scheduled-end-time" => {
                        request_snippet_init(&mut request);
                        request.snippet.as_mut().unwrap().scheduled_end_time = Some(value.unwrap_or("").to_string());
                    },
                "snippet.thumbnails.default.url" => {
                        request_snippet_thumbnails_default_init(&mut request);
                        request.snippet.as_mut().unwrap().thumbnails.as_mut().unwrap().default.as_mut().unwrap().url = Some(value.unwrap_or("").to_string());
                    },
                "snippet.thumbnails.default.width" => {
                        request_snippet_thumbnails_default_init(&mut request);
                        request.snippet.as_mut().unwrap().thumbnails.as_mut().unwrap().default.as_mut().unwrap().width = Some(arg_from_str(value.unwrap_or("-0"), err, "snippet.thumbnails.default.width", "integer"));
                    },
                "snippet.thumbnails.default.height" => {
                        request_snippet_thumbnails_default_init(&mut request);
                        request.snippet.as_mut().unwrap().thumbnails.as_mut().unwrap().default.as_mut().unwrap().height = Some(arg_from_str(value.unwrap_or("-0"), err, "snippet.thumbnails.default.height", "integer"));
                    },
                "snippet.thumbnails.high.url" => {
                        request_snippet_thumbnails_high_init(&mut request);
                        request.snippet.as_mut().unwrap().thumbnails.as_mut().unwrap().high.as_mut().unwrap().url = Some(value.unwrap_or("").to_string());
                    },
                "snippet.thumbnails.high.width" => {
                        request_snippet_thumbnails_high_init(&mut request);
                        request.snippet.as_mut().unwrap().thumbnails.as_mut().unwrap().high.as_mut().unwrap().width = Some(arg_from_str(value.unwrap_or("-0"), err, "snippet.thumbnails.high.width", "integer"));
                    },
                "snippet.thumbnails.high.height" => {
                        request_snippet_thumbnails_high_init(&mut request);
                        request.snippet.as_mut().unwrap().thumbnails.as_mut().unwrap().high.as_mut().unwrap().height = Some(arg_from_str(value.unwrap_or("-0"), err, "snippet.thumbnails.high.height", "integer"));
                    },
                "snippet.thumbnails.medium.url" => {
                        request_snippet_thumbnails_medium_init(&mut request);
                        request.snippet.as_mut().unwrap().thumbnails.as_mut().unwrap().medium.as_mut().unwrap().url = Some(value.unwrap_or("").to_string());
                    },
                "snippet.thumbnails.medium.width" => {
                        request_snippet_thumbnails_medium_init(&mut request);
                        request.snippet.as_mut().unwrap().thumbnails.as_mut().unwrap().medium.as_mut().unwrap().width = Some(arg_from_str(value.unwrap_or("-0"), err, "snippet.thumbnails.medium.width", "integer"));
                    },
                "snippet.thumbnails.medium.height" => {
                        request_snippet_thumbnails_medium_init(&mut request);
                        request.snippet.as_mut().unwrap().thumbnails.as_mut().unwrap().medium.as_mut().unwrap().height = Some(arg_from_str(value.unwrap_or("-0"), err, "snippet.thumbnails.medium.height", "integer"));
                    },
                "snippet.thumbnails.maxres.url" => {
                        request_snippet_thumbnails_maxres_init(&mut request);
                        request.snippet.as_mut().unwrap().thumbnails.as_mut().unwrap().maxres.as_mut().unwrap().url = Some(value.unwrap_or("").to_string());
                    },
                "snippet.thumbnails.maxres.width" => {
                        request_snippet_thumbnails_maxres_init(&mut request);
                        request.snippet.as_mut().unwrap().thumbnails.as_mut().unwrap().maxres.as_mut().unwrap().width = Some(arg_from_str(value.unwrap_or("-0"), err, "snippet.thumbnails.maxres.width", "integer"));
                    },
                "snippet.thumbnails.maxres.height" => {
                        request_snippet_thumbnails_maxres_init(&mut request);
                        request.snippet.as_mut().unwrap().thumbnails.as_mut().unwrap().maxres.as_mut().unwrap().height = Some(arg_from_str(value.unwrap_or("-0"), err, "snippet.thumbnails.maxres.height", "integer"));
                    },
                "snippet.thumbnails.standard.url" => {
                        request_snippet_thumbnails_standard_init(&mut request);
                        request.snippet.as_mut().unwrap().thumbnails.as_mut().unwrap().standard.as_mut().unwrap().url = Some(value.unwrap_or("").to_string());
                    },
                "snippet.thumbnails.standard.width" => {
                        request_snippet_thumbnails_standard_init(&mut request);
                        request.snippet.as_mut().unwrap().thumbnails.as_mut().unwrap().standard.as_mut().unwrap().width = Some(arg_from_str(value.unwrap_or("-0"), err, "snippet.thumbnails.standard.width", "integer"));
                    },
                "snippet.thumbnails.standard.height" => {
                        request_snippet_thumbnails_standard_init(&mut request);
                        request.snippet.as_mut().unwrap().thumbnails.as_mut().unwrap().standard.as_mut().unwrap().height = Some(arg_from_str(value.unwrap_or("-0"), err, "snippet.thumbnails.standard.height", "integer"));
                    },
                "etag" => {
                        request_snippet_init(&mut request);
                        request.etag = Some(value.unwrap_or("").to_string());
                    },
                "id" => {
                        request_snippet_init(&mut request);
                        request.id = Some(value.unwrap_or("").to_string());
                    },
                _ => {
                    let suggestion = FieldCursor::did_you_mean(key, &vec!["actual-end-time", "actual-start-time", "bound-stream-id", "broadcast-stream-delay-ms", "channel-id", "content-details", "default", "description", "embed-html", "enable-closed-captions", "enable-content-encryption", "enable-dvr", "enable-embed", "enable-monitor-stream", "etag", "height", "high", "id", "is-default-broadcast", "kind", "life-cycle-status", "live-broadcast-priority", "maxres", "medium", "monitor-stream", "privacy-status", "published-at", "record-from-start", "recording-status", "scheduled-end-time", "scheduled-start-time", "snippet", "standard", "start-with-slate", "status", "thumbnails", "title", "url", "width"]);
                    err.issues.push(CLIError::Field(FieldError::Unknown(temp_cursor.to_string(), suggestion, value.map(|v| v.to_string()))));
                }
            }
        }
        let mut call = self.hub.live_broadcasts().update(request);
        for parg in opt.values_of("v").unwrap_or(Vec::new()).iter() {
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
                                                Vec::new() + &self.gp + &["on-behalf-of-content-owner-channel", "on-behalf-of-content-owner"]
                                                            ));
                    }
                }
            }
        }
        let protocol = CallType::Standard;
        if dry_run {
            Ok(())
        } else {
            assert!(err.issues.len() == 0);
            for scope in self.opt.values_of("url").unwrap_or(Vec::new()).iter() {
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
                    let mut value = json::value::to_value(&output_schema);
                    remove_json_null_values(&mut value);
                    serde::json::to_writer_pretty(&mut ostream, &value).unwrap();
                    Ok(())
                }
            }
        }
    }

    fn _live_streams_delete(&self, opt: &ArgMatches<'n, 'a>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        let mut call = self.hub.live_streams().delete(opt.value_of("id").unwrap_or(""));
        for parg in opt.values_of("v").unwrap_or(Vec::new()).iter() {
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
                                                Vec::new() + &self.gp + &["on-behalf-of-content-owner-channel", "on-behalf-of-content-owner"]
                                                            ));
                    }
                }
            }
        }
        let protocol = CallType::Standard;
        if dry_run {
            Ok(())
        } else {
            assert!(err.issues.len() == 0);
            for scope in self.opt.values_of("url").unwrap_or(Vec::new()).iter() {
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

    fn _live_streams_insert(&self, opt: &ArgMatches<'n, 'a>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        
        let mut request = api::LiveStream::default();
        let mut field_cursor = FieldCursor::default();
        for kvarg in opt.values_of("kv").unwrap_or(Vec::new()).iter() {
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
            fn request_cdn_ingestion_info_init(request: &mut api::LiveStream) {
                request_cdn_init(request);
                if request.cdn.as_mut().unwrap().ingestion_info.is_none() {
                    request.cdn.as_mut().unwrap().ingestion_info = Some(Default::default());
                }
            }
            
            fn request_cdn_init(request: &mut api::LiveStream) {
                if request.cdn.is_none() {
                    request.cdn = Some(Default::default());
                }
            }
            
            fn request_content_details_init(request: &mut api::LiveStream) {
                if request.content_details.is_none() {
                    request.content_details = Some(Default::default());
                }
            }
            
            fn request_snippet_init(request: &mut api::LiveStream) {
                if request.snippet.is_none() {
                    request.snippet = Some(Default::default());
                }
            }
            
            fn request_status_init(request: &mut api::LiveStream) {
                if request.status.is_none() {
                    request.status = Some(Default::default());
                }
            }
            
            match &temp_cursor.to_string()[..] {
                "status.is-default-stream" => {
                        request_status_init(&mut request);
                        request.status.as_mut().unwrap().is_default_stream = Some(arg_from_str(value.unwrap_or("false"), err, "status.is-default-stream", "boolean"));
                    },
                "status.stream-status" => {
                        request_status_init(&mut request);
                        request.status.as_mut().unwrap().stream_status = Some(value.unwrap_or("").to_string());
                    },
                "kind" => {
                        request_status_init(&mut request);
                        request.kind = Some(value.unwrap_or("").to_string());
                    },
                "content-details.is-reusable" => {
                        request_content_details_init(&mut request);
                        request.content_details.as_mut().unwrap().is_reusable = Some(arg_from_str(value.unwrap_or("false"), err, "content-details.is-reusable", "boolean"));
                    },
                "content-details.closed-captions-ingestion-url" => {
                        request_content_details_init(&mut request);
                        request.content_details.as_mut().unwrap().closed_captions_ingestion_url = Some(value.unwrap_or("").to_string());
                    },
                "cdn.ingestion-type" => {
                        request_cdn_init(&mut request);
                        request.cdn.as_mut().unwrap().ingestion_type = Some(value.unwrap_or("").to_string());
                    },
                "cdn.ingestion-info.backup-ingestion-address" => {
                        request_cdn_ingestion_info_init(&mut request);
                        request.cdn.as_mut().unwrap().ingestion_info.as_mut().unwrap().backup_ingestion_address = Some(value.unwrap_or("").to_string());
                    },
                "cdn.ingestion-info.stream-name" => {
                        request_cdn_ingestion_info_init(&mut request);
                        request.cdn.as_mut().unwrap().ingestion_info.as_mut().unwrap().stream_name = Some(value.unwrap_or("").to_string());
                    },
                "cdn.ingestion-info.ingestion-address" => {
                        request_cdn_ingestion_info_init(&mut request);
                        request.cdn.as_mut().unwrap().ingestion_info.as_mut().unwrap().ingestion_address = Some(value.unwrap_or("").to_string());
                    },
                "cdn.format" => {
                        request_cdn_ingestion_info_init(&mut request);
                        request.cdn.as_mut().unwrap().format = Some(value.unwrap_or("").to_string());
                    },
                "snippet.channel-id" => {
                        request_snippet_init(&mut request);
                        request.snippet.as_mut().unwrap().channel_id = Some(value.unwrap_or("").to_string());
                    },
                "snippet.description" => {
                        request_snippet_init(&mut request);
                        request.snippet.as_mut().unwrap().description = Some(value.unwrap_or("").to_string());
                    },
                "snippet.published-at" => {
                        request_snippet_init(&mut request);
                        request.snippet.as_mut().unwrap().published_at = Some(value.unwrap_or("").to_string());
                    },
                "snippet.title" => {
                        request_snippet_init(&mut request);
                        request.snippet.as_mut().unwrap().title = Some(value.unwrap_or("").to_string());
                    },
                "etag" => {
                        request_snippet_init(&mut request);
                        request.etag = Some(value.unwrap_or("").to_string());
                    },
                "id" => {
                        request_snippet_init(&mut request);
                        request.id = Some(value.unwrap_or("").to_string());
                    },
                _ => {
                    let suggestion = FieldCursor::did_you_mean(key, &vec!["backup-ingestion-address", "cdn", "channel-id", "closed-captions-ingestion-url", "content-details", "description", "etag", "format", "id", "ingestion-address", "ingestion-info", "ingestion-type", "is-default-stream", "is-reusable", "kind", "published-at", "snippet", "status", "stream-name", "stream-status", "title"]);
                    err.issues.push(CLIError::Field(FieldError::Unknown(temp_cursor.to_string(), suggestion, value.map(|v| v.to_string()))));
                }
            }
        }
        let mut call = self.hub.live_streams().insert(request);
        for parg in opt.values_of("v").unwrap_or(Vec::new()).iter() {
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
                                                Vec::new() + &self.gp + &["on-behalf-of-content-owner-channel", "on-behalf-of-content-owner"]
                                                            ));
                    }
                }
            }
        }
        let protocol = CallType::Standard;
        if dry_run {
            Ok(())
        } else {
            assert!(err.issues.len() == 0);
            for scope in self.opt.values_of("url").unwrap_or(Vec::new()).iter() {
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
                    let mut value = json::value::to_value(&output_schema);
                    remove_json_null_values(&mut value);
                    serde::json::to_writer_pretty(&mut ostream, &value).unwrap();
                    Ok(())
                }
            }
        }
    }

    fn _live_streams_list(&self, opt: &ArgMatches<'n, 'a>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        let mut call = self.hub.live_streams().list(opt.value_of("part").unwrap_or(""));
        for parg in opt.values_of("v").unwrap_or(Vec::new()).iter() {
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
                                                Vec::new() + &self.gp + &["on-behalf-of-content-owner", "on-behalf-of-content-owner-channel", "mine", "max-results", "page-token", "id"]
                                                            ));
                    }
                }
            }
        }
        let protocol = CallType::Standard;
        if dry_run {
            Ok(())
        } else {
            assert!(err.issues.len() == 0);
            for scope in self.opt.values_of("url").unwrap_or(Vec::new()).iter() {
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
                    let mut value = json::value::to_value(&output_schema);
                    remove_json_null_values(&mut value);
                    serde::json::to_writer_pretty(&mut ostream, &value).unwrap();
                    Ok(())
                }
            }
        }
    }

    fn _live_streams_update(&self, opt: &ArgMatches<'n, 'a>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        
        let mut request = api::LiveStream::default();
        let mut field_cursor = FieldCursor::default();
        for kvarg in opt.values_of("kv").unwrap_or(Vec::new()).iter() {
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
            fn request_cdn_ingestion_info_init(request: &mut api::LiveStream) {
                request_cdn_init(request);
                if request.cdn.as_mut().unwrap().ingestion_info.is_none() {
                    request.cdn.as_mut().unwrap().ingestion_info = Some(Default::default());
                }
            }
            
            fn request_cdn_init(request: &mut api::LiveStream) {
                if request.cdn.is_none() {
                    request.cdn = Some(Default::default());
                }
            }
            
            fn request_content_details_init(request: &mut api::LiveStream) {
                if request.content_details.is_none() {
                    request.content_details = Some(Default::default());
                }
            }
            
            fn request_snippet_init(request: &mut api::LiveStream) {
                if request.snippet.is_none() {
                    request.snippet = Some(Default::default());
                }
            }
            
            fn request_status_init(request: &mut api::LiveStream) {
                if request.status.is_none() {
                    request.status = Some(Default::default());
                }
            }
            
            match &temp_cursor.to_string()[..] {
                "status.is-default-stream" => {
                        request_status_init(&mut request);
                        request.status.as_mut().unwrap().is_default_stream = Some(arg_from_str(value.unwrap_or("false"), err, "status.is-default-stream", "boolean"));
                    },
                "status.stream-status" => {
                        request_status_init(&mut request);
                        request.status.as_mut().unwrap().stream_status = Some(value.unwrap_or("").to_string());
                    },
                "kind" => {
                        request_status_init(&mut request);
                        request.kind = Some(value.unwrap_or("").to_string());
                    },
                "content-details.is-reusable" => {
                        request_content_details_init(&mut request);
                        request.content_details.as_mut().unwrap().is_reusable = Some(arg_from_str(value.unwrap_or("false"), err, "content-details.is-reusable", "boolean"));
                    },
                "content-details.closed-captions-ingestion-url" => {
                        request_content_details_init(&mut request);
                        request.content_details.as_mut().unwrap().closed_captions_ingestion_url = Some(value.unwrap_or("").to_string());
                    },
                "cdn.ingestion-type" => {
                        request_cdn_init(&mut request);
                        request.cdn.as_mut().unwrap().ingestion_type = Some(value.unwrap_or("").to_string());
                    },
                "cdn.ingestion-info.backup-ingestion-address" => {
                        request_cdn_ingestion_info_init(&mut request);
                        request.cdn.as_mut().unwrap().ingestion_info.as_mut().unwrap().backup_ingestion_address = Some(value.unwrap_or("").to_string());
                    },
                "cdn.ingestion-info.stream-name" => {
                        request_cdn_ingestion_info_init(&mut request);
                        request.cdn.as_mut().unwrap().ingestion_info.as_mut().unwrap().stream_name = Some(value.unwrap_or("").to_string());
                    },
                "cdn.ingestion-info.ingestion-address" => {
                        request_cdn_ingestion_info_init(&mut request);
                        request.cdn.as_mut().unwrap().ingestion_info.as_mut().unwrap().ingestion_address = Some(value.unwrap_or("").to_string());
                    },
                "cdn.format" => {
                        request_cdn_ingestion_info_init(&mut request);
                        request.cdn.as_mut().unwrap().format = Some(value.unwrap_or("").to_string());
                    },
                "snippet.channel-id" => {
                        request_snippet_init(&mut request);
                        request.snippet.as_mut().unwrap().channel_id = Some(value.unwrap_or("").to_string());
                    },
                "snippet.description" => {
                        request_snippet_init(&mut request);
                        request.snippet.as_mut().unwrap().description = Some(value.unwrap_or("").to_string());
                    },
                "snippet.published-at" => {
                        request_snippet_init(&mut request);
                        request.snippet.as_mut().unwrap().published_at = Some(value.unwrap_or("").to_string());
                    },
                "snippet.title" => {
                        request_snippet_init(&mut request);
                        request.snippet.as_mut().unwrap().title = Some(value.unwrap_or("").to_string());
                    },
                "etag" => {
                        request_snippet_init(&mut request);
                        request.etag = Some(value.unwrap_or("").to_string());
                    },
                "id" => {
                        request_snippet_init(&mut request);
                        request.id = Some(value.unwrap_or("").to_string());
                    },
                _ => {
                    let suggestion = FieldCursor::did_you_mean(key, &vec!["backup-ingestion-address", "cdn", "channel-id", "closed-captions-ingestion-url", "content-details", "description", "etag", "format", "id", "ingestion-address", "ingestion-info", "ingestion-type", "is-default-stream", "is-reusable", "kind", "published-at", "snippet", "status", "stream-name", "stream-status", "title"]);
                    err.issues.push(CLIError::Field(FieldError::Unknown(temp_cursor.to_string(), suggestion, value.map(|v| v.to_string()))));
                }
            }
        }
        let mut call = self.hub.live_streams().update(request);
        for parg in opt.values_of("v").unwrap_or(Vec::new()).iter() {
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
                                                Vec::new() + &self.gp + &["on-behalf-of-content-owner-channel", "on-behalf-of-content-owner"]
                                                            ));
                    }
                }
            }
        }
        let protocol = CallType::Standard;
        if dry_run {
            Ok(())
        } else {
            assert!(err.issues.len() == 0);
            for scope in self.opt.values_of("url").unwrap_or(Vec::new()).iter() {
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
                    let mut value = json::value::to_value(&output_schema);
                    remove_json_null_values(&mut value);
                    serde::json::to_writer_pretty(&mut ostream, &value).unwrap();
                    Ok(())
                }
            }
        }
    }

    fn _playlist_items_delete(&self, opt: &ArgMatches<'n, 'a>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        let mut call = self.hub.playlist_items().delete(opt.value_of("id").unwrap_or(""));
        for parg in opt.values_of("v").unwrap_or(Vec::new()).iter() {
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
                                                Vec::new() + &self.gp + &[]
                                                            ));
                    }
                }
            }
        }
        let protocol = CallType::Standard;
        if dry_run {
            Ok(())
        } else {
            assert!(err.issues.len() == 0);
            for scope in self.opt.values_of("url").unwrap_or(Vec::new()).iter() {
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

    fn _playlist_items_insert(&self, opt: &ArgMatches<'n, 'a>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        
        let mut request = api::PlaylistItem::default();
        let mut field_cursor = FieldCursor::default();
        for kvarg in opt.values_of("kv").unwrap_or(Vec::new()).iter() {
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
            fn request_content_details_init(request: &mut api::PlaylistItem) {
                if request.content_details.is_none() {
                    request.content_details = Some(Default::default());
                }
            }
            
            fn request_snippet_init(request: &mut api::PlaylistItem) {
                if request.snippet.is_none() {
                    request.snippet = Some(Default::default());
                }
            }
            
            fn request_snippet_resource_id_init(request: &mut api::PlaylistItem) {
                request_snippet_init(request);
                if request.snippet.as_mut().unwrap().resource_id.is_none() {
                    request.snippet.as_mut().unwrap().resource_id = Some(Default::default());
                }
            }
            
            fn request_snippet_thumbnails_default_init(request: &mut api::PlaylistItem) {
                request_snippet_thumbnails_init(request);
                if request.snippet.as_mut().unwrap().thumbnails.as_mut().unwrap().default.is_none() {
                    request.snippet.as_mut().unwrap().thumbnails.as_mut().unwrap().default = Some(Default::default());
                }
            }
            
            fn request_snippet_thumbnails_high_init(request: &mut api::PlaylistItem) {
                request_snippet_thumbnails_init(request);
                if request.snippet.as_mut().unwrap().thumbnails.as_mut().unwrap().high.is_none() {
                    request.snippet.as_mut().unwrap().thumbnails.as_mut().unwrap().high = Some(Default::default());
                }
            }
            
            fn request_snippet_thumbnails_init(request: &mut api::PlaylistItem) {
                request_snippet_init(request);
                if request.snippet.as_mut().unwrap().thumbnails.is_none() {
                    request.snippet.as_mut().unwrap().thumbnails = Some(Default::default());
                }
            }
            
            fn request_snippet_thumbnails_maxres_init(request: &mut api::PlaylistItem) {
                request_snippet_thumbnails_init(request);
                if request.snippet.as_mut().unwrap().thumbnails.as_mut().unwrap().maxres.is_none() {
                    request.snippet.as_mut().unwrap().thumbnails.as_mut().unwrap().maxres = Some(Default::default());
                }
            }
            
            fn request_snippet_thumbnails_medium_init(request: &mut api::PlaylistItem) {
                request_snippet_thumbnails_init(request);
                if request.snippet.as_mut().unwrap().thumbnails.as_mut().unwrap().medium.is_none() {
                    request.snippet.as_mut().unwrap().thumbnails.as_mut().unwrap().medium = Some(Default::default());
                }
            }
            
            fn request_snippet_thumbnails_standard_init(request: &mut api::PlaylistItem) {
                request_snippet_thumbnails_init(request);
                if request.snippet.as_mut().unwrap().thumbnails.as_mut().unwrap().standard.is_none() {
                    request.snippet.as_mut().unwrap().thumbnails.as_mut().unwrap().standard = Some(Default::default());
                }
            }
            
            fn request_status_init(request: &mut api::PlaylistItem) {
                if request.status.is_none() {
                    request.status = Some(Default::default());
                }
            }
            
            match &temp_cursor.to_string()[..] {
                "status.privacy-status" => {
                        request_status_init(&mut request);
                        request.status.as_mut().unwrap().privacy_status = Some(value.unwrap_or("").to_string());
                    },
                "kind" => {
                        request_status_init(&mut request);
                        request.kind = Some(value.unwrap_or("").to_string());
                    },
                "content-details.note" => {
                        request_content_details_init(&mut request);
                        request.content_details.as_mut().unwrap().note = Some(value.unwrap_or("").to_string());
                    },
                "content-details.start-at" => {
                        request_content_details_init(&mut request);
                        request.content_details.as_mut().unwrap().start_at = Some(value.unwrap_or("").to_string());
                    },
                "content-details.end-at" => {
                        request_content_details_init(&mut request);
                        request.content_details.as_mut().unwrap().end_at = Some(value.unwrap_or("").to_string());
                    },
                "content-details.video-id" => {
                        request_content_details_init(&mut request);
                        request.content_details.as_mut().unwrap().video_id = Some(value.unwrap_or("").to_string());
                    },
                "snippet.playlist-id" => {
                        request_snippet_init(&mut request);
                        request.snippet.as_mut().unwrap().playlist_id = Some(value.unwrap_or("").to_string());
                    },
                "snippet.description" => {
                        request_snippet_init(&mut request);
                        request.snippet.as_mut().unwrap().description = Some(value.unwrap_or("").to_string());
                    },
                "snippet.title" => {
                        request_snippet_init(&mut request);
                        request.snippet.as_mut().unwrap().title = Some(value.unwrap_or("").to_string());
                    },
                "snippet.resource-id.kind" => {
                        request_snippet_resource_id_init(&mut request);
                        request.snippet.as_mut().unwrap().resource_id.as_mut().unwrap().kind = Some(value.unwrap_or("").to_string());
                    },
                "snippet.resource-id.channel-id" => {
                        request_snippet_resource_id_init(&mut request);
                        request.snippet.as_mut().unwrap().resource_id.as_mut().unwrap().channel_id = Some(value.unwrap_or("").to_string());
                    },
                "snippet.resource-id.playlist-id" => {
                        request_snippet_resource_id_init(&mut request);
                        request.snippet.as_mut().unwrap().resource_id.as_mut().unwrap().playlist_id = Some(value.unwrap_or("").to_string());
                    },
                "snippet.resource-id.video-id" => {
                        request_snippet_resource_id_init(&mut request);
                        request.snippet.as_mut().unwrap().resource_id.as_mut().unwrap().video_id = Some(value.unwrap_or("").to_string());
                    },
                "snippet.channel-id" => {
                        request_snippet_resource_id_init(&mut request);
                        request.snippet.as_mut().unwrap().channel_id = Some(value.unwrap_or("").to_string());
                    },
                "snippet.published-at" => {
                        request_snippet_resource_id_init(&mut request);
                        request.snippet.as_mut().unwrap().published_at = Some(value.unwrap_or("").to_string());
                    },
                "snippet.channel-title" => {
                        request_snippet_resource_id_init(&mut request);
                        request.snippet.as_mut().unwrap().channel_title = Some(value.unwrap_or("").to_string());
                    },
                "snippet.position" => {
                        request_snippet_resource_id_init(&mut request);
                        request.snippet.as_mut().unwrap().position = Some(arg_from_str(value.unwrap_or("-0"), err, "snippet.position", "integer"));
                    },
                "snippet.thumbnails.default.url" => {
                        request_snippet_thumbnails_default_init(&mut request);
                        request.snippet.as_mut().unwrap().thumbnails.as_mut().unwrap().default.as_mut().unwrap().url = Some(value.unwrap_or("").to_string());
                    },
                "snippet.thumbnails.default.width" => {
                        request_snippet_thumbnails_default_init(&mut request);
                        request.snippet.as_mut().unwrap().thumbnails.as_mut().unwrap().default.as_mut().unwrap().width = Some(arg_from_str(value.unwrap_or("-0"), err, "snippet.thumbnails.default.width", "integer"));
                    },
                "snippet.thumbnails.default.height" => {
                        request_snippet_thumbnails_default_init(&mut request);
                        request.snippet.as_mut().unwrap().thumbnails.as_mut().unwrap().default.as_mut().unwrap().height = Some(arg_from_str(value.unwrap_or("-0"), err, "snippet.thumbnails.default.height", "integer"));
                    },
                "snippet.thumbnails.high.url" => {
                        request_snippet_thumbnails_high_init(&mut request);
                        request.snippet.as_mut().unwrap().thumbnails.as_mut().unwrap().high.as_mut().unwrap().url = Some(value.unwrap_or("").to_string());
                    },
                "snippet.thumbnails.high.width" => {
                        request_snippet_thumbnails_high_init(&mut request);
                        request.snippet.as_mut().unwrap().thumbnails.as_mut().unwrap().high.as_mut().unwrap().width = Some(arg_from_str(value.unwrap_or("-0"), err, "snippet.thumbnails.high.width", "integer"));
                    },
                "snippet.thumbnails.high.height" => {
                        request_snippet_thumbnails_high_init(&mut request);
                        request.snippet.as_mut().unwrap().thumbnails.as_mut().unwrap().high.as_mut().unwrap().height = Some(arg_from_str(value.unwrap_or("-0"), err, "snippet.thumbnails.high.height", "integer"));
                    },
                "snippet.thumbnails.medium.url" => {
                        request_snippet_thumbnails_medium_init(&mut request);
                        request.snippet.as_mut().unwrap().thumbnails.as_mut().unwrap().medium.as_mut().unwrap().url = Some(value.unwrap_or("").to_string());
                    },
                "snippet.thumbnails.medium.width" => {
                        request_snippet_thumbnails_medium_init(&mut request);
                        request.snippet.as_mut().unwrap().thumbnails.as_mut().unwrap().medium.as_mut().unwrap().width = Some(arg_from_str(value.unwrap_or("-0"), err, "snippet.thumbnails.medium.width", "integer"));
                    },
                "snippet.thumbnails.medium.height" => {
                        request_snippet_thumbnails_medium_init(&mut request);
                        request.snippet.as_mut().unwrap().thumbnails.as_mut().unwrap().medium.as_mut().unwrap().height = Some(arg_from_str(value.unwrap_or("-0"), err, "snippet.thumbnails.medium.height", "integer"));
                    },
                "snippet.thumbnails.maxres.url" => {
                        request_snippet_thumbnails_maxres_init(&mut request);
                        request.snippet.as_mut().unwrap().thumbnails.as_mut().unwrap().maxres.as_mut().unwrap().url = Some(value.unwrap_or("").to_string());
                    },
                "snippet.thumbnails.maxres.width" => {
                        request_snippet_thumbnails_maxres_init(&mut request);
                        request.snippet.as_mut().unwrap().thumbnails.as_mut().unwrap().maxres.as_mut().unwrap().width = Some(arg_from_str(value.unwrap_or("-0"), err, "snippet.thumbnails.maxres.width", "integer"));
                    },
                "snippet.thumbnails.maxres.height" => {
                        request_snippet_thumbnails_maxres_init(&mut request);
                        request.snippet.as_mut().unwrap().thumbnails.as_mut().unwrap().maxres.as_mut().unwrap().height = Some(arg_from_str(value.unwrap_or("-0"), err, "snippet.thumbnails.maxres.height", "integer"));
                    },
                "snippet.thumbnails.standard.url" => {
                        request_snippet_thumbnails_standard_init(&mut request);
                        request.snippet.as_mut().unwrap().thumbnails.as_mut().unwrap().standard.as_mut().unwrap().url = Some(value.unwrap_or("").to_string());
                    },
                "snippet.thumbnails.standard.width" => {
                        request_snippet_thumbnails_standard_init(&mut request);
                        request.snippet.as_mut().unwrap().thumbnails.as_mut().unwrap().standard.as_mut().unwrap().width = Some(arg_from_str(value.unwrap_or("-0"), err, "snippet.thumbnails.standard.width", "integer"));
                    },
                "snippet.thumbnails.standard.height" => {
                        request_snippet_thumbnails_standard_init(&mut request);
                        request.snippet.as_mut().unwrap().thumbnails.as_mut().unwrap().standard.as_mut().unwrap().height = Some(arg_from_str(value.unwrap_or("-0"), err, "snippet.thumbnails.standard.height", "integer"));
                    },
                "etag" => {
                        request_snippet_init(&mut request);
                        request.etag = Some(value.unwrap_or("").to_string());
                    },
                "id" => {
                        request_snippet_init(&mut request);
                        request.id = Some(value.unwrap_or("").to_string());
                    },
                _ => {
                    let suggestion = FieldCursor::did_you_mean(key, &vec!["channel-id", "channel-title", "content-details", "default", "description", "end-at", "etag", "height", "high", "id", "kind", "maxres", "medium", "note", "playlist-id", "position", "privacy-status", "published-at", "resource-id", "snippet", "standard", "start-at", "status", "thumbnails", "title", "url", "video-id", "width"]);
                    err.issues.push(CLIError::Field(FieldError::Unknown(temp_cursor.to_string(), suggestion, value.map(|v| v.to_string()))));
                }
            }
        }
        let mut call = self.hub.playlist_items().insert(request);
        for parg in opt.values_of("v").unwrap_or(Vec::new()).iter() {
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
                                                Vec::new() + &self.gp + &["on-behalf-of-content-owner"]
                                                            ));
                    }
                }
            }
        }
        let protocol = CallType::Standard;
        if dry_run {
            Ok(())
        } else {
            assert!(err.issues.len() == 0);
            for scope in self.opt.values_of("url").unwrap_or(Vec::new()).iter() {
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
                    let mut value = json::value::to_value(&output_schema);
                    remove_json_null_values(&mut value);
                    serde::json::to_writer_pretty(&mut ostream, &value).unwrap();
                    Ok(())
                }
            }
        }
    }

    fn _playlist_items_list(&self, opt: &ArgMatches<'n, 'a>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        let mut call = self.hub.playlist_items().list(opt.value_of("part").unwrap_or(""));
        for parg in opt.values_of("v").unwrap_or(Vec::new()).iter() {
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
                                                Vec::new() + &self.gp + &["on-behalf-of-content-owner", "playlist-id", "video-id", "max-results", "page-token", "id"]
                                                            ));
                    }
                }
            }
        }
        let protocol = CallType::Standard;
        if dry_run {
            Ok(())
        } else {
            assert!(err.issues.len() == 0);
            for scope in self.opt.values_of("url").unwrap_or(Vec::new()).iter() {
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
                    let mut value = json::value::to_value(&output_schema);
                    remove_json_null_values(&mut value);
                    serde::json::to_writer_pretty(&mut ostream, &value).unwrap();
                    Ok(())
                }
            }
        }
    }

    fn _playlist_items_update(&self, opt: &ArgMatches<'n, 'a>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        
        let mut request = api::PlaylistItem::default();
        let mut field_cursor = FieldCursor::default();
        for kvarg in opt.values_of("kv").unwrap_or(Vec::new()).iter() {
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
            fn request_content_details_init(request: &mut api::PlaylistItem) {
                if request.content_details.is_none() {
                    request.content_details = Some(Default::default());
                }
            }
            
            fn request_snippet_init(request: &mut api::PlaylistItem) {
                if request.snippet.is_none() {
                    request.snippet = Some(Default::default());
                }
            }
            
            fn request_snippet_resource_id_init(request: &mut api::PlaylistItem) {
                request_snippet_init(request);
                if request.snippet.as_mut().unwrap().resource_id.is_none() {
                    request.snippet.as_mut().unwrap().resource_id = Some(Default::default());
                }
            }
            
            fn request_snippet_thumbnails_default_init(request: &mut api::PlaylistItem) {
                request_snippet_thumbnails_init(request);
                if request.snippet.as_mut().unwrap().thumbnails.as_mut().unwrap().default.is_none() {
                    request.snippet.as_mut().unwrap().thumbnails.as_mut().unwrap().default = Some(Default::default());
                }
            }
            
            fn request_snippet_thumbnails_high_init(request: &mut api::PlaylistItem) {
                request_snippet_thumbnails_init(request);
                if request.snippet.as_mut().unwrap().thumbnails.as_mut().unwrap().high.is_none() {
                    request.snippet.as_mut().unwrap().thumbnails.as_mut().unwrap().high = Some(Default::default());
                }
            }
            
            fn request_snippet_thumbnails_init(request: &mut api::PlaylistItem) {
                request_snippet_init(request);
                if request.snippet.as_mut().unwrap().thumbnails.is_none() {
                    request.snippet.as_mut().unwrap().thumbnails = Some(Default::default());
                }
            }
            
            fn request_snippet_thumbnails_maxres_init(request: &mut api::PlaylistItem) {
                request_snippet_thumbnails_init(request);
                if request.snippet.as_mut().unwrap().thumbnails.as_mut().unwrap().maxres.is_none() {
                    request.snippet.as_mut().unwrap().thumbnails.as_mut().unwrap().maxres = Some(Default::default());
                }
            }
            
            fn request_snippet_thumbnails_medium_init(request: &mut api::PlaylistItem) {
                request_snippet_thumbnails_init(request);
                if request.snippet.as_mut().unwrap().thumbnails.as_mut().unwrap().medium.is_none() {
                    request.snippet.as_mut().unwrap().thumbnails.as_mut().unwrap().medium = Some(Default::default());
                }
            }
            
            fn request_snippet_thumbnails_standard_init(request: &mut api::PlaylistItem) {
                request_snippet_thumbnails_init(request);
                if request.snippet.as_mut().unwrap().thumbnails.as_mut().unwrap().standard.is_none() {
                    request.snippet.as_mut().unwrap().thumbnails.as_mut().unwrap().standard = Some(Default::default());
                }
            }
            
            fn request_status_init(request: &mut api::PlaylistItem) {
                if request.status.is_none() {
                    request.status = Some(Default::default());
                }
            }
            
            match &temp_cursor.to_string()[..] {
                "status.privacy-status" => {
                        request_status_init(&mut request);
                        request.status.as_mut().unwrap().privacy_status = Some(value.unwrap_or("").to_string());
                    },
                "kind" => {
                        request_status_init(&mut request);
                        request.kind = Some(value.unwrap_or("").to_string());
                    },
                "content-details.note" => {
                        request_content_details_init(&mut request);
                        request.content_details.as_mut().unwrap().note = Some(value.unwrap_or("").to_string());
                    },
                "content-details.start-at" => {
                        request_content_details_init(&mut request);
                        request.content_details.as_mut().unwrap().start_at = Some(value.unwrap_or("").to_string());
                    },
                "content-details.end-at" => {
                        request_content_details_init(&mut request);
                        request.content_details.as_mut().unwrap().end_at = Some(value.unwrap_or("").to_string());
                    },
                "content-details.video-id" => {
                        request_content_details_init(&mut request);
                        request.content_details.as_mut().unwrap().video_id = Some(value.unwrap_or("").to_string());
                    },
                "snippet.playlist-id" => {
                        request_snippet_init(&mut request);
                        request.snippet.as_mut().unwrap().playlist_id = Some(value.unwrap_or("").to_string());
                    },
                "snippet.description" => {
                        request_snippet_init(&mut request);
                        request.snippet.as_mut().unwrap().description = Some(value.unwrap_or("").to_string());
                    },
                "snippet.title" => {
                        request_snippet_init(&mut request);
                        request.snippet.as_mut().unwrap().title = Some(value.unwrap_or("").to_string());
                    },
                "snippet.resource-id.kind" => {
                        request_snippet_resource_id_init(&mut request);
                        request.snippet.as_mut().unwrap().resource_id.as_mut().unwrap().kind = Some(value.unwrap_or("").to_string());
                    },
                "snippet.resource-id.channel-id" => {
                        request_snippet_resource_id_init(&mut request);
                        request.snippet.as_mut().unwrap().resource_id.as_mut().unwrap().channel_id = Some(value.unwrap_or("").to_string());
                    },
                "snippet.resource-id.playlist-id" => {
                        request_snippet_resource_id_init(&mut request);
                        request.snippet.as_mut().unwrap().resource_id.as_mut().unwrap().playlist_id = Some(value.unwrap_or("").to_string());
                    },
                "snippet.resource-id.video-id" => {
                        request_snippet_resource_id_init(&mut request);
                        request.snippet.as_mut().unwrap().resource_id.as_mut().unwrap().video_id = Some(value.unwrap_or("").to_string());
                    },
                "snippet.channel-id" => {
                        request_snippet_resource_id_init(&mut request);
                        request.snippet.as_mut().unwrap().channel_id = Some(value.unwrap_or("").to_string());
                    },
                "snippet.published-at" => {
                        request_snippet_resource_id_init(&mut request);
                        request.snippet.as_mut().unwrap().published_at = Some(value.unwrap_or("").to_string());
                    },
                "snippet.channel-title" => {
                        request_snippet_resource_id_init(&mut request);
                        request.snippet.as_mut().unwrap().channel_title = Some(value.unwrap_or("").to_string());
                    },
                "snippet.position" => {
                        request_snippet_resource_id_init(&mut request);
                        request.snippet.as_mut().unwrap().position = Some(arg_from_str(value.unwrap_or("-0"), err, "snippet.position", "integer"));
                    },
                "snippet.thumbnails.default.url" => {
                        request_snippet_thumbnails_default_init(&mut request);
                        request.snippet.as_mut().unwrap().thumbnails.as_mut().unwrap().default.as_mut().unwrap().url = Some(value.unwrap_or("").to_string());
                    },
                "snippet.thumbnails.default.width" => {
                        request_snippet_thumbnails_default_init(&mut request);
                        request.snippet.as_mut().unwrap().thumbnails.as_mut().unwrap().default.as_mut().unwrap().width = Some(arg_from_str(value.unwrap_or("-0"), err, "snippet.thumbnails.default.width", "integer"));
                    },
                "snippet.thumbnails.default.height" => {
                        request_snippet_thumbnails_default_init(&mut request);
                        request.snippet.as_mut().unwrap().thumbnails.as_mut().unwrap().default.as_mut().unwrap().height = Some(arg_from_str(value.unwrap_or("-0"), err, "snippet.thumbnails.default.height", "integer"));
                    },
                "snippet.thumbnails.high.url" => {
                        request_snippet_thumbnails_high_init(&mut request);
                        request.snippet.as_mut().unwrap().thumbnails.as_mut().unwrap().high.as_mut().unwrap().url = Some(value.unwrap_or("").to_string());
                    },
                "snippet.thumbnails.high.width" => {
                        request_snippet_thumbnails_high_init(&mut request);
                        request.snippet.as_mut().unwrap().thumbnails.as_mut().unwrap().high.as_mut().unwrap().width = Some(arg_from_str(value.unwrap_or("-0"), err, "snippet.thumbnails.high.width", "integer"));
                    },
                "snippet.thumbnails.high.height" => {
                        request_snippet_thumbnails_high_init(&mut request);
                        request.snippet.as_mut().unwrap().thumbnails.as_mut().unwrap().high.as_mut().unwrap().height = Some(arg_from_str(value.unwrap_or("-0"), err, "snippet.thumbnails.high.height", "integer"));
                    },
                "snippet.thumbnails.medium.url" => {
                        request_snippet_thumbnails_medium_init(&mut request);
                        request.snippet.as_mut().unwrap().thumbnails.as_mut().unwrap().medium.as_mut().unwrap().url = Some(value.unwrap_or("").to_string());
                    },
                "snippet.thumbnails.medium.width" => {
                        request_snippet_thumbnails_medium_init(&mut request);
                        request.snippet.as_mut().unwrap().thumbnails.as_mut().unwrap().medium.as_mut().unwrap().width = Some(arg_from_str(value.unwrap_or("-0"), err, "snippet.thumbnails.medium.width", "integer"));
                    },
                "snippet.thumbnails.medium.height" => {
                        request_snippet_thumbnails_medium_init(&mut request);
                        request.snippet.as_mut().unwrap().thumbnails.as_mut().unwrap().medium.as_mut().unwrap().height = Some(arg_from_str(value.unwrap_or("-0"), err, "snippet.thumbnails.medium.height", "integer"));
                    },
                "snippet.thumbnails.maxres.url" => {
                        request_snippet_thumbnails_maxres_init(&mut request);
                        request.snippet.as_mut().unwrap().thumbnails.as_mut().unwrap().maxres.as_mut().unwrap().url = Some(value.unwrap_or("").to_string());
                    },
                "snippet.thumbnails.maxres.width" => {
                        request_snippet_thumbnails_maxres_init(&mut request);
                        request.snippet.as_mut().unwrap().thumbnails.as_mut().unwrap().maxres.as_mut().unwrap().width = Some(arg_from_str(value.unwrap_or("-0"), err, "snippet.thumbnails.maxres.width", "integer"));
                    },
                "snippet.thumbnails.maxres.height" => {
                        request_snippet_thumbnails_maxres_init(&mut request);
                        request.snippet.as_mut().unwrap().thumbnails.as_mut().unwrap().maxres.as_mut().unwrap().height = Some(arg_from_str(value.unwrap_or("-0"), err, "snippet.thumbnails.maxres.height", "integer"));
                    },
                "snippet.thumbnails.standard.url" => {
                        request_snippet_thumbnails_standard_init(&mut request);
                        request.snippet.as_mut().unwrap().thumbnails.as_mut().unwrap().standard.as_mut().unwrap().url = Some(value.unwrap_or("").to_string());
                    },
                "snippet.thumbnails.standard.width" => {
                        request_snippet_thumbnails_standard_init(&mut request);
                        request.snippet.as_mut().unwrap().thumbnails.as_mut().unwrap().standard.as_mut().unwrap().width = Some(arg_from_str(value.unwrap_or("-0"), err, "snippet.thumbnails.standard.width", "integer"));
                    },
                "snippet.thumbnails.standard.height" => {
                        request_snippet_thumbnails_standard_init(&mut request);
                        request.snippet.as_mut().unwrap().thumbnails.as_mut().unwrap().standard.as_mut().unwrap().height = Some(arg_from_str(value.unwrap_or("-0"), err, "snippet.thumbnails.standard.height", "integer"));
                    },
                "etag" => {
                        request_snippet_init(&mut request);
                        request.etag = Some(value.unwrap_or("").to_string());
                    },
                "id" => {
                        request_snippet_init(&mut request);
                        request.id = Some(value.unwrap_or("").to_string());
                    },
                _ => {
                    let suggestion = FieldCursor::did_you_mean(key, &vec!["channel-id", "channel-title", "content-details", "default", "description", "end-at", "etag", "height", "high", "id", "kind", "maxres", "medium", "note", "playlist-id", "position", "privacy-status", "published-at", "resource-id", "snippet", "standard", "start-at", "status", "thumbnails", "title", "url", "video-id", "width"]);
                    err.issues.push(CLIError::Field(FieldError::Unknown(temp_cursor.to_string(), suggestion, value.map(|v| v.to_string()))));
                }
            }
        }
        let mut call = self.hub.playlist_items().update(request);
        for parg in opt.values_of("v").unwrap_or(Vec::new()).iter() {
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
                                                Vec::new() + &self.gp + &[]
                                                            ));
                    }
                }
            }
        }
        let protocol = CallType::Standard;
        if dry_run {
            Ok(())
        } else {
            assert!(err.issues.len() == 0);
            for scope in self.opt.values_of("url").unwrap_or(Vec::new()).iter() {
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
                    let mut value = json::value::to_value(&output_schema);
                    remove_json_null_values(&mut value);
                    serde::json::to_writer_pretty(&mut ostream, &value).unwrap();
                    Ok(())
                }
            }
        }
    }

    fn _playlists_delete(&self, opt: &ArgMatches<'n, 'a>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        let mut call = self.hub.playlists().delete(opt.value_of("id").unwrap_or(""));
        for parg in opt.values_of("v").unwrap_or(Vec::new()).iter() {
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
                                                Vec::new() + &self.gp + &["on-behalf-of-content-owner"]
                                                            ));
                    }
                }
            }
        }
        let protocol = CallType::Standard;
        if dry_run {
            Ok(())
        } else {
            assert!(err.issues.len() == 0);
            for scope in self.opt.values_of("url").unwrap_or(Vec::new()).iter() {
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

    fn _playlists_insert(&self, opt: &ArgMatches<'n, 'a>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        
        let mut request = api::Playlist::default();
        let mut field_cursor = FieldCursor::default();
        for kvarg in opt.values_of("kv").unwrap_or(Vec::new()).iter() {
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
            fn request_content_details_init(request: &mut api::Playlist) {
                if request.content_details.is_none() {
                    request.content_details = Some(Default::default());
                }
            }
            
            fn request_player_init(request: &mut api::Playlist) {
                if request.player.is_none() {
                    request.player = Some(Default::default());
                }
            }
            
            fn request_snippet_init(request: &mut api::Playlist) {
                if request.snippet.is_none() {
                    request.snippet = Some(Default::default());
                }
            }
            
            fn request_snippet_localized_init(request: &mut api::Playlist) {
                request_snippet_init(request);
                if request.snippet.as_mut().unwrap().localized.is_none() {
                    request.snippet.as_mut().unwrap().localized = Some(Default::default());
                }
            }
            
            fn request_snippet_thumbnails_default_init(request: &mut api::Playlist) {
                request_snippet_thumbnails_init(request);
                if request.snippet.as_mut().unwrap().thumbnails.as_mut().unwrap().default.is_none() {
                    request.snippet.as_mut().unwrap().thumbnails.as_mut().unwrap().default = Some(Default::default());
                }
            }
            
            fn request_snippet_thumbnails_high_init(request: &mut api::Playlist) {
                request_snippet_thumbnails_init(request);
                if request.snippet.as_mut().unwrap().thumbnails.as_mut().unwrap().high.is_none() {
                    request.snippet.as_mut().unwrap().thumbnails.as_mut().unwrap().high = Some(Default::default());
                }
            }
            
            fn request_snippet_thumbnails_init(request: &mut api::Playlist) {
                request_snippet_init(request);
                if request.snippet.as_mut().unwrap().thumbnails.is_none() {
                    request.snippet.as_mut().unwrap().thumbnails = Some(Default::default());
                }
            }
            
            fn request_snippet_thumbnails_maxres_init(request: &mut api::Playlist) {
                request_snippet_thumbnails_init(request);
                if request.snippet.as_mut().unwrap().thumbnails.as_mut().unwrap().maxres.is_none() {
                    request.snippet.as_mut().unwrap().thumbnails.as_mut().unwrap().maxres = Some(Default::default());
                }
            }
            
            fn request_snippet_thumbnails_medium_init(request: &mut api::Playlist) {
                request_snippet_thumbnails_init(request);
                if request.snippet.as_mut().unwrap().thumbnails.as_mut().unwrap().medium.is_none() {
                    request.snippet.as_mut().unwrap().thumbnails.as_mut().unwrap().medium = Some(Default::default());
                }
            }
            
            fn request_snippet_thumbnails_standard_init(request: &mut api::Playlist) {
                request_snippet_thumbnails_init(request);
                if request.snippet.as_mut().unwrap().thumbnails.as_mut().unwrap().standard.is_none() {
                    request.snippet.as_mut().unwrap().thumbnails.as_mut().unwrap().standard = Some(Default::default());
                }
            }
            
            fn request_status_init(request: &mut api::Playlist) {
                if request.status.is_none() {
                    request.status = Some(Default::default());
                }
            }
            
            match &temp_cursor.to_string()[..] {
                "status.privacy-status" => {
                        request_status_init(&mut request);
                        request.status.as_mut().unwrap().privacy_status = Some(value.unwrap_or("").to_string());
                    },
                "kind" => {
                        request_status_init(&mut request);
                        request.kind = Some(value.unwrap_or("").to_string());
                    },
                "content-details.item-count" => {
                        request_content_details_init(&mut request);
                        request.content_details.as_mut().unwrap().item_count = Some(arg_from_str(value.unwrap_or("-0"), err, "content-details.item-count", "integer"));
                    },
                "snippet.description" => {
                        request_snippet_init(&mut request);
                        request.snippet.as_mut().unwrap().description = Some(value.unwrap_or("").to_string());
                    },
                "snippet.tags" => {
                        request_snippet_init(&mut request);
                        if request.snippet.as_mut().unwrap().tags.is_none() {
                           request.snippet.as_mut().unwrap().tags = Some(Default::default());
                        }
                                        request.snippet.as_mut().unwrap().tags.as_mut().unwrap().push(value.unwrap_or("").to_string());
                    },
                "snippet.channel-id" => {
                        request_snippet_init(&mut request);
                        request.snippet.as_mut().unwrap().channel_id = Some(value.unwrap_or("").to_string());
                    },
                "snippet.published-at" => {
                        request_snippet_init(&mut request);
                        request.snippet.as_mut().unwrap().published_at = Some(value.unwrap_or("").to_string());
                    },
                "snippet.channel-title" => {
                        request_snippet_init(&mut request);
                        request.snippet.as_mut().unwrap().channel_title = Some(value.unwrap_or("").to_string());
                    },
                "snippet.title" => {
                        request_snippet_init(&mut request);
                        request.snippet.as_mut().unwrap().title = Some(value.unwrap_or("").to_string());
                    },
                "snippet.default-language" => {
                        request_snippet_init(&mut request);
                        request.snippet.as_mut().unwrap().default_language = Some(value.unwrap_or("").to_string());
                    },
                "snippet.localized.description" => {
                        request_snippet_localized_init(&mut request);
                        request.snippet.as_mut().unwrap().localized.as_mut().unwrap().description = Some(value.unwrap_or("").to_string());
                    },
                "snippet.localized.title" => {
                        request_snippet_localized_init(&mut request);
                        request.snippet.as_mut().unwrap().localized.as_mut().unwrap().title = Some(value.unwrap_or("").to_string());
                    },
                "snippet.thumbnails.default.url" => {
                        request_snippet_thumbnails_default_init(&mut request);
                        request.snippet.as_mut().unwrap().thumbnails.as_mut().unwrap().default.as_mut().unwrap().url = Some(value.unwrap_or("").to_string());
                    },
                "snippet.thumbnails.default.width" => {
                        request_snippet_thumbnails_default_init(&mut request);
                        request.snippet.as_mut().unwrap().thumbnails.as_mut().unwrap().default.as_mut().unwrap().width = Some(arg_from_str(value.unwrap_or("-0"), err, "snippet.thumbnails.default.width", "integer"));
                    },
                "snippet.thumbnails.default.height" => {
                        request_snippet_thumbnails_default_init(&mut request);
                        request.snippet.as_mut().unwrap().thumbnails.as_mut().unwrap().default.as_mut().unwrap().height = Some(arg_from_str(value.unwrap_or("-0"), err, "snippet.thumbnails.default.height", "integer"));
                    },
                "snippet.thumbnails.high.url" => {
                        request_snippet_thumbnails_high_init(&mut request);
                        request.snippet.as_mut().unwrap().thumbnails.as_mut().unwrap().high.as_mut().unwrap().url = Some(value.unwrap_or("").to_string());
                    },
                "snippet.thumbnails.high.width" => {
                        request_snippet_thumbnails_high_init(&mut request);
                        request.snippet.as_mut().unwrap().thumbnails.as_mut().unwrap().high.as_mut().unwrap().width = Some(arg_from_str(value.unwrap_or("-0"), err, "snippet.thumbnails.high.width", "integer"));
                    },
                "snippet.thumbnails.high.height" => {
                        request_snippet_thumbnails_high_init(&mut request);
                        request.snippet.as_mut().unwrap().thumbnails.as_mut().unwrap().high.as_mut().unwrap().height = Some(arg_from_str(value.unwrap_or("-0"), err, "snippet.thumbnails.high.height", "integer"));
                    },
                "snippet.thumbnails.medium.url" => {
                        request_snippet_thumbnails_medium_init(&mut request);
                        request.snippet.as_mut().unwrap().thumbnails.as_mut().unwrap().medium.as_mut().unwrap().url = Some(value.unwrap_or("").to_string());
                    },
                "snippet.thumbnails.medium.width" => {
                        request_snippet_thumbnails_medium_init(&mut request);
                        request.snippet.as_mut().unwrap().thumbnails.as_mut().unwrap().medium.as_mut().unwrap().width = Some(arg_from_str(value.unwrap_or("-0"), err, "snippet.thumbnails.medium.width", "integer"));
                    },
                "snippet.thumbnails.medium.height" => {
                        request_snippet_thumbnails_medium_init(&mut request);
                        request.snippet.as_mut().unwrap().thumbnails.as_mut().unwrap().medium.as_mut().unwrap().height = Some(arg_from_str(value.unwrap_or("-0"), err, "snippet.thumbnails.medium.height", "integer"));
                    },
                "snippet.thumbnails.maxres.url" => {
                        request_snippet_thumbnails_maxres_init(&mut request);
                        request.snippet.as_mut().unwrap().thumbnails.as_mut().unwrap().maxres.as_mut().unwrap().url = Some(value.unwrap_or("").to_string());
                    },
                "snippet.thumbnails.maxres.width" => {
                        request_snippet_thumbnails_maxres_init(&mut request);
                        request.snippet.as_mut().unwrap().thumbnails.as_mut().unwrap().maxres.as_mut().unwrap().width = Some(arg_from_str(value.unwrap_or("-0"), err, "snippet.thumbnails.maxres.width", "integer"));
                    },
                "snippet.thumbnails.maxres.height" => {
                        request_snippet_thumbnails_maxres_init(&mut request);
                        request.snippet.as_mut().unwrap().thumbnails.as_mut().unwrap().maxres.as_mut().unwrap().height = Some(arg_from_str(value.unwrap_or("-0"), err, "snippet.thumbnails.maxres.height", "integer"));
                    },
                "snippet.thumbnails.standard.url" => {
                        request_snippet_thumbnails_standard_init(&mut request);
                        request.snippet.as_mut().unwrap().thumbnails.as_mut().unwrap().standard.as_mut().unwrap().url = Some(value.unwrap_or("").to_string());
                    },
                "snippet.thumbnails.standard.width" => {
                        request_snippet_thumbnails_standard_init(&mut request);
                        request.snippet.as_mut().unwrap().thumbnails.as_mut().unwrap().standard.as_mut().unwrap().width = Some(arg_from_str(value.unwrap_or("-0"), err, "snippet.thumbnails.standard.width", "integer"));
                    },
                "snippet.thumbnails.standard.height" => {
                        request_snippet_thumbnails_standard_init(&mut request);
                        request.snippet.as_mut().unwrap().thumbnails.as_mut().unwrap().standard.as_mut().unwrap().height = Some(arg_from_str(value.unwrap_or("-0"), err, "snippet.thumbnails.standard.height", "integer"));
                    },
                "player.embed-html" => {
                        request_player_init(&mut request);
                        request.player.as_mut().unwrap().embed_html = Some(value.unwrap_or("").to_string());
                    },
                "etag" => {
                        request_player_init(&mut request);
                        request.etag = Some(value.unwrap_or("").to_string());
                    },
                "id" => {
                        request_player_init(&mut request);
                        request.id = Some(value.unwrap_or("").to_string());
                    },
                _ => {
                    let suggestion = FieldCursor::did_you_mean(key, &vec!["channel-id", "channel-title", "content-details", "default", "default-language", "description", "embed-html", "etag", "height", "high", "id", "item-count", "kind", "localized", "maxres", "medium", "player", "privacy-status", "published-at", "snippet", "standard", "status", "tags", "thumbnails", "title", "url", "width"]);
                    err.issues.push(CLIError::Field(FieldError::Unknown(temp_cursor.to_string(), suggestion, value.map(|v| v.to_string()))));
                }
            }
        }
        let mut call = self.hub.playlists().insert(request);
        for parg in opt.values_of("v").unwrap_or(Vec::new()).iter() {
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
                                                Vec::new() + &self.gp + &["on-behalf-of-content-owner-channel", "on-behalf-of-content-owner"]
                                                            ));
                    }
                }
            }
        }
        let protocol = CallType::Standard;
        if dry_run {
            Ok(())
        } else {
            assert!(err.issues.len() == 0);
            for scope in self.opt.values_of("url").unwrap_or(Vec::new()).iter() {
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
                    let mut value = json::value::to_value(&output_schema);
                    remove_json_null_values(&mut value);
                    serde::json::to_writer_pretty(&mut ostream, &value).unwrap();
                    Ok(())
                }
            }
        }
    }

    fn _playlists_list(&self, opt: &ArgMatches<'n, 'a>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        let mut call = self.hub.playlists().list(opt.value_of("part").unwrap_or(""));
        for parg in opt.values_of("v").unwrap_or(Vec::new()).iter() {
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
                                                Vec::new() + &self.gp + &["on-behalf-of-content-owner", "on-behalf-of-content-owner-channel", "channel-id", "mine", "max-results", "page-token", "hl", "id"]
                                                            ));
                    }
                }
            }
        }
        let protocol = CallType::Standard;
        if dry_run {
            Ok(())
        } else {
            assert!(err.issues.len() == 0);
            for scope in self.opt.values_of("url").unwrap_or(Vec::new()).iter() {
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
                    let mut value = json::value::to_value(&output_schema);
                    remove_json_null_values(&mut value);
                    serde::json::to_writer_pretty(&mut ostream, &value).unwrap();
                    Ok(())
                }
            }
        }
    }

    fn _playlists_update(&self, opt: &ArgMatches<'n, 'a>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        
        let mut request = api::Playlist::default();
        let mut field_cursor = FieldCursor::default();
        for kvarg in opt.values_of("kv").unwrap_or(Vec::new()).iter() {
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
            fn request_content_details_init(request: &mut api::Playlist) {
                if request.content_details.is_none() {
                    request.content_details = Some(Default::default());
                }
            }
            
            fn request_player_init(request: &mut api::Playlist) {
                if request.player.is_none() {
                    request.player = Some(Default::default());
                }
            }
            
            fn request_snippet_init(request: &mut api::Playlist) {
                if request.snippet.is_none() {
                    request.snippet = Some(Default::default());
                }
            }
            
            fn request_snippet_localized_init(request: &mut api::Playlist) {
                request_snippet_init(request);
                if request.snippet.as_mut().unwrap().localized.is_none() {
                    request.snippet.as_mut().unwrap().localized = Some(Default::default());
                }
            }
            
            fn request_snippet_thumbnails_default_init(request: &mut api::Playlist) {
                request_snippet_thumbnails_init(request);
                if request.snippet.as_mut().unwrap().thumbnails.as_mut().unwrap().default.is_none() {
                    request.snippet.as_mut().unwrap().thumbnails.as_mut().unwrap().default = Some(Default::default());
                }
            }
            
            fn request_snippet_thumbnails_high_init(request: &mut api::Playlist) {
                request_snippet_thumbnails_init(request);
                if request.snippet.as_mut().unwrap().thumbnails.as_mut().unwrap().high.is_none() {
                    request.snippet.as_mut().unwrap().thumbnails.as_mut().unwrap().high = Some(Default::default());
                }
            }
            
            fn request_snippet_thumbnails_init(request: &mut api::Playlist) {
                request_snippet_init(request);
                if request.snippet.as_mut().unwrap().thumbnails.is_none() {
                    request.snippet.as_mut().unwrap().thumbnails = Some(Default::default());
                }
            }
            
            fn request_snippet_thumbnails_maxres_init(request: &mut api::Playlist) {
                request_snippet_thumbnails_init(request);
                if request.snippet.as_mut().unwrap().thumbnails.as_mut().unwrap().maxres.is_none() {
                    request.snippet.as_mut().unwrap().thumbnails.as_mut().unwrap().maxres = Some(Default::default());
                }
            }
            
            fn request_snippet_thumbnails_medium_init(request: &mut api::Playlist) {
                request_snippet_thumbnails_init(request);
                if request.snippet.as_mut().unwrap().thumbnails.as_mut().unwrap().medium.is_none() {
                    request.snippet.as_mut().unwrap().thumbnails.as_mut().unwrap().medium = Some(Default::default());
                }
            }
            
            fn request_snippet_thumbnails_standard_init(request: &mut api::Playlist) {
                request_snippet_thumbnails_init(request);
                if request.snippet.as_mut().unwrap().thumbnails.as_mut().unwrap().standard.is_none() {
                    request.snippet.as_mut().unwrap().thumbnails.as_mut().unwrap().standard = Some(Default::default());
                }
            }
            
            fn request_status_init(request: &mut api::Playlist) {
                if request.status.is_none() {
                    request.status = Some(Default::default());
                }
            }
            
            match &temp_cursor.to_string()[..] {
                "status.privacy-status" => {
                        request_status_init(&mut request);
                        request.status.as_mut().unwrap().privacy_status = Some(value.unwrap_or("").to_string());
                    },
                "kind" => {
                        request_status_init(&mut request);
                        request.kind = Some(value.unwrap_or("").to_string());
                    },
                "content-details.item-count" => {
                        request_content_details_init(&mut request);
                        request.content_details.as_mut().unwrap().item_count = Some(arg_from_str(value.unwrap_or("-0"), err, "content-details.item-count", "integer"));
                    },
                "snippet.description" => {
                        request_snippet_init(&mut request);
                        request.snippet.as_mut().unwrap().description = Some(value.unwrap_or("").to_string());
                    },
                "snippet.tags" => {
                        request_snippet_init(&mut request);
                        if request.snippet.as_mut().unwrap().tags.is_none() {
                           request.snippet.as_mut().unwrap().tags = Some(Default::default());
                        }
                                        request.snippet.as_mut().unwrap().tags.as_mut().unwrap().push(value.unwrap_or("").to_string());
                    },
                "snippet.channel-id" => {
                        request_snippet_init(&mut request);
                        request.snippet.as_mut().unwrap().channel_id = Some(value.unwrap_or("").to_string());
                    },
                "snippet.published-at" => {
                        request_snippet_init(&mut request);
                        request.snippet.as_mut().unwrap().published_at = Some(value.unwrap_or("").to_string());
                    },
                "snippet.channel-title" => {
                        request_snippet_init(&mut request);
                        request.snippet.as_mut().unwrap().channel_title = Some(value.unwrap_or("").to_string());
                    },
                "snippet.title" => {
                        request_snippet_init(&mut request);
                        request.snippet.as_mut().unwrap().title = Some(value.unwrap_or("").to_string());
                    },
                "snippet.default-language" => {
                        request_snippet_init(&mut request);
                        request.snippet.as_mut().unwrap().default_language = Some(value.unwrap_or("").to_string());
                    },
                "snippet.localized.description" => {
                        request_snippet_localized_init(&mut request);
                        request.snippet.as_mut().unwrap().localized.as_mut().unwrap().description = Some(value.unwrap_or("").to_string());
                    },
                "snippet.localized.title" => {
                        request_snippet_localized_init(&mut request);
                        request.snippet.as_mut().unwrap().localized.as_mut().unwrap().title = Some(value.unwrap_or("").to_string());
                    },
                "snippet.thumbnails.default.url" => {
                        request_snippet_thumbnails_default_init(&mut request);
                        request.snippet.as_mut().unwrap().thumbnails.as_mut().unwrap().default.as_mut().unwrap().url = Some(value.unwrap_or("").to_string());
                    },
                "snippet.thumbnails.default.width" => {
                        request_snippet_thumbnails_default_init(&mut request);
                        request.snippet.as_mut().unwrap().thumbnails.as_mut().unwrap().default.as_mut().unwrap().width = Some(arg_from_str(value.unwrap_or("-0"), err, "snippet.thumbnails.default.width", "integer"));
                    },
                "snippet.thumbnails.default.height" => {
                        request_snippet_thumbnails_default_init(&mut request);
                        request.snippet.as_mut().unwrap().thumbnails.as_mut().unwrap().default.as_mut().unwrap().height = Some(arg_from_str(value.unwrap_or("-0"), err, "snippet.thumbnails.default.height", "integer"));
                    },
                "snippet.thumbnails.high.url" => {
                        request_snippet_thumbnails_high_init(&mut request);
                        request.snippet.as_mut().unwrap().thumbnails.as_mut().unwrap().high.as_mut().unwrap().url = Some(value.unwrap_or("").to_string());
                    },
                "snippet.thumbnails.high.width" => {
                        request_snippet_thumbnails_high_init(&mut request);
                        request.snippet.as_mut().unwrap().thumbnails.as_mut().unwrap().high.as_mut().unwrap().width = Some(arg_from_str(value.unwrap_or("-0"), err, "snippet.thumbnails.high.width", "integer"));
                    },
                "snippet.thumbnails.high.height" => {
                        request_snippet_thumbnails_high_init(&mut request);
                        request.snippet.as_mut().unwrap().thumbnails.as_mut().unwrap().high.as_mut().unwrap().height = Some(arg_from_str(value.unwrap_or("-0"), err, "snippet.thumbnails.high.height", "integer"));
                    },
                "snippet.thumbnails.medium.url" => {
                        request_snippet_thumbnails_medium_init(&mut request);
                        request.snippet.as_mut().unwrap().thumbnails.as_mut().unwrap().medium.as_mut().unwrap().url = Some(value.unwrap_or("").to_string());
                    },
                "snippet.thumbnails.medium.width" => {
                        request_snippet_thumbnails_medium_init(&mut request);
                        request.snippet.as_mut().unwrap().thumbnails.as_mut().unwrap().medium.as_mut().unwrap().width = Some(arg_from_str(value.unwrap_or("-0"), err, "snippet.thumbnails.medium.width", "integer"));
                    },
                "snippet.thumbnails.medium.height" => {
                        request_snippet_thumbnails_medium_init(&mut request);
                        request.snippet.as_mut().unwrap().thumbnails.as_mut().unwrap().medium.as_mut().unwrap().height = Some(arg_from_str(value.unwrap_or("-0"), err, "snippet.thumbnails.medium.height", "integer"));
                    },
                "snippet.thumbnails.maxres.url" => {
                        request_snippet_thumbnails_maxres_init(&mut request);
                        request.snippet.as_mut().unwrap().thumbnails.as_mut().unwrap().maxres.as_mut().unwrap().url = Some(value.unwrap_or("").to_string());
                    },
                "snippet.thumbnails.maxres.width" => {
                        request_snippet_thumbnails_maxres_init(&mut request);
                        request.snippet.as_mut().unwrap().thumbnails.as_mut().unwrap().maxres.as_mut().unwrap().width = Some(arg_from_str(value.unwrap_or("-0"), err, "snippet.thumbnails.maxres.width", "integer"));
                    },
                "snippet.thumbnails.maxres.height" => {
                        request_snippet_thumbnails_maxres_init(&mut request);
                        request.snippet.as_mut().unwrap().thumbnails.as_mut().unwrap().maxres.as_mut().unwrap().height = Some(arg_from_str(value.unwrap_or("-0"), err, "snippet.thumbnails.maxres.height", "integer"));
                    },
                "snippet.thumbnails.standard.url" => {
                        request_snippet_thumbnails_standard_init(&mut request);
                        request.snippet.as_mut().unwrap().thumbnails.as_mut().unwrap().standard.as_mut().unwrap().url = Some(value.unwrap_or("").to_string());
                    },
                "snippet.thumbnails.standard.width" => {
                        request_snippet_thumbnails_standard_init(&mut request);
                        request.snippet.as_mut().unwrap().thumbnails.as_mut().unwrap().standard.as_mut().unwrap().width = Some(arg_from_str(value.unwrap_or("-0"), err, "snippet.thumbnails.standard.width", "integer"));
                    },
                "snippet.thumbnails.standard.height" => {
                        request_snippet_thumbnails_standard_init(&mut request);
                        request.snippet.as_mut().unwrap().thumbnails.as_mut().unwrap().standard.as_mut().unwrap().height = Some(arg_from_str(value.unwrap_or("-0"), err, "snippet.thumbnails.standard.height", "integer"));
                    },
                "player.embed-html" => {
                        request_player_init(&mut request);
                        request.player.as_mut().unwrap().embed_html = Some(value.unwrap_or("").to_string());
                    },
                "etag" => {
                        request_player_init(&mut request);
                        request.etag = Some(value.unwrap_or("").to_string());
                    },
                "id" => {
                        request_player_init(&mut request);
                        request.id = Some(value.unwrap_or("").to_string());
                    },
                _ => {
                    let suggestion = FieldCursor::did_you_mean(key, &vec!["channel-id", "channel-title", "content-details", "default", "default-language", "description", "embed-html", "etag", "height", "high", "id", "item-count", "kind", "localized", "maxres", "medium", "player", "privacy-status", "published-at", "snippet", "standard", "status", "tags", "thumbnails", "title", "url", "width"]);
                    err.issues.push(CLIError::Field(FieldError::Unknown(temp_cursor.to_string(), suggestion, value.map(|v| v.to_string()))));
                }
            }
        }
        let mut call = self.hub.playlists().update(request);
        for parg in opt.values_of("v").unwrap_or(Vec::new()).iter() {
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
                                                Vec::new() + &self.gp + &["on-behalf-of-content-owner"]
                                                            ));
                    }
                }
            }
        }
        let protocol = CallType::Standard;
        if dry_run {
            Ok(())
        } else {
            assert!(err.issues.len() == 0);
            for scope in self.opt.values_of("url").unwrap_or(Vec::new()).iter() {
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
                    let mut value = json::value::to_value(&output_schema);
                    remove_json_null_values(&mut value);
                    serde::json::to_writer_pretty(&mut ostream, &value).unwrap();
                    Ok(())
                }
            }
        }
    }

    fn _search_list(&self, opt: &ArgMatches<'n, 'a>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        let mut call = self.hub.search().list(opt.value_of("part").unwrap_or(""));
        for parg in opt.values_of("v").unwrap_or(Vec::new()).iter() {
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
                                                Vec::new() + &self.gp + &["location-radius", "channel-id", "video-syndicated", "event-type", "channel-type", "video-caption", "published-after", "on-behalf-of-content-owner", "video-category-id", "for-content-owner", "region-code", "location", "for-developer", "video-type", "type", "topic-id", "published-before", "video-dimension", "video-license", "max-results", "related-to-video-id", "video-definition", "page-token", "video-duration", "relevance-language", "for-mine", "q", "safe-search", "video-embeddable", "order"]
                                                            ));
                    }
                }
            }
        }
        let protocol = CallType::Standard;
        if dry_run {
            Ok(())
        } else {
            assert!(err.issues.len() == 0);
            for scope in self.opt.values_of("url").unwrap_or(Vec::new()).iter() {
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
                    let mut value = json::value::to_value(&output_schema);
                    remove_json_null_values(&mut value);
                    serde::json::to_writer_pretty(&mut ostream, &value).unwrap();
                    Ok(())
                }
            }
        }
    }

    fn _subscriptions_delete(&self, opt: &ArgMatches<'n, 'a>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        let mut call = self.hub.subscriptions().delete(opt.value_of("id").unwrap_or(""));
        for parg in opt.values_of("v").unwrap_or(Vec::new()).iter() {
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
                                                Vec::new() + &self.gp + &[]
                                                            ));
                    }
                }
            }
        }
        let protocol = CallType::Standard;
        if dry_run {
            Ok(())
        } else {
            assert!(err.issues.len() == 0);
            for scope in self.opt.values_of("url").unwrap_or(Vec::new()).iter() {
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

    fn _subscriptions_insert(&self, opt: &ArgMatches<'n, 'a>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        
        let mut request = api::Subscription::default();
        let mut field_cursor = FieldCursor::default();
        for kvarg in opt.values_of("kv").unwrap_or(Vec::new()).iter() {
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
            fn request_content_details_init(request: &mut api::Subscription) {
                if request.content_details.is_none() {
                    request.content_details = Some(Default::default());
                }
            }
            
            fn request_snippet_init(request: &mut api::Subscription) {
                if request.snippet.is_none() {
                    request.snippet = Some(Default::default());
                }
            }
            
            fn request_snippet_resource_id_init(request: &mut api::Subscription) {
                request_snippet_init(request);
                if request.snippet.as_mut().unwrap().resource_id.is_none() {
                    request.snippet.as_mut().unwrap().resource_id = Some(Default::default());
                }
            }
            
            fn request_snippet_thumbnails_default_init(request: &mut api::Subscription) {
                request_snippet_thumbnails_init(request);
                if request.snippet.as_mut().unwrap().thumbnails.as_mut().unwrap().default.is_none() {
                    request.snippet.as_mut().unwrap().thumbnails.as_mut().unwrap().default = Some(Default::default());
                }
            }
            
            fn request_snippet_thumbnails_high_init(request: &mut api::Subscription) {
                request_snippet_thumbnails_init(request);
                if request.snippet.as_mut().unwrap().thumbnails.as_mut().unwrap().high.is_none() {
                    request.snippet.as_mut().unwrap().thumbnails.as_mut().unwrap().high = Some(Default::default());
                }
            }
            
            fn request_snippet_thumbnails_init(request: &mut api::Subscription) {
                request_snippet_init(request);
                if request.snippet.as_mut().unwrap().thumbnails.is_none() {
                    request.snippet.as_mut().unwrap().thumbnails = Some(Default::default());
                }
            }
            
            fn request_snippet_thumbnails_maxres_init(request: &mut api::Subscription) {
                request_snippet_thumbnails_init(request);
                if request.snippet.as_mut().unwrap().thumbnails.as_mut().unwrap().maxres.is_none() {
                    request.snippet.as_mut().unwrap().thumbnails.as_mut().unwrap().maxres = Some(Default::default());
                }
            }
            
            fn request_snippet_thumbnails_medium_init(request: &mut api::Subscription) {
                request_snippet_thumbnails_init(request);
                if request.snippet.as_mut().unwrap().thumbnails.as_mut().unwrap().medium.is_none() {
                    request.snippet.as_mut().unwrap().thumbnails.as_mut().unwrap().medium = Some(Default::default());
                }
            }
            
            fn request_snippet_thumbnails_standard_init(request: &mut api::Subscription) {
                request_snippet_thumbnails_init(request);
                if request.snippet.as_mut().unwrap().thumbnails.as_mut().unwrap().standard.is_none() {
                    request.snippet.as_mut().unwrap().thumbnails.as_mut().unwrap().standard = Some(Default::default());
                }
            }
            
            fn request_subscriber_snippet_init(request: &mut api::Subscription) {
                if request.subscriber_snippet.is_none() {
                    request.subscriber_snippet = Some(Default::default());
                }
            }
            
            fn request_subscriber_snippet_thumbnails_default_init(request: &mut api::Subscription) {
                request_subscriber_snippet_thumbnails_init(request);
                if request.subscriber_snippet.as_mut().unwrap().thumbnails.as_mut().unwrap().default.is_none() {
                    request.subscriber_snippet.as_mut().unwrap().thumbnails.as_mut().unwrap().default = Some(Default::default());
                }
            }
            
            fn request_subscriber_snippet_thumbnails_high_init(request: &mut api::Subscription) {
                request_subscriber_snippet_thumbnails_init(request);
                if request.subscriber_snippet.as_mut().unwrap().thumbnails.as_mut().unwrap().high.is_none() {
                    request.subscriber_snippet.as_mut().unwrap().thumbnails.as_mut().unwrap().high = Some(Default::default());
                }
            }
            
            fn request_subscriber_snippet_thumbnails_init(request: &mut api::Subscription) {
                request_subscriber_snippet_init(request);
                if request.subscriber_snippet.as_mut().unwrap().thumbnails.is_none() {
                    request.subscriber_snippet.as_mut().unwrap().thumbnails = Some(Default::default());
                }
            }
            
            fn request_subscriber_snippet_thumbnails_maxres_init(request: &mut api::Subscription) {
                request_subscriber_snippet_thumbnails_init(request);
                if request.subscriber_snippet.as_mut().unwrap().thumbnails.as_mut().unwrap().maxres.is_none() {
                    request.subscriber_snippet.as_mut().unwrap().thumbnails.as_mut().unwrap().maxres = Some(Default::default());
                }
            }
            
            fn request_subscriber_snippet_thumbnails_medium_init(request: &mut api::Subscription) {
                request_subscriber_snippet_thumbnails_init(request);
                if request.subscriber_snippet.as_mut().unwrap().thumbnails.as_mut().unwrap().medium.is_none() {
                    request.subscriber_snippet.as_mut().unwrap().thumbnails.as_mut().unwrap().medium = Some(Default::default());
                }
            }
            
            fn request_subscriber_snippet_thumbnails_standard_init(request: &mut api::Subscription) {
                request_subscriber_snippet_thumbnails_init(request);
                if request.subscriber_snippet.as_mut().unwrap().thumbnails.as_mut().unwrap().standard.is_none() {
                    request.subscriber_snippet.as_mut().unwrap().thumbnails.as_mut().unwrap().standard = Some(Default::default());
                }
            }
            
            match &temp_cursor.to_string()[..] {
                "kind" => {
                        request.kind = Some(value.unwrap_or("").to_string());
                    },
                "content-details.new-item-count" => {
                        request_content_details_init(&mut request);
                        request.content_details.as_mut().unwrap().new_item_count = Some(arg_from_str(value.unwrap_or("-0"), err, "content-details.new-item-count", "integer"));
                    },
                "content-details.activity-type" => {
                        request_content_details_init(&mut request);
                        request.content_details.as_mut().unwrap().activity_type = Some(value.unwrap_or("").to_string());
                    },
                "content-details.total-item-count" => {
                        request_content_details_init(&mut request);
                        request.content_details.as_mut().unwrap().total_item_count = Some(arg_from_str(value.unwrap_or("-0"), err, "content-details.total-item-count", "integer"));
                    },
                "snippet.description" => {
                        request_snippet_init(&mut request);
                        request.snippet.as_mut().unwrap().description = Some(value.unwrap_or("").to_string());
                    },
                "snippet.title" => {
                        request_snippet_init(&mut request);
                        request.snippet.as_mut().unwrap().title = Some(value.unwrap_or("").to_string());
                    },
                "snippet.resource-id.kind" => {
                        request_snippet_resource_id_init(&mut request);
                        request.snippet.as_mut().unwrap().resource_id.as_mut().unwrap().kind = Some(value.unwrap_or("").to_string());
                    },
                "snippet.resource-id.channel-id" => {
                        request_snippet_resource_id_init(&mut request);
                        request.snippet.as_mut().unwrap().resource_id.as_mut().unwrap().channel_id = Some(value.unwrap_or("").to_string());
                    },
                "snippet.resource-id.playlist-id" => {
                        request_snippet_resource_id_init(&mut request);
                        request.snippet.as_mut().unwrap().resource_id.as_mut().unwrap().playlist_id = Some(value.unwrap_or("").to_string());
                    },
                "snippet.resource-id.video-id" => {
                        request_snippet_resource_id_init(&mut request);
                        request.snippet.as_mut().unwrap().resource_id.as_mut().unwrap().video_id = Some(value.unwrap_or("").to_string());
                    },
                "snippet.channel-id" => {
                        request_snippet_resource_id_init(&mut request);
                        request.snippet.as_mut().unwrap().channel_id = Some(value.unwrap_or("").to_string());
                    },
                "snippet.published-at" => {
                        request_snippet_resource_id_init(&mut request);
                        request.snippet.as_mut().unwrap().published_at = Some(value.unwrap_or("").to_string());
                    },
                "snippet.channel-title" => {
                        request_snippet_resource_id_init(&mut request);
                        request.snippet.as_mut().unwrap().channel_title = Some(value.unwrap_or("").to_string());
                    },
                "snippet.thumbnails.default.url" => {
                        request_snippet_thumbnails_default_init(&mut request);
                        request.snippet.as_mut().unwrap().thumbnails.as_mut().unwrap().default.as_mut().unwrap().url = Some(value.unwrap_or("").to_string());
                    },
                "snippet.thumbnails.default.width" => {
                        request_snippet_thumbnails_default_init(&mut request);
                        request.snippet.as_mut().unwrap().thumbnails.as_mut().unwrap().default.as_mut().unwrap().width = Some(arg_from_str(value.unwrap_or("-0"), err, "snippet.thumbnails.default.width", "integer"));
                    },
                "snippet.thumbnails.default.height" => {
                        request_snippet_thumbnails_default_init(&mut request);
                        request.snippet.as_mut().unwrap().thumbnails.as_mut().unwrap().default.as_mut().unwrap().height = Some(arg_from_str(value.unwrap_or("-0"), err, "snippet.thumbnails.default.height", "integer"));
                    },
                "snippet.thumbnails.high.url" => {
                        request_snippet_thumbnails_high_init(&mut request);
                        request.snippet.as_mut().unwrap().thumbnails.as_mut().unwrap().high.as_mut().unwrap().url = Some(value.unwrap_or("").to_string());
                    },
                "snippet.thumbnails.high.width" => {
                        request_snippet_thumbnails_high_init(&mut request);
                        request.snippet.as_mut().unwrap().thumbnails.as_mut().unwrap().high.as_mut().unwrap().width = Some(arg_from_str(value.unwrap_or("-0"), err, "snippet.thumbnails.high.width", "integer"));
                    },
                "snippet.thumbnails.high.height" => {
                        request_snippet_thumbnails_high_init(&mut request);
                        request.snippet.as_mut().unwrap().thumbnails.as_mut().unwrap().high.as_mut().unwrap().height = Some(arg_from_str(value.unwrap_or("-0"), err, "snippet.thumbnails.high.height", "integer"));
                    },
                "snippet.thumbnails.medium.url" => {
                        request_snippet_thumbnails_medium_init(&mut request);
                        request.snippet.as_mut().unwrap().thumbnails.as_mut().unwrap().medium.as_mut().unwrap().url = Some(value.unwrap_or("").to_string());
                    },
                "snippet.thumbnails.medium.width" => {
                        request_snippet_thumbnails_medium_init(&mut request);
                        request.snippet.as_mut().unwrap().thumbnails.as_mut().unwrap().medium.as_mut().unwrap().width = Some(arg_from_str(value.unwrap_or("-0"), err, "snippet.thumbnails.medium.width", "integer"));
                    },
                "snippet.thumbnails.medium.height" => {
                        request_snippet_thumbnails_medium_init(&mut request);
                        request.snippet.as_mut().unwrap().thumbnails.as_mut().unwrap().medium.as_mut().unwrap().height = Some(arg_from_str(value.unwrap_or("-0"), err, "snippet.thumbnails.medium.height", "integer"));
                    },
                "snippet.thumbnails.maxres.url" => {
                        request_snippet_thumbnails_maxres_init(&mut request);
                        request.snippet.as_mut().unwrap().thumbnails.as_mut().unwrap().maxres.as_mut().unwrap().url = Some(value.unwrap_or("").to_string());
                    },
                "snippet.thumbnails.maxres.width" => {
                        request_snippet_thumbnails_maxres_init(&mut request);
                        request.snippet.as_mut().unwrap().thumbnails.as_mut().unwrap().maxres.as_mut().unwrap().width = Some(arg_from_str(value.unwrap_or("-0"), err, "snippet.thumbnails.maxres.width", "integer"));
                    },
                "snippet.thumbnails.maxres.height" => {
                        request_snippet_thumbnails_maxres_init(&mut request);
                        request.snippet.as_mut().unwrap().thumbnails.as_mut().unwrap().maxres.as_mut().unwrap().height = Some(arg_from_str(value.unwrap_or("-0"), err, "snippet.thumbnails.maxres.height", "integer"));
                    },
                "snippet.thumbnails.standard.url" => {
                        request_snippet_thumbnails_standard_init(&mut request);
                        request.snippet.as_mut().unwrap().thumbnails.as_mut().unwrap().standard.as_mut().unwrap().url = Some(value.unwrap_or("").to_string());
                    },
                "snippet.thumbnails.standard.width" => {
                        request_snippet_thumbnails_standard_init(&mut request);
                        request.snippet.as_mut().unwrap().thumbnails.as_mut().unwrap().standard.as_mut().unwrap().width = Some(arg_from_str(value.unwrap_or("-0"), err, "snippet.thumbnails.standard.width", "integer"));
                    },
                "snippet.thumbnails.standard.height" => {
                        request_snippet_thumbnails_standard_init(&mut request);
                        request.snippet.as_mut().unwrap().thumbnails.as_mut().unwrap().standard.as_mut().unwrap().height = Some(arg_from_str(value.unwrap_or("-0"), err, "snippet.thumbnails.standard.height", "integer"));
                    },
                "etag" => {
                        request_snippet_init(&mut request);
                        request.etag = Some(value.unwrap_or("").to_string());
                    },
                "subscriber-snippet.title" => {
                        request_subscriber_snippet_init(&mut request);
                        request.subscriber_snippet.as_mut().unwrap().title = Some(value.unwrap_or("").to_string());
                    },
                "subscriber-snippet.channel-id" => {
                        request_subscriber_snippet_init(&mut request);
                        request.subscriber_snippet.as_mut().unwrap().channel_id = Some(value.unwrap_or("").to_string());
                    },
                "subscriber-snippet.description" => {
                        request_subscriber_snippet_init(&mut request);
                        request.subscriber_snippet.as_mut().unwrap().description = Some(value.unwrap_or("").to_string());
                    },
                "subscriber-snippet.thumbnails.default.url" => {
                        request_subscriber_snippet_thumbnails_default_init(&mut request);
                        request.subscriber_snippet.as_mut().unwrap().thumbnails.as_mut().unwrap().default.as_mut().unwrap().url = Some(value.unwrap_or("").to_string());
                    },
                "subscriber-snippet.thumbnails.default.width" => {
                        request_subscriber_snippet_thumbnails_default_init(&mut request);
                        request.subscriber_snippet.as_mut().unwrap().thumbnails.as_mut().unwrap().default.as_mut().unwrap().width = Some(arg_from_str(value.unwrap_or("-0"), err, "subscriber-snippet.thumbnails.default.width", "integer"));
                    },
                "subscriber-snippet.thumbnails.default.height" => {
                        request_subscriber_snippet_thumbnails_default_init(&mut request);
                        request.subscriber_snippet.as_mut().unwrap().thumbnails.as_mut().unwrap().default.as_mut().unwrap().height = Some(arg_from_str(value.unwrap_or("-0"), err, "subscriber-snippet.thumbnails.default.height", "integer"));
                    },
                "subscriber-snippet.thumbnails.high.url" => {
                        request_subscriber_snippet_thumbnails_high_init(&mut request);
                        request.subscriber_snippet.as_mut().unwrap().thumbnails.as_mut().unwrap().high.as_mut().unwrap().url = Some(value.unwrap_or("").to_string());
                    },
                "subscriber-snippet.thumbnails.high.width" => {
                        request_subscriber_snippet_thumbnails_high_init(&mut request);
                        request.subscriber_snippet.as_mut().unwrap().thumbnails.as_mut().unwrap().high.as_mut().unwrap().width = Some(arg_from_str(value.unwrap_or("-0"), err, "subscriber-snippet.thumbnails.high.width", "integer"));
                    },
                "subscriber-snippet.thumbnails.high.height" => {
                        request_subscriber_snippet_thumbnails_high_init(&mut request);
                        request.subscriber_snippet.as_mut().unwrap().thumbnails.as_mut().unwrap().high.as_mut().unwrap().height = Some(arg_from_str(value.unwrap_or("-0"), err, "subscriber-snippet.thumbnails.high.height", "integer"));
                    },
                "subscriber-snippet.thumbnails.medium.url" => {
                        request_subscriber_snippet_thumbnails_medium_init(&mut request);
                        request.subscriber_snippet.as_mut().unwrap().thumbnails.as_mut().unwrap().medium.as_mut().unwrap().url = Some(value.unwrap_or("").to_string());
                    },
                "subscriber-snippet.thumbnails.medium.width" => {
                        request_subscriber_snippet_thumbnails_medium_init(&mut request);
                        request.subscriber_snippet.as_mut().unwrap().thumbnails.as_mut().unwrap().medium.as_mut().unwrap().width = Some(arg_from_str(value.unwrap_or("-0"), err, "subscriber-snippet.thumbnails.medium.width", "integer"));
                    },
                "subscriber-snippet.thumbnails.medium.height" => {
                        request_subscriber_snippet_thumbnails_medium_init(&mut request);
                        request.subscriber_snippet.as_mut().unwrap().thumbnails.as_mut().unwrap().medium.as_mut().unwrap().height = Some(arg_from_str(value.unwrap_or("-0"), err, "subscriber-snippet.thumbnails.medium.height", "integer"));
                    },
                "subscriber-snippet.thumbnails.maxres.url" => {
                        request_subscriber_snippet_thumbnails_maxres_init(&mut request);
                        request.subscriber_snippet.as_mut().unwrap().thumbnails.as_mut().unwrap().maxres.as_mut().unwrap().url = Some(value.unwrap_or("").to_string());
                    },
                "subscriber-snippet.thumbnails.maxres.width" => {
                        request_subscriber_snippet_thumbnails_maxres_init(&mut request);
                        request.subscriber_snippet.as_mut().unwrap().thumbnails.as_mut().unwrap().maxres.as_mut().unwrap().width = Some(arg_from_str(value.unwrap_or("-0"), err, "subscriber-snippet.thumbnails.maxres.width", "integer"));
                    },
                "subscriber-snippet.thumbnails.maxres.height" => {
                        request_subscriber_snippet_thumbnails_maxres_init(&mut request);
                        request.subscriber_snippet.as_mut().unwrap().thumbnails.as_mut().unwrap().maxres.as_mut().unwrap().height = Some(arg_from_str(value.unwrap_or("-0"), err, "subscriber-snippet.thumbnails.maxres.height", "integer"));
                    },
                "subscriber-snippet.thumbnails.standard.url" => {
                        request_subscriber_snippet_thumbnails_standard_init(&mut request);
                        request.subscriber_snippet.as_mut().unwrap().thumbnails.as_mut().unwrap().standard.as_mut().unwrap().url = Some(value.unwrap_or("").to_string());
                    },
                "subscriber-snippet.thumbnails.standard.width" => {
                        request_subscriber_snippet_thumbnails_standard_init(&mut request);
                        request.subscriber_snippet.as_mut().unwrap().thumbnails.as_mut().unwrap().standard.as_mut().unwrap().width = Some(arg_from_str(value.unwrap_or("-0"), err, "subscriber-snippet.thumbnails.standard.width", "integer"));
                    },
                "subscriber-snippet.thumbnails.standard.height" => {
                        request_subscriber_snippet_thumbnails_standard_init(&mut request);
                        request.subscriber_snippet.as_mut().unwrap().thumbnails.as_mut().unwrap().standard.as_mut().unwrap().height = Some(arg_from_str(value.unwrap_or("-0"), err, "subscriber-snippet.thumbnails.standard.height", "integer"));
                    },
                "id" => {
                        request_subscriber_snippet_init(&mut request);
                        request.id = Some(value.unwrap_or("").to_string());
                    },
                _ => {
                    let suggestion = FieldCursor::did_you_mean(key, &vec!["activity-type", "channel-id", "channel-title", "content-details", "default", "description", "etag", "height", "high", "id", "kind", "maxres", "medium", "new-item-count", "playlist-id", "published-at", "resource-id", "snippet", "standard", "subscriber-snippet", "thumbnails", "title", "total-item-count", "url", "video-id", "width"]);
                    err.issues.push(CLIError::Field(FieldError::Unknown(temp_cursor.to_string(), suggestion, value.map(|v| v.to_string()))));
                }
            }
        }
        let mut call = self.hub.subscriptions().insert(request);
        for parg in opt.values_of("v").unwrap_or(Vec::new()).iter() {
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
                                                Vec::new() + &self.gp + &[]
                                                            ));
                    }
                }
            }
        }
        let protocol = CallType::Standard;
        if dry_run {
            Ok(())
        } else {
            assert!(err.issues.len() == 0);
            for scope in self.opt.values_of("url").unwrap_or(Vec::new()).iter() {
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
                    let mut value = json::value::to_value(&output_schema);
                    remove_json_null_values(&mut value);
                    serde::json::to_writer_pretty(&mut ostream, &value).unwrap();
                    Ok(())
                }
            }
        }
    }

    fn _subscriptions_list(&self, opt: &ArgMatches<'n, 'a>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        let mut call = self.hub.subscriptions().list(opt.value_of("part").unwrap_or(""));
        for parg in opt.values_of("v").unwrap_or(Vec::new()).iter() {
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
                                                Vec::new() + &self.gp + &["on-behalf-of-content-owner", "on-behalf-of-content-owner-channel", "channel-id", "mine", "max-results", "id", "page-token", "my-subscribers", "for-channel-id", "order"]
                                                            ));
                    }
                }
            }
        }
        let protocol = CallType::Standard;
        if dry_run {
            Ok(())
        } else {
            assert!(err.issues.len() == 0);
            for scope in self.opt.values_of("url").unwrap_or(Vec::new()).iter() {
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
                    let mut value = json::value::to_value(&output_schema);
                    remove_json_null_values(&mut value);
                    serde::json::to_writer_pretty(&mut ostream, &value).unwrap();
                    Ok(())
                }
            }
        }
    }

    fn _thumbnails_set(&self, opt: &ArgMatches<'n, 'a>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        let mut call = self.hub.thumbnails().set(opt.value_of("video-id").unwrap_or(""));
        for parg in opt.values_of("v").unwrap_or(Vec::new()).iter() {
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
                                                Vec::new() + &self.gp + &["on-behalf-of-content-owner"]
                                                            ));
                    }
                }
            }
        }
        let vals = opt.values_of("mode").unwrap();
        let protocol = calltype_from_str(vals[0], ["simple", "resumable"].iter().map(|&v| v.to_string()).collect(), err);
        let mut input_file = input_file_from_opts(vals[1], err);
        let mime_type = input_mime_from_opts(opt.value_of("mime").unwrap_or("application/octet-stream"), err);
        if dry_run {
            Ok(())
        } else {
            assert!(err.issues.len() == 0);
            for scope in self.opt.values_of("url").unwrap_or(Vec::new()).iter() {
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
                    let mut value = json::value::to_value(&output_schema);
                    remove_json_null_values(&mut value);
                    serde::json::to_writer_pretty(&mut ostream, &value).unwrap();
                    Ok(())
                }
            }
        }
    }

    fn _video_abuse_report_reasons_list(&self, opt: &ArgMatches<'n, 'a>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        let mut call = self.hub.video_abuse_report_reasons().list(opt.value_of("part").unwrap_or(""));
        for parg in opt.values_of("v").unwrap_or(Vec::new()).iter() {
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
                                                Vec::new() + &self.gp + &["hl"]
                                                            ));
                    }
                }
            }
        }
        let protocol = CallType::Standard;
        if dry_run {
            Ok(())
        } else {
            assert!(err.issues.len() == 0);
            for scope in self.opt.values_of("url").unwrap_or(Vec::new()).iter() {
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
                    let mut value = json::value::to_value(&output_schema);
                    remove_json_null_values(&mut value);
                    serde::json::to_writer_pretty(&mut ostream, &value).unwrap();
                    Ok(())
                }
            }
        }
    }

    fn _video_categories_list(&self, opt: &ArgMatches<'n, 'a>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        let mut call = self.hub.video_categories().list(opt.value_of("part").unwrap_or(""));
        for parg in opt.values_of("v").unwrap_or(Vec::new()).iter() {
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
                                                Vec::new() + &self.gp + &["region-code", "id", "hl"]
                                                            ));
                    }
                }
            }
        }
        let protocol = CallType::Standard;
        if dry_run {
            Ok(())
        } else {
            assert!(err.issues.len() == 0);
            for scope in self.opt.values_of("url").unwrap_or(Vec::new()).iter() {
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
                    let mut value = json::value::to_value(&output_schema);
                    remove_json_null_values(&mut value);
                    serde::json::to_writer_pretty(&mut ostream, &value).unwrap();
                    Ok(())
                }
            }
        }
    }

    fn _videos_delete(&self, opt: &ArgMatches<'n, 'a>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        let mut call = self.hub.videos().delete(opt.value_of("id").unwrap_or(""));
        for parg in opt.values_of("v").unwrap_or(Vec::new()).iter() {
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
                                                Vec::new() + &self.gp + &["on-behalf-of-content-owner"]
                                                            ));
                    }
                }
            }
        }
        let protocol = CallType::Standard;
        if dry_run {
            Ok(())
        } else {
            assert!(err.issues.len() == 0);
            for scope in self.opt.values_of("url").unwrap_or(Vec::new()).iter() {
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

    fn _videos_get_rating(&self, opt: &ArgMatches<'n, 'a>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        let mut call = self.hub.videos().get_rating(opt.value_of("id").unwrap_or(""));
        for parg in opt.values_of("v").unwrap_or(Vec::new()).iter() {
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
                                                Vec::new() + &self.gp + &["on-behalf-of-content-owner"]
                                                            ));
                    }
                }
            }
        }
        let protocol = CallType::Standard;
        if dry_run {
            Ok(())
        } else {
            assert!(err.issues.len() == 0);
            for scope in self.opt.values_of("url").unwrap_or(Vec::new()).iter() {
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
                    let mut value = json::value::to_value(&output_schema);
                    remove_json_null_values(&mut value);
                    serde::json::to_writer_pretty(&mut ostream, &value).unwrap();
                    Ok(())
                }
            }
        }
    }

    fn _videos_insert(&self, opt: &ArgMatches<'n, 'a>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        
        let mut request = api::Video::default();
        let mut field_cursor = FieldCursor::default();
        for kvarg in opt.values_of("kv").unwrap_or(Vec::new()).iter() {
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
            fn request_age_gating_init(request: &mut api::Video) {
                if request.age_gating.is_none() {
                    request.age_gating = Some(Default::default());
                }
            }
            
            fn request_content_details_content_rating_init(request: &mut api::Video) {
                request_content_details_init(request);
                if request.content_details.as_mut().unwrap().content_rating.is_none() {
                    request.content_details.as_mut().unwrap().content_rating = Some(Default::default());
                }
            }
            
            fn request_content_details_country_restriction_init(request: &mut api::Video) {
                request_content_details_init(request);
                if request.content_details.as_mut().unwrap().country_restriction.is_none() {
                    request.content_details.as_mut().unwrap().country_restriction = Some(Default::default());
                }
            }
            
            fn request_content_details_init(request: &mut api::Video) {
                if request.content_details.is_none() {
                    request.content_details = Some(Default::default());
                }
            }
            
            fn request_content_details_region_restriction_init(request: &mut api::Video) {
                request_content_details_init(request);
                if request.content_details.as_mut().unwrap().region_restriction.is_none() {
                    request.content_details.as_mut().unwrap().region_restriction = Some(Default::default());
                }
            }
            
            fn request_file_details_init(request: &mut api::Video) {
                if request.file_details.is_none() {
                    request.file_details = Some(Default::default());
                }
            }
            
            fn request_file_details_recording_location_init(request: &mut api::Video) {
                request_file_details_init(request);
                if request.file_details.as_mut().unwrap().recording_location.is_none() {
                    request.file_details.as_mut().unwrap().recording_location = Some(Default::default());
                }
            }
            
            fn request_live_streaming_details_init(request: &mut api::Video) {
                if request.live_streaming_details.is_none() {
                    request.live_streaming_details = Some(Default::default());
                }
            }
            
            fn request_monetization_details_access_init(request: &mut api::Video) {
                request_monetization_details_init(request);
                if request.monetization_details.as_mut().unwrap().access.is_none() {
                    request.monetization_details.as_mut().unwrap().access = Some(Default::default());
                }
            }
            
            fn request_monetization_details_init(request: &mut api::Video) {
                if request.monetization_details.is_none() {
                    request.monetization_details = Some(Default::default());
                }
            }
            
            fn request_player_init(request: &mut api::Video) {
                if request.player.is_none() {
                    request.player = Some(Default::default());
                }
            }
            
            fn request_processing_details_init(request: &mut api::Video) {
                if request.processing_details.is_none() {
                    request.processing_details = Some(Default::default());
                }
            }
            
            fn request_processing_details_processing_progress_init(request: &mut api::Video) {
                request_processing_details_init(request);
                if request.processing_details.as_mut().unwrap().processing_progress.is_none() {
                    request.processing_details.as_mut().unwrap().processing_progress = Some(Default::default());
                }
            }
            
            fn request_project_details_init(request: &mut api::Video) {
                if request.project_details.is_none() {
                    request.project_details = Some(Default::default());
                }
            }
            
            fn request_recording_details_init(request: &mut api::Video) {
                if request.recording_details.is_none() {
                    request.recording_details = Some(Default::default());
                }
            }
            
            fn request_recording_details_location_init(request: &mut api::Video) {
                request_recording_details_init(request);
                if request.recording_details.as_mut().unwrap().location.is_none() {
                    request.recording_details.as_mut().unwrap().location = Some(Default::default());
                }
            }
            
            fn request_snippet_init(request: &mut api::Video) {
                if request.snippet.is_none() {
                    request.snippet = Some(Default::default());
                }
            }
            
            fn request_snippet_localized_init(request: &mut api::Video) {
                request_snippet_init(request);
                if request.snippet.as_mut().unwrap().localized.is_none() {
                    request.snippet.as_mut().unwrap().localized = Some(Default::default());
                }
            }
            
            fn request_snippet_thumbnails_default_init(request: &mut api::Video) {
                request_snippet_thumbnails_init(request);
                if request.snippet.as_mut().unwrap().thumbnails.as_mut().unwrap().default.is_none() {
                    request.snippet.as_mut().unwrap().thumbnails.as_mut().unwrap().default = Some(Default::default());
                }
            }
            
            fn request_snippet_thumbnails_high_init(request: &mut api::Video) {
                request_snippet_thumbnails_init(request);
                if request.snippet.as_mut().unwrap().thumbnails.as_mut().unwrap().high.is_none() {
                    request.snippet.as_mut().unwrap().thumbnails.as_mut().unwrap().high = Some(Default::default());
                }
            }
            
            fn request_snippet_thumbnails_init(request: &mut api::Video) {
                request_snippet_init(request);
                if request.snippet.as_mut().unwrap().thumbnails.is_none() {
                    request.snippet.as_mut().unwrap().thumbnails = Some(Default::default());
                }
            }
            
            fn request_snippet_thumbnails_maxres_init(request: &mut api::Video) {
                request_snippet_thumbnails_init(request);
                if request.snippet.as_mut().unwrap().thumbnails.as_mut().unwrap().maxres.is_none() {
                    request.snippet.as_mut().unwrap().thumbnails.as_mut().unwrap().maxres = Some(Default::default());
                }
            }
            
            fn request_snippet_thumbnails_medium_init(request: &mut api::Video) {
                request_snippet_thumbnails_init(request);
                if request.snippet.as_mut().unwrap().thumbnails.as_mut().unwrap().medium.is_none() {
                    request.snippet.as_mut().unwrap().thumbnails.as_mut().unwrap().medium = Some(Default::default());
                }
            }
            
            fn request_snippet_thumbnails_standard_init(request: &mut api::Video) {
                request_snippet_thumbnails_init(request);
                if request.snippet.as_mut().unwrap().thumbnails.as_mut().unwrap().standard.is_none() {
                    request.snippet.as_mut().unwrap().thumbnails.as_mut().unwrap().standard = Some(Default::default());
                }
            }
            
            fn request_statistics_init(request: &mut api::Video) {
                if request.statistics.is_none() {
                    request.statistics = Some(Default::default());
                }
            }
            
            fn request_status_init(request: &mut api::Video) {
                if request.status.is_none() {
                    request.status = Some(Default::default());
                }
            }
            
            fn request_suggestions_init(request: &mut api::Video) {
                if request.suggestions.is_none() {
                    request.suggestions = Some(Default::default());
                }
            }
            
            fn request_topic_details_init(request: &mut api::Video) {
                if request.topic_details.is_none() {
                    request.topic_details = Some(Default::default());
                }
            }
            
            match &temp_cursor.to_string()[..] {
                "status.license" => {
                        request_status_init(&mut request);
                        request.status.as_mut().unwrap().license = Some(value.unwrap_or("").to_string());
                    },
                "status.embeddable" => {
                        request_status_init(&mut request);
                        request.status.as_mut().unwrap().embeddable = Some(arg_from_str(value.unwrap_or("false"), err, "status.embeddable", "boolean"));
                    },
                "status.privacy-status" => {
                        request_status_init(&mut request);
                        request.status.as_mut().unwrap().privacy_status = Some(value.unwrap_or("").to_string());
                    },
                "status.publish-at" => {
                        request_status_init(&mut request);
                        request.status.as_mut().unwrap().publish_at = Some(value.unwrap_or("").to_string());
                    },
                "status.public-stats-viewable" => {
                        request_status_init(&mut request);
                        request.status.as_mut().unwrap().public_stats_viewable = Some(arg_from_str(value.unwrap_or("false"), err, "status.public-stats-viewable", "boolean"));
                    },
                "status.upload-status" => {
                        request_status_init(&mut request);
                        request.status.as_mut().unwrap().upload_status = Some(value.unwrap_or("").to_string());
                    },
                "status.rejection-reason" => {
                        request_status_init(&mut request);
                        request.status.as_mut().unwrap().rejection_reason = Some(value.unwrap_or("").to_string());
                    },
                "status.failure-reason" => {
                        request_status_init(&mut request);
                        request.status.as_mut().unwrap().failure_reason = Some(value.unwrap_or("").to_string());
                    },
                "topic-details.topic-ids" => {
                        request_topic_details_init(&mut request);
                        if request.topic_details.as_mut().unwrap().topic_ids.is_none() {
                           request.topic_details.as_mut().unwrap().topic_ids = Some(Default::default());
                        }
                                        request.topic_details.as_mut().unwrap().topic_ids.as_mut().unwrap().push(value.unwrap_or("").to_string());
                    },
                "topic-details.relevant-topic-ids" => {
                        request_topic_details_init(&mut request);
                        if request.topic_details.as_mut().unwrap().relevant_topic_ids.is_none() {
                           request.topic_details.as_mut().unwrap().relevant_topic_ids = Some(Default::default());
                        }
                                        request.topic_details.as_mut().unwrap().relevant_topic_ids.as_mut().unwrap().push(value.unwrap_or("").to_string());
                    },
                "kind" => {
                        request_topic_details_init(&mut request);
                        request.kind = Some(value.unwrap_or("").to_string());
                    },
                "statistics.comment-count" => {
                        request_statistics_init(&mut request);
                        request.statistics.as_mut().unwrap().comment_count = Some(arg_from_str(value.unwrap_or("-0"), err, "statistics.comment-count", "int64"));
                    },
                "statistics.view-count" => {
                        request_statistics_init(&mut request);
                        request.statistics.as_mut().unwrap().view_count = Some(arg_from_str(value.unwrap_or("-0"), err, "statistics.view-count", "int64"));
                    },
                "statistics.favorite-count" => {
                        request_statistics_init(&mut request);
                        request.statistics.as_mut().unwrap().favorite_count = Some(arg_from_str(value.unwrap_or("-0"), err, "statistics.favorite-count", "int64"));
                    },
                "statistics.dislike-count" => {
                        request_statistics_init(&mut request);
                        request.statistics.as_mut().unwrap().dislike_count = Some(arg_from_str(value.unwrap_or("-0"), err, "statistics.dislike-count", "int64"));
                    },
                "statistics.like-count" => {
                        request_statistics_init(&mut request);
                        request.statistics.as_mut().unwrap().like_count = Some(arg_from_str(value.unwrap_or("-0"), err, "statistics.like-count", "int64"));
                    },
                "content-details.definition" => {
                        request_content_details_init(&mut request);
                        request.content_details.as_mut().unwrap().definition = Some(value.unwrap_or("").to_string());
                    },
                "content-details.country-restriction.exception" => {
                        request_content_details_country_restriction_init(&mut request);
                        if request.content_details.as_mut().unwrap().country_restriction.as_mut().unwrap().exception.is_none() {
                           request.content_details.as_mut().unwrap().country_restriction.as_mut().unwrap().exception = Some(Default::default());
                        }
                                        request.content_details.as_mut().unwrap().country_restriction.as_mut().unwrap().exception.as_mut().unwrap().push(value.unwrap_or("").to_string());
                    },
                "content-details.country-restriction.allowed" => {
                        request_content_details_country_restriction_init(&mut request);
                        request.content_details.as_mut().unwrap().country_restriction.as_mut().unwrap().allowed = Some(arg_from_str(value.unwrap_or("false"), err, "content-details.country-restriction.allowed", "boolean"));
                    },
                "content-details.content-rating.yt-rating" => {
                        request_content_details_content_rating_init(&mut request);
                        request.content_details.as_mut().unwrap().content_rating.as_mut().unwrap().yt_rating = Some(value.unwrap_or("").to_string());
                    },
                "content-details.content-rating.catvfr-rating" => {
                        request_content_details_content_rating_init(&mut request);
                        request.content_details.as_mut().unwrap().content_rating.as_mut().unwrap().catvfr_rating = Some(value.unwrap_or("").to_string());
                    },
                "content-details.content-rating.cbfc-rating" => {
                        request_content_details_content_rating_init(&mut request);
                        request.content_details.as_mut().unwrap().content_rating.as_mut().unwrap().cbfc_rating = Some(value.unwrap_or("").to_string());
                    },
                "content-details.content-rating.bfvc-rating" => {
                        request_content_details_content_rating_init(&mut request);
                        request.content_details.as_mut().unwrap().content_rating.as_mut().unwrap().bfvc_rating = Some(value.unwrap_or("").to_string());
                    },
                "content-details.content-rating.mda-rating" => {
                        request_content_details_content_rating_init(&mut request);
                        request.content_details.as_mut().unwrap().content_rating.as_mut().unwrap().mda_rating = Some(value.unwrap_or("").to_string());
                    },
                "content-details.content-rating.acb-rating" => {
                        request_content_details_content_rating_init(&mut request);
                        request.content_details.as_mut().unwrap().content_rating.as_mut().unwrap().acb_rating = Some(value.unwrap_or("").to_string());
                    },
                "content-details.content-rating.nfvcb-rating" => {
                        request_content_details_content_rating_init(&mut request);
                        request.content_details.as_mut().unwrap().content_rating.as_mut().unwrap().nfvcb_rating = Some(value.unwrap_or("").to_string());
                    },
                "content-details.content-rating.bmukk-rating" => {
                        request_content_details_content_rating_init(&mut request);
                        request.content_details.as_mut().unwrap().content_rating.as_mut().unwrap().bmukk_rating = Some(value.unwrap_or("").to_string());
                    },
                "content-details.content-rating.chfilm-rating" => {
                        request_content_details_content_rating_init(&mut request);
                        request.content_details.as_mut().unwrap().content_rating.as_mut().unwrap().chfilm_rating = Some(value.unwrap_or("").to_string());
                    },
                "content-details.content-rating.resorteviolencia-rating" => {
                        request_content_details_content_rating_init(&mut request);
                        request.content_details.as_mut().unwrap().content_rating.as_mut().unwrap().resorteviolencia_rating = Some(value.unwrap_or("").to_string());
                    },
                "content-details.content-rating.csa-rating" => {
                        request_content_details_content_rating_init(&mut request);
                        request.content_details.as_mut().unwrap().content_rating.as_mut().unwrap().csa_rating = Some(value.unwrap_or("").to_string());
                    },
                "content-details.content-rating.moctw-rating" => {
                        request_content_details_content_rating_init(&mut request);
                        request.content_details.as_mut().unwrap().content_rating.as_mut().unwrap().moctw_rating = Some(value.unwrap_or("").to_string());
                    },
                "content-details.content-rating.anatel-rating" => {
                        request_content_details_content_rating_init(&mut request);
                        request.content_details.as_mut().unwrap().content_rating.as_mut().unwrap().anatel_rating = Some(value.unwrap_or("").to_string());
                    },
                "content-details.content-rating.catv-rating" => {
                        request_content_details_content_rating_init(&mut request);
                        request.content_details.as_mut().unwrap().content_rating.as_mut().unwrap().catv_rating = Some(value.unwrap_or("").to_string());
                    },
                "content-details.content-rating.pefilm-rating" => {
                        request_content_details_content_rating_init(&mut request);
                        request.content_details.as_mut().unwrap().content_rating.as_mut().unwrap().pefilm_rating = Some(value.unwrap_or("").to_string());
                    },
                "content-details.content-rating.djctq-rating-reasons" => {
                        request_content_details_content_rating_init(&mut request);
                        if request.content_details.as_mut().unwrap().content_rating.as_mut().unwrap().djctq_rating_reasons.is_none() {
                           request.content_details.as_mut().unwrap().content_rating.as_mut().unwrap().djctq_rating_reasons = Some(Default::default());
                        }
                                        request.content_details.as_mut().unwrap().content_rating.as_mut().unwrap().djctq_rating_reasons.as_mut().unwrap().push(value.unwrap_or("").to_string());
                    },
                "content-details.content-rating.incaa-rating" => {
                        request_content_details_content_rating_init(&mut request);
                        request.content_details.as_mut().unwrap().content_rating.as_mut().unwrap().incaa_rating = Some(value.unwrap_or("").to_string());
                    },
                "content-details.content-rating.oflc-rating" => {
                        request_content_details_content_rating_init(&mut request);
                        request.content_details.as_mut().unwrap().content_rating.as_mut().unwrap().oflc_rating = Some(value.unwrap_or("").to_string());
                    },
                "content-details.content-rating.fpb-rating" => {
                        request_content_details_content_rating_init(&mut request);
                        request.content_details.as_mut().unwrap().content_rating.as_mut().unwrap().fpb_rating = Some(value.unwrap_or("").to_string());
                    },
                "content-details.content-rating.mccaa-rating" => {
                        request_content_details_content_rating_init(&mut request);
                        request.content_details.as_mut().unwrap().content_rating.as_mut().unwrap().mccaa_rating = Some(value.unwrap_or("").to_string());
                    },
                "content-details.content-rating.tvpg-rating" => {
                        request_content_details_content_rating_init(&mut request);
                        request.content_details.as_mut().unwrap().content_rating.as_mut().unwrap().tvpg_rating = Some(value.unwrap_or("").to_string());
                    },
                "content-details.content-rating.rtc-rating" => {
                        request_content_details_content_rating_init(&mut request);
                        request.content_details.as_mut().unwrap().content_rating.as_mut().unwrap().rtc_rating = Some(value.unwrap_or("").to_string());
                    },
                "content-details.content-rating.cscf-rating" => {
                        request_content_details_content_rating_init(&mut request);
                        request.content_details.as_mut().unwrap().content_rating.as_mut().unwrap().cscf_rating = Some(value.unwrap_or("").to_string());
                    },
                "content-details.content-rating.fsk-rating" => {
                        request_content_details_content_rating_init(&mut request);
                        request.content_details.as_mut().unwrap().content_rating.as_mut().unwrap().fsk_rating = Some(value.unwrap_or("").to_string());
                    },
                "content-details.content-rating.bbfc-rating" => {
                        request_content_details_content_rating_init(&mut request);
                        request.content_details.as_mut().unwrap().content_rating.as_mut().unwrap().bbfc_rating = Some(value.unwrap_or("").to_string());
                    },
                "content-details.content-rating.kmrb-rating" => {
                        request_content_details_content_rating_init(&mut request);
                        request.content_details.as_mut().unwrap().content_rating.as_mut().unwrap().kmrb_rating = Some(value.unwrap_or("").to_string());
                    },
                "content-details.content-rating.smsa-rating" => {
                        request_content_details_content_rating_init(&mut request);
                        request.content_details.as_mut().unwrap().content_rating.as_mut().unwrap().smsa_rating = Some(value.unwrap_or("").to_string());
                    },
                "content-details.content-rating.egfilm-rating" => {
                        request_content_details_content_rating_init(&mut request);
                        request.content_details.as_mut().unwrap().content_rating.as_mut().unwrap().egfilm_rating = Some(value.unwrap_or("").to_string());
                    },
                "content-details.content-rating.cicf-rating" => {
                        request_content_details_content_rating_init(&mut request);
                        request.content_details.as_mut().unwrap().content_rating.as_mut().unwrap().cicf_rating = Some(value.unwrap_or("").to_string());
                    },
                "content-details.content-rating.nbcpl-rating" => {
                        request_content_details_content_rating_init(&mut request);
                        request.content_details.as_mut().unwrap().content_rating.as_mut().unwrap().nbcpl_rating = Some(value.unwrap_or("").to_string());
                    },
                "content-details.content-rating.nbc-rating" => {
                        request_content_details_content_rating_init(&mut request);
                        request.content_details.as_mut().unwrap().content_rating.as_mut().unwrap().nbc_rating = Some(value.unwrap_or("").to_string());
                    },
                "content-details.content-rating.djctq-rating" => {
                        request_content_details_content_rating_init(&mut request);
                        request.content_details.as_mut().unwrap().content_rating.as_mut().unwrap().djctq_rating = Some(value.unwrap_or("").to_string());
                    },
                "content-details.content-rating.ifco-rating" => {
                        request_content_details_content_rating_init(&mut request);
                        request.content_details.as_mut().unwrap().content_rating.as_mut().unwrap().ifco_rating = Some(value.unwrap_or("").to_string());
                    },
                "content-details.content-rating.fco-rating" => {
                        request_content_details_content_rating_init(&mut request);
                        request.content_details.as_mut().unwrap().content_rating.as_mut().unwrap().fco_rating = Some(value.unwrap_or("").to_string());
                    },
                "content-details.content-rating.eefilm-rating" => {
                        request_content_details_content_rating_init(&mut request);
                        request.content_details.as_mut().unwrap().content_rating.as_mut().unwrap().eefilm_rating = Some(value.unwrap_or("").to_string());
                    },
                "content-details.content-rating.medietilsynet-rating" => {
                        request_content_details_content_rating_init(&mut request);
                        request.content_details.as_mut().unwrap().content_rating.as_mut().unwrap().medietilsynet_rating = Some(value.unwrap_or("").to_string());
                    },
                "content-details.content-rating.grfilm-rating" => {
                        request_content_details_content_rating_init(&mut request);
                        request.content_details.as_mut().unwrap().content_rating.as_mut().unwrap().grfilm_rating = Some(value.unwrap_or("").to_string());
                    },
                "content-details.content-rating.ccc-rating" => {
                        request_content_details_content_rating_init(&mut request);
                        request.content_details.as_mut().unwrap().content_rating.as_mut().unwrap().ccc_rating = Some(value.unwrap_or("").to_string());
                    },
                "content-details.content-rating.rte-rating" => {
                        request_content_details_content_rating_init(&mut request);
                        request.content_details.as_mut().unwrap().content_rating.as_mut().unwrap().rte_rating = Some(value.unwrap_or("").to_string());
                    },
                "content-details.content-rating.czfilm-rating" => {
                        request_content_details_content_rating_init(&mut request);
                        request.content_details.as_mut().unwrap().content_rating.as_mut().unwrap().czfilm_rating = Some(value.unwrap_or("").to_string());
                    },
                "content-details.content-rating.lsf-rating" => {
                        request_content_details_content_rating_init(&mut request);
                        request.content_details.as_mut().unwrap().content_rating.as_mut().unwrap().lsf_rating = Some(value.unwrap_or("").to_string());
                    },
                "content-details.content-rating.fmoc-rating" => {
                        request_content_details_content_rating_init(&mut request);
                        request.content_details.as_mut().unwrap().content_rating.as_mut().unwrap().fmoc_rating = Some(value.unwrap_or("").to_string());
                    },
                "content-details.content-rating.eirin-rating" => {
                        request_content_details_content_rating_init(&mut request);
                        request.content_details.as_mut().unwrap().content_rating.as_mut().unwrap().eirin_rating = Some(value.unwrap_or("").to_string());
                    },
                "content-details.content-rating.cce-rating" => {
                        request_content_details_content_rating_init(&mut request);
                        request.content_details.as_mut().unwrap().content_rating.as_mut().unwrap().cce_rating = Some(value.unwrap_or("").to_string());
                    },
                "content-details.content-rating.nkclv-rating" => {
                        request_content_details_content_rating_init(&mut request);
                        request.content_details.as_mut().unwrap().content_rating.as_mut().unwrap().nkclv_rating = Some(value.unwrap_or("").to_string());
                    },
                "content-details.content-rating.mtrcb-rating" => {
                        request_content_details_content_rating_init(&mut request);
                        request.content_details.as_mut().unwrap().content_rating.as_mut().unwrap().mtrcb_rating = Some(value.unwrap_or("").to_string());
                    },
                "content-details.content-rating.mibac-rating" => {
                        request_content_details_content_rating_init(&mut request);
                        request.content_details.as_mut().unwrap().content_rating.as_mut().unwrap().mibac_rating = Some(value.unwrap_or("").to_string());
                    },
                "content-details.content-rating.ilfilm-rating" => {
                        request_content_details_content_rating_init(&mut request);
                        request.content_details.as_mut().unwrap().content_rating.as_mut().unwrap().ilfilm_rating = Some(value.unwrap_or("").to_string());
                    },
                "content-details.content-rating.smais-rating" => {
                        request_content_details_content_rating_init(&mut request);
                        request.content_details.as_mut().unwrap().content_rating.as_mut().unwrap().smais_rating = Some(value.unwrap_or("").to_string());
                    },
                "content-details.content-rating.russia-rating" => {
                        request_content_details_content_rating_init(&mut request);
                        request.content_details.as_mut().unwrap().content_rating.as_mut().unwrap().russia_rating = Some(value.unwrap_or("").to_string());
                    },
                "content-details.content-rating.mpaa-rating" => {
                        request_content_details_content_rating_init(&mut request);
                        request.content_details.as_mut().unwrap().content_rating.as_mut().unwrap().mpaa_rating = Some(value.unwrap_or("").to_string());
                    },
                "content-details.content-rating.kfcb-rating" => {
                        request_content_details_content_rating_init(&mut request);
                        request.content_details.as_mut().unwrap().content_rating.as_mut().unwrap().kfcb_rating = Some(value.unwrap_or("").to_string());
                    },
                "content-details.content-rating.agcom-rating" => {
                        request_content_details_content_rating_init(&mut request);
                        request.content_details.as_mut().unwrap().content_rating.as_mut().unwrap().agcom_rating = Some(value.unwrap_or("").to_string());
                    },
                "content-details.content-rating.chvrs-rating" => {
                        request_content_details_content_rating_init(&mut request);
                        request.content_details.as_mut().unwrap().content_rating.as_mut().unwrap().chvrs_rating = Some(value.unwrap_or("").to_string());
                    },
                "content-details.content-rating.cna-rating" => {
                        request_content_details_content_rating_init(&mut request);
                        request.content_details.as_mut().unwrap().content_rating.as_mut().unwrap().cna_rating = Some(value.unwrap_or("").to_string());
                    },
                "content-details.content-rating.icaa-rating" => {
                        request_content_details_content_rating_init(&mut request);
                        request.content_details.as_mut().unwrap().content_rating.as_mut().unwrap().icaa_rating = Some(value.unwrap_or("").to_string());
                    },
                "content-details.content-rating.mccyp-rating" => {
                        request_content_details_content_rating_init(&mut request);
                        request.content_details.as_mut().unwrap().content_rating.as_mut().unwrap().mccyp_rating = Some(value.unwrap_or("").to_string());
                    },
                "content-details.content-rating.nfrc-rating" => {
                        request_content_details_content_rating_init(&mut request);
                        request.content_details.as_mut().unwrap().content_rating.as_mut().unwrap().nfrc_rating = Some(value.unwrap_or("").to_string());
                    },
                "content-details.content-rating.skfilm-rating" => {
                        request_content_details_content_rating_init(&mut request);
                        request.content_details.as_mut().unwrap().content_rating.as_mut().unwrap().skfilm_rating = Some(value.unwrap_or("").to_string());
                    },
                "content-details.content-rating.moc-rating" => {
                        request_content_details_content_rating_init(&mut request);
                        request.content_details.as_mut().unwrap().content_rating.as_mut().unwrap().moc_rating = Some(value.unwrap_or("").to_string());
                    },
                "content-details.content-rating.rcnof-rating" => {
                        request_content_details_content_rating_init(&mut request);
                        request.content_details.as_mut().unwrap().content_rating.as_mut().unwrap().rcnof_rating = Some(value.unwrap_or("").to_string());
                    },
                "content-details.content-rating.meku-rating" => {
                        request_content_details_content_rating_init(&mut request);
                        request.content_details.as_mut().unwrap().content_rating.as_mut().unwrap().meku_rating = Some(value.unwrap_or("").to_string());
                    },
                "content-details.content-rating.fcbm-rating" => {
                        request_content_details_content_rating_init(&mut request);
                        request.content_details.as_mut().unwrap().content_rating.as_mut().unwrap().fcbm_rating = Some(value.unwrap_or("").to_string());
                    },
                "content-details.content-rating.kijkwijzer-rating" => {
                        request_content_details_content_rating_init(&mut request);
                        request.content_details.as_mut().unwrap().content_rating.as_mut().unwrap().kijkwijzer_rating = Some(value.unwrap_or("").to_string());
                    },
                "content-details.caption" => {
                        request_content_details_content_rating_init(&mut request);
                        request.content_details.as_mut().unwrap().caption = Some(value.unwrap_or("").to_string());
                    },
                "content-details.region-restriction.blocked" => {
                        request_content_details_region_restriction_init(&mut request);
                        if request.content_details.as_mut().unwrap().region_restriction.as_mut().unwrap().blocked.is_none() {
                           request.content_details.as_mut().unwrap().region_restriction.as_mut().unwrap().blocked = Some(Default::default());
                        }
                                        request.content_details.as_mut().unwrap().region_restriction.as_mut().unwrap().blocked.as_mut().unwrap().push(value.unwrap_or("").to_string());
                    },
                "content-details.region-restriction.allowed" => {
                        request_content_details_region_restriction_init(&mut request);
                        if request.content_details.as_mut().unwrap().region_restriction.as_mut().unwrap().allowed.is_none() {
                           request.content_details.as_mut().unwrap().region_restriction.as_mut().unwrap().allowed = Some(Default::default());
                        }
                                        request.content_details.as_mut().unwrap().region_restriction.as_mut().unwrap().allowed.as_mut().unwrap().push(value.unwrap_or("").to_string());
                    },
                "content-details.duration" => {
                        request_content_details_region_restriction_init(&mut request);
                        request.content_details.as_mut().unwrap().duration = Some(value.unwrap_or("").to_string());
                    },
                "content-details.licensed-content" => {
                        request_content_details_region_restriction_init(&mut request);
                        request.content_details.as_mut().unwrap().licensed_content = Some(arg_from_str(value.unwrap_or("false"), err, "content-details.licensed-content", "boolean"));
                    },
                "content-details.dimension" => {
                        request_content_details_region_restriction_init(&mut request);
                        request.content_details.as_mut().unwrap().dimension = Some(value.unwrap_or("").to_string());
                    },
                "monetization-details.access.exception" => {
                        request_monetization_details_access_init(&mut request);
                        if request.monetization_details.as_mut().unwrap().access.as_mut().unwrap().exception.is_none() {
                           request.monetization_details.as_mut().unwrap().access.as_mut().unwrap().exception = Some(Default::default());
                        }
                                        request.monetization_details.as_mut().unwrap().access.as_mut().unwrap().exception.as_mut().unwrap().push(value.unwrap_or("").to_string());
                    },
                "monetization-details.access.allowed" => {
                        request_monetization_details_access_init(&mut request);
                        request.monetization_details.as_mut().unwrap().access.as_mut().unwrap().allowed = Some(arg_from_str(value.unwrap_or("false"), err, "monetization-details.access.allowed", "boolean"));
                    },
                "age-gating.restricted" => {
                        request_age_gating_init(&mut request);
                        request.age_gating.as_mut().unwrap().restricted = Some(arg_from_str(value.unwrap_or("false"), err, "age-gating.restricted", "boolean"));
                    },
                "age-gating.alcohol-content" => {
                        request_age_gating_init(&mut request);
                        request.age_gating.as_mut().unwrap().alcohol_content = Some(arg_from_str(value.unwrap_or("false"), err, "age-gating.alcohol-content", "boolean"));
                    },
                "age-gating.video-game-rating" => {
                        request_age_gating_init(&mut request);
                        request.age_gating.as_mut().unwrap().video_game_rating = Some(value.unwrap_or("").to_string());
                    },
                "suggestions.processing-errors" => {
                        request_suggestions_init(&mut request);
                        if request.suggestions.as_mut().unwrap().processing_errors.is_none() {
                           request.suggestions.as_mut().unwrap().processing_errors = Some(Default::default());
                        }
                                        request.suggestions.as_mut().unwrap().processing_errors.as_mut().unwrap().push(value.unwrap_or("").to_string());
                    },
                "suggestions.editor-suggestions" => {
                        request_suggestions_init(&mut request);
                        if request.suggestions.as_mut().unwrap().editor_suggestions.is_none() {
                           request.suggestions.as_mut().unwrap().editor_suggestions = Some(Default::default());
                        }
                                        request.suggestions.as_mut().unwrap().editor_suggestions.as_mut().unwrap().push(value.unwrap_or("").to_string());
                    },
                "suggestions.processing-warnings" => {
                        request_suggestions_init(&mut request);
                        if request.suggestions.as_mut().unwrap().processing_warnings.is_none() {
                           request.suggestions.as_mut().unwrap().processing_warnings = Some(Default::default());
                        }
                                        request.suggestions.as_mut().unwrap().processing_warnings.as_mut().unwrap().push(value.unwrap_or("").to_string());
                    },
                "suggestions.processing-hints" => {
                        request_suggestions_init(&mut request);
                        if request.suggestions.as_mut().unwrap().processing_hints.is_none() {
                           request.suggestions.as_mut().unwrap().processing_hints = Some(Default::default());
                        }
                                        request.suggestions.as_mut().unwrap().processing_hints.as_mut().unwrap().push(value.unwrap_or("").to_string());
                    },
                "live-streaming-details.concurrent-viewers" => {
                        request_live_streaming_details_init(&mut request);
                        request.live_streaming_details.as_mut().unwrap().concurrent_viewers = Some(value.unwrap_or("").to_string());
                    },
                "live-streaming-details.scheduled-start-time" => {
                        request_live_streaming_details_init(&mut request);
                        request.live_streaming_details.as_mut().unwrap().scheduled_start_time = Some(value.unwrap_or("").to_string());
                    },
                "live-streaming-details.scheduled-end-time" => {
                        request_live_streaming_details_init(&mut request);
                        request.live_streaming_details.as_mut().unwrap().scheduled_end_time = Some(value.unwrap_or("").to_string());
                    },
                "live-streaming-details.actual-start-time" => {
                        request_live_streaming_details_init(&mut request);
                        request.live_streaming_details.as_mut().unwrap().actual_start_time = Some(value.unwrap_or("").to_string());
                    },
                "live-streaming-details.actual-end-time" => {
                        request_live_streaming_details_init(&mut request);
                        request.live_streaming_details.as_mut().unwrap().actual_end_time = Some(value.unwrap_or("").to_string());
                    },
                "file-details.bitrate-bps" => {
                        request_file_details_init(&mut request);
                        request.file_details.as_mut().unwrap().bitrate_bps = Some(value.unwrap_or("").to_string());
                    },
                "file-details.container" => {
                        request_file_details_init(&mut request);
                        request.file_details.as_mut().unwrap().container = Some(value.unwrap_or("").to_string());
                    },
                "file-details.recording-location.latitude" => {
                        request_file_details_recording_location_init(&mut request);
                        request.file_details.as_mut().unwrap().recording_location.as_mut().unwrap().latitude = Some(arg_from_str(value.unwrap_or("0.0"), err, "file-details.recording-location.latitude", "number"));
                    },
                "file-details.recording-location.altitude" => {
                        request_file_details_recording_location_init(&mut request);
                        request.file_details.as_mut().unwrap().recording_location.as_mut().unwrap().altitude = Some(arg_from_str(value.unwrap_or("0.0"), err, "file-details.recording-location.altitude", "number"));
                    },
                "file-details.recording-location.longitude" => {
                        request_file_details_recording_location_init(&mut request);
                        request.file_details.as_mut().unwrap().recording_location.as_mut().unwrap().longitude = Some(arg_from_str(value.unwrap_or("0.0"), err, "file-details.recording-location.longitude", "number"));
                    },
                "file-details.file-type" => {
                        request_file_details_recording_location_init(&mut request);
                        request.file_details.as_mut().unwrap().file_type = Some(value.unwrap_or("").to_string());
                    },
                "file-details.creation-time" => {
                        request_file_details_recording_location_init(&mut request);
                        request.file_details.as_mut().unwrap().creation_time = Some(value.unwrap_or("").to_string());
                    },
                "file-details.duration-ms" => {
                        request_file_details_recording_location_init(&mut request);
                        request.file_details.as_mut().unwrap().duration_ms = Some(value.unwrap_or("").to_string());
                    },
                "file-details.file-name" => {
                        request_file_details_recording_location_init(&mut request);
                        request.file_details.as_mut().unwrap().file_name = Some(value.unwrap_or("").to_string());
                    },
                "file-details.file-size" => {
                        request_file_details_recording_location_init(&mut request);
                        request.file_details.as_mut().unwrap().file_size = Some(value.unwrap_or("").to_string());
                    },
                "snippet.description" => {
                        request_snippet_init(&mut request);
                        request.snippet.as_mut().unwrap().description = Some(value.unwrap_or("").to_string());
                    },
                "snippet.tags" => {
                        request_snippet_init(&mut request);
                        if request.snippet.as_mut().unwrap().tags.is_none() {
                           request.snippet.as_mut().unwrap().tags = Some(Default::default());
                        }
                                        request.snippet.as_mut().unwrap().tags.as_mut().unwrap().push(value.unwrap_or("").to_string());
                    },
                "snippet.channel-id" => {
                        request_snippet_init(&mut request);
                        request.snippet.as_mut().unwrap().channel_id = Some(value.unwrap_or("").to_string());
                    },
                "snippet.published-at" => {
                        request_snippet_init(&mut request);
                        request.snippet.as_mut().unwrap().published_at = Some(value.unwrap_or("").to_string());
                    },
                "snippet.live-broadcast-content" => {
                        request_snippet_init(&mut request);
                        request.snippet.as_mut().unwrap().live_broadcast_content = Some(value.unwrap_or("").to_string());
                    },
                "snippet.default-language" => {
                        request_snippet_init(&mut request);
                        request.snippet.as_mut().unwrap().default_language = Some(value.unwrap_or("").to_string());
                    },
                "snippet.channel-title" => {
                        request_snippet_init(&mut request);
                        request.snippet.as_mut().unwrap().channel_title = Some(value.unwrap_or("").to_string());
                    },
                "snippet.title" => {
                        request_snippet_init(&mut request);
                        request.snippet.as_mut().unwrap().title = Some(value.unwrap_or("").to_string());
                    },
                "snippet.category-id" => {
                        request_snippet_init(&mut request);
                        request.snippet.as_mut().unwrap().category_id = Some(value.unwrap_or("").to_string());
                    },
                "snippet.localized.description" => {
                        request_snippet_localized_init(&mut request);
                        request.snippet.as_mut().unwrap().localized.as_mut().unwrap().description = Some(value.unwrap_or("").to_string());
                    },
                "snippet.localized.title" => {
                        request_snippet_localized_init(&mut request);
                        request.snippet.as_mut().unwrap().localized.as_mut().unwrap().title = Some(value.unwrap_or("").to_string());
                    },
                "snippet.thumbnails.default.url" => {
                        request_snippet_thumbnails_default_init(&mut request);
                        request.snippet.as_mut().unwrap().thumbnails.as_mut().unwrap().default.as_mut().unwrap().url = Some(value.unwrap_or("").to_string());
                    },
                "snippet.thumbnails.default.width" => {
                        request_snippet_thumbnails_default_init(&mut request);
                        request.snippet.as_mut().unwrap().thumbnails.as_mut().unwrap().default.as_mut().unwrap().width = Some(arg_from_str(value.unwrap_or("-0"), err, "snippet.thumbnails.default.width", "integer"));
                    },
                "snippet.thumbnails.default.height" => {
                        request_snippet_thumbnails_default_init(&mut request);
                        request.snippet.as_mut().unwrap().thumbnails.as_mut().unwrap().default.as_mut().unwrap().height = Some(arg_from_str(value.unwrap_or("-0"), err, "snippet.thumbnails.default.height", "integer"));
                    },
                "snippet.thumbnails.high.url" => {
                        request_snippet_thumbnails_high_init(&mut request);
                        request.snippet.as_mut().unwrap().thumbnails.as_mut().unwrap().high.as_mut().unwrap().url = Some(value.unwrap_or("").to_string());
                    },
                "snippet.thumbnails.high.width" => {
                        request_snippet_thumbnails_high_init(&mut request);
                        request.snippet.as_mut().unwrap().thumbnails.as_mut().unwrap().high.as_mut().unwrap().width = Some(arg_from_str(value.unwrap_or("-0"), err, "snippet.thumbnails.high.width", "integer"));
                    },
                "snippet.thumbnails.high.height" => {
                        request_snippet_thumbnails_high_init(&mut request);
                        request.snippet.as_mut().unwrap().thumbnails.as_mut().unwrap().high.as_mut().unwrap().height = Some(arg_from_str(value.unwrap_or("-0"), err, "snippet.thumbnails.high.height", "integer"));
                    },
                "snippet.thumbnails.medium.url" => {
                        request_snippet_thumbnails_medium_init(&mut request);
                        request.snippet.as_mut().unwrap().thumbnails.as_mut().unwrap().medium.as_mut().unwrap().url = Some(value.unwrap_or("").to_string());
                    },
                "snippet.thumbnails.medium.width" => {
                        request_snippet_thumbnails_medium_init(&mut request);
                        request.snippet.as_mut().unwrap().thumbnails.as_mut().unwrap().medium.as_mut().unwrap().width = Some(arg_from_str(value.unwrap_or("-0"), err, "snippet.thumbnails.medium.width", "integer"));
                    },
                "snippet.thumbnails.medium.height" => {
                        request_snippet_thumbnails_medium_init(&mut request);
                        request.snippet.as_mut().unwrap().thumbnails.as_mut().unwrap().medium.as_mut().unwrap().height = Some(arg_from_str(value.unwrap_or("-0"), err, "snippet.thumbnails.medium.height", "integer"));
                    },
                "snippet.thumbnails.maxres.url" => {
                        request_snippet_thumbnails_maxres_init(&mut request);
                        request.snippet.as_mut().unwrap().thumbnails.as_mut().unwrap().maxres.as_mut().unwrap().url = Some(value.unwrap_or("").to_string());
                    },
                "snippet.thumbnails.maxres.width" => {
                        request_snippet_thumbnails_maxres_init(&mut request);
                        request.snippet.as_mut().unwrap().thumbnails.as_mut().unwrap().maxres.as_mut().unwrap().width = Some(arg_from_str(value.unwrap_or("-0"), err, "snippet.thumbnails.maxres.width", "integer"));
                    },
                "snippet.thumbnails.maxres.height" => {
                        request_snippet_thumbnails_maxres_init(&mut request);
                        request.snippet.as_mut().unwrap().thumbnails.as_mut().unwrap().maxres.as_mut().unwrap().height = Some(arg_from_str(value.unwrap_or("-0"), err, "snippet.thumbnails.maxres.height", "integer"));
                    },
                "snippet.thumbnails.standard.url" => {
                        request_snippet_thumbnails_standard_init(&mut request);
                        request.snippet.as_mut().unwrap().thumbnails.as_mut().unwrap().standard.as_mut().unwrap().url = Some(value.unwrap_or("").to_string());
                    },
                "snippet.thumbnails.standard.width" => {
                        request_snippet_thumbnails_standard_init(&mut request);
                        request.snippet.as_mut().unwrap().thumbnails.as_mut().unwrap().standard.as_mut().unwrap().width = Some(arg_from_str(value.unwrap_or("-0"), err, "snippet.thumbnails.standard.width", "integer"));
                    },
                "snippet.thumbnails.standard.height" => {
                        request_snippet_thumbnails_standard_init(&mut request);
                        request.snippet.as_mut().unwrap().thumbnails.as_mut().unwrap().standard.as_mut().unwrap().height = Some(arg_from_str(value.unwrap_or("-0"), err, "snippet.thumbnails.standard.height", "integer"));
                    },
                "player.embed-html" => {
                        request_player_init(&mut request);
                        request.player.as_mut().unwrap().embed_html = Some(value.unwrap_or("").to_string());
                    },
                "processing-details.file-details-availability" => {
                        request_processing_details_init(&mut request);
                        request.processing_details.as_mut().unwrap().file_details_availability = Some(value.unwrap_or("").to_string());
                    },
                "processing-details.editor-suggestions-availability" => {
                        request_processing_details_init(&mut request);
                        request.processing_details.as_mut().unwrap().editor_suggestions_availability = Some(value.unwrap_or("").to_string());
                    },
                "processing-details.processing-status" => {
                        request_processing_details_init(&mut request);
                        request.processing_details.as_mut().unwrap().processing_status = Some(value.unwrap_or("").to_string());
                    },
                "processing-details.processing-issues-availability" => {
                        request_processing_details_init(&mut request);
                        request.processing_details.as_mut().unwrap().processing_issues_availability = Some(value.unwrap_or("").to_string());
                    },
                "processing-details.processing-failure-reason" => {
                        request_processing_details_init(&mut request);
                        request.processing_details.as_mut().unwrap().processing_failure_reason = Some(value.unwrap_or("").to_string());
                    },
                "processing-details.thumbnails-availability" => {
                        request_processing_details_init(&mut request);
                        request.processing_details.as_mut().unwrap().thumbnails_availability = Some(value.unwrap_or("").to_string());
                    },
                "processing-details.processing-progress.time-left-ms" => {
                        request_processing_details_processing_progress_init(&mut request);
                        request.processing_details.as_mut().unwrap().processing_progress.as_mut().unwrap().time_left_ms = Some(value.unwrap_or("").to_string());
                    },
                "processing-details.processing-progress.parts-processed" => {
                        request_processing_details_processing_progress_init(&mut request);
                        request.processing_details.as_mut().unwrap().processing_progress.as_mut().unwrap().parts_processed = Some(value.unwrap_or("").to_string());
                    },
                "processing-details.processing-progress.parts-total" => {
                        request_processing_details_processing_progress_init(&mut request);
                        request.processing_details.as_mut().unwrap().processing_progress.as_mut().unwrap().parts_total = Some(value.unwrap_or("").to_string());
                    },
                "processing-details.tag-suggestions-availability" => {
                        request_processing_details_processing_progress_init(&mut request);
                        request.processing_details.as_mut().unwrap().tag_suggestions_availability = Some(value.unwrap_or("").to_string());
                    },
                "etag" => {
                        request_processing_details_init(&mut request);
                        request.etag = Some(value.unwrap_or("").to_string());
                    },
                "project-details.tags" => {
                        request_project_details_init(&mut request);
                        if request.project_details.as_mut().unwrap().tags.is_none() {
                           request.project_details.as_mut().unwrap().tags = Some(Default::default());
                        }
                                        request.project_details.as_mut().unwrap().tags.as_mut().unwrap().push(value.unwrap_or("").to_string());
                    },
                "recording-details.recording-date" => {
                        request_recording_details_init(&mut request);
                        request.recording_details.as_mut().unwrap().recording_date = Some(value.unwrap_or("").to_string());
                    },
                "recording-details.location-description" => {
                        request_recording_details_init(&mut request);
                        request.recording_details.as_mut().unwrap().location_description = Some(value.unwrap_or("").to_string());
                    },
                "recording-details.location.latitude" => {
                        request_recording_details_location_init(&mut request);
                        request.recording_details.as_mut().unwrap().location.as_mut().unwrap().latitude = Some(arg_from_str(value.unwrap_or("0.0"), err, "recording-details.location.latitude", "number"));
                    },
                "recording-details.location.altitude" => {
                        request_recording_details_location_init(&mut request);
                        request.recording_details.as_mut().unwrap().location.as_mut().unwrap().altitude = Some(arg_from_str(value.unwrap_or("0.0"), err, "recording-details.location.altitude", "number"));
                    },
                "recording-details.location.longitude" => {
                        request_recording_details_location_init(&mut request);
                        request.recording_details.as_mut().unwrap().location.as_mut().unwrap().longitude = Some(arg_from_str(value.unwrap_or("0.0"), err, "recording-details.location.longitude", "number"));
                    },
                "id" => {
                        request_recording_details_init(&mut request);
                        request.id = Some(value.unwrap_or("").to_string());
                    },
                _ => {
                    let suggestion = FieldCursor::did_you_mean(key, &vec!["acb-rating", "access", "actual-end-time", "actual-start-time", "agcom-rating", "age-gating", "alcohol-content", "allowed", "altitude", "anatel-rating", "bbfc-rating", "bfvc-rating", "bitrate-bps", "blocked", "bmukk-rating", "caption", "category-id", "catv-rating", "catvfr-rating", "cbfc-rating", "ccc-rating", "cce-rating", "channel-id", "channel-title", "chfilm-rating", "chvrs-rating", "cicf-rating", "cna-rating", "comment-count", "concurrent-viewers", "container", "content-details", "content-rating", "country-restriction", "creation-time", "csa-rating", "cscf-rating", "czfilm-rating", "default", "default-language", "definition", "description", "dimension", "dislike-count", "djctq-rating", "djctq-rating-reasons", "duration", "duration-ms", "editor-suggestions", "editor-suggestions-availability", "eefilm-rating", "egfilm-rating", "eirin-rating", "embed-html", "embeddable", "etag", "exception", "failure-reason", "favorite-count", "fcbm-rating", "fco-rating", "file-details", "file-details-availability", "file-name", "file-size", "file-type", "fmoc-rating", "fpb-rating", "fsk-rating", "grfilm-rating", "height", "high", "icaa-rating", "id", "ifco-rating", "ilfilm-rating", "incaa-rating", "kfcb-rating", "kijkwijzer-rating", "kind", "kmrb-rating", "latitude", "license", "licensed-content", "like-count", "live-broadcast-content", "live-streaming-details", "localized", "location", "location-description", "longitude", "lsf-rating", "maxres", "mccaa-rating", "mccyp-rating", "mda-rating", "medietilsynet-rating", "medium", "meku-rating", "mibac-rating", "moc-rating", "moctw-rating", "monetization-details", "mpaa-rating", "mtrcb-rating", "nbc-rating", "nbcpl-rating", "nfrc-rating", "nfvcb-rating", "nkclv-rating", "oflc-rating", "parts-processed", "parts-total", "pefilm-rating", "player", "privacy-status", "processing-details", "processing-errors", "processing-failure-reason", "processing-hints", "processing-issues-availability", "processing-progress", "processing-status", "processing-warnings", "project-details", "public-stats-viewable", "publish-at", "published-at", "rcnof-rating", "recording-date", "recording-details", "recording-location", "region-restriction", "rejection-reason", "relevant-topic-ids", "resorteviolencia-rating", "restricted", "rtc-rating", "rte-rating", "russia-rating", "scheduled-end-time", "scheduled-start-time", "skfilm-rating", "smais-rating", "smsa-rating", "snippet", "standard", "statistics", "status", "suggestions", "tag-suggestions-availability", "tags", "thumbnails", "thumbnails-availability", "time-left-ms", "title", "topic-details", "topic-ids", "tvpg-rating", "upload-status", "url", "video-game-rating", "view-count", "width", "yt-rating"]);
                    err.issues.push(CLIError::Field(FieldError::Unknown(temp_cursor.to_string(), suggestion, value.map(|v| v.to_string()))));
                }
            }
        }
        let mut call = self.hub.videos().insert(request);
        for parg in opt.values_of("v").unwrap_or(Vec::new()).iter() {
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
                                                Vec::new() + &self.gp + &["on-behalf-of-content-owner-channel", "on-behalf-of-content-owner", "auto-levels", "notify-subscribers", "stabilize"]
                                                            ));
                    }
                }
            }
        }
        let vals = opt.values_of("mode").unwrap();
        let protocol = calltype_from_str(vals[0], ["simple", "resumable"].iter().map(|&v| v.to_string()).collect(), err);
        let mut input_file = input_file_from_opts(vals[1], err);
        let mime_type = input_mime_from_opts(opt.value_of("mime").unwrap_or("application/octet-stream"), err);
        if dry_run {
            Ok(())
        } else {
            assert!(err.issues.len() == 0);
            for scope in self.opt.values_of("url").unwrap_or(Vec::new()).iter() {
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
                    let mut value = json::value::to_value(&output_schema);
                    remove_json_null_values(&mut value);
                    serde::json::to_writer_pretty(&mut ostream, &value).unwrap();
                    Ok(())
                }
            }
        }
    }

    fn _videos_list(&self, opt: &ArgMatches<'n, 'a>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        let mut call = self.hub.videos().list(opt.value_of("part").unwrap_or(""));
        for parg in opt.values_of("v").unwrap_or(Vec::new()).iter() {
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
                "max-results" => {
                    call = call.max_results(arg_from_str(value.unwrap_or("-0"), err, "max-results", "integer"));
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
                                                Vec::new() + &self.gp + &["on-behalf-of-content-owner", "region-code", "page-token", "locale", "chart", "max-results", "video-category-id", "hl", "my-rating", "id"]
                                                            ));
                    }
                }
            }
        }
        let protocol = CallType::Standard;
        if dry_run {
            Ok(())
        } else {
            assert!(err.issues.len() == 0);
            for scope in self.opt.values_of("url").unwrap_or(Vec::new()).iter() {
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
                    let mut value = json::value::to_value(&output_schema);
                    remove_json_null_values(&mut value);
                    serde::json::to_writer_pretty(&mut ostream, &value).unwrap();
                    Ok(())
                }
            }
        }
    }

    fn _videos_rate(&self, opt: &ArgMatches<'n, 'a>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        let mut call = self.hub.videos().rate(opt.value_of("id").unwrap_or(""), opt.value_of("rating").unwrap_or(""));
        for parg in opt.values_of("v").unwrap_or(Vec::new()).iter() {
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
                                                Vec::new() + &self.gp + &["on-behalf-of-content-owner"]
                                                            ));
                    }
                }
            }
        }
        let protocol = CallType::Standard;
        if dry_run {
            Ok(())
        } else {
            assert!(err.issues.len() == 0);
            for scope in self.opt.values_of("url").unwrap_or(Vec::new()).iter() {
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

    fn _videos_report_abuse(&self, opt: &ArgMatches<'n, 'a>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        
        let mut request = api::VideoAbuseReport::default();
        let mut field_cursor = FieldCursor::default();
        for kvarg in opt.values_of("kv").unwrap_or(Vec::new()).iter() {
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
            match &temp_cursor.to_string()[..] {
                "secondary-reason-id" => {
                        request.secondary_reason_id = Some(value.unwrap_or("").to_string());
                    },
                "reason-id" => {
                        request.reason_id = Some(value.unwrap_or("").to_string());
                    },
                "language" => {
                        request.language = Some(value.unwrap_or("").to_string());
                    },
                "comments" => {
                        request.comments = Some(value.unwrap_or("").to_string());
                    },
                "video-id" => {
                        request.video_id = Some(value.unwrap_or("").to_string());
                    },
                _ => {
                    let suggestion = FieldCursor::did_you_mean(key, &vec!["comments", "language", "reason-id", "secondary-reason-id", "video-id"]);
                    err.issues.push(CLIError::Field(FieldError::Unknown(temp_cursor.to_string(), suggestion, value.map(|v| v.to_string()))));
                }
            }
        }
        let mut call = self.hub.videos().report_abuse(request);
        for parg in opt.values_of("v").unwrap_or(Vec::new()).iter() {
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
                                                Vec::new() + &self.gp + &["on-behalf-of-content-owner"]
                                                            ));
                    }
                }
            }
        }
        let protocol = CallType::Standard;
        if dry_run {
            Ok(())
        } else {
            assert!(err.issues.len() == 0);
            for scope in self.opt.values_of("url").unwrap_or(Vec::new()).iter() {
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

    fn _videos_update(&self, opt: &ArgMatches<'n, 'a>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        
        let mut request = api::Video::default();
        let mut field_cursor = FieldCursor::default();
        for kvarg in opt.values_of("kv").unwrap_or(Vec::new()).iter() {
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
            fn request_age_gating_init(request: &mut api::Video) {
                if request.age_gating.is_none() {
                    request.age_gating = Some(Default::default());
                }
            }
            
            fn request_content_details_content_rating_init(request: &mut api::Video) {
                request_content_details_init(request);
                if request.content_details.as_mut().unwrap().content_rating.is_none() {
                    request.content_details.as_mut().unwrap().content_rating = Some(Default::default());
                }
            }
            
            fn request_content_details_country_restriction_init(request: &mut api::Video) {
                request_content_details_init(request);
                if request.content_details.as_mut().unwrap().country_restriction.is_none() {
                    request.content_details.as_mut().unwrap().country_restriction = Some(Default::default());
                }
            }
            
            fn request_content_details_init(request: &mut api::Video) {
                if request.content_details.is_none() {
                    request.content_details = Some(Default::default());
                }
            }
            
            fn request_content_details_region_restriction_init(request: &mut api::Video) {
                request_content_details_init(request);
                if request.content_details.as_mut().unwrap().region_restriction.is_none() {
                    request.content_details.as_mut().unwrap().region_restriction = Some(Default::default());
                }
            }
            
            fn request_file_details_init(request: &mut api::Video) {
                if request.file_details.is_none() {
                    request.file_details = Some(Default::default());
                }
            }
            
            fn request_file_details_recording_location_init(request: &mut api::Video) {
                request_file_details_init(request);
                if request.file_details.as_mut().unwrap().recording_location.is_none() {
                    request.file_details.as_mut().unwrap().recording_location = Some(Default::default());
                }
            }
            
            fn request_live_streaming_details_init(request: &mut api::Video) {
                if request.live_streaming_details.is_none() {
                    request.live_streaming_details = Some(Default::default());
                }
            }
            
            fn request_monetization_details_access_init(request: &mut api::Video) {
                request_monetization_details_init(request);
                if request.monetization_details.as_mut().unwrap().access.is_none() {
                    request.monetization_details.as_mut().unwrap().access = Some(Default::default());
                }
            }
            
            fn request_monetization_details_init(request: &mut api::Video) {
                if request.monetization_details.is_none() {
                    request.monetization_details = Some(Default::default());
                }
            }
            
            fn request_player_init(request: &mut api::Video) {
                if request.player.is_none() {
                    request.player = Some(Default::default());
                }
            }
            
            fn request_processing_details_init(request: &mut api::Video) {
                if request.processing_details.is_none() {
                    request.processing_details = Some(Default::default());
                }
            }
            
            fn request_processing_details_processing_progress_init(request: &mut api::Video) {
                request_processing_details_init(request);
                if request.processing_details.as_mut().unwrap().processing_progress.is_none() {
                    request.processing_details.as_mut().unwrap().processing_progress = Some(Default::default());
                }
            }
            
            fn request_project_details_init(request: &mut api::Video) {
                if request.project_details.is_none() {
                    request.project_details = Some(Default::default());
                }
            }
            
            fn request_recording_details_init(request: &mut api::Video) {
                if request.recording_details.is_none() {
                    request.recording_details = Some(Default::default());
                }
            }
            
            fn request_recording_details_location_init(request: &mut api::Video) {
                request_recording_details_init(request);
                if request.recording_details.as_mut().unwrap().location.is_none() {
                    request.recording_details.as_mut().unwrap().location = Some(Default::default());
                }
            }
            
            fn request_snippet_init(request: &mut api::Video) {
                if request.snippet.is_none() {
                    request.snippet = Some(Default::default());
                }
            }
            
            fn request_snippet_localized_init(request: &mut api::Video) {
                request_snippet_init(request);
                if request.snippet.as_mut().unwrap().localized.is_none() {
                    request.snippet.as_mut().unwrap().localized = Some(Default::default());
                }
            }
            
            fn request_snippet_thumbnails_default_init(request: &mut api::Video) {
                request_snippet_thumbnails_init(request);
                if request.snippet.as_mut().unwrap().thumbnails.as_mut().unwrap().default.is_none() {
                    request.snippet.as_mut().unwrap().thumbnails.as_mut().unwrap().default = Some(Default::default());
                }
            }
            
            fn request_snippet_thumbnails_high_init(request: &mut api::Video) {
                request_snippet_thumbnails_init(request);
                if request.snippet.as_mut().unwrap().thumbnails.as_mut().unwrap().high.is_none() {
                    request.snippet.as_mut().unwrap().thumbnails.as_mut().unwrap().high = Some(Default::default());
                }
            }
            
            fn request_snippet_thumbnails_init(request: &mut api::Video) {
                request_snippet_init(request);
                if request.snippet.as_mut().unwrap().thumbnails.is_none() {
                    request.snippet.as_mut().unwrap().thumbnails = Some(Default::default());
                }
            }
            
            fn request_snippet_thumbnails_maxres_init(request: &mut api::Video) {
                request_snippet_thumbnails_init(request);
                if request.snippet.as_mut().unwrap().thumbnails.as_mut().unwrap().maxres.is_none() {
                    request.snippet.as_mut().unwrap().thumbnails.as_mut().unwrap().maxres = Some(Default::default());
                }
            }
            
            fn request_snippet_thumbnails_medium_init(request: &mut api::Video) {
                request_snippet_thumbnails_init(request);
                if request.snippet.as_mut().unwrap().thumbnails.as_mut().unwrap().medium.is_none() {
                    request.snippet.as_mut().unwrap().thumbnails.as_mut().unwrap().medium = Some(Default::default());
                }
            }
            
            fn request_snippet_thumbnails_standard_init(request: &mut api::Video) {
                request_snippet_thumbnails_init(request);
                if request.snippet.as_mut().unwrap().thumbnails.as_mut().unwrap().standard.is_none() {
                    request.snippet.as_mut().unwrap().thumbnails.as_mut().unwrap().standard = Some(Default::default());
                }
            }
            
            fn request_statistics_init(request: &mut api::Video) {
                if request.statistics.is_none() {
                    request.statistics = Some(Default::default());
                }
            }
            
            fn request_status_init(request: &mut api::Video) {
                if request.status.is_none() {
                    request.status = Some(Default::default());
                }
            }
            
            fn request_suggestions_init(request: &mut api::Video) {
                if request.suggestions.is_none() {
                    request.suggestions = Some(Default::default());
                }
            }
            
            fn request_topic_details_init(request: &mut api::Video) {
                if request.topic_details.is_none() {
                    request.topic_details = Some(Default::default());
                }
            }
            
            match &temp_cursor.to_string()[..] {
                "status.license" => {
                        request_status_init(&mut request);
                        request.status.as_mut().unwrap().license = Some(value.unwrap_or("").to_string());
                    },
                "status.embeddable" => {
                        request_status_init(&mut request);
                        request.status.as_mut().unwrap().embeddable = Some(arg_from_str(value.unwrap_or("false"), err, "status.embeddable", "boolean"));
                    },
                "status.privacy-status" => {
                        request_status_init(&mut request);
                        request.status.as_mut().unwrap().privacy_status = Some(value.unwrap_or("").to_string());
                    },
                "status.publish-at" => {
                        request_status_init(&mut request);
                        request.status.as_mut().unwrap().publish_at = Some(value.unwrap_or("").to_string());
                    },
                "status.public-stats-viewable" => {
                        request_status_init(&mut request);
                        request.status.as_mut().unwrap().public_stats_viewable = Some(arg_from_str(value.unwrap_or("false"), err, "status.public-stats-viewable", "boolean"));
                    },
                "status.upload-status" => {
                        request_status_init(&mut request);
                        request.status.as_mut().unwrap().upload_status = Some(value.unwrap_or("").to_string());
                    },
                "status.rejection-reason" => {
                        request_status_init(&mut request);
                        request.status.as_mut().unwrap().rejection_reason = Some(value.unwrap_or("").to_string());
                    },
                "status.failure-reason" => {
                        request_status_init(&mut request);
                        request.status.as_mut().unwrap().failure_reason = Some(value.unwrap_or("").to_string());
                    },
                "topic-details.topic-ids" => {
                        request_topic_details_init(&mut request);
                        if request.topic_details.as_mut().unwrap().topic_ids.is_none() {
                           request.topic_details.as_mut().unwrap().topic_ids = Some(Default::default());
                        }
                                        request.topic_details.as_mut().unwrap().topic_ids.as_mut().unwrap().push(value.unwrap_or("").to_string());
                    },
                "topic-details.relevant-topic-ids" => {
                        request_topic_details_init(&mut request);
                        if request.topic_details.as_mut().unwrap().relevant_topic_ids.is_none() {
                           request.topic_details.as_mut().unwrap().relevant_topic_ids = Some(Default::default());
                        }
                                        request.topic_details.as_mut().unwrap().relevant_topic_ids.as_mut().unwrap().push(value.unwrap_or("").to_string());
                    },
                "kind" => {
                        request_topic_details_init(&mut request);
                        request.kind = Some(value.unwrap_or("").to_string());
                    },
                "statistics.comment-count" => {
                        request_statistics_init(&mut request);
                        request.statistics.as_mut().unwrap().comment_count = Some(arg_from_str(value.unwrap_or("-0"), err, "statistics.comment-count", "int64"));
                    },
                "statistics.view-count" => {
                        request_statistics_init(&mut request);
                        request.statistics.as_mut().unwrap().view_count = Some(arg_from_str(value.unwrap_or("-0"), err, "statistics.view-count", "int64"));
                    },
                "statistics.favorite-count" => {
                        request_statistics_init(&mut request);
                        request.statistics.as_mut().unwrap().favorite_count = Some(arg_from_str(value.unwrap_or("-0"), err, "statistics.favorite-count", "int64"));
                    },
                "statistics.dislike-count" => {
                        request_statistics_init(&mut request);
                        request.statistics.as_mut().unwrap().dislike_count = Some(arg_from_str(value.unwrap_or("-0"), err, "statistics.dislike-count", "int64"));
                    },
                "statistics.like-count" => {
                        request_statistics_init(&mut request);
                        request.statistics.as_mut().unwrap().like_count = Some(arg_from_str(value.unwrap_or("-0"), err, "statistics.like-count", "int64"));
                    },
                "content-details.definition" => {
                        request_content_details_init(&mut request);
                        request.content_details.as_mut().unwrap().definition = Some(value.unwrap_or("").to_string());
                    },
                "content-details.country-restriction.exception" => {
                        request_content_details_country_restriction_init(&mut request);
                        if request.content_details.as_mut().unwrap().country_restriction.as_mut().unwrap().exception.is_none() {
                           request.content_details.as_mut().unwrap().country_restriction.as_mut().unwrap().exception = Some(Default::default());
                        }
                                        request.content_details.as_mut().unwrap().country_restriction.as_mut().unwrap().exception.as_mut().unwrap().push(value.unwrap_or("").to_string());
                    },
                "content-details.country-restriction.allowed" => {
                        request_content_details_country_restriction_init(&mut request);
                        request.content_details.as_mut().unwrap().country_restriction.as_mut().unwrap().allowed = Some(arg_from_str(value.unwrap_or("false"), err, "content-details.country-restriction.allowed", "boolean"));
                    },
                "content-details.content-rating.yt-rating" => {
                        request_content_details_content_rating_init(&mut request);
                        request.content_details.as_mut().unwrap().content_rating.as_mut().unwrap().yt_rating = Some(value.unwrap_or("").to_string());
                    },
                "content-details.content-rating.catvfr-rating" => {
                        request_content_details_content_rating_init(&mut request);
                        request.content_details.as_mut().unwrap().content_rating.as_mut().unwrap().catvfr_rating = Some(value.unwrap_or("").to_string());
                    },
                "content-details.content-rating.cbfc-rating" => {
                        request_content_details_content_rating_init(&mut request);
                        request.content_details.as_mut().unwrap().content_rating.as_mut().unwrap().cbfc_rating = Some(value.unwrap_or("").to_string());
                    },
                "content-details.content-rating.bfvc-rating" => {
                        request_content_details_content_rating_init(&mut request);
                        request.content_details.as_mut().unwrap().content_rating.as_mut().unwrap().bfvc_rating = Some(value.unwrap_or("").to_string());
                    },
                "content-details.content-rating.mda-rating" => {
                        request_content_details_content_rating_init(&mut request);
                        request.content_details.as_mut().unwrap().content_rating.as_mut().unwrap().mda_rating = Some(value.unwrap_or("").to_string());
                    },
                "content-details.content-rating.acb-rating" => {
                        request_content_details_content_rating_init(&mut request);
                        request.content_details.as_mut().unwrap().content_rating.as_mut().unwrap().acb_rating = Some(value.unwrap_or("").to_string());
                    },
                "content-details.content-rating.nfvcb-rating" => {
                        request_content_details_content_rating_init(&mut request);
                        request.content_details.as_mut().unwrap().content_rating.as_mut().unwrap().nfvcb_rating = Some(value.unwrap_or("").to_string());
                    },
                "content-details.content-rating.bmukk-rating" => {
                        request_content_details_content_rating_init(&mut request);
                        request.content_details.as_mut().unwrap().content_rating.as_mut().unwrap().bmukk_rating = Some(value.unwrap_or("").to_string());
                    },
                "content-details.content-rating.chfilm-rating" => {
                        request_content_details_content_rating_init(&mut request);
                        request.content_details.as_mut().unwrap().content_rating.as_mut().unwrap().chfilm_rating = Some(value.unwrap_or("").to_string());
                    },
                "content-details.content-rating.resorteviolencia-rating" => {
                        request_content_details_content_rating_init(&mut request);
                        request.content_details.as_mut().unwrap().content_rating.as_mut().unwrap().resorteviolencia_rating = Some(value.unwrap_or("").to_string());
                    },
                "content-details.content-rating.csa-rating" => {
                        request_content_details_content_rating_init(&mut request);
                        request.content_details.as_mut().unwrap().content_rating.as_mut().unwrap().csa_rating = Some(value.unwrap_or("").to_string());
                    },
                "content-details.content-rating.moctw-rating" => {
                        request_content_details_content_rating_init(&mut request);
                        request.content_details.as_mut().unwrap().content_rating.as_mut().unwrap().moctw_rating = Some(value.unwrap_or("").to_string());
                    },
                "content-details.content-rating.anatel-rating" => {
                        request_content_details_content_rating_init(&mut request);
                        request.content_details.as_mut().unwrap().content_rating.as_mut().unwrap().anatel_rating = Some(value.unwrap_or("").to_string());
                    },
                "content-details.content-rating.catv-rating" => {
                        request_content_details_content_rating_init(&mut request);
                        request.content_details.as_mut().unwrap().content_rating.as_mut().unwrap().catv_rating = Some(value.unwrap_or("").to_string());
                    },
                "content-details.content-rating.pefilm-rating" => {
                        request_content_details_content_rating_init(&mut request);
                        request.content_details.as_mut().unwrap().content_rating.as_mut().unwrap().pefilm_rating = Some(value.unwrap_or("").to_string());
                    },
                "content-details.content-rating.djctq-rating-reasons" => {
                        request_content_details_content_rating_init(&mut request);
                        if request.content_details.as_mut().unwrap().content_rating.as_mut().unwrap().djctq_rating_reasons.is_none() {
                           request.content_details.as_mut().unwrap().content_rating.as_mut().unwrap().djctq_rating_reasons = Some(Default::default());
                        }
                                        request.content_details.as_mut().unwrap().content_rating.as_mut().unwrap().djctq_rating_reasons.as_mut().unwrap().push(value.unwrap_or("").to_string());
                    },
                "content-details.content-rating.incaa-rating" => {
                        request_content_details_content_rating_init(&mut request);
                        request.content_details.as_mut().unwrap().content_rating.as_mut().unwrap().incaa_rating = Some(value.unwrap_or("").to_string());
                    },
                "content-details.content-rating.oflc-rating" => {
                        request_content_details_content_rating_init(&mut request);
                        request.content_details.as_mut().unwrap().content_rating.as_mut().unwrap().oflc_rating = Some(value.unwrap_or("").to_string());
                    },
                "content-details.content-rating.fpb-rating" => {
                        request_content_details_content_rating_init(&mut request);
                        request.content_details.as_mut().unwrap().content_rating.as_mut().unwrap().fpb_rating = Some(value.unwrap_or("").to_string());
                    },
                "content-details.content-rating.mccaa-rating" => {
                        request_content_details_content_rating_init(&mut request);
                        request.content_details.as_mut().unwrap().content_rating.as_mut().unwrap().mccaa_rating = Some(value.unwrap_or("").to_string());
                    },
                "content-details.content-rating.tvpg-rating" => {
                        request_content_details_content_rating_init(&mut request);
                        request.content_details.as_mut().unwrap().content_rating.as_mut().unwrap().tvpg_rating = Some(value.unwrap_or("").to_string());
                    },
                "content-details.content-rating.rtc-rating" => {
                        request_content_details_content_rating_init(&mut request);
                        request.content_details.as_mut().unwrap().content_rating.as_mut().unwrap().rtc_rating = Some(value.unwrap_or("").to_string());
                    },
                "content-details.content-rating.cscf-rating" => {
                        request_content_details_content_rating_init(&mut request);
                        request.content_details.as_mut().unwrap().content_rating.as_mut().unwrap().cscf_rating = Some(value.unwrap_or("").to_string());
                    },
                "content-details.content-rating.fsk-rating" => {
                        request_content_details_content_rating_init(&mut request);
                        request.content_details.as_mut().unwrap().content_rating.as_mut().unwrap().fsk_rating = Some(value.unwrap_or("").to_string());
                    },
                "content-details.content-rating.bbfc-rating" => {
                        request_content_details_content_rating_init(&mut request);
                        request.content_details.as_mut().unwrap().content_rating.as_mut().unwrap().bbfc_rating = Some(value.unwrap_or("").to_string());
                    },
                "content-details.content-rating.kmrb-rating" => {
                        request_content_details_content_rating_init(&mut request);
                        request.content_details.as_mut().unwrap().content_rating.as_mut().unwrap().kmrb_rating = Some(value.unwrap_or("").to_string());
                    },
                "content-details.content-rating.smsa-rating" => {
                        request_content_details_content_rating_init(&mut request);
                        request.content_details.as_mut().unwrap().content_rating.as_mut().unwrap().smsa_rating = Some(value.unwrap_or("").to_string());
                    },
                "content-details.content-rating.egfilm-rating" => {
                        request_content_details_content_rating_init(&mut request);
                        request.content_details.as_mut().unwrap().content_rating.as_mut().unwrap().egfilm_rating = Some(value.unwrap_or("").to_string());
                    },
                "content-details.content-rating.cicf-rating" => {
                        request_content_details_content_rating_init(&mut request);
                        request.content_details.as_mut().unwrap().content_rating.as_mut().unwrap().cicf_rating = Some(value.unwrap_or("").to_string());
                    },
                "content-details.content-rating.nbcpl-rating" => {
                        request_content_details_content_rating_init(&mut request);
                        request.content_details.as_mut().unwrap().content_rating.as_mut().unwrap().nbcpl_rating = Some(value.unwrap_or("").to_string());
                    },
                "content-details.content-rating.nbc-rating" => {
                        request_content_details_content_rating_init(&mut request);
                        request.content_details.as_mut().unwrap().content_rating.as_mut().unwrap().nbc_rating = Some(value.unwrap_or("").to_string());
                    },
                "content-details.content-rating.djctq-rating" => {
                        request_content_details_content_rating_init(&mut request);
                        request.content_details.as_mut().unwrap().content_rating.as_mut().unwrap().djctq_rating = Some(value.unwrap_or("").to_string());
                    },
                "content-details.content-rating.ifco-rating" => {
                        request_content_details_content_rating_init(&mut request);
                        request.content_details.as_mut().unwrap().content_rating.as_mut().unwrap().ifco_rating = Some(value.unwrap_or("").to_string());
                    },
                "content-details.content-rating.fco-rating" => {
                        request_content_details_content_rating_init(&mut request);
                        request.content_details.as_mut().unwrap().content_rating.as_mut().unwrap().fco_rating = Some(value.unwrap_or("").to_string());
                    },
                "content-details.content-rating.eefilm-rating" => {
                        request_content_details_content_rating_init(&mut request);
                        request.content_details.as_mut().unwrap().content_rating.as_mut().unwrap().eefilm_rating = Some(value.unwrap_or("").to_string());
                    },
                "content-details.content-rating.medietilsynet-rating" => {
                        request_content_details_content_rating_init(&mut request);
                        request.content_details.as_mut().unwrap().content_rating.as_mut().unwrap().medietilsynet_rating = Some(value.unwrap_or("").to_string());
                    },
                "content-details.content-rating.grfilm-rating" => {
                        request_content_details_content_rating_init(&mut request);
                        request.content_details.as_mut().unwrap().content_rating.as_mut().unwrap().grfilm_rating = Some(value.unwrap_or("").to_string());
                    },
                "content-details.content-rating.ccc-rating" => {
                        request_content_details_content_rating_init(&mut request);
                        request.content_details.as_mut().unwrap().content_rating.as_mut().unwrap().ccc_rating = Some(value.unwrap_or("").to_string());
                    },
                "content-details.content-rating.rte-rating" => {
                        request_content_details_content_rating_init(&mut request);
                        request.content_details.as_mut().unwrap().content_rating.as_mut().unwrap().rte_rating = Some(value.unwrap_or("").to_string());
                    },
                "content-details.content-rating.czfilm-rating" => {
                        request_content_details_content_rating_init(&mut request);
                        request.content_details.as_mut().unwrap().content_rating.as_mut().unwrap().czfilm_rating = Some(value.unwrap_or("").to_string());
                    },
                "content-details.content-rating.lsf-rating" => {
                        request_content_details_content_rating_init(&mut request);
                        request.content_details.as_mut().unwrap().content_rating.as_mut().unwrap().lsf_rating = Some(value.unwrap_or("").to_string());
                    },
                "content-details.content-rating.fmoc-rating" => {
                        request_content_details_content_rating_init(&mut request);
                        request.content_details.as_mut().unwrap().content_rating.as_mut().unwrap().fmoc_rating = Some(value.unwrap_or("").to_string());
                    },
                "content-details.content-rating.eirin-rating" => {
                        request_content_details_content_rating_init(&mut request);
                        request.content_details.as_mut().unwrap().content_rating.as_mut().unwrap().eirin_rating = Some(value.unwrap_or("").to_string());
                    },
                "content-details.content-rating.cce-rating" => {
                        request_content_details_content_rating_init(&mut request);
                        request.content_details.as_mut().unwrap().content_rating.as_mut().unwrap().cce_rating = Some(value.unwrap_or("").to_string());
                    },
                "content-details.content-rating.nkclv-rating" => {
                        request_content_details_content_rating_init(&mut request);
                        request.content_details.as_mut().unwrap().content_rating.as_mut().unwrap().nkclv_rating = Some(value.unwrap_or("").to_string());
                    },
                "content-details.content-rating.mtrcb-rating" => {
                        request_content_details_content_rating_init(&mut request);
                        request.content_details.as_mut().unwrap().content_rating.as_mut().unwrap().mtrcb_rating = Some(value.unwrap_or("").to_string());
                    },
                "content-details.content-rating.mibac-rating" => {
                        request_content_details_content_rating_init(&mut request);
                        request.content_details.as_mut().unwrap().content_rating.as_mut().unwrap().mibac_rating = Some(value.unwrap_or("").to_string());
                    },
                "content-details.content-rating.ilfilm-rating" => {
                        request_content_details_content_rating_init(&mut request);
                        request.content_details.as_mut().unwrap().content_rating.as_mut().unwrap().ilfilm_rating = Some(value.unwrap_or("").to_string());
                    },
                "content-details.content-rating.smais-rating" => {
                        request_content_details_content_rating_init(&mut request);
                        request.content_details.as_mut().unwrap().content_rating.as_mut().unwrap().smais_rating = Some(value.unwrap_or("").to_string());
                    },
                "content-details.content-rating.russia-rating" => {
                        request_content_details_content_rating_init(&mut request);
                        request.content_details.as_mut().unwrap().content_rating.as_mut().unwrap().russia_rating = Some(value.unwrap_or("").to_string());
                    },
                "content-details.content-rating.mpaa-rating" => {
                        request_content_details_content_rating_init(&mut request);
                        request.content_details.as_mut().unwrap().content_rating.as_mut().unwrap().mpaa_rating = Some(value.unwrap_or("").to_string());
                    },
                "content-details.content-rating.kfcb-rating" => {
                        request_content_details_content_rating_init(&mut request);
                        request.content_details.as_mut().unwrap().content_rating.as_mut().unwrap().kfcb_rating = Some(value.unwrap_or("").to_string());
                    },
                "content-details.content-rating.agcom-rating" => {
                        request_content_details_content_rating_init(&mut request);
                        request.content_details.as_mut().unwrap().content_rating.as_mut().unwrap().agcom_rating = Some(value.unwrap_or("").to_string());
                    },
                "content-details.content-rating.chvrs-rating" => {
                        request_content_details_content_rating_init(&mut request);
                        request.content_details.as_mut().unwrap().content_rating.as_mut().unwrap().chvrs_rating = Some(value.unwrap_or("").to_string());
                    },
                "content-details.content-rating.cna-rating" => {
                        request_content_details_content_rating_init(&mut request);
                        request.content_details.as_mut().unwrap().content_rating.as_mut().unwrap().cna_rating = Some(value.unwrap_or("").to_string());
                    },
                "content-details.content-rating.icaa-rating" => {
                        request_content_details_content_rating_init(&mut request);
                        request.content_details.as_mut().unwrap().content_rating.as_mut().unwrap().icaa_rating = Some(value.unwrap_or("").to_string());
                    },
                "content-details.content-rating.mccyp-rating" => {
                        request_content_details_content_rating_init(&mut request);
                        request.content_details.as_mut().unwrap().content_rating.as_mut().unwrap().mccyp_rating = Some(value.unwrap_or("").to_string());
                    },
                "content-details.content-rating.nfrc-rating" => {
                        request_content_details_content_rating_init(&mut request);
                        request.content_details.as_mut().unwrap().content_rating.as_mut().unwrap().nfrc_rating = Some(value.unwrap_or("").to_string());
                    },
                "content-details.content-rating.skfilm-rating" => {
                        request_content_details_content_rating_init(&mut request);
                        request.content_details.as_mut().unwrap().content_rating.as_mut().unwrap().skfilm_rating = Some(value.unwrap_or("").to_string());
                    },
                "content-details.content-rating.moc-rating" => {
                        request_content_details_content_rating_init(&mut request);
                        request.content_details.as_mut().unwrap().content_rating.as_mut().unwrap().moc_rating = Some(value.unwrap_or("").to_string());
                    },
                "content-details.content-rating.rcnof-rating" => {
                        request_content_details_content_rating_init(&mut request);
                        request.content_details.as_mut().unwrap().content_rating.as_mut().unwrap().rcnof_rating = Some(value.unwrap_or("").to_string());
                    },
                "content-details.content-rating.meku-rating" => {
                        request_content_details_content_rating_init(&mut request);
                        request.content_details.as_mut().unwrap().content_rating.as_mut().unwrap().meku_rating = Some(value.unwrap_or("").to_string());
                    },
                "content-details.content-rating.fcbm-rating" => {
                        request_content_details_content_rating_init(&mut request);
                        request.content_details.as_mut().unwrap().content_rating.as_mut().unwrap().fcbm_rating = Some(value.unwrap_or("").to_string());
                    },
                "content-details.content-rating.kijkwijzer-rating" => {
                        request_content_details_content_rating_init(&mut request);
                        request.content_details.as_mut().unwrap().content_rating.as_mut().unwrap().kijkwijzer_rating = Some(value.unwrap_or("").to_string());
                    },
                "content-details.caption" => {
                        request_content_details_content_rating_init(&mut request);
                        request.content_details.as_mut().unwrap().caption = Some(value.unwrap_or("").to_string());
                    },
                "content-details.region-restriction.blocked" => {
                        request_content_details_region_restriction_init(&mut request);
                        if request.content_details.as_mut().unwrap().region_restriction.as_mut().unwrap().blocked.is_none() {
                           request.content_details.as_mut().unwrap().region_restriction.as_mut().unwrap().blocked = Some(Default::default());
                        }
                                        request.content_details.as_mut().unwrap().region_restriction.as_mut().unwrap().blocked.as_mut().unwrap().push(value.unwrap_or("").to_string());
                    },
                "content-details.region-restriction.allowed" => {
                        request_content_details_region_restriction_init(&mut request);
                        if request.content_details.as_mut().unwrap().region_restriction.as_mut().unwrap().allowed.is_none() {
                           request.content_details.as_mut().unwrap().region_restriction.as_mut().unwrap().allowed = Some(Default::default());
                        }
                                        request.content_details.as_mut().unwrap().region_restriction.as_mut().unwrap().allowed.as_mut().unwrap().push(value.unwrap_or("").to_string());
                    },
                "content-details.duration" => {
                        request_content_details_region_restriction_init(&mut request);
                        request.content_details.as_mut().unwrap().duration = Some(value.unwrap_or("").to_string());
                    },
                "content-details.licensed-content" => {
                        request_content_details_region_restriction_init(&mut request);
                        request.content_details.as_mut().unwrap().licensed_content = Some(arg_from_str(value.unwrap_or("false"), err, "content-details.licensed-content", "boolean"));
                    },
                "content-details.dimension" => {
                        request_content_details_region_restriction_init(&mut request);
                        request.content_details.as_mut().unwrap().dimension = Some(value.unwrap_or("").to_string());
                    },
                "monetization-details.access.exception" => {
                        request_monetization_details_access_init(&mut request);
                        if request.monetization_details.as_mut().unwrap().access.as_mut().unwrap().exception.is_none() {
                           request.monetization_details.as_mut().unwrap().access.as_mut().unwrap().exception = Some(Default::default());
                        }
                                        request.monetization_details.as_mut().unwrap().access.as_mut().unwrap().exception.as_mut().unwrap().push(value.unwrap_or("").to_string());
                    },
                "monetization-details.access.allowed" => {
                        request_monetization_details_access_init(&mut request);
                        request.monetization_details.as_mut().unwrap().access.as_mut().unwrap().allowed = Some(arg_from_str(value.unwrap_or("false"), err, "monetization-details.access.allowed", "boolean"));
                    },
                "age-gating.restricted" => {
                        request_age_gating_init(&mut request);
                        request.age_gating.as_mut().unwrap().restricted = Some(arg_from_str(value.unwrap_or("false"), err, "age-gating.restricted", "boolean"));
                    },
                "age-gating.alcohol-content" => {
                        request_age_gating_init(&mut request);
                        request.age_gating.as_mut().unwrap().alcohol_content = Some(arg_from_str(value.unwrap_or("false"), err, "age-gating.alcohol-content", "boolean"));
                    },
                "age-gating.video-game-rating" => {
                        request_age_gating_init(&mut request);
                        request.age_gating.as_mut().unwrap().video_game_rating = Some(value.unwrap_or("").to_string());
                    },
                "suggestions.processing-errors" => {
                        request_suggestions_init(&mut request);
                        if request.suggestions.as_mut().unwrap().processing_errors.is_none() {
                           request.suggestions.as_mut().unwrap().processing_errors = Some(Default::default());
                        }
                                        request.suggestions.as_mut().unwrap().processing_errors.as_mut().unwrap().push(value.unwrap_or("").to_string());
                    },
                "suggestions.editor-suggestions" => {
                        request_suggestions_init(&mut request);
                        if request.suggestions.as_mut().unwrap().editor_suggestions.is_none() {
                           request.suggestions.as_mut().unwrap().editor_suggestions = Some(Default::default());
                        }
                                        request.suggestions.as_mut().unwrap().editor_suggestions.as_mut().unwrap().push(value.unwrap_or("").to_string());
                    },
                "suggestions.processing-warnings" => {
                        request_suggestions_init(&mut request);
                        if request.suggestions.as_mut().unwrap().processing_warnings.is_none() {
                           request.suggestions.as_mut().unwrap().processing_warnings = Some(Default::default());
                        }
                                        request.suggestions.as_mut().unwrap().processing_warnings.as_mut().unwrap().push(value.unwrap_or("").to_string());
                    },
                "suggestions.processing-hints" => {
                        request_suggestions_init(&mut request);
                        if request.suggestions.as_mut().unwrap().processing_hints.is_none() {
                           request.suggestions.as_mut().unwrap().processing_hints = Some(Default::default());
                        }
                                        request.suggestions.as_mut().unwrap().processing_hints.as_mut().unwrap().push(value.unwrap_or("").to_string());
                    },
                "live-streaming-details.concurrent-viewers" => {
                        request_live_streaming_details_init(&mut request);
                        request.live_streaming_details.as_mut().unwrap().concurrent_viewers = Some(value.unwrap_or("").to_string());
                    },
                "live-streaming-details.scheduled-start-time" => {
                        request_live_streaming_details_init(&mut request);
                        request.live_streaming_details.as_mut().unwrap().scheduled_start_time = Some(value.unwrap_or("").to_string());
                    },
                "live-streaming-details.scheduled-end-time" => {
                        request_live_streaming_details_init(&mut request);
                        request.live_streaming_details.as_mut().unwrap().scheduled_end_time = Some(value.unwrap_or("").to_string());
                    },
                "live-streaming-details.actual-start-time" => {
                        request_live_streaming_details_init(&mut request);
                        request.live_streaming_details.as_mut().unwrap().actual_start_time = Some(value.unwrap_or("").to_string());
                    },
                "live-streaming-details.actual-end-time" => {
                        request_live_streaming_details_init(&mut request);
                        request.live_streaming_details.as_mut().unwrap().actual_end_time = Some(value.unwrap_or("").to_string());
                    },
                "file-details.bitrate-bps" => {
                        request_file_details_init(&mut request);
                        request.file_details.as_mut().unwrap().bitrate_bps = Some(value.unwrap_or("").to_string());
                    },
                "file-details.container" => {
                        request_file_details_init(&mut request);
                        request.file_details.as_mut().unwrap().container = Some(value.unwrap_or("").to_string());
                    },
                "file-details.recording-location.latitude" => {
                        request_file_details_recording_location_init(&mut request);
                        request.file_details.as_mut().unwrap().recording_location.as_mut().unwrap().latitude = Some(arg_from_str(value.unwrap_or("0.0"), err, "file-details.recording-location.latitude", "number"));
                    },
                "file-details.recording-location.altitude" => {
                        request_file_details_recording_location_init(&mut request);
                        request.file_details.as_mut().unwrap().recording_location.as_mut().unwrap().altitude = Some(arg_from_str(value.unwrap_or("0.0"), err, "file-details.recording-location.altitude", "number"));
                    },
                "file-details.recording-location.longitude" => {
                        request_file_details_recording_location_init(&mut request);
                        request.file_details.as_mut().unwrap().recording_location.as_mut().unwrap().longitude = Some(arg_from_str(value.unwrap_or("0.0"), err, "file-details.recording-location.longitude", "number"));
                    },
                "file-details.file-type" => {
                        request_file_details_recording_location_init(&mut request);
                        request.file_details.as_mut().unwrap().file_type = Some(value.unwrap_or("").to_string());
                    },
                "file-details.creation-time" => {
                        request_file_details_recording_location_init(&mut request);
                        request.file_details.as_mut().unwrap().creation_time = Some(value.unwrap_or("").to_string());
                    },
                "file-details.duration-ms" => {
                        request_file_details_recording_location_init(&mut request);
                        request.file_details.as_mut().unwrap().duration_ms = Some(value.unwrap_or("").to_string());
                    },
                "file-details.file-name" => {
                        request_file_details_recording_location_init(&mut request);
                        request.file_details.as_mut().unwrap().file_name = Some(value.unwrap_or("").to_string());
                    },
                "file-details.file-size" => {
                        request_file_details_recording_location_init(&mut request);
                        request.file_details.as_mut().unwrap().file_size = Some(value.unwrap_or("").to_string());
                    },
                "snippet.description" => {
                        request_snippet_init(&mut request);
                        request.snippet.as_mut().unwrap().description = Some(value.unwrap_or("").to_string());
                    },
                "snippet.tags" => {
                        request_snippet_init(&mut request);
                        if request.snippet.as_mut().unwrap().tags.is_none() {
                           request.snippet.as_mut().unwrap().tags = Some(Default::default());
                        }
                                        request.snippet.as_mut().unwrap().tags.as_mut().unwrap().push(value.unwrap_or("").to_string());
                    },
                "snippet.channel-id" => {
                        request_snippet_init(&mut request);
                        request.snippet.as_mut().unwrap().channel_id = Some(value.unwrap_or("").to_string());
                    },
                "snippet.published-at" => {
                        request_snippet_init(&mut request);
                        request.snippet.as_mut().unwrap().published_at = Some(value.unwrap_or("").to_string());
                    },
                "snippet.live-broadcast-content" => {
                        request_snippet_init(&mut request);
                        request.snippet.as_mut().unwrap().live_broadcast_content = Some(value.unwrap_or("").to_string());
                    },
                "snippet.default-language" => {
                        request_snippet_init(&mut request);
                        request.snippet.as_mut().unwrap().default_language = Some(value.unwrap_or("").to_string());
                    },
                "snippet.channel-title" => {
                        request_snippet_init(&mut request);
                        request.snippet.as_mut().unwrap().channel_title = Some(value.unwrap_or("").to_string());
                    },
                "snippet.title" => {
                        request_snippet_init(&mut request);
                        request.snippet.as_mut().unwrap().title = Some(value.unwrap_or("").to_string());
                    },
                "snippet.category-id" => {
                        request_snippet_init(&mut request);
                        request.snippet.as_mut().unwrap().category_id = Some(value.unwrap_or("").to_string());
                    },
                "snippet.localized.description" => {
                        request_snippet_localized_init(&mut request);
                        request.snippet.as_mut().unwrap().localized.as_mut().unwrap().description = Some(value.unwrap_or("").to_string());
                    },
                "snippet.localized.title" => {
                        request_snippet_localized_init(&mut request);
                        request.snippet.as_mut().unwrap().localized.as_mut().unwrap().title = Some(value.unwrap_or("").to_string());
                    },
                "snippet.thumbnails.default.url" => {
                        request_snippet_thumbnails_default_init(&mut request);
                        request.snippet.as_mut().unwrap().thumbnails.as_mut().unwrap().default.as_mut().unwrap().url = Some(value.unwrap_or("").to_string());
                    },
                "snippet.thumbnails.default.width" => {
                        request_snippet_thumbnails_default_init(&mut request);
                        request.snippet.as_mut().unwrap().thumbnails.as_mut().unwrap().default.as_mut().unwrap().width = Some(arg_from_str(value.unwrap_or("-0"), err, "snippet.thumbnails.default.width", "integer"));
                    },
                "snippet.thumbnails.default.height" => {
                        request_snippet_thumbnails_default_init(&mut request);
                        request.snippet.as_mut().unwrap().thumbnails.as_mut().unwrap().default.as_mut().unwrap().height = Some(arg_from_str(value.unwrap_or("-0"), err, "snippet.thumbnails.default.height", "integer"));
                    },
                "snippet.thumbnails.high.url" => {
                        request_snippet_thumbnails_high_init(&mut request);
                        request.snippet.as_mut().unwrap().thumbnails.as_mut().unwrap().high.as_mut().unwrap().url = Some(value.unwrap_or("").to_string());
                    },
                "snippet.thumbnails.high.width" => {
                        request_snippet_thumbnails_high_init(&mut request);
                        request.snippet.as_mut().unwrap().thumbnails.as_mut().unwrap().high.as_mut().unwrap().width = Some(arg_from_str(value.unwrap_or("-0"), err, "snippet.thumbnails.high.width", "integer"));
                    },
                "snippet.thumbnails.high.height" => {
                        request_snippet_thumbnails_high_init(&mut request);
                        request.snippet.as_mut().unwrap().thumbnails.as_mut().unwrap().high.as_mut().unwrap().height = Some(arg_from_str(value.unwrap_or("-0"), err, "snippet.thumbnails.high.height", "integer"));
                    },
                "snippet.thumbnails.medium.url" => {
                        request_snippet_thumbnails_medium_init(&mut request);
                        request.snippet.as_mut().unwrap().thumbnails.as_mut().unwrap().medium.as_mut().unwrap().url = Some(value.unwrap_or("").to_string());
                    },
                "snippet.thumbnails.medium.width" => {
                        request_snippet_thumbnails_medium_init(&mut request);
                        request.snippet.as_mut().unwrap().thumbnails.as_mut().unwrap().medium.as_mut().unwrap().width = Some(arg_from_str(value.unwrap_or("-0"), err, "snippet.thumbnails.medium.width", "integer"));
                    },
                "snippet.thumbnails.medium.height" => {
                        request_snippet_thumbnails_medium_init(&mut request);
                        request.snippet.as_mut().unwrap().thumbnails.as_mut().unwrap().medium.as_mut().unwrap().height = Some(arg_from_str(value.unwrap_or("-0"), err, "snippet.thumbnails.medium.height", "integer"));
                    },
                "snippet.thumbnails.maxres.url" => {
                        request_snippet_thumbnails_maxres_init(&mut request);
                        request.snippet.as_mut().unwrap().thumbnails.as_mut().unwrap().maxres.as_mut().unwrap().url = Some(value.unwrap_or("").to_string());
                    },
                "snippet.thumbnails.maxres.width" => {
                        request_snippet_thumbnails_maxres_init(&mut request);
                        request.snippet.as_mut().unwrap().thumbnails.as_mut().unwrap().maxres.as_mut().unwrap().width = Some(arg_from_str(value.unwrap_or("-0"), err, "snippet.thumbnails.maxres.width", "integer"));
                    },
                "snippet.thumbnails.maxres.height" => {
                        request_snippet_thumbnails_maxres_init(&mut request);
                        request.snippet.as_mut().unwrap().thumbnails.as_mut().unwrap().maxres.as_mut().unwrap().height = Some(arg_from_str(value.unwrap_or("-0"), err, "snippet.thumbnails.maxres.height", "integer"));
                    },
                "snippet.thumbnails.standard.url" => {
                        request_snippet_thumbnails_standard_init(&mut request);
                        request.snippet.as_mut().unwrap().thumbnails.as_mut().unwrap().standard.as_mut().unwrap().url = Some(value.unwrap_or("").to_string());
                    },
                "snippet.thumbnails.standard.width" => {
                        request_snippet_thumbnails_standard_init(&mut request);
                        request.snippet.as_mut().unwrap().thumbnails.as_mut().unwrap().standard.as_mut().unwrap().width = Some(arg_from_str(value.unwrap_or("-0"), err, "snippet.thumbnails.standard.width", "integer"));
                    },
                "snippet.thumbnails.standard.height" => {
                        request_snippet_thumbnails_standard_init(&mut request);
                        request.snippet.as_mut().unwrap().thumbnails.as_mut().unwrap().standard.as_mut().unwrap().height = Some(arg_from_str(value.unwrap_or("-0"), err, "snippet.thumbnails.standard.height", "integer"));
                    },
                "player.embed-html" => {
                        request_player_init(&mut request);
                        request.player.as_mut().unwrap().embed_html = Some(value.unwrap_or("").to_string());
                    },
                "processing-details.file-details-availability" => {
                        request_processing_details_init(&mut request);
                        request.processing_details.as_mut().unwrap().file_details_availability = Some(value.unwrap_or("").to_string());
                    },
                "processing-details.editor-suggestions-availability" => {
                        request_processing_details_init(&mut request);
                        request.processing_details.as_mut().unwrap().editor_suggestions_availability = Some(value.unwrap_or("").to_string());
                    },
                "processing-details.processing-status" => {
                        request_processing_details_init(&mut request);
                        request.processing_details.as_mut().unwrap().processing_status = Some(value.unwrap_or("").to_string());
                    },
                "processing-details.processing-issues-availability" => {
                        request_processing_details_init(&mut request);
                        request.processing_details.as_mut().unwrap().processing_issues_availability = Some(value.unwrap_or("").to_string());
                    },
                "processing-details.processing-failure-reason" => {
                        request_processing_details_init(&mut request);
                        request.processing_details.as_mut().unwrap().processing_failure_reason = Some(value.unwrap_or("").to_string());
                    },
                "processing-details.thumbnails-availability" => {
                        request_processing_details_init(&mut request);
                        request.processing_details.as_mut().unwrap().thumbnails_availability = Some(value.unwrap_or("").to_string());
                    },
                "processing-details.processing-progress.time-left-ms" => {
                        request_processing_details_processing_progress_init(&mut request);
                        request.processing_details.as_mut().unwrap().processing_progress.as_mut().unwrap().time_left_ms = Some(value.unwrap_or("").to_string());
                    },
                "processing-details.processing-progress.parts-processed" => {
                        request_processing_details_processing_progress_init(&mut request);
                        request.processing_details.as_mut().unwrap().processing_progress.as_mut().unwrap().parts_processed = Some(value.unwrap_or("").to_string());
                    },
                "processing-details.processing-progress.parts-total" => {
                        request_processing_details_processing_progress_init(&mut request);
                        request.processing_details.as_mut().unwrap().processing_progress.as_mut().unwrap().parts_total = Some(value.unwrap_or("").to_string());
                    },
                "processing-details.tag-suggestions-availability" => {
                        request_processing_details_processing_progress_init(&mut request);
                        request.processing_details.as_mut().unwrap().tag_suggestions_availability = Some(value.unwrap_or("").to_string());
                    },
                "etag" => {
                        request_processing_details_init(&mut request);
                        request.etag = Some(value.unwrap_or("").to_string());
                    },
                "project-details.tags" => {
                        request_project_details_init(&mut request);
                        if request.project_details.as_mut().unwrap().tags.is_none() {
                           request.project_details.as_mut().unwrap().tags = Some(Default::default());
                        }
                                        request.project_details.as_mut().unwrap().tags.as_mut().unwrap().push(value.unwrap_or("").to_string());
                    },
                "recording-details.recording-date" => {
                        request_recording_details_init(&mut request);
                        request.recording_details.as_mut().unwrap().recording_date = Some(value.unwrap_or("").to_string());
                    },
                "recording-details.location-description" => {
                        request_recording_details_init(&mut request);
                        request.recording_details.as_mut().unwrap().location_description = Some(value.unwrap_or("").to_string());
                    },
                "recording-details.location.latitude" => {
                        request_recording_details_location_init(&mut request);
                        request.recording_details.as_mut().unwrap().location.as_mut().unwrap().latitude = Some(arg_from_str(value.unwrap_or("0.0"), err, "recording-details.location.latitude", "number"));
                    },
                "recording-details.location.altitude" => {
                        request_recording_details_location_init(&mut request);
                        request.recording_details.as_mut().unwrap().location.as_mut().unwrap().altitude = Some(arg_from_str(value.unwrap_or("0.0"), err, "recording-details.location.altitude", "number"));
                    },
                "recording-details.location.longitude" => {
                        request_recording_details_location_init(&mut request);
                        request.recording_details.as_mut().unwrap().location.as_mut().unwrap().longitude = Some(arg_from_str(value.unwrap_or("0.0"), err, "recording-details.location.longitude", "number"));
                    },
                "id" => {
                        request_recording_details_init(&mut request);
                        request.id = Some(value.unwrap_or("").to_string());
                    },
                _ => {
                    let suggestion = FieldCursor::did_you_mean(key, &vec!["acb-rating", "access", "actual-end-time", "actual-start-time", "agcom-rating", "age-gating", "alcohol-content", "allowed", "altitude", "anatel-rating", "bbfc-rating", "bfvc-rating", "bitrate-bps", "blocked", "bmukk-rating", "caption", "category-id", "catv-rating", "catvfr-rating", "cbfc-rating", "ccc-rating", "cce-rating", "channel-id", "channel-title", "chfilm-rating", "chvrs-rating", "cicf-rating", "cna-rating", "comment-count", "concurrent-viewers", "container", "content-details", "content-rating", "country-restriction", "creation-time", "csa-rating", "cscf-rating", "czfilm-rating", "default", "default-language", "definition", "description", "dimension", "dislike-count", "djctq-rating", "djctq-rating-reasons", "duration", "duration-ms", "editor-suggestions", "editor-suggestions-availability", "eefilm-rating", "egfilm-rating", "eirin-rating", "embed-html", "embeddable", "etag", "exception", "failure-reason", "favorite-count", "fcbm-rating", "fco-rating", "file-details", "file-details-availability", "file-name", "file-size", "file-type", "fmoc-rating", "fpb-rating", "fsk-rating", "grfilm-rating", "height", "high", "icaa-rating", "id", "ifco-rating", "ilfilm-rating", "incaa-rating", "kfcb-rating", "kijkwijzer-rating", "kind", "kmrb-rating", "latitude", "license", "licensed-content", "like-count", "live-broadcast-content", "live-streaming-details", "localized", "location", "location-description", "longitude", "lsf-rating", "maxres", "mccaa-rating", "mccyp-rating", "mda-rating", "medietilsynet-rating", "medium", "meku-rating", "mibac-rating", "moc-rating", "moctw-rating", "monetization-details", "mpaa-rating", "mtrcb-rating", "nbc-rating", "nbcpl-rating", "nfrc-rating", "nfvcb-rating", "nkclv-rating", "oflc-rating", "parts-processed", "parts-total", "pefilm-rating", "player", "privacy-status", "processing-details", "processing-errors", "processing-failure-reason", "processing-hints", "processing-issues-availability", "processing-progress", "processing-status", "processing-warnings", "project-details", "public-stats-viewable", "publish-at", "published-at", "rcnof-rating", "recording-date", "recording-details", "recording-location", "region-restriction", "rejection-reason", "relevant-topic-ids", "resorteviolencia-rating", "restricted", "rtc-rating", "rte-rating", "russia-rating", "scheduled-end-time", "scheduled-start-time", "skfilm-rating", "smais-rating", "smsa-rating", "snippet", "standard", "statistics", "status", "suggestions", "tag-suggestions-availability", "tags", "thumbnails", "thumbnails-availability", "time-left-ms", "title", "topic-details", "topic-ids", "tvpg-rating", "upload-status", "url", "video-game-rating", "view-count", "width", "yt-rating"]);
                    err.issues.push(CLIError::Field(FieldError::Unknown(temp_cursor.to_string(), suggestion, value.map(|v| v.to_string()))));
                }
            }
        }
        let mut call = self.hub.videos().update(request);
        for parg in opt.values_of("v").unwrap_or(Vec::new()).iter() {
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
                                                Vec::new() + &self.gp + &["on-behalf-of-content-owner"]
                                                            ));
                    }
                }
            }
        }
        let protocol = CallType::Standard;
        if dry_run {
            Ok(())
        } else {
            assert!(err.issues.len() == 0);
            for scope in self.opt.values_of("url").unwrap_or(Vec::new()).iter() {
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
                    let mut value = json::value::to_value(&output_schema);
                    remove_json_null_values(&mut value);
                    serde::json::to_writer_pretty(&mut ostream, &value).unwrap();
                    Ok(())
                }
            }
        }
    }

    fn _watermarks_set(&self, opt: &ArgMatches<'n, 'a>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        
        let mut request = api::InvideoBranding::default();
        let mut field_cursor = FieldCursor::default();
        for kvarg in opt.values_of("kv").unwrap_or(Vec::new()).iter() {
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
            fn request_position_init(request: &mut api::InvideoBranding) {
                if request.position.is_none() {
                    request.position = Some(Default::default());
                }
            }
            
            fn request_timing_init(request: &mut api::InvideoBranding) {
                if request.timing.is_none() {
                    request.timing = Some(Default::default());
                }
            }
            
            match &temp_cursor.to_string()[..] {
                "target-channel-id" => {
                        request.target_channel_id = Some(value.unwrap_or("").to_string());
                    },
                "position.corner-position" => {
                        request_position_init(&mut request);
                        request.position.as_mut().unwrap().corner_position = Some(value.unwrap_or("").to_string());
                    },
                "position.type" => {
                        request_position_init(&mut request);
                        request.position.as_mut().unwrap().type_ = Some(value.unwrap_or("").to_string());
                    },
                "image-url" => {
                        request_position_init(&mut request);
                        request.image_url = Some(value.unwrap_or("").to_string());
                    },
                "timing.offset-ms" => {
                        request_timing_init(&mut request);
                        request.timing.as_mut().unwrap().offset_ms = Some(value.unwrap_or("").to_string());
                    },
                "timing.type" => {
                        request_timing_init(&mut request);
                        request.timing.as_mut().unwrap().type_ = Some(value.unwrap_or("").to_string());
                    },
                "timing.duration-ms" => {
                        request_timing_init(&mut request);
                        request.timing.as_mut().unwrap().duration_ms = Some(value.unwrap_or("").to_string());
                    },
                "image-bytes" => {
                        request_timing_init(&mut request);
                        request.image_bytes = Some(value.unwrap_or("").to_string());
                    },
                _ => {
                    let suggestion = FieldCursor::did_you_mean(key, &vec!["corner-position", "duration-ms", "image-bytes", "image-url", "offset-ms", "position", "target-channel-id", "timing", "type"]);
                    err.issues.push(CLIError::Field(FieldError::Unknown(temp_cursor.to_string(), suggestion, value.map(|v| v.to_string()))));
                }
            }
        }
        let mut call = self.hub.watermarks().set(request, opt.value_of("channel-id").unwrap_or(""));
        for parg in opt.values_of("v").unwrap_or(Vec::new()).iter() {
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
                                                Vec::new() + &self.gp + &["on-behalf-of-content-owner"]
                                                            ));
                    }
                }
            }
        }
        let vals = opt.values_of("mode").unwrap();
        let protocol = calltype_from_str(vals[0], ["simple", "resumable"].iter().map(|&v| v.to_string()).collect(), err);
        let mut input_file = input_file_from_opts(vals[1], err);
        let mime_type = input_mime_from_opts(opt.value_of("mime").unwrap_or("application/octet-stream"), err);
        if dry_run {
            Ok(())
        } else {
            assert!(err.issues.len() == 0);
            for scope in self.opt.values_of("url").unwrap_or(Vec::new()).iter() {
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

    fn _watermarks_unset(&self, opt: &ArgMatches<'n, 'a>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        let mut call = self.hub.watermarks().unset(opt.value_of("channel-id").unwrap_or(""));
        for parg in opt.values_of("v").unwrap_or(Vec::new()).iter() {
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
                                                Vec::new() + &self.gp + &["on-behalf-of-content-owner"]
                                                            ));
                    }
                }
            }
        }
        let protocol = CallType::Standard;
        if dry_run {
            Ok(())
        } else {
            assert!(err.issues.len() == 0);
            for scope in self.opt.values_of("url").unwrap_or(Vec::new()).iter() {
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
    fn new(opt: ArgMatches<'a, 'n>) -> Result<Engine<'a, 'n>, InvalidOptionsError> {
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
                                                    connector: hyper::net::HttpConnector(None) 
                                                })
                                        } else {
                                            hyper::Client::new()
                                        },
                                        JsonTokenStorage {
                                          program_name: "youtube3",
                                          db_dir: config_dir.clone(),
                                        }, None);

        let client = 
            if opt.is_present("debug") {
                hyper::Client::with_connector(mock::TeeConnector {
                        connector: hyper::net::HttpConnector(None) 
                    })
            } else {
                hyper::Client::new()
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
                     Some(r##"The part parameter specifies a comma-separated list of one or more activity resource properties that the API response will include. The part names that you can include in the parameter value are id, snippet, and contentDetails.
        
        If the parameter identifies a property that contains child properties, the child properties will be included in the response. For example, in a activity resource, the snippet property contains other properties that identify the type of activity, a display title for the activity, and so forth. If you set part=snippet, the API response will also contain all of those nested properties."##),
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
                     Some(r##"The part parameter specifies a comma-separated list of one or more channel resource properties that the API response will include. The part names that you can include in the parameter value are id, snippet, contentDetails, statistics, topicDetails, and invideoPromotion.
        
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
                    Some(r##"Updates a channel's metadata."##),
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
                    Some(r##"Creates a new comment thread and top level comment."##),
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
                     Some(r##"The part parameter specifies the commentThread resource parts that the API response will include. Supported values are id, snippet and replies."##),
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
                    Some(r##"Modifies an existing comment."##),
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
                     Some(r##"The id parameter specifies the comment ID for the resource that should be deleted."##),
                     Some(true),
                     Some(false)),
        
                    (Some(r##"v"##),
                     Some(r##"p"##),
                     Some(r##"Set various optional parameters, matching the key=value form"##),
                     Some(false),
                     Some(true)),
                  ]),
            ("insert",  
                    Some(r##"Creates a new comment.
        
        Note: to create a top level comment it is also necessary to create a comment thread. Both are accomplished through the commentThreads resource."##),
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
                     Some(r##"The part parameter specifies the comment resource parts that the API response will include. Supported values are id and snippet."##),
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
                    Some(r##"Expresses the caller's opinion that a comment is spam."##),
                    "Details at http://byron.github.io/google-apis-rs/google_youtube3_cli/comments_mark-as-spam",
                  vec![
                    (Some(r##"id"##),
                     None,
                     Some(r##"The id parameter specifies a comma-separated list of IDs of comments which should get flagged as spam."##),
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
                     Some(r##"The id parameter specifies a comma-separated list of IDs of comments whose moderation status should be updated."##),
                     Some(true),
                     Some(false)),
        
                    (Some(r##"moderation-status"##),
                     None,
                     Some(r##"Determines the new moderation status of the specified comments."##),
                     Some(true),
                     Some(false)),
        
                    (Some(r##"v"##),
                     Some(r##"p"##),
                     Some(r##"Set various optional parameters, matching the key=value form"##),
                     Some(false),
                     Some(true)),
                  ]),
            ("update",  
                    Some(r##"Modifies an existing comment."##),
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
                     Some(r##"The part parameter specifies a comma-separated list of one or more guideCategory resource properties that the API response will include. The part names that you can include in the parameter value are id and snippet.
        
        If the parameter identifies a property that contains child properties, the child properties will be included in the response. For example, in a guideCategory resource, the snippet property contains other properties, such as the category's title. If you set part=snippet, the API response will also contain all of those nested properties."##),
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
                    Some(r##"Returns a list of supported languages."##),
                    "Details at http://byron.github.io/google-apis-rs/google_youtube3_cli/i18n-languages_list",
                  vec![
                    (Some(r##"part"##),
                     None,
                     Some(r##"The part parameter specifies a comma-separated list of one or more i18nLanguage resource properties that the API response will include. The part names that you can include in the parameter value are id and snippet."##),
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
                    Some(r##"Returns a list of supported regions."##),
                    "Details at http://byron.github.io/google-apis-rs/google_youtube3_cli/i18n-regions_list",
                  vec![
                    (Some(r##"part"##),
                     None,
                     Some(r##"The part parameter specifies a comma-separated list of one or more i18nRegion resource properties that the API response will include. The part names that you can include in the parameter value are id and snippet."##),
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
                    Some(r##"Binds a YouTube broadcast to a stream or removes an existing binding between a broadcast and a stream. A broadcast can only be bound to one video stream."##),
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
                     Some(r##"The part parameter specifies a comma-separated list of one or more playlistItem resource properties that the API response will include. The part names that you can include in the parameter value are id, snippet, contentDetails, and status.
        
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
                     Some(r##"The part parameter specifies a comma-separated list of one or more playlist resource properties that the API response will include. The part names that you can include in the parameter value are id, snippet, status, and contentDetails.
        
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
                     Some(r##"The part parameter specifies a comma-separated list of one or more search resource properties that the API response will include. The part names that you can include in the parameter value are id and snippet.
        
        If the parameter identifies a property that contains child properties, the child properties will be included in the response. For example, in a search result, the snippet property contains other properties that identify the result's title, description, and so forth. If you set part=snippet, the API response will also contain all of those nested properties."##),
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
                     Some(r##"The part parameter specifies a comma-separated list of one or more subscription resource properties that the API response will include. The part names that you can include in the parameter value are id, snippet, and contentDetails.
        
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
                     Some(r##"The part parameter specifies a comma-separated list of one or more video resource properties that the API response will include. The part names that you can include in the parameter value are id, snippet, contentDetails, fileDetails, liveStreamingDetails, localizations, player, processingDetails, recordingDetails, statistics, status, suggestions, and topicDetails.
        
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
                     Some(r##"The channelId parameter specifies a YouTube channel ID for which the watermark is being provided."##),
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
                    Some(r##"Deletes a watermark."##),
                    "Details at http://byron.github.io/google-apis-rs/google_youtube3_cli/watermarks_unset",
                  vec![
                    (Some(r##"channel-id"##),
                     None,
                     Some(r##"The channelId parameter specifies a YouTube channel ID for which the watermark is being unset."##),
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
           .version("0.2.0+20150414")
           .about("Programmatic access to YouTube features.")
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
           
           for &(main_command_name, ref about, ref subcommands) in arg_data.iter() {
               let mut mcmd = SubCommand::new(main_command_name).about(about);
           
               for &(sub_command_name, ref desc, url_info, ref args) in subcommands {
                   let mut scmd = SubCommand::new(sub_command_name);
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
                       let mut arg = Arg::with_name(arg_name_str);
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
            env::set_exit_status(err.exit_code);
            writeln!(io::stderr(), "{}", err).ok();
        },
        Ok(engine) => {
            if let Err(doit_err) = engine.doit() {
                env::set_exit_status(1);
                match doit_err {
                    DoitError::IoError(path, err) => {
                        writeln!(io::stderr(), "Failed to open output file '{}': {}", path, err).ok();
                    },
                    DoitError::ApiError(err) => {
                        if debug {
                            writeln!(io::stderr(), "{:?}", err).ok();
                        } else {
                            writeln!(io::stderr(), "{}", err).ok();
                        }
                    }
                }
            }
        }
    }
}