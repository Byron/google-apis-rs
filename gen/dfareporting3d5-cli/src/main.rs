// DO NOT EDIT !
// This file was generated automatically from 'src/generator/templates/cli/main.rs.mako'
// DO NOT EDIT !
#![allow(unused_variables, unused_imports, dead_code, unused_mut)]

#[macro_use]
extern crate clap;

use std::env;
use std::io::{self, Write};
use clap::{App, SubCommand, Arg};

use google_dfareporting3d5::{api, Error, oauth2, client::chrono, FieldMask};


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
    hub: api::Dfareporting<S>,
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
    async fn _media_upload(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
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
                    "asset-identifier.name" => Some(("assetIdentifier.name", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "asset-identifier.type" => Some(("assetIdentifier.type", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "detected-features" => Some(("detectedFeatures", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Vec })),
                    "id" => Some(("id", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "id-dimension-value.dimension-name" => Some(("idDimensionValue.dimensionName", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "id-dimension-value.etag" => Some(("idDimensionValue.etag", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "id-dimension-value.id" => Some(("idDimensionValue.id", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "id-dimension-value.kind" => Some(("idDimensionValue.kind", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "id-dimension-value.match-type" => Some(("idDimensionValue.matchType", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "id-dimension-value.value" => Some(("idDimensionValue.value", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "kind" => Some(("kind", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "media-request-info.current-bytes" => Some(("mediaRequestInfo.currentBytes", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "media-request-info.custom-data" => Some(("mediaRequestInfo.customData", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "media-request-info.diff-object-version" => Some(("mediaRequestInfo.diffObjectVersion", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "media-request-info.final-status" => Some(("mediaRequestInfo.finalStatus", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "media-request-info.notification-type" => Some(("mediaRequestInfo.notificationType", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "media-request-info.request-id" => Some(("mediaRequestInfo.requestId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "media-request-info.request-received-params-serving-info" => Some(("mediaRequestInfo.requestReceivedParamsServingInfo", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "media-request-info.total-bytes" => Some(("mediaRequestInfo.totalBytes", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "media-request-info.total-bytes-is-estimated" => Some(("mediaRequestInfo.totalBytesIsEstimated", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "media-response-info.custom-data" => Some(("mediaResponseInfo.customData", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "media-response-info.data-storage-transform" => Some(("mediaResponseInfo.dataStorageTransform", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "media-response-info.dynamic-drop-target" => Some(("mediaResponseInfo.dynamicDropTarget", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "media-response-info.dynamic-dropzone" => Some(("mediaResponseInfo.dynamicDropzone", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "media-response-info.request-class" => Some(("mediaResponseInfo.requestClass", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "media-response-info.scotty-agent-user-id" => Some(("mediaResponseInfo.scottyAgentUserId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "media-response-info.scotty-customer-log" => Some(("mediaResponseInfo.scottyCustomerLog", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "media-response-info.traffic-class-field" => Some(("mediaResponseInfo.trafficClassField", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "media-response-info.verify-hash-from-header" => Some(("mediaResponseInfo.verifyHashFromHeader", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "rich-media" => Some(("richMedia", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "warned-validation-rules" => Some(("warnedValidationRules", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Vec })),
                    _ => {
                        let suggestion = FieldCursor::did_you_mean(key, &vec!["asset-identifier", "current-bytes", "custom-data", "data-storage-transform", "detected-features", "diff-object-version", "dimension-name", "dynamic-drop-target", "dynamic-dropzone", "etag", "final-status", "id", "id-dimension-value", "kind", "match-type", "media-request-info", "media-response-info", "name", "notification-type", "request-class", "request-id", "request-received-params-serving-info", "rich-media", "scotty-agent-user-id", "scotty-customer-log", "total-bytes", "total-bytes-is-estimated", "traffic-class-field", "type", "value", "verify-hash-from-header", "warned-validation-rules"]);
                        err.issues.push(CLIError::Field(FieldError::Unknown(temp_cursor.to_string(), suggestion, value.map(|v| v.to_string()))));
                        None
                    }
                };
            if let Some((field_cursor_str, type_info)) = type_info {
                FieldCursor::from(field_cursor_str).set_json_value(&mut object, value.unwrap(), type_info, err, &temp_cursor);
            }
        }
        let mut request: api::CreativeAssetMetadata = json::value::from_value(object).unwrap();
        let mut call = self.hub.media().upload(request, opt.value_of("profile-id").unwrap_or(""), opt.value_of("advertiser-id").unwrap_or(""));
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

    async fn _doit(&self, dry_run: bool) -> Result<Result<(), DoitError>, Option<InvalidOptionsError>> {
        let mut err = InvalidOptionsError::new();
        let mut call_result: Result<(), DoitError> = Ok(());
        let mut err_opt: Option<InvalidOptionsError> = None;
        match self.opt.subcommand() {
            ("media", Some(opt)) => {
                match opt.subcommand() {
                    ("upload", Some(opt)) => {
                        call_result = self._media_upload(opt, dry_run, &mut err).await;
                    },
                    _ => {
                        err.issues.push(CLIError::MissingMethodError("media".to_string()));
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

            match client::application_secret_from_directory(&config_dir, "dfareporting3d5-secret.json",
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
        ).persist_tokens_to_disk(format!("{}/dfareporting3d5", config_dir)).build().await.unwrap();

        let engine = Engine {
            opt: opt,
            hub: api::Dfareporting::new(client, auth),
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
        ("media", "methods: 'upload'", vec![
            ("upload",
                    Some(r##"Inserts a new creative asset."##),
                    "Details at http://byron.github.io/google-apis-rs/google_dfareporting3d5_cli/media_upload",
                  vec![
                    (Some(r##"profile-id"##),
                     None,
                     Some(r##"User profile ID associated with this request."##),
                     Some(true),
                     Some(false)),
        
                    (Some(r##"advertiser-id"##),
                     None,
                     Some(r##"Advertiser ID of this creative. This is a required field."##),
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
        
                    (Some(r##"out"##),
                     Some(r##"o"##),
                     Some(r##"Specify the file into which to write the program's output"##),
                     Some(false),
                     Some(false)),
                  ]),
            ]),
        
    ];
    
    let mut app = App::new("dfareporting3d5")
           .author("Sebastian Thiel <byronimo@gmail.com>")
           .version("5.0.5+20240625")
           .about("Build applications to efficiently manage large or complex trafficking, reporting, and attribution workflows for Campaign Manager 360.")
           .after_help("All documentation details can be found at http://byron.github.io/google-apis-rs/google_dfareporting3d5_cli")
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
