// DO NOT EDIT !
// This file was generated automatically from 'src/generator/templates/cli/main.rs.mako'
// DO NOT EDIT !
#![allow(unused_variables, unused_imports, dead_code, unused_mut)]

#[macro_use]
extern crate clap;

use std::io::Write;

use clap::{App, Arg, SubCommand};

use google_homegraph1::{api, yup_oauth2, Error};

use google_apis_common as apis_common;
use google_clis_common as common;

use std::str::FromStr;

use clap::ArgMatches;
use http_body_util::BodyExt;

use common::{
    arg_from_str, calltype_from_str, input_file_from_opts, input_mime_from_opts, parse_kv_arg,
    remove_json_null_values, writer_from_opts, CLIError, CallType, ComplexType, FieldCursor,
    FieldError, InvalidOptionsError, JsonType, JsonTypeInfo, UploadProtocol,
};

enum DoitError {
    IoError(String, std::io::Error),
    ApiError(Error),
}

struct Engine<'n, C> {
    opt: ArgMatches<'n>,
    hub: api::HomeGraphService<C>,
    gp: Vec<&'static str>,
    gpm: Vec<(&'static str, &'static str)>,
}

impl<'n, C> Engine<'n, C>
where
    C: apis_common::Connector,
{
    async fn _agent_users_delete(
        &self,
        opt: &ArgMatches<'n>,
        dry_run: bool,
        err: &mut InvalidOptionsError,
    ) -> Result<(), DoitError> {
        let mut call = self
            .hub
            .agent_users()
            .delete(opt.value_of("agent-user-id").unwrap_or(""));
        for parg in opt
            .values_of("v")
            .map(|i| i.collect())
            .unwrap_or(Vec::new())
            .iter()
        {
            let (key, value) = parse_kv_arg(&*parg, err, false);
            match key {
                "request-id" => {
                    call = call.request_id(value.unwrap_or(""));
                }
                _ => {
                    let mut found = false;
                    for param in &self.gp {
                        if key == *param {
                            found = true;
                            call = call.param(
                                self.gpm.iter().find(|t| t.0 == key).unwrap_or(&("", key)).1,
                                value.unwrap_or("unset"),
                            );
                            break;
                        }
                    }
                    if !found {
                        err.issues
                            .push(CLIError::UnknownParameter(key.to_string(), {
                                let mut v = Vec::new();
                                v.extend(self.gp.iter().map(|v| *v));
                                v.extend(["request-id"].iter().map(|v| *v));
                                v
                            }));
                    }
                }
            }
        }
        let protocol = CallType::Standard;
        if dry_run {
            Ok(())
        } else {
            assert!(err.issues.len() == 0);
            for scope in self
                .opt
                .values_of("url")
                .map(|i| i.collect())
                .unwrap_or(Vec::new())
                .iter()
            {
                call = call.add_scope(scope);
            }
            let mut ostream = match writer_from_opts(opt.value_of("out")) {
                Ok(mut f) => f,
                Err(io_err) => {
                    return Err(DoitError::IoError(
                        opt.value_of("out").unwrap_or("-").to_string(),
                        io_err,
                    ))
                }
            };
            match match protocol {
                CallType::Standard => call.doit().await,
                _ => unreachable!(),
            } {
                Err(api_err) => Err(DoitError::ApiError(api_err)),
                Ok((mut response, output_schema)) => {
                    let mut value =
                        serde_json::value::to_value(&output_schema).expect("serde to work");
                    remove_json_null_values(&mut value);
                    serde_json::to_writer_pretty(&mut ostream, &value).unwrap();
                    ostream.flush().unwrap();
                    Ok(())
                }
            }
        }
    }

    async fn _devices_query(
        &self,
        opt: &ArgMatches<'n>,
        dry_run: bool,
        err: &mut InvalidOptionsError,
    ) -> Result<(), DoitError> {
        let mut field_cursor = FieldCursor::default();
        let mut object = serde_json::value::Value::Object(Default::default());

        for kvarg in opt
            .values_of("kv")
            .map(|i| i.collect())
            .unwrap_or(Vec::new())
            .iter()
        {
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

            let type_info: Option<(&'static str, JsonTypeInfo)> = match &temp_cursor.to_string()[..]
            {
                "agent-user-id" => Some((
                    "agentUserId",
                    JsonTypeInfo {
                        jtype: JsonType::String,
                        ctype: ComplexType::Pod,
                    },
                )),
                "request-id" => Some((
                    "requestId",
                    JsonTypeInfo {
                        jtype: JsonType::String,
                        ctype: ComplexType::Pod,
                    },
                )),
                _ => {
                    let suggestion =
                        FieldCursor::did_you_mean(key, &vec!["agent-user-id", "request-id"]);
                    err.issues.push(CLIError::Field(FieldError::Unknown(
                        temp_cursor.to_string(),
                        suggestion,
                        value.map(|v| v.to_string()),
                    )));
                    None
                }
            };
            if let Some((field_cursor_str, type_info)) = type_info {
                FieldCursor::from(field_cursor_str).set_json_value(
                    &mut object,
                    value.unwrap(),
                    type_info,
                    err,
                    &temp_cursor,
                );
            }
        }
        let mut request: api::QueryRequest = serde_json::value::from_value(object).unwrap();
        let mut call = self.hub.devices().query(request);
        for parg in opt
            .values_of("v")
            .map(|i| i.collect())
            .unwrap_or(Vec::new())
            .iter()
        {
            let (key, value) = parse_kv_arg(&*parg, err, false);
            match key {
                _ => {
                    let mut found = false;
                    for param in &self.gp {
                        if key == *param {
                            found = true;
                            call = call.param(
                                self.gpm.iter().find(|t| t.0 == key).unwrap_or(&("", key)).1,
                                value.unwrap_or("unset"),
                            );
                            break;
                        }
                    }
                    if !found {
                        err.issues
                            .push(CLIError::UnknownParameter(key.to_string(), {
                                let mut v = Vec::new();
                                v.extend(self.gp.iter().map(|v| *v));
                                v
                            }));
                    }
                }
            }
        }
        let protocol = CallType::Standard;
        if dry_run {
            Ok(())
        } else {
            assert!(err.issues.len() == 0);
            for scope in self
                .opt
                .values_of("url")
                .map(|i| i.collect())
                .unwrap_or(Vec::new())
                .iter()
            {
                call = call.add_scope(scope);
            }
            let mut ostream = match writer_from_opts(opt.value_of("out")) {
                Ok(mut f) => f,
                Err(io_err) => {
                    return Err(DoitError::IoError(
                        opt.value_of("out").unwrap_or("-").to_string(),
                        io_err,
                    ))
                }
            };
            match match protocol {
                CallType::Standard => call.doit().await,
                _ => unreachable!(),
            } {
                Err(api_err) => Err(DoitError::ApiError(api_err)),
                Ok((mut response, output_schema)) => {
                    let mut value =
                        serde_json::value::to_value(&output_schema).expect("serde to work");
                    remove_json_null_values(&mut value);
                    serde_json::to_writer_pretty(&mut ostream, &value).unwrap();
                    ostream.flush().unwrap();
                    Ok(())
                }
            }
        }
    }

    async fn _devices_report_state_and_notification(
        &self,
        opt: &ArgMatches<'n>,
        dry_run: bool,
        err: &mut InvalidOptionsError,
    ) -> Result<(), DoitError> {
        let mut field_cursor = FieldCursor::default();
        let mut object = serde_json::value::Value::Object(Default::default());

        for kvarg in opt
            .values_of("kv")
            .map(|i| i.collect())
            .unwrap_or(Vec::new())
            .iter()
        {
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

            let type_info: Option<(&'static str, JsonTypeInfo)> = match &temp_cursor.to_string()[..]
            {
                "agent-user-id" => Some((
                    "agentUserId",
                    JsonTypeInfo {
                        jtype: JsonType::String,
                        ctype: ComplexType::Pod,
                    },
                )),
                "event-id" => Some((
                    "eventId",
                    JsonTypeInfo {
                        jtype: JsonType::String,
                        ctype: ComplexType::Pod,
                    },
                )),
                "follow-up-token" => Some((
                    "followUpToken",
                    JsonTypeInfo {
                        jtype: JsonType::String,
                        ctype: ComplexType::Pod,
                    },
                )),
                "request-id" => Some((
                    "requestId",
                    JsonTypeInfo {
                        jtype: JsonType::String,
                        ctype: ComplexType::Pod,
                    },
                )),
                _ => {
                    let suggestion = FieldCursor::did_you_mean(
                        key,
                        &vec!["agent-user-id", "event-id", "follow-up-token", "request-id"],
                    );
                    err.issues.push(CLIError::Field(FieldError::Unknown(
                        temp_cursor.to_string(),
                        suggestion,
                        value.map(|v| v.to_string()),
                    )));
                    None
                }
            };
            if let Some((field_cursor_str, type_info)) = type_info {
                FieldCursor::from(field_cursor_str).set_json_value(
                    &mut object,
                    value.unwrap(),
                    type_info,
                    err,
                    &temp_cursor,
                );
            }
        }
        let mut request: api::ReportStateAndNotificationRequest =
            serde_json::value::from_value(object).unwrap();
        let mut call = self.hub.devices().report_state_and_notification(request);
        for parg in opt
            .values_of("v")
            .map(|i| i.collect())
            .unwrap_or(Vec::new())
            .iter()
        {
            let (key, value) = parse_kv_arg(&*parg, err, false);
            match key {
                _ => {
                    let mut found = false;
                    for param in &self.gp {
                        if key == *param {
                            found = true;
                            call = call.param(
                                self.gpm.iter().find(|t| t.0 == key).unwrap_or(&("", key)).1,
                                value.unwrap_or("unset"),
                            );
                            break;
                        }
                    }
                    if !found {
                        err.issues
                            .push(CLIError::UnknownParameter(key.to_string(), {
                                let mut v = Vec::new();
                                v.extend(self.gp.iter().map(|v| *v));
                                v
                            }));
                    }
                }
            }
        }
        let protocol = CallType::Standard;
        if dry_run {
            Ok(())
        } else {
            assert!(err.issues.len() == 0);
            for scope in self
                .opt
                .values_of("url")
                .map(|i| i.collect())
                .unwrap_or(Vec::new())
                .iter()
            {
                call = call.add_scope(scope);
            }
            let mut ostream = match writer_from_opts(opt.value_of("out")) {
                Ok(mut f) => f,
                Err(io_err) => {
                    return Err(DoitError::IoError(
                        opt.value_of("out").unwrap_or("-").to_string(),
                        io_err,
                    ))
                }
            };
            match match protocol {
                CallType::Standard => call.doit().await,
                _ => unreachable!(),
            } {
                Err(api_err) => Err(DoitError::ApiError(api_err)),
                Ok((mut response, output_schema)) => {
                    let mut value =
                        serde_json::value::to_value(&output_schema).expect("serde to work");
                    remove_json_null_values(&mut value);
                    serde_json::to_writer_pretty(&mut ostream, &value).unwrap();
                    ostream.flush().unwrap();
                    Ok(())
                }
            }
        }
    }

    async fn _devices_request_sync(
        &self,
        opt: &ArgMatches<'n>,
        dry_run: bool,
        err: &mut InvalidOptionsError,
    ) -> Result<(), DoitError> {
        let mut field_cursor = FieldCursor::default();
        let mut object = serde_json::value::Value::Object(Default::default());

        for kvarg in opt
            .values_of("kv")
            .map(|i| i.collect())
            .unwrap_or(Vec::new())
            .iter()
        {
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

            let type_info: Option<(&'static str, JsonTypeInfo)> = match &temp_cursor.to_string()[..]
            {
                "agent-user-id" => Some((
                    "agentUserId",
                    JsonTypeInfo {
                        jtype: JsonType::String,
                        ctype: ComplexType::Pod,
                    },
                )),
                "async" => Some((
                    "async",
                    JsonTypeInfo {
                        jtype: JsonType::Boolean,
                        ctype: ComplexType::Pod,
                    },
                )),
                _ => {
                    let suggestion =
                        FieldCursor::did_you_mean(key, &vec!["agent-user-id", "async"]);
                    err.issues.push(CLIError::Field(FieldError::Unknown(
                        temp_cursor.to_string(),
                        suggestion,
                        value.map(|v| v.to_string()),
                    )));
                    None
                }
            };
            if let Some((field_cursor_str, type_info)) = type_info {
                FieldCursor::from(field_cursor_str).set_json_value(
                    &mut object,
                    value.unwrap(),
                    type_info,
                    err,
                    &temp_cursor,
                );
            }
        }
        let mut request: api::RequestSyncDevicesRequest =
            serde_json::value::from_value(object).unwrap();
        let mut call = self.hub.devices().request_sync(request);
        for parg in opt
            .values_of("v")
            .map(|i| i.collect())
            .unwrap_or(Vec::new())
            .iter()
        {
            let (key, value) = parse_kv_arg(&*parg, err, false);
            match key {
                _ => {
                    let mut found = false;
                    for param in &self.gp {
                        if key == *param {
                            found = true;
                            call = call.param(
                                self.gpm.iter().find(|t| t.0 == key).unwrap_or(&("", key)).1,
                                value.unwrap_or("unset"),
                            );
                            break;
                        }
                    }
                    if !found {
                        err.issues
                            .push(CLIError::UnknownParameter(key.to_string(), {
                                let mut v = Vec::new();
                                v.extend(self.gp.iter().map(|v| *v));
                                v
                            }));
                    }
                }
            }
        }
        let protocol = CallType::Standard;
        if dry_run {
            Ok(())
        } else {
            assert!(err.issues.len() == 0);
            for scope in self
                .opt
                .values_of("url")
                .map(|i| i.collect())
                .unwrap_or(Vec::new())
                .iter()
            {
                call = call.add_scope(scope);
            }
            let mut ostream = match writer_from_opts(opt.value_of("out")) {
                Ok(mut f) => f,
                Err(io_err) => {
                    return Err(DoitError::IoError(
                        opt.value_of("out").unwrap_or("-").to_string(),
                        io_err,
                    ))
                }
            };
            match match protocol {
                CallType::Standard => call.doit().await,
                _ => unreachable!(),
            } {
                Err(api_err) => Err(DoitError::ApiError(api_err)),
                Ok((mut response, output_schema)) => {
                    let mut value =
                        serde_json::value::to_value(&output_schema).expect("serde to work");
                    remove_json_null_values(&mut value);
                    serde_json::to_writer_pretty(&mut ostream, &value).unwrap();
                    ostream.flush().unwrap();
                    Ok(())
                }
            }
        }
    }

    async fn _devices_sync(
        &self,
        opt: &ArgMatches<'n>,
        dry_run: bool,
        err: &mut InvalidOptionsError,
    ) -> Result<(), DoitError> {
        let mut field_cursor = FieldCursor::default();
        let mut object = serde_json::value::Value::Object(Default::default());

        for kvarg in opt
            .values_of("kv")
            .map(|i| i.collect())
            .unwrap_or(Vec::new())
            .iter()
        {
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

            let type_info: Option<(&'static str, JsonTypeInfo)> = match &temp_cursor.to_string()[..]
            {
                "agent-user-id" => Some((
                    "agentUserId",
                    JsonTypeInfo {
                        jtype: JsonType::String,
                        ctype: ComplexType::Pod,
                    },
                )),
                "request-id" => Some((
                    "requestId",
                    JsonTypeInfo {
                        jtype: JsonType::String,
                        ctype: ComplexType::Pod,
                    },
                )),
                _ => {
                    let suggestion =
                        FieldCursor::did_you_mean(key, &vec!["agent-user-id", "request-id"]);
                    err.issues.push(CLIError::Field(FieldError::Unknown(
                        temp_cursor.to_string(),
                        suggestion,
                        value.map(|v| v.to_string()),
                    )));
                    None
                }
            };
            if let Some((field_cursor_str, type_info)) = type_info {
                FieldCursor::from(field_cursor_str).set_json_value(
                    &mut object,
                    value.unwrap(),
                    type_info,
                    err,
                    &temp_cursor,
                );
            }
        }
        let mut request: api::SyncRequest = serde_json::value::from_value(object).unwrap();
        let mut call = self.hub.devices().sync(request);
        for parg in opt
            .values_of("v")
            .map(|i| i.collect())
            .unwrap_or(Vec::new())
            .iter()
        {
            let (key, value) = parse_kv_arg(&*parg, err, false);
            match key {
                _ => {
                    let mut found = false;
                    for param in &self.gp {
                        if key == *param {
                            found = true;
                            call = call.param(
                                self.gpm.iter().find(|t| t.0 == key).unwrap_or(&("", key)).1,
                                value.unwrap_or("unset"),
                            );
                            break;
                        }
                    }
                    if !found {
                        err.issues
                            .push(CLIError::UnknownParameter(key.to_string(), {
                                let mut v = Vec::new();
                                v.extend(self.gp.iter().map(|v| *v));
                                v
                            }));
                    }
                }
            }
        }
        let protocol = CallType::Standard;
        if dry_run {
            Ok(())
        } else {
            assert!(err.issues.len() == 0);
            for scope in self
                .opt
                .values_of("url")
                .map(|i| i.collect())
                .unwrap_or(Vec::new())
                .iter()
            {
                call = call.add_scope(scope);
            }
            let mut ostream = match writer_from_opts(opt.value_of("out")) {
                Ok(mut f) => f,
                Err(io_err) => {
                    return Err(DoitError::IoError(
                        opt.value_of("out").unwrap_or("-").to_string(),
                        io_err,
                    ))
                }
            };
            match match protocol {
                CallType::Standard => call.doit().await,
                _ => unreachable!(),
            } {
                Err(api_err) => Err(DoitError::ApiError(api_err)),
                Ok((mut response, output_schema)) => {
                    let mut value =
                        serde_json::value::to_value(&output_schema).expect("serde to work");
                    remove_json_null_values(&mut value);
                    serde_json::to_writer_pretty(&mut ostream, &value).unwrap();
                    ostream.flush().unwrap();
                    Ok(())
                }
            }
        }
    }

    async fn _doit(
        &self,
        dry_run: bool,
    ) -> Result<Result<(), DoitError>, Option<InvalidOptionsError>> {
        let mut err = InvalidOptionsError::new();
        let mut call_result: Result<(), DoitError> = Ok(());
        let mut err_opt: Option<InvalidOptionsError> = None;
        match self.opt.subcommand() {
            ("agent-users", Some(opt)) => match opt.subcommand() {
                ("delete", Some(opt)) => {
                    call_result = self._agent_users_delete(opt, dry_run, &mut err).await;
                }
                _ => {
                    err.issues
                        .push(CLIError::MissingMethodError("agent-users".to_string()));
                    writeln!(std::io::stderr(), "{}\n", opt.usage()).ok();
                }
            },
            ("devices", Some(opt)) => match opt.subcommand() {
                ("query", Some(opt)) => {
                    call_result = self._devices_query(opt, dry_run, &mut err).await;
                }
                ("report-state-and-notification", Some(opt)) => {
                    call_result = self
                        ._devices_report_state_and_notification(opt, dry_run, &mut err)
                        .await;
                }
                ("request-sync", Some(opt)) => {
                    call_result = self._devices_request_sync(opt, dry_run, &mut err).await;
                }
                ("sync", Some(opt)) => {
                    call_result = self._devices_sync(opt, dry_run, &mut err).await;
                }
                _ => {
                    err.issues
                        .push(CLIError::MissingMethodError("devices".to_string()));
                    writeln!(std::io::stderr(), "{}\n", opt.usage()).ok();
                }
            },
            _ => {
                err.issues.push(CLIError::MissingCommandError);
                writeln!(std::io::stderr(), "{}\n", self.opt.usage()).ok();
            }
        }

        if dry_run {
            if !err.issues.is_empty() {
                err_opt = Some(err);
            }
            Err(err_opt)
        } else {
            Ok(call_result)
        }
    }

    // Please note that this call will fail if any part of the opt can't be handled
    async fn new(opt: ArgMatches<'n>, connector: C) -> Result<Engine<'n, C>, InvalidOptionsError> {
        let (config_dir, secret) = {
            let config_dir = match common::assure_config_dir_exists(
                opt.value_of("folder").unwrap_or("~/.google-service-cli"),
            ) {
                Err(e) => return Err(InvalidOptionsError::single(e, 3)),
                Ok(p) => p,
            };

            match common::application_secret_from_directory(&config_dir, "homegraph1-secret.json",
                                                         "{\"installed\":{\"auth_uri\":\"https://accounts.google.com/o/oauth2/auth\",\"client_secret\":\"hCsslbCUyfehWMmbkG8vTYxG\",\"token_uri\":\"https://accounts.google.com/o/oauth2/token\",\"client_email\":\"\",\"redirect_uris\":[\"urn:ietf:wg:oauth:2.0:oob\",\"oob\"],\"client_x509_cert_url\":\"\",\"client_id\":\"620010449518-9ngf7o4dhs0dka470npqvor6dc5lqb9b.apps.googleusercontent.com\",\"auth_provider_x509_cert_url\":\"https://www.googleapis.com/oauth2/v1/certs\"}}") {
                Ok(secret) => (config_dir, secret),
                Err(e) => return Err(InvalidOptionsError::single(e, 4))
            }
        };

        let executor = hyper_util::rt::TokioExecutor::new();
        let client =
            hyper_util::client::legacy::Client::builder(executor.clone()).build(connector.clone());

        let auth = yup_oauth2::InstalledFlowAuthenticator::with_client(
            secret,
            yup_oauth2::InstalledFlowReturnMethod::HTTPRedirect,
            yup_oauth2::client::CustomHyperClientBuilder::from(
                hyper_util::client::legacy::Client::builder(executor).build(connector),
            ),
        )
        .persist_tokens_to_disk(format!("{}/homegraph1", config_dir))
        .build()
        .await
        .unwrap();

        let engine = Engine {
            opt,
            hub: api::HomeGraphService::new(client, auth),
            gp: vec![
                "$-xgafv",
                "access-token",
                "alt",
                "callback",
                "fields",
                "key",
                "oauth-token",
                "pretty-print",
                "quota-user",
                "upload-type",
                "upload-protocol",
            ],
            gpm: vec![
                ("$-xgafv", "$.xgafv"),
                ("access-token", "access_token"),
                ("oauth-token", "oauth_token"),
                ("pretty-print", "prettyPrint"),
                ("quota-user", "quotaUser"),
                ("upload-type", "uploadType"),
                ("upload-protocol", "upload_protocol"),
            ],
        };

        match engine._doit(true).await {
            Err(Some(err)) => Err(err),
            Err(None) => Ok(engine),
            Ok(_) => unreachable!(),
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
        ("agent-users", "methods: 'delete'", vec![
            ("delete",
                    Some(r##"Unlinks the given third-party user from your smart home Action. All data related to this user will be deleted. For more details on how users link their accounts, see [fulfillment and authentication](https://developers.home.google.com/cloud-to-cloud/primer/fulfillment). The third-party user's identity is passed in via the `agent_user_id` (see DeleteAgentUserRequest). This request must be authorized using service account credentials from your Actions console project."##),
                    "Details at http://byron.github.io/google-apis-rs/google_homegraph1_cli/agent-users_delete",
                  vec![
                    (Some(r##"agent-user-id"##),
                     None,
                     Some(r##"Required. Third-party user ID."##),
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
            ("devices", "methods: 'query', 'report-state-and-notification', 'request-sync' and 'sync'", vec![
            ("query",
                    Some(r##"Gets the current states in Home Graph for the given set of the third-party user's devices. The third-party user's identity is passed in via the `agent_user_id` (see QueryRequest). This request must be authorized using service account credentials from your Actions console project."##),
                    "Details at http://byron.github.io/google-apis-rs/google_homegraph1_cli/devices_query",
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
            ("report-state-and-notification",
                    Some(r##"Reports device state and optionally sends device notifications. Called by your smart home Action when the state of a third-party device changes or you need to send a notification about the device. See [Implement Report State](https://developers.home.google.com/cloud-to-cloud/integration/report-state) for more information. This method updates the device state according to its declared [traits](https://developers.home.google.com/cloud-to-cloud/primer/device-types-and-traits). Publishing a new state value outside of these traits will result in an `INVALID_ARGUMENT` error response. The third-party user's identity is passed in via the `agent_user_id` (see ReportStateAndNotificationRequest). This request must be authorized using service account credentials from your Actions console project."##),
                    "Details at http://byron.github.io/google-apis-rs/google_homegraph1_cli/devices_report-state-and-notification",
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
            ("request-sync",
                    Some(r##"Requests Google to send an `action.devices.SYNC` [intent](https://developers.home.google.com/cloud-to-cloud/intents/sync) to your smart home Action to update device metadata for the given user. The third-party user's identity is passed via the `agent_user_id` (see RequestSyncDevicesRequest). This request must be authorized using service account credentials from your Actions console project."##),
                    "Details at http://byron.github.io/google-apis-rs/google_homegraph1_cli/devices_request-sync",
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
            ("sync",
                    Some(r##"Gets all the devices associated with the given third-party user. The third-party user's identity is passed in via the `agent_user_id` (see SyncRequest). This request must be authorized using service account credentials from your Actions console project."##),
                    "Details at http://byron.github.io/google-apis-rs/google_homegraph1_cli/devices_sync",
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
        ];

    let mut app = App::new("homegraph1")
           .author("Sebastian Thiel <byronimo@gmail.com>")
           .version("7.0.0+20251218")
           .about("")
           .after_help("All documentation details can be found at http://byron.github.io/google-apis-rs/google_homegraph1_cli")
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
                let arg_name_str = match (arg_name, flag) {
                    (&Some(an), _) => an,
                    (_, &Some(f)) => f,
                    _ => unreachable!(),
                };
                let mut arg = Arg::with_name(arg_name_str).empty_values(false);
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
    let connector = hyper_rustls::HttpsConnectorBuilder::new()
        .with_native_roots()
        .unwrap()
        .https_or_http()
        .enable_http2()
        .build();

    match Engine::new(matches, connector).await {
        Err(err) => {
            exit_status = err.exit_code;
            writeln!(std::io::stderr(), "{}", err).ok();
        }
        Ok(engine) => {
            if let Err(doit_err) = engine.doit().await {
                exit_status = 1;
                match doit_err {
                    DoitError::IoError(path, err) => {
                        writeln!(
                            std::io::stderr(),
                            "Failed to open output file '{}': {}",
                            path,
                            err
                        )
                        .ok();
                    }
                    DoitError::ApiError(err) => {
                        if debug {
                            writeln!(std::io::stderr(), "{:#?}", err).ok();
                        } else {
                            writeln!(std::io::stderr(), "{}", err).ok();
                        }
                    }
                }
            }
        }
    }

    std::process::exit(exit_status);
}
