// DO NOT EDIT !
// This file was generated automatically from 'src/mako/cli/main.rs.mako'
// DO NOT EDIT !
#![feature(plugin, exit_status)]
#![plugin(docopt_macros)]
#![allow(unused_variables, unused_imports, dead_code, unused_mut)]

extern crate docopt;
extern crate yup_oauth2 as oauth2;
extern crate rustc_serialize;
extern crate serde;
extern crate hyper;
extern crate mime;
extern crate google_youtube3 as api;

use std::env;
use std::io::{self, Write};

docopt!(Options derive Debug, "
Usage: 
  youtube3 [options] activities insert -r <kv>... [-p <v>]... [-o <out>]
  youtube3 [options] activities list <part> [-p <v>]... [-o <out>]
  youtube3 [options] captions delete <id> [-p <v>]...
  youtube3 [options] captions download <id> [-p <v>]... [-o <out>]
  youtube3 [options] captions insert -r <kv>... -u (simple|resumable) <file> <mime> [-p <v>]... [-o <out>]
  youtube3 [options] captions list <part> <video-id> [-p <v>]... [-o <out>]
  youtube3 [options] captions update -r <kv>... -u (simple|resumable) <file> <mime> [-p <v>]... [-o <out>]
  youtube3 [options] channel-banners insert -r <kv>... -u (simple|resumable) <file> <mime> [-p <v>]... [-o <out>]
  youtube3 [options] channel-sections delete <id> [-p <v>]...
  youtube3 [options] channel-sections insert -r <kv>... [-p <v>]... [-o <out>]
  youtube3 [options] channel-sections list <part> [-p <v>]... [-o <out>]
  youtube3 [options] channel-sections update -r <kv>... [-p <v>]... [-o <out>]
  youtube3 [options] channels list <part> [-p <v>]... [-o <out>]
  youtube3 [options] channels update -r <kv>... [-p <v>]... [-o <out>]
  youtube3 [options] guide-categories list <part> [-p <v>]... [-o <out>]
  youtube3 [options] i18n-languages list <part> [-p <v>]... [-o <out>]
  youtube3 [options] i18n-regions list <part> [-p <v>]... [-o <out>]
  youtube3 [options] live-broadcasts bind <id> <part> [-p <v>]... [-o <out>]
  youtube3 [options] live-broadcasts control <id> <part> [-p <v>]... [-o <out>]
  youtube3 [options] live-broadcasts delete <id> [-p <v>]...
  youtube3 [options] live-broadcasts insert -r <kv>... [-p <v>]... [-o <out>]
  youtube3 [options] live-broadcasts list <part> [-p <v>]... [-o <out>]
  youtube3 [options] live-broadcasts transition <broadcast-status> <id> <part> [-p <v>]... [-o <out>]
  youtube3 [options] live-broadcasts update -r <kv>... [-p <v>]... [-o <out>]
  youtube3 [options] live-streams delete <id> [-p <v>]...
  youtube3 [options] live-streams insert -r <kv>... [-p <v>]... [-o <out>]
  youtube3 [options] live-streams list <part> [-p <v>]... [-o <out>]
  youtube3 [options] live-streams update -r <kv>... [-p <v>]... [-o <out>]
  youtube3 [options] playlist-items delete <id> [-p <v>]...
  youtube3 [options] playlist-items insert -r <kv>... [-p <v>]... [-o <out>]
  youtube3 [options] playlist-items list <part> [-p <v>]... [-o <out>]
  youtube3 [options] playlist-items update -r <kv>... [-p <v>]... [-o <out>]
  youtube3 [options] playlists delete <id> [-p <v>]...
  youtube3 [options] playlists insert -r <kv>... [-p <v>]... [-o <out>]
  youtube3 [options] playlists list <part> [-p <v>]... [-o <out>]
  youtube3 [options] playlists update -r <kv>... [-p <v>]... [-o <out>]
  youtube3 [options] search list <part> [-p <v>]... [-o <out>]
  youtube3 [options] subscriptions delete <id> [-p <v>]...
  youtube3 [options] subscriptions insert -r <kv>... [-p <v>]... [-o <out>]
  youtube3 [options] subscriptions list <part> [-p <v>]... [-o <out>]
  youtube3 [options] thumbnails set <video-id> -u (simple|resumable) <file> <mime> [-p <v>]... [-o <out>]
  youtube3 [options] video-categories list <part> [-p <v>]... [-o <out>]
  youtube3 [options] videos delete <id> [-p <v>]...
  youtube3 [options] videos get-rating <id> [-p <v>]... [-o <out>]
  youtube3 [options] videos insert -r <kv>... -u (simple|resumable) <file> <mime> [-p <v>]... [-o <out>]
  youtube3 [options] videos list <part> [-p <v>]... [-o <out>]
  youtube3 [options] videos rate <id> <rating> [-p <v>]...
  youtube3 [options] videos update -r <kv>... [-p <v>]... [-o <out>]
  youtube3 [options] watermarks set <channel-id> -r <kv>... -u (simple|resumable) <file> <mime> [-p <v>]...
  youtube3 [options] watermarks unset <channel-id> [-p <v>]...
  youtube3 --help

All documentation details can be found TODO: <URL to github.io docs here, see #51>

Configuration:
  --scope <url>  
            Specify the authentication a method should be executed in. Each scope requires
            the user to grant this application permission to use it.
            If unset, it defaults to the shortest scope url for a particular method.
  --config-dir <folder>
            A directory into which we will store our persistent data. Defaults to a user-writable
            directory that we will create during the first invocation.
            [default: ~/.google-service-cli]
");

mod cmn;
use cmn::{InvalidOptionsError, CLIError, JsonTokenStorage, arg_from_str, writer_from_opts, parse_kv_arg, 
          input_file_from_opts, input_mime_from_opts, FieldCursor, FieldError};

use std::default::Default;
use std::str::FromStr;

use oauth2::{Authenticator, DefaultAuthenticatorDelegate};
use rustc_serialize::json;

struct Engine {
    opt: Options,
    hub: api::YouTube<hyper::Client, Authenticator<DefaultAuthenticatorDelegate, JsonTokenStorage, hyper::Client>>,
}


impl Engine {
    fn _activities_insert(&self, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Option<api::Error> {
            let mut request: api::Activity = Default::default();
        let mut call = self.hub.activities().insert(&request);
        for parg in self.opt.arg_v.iter() {
            let (key, value) = parse_kv_arg(&*parg, err);
            match key {
                "alt"
                |"fields"
                |"key"
                |"oauth-token"
                |"pretty-print"
                |"quota-user"
                |"user-ip" => {
                    let map = [
                        ("oauth-token", "oauth_token"),
                        ("pretty-print", "prettyPrint"),
                        ("quota-user", "quotaUser"),
                        ("user-ip", "userIp"),
                    ];
                    call = call.param(map.iter().find(|t| t.0 == key).unwrap_or(&("", key)).1, value.unwrap_or("unset"))
                },
                _ => err.issues.push(CLIError::UnknownParameter(key.to_string())),
            }
        }
        let mut field_name: FieldCursor = Default::default();
        for kvarg in self.opt.arg_kv.iter() {
            let (key, value) = parse_kv_arg(&*kvarg, err);
            if let Err(field_err) = field_name.set(&*key) {
                err.issues.push(field_err);
            }
            fn request_content_details_init(request: &mut api::Activity) {
                if request.content_details.is_none() {
                    request.content_details = Some(Default::default());
                }
            }
            
            fn request_snippet_init(request: &mut api::Activity) {
                if request.snippet.is_none() {
                    request.snippet = Some(Default::default());
                }
            }
            
            match &field_name.to_string()[..] {
                "snippet.thumbnails.default.url" => {
                        request_snippet_init(&mut request);
                        request.snippet.as_mut().unwrap().thumbnails.default.url = Some(value.unwrap_or("").to_string());
                    },
                "snippet.thumbnails.default.width" => {
                        request_snippet_init(&mut request);
                        request.snippet.as_mut().unwrap().thumbnails.default.width = Some(arg_from_str(value.unwrap_or("-0"), err, "snippet.thumbnails.default.width", "integer"));
                    },
                "snippet.thumbnails.default.height" => {
                        request_snippet_init(&mut request);
                        request.snippet.as_mut().unwrap().thumbnails.default.height = Some(arg_from_str(value.unwrap_or("-0"), err, "snippet.thumbnails.default.height", "integer"));
                    },
                "snippet.thumbnails.high.url" => {
                        request_snippet_init(&mut request);
                        request.snippet.as_mut().unwrap().thumbnails.high.url = Some(value.unwrap_or("").to_string());
                    },
                "snippet.thumbnails.high.width" => {
                        request_snippet_init(&mut request);
                        request.snippet.as_mut().unwrap().thumbnails.high.width = Some(arg_from_str(value.unwrap_or("-0"), err, "snippet.thumbnails.high.width", "integer"));
                    },
                "snippet.thumbnails.high.height" => {
                        request_snippet_init(&mut request);
                        request.snippet.as_mut().unwrap().thumbnails.high.height = Some(arg_from_str(value.unwrap_or("-0"), err, "snippet.thumbnails.high.height", "integer"));
                    },
                "snippet.thumbnails.medium.url" => {
                        request_snippet_init(&mut request);
                        request.snippet.as_mut().unwrap().thumbnails.medium.url = Some(value.unwrap_or("").to_string());
                    },
                "snippet.thumbnails.medium.width" => {
                        request_snippet_init(&mut request);
                        request.snippet.as_mut().unwrap().thumbnails.medium.width = Some(arg_from_str(value.unwrap_or("-0"), err, "snippet.thumbnails.medium.width", "integer"));
                    },
                "snippet.thumbnails.medium.height" => {
                        request_snippet_init(&mut request);
                        request.snippet.as_mut().unwrap().thumbnails.medium.height = Some(arg_from_str(value.unwrap_or("-0"), err, "snippet.thumbnails.medium.height", "integer"));
                    },
                "snippet.thumbnails.maxres.url" => {
                        request_snippet_init(&mut request);
                        request.snippet.as_mut().unwrap().thumbnails.maxres.url = Some(value.unwrap_or("").to_string());
                    },
                "snippet.thumbnails.maxres.width" => {
                        request_snippet_init(&mut request);
                        request.snippet.as_mut().unwrap().thumbnails.maxres.width = Some(arg_from_str(value.unwrap_or("-0"), err, "snippet.thumbnails.maxres.width", "integer"));
                    },
                "snippet.thumbnails.maxres.height" => {
                        request_snippet_init(&mut request);
                        request.snippet.as_mut().unwrap().thumbnails.maxres.height = Some(arg_from_str(value.unwrap_or("-0"), err, "snippet.thumbnails.maxres.height", "integer"));
                    },
                "snippet.thumbnails.standard.url" => {
                        request_snippet_init(&mut request);
                        request.snippet.as_mut().unwrap().thumbnails.standard.url = Some(value.unwrap_or("").to_string());
                    },
                "snippet.thumbnails.standard.width" => {
                        request_snippet_init(&mut request);
                        request.snippet.as_mut().unwrap().thumbnails.standard.width = Some(arg_from_str(value.unwrap_or("-0"), err, "snippet.thumbnails.standard.width", "integer"));
                    },
                "snippet.thumbnails.standard.height" => {
                        request_snippet_init(&mut request);
                        request.snippet.as_mut().unwrap().thumbnails.standard.height = Some(arg_from_str(value.unwrap_or("-0"), err, "snippet.thumbnails.standard.height", "integer"));
                    },
                "snippet.title" => {
                        request_snippet_init(&mut request);
                        request.snippet.as_mut().unwrap().title = value.unwrap_or("").to_string();
                    },
                "snippet.channel-id" => {
                        request_snippet_init(&mut request);
                        request.snippet.as_mut().unwrap().channel_id = value.unwrap_or("").to_string();
                    },
                "snippet.published-at" => {
                        request_snippet_init(&mut request);
                        request.snippet.as_mut().unwrap().published_at = value.unwrap_or("").to_string();
                    },
                "snippet.channel-title" => {
                        request_snippet_init(&mut request);
                        request.snippet.as_mut().unwrap().channel_title = value.unwrap_or("").to_string();
                    },
                "snippet.type" => {
                        request_snippet_init(&mut request);
                        request.snippet.as_mut().unwrap().type_ = value.unwrap_or("").to_string();
                    },
                "snippet.group-id" => {
                        request_snippet_init(&mut request);
                        request.snippet.as_mut().unwrap().group_id = value.unwrap_or("").to_string();
                    },
                "snippet.description" => {
                        request_snippet_init(&mut request);
                        request.snippet.as_mut().unwrap().description = value.unwrap_or("").to_string();
                    },
                "content-details.comment.resource-id.kind" => {
                        request_content_details_init(&mut request);
                        request.content_details.as_mut().unwrap().comment.resource_id.kind = value.unwrap_or("").to_string();
                    },
                "content-details.comment.resource-id.channel-id" => {
                        request_content_details_init(&mut request);
                        request.content_details.as_mut().unwrap().comment.resource_id.channel_id = value.unwrap_or("").to_string();
                    },
                "content-details.comment.resource-id.playlist-id" => {
                        request_content_details_init(&mut request);
                        request.content_details.as_mut().unwrap().comment.resource_id.playlist_id = value.unwrap_or("").to_string();
                    },
                "content-details.comment.resource-id.video-id" => {
                        request_content_details_init(&mut request);
                        request.content_details.as_mut().unwrap().comment.resource_id.video_id = value.unwrap_or("").to_string();
                    },
                "content-details.playlist-item.resource-id.kind" => {
                        request_content_details_init(&mut request);
                        request.content_details.as_mut().unwrap().playlist_item.resource_id.kind = value.unwrap_or("").to_string();
                    },
                "content-details.playlist-item.resource-id.channel-id" => {
                        request_content_details_init(&mut request);
                        request.content_details.as_mut().unwrap().playlist_item.resource_id.channel_id = value.unwrap_or("").to_string();
                    },
                "content-details.playlist-item.resource-id.playlist-id" => {
                        request_content_details_init(&mut request);
                        request.content_details.as_mut().unwrap().playlist_item.resource_id.playlist_id = value.unwrap_or("").to_string();
                    },
                "content-details.playlist-item.resource-id.video-id" => {
                        request_content_details_init(&mut request);
                        request.content_details.as_mut().unwrap().playlist_item.resource_id.video_id = value.unwrap_or("").to_string();
                    },
                "content-details.playlist-item.playlist-id" => {
                        request_content_details_init(&mut request);
                        request.content_details.as_mut().unwrap().playlist_item.playlist_id = value.unwrap_or("").to_string();
                    },
                "content-details.playlist-item.playlist-item-id" => {
                        request_content_details_init(&mut request);
                        request.content_details.as_mut().unwrap().playlist_item.playlist_item_id = value.unwrap_or("").to_string();
                    },
                "content-details.like.resource-id.kind" => {
                        request_content_details_init(&mut request);
                        request.content_details.as_mut().unwrap().like.resource_id.kind = value.unwrap_or("").to_string();
                    },
                "content-details.like.resource-id.channel-id" => {
                        request_content_details_init(&mut request);
                        request.content_details.as_mut().unwrap().like.resource_id.channel_id = value.unwrap_or("").to_string();
                    },
                "content-details.like.resource-id.playlist-id" => {
                        request_content_details_init(&mut request);
                        request.content_details.as_mut().unwrap().like.resource_id.playlist_id = value.unwrap_or("").to_string();
                    },
                "content-details.like.resource-id.video-id" => {
                        request_content_details_init(&mut request);
                        request.content_details.as_mut().unwrap().like.resource_id.video_id = value.unwrap_or("").to_string();
                    },
                "content-details.promoted-item.cta-type" => {
                        request_content_details_init(&mut request);
                        request.content_details.as_mut().unwrap().promoted_item.cta_type = value.unwrap_or("").to_string();
                    },
                "content-details.promoted-item.ad-tag" => {
                        request_content_details_init(&mut request);
                        request.content_details.as_mut().unwrap().promoted_item.ad_tag = value.unwrap_or("").to_string();
                    },
                "content-details.promoted-item.destination-url" => {
                        request_content_details_init(&mut request);
                        request.content_details.as_mut().unwrap().promoted_item.destination_url = value.unwrap_or("").to_string();
                    },
                "content-details.promoted-item.forecasting-url" => {
                        request_content_details_init(&mut request);
                        request.content_details.as_mut().unwrap().promoted_item.forecasting_url.push(value.unwrap_or("").to_string());
                    },
                "content-details.promoted-item.impression-url" => {
                        request_content_details_init(&mut request);
                        request.content_details.as_mut().unwrap().promoted_item.impression_url.push(value.unwrap_or("").to_string());
                    },
                "content-details.promoted-item.creative-view-url" => {
                        request_content_details_init(&mut request);
                        request.content_details.as_mut().unwrap().promoted_item.creative_view_url = value.unwrap_or("").to_string();
                    },
                "content-details.promoted-item.video-id" => {
                        request_content_details_init(&mut request);
                        request.content_details.as_mut().unwrap().promoted_item.video_id = value.unwrap_or("").to_string();
                    },
                "content-details.promoted-item.description-text" => {
                        request_content_details_init(&mut request);
                        request.content_details.as_mut().unwrap().promoted_item.description_text = value.unwrap_or("").to_string();
                    },
                "content-details.promoted-item.custom-cta-button-text" => {
                        request_content_details_init(&mut request);
                        request.content_details.as_mut().unwrap().promoted_item.custom_cta_button_text = value.unwrap_or("").to_string();
                    },
                "content-details.promoted-item.click-tracking-url" => {
                        request_content_details_init(&mut request);
                        request.content_details.as_mut().unwrap().promoted_item.click_tracking_url = value.unwrap_or("").to_string();
                    },
                "content-details.social.resource-id.kind" => {
                        request_content_details_init(&mut request);
                        request.content_details.as_mut().unwrap().social.resource_id.kind = value.unwrap_or("").to_string();
                    },
                "content-details.social.resource-id.channel-id" => {
                        request_content_details_init(&mut request);
                        request.content_details.as_mut().unwrap().social.resource_id.channel_id = value.unwrap_or("").to_string();
                    },
                "content-details.social.resource-id.playlist-id" => {
                        request_content_details_init(&mut request);
                        request.content_details.as_mut().unwrap().social.resource_id.playlist_id = value.unwrap_or("").to_string();
                    },
                "content-details.social.resource-id.video-id" => {
                        request_content_details_init(&mut request);
                        request.content_details.as_mut().unwrap().social.resource_id.video_id = value.unwrap_or("").to_string();
                    },
                "content-details.social.image-url" => {
                        request_content_details_init(&mut request);
                        request.content_details.as_mut().unwrap().social.image_url = value.unwrap_or("").to_string();
                    },
                "content-details.social.type" => {
                        request_content_details_init(&mut request);
                        request.content_details.as_mut().unwrap().social.type_ = value.unwrap_or("").to_string();
                    },
                "content-details.social.reference-url" => {
                        request_content_details_init(&mut request);
                        request.content_details.as_mut().unwrap().social.reference_url = value.unwrap_or("").to_string();
                    },
                "content-details.social.author" => {
                        request_content_details_init(&mut request);
                        request.content_details.as_mut().unwrap().social.author = value.unwrap_or("").to_string();
                    },
                "content-details.favorite.resource-id.kind" => {
                        request_content_details_init(&mut request);
                        request.content_details.as_mut().unwrap().favorite.resource_id.kind = value.unwrap_or("").to_string();
                    },
                "content-details.favorite.resource-id.channel-id" => {
                        request_content_details_init(&mut request);
                        request.content_details.as_mut().unwrap().favorite.resource_id.channel_id = value.unwrap_or("").to_string();
                    },
                "content-details.favorite.resource-id.playlist-id" => {
                        request_content_details_init(&mut request);
                        request.content_details.as_mut().unwrap().favorite.resource_id.playlist_id = value.unwrap_or("").to_string();
                    },
                "content-details.favorite.resource-id.video-id" => {
                        request_content_details_init(&mut request);
                        request.content_details.as_mut().unwrap().favorite.resource_id.video_id = value.unwrap_or("").to_string();
                    },
                "content-details.upload.video-id" => {
                        request_content_details_init(&mut request);
                        request.content_details.as_mut().unwrap().upload.video_id = value.unwrap_or("").to_string();
                    },
                "content-details.recommendation.resource-id.kind" => {
                        request_content_details_init(&mut request);
                        request.content_details.as_mut().unwrap().recommendation.resource_id.kind = value.unwrap_or("").to_string();
                    },
                "content-details.recommendation.resource-id.channel-id" => {
                        request_content_details_init(&mut request);
                        request.content_details.as_mut().unwrap().recommendation.resource_id.channel_id = value.unwrap_or("").to_string();
                    },
                "content-details.recommendation.resource-id.playlist-id" => {
                        request_content_details_init(&mut request);
                        request.content_details.as_mut().unwrap().recommendation.resource_id.playlist_id = value.unwrap_or("").to_string();
                    },
                "content-details.recommendation.resource-id.video-id" => {
                        request_content_details_init(&mut request);
                        request.content_details.as_mut().unwrap().recommendation.resource_id.video_id = value.unwrap_or("").to_string();
                    },
                "content-details.recommendation.reason" => {
                        request_content_details_init(&mut request);
                        request.content_details.as_mut().unwrap().recommendation.reason = value.unwrap_or("").to_string();
                    },
                "content-details.recommendation.seed-resource-id.kind" => {
                        request_content_details_init(&mut request);
                        request.content_details.as_mut().unwrap().recommendation.seed_resource_id.kind = value.unwrap_or("").to_string();
                    },
                "content-details.recommendation.seed-resource-id.channel-id" => {
                        request_content_details_init(&mut request);
                        request.content_details.as_mut().unwrap().recommendation.seed_resource_id.channel_id = value.unwrap_or("").to_string();
                    },
                "content-details.recommendation.seed-resource-id.playlist-id" => {
                        request_content_details_init(&mut request);
                        request.content_details.as_mut().unwrap().recommendation.seed_resource_id.playlist_id = value.unwrap_or("").to_string();
                    },
                "content-details.recommendation.seed-resource-id.video-id" => {
                        request_content_details_init(&mut request);
                        request.content_details.as_mut().unwrap().recommendation.seed_resource_id.video_id = value.unwrap_or("").to_string();
                    },
                "content-details.subscription.resource-id.kind" => {
                        request_content_details_init(&mut request);
                        request.content_details.as_mut().unwrap().subscription.resource_id.kind = value.unwrap_or("").to_string();
                    },
                "content-details.subscription.resource-id.channel-id" => {
                        request_content_details_init(&mut request);
                        request.content_details.as_mut().unwrap().subscription.resource_id.channel_id = value.unwrap_or("").to_string();
                    },
                "content-details.subscription.resource-id.playlist-id" => {
                        request_content_details_init(&mut request);
                        request.content_details.as_mut().unwrap().subscription.resource_id.playlist_id = value.unwrap_or("").to_string();
                    },
                "content-details.subscription.resource-id.video-id" => {
                        request_content_details_init(&mut request);
                        request.content_details.as_mut().unwrap().subscription.resource_id.video_id = value.unwrap_or("").to_string();
                    },
                "content-details.bulletin.resource-id.kind" => {
                        request_content_details_init(&mut request);
                        request.content_details.as_mut().unwrap().bulletin.resource_id.kind = value.unwrap_or("").to_string();
                    },
                "content-details.bulletin.resource-id.channel-id" => {
                        request_content_details_init(&mut request);
                        request.content_details.as_mut().unwrap().bulletin.resource_id.channel_id = value.unwrap_or("").to_string();
                    },
                "content-details.bulletin.resource-id.playlist-id" => {
                        request_content_details_init(&mut request);
                        request.content_details.as_mut().unwrap().bulletin.resource_id.playlist_id = value.unwrap_or("").to_string();
                    },
                "content-details.bulletin.resource-id.video-id" => {
                        request_content_details_init(&mut request);
                        request.content_details.as_mut().unwrap().bulletin.resource_id.video_id = value.unwrap_or("").to_string();
                    },
                "content-details.channel-item.resource-id.kind" => {
                        request_content_details_init(&mut request);
                        request.content_details.as_mut().unwrap().channel_item.resource_id.kind = value.unwrap_or("").to_string();
                    },
                "content-details.channel-item.resource-id.channel-id" => {
                        request_content_details_init(&mut request);
                        request.content_details.as_mut().unwrap().channel_item.resource_id.channel_id = value.unwrap_or("").to_string();
                    },
                "content-details.channel-item.resource-id.playlist-id" => {
                        request_content_details_init(&mut request);
                        request.content_details.as_mut().unwrap().channel_item.resource_id.playlist_id = value.unwrap_or("").to_string();
                    },
                "content-details.channel-item.resource-id.video-id" => {
                        request_content_details_init(&mut request);
                        request.content_details.as_mut().unwrap().channel_item.resource_id.video_id = value.unwrap_or("").to_string();
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
                    err.issues.push(CLIError::Field(FieldError::Unknown(field_name.to_string())));
                }
            }
        }
        let protocol = "standard-request";
        if dry_run {
            None
        } else {
            assert!(err.issues.len() == 0);
            let mut ostream = writer_from_opts(self.opt.flag_o, &self.opt.arg_out);
            match match protocol {
                "standard-request" => call.doit(),
                _ => unreachable!(),
            } {
                Err(api_err) => Some(api_err),
                Ok((mut response, output_schema)) => {
                    println!("DEBUG: REMOVE ME {:?}", response);
                    serde::json::to_writer(&mut ostream, &output_schema).unwrap();
                    None
                }
            }
        }
    }

    fn _activities_list(&self, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Option<api::Error> {
        let mut call = self.hub.activities().list(&self.opt.arg_part);
        for parg in self.opt.arg_v.iter() {
            let (key, value) = parse_kv_arg(&*parg, err);
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
                "alt"
                |"fields"
                |"key"
                |"oauth-token"
                |"pretty-print"
                |"quota-user"
                |"user-ip" => {
                    let map = [
                        ("oauth-token", "oauth_token"),
                        ("pretty-print", "prettyPrint"),
                        ("quota-user", "quotaUser"),
                        ("user-ip", "userIp"),
                    ];
                    call = call.param(map.iter().find(|t| t.0 == key).unwrap_or(&("", key)).1, value.unwrap_or("unset"))
                },
                _ => err.issues.push(CLIError::UnknownParameter(key.to_string())),
            }
        }
        let protocol = "standard-request";
        if dry_run {
            None
        } else {
            assert!(err.issues.len() == 0);
            let mut ostream = writer_from_opts(self.opt.flag_o, &self.opt.arg_out);
            match match protocol {
                "standard-request" => call.doit(),
                _ => unreachable!(),
            } {
                Err(api_err) => Some(api_err),
                Ok((mut response, output_schema)) => {
                    println!("DEBUG: REMOVE ME {:?}", response);
                    serde::json::to_writer(&mut ostream, &output_schema).unwrap();
                    None
                }
            }
        }
    }

    fn _captions_delete(&self, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Option<api::Error> {
        let mut call = self.hub.captions().delete(&self.opt.arg_id);
        for parg in self.opt.arg_v.iter() {
            let (key, value) = parse_kv_arg(&*parg, err);
            match key {
                "on-behalf-of" => {
                    call = call.on_behalf_of(value.unwrap_or(""));
                },
                "debug-project-id-override" => {
                    call = call.debug_project_id_override(value.unwrap_or(""));
                },
                "alt"
                |"fields"
                |"key"
                |"oauth-token"
                |"pretty-print"
                |"quota-user"
                |"user-ip" => {
                    let map = [
                        ("oauth-token", "oauth_token"),
                        ("pretty-print", "prettyPrint"),
                        ("quota-user", "quotaUser"),
                        ("user-ip", "userIp"),
                    ];
                    call = call.param(map.iter().find(|t| t.0 == key).unwrap_or(&("", key)).1, value.unwrap_or("unset"))
                },
                _ => err.issues.push(CLIError::UnknownParameter(key.to_string())),
            }
        }
        let protocol = "standard-request";
        if dry_run {
            None
        } else {
            assert!(err.issues.len() == 0);
            match match protocol {
                "standard-request" => call.doit(),
                _ => unreachable!(),
            } {
                Err(api_err) => Some(api_err),
                Ok(mut response) => {
                    println!("DEBUG: REMOVE ME {:?}", response);
                    None
                }
            }
        }
    }

    fn _captions_download(&self, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Option<api::Error> {
        let mut download_mode = false;
        let mut call = self.hub.captions().download(&self.opt.arg_id);
        for parg in self.opt.arg_v.iter() {
            let (key, value) = parse_kv_arg(&*parg, err);
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
                "alt"
                |"fields"
                |"key"
                |"oauth-token"
                |"pretty-print"
                |"quota-user"
                |"user-ip" => {
                    if key == "alt" && value.unwrap_or("unset") == "media" {
                        download_mode = true;
                    }
                    let map = [
                        ("oauth-token", "oauth_token"),
                        ("pretty-print", "prettyPrint"),
                        ("quota-user", "quotaUser"),
                        ("user-ip", "userIp"),
                    ];
                    call = call.param(map.iter().find(|t| t.0 == key).unwrap_or(&("", key)).1, value.unwrap_or("unset"))
                },
                _ => err.issues.push(CLIError::UnknownParameter(key.to_string())),
            }
        }
        let protocol = "standard-request";
        if dry_run {
            None
        } else {
            assert!(err.issues.len() == 0);
            let mut ostream = writer_from_opts(self.opt.flag_o, &self.opt.arg_out);
            match match protocol {
                "standard-request" => call.doit(),
                _ => unreachable!(),
            } {
                Err(api_err) => Some(api_err),
                Ok(mut response) => {
                    println!("DEBUG: REMOVE ME {:?}", response);
                    if !download_mode {
                    } else {
                    io::copy(&mut response, &mut ostream).unwrap();
                    }
                    None
                }
            }
        }
    }

    fn _captions_insert(&self, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Option<api::Error> {
            let mut request: api::Caption = Default::default();
        let mut call = self.hub.captions().insert(&request);
        for parg in self.opt.arg_v.iter() {
            let (key, value) = parse_kv_arg(&*parg, err);
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
                "alt"
                |"fields"
                |"key"
                |"oauth-token"
                |"pretty-print"
                |"quota-user"
                |"user-ip" => {
                    let map = [
                        ("oauth-token", "oauth_token"),
                        ("pretty-print", "prettyPrint"),
                        ("quota-user", "quotaUser"),
                        ("user-ip", "userIp"),
                    ];
                    call = call.param(map.iter().find(|t| t.0 == key).unwrap_or(&("", key)).1, value.unwrap_or("unset"))
                },
                _ => err.issues.push(CLIError::UnknownParameter(key.to_string())),
            }
        }
        let mut field_name: FieldCursor = Default::default();
        for kvarg in self.opt.arg_kv.iter() {
            let (key, value) = parse_kv_arg(&*kvarg, err);
            if let Err(field_err) = field_name.set(&*key) {
                err.issues.push(field_err);
            }
            fn request_snippet_init(request: &mut api::Caption) {
                if request.snippet.is_none() {
                    request.snippet = Some(Default::default());
                }
            }
            
            match &field_name.to_string()[..] {
                "snippet.status" => {
                        request_snippet_init(&mut request);
                        request.snippet.as_mut().unwrap().status = value.unwrap_or("").to_string();
                    },
                "snippet.audio-track-type" => {
                        request_snippet_init(&mut request);
                        request.snippet.as_mut().unwrap().audio_track_type = value.unwrap_or("").to_string();
                    },
                "snippet.language" => {
                        request_snippet_init(&mut request);
                        request.snippet.as_mut().unwrap().language = value.unwrap_or("").to_string();
                    },
                "snippet.name" => {
                        request_snippet_init(&mut request);
                        request.snippet.as_mut().unwrap().name = value.unwrap_or("").to_string();
                    },
                "snippet.video-id" => {
                        request_snippet_init(&mut request);
                        request.snippet.as_mut().unwrap().video_id = value.unwrap_or("").to_string();
                    },
                "snippet.is-draft" => {
                        request_snippet_init(&mut request);
                        request.snippet.as_mut().unwrap().is_draft = arg_from_str(value.unwrap_or("false"), err, "snippet.is-draft", "boolean");
                    },
                "snippet.is-auto-synced" => {
                        request_snippet_init(&mut request);
                        request.snippet.as_mut().unwrap().is_auto_synced = arg_from_str(value.unwrap_or("false"), err, "snippet.is-auto-synced", "boolean");
                    },
                "snippet.track-kind" => {
                        request_snippet_init(&mut request);
                        request.snippet.as_mut().unwrap().track_kind = value.unwrap_or("").to_string();
                    },
                "snippet.last-updated" => {
                        request_snippet_init(&mut request);
                        request.snippet.as_mut().unwrap().last_updated = value.unwrap_or("").to_string();
                    },
                "snippet.is-cc" => {
                        request_snippet_init(&mut request);
                        request.snippet.as_mut().unwrap().is_cc = arg_from_str(value.unwrap_or("false"), err, "snippet.is-cc", "boolean");
                    },
                "snippet.is-easy-reader" => {
                        request_snippet_init(&mut request);
                        request.snippet.as_mut().unwrap().is_easy_reader = arg_from_str(value.unwrap_or("false"), err, "snippet.is-easy-reader", "boolean");
                    },
                "snippet.is-large" => {
                        request_snippet_init(&mut request);
                        request.snippet.as_mut().unwrap().is_large = arg_from_str(value.unwrap_or("false"), err, "snippet.is-large", "boolean");
                    },
                "snippet.failure-reason" => {
                        request_snippet_init(&mut request);
                        request.snippet.as_mut().unwrap().failure_reason = value.unwrap_or("").to_string();
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
                    err.issues.push(CLIError::Field(FieldError::Unknown(field_name.to_string())));
                }
            }
        }
        let protocol = 
            if self.opt.cmd_simple {
                "simple"
            } else if self.opt.cmd_resumable {
                "resumable"
            } else { 
                unreachable!() 
            };
        let mut input_file = input_file_from_opts(&self.opt.arg_file, err);
        let mime_type = input_mime_from_opts(&self.opt.arg_mime, err);
        if dry_run {
            None
        } else {
            assert!(err.issues.len() == 0);
            let mut ostream = writer_from_opts(self.opt.flag_o, &self.opt.arg_out);
            match match protocol {
                "simple" => call.upload(input_file.unwrap(), mime_type.unwrap()),
                "resumable" => call.upload_resumable(input_file.unwrap(), mime_type.unwrap()),
                _ => unreachable!(),
            } {
                Err(api_err) => Some(api_err),
                Ok((mut response, output_schema)) => {
                    println!("DEBUG: REMOVE ME {:?}", response);
                    serde::json::to_writer(&mut ostream, &output_schema).unwrap();
                    None
                }
            }
        }
    }

    fn _captions_list(&self, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Option<api::Error> {
        let mut call = self.hub.captions().list(&self.opt.arg_part, &self.opt.arg_video_id);
        for parg in self.opt.arg_v.iter() {
            let (key, value) = parse_kv_arg(&*parg, err);
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
                "alt"
                |"fields"
                |"key"
                |"oauth-token"
                |"pretty-print"
                |"quota-user"
                |"user-ip" => {
                    let map = [
                        ("oauth-token", "oauth_token"),
                        ("pretty-print", "prettyPrint"),
                        ("quota-user", "quotaUser"),
                        ("user-ip", "userIp"),
                    ];
                    call = call.param(map.iter().find(|t| t.0 == key).unwrap_or(&("", key)).1, value.unwrap_or("unset"))
                },
                _ => err.issues.push(CLIError::UnknownParameter(key.to_string())),
            }
        }
        let protocol = "standard-request";
        if dry_run {
            None
        } else {
            assert!(err.issues.len() == 0);
            let mut ostream = writer_from_opts(self.opt.flag_o, &self.opt.arg_out);
            match match protocol {
                "standard-request" => call.doit(),
                _ => unreachable!(),
            } {
                Err(api_err) => Some(api_err),
                Ok((mut response, output_schema)) => {
                    println!("DEBUG: REMOVE ME {:?}", response);
                    serde::json::to_writer(&mut ostream, &output_schema).unwrap();
                    None
                }
            }
        }
    }

    fn _captions_update(&self, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Option<api::Error> {
            let mut request: api::Caption = Default::default();
        let mut call = self.hub.captions().update(&request);
        for parg in self.opt.arg_v.iter() {
            let (key, value) = parse_kv_arg(&*parg, err);
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
                "alt"
                |"fields"
                |"key"
                |"oauth-token"
                |"pretty-print"
                |"quota-user"
                |"user-ip" => {
                    let map = [
                        ("oauth-token", "oauth_token"),
                        ("pretty-print", "prettyPrint"),
                        ("quota-user", "quotaUser"),
                        ("user-ip", "userIp"),
                    ];
                    call = call.param(map.iter().find(|t| t.0 == key).unwrap_or(&("", key)).1, value.unwrap_or("unset"))
                },
                _ => err.issues.push(CLIError::UnknownParameter(key.to_string())),
            }
        }
        let mut field_name: FieldCursor = Default::default();
        for kvarg in self.opt.arg_kv.iter() {
            let (key, value) = parse_kv_arg(&*kvarg, err);
            if let Err(field_err) = field_name.set(&*key) {
                err.issues.push(field_err);
            }
            fn request_snippet_init(request: &mut api::Caption) {
                if request.snippet.is_none() {
                    request.snippet = Some(Default::default());
                }
            }
            
            match &field_name.to_string()[..] {
                "snippet.status" => {
                        request_snippet_init(&mut request);
                        request.snippet.as_mut().unwrap().status = value.unwrap_or("").to_string();
                    },
                "snippet.audio-track-type" => {
                        request_snippet_init(&mut request);
                        request.snippet.as_mut().unwrap().audio_track_type = value.unwrap_or("").to_string();
                    },
                "snippet.language" => {
                        request_snippet_init(&mut request);
                        request.snippet.as_mut().unwrap().language = value.unwrap_or("").to_string();
                    },
                "snippet.name" => {
                        request_snippet_init(&mut request);
                        request.snippet.as_mut().unwrap().name = value.unwrap_or("").to_string();
                    },
                "snippet.video-id" => {
                        request_snippet_init(&mut request);
                        request.snippet.as_mut().unwrap().video_id = value.unwrap_or("").to_string();
                    },
                "snippet.is-draft" => {
                        request_snippet_init(&mut request);
                        request.snippet.as_mut().unwrap().is_draft = arg_from_str(value.unwrap_or("false"), err, "snippet.is-draft", "boolean");
                    },
                "snippet.is-auto-synced" => {
                        request_snippet_init(&mut request);
                        request.snippet.as_mut().unwrap().is_auto_synced = arg_from_str(value.unwrap_or("false"), err, "snippet.is-auto-synced", "boolean");
                    },
                "snippet.track-kind" => {
                        request_snippet_init(&mut request);
                        request.snippet.as_mut().unwrap().track_kind = value.unwrap_or("").to_string();
                    },
                "snippet.last-updated" => {
                        request_snippet_init(&mut request);
                        request.snippet.as_mut().unwrap().last_updated = value.unwrap_or("").to_string();
                    },
                "snippet.is-cc" => {
                        request_snippet_init(&mut request);
                        request.snippet.as_mut().unwrap().is_cc = arg_from_str(value.unwrap_or("false"), err, "snippet.is-cc", "boolean");
                    },
                "snippet.is-easy-reader" => {
                        request_snippet_init(&mut request);
                        request.snippet.as_mut().unwrap().is_easy_reader = arg_from_str(value.unwrap_or("false"), err, "snippet.is-easy-reader", "boolean");
                    },
                "snippet.is-large" => {
                        request_snippet_init(&mut request);
                        request.snippet.as_mut().unwrap().is_large = arg_from_str(value.unwrap_or("false"), err, "snippet.is-large", "boolean");
                    },
                "snippet.failure-reason" => {
                        request_snippet_init(&mut request);
                        request.snippet.as_mut().unwrap().failure_reason = value.unwrap_or("").to_string();
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
                    err.issues.push(CLIError::Field(FieldError::Unknown(field_name.to_string())));
                }
            }
        }
        let protocol = 
            if self.opt.cmd_simple {
                "simple"
            } else if self.opt.cmd_resumable {
                "resumable"
            } else { 
                unreachable!() 
            };
        let mut input_file = input_file_from_opts(&self.opt.arg_file, err);
        let mime_type = input_mime_from_opts(&self.opt.arg_mime, err);
        if dry_run {
            None
        } else {
            assert!(err.issues.len() == 0);
            let mut ostream = writer_from_opts(self.opt.flag_o, &self.opt.arg_out);
            match match protocol {
                "simple" => call.upload(input_file.unwrap(), mime_type.unwrap()),
                "resumable" => call.upload_resumable(input_file.unwrap(), mime_type.unwrap()),
                _ => unreachable!(),
            } {
                Err(api_err) => Some(api_err),
                Ok((mut response, output_schema)) => {
                    println!("DEBUG: REMOVE ME {:?}", response);
                    serde::json::to_writer(&mut ostream, &output_schema).unwrap();
                    None
                }
            }
        }
    }

    fn _channel_banners_insert(&self, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Option<api::Error> {
            let mut request: api::ChannelBannerResource = Default::default();
        let mut call = self.hub.channel_banners().insert(&request);
        for parg in self.opt.arg_v.iter() {
            let (key, value) = parse_kv_arg(&*parg, err);
            match key {
                "on-behalf-of-content-owner" => {
                    call = call.on_behalf_of_content_owner(value.unwrap_or(""));
                },
                "alt"
                |"fields"
                |"key"
                |"oauth-token"
                |"pretty-print"
                |"quota-user"
                |"user-ip" => {
                    let map = [
                        ("oauth-token", "oauth_token"),
                        ("pretty-print", "prettyPrint"),
                        ("quota-user", "quotaUser"),
                        ("user-ip", "userIp"),
                    ];
                    call = call.param(map.iter().find(|t| t.0 == key).unwrap_or(&("", key)).1, value.unwrap_or("unset"))
                },
                _ => err.issues.push(CLIError::UnknownParameter(key.to_string())),
            }
        }
        let mut field_name: FieldCursor = Default::default();
        for kvarg in self.opt.arg_kv.iter() {
            let (key, value) = parse_kv_arg(&*kvarg, err);
            if let Err(field_err) = field_name.set(&*key) {
                err.issues.push(field_err);
            }
            match &field_name.to_string()[..] {
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
                    err.issues.push(CLIError::Field(FieldError::Unknown(field_name.to_string())));
                }
            }
        }
        let protocol = 
            if self.opt.cmd_simple {
                "simple"
            } else if self.opt.cmd_resumable {
                "resumable"
            } else { 
                unreachable!() 
            };
        let mut input_file = input_file_from_opts(&self.opt.arg_file, err);
        let mime_type = input_mime_from_opts(&self.opt.arg_mime, err);
        if dry_run {
            None
        } else {
            assert!(err.issues.len() == 0);
            let mut ostream = writer_from_opts(self.opt.flag_o, &self.opt.arg_out);
            match match protocol {
                "simple" => call.upload(input_file.unwrap(), mime_type.unwrap()),
                "resumable" => call.upload_resumable(input_file.unwrap(), mime_type.unwrap()),
                _ => unreachable!(),
            } {
                Err(api_err) => Some(api_err),
                Ok((mut response, output_schema)) => {
                    println!("DEBUG: REMOVE ME {:?}", response);
                    serde::json::to_writer(&mut ostream, &output_schema).unwrap();
                    None
                }
            }
        }
    }

    fn _channel_sections_delete(&self, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Option<api::Error> {
        let mut call = self.hub.channel_sections().delete(&self.opt.arg_id);
        for parg in self.opt.arg_v.iter() {
            let (key, value) = parse_kv_arg(&*parg, err);
            match key {
                "on-behalf-of-content-owner" => {
                    call = call.on_behalf_of_content_owner(value.unwrap_or(""));
                },
                "alt"
                |"fields"
                |"key"
                |"oauth-token"
                |"pretty-print"
                |"quota-user"
                |"user-ip" => {
                    let map = [
                        ("oauth-token", "oauth_token"),
                        ("pretty-print", "prettyPrint"),
                        ("quota-user", "quotaUser"),
                        ("user-ip", "userIp"),
                    ];
                    call = call.param(map.iter().find(|t| t.0 == key).unwrap_or(&("", key)).1, value.unwrap_or("unset"))
                },
                _ => err.issues.push(CLIError::UnknownParameter(key.to_string())),
            }
        }
        let protocol = "standard-request";
        if dry_run {
            None
        } else {
            assert!(err.issues.len() == 0);
            match match protocol {
                "standard-request" => call.doit(),
                _ => unreachable!(),
            } {
                Err(api_err) => Some(api_err),
                Ok(mut response) => {
                    println!("DEBUG: REMOVE ME {:?}", response);
                    None
                }
            }
        }
    }

    fn _channel_sections_insert(&self, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Option<api::Error> {
            let mut request: api::ChannelSection = Default::default();
        let mut call = self.hub.channel_sections().insert(&request);
        for parg in self.opt.arg_v.iter() {
            let (key, value) = parse_kv_arg(&*parg, err);
            match key {
                "on-behalf-of-content-owner-channel" => {
                    call = call.on_behalf_of_content_owner_channel(value.unwrap_or(""));
                },
                "on-behalf-of-content-owner" => {
                    call = call.on_behalf_of_content_owner(value.unwrap_or(""));
                },
                "alt"
                |"fields"
                |"key"
                |"oauth-token"
                |"pretty-print"
                |"quota-user"
                |"user-ip" => {
                    let map = [
                        ("oauth-token", "oauth_token"),
                        ("pretty-print", "prettyPrint"),
                        ("quota-user", "quotaUser"),
                        ("user-ip", "userIp"),
                    ];
                    call = call.param(map.iter().find(|t| t.0 == key).unwrap_or(&("", key)).1, value.unwrap_or("unset"))
                },
                _ => err.issues.push(CLIError::UnknownParameter(key.to_string())),
            }
        }
        let mut field_name: FieldCursor = Default::default();
        for kvarg in self.opt.arg_kv.iter() {
            let (key, value) = parse_kv_arg(&*kvarg, err);
            if let Err(field_err) = field_name.set(&*key) {
                err.issues.push(field_err);
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
            
            match &field_name.to_string()[..] {
                "snippet.style" => {
                        request_snippet_init(&mut request);
                        request.snippet.as_mut().unwrap().style = value.unwrap_or("").to_string();
                    },
                "snippet.title" => {
                        request_snippet_init(&mut request);
                        request.snippet.as_mut().unwrap().title = value.unwrap_or("").to_string();
                    },
                "snippet.channel-id" => {
                        request_snippet_init(&mut request);
                        request.snippet.as_mut().unwrap().channel_id = value.unwrap_or("").to_string();
                    },
                "snippet.default-language" => {
                        request_snippet_init(&mut request);
                        request.snippet.as_mut().unwrap().default_language = value.unwrap_or("").to_string();
                    },
                "snippet.position" => {
                        request_snippet_init(&mut request);
                        request.snippet.as_mut().unwrap().position = arg_from_str(value.unwrap_or("-0"), err, "snippet.position", "integer");
                    },
                "snippet.type" => {
                        request_snippet_init(&mut request);
                        request.snippet.as_mut().unwrap().type_ = value.unwrap_or("").to_string();
                    },
                "snippet.localized.title" => {
                        request_snippet_init(&mut request);
                        request.snippet.as_mut().unwrap().localized.title = value.unwrap_or("").to_string();
                    },
                "content-details.channels" => {
                        request_content_details_init(&mut request);
                        request.content_details.as_mut().unwrap().channels.push(value.unwrap_or("").to_string());
                    },
                "content-details.playlists" => {
                        request_content_details_init(&mut request);
                        request.content_details.as_mut().unwrap().playlists.push(value.unwrap_or("").to_string());
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
                    err.issues.push(CLIError::Field(FieldError::Unknown(field_name.to_string())));
                }
            }
        }
        let protocol = "standard-request";
        if dry_run {
            None
        } else {
            assert!(err.issues.len() == 0);
            let mut ostream = writer_from_opts(self.opt.flag_o, &self.opt.arg_out);
            match match protocol {
                "standard-request" => call.doit(),
                _ => unreachable!(),
            } {
                Err(api_err) => Some(api_err),
                Ok((mut response, output_schema)) => {
                    println!("DEBUG: REMOVE ME {:?}", response);
                    serde::json::to_writer(&mut ostream, &output_schema).unwrap();
                    None
                }
            }
        }
    }

    fn _channel_sections_list(&self, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Option<api::Error> {
        let mut call = self.hub.channel_sections().list(&self.opt.arg_part);
        for parg in self.opt.arg_v.iter() {
            let (key, value) = parse_kv_arg(&*parg, err);
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
                "alt"
                |"fields"
                |"key"
                |"oauth-token"
                |"pretty-print"
                |"quota-user"
                |"user-ip" => {
                    let map = [
                        ("oauth-token", "oauth_token"),
                        ("pretty-print", "prettyPrint"),
                        ("quota-user", "quotaUser"),
                        ("user-ip", "userIp"),
                    ];
                    call = call.param(map.iter().find(|t| t.0 == key).unwrap_or(&("", key)).1, value.unwrap_or("unset"))
                },
                _ => err.issues.push(CLIError::UnknownParameter(key.to_string())),
            }
        }
        let protocol = "standard-request";
        if dry_run {
            None
        } else {
            assert!(err.issues.len() == 0);
            let mut ostream = writer_from_opts(self.opt.flag_o, &self.opt.arg_out);
            match match protocol {
                "standard-request" => call.doit(),
                _ => unreachable!(),
            } {
                Err(api_err) => Some(api_err),
                Ok((mut response, output_schema)) => {
                    println!("DEBUG: REMOVE ME {:?}", response);
                    serde::json::to_writer(&mut ostream, &output_schema).unwrap();
                    None
                }
            }
        }
    }

    fn _channel_sections_update(&self, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Option<api::Error> {
            let mut request: api::ChannelSection = Default::default();
        let mut call = self.hub.channel_sections().update(&request);
        for parg in self.opt.arg_v.iter() {
            let (key, value) = parse_kv_arg(&*parg, err);
            match key {
                "on-behalf-of-content-owner" => {
                    call = call.on_behalf_of_content_owner(value.unwrap_or(""));
                },
                "alt"
                |"fields"
                |"key"
                |"oauth-token"
                |"pretty-print"
                |"quota-user"
                |"user-ip" => {
                    let map = [
                        ("oauth-token", "oauth_token"),
                        ("pretty-print", "prettyPrint"),
                        ("quota-user", "quotaUser"),
                        ("user-ip", "userIp"),
                    ];
                    call = call.param(map.iter().find(|t| t.0 == key).unwrap_or(&("", key)).1, value.unwrap_or("unset"))
                },
                _ => err.issues.push(CLIError::UnknownParameter(key.to_string())),
            }
        }
        let mut field_name: FieldCursor = Default::default();
        for kvarg in self.opt.arg_kv.iter() {
            let (key, value) = parse_kv_arg(&*kvarg, err);
            if let Err(field_err) = field_name.set(&*key) {
                err.issues.push(field_err);
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
            
            match &field_name.to_string()[..] {
                "snippet.style" => {
                        request_snippet_init(&mut request);
                        request.snippet.as_mut().unwrap().style = value.unwrap_or("").to_string();
                    },
                "snippet.title" => {
                        request_snippet_init(&mut request);
                        request.snippet.as_mut().unwrap().title = value.unwrap_or("").to_string();
                    },
                "snippet.channel-id" => {
                        request_snippet_init(&mut request);
                        request.snippet.as_mut().unwrap().channel_id = value.unwrap_or("").to_string();
                    },
                "snippet.default-language" => {
                        request_snippet_init(&mut request);
                        request.snippet.as_mut().unwrap().default_language = value.unwrap_or("").to_string();
                    },
                "snippet.position" => {
                        request_snippet_init(&mut request);
                        request.snippet.as_mut().unwrap().position = arg_from_str(value.unwrap_or("-0"), err, "snippet.position", "integer");
                    },
                "snippet.type" => {
                        request_snippet_init(&mut request);
                        request.snippet.as_mut().unwrap().type_ = value.unwrap_or("").to_string();
                    },
                "snippet.localized.title" => {
                        request_snippet_init(&mut request);
                        request.snippet.as_mut().unwrap().localized.title = value.unwrap_or("").to_string();
                    },
                "content-details.channels" => {
                        request_content_details_init(&mut request);
                        request.content_details.as_mut().unwrap().channels.push(value.unwrap_or("").to_string());
                    },
                "content-details.playlists" => {
                        request_content_details_init(&mut request);
                        request.content_details.as_mut().unwrap().playlists.push(value.unwrap_or("").to_string());
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
                    err.issues.push(CLIError::Field(FieldError::Unknown(field_name.to_string())));
                }
            }
        }
        let protocol = "standard-request";
        if dry_run {
            None
        } else {
            assert!(err.issues.len() == 0);
            let mut ostream = writer_from_opts(self.opt.flag_o, &self.opt.arg_out);
            match match protocol {
                "standard-request" => call.doit(),
                _ => unreachable!(),
            } {
                Err(api_err) => Some(api_err),
                Ok((mut response, output_schema)) => {
                    println!("DEBUG: REMOVE ME {:?}", response);
                    serde::json::to_writer(&mut ostream, &output_schema).unwrap();
                    None
                }
            }
        }
    }

    fn _channels_list(&self, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Option<api::Error> {
        let mut call = self.hub.channels().list(&self.opt.arg_part);
        for parg in self.opt.arg_v.iter() {
            let (key, value) = parse_kv_arg(&*parg, err);
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
                "alt"
                |"fields"
                |"key"
                |"oauth-token"
                |"pretty-print"
                |"quota-user"
                |"user-ip" => {
                    let map = [
                        ("oauth-token", "oauth_token"),
                        ("pretty-print", "prettyPrint"),
                        ("quota-user", "quotaUser"),
                        ("user-ip", "userIp"),
                    ];
                    call = call.param(map.iter().find(|t| t.0 == key).unwrap_or(&("", key)).1, value.unwrap_or("unset"))
                },
                _ => err.issues.push(CLIError::UnknownParameter(key.to_string())),
            }
        }
        let protocol = "standard-request";
        if dry_run {
            None
        } else {
            assert!(err.issues.len() == 0);
            let mut ostream = writer_from_opts(self.opt.flag_o, &self.opt.arg_out);
            match match protocol {
                "standard-request" => call.doit(),
                _ => unreachable!(),
            } {
                Err(api_err) => Some(api_err),
                Ok((mut response, output_schema)) => {
                    println!("DEBUG: REMOVE ME {:?}", response);
                    serde::json::to_writer(&mut ostream, &output_schema).unwrap();
                    None
                }
            }
        }
    }

    fn _channels_update(&self, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Option<api::Error> {
            let mut request: api::Channel = Default::default();
        let mut call = self.hub.channels().update(&request);
        for parg in self.opt.arg_v.iter() {
            let (key, value) = parse_kv_arg(&*parg, err);
            match key {
                "on-behalf-of-content-owner" => {
                    call = call.on_behalf_of_content_owner(value.unwrap_or(""));
                },
                "alt"
                |"fields"
                |"key"
                |"oauth-token"
                |"pretty-print"
                |"quota-user"
                |"user-ip" => {
                    let map = [
                        ("oauth-token", "oauth_token"),
                        ("pretty-print", "prettyPrint"),
                        ("quota-user", "quotaUser"),
                        ("user-ip", "userIp"),
                    ];
                    call = call.param(map.iter().find(|t| t.0 == key).unwrap_or(&("", key)).1, value.unwrap_or("unset"))
                },
                _ => err.issues.push(CLIError::UnknownParameter(key.to_string())),
            }
        }
        let mut field_name: FieldCursor = Default::default();
        for kvarg in self.opt.arg_kv.iter() {
            let (key, value) = parse_kv_arg(&*kvarg, err);
            if let Err(field_err) = field_name.set(&*key) {
                err.issues.push(field_err);
            }
            fn request_audit_details_init(request: &mut api::Channel) {
                if request.audit_details.is_none() {
                    request.audit_details = Some(Default::default());
                }
            }
            
            fn request_branding_settings_init(request: &mut api::Channel) {
                if request.branding_settings.is_none() {
                    request.branding_settings = Some(Default::default());
                }
            }
            
            fn request_content_details_init(request: &mut api::Channel) {
                if request.content_details.is_none() {
                    request.content_details = Some(Default::default());
                }
            }
            
            fn request_content_owner_details_init(request: &mut api::Channel) {
                if request.content_owner_details.is_none() {
                    request.content_owner_details = Some(Default::default());
                }
            }
            
            fn request_invideo_promotion_init(request: &mut api::Channel) {
                if request.invideo_promotion.is_none() {
                    request.invideo_promotion = Some(Default::default());
                }
            }
            
            fn request_snippet_init(request: &mut api::Channel) {
                if request.snippet.is_none() {
                    request.snippet = Some(Default::default());
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
            
            match &field_name.to_string()[..] {
                "status.privacy-status" => {
                        request_status_init(&mut request);
                        request.status.as_mut().unwrap().privacy_status = value.unwrap_or("").to_string();
                    },
                "status.is-linked" => {
                        request_status_init(&mut request);
                        request.status.as_mut().unwrap().is_linked = arg_from_str(value.unwrap_or("false"), err, "status.is-linked", "boolean");
                    },
                "status.long-uploads-status" => {
                        request_status_init(&mut request);
                        request.status.as_mut().unwrap().long_uploads_status = value.unwrap_or("").to_string();
                    },
                "invideo-promotion.default-timing.offset-ms" => {
                        request_invideo_promotion_init(&mut request);
                        request.invideo_promotion.as_mut().unwrap().default_timing.offset_ms = value.unwrap_or("").to_string();
                    },
                "invideo-promotion.default-timing.type" => {
                        request_invideo_promotion_init(&mut request);
                        request.invideo_promotion.as_mut().unwrap().default_timing.type_ = value.unwrap_or("").to_string();
                    },
                "invideo-promotion.default-timing.duration-ms" => {
                        request_invideo_promotion_init(&mut request);
                        request.invideo_promotion.as_mut().unwrap().default_timing.duration_ms = value.unwrap_or("").to_string();
                    },
                "invideo-promotion.position.corner-position" => {
                        request_invideo_promotion_init(&mut request);
                        request.invideo_promotion.as_mut().unwrap().position.corner_position = value.unwrap_or("").to_string();
                    },
                "invideo-promotion.position.type" => {
                        request_invideo_promotion_init(&mut request);
                        request.invideo_promotion.as_mut().unwrap().position.type_ = value.unwrap_or("").to_string();
                    },
                "invideo-promotion.use-smart-timing" => {
                        request_invideo_promotion_init(&mut request);
                        request.invideo_promotion.as_mut().unwrap().use_smart_timing = arg_from_str(value.unwrap_or("false"), err, "invideo-promotion.use-smart-timing", "boolean");
                    },
                "kind" => {
                        request_invideo_promotion_init(&mut request);
                        request.kind = Some(value.unwrap_or("").to_string());
                    },
                "statistics.comment-count" => {
                        request_statistics_init(&mut request);
                        request.statistics.as_mut().unwrap().comment_count = arg_from_str(value.unwrap_or("-0"), err, "statistics.comment-count", "int64");
                    },
                "statistics.subscriber-count" => {
                        request_statistics_init(&mut request);
                        request.statistics.as_mut().unwrap().subscriber_count = arg_from_str(value.unwrap_or("-0"), err, "statistics.subscriber-count", "int64");
                    },
                "statistics.video-count" => {
                        request_statistics_init(&mut request);
                        request.statistics.as_mut().unwrap().video_count = arg_from_str(value.unwrap_or("-0"), err, "statistics.video-count", "int64");
                    },
                "statistics.hidden-subscriber-count" => {
                        request_statistics_init(&mut request);
                        request.statistics.as_mut().unwrap().hidden_subscriber_count = arg_from_str(value.unwrap_or("false"), err, "statistics.hidden-subscriber-count", "boolean");
                    },
                "statistics.view-count" => {
                        request_statistics_init(&mut request);
                        request.statistics.as_mut().unwrap().view_count = arg_from_str(value.unwrap_or("-0"), err, "statistics.view-count", "int64");
                    },
                "content-owner-details.content-owner" => {
                        request_content_owner_details_init(&mut request);
                        request.content_owner_details.as_mut().unwrap().content_owner = value.unwrap_or("").to_string();
                    },
                "content-owner-details.time-linked" => {
                        request_content_owner_details_init(&mut request);
                        request.content_owner_details.as_mut().unwrap().time_linked = value.unwrap_or("").to_string();
                    },
                "topic-details.topic-ids" => {
                        request_topic_details_init(&mut request);
                        request.topic_details.as_mut().unwrap().topic_ids.push(value.unwrap_or("").to_string());
                    },
                "content-details.related-playlists.watch-later" => {
                        request_content_details_init(&mut request);
                        request.content_details.as_mut().unwrap().related_playlists.watch_later = value.unwrap_or("").to_string();
                    },
                "content-details.related-playlists.watch-history" => {
                        request_content_details_init(&mut request);
                        request.content_details.as_mut().unwrap().related_playlists.watch_history = value.unwrap_or("").to_string();
                    },
                "content-details.related-playlists.uploads" => {
                        request_content_details_init(&mut request);
                        request.content_details.as_mut().unwrap().related_playlists.uploads = value.unwrap_or("").to_string();
                    },
                "content-details.related-playlists.favorites" => {
                        request_content_details_init(&mut request);
                        request.content_details.as_mut().unwrap().related_playlists.favorites = value.unwrap_or("").to_string();
                    },
                "content-details.related-playlists.likes" => {
                        request_content_details_init(&mut request);
                        request.content_details.as_mut().unwrap().related_playlists.likes = value.unwrap_or("").to_string();
                    },
                "content-details.google-plus-user-id" => {
                        request_content_details_init(&mut request);
                        request.content_details.as_mut().unwrap().google_plus_user_id = value.unwrap_or("").to_string();
                    },
                "branding-settings.image.large-branded-banner-image-imap-script.default" => {
                        request_branding_settings_init(&mut request);
                        request.branding_settings.as_mut().unwrap().image.large_branded_banner_image_imap_script.default = value.unwrap_or("").to_string();
                    },
                "branding-settings.image.large-branded-banner-image-imap-script.default-language.value" => {
                        request_branding_settings_init(&mut request);
                        request.branding_settings.as_mut().unwrap().image.large_branded_banner_image_imap_script.default_language.value = value.unwrap_or("").to_string();
                    },
                "branding-settings.image.small-branded-banner-image-url.default" => {
                        request_branding_settings_init(&mut request);
                        request.branding_settings.as_mut().unwrap().image.small_branded_banner_image_url.default = value.unwrap_or("").to_string();
                    },
                "branding-settings.image.small-branded-banner-image-url.default-language.value" => {
                        request_branding_settings_init(&mut request);
                        request.branding_settings.as_mut().unwrap().image.small_branded_banner_image_url.default_language.value = value.unwrap_or("").to_string();
                    },
                "branding-settings.image.banner-tv-image-url" => {
                        request_branding_settings_init(&mut request);
                        request.branding_settings.as_mut().unwrap().image.banner_tv_image_url = value.unwrap_or("").to_string();
                    },
                "branding-settings.image.banner-tv-low-image-url" => {
                        request_branding_settings_init(&mut request);
                        request.branding_settings.as_mut().unwrap().image.banner_tv_low_image_url = value.unwrap_or("").to_string();
                    },
                "branding-settings.image.large-branded-banner-image-url.default" => {
                        request_branding_settings_init(&mut request);
                        request.branding_settings.as_mut().unwrap().image.large_branded_banner_image_url.default = value.unwrap_or("").to_string();
                    },
                "branding-settings.image.large-branded-banner-image-url.default-language.value" => {
                        request_branding_settings_init(&mut request);
                        request.branding_settings.as_mut().unwrap().image.large_branded_banner_image_url.default_language.value = value.unwrap_or("").to_string();
                    },
                "branding-settings.image.banner-tv-high-image-url" => {
                        request_branding_settings_init(&mut request);
                        request.branding_settings.as_mut().unwrap().image.banner_tv_high_image_url = value.unwrap_or("").to_string();
                    },
                "branding-settings.image.background-image-url.default" => {
                        request_branding_settings_init(&mut request);
                        request.branding_settings.as_mut().unwrap().image.background_image_url.default = value.unwrap_or("").to_string();
                    },
                "branding-settings.image.background-image-url.default-language.value" => {
                        request_branding_settings_init(&mut request);
                        request.branding_settings.as_mut().unwrap().image.background_image_url.default_language.value = value.unwrap_or("").to_string();
                    },
                "branding-settings.image.small-branded-banner-image-imap-script.default" => {
                        request_branding_settings_init(&mut request);
                        request.branding_settings.as_mut().unwrap().image.small_branded_banner_image_imap_script.default = value.unwrap_or("").to_string();
                    },
                "branding-settings.image.small-branded-banner-image-imap-script.default-language.value" => {
                        request_branding_settings_init(&mut request);
                        request.branding_settings.as_mut().unwrap().image.small_branded_banner_image_imap_script.default_language.value = value.unwrap_or("").to_string();
                    },
                "branding-settings.image.banner-external-url" => {
                        request_branding_settings_init(&mut request);
                        request.branding_settings.as_mut().unwrap().image.banner_external_url = value.unwrap_or("").to_string();
                    },
                "branding-settings.image.watch-icon-image-url" => {
                        request_branding_settings_init(&mut request);
                        request.branding_settings.as_mut().unwrap().image.watch_icon_image_url = value.unwrap_or("").to_string();
                    },
                "branding-settings.image.banner-tv-medium-image-url" => {
                        request_branding_settings_init(&mut request);
                        request.branding_settings.as_mut().unwrap().image.banner_tv_medium_image_url = value.unwrap_or("").to_string();
                    },
                "branding-settings.image.banner-mobile-image-url" => {
                        request_branding_settings_init(&mut request);
                        request.branding_settings.as_mut().unwrap().image.banner_mobile_image_url = value.unwrap_or("").to_string();
                    },
                "branding-settings.image.banner-tablet-hd-image-url" => {
                        request_branding_settings_init(&mut request);
                        request.branding_settings.as_mut().unwrap().image.banner_tablet_hd_image_url = value.unwrap_or("").to_string();
                    },
                "branding-settings.image.banner-tablet-low-image-url" => {
                        request_branding_settings_init(&mut request);
                        request.branding_settings.as_mut().unwrap().image.banner_tablet_low_image_url = value.unwrap_or("").to_string();
                    },
                "branding-settings.image.tracking-image-url" => {
                        request_branding_settings_init(&mut request);
                        request.branding_settings.as_mut().unwrap().image.tracking_image_url = value.unwrap_or("").to_string();
                    },
                "branding-settings.image.banner-mobile-extra-hd-image-url" => {
                        request_branding_settings_init(&mut request);
                        request.branding_settings.as_mut().unwrap().image.banner_mobile_extra_hd_image_url = value.unwrap_or("").to_string();
                    },
                "branding-settings.image.banner-tablet-image-url" => {
                        request_branding_settings_init(&mut request);
                        request.branding_settings.as_mut().unwrap().image.banner_tablet_image_url = value.unwrap_or("").to_string();
                    },
                "branding-settings.image.banner-mobile-low-image-url" => {
                        request_branding_settings_init(&mut request);
                        request.branding_settings.as_mut().unwrap().image.banner_mobile_low_image_url = value.unwrap_or("").to_string();
                    },
                "branding-settings.image.banner-mobile-medium-hd-image-url" => {
                        request_branding_settings_init(&mut request);
                        request.branding_settings.as_mut().unwrap().image.banner_mobile_medium_hd_image_url = value.unwrap_or("").to_string();
                    },
                "branding-settings.image.banner-tablet-extra-hd-image-url" => {
                        request_branding_settings_init(&mut request);
                        request.branding_settings.as_mut().unwrap().image.banner_tablet_extra_hd_image_url = value.unwrap_or("").to_string();
                    },
                "branding-settings.image.banner-image-url" => {
                        request_branding_settings_init(&mut request);
                        request.branding_settings.as_mut().unwrap().image.banner_image_url = value.unwrap_or("").to_string();
                    },
                "branding-settings.image.banner-mobile-hd-image-url" => {
                        request_branding_settings_init(&mut request);
                        request.branding_settings.as_mut().unwrap().image.banner_mobile_hd_image_url = value.unwrap_or("").to_string();
                    },
                "branding-settings.watch.text-color" => {
                        request_branding_settings_init(&mut request);
                        request.branding_settings.as_mut().unwrap().watch.text_color = value.unwrap_or("").to_string();
                    },
                "branding-settings.watch.featured-playlist-id" => {
                        request_branding_settings_init(&mut request);
                        request.branding_settings.as_mut().unwrap().watch.featured_playlist_id = value.unwrap_or("").to_string();
                    },
                "branding-settings.watch.background-color" => {
                        request_branding_settings_init(&mut request);
                        request.branding_settings.as_mut().unwrap().watch.background_color = value.unwrap_or("").to_string();
                    },
                "branding-settings.channel.description" => {
                        request_branding_settings_init(&mut request);
                        request.branding_settings.as_mut().unwrap().channel.description = value.unwrap_or("").to_string();
                    },
                "branding-settings.channel.title" => {
                        request_branding_settings_init(&mut request);
                        request.branding_settings.as_mut().unwrap().channel.title = value.unwrap_or("").to_string();
                    },
                "branding-settings.channel.moderate-comments" => {
                        request_branding_settings_init(&mut request);
                        request.branding_settings.as_mut().unwrap().channel.moderate_comments = arg_from_str(value.unwrap_or("false"), err, "branding-settings.channel.moderate-comments", "boolean");
                    },
                "branding-settings.channel.show-browse-view" => {
                        request_branding_settings_init(&mut request);
                        request.branding_settings.as_mut().unwrap().channel.show_browse_view = arg_from_str(value.unwrap_or("false"), err, "branding-settings.channel.show-browse-view", "boolean");
                    },
                "branding-settings.channel.featured-channels-title" => {
                        request_branding_settings_init(&mut request);
                        request.branding_settings.as_mut().unwrap().channel.featured_channels_title = value.unwrap_or("").to_string();
                    },
                "branding-settings.channel.default-language" => {
                        request_branding_settings_init(&mut request);
                        request.branding_settings.as_mut().unwrap().channel.default_language = value.unwrap_or("").to_string();
                    },
                "branding-settings.channel.unsubscribed-trailer" => {
                        request_branding_settings_init(&mut request);
                        request.branding_settings.as_mut().unwrap().channel.unsubscribed_trailer = value.unwrap_or("").to_string();
                    },
                "branding-settings.channel.keywords" => {
                        request_branding_settings_init(&mut request);
                        request.branding_settings.as_mut().unwrap().channel.keywords = value.unwrap_or("").to_string();
                    },
                "branding-settings.channel.profile-color" => {
                        request_branding_settings_init(&mut request);
                        request.branding_settings.as_mut().unwrap().channel.profile_color = value.unwrap_or("").to_string();
                    },
                "branding-settings.channel.default-tab" => {
                        request_branding_settings_init(&mut request);
                        request.branding_settings.as_mut().unwrap().channel.default_tab = value.unwrap_or("").to_string();
                    },
                "branding-settings.channel.featured-channels-urls" => {
                        request_branding_settings_init(&mut request);
                        request.branding_settings.as_mut().unwrap().channel.featured_channels_urls.push(value.unwrap_or("").to_string());
                    },
                "branding-settings.channel.tracking-analytics-account-id" => {
                        request_branding_settings_init(&mut request);
                        request.branding_settings.as_mut().unwrap().channel.tracking_analytics_account_id = value.unwrap_or("").to_string();
                    },
                "branding-settings.channel.show-related-channels" => {
                        request_branding_settings_init(&mut request);
                        request.branding_settings.as_mut().unwrap().channel.show_related_channels = arg_from_str(value.unwrap_or("false"), err, "branding-settings.channel.show-related-channels", "boolean");
                    },
                "snippet.thumbnails.default.url" => {
                        request_snippet_init(&mut request);
                        request.snippet.as_mut().unwrap().thumbnails.default.url = Some(value.unwrap_or("").to_string());
                    },
                "snippet.thumbnails.default.width" => {
                        request_snippet_init(&mut request);
                        request.snippet.as_mut().unwrap().thumbnails.default.width = Some(arg_from_str(value.unwrap_or("-0"), err, "snippet.thumbnails.default.width", "integer"));
                    },
                "snippet.thumbnails.default.height" => {
                        request_snippet_init(&mut request);
                        request.snippet.as_mut().unwrap().thumbnails.default.height = Some(arg_from_str(value.unwrap_or("-0"), err, "snippet.thumbnails.default.height", "integer"));
                    },
                "snippet.thumbnails.high.url" => {
                        request_snippet_init(&mut request);
                        request.snippet.as_mut().unwrap().thumbnails.high.url = Some(value.unwrap_or("").to_string());
                    },
                "snippet.thumbnails.high.width" => {
                        request_snippet_init(&mut request);
                        request.snippet.as_mut().unwrap().thumbnails.high.width = Some(arg_from_str(value.unwrap_or("-0"), err, "snippet.thumbnails.high.width", "integer"));
                    },
                "snippet.thumbnails.high.height" => {
                        request_snippet_init(&mut request);
                        request.snippet.as_mut().unwrap().thumbnails.high.height = Some(arg_from_str(value.unwrap_or("-0"), err, "snippet.thumbnails.high.height", "integer"));
                    },
                "snippet.thumbnails.medium.url" => {
                        request_snippet_init(&mut request);
                        request.snippet.as_mut().unwrap().thumbnails.medium.url = Some(value.unwrap_or("").to_string());
                    },
                "snippet.thumbnails.medium.width" => {
                        request_snippet_init(&mut request);
                        request.snippet.as_mut().unwrap().thumbnails.medium.width = Some(arg_from_str(value.unwrap_or("-0"), err, "snippet.thumbnails.medium.width", "integer"));
                    },
                "snippet.thumbnails.medium.height" => {
                        request_snippet_init(&mut request);
                        request.snippet.as_mut().unwrap().thumbnails.medium.height = Some(arg_from_str(value.unwrap_or("-0"), err, "snippet.thumbnails.medium.height", "integer"));
                    },
                "snippet.thumbnails.maxres.url" => {
                        request_snippet_init(&mut request);
                        request.snippet.as_mut().unwrap().thumbnails.maxres.url = Some(value.unwrap_or("").to_string());
                    },
                "snippet.thumbnails.maxres.width" => {
                        request_snippet_init(&mut request);
                        request.snippet.as_mut().unwrap().thumbnails.maxres.width = Some(arg_from_str(value.unwrap_or("-0"), err, "snippet.thumbnails.maxres.width", "integer"));
                    },
                "snippet.thumbnails.maxres.height" => {
                        request_snippet_init(&mut request);
                        request.snippet.as_mut().unwrap().thumbnails.maxres.height = Some(arg_from_str(value.unwrap_or("-0"), err, "snippet.thumbnails.maxres.height", "integer"));
                    },
                "snippet.thumbnails.standard.url" => {
                        request_snippet_init(&mut request);
                        request.snippet.as_mut().unwrap().thumbnails.standard.url = Some(value.unwrap_or("").to_string());
                    },
                "snippet.thumbnails.standard.width" => {
                        request_snippet_init(&mut request);
                        request.snippet.as_mut().unwrap().thumbnails.standard.width = Some(arg_from_str(value.unwrap_or("-0"), err, "snippet.thumbnails.standard.width", "integer"));
                    },
                "snippet.thumbnails.standard.height" => {
                        request_snippet_init(&mut request);
                        request.snippet.as_mut().unwrap().thumbnails.standard.height = Some(arg_from_str(value.unwrap_or("-0"), err, "snippet.thumbnails.standard.height", "integer"));
                    },
                "snippet.title" => {
                        request_snippet_init(&mut request);
                        request.snippet.as_mut().unwrap().title = value.unwrap_or("").to_string();
                    },
                "snippet.default-language" => {
                        request_snippet_init(&mut request);
                        request.snippet.as_mut().unwrap().default_language = value.unwrap_or("").to_string();
                    },
                "snippet.published-at" => {
                        request_snippet_init(&mut request);
                        request.snippet.as_mut().unwrap().published_at = value.unwrap_or("").to_string();
                    },
                "snippet.localized.description" => {
                        request_snippet_init(&mut request);
                        request.snippet.as_mut().unwrap().localized.description = value.unwrap_or("").to_string();
                    },
                "snippet.localized.title" => {
                        request_snippet_init(&mut request);
                        request.snippet.as_mut().unwrap().localized.title = value.unwrap_or("").to_string();
                    },
                "snippet.description" => {
                        request_snippet_init(&mut request);
                        request.snippet.as_mut().unwrap().description = value.unwrap_or("").to_string();
                    },
                "audit-details.community-guidelines-good-standing" => {
                        request_audit_details_init(&mut request);
                        request.audit_details.as_mut().unwrap().community_guidelines_good_standing = arg_from_str(value.unwrap_or("false"), err, "audit-details.community-guidelines-good-standing", "boolean");
                    },
                "audit-details.content-id-claims-good-standing" => {
                        request_audit_details_init(&mut request);
                        request.audit_details.as_mut().unwrap().content_id_claims_good_standing = arg_from_str(value.unwrap_or("false"), err, "audit-details.content-id-claims-good-standing", "boolean");
                    },
                "audit-details.overall-good-standing" => {
                        request_audit_details_init(&mut request);
                        request.audit_details.as_mut().unwrap().overall_good_standing = arg_from_str(value.unwrap_or("false"), err, "audit-details.overall-good-standing", "boolean");
                    },
                "audit-details.copyright-strikes-good-standing" => {
                        request_audit_details_init(&mut request);
                        request.audit_details.as_mut().unwrap().copyright_strikes_good_standing = arg_from_str(value.unwrap_or("false"), err, "audit-details.copyright-strikes-good-standing", "boolean");
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
                    err.issues.push(CLIError::Field(FieldError::Unknown(field_name.to_string())));
                }
            }
        }
        let protocol = "standard-request";
        if dry_run {
            None
        } else {
            assert!(err.issues.len() == 0);
            let mut ostream = writer_from_opts(self.opt.flag_o, &self.opt.arg_out);
            match match protocol {
                "standard-request" => call.doit(),
                _ => unreachable!(),
            } {
                Err(api_err) => Some(api_err),
                Ok((mut response, output_schema)) => {
                    println!("DEBUG: REMOVE ME {:?}", response);
                    serde::json::to_writer(&mut ostream, &output_schema).unwrap();
                    None
                }
            }
        }
    }

    fn _guide_categories_list(&self, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Option<api::Error> {
        let mut call = self.hub.guide_categories().list(&self.opt.arg_part);
        for parg in self.opt.arg_v.iter() {
            let (key, value) = parse_kv_arg(&*parg, err);
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
                "alt"
                |"fields"
                |"key"
                |"oauth-token"
                |"pretty-print"
                |"quota-user"
                |"user-ip" => {
                    let map = [
                        ("oauth-token", "oauth_token"),
                        ("pretty-print", "prettyPrint"),
                        ("quota-user", "quotaUser"),
                        ("user-ip", "userIp"),
                    ];
                    call = call.param(map.iter().find(|t| t.0 == key).unwrap_or(&("", key)).1, value.unwrap_or("unset"))
                },
                _ => err.issues.push(CLIError::UnknownParameter(key.to_string())),
            }
        }
        let protocol = "standard-request";
        if dry_run {
            None
        } else {
            assert!(err.issues.len() == 0);
            let mut ostream = writer_from_opts(self.opt.flag_o, &self.opt.arg_out);
            match match protocol {
                "standard-request" => call.doit(),
                _ => unreachable!(),
            } {
                Err(api_err) => Some(api_err),
                Ok((mut response, output_schema)) => {
                    println!("DEBUG: REMOVE ME {:?}", response);
                    serde::json::to_writer(&mut ostream, &output_schema).unwrap();
                    None
                }
            }
        }
    }

    fn _i18n_languages_list(&self, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Option<api::Error> {
        let mut call = self.hub.i18n_languages().list(&self.opt.arg_part);
        for parg in self.opt.arg_v.iter() {
            let (key, value) = parse_kv_arg(&*parg, err);
            match key {
                "hl" => {
                    call = call.hl(value.unwrap_or(""));
                },
                "alt"
                |"fields"
                |"key"
                |"oauth-token"
                |"pretty-print"
                |"quota-user"
                |"user-ip" => {
                    let map = [
                        ("oauth-token", "oauth_token"),
                        ("pretty-print", "prettyPrint"),
                        ("quota-user", "quotaUser"),
                        ("user-ip", "userIp"),
                    ];
                    call = call.param(map.iter().find(|t| t.0 == key).unwrap_or(&("", key)).1, value.unwrap_or("unset"))
                },
                _ => err.issues.push(CLIError::UnknownParameter(key.to_string())),
            }
        }
        let protocol = "standard-request";
        if dry_run {
            None
        } else {
            assert!(err.issues.len() == 0);
            let mut ostream = writer_from_opts(self.opt.flag_o, &self.opt.arg_out);
            match match protocol {
                "standard-request" => call.doit(),
                _ => unreachable!(),
            } {
                Err(api_err) => Some(api_err),
                Ok((mut response, output_schema)) => {
                    println!("DEBUG: REMOVE ME {:?}", response);
                    serde::json::to_writer(&mut ostream, &output_schema).unwrap();
                    None
                }
            }
        }
    }

    fn _i18n_regions_list(&self, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Option<api::Error> {
        let mut call = self.hub.i18n_regions().list(&self.opt.arg_part);
        for parg in self.opt.arg_v.iter() {
            let (key, value) = parse_kv_arg(&*parg, err);
            match key {
                "hl" => {
                    call = call.hl(value.unwrap_or(""));
                },
                "alt"
                |"fields"
                |"key"
                |"oauth-token"
                |"pretty-print"
                |"quota-user"
                |"user-ip" => {
                    let map = [
                        ("oauth-token", "oauth_token"),
                        ("pretty-print", "prettyPrint"),
                        ("quota-user", "quotaUser"),
                        ("user-ip", "userIp"),
                    ];
                    call = call.param(map.iter().find(|t| t.0 == key).unwrap_or(&("", key)).1, value.unwrap_or("unset"))
                },
                _ => err.issues.push(CLIError::UnknownParameter(key.to_string())),
            }
        }
        let protocol = "standard-request";
        if dry_run {
            None
        } else {
            assert!(err.issues.len() == 0);
            let mut ostream = writer_from_opts(self.opt.flag_o, &self.opt.arg_out);
            match match protocol {
                "standard-request" => call.doit(),
                _ => unreachable!(),
            } {
                Err(api_err) => Some(api_err),
                Ok((mut response, output_schema)) => {
                    println!("DEBUG: REMOVE ME {:?}", response);
                    serde::json::to_writer(&mut ostream, &output_schema).unwrap();
                    None
                }
            }
        }
    }

    fn _live_broadcasts_bind(&self, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Option<api::Error> {
        let mut call = self.hub.live_broadcasts().bind(&self.opt.arg_id, &self.opt.arg_part);
        for parg in self.opt.arg_v.iter() {
            let (key, value) = parse_kv_arg(&*parg, err);
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
                "alt"
                |"fields"
                |"key"
                |"oauth-token"
                |"pretty-print"
                |"quota-user"
                |"user-ip" => {
                    let map = [
                        ("oauth-token", "oauth_token"),
                        ("pretty-print", "prettyPrint"),
                        ("quota-user", "quotaUser"),
                        ("user-ip", "userIp"),
                    ];
                    call = call.param(map.iter().find(|t| t.0 == key).unwrap_or(&("", key)).1, value.unwrap_or("unset"))
                },
                _ => err.issues.push(CLIError::UnknownParameter(key.to_string())),
            }
        }
        let protocol = "standard-request";
        if dry_run {
            None
        } else {
            assert!(err.issues.len() == 0);
            let mut ostream = writer_from_opts(self.opt.flag_o, &self.opt.arg_out);
            match match protocol {
                "standard-request" => call.doit(),
                _ => unreachable!(),
            } {
                Err(api_err) => Some(api_err),
                Ok((mut response, output_schema)) => {
                    println!("DEBUG: REMOVE ME {:?}", response);
                    serde::json::to_writer(&mut ostream, &output_schema).unwrap();
                    None
                }
            }
        }
    }

    fn _live_broadcasts_control(&self, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Option<api::Error> {
        let mut call = self.hub.live_broadcasts().control(&self.opt.arg_id, &self.opt.arg_part);
        for parg in self.opt.arg_v.iter() {
            let (key, value) = parse_kv_arg(&*parg, err);
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
                "alt"
                |"fields"
                |"key"
                |"oauth-token"
                |"pretty-print"
                |"quota-user"
                |"user-ip" => {
                    let map = [
                        ("oauth-token", "oauth_token"),
                        ("pretty-print", "prettyPrint"),
                        ("quota-user", "quotaUser"),
                        ("user-ip", "userIp"),
                    ];
                    call = call.param(map.iter().find(|t| t.0 == key).unwrap_or(&("", key)).1, value.unwrap_or("unset"))
                },
                _ => err.issues.push(CLIError::UnknownParameter(key.to_string())),
            }
        }
        let protocol = "standard-request";
        if dry_run {
            None
        } else {
            assert!(err.issues.len() == 0);
            let mut ostream = writer_from_opts(self.opt.flag_o, &self.opt.arg_out);
            match match protocol {
                "standard-request" => call.doit(),
                _ => unreachable!(),
            } {
                Err(api_err) => Some(api_err),
                Ok((mut response, output_schema)) => {
                    println!("DEBUG: REMOVE ME {:?}", response);
                    serde::json::to_writer(&mut ostream, &output_schema).unwrap();
                    None
                }
            }
        }
    }

    fn _live_broadcasts_delete(&self, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Option<api::Error> {
        let mut call = self.hub.live_broadcasts().delete(&self.opt.arg_id);
        for parg in self.opt.arg_v.iter() {
            let (key, value) = parse_kv_arg(&*parg, err);
            match key {
                "on-behalf-of-content-owner-channel" => {
                    call = call.on_behalf_of_content_owner_channel(value.unwrap_or(""));
                },
                "on-behalf-of-content-owner" => {
                    call = call.on_behalf_of_content_owner(value.unwrap_or(""));
                },
                "alt"
                |"fields"
                |"key"
                |"oauth-token"
                |"pretty-print"
                |"quota-user"
                |"user-ip" => {
                    let map = [
                        ("oauth-token", "oauth_token"),
                        ("pretty-print", "prettyPrint"),
                        ("quota-user", "quotaUser"),
                        ("user-ip", "userIp"),
                    ];
                    call = call.param(map.iter().find(|t| t.0 == key).unwrap_or(&("", key)).1, value.unwrap_or("unset"))
                },
                _ => err.issues.push(CLIError::UnknownParameter(key.to_string())),
            }
        }
        let protocol = "standard-request";
        if dry_run {
            None
        } else {
            assert!(err.issues.len() == 0);
            match match protocol {
                "standard-request" => call.doit(),
                _ => unreachable!(),
            } {
                Err(api_err) => Some(api_err),
                Ok(mut response) => {
                    println!("DEBUG: REMOVE ME {:?}", response);
                    None
                }
            }
        }
    }

    fn _live_broadcasts_insert(&self, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Option<api::Error> {
            let mut request: api::LiveBroadcast = Default::default();
        let mut call = self.hub.live_broadcasts().insert(&request);
        for parg in self.opt.arg_v.iter() {
            let (key, value) = parse_kv_arg(&*parg, err);
            match key {
                "on-behalf-of-content-owner-channel" => {
                    call = call.on_behalf_of_content_owner_channel(value.unwrap_or(""));
                },
                "on-behalf-of-content-owner" => {
                    call = call.on_behalf_of_content_owner(value.unwrap_or(""));
                },
                "alt"
                |"fields"
                |"key"
                |"oauth-token"
                |"pretty-print"
                |"quota-user"
                |"user-ip" => {
                    let map = [
                        ("oauth-token", "oauth_token"),
                        ("pretty-print", "prettyPrint"),
                        ("quota-user", "quotaUser"),
                        ("user-ip", "userIp"),
                    ];
                    call = call.param(map.iter().find(|t| t.0 == key).unwrap_or(&("", key)).1, value.unwrap_or("unset"))
                },
                _ => err.issues.push(CLIError::UnknownParameter(key.to_string())),
            }
        }
        let mut field_name: FieldCursor = Default::default();
        for kvarg in self.opt.arg_kv.iter() {
            let (key, value) = parse_kv_arg(&*kvarg, err);
            if let Err(field_err) = field_name.set(&*key) {
                err.issues.push(field_err);
            }
            fn request_content_details_init(request: &mut api::LiveBroadcast) {
                if request.content_details.is_none() {
                    request.content_details = Some(Default::default());
                }
            }
            
            fn request_snippet_init(request: &mut api::LiveBroadcast) {
                if request.snippet.is_none() {
                    request.snippet = Some(Default::default());
                }
            }
            
            fn request_status_init(request: &mut api::LiveBroadcast) {
                if request.status.is_none() {
                    request.status = Some(Default::default());
                }
            }
            
            match &field_name.to_string()[..] {
                "status.recording-status" => {
                        request_status_init(&mut request);
                        request.status.as_mut().unwrap().recording_status = value.unwrap_or("").to_string();
                    },
                "status.privacy-status" => {
                        request_status_init(&mut request);
                        request.status.as_mut().unwrap().privacy_status = value.unwrap_or("").to_string();
                    },
                "status.life-cycle-status" => {
                        request_status_init(&mut request);
                        request.status.as_mut().unwrap().life_cycle_status = value.unwrap_or("").to_string();
                    },
                "status.live-broadcast-priority" => {
                        request_status_init(&mut request);
                        request.status.as_mut().unwrap().live_broadcast_priority = value.unwrap_or("").to_string();
                    },
                "kind" => {
                        request_status_init(&mut request);
                        request.kind = Some(value.unwrap_or("").to_string());
                    },
                "content-details.start-with-slate" => {
                        request_content_details_init(&mut request);
                        request.content_details.as_mut().unwrap().start_with_slate = arg_from_str(value.unwrap_or("false"), err, "content-details.start-with-slate", "boolean");
                    },
                "content-details.bound-stream-id" => {
                        request_content_details_init(&mut request);
                        request.content_details.as_mut().unwrap().bound_stream_id = value.unwrap_or("").to_string();
                    },
                "content-details.enable-embed" => {
                        request_content_details_init(&mut request);
                        request.content_details.as_mut().unwrap().enable_embed = arg_from_str(value.unwrap_or("false"), err, "content-details.enable-embed", "boolean");
                    },
                "content-details.enable-closed-captions" => {
                        request_content_details_init(&mut request);
                        request.content_details.as_mut().unwrap().enable_closed_captions = arg_from_str(value.unwrap_or("false"), err, "content-details.enable-closed-captions", "boolean");
                    },
                "content-details.enable-content-encryption" => {
                        request_content_details_init(&mut request);
                        request.content_details.as_mut().unwrap().enable_content_encryption = arg_from_str(value.unwrap_or("false"), err, "content-details.enable-content-encryption", "boolean");
                    },
                "content-details.record-from-start" => {
                        request_content_details_init(&mut request);
                        request.content_details.as_mut().unwrap().record_from_start = arg_from_str(value.unwrap_or("false"), err, "content-details.record-from-start", "boolean");
                    },
                "content-details.enable-dvr" => {
                        request_content_details_init(&mut request);
                        request.content_details.as_mut().unwrap().enable_dvr = arg_from_str(value.unwrap_or("false"), err, "content-details.enable-dvr", "boolean");
                    },
                "content-details.monitor-stream.broadcast-stream-delay-ms" => {
                        request_content_details_init(&mut request);
                        request.content_details.as_mut().unwrap().monitor_stream.broadcast_stream_delay_ms = arg_from_str(value.unwrap_or("-0"), err, "content-details.monitor-stream.broadcast-stream-delay-ms", "integer");
                    },
                "content-details.monitor-stream.embed-html" => {
                        request_content_details_init(&mut request);
                        request.content_details.as_mut().unwrap().monitor_stream.embed_html = value.unwrap_or("").to_string();
                    },
                "content-details.monitor-stream.enable-monitor-stream" => {
                        request_content_details_init(&mut request);
                        request.content_details.as_mut().unwrap().monitor_stream.enable_monitor_stream = arg_from_str(value.unwrap_or("false"), err, "content-details.monitor-stream.enable-monitor-stream", "boolean");
                    },
                "snippet.actual-end-time" => {
                        request_snippet_init(&mut request);
                        request.snippet.as_mut().unwrap().actual_end_time = value.unwrap_or("").to_string();
                    },
                "snippet.description" => {
                        request_snippet_init(&mut request);
                        request.snippet.as_mut().unwrap().description = value.unwrap_or("").to_string();
                    },
                "snippet.title" => {
                        request_snippet_init(&mut request);
                        request.snippet.as_mut().unwrap().title = value.unwrap_or("").to_string();
                    },
                "snippet.channel-id" => {
                        request_snippet_init(&mut request);
                        request.snippet.as_mut().unwrap().channel_id = value.unwrap_or("").to_string();
                    },
                "snippet.published-at" => {
                        request_snippet_init(&mut request);
                        request.snippet.as_mut().unwrap().published_at = value.unwrap_or("").to_string();
                    },
                "snippet.scheduled-start-time" => {
                        request_snippet_init(&mut request);
                        request.snippet.as_mut().unwrap().scheduled_start_time = value.unwrap_or("").to_string();
                    },
                "snippet.actual-start-time" => {
                        request_snippet_init(&mut request);
                        request.snippet.as_mut().unwrap().actual_start_time = value.unwrap_or("").to_string();
                    },
                "snippet.scheduled-end-time" => {
                        request_snippet_init(&mut request);
                        request.snippet.as_mut().unwrap().scheduled_end_time = value.unwrap_or("").to_string();
                    },
                "snippet.thumbnails.default.url" => {
                        request_snippet_init(&mut request);
                        request.snippet.as_mut().unwrap().thumbnails.default.url = Some(value.unwrap_or("").to_string());
                    },
                "snippet.thumbnails.default.width" => {
                        request_snippet_init(&mut request);
                        request.snippet.as_mut().unwrap().thumbnails.default.width = Some(arg_from_str(value.unwrap_or("-0"), err, "snippet.thumbnails.default.width", "integer"));
                    },
                "snippet.thumbnails.default.height" => {
                        request_snippet_init(&mut request);
                        request.snippet.as_mut().unwrap().thumbnails.default.height = Some(arg_from_str(value.unwrap_or("-0"), err, "snippet.thumbnails.default.height", "integer"));
                    },
                "snippet.thumbnails.high.url" => {
                        request_snippet_init(&mut request);
                        request.snippet.as_mut().unwrap().thumbnails.high.url = Some(value.unwrap_or("").to_string());
                    },
                "snippet.thumbnails.high.width" => {
                        request_snippet_init(&mut request);
                        request.snippet.as_mut().unwrap().thumbnails.high.width = Some(arg_from_str(value.unwrap_or("-0"), err, "snippet.thumbnails.high.width", "integer"));
                    },
                "snippet.thumbnails.high.height" => {
                        request_snippet_init(&mut request);
                        request.snippet.as_mut().unwrap().thumbnails.high.height = Some(arg_from_str(value.unwrap_or("-0"), err, "snippet.thumbnails.high.height", "integer"));
                    },
                "snippet.thumbnails.medium.url" => {
                        request_snippet_init(&mut request);
                        request.snippet.as_mut().unwrap().thumbnails.medium.url = Some(value.unwrap_or("").to_string());
                    },
                "snippet.thumbnails.medium.width" => {
                        request_snippet_init(&mut request);
                        request.snippet.as_mut().unwrap().thumbnails.medium.width = Some(arg_from_str(value.unwrap_or("-0"), err, "snippet.thumbnails.medium.width", "integer"));
                    },
                "snippet.thumbnails.medium.height" => {
                        request_snippet_init(&mut request);
                        request.snippet.as_mut().unwrap().thumbnails.medium.height = Some(arg_from_str(value.unwrap_or("-0"), err, "snippet.thumbnails.medium.height", "integer"));
                    },
                "snippet.thumbnails.maxres.url" => {
                        request_snippet_init(&mut request);
                        request.snippet.as_mut().unwrap().thumbnails.maxres.url = Some(value.unwrap_or("").to_string());
                    },
                "snippet.thumbnails.maxres.width" => {
                        request_snippet_init(&mut request);
                        request.snippet.as_mut().unwrap().thumbnails.maxres.width = Some(arg_from_str(value.unwrap_or("-0"), err, "snippet.thumbnails.maxres.width", "integer"));
                    },
                "snippet.thumbnails.maxres.height" => {
                        request_snippet_init(&mut request);
                        request.snippet.as_mut().unwrap().thumbnails.maxres.height = Some(arg_from_str(value.unwrap_or("-0"), err, "snippet.thumbnails.maxres.height", "integer"));
                    },
                "snippet.thumbnails.standard.url" => {
                        request_snippet_init(&mut request);
                        request.snippet.as_mut().unwrap().thumbnails.standard.url = Some(value.unwrap_or("").to_string());
                    },
                "snippet.thumbnails.standard.width" => {
                        request_snippet_init(&mut request);
                        request.snippet.as_mut().unwrap().thumbnails.standard.width = Some(arg_from_str(value.unwrap_or("-0"), err, "snippet.thumbnails.standard.width", "integer"));
                    },
                "snippet.thumbnails.standard.height" => {
                        request_snippet_init(&mut request);
                        request.snippet.as_mut().unwrap().thumbnails.standard.height = Some(arg_from_str(value.unwrap_or("-0"), err, "snippet.thumbnails.standard.height", "integer"));
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
                    err.issues.push(CLIError::Field(FieldError::Unknown(field_name.to_string())));
                }
            }
        }
        let protocol = "standard-request";
        if dry_run {
            None
        } else {
            assert!(err.issues.len() == 0);
            let mut ostream = writer_from_opts(self.opt.flag_o, &self.opt.arg_out);
            match match protocol {
                "standard-request" => call.doit(),
                _ => unreachable!(),
            } {
                Err(api_err) => Some(api_err),
                Ok((mut response, output_schema)) => {
                    println!("DEBUG: REMOVE ME {:?}", response);
                    serde::json::to_writer(&mut ostream, &output_schema).unwrap();
                    None
                }
            }
        }
    }

    fn _live_broadcasts_list(&self, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Option<api::Error> {
        let mut call = self.hub.live_broadcasts().list(&self.opt.arg_part);
        for parg in self.opt.arg_v.iter() {
            let (key, value) = parse_kv_arg(&*parg, err);
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
                "alt"
                |"fields"
                |"key"
                |"oauth-token"
                |"pretty-print"
                |"quota-user"
                |"user-ip" => {
                    let map = [
                        ("oauth-token", "oauth_token"),
                        ("pretty-print", "prettyPrint"),
                        ("quota-user", "quotaUser"),
                        ("user-ip", "userIp"),
                    ];
                    call = call.param(map.iter().find(|t| t.0 == key).unwrap_or(&("", key)).1, value.unwrap_or("unset"))
                },
                _ => err.issues.push(CLIError::UnknownParameter(key.to_string())),
            }
        }
        let protocol = "standard-request";
        if dry_run {
            None
        } else {
            assert!(err.issues.len() == 0);
            let mut ostream = writer_from_opts(self.opt.flag_o, &self.opt.arg_out);
            match match protocol {
                "standard-request" => call.doit(),
                _ => unreachable!(),
            } {
                Err(api_err) => Some(api_err),
                Ok((mut response, output_schema)) => {
                    println!("DEBUG: REMOVE ME {:?}", response);
                    serde::json::to_writer(&mut ostream, &output_schema).unwrap();
                    None
                }
            }
        }
    }

    fn _live_broadcasts_transition(&self, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Option<api::Error> {
        let mut call = self.hub.live_broadcasts().transition(&self.opt.arg_broadcast_status, &self.opt.arg_id, &self.opt.arg_part);
        for parg in self.opt.arg_v.iter() {
            let (key, value) = parse_kv_arg(&*parg, err);
            match key {
                "on-behalf-of-content-owner-channel" => {
                    call = call.on_behalf_of_content_owner_channel(value.unwrap_or(""));
                },
                "on-behalf-of-content-owner" => {
                    call = call.on_behalf_of_content_owner(value.unwrap_or(""));
                },
                "alt"
                |"fields"
                |"key"
                |"oauth-token"
                |"pretty-print"
                |"quota-user"
                |"user-ip" => {
                    let map = [
                        ("oauth-token", "oauth_token"),
                        ("pretty-print", "prettyPrint"),
                        ("quota-user", "quotaUser"),
                        ("user-ip", "userIp"),
                    ];
                    call = call.param(map.iter().find(|t| t.0 == key).unwrap_or(&("", key)).1, value.unwrap_or("unset"))
                },
                _ => err.issues.push(CLIError::UnknownParameter(key.to_string())),
            }
        }
        let protocol = "standard-request";
        if dry_run {
            None
        } else {
            assert!(err.issues.len() == 0);
            let mut ostream = writer_from_opts(self.opt.flag_o, &self.opt.arg_out);
            match match protocol {
                "standard-request" => call.doit(),
                _ => unreachable!(),
            } {
                Err(api_err) => Some(api_err),
                Ok((mut response, output_schema)) => {
                    println!("DEBUG: REMOVE ME {:?}", response);
                    serde::json::to_writer(&mut ostream, &output_schema).unwrap();
                    None
                }
            }
        }
    }

    fn _live_broadcasts_update(&self, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Option<api::Error> {
            let mut request: api::LiveBroadcast = Default::default();
        let mut call = self.hub.live_broadcasts().update(&request);
        for parg in self.opt.arg_v.iter() {
            let (key, value) = parse_kv_arg(&*parg, err);
            match key {
                "on-behalf-of-content-owner-channel" => {
                    call = call.on_behalf_of_content_owner_channel(value.unwrap_or(""));
                },
                "on-behalf-of-content-owner" => {
                    call = call.on_behalf_of_content_owner(value.unwrap_or(""));
                },
                "alt"
                |"fields"
                |"key"
                |"oauth-token"
                |"pretty-print"
                |"quota-user"
                |"user-ip" => {
                    let map = [
                        ("oauth-token", "oauth_token"),
                        ("pretty-print", "prettyPrint"),
                        ("quota-user", "quotaUser"),
                        ("user-ip", "userIp"),
                    ];
                    call = call.param(map.iter().find(|t| t.0 == key).unwrap_or(&("", key)).1, value.unwrap_or("unset"))
                },
                _ => err.issues.push(CLIError::UnknownParameter(key.to_string())),
            }
        }
        let mut field_name: FieldCursor = Default::default();
        for kvarg in self.opt.arg_kv.iter() {
            let (key, value) = parse_kv_arg(&*kvarg, err);
            if let Err(field_err) = field_name.set(&*key) {
                err.issues.push(field_err);
            }
            fn request_content_details_init(request: &mut api::LiveBroadcast) {
                if request.content_details.is_none() {
                    request.content_details = Some(Default::default());
                }
            }
            
            fn request_snippet_init(request: &mut api::LiveBroadcast) {
                if request.snippet.is_none() {
                    request.snippet = Some(Default::default());
                }
            }
            
            fn request_status_init(request: &mut api::LiveBroadcast) {
                if request.status.is_none() {
                    request.status = Some(Default::default());
                }
            }
            
            match &field_name.to_string()[..] {
                "status.recording-status" => {
                        request_status_init(&mut request);
                        request.status.as_mut().unwrap().recording_status = value.unwrap_or("").to_string();
                    },
                "status.privacy-status" => {
                        request_status_init(&mut request);
                        request.status.as_mut().unwrap().privacy_status = value.unwrap_or("").to_string();
                    },
                "status.life-cycle-status" => {
                        request_status_init(&mut request);
                        request.status.as_mut().unwrap().life_cycle_status = value.unwrap_or("").to_string();
                    },
                "status.live-broadcast-priority" => {
                        request_status_init(&mut request);
                        request.status.as_mut().unwrap().live_broadcast_priority = value.unwrap_or("").to_string();
                    },
                "kind" => {
                        request_status_init(&mut request);
                        request.kind = Some(value.unwrap_or("").to_string());
                    },
                "content-details.start-with-slate" => {
                        request_content_details_init(&mut request);
                        request.content_details.as_mut().unwrap().start_with_slate = arg_from_str(value.unwrap_or("false"), err, "content-details.start-with-slate", "boolean");
                    },
                "content-details.bound-stream-id" => {
                        request_content_details_init(&mut request);
                        request.content_details.as_mut().unwrap().bound_stream_id = value.unwrap_or("").to_string();
                    },
                "content-details.enable-embed" => {
                        request_content_details_init(&mut request);
                        request.content_details.as_mut().unwrap().enable_embed = arg_from_str(value.unwrap_or("false"), err, "content-details.enable-embed", "boolean");
                    },
                "content-details.enable-closed-captions" => {
                        request_content_details_init(&mut request);
                        request.content_details.as_mut().unwrap().enable_closed_captions = arg_from_str(value.unwrap_or("false"), err, "content-details.enable-closed-captions", "boolean");
                    },
                "content-details.enable-content-encryption" => {
                        request_content_details_init(&mut request);
                        request.content_details.as_mut().unwrap().enable_content_encryption = arg_from_str(value.unwrap_or("false"), err, "content-details.enable-content-encryption", "boolean");
                    },
                "content-details.record-from-start" => {
                        request_content_details_init(&mut request);
                        request.content_details.as_mut().unwrap().record_from_start = arg_from_str(value.unwrap_or("false"), err, "content-details.record-from-start", "boolean");
                    },
                "content-details.enable-dvr" => {
                        request_content_details_init(&mut request);
                        request.content_details.as_mut().unwrap().enable_dvr = arg_from_str(value.unwrap_or("false"), err, "content-details.enable-dvr", "boolean");
                    },
                "content-details.monitor-stream.broadcast-stream-delay-ms" => {
                        request_content_details_init(&mut request);
                        request.content_details.as_mut().unwrap().monitor_stream.broadcast_stream_delay_ms = arg_from_str(value.unwrap_or("-0"), err, "content-details.monitor-stream.broadcast-stream-delay-ms", "integer");
                    },
                "content-details.monitor-stream.embed-html" => {
                        request_content_details_init(&mut request);
                        request.content_details.as_mut().unwrap().monitor_stream.embed_html = value.unwrap_or("").to_string();
                    },
                "content-details.monitor-stream.enable-monitor-stream" => {
                        request_content_details_init(&mut request);
                        request.content_details.as_mut().unwrap().monitor_stream.enable_monitor_stream = arg_from_str(value.unwrap_or("false"), err, "content-details.monitor-stream.enable-monitor-stream", "boolean");
                    },
                "snippet.actual-end-time" => {
                        request_snippet_init(&mut request);
                        request.snippet.as_mut().unwrap().actual_end_time = value.unwrap_or("").to_string();
                    },
                "snippet.description" => {
                        request_snippet_init(&mut request);
                        request.snippet.as_mut().unwrap().description = value.unwrap_or("").to_string();
                    },
                "snippet.title" => {
                        request_snippet_init(&mut request);
                        request.snippet.as_mut().unwrap().title = value.unwrap_or("").to_string();
                    },
                "snippet.channel-id" => {
                        request_snippet_init(&mut request);
                        request.snippet.as_mut().unwrap().channel_id = value.unwrap_or("").to_string();
                    },
                "snippet.published-at" => {
                        request_snippet_init(&mut request);
                        request.snippet.as_mut().unwrap().published_at = value.unwrap_or("").to_string();
                    },
                "snippet.scheduled-start-time" => {
                        request_snippet_init(&mut request);
                        request.snippet.as_mut().unwrap().scheduled_start_time = value.unwrap_or("").to_string();
                    },
                "snippet.actual-start-time" => {
                        request_snippet_init(&mut request);
                        request.snippet.as_mut().unwrap().actual_start_time = value.unwrap_or("").to_string();
                    },
                "snippet.scheduled-end-time" => {
                        request_snippet_init(&mut request);
                        request.snippet.as_mut().unwrap().scheduled_end_time = value.unwrap_or("").to_string();
                    },
                "snippet.thumbnails.default.url" => {
                        request_snippet_init(&mut request);
                        request.snippet.as_mut().unwrap().thumbnails.default.url = Some(value.unwrap_or("").to_string());
                    },
                "snippet.thumbnails.default.width" => {
                        request_snippet_init(&mut request);
                        request.snippet.as_mut().unwrap().thumbnails.default.width = Some(arg_from_str(value.unwrap_or("-0"), err, "snippet.thumbnails.default.width", "integer"));
                    },
                "snippet.thumbnails.default.height" => {
                        request_snippet_init(&mut request);
                        request.snippet.as_mut().unwrap().thumbnails.default.height = Some(arg_from_str(value.unwrap_or("-0"), err, "snippet.thumbnails.default.height", "integer"));
                    },
                "snippet.thumbnails.high.url" => {
                        request_snippet_init(&mut request);
                        request.snippet.as_mut().unwrap().thumbnails.high.url = Some(value.unwrap_or("").to_string());
                    },
                "snippet.thumbnails.high.width" => {
                        request_snippet_init(&mut request);
                        request.snippet.as_mut().unwrap().thumbnails.high.width = Some(arg_from_str(value.unwrap_or("-0"), err, "snippet.thumbnails.high.width", "integer"));
                    },
                "snippet.thumbnails.high.height" => {
                        request_snippet_init(&mut request);
                        request.snippet.as_mut().unwrap().thumbnails.high.height = Some(arg_from_str(value.unwrap_or("-0"), err, "snippet.thumbnails.high.height", "integer"));
                    },
                "snippet.thumbnails.medium.url" => {
                        request_snippet_init(&mut request);
                        request.snippet.as_mut().unwrap().thumbnails.medium.url = Some(value.unwrap_or("").to_string());
                    },
                "snippet.thumbnails.medium.width" => {
                        request_snippet_init(&mut request);
                        request.snippet.as_mut().unwrap().thumbnails.medium.width = Some(arg_from_str(value.unwrap_or("-0"), err, "snippet.thumbnails.medium.width", "integer"));
                    },
                "snippet.thumbnails.medium.height" => {
                        request_snippet_init(&mut request);
                        request.snippet.as_mut().unwrap().thumbnails.medium.height = Some(arg_from_str(value.unwrap_or("-0"), err, "snippet.thumbnails.medium.height", "integer"));
                    },
                "snippet.thumbnails.maxres.url" => {
                        request_snippet_init(&mut request);
                        request.snippet.as_mut().unwrap().thumbnails.maxres.url = Some(value.unwrap_or("").to_string());
                    },
                "snippet.thumbnails.maxres.width" => {
                        request_snippet_init(&mut request);
                        request.snippet.as_mut().unwrap().thumbnails.maxres.width = Some(arg_from_str(value.unwrap_or("-0"), err, "snippet.thumbnails.maxres.width", "integer"));
                    },
                "snippet.thumbnails.maxres.height" => {
                        request_snippet_init(&mut request);
                        request.snippet.as_mut().unwrap().thumbnails.maxres.height = Some(arg_from_str(value.unwrap_or("-0"), err, "snippet.thumbnails.maxres.height", "integer"));
                    },
                "snippet.thumbnails.standard.url" => {
                        request_snippet_init(&mut request);
                        request.snippet.as_mut().unwrap().thumbnails.standard.url = Some(value.unwrap_or("").to_string());
                    },
                "snippet.thumbnails.standard.width" => {
                        request_snippet_init(&mut request);
                        request.snippet.as_mut().unwrap().thumbnails.standard.width = Some(arg_from_str(value.unwrap_or("-0"), err, "snippet.thumbnails.standard.width", "integer"));
                    },
                "snippet.thumbnails.standard.height" => {
                        request_snippet_init(&mut request);
                        request.snippet.as_mut().unwrap().thumbnails.standard.height = Some(arg_from_str(value.unwrap_or("-0"), err, "snippet.thumbnails.standard.height", "integer"));
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
                    err.issues.push(CLIError::Field(FieldError::Unknown(field_name.to_string())));
                }
            }
        }
        let protocol = "standard-request";
        if dry_run {
            None
        } else {
            assert!(err.issues.len() == 0);
            let mut ostream = writer_from_opts(self.opt.flag_o, &self.opt.arg_out);
            match match protocol {
                "standard-request" => call.doit(),
                _ => unreachable!(),
            } {
                Err(api_err) => Some(api_err),
                Ok((mut response, output_schema)) => {
                    println!("DEBUG: REMOVE ME {:?}", response);
                    serde::json::to_writer(&mut ostream, &output_schema).unwrap();
                    None
                }
            }
        }
    }

    fn _live_streams_delete(&self, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Option<api::Error> {
        let mut call = self.hub.live_streams().delete(&self.opt.arg_id);
        for parg in self.opt.arg_v.iter() {
            let (key, value) = parse_kv_arg(&*parg, err);
            match key {
                "on-behalf-of-content-owner-channel" => {
                    call = call.on_behalf_of_content_owner_channel(value.unwrap_or(""));
                },
                "on-behalf-of-content-owner" => {
                    call = call.on_behalf_of_content_owner(value.unwrap_or(""));
                },
                "alt"
                |"fields"
                |"key"
                |"oauth-token"
                |"pretty-print"
                |"quota-user"
                |"user-ip" => {
                    let map = [
                        ("oauth-token", "oauth_token"),
                        ("pretty-print", "prettyPrint"),
                        ("quota-user", "quotaUser"),
                        ("user-ip", "userIp"),
                    ];
                    call = call.param(map.iter().find(|t| t.0 == key).unwrap_or(&("", key)).1, value.unwrap_or("unset"))
                },
                _ => err.issues.push(CLIError::UnknownParameter(key.to_string())),
            }
        }
        let protocol = "standard-request";
        if dry_run {
            None
        } else {
            assert!(err.issues.len() == 0);
            match match protocol {
                "standard-request" => call.doit(),
                _ => unreachable!(),
            } {
                Err(api_err) => Some(api_err),
                Ok(mut response) => {
                    println!("DEBUG: REMOVE ME {:?}", response);
                    None
                }
            }
        }
    }

    fn _live_streams_insert(&self, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Option<api::Error> {
            let mut request: api::LiveStream = Default::default();
        let mut call = self.hub.live_streams().insert(&request);
        for parg in self.opt.arg_v.iter() {
            let (key, value) = parse_kv_arg(&*parg, err);
            match key {
                "on-behalf-of-content-owner-channel" => {
                    call = call.on_behalf_of_content_owner_channel(value.unwrap_or(""));
                },
                "on-behalf-of-content-owner" => {
                    call = call.on_behalf_of_content_owner(value.unwrap_or(""));
                },
                "alt"
                |"fields"
                |"key"
                |"oauth-token"
                |"pretty-print"
                |"quota-user"
                |"user-ip" => {
                    let map = [
                        ("oauth-token", "oauth_token"),
                        ("pretty-print", "prettyPrint"),
                        ("quota-user", "quotaUser"),
                        ("user-ip", "userIp"),
                    ];
                    call = call.param(map.iter().find(|t| t.0 == key).unwrap_or(&("", key)).1, value.unwrap_or("unset"))
                },
                _ => err.issues.push(CLIError::UnknownParameter(key.to_string())),
            }
        }
        let mut field_name: FieldCursor = Default::default();
        for kvarg in self.opt.arg_kv.iter() {
            let (key, value) = parse_kv_arg(&*kvarg, err);
            if let Err(field_err) = field_name.set(&*key) {
                err.issues.push(field_err);
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
            
            match &field_name.to_string()[..] {
                "status.stream-status" => {
                        request_status_init(&mut request);
                        request.status.as_mut().unwrap().stream_status = value.unwrap_or("").to_string();
                    },
                "kind" => {
                        request_status_init(&mut request);
                        request.kind = Some(value.unwrap_or("").to_string());
                    },
                "content-details.is-reusable" => {
                        request_content_details_init(&mut request);
                        request.content_details.as_mut().unwrap().is_reusable = arg_from_str(value.unwrap_or("false"), err, "content-details.is-reusable", "boolean");
                    },
                "content-details.closed-captions-ingestion-url" => {
                        request_content_details_init(&mut request);
                        request.content_details.as_mut().unwrap().closed_captions_ingestion_url = value.unwrap_or("").to_string();
                    },
                "cdn.ingestion-type" => {
                        request_cdn_init(&mut request);
                        request.cdn.as_mut().unwrap().ingestion_type = value.unwrap_or("").to_string();
                    },
                "cdn.ingestion-info.backup-ingestion-address" => {
                        request_cdn_init(&mut request);
                        request.cdn.as_mut().unwrap().ingestion_info.backup_ingestion_address = value.unwrap_or("").to_string();
                    },
                "cdn.ingestion-info.stream-name" => {
                        request_cdn_init(&mut request);
                        request.cdn.as_mut().unwrap().ingestion_info.stream_name = value.unwrap_or("").to_string();
                    },
                "cdn.ingestion-info.ingestion-address" => {
                        request_cdn_init(&mut request);
                        request.cdn.as_mut().unwrap().ingestion_info.ingestion_address = value.unwrap_or("").to_string();
                    },
                "cdn.format" => {
                        request_cdn_init(&mut request);
                        request.cdn.as_mut().unwrap().format = value.unwrap_or("").to_string();
                    },
                "snippet.channel-id" => {
                        request_snippet_init(&mut request);
                        request.snippet.as_mut().unwrap().channel_id = value.unwrap_or("").to_string();
                    },
                "snippet.description" => {
                        request_snippet_init(&mut request);
                        request.snippet.as_mut().unwrap().description = value.unwrap_or("").to_string();
                    },
                "snippet.published-at" => {
                        request_snippet_init(&mut request);
                        request.snippet.as_mut().unwrap().published_at = value.unwrap_or("").to_string();
                    },
                "snippet.title" => {
                        request_snippet_init(&mut request);
                        request.snippet.as_mut().unwrap().title = value.unwrap_or("").to_string();
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
                    err.issues.push(CLIError::Field(FieldError::Unknown(field_name.to_string())));
                }
            }
        }
        let protocol = "standard-request";
        if dry_run {
            None
        } else {
            assert!(err.issues.len() == 0);
            let mut ostream = writer_from_opts(self.opt.flag_o, &self.opt.arg_out);
            match match protocol {
                "standard-request" => call.doit(),
                _ => unreachable!(),
            } {
                Err(api_err) => Some(api_err),
                Ok((mut response, output_schema)) => {
                    println!("DEBUG: REMOVE ME {:?}", response);
                    serde::json::to_writer(&mut ostream, &output_schema).unwrap();
                    None
                }
            }
        }
    }

    fn _live_streams_list(&self, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Option<api::Error> {
        let mut call = self.hub.live_streams().list(&self.opt.arg_part);
        for parg in self.opt.arg_v.iter() {
            let (key, value) = parse_kv_arg(&*parg, err);
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
                "alt"
                |"fields"
                |"key"
                |"oauth-token"
                |"pretty-print"
                |"quota-user"
                |"user-ip" => {
                    let map = [
                        ("oauth-token", "oauth_token"),
                        ("pretty-print", "prettyPrint"),
                        ("quota-user", "quotaUser"),
                        ("user-ip", "userIp"),
                    ];
                    call = call.param(map.iter().find(|t| t.0 == key).unwrap_or(&("", key)).1, value.unwrap_or("unset"))
                },
                _ => err.issues.push(CLIError::UnknownParameter(key.to_string())),
            }
        }
        let protocol = "standard-request";
        if dry_run {
            None
        } else {
            assert!(err.issues.len() == 0);
            let mut ostream = writer_from_opts(self.opt.flag_o, &self.opt.arg_out);
            match match protocol {
                "standard-request" => call.doit(),
                _ => unreachable!(),
            } {
                Err(api_err) => Some(api_err),
                Ok((mut response, output_schema)) => {
                    println!("DEBUG: REMOVE ME {:?}", response);
                    serde::json::to_writer(&mut ostream, &output_schema).unwrap();
                    None
                }
            }
        }
    }

    fn _live_streams_update(&self, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Option<api::Error> {
            let mut request: api::LiveStream = Default::default();
        let mut call = self.hub.live_streams().update(&request);
        for parg in self.opt.arg_v.iter() {
            let (key, value) = parse_kv_arg(&*parg, err);
            match key {
                "on-behalf-of-content-owner-channel" => {
                    call = call.on_behalf_of_content_owner_channel(value.unwrap_or(""));
                },
                "on-behalf-of-content-owner" => {
                    call = call.on_behalf_of_content_owner(value.unwrap_or(""));
                },
                "alt"
                |"fields"
                |"key"
                |"oauth-token"
                |"pretty-print"
                |"quota-user"
                |"user-ip" => {
                    let map = [
                        ("oauth-token", "oauth_token"),
                        ("pretty-print", "prettyPrint"),
                        ("quota-user", "quotaUser"),
                        ("user-ip", "userIp"),
                    ];
                    call = call.param(map.iter().find(|t| t.0 == key).unwrap_or(&("", key)).1, value.unwrap_or("unset"))
                },
                _ => err.issues.push(CLIError::UnknownParameter(key.to_string())),
            }
        }
        let mut field_name: FieldCursor = Default::default();
        for kvarg in self.opt.arg_kv.iter() {
            let (key, value) = parse_kv_arg(&*kvarg, err);
            if let Err(field_err) = field_name.set(&*key) {
                err.issues.push(field_err);
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
            
            match &field_name.to_string()[..] {
                "status.stream-status" => {
                        request_status_init(&mut request);
                        request.status.as_mut().unwrap().stream_status = value.unwrap_or("").to_string();
                    },
                "kind" => {
                        request_status_init(&mut request);
                        request.kind = Some(value.unwrap_or("").to_string());
                    },
                "content-details.is-reusable" => {
                        request_content_details_init(&mut request);
                        request.content_details.as_mut().unwrap().is_reusable = arg_from_str(value.unwrap_or("false"), err, "content-details.is-reusable", "boolean");
                    },
                "content-details.closed-captions-ingestion-url" => {
                        request_content_details_init(&mut request);
                        request.content_details.as_mut().unwrap().closed_captions_ingestion_url = value.unwrap_or("").to_string();
                    },
                "cdn.ingestion-type" => {
                        request_cdn_init(&mut request);
                        request.cdn.as_mut().unwrap().ingestion_type = value.unwrap_or("").to_string();
                    },
                "cdn.ingestion-info.backup-ingestion-address" => {
                        request_cdn_init(&mut request);
                        request.cdn.as_mut().unwrap().ingestion_info.backup_ingestion_address = value.unwrap_or("").to_string();
                    },
                "cdn.ingestion-info.stream-name" => {
                        request_cdn_init(&mut request);
                        request.cdn.as_mut().unwrap().ingestion_info.stream_name = value.unwrap_or("").to_string();
                    },
                "cdn.ingestion-info.ingestion-address" => {
                        request_cdn_init(&mut request);
                        request.cdn.as_mut().unwrap().ingestion_info.ingestion_address = value.unwrap_or("").to_string();
                    },
                "cdn.format" => {
                        request_cdn_init(&mut request);
                        request.cdn.as_mut().unwrap().format = value.unwrap_or("").to_string();
                    },
                "snippet.channel-id" => {
                        request_snippet_init(&mut request);
                        request.snippet.as_mut().unwrap().channel_id = value.unwrap_or("").to_string();
                    },
                "snippet.description" => {
                        request_snippet_init(&mut request);
                        request.snippet.as_mut().unwrap().description = value.unwrap_or("").to_string();
                    },
                "snippet.published-at" => {
                        request_snippet_init(&mut request);
                        request.snippet.as_mut().unwrap().published_at = value.unwrap_or("").to_string();
                    },
                "snippet.title" => {
                        request_snippet_init(&mut request);
                        request.snippet.as_mut().unwrap().title = value.unwrap_or("").to_string();
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
                    err.issues.push(CLIError::Field(FieldError::Unknown(field_name.to_string())));
                }
            }
        }
        let protocol = "standard-request";
        if dry_run {
            None
        } else {
            assert!(err.issues.len() == 0);
            let mut ostream = writer_from_opts(self.opt.flag_o, &self.opt.arg_out);
            match match protocol {
                "standard-request" => call.doit(),
                _ => unreachable!(),
            } {
                Err(api_err) => Some(api_err),
                Ok((mut response, output_schema)) => {
                    println!("DEBUG: REMOVE ME {:?}", response);
                    serde::json::to_writer(&mut ostream, &output_schema).unwrap();
                    None
                }
            }
        }
    }

    fn _playlist_items_delete(&self, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Option<api::Error> {
        let mut call = self.hub.playlist_items().delete(&self.opt.arg_id);
        for parg in self.opt.arg_v.iter() {
            let (key, value) = parse_kv_arg(&*parg, err);
            match key {
                "alt"
                |"fields"
                |"key"
                |"oauth-token"
                |"pretty-print"
                |"quota-user"
                |"user-ip" => {
                    let map = [
                        ("oauth-token", "oauth_token"),
                        ("pretty-print", "prettyPrint"),
                        ("quota-user", "quotaUser"),
                        ("user-ip", "userIp"),
                    ];
                    call = call.param(map.iter().find(|t| t.0 == key).unwrap_or(&("", key)).1, value.unwrap_or("unset"))
                },
                _ => err.issues.push(CLIError::UnknownParameter(key.to_string())),
            }
        }
        let protocol = "standard-request";
        if dry_run {
            None
        } else {
            assert!(err.issues.len() == 0);
            match match protocol {
                "standard-request" => call.doit(),
                _ => unreachable!(),
            } {
                Err(api_err) => Some(api_err),
                Ok(mut response) => {
                    println!("DEBUG: REMOVE ME {:?}", response);
                    None
                }
            }
        }
    }

    fn _playlist_items_insert(&self, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Option<api::Error> {
            let mut request: api::PlaylistItem = Default::default();
        let mut call = self.hub.playlist_items().insert(&request);
        for parg in self.opt.arg_v.iter() {
            let (key, value) = parse_kv_arg(&*parg, err);
            match key {
                "on-behalf-of-content-owner" => {
                    call = call.on_behalf_of_content_owner(value.unwrap_or(""));
                },
                "alt"
                |"fields"
                |"key"
                |"oauth-token"
                |"pretty-print"
                |"quota-user"
                |"user-ip" => {
                    let map = [
                        ("oauth-token", "oauth_token"),
                        ("pretty-print", "prettyPrint"),
                        ("quota-user", "quotaUser"),
                        ("user-ip", "userIp"),
                    ];
                    call = call.param(map.iter().find(|t| t.0 == key).unwrap_or(&("", key)).1, value.unwrap_or("unset"))
                },
                _ => err.issues.push(CLIError::UnknownParameter(key.to_string())),
            }
        }
        let mut field_name: FieldCursor = Default::default();
        for kvarg in self.opt.arg_kv.iter() {
            let (key, value) = parse_kv_arg(&*kvarg, err);
            if let Err(field_err) = field_name.set(&*key) {
                err.issues.push(field_err);
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
            
            fn request_status_init(request: &mut api::PlaylistItem) {
                if request.status.is_none() {
                    request.status = Some(Default::default());
                }
            }
            
            match &field_name.to_string()[..] {
                "status.privacy-status" => {
                        request_status_init(&mut request);
                        request.status.as_mut().unwrap().privacy_status = value.unwrap_or("").to_string();
                    },
                "kind" => {
                        request_status_init(&mut request);
                        request.kind = Some(value.unwrap_or("").to_string());
                    },
                "content-details.note" => {
                        request_content_details_init(&mut request);
                        request.content_details.as_mut().unwrap().note = value.unwrap_or("").to_string();
                    },
                "content-details.start-at" => {
                        request_content_details_init(&mut request);
                        request.content_details.as_mut().unwrap().start_at = value.unwrap_or("").to_string();
                    },
                "content-details.end-at" => {
                        request_content_details_init(&mut request);
                        request.content_details.as_mut().unwrap().end_at = value.unwrap_or("").to_string();
                    },
                "content-details.video-id" => {
                        request_content_details_init(&mut request);
                        request.content_details.as_mut().unwrap().video_id = value.unwrap_or("").to_string();
                    },
                "snippet.playlist-id" => {
                        request_snippet_init(&mut request);
                        request.snippet.as_mut().unwrap().playlist_id = value.unwrap_or("").to_string();
                    },
                "snippet.description" => {
                        request_snippet_init(&mut request);
                        request.snippet.as_mut().unwrap().description = value.unwrap_or("").to_string();
                    },
                "snippet.title" => {
                        request_snippet_init(&mut request);
                        request.snippet.as_mut().unwrap().title = value.unwrap_or("").to_string();
                    },
                "snippet.resource-id.kind" => {
                        request_snippet_init(&mut request);
                        request.snippet.as_mut().unwrap().resource_id.kind = value.unwrap_or("").to_string();
                    },
                "snippet.resource-id.channel-id" => {
                        request_snippet_init(&mut request);
                        request.snippet.as_mut().unwrap().resource_id.channel_id = value.unwrap_or("").to_string();
                    },
                "snippet.resource-id.playlist-id" => {
                        request_snippet_init(&mut request);
                        request.snippet.as_mut().unwrap().resource_id.playlist_id = value.unwrap_or("").to_string();
                    },
                "snippet.resource-id.video-id" => {
                        request_snippet_init(&mut request);
                        request.snippet.as_mut().unwrap().resource_id.video_id = value.unwrap_or("").to_string();
                    },
                "snippet.channel-id" => {
                        request_snippet_init(&mut request);
                        request.snippet.as_mut().unwrap().channel_id = value.unwrap_or("").to_string();
                    },
                "snippet.published-at" => {
                        request_snippet_init(&mut request);
                        request.snippet.as_mut().unwrap().published_at = value.unwrap_or("").to_string();
                    },
                "snippet.channel-title" => {
                        request_snippet_init(&mut request);
                        request.snippet.as_mut().unwrap().channel_title = value.unwrap_or("").to_string();
                    },
                "snippet.position" => {
                        request_snippet_init(&mut request);
                        request.snippet.as_mut().unwrap().position = arg_from_str(value.unwrap_or("-0"), err, "snippet.position", "integer");
                    },
                "snippet.thumbnails.default.url" => {
                        request_snippet_init(&mut request);
                        request.snippet.as_mut().unwrap().thumbnails.default.url = Some(value.unwrap_or("").to_string());
                    },
                "snippet.thumbnails.default.width" => {
                        request_snippet_init(&mut request);
                        request.snippet.as_mut().unwrap().thumbnails.default.width = Some(arg_from_str(value.unwrap_or("-0"), err, "snippet.thumbnails.default.width", "integer"));
                    },
                "snippet.thumbnails.default.height" => {
                        request_snippet_init(&mut request);
                        request.snippet.as_mut().unwrap().thumbnails.default.height = Some(arg_from_str(value.unwrap_or("-0"), err, "snippet.thumbnails.default.height", "integer"));
                    },
                "snippet.thumbnails.high.url" => {
                        request_snippet_init(&mut request);
                        request.snippet.as_mut().unwrap().thumbnails.high.url = Some(value.unwrap_or("").to_string());
                    },
                "snippet.thumbnails.high.width" => {
                        request_snippet_init(&mut request);
                        request.snippet.as_mut().unwrap().thumbnails.high.width = Some(arg_from_str(value.unwrap_or("-0"), err, "snippet.thumbnails.high.width", "integer"));
                    },
                "snippet.thumbnails.high.height" => {
                        request_snippet_init(&mut request);
                        request.snippet.as_mut().unwrap().thumbnails.high.height = Some(arg_from_str(value.unwrap_or("-0"), err, "snippet.thumbnails.high.height", "integer"));
                    },
                "snippet.thumbnails.medium.url" => {
                        request_snippet_init(&mut request);
                        request.snippet.as_mut().unwrap().thumbnails.medium.url = Some(value.unwrap_or("").to_string());
                    },
                "snippet.thumbnails.medium.width" => {
                        request_snippet_init(&mut request);
                        request.snippet.as_mut().unwrap().thumbnails.medium.width = Some(arg_from_str(value.unwrap_or("-0"), err, "snippet.thumbnails.medium.width", "integer"));
                    },
                "snippet.thumbnails.medium.height" => {
                        request_snippet_init(&mut request);
                        request.snippet.as_mut().unwrap().thumbnails.medium.height = Some(arg_from_str(value.unwrap_or("-0"), err, "snippet.thumbnails.medium.height", "integer"));
                    },
                "snippet.thumbnails.maxres.url" => {
                        request_snippet_init(&mut request);
                        request.snippet.as_mut().unwrap().thumbnails.maxres.url = Some(value.unwrap_or("").to_string());
                    },
                "snippet.thumbnails.maxres.width" => {
                        request_snippet_init(&mut request);
                        request.snippet.as_mut().unwrap().thumbnails.maxres.width = Some(arg_from_str(value.unwrap_or("-0"), err, "snippet.thumbnails.maxres.width", "integer"));
                    },
                "snippet.thumbnails.maxres.height" => {
                        request_snippet_init(&mut request);
                        request.snippet.as_mut().unwrap().thumbnails.maxres.height = Some(arg_from_str(value.unwrap_or("-0"), err, "snippet.thumbnails.maxres.height", "integer"));
                    },
                "snippet.thumbnails.standard.url" => {
                        request_snippet_init(&mut request);
                        request.snippet.as_mut().unwrap().thumbnails.standard.url = Some(value.unwrap_or("").to_string());
                    },
                "snippet.thumbnails.standard.width" => {
                        request_snippet_init(&mut request);
                        request.snippet.as_mut().unwrap().thumbnails.standard.width = Some(arg_from_str(value.unwrap_or("-0"), err, "snippet.thumbnails.standard.width", "integer"));
                    },
                "snippet.thumbnails.standard.height" => {
                        request_snippet_init(&mut request);
                        request.snippet.as_mut().unwrap().thumbnails.standard.height = Some(arg_from_str(value.unwrap_or("-0"), err, "snippet.thumbnails.standard.height", "integer"));
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
                    err.issues.push(CLIError::Field(FieldError::Unknown(field_name.to_string())));
                }
            }
        }
        let protocol = "standard-request";
        if dry_run {
            None
        } else {
            assert!(err.issues.len() == 0);
            let mut ostream = writer_from_opts(self.opt.flag_o, &self.opt.arg_out);
            match match protocol {
                "standard-request" => call.doit(),
                _ => unreachable!(),
            } {
                Err(api_err) => Some(api_err),
                Ok((mut response, output_schema)) => {
                    println!("DEBUG: REMOVE ME {:?}", response);
                    serde::json::to_writer(&mut ostream, &output_schema).unwrap();
                    None
                }
            }
        }
    }

    fn _playlist_items_list(&self, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Option<api::Error> {
        let mut call = self.hub.playlist_items().list(&self.opt.arg_part);
        for parg in self.opt.arg_v.iter() {
            let (key, value) = parse_kv_arg(&*parg, err);
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
                "alt"
                |"fields"
                |"key"
                |"oauth-token"
                |"pretty-print"
                |"quota-user"
                |"user-ip" => {
                    let map = [
                        ("oauth-token", "oauth_token"),
                        ("pretty-print", "prettyPrint"),
                        ("quota-user", "quotaUser"),
                        ("user-ip", "userIp"),
                    ];
                    call = call.param(map.iter().find(|t| t.0 == key).unwrap_or(&("", key)).1, value.unwrap_or("unset"))
                },
                _ => err.issues.push(CLIError::UnknownParameter(key.to_string())),
            }
        }
        let protocol = "standard-request";
        if dry_run {
            None
        } else {
            assert!(err.issues.len() == 0);
            let mut ostream = writer_from_opts(self.opt.flag_o, &self.opt.arg_out);
            match match protocol {
                "standard-request" => call.doit(),
                _ => unreachable!(),
            } {
                Err(api_err) => Some(api_err),
                Ok((mut response, output_schema)) => {
                    println!("DEBUG: REMOVE ME {:?}", response);
                    serde::json::to_writer(&mut ostream, &output_schema).unwrap();
                    None
                }
            }
        }
    }

    fn _playlist_items_update(&self, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Option<api::Error> {
            let mut request: api::PlaylistItem = Default::default();
        let mut call = self.hub.playlist_items().update(&request);
        for parg in self.opt.arg_v.iter() {
            let (key, value) = parse_kv_arg(&*parg, err);
            match key {
                "alt"
                |"fields"
                |"key"
                |"oauth-token"
                |"pretty-print"
                |"quota-user"
                |"user-ip" => {
                    let map = [
                        ("oauth-token", "oauth_token"),
                        ("pretty-print", "prettyPrint"),
                        ("quota-user", "quotaUser"),
                        ("user-ip", "userIp"),
                    ];
                    call = call.param(map.iter().find(|t| t.0 == key).unwrap_or(&("", key)).1, value.unwrap_or("unset"))
                },
                _ => err.issues.push(CLIError::UnknownParameter(key.to_string())),
            }
        }
        let mut field_name: FieldCursor = Default::default();
        for kvarg in self.opt.arg_kv.iter() {
            let (key, value) = parse_kv_arg(&*kvarg, err);
            if let Err(field_err) = field_name.set(&*key) {
                err.issues.push(field_err);
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
            
            fn request_status_init(request: &mut api::PlaylistItem) {
                if request.status.is_none() {
                    request.status = Some(Default::default());
                }
            }
            
            match &field_name.to_string()[..] {
                "status.privacy-status" => {
                        request_status_init(&mut request);
                        request.status.as_mut().unwrap().privacy_status = value.unwrap_or("").to_string();
                    },
                "kind" => {
                        request_status_init(&mut request);
                        request.kind = Some(value.unwrap_or("").to_string());
                    },
                "content-details.note" => {
                        request_content_details_init(&mut request);
                        request.content_details.as_mut().unwrap().note = value.unwrap_or("").to_string();
                    },
                "content-details.start-at" => {
                        request_content_details_init(&mut request);
                        request.content_details.as_mut().unwrap().start_at = value.unwrap_or("").to_string();
                    },
                "content-details.end-at" => {
                        request_content_details_init(&mut request);
                        request.content_details.as_mut().unwrap().end_at = value.unwrap_or("").to_string();
                    },
                "content-details.video-id" => {
                        request_content_details_init(&mut request);
                        request.content_details.as_mut().unwrap().video_id = value.unwrap_or("").to_string();
                    },
                "snippet.playlist-id" => {
                        request_snippet_init(&mut request);
                        request.snippet.as_mut().unwrap().playlist_id = value.unwrap_or("").to_string();
                    },
                "snippet.description" => {
                        request_snippet_init(&mut request);
                        request.snippet.as_mut().unwrap().description = value.unwrap_or("").to_string();
                    },
                "snippet.title" => {
                        request_snippet_init(&mut request);
                        request.snippet.as_mut().unwrap().title = value.unwrap_or("").to_string();
                    },
                "snippet.resource-id.kind" => {
                        request_snippet_init(&mut request);
                        request.snippet.as_mut().unwrap().resource_id.kind = value.unwrap_or("").to_string();
                    },
                "snippet.resource-id.channel-id" => {
                        request_snippet_init(&mut request);
                        request.snippet.as_mut().unwrap().resource_id.channel_id = value.unwrap_or("").to_string();
                    },
                "snippet.resource-id.playlist-id" => {
                        request_snippet_init(&mut request);
                        request.snippet.as_mut().unwrap().resource_id.playlist_id = value.unwrap_or("").to_string();
                    },
                "snippet.resource-id.video-id" => {
                        request_snippet_init(&mut request);
                        request.snippet.as_mut().unwrap().resource_id.video_id = value.unwrap_or("").to_string();
                    },
                "snippet.channel-id" => {
                        request_snippet_init(&mut request);
                        request.snippet.as_mut().unwrap().channel_id = value.unwrap_or("").to_string();
                    },
                "snippet.published-at" => {
                        request_snippet_init(&mut request);
                        request.snippet.as_mut().unwrap().published_at = value.unwrap_or("").to_string();
                    },
                "snippet.channel-title" => {
                        request_snippet_init(&mut request);
                        request.snippet.as_mut().unwrap().channel_title = value.unwrap_or("").to_string();
                    },
                "snippet.position" => {
                        request_snippet_init(&mut request);
                        request.snippet.as_mut().unwrap().position = arg_from_str(value.unwrap_or("-0"), err, "snippet.position", "integer");
                    },
                "snippet.thumbnails.default.url" => {
                        request_snippet_init(&mut request);
                        request.snippet.as_mut().unwrap().thumbnails.default.url = Some(value.unwrap_or("").to_string());
                    },
                "snippet.thumbnails.default.width" => {
                        request_snippet_init(&mut request);
                        request.snippet.as_mut().unwrap().thumbnails.default.width = Some(arg_from_str(value.unwrap_or("-0"), err, "snippet.thumbnails.default.width", "integer"));
                    },
                "snippet.thumbnails.default.height" => {
                        request_snippet_init(&mut request);
                        request.snippet.as_mut().unwrap().thumbnails.default.height = Some(arg_from_str(value.unwrap_or("-0"), err, "snippet.thumbnails.default.height", "integer"));
                    },
                "snippet.thumbnails.high.url" => {
                        request_snippet_init(&mut request);
                        request.snippet.as_mut().unwrap().thumbnails.high.url = Some(value.unwrap_or("").to_string());
                    },
                "snippet.thumbnails.high.width" => {
                        request_snippet_init(&mut request);
                        request.snippet.as_mut().unwrap().thumbnails.high.width = Some(arg_from_str(value.unwrap_or("-0"), err, "snippet.thumbnails.high.width", "integer"));
                    },
                "snippet.thumbnails.high.height" => {
                        request_snippet_init(&mut request);
                        request.snippet.as_mut().unwrap().thumbnails.high.height = Some(arg_from_str(value.unwrap_or("-0"), err, "snippet.thumbnails.high.height", "integer"));
                    },
                "snippet.thumbnails.medium.url" => {
                        request_snippet_init(&mut request);
                        request.snippet.as_mut().unwrap().thumbnails.medium.url = Some(value.unwrap_or("").to_string());
                    },
                "snippet.thumbnails.medium.width" => {
                        request_snippet_init(&mut request);
                        request.snippet.as_mut().unwrap().thumbnails.medium.width = Some(arg_from_str(value.unwrap_or("-0"), err, "snippet.thumbnails.medium.width", "integer"));
                    },
                "snippet.thumbnails.medium.height" => {
                        request_snippet_init(&mut request);
                        request.snippet.as_mut().unwrap().thumbnails.medium.height = Some(arg_from_str(value.unwrap_or("-0"), err, "snippet.thumbnails.medium.height", "integer"));
                    },
                "snippet.thumbnails.maxres.url" => {
                        request_snippet_init(&mut request);
                        request.snippet.as_mut().unwrap().thumbnails.maxres.url = Some(value.unwrap_or("").to_string());
                    },
                "snippet.thumbnails.maxres.width" => {
                        request_snippet_init(&mut request);
                        request.snippet.as_mut().unwrap().thumbnails.maxres.width = Some(arg_from_str(value.unwrap_or("-0"), err, "snippet.thumbnails.maxres.width", "integer"));
                    },
                "snippet.thumbnails.maxres.height" => {
                        request_snippet_init(&mut request);
                        request.snippet.as_mut().unwrap().thumbnails.maxres.height = Some(arg_from_str(value.unwrap_or("-0"), err, "snippet.thumbnails.maxres.height", "integer"));
                    },
                "snippet.thumbnails.standard.url" => {
                        request_snippet_init(&mut request);
                        request.snippet.as_mut().unwrap().thumbnails.standard.url = Some(value.unwrap_or("").to_string());
                    },
                "snippet.thumbnails.standard.width" => {
                        request_snippet_init(&mut request);
                        request.snippet.as_mut().unwrap().thumbnails.standard.width = Some(arg_from_str(value.unwrap_or("-0"), err, "snippet.thumbnails.standard.width", "integer"));
                    },
                "snippet.thumbnails.standard.height" => {
                        request_snippet_init(&mut request);
                        request.snippet.as_mut().unwrap().thumbnails.standard.height = Some(arg_from_str(value.unwrap_or("-0"), err, "snippet.thumbnails.standard.height", "integer"));
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
                    err.issues.push(CLIError::Field(FieldError::Unknown(field_name.to_string())));
                }
            }
        }
        let protocol = "standard-request";
        if dry_run {
            None
        } else {
            assert!(err.issues.len() == 0);
            let mut ostream = writer_from_opts(self.opt.flag_o, &self.opt.arg_out);
            match match protocol {
                "standard-request" => call.doit(),
                _ => unreachable!(),
            } {
                Err(api_err) => Some(api_err),
                Ok((mut response, output_schema)) => {
                    println!("DEBUG: REMOVE ME {:?}", response);
                    serde::json::to_writer(&mut ostream, &output_schema).unwrap();
                    None
                }
            }
        }
    }

    fn _playlists_delete(&self, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Option<api::Error> {
        let mut call = self.hub.playlists().delete(&self.opt.arg_id);
        for parg in self.opt.arg_v.iter() {
            let (key, value) = parse_kv_arg(&*parg, err);
            match key {
                "on-behalf-of-content-owner" => {
                    call = call.on_behalf_of_content_owner(value.unwrap_or(""));
                },
                "alt"
                |"fields"
                |"key"
                |"oauth-token"
                |"pretty-print"
                |"quota-user"
                |"user-ip" => {
                    let map = [
                        ("oauth-token", "oauth_token"),
                        ("pretty-print", "prettyPrint"),
                        ("quota-user", "quotaUser"),
                        ("user-ip", "userIp"),
                    ];
                    call = call.param(map.iter().find(|t| t.0 == key).unwrap_or(&("", key)).1, value.unwrap_or("unset"))
                },
                _ => err.issues.push(CLIError::UnknownParameter(key.to_string())),
            }
        }
        let protocol = "standard-request";
        if dry_run {
            None
        } else {
            assert!(err.issues.len() == 0);
            match match protocol {
                "standard-request" => call.doit(),
                _ => unreachable!(),
            } {
                Err(api_err) => Some(api_err),
                Ok(mut response) => {
                    println!("DEBUG: REMOVE ME {:?}", response);
                    None
                }
            }
        }
    }

    fn _playlists_insert(&self, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Option<api::Error> {
            let mut request: api::Playlist = Default::default();
        let mut call = self.hub.playlists().insert(&request);
        for parg in self.opt.arg_v.iter() {
            let (key, value) = parse_kv_arg(&*parg, err);
            match key {
                "on-behalf-of-content-owner-channel" => {
                    call = call.on_behalf_of_content_owner_channel(value.unwrap_or(""));
                },
                "on-behalf-of-content-owner" => {
                    call = call.on_behalf_of_content_owner(value.unwrap_or(""));
                },
                "alt"
                |"fields"
                |"key"
                |"oauth-token"
                |"pretty-print"
                |"quota-user"
                |"user-ip" => {
                    let map = [
                        ("oauth-token", "oauth_token"),
                        ("pretty-print", "prettyPrint"),
                        ("quota-user", "quotaUser"),
                        ("user-ip", "userIp"),
                    ];
                    call = call.param(map.iter().find(|t| t.0 == key).unwrap_or(&("", key)).1, value.unwrap_or("unset"))
                },
                _ => err.issues.push(CLIError::UnknownParameter(key.to_string())),
            }
        }
        let mut field_name: FieldCursor = Default::default();
        for kvarg in self.opt.arg_kv.iter() {
            let (key, value) = parse_kv_arg(&*kvarg, err);
            if let Err(field_err) = field_name.set(&*key) {
                err.issues.push(field_err);
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
            
            fn request_status_init(request: &mut api::Playlist) {
                if request.status.is_none() {
                    request.status = Some(Default::default());
                }
            }
            
            match &field_name.to_string()[..] {
                "status.privacy-status" => {
                        request_status_init(&mut request);
                        request.status.as_mut().unwrap().privacy_status = value.unwrap_or("").to_string();
                    },
                "kind" => {
                        request_status_init(&mut request);
                        request.kind = Some(value.unwrap_or("").to_string());
                    },
                "content-details.item-count" => {
                        request_content_details_init(&mut request);
                        request.content_details.as_mut().unwrap().item_count = arg_from_str(value.unwrap_or("-0"), err, "content-details.item-count", "integer");
                    },
                "snippet.description" => {
                        request_snippet_init(&mut request);
                        request.snippet.as_mut().unwrap().description = value.unwrap_or("").to_string();
                    },
                "snippet.tags" => {
                        request_snippet_init(&mut request);
                        request.snippet.as_mut().unwrap().tags.push(value.unwrap_or("").to_string());
                    },
                "snippet.channel-id" => {
                        request_snippet_init(&mut request);
                        request.snippet.as_mut().unwrap().channel_id = value.unwrap_or("").to_string();
                    },
                "snippet.published-at" => {
                        request_snippet_init(&mut request);
                        request.snippet.as_mut().unwrap().published_at = value.unwrap_or("").to_string();
                    },
                "snippet.channel-title" => {
                        request_snippet_init(&mut request);
                        request.snippet.as_mut().unwrap().channel_title = value.unwrap_or("").to_string();
                    },
                "snippet.title" => {
                        request_snippet_init(&mut request);
                        request.snippet.as_mut().unwrap().title = value.unwrap_or("").to_string();
                    },
                "snippet.default-language" => {
                        request_snippet_init(&mut request);
                        request.snippet.as_mut().unwrap().default_language = value.unwrap_or("").to_string();
                    },
                "snippet.localized.description" => {
                        request_snippet_init(&mut request);
                        request.snippet.as_mut().unwrap().localized.description = value.unwrap_or("").to_string();
                    },
                "snippet.localized.title" => {
                        request_snippet_init(&mut request);
                        request.snippet.as_mut().unwrap().localized.title = value.unwrap_or("").to_string();
                    },
                "snippet.thumbnails.default.url" => {
                        request_snippet_init(&mut request);
                        request.snippet.as_mut().unwrap().thumbnails.default.url = Some(value.unwrap_or("").to_string());
                    },
                "snippet.thumbnails.default.width" => {
                        request_snippet_init(&mut request);
                        request.snippet.as_mut().unwrap().thumbnails.default.width = Some(arg_from_str(value.unwrap_or("-0"), err, "snippet.thumbnails.default.width", "integer"));
                    },
                "snippet.thumbnails.default.height" => {
                        request_snippet_init(&mut request);
                        request.snippet.as_mut().unwrap().thumbnails.default.height = Some(arg_from_str(value.unwrap_or("-0"), err, "snippet.thumbnails.default.height", "integer"));
                    },
                "snippet.thumbnails.high.url" => {
                        request_snippet_init(&mut request);
                        request.snippet.as_mut().unwrap().thumbnails.high.url = Some(value.unwrap_or("").to_string());
                    },
                "snippet.thumbnails.high.width" => {
                        request_snippet_init(&mut request);
                        request.snippet.as_mut().unwrap().thumbnails.high.width = Some(arg_from_str(value.unwrap_or("-0"), err, "snippet.thumbnails.high.width", "integer"));
                    },
                "snippet.thumbnails.high.height" => {
                        request_snippet_init(&mut request);
                        request.snippet.as_mut().unwrap().thumbnails.high.height = Some(arg_from_str(value.unwrap_or("-0"), err, "snippet.thumbnails.high.height", "integer"));
                    },
                "snippet.thumbnails.medium.url" => {
                        request_snippet_init(&mut request);
                        request.snippet.as_mut().unwrap().thumbnails.medium.url = Some(value.unwrap_or("").to_string());
                    },
                "snippet.thumbnails.medium.width" => {
                        request_snippet_init(&mut request);
                        request.snippet.as_mut().unwrap().thumbnails.medium.width = Some(arg_from_str(value.unwrap_or("-0"), err, "snippet.thumbnails.medium.width", "integer"));
                    },
                "snippet.thumbnails.medium.height" => {
                        request_snippet_init(&mut request);
                        request.snippet.as_mut().unwrap().thumbnails.medium.height = Some(arg_from_str(value.unwrap_or("-0"), err, "snippet.thumbnails.medium.height", "integer"));
                    },
                "snippet.thumbnails.maxres.url" => {
                        request_snippet_init(&mut request);
                        request.snippet.as_mut().unwrap().thumbnails.maxres.url = Some(value.unwrap_or("").to_string());
                    },
                "snippet.thumbnails.maxres.width" => {
                        request_snippet_init(&mut request);
                        request.snippet.as_mut().unwrap().thumbnails.maxres.width = Some(arg_from_str(value.unwrap_or("-0"), err, "snippet.thumbnails.maxres.width", "integer"));
                    },
                "snippet.thumbnails.maxres.height" => {
                        request_snippet_init(&mut request);
                        request.snippet.as_mut().unwrap().thumbnails.maxres.height = Some(arg_from_str(value.unwrap_or("-0"), err, "snippet.thumbnails.maxres.height", "integer"));
                    },
                "snippet.thumbnails.standard.url" => {
                        request_snippet_init(&mut request);
                        request.snippet.as_mut().unwrap().thumbnails.standard.url = Some(value.unwrap_or("").to_string());
                    },
                "snippet.thumbnails.standard.width" => {
                        request_snippet_init(&mut request);
                        request.snippet.as_mut().unwrap().thumbnails.standard.width = Some(arg_from_str(value.unwrap_or("-0"), err, "snippet.thumbnails.standard.width", "integer"));
                    },
                "snippet.thumbnails.standard.height" => {
                        request_snippet_init(&mut request);
                        request.snippet.as_mut().unwrap().thumbnails.standard.height = Some(arg_from_str(value.unwrap_or("-0"), err, "snippet.thumbnails.standard.height", "integer"));
                    },
                "player.embed-html" => {
                        request_player_init(&mut request);
                        request.player.as_mut().unwrap().embed_html = value.unwrap_or("").to_string();
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
                    err.issues.push(CLIError::Field(FieldError::Unknown(field_name.to_string())));
                }
            }
        }
        let protocol = "standard-request";
        if dry_run {
            None
        } else {
            assert!(err.issues.len() == 0);
            let mut ostream = writer_from_opts(self.opt.flag_o, &self.opt.arg_out);
            match match protocol {
                "standard-request" => call.doit(),
                _ => unreachable!(),
            } {
                Err(api_err) => Some(api_err),
                Ok((mut response, output_schema)) => {
                    println!("DEBUG: REMOVE ME {:?}", response);
                    serde::json::to_writer(&mut ostream, &output_schema).unwrap();
                    None
                }
            }
        }
    }

    fn _playlists_list(&self, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Option<api::Error> {
        let mut call = self.hub.playlists().list(&self.opt.arg_part);
        for parg in self.opt.arg_v.iter() {
            let (key, value) = parse_kv_arg(&*parg, err);
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
                "alt"
                |"fields"
                |"key"
                |"oauth-token"
                |"pretty-print"
                |"quota-user"
                |"user-ip" => {
                    let map = [
                        ("oauth-token", "oauth_token"),
                        ("pretty-print", "prettyPrint"),
                        ("quota-user", "quotaUser"),
                        ("user-ip", "userIp"),
                    ];
                    call = call.param(map.iter().find(|t| t.0 == key).unwrap_or(&("", key)).1, value.unwrap_or("unset"))
                },
                _ => err.issues.push(CLIError::UnknownParameter(key.to_string())),
            }
        }
        let protocol = "standard-request";
        if dry_run {
            None
        } else {
            assert!(err.issues.len() == 0);
            let mut ostream = writer_from_opts(self.opt.flag_o, &self.opt.arg_out);
            match match protocol {
                "standard-request" => call.doit(),
                _ => unreachable!(),
            } {
                Err(api_err) => Some(api_err),
                Ok((mut response, output_schema)) => {
                    println!("DEBUG: REMOVE ME {:?}", response);
                    serde::json::to_writer(&mut ostream, &output_schema).unwrap();
                    None
                }
            }
        }
    }

    fn _playlists_update(&self, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Option<api::Error> {
            let mut request: api::Playlist = Default::default();
        let mut call = self.hub.playlists().update(&request);
        for parg in self.opt.arg_v.iter() {
            let (key, value) = parse_kv_arg(&*parg, err);
            match key {
                "on-behalf-of-content-owner" => {
                    call = call.on_behalf_of_content_owner(value.unwrap_or(""));
                },
                "alt"
                |"fields"
                |"key"
                |"oauth-token"
                |"pretty-print"
                |"quota-user"
                |"user-ip" => {
                    let map = [
                        ("oauth-token", "oauth_token"),
                        ("pretty-print", "prettyPrint"),
                        ("quota-user", "quotaUser"),
                        ("user-ip", "userIp"),
                    ];
                    call = call.param(map.iter().find(|t| t.0 == key).unwrap_or(&("", key)).1, value.unwrap_or("unset"))
                },
                _ => err.issues.push(CLIError::UnknownParameter(key.to_string())),
            }
        }
        let mut field_name: FieldCursor = Default::default();
        for kvarg in self.opt.arg_kv.iter() {
            let (key, value) = parse_kv_arg(&*kvarg, err);
            if let Err(field_err) = field_name.set(&*key) {
                err.issues.push(field_err);
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
            
            fn request_status_init(request: &mut api::Playlist) {
                if request.status.is_none() {
                    request.status = Some(Default::default());
                }
            }
            
            match &field_name.to_string()[..] {
                "status.privacy-status" => {
                        request_status_init(&mut request);
                        request.status.as_mut().unwrap().privacy_status = value.unwrap_or("").to_string();
                    },
                "kind" => {
                        request_status_init(&mut request);
                        request.kind = Some(value.unwrap_or("").to_string());
                    },
                "content-details.item-count" => {
                        request_content_details_init(&mut request);
                        request.content_details.as_mut().unwrap().item_count = arg_from_str(value.unwrap_or("-0"), err, "content-details.item-count", "integer");
                    },
                "snippet.description" => {
                        request_snippet_init(&mut request);
                        request.snippet.as_mut().unwrap().description = value.unwrap_or("").to_string();
                    },
                "snippet.tags" => {
                        request_snippet_init(&mut request);
                        request.snippet.as_mut().unwrap().tags.push(value.unwrap_or("").to_string());
                    },
                "snippet.channel-id" => {
                        request_snippet_init(&mut request);
                        request.snippet.as_mut().unwrap().channel_id = value.unwrap_or("").to_string();
                    },
                "snippet.published-at" => {
                        request_snippet_init(&mut request);
                        request.snippet.as_mut().unwrap().published_at = value.unwrap_or("").to_string();
                    },
                "snippet.channel-title" => {
                        request_snippet_init(&mut request);
                        request.snippet.as_mut().unwrap().channel_title = value.unwrap_or("").to_string();
                    },
                "snippet.title" => {
                        request_snippet_init(&mut request);
                        request.snippet.as_mut().unwrap().title = value.unwrap_or("").to_string();
                    },
                "snippet.default-language" => {
                        request_snippet_init(&mut request);
                        request.snippet.as_mut().unwrap().default_language = value.unwrap_or("").to_string();
                    },
                "snippet.localized.description" => {
                        request_snippet_init(&mut request);
                        request.snippet.as_mut().unwrap().localized.description = value.unwrap_or("").to_string();
                    },
                "snippet.localized.title" => {
                        request_snippet_init(&mut request);
                        request.snippet.as_mut().unwrap().localized.title = value.unwrap_or("").to_string();
                    },
                "snippet.thumbnails.default.url" => {
                        request_snippet_init(&mut request);
                        request.snippet.as_mut().unwrap().thumbnails.default.url = Some(value.unwrap_or("").to_string());
                    },
                "snippet.thumbnails.default.width" => {
                        request_snippet_init(&mut request);
                        request.snippet.as_mut().unwrap().thumbnails.default.width = Some(arg_from_str(value.unwrap_or("-0"), err, "snippet.thumbnails.default.width", "integer"));
                    },
                "snippet.thumbnails.default.height" => {
                        request_snippet_init(&mut request);
                        request.snippet.as_mut().unwrap().thumbnails.default.height = Some(arg_from_str(value.unwrap_or("-0"), err, "snippet.thumbnails.default.height", "integer"));
                    },
                "snippet.thumbnails.high.url" => {
                        request_snippet_init(&mut request);
                        request.snippet.as_mut().unwrap().thumbnails.high.url = Some(value.unwrap_or("").to_string());
                    },
                "snippet.thumbnails.high.width" => {
                        request_snippet_init(&mut request);
                        request.snippet.as_mut().unwrap().thumbnails.high.width = Some(arg_from_str(value.unwrap_or("-0"), err, "snippet.thumbnails.high.width", "integer"));
                    },
                "snippet.thumbnails.high.height" => {
                        request_snippet_init(&mut request);
                        request.snippet.as_mut().unwrap().thumbnails.high.height = Some(arg_from_str(value.unwrap_or("-0"), err, "snippet.thumbnails.high.height", "integer"));
                    },
                "snippet.thumbnails.medium.url" => {
                        request_snippet_init(&mut request);
                        request.snippet.as_mut().unwrap().thumbnails.medium.url = Some(value.unwrap_or("").to_string());
                    },
                "snippet.thumbnails.medium.width" => {
                        request_snippet_init(&mut request);
                        request.snippet.as_mut().unwrap().thumbnails.medium.width = Some(arg_from_str(value.unwrap_or("-0"), err, "snippet.thumbnails.medium.width", "integer"));
                    },
                "snippet.thumbnails.medium.height" => {
                        request_snippet_init(&mut request);
                        request.snippet.as_mut().unwrap().thumbnails.medium.height = Some(arg_from_str(value.unwrap_or("-0"), err, "snippet.thumbnails.medium.height", "integer"));
                    },
                "snippet.thumbnails.maxres.url" => {
                        request_snippet_init(&mut request);
                        request.snippet.as_mut().unwrap().thumbnails.maxres.url = Some(value.unwrap_or("").to_string());
                    },
                "snippet.thumbnails.maxres.width" => {
                        request_snippet_init(&mut request);
                        request.snippet.as_mut().unwrap().thumbnails.maxres.width = Some(arg_from_str(value.unwrap_or("-0"), err, "snippet.thumbnails.maxres.width", "integer"));
                    },
                "snippet.thumbnails.maxres.height" => {
                        request_snippet_init(&mut request);
                        request.snippet.as_mut().unwrap().thumbnails.maxres.height = Some(arg_from_str(value.unwrap_or("-0"), err, "snippet.thumbnails.maxres.height", "integer"));
                    },
                "snippet.thumbnails.standard.url" => {
                        request_snippet_init(&mut request);
                        request.snippet.as_mut().unwrap().thumbnails.standard.url = Some(value.unwrap_or("").to_string());
                    },
                "snippet.thumbnails.standard.width" => {
                        request_snippet_init(&mut request);
                        request.snippet.as_mut().unwrap().thumbnails.standard.width = Some(arg_from_str(value.unwrap_or("-0"), err, "snippet.thumbnails.standard.width", "integer"));
                    },
                "snippet.thumbnails.standard.height" => {
                        request_snippet_init(&mut request);
                        request.snippet.as_mut().unwrap().thumbnails.standard.height = Some(arg_from_str(value.unwrap_or("-0"), err, "snippet.thumbnails.standard.height", "integer"));
                    },
                "player.embed-html" => {
                        request_player_init(&mut request);
                        request.player.as_mut().unwrap().embed_html = value.unwrap_or("").to_string();
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
                    err.issues.push(CLIError::Field(FieldError::Unknown(field_name.to_string())));
                }
            }
        }
        let protocol = "standard-request";
        if dry_run {
            None
        } else {
            assert!(err.issues.len() == 0);
            let mut ostream = writer_from_opts(self.opt.flag_o, &self.opt.arg_out);
            match match protocol {
                "standard-request" => call.doit(),
                _ => unreachable!(),
            } {
                Err(api_err) => Some(api_err),
                Ok((mut response, output_schema)) => {
                    println!("DEBUG: REMOVE ME {:?}", response);
                    serde::json::to_writer(&mut ostream, &output_schema).unwrap();
                    None
                }
            }
        }
    }

    fn _search_list(&self, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Option<api::Error> {
        let mut call = self.hub.search().list(&self.opt.arg_part);
        for parg in self.opt.arg_v.iter() {
            let (key, value) = parse_kv_arg(&*parg, err);
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
                "alt"
                |"fields"
                |"key"
                |"oauth-token"
                |"pretty-print"
                |"quota-user"
                |"user-ip" => {
                    let map = [
                        ("oauth-token", "oauth_token"),
                        ("pretty-print", "prettyPrint"),
                        ("quota-user", "quotaUser"),
                        ("user-ip", "userIp"),
                    ];
                    call = call.param(map.iter().find(|t| t.0 == key).unwrap_or(&("", key)).1, value.unwrap_or("unset"))
                },
                _ => err.issues.push(CLIError::UnknownParameter(key.to_string())),
            }
        }
        let protocol = "standard-request";
        if dry_run {
            None
        } else {
            assert!(err.issues.len() == 0);
            let mut ostream = writer_from_opts(self.opt.flag_o, &self.opt.arg_out);
            match match protocol {
                "standard-request" => call.doit(),
                _ => unreachable!(),
            } {
                Err(api_err) => Some(api_err),
                Ok((mut response, output_schema)) => {
                    println!("DEBUG: REMOVE ME {:?}", response);
                    serde::json::to_writer(&mut ostream, &output_schema).unwrap();
                    None
                }
            }
        }
    }

    fn _subscriptions_delete(&self, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Option<api::Error> {
        let mut call = self.hub.subscriptions().delete(&self.opt.arg_id);
        for parg in self.opt.arg_v.iter() {
            let (key, value) = parse_kv_arg(&*parg, err);
            match key {
                "alt"
                |"fields"
                |"key"
                |"oauth-token"
                |"pretty-print"
                |"quota-user"
                |"user-ip" => {
                    let map = [
                        ("oauth-token", "oauth_token"),
                        ("pretty-print", "prettyPrint"),
                        ("quota-user", "quotaUser"),
                        ("user-ip", "userIp"),
                    ];
                    call = call.param(map.iter().find(|t| t.0 == key).unwrap_or(&("", key)).1, value.unwrap_or("unset"))
                },
                _ => err.issues.push(CLIError::UnknownParameter(key.to_string())),
            }
        }
        let protocol = "standard-request";
        if dry_run {
            None
        } else {
            assert!(err.issues.len() == 0);
            match match protocol {
                "standard-request" => call.doit(),
                _ => unreachable!(),
            } {
                Err(api_err) => Some(api_err),
                Ok(mut response) => {
                    println!("DEBUG: REMOVE ME {:?}", response);
                    None
                }
            }
        }
    }

    fn _subscriptions_insert(&self, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Option<api::Error> {
            let mut request: api::Subscription = Default::default();
        let mut call = self.hub.subscriptions().insert(&request);
        for parg in self.opt.arg_v.iter() {
            let (key, value) = parse_kv_arg(&*parg, err);
            match key {
                "alt"
                |"fields"
                |"key"
                |"oauth-token"
                |"pretty-print"
                |"quota-user"
                |"user-ip" => {
                    let map = [
                        ("oauth-token", "oauth_token"),
                        ("pretty-print", "prettyPrint"),
                        ("quota-user", "quotaUser"),
                        ("user-ip", "userIp"),
                    ];
                    call = call.param(map.iter().find(|t| t.0 == key).unwrap_or(&("", key)).1, value.unwrap_or("unset"))
                },
                _ => err.issues.push(CLIError::UnknownParameter(key.to_string())),
            }
        }
        let mut field_name: FieldCursor = Default::default();
        for kvarg in self.opt.arg_kv.iter() {
            let (key, value) = parse_kv_arg(&*kvarg, err);
            if let Err(field_err) = field_name.set(&*key) {
                err.issues.push(field_err);
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
            
            fn request_subscriber_snippet_init(request: &mut api::Subscription) {
                if request.subscriber_snippet.is_none() {
                    request.subscriber_snippet = Some(Default::default());
                }
            }
            
            match &field_name.to_string()[..] {
                "kind" => {
                        request.kind = Some(value.unwrap_or("").to_string());
                    },
                "content-details.new-item-count" => {
                        request_content_details_init(&mut request);
                        request.content_details.as_mut().unwrap().new_item_count = arg_from_str(value.unwrap_or("-0"), err, "content-details.new-item-count", "integer");
                    },
                "content-details.activity-type" => {
                        request_content_details_init(&mut request);
                        request.content_details.as_mut().unwrap().activity_type = value.unwrap_or("").to_string();
                    },
                "content-details.total-item-count" => {
                        request_content_details_init(&mut request);
                        request.content_details.as_mut().unwrap().total_item_count = arg_from_str(value.unwrap_or("-0"), err, "content-details.total-item-count", "integer");
                    },
                "snippet.description" => {
                        request_snippet_init(&mut request);
                        request.snippet.as_mut().unwrap().description = value.unwrap_or("").to_string();
                    },
                "snippet.title" => {
                        request_snippet_init(&mut request);
                        request.snippet.as_mut().unwrap().title = value.unwrap_or("").to_string();
                    },
                "snippet.resource-id.kind" => {
                        request_snippet_init(&mut request);
                        request.snippet.as_mut().unwrap().resource_id.kind = value.unwrap_or("").to_string();
                    },
                "snippet.resource-id.channel-id" => {
                        request_snippet_init(&mut request);
                        request.snippet.as_mut().unwrap().resource_id.channel_id = value.unwrap_or("").to_string();
                    },
                "snippet.resource-id.playlist-id" => {
                        request_snippet_init(&mut request);
                        request.snippet.as_mut().unwrap().resource_id.playlist_id = value.unwrap_or("").to_string();
                    },
                "snippet.resource-id.video-id" => {
                        request_snippet_init(&mut request);
                        request.snippet.as_mut().unwrap().resource_id.video_id = value.unwrap_or("").to_string();
                    },
                "snippet.channel-id" => {
                        request_snippet_init(&mut request);
                        request.snippet.as_mut().unwrap().channel_id = value.unwrap_or("").to_string();
                    },
                "snippet.published-at" => {
                        request_snippet_init(&mut request);
                        request.snippet.as_mut().unwrap().published_at = value.unwrap_or("").to_string();
                    },
                "snippet.channel-title" => {
                        request_snippet_init(&mut request);
                        request.snippet.as_mut().unwrap().channel_title = value.unwrap_or("").to_string();
                    },
                "snippet.thumbnails.default.url" => {
                        request_snippet_init(&mut request);
                        request.snippet.as_mut().unwrap().thumbnails.default.url = Some(value.unwrap_or("").to_string());
                    },
                "snippet.thumbnails.default.width" => {
                        request_snippet_init(&mut request);
                        request.snippet.as_mut().unwrap().thumbnails.default.width = Some(arg_from_str(value.unwrap_or("-0"), err, "snippet.thumbnails.default.width", "integer"));
                    },
                "snippet.thumbnails.default.height" => {
                        request_snippet_init(&mut request);
                        request.snippet.as_mut().unwrap().thumbnails.default.height = Some(arg_from_str(value.unwrap_or("-0"), err, "snippet.thumbnails.default.height", "integer"));
                    },
                "snippet.thumbnails.high.url" => {
                        request_snippet_init(&mut request);
                        request.snippet.as_mut().unwrap().thumbnails.high.url = Some(value.unwrap_or("").to_string());
                    },
                "snippet.thumbnails.high.width" => {
                        request_snippet_init(&mut request);
                        request.snippet.as_mut().unwrap().thumbnails.high.width = Some(arg_from_str(value.unwrap_or("-0"), err, "snippet.thumbnails.high.width", "integer"));
                    },
                "snippet.thumbnails.high.height" => {
                        request_snippet_init(&mut request);
                        request.snippet.as_mut().unwrap().thumbnails.high.height = Some(arg_from_str(value.unwrap_or("-0"), err, "snippet.thumbnails.high.height", "integer"));
                    },
                "snippet.thumbnails.medium.url" => {
                        request_snippet_init(&mut request);
                        request.snippet.as_mut().unwrap().thumbnails.medium.url = Some(value.unwrap_or("").to_string());
                    },
                "snippet.thumbnails.medium.width" => {
                        request_snippet_init(&mut request);
                        request.snippet.as_mut().unwrap().thumbnails.medium.width = Some(arg_from_str(value.unwrap_or("-0"), err, "snippet.thumbnails.medium.width", "integer"));
                    },
                "snippet.thumbnails.medium.height" => {
                        request_snippet_init(&mut request);
                        request.snippet.as_mut().unwrap().thumbnails.medium.height = Some(arg_from_str(value.unwrap_or("-0"), err, "snippet.thumbnails.medium.height", "integer"));
                    },
                "snippet.thumbnails.maxres.url" => {
                        request_snippet_init(&mut request);
                        request.snippet.as_mut().unwrap().thumbnails.maxres.url = Some(value.unwrap_or("").to_string());
                    },
                "snippet.thumbnails.maxres.width" => {
                        request_snippet_init(&mut request);
                        request.snippet.as_mut().unwrap().thumbnails.maxres.width = Some(arg_from_str(value.unwrap_or("-0"), err, "snippet.thumbnails.maxres.width", "integer"));
                    },
                "snippet.thumbnails.maxres.height" => {
                        request_snippet_init(&mut request);
                        request.snippet.as_mut().unwrap().thumbnails.maxres.height = Some(arg_from_str(value.unwrap_or("-0"), err, "snippet.thumbnails.maxres.height", "integer"));
                    },
                "snippet.thumbnails.standard.url" => {
                        request_snippet_init(&mut request);
                        request.snippet.as_mut().unwrap().thumbnails.standard.url = Some(value.unwrap_or("").to_string());
                    },
                "snippet.thumbnails.standard.width" => {
                        request_snippet_init(&mut request);
                        request.snippet.as_mut().unwrap().thumbnails.standard.width = Some(arg_from_str(value.unwrap_or("-0"), err, "snippet.thumbnails.standard.width", "integer"));
                    },
                "snippet.thumbnails.standard.height" => {
                        request_snippet_init(&mut request);
                        request.snippet.as_mut().unwrap().thumbnails.standard.height = Some(arg_from_str(value.unwrap_or("-0"), err, "snippet.thumbnails.standard.height", "integer"));
                    },
                "etag" => {
                        request_snippet_init(&mut request);
                        request.etag = Some(value.unwrap_or("").to_string());
                    },
                "subscriber-snippet.title" => {
                        request_subscriber_snippet_init(&mut request);
                        request.subscriber_snippet.as_mut().unwrap().title = value.unwrap_or("").to_string();
                    },
                "subscriber-snippet.channel-id" => {
                        request_subscriber_snippet_init(&mut request);
                        request.subscriber_snippet.as_mut().unwrap().channel_id = value.unwrap_or("").to_string();
                    },
                "subscriber-snippet.description" => {
                        request_subscriber_snippet_init(&mut request);
                        request.subscriber_snippet.as_mut().unwrap().description = value.unwrap_or("").to_string();
                    },
                "subscriber-snippet.thumbnails.default.url" => {
                        request_subscriber_snippet_init(&mut request);
                        request.subscriber_snippet.as_mut().unwrap().thumbnails.default.url = Some(value.unwrap_or("").to_string());
                    },
                "subscriber-snippet.thumbnails.default.width" => {
                        request_subscriber_snippet_init(&mut request);
                        request.subscriber_snippet.as_mut().unwrap().thumbnails.default.width = Some(arg_from_str(value.unwrap_or("-0"), err, "subscriber-snippet.thumbnails.default.width", "integer"));
                    },
                "subscriber-snippet.thumbnails.default.height" => {
                        request_subscriber_snippet_init(&mut request);
                        request.subscriber_snippet.as_mut().unwrap().thumbnails.default.height = Some(arg_from_str(value.unwrap_or("-0"), err, "subscriber-snippet.thumbnails.default.height", "integer"));
                    },
                "subscriber-snippet.thumbnails.high.url" => {
                        request_subscriber_snippet_init(&mut request);
                        request.subscriber_snippet.as_mut().unwrap().thumbnails.high.url = Some(value.unwrap_or("").to_string());
                    },
                "subscriber-snippet.thumbnails.high.width" => {
                        request_subscriber_snippet_init(&mut request);
                        request.subscriber_snippet.as_mut().unwrap().thumbnails.high.width = Some(arg_from_str(value.unwrap_or("-0"), err, "subscriber-snippet.thumbnails.high.width", "integer"));
                    },
                "subscriber-snippet.thumbnails.high.height" => {
                        request_subscriber_snippet_init(&mut request);
                        request.subscriber_snippet.as_mut().unwrap().thumbnails.high.height = Some(arg_from_str(value.unwrap_or("-0"), err, "subscriber-snippet.thumbnails.high.height", "integer"));
                    },
                "subscriber-snippet.thumbnails.medium.url" => {
                        request_subscriber_snippet_init(&mut request);
                        request.subscriber_snippet.as_mut().unwrap().thumbnails.medium.url = Some(value.unwrap_or("").to_string());
                    },
                "subscriber-snippet.thumbnails.medium.width" => {
                        request_subscriber_snippet_init(&mut request);
                        request.subscriber_snippet.as_mut().unwrap().thumbnails.medium.width = Some(arg_from_str(value.unwrap_or("-0"), err, "subscriber-snippet.thumbnails.medium.width", "integer"));
                    },
                "subscriber-snippet.thumbnails.medium.height" => {
                        request_subscriber_snippet_init(&mut request);
                        request.subscriber_snippet.as_mut().unwrap().thumbnails.medium.height = Some(arg_from_str(value.unwrap_or("-0"), err, "subscriber-snippet.thumbnails.medium.height", "integer"));
                    },
                "subscriber-snippet.thumbnails.maxres.url" => {
                        request_subscriber_snippet_init(&mut request);
                        request.subscriber_snippet.as_mut().unwrap().thumbnails.maxres.url = Some(value.unwrap_or("").to_string());
                    },
                "subscriber-snippet.thumbnails.maxres.width" => {
                        request_subscriber_snippet_init(&mut request);
                        request.subscriber_snippet.as_mut().unwrap().thumbnails.maxres.width = Some(arg_from_str(value.unwrap_or("-0"), err, "subscriber-snippet.thumbnails.maxres.width", "integer"));
                    },
                "subscriber-snippet.thumbnails.maxres.height" => {
                        request_subscriber_snippet_init(&mut request);
                        request.subscriber_snippet.as_mut().unwrap().thumbnails.maxres.height = Some(arg_from_str(value.unwrap_or("-0"), err, "subscriber-snippet.thumbnails.maxres.height", "integer"));
                    },
                "subscriber-snippet.thumbnails.standard.url" => {
                        request_subscriber_snippet_init(&mut request);
                        request.subscriber_snippet.as_mut().unwrap().thumbnails.standard.url = Some(value.unwrap_or("").to_string());
                    },
                "subscriber-snippet.thumbnails.standard.width" => {
                        request_subscriber_snippet_init(&mut request);
                        request.subscriber_snippet.as_mut().unwrap().thumbnails.standard.width = Some(arg_from_str(value.unwrap_or("-0"), err, "subscriber-snippet.thumbnails.standard.width", "integer"));
                    },
                "subscriber-snippet.thumbnails.standard.height" => {
                        request_subscriber_snippet_init(&mut request);
                        request.subscriber_snippet.as_mut().unwrap().thumbnails.standard.height = Some(arg_from_str(value.unwrap_or("-0"), err, "subscriber-snippet.thumbnails.standard.height", "integer"));
                    },
                "id" => {
                        request_subscriber_snippet_init(&mut request);
                        request.id = Some(value.unwrap_or("").to_string());
                    },
                _ => {
                    err.issues.push(CLIError::Field(FieldError::Unknown(field_name.to_string())));
                }
            }
        }
        let protocol = "standard-request";
        if dry_run {
            None
        } else {
            assert!(err.issues.len() == 0);
            let mut ostream = writer_from_opts(self.opt.flag_o, &self.opt.arg_out);
            match match protocol {
                "standard-request" => call.doit(),
                _ => unreachable!(),
            } {
                Err(api_err) => Some(api_err),
                Ok((mut response, output_schema)) => {
                    println!("DEBUG: REMOVE ME {:?}", response);
                    serde::json::to_writer(&mut ostream, &output_schema).unwrap();
                    None
                }
            }
        }
    }

    fn _subscriptions_list(&self, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Option<api::Error> {
        let mut call = self.hub.subscriptions().list(&self.opt.arg_part);
        for parg in self.opt.arg_v.iter() {
            let (key, value) = parse_kv_arg(&*parg, err);
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
                "alt"
                |"fields"
                |"key"
                |"oauth-token"
                |"pretty-print"
                |"quota-user"
                |"user-ip" => {
                    let map = [
                        ("oauth-token", "oauth_token"),
                        ("pretty-print", "prettyPrint"),
                        ("quota-user", "quotaUser"),
                        ("user-ip", "userIp"),
                    ];
                    call = call.param(map.iter().find(|t| t.0 == key).unwrap_or(&("", key)).1, value.unwrap_or("unset"))
                },
                _ => err.issues.push(CLIError::UnknownParameter(key.to_string())),
            }
        }
        let protocol = "standard-request";
        if dry_run {
            None
        } else {
            assert!(err.issues.len() == 0);
            let mut ostream = writer_from_opts(self.opt.flag_o, &self.opt.arg_out);
            match match protocol {
                "standard-request" => call.doit(),
                _ => unreachable!(),
            } {
                Err(api_err) => Some(api_err),
                Ok((mut response, output_schema)) => {
                    println!("DEBUG: REMOVE ME {:?}", response);
                    serde::json::to_writer(&mut ostream, &output_schema).unwrap();
                    None
                }
            }
        }
    }

    fn _thumbnails_set(&self, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Option<api::Error> {
        let mut call = self.hub.thumbnails().set(&self.opt.arg_video_id);
        for parg in self.opt.arg_v.iter() {
            let (key, value) = parse_kv_arg(&*parg, err);
            match key {
                "on-behalf-of-content-owner" => {
                    call = call.on_behalf_of_content_owner(value.unwrap_or(""));
                },
                "alt"
                |"fields"
                |"key"
                |"oauth-token"
                |"pretty-print"
                |"quota-user"
                |"user-ip" => {
                    let map = [
                        ("oauth-token", "oauth_token"),
                        ("pretty-print", "prettyPrint"),
                        ("quota-user", "quotaUser"),
                        ("user-ip", "userIp"),
                    ];
                    call = call.param(map.iter().find(|t| t.0 == key).unwrap_or(&("", key)).1, value.unwrap_or("unset"))
                },
                _ => err.issues.push(CLIError::UnknownParameter(key.to_string())),
            }
        }
        let protocol = 
            if self.opt.cmd_simple {
                "simple"
            } else if self.opt.cmd_resumable {
                "resumable"
            } else { 
                unreachable!() 
            };
        let mut input_file = input_file_from_opts(&self.opt.arg_file, err);
        let mime_type = input_mime_from_opts(&self.opt.arg_mime, err);
        if dry_run {
            None
        } else {
            assert!(err.issues.len() == 0);
            let mut ostream = writer_from_opts(self.opt.flag_o, &self.opt.arg_out);
            match match protocol {
                "simple" => call.upload(input_file.unwrap(), mime_type.unwrap()),
                "resumable" => call.upload_resumable(input_file.unwrap(), mime_type.unwrap()),
                _ => unreachable!(),
            } {
                Err(api_err) => Some(api_err),
                Ok((mut response, output_schema)) => {
                    println!("DEBUG: REMOVE ME {:?}", response);
                    serde::json::to_writer(&mut ostream, &output_schema).unwrap();
                    None
                }
            }
        }
    }

    fn _video_categories_list(&self, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Option<api::Error> {
        let mut call = self.hub.video_categories().list(&self.opt.arg_part);
        for parg in self.opt.arg_v.iter() {
            let (key, value) = parse_kv_arg(&*parg, err);
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
                "alt"
                |"fields"
                |"key"
                |"oauth-token"
                |"pretty-print"
                |"quota-user"
                |"user-ip" => {
                    let map = [
                        ("oauth-token", "oauth_token"),
                        ("pretty-print", "prettyPrint"),
                        ("quota-user", "quotaUser"),
                        ("user-ip", "userIp"),
                    ];
                    call = call.param(map.iter().find(|t| t.0 == key).unwrap_or(&("", key)).1, value.unwrap_or("unset"))
                },
                _ => err.issues.push(CLIError::UnknownParameter(key.to_string())),
            }
        }
        let protocol = "standard-request";
        if dry_run {
            None
        } else {
            assert!(err.issues.len() == 0);
            let mut ostream = writer_from_opts(self.opt.flag_o, &self.opt.arg_out);
            match match protocol {
                "standard-request" => call.doit(),
                _ => unreachable!(),
            } {
                Err(api_err) => Some(api_err),
                Ok((mut response, output_schema)) => {
                    println!("DEBUG: REMOVE ME {:?}", response);
                    serde::json::to_writer(&mut ostream, &output_schema).unwrap();
                    None
                }
            }
        }
    }

    fn _videos_delete(&self, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Option<api::Error> {
        let mut call = self.hub.videos().delete(&self.opt.arg_id);
        for parg in self.opt.arg_v.iter() {
            let (key, value) = parse_kv_arg(&*parg, err);
            match key {
                "on-behalf-of-content-owner" => {
                    call = call.on_behalf_of_content_owner(value.unwrap_or(""));
                },
                "alt"
                |"fields"
                |"key"
                |"oauth-token"
                |"pretty-print"
                |"quota-user"
                |"user-ip" => {
                    let map = [
                        ("oauth-token", "oauth_token"),
                        ("pretty-print", "prettyPrint"),
                        ("quota-user", "quotaUser"),
                        ("user-ip", "userIp"),
                    ];
                    call = call.param(map.iter().find(|t| t.0 == key).unwrap_or(&("", key)).1, value.unwrap_or("unset"))
                },
                _ => err.issues.push(CLIError::UnknownParameter(key.to_string())),
            }
        }
        let protocol = "standard-request";
        if dry_run {
            None
        } else {
            assert!(err.issues.len() == 0);
            match match protocol {
                "standard-request" => call.doit(),
                _ => unreachable!(),
            } {
                Err(api_err) => Some(api_err),
                Ok(mut response) => {
                    println!("DEBUG: REMOVE ME {:?}", response);
                    None
                }
            }
        }
    }

    fn _videos_get_rating(&self, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Option<api::Error> {
        let mut call = self.hub.videos().get_rating(&self.opt.arg_id);
        for parg in self.opt.arg_v.iter() {
            let (key, value) = parse_kv_arg(&*parg, err);
            match key {
                "on-behalf-of-content-owner" => {
                    call = call.on_behalf_of_content_owner(value.unwrap_or(""));
                },
                "alt"
                |"fields"
                |"key"
                |"oauth-token"
                |"pretty-print"
                |"quota-user"
                |"user-ip" => {
                    let map = [
                        ("oauth-token", "oauth_token"),
                        ("pretty-print", "prettyPrint"),
                        ("quota-user", "quotaUser"),
                        ("user-ip", "userIp"),
                    ];
                    call = call.param(map.iter().find(|t| t.0 == key).unwrap_or(&("", key)).1, value.unwrap_or("unset"))
                },
                _ => err.issues.push(CLIError::UnknownParameter(key.to_string())),
            }
        }
        let protocol = "standard-request";
        if dry_run {
            None
        } else {
            assert!(err.issues.len() == 0);
            let mut ostream = writer_from_opts(self.opt.flag_o, &self.opt.arg_out);
            match match protocol {
                "standard-request" => call.doit(),
                _ => unreachable!(),
            } {
                Err(api_err) => Some(api_err),
                Ok((mut response, output_schema)) => {
                    println!("DEBUG: REMOVE ME {:?}", response);
                    serde::json::to_writer(&mut ostream, &output_schema).unwrap();
                    None
                }
            }
        }
    }

    fn _videos_insert(&self, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Option<api::Error> {
            let mut request: api::Video = Default::default();
        let mut call = self.hub.videos().insert(&request);
        for parg in self.opt.arg_v.iter() {
            let (key, value) = parse_kv_arg(&*parg, err);
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
                "alt"
                |"fields"
                |"key"
                |"oauth-token"
                |"pretty-print"
                |"quota-user"
                |"user-ip" => {
                    let map = [
                        ("oauth-token", "oauth_token"),
                        ("pretty-print", "prettyPrint"),
                        ("quota-user", "quotaUser"),
                        ("user-ip", "userIp"),
                    ];
                    call = call.param(map.iter().find(|t| t.0 == key).unwrap_or(&("", key)).1, value.unwrap_or("unset"))
                },
                _ => err.issues.push(CLIError::UnknownParameter(key.to_string())),
            }
        }
        let mut field_name: FieldCursor = Default::default();
        for kvarg in self.opt.arg_kv.iter() {
            let (key, value) = parse_kv_arg(&*kvarg, err);
            if let Err(field_err) = field_name.set(&*key) {
                err.issues.push(field_err);
            }
            fn request_age_gating_init(request: &mut api::Video) {
                if request.age_gating.is_none() {
                    request.age_gating = Some(Default::default());
                }
            }
            
            fn request_content_details_init(request: &mut api::Video) {
                if request.content_details.is_none() {
                    request.content_details = Some(Default::default());
                }
            }
            
            fn request_file_details_init(request: &mut api::Video) {
                if request.file_details.is_none() {
                    request.file_details = Some(Default::default());
                }
            }
            
            fn request_live_streaming_details_init(request: &mut api::Video) {
                if request.live_streaming_details.is_none() {
                    request.live_streaming_details = Some(Default::default());
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
            
            fn request_snippet_init(request: &mut api::Video) {
                if request.snippet.is_none() {
                    request.snippet = Some(Default::default());
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
            
            match &field_name.to_string()[..] {
                "status.license" => {
                        request_status_init(&mut request);
                        request.status.as_mut().unwrap().license = value.unwrap_or("").to_string();
                    },
                "status.embeddable" => {
                        request_status_init(&mut request);
                        request.status.as_mut().unwrap().embeddable = arg_from_str(value.unwrap_or("false"), err, "status.embeddable", "boolean");
                    },
                "status.privacy-status" => {
                        request_status_init(&mut request);
                        request.status.as_mut().unwrap().privacy_status = value.unwrap_or("").to_string();
                    },
                "status.publish-at" => {
                        request_status_init(&mut request);
                        request.status.as_mut().unwrap().publish_at = value.unwrap_or("").to_string();
                    },
                "status.public-stats-viewable" => {
                        request_status_init(&mut request);
                        request.status.as_mut().unwrap().public_stats_viewable = arg_from_str(value.unwrap_or("false"), err, "status.public-stats-viewable", "boolean");
                    },
                "status.upload-status" => {
                        request_status_init(&mut request);
                        request.status.as_mut().unwrap().upload_status = value.unwrap_or("").to_string();
                    },
                "status.rejection-reason" => {
                        request_status_init(&mut request);
                        request.status.as_mut().unwrap().rejection_reason = value.unwrap_or("").to_string();
                    },
                "status.failure-reason" => {
                        request_status_init(&mut request);
                        request.status.as_mut().unwrap().failure_reason = value.unwrap_or("").to_string();
                    },
                "topic-details.topic-ids" => {
                        request_topic_details_init(&mut request);
                        request.topic_details.as_mut().unwrap().topic_ids.push(value.unwrap_or("").to_string());
                    },
                "topic-details.relevant-topic-ids" => {
                        request_topic_details_init(&mut request);
                        request.topic_details.as_mut().unwrap().relevant_topic_ids.push(value.unwrap_or("").to_string());
                    },
                "kind" => {
                        request_topic_details_init(&mut request);
                        request.kind = Some(value.unwrap_or("").to_string());
                    },
                "statistics.comment-count" => {
                        request_statistics_init(&mut request);
                        request.statistics.as_mut().unwrap().comment_count = arg_from_str(value.unwrap_or("-0"), err, "statistics.comment-count", "int64");
                    },
                "statistics.view-count" => {
                        request_statistics_init(&mut request);
                        request.statistics.as_mut().unwrap().view_count = arg_from_str(value.unwrap_or("-0"), err, "statistics.view-count", "int64");
                    },
                "statistics.favorite-count" => {
                        request_statistics_init(&mut request);
                        request.statistics.as_mut().unwrap().favorite_count = arg_from_str(value.unwrap_or("-0"), err, "statistics.favorite-count", "int64");
                    },
                "statistics.dislike-count" => {
                        request_statistics_init(&mut request);
                        request.statistics.as_mut().unwrap().dislike_count = arg_from_str(value.unwrap_or("-0"), err, "statistics.dislike-count", "int64");
                    },
                "statistics.like-count" => {
                        request_statistics_init(&mut request);
                        request.statistics.as_mut().unwrap().like_count = arg_from_str(value.unwrap_or("-0"), err, "statistics.like-count", "int64");
                    },
                "content-details.definition" => {
                        request_content_details_init(&mut request);
                        request.content_details.as_mut().unwrap().definition = value.unwrap_or("").to_string();
                    },
                "content-details.country-restriction.exception" => {
                        request_content_details_init(&mut request);
                        request.content_details.as_mut().unwrap().country_restriction.exception.push(value.unwrap_or("").to_string());
                    },
                "content-details.country-restriction.allowed" => {
                        request_content_details_init(&mut request);
                        request.content_details.as_mut().unwrap().country_restriction.allowed = arg_from_str(value.unwrap_or("false"), err, "content-details.country-restriction.allowed", "boolean");
                    },
                "content-details.content-rating.yt-rating" => {
                        request_content_details_init(&mut request);
                        request.content_details.as_mut().unwrap().content_rating.yt_rating = value.unwrap_or("").to_string();
                    },
                "content-details.content-rating.catvfr-rating" => {
                        request_content_details_init(&mut request);
                        request.content_details.as_mut().unwrap().content_rating.catvfr_rating = value.unwrap_or("").to_string();
                    },
                "content-details.content-rating.cbfc-rating" => {
                        request_content_details_init(&mut request);
                        request.content_details.as_mut().unwrap().content_rating.cbfc_rating = value.unwrap_or("").to_string();
                    },
                "content-details.content-rating.bfvc-rating" => {
                        request_content_details_init(&mut request);
                        request.content_details.as_mut().unwrap().content_rating.bfvc_rating = value.unwrap_or("").to_string();
                    },
                "content-details.content-rating.mda-rating" => {
                        request_content_details_init(&mut request);
                        request.content_details.as_mut().unwrap().content_rating.mda_rating = value.unwrap_or("").to_string();
                    },
                "content-details.content-rating.acb-rating" => {
                        request_content_details_init(&mut request);
                        request.content_details.as_mut().unwrap().content_rating.acb_rating = value.unwrap_or("").to_string();
                    },
                "content-details.content-rating.nfvcb-rating" => {
                        request_content_details_init(&mut request);
                        request.content_details.as_mut().unwrap().content_rating.nfvcb_rating = value.unwrap_or("").to_string();
                    },
                "content-details.content-rating.bmukk-rating" => {
                        request_content_details_init(&mut request);
                        request.content_details.as_mut().unwrap().content_rating.bmukk_rating = value.unwrap_or("").to_string();
                    },
                "content-details.content-rating.chfilm-rating" => {
                        request_content_details_init(&mut request);
                        request.content_details.as_mut().unwrap().content_rating.chfilm_rating = value.unwrap_or("").to_string();
                    },
                "content-details.content-rating.resorteviolencia-rating" => {
                        request_content_details_init(&mut request);
                        request.content_details.as_mut().unwrap().content_rating.resorteviolencia_rating = value.unwrap_or("").to_string();
                    },
                "content-details.content-rating.csa-rating" => {
                        request_content_details_init(&mut request);
                        request.content_details.as_mut().unwrap().content_rating.csa_rating = value.unwrap_or("").to_string();
                    },
                "content-details.content-rating.moctw-rating" => {
                        request_content_details_init(&mut request);
                        request.content_details.as_mut().unwrap().content_rating.moctw_rating = value.unwrap_or("").to_string();
                    },
                "content-details.content-rating.anatel-rating" => {
                        request_content_details_init(&mut request);
                        request.content_details.as_mut().unwrap().content_rating.anatel_rating = value.unwrap_or("").to_string();
                    },
                "content-details.content-rating.catv-rating" => {
                        request_content_details_init(&mut request);
                        request.content_details.as_mut().unwrap().content_rating.catv_rating = value.unwrap_or("").to_string();
                    },
                "content-details.content-rating.pefilm-rating" => {
                        request_content_details_init(&mut request);
                        request.content_details.as_mut().unwrap().content_rating.pefilm_rating = value.unwrap_or("").to_string();
                    },
                "content-details.content-rating.djctq-rating-reasons" => {
                        request_content_details_init(&mut request);
                        request.content_details.as_mut().unwrap().content_rating.djctq_rating_reasons.push(value.unwrap_or("").to_string());
                    },
                "content-details.content-rating.incaa-rating" => {
                        request_content_details_init(&mut request);
                        request.content_details.as_mut().unwrap().content_rating.incaa_rating = value.unwrap_or("").to_string();
                    },
                "content-details.content-rating.oflc-rating" => {
                        request_content_details_init(&mut request);
                        request.content_details.as_mut().unwrap().content_rating.oflc_rating = value.unwrap_or("").to_string();
                    },
                "content-details.content-rating.fpb-rating" => {
                        request_content_details_init(&mut request);
                        request.content_details.as_mut().unwrap().content_rating.fpb_rating = value.unwrap_or("").to_string();
                    },
                "content-details.content-rating.mccaa-rating" => {
                        request_content_details_init(&mut request);
                        request.content_details.as_mut().unwrap().content_rating.mccaa_rating = value.unwrap_or("").to_string();
                    },
                "content-details.content-rating.tvpg-rating" => {
                        request_content_details_init(&mut request);
                        request.content_details.as_mut().unwrap().content_rating.tvpg_rating = value.unwrap_or("").to_string();
                    },
                "content-details.content-rating.rtc-rating" => {
                        request_content_details_init(&mut request);
                        request.content_details.as_mut().unwrap().content_rating.rtc_rating = value.unwrap_or("").to_string();
                    },
                "content-details.content-rating.cscf-rating" => {
                        request_content_details_init(&mut request);
                        request.content_details.as_mut().unwrap().content_rating.cscf_rating = value.unwrap_or("").to_string();
                    },
                "content-details.content-rating.fsk-rating" => {
                        request_content_details_init(&mut request);
                        request.content_details.as_mut().unwrap().content_rating.fsk_rating = value.unwrap_or("").to_string();
                    },
                "content-details.content-rating.bbfc-rating" => {
                        request_content_details_init(&mut request);
                        request.content_details.as_mut().unwrap().content_rating.bbfc_rating = value.unwrap_or("").to_string();
                    },
                "content-details.content-rating.kmrb-rating" => {
                        request_content_details_init(&mut request);
                        request.content_details.as_mut().unwrap().content_rating.kmrb_rating = value.unwrap_or("").to_string();
                    },
                "content-details.content-rating.smsa-rating" => {
                        request_content_details_init(&mut request);
                        request.content_details.as_mut().unwrap().content_rating.smsa_rating = value.unwrap_or("").to_string();
                    },
                "content-details.content-rating.egfilm-rating" => {
                        request_content_details_init(&mut request);
                        request.content_details.as_mut().unwrap().content_rating.egfilm_rating = value.unwrap_or("").to_string();
                    },
                "content-details.content-rating.cicf-rating" => {
                        request_content_details_init(&mut request);
                        request.content_details.as_mut().unwrap().content_rating.cicf_rating = value.unwrap_or("").to_string();
                    },
                "content-details.content-rating.nbcpl-rating" => {
                        request_content_details_init(&mut request);
                        request.content_details.as_mut().unwrap().content_rating.nbcpl_rating = value.unwrap_or("").to_string();
                    },
                "content-details.content-rating.nbc-rating" => {
                        request_content_details_init(&mut request);
                        request.content_details.as_mut().unwrap().content_rating.nbc_rating = value.unwrap_or("").to_string();
                    },
                "content-details.content-rating.djctq-rating" => {
                        request_content_details_init(&mut request);
                        request.content_details.as_mut().unwrap().content_rating.djctq_rating = value.unwrap_or("").to_string();
                    },
                "content-details.content-rating.ifco-rating" => {
                        request_content_details_init(&mut request);
                        request.content_details.as_mut().unwrap().content_rating.ifco_rating = value.unwrap_or("").to_string();
                    },
                "content-details.content-rating.fco-rating" => {
                        request_content_details_init(&mut request);
                        request.content_details.as_mut().unwrap().content_rating.fco_rating = value.unwrap_or("").to_string();
                    },
                "content-details.content-rating.eefilm-rating" => {
                        request_content_details_init(&mut request);
                        request.content_details.as_mut().unwrap().content_rating.eefilm_rating = value.unwrap_or("").to_string();
                    },
                "content-details.content-rating.medietilsynet-rating" => {
                        request_content_details_init(&mut request);
                        request.content_details.as_mut().unwrap().content_rating.medietilsynet_rating = value.unwrap_or("").to_string();
                    },
                "content-details.content-rating.grfilm-rating" => {
                        request_content_details_init(&mut request);
                        request.content_details.as_mut().unwrap().content_rating.grfilm_rating = value.unwrap_or("").to_string();
                    },
                "content-details.content-rating.ccc-rating" => {
                        request_content_details_init(&mut request);
                        request.content_details.as_mut().unwrap().content_rating.ccc_rating = value.unwrap_or("").to_string();
                    },
                "content-details.content-rating.rte-rating" => {
                        request_content_details_init(&mut request);
                        request.content_details.as_mut().unwrap().content_rating.rte_rating = value.unwrap_or("").to_string();
                    },
                "content-details.content-rating.czfilm-rating" => {
                        request_content_details_init(&mut request);
                        request.content_details.as_mut().unwrap().content_rating.czfilm_rating = value.unwrap_or("").to_string();
                    },
                "content-details.content-rating.lsf-rating" => {
                        request_content_details_init(&mut request);
                        request.content_details.as_mut().unwrap().content_rating.lsf_rating = value.unwrap_or("").to_string();
                    },
                "content-details.content-rating.fmoc-rating" => {
                        request_content_details_init(&mut request);
                        request.content_details.as_mut().unwrap().content_rating.fmoc_rating = value.unwrap_or("").to_string();
                    },
                "content-details.content-rating.eirin-rating" => {
                        request_content_details_init(&mut request);
                        request.content_details.as_mut().unwrap().content_rating.eirin_rating = value.unwrap_or("").to_string();
                    },
                "content-details.content-rating.cce-rating" => {
                        request_content_details_init(&mut request);
                        request.content_details.as_mut().unwrap().content_rating.cce_rating = value.unwrap_or("").to_string();
                    },
                "content-details.content-rating.nkclv-rating" => {
                        request_content_details_init(&mut request);
                        request.content_details.as_mut().unwrap().content_rating.nkclv_rating = value.unwrap_or("").to_string();
                    },
                "content-details.content-rating.mtrcb-rating" => {
                        request_content_details_init(&mut request);
                        request.content_details.as_mut().unwrap().content_rating.mtrcb_rating = value.unwrap_or("").to_string();
                    },
                "content-details.content-rating.mibac-rating" => {
                        request_content_details_init(&mut request);
                        request.content_details.as_mut().unwrap().content_rating.mibac_rating = value.unwrap_or("").to_string();
                    },
                "content-details.content-rating.ilfilm-rating" => {
                        request_content_details_init(&mut request);
                        request.content_details.as_mut().unwrap().content_rating.ilfilm_rating = value.unwrap_or("").to_string();
                    },
                "content-details.content-rating.smais-rating" => {
                        request_content_details_init(&mut request);
                        request.content_details.as_mut().unwrap().content_rating.smais_rating = value.unwrap_or("").to_string();
                    },
                "content-details.content-rating.russia-rating" => {
                        request_content_details_init(&mut request);
                        request.content_details.as_mut().unwrap().content_rating.russia_rating = value.unwrap_or("").to_string();
                    },
                "content-details.content-rating.mpaa-rating" => {
                        request_content_details_init(&mut request);
                        request.content_details.as_mut().unwrap().content_rating.mpaa_rating = value.unwrap_or("").to_string();
                    },
                "content-details.content-rating.kfcb-rating" => {
                        request_content_details_init(&mut request);
                        request.content_details.as_mut().unwrap().content_rating.kfcb_rating = value.unwrap_or("").to_string();
                    },
                "content-details.content-rating.agcom-rating" => {
                        request_content_details_init(&mut request);
                        request.content_details.as_mut().unwrap().content_rating.agcom_rating = value.unwrap_or("").to_string();
                    },
                "content-details.content-rating.chvrs-rating" => {
                        request_content_details_init(&mut request);
                        request.content_details.as_mut().unwrap().content_rating.chvrs_rating = value.unwrap_or("").to_string();
                    },
                "content-details.content-rating.cna-rating" => {
                        request_content_details_init(&mut request);
                        request.content_details.as_mut().unwrap().content_rating.cna_rating = value.unwrap_or("").to_string();
                    },
                "content-details.content-rating.icaa-rating" => {
                        request_content_details_init(&mut request);
                        request.content_details.as_mut().unwrap().content_rating.icaa_rating = value.unwrap_or("").to_string();
                    },
                "content-details.content-rating.mccyp-rating" => {
                        request_content_details_init(&mut request);
                        request.content_details.as_mut().unwrap().content_rating.mccyp_rating = value.unwrap_or("").to_string();
                    },
                "content-details.content-rating.nfrc-rating" => {
                        request_content_details_init(&mut request);
                        request.content_details.as_mut().unwrap().content_rating.nfrc_rating = value.unwrap_or("").to_string();
                    },
                "content-details.content-rating.skfilm-rating" => {
                        request_content_details_init(&mut request);
                        request.content_details.as_mut().unwrap().content_rating.skfilm_rating = value.unwrap_or("").to_string();
                    },
                "content-details.content-rating.moc-rating" => {
                        request_content_details_init(&mut request);
                        request.content_details.as_mut().unwrap().content_rating.moc_rating = value.unwrap_or("").to_string();
                    },
                "content-details.content-rating.rcnof-rating" => {
                        request_content_details_init(&mut request);
                        request.content_details.as_mut().unwrap().content_rating.rcnof_rating = value.unwrap_or("").to_string();
                    },
                "content-details.content-rating.meku-rating" => {
                        request_content_details_init(&mut request);
                        request.content_details.as_mut().unwrap().content_rating.meku_rating = value.unwrap_or("").to_string();
                    },
                "content-details.content-rating.fcbm-rating" => {
                        request_content_details_init(&mut request);
                        request.content_details.as_mut().unwrap().content_rating.fcbm_rating = value.unwrap_or("").to_string();
                    },
                "content-details.content-rating.kijkwijzer-rating" => {
                        request_content_details_init(&mut request);
                        request.content_details.as_mut().unwrap().content_rating.kijkwijzer_rating = value.unwrap_or("").to_string();
                    },
                "content-details.caption" => {
                        request_content_details_init(&mut request);
                        request.content_details.as_mut().unwrap().caption = value.unwrap_or("").to_string();
                    },
                "content-details.region-restriction.blocked" => {
                        request_content_details_init(&mut request);
                        request.content_details.as_mut().unwrap().region_restriction.blocked.push(value.unwrap_or("").to_string());
                    },
                "content-details.region-restriction.allowed" => {
                        request_content_details_init(&mut request);
                        request.content_details.as_mut().unwrap().region_restriction.allowed.push(value.unwrap_or("").to_string());
                    },
                "content-details.duration" => {
                        request_content_details_init(&mut request);
                        request.content_details.as_mut().unwrap().duration = value.unwrap_or("").to_string();
                    },
                "content-details.licensed-content" => {
                        request_content_details_init(&mut request);
                        request.content_details.as_mut().unwrap().licensed_content = arg_from_str(value.unwrap_or("false"), err, "content-details.licensed-content", "boolean");
                    },
                "content-details.dimension" => {
                        request_content_details_init(&mut request);
                        request.content_details.as_mut().unwrap().dimension = value.unwrap_or("").to_string();
                    },
                "monetization-details.access.exception" => {
                        request_monetization_details_init(&mut request);
                        request.monetization_details.as_mut().unwrap().access.exception.push(value.unwrap_or("").to_string());
                    },
                "monetization-details.access.allowed" => {
                        request_monetization_details_init(&mut request);
                        request.monetization_details.as_mut().unwrap().access.allowed = arg_from_str(value.unwrap_or("false"), err, "monetization-details.access.allowed", "boolean");
                    },
                "age-gating.restricted" => {
                        request_age_gating_init(&mut request);
                        request.age_gating.as_mut().unwrap().restricted = arg_from_str(value.unwrap_or("false"), err, "age-gating.restricted", "boolean");
                    },
                "age-gating.alcohol-content" => {
                        request_age_gating_init(&mut request);
                        request.age_gating.as_mut().unwrap().alcohol_content = arg_from_str(value.unwrap_or("false"), err, "age-gating.alcohol-content", "boolean");
                    },
                "age-gating.video-game-rating" => {
                        request_age_gating_init(&mut request);
                        request.age_gating.as_mut().unwrap().video_game_rating = value.unwrap_or("").to_string();
                    },
                "suggestions.processing-errors" => {
                        request_suggestions_init(&mut request);
                        request.suggestions.as_mut().unwrap().processing_errors.push(value.unwrap_or("").to_string());
                    },
                "suggestions.editor-suggestions" => {
                        request_suggestions_init(&mut request);
                        request.suggestions.as_mut().unwrap().editor_suggestions.push(value.unwrap_or("").to_string());
                    },
                "suggestions.processing-warnings" => {
                        request_suggestions_init(&mut request);
                        request.suggestions.as_mut().unwrap().processing_warnings.push(value.unwrap_or("").to_string());
                    },
                "suggestions.processing-hints" => {
                        request_suggestions_init(&mut request);
                        request.suggestions.as_mut().unwrap().processing_hints.push(value.unwrap_or("").to_string());
                    },
                "live-streaming-details.concurrent-viewers" => {
                        request_live_streaming_details_init(&mut request);
                        request.live_streaming_details.as_mut().unwrap().concurrent_viewers = value.unwrap_or("").to_string();
                    },
                "live-streaming-details.scheduled-start-time" => {
                        request_live_streaming_details_init(&mut request);
                        request.live_streaming_details.as_mut().unwrap().scheduled_start_time = value.unwrap_or("").to_string();
                    },
                "live-streaming-details.scheduled-end-time" => {
                        request_live_streaming_details_init(&mut request);
                        request.live_streaming_details.as_mut().unwrap().scheduled_end_time = value.unwrap_or("").to_string();
                    },
                "live-streaming-details.actual-start-time" => {
                        request_live_streaming_details_init(&mut request);
                        request.live_streaming_details.as_mut().unwrap().actual_start_time = value.unwrap_or("").to_string();
                    },
                "live-streaming-details.actual-end-time" => {
                        request_live_streaming_details_init(&mut request);
                        request.live_streaming_details.as_mut().unwrap().actual_end_time = value.unwrap_or("").to_string();
                    },
                "file-details.bitrate-bps" => {
                        request_file_details_init(&mut request);
                        request.file_details.as_mut().unwrap().bitrate_bps = value.unwrap_or("").to_string();
                    },
                "file-details.container" => {
                        request_file_details_init(&mut request);
                        request.file_details.as_mut().unwrap().container = value.unwrap_or("").to_string();
                    },
                "file-details.recording-location.latitude" => {
                        request_file_details_init(&mut request);
                        request.file_details.as_mut().unwrap().recording_location.latitude = arg_from_str(value.unwrap_or("0.0"), err, "file-details.recording-location.latitude", "number");
                    },
                "file-details.recording-location.altitude" => {
                        request_file_details_init(&mut request);
                        request.file_details.as_mut().unwrap().recording_location.altitude = arg_from_str(value.unwrap_or("0.0"), err, "file-details.recording-location.altitude", "number");
                    },
                "file-details.recording-location.longitude" => {
                        request_file_details_init(&mut request);
                        request.file_details.as_mut().unwrap().recording_location.longitude = arg_from_str(value.unwrap_or("0.0"), err, "file-details.recording-location.longitude", "number");
                    },
                "file-details.file-type" => {
                        request_file_details_init(&mut request);
                        request.file_details.as_mut().unwrap().file_type = value.unwrap_or("").to_string();
                    },
                "file-details.creation-time" => {
                        request_file_details_init(&mut request);
                        request.file_details.as_mut().unwrap().creation_time = value.unwrap_or("").to_string();
                    },
                "file-details.duration-ms" => {
                        request_file_details_init(&mut request);
                        request.file_details.as_mut().unwrap().duration_ms = value.unwrap_or("").to_string();
                    },
                "file-details.file-name" => {
                        request_file_details_init(&mut request);
                        request.file_details.as_mut().unwrap().file_name = value.unwrap_or("").to_string();
                    },
                "file-details.file-size" => {
                        request_file_details_init(&mut request);
                        request.file_details.as_mut().unwrap().file_size = value.unwrap_or("").to_string();
                    },
                "snippet.description" => {
                        request_snippet_init(&mut request);
                        request.snippet.as_mut().unwrap().description = value.unwrap_or("").to_string();
                    },
                "snippet.tags" => {
                        request_snippet_init(&mut request);
                        request.snippet.as_mut().unwrap().tags.push(value.unwrap_or("").to_string());
                    },
                "snippet.channel-id" => {
                        request_snippet_init(&mut request);
                        request.snippet.as_mut().unwrap().channel_id = value.unwrap_or("").to_string();
                    },
                "snippet.published-at" => {
                        request_snippet_init(&mut request);
                        request.snippet.as_mut().unwrap().published_at = value.unwrap_or("").to_string();
                    },
                "snippet.live-broadcast-content" => {
                        request_snippet_init(&mut request);
                        request.snippet.as_mut().unwrap().live_broadcast_content = value.unwrap_or("").to_string();
                    },
                "snippet.default-language" => {
                        request_snippet_init(&mut request);
                        request.snippet.as_mut().unwrap().default_language = value.unwrap_or("").to_string();
                    },
                "snippet.channel-title" => {
                        request_snippet_init(&mut request);
                        request.snippet.as_mut().unwrap().channel_title = value.unwrap_or("").to_string();
                    },
                "snippet.title" => {
                        request_snippet_init(&mut request);
                        request.snippet.as_mut().unwrap().title = value.unwrap_or("").to_string();
                    },
                "snippet.category-id" => {
                        request_snippet_init(&mut request);
                        request.snippet.as_mut().unwrap().category_id = value.unwrap_or("").to_string();
                    },
                "snippet.localized.description" => {
                        request_snippet_init(&mut request);
                        request.snippet.as_mut().unwrap().localized.description = value.unwrap_or("").to_string();
                    },
                "snippet.localized.title" => {
                        request_snippet_init(&mut request);
                        request.snippet.as_mut().unwrap().localized.title = value.unwrap_or("").to_string();
                    },
                "snippet.thumbnails.default.url" => {
                        request_snippet_init(&mut request);
                        request.snippet.as_mut().unwrap().thumbnails.default.url = Some(value.unwrap_or("").to_string());
                    },
                "snippet.thumbnails.default.width" => {
                        request_snippet_init(&mut request);
                        request.snippet.as_mut().unwrap().thumbnails.default.width = Some(arg_from_str(value.unwrap_or("-0"), err, "snippet.thumbnails.default.width", "integer"));
                    },
                "snippet.thumbnails.default.height" => {
                        request_snippet_init(&mut request);
                        request.snippet.as_mut().unwrap().thumbnails.default.height = Some(arg_from_str(value.unwrap_or("-0"), err, "snippet.thumbnails.default.height", "integer"));
                    },
                "snippet.thumbnails.high.url" => {
                        request_snippet_init(&mut request);
                        request.snippet.as_mut().unwrap().thumbnails.high.url = Some(value.unwrap_or("").to_string());
                    },
                "snippet.thumbnails.high.width" => {
                        request_snippet_init(&mut request);
                        request.snippet.as_mut().unwrap().thumbnails.high.width = Some(arg_from_str(value.unwrap_or("-0"), err, "snippet.thumbnails.high.width", "integer"));
                    },
                "snippet.thumbnails.high.height" => {
                        request_snippet_init(&mut request);
                        request.snippet.as_mut().unwrap().thumbnails.high.height = Some(arg_from_str(value.unwrap_or("-0"), err, "snippet.thumbnails.high.height", "integer"));
                    },
                "snippet.thumbnails.medium.url" => {
                        request_snippet_init(&mut request);
                        request.snippet.as_mut().unwrap().thumbnails.medium.url = Some(value.unwrap_or("").to_string());
                    },
                "snippet.thumbnails.medium.width" => {
                        request_snippet_init(&mut request);
                        request.snippet.as_mut().unwrap().thumbnails.medium.width = Some(arg_from_str(value.unwrap_or("-0"), err, "snippet.thumbnails.medium.width", "integer"));
                    },
                "snippet.thumbnails.medium.height" => {
                        request_snippet_init(&mut request);
                        request.snippet.as_mut().unwrap().thumbnails.medium.height = Some(arg_from_str(value.unwrap_or("-0"), err, "snippet.thumbnails.medium.height", "integer"));
                    },
                "snippet.thumbnails.maxres.url" => {
                        request_snippet_init(&mut request);
                        request.snippet.as_mut().unwrap().thumbnails.maxres.url = Some(value.unwrap_or("").to_string());
                    },
                "snippet.thumbnails.maxres.width" => {
                        request_snippet_init(&mut request);
                        request.snippet.as_mut().unwrap().thumbnails.maxres.width = Some(arg_from_str(value.unwrap_or("-0"), err, "snippet.thumbnails.maxres.width", "integer"));
                    },
                "snippet.thumbnails.maxres.height" => {
                        request_snippet_init(&mut request);
                        request.snippet.as_mut().unwrap().thumbnails.maxres.height = Some(arg_from_str(value.unwrap_or("-0"), err, "snippet.thumbnails.maxres.height", "integer"));
                    },
                "snippet.thumbnails.standard.url" => {
                        request_snippet_init(&mut request);
                        request.snippet.as_mut().unwrap().thumbnails.standard.url = Some(value.unwrap_or("").to_string());
                    },
                "snippet.thumbnails.standard.width" => {
                        request_snippet_init(&mut request);
                        request.snippet.as_mut().unwrap().thumbnails.standard.width = Some(arg_from_str(value.unwrap_or("-0"), err, "snippet.thumbnails.standard.width", "integer"));
                    },
                "snippet.thumbnails.standard.height" => {
                        request_snippet_init(&mut request);
                        request.snippet.as_mut().unwrap().thumbnails.standard.height = Some(arg_from_str(value.unwrap_or("-0"), err, "snippet.thumbnails.standard.height", "integer"));
                    },
                "player.embed-html" => {
                        request_player_init(&mut request);
                        request.player.as_mut().unwrap().embed_html = value.unwrap_or("").to_string();
                    },
                "processing-details.file-details-availability" => {
                        request_processing_details_init(&mut request);
                        request.processing_details.as_mut().unwrap().file_details_availability = value.unwrap_or("").to_string();
                    },
                "processing-details.editor-suggestions-availability" => {
                        request_processing_details_init(&mut request);
                        request.processing_details.as_mut().unwrap().editor_suggestions_availability = value.unwrap_or("").to_string();
                    },
                "processing-details.processing-status" => {
                        request_processing_details_init(&mut request);
                        request.processing_details.as_mut().unwrap().processing_status = value.unwrap_or("").to_string();
                    },
                "processing-details.processing-issues-availability" => {
                        request_processing_details_init(&mut request);
                        request.processing_details.as_mut().unwrap().processing_issues_availability = value.unwrap_or("").to_string();
                    },
                "processing-details.processing-failure-reason" => {
                        request_processing_details_init(&mut request);
                        request.processing_details.as_mut().unwrap().processing_failure_reason = value.unwrap_or("").to_string();
                    },
                "processing-details.thumbnails-availability" => {
                        request_processing_details_init(&mut request);
                        request.processing_details.as_mut().unwrap().thumbnails_availability = value.unwrap_or("").to_string();
                    },
                "processing-details.processing-progress.time-left-ms" => {
                        request_processing_details_init(&mut request);
                        request.processing_details.as_mut().unwrap().processing_progress.time_left_ms = value.unwrap_or("").to_string();
                    },
                "processing-details.processing-progress.parts-processed" => {
                        request_processing_details_init(&mut request);
                        request.processing_details.as_mut().unwrap().processing_progress.parts_processed = value.unwrap_or("").to_string();
                    },
                "processing-details.processing-progress.parts-total" => {
                        request_processing_details_init(&mut request);
                        request.processing_details.as_mut().unwrap().processing_progress.parts_total = value.unwrap_or("").to_string();
                    },
                "processing-details.tag-suggestions-availability" => {
                        request_processing_details_init(&mut request);
                        request.processing_details.as_mut().unwrap().tag_suggestions_availability = value.unwrap_or("").to_string();
                    },
                "etag" => {
                        request_processing_details_init(&mut request);
                        request.etag = Some(value.unwrap_or("").to_string());
                    },
                "project-details.tags" => {
                        request_project_details_init(&mut request);
                        request.project_details.as_mut().unwrap().tags.push(value.unwrap_or("").to_string());
                    },
                "recording-details.recording-date" => {
                        request_recording_details_init(&mut request);
                        request.recording_details.as_mut().unwrap().recording_date = value.unwrap_or("").to_string();
                    },
                "recording-details.location-description" => {
                        request_recording_details_init(&mut request);
                        request.recording_details.as_mut().unwrap().location_description = value.unwrap_or("").to_string();
                    },
                "recording-details.location.latitude" => {
                        request_recording_details_init(&mut request);
                        request.recording_details.as_mut().unwrap().location.latitude = arg_from_str(value.unwrap_or("0.0"), err, "recording-details.location.latitude", "number");
                    },
                "recording-details.location.altitude" => {
                        request_recording_details_init(&mut request);
                        request.recording_details.as_mut().unwrap().location.altitude = arg_from_str(value.unwrap_or("0.0"), err, "recording-details.location.altitude", "number");
                    },
                "recording-details.location.longitude" => {
                        request_recording_details_init(&mut request);
                        request.recording_details.as_mut().unwrap().location.longitude = arg_from_str(value.unwrap_or("0.0"), err, "recording-details.location.longitude", "number");
                    },
                "id" => {
                        request_recording_details_init(&mut request);
                        request.id = Some(value.unwrap_or("").to_string());
                    },
                _ => {
                    err.issues.push(CLIError::Field(FieldError::Unknown(field_name.to_string())));
                }
            }
        }
        let protocol = 
            if self.opt.cmd_simple {
                "simple"
            } else if self.opt.cmd_resumable {
                "resumable"
            } else { 
                unreachable!() 
            };
        let mut input_file = input_file_from_opts(&self.opt.arg_file, err);
        let mime_type = input_mime_from_opts(&self.opt.arg_mime, err);
        if dry_run {
            None
        } else {
            assert!(err.issues.len() == 0);
            let mut ostream = writer_from_opts(self.opt.flag_o, &self.opt.arg_out);
            match match protocol {
                "simple" => call.upload(input_file.unwrap(), mime_type.unwrap()),
                "resumable" => call.upload_resumable(input_file.unwrap(), mime_type.unwrap()),
                _ => unreachable!(),
            } {
                Err(api_err) => Some(api_err),
                Ok((mut response, output_schema)) => {
                    println!("DEBUG: REMOVE ME {:?}", response);
                    serde::json::to_writer(&mut ostream, &output_schema).unwrap();
                    None
                }
            }
        }
    }

    fn _videos_list(&self, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Option<api::Error> {
        let mut call = self.hub.videos().list(&self.opt.arg_part);
        for parg in self.opt.arg_v.iter() {
            let (key, value) = parse_kv_arg(&*parg, err);
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
                "alt"
                |"fields"
                |"key"
                |"oauth-token"
                |"pretty-print"
                |"quota-user"
                |"user-ip" => {
                    let map = [
                        ("oauth-token", "oauth_token"),
                        ("pretty-print", "prettyPrint"),
                        ("quota-user", "quotaUser"),
                        ("user-ip", "userIp"),
                    ];
                    call = call.param(map.iter().find(|t| t.0 == key).unwrap_or(&("", key)).1, value.unwrap_or("unset"))
                },
                _ => err.issues.push(CLIError::UnknownParameter(key.to_string())),
            }
        }
        let protocol = "standard-request";
        if dry_run {
            None
        } else {
            assert!(err.issues.len() == 0);
            let mut ostream = writer_from_opts(self.opt.flag_o, &self.opt.arg_out);
            match match protocol {
                "standard-request" => call.doit(),
                _ => unreachable!(),
            } {
                Err(api_err) => Some(api_err),
                Ok((mut response, output_schema)) => {
                    println!("DEBUG: REMOVE ME {:?}", response);
                    serde::json::to_writer(&mut ostream, &output_schema).unwrap();
                    None
                }
            }
        }
    }

    fn _videos_rate(&self, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Option<api::Error> {
        let mut call = self.hub.videos().rate(&self.opt.arg_id, &self.opt.arg_rating);
        for parg in self.opt.arg_v.iter() {
            let (key, value) = parse_kv_arg(&*parg, err);
            match key {
                "on-behalf-of-content-owner" => {
                    call = call.on_behalf_of_content_owner(value.unwrap_or(""));
                },
                "alt"
                |"fields"
                |"key"
                |"oauth-token"
                |"pretty-print"
                |"quota-user"
                |"user-ip" => {
                    let map = [
                        ("oauth-token", "oauth_token"),
                        ("pretty-print", "prettyPrint"),
                        ("quota-user", "quotaUser"),
                        ("user-ip", "userIp"),
                    ];
                    call = call.param(map.iter().find(|t| t.0 == key).unwrap_or(&("", key)).1, value.unwrap_or("unset"))
                },
                _ => err.issues.push(CLIError::UnknownParameter(key.to_string())),
            }
        }
        let protocol = "standard-request";
        if dry_run {
            None
        } else {
            assert!(err.issues.len() == 0);
            match match protocol {
                "standard-request" => call.doit(),
                _ => unreachable!(),
            } {
                Err(api_err) => Some(api_err),
                Ok(mut response) => {
                    println!("DEBUG: REMOVE ME {:?}", response);
                    None
                }
            }
        }
    }

    fn _videos_update(&self, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Option<api::Error> {
            let mut request: api::Video = Default::default();
        let mut call = self.hub.videos().update(&request);
        for parg in self.opt.arg_v.iter() {
            let (key, value) = parse_kv_arg(&*parg, err);
            match key {
                "on-behalf-of-content-owner" => {
                    call = call.on_behalf_of_content_owner(value.unwrap_or(""));
                },
                "alt"
                |"fields"
                |"key"
                |"oauth-token"
                |"pretty-print"
                |"quota-user"
                |"user-ip" => {
                    let map = [
                        ("oauth-token", "oauth_token"),
                        ("pretty-print", "prettyPrint"),
                        ("quota-user", "quotaUser"),
                        ("user-ip", "userIp"),
                    ];
                    call = call.param(map.iter().find(|t| t.0 == key).unwrap_or(&("", key)).1, value.unwrap_or("unset"))
                },
                _ => err.issues.push(CLIError::UnknownParameter(key.to_string())),
            }
        }
        let mut field_name: FieldCursor = Default::default();
        for kvarg in self.opt.arg_kv.iter() {
            let (key, value) = parse_kv_arg(&*kvarg, err);
            if let Err(field_err) = field_name.set(&*key) {
                err.issues.push(field_err);
            }
            fn request_age_gating_init(request: &mut api::Video) {
                if request.age_gating.is_none() {
                    request.age_gating = Some(Default::default());
                }
            }
            
            fn request_content_details_init(request: &mut api::Video) {
                if request.content_details.is_none() {
                    request.content_details = Some(Default::default());
                }
            }
            
            fn request_file_details_init(request: &mut api::Video) {
                if request.file_details.is_none() {
                    request.file_details = Some(Default::default());
                }
            }
            
            fn request_live_streaming_details_init(request: &mut api::Video) {
                if request.live_streaming_details.is_none() {
                    request.live_streaming_details = Some(Default::default());
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
            
            fn request_snippet_init(request: &mut api::Video) {
                if request.snippet.is_none() {
                    request.snippet = Some(Default::default());
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
            
            match &field_name.to_string()[..] {
                "status.license" => {
                        request_status_init(&mut request);
                        request.status.as_mut().unwrap().license = value.unwrap_or("").to_string();
                    },
                "status.embeddable" => {
                        request_status_init(&mut request);
                        request.status.as_mut().unwrap().embeddable = arg_from_str(value.unwrap_or("false"), err, "status.embeddable", "boolean");
                    },
                "status.privacy-status" => {
                        request_status_init(&mut request);
                        request.status.as_mut().unwrap().privacy_status = value.unwrap_or("").to_string();
                    },
                "status.publish-at" => {
                        request_status_init(&mut request);
                        request.status.as_mut().unwrap().publish_at = value.unwrap_or("").to_string();
                    },
                "status.public-stats-viewable" => {
                        request_status_init(&mut request);
                        request.status.as_mut().unwrap().public_stats_viewable = arg_from_str(value.unwrap_or("false"), err, "status.public-stats-viewable", "boolean");
                    },
                "status.upload-status" => {
                        request_status_init(&mut request);
                        request.status.as_mut().unwrap().upload_status = value.unwrap_or("").to_string();
                    },
                "status.rejection-reason" => {
                        request_status_init(&mut request);
                        request.status.as_mut().unwrap().rejection_reason = value.unwrap_or("").to_string();
                    },
                "status.failure-reason" => {
                        request_status_init(&mut request);
                        request.status.as_mut().unwrap().failure_reason = value.unwrap_or("").to_string();
                    },
                "topic-details.topic-ids" => {
                        request_topic_details_init(&mut request);
                        request.topic_details.as_mut().unwrap().topic_ids.push(value.unwrap_or("").to_string());
                    },
                "topic-details.relevant-topic-ids" => {
                        request_topic_details_init(&mut request);
                        request.topic_details.as_mut().unwrap().relevant_topic_ids.push(value.unwrap_or("").to_string());
                    },
                "kind" => {
                        request_topic_details_init(&mut request);
                        request.kind = Some(value.unwrap_or("").to_string());
                    },
                "statistics.comment-count" => {
                        request_statistics_init(&mut request);
                        request.statistics.as_mut().unwrap().comment_count = arg_from_str(value.unwrap_or("-0"), err, "statistics.comment-count", "int64");
                    },
                "statistics.view-count" => {
                        request_statistics_init(&mut request);
                        request.statistics.as_mut().unwrap().view_count = arg_from_str(value.unwrap_or("-0"), err, "statistics.view-count", "int64");
                    },
                "statistics.favorite-count" => {
                        request_statistics_init(&mut request);
                        request.statistics.as_mut().unwrap().favorite_count = arg_from_str(value.unwrap_or("-0"), err, "statistics.favorite-count", "int64");
                    },
                "statistics.dislike-count" => {
                        request_statistics_init(&mut request);
                        request.statistics.as_mut().unwrap().dislike_count = arg_from_str(value.unwrap_or("-0"), err, "statistics.dislike-count", "int64");
                    },
                "statistics.like-count" => {
                        request_statistics_init(&mut request);
                        request.statistics.as_mut().unwrap().like_count = arg_from_str(value.unwrap_or("-0"), err, "statistics.like-count", "int64");
                    },
                "content-details.definition" => {
                        request_content_details_init(&mut request);
                        request.content_details.as_mut().unwrap().definition = value.unwrap_or("").to_string();
                    },
                "content-details.country-restriction.exception" => {
                        request_content_details_init(&mut request);
                        request.content_details.as_mut().unwrap().country_restriction.exception.push(value.unwrap_or("").to_string());
                    },
                "content-details.country-restriction.allowed" => {
                        request_content_details_init(&mut request);
                        request.content_details.as_mut().unwrap().country_restriction.allowed = arg_from_str(value.unwrap_or("false"), err, "content-details.country-restriction.allowed", "boolean");
                    },
                "content-details.content-rating.yt-rating" => {
                        request_content_details_init(&mut request);
                        request.content_details.as_mut().unwrap().content_rating.yt_rating = value.unwrap_or("").to_string();
                    },
                "content-details.content-rating.catvfr-rating" => {
                        request_content_details_init(&mut request);
                        request.content_details.as_mut().unwrap().content_rating.catvfr_rating = value.unwrap_or("").to_string();
                    },
                "content-details.content-rating.cbfc-rating" => {
                        request_content_details_init(&mut request);
                        request.content_details.as_mut().unwrap().content_rating.cbfc_rating = value.unwrap_or("").to_string();
                    },
                "content-details.content-rating.bfvc-rating" => {
                        request_content_details_init(&mut request);
                        request.content_details.as_mut().unwrap().content_rating.bfvc_rating = value.unwrap_or("").to_string();
                    },
                "content-details.content-rating.mda-rating" => {
                        request_content_details_init(&mut request);
                        request.content_details.as_mut().unwrap().content_rating.mda_rating = value.unwrap_or("").to_string();
                    },
                "content-details.content-rating.acb-rating" => {
                        request_content_details_init(&mut request);
                        request.content_details.as_mut().unwrap().content_rating.acb_rating = value.unwrap_or("").to_string();
                    },
                "content-details.content-rating.nfvcb-rating" => {
                        request_content_details_init(&mut request);
                        request.content_details.as_mut().unwrap().content_rating.nfvcb_rating = value.unwrap_or("").to_string();
                    },
                "content-details.content-rating.bmukk-rating" => {
                        request_content_details_init(&mut request);
                        request.content_details.as_mut().unwrap().content_rating.bmukk_rating = value.unwrap_or("").to_string();
                    },
                "content-details.content-rating.chfilm-rating" => {
                        request_content_details_init(&mut request);
                        request.content_details.as_mut().unwrap().content_rating.chfilm_rating = value.unwrap_or("").to_string();
                    },
                "content-details.content-rating.resorteviolencia-rating" => {
                        request_content_details_init(&mut request);
                        request.content_details.as_mut().unwrap().content_rating.resorteviolencia_rating = value.unwrap_or("").to_string();
                    },
                "content-details.content-rating.csa-rating" => {
                        request_content_details_init(&mut request);
                        request.content_details.as_mut().unwrap().content_rating.csa_rating = value.unwrap_or("").to_string();
                    },
                "content-details.content-rating.moctw-rating" => {
                        request_content_details_init(&mut request);
                        request.content_details.as_mut().unwrap().content_rating.moctw_rating = value.unwrap_or("").to_string();
                    },
                "content-details.content-rating.anatel-rating" => {
                        request_content_details_init(&mut request);
                        request.content_details.as_mut().unwrap().content_rating.anatel_rating = value.unwrap_or("").to_string();
                    },
                "content-details.content-rating.catv-rating" => {
                        request_content_details_init(&mut request);
                        request.content_details.as_mut().unwrap().content_rating.catv_rating = value.unwrap_or("").to_string();
                    },
                "content-details.content-rating.pefilm-rating" => {
                        request_content_details_init(&mut request);
                        request.content_details.as_mut().unwrap().content_rating.pefilm_rating = value.unwrap_or("").to_string();
                    },
                "content-details.content-rating.djctq-rating-reasons" => {
                        request_content_details_init(&mut request);
                        request.content_details.as_mut().unwrap().content_rating.djctq_rating_reasons.push(value.unwrap_or("").to_string());
                    },
                "content-details.content-rating.incaa-rating" => {
                        request_content_details_init(&mut request);
                        request.content_details.as_mut().unwrap().content_rating.incaa_rating = value.unwrap_or("").to_string();
                    },
                "content-details.content-rating.oflc-rating" => {
                        request_content_details_init(&mut request);
                        request.content_details.as_mut().unwrap().content_rating.oflc_rating = value.unwrap_or("").to_string();
                    },
                "content-details.content-rating.fpb-rating" => {
                        request_content_details_init(&mut request);
                        request.content_details.as_mut().unwrap().content_rating.fpb_rating = value.unwrap_or("").to_string();
                    },
                "content-details.content-rating.mccaa-rating" => {
                        request_content_details_init(&mut request);
                        request.content_details.as_mut().unwrap().content_rating.mccaa_rating = value.unwrap_or("").to_string();
                    },
                "content-details.content-rating.tvpg-rating" => {
                        request_content_details_init(&mut request);
                        request.content_details.as_mut().unwrap().content_rating.tvpg_rating = value.unwrap_or("").to_string();
                    },
                "content-details.content-rating.rtc-rating" => {
                        request_content_details_init(&mut request);
                        request.content_details.as_mut().unwrap().content_rating.rtc_rating = value.unwrap_or("").to_string();
                    },
                "content-details.content-rating.cscf-rating" => {
                        request_content_details_init(&mut request);
                        request.content_details.as_mut().unwrap().content_rating.cscf_rating = value.unwrap_or("").to_string();
                    },
                "content-details.content-rating.fsk-rating" => {
                        request_content_details_init(&mut request);
                        request.content_details.as_mut().unwrap().content_rating.fsk_rating = value.unwrap_or("").to_string();
                    },
                "content-details.content-rating.bbfc-rating" => {
                        request_content_details_init(&mut request);
                        request.content_details.as_mut().unwrap().content_rating.bbfc_rating = value.unwrap_or("").to_string();
                    },
                "content-details.content-rating.kmrb-rating" => {
                        request_content_details_init(&mut request);
                        request.content_details.as_mut().unwrap().content_rating.kmrb_rating = value.unwrap_or("").to_string();
                    },
                "content-details.content-rating.smsa-rating" => {
                        request_content_details_init(&mut request);
                        request.content_details.as_mut().unwrap().content_rating.smsa_rating = value.unwrap_or("").to_string();
                    },
                "content-details.content-rating.egfilm-rating" => {
                        request_content_details_init(&mut request);
                        request.content_details.as_mut().unwrap().content_rating.egfilm_rating = value.unwrap_or("").to_string();
                    },
                "content-details.content-rating.cicf-rating" => {
                        request_content_details_init(&mut request);
                        request.content_details.as_mut().unwrap().content_rating.cicf_rating = value.unwrap_or("").to_string();
                    },
                "content-details.content-rating.nbcpl-rating" => {
                        request_content_details_init(&mut request);
                        request.content_details.as_mut().unwrap().content_rating.nbcpl_rating = value.unwrap_or("").to_string();
                    },
                "content-details.content-rating.nbc-rating" => {
                        request_content_details_init(&mut request);
                        request.content_details.as_mut().unwrap().content_rating.nbc_rating = value.unwrap_or("").to_string();
                    },
                "content-details.content-rating.djctq-rating" => {
                        request_content_details_init(&mut request);
                        request.content_details.as_mut().unwrap().content_rating.djctq_rating = value.unwrap_or("").to_string();
                    },
                "content-details.content-rating.ifco-rating" => {
                        request_content_details_init(&mut request);
                        request.content_details.as_mut().unwrap().content_rating.ifco_rating = value.unwrap_or("").to_string();
                    },
                "content-details.content-rating.fco-rating" => {
                        request_content_details_init(&mut request);
                        request.content_details.as_mut().unwrap().content_rating.fco_rating = value.unwrap_or("").to_string();
                    },
                "content-details.content-rating.eefilm-rating" => {
                        request_content_details_init(&mut request);
                        request.content_details.as_mut().unwrap().content_rating.eefilm_rating = value.unwrap_or("").to_string();
                    },
                "content-details.content-rating.medietilsynet-rating" => {
                        request_content_details_init(&mut request);
                        request.content_details.as_mut().unwrap().content_rating.medietilsynet_rating = value.unwrap_or("").to_string();
                    },
                "content-details.content-rating.grfilm-rating" => {
                        request_content_details_init(&mut request);
                        request.content_details.as_mut().unwrap().content_rating.grfilm_rating = value.unwrap_or("").to_string();
                    },
                "content-details.content-rating.ccc-rating" => {
                        request_content_details_init(&mut request);
                        request.content_details.as_mut().unwrap().content_rating.ccc_rating = value.unwrap_or("").to_string();
                    },
                "content-details.content-rating.rte-rating" => {
                        request_content_details_init(&mut request);
                        request.content_details.as_mut().unwrap().content_rating.rte_rating = value.unwrap_or("").to_string();
                    },
                "content-details.content-rating.czfilm-rating" => {
                        request_content_details_init(&mut request);
                        request.content_details.as_mut().unwrap().content_rating.czfilm_rating = value.unwrap_or("").to_string();
                    },
                "content-details.content-rating.lsf-rating" => {
                        request_content_details_init(&mut request);
                        request.content_details.as_mut().unwrap().content_rating.lsf_rating = value.unwrap_or("").to_string();
                    },
                "content-details.content-rating.fmoc-rating" => {
                        request_content_details_init(&mut request);
                        request.content_details.as_mut().unwrap().content_rating.fmoc_rating = value.unwrap_or("").to_string();
                    },
                "content-details.content-rating.eirin-rating" => {
                        request_content_details_init(&mut request);
                        request.content_details.as_mut().unwrap().content_rating.eirin_rating = value.unwrap_or("").to_string();
                    },
                "content-details.content-rating.cce-rating" => {
                        request_content_details_init(&mut request);
                        request.content_details.as_mut().unwrap().content_rating.cce_rating = value.unwrap_or("").to_string();
                    },
                "content-details.content-rating.nkclv-rating" => {
                        request_content_details_init(&mut request);
                        request.content_details.as_mut().unwrap().content_rating.nkclv_rating = value.unwrap_or("").to_string();
                    },
                "content-details.content-rating.mtrcb-rating" => {
                        request_content_details_init(&mut request);
                        request.content_details.as_mut().unwrap().content_rating.mtrcb_rating = value.unwrap_or("").to_string();
                    },
                "content-details.content-rating.mibac-rating" => {
                        request_content_details_init(&mut request);
                        request.content_details.as_mut().unwrap().content_rating.mibac_rating = value.unwrap_or("").to_string();
                    },
                "content-details.content-rating.ilfilm-rating" => {
                        request_content_details_init(&mut request);
                        request.content_details.as_mut().unwrap().content_rating.ilfilm_rating = value.unwrap_or("").to_string();
                    },
                "content-details.content-rating.smais-rating" => {
                        request_content_details_init(&mut request);
                        request.content_details.as_mut().unwrap().content_rating.smais_rating = value.unwrap_or("").to_string();
                    },
                "content-details.content-rating.russia-rating" => {
                        request_content_details_init(&mut request);
                        request.content_details.as_mut().unwrap().content_rating.russia_rating = value.unwrap_or("").to_string();
                    },
                "content-details.content-rating.mpaa-rating" => {
                        request_content_details_init(&mut request);
                        request.content_details.as_mut().unwrap().content_rating.mpaa_rating = value.unwrap_or("").to_string();
                    },
                "content-details.content-rating.kfcb-rating" => {
                        request_content_details_init(&mut request);
                        request.content_details.as_mut().unwrap().content_rating.kfcb_rating = value.unwrap_or("").to_string();
                    },
                "content-details.content-rating.agcom-rating" => {
                        request_content_details_init(&mut request);
                        request.content_details.as_mut().unwrap().content_rating.agcom_rating = value.unwrap_or("").to_string();
                    },
                "content-details.content-rating.chvrs-rating" => {
                        request_content_details_init(&mut request);
                        request.content_details.as_mut().unwrap().content_rating.chvrs_rating = value.unwrap_or("").to_string();
                    },
                "content-details.content-rating.cna-rating" => {
                        request_content_details_init(&mut request);
                        request.content_details.as_mut().unwrap().content_rating.cna_rating = value.unwrap_or("").to_string();
                    },
                "content-details.content-rating.icaa-rating" => {
                        request_content_details_init(&mut request);
                        request.content_details.as_mut().unwrap().content_rating.icaa_rating = value.unwrap_or("").to_string();
                    },
                "content-details.content-rating.mccyp-rating" => {
                        request_content_details_init(&mut request);
                        request.content_details.as_mut().unwrap().content_rating.mccyp_rating = value.unwrap_or("").to_string();
                    },
                "content-details.content-rating.nfrc-rating" => {
                        request_content_details_init(&mut request);
                        request.content_details.as_mut().unwrap().content_rating.nfrc_rating = value.unwrap_or("").to_string();
                    },
                "content-details.content-rating.skfilm-rating" => {
                        request_content_details_init(&mut request);
                        request.content_details.as_mut().unwrap().content_rating.skfilm_rating = value.unwrap_or("").to_string();
                    },
                "content-details.content-rating.moc-rating" => {
                        request_content_details_init(&mut request);
                        request.content_details.as_mut().unwrap().content_rating.moc_rating = value.unwrap_or("").to_string();
                    },
                "content-details.content-rating.rcnof-rating" => {
                        request_content_details_init(&mut request);
                        request.content_details.as_mut().unwrap().content_rating.rcnof_rating = value.unwrap_or("").to_string();
                    },
                "content-details.content-rating.meku-rating" => {
                        request_content_details_init(&mut request);
                        request.content_details.as_mut().unwrap().content_rating.meku_rating = value.unwrap_or("").to_string();
                    },
                "content-details.content-rating.fcbm-rating" => {
                        request_content_details_init(&mut request);
                        request.content_details.as_mut().unwrap().content_rating.fcbm_rating = value.unwrap_or("").to_string();
                    },
                "content-details.content-rating.kijkwijzer-rating" => {
                        request_content_details_init(&mut request);
                        request.content_details.as_mut().unwrap().content_rating.kijkwijzer_rating = value.unwrap_or("").to_string();
                    },
                "content-details.caption" => {
                        request_content_details_init(&mut request);
                        request.content_details.as_mut().unwrap().caption = value.unwrap_or("").to_string();
                    },
                "content-details.region-restriction.blocked" => {
                        request_content_details_init(&mut request);
                        request.content_details.as_mut().unwrap().region_restriction.blocked.push(value.unwrap_or("").to_string());
                    },
                "content-details.region-restriction.allowed" => {
                        request_content_details_init(&mut request);
                        request.content_details.as_mut().unwrap().region_restriction.allowed.push(value.unwrap_or("").to_string());
                    },
                "content-details.duration" => {
                        request_content_details_init(&mut request);
                        request.content_details.as_mut().unwrap().duration = value.unwrap_or("").to_string();
                    },
                "content-details.licensed-content" => {
                        request_content_details_init(&mut request);
                        request.content_details.as_mut().unwrap().licensed_content = arg_from_str(value.unwrap_or("false"), err, "content-details.licensed-content", "boolean");
                    },
                "content-details.dimension" => {
                        request_content_details_init(&mut request);
                        request.content_details.as_mut().unwrap().dimension = value.unwrap_or("").to_string();
                    },
                "monetization-details.access.exception" => {
                        request_monetization_details_init(&mut request);
                        request.monetization_details.as_mut().unwrap().access.exception.push(value.unwrap_or("").to_string());
                    },
                "monetization-details.access.allowed" => {
                        request_monetization_details_init(&mut request);
                        request.monetization_details.as_mut().unwrap().access.allowed = arg_from_str(value.unwrap_or("false"), err, "monetization-details.access.allowed", "boolean");
                    },
                "age-gating.restricted" => {
                        request_age_gating_init(&mut request);
                        request.age_gating.as_mut().unwrap().restricted = arg_from_str(value.unwrap_or("false"), err, "age-gating.restricted", "boolean");
                    },
                "age-gating.alcohol-content" => {
                        request_age_gating_init(&mut request);
                        request.age_gating.as_mut().unwrap().alcohol_content = arg_from_str(value.unwrap_or("false"), err, "age-gating.alcohol-content", "boolean");
                    },
                "age-gating.video-game-rating" => {
                        request_age_gating_init(&mut request);
                        request.age_gating.as_mut().unwrap().video_game_rating = value.unwrap_or("").to_string();
                    },
                "suggestions.processing-errors" => {
                        request_suggestions_init(&mut request);
                        request.suggestions.as_mut().unwrap().processing_errors.push(value.unwrap_or("").to_string());
                    },
                "suggestions.editor-suggestions" => {
                        request_suggestions_init(&mut request);
                        request.suggestions.as_mut().unwrap().editor_suggestions.push(value.unwrap_or("").to_string());
                    },
                "suggestions.processing-warnings" => {
                        request_suggestions_init(&mut request);
                        request.suggestions.as_mut().unwrap().processing_warnings.push(value.unwrap_or("").to_string());
                    },
                "suggestions.processing-hints" => {
                        request_suggestions_init(&mut request);
                        request.suggestions.as_mut().unwrap().processing_hints.push(value.unwrap_or("").to_string());
                    },
                "live-streaming-details.concurrent-viewers" => {
                        request_live_streaming_details_init(&mut request);
                        request.live_streaming_details.as_mut().unwrap().concurrent_viewers = value.unwrap_or("").to_string();
                    },
                "live-streaming-details.scheduled-start-time" => {
                        request_live_streaming_details_init(&mut request);
                        request.live_streaming_details.as_mut().unwrap().scheduled_start_time = value.unwrap_or("").to_string();
                    },
                "live-streaming-details.scheduled-end-time" => {
                        request_live_streaming_details_init(&mut request);
                        request.live_streaming_details.as_mut().unwrap().scheduled_end_time = value.unwrap_or("").to_string();
                    },
                "live-streaming-details.actual-start-time" => {
                        request_live_streaming_details_init(&mut request);
                        request.live_streaming_details.as_mut().unwrap().actual_start_time = value.unwrap_or("").to_string();
                    },
                "live-streaming-details.actual-end-time" => {
                        request_live_streaming_details_init(&mut request);
                        request.live_streaming_details.as_mut().unwrap().actual_end_time = value.unwrap_or("").to_string();
                    },
                "file-details.bitrate-bps" => {
                        request_file_details_init(&mut request);
                        request.file_details.as_mut().unwrap().bitrate_bps = value.unwrap_or("").to_string();
                    },
                "file-details.container" => {
                        request_file_details_init(&mut request);
                        request.file_details.as_mut().unwrap().container = value.unwrap_or("").to_string();
                    },
                "file-details.recording-location.latitude" => {
                        request_file_details_init(&mut request);
                        request.file_details.as_mut().unwrap().recording_location.latitude = arg_from_str(value.unwrap_or("0.0"), err, "file-details.recording-location.latitude", "number");
                    },
                "file-details.recording-location.altitude" => {
                        request_file_details_init(&mut request);
                        request.file_details.as_mut().unwrap().recording_location.altitude = arg_from_str(value.unwrap_or("0.0"), err, "file-details.recording-location.altitude", "number");
                    },
                "file-details.recording-location.longitude" => {
                        request_file_details_init(&mut request);
                        request.file_details.as_mut().unwrap().recording_location.longitude = arg_from_str(value.unwrap_or("0.0"), err, "file-details.recording-location.longitude", "number");
                    },
                "file-details.file-type" => {
                        request_file_details_init(&mut request);
                        request.file_details.as_mut().unwrap().file_type = value.unwrap_or("").to_string();
                    },
                "file-details.creation-time" => {
                        request_file_details_init(&mut request);
                        request.file_details.as_mut().unwrap().creation_time = value.unwrap_or("").to_string();
                    },
                "file-details.duration-ms" => {
                        request_file_details_init(&mut request);
                        request.file_details.as_mut().unwrap().duration_ms = value.unwrap_or("").to_string();
                    },
                "file-details.file-name" => {
                        request_file_details_init(&mut request);
                        request.file_details.as_mut().unwrap().file_name = value.unwrap_or("").to_string();
                    },
                "file-details.file-size" => {
                        request_file_details_init(&mut request);
                        request.file_details.as_mut().unwrap().file_size = value.unwrap_or("").to_string();
                    },
                "snippet.description" => {
                        request_snippet_init(&mut request);
                        request.snippet.as_mut().unwrap().description = value.unwrap_or("").to_string();
                    },
                "snippet.tags" => {
                        request_snippet_init(&mut request);
                        request.snippet.as_mut().unwrap().tags.push(value.unwrap_or("").to_string());
                    },
                "snippet.channel-id" => {
                        request_snippet_init(&mut request);
                        request.snippet.as_mut().unwrap().channel_id = value.unwrap_or("").to_string();
                    },
                "snippet.published-at" => {
                        request_snippet_init(&mut request);
                        request.snippet.as_mut().unwrap().published_at = value.unwrap_or("").to_string();
                    },
                "snippet.live-broadcast-content" => {
                        request_snippet_init(&mut request);
                        request.snippet.as_mut().unwrap().live_broadcast_content = value.unwrap_or("").to_string();
                    },
                "snippet.default-language" => {
                        request_snippet_init(&mut request);
                        request.snippet.as_mut().unwrap().default_language = value.unwrap_or("").to_string();
                    },
                "snippet.channel-title" => {
                        request_snippet_init(&mut request);
                        request.snippet.as_mut().unwrap().channel_title = value.unwrap_or("").to_string();
                    },
                "snippet.title" => {
                        request_snippet_init(&mut request);
                        request.snippet.as_mut().unwrap().title = value.unwrap_or("").to_string();
                    },
                "snippet.category-id" => {
                        request_snippet_init(&mut request);
                        request.snippet.as_mut().unwrap().category_id = value.unwrap_or("").to_string();
                    },
                "snippet.localized.description" => {
                        request_snippet_init(&mut request);
                        request.snippet.as_mut().unwrap().localized.description = value.unwrap_or("").to_string();
                    },
                "snippet.localized.title" => {
                        request_snippet_init(&mut request);
                        request.snippet.as_mut().unwrap().localized.title = value.unwrap_or("").to_string();
                    },
                "snippet.thumbnails.default.url" => {
                        request_snippet_init(&mut request);
                        request.snippet.as_mut().unwrap().thumbnails.default.url = Some(value.unwrap_or("").to_string());
                    },
                "snippet.thumbnails.default.width" => {
                        request_snippet_init(&mut request);
                        request.snippet.as_mut().unwrap().thumbnails.default.width = Some(arg_from_str(value.unwrap_or("-0"), err, "snippet.thumbnails.default.width", "integer"));
                    },
                "snippet.thumbnails.default.height" => {
                        request_snippet_init(&mut request);
                        request.snippet.as_mut().unwrap().thumbnails.default.height = Some(arg_from_str(value.unwrap_or("-0"), err, "snippet.thumbnails.default.height", "integer"));
                    },
                "snippet.thumbnails.high.url" => {
                        request_snippet_init(&mut request);
                        request.snippet.as_mut().unwrap().thumbnails.high.url = Some(value.unwrap_or("").to_string());
                    },
                "snippet.thumbnails.high.width" => {
                        request_snippet_init(&mut request);
                        request.snippet.as_mut().unwrap().thumbnails.high.width = Some(arg_from_str(value.unwrap_or("-0"), err, "snippet.thumbnails.high.width", "integer"));
                    },
                "snippet.thumbnails.high.height" => {
                        request_snippet_init(&mut request);
                        request.snippet.as_mut().unwrap().thumbnails.high.height = Some(arg_from_str(value.unwrap_or("-0"), err, "snippet.thumbnails.high.height", "integer"));
                    },
                "snippet.thumbnails.medium.url" => {
                        request_snippet_init(&mut request);
                        request.snippet.as_mut().unwrap().thumbnails.medium.url = Some(value.unwrap_or("").to_string());
                    },
                "snippet.thumbnails.medium.width" => {
                        request_snippet_init(&mut request);
                        request.snippet.as_mut().unwrap().thumbnails.medium.width = Some(arg_from_str(value.unwrap_or("-0"), err, "snippet.thumbnails.medium.width", "integer"));
                    },
                "snippet.thumbnails.medium.height" => {
                        request_snippet_init(&mut request);
                        request.snippet.as_mut().unwrap().thumbnails.medium.height = Some(arg_from_str(value.unwrap_or("-0"), err, "snippet.thumbnails.medium.height", "integer"));
                    },
                "snippet.thumbnails.maxres.url" => {
                        request_snippet_init(&mut request);
                        request.snippet.as_mut().unwrap().thumbnails.maxres.url = Some(value.unwrap_or("").to_string());
                    },
                "snippet.thumbnails.maxres.width" => {
                        request_snippet_init(&mut request);
                        request.snippet.as_mut().unwrap().thumbnails.maxres.width = Some(arg_from_str(value.unwrap_or("-0"), err, "snippet.thumbnails.maxres.width", "integer"));
                    },
                "snippet.thumbnails.maxres.height" => {
                        request_snippet_init(&mut request);
                        request.snippet.as_mut().unwrap().thumbnails.maxres.height = Some(arg_from_str(value.unwrap_or("-0"), err, "snippet.thumbnails.maxres.height", "integer"));
                    },
                "snippet.thumbnails.standard.url" => {
                        request_snippet_init(&mut request);
                        request.snippet.as_mut().unwrap().thumbnails.standard.url = Some(value.unwrap_or("").to_string());
                    },
                "snippet.thumbnails.standard.width" => {
                        request_snippet_init(&mut request);
                        request.snippet.as_mut().unwrap().thumbnails.standard.width = Some(arg_from_str(value.unwrap_or("-0"), err, "snippet.thumbnails.standard.width", "integer"));
                    },
                "snippet.thumbnails.standard.height" => {
                        request_snippet_init(&mut request);
                        request.snippet.as_mut().unwrap().thumbnails.standard.height = Some(arg_from_str(value.unwrap_or("-0"), err, "snippet.thumbnails.standard.height", "integer"));
                    },
                "player.embed-html" => {
                        request_player_init(&mut request);
                        request.player.as_mut().unwrap().embed_html = value.unwrap_or("").to_string();
                    },
                "processing-details.file-details-availability" => {
                        request_processing_details_init(&mut request);
                        request.processing_details.as_mut().unwrap().file_details_availability = value.unwrap_or("").to_string();
                    },
                "processing-details.editor-suggestions-availability" => {
                        request_processing_details_init(&mut request);
                        request.processing_details.as_mut().unwrap().editor_suggestions_availability = value.unwrap_or("").to_string();
                    },
                "processing-details.processing-status" => {
                        request_processing_details_init(&mut request);
                        request.processing_details.as_mut().unwrap().processing_status = value.unwrap_or("").to_string();
                    },
                "processing-details.processing-issues-availability" => {
                        request_processing_details_init(&mut request);
                        request.processing_details.as_mut().unwrap().processing_issues_availability = value.unwrap_or("").to_string();
                    },
                "processing-details.processing-failure-reason" => {
                        request_processing_details_init(&mut request);
                        request.processing_details.as_mut().unwrap().processing_failure_reason = value.unwrap_or("").to_string();
                    },
                "processing-details.thumbnails-availability" => {
                        request_processing_details_init(&mut request);
                        request.processing_details.as_mut().unwrap().thumbnails_availability = value.unwrap_or("").to_string();
                    },
                "processing-details.processing-progress.time-left-ms" => {
                        request_processing_details_init(&mut request);
                        request.processing_details.as_mut().unwrap().processing_progress.time_left_ms = value.unwrap_or("").to_string();
                    },
                "processing-details.processing-progress.parts-processed" => {
                        request_processing_details_init(&mut request);
                        request.processing_details.as_mut().unwrap().processing_progress.parts_processed = value.unwrap_or("").to_string();
                    },
                "processing-details.processing-progress.parts-total" => {
                        request_processing_details_init(&mut request);
                        request.processing_details.as_mut().unwrap().processing_progress.parts_total = value.unwrap_or("").to_string();
                    },
                "processing-details.tag-suggestions-availability" => {
                        request_processing_details_init(&mut request);
                        request.processing_details.as_mut().unwrap().tag_suggestions_availability = value.unwrap_or("").to_string();
                    },
                "etag" => {
                        request_processing_details_init(&mut request);
                        request.etag = Some(value.unwrap_or("").to_string());
                    },
                "project-details.tags" => {
                        request_project_details_init(&mut request);
                        request.project_details.as_mut().unwrap().tags.push(value.unwrap_or("").to_string());
                    },
                "recording-details.recording-date" => {
                        request_recording_details_init(&mut request);
                        request.recording_details.as_mut().unwrap().recording_date = value.unwrap_or("").to_string();
                    },
                "recording-details.location-description" => {
                        request_recording_details_init(&mut request);
                        request.recording_details.as_mut().unwrap().location_description = value.unwrap_or("").to_string();
                    },
                "recording-details.location.latitude" => {
                        request_recording_details_init(&mut request);
                        request.recording_details.as_mut().unwrap().location.latitude = arg_from_str(value.unwrap_or("0.0"), err, "recording-details.location.latitude", "number");
                    },
                "recording-details.location.altitude" => {
                        request_recording_details_init(&mut request);
                        request.recording_details.as_mut().unwrap().location.altitude = arg_from_str(value.unwrap_or("0.0"), err, "recording-details.location.altitude", "number");
                    },
                "recording-details.location.longitude" => {
                        request_recording_details_init(&mut request);
                        request.recording_details.as_mut().unwrap().location.longitude = arg_from_str(value.unwrap_or("0.0"), err, "recording-details.location.longitude", "number");
                    },
                "id" => {
                        request_recording_details_init(&mut request);
                        request.id = Some(value.unwrap_or("").to_string());
                    },
                _ => {
                    err.issues.push(CLIError::Field(FieldError::Unknown(field_name.to_string())));
                }
            }
        }
        let protocol = "standard-request";
        if dry_run {
            None
        } else {
            assert!(err.issues.len() == 0);
            let mut ostream = writer_from_opts(self.opt.flag_o, &self.opt.arg_out);
            match match protocol {
                "standard-request" => call.doit(),
                _ => unreachable!(),
            } {
                Err(api_err) => Some(api_err),
                Ok((mut response, output_schema)) => {
                    println!("DEBUG: REMOVE ME {:?}", response);
                    serde::json::to_writer(&mut ostream, &output_schema).unwrap();
                    None
                }
            }
        }
    }

    fn _watermarks_set(&self, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Option<api::Error> {
            let mut request: api::InvideoBranding = Default::default();
        let mut call = self.hub.watermarks().set(&request, &self.opt.arg_channel_id);
        for parg in self.opt.arg_v.iter() {
            let (key, value) = parse_kv_arg(&*parg, err);
            match key {
                "on-behalf-of-content-owner" => {
                    call = call.on_behalf_of_content_owner(value.unwrap_or(""));
                },
                "alt"
                |"fields"
                |"key"
                |"oauth-token"
                |"pretty-print"
                |"quota-user"
                |"user-ip" => {
                    let map = [
                        ("oauth-token", "oauth_token"),
                        ("pretty-print", "prettyPrint"),
                        ("quota-user", "quotaUser"),
                        ("user-ip", "userIp"),
                    ];
                    call = call.param(map.iter().find(|t| t.0 == key).unwrap_or(&("", key)).1, value.unwrap_or("unset"))
                },
                _ => err.issues.push(CLIError::UnknownParameter(key.to_string())),
            }
        }
        let mut field_name: FieldCursor = Default::default();
        for kvarg in self.opt.arg_kv.iter() {
            let (key, value) = parse_kv_arg(&*kvarg, err);
            if let Err(field_err) = field_name.set(&*key) {
                err.issues.push(field_err);
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
            
            match &field_name.to_string()[..] {
                "target-channel-id" => {
                        request.target_channel_id = Some(value.unwrap_or("").to_string());
                    },
                "position.corner-position" => {
                        request_position_init(&mut request);
                        request.position.as_mut().unwrap().corner_position = value.unwrap_or("").to_string();
                    },
                "position.type" => {
                        request_position_init(&mut request);
                        request.position.as_mut().unwrap().type_ = value.unwrap_or("").to_string();
                    },
                "image-url" => {
                        request_position_init(&mut request);
                        request.image_url = Some(value.unwrap_or("").to_string());
                    },
                "timing.offset-ms" => {
                        request_timing_init(&mut request);
                        request.timing.as_mut().unwrap().offset_ms = value.unwrap_or("").to_string();
                    },
                "timing.type" => {
                        request_timing_init(&mut request);
                        request.timing.as_mut().unwrap().type_ = value.unwrap_or("").to_string();
                    },
                "timing.duration-ms" => {
                        request_timing_init(&mut request);
                        request.timing.as_mut().unwrap().duration_ms = value.unwrap_or("").to_string();
                    },
                "image-bytes" => {
                        request_timing_init(&mut request);
                        request.image_bytes = Some(value.unwrap_or("").to_string());
                    },
                _ => {
                    err.issues.push(CLIError::Field(FieldError::Unknown(field_name.to_string())));
                }
            }
        }
        let protocol = 
            if self.opt.cmd_simple {
                "simple"
            } else if self.opt.cmd_resumable {
                "resumable"
            } else { 
                unreachable!() 
            };
        let mut input_file = input_file_from_opts(&self.opt.arg_file, err);
        let mime_type = input_mime_from_opts(&self.opt.arg_mime, err);
        if dry_run {
            None
        } else {
            assert!(err.issues.len() == 0);
            match match protocol {
                "simple" => call.upload(input_file.unwrap(), mime_type.unwrap()),
                "resumable" => call.upload_resumable(input_file.unwrap(), mime_type.unwrap()),
                _ => unreachable!(),
            } {
                Err(api_err) => Some(api_err),
                Ok(mut response) => {
                    println!("DEBUG: REMOVE ME {:?}", response);
                    None
                }
            }
        }
    }

    fn _watermarks_unset(&self, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Option<api::Error> {
        let mut call = self.hub.watermarks().unset(&self.opt.arg_channel_id);
        for parg in self.opt.arg_v.iter() {
            let (key, value) = parse_kv_arg(&*parg, err);
            match key {
                "on-behalf-of-content-owner" => {
                    call = call.on_behalf_of_content_owner(value.unwrap_or(""));
                },
                "alt"
                |"fields"
                |"key"
                |"oauth-token"
                |"pretty-print"
                |"quota-user"
                |"user-ip" => {
                    let map = [
                        ("oauth-token", "oauth_token"),
                        ("pretty-print", "prettyPrint"),
                        ("quota-user", "quotaUser"),
                        ("user-ip", "userIp"),
                    ];
                    call = call.param(map.iter().find(|t| t.0 == key).unwrap_or(&("", key)).1, value.unwrap_or("unset"))
                },
                _ => err.issues.push(CLIError::UnknownParameter(key.to_string())),
            }
        }
        let protocol = "standard-request";
        if dry_run {
            None
        } else {
            assert!(err.issues.len() == 0);
            match match protocol {
                "standard-request" => call.doit(),
                _ => unreachable!(),
            } {
                Err(api_err) => Some(api_err),
                Ok(mut response) => {
                    println!("DEBUG: REMOVE ME {:?}", response);
                    None
                }
            }
        }
    }

    fn _doit(&self, dry_run: bool) -> (Option<api::Error>, Option<InvalidOptionsError>) {
        let mut err = InvalidOptionsError::new();
        let mut call_result: Option<api::Error>;
        let mut err_opt: Option<InvalidOptionsError> = None;

        if self.opt.cmd_activities {
            if self.opt.cmd_insert {
                call_result = self._activities_insert(dry_run, &mut err);
            } else if self.opt.cmd_list {
                call_result = self._activities_list(dry_run, &mut err);
            } else {
                unreachable!();
            }
        } else if self.opt.cmd_captions {
            if self.opt.cmd_delete {
                call_result = self._captions_delete(dry_run, &mut err);
            } else if self.opt.cmd_download {
                call_result = self._captions_download(dry_run, &mut err);
            } else if self.opt.cmd_insert {
                call_result = self._captions_insert(dry_run, &mut err);
            } else if self.opt.cmd_list {
                call_result = self._captions_list(dry_run, &mut err);
            } else if self.opt.cmd_update {
                call_result = self._captions_update(dry_run, &mut err);
            } else {
                unreachable!();
            }
        } else if self.opt.cmd_channel_banners {
            if self.opt.cmd_insert {
                call_result = self._channel_banners_insert(dry_run, &mut err);
            } else {
                unreachable!();
            }
        } else if self.opt.cmd_channel_sections {
            if self.opt.cmd_delete {
                call_result = self._channel_sections_delete(dry_run, &mut err);
            } else if self.opt.cmd_insert {
                call_result = self._channel_sections_insert(dry_run, &mut err);
            } else if self.opt.cmd_list {
                call_result = self._channel_sections_list(dry_run, &mut err);
            } else if self.opt.cmd_update {
                call_result = self._channel_sections_update(dry_run, &mut err);
            } else {
                unreachable!();
            }
        } else if self.opt.cmd_channels {
            if self.opt.cmd_list {
                call_result = self._channels_list(dry_run, &mut err);
            } else if self.opt.cmd_update {
                call_result = self._channels_update(dry_run, &mut err);
            } else {
                unreachable!();
            }
        } else if self.opt.cmd_guide_categories {
            if self.opt.cmd_list {
                call_result = self._guide_categories_list(dry_run, &mut err);
            } else {
                unreachable!();
            }
        } else if self.opt.cmd_i18n_languages {
            if self.opt.cmd_list {
                call_result = self._i18n_languages_list(dry_run, &mut err);
            } else {
                unreachable!();
            }
        } else if self.opt.cmd_i18n_regions {
            if self.opt.cmd_list {
                call_result = self._i18n_regions_list(dry_run, &mut err);
            } else {
                unreachable!();
            }
        } else if self.opt.cmd_live_broadcasts {
            if self.opt.cmd_bind {
                call_result = self._live_broadcasts_bind(dry_run, &mut err);
            } else if self.opt.cmd_control {
                call_result = self._live_broadcasts_control(dry_run, &mut err);
            } else if self.opt.cmd_delete {
                call_result = self._live_broadcasts_delete(dry_run, &mut err);
            } else if self.opt.cmd_insert {
                call_result = self._live_broadcasts_insert(dry_run, &mut err);
            } else if self.opt.cmd_list {
                call_result = self._live_broadcasts_list(dry_run, &mut err);
            } else if self.opt.cmd_transition {
                call_result = self._live_broadcasts_transition(dry_run, &mut err);
            } else if self.opt.cmd_update {
                call_result = self._live_broadcasts_update(dry_run, &mut err);
            } else {
                unreachable!();
            }
        } else if self.opt.cmd_live_streams {
            if self.opt.cmd_delete {
                call_result = self._live_streams_delete(dry_run, &mut err);
            } else if self.opt.cmd_insert {
                call_result = self._live_streams_insert(dry_run, &mut err);
            } else if self.opt.cmd_list {
                call_result = self._live_streams_list(dry_run, &mut err);
            } else if self.opt.cmd_update {
                call_result = self._live_streams_update(dry_run, &mut err);
            } else {
                unreachable!();
            }
        } else if self.opt.cmd_playlist_items {
            if self.opt.cmd_delete {
                call_result = self._playlist_items_delete(dry_run, &mut err);
            } else if self.opt.cmd_insert {
                call_result = self._playlist_items_insert(dry_run, &mut err);
            } else if self.opt.cmd_list {
                call_result = self._playlist_items_list(dry_run, &mut err);
            } else if self.opt.cmd_update {
                call_result = self._playlist_items_update(dry_run, &mut err);
            } else {
                unreachable!();
            }
        } else if self.opt.cmd_playlists {
            if self.opt.cmd_delete {
                call_result = self._playlists_delete(dry_run, &mut err);
            } else if self.opt.cmd_insert {
                call_result = self._playlists_insert(dry_run, &mut err);
            } else if self.opt.cmd_list {
                call_result = self._playlists_list(dry_run, &mut err);
            } else if self.opt.cmd_update {
                call_result = self._playlists_update(dry_run, &mut err);
            } else {
                unreachable!();
            }
        } else if self.opt.cmd_search {
            if self.opt.cmd_list {
                call_result = self._search_list(dry_run, &mut err);
            } else {
                unreachable!();
            }
        } else if self.opt.cmd_subscriptions {
            if self.opt.cmd_delete {
                call_result = self._subscriptions_delete(dry_run, &mut err);
            } else if self.opt.cmd_insert {
                call_result = self._subscriptions_insert(dry_run, &mut err);
            } else if self.opt.cmd_list {
                call_result = self._subscriptions_list(dry_run, &mut err);
            } else {
                unreachable!();
            }
        } else if self.opt.cmd_thumbnails {
            if self.opt.cmd_set {
                call_result = self._thumbnails_set(dry_run, &mut err);
            } else {
                unreachable!();
            }
        } else if self.opt.cmd_video_categories {
            if self.opt.cmd_list {
                call_result = self._video_categories_list(dry_run, &mut err);
            } else {
                unreachable!();
            }
        } else if self.opt.cmd_videos {
            if self.opt.cmd_delete {
                call_result = self._videos_delete(dry_run, &mut err);
            } else if self.opt.cmd_get_rating {
                call_result = self._videos_get_rating(dry_run, &mut err);
            } else if self.opt.cmd_insert {
                call_result = self._videos_insert(dry_run, &mut err);
            } else if self.opt.cmd_list {
                call_result = self._videos_list(dry_run, &mut err);
            } else if self.opt.cmd_rate {
                call_result = self._videos_rate(dry_run, &mut err);
            } else if self.opt.cmd_update {
                call_result = self._videos_update(dry_run, &mut err);
            } else {
                unreachable!();
            }
        } else if self.opt.cmd_watermarks {
            if self.opt.cmd_set {
                call_result = self._watermarks_set(dry_run, &mut err);
            } else if self.opt.cmd_unset {
                call_result = self._watermarks_unset(dry_run, &mut err);
            } else {
                unreachable!();
            }
        } else {
            unreachable!();
        }

        if dry_run {
            if err.issues.len() > 0 {
                err_opt = Some(err);
            }
        }
        (call_result, err_opt)
    }

    // Please note that this call will fail if any part of the opt can't be handled
    fn new(opt: Options) -> Result<Engine, InvalidOptionsError> {
        let (config_dir, secret) = {
            let config_dir = match cmn::assure_config_dir_exists(&opt.flag_config_dir) {
                Err(e) => return Err(InvalidOptionsError::single(e, 3)),
                Ok(p) => p,
            };

            match cmn::application_secret_from_directory(&config_dir, "youtube3-secret.json") {
                Ok(secret) => (config_dir, secret),
                Err(e) => return Err(InvalidOptionsError::single(e, 4))
            }
        };

        let auth = Authenticator::new(&secret, DefaultAuthenticatorDelegate,
                                      hyper::Client::new(),
                                      JsonTokenStorage {
                                        program_name: "youtube3",
                                        db_dir: config_dir.clone(),
                                      }, None);
        let engine = Engine {
            opt: opt,
            hub: api::YouTube::new(hyper::Client::new(), auth),
        };

        match engine._doit(true) {
            (_, Some(err)) => Err(err),
            _ => Ok(engine),
        }
    }

    // Execute the call with all the bells and whistles, informing the caller only if there was an error.
    // The absense of one indicates success.
    fn doit(&self) -> Option<api::Error> {
        self._doit(false).0
    }
}

fn main() {
    let opts: Options = Options::docopt().decode().unwrap_or_else(|e| e.exit());
    match Engine::new(opts) {
        Err(err) => {
            write!(io::stderr(), "{}", err).ok();
            env::set_exit_status(err.exit_code);
        },
        Ok(engine) => {
            if let Some(err) = engine.doit() {
                write!(io::stderr(), "{}", err).ok();
                env::set_exit_status(1);
            }
        }
    }
}