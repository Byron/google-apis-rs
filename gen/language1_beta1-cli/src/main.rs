// DO NOT EDIT !
// This file was generated automatically from 'src/generator/templates/cli/main.rs.mako'
// DO NOT EDIT !
#![allow(unused_variables, unused_imports, dead_code, unused_mut)]

#[macro_use]
extern crate clap;

use std::io::Write;

use clap::{App, Arg, SubCommand};

use google_language1_beta1::{api, yup_oauth2, Error};

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
    hub: api::CloudNaturalLanguage<C>,
    gp: Vec<&'static str>,
    gpm: Vec<(&'static str, &'static str)>,
}

impl<'n, C> Engine<'n, C>
where
    C: apis_common::Connector,
{
    async fn _documents_analyze_entities(
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
                "document.content" => Some((
                    "document.content",
                    JsonTypeInfo {
                        jtype: JsonType::String,
                        ctype: ComplexType::Pod,
                    },
                )),
                "document.gcs-content-uri" => Some((
                    "document.gcsContentUri",
                    JsonTypeInfo {
                        jtype: JsonType::String,
                        ctype: ComplexType::Pod,
                    },
                )),
                "document.language" => Some((
                    "document.language",
                    JsonTypeInfo {
                        jtype: JsonType::String,
                        ctype: ComplexType::Pod,
                    },
                )),
                "document.type" => Some((
                    "document.type",
                    JsonTypeInfo {
                        jtype: JsonType::String,
                        ctype: ComplexType::Pod,
                    },
                )),
                "encoding-type" => Some((
                    "encodingType",
                    JsonTypeInfo {
                        jtype: JsonType::String,
                        ctype: ComplexType::Pod,
                    },
                )),
                _ => {
                    let suggestion = FieldCursor::did_you_mean(
                        key,
                        &vec![
                            "content",
                            "document",
                            "encoding-type",
                            "gcs-content-uri",
                            "language",
                            "type",
                        ],
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
        let mut request: api::AnalyzeEntitiesRequest =
            serde_json::value::from_value(object).unwrap();
        let mut call = self.hub.documents().analyze_entities(request);
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

    async fn _documents_analyze_sentiment(
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
                "document.content" => Some((
                    "document.content",
                    JsonTypeInfo {
                        jtype: JsonType::String,
                        ctype: ComplexType::Pod,
                    },
                )),
                "document.gcs-content-uri" => Some((
                    "document.gcsContentUri",
                    JsonTypeInfo {
                        jtype: JsonType::String,
                        ctype: ComplexType::Pod,
                    },
                )),
                "document.language" => Some((
                    "document.language",
                    JsonTypeInfo {
                        jtype: JsonType::String,
                        ctype: ComplexType::Pod,
                    },
                )),
                "document.type" => Some((
                    "document.type",
                    JsonTypeInfo {
                        jtype: JsonType::String,
                        ctype: ComplexType::Pod,
                    },
                )),
                "encoding-type" => Some((
                    "encodingType",
                    JsonTypeInfo {
                        jtype: JsonType::String,
                        ctype: ComplexType::Pod,
                    },
                )),
                _ => {
                    let suggestion = FieldCursor::did_you_mean(
                        key,
                        &vec![
                            "content",
                            "document",
                            "encoding-type",
                            "gcs-content-uri",
                            "language",
                            "type",
                        ],
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
        let mut request: api::AnalyzeSentimentRequest =
            serde_json::value::from_value(object).unwrap();
        let mut call = self.hub.documents().analyze_sentiment(request);
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

    async fn _documents_analyze_syntax(
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
                "document.content" => Some((
                    "document.content",
                    JsonTypeInfo {
                        jtype: JsonType::String,
                        ctype: ComplexType::Pod,
                    },
                )),
                "document.gcs-content-uri" => Some((
                    "document.gcsContentUri",
                    JsonTypeInfo {
                        jtype: JsonType::String,
                        ctype: ComplexType::Pod,
                    },
                )),
                "document.language" => Some((
                    "document.language",
                    JsonTypeInfo {
                        jtype: JsonType::String,
                        ctype: ComplexType::Pod,
                    },
                )),
                "document.type" => Some((
                    "document.type",
                    JsonTypeInfo {
                        jtype: JsonType::String,
                        ctype: ComplexType::Pod,
                    },
                )),
                "encoding-type" => Some((
                    "encodingType",
                    JsonTypeInfo {
                        jtype: JsonType::String,
                        ctype: ComplexType::Pod,
                    },
                )),
                _ => {
                    let suggestion = FieldCursor::did_you_mean(
                        key,
                        &vec![
                            "content",
                            "document",
                            "encoding-type",
                            "gcs-content-uri",
                            "language",
                            "type",
                        ],
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
        let mut request: api::AnalyzeSyntaxRequest = serde_json::value::from_value(object).unwrap();
        let mut call = self.hub.documents().analyze_syntax(request);
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

    async fn _documents_annotate_text(
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
                "document.content" => Some((
                    "document.content",
                    JsonTypeInfo {
                        jtype: JsonType::String,
                        ctype: ComplexType::Pod,
                    },
                )),
                "document.gcs-content-uri" => Some((
                    "document.gcsContentUri",
                    JsonTypeInfo {
                        jtype: JsonType::String,
                        ctype: ComplexType::Pod,
                    },
                )),
                "document.language" => Some((
                    "document.language",
                    JsonTypeInfo {
                        jtype: JsonType::String,
                        ctype: ComplexType::Pod,
                    },
                )),
                "document.type" => Some((
                    "document.type",
                    JsonTypeInfo {
                        jtype: JsonType::String,
                        ctype: ComplexType::Pod,
                    },
                )),
                "encoding-type" => Some((
                    "encodingType",
                    JsonTypeInfo {
                        jtype: JsonType::String,
                        ctype: ComplexType::Pod,
                    },
                )),
                "features.extract-document-sentiment" => Some((
                    "features.extractDocumentSentiment",
                    JsonTypeInfo {
                        jtype: JsonType::Boolean,
                        ctype: ComplexType::Pod,
                    },
                )),
                "features.extract-entities" => Some((
                    "features.extractEntities",
                    JsonTypeInfo {
                        jtype: JsonType::Boolean,
                        ctype: ComplexType::Pod,
                    },
                )),
                "features.extract-syntax" => Some((
                    "features.extractSyntax",
                    JsonTypeInfo {
                        jtype: JsonType::Boolean,
                        ctype: ComplexType::Pod,
                    },
                )),
                _ => {
                    let suggestion = FieldCursor::did_you_mean(
                        key,
                        &vec![
                            "content",
                            "document",
                            "encoding-type",
                            "extract-document-sentiment",
                            "extract-entities",
                            "extract-syntax",
                            "features",
                            "gcs-content-uri",
                            "language",
                            "type",
                        ],
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
        let mut request: api::AnnotateTextRequest = serde_json::value::from_value(object).unwrap();
        let mut call = self.hub.documents().annotate_text(request);
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
            ("documents", Some(opt)) => match opt.subcommand() {
                ("analyze-entities", Some(opt)) => {
                    call_result = self
                        ._documents_analyze_entities(opt, dry_run, &mut err)
                        .await;
                }
                ("analyze-sentiment", Some(opt)) => {
                    call_result = self
                        ._documents_analyze_sentiment(opt, dry_run, &mut err)
                        .await;
                }
                ("analyze-syntax", Some(opt)) => {
                    call_result = self._documents_analyze_syntax(opt, dry_run, &mut err).await;
                }
                ("annotate-text", Some(opt)) => {
                    call_result = self._documents_annotate_text(opt, dry_run, &mut err).await;
                }
                _ => {
                    err.issues
                        .push(CLIError::MissingMethodError("documents".to_string()));
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

            match common::application_secret_from_directory(&config_dir, "language1-beta1-secret.json",
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
            hyper_util::client::legacy::Client::builder(executor).build(connector),
        )
        .persist_tokens_to_disk(format!("{}/language1-beta1", config_dir))
        .build()
        .await
        .unwrap();

        let engine = Engine {
            opt,
            hub: api::CloudNaturalLanguage::new(client, auth),
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
        ("documents", "methods: 'analyze-entities', 'analyze-sentiment', 'analyze-syntax' and 'annotate-text'", vec![
            ("analyze-entities",
                    Some(r##"Finds named entities (currently proper names and common nouns) in the text along with entity types, salience, mentions for each entity, and other properties."##),
                    "Details at http://byron.github.io/google-apis-rs/google_language1_beta1_cli/documents_analyze-entities",
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
            ("analyze-sentiment",
                    Some(r##"Analyzes the sentiment of the provided text."##),
                    "Details at http://byron.github.io/google-apis-rs/google_language1_beta1_cli/documents_analyze-sentiment",
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
            ("analyze-syntax",
                    Some(r##"Analyzes the syntax of the text and provides sentence boundaries and tokenization along with part of speech tags, dependency trees, and other properties."##),
                    "Details at http://byron.github.io/google-apis-rs/google_language1_beta1_cli/documents_analyze-syntax",
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
            ("annotate-text",
                    Some(r##"A convenience method that provides all the features that analyzeSentiment, analyzeEntities, and analyzeSyntax provide in one call."##),
                    "Details at http://byron.github.io/google-apis-rs/google_language1_beta1_cli/documents_annotate-text",
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

    let mut app = App::new("language1-beta1")
           .author("Sebastian Thiel <byronimo@gmail.com>")
           .version("6.0.0+20220218")
           .about("Provides natural language understanding technologies, such as sentiment analysis, entity recognition, entity sentiment analysis, and other text annotations, to developers.")
           .after_help("All documentation details can be found at http://byron.github.io/google-apis-rs/google_language1_beta1_cli")
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
        .enable_http1()
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
