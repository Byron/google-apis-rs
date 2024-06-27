// DO NOT EDIT !
// This file was generated automatically from 'src/generator/templates/cli/main.rs.mako'
// DO NOT EDIT !
#![allow(unused_variables, unused_imports, dead_code, unused_mut)]

#[macro_use]
extern crate clap;

use std::env;
use std::io::{self, Write};
use clap::{App, SubCommand, Arg};

use google_servicecontrol2::{api, Error, oauth2, client::chrono, FieldMask};


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
    hub: api::ServiceControl<S>,
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
    async fn _services_check(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
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
                    "attributes.api.operation" => Some(("attributes.api.operation", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "attributes.api.protocol" => Some(("attributes.api.protocol", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "attributes.api.service" => Some(("attributes.api.service", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "attributes.api.version" => Some(("attributes.api.version", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "attributes.destination.ip" => Some(("attributes.destination.ip", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "attributes.destination.labels" => Some(("attributes.destination.labels", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Map })),
                    "attributes.destination.port" => Some(("attributes.destination.port", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "attributes.destination.principal" => Some(("attributes.destination.principal", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "attributes.destination.region-code" => Some(("attributes.destination.regionCode", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "attributes.origin.ip" => Some(("attributes.origin.ip", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "attributes.origin.labels" => Some(("attributes.origin.labels", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Map })),
                    "attributes.origin.port" => Some(("attributes.origin.port", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "attributes.origin.principal" => Some(("attributes.origin.principal", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "attributes.origin.region-code" => Some(("attributes.origin.regionCode", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "attributes.request.auth.access-levels" => Some(("attributes.request.auth.accessLevels", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Vec })),
                    "attributes.request.auth.audiences" => Some(("attributes.request.auth.audiences", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Vec })),
                    "attributes.request.auth.credential-id" => Some(("attributes.request.auth.credentialId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "attributes.request.auth.presenter" => Some(("attributes.request.auth.presenter", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "attributes.request.auth.principal" => Some(("attributes.request.auth.principal", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "attributes.request.headers" => Some(("attributes.request.headers", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Map })),
                    "attributes.request.host" => Some(("attributes.request.host", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "attributes.request.id" => Some(("attributes.request.id", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "attributes.request.method" => Some(("attributes.request.method", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "attributes.request.path" => Some(("attributes.request.path", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "attributes.request.protocol" => Some(("attributes.request.protocol", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "attributes.request.query" => Some(("attributes.request.query", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "attributes.request.reason" => Some(("attributes.request.reason", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "attributes.request.scheme" => Some(("attributes.request.scheme", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "attributes.request.size" => Some(("attributes.request.size", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "attributes.request.time" => Some(("attributes.request.time", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "attributes.resource.annotations" => Some(("attributes.resource.annotations", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Map })),
                    "attributes.resource.create-time" => Some(("attributes.resource.createTime", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "attributes.resource.delete-time" => Some(("attributes.resource.deleteTime", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "attributes.resource.display-name" => Some(("attributes.resource.displayName", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "attributes.resource.etag" => Some(("attributes.resource.etag", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "attributes.resource.labels" => Some(("attributes.resource.labels", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Map })),
                    "attributes.resource.location" => Some(("attributes.resource.location", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "attributes.resource.name" => Some(("attributes.resource.name", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "attributes.resource.service" => Some(("attributes.resource.service", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "attributes.resource.type" => Some(("attributes.resource.type", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "attributes.resource.uid" => Some(("attributes.resource.uid", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "attributes.resource.update-time" => Some(("attributes.resource.updateTime", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "attributes.response.backend-latency" => Some(("attributes.response.backendLatency", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "attributes.response.code" => Some(("attributes.response.code", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "attributes.response.headers" => Some(("attributes.response.headers", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Map })),
                    "attributes.response.size" => Some(("attributes.response.size", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "attributes.response.time" => Some(("attributes.response.time", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "attributes.source.ip" => Some(("attributes.source.ip", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "attributes.source.labels" => Some(("attributes.source.labels", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Map })),
                    "attributes.source.port" => Some(("attributes.source.port", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "attributes.source.principal" => Some(("attributes.source.principal", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "attributes.source.region-code" => Some(("attributes.source.regionCode", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "flags" => Some(("flags", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "service-config-id" => Some(("serviceConfigId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    _ => {
                        let suggestion = FieldCursor::did_you_mean(key, &vec!["access-levels", "annotations", "api", "attributes", "audiences", "auth", "backend-latency", "code", "create-time", "credential-id", "delete-time", "destination", "display-name", "etag", "flags", "headers", "host", "id", "ip", "labels", "location", "method", "name", "operation", "origin", "path", "port", "presenter", "principal", "protocol", "query", "reason", "region-code", "request", "resource", "response", "scheme", "service", "service-config-id", "size", "source", "time", "type", "uid", "update-time", "version"]);
                        err.issues.push(CLIError::Field(FieldError::Unknown(temp_cursor.to_string(), suggestion, value.map(|v| v.to_string()))));
                        None
                    }
                };
            if let Some((field_cursor_str, type_info)) = type_info {
                FieldCursor::from(field_cursor_str).set_json_value(&mut object, value.unwrap(), type_info, err, &temp_cursor);
            }
        }
        let mut request: api::CheckRequest = json::value::from_value(object).unwrap();
        let mut call = self.hub.services().check(request, opt.value_of("service-name").unwrap_or(""));
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

    async fn _services_report(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
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
                    "service-config-id" => Some(("serviceConfigId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    _ => {
                        let suggestion = FieldCursor::did_you_mean(key, &vec!["service-config-id"]);
                        err.issues.push(CLIError::Field(FieldError::Unknown(temp_cursor.to_string(), suggestion, value.map(|v| v.to_string()))));
                        None
                    }
                };
            if let Some((field_cursor_str, type_info)) = type_info {
                FieldCursor::from(field_cursor_str).set_json_value(&mut object, value.unwrap(), type_info, err, &temp_cursor);
            }
        }
        let mut request: api::ReportRequest = json::value::from_value(object).unwrap();
        let mut call = self.hub.services().report(request, opt.value_of("service-name").unwrap_or(""));
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
            ("services", Some(opt)) => {
                match opt.subcommand() {
                    ("check", Some(opt)) => {
                        call_result = self._services_check(opt, dry_run, &mut err).await;
                    },
                    ("report", Some(opt)) => {
                        call_result = self._services_report(opt, dry_run, &mut err).await;
                    },
                    _ => {
                        err.issues.push(CLIError::MissingMethodError("services".to_string()));
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

            match client::application_secret_from_directory(&config_dir, "servicecontrol2-secret.json",
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
        ).persist_tokens_to_disk(format!("{}/servicecontrol2", config_dir)).build().await.unwrap();

        let engine = Engine {
            opt: opt,
            hub: api::ServiceControl::new(client, auth),
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
        ("services", "methods: 'check' and 'report'", vec![
            ("check",
                    Some(r##"Private Preview. This feature is only available for approved services. This method provides admission control for services that are integrated with [Service Infrastructure](https://cloud.google.com/service-infrastructure). It checks whether an operation should be allowed based on the service configuration and relevant policies. It must be called before the operation is executed. For more information, see [Admission Control](https://cloud.google.com/service-infrastructure/docs/admission-control). NOTE: The admission control has an expected policy propagation delay of 60s. The caller **must** not depend on the most recent policy changes. NOTE: The admission control has a hard limit of 1 referenced resources per call. If an operation refers to more than 1 resources, the caller must call the Check method multiple times. This method requires the `servicemanagement.services.check` permission on the specified service. For more information, see [Service Control API Access Control](https://cloud.google.com/service-infrastructure/docs/service-control/access-control)."##),
                    "Details at http://byron.github.io/google-apis-rs/google_servicecontrol2_cli/services_check",
                  vec![
                    (Some(r##"service-name"##),
                     None,
                     Some(r##"The service name as specified in its service configuration. For example, `"pubsub.googleapis.com"`. See [google.api.Service](https://cloud.google.com/service-management/reference/rpc/google.api#google.api.Service) for the definition of a service name."##),
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
            ("report",
                    Some(r##"Private Preview. This feature is only available for approved services. This method provides telemetry reporting for services that are integrated with [Service Infrastructure](https://cloud.google.com/service-infrastructure). It reports a list of operations that have occurred on a service. It must be called after the operations have been executed. For more information, see [Telemetry Reporting](https://cloud.google.com/service-infrastructure/docs/telemetry-reporting). NOTE: The telemetry reporting has a hard limit of 1000 operations and 1MB per Report call. It is recommended to have no more than 100 operations per call. This method requires the `servicemanagement.services.report` permission on the specified service. For more information, see [Service Control API Access Control](https://cloud.google.com/service-infrastructure/docs/service-control/access-control)."##),
                    "Details at http://byron.github.io/google-apis-rs/google_servicecontrol2_cli/services_report",
                  vec![
                    (Some(r##"service-name"##),
                     None,
                     Some(r##"The service name as specified in its service configuration. For example, `"pubsub.googleapis.com"`. See [google.api.Service](https://cloud.google.com/service-management/reference/rpc/google.api#google.api.Service) for the definition of a service name."##),
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
    
    let mut app = App::new("servicecontrol2")
           .author("Sebastian Thiel <byronimo@gmail.com>")
           .version("5.0.5+20240607")
           .about("Provides admission control and telemetry reporting for services integrated with Service Infrastructure. ")
           .after_help("All documentation details can be found at http://byron.github.io/google-apis-rs/google_servicecontrol2_cli")
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
