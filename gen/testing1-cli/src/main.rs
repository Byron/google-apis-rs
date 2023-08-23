// DO NOT EDIT !
// This file was generated automatically from 'src/generator/templates/cli/main.rs.mako'
// DO NOT EDIT !
#![allow(unused_variables, unused_imports, dead_code, unused_mut)]

#[macro_use]
extern crate clap;

use std::env;
use std::io::{self, Write};
use clap::{App, SubCommand, Arg};

use google_testing1::{api, Error, oauth2, client::chrono, FieldMask};


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
    hub: api::Testing<S>,
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
    async fn _application_detail_service_get_apk_details(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
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
                    "gcs-path" => Some(("gcsPath", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    _ => {
                        let suggestion = FieldCursor::did_you_mean(key, &vec!["gcs-path"]);
                        err.issues.push(CLIError::Field(FieldError::Unknown(temp_cursor.to_string(), suggestion, value.map(|v| v.to_string()))));
                        None
                    }
                };
            if let Some((field_cursor_str, type_info)) = type_info {
                FieldCursor::from(field_cursor_str).set_json_value(&mut object, value.unwrap(), type_info, err, &temp_cursor);
            }
        }
        let mut request: api::FileReference = json::value::from_value(object).unwrap();
        let mut call = self.hub.application_detail_service().get_apk_details(request);
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

    async fn _projects_test_matrices_cancel(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        let mut call = self.hub.projects().test_matrices_cancel(opt.value_of("project-id").unwrap_or(""), opt.value_of("test-matrix-id").unwrap_or(""));
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

    async fn _projects_test_matrices_create(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
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
                    "client-info.name" => Some(("clientInfo.name", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "environment-matrix.android-matrix.android-model-ids" => Some(("environmentMatrix.androidMatrix.androidModelIds", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Vec })),
                    "environment-matrix.android-matrix.android-version-ids" => Some(("environmentMatrix.androidMatrix.androidVersionIds", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Vec })),
                    "environment-matrix.android-matrix.locales" => Some(("environmentMatrix.androidMatrix.locales", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Vec })),
                    "environment-matrix.android-matrix.orientations" => Some(("environmentMatrix.androidMatrix.orientations", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Vec })),
                    "fail-fast" => Some(("failFast", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "flaky-test-attempts" => Some(("flakyTestAttempts", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "invalid-matrix-details" => Some(("invalidMatrixDetails", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "outcome-summary" => Some(("outcomeSummary", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "project-id" => Some(("projectId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "result-storage.google-cloud-storage.gcs-path" => Some(("resultStorage.googleCloudStorage.gcsPath", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "result-storage.results-url" => Some(("resultStorage.resultsUrl", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "result-storage.tool-results-execution.execution-id" => Some(("resultStorage.toolResultsExecution.executionId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "result-storage.tool-results-execution.history-id" => Some(("resultStorage.toolResultsExecution.historyId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "result-storage.tool-results-execution.project-id" => Some(("resultStorage.toolResultsExecution.projectId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "result-storage.tool-results-history.history-id" => Some(("resultStorage.toolResultsHistory.historyId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "result-storage.tool-results-history.project-id" => Some(("resultStorage.toolResultsHistory.projectId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "state" => Some(("state", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "test-matrix-id" => Some(("testMatrixId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "test-specification.android-instrumentation-test.app-apk.gcs-path" => Some(("testSpecification.androidInstrumentationTest.appApk.gcsPath", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "test-specification.android-instrumentation-test.app-bundle.bundle-location.gcs-path" => Some(("testSpecification.androidInstrumentationTest.appBundle.bundleLocation.gcsPath", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "test-specification.android-instrumentation-test.app-package-id" => Some(("testSpecification.androidInstrumentationTest.appPackageId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "test-specification.android-instrumentation-test.orchestrator-option" => Some(("testSpecification.androidInstrumentationTest.orchestratorOption", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "test-specification.android-instrumentation-test.sharding-option.uniform-sharding.num-shards" => Some(("testSpecification.androidInstrumentationTest.shardingOption.uniformSharding.numShards", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "test-specification.android-instrumentation-test.test-apk.gcs-path" => Some(("testSpecification.androidInstrumentationTest.testApk.gcsPath", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "test-specification.android-instrumentation-test.test-package-id" => Some(("testSpecification.androidInstrumentationTest.testPackageId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "test-specification.android-instrumentation-test.test-runner-class" => Some(("testSpecification.androidInstrumentationTest.testRunnerClass", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "test-specification.android-instrumentation-test.test-targets" => Some(("testSpecification.androidInstrumentationTest.testTargets", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Vec })),
                    "test-specification.android-robo-test.app-apk.gcs-path" => Some(("testSpecification.androidRoboTest.appApk.gcsPath", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "test-specification.android-robo-test.app-bundle.bundle-location.gcs-path" => Some(("testSpecification.androidRoboTest.appBundle.bundleLocation.gcsPath", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "test-specification.android-robo-test.app-initial-activity" => Some(("testSpecification.androidRoboTest.appInitialActivity", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "test-specification.android-robo-test.app-package-id" => Some(("testSpecification.androidRoboTest.appPackageId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "test-specification.android-robo-test.max-depth" => Some(("testSpecification.androidRoboTest.maxDepth", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "test-specification.android-robo-test.max-steps" => Some(("testSpecification.androidRoboTest.maxSteps", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "test-specification.android-robo-test.robo-mode" => Some(("testSpecification.androidRoboTest.roboMode", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "test-specification.android-robo-test.robo-script.gcs-path" => Some(("testSpecification.androidRoboTest.roboScript.gcsPath", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "test-specification.android-test-loop.app-apk.gcs-path" => Some(("testSpecification.androidTestLoop.appApk.gcsPath", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "test-specification.android-test-loop.app-bundle.bundle-location.gcs-path" => Some(("testSpecification.androidTestLoop.appBundle.bundleLocation.gcsPath", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "test-specification.android-test-loop.app-package-id" => Some(("testSpecification.androidTestLoop.appPackageId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "test-specification.android-test-loop.scenario-labels" => Some(("testSpecification.androidTestLoop.scenarioLabels", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Vec })),
                    "test-specification.android-test-loop.scenarios" => Some(("testSpecification.androidTestLoop.scenarios", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Vec })),
                    "test-specification.disable-performance-metrics" => Some(("testSpecification.disablePerformanceMetrics", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "test-specification.disable-video-recording" => Some(("testSpecification.disableVideoRecording", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "test-specification.ios-test-loop.app-bundle-id" => Some(("testSpecification.iosTestLoop.appBundleId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "test-specification.ios-test-loop.app-ipa.gcs-path" => Some(("testSpecification.iosTestLoop.appIpa.gcsPath", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "test-specification.ios-test-loop.scenarios" => Some(("testSpecification.iosTestLoop.scenarios", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Vec })),
                    "test-specification.ios-test-setup.network-profile" => Some(("testSpecification.iosTestSetup.networkProfile", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "test-specification.ios-xc-test.app-bundle-id" => Some(("testSpecification.iosXcTest.appBundleId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "test-specification.ios-xc-test.test-special-entitlements" => Some(("testSpecification.iosXcTest.testSpecialEntitlements", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "test-specification.ios-xc-test.tests-zip.gcs-path" => Some(("testSpecification.iosXcTest.testsZip.gcsPath", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "test-specification.ios-xc-test.xcode-version" => Some(("testSpecification.iosXcTest.xcodeVersion", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "test-specification.ios-xc-test.xctestrun.gcs-path" => Some(("testSpecification.iosXcTest.xctestrun.gcsPath", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "test-specification.test-setup.directories-to-pull" => Some(("testSpecification.testSetup.directoriesToPull", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Vec })),
                    "test-specification.test-setup.dont-autogrant-permissions" => Some(("testSpecification.testSetup.dontAutograntPermissions", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "test-specification.test-setup.network-profile" => Some(("testSpecification.testSetup.networkProfile", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "test-specification.test-setup.systrace.duration-seconds" => Some(("testSpecification.testSetup.systrace.durationSeconds", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "test-specification.test-timeout" => Some(("testSpecification.testTimeout", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "timestamp" => Some(("timestamp", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    _ => {
                        let suggestion = FieldCursor::did_you_mean(key, &vec!["android-instrumentation-test", "android-matrix", "android-model-ids", "android-robo-test", "android-test-loop", "android-version-ids", "app-apk", "app-bundle", "app-bundle-id", "app-initial-activity", "app-ipa", "app-package-id", "bundle-location", "client-info", "directories-to-pull", "disable-performance-metrics", "disable-video-recording", "dont-autogrant-permissions", "duration-seconds", "environment-matrix", "execution-id", "fail-fast", "flaky-test-attempts", "gcs-path", "google-cloud-storage", "history-id", "invalid-matrix-details", "ios-test-loop", "ios-test-setup", "ios-xc-test", "locales", "max-depth", "max-steps", "name", "network-profile", "num-shards", "orchestrator-option", "orientations", "outcome-summary", "project-id", "result-storage", "results-url", "robo-mode", "robo-script", "scenario-labels", "scenarios", "sharding-option", "state", "systrace", "test-apk", "test-matrix-id", "test-package-id", "test-runner-class", "test-setup", "test-special-entitlements", "test-specification", "test-targets", "test-timeout", "tests-zip", "timestamp", "tool-results-execution", "tool-results-history", "uniform-sharding", "xcode-version", "xctestrun"]);
                        err.issues.push(CLIError::Field(FieldError::Unknown(temp_cursor.to_string(), suggestion, value.map(|v| v.to_string()))));
                        None
                    }
                };
            if let Some((field_cursor_str, type_info)) = type_info {
                FieldCursor::from(field_cursor_str).set_json_value(&mut object, value.unwrap(), type_info, err, &temp_cursor);
            }
        }
        let mut request: api::TestMatrix = json::value::from_value(object).unwrap();
        let mut call = self.hub.projects().test_matrices_create(request, opt.value_of("project-id").unwrap_or(""));
        for parg in opt.values_of("v").map(|i|i.collect()).unwrap_or(Vec::new()).iter() {
            let (key, value) = parse_kv_arg(&*parg, err, false);
            match key {
                "request-id" => {
                    call = call.request_id(value.unwrap_or(""));
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
                                                                           v.extend(["request-id"].iter().map(|v|*v));
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

    async fn _projects_test_matrices_get(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        let mut call = self.hub.projects().test_matrices_get(opt.value_of("project-id").unwrap_or(""), opt.value_of("test-matrix-id").unwrap_or(""));
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

    async fn _test_environment_catalog_get(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        let mut call = self.hub.test_environment_catalog().get(opt.value_of("environment-type").unwrap_or(""));
        for parg in opt.values_of("v").map(|i|i.collect()).unwrap_or(Vec::new()).iter() {
            let (key, value) = parse_kv_arg(&*parg, err, false);
            match key {
                "project-id" => {
                    call = call.project_id(value.unwrap_or(""));
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
                                                                           v.extend(["project-id"].iter().map(|v|*v));
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
            ("application-detail-service", Some(opt)) => {
                match opt.subcommand() {
                    ("get-apk-details", Some(opt)) => {
                        call_result = self._application_detail_service_get_apk_details(opt, dry_run, &mut err).await;
                    },
                    _ => {
                        err.issues.push(CLIError::MissingMethodError("application-detail-service".to_string()));
                        writeln!(io::stderr(), "{}\n", opt.usage()).ok();
                    }
                }
            },
            ("projects", Some(opt)) => {
                match opt.subcommand() {
                    ("test-matrices-cancel", Some(opt)) => {
                        call_result = self._projects_test_matrices_cancel(opt, dry_run, &mut err).await;
                    },
                    ("test-matrices-create", Some(opt)) => {
                        call_result = self._projects_test_matrices_create(opt, dry_run, &mut err).await;
                    },
                    ("test-matrices-get", Some(opt)) => {
                        call_result = self._projects_test_matrices_get(opt, dry_run, &mut err).await;
                    },
                    _ => {
                        err.issues.push(CLIError::MissingMethodError("projects".to_string()));
                        writeln!(io::stderr(), "{}\n", opt.usage()).ok();
                    }
                }
            },
            ("test-environment-catalog", Some(opt)) => {
                match opt.subcommand() {
                    ("get", Some(opt)) => {
                        call_result = self._test_environment_catalog_get(opt, dry_run, &mut err).await;
                    },
                    _ => {
                        err.issues.push(CLIError::MissingMethodError("test-environment-catalog".to_string()));
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

            match client::application_secret_from_directory(&config_dir, "testing1-secret.json",
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
        ).persist_tokens_to_disk(format!("{}/testing1", config_dir)).build().await.unwrap();

        let engine = Engine {
            opt: opt,
            hub: api::Testing::new(client, auth),
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
        ("application-detail-service", "methods: 'get-apk-details'", vec![
            ("get-apk-details",
                    Some(r##"Gets the details of an Android application APK."##),
                    "Details at http://byron.github.io/google-apis-rs/google_testing1_cli/application-detail-service_get-apk-details",
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
        
        ("projects", "methods: 'test-matrices-cancel', 'test-matrices-create' and 'test-matrices-get'", vec![
            ("test-matrices-cancel",
                    Some(r##"Cancels unfinished test executions in a test matrix. This call returns immediately and cancellation proceeds asynchronously. If the matrix is already final, this operation will have no effect. May return any of the following canonical error codes: - PERMISSION_DENIED - if the user is not authorized to read project - INVALID_ARGUMENT - if the request is malformed - NOT_FOUND - if the Test Matrix does not exist"##),
                    "Details at http://byron.github.io/google-apis-rs/google_testing1_cli/projects_test-matrices-cancel",
                  vec![
                    (Some(r##"project-id"##),
                     None,
                     Some(r##"Cloud project that owns the test."##),
                     Some(true),
                     Some(false)),
        
                    (Some(r##"test-matrix-id"##),
                     None,
                     Some(r##"Test matrix that will be canceled."##),
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
            ("test-matrices-create",
                    Some(r##"Creates and runs a matrix of tests according to the given specifications. Unsupported environments will be returned in the state UNSUPPORTED. A test matrix is limited to use at most 2000 devices in parallel. May return any of the following canonical error codes: - PERMISSION_DENIED - if the user is not authorized to write to project - INVALID_ARGUMENT - if the request is malformed or if the matrix tries to use too many simultaneous devices."##),
                    "Details at http://byron.github.io/google-apis-rs/google_testing1_cli/projects_test-matrices-create",
                  vec![
                    (Some(r##"project-id"##),
                     None,
                     Some(r##"The GCE project under which this job will run."##),
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
            ("test-matrices-get",
                    Some(r##"Checks the status of a test matrix. May return any of the following canonical error codes: - PERMISSION_DENIED - if the user is not authorized to read project - INVALID_ARGUMENT - if the request is malformed - NOT_FOUND - if the Test Matrix does not exist"##),
                    "Details at http://byron.github.io/google-apis-rs/google_testing1_cli/projects_test-matrices-get",
                  vec![
                    (Some(r##"project-id"##),
                     None,
                     Some(r##"Cloud project that owns the test matrix."##),
                     Some(true),
                     Some(false)),
        
                    (Some(r##"test-matrix-id"##),
                     None,
                     Some(r##"Unique test matrix id which was assigned by the service."##),
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
        
        ("test-environment-catalog", "methods: 'get'", vec![
            ("get",
                    Some(r##"Gets the catalog of supported test environments. May return any of the following canonical error codes: - INVALID_ARGUMENT - if the request is malformed - NOT_FOUND - if the environment type does not exist - INTERNAL - if an internal error occurred"##),
                    "Details at http://byron.github.io/google-apis-rs/google_testing1_cli/test-environment-catalog_get",
                  vec![
                    (Some(r##"environment-type"##),
                     None,
                     Some(r##"Required. The type of environment that should be listed."##),
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
        
    ];
    
    let mut app = App::new("testing1")
           .author("Sebastian Thiel <byronimo@gmail.com>")
           .version("5.0.3+20230119")
           .about("Allows developers to run automated tests for their mobile applications on Google infrastructure.")
           .after_help("All documentation details can be found at http://byron.github.io/google-apis-rs/google_testing1_cli")
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
