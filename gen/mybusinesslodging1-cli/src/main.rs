// DO NOT EDIT !
// This file was generated automatically from 'src/generator/templates/cli/main.rs.mako'
// DO NOT EDIT !
#![allow(unused_variables, unused_imports, dead_code, unused_mut)]

#[macro_use]
extern crate clap;

use std::env;
use std::io::{self, Write};
use clap::{App, SubCommand, Arg};

use google_mybusinesslodging1::{api, Error, oauth2, client::chrono, FieldMask};


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
    hub: api::MyBusinessLodging<S>,
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
    async fn _locations_get_lodging(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        let mut call = self.hub.locations().get_lodging(opt.value_of("name").unwrap_or(""));
        for parg in opt.values_of("v").map(|i|i.collect()).unwrap_or(Vec::new()).iter() {
            let (key, value) = parse_kv_arg(&*parg, err, false);
            match key {
                "read-mask" => {
                    call = call.read_mask(        value.map(|v| arg_from_str(v, err, "read-mask", "google-fieldmask")).unwrap_or(FieldMask::default()));
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
                                                                           v.extend(["read-mask"].iter().map(|v|*v));
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

    async fn _locations_lodging_get_google_updated(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        let mut call = self.hub.locations().lodging_get_google_updated(opt.value_of("name").unwrap_or(""));
        for parg in opt.values_of("v").map(|i|i.collect()).unwrap_or(Vec::new()).iter() {
            let (key, value) = parse_kv_arg(&*parg, err, false);
            match key {
                "read-mask" => {
                    call = call.read_mask(        value.map(|v| arg_from_str(v, err, "read-mask", "google-fieldmask")).unwrap_or(FieldMask::default()));
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
                                                                           v.extend(["read-mask"].iter().map(|v|*v));
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

    async fn _locations_update_lodging(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
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
                    "accessibility.mobility-accessible" => Some(("accessibility.mobilityAccessible", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "accessibility.mobility-accessible-elevator" => Some(("accessibility.mobilityAccessibleElevator", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "accessibility.mobility-accessible-elevator-exception" => Some(("accessibility.mobilityAccessibleElevatorException", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "accessibility.mobility-accessible-exception" => Some(("accessibility.mobilityAccessibleException", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "accessibility.mobility-accessible-parking" => Some(("accessibility.mobilityAccessibleParking", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "accessibility.mobility-accessible-parking-exception" => Some(("accessibility.mobilityAccessibleParkingException", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "accessibility.mobility-accessible-pool" => Some(("accessibility.mobilityAccessiblePool", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "accessibility.mobility-accessible-pool-exception" => Some(("accessibility.mobilityAccessiblePoolException", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "activities.beach-access" => Some(("activities.beachAccess", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "activities.beach-access-exception" => Some(("activities.beachAccessException", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "activities.beach-front" => Some(("activities.beachFront", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "activities.beach-front-exception" => Some(("activities.beachFrontException", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "activities.bicycle-rental" => Some(("activities.bicycleRental", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "activities.bicycle-rental-exception" => Some(("activities.bicycleRentalException", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "activities.boutique-stores" => Some(("activities.boutiqueStores", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "activities.boutique-stores-exception" => Some(("activities.boutiqueStoresException", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "activities.casino" => Some(("activities.casino", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "activities.casino-exception" => Some(("activities.casinoException", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "activities.free-bicycle-rental" => Some(("activities.freeBicycleRental", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "activities.free-bicycle-rental-exception" => Some(("activities.freeBicycleRentalException", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "activities.free-watercraft-rental" => Some(("activities.freeWatercraftRental", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "activities.free-watercraft-rental-exception" => Some(("activities.freeWatercraftRentalException", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "activities.game-room" => Some(("activities.gameRoom", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "activities.game-room-exception" => Some(("activities.gameRoomException", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "activities.golf" => Some(("activities.golf", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "activities.golf-exception" => Some(("activities.golfException", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "activities.horseback-riding" => Some(("activities.horsebackRiding", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "activities.horseback-riding-exception" => Some(("activities.horsebackRidingException", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "activities.nightclub" => Some(("activities.nightclub", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "activities.nightclub-exception" => Some(("activities.nightclubException", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "activities.private-beach" => Some(("activities.privateBeach", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "activities.private-beach-exception" => Some(("activities.privateBeachException", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "activities.scuba" => Some(("activities.scuba", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "activities.scuba-exception" => Some(("activities.scubaException", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "activities.snorkeling" => Some(("activities.snorkeling", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "activities.snorkeling-exception" => Some(("activities.snorkelingException", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "activities.tennis" => Some(("activities.tennis", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "activities.tennis-exception" => Some(("activities.tennisException", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "activities.water-skiing" => Some(("activities.waterSkiing", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "activities.water-skiing-exception" => Some(("activities.waterSkiingException", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "activities.watercraft-rental" => Some(("activities.watercraftRental", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "activities.watercraft-rental-exception" => Some(("activities.watercraftRentalException", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "all-units.bungalow-or-villa" => Some(("allUnits.bungalowOrVilla", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "all-units.bungalow-or-villa-exception" => Some(("allUnits.bungalowOrVillaException", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "all-units.connecting-unit-available" => Some(("allUnits.connectingUnitAvailable", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "all-units.connecting-unit-available-exception" => Some(("allUnits.connectingUnitAvailableException", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "all-units.executive-floor" => Some(("allUnits.executiveFloor", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "all-units.executive-floor-exception" => Some(("allUnits.executiveFloorException", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "all-units.max-adult-occupants-count" => Some(("allUnits.maxAdultOccupantsCount", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "all-units.max-adult-occupants-count-exception" => Some(("allUnits.maxAdultOccupantsCountException", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "all-units.max-child-occupants-count" => Some(("allUnits.maxChildOccupantsCount", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "all-units.max-child-occupants-count-exception" => Some(("allUnits.maxChildOccupantsCountException", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "all-units.max-occupants-count" => Some(("allUnits.maxOccupantsCount", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "all-units.max-occupants-count-exception" => Some(("allUnits.maxOccupantsCountException", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "all-units.private-home" => Some(("allUnits.privateHome", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "all-units.private-home-exception" => Some(("allUnits.privateHomeException", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "all-units.suite" => Some(("allUnits.suite", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "all-units.suite-exception" => Some(("allUnits.suiteException", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "all-units.tier" => Some(("allUnits.tier", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "all-units.tier-exception" => Some(("allUnits.tierException", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "all-units.total-living-areas.accessibility.ada-compliant-unit" => Some(("allUnits.totalLivingAreas.accessibility.adaCompliantUnit", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "all-units.total-living-areas.accessibility.ada-compliant-unit-exception" => Some(("allUnits.totalLivingAreas.accessibility.adaCompliantUnitException", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "all-units.total-living-areas.accessibility.hearing-accessible-doorbell" => Some(("allUnits.totalLivingAreas.accessibility.hearingAccessibleDoorbell", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "all-units.total-living-areas.accessibility.hearing-accessible-doorbell-exception" => Some(("allUnits.totalLivingAreas.accessibility.hearingAccessibleDoorbellException", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "all-units.total-living-areas.accessibility.hearing-accessible-fire-alarm" => Some(("allUnits.totalLivingAreas.accessibility.hearingAccessibleFireAlarm", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "all-units.total-living-areas.accessibility.hearing-accessible-fire-alarm-exception" => Some(("allUnits.totalLivingAreas.accessibility.hearingAccessibleFireAlarmException", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "all-units.total-living-areas.accessibility.hearing-accessible-unit" => Some(("allUnits.totalLivingAreas.accessibility.hearingAccessibleUnit", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "all-units.total-living-areas.accessibility.hearing-accessible-unit-exception" => Some(("allUnits.totalLivingAreas.accessibility.hearingAccessibleUnitException", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "all-units.total-living-areas.accessibility.mobility-accessible-bathtub" => Some(("allUnits.totalLivingAreas.accessibility.mobilityAccessibleBathtub", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "all-units.total-living-areas.accessibility.mobility-accessible-bathtub-exception" => Some(("allUnits.totalLivingAreas.accessibility.mobilityAccessibleBathtubException", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "all-units.total-living-areas.accessibility.mobility-accessible-shower" => Some(("allUnits.totalLivingAreas.accessibility.mobilityAccessibleShower", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "all-units.total-living-areas.accessibility.mobility-accessible-shower-exception" => Some(("allUnits.totalLivingAreas.accessibility.mobilityAccessibleShowerException", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "all-units.total-living-areas.accessibility.mobility-accessible-toilet" => Some(("allUnits.totalLivingAreas.accessibility.mobilityAccessibleToilet", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "all-units.total-living-areas.accessibility.mobility-accessible-toilet-exception" => Some(("allUnits.totalLivingAreas.accessibility.mobilityAccessibleToiletException", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "all-units.total-living-areas.accessibility.mobility-accessible-unit" => Some(("allUnits.totalLivingAreas.accessibility.mobilityAccessibleUnit", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "all-units.total-living-areas.accessibility.mobility-accessible-unit-exception" => Some(("allUnits.totalLivingAreas.accessibility.mobilityAccessibleUnitException", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "all-units.total-living-areas.eating.coffee-maker" => Some(("allUnits.totalLivingAreas.eating.coffeeMaker", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "all-units.total-living-areas.eating.coffee-maker-exception" => Some(("allUnits.totalLivingAreas.eating.coffeeMakerException", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "all-units.total-living-areas.eating.cookware" => Some(("allUnits.totalLivingAreas.eating.cookware", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "all-units.total-living-areas.eating.cookware-exception" => Some(("allUnits.totalLivingAreas.eating.cookwareException", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "all-units.total-living-areas.eating.dishwasher" => Some(("allUnits.totalLivingAreas.eating.dishwasher", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "all-units.total-living-areas.eating.dishwasher-exception" => Some(("allUnits.totalLivingAreas.eating.dishwasherException", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "all-units.total-living-areas.eating.indoor-grill" => Some(("allUnits.totalLivingAreas.eating.indoorGrill", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "all-units.total-living-areas.eating.indoor-grill-exception" => Some(("allUnits.totalLivingAreas.eating.indoorGrillException", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "all-units.total-living-areas.eating.kettle" => Some(("allUnits.totalLivingAreas.eating.kettle", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "all-units.total-living-areas.eating.kettle-exception" => Some(("allUnits.totalLivingAreas.eating.kettleException", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "all-units.total-living-areas.eating.kitchen-available" => Some(("allUnits.totalLivingAreas.eating.kitchenAvailable", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "all-units.total-living-areas.eating.kitchen-available-exception" => Some(("allUnits.totalLivingAreas.eating.kitchenAvailableException", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "all-units.total-living-areas.eating.microwave" => Some(("allUnits.totalLivingAreas.eating.microwave", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "all-units.total-living-areas.eating.microwave-exception" => Some(("allUnits.totalLivingAreas.eating.microwaveException", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "all-units.total-living-areas.eating.minibar" => Some(("allUnits.totalLivingAreas.eating.minibar", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "all-units.total-living-areas.eating.minibar-exception" => Some(("allUnits.totalLivingAreas.eating.minibarException", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "all-units.total-living-areas.eating.outdoor-grill" => Some(("allUnits.totalLivingAreas.eating.outdoorGrill", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "all-units.total-living-areas.eating.outdoor-grill-exception" => Some(("allUnits.totalLivingAreas.eating.outdoorGrillException", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "all-units.total-living-areas.eating.oven" => Some(("allUnits.totalLivingAreas.eating.oven", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "all-units.total-living-areas.eating.oven-exception" => Some(("allUnits.totalLivingAreas.eating.ovenException", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "all-units.total-living-areas.eating.refrigerator" => Some(("allUnits.totalLivingAreas.eating.refrigerator", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "all-units.total-living-areas.eating.refrigerator-exception" => Some(("allUnits.totalLivingAreas.eating.refrigeratorException", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "all-units.total-living-areas.eating.sink" => Some(("allUnits.totalLivingAreas.eating.sink", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "all-units.total-living-areas.eating.sink-exception" => Some(("allUnits.totalLivingAreas.eating.sinkException", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "all-units.total-living-areas.eating.snackbar" => Some(("allUnits.totalLivingAreas.eating.snackbar", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "all-units.total-living-areas.eating.snackbar-exception" => Some(("allUnits.totalLivingAreas.eating.snackbarException", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "all-units.total-living-areas.eating.stove" => Some(("allUnits.totalLivingAreas.eating.stove", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "all-units.total-living-areas.eating.stove-exception" => Some(("allUnits.totalLivingAreas.eating.stoveException", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "all-units.total-living-areas.eating.tea-station" => Some(("allUnits.totalLivingAreas.eating.teaStation", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "all-units.total-living-areas.eating.tea-station-exception" => Some(("allUnits.totalLivingAreas.eating.teaStationException", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "all-units.total-living-areas.eating.toaster" => Some(("allUnits.totalLivingAreas.eating.toaster", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "all-units.total-living-areas.eating.toaster-exception" => Some(("allUnits.totalLivingAreas.eating.toasterException", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "all-units.total-living-areas.features.air-conditioning" => Some(("allUnits.totalLivingAreas.features.airConditioning", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "all-units.total-living-areas.features.air-conditioning-exception" => Some(("allUnits.totalLivingAreas.features.airConditioningException", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "all-units.total-living-areas.features.bathtub" => Some(("allUnits.totalLivingAreas.features.bathtub", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "all-units.total-living-areas.features.bathtub-exception" => Some(("allUnits.totalLivingAreas.features.bathtubException", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "all-units.total-living-areas.features.bidet" => Some(("allUnits.totalLivingAreas.features.bidet", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "all-units.total-living-areas.features.bidet-exception" => Some(("allUnits.totalLivingAreas.features.bidetException", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "all-units.total-living-areas.features.dryer" => Some(("allUnits.totalLivingAreas.features.dryer", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "all-units.total-living-areas.features.dryer-exception" => Some(("allUnits.totalLivingAreas.features.dryerException", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "all-units.total-living-areas.features.electronic-room-key" => Some(("allUnits.totalLivingAreas.features.electronicRoomKey", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "all-units.total-living-areas.features.electronic-room-key-exception" => Some(("allUnits.totalLivingAreas.features.electronicRoomKeyException", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "all-units.total-living-areas.features.fireplace" => Some(("allUnits.totalLivingAreas.features.fireplace", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "all-units.total-living-areas.features.fireplace-exception" => Some(("allUnits.totalLivingAreas.features.fireplaceException", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "all-units.total-living-areas.features.hairdryer" => Some(("allUnits.totalLivingAreas.features.hairdryer", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "all-units.total-living-areas.features.hairdryer-exception" => Some(("allUnits.totalLivingAreas.features.hairdryerException", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "all-units.total-living-areas.features.heating" => Some(("allUnits.totalLivingAreas.features.heating", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "all-units.total-living-areas.features.heating-exception" => Some(("allUnits.totalLivingAreas.features.heatingException", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "all-units.total-living-areas.features.inunit-safe" => Some(("allUnits.totalLivingAreas.features.inunitSafe", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "all-units.total-living-areas.features.inunit-safe-exception" => Some(("allUnits.totalLivingAreas.features.inunitSafeException", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "all-units.total-living-areas.features.inunit-wifi-available" => Some(("allUnits.totalLivingAreas.features.inunitWifiAvailable", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "all-units.total-living-areas.features.inunit-wifi-available-exception" => Some(("allUnits.totalLivingAreas.features.inunitWifiAvailableException", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "all-units.total-living-areas.features.ironing-equipment" => Some(("allUnits.totalLivingAreas.features.ironingEquipment", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "all-units.total-living-areas.features.ironing-equipment-exception" => Some(("allUnits.totalLivingAreas.features.ironingEquipmentException", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "all-units.total-living-areas.features.pay-per-view-movies" => Some(("allUnits.totalLivingAreas.features.payPerViewMovies", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "all-units.total-living-areas.features.pay-per-view-movies-exception" => Some(("allUnits.totalLivingAreas.features.payPerViewMoviesException", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "all-units.total-living-areas.features.private-bathroom" => Some(("allUnits.totalLivingAreas.features.privateBathroom", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "all-units.total-living-areas.features.private-bathroom-exception" => Some(("allUnits.totalLivingAreas.features.privateBathroomException", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "all-units.total-living-areas.features.shower" => Some(("allUnits.totalLivingAreas.features.shower", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "all-units.total-living-areas.features.shower-exception" => Some(("allUnits.totalLivingAreas.features.showerException", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "all-units.total-living-areas.features.toilet" => Some(("allUnits.totalLivingAreas.features.toilet", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "all-units.total-living-areas.features.toilet-exception" => Some(("allUnits.totalLivingAreas.features.toiletException", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "all-units.total-living-areas.features.tv" => Some(("allUnits.totalLivingAreas.features.tv", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "all-units.total-living-areas.features.tv-casting" => Some(("allUnits.totalLivingAreas.features.tvCasting", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "all-units.total-living-areas.features.tv-casting-exception" => Some(("allUnits.totalLivingAreas.features.tvCastingException", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "all-units.total-living-areas.features.tv-exception" => Some(("allUnits.totalLivingAreas.features.tvException", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "all-units.total-living-areas.features.tv-streaming" => Some(("allUnits.totalLivingAreas.features.tvStreaming", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "all-units.total-living-areas.features.tv-streaming-exception" => Some(("allUnits.totalLivingAreas.features.tvStreamingException", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "all-units.total-living-areas.features.universal-power-adapters" => Some(("allUnits.totalLivingAreas.features.universalPowerAdapters", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "all-units.total-living-areas.features.universal-power-adapters-exception" => Some(("allUnits.totalLivingAreas.features.universalPowerAdaptersException", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "all-units.total-living-areas.features.washer" => Some(("allUnits.totalLivingAreas.features.washer", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "all-units.total-living-areas.features.washer-exception" => Some(("allUnits.totalLivingAreas.features.washerException", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "all-units.total-living-areas.layout.balcony" => Some(("allUnits.totalLivingAreas.layout.balcony", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "all-units.total-living-areas.layout.balcony-exception" => Some(("allUnits.totalLivingAreas.layout.balconyException", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "all-units.total-living-areas.layout.living-area-sq-meters" => Some(("allUnits.totalLivingAreas.layout.livingAreaSqMeters", JsonTypeInfo { jtype: JsonType::Float, ctype: ComplexType::Pod })),
                    "all-units.total-living-areas.layout.living-area-sq-meters-exception" => Some(("allUnits.totalLivingAreas.layout.livingAreaSqMetersException", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "all-units.total-living-areas.layout.loft" => Some(("allUnits.totalLivingAreas.layout.loft", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "all-units.total-living-areas.layout.loft-exception" => Some(("allUnits.totalLivingAreas.layout.loftException", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "all-units.total-living-areas.layout.non-smoking" => Some(("allUnits.totalLivingAreas.layout.nonSmoking", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "all-units.total-living-areas.layout.non-smoking-exception" => Some(("allUnits.totalLivingAreas.layout.nonSmokingException", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "all-units.total-living-areas.layout.patio" => Some(("allUnits.totalLivingAreas.layout.patio", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "all-units.total-living-areas.layout.patio-exception" => Some(("allUnits.totalLivingAreas.layout.patioException", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "all-units.total-living-areas.layout.stairs" => Some(("allUnits.totalLivingAreas.layout.stairs", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "all-units.total-living-areas.layout.stairs-exception" => Some(("allUnits.totalLivingAreas.layout.stairsException", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "all-units.total-living-areas.sleeping.beds-count" => Some(("allUnits.totalLivingAreas.sleeping.bedsCount", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "all-units.total-living-areas.sleeping.beds-count-exception" => Some(("allUnits.totalLivingAreas.sleeping.bedsCountException", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "all-units.total-living-areas.sleeping.bunk-beds-count" => Some(("allUnits.totalLivingAreas.sleeping.bunkBedsCount", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "all-units.total-living-areas.sleeping.bunk-beds-count-exception" => Some(("allUnits.totalLivingAreas.sleeping.bunkBedsCountException", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "all-units.total-living-areas.sleeping.cribs-count" => Some(("allUnits.totalLivingAreas.sleeping.cribsCount", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "all-units.total-living-areas.sleeping.cribs-count-exception" => Some(("allUnits.totalLivingAreas.sleeping.cribsCountException", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "all-units.total-living-areas.sleeping.double-beds-count" => Some(("allUnits.totalLivingAreas.sleeping.doubleBedsCount", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "all-units.total-living-areas.sleeping.double-beds-count-exception" => Some(("allUnits.totalLivingAreas.sleeping.doubleBedsCountException", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "all-units.total-living-areas.sleeping.feather-pillows" => Some(("allUnits.totalLivingAreas.sleeping.featherPillows", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "all-units.total-living-areas.sleeping.feather-pillows-exception" => Some(("allUnits.totalLivingAreas.sleeping.featherPillowsException", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "all-units.total-living-areas.sleeping.hypoallergenic-bedding" => Some(("allUnits.totalLivingAreas.sleeping.hypoallergenicBedding", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "all-units.total-living-areas.sleeping.hypoallergenic-bedding-exception" => Some(("allUnits.totalLivingAreas.sleeping.hypoallergenicBeddingException", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "all-units.total-living-areas.sleeping.king-beds-count" => Some(("allUnits.totalLivingAreas.sleeping.kingBedsCount", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "all-units.total-living-areas.sleeping.king-beds-count-exception" => Some(("allUnits.totalLivingAreas.sleeping.kingBedsCountException", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "all-units.total-living-areas.sleeping.memory-foam-pillows" => Some(("allUnits.totalLivingAreas.sleeping.memoryFoamPillows", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "all-units.total-living-areas.sleeping.memory-foam-pillows-exception" => Some(("allUnits.totalLivingAreas.sleeping.memoryFoamPillowsException", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "all-units.total-living-areas.sleeping.other-beds-count" => Some(("allUnits.totalLivingAreas.sleeping.otherBedsCount", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "all-units.total-living-areas.sleeping.other-beds-count-exception" => Some(("allUnits.totalLivingAreas.sleeping.otherBedsCountException", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "all-units.total-living-areas.sleeping.queen-beds-count" => Some(("allUnits.totalLivingAreas.sleeping.queenBedsCount", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "all-units.total-living-areas.sleeping.queen-beds-count-exception" => Some(("allUnits.totalLivingAreas.sleeping.queenBedsCountException", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "all-units.total-living-areas.sleeping.roll-away-beds-count" => Some(("allUnits.totalLivingAreas.sleeping.rollAwayBedsCount", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "all-units.total-living-areas.sleeping.roll-away-beds-count-exception" => Some(("allUnits.totalLivingAreas.sleeping.rollAwayBedsCountException", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "all-units.total-living-areas.sleeping.single-or-twin-beds-count" => Some(("allUnits.totalLivingAreas.sleeping.singleOrTwinBedsCount", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "all-units.total-living-areas.sleeping.single-or-twin-beds-count-exception" => Some(("allUnits.totalLivingAreas.sleeping.singleOrTwinBedsCountException", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "all-units.total-living-areas.sleeping.sofa-beds-count" => Some(("allUnits.totalLivingAreas.sleeping.sofaBedsCount", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "all-units.total-living-areas.sleeping.sofa-beds-count-exception" => Some(("allUnits.totalLivingAreas.sleeping.sofaBedsCountException", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "all-units.total-living-areas.sleeping.synthetic-pillows" => Some(("allUnits.totalLivingAreas.sleeping.syntheticPillows", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "all-units.total-living-areas.sleeping.synthetic-pillows-exception" => Some(("allUnits.totalLivingAreas.sleeping.syntheticPillowsException", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "all-units.views.beach-view" => Some(("allUnits.views.beachView", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "all-units.views.beach-view-exception" => Some(("allUnits.views.beachViewException", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "all-units.views.city-view" => Some(("allUnits.views.cityView", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "all-units.views.city-view-exception" => Some(("allUnits.views.cityViewException", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "all-units.views.garden-view" => Some(("allUnits.views.gardenView", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "all-units.views.garden-view-exception" => Some(("allUnits.views.gardenViewException", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "all-units.views.lake-view" => Some(("allUnits.views.lakeView", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "all-units.views.lake-view-exception" => Some(("allUnits.views.lakeViewException", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "all-units.views.landmark-view" => Some(("allUnits.views.landmarkView", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "all-units.views.landmark-view-exception" => Some(("allUnits.views.landmarkViewException", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "all-units.views.ocean-view" => Some(("allUnits.views.oceanView", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "all-units.views.ocean-view-exception" => Some(("allUnits.views.oceanViewException", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "all-units.views.pool-view" => Some(("allUnits.views.poolView", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "all-units.views.pool-view-exception" => Some(("allUnits.views.poolViewException", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "all-units.views.valley-view" => Some(("allUnits.views.valleyView", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "all-units.views.valley-view-exception" => Some(("allUnits.views.valleyViewException", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "business.business-center" => Some(("business.businessCenter", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "business.business-center-exception" => Some(("business.businessCenterException", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "business.meeting-rooms" => Some(("business.meetingRooms", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "business.meeting-rooms-count" => Some(("business.meetingRoomsCount", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "business.meeting-rooms-count-exception" => Some(("business.meetingRoomsCountException", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "business.meeting-rooms-exception" => Some(("business.meetingRoomsException", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "common-living-area.accessibility.ada-compliant-unit" => Some(("commonLivingArea.accessibility.adaCompliantUnit", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "common-living-area.accessibility.ada-compliant-unit-exception" => Some(("commonLivingArea.accessibility.adaCompliantUnitException", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "common-living-area.accessibility.hearing-accessible-doorbell" => Some(("commonLivingArea.accessibility.hearingAccessibleDoorbell", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "common-living-area.accessibility.hearing-accessible-doorbell-exception" => Some(("commonLivingArea.accessibility.hearingAccessibleDoorbellException", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "common-living-area.accessibility.hearing-accessible-fire-alarm" => Some(("commonLivingArea.accessibility.hearingAccessibleFireAlarm", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "common-living-area.accessibility.hearing-accessible-fire-alarm-exception" => Some(("commonLivingArea.accessibility.hearingAccessibleFireAlarmException", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "common-living-area.accessibility.hearing-accessible-unit" => Some(("commonLivingArea.accessibility.hearingAccessibleUnit", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "common-living-area.accessibility.hearing-accessible-unit-exception" => Some(("commonLivingArea.accessibility.hearingAccessibleUnitException", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "common-living-area.accessibility.mobility-accessible-bathtub" => Some(("commonLivingArea.accessibility.mobilityAccessibleBathtub", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "common-living-area.accessibility.mobility-accessible-bathtub-exception" => Some(("commonLivingArea.accessibility.mobilityAccessibleBathtubException", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "common-living-area.accessibility.mobility-accessible-shower" => Some(("commonLivingArea.accessibility.mobilityAccessibleShower", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "common-living-area.accessibility.mobility-accessible-shower-exception" => Some(("commonLivingArea.accessibility.mobilityAccessibleShowerException", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "common-living-area.accessibility.mobility-accessible-toilet" => Some(("commonLivingArea.accessibility.mobilityAccessibleToilet", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "common-living-area.accessibility.mobility-accessible-toilet-exception" => Some(("commonLivingArea.accessibility.mobilityAccessibleToiletException", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "common-living-area.accessibility.mobility-accessible-unit" => Some(("commonLivingArea.accessibility.mobilityAccessibleUnit", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "common-living-area.accessibility.mobility-accessible-unit-exception" => Some(("commonLivingArea.accessibility.mobilityAccessibleUnitException", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "common-living-area.eating.coffee-maker" => Some(("commonLivingArea.eating.coffeeMaker", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "common-living-area.eating.coffee-maker-exception" => Some(("commonLivingArea.eating.coffeeMakerException", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "common-living-area.eating.cookware" => Some(("commonLivingArea.eating.cookware", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "common-living-area.eating.cookware-exception" => Some(("commonLivingArea.eating.cookwareException", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "common-living-area.eating.dishwasher" => Some(("commonLivingArea.eating.dishwasher", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "common-living-area.eating.dishwasher-exception" => Some(("commonLivingArea.eating.dishwasherException", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "common-living-area.eating.indoor-grill" => Some(("commonLivingArea.eating.indoorGrill", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "common-living-area.eating.indoor-grill-exception" => Some(("commonLivingArea.eating.indoorGrillException", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "common-living-area.eating.kettle" => Some(("commonLivingArea.eating.kettle", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "common-living-area.eating.kettle-exception" => Some(("commonLivingArea.eating.kettleException", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "common-living-area.eating.kitchen-available" => Some(("commonLivingArea.eating.kitchenAvailable", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "common-living-area.eating.kitchen-available-exception" => Some(("commonLivingArea.eating.kitchenAvailableException", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "common-living-area.eating.microwave" => Some(("commonLivingArea.eating.microwave", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "common-living-area.eating.microwave-exception" => Some(("commonLivingArea.eating.microwaveException", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "common-living-area.eating.minibar" => Some(("commonLivingArea.eating.minibar", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "common-living-area.eating.minibar-exception" => Some(("commonLivingArea.eating.minibarException", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "common-living-area.eating.outdoor-grill" => Some(("commonLivingArea.eating.outdoorGrill", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "common-living-area.eating.outdoor-grill-exception" => Some(("commonLivingArea.eating.outdoorGrillException", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "common-living-area.eating.oven" => Some(("commonLivingArea.eating.oven", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "common-living-area.eating.oven-exception" => Some(("commonLivingArea.eating.ovenException", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "common-living-area.eating.refrigerator" => Some(("commonLivingArea.eating.refrigerator", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "common-living-area.eating.refrigerator-exception" => Some(("commonLivingArea.eating.refrigeratorException", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "common-living-area.eating.sink" => Some(("commonLivingArea.eating.sink", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "common-living-area.eating.sink-exception" => Some(("commonLivingArea.eating.sinkException", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "common-living-area.eating.snackbar" => Some(("commonLivingArea.eating.snackbar", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "common-living-area.eating.snackbar-exception" => Some(("commonLivingArea.eating.snackbarException", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "common-living-area.eating.stove" => Some(("commonLivingArea.eating.stove", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "common-living-area.eating.stove-exception" => Some(("commonLivingArea.eating.stoveException", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "common-living-area.eating.tea-station" => Some(("commonLivingArea.eating.teaStation", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "common-living-area.eating.tea-station-exception" => Some(("commonLivingArea.eating.teaStationException", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "common-living-area.eating.toaster" => Some(("commonLivingArea.eating.toaster", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "common-living-area.eating.toaster-exception" => Some(("commonLivingArea.eating.toasterException", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "common-living-area.features.air-conditioning" => Some(("commonLivingArea.features.airConditioning", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "common-living-area.features.air-conditioning-exception" => Some(("commonLivingArea.features.airConditioningException", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "common-living-area.features.bathtub" => Some(("commonLivingArea.features.bathtub", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "common-living-area.features.bathtub-exception" => Some(("commonLivingArea.features.bathtubException", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "common-living-area.features.bidet" => Some(("commonLivingArea.features.bidet", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "common-living-area.features.bidet-exception" => Some(("commonLivingArea.features.bidetException", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "common-living-area.features.dryer" => Some(("commonLivingArea.features.dryer", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "common-living-area.features.dryer-exception" => Some(("commonLivingArea.features.dryerException", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "common-living-area.features.electronic-room-key" => Some(("commonLivingArea.features.electronicRoomKey", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "common-living-area.features.electronic-room-key-exception" => Some(("commonLivingArea.features.electronicRoomKeyException", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "common-living-area.features.fireplace" => Some(("commonLivingArea.features.fireplace", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "common-living-area.features.fireplace-exception" => Some(("commonLivingArea.features.fireplaceException", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "common-living-area.features.hairdryer" => Some(("commonLivingArea.features.hairdryer", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "common-living-area.features.hairdryer-exception" => Some(("commonLivingArea.features.hairdryerException", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "common-living-area.features.heating" => Some(("commonLivingArea.features.heating", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "common-living-area.features.heating-exception" => Some(("commonLivingArea.features.heatingException", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "common-living-area.features.inunit-safe" => Some(("commonLivingArea.features.inunitSafe", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "common-living-area.features.inunit-safe-exception" => Some(("commonLivingArea.features.inunitSafeException", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "common-living-area.features.inunit-wifi-available" => Some(("commonLivingArea.features.inunitWifiAvailable", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "common-living-area.features.inunit-wifi-available-exception" => Some(("commonLivingArea.features.inunitWifiAvailableException", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "common-living-area.features.ironing-equipment" => Some(("commonLivingArea.features.ironingEquipment", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "common-living-area.features.ironing-equipment-exception" => Some(("commonLivingArea.features.ironingEquipmentException", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "common-living-area.features.pay-per-view-movies" => Some(("commonLivingArea.features.payPerViewMovies", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "common-living-area.features.pay-per-view-movies-exception" => Some(("commonLivingArea.features.payPerViewMoviesException", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "common-living-area.features.private-bathroom" => Some(("commonLivingArea.features.privateBathroom", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "common-living-area.features.private-bathroom-exception" => Some(("commonLivingArea.features.privateBathroomException", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "common-living-area.features.shower" => Some(("commonLivingArea.features.shower", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "common-living-area.features.shower-exception" => Some(("commonLivingArea.features.showerException", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "common-living-area.features.toilet" => Some(("commonLivingArea.features.toilet", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "common-living-area.features.toilet-exception" => Some(("commonLivingArea.features.toiletException", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "common-living-area.features.tv" => Some(("commonLivingArea.features.tv", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "common-living-area.features.tv-casting" => Some(("commonLivingArea.features.tvCasting", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "common-living-area.features.tv-casting-exception" => Some(("commonLivingArea.features.tvCastingException", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "common-living-area.features.tv-exception" => Some(("commonLivingArea.features.tvException", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "common-living-area.features.tv-streaming" => Some(("commonLivingArea.features.tvStreaming", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "common-living-area.features.tv-streaming-exception" => Some(("commonLivingArea.features.tvStreamingException", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "common-living-area.features.universal-power-adapters" => Some(("commonLivingArea.features.universalPowerAdapters", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "common-living-area.features.universal-power-adapters-exception" => Some(("commonLivingArea.features.universalPowerAdaptersException", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "common-living-area.features.washer" => Some(("commonLivingArea.features.washer", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "common-living-area.features.washer-exception" => Some(("commonLivingArea.features.washerException", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "common-living-area.layout.balcony" => Some(("commonLivingArea.layout.balcony", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "common-living-area.layout.balcony-exception" => Some(("commonLivingArea.layout.balconyException", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "common-living-area.layout.living-area-sq-meters" => Some(("commonLivingArea.layout.livingAreaSqMeters", JsonTypeInfo { jtype: JsonType::Float, ctype: ComplexType::Pod })),
                    "common-living-area.layout.living-area-sq-meters-exception" => Some(("commonLivingArea.layout.livingAreaSqMetersException", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "common-living-area.layout.loft" => Some(("commonLivingArea.layout.loft", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "common-living-area.layout.loft-exception" => Some(("commonLivingArea.layout.loftException", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "common-living-area.layout.non-smoking" => Some(("commonLivingArea.layout.nonSmoking", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "common-living-area.layout.non-smoking-exception" => Some(("commonLivingArea.layout.nonSmokingException", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "common-living-area.layout.patio" => Some(("commonLivingArea.layout.patio", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "common-living-area.layout.patio-exception" => Some(("commonLivingArea.layout.patioException", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "common-living-area.layout.stairs" => Some(("commonLivingArea.layout.stairs", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "common-living-area.layout.stairs-exception" => Some(("commonLivingArea.layout.stairsException", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "common-living-area.sleeping.beds-count" => Some(("commonLivingArea.sleeping.bedsCount", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "common-living-area.sleeping.beds-count-exception" => Some(("commonLivingArea.sleeping.bedsCountException", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "common-living-area.sleeping.bunk-beds-count" => Some(("commonLivingArea.sleeping.bunkBedsCount", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "common-living-area.sleeping.bunk-beds-count-exception" => Some(("commonLivingArea.sleeping.bunkBedsCountException", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "common-living-area.sleeping.cribs-count" => Some(("commonLivingArea.sleeping.cribsCount", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "common-living-area.sleeping.cribs-count-exception" => Some(("commonLivingArea.sleeping.cribsCountException", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "common-living-area.sleeping.double-beds-count" => Some(("commonLivingArea.sleeping.doubleBedsCount", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "common-living-area.sleeping.double-beds-count-exception" => Some(("commonLivingArea.sleeping.doubleBedsCountException", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "common-living-area.sleeping.feather-pillows" => Some(("commonLivingArea.sleeping.featherPillows", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "common-living-area.sleeping.feather-pillows-exception" => Some(("commonLivingArea.sleeping.featherPillowsException", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "common-living-area.sleeping.hypoallergenic-bedding" => Some(("commonLivingArea.sleeping.hypoallergenicBedding", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "common-living-area.sleeping.hypoallergenic-bedding-exception" => Some(("commonLivingArea.sleeping.hypoallergenicBeddingException", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "common-living-area.sleeping.king-beds-count" => Some(("commonLivingArea.sleeping.kingBedsCount", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "common-living-area.sleeping.king-beds-count-exception" => Some(("commonLivingArea.sleeping.kingBedsCountException", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "common-living-area.sleeping.memory-foam-pillows" => Some(("commonLivingArea.sleeping.memoryFoamPillows", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "common-living-area.sleeping.memory-foam-pillows-exception" => Some(("commonLivingArea.sleeping.memoryFoamPillowsException", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "common-living-area.sleeping.other-beds-count" => Some(("commonLivingArea.sleeping.otherBedsCount", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "common-living-area.sleeping.other-beds-count-exception" => Some(("commonLivingArea.sleeping.otherBedsCountException", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "common-living-area.sleeping.queen-beds-count" => Some(("commonLivingArea.sleeping.queenBedsCount", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "common-living-area.sleeping.queen-beds-count-exception" => Some(("commonLivingArea.sleeping.queenBedsCountException", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "common-living-area.sleeping.roll-away-beds-count" => Some(("commonLivingArea.sleeping.rollAwayBedsCount", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "common-living-area.sleeping.roll-away-beds-count-exception" => Some(("commonLivingArea.sleeping.rollAwayBedsCountException", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "common-living-area.sleeping.single-or-twin-beds-count" => Some(("commonLivingArea.sleeping.singleOrTwinBedsCount", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "common-living-area.sleeping.single-or-twin-beds-count-exception" => Some(("commonLivingArea.sleeping.singleOrTwinBedsCountException", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "common-living-area.sleeping.sofa-beds-count" => Some(("commonLivingArea.sleeping.sofaBedsCount", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "common-living-area.sleeping.sofa-beds-count-exception" => Some(("commonLivingArea.sleeping.sofaBedsCountException", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "common-living-area.sleeping.synthetic-pillows" => Some(("commonLivingArea.sleeping.syntheticPillows", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "common-living-area.sleeping.synthetic-pillows-exception" => Some(("commonLivingArea.sleeping.syntheticPillowsException", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "connectivity.free-wifi" => Some(("connectivity.freeWifi", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "connectivity.free-wifi-exception" => Some(("connectivity.freeWifiException", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "connectivity.public-area-wifi-available" => Some(("connectivity.publicAreaWifiAvailable", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "connectivity.public-area-wifi-available-exception" => Some(("connectivity.publicAreaWifiAvailableException", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "connectivity.public-internet-terminal" => Some(("connectivity.publicInternetTerminal", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "connectivity.public-internet-terminal-exception" => Some(("connectivity.publicInternetTerminalException", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "connectivity.wifi-available" => Some(("connectivity.wifiAvailable", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "connectivity.wifi-available-exception" => Some(("connectivity.wifiAvailableException", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "families.babysitting" => Some(("families.babysitting", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "families.babysitting-exception" => Some(("families.babysittingException", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "families.kids-activities" => Some(("families.kidsActivities", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "families.kids-activities-exception" => Some(("families.kidsActivitiesException", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "families.kids-club" => Some(("families.kidsClub", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "families.kids-club-exception" => Some(("families.kidsClubException", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "families.kids-friendly" => Some(("families.kidsFriendly", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "families.kids-friendly-exception" => Some(("families.kidsFriendlyException", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "food-and-drink.bar" => Some(("foodAndDrink.bar", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "food-and-drink.bar-exception" => Some(("foodAndDrink.barException", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "food-and-drink.breakfast-available" => Some(("foodAndDrink.breakfastAvailable", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "food-and-drink.breakfast-available-exception" => Some(("foodAndDrink.breakfastAvailableException", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "food-and-drink.breakfast-buffet" => Some(("foodAndDrink.breakfastBuffet", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "food-and-drink.breakfast-buffet-exception" => Some(("foodAndDrink.breakfastBuffetException", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "food-and-drink.buffet" => Some(("foodAndDrink.buffet", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "food-and-drink.buffet-exception" => Some(("foodAndDrink.buffetException", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "food-and-drink.dinner-buffet" => Some(("foodAndDrink.dinnerBuffet", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "food-and-drink.dinner-buffet-exception" => Some(("foodAndDrink.dinnerBuffetException", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "food-and-drink.free-breakfast" => Some(("foodAndDrink.freeBreakfast", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "food-and-drink.free-breakfast-exception" => Some(("foodAndDrink.freeBreakfastException", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "food-and-drink.restaurant" => Some(("foodAndDrink.restaurant", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "food-and-drink.restaurant-exception" => Some(("foodAndDrink.restaurantException", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "food-and-drink.restaurants-count" => Some(("foodAndDrink.restaurantsCount", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "food-and-drink.restaurants-count-exception" => Some(("foodAndDrink.restaurantsCountException", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "food-and-drink.room-service" => Some(("foodAndDrink.roomService", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "food-and-drink.room-service-exception" => Some(("foodAndDrink.roomServiceException", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "food-and-drink.table-service" => Some(("foodAndDrink.tableService", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "food-and-drink.table-service-exception" => Some(("foodAndDrink.tableServiceException", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "food-and-drink.twenty-four-hour-room-service" => Some(("foodAndDrink.twentyFourHourRoomService", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "food-and-drink.twenty-four-hour-room-service-exception" => Some(("foodAndDrink.twentyFourHourRoomServiceException", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "food-and-drink.vending-machine" => Some(("foodAndDrink.vendingMachine", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "food-and-drink.vending-machine-exception" => Some(("foodAndDrink.vendingMachineException", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "health-and-safety.enhanced-cleaning.commercial-grade-disinfectant-cleaning" => Some(("healthAndSafety.enhancedCleaning.commercialGradeDisinfectantCleaning", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "health-and-safety.enhanced-cleaning.commercial-grade-disinfectant-cleaning-exception" => Some(("healthAndSafety.enhancedCleaning.commercialGradeDisinfectantCleaningException", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "health-and-safety.enhanced-cleaning.common-areas-enhanced-cleaning" => Some(("healthAndSafety.enhancedCleaning.commonAreasEnhancedCleaning", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "health-and-safety.enhanced-cleaning.common-areas-enhanced-cleaning-exception" => Some(("healthAndSafety.enhancedCleaning.commonAreasEnhancedCleaningException", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "health-and-safety.enhanced-cleaning.employees-trained-cleaning-procedures" => Some(("healthAndSafety.enhancedCleaning.employeesTrainedCleaningProcedures", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "health-and-safety.enhanced-cleaning.employees-trained-cleaning-procedures-exception" => Some(("healthAndSafety.enhancedCleaning.employeesTrainedCleaningProceduresException", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "health-and-safety.enhanced-cleaning.employees-trained-thorough-hand-washing" => Some(("healthAndSafety.enhancedCleaning.employeesTrainedThoroughHandWashing", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "health-and-safety.enhanced-cleaning.employees-trained-thorough-hand-washing-exception" => Some(("healthAndSafety.enhancedCleaning.employeesTrainedThoroughHandWashingException", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "health-and-safety.enhanced-cleaning.employees-wear-protective-equipment" => Some(("healthAndSafety.enhancedCleaning.employeesWearProtectiveEquipment", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "health-and-safety.enhanced-cleaning.employees-wear-protective-equipment-exception" => Some(("healthAndSafety.enhancedCleaning.employeesWearProtectiveEquipmentException", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "health-and-safety.enhanced-cleaning.guest-rooms-enhanced-cleaning" => Some(("healthAndSafety.enhancedCleaning.guestRoomsEnhancedCleaning", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "health-and-safety.enhanced-cleaning.guest-rooms-enhanced-cleaning-exception" => Some(("healthAndSafety.enhancedCleaning.guestRoomsEnhancedCleaningException", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "health-and-safety.increased-food-safety.dining-areas-additional-sanitation" => Some(("healthAndSafety.increasedFoodSafety.diningAreasAdditionalSanitation", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "health-and-safety.increased-food-safety.dining-areas-additional-sanitation-exception" => Some(("healthAndSafety.increasedFoodSafety.diningAreasAdditionalSanitationException", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "health-and-safety.increased-food-safety.disposable-flatware" => Some(("healthAndSafety.increasedFoodSafety.disposableFlatware", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "health-and-safety.increased-food-safety.disposable-flatware-exception" => Some(("healthAndSafety.increasedFoodSafety.disposableFlatwareException", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "health-and-safety.increased-food-safety.food-preparation-and-serving-additional-safety" => Some(("healthAndSafety.increasedFoodSafety.foodPreparationAndServingAdditionalSafety", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "health-and-safety.increased-food-safety.food-preparation-and-serving-additional-safety-exception" => Some(("healthAndSafety.increasedFoodSafety.foodPreparationAndServingAdditionalSafetyException", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "health-and-safety.increased-food-safety.individual-packaged-meals" => Some(("healthAndSafety.increasedFoodSafety.individualPackagedMeals", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "health-and-safety.increased-food-safety.individual-packaged-meals-exception" => Some(("healthAndSafety.increasedFoodSafety.individualPackagedMealsException", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "health-and-safety.increased-food-safety.single-use-food-menus" => Some(("healthAndSafety.increasedFoodSafety.singleUseFoodMenus", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "health-and-safety.increased-food-safety.single-use-food-menus-exception" => Some(("healthAndSafety.increasedFoodSafety.singleUseFoodMenusException", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "health-and-safety.minimized-contact.contactless-checkin-checkout" => Some(("healthAndSafety.minimizedContact.contactlessCheckinCheckout", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "health-and-safety.minimized-contact.contactless-checkin-checkout-exception" => Some(("healthAndSafety.minimizedContact.contactlessCheckinCheckoutException", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "health-and-safety.minimized-contact.digital-guest-room-keys" => Some(("healthAndSafety.minimizedContact.digitalGuestRoomKeys", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "health-and-safety.minimized-contact.digital-guest-room-keys-exception" => Some(("healthAndSafety.minimizedContact.digitalGuestRoomKeysException", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "health-and-safety.minimized-contact.housekeeping-scheduled-request-only" => Some(("healthAndSafety.minimizedContact.housekeepingScheduledRequestOnly", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "health-and-safety.minimized-contact.housekeeping-scheduled-request-only-exception" => Some(("healthAndSafety.minimizedContact.housekeepingScheduledRequestOnlyException", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "health-and-safety.minimized-contact.no-high-touch-items-common-areas" => Some(("healthAndSafety.minimizedContact.noHighTouchItemsCommonAreas", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "health-and-safety.minimized-contact.no-high-touch-items-common-areas-exception" => Some(("healthAndSafety.minimizedContact.noHighTouchItemsCommonAreasException", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "health-and-safety.minimized-contact.no-high-touch-items-guest-rooms" => Some(("healthAndSafety.minimizedContact.noHighTouchItemsGuestRooms", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "health-and-safety.minimized-contact.no-high-touch-items-guest-rooms-exception" => Some(("healthAndSafety.minimizedContact.noHighTouchItemsGuestRoomsException", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "health-and-safety.minimized-contact.plastic-keycards-disinfected" => Some(("healthAndSafety.minimizedContact.plasticKeycardsDisinfected", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "health-and-safety.minimized-contact.plastic-keycards-disinfected-exception" => Some(("healthAndSafety.minimizedContact.plasticKeycardsDisinfectedException", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "health-and-safety.minimized-contact.room-bookings-buffer" => Some(("healthAndSafety.minimizedContact.roomBookingsBuffer", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "health-and-safety.minimized-contact.room-bookings-buffer-exception" => Some(("healthAndSafety.minimizedContact.roomBookingsBufferException", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "health-and-safety.personal-protection.common-areas-offer-sanitizing-items" => Some(("healthAndSafety.personalProtection.commonAreasOfferSanitizingItems", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "health-and-safety.personal-protection.common-areas-offer-sanitizing-items-exception" => Some(("healthAndSafety.personalProtection.commonAreasOfferSanitizingItemsException", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "health-and-safety.personal-protection.face-mask-required" => Some(("healthAndSafety.personalProtection.faceMaskRequired", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "health-and-safety.personal-protection.face-mask-required-exception" => Some(("healthAndSafety.personalProtection.faceMaskRequiredException", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "health-and-safety.personal-protection.guest-room-hygiene-kits-available" => Some(("healthAndSafety.personalProtection.guestRoomHygieneKitsAvailable", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "health-and-safety.personal-protection.guest-room-hygiene-kits-available-exception" => Some(("healthAndSafety.personalProtection.guestRoomHygieneKitsAvailableException", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "health-and-safety.personal-protection.protective-equipment-available" => Some(("healthAndSafety.personalProtection.protectiveEquipmentAvailable", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "health-and-safety.personal-protection.protective-equipment-available-exception" => Some(("healthAndSafety.personalProtection.protectiveEquipmentAvailableException", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "health-and-safety.physical-distancing.common-areas-physical-distancing-arranged" => Some(("healthAndSafety.physicalDistancing.commonAreasPhysicalDistancingArranged", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "health-and-safety.physical-distancing.common-areas-physical-distancing-arranged-exception" => Some(("healthAndSafety.physicalDistancing.commonAreasPhysicalDistancingArrangedException", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "health-and-safety.physical-distancing.physical-distancing-required" => Some(("healthAndSafety.physicalDistancing.physicalDistancingRequired", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "health-and-safety.physical-distancing.physical-distancing-required-exception" => Some(("healthAndSafety.physicalDistancing.physicalDistancingRequiredException", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "health-and-safety.physical-distancing.safety-dividers" => Some(("healthAndSafety.physicalDistancing.safetyDividers", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "health-and-safety.physical-distancing.safety-dividers-exception" => Some(("healthAndSafety.physicalDistancing.safetyDividersException", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "health-and-safety.physical-distancing.shared-areas-limited-occupancy" => Some(("healthAndSafety.physicalDistancing.sharedAreasLimitedOccupancy", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "health-and-safety.physical-distancing.shared-areas-limited-occupancy-exception" => Some(("healthAndSafety.physicalDistancing.sharedAreasLimitedOccupancyException", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "health-and-safety.physical-distancing.wellness-areas-have-private-spaces" => Some(("healthAndSafety.physicalDistancing.wellnessAreasHavePrivateSpaces", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "health-and-safety.physical-distancing.wellness-areas-have-private-spaces-exception" => Some(("healthAndSafety.physicalDistancing.wellnessAreasHavePrivateSpacesException", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "housekeeping.daily-housekeeping" => Some(("housekeeping.dailyHousekeeping", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "housekeeping.daily-housekeeping-exception" => Some(("housekeeping.dailyHousekeepingException", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "housekeeping.housekeeping-available" => Some(("housekeeping.housekeepingAvailable", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "housekeeping.housekeeping-available-exception" => Some(("housekeeping.housekeepingAvailableException", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "housekeeping.turndown-service" => Some(("housekeeping.turndownService", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "housekeeping.turndown-service-exception" => Some(("housekeeping.turndownServiceException", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "metadata.update-time" => Some(("metadata.updateTime", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "name" => Some(("name", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "parking.electric-car-charging-stations" => Some(("parking.electricCarChargingStations", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "parking.electric-car-charging-stations-exception" => Some(("parking.electricCarChargingStationsException", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "parking.free-parking" => Some(("parking.freeParking", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "parking.free-parking-exception" => Some(("parking.freeParkingException", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "parking.free-self-parking" => Some(("parking.freeSelfParking", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "parking.free-self-parking-exception" => Some(("parking.freeSelfParkingException", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "parking.free-valet-parking" => Some(("parking.freeValetParking", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "parking.free-valet-parking-exception" => Some(("parking.freeValetParkingException", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "parking.parking-available" => Some(("parking.parkingAvailable", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "parking.parking-available-exception" => Some(("parking.parkingAvailableException", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "parking.self-parking-available" => Some(("parking.selfParkingAvailable", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "parking.self-parking-available-exception" => Some(("parking.selfParkingAvailableException", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "parking.valet-parking-available" => Some(("parking.valetParkingAvailable", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "parking.valet-parking-available-exception" => Some(("parking.valetParkingAvailableException", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "pets.cats-allowed" => Some(("pets.catsAllowed", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "pets.cats-allowed-exception" => Some(("pets.catsAllowedException", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "pets.dogs-allowed" => Some(("pets.dogsAllowed", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "pets.dogs-allowed-exception" => Some(("pets.dogsAllowedException", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "pets.pets-allowed" => Some(("pets.petsAllowed", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "pets.pets-allowed-exception" => Some(("pets.petsAllowedException", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "pets.pets-allowed-free" => Some(("pets.petsAllowedFree", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "pets.pets-allowed-free-exception" => Some(("pets.petsAllowedFreeException", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "policies.all-inclusive-available" => Some(("policies.allInclusiveAvailable", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "policies.all-inclusive-available-exception" => Some(("policies.allInclusiveAvailableException", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "policies.all-inclusive-only" => Some(("policies.allInclusiveOnly", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "policies.all-inclusive-only-exception" => Some(("policies.allInclusiveOnlyException", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "policies.checkin-time.hours" => Some(("policies.checkinTime.hours", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "policies.checkin-time.minutes" => Some(("policies.checkinTime.minutes", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "policies.checkin-time.nanos" => Some(("policies.checkinTime.nanos", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "policies.checkin-time.seconds" => Some(("policies.checkinTime.seconds", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "policies.checkin-time-exception" => Some(("policies.checkinTimeException", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "policies.checkout-time.hours" => Some(("policies.checkoutTime.hours", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "policies.checkout-time.minutes" => Some(("policies.checkoutTime.minutes", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "policies.checkout-time.nanos" => Some(("policies.checkoutTime.nanos", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "policies.checkout-time.seconds" => Some(("policies.checkoutTime.seconds", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "policies.checkout-time-exception" => Some(("policies.checkoutTimeException", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "policies.kids-stay-free" => Some(("policies.kidsStayFree", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "policies.kids-stay-free-exception" => Some(("policies.kidsStayFreeException", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "policies.max-child-age" => Some(("policies.maxChildAge", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "policies.max-child-age-exception" => Some(("policies.maxChildAgeException", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "policies.max-kids-stay-free-count" => Some(("policies.maxKidsStayFreeCount", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "policies.max-kids-stay-free-count-exception" => Some(("policies.maxKidsStayFreeCountException", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "policies.payment-options.cash" => Some(("policies.paymentOptions.cash", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "policies.payment-options.cash-exception" => Some(("policies.paymentOptions.cashException", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "policies.payment-options.cheque" => Some(("policies.paymentOptions.cheque", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "policies.payment-options.cheque-exception" => Some(("policies.paymentOptions.chequeException", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "policies.payment-options.credit-card" => Some(("policies.paymentOptions.creditCard", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "policies.payment-options.credit-card-exception" => Some(("policies.paymentOptions.creditCardException", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "policies.payment-options.debit-card" => Some(("policies.paymentOptions.debitCard", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "policies.payment-options.debit-card-exception" => Some(("policies.paymentOptions.debitCardException", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "policies.payment-options.mobile-nfc" => Some(("policies.paymentOptions.mobileNfc", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "policies.payment-options.mobile-nfc-exception" => Some(("policies.paymentOptions.mobileNfcException", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "policies.smoke-free-property" => Some(("policies.smokeFreeProperty", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "policies.smoke-free-property-exception" => Some(("policies.smokeFreePropertyException", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "pools.adult-pool" => Some(("pools.adultPool", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "pools.adult-pool-exception" => Some(("pools.adultPoolException", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "pools.hot-tub" => Some(("pools.hotTub", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "pools.hot-tub-exception" => Some(("pools.hotTubException", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "pools.indoor-pool" => Some(("pools.indoorPool", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "pools.indoor-pool-exception" => Some(("pools.indoorPoolException", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "pools.indoor-pools-count" => Some(("pools.indoorPoolsCount", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "pools.indoor-pools-count-exception" => Some(("pools.indoorPoolsCountException", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "pools.lazy-river" => Some(("pools.lazyRiver", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "pools.lazy-river-exception" => Some(("pools.lazyRiverException", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "pools.lifeguard" => Some(("pools.lifeguard", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "pools.lifeguard-exception" => Some(("pools.lifeguardException", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "pools.outdoor-pool" => Some(("pools.outdoorPool", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "pools.outdoor-pool-exception" => Some(("pools.outdoorPoolException", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "pools.outdoor-pools-count" => Some(("pools.outdoorPoolsCount", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "pools.outdoor-pools-count-exception" => Some(("pools.outdoorPoolsCountException", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "pools.pool" => Some(("pools.pool", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "pools.pool-exception" => Some(("pools.poolException", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "pools.pools-count" => Some(("pools.poolsCount", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "pools.pools-count-exception" => Some(("pools.poolsCountException", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "pools.wading-pool" => Some(("pools.wadingPool", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "pools.wading-pool-exception" => Some(("pools.wadingPoolException", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "pools.water-park" => Some(("pools.waterPark", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "pools.water-park-exception" => Some(("pools.waterParkException", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "pools.waterslide" => Some(("pools.waterslide", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "pools.waterslide-exception" => Some(("pools.waterslideException", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "pools.wave-pool" => Some(("pools.wavePool", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "pools.wave-pool-exception" => Some(("pools.wavePoolException", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "property.built-year" => Some(("property.builtYear", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "property.built-year-exception" => Some(("property.builtYearException", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "property.floors-count" => Some(("property.floorsCount", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "property.floors-count-exception" => Some(("property.floorsCountException", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "property.last-renovated-year" => Some(("property.lastRenovatedYear", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "property.last-renovated-year-exception" => Some(("property.lastRenovatedYearException", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "property.rooms-count" => Some(("property.roomsCount", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "property.rooms-count-exception" => Some(("property.roomsCountException", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "services.baggage-storage" => Some(("services.baggageStorage", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "services.baggage-storage-exception" => Some(("services.baggageStorageException", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "services.concierge" => Some(("services.concierge", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "services.concierge-exception" => Some(("services.conciergeException", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "services.convenience-store" => Some(("services.convenienceStore", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "services.convenience-store-exception" => Some(("services.convenienceStoreException", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "services.currency-exchange" => Some(("services.currencyExchange", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "services.currency-exchange-exception" => Some(("services.currencyExchangeException", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "services.elevator" => Some(("services.elevator", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "services.elevator-exception" => Some(("services.elevatorException", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "services.front-desk" => Some(("services.frontDesk", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "services.front-desk-exception" => Some(("services.frontDeskException", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "services.full-service-laundry" => Some(("services.fullServiceLaundry", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "services.full-service-laundry-exception" => Some(("services.fullServiceLaundryException", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "services.gift-shop" => Some(("services.giftShop", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "services.gift-shop-exception" => Some(("services.giftShopException", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "services.self-service-laundry" => Some(("services.selfServiceLaundry", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "services.self-service-laundry-exception" => Some(("services.selfServiceLaundryException", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "services.social-hour" => Some(("services.socialHour", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "services.social-hour-exception" => Some(("services.socialHourException", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "services.twenty-four-hour-front-desk" => Some(("services.twentyFourHourFrontDesk", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "services.twenty-four-hour-front-desk-exception" => Some(("services.twentyFourHourFrontDeskException", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "services.wake-up-calls" => Some(("services.wakeUpCalls", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "services.wake-up-calls-exception" => Some(("services.wakeUpCallsException", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "some-units.bungalow-or-villa" => Some(("someUnits.bungalowOrVilla", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "some-units.bungalow-or-villa-exception" => Some(("someUnits.bungalowOrVillaException", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "some-units.connecting-unit-available" => Some(("someUnits.connectingUnitAvailable", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "some-units.connecting-unit-available-exception" => Some(("someUnits.connectingUnitAvailableException", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "some-units.executive-floor" => Some(("someUnits.executiveFloor", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "some-units.executive-floor-exception" => Some(("someUnits.executiveFloorException", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "some-units.max-adult-occupants-count" => Some(("someUnits.maxAdultOccupantsCount", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "some-units.max-adult-occupants-count-exception" => Some(("someUnits.maxAdultOccupantsCountException", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "some-units.max-child-occupants-count" => Some(("someUnits.maxChildOccupantsCount", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "some-units.max-child-occupants-count-exception" => Some(("someUnits.maxChildOccupantsCountException", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "some-units.max-occupants-count" => Some(("someUnits.maxOccupantsCount", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "some-units.max-occupants-count-exception" => Some(("someUnits.maxOccupantsCountException", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "some-units.private-home" => Some(("someUnits.privateHome", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "some-units.private-home-exception" => Some(("someUnits.privateHomeException", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "some-units.suite" => Some(("someUnits.suite", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "some-units.suite-exception" => Some(("someUnits.suiteException", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "some-units.tier" => Some(("someUnits.tier", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "some-units.tier-exception" => Some(("someUnits.tierException", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "some-units.total-living-areas.accessibility.ada-compliant-unit" => Some(("someUnits.totalLivingAreas.accessibility.adaCompliantUnit", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "some-units.total-living-areas.accessibility.ada-compliant-unit-exception" => Some(("someUnits.totalLivingAreas.accessibility.adaCompliantUnitException", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "some-units.total-living-areas.accessibility.hearing-accessible-doorbell" => Some(("someUnits.totalLivingAreas.accessibility.hearingAccessibleDoorbell", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "some-units.total-living-areas.accessibility.hearing-accessible-doorbell-exception" => Some(("someUnits.totalLivingAreas.accessibility.hearingAccessibleDoorbellException", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "some-units.total-living-areas.accessibility.hearing-accessible-fire-alarm" => Some(("someUnits.totalLivingAreas.accessibility.hearingAccessibleFireAlarm", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "some-units.total-living-areas.accessibility.hearing-accessible-fire-alarm-exception" => Some(("someUnits.totalLivingAreas.accessibility.hearingAccessibleFireAlarmException", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "some-units.total-living-areas.accessibility.hearing-accessible-unit" => Some(("someUnits.totalLivingAreas.accessibility.hearingAccessibleUnit", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "some-units.total-living-areas.accessibility.hearing-accessible-unit-exception" => Some(("someUnits.totalLivingAreas.accessibility.hearingAccessibleUnitException", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "some-units.total-living-areas.accessibility.mobility-accessible-bathtub" => Some(("someUnits.totalLivingAreas.accessibility.mobilityAccessibleBathtub", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "some-units.total-living-areas.accessibility.mobility-accessible-bathtub-exception" => Some(("someUnits.totalLivingAreas.accessibility.mobilityAccessibleBathtubException", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "some-units.total-living-areas.accessibility.mobility-accessible-shower" => Some(("someUnits.totalLivingAreas.accessibility.mobilityAccessibleShower", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "some-units.total-living-areas.accessibility.mobility-accessible-shower-exception" => Some(("someUnits.totalLivingAreas.accessibility.mobilityAccessibleShowerException", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "some-units.total-living-areas.accessibility.mobility-accessible-toilet" => Some(("someUnits.totalLivingAreas.accessibility.mobilityAccessibleToilet", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "some-units.total-living-areas.accessibility.mobility-accessible-toilet-exception" => Some(("someUnits.totalLivingAreas.accessibility.mobilityAccessibleToiletException", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "some-units.total-living-areas.accessibility.mobility-accessible-unit" => Some(("someUnits.totalLivingAreas.accessibility.mobilityAccessibleUnit", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "some-units.total-living-areas.accessibility.mobility-accessible-unit-exception" => Some(("someUnits.totalLivingAreas.accessibility.mobilityAccessibleUnitException", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "some-units.total-living-areas.eating.coffee-maker" => Some(("someUnits.totalLivingAreas.eating.coffeeMaker", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "some-units.total-living-areas.eating.coffee-maker-exception" => Some(("someUnits.totalLivingAreas.eating.coffeeMakerException", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "some-units.total-living-areas.eating.cookware" => Some(("someUnits.totalLivingAreas.eating.cookware", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "some-units.total-living-areas.eating.cookware-exception" => Some(("someUnits.totalLivingAreas.eating.cookwareException", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "some-units.total-living-areas.eating.dishwasher" => Some(("someUnits.totalLivingAreas.eating.dishwasher", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "some-units.total-living-areas.eating.dishwasher-exception" => Some(("someUnits.totalLivingAreas.eating.dishwasherException", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "some-units.total-living-areas.eating.indoor-grill" => Some(("someUnits.totalLivingAreas.eating.indoorGrill", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "some-units.total-living-areas.eating.indoor-grill-exception" => Some(("someUnits.totalLivingAreas.eating.indoorGrillException", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "some-units.total-living-areas.eating.kettle" => Some(("someUnits.totalLivingAreas.eating.kettle", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "some-units.total-living-areas.eating.kettle-exception" => Some(("someUnits.totalLivingAreas.eating.kettleException", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "some-units.total-living-areas.eating.kitchen-available" => Some(("someUnits.totalLivingAreas.eating.kitchenAvailable", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "some-units.total-living-areas.eating.kitchen-available-exception" => Some(("someUnits.totalLivingAreas.eating.kitchenAvailableException", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "some-units.total-living-areas.eating.microwave" => Some(("someUnits.totalLivingAreas.eating.microwave", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "some-units.total-living-areas.eating.microwave-exception" => Some(("someUnits.totalLivingAreas.eating.microwaveException", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "some-units.total-living-areas.eating.minibar" => Some(("someUnits.totalLivingAreas.eating.minibar", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "some-units.total-living-areas.eating.minibar-exception" => Some(("someUnits.totalLivingAreas.eating.minibarException", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "some-units.total-living-areas.eating.outdoor-grill" => Some(("someUnits.totalLivingAreas.eating.outdoorGrill", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "some-units.total-living-areas.eating.outdoor-grill-exception" => Some(("someUnits.totalLivingAreas.eating.outdoorGrillException", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "some-units.total-living-areas.eating.oven" => Some(("someUnits.totalLivingAreas.eating.oven", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "some-units.total-living-areas.eating.oven-exception" => Some(("someUnits.totalLivingAreas.eating.ovenException", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "some-units.total-living-areas.eating.refrigerator" => Some(("someUnits.totalLivingAreas.eating.refrigerator", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "some-units.total-living-areas.eating.refrigerator-exception" => Some(("someUnits.totalLivingAreas.eating.refrigeratorException", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "some-units.total-living-areas.eating.sink" => Some(("someUnits.totalLivingAreas.eating.sink", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "some-units.total-living-areas.eating.sink-exception" => Some(("someUnits.totalLivingAreas.eating.sinkException", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "some-units.total-living-areas.eating.snackbar" => Some(("someUnits.totalLivingAreas.eating.snackbar", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "some-units.total-living-areas.eating.snackbar-exception" => Some(("someUnits.totalLivingAreas.eating.snackbarException", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "some-units.total-living-areas.eating.stove" => Some(("someUnits.totalLivingAreas.eating.stove", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "some-units.total-living-areas.eating.stove-exception" => Some(("someUnits.totalLivingAreas.eating.stoveException", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "some-units.total-living-areas.eating.tea-station" => Some(("someUnits.totalLivingAreas.eating.teaStation", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "some-units.total-living-areas.eating.tea-station-exception" => Some(("someUnits.totalLivingAreas.eating.teaStationException", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "some-units.total-living-areas.eating.toaster" => Some(("someUnits.totalLivingAreas.eating.toaster", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "some-units.total-living-areas.eating.toaster-exception" => Some(("someUnits.totalLivingAreas.eating.toasterException", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "some-units.total-living-areas.features.air-conditioning" => Some(("someUnits.totalLivingAreas.features.airConditioning", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "some-units.total-living-areas.features.air-conditioning-exception" => Some(("someUnits.totalLivingAreas.features.airConditioningException", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "some-units.total-living-areas.features.bathtub" => Some(("someUnits.totalLivingAreas.features.bathtub", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "some-units.total-living-areas.features.bathtub-exception" => Some(("someUnits.totalLivingAreas.features.bathtubException", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "some-units.total-living-areas.features.bidet" => Some(("someUnits.totalLivingAreas.features.bidet", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "some-units.total-living-areas.features.bidet-exception" => Some(("someUnits.totalLivingAreas.features.bidetException", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "some-units.total-living-areas.features.dryer" => Some(("someUnits.totalLivingAreas.features.dryer", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "some-units.total-living-areas.features.dryer-exception" => Some(("someUnits.totalLivingAreas.features.dryerException", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "some-units.total-living-areas.features.electronic-room-key" => Some(("someUnits.totalLivingAreas.features.electronicRoomKey", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "some-units.total-living-areas.features.electronic-room-key-exception" => Some(("someUnits.totalLivingAreas.features.electronicRoomKeyException", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "some-units.total-living-areas.features.fireplace" => Some(("someUnits.totalLivingAreas.features.fireplace", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "some-units.total-living-areas.features.fireplace-exception" => Some(("someUnits.totalLivingAreas.features.fireplaceException", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "some-units.total-living-areas.features.hairdryer" => Some(("someUnits.totalLivingAreas.features.hairdryer", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "some-units.total-living-areas.features.hairdryer-exception" => Some(("someUnits.totalLivingAreas.features.hairdryerException", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "some-units.total-living-areas.features.heating" => Some(("someUnits.totalLivingAreas.features.heating", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "some-units.total-living-areas.features.heating-exception" => Some(("someUnits.totalLivingAreas.features.heatingException", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "some-units.total-living-areas.features.inunit-safe" => Some(("someUnits.totalLivingAreas.features.inunitSafe", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "some-units.total-living-areas.features.inunit-safe-exception" => Some(("someUnits.totalLivingAreas.features.inunitSafeException", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "some-units.total-living-areas.features.inunit-wifi-available" => Some(("someUnits.totalLivingAreas.features.inunitWifiAvailable", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "some-units.total-living-areas.features.inunit-wifi-available-exception" => Some(("someUnits.totalLivingAreas.features.inunitWifiAvailableException", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "some-units.total-living-areas.features.ironing-equipment" => Some(("someUnits.totalLivingAreas.features.ironingEquipment", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "some-units.total-living-areas.features.ironing-equipment-exception" => Some(("someUnits.totalLivingAreas.features.ironingEquipmentException", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "some-units.total-living-areas.features.pay-per-view-movies" => Some(("someUnits.totalLivingAreas.features.payPerViewMovies", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "some-units.total-living-areas.features.pay-per-view-movies-exception" => Some(("someUnits.totalLivingAreas.features.payPerViewMoviesException", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "some-units.total-living-areas.features.private-bathroom" => Some(("someUnits.totalLivingAreas.features.privateBathroom", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "some-units.total-living-areas.features.private-bathroom-exception" => Some(("someUnits.totalLivingAreas.features.privateBathroomException", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "some-units.total-living-areas.features.shower" => Some(("someUnits.totalLivingAreas.features.shower", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "some-units.total-living-areas.features.shower-exception" => Some(("someUnits.totalLivingAreas.features.showerException", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "some-units.total-living-areas.features.toilet" => Some(("someUnits.totalLivingAreas.features.toilet", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "some-units.total-living-areas.features.toilet-exception" => Some(("someUnits.totalLivingAreas.features.toiletException", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "some-units.total-living-areas.features.tv" => Some(("someUnits.totalLivingAreas.features.tv", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "some-units.total-living-areas.features.tv-casting" => Some(("someUnits.totalLivingAreas.features.tvCasting", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "some-units.total-living-areas.features.tv-casting-exception" => Some(("someUnits.totalLivingAreas.features.tvCastingException", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "some-units.total-living-areas.features.tv-exception" => Some(("someUnits.totalLivingAreas.features.tvException", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "some-units.total-living-areas.features.tv-streaming" => Some(("someUnits.totalLivingAreas.features.tvStreaming", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "some-units.total-living-areas.features.tv-streaming-exception" => Some(("someUnits.totalLivingAreas.features.tvStreamingException", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "some-units.total-living-areas.features.universal-power-adapters" => Some(("someUnits.totalLivingAreas.features.universalPowerAdapters", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "some-units.total-living-areas.features.universal-power-adapters-exception" => Some(("someUnits.totalLivingAreas.features.universalPowerAdaptersException", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "some-units.total-living-areas.features.washer" => Some(("someUnits.totalLivingAreas.features.washer", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "some-units.total-living-areas.features.washer-exception" => Some(("someUnits.totalLivingAreas.features.washerException", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "some-units.total-living-areas.layout.balcony" => Some(("someUnits.totalLivingAreas.layout.balcony", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "some-units.total-living-areas.layout.balcony-exception" => Some(("someUnits.totalLivingAreas.layout.balconyException", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "some-units.total-living-areas.layout.living-area-sq-meters" => Some(("someUnits.totalLivingAreas.layout.livingAreaSqMeters", JsonTypeInfo { jtype: JsonType::Float, ctype: ComplexType::Pod })),
                    "some-units.total-living-areas.layout.living-area-sq-meters-exception" => Some(("someUnits.totalLivingAreas.layout.livingAreaSqMetersException", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "some-units.total-living-areas.layout.loft" => Some(("someUnits.totalLivingAreas.layout.loft", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "some-units.total-living-areas.layout.loft-exception" => Some(("someUnits.totalLivingAreas.layout.loftException", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "some-units.total-living-areas.layout.non-smoking" => Some(("someUnits.totalLivingAreas.layout.nonSmoking", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "some-units.total-living-areas.layout.non-smoking-exception" => Some(("someUnits.totalLivingAreas.layout.nonSmokingException", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "some-units.total-living-areas.layout.patio" => Some(("someUnits.totalLivingAreas.layout.patio", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "some-units.total-living-areas.layout.patio-exception" => Some(("someUnits.totalLivingAreas.layout.patioException", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "some-units.total-living-areas.layout.stairs" => Some(("someUnits.totalLivingAreas.layout.stairs", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "some-units.total-living-areas.layout.stairs-exception" => Some(("someUnits.totalLivingAreas.layout.stairsException", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "some-units.total-living-areas.sleeping.beds-count" => Some(("someUnits.totalLivingAreas.sleeping.bedsCount", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "some-units.total-living-areas.sleeping.beds-count-exception" => Some(("someUnits.totalLivingAreas.sleeping.bedsCountException", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "some-units.total-living-areas.sleeping.bunk-beds-count" => Some(("someUnits.totalLivingAreas.sleeping.bunkBedsCount", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "some-units.total-living-areas.sleeping.bunk-beds-count-exception" => Some(("someUnits.totalLivingAreas.sleeping.bunkBedsCountException", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "some-units.total-living-areas.sleeping.cribs-count" => Some(("someUnits.totalLivingAreas.sleeping.cribsCount", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "some-units.total-living-areas.sleeping.cribs-count-exception" => Some(("someUnits.totalLivingAreas.sleeping.cribsCountException", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "some-units.total-living-areas.sleeping.double-beds-count" => Some(("someUnits.totalLivingAreas.sleeping.doubleBedsCount", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "some-units.total-living-areas.sleeping.double-beds-count-exception" => Some(("someUnits.totalLivingAreas.sleeping.doubleBedsCountException", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "some-units.total-living-areas.sleeping.feather-pillows" => Some(("someUnits.totalLivingAreas.sleeping.featherPillows", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "some-units.total-living-areas.sleeping.feather-pillows-exception" => Some(("someUnits.totalLivingAreas.sleeping.featherPillowsException", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "some-units.total-living-areas.sleeping.hypoallergenic-bedding" => Some(("someUnits.totalLivingAreas.sleeping.hypoallergenicBedding", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "some-units.total-living-areas.sleeping.hypoallergenic-bedding-exception" => Some(("someUnits.totalLivingAreas.sleeping.hypoallergenicBeddingException", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "some-units.total-living-areas.sleeping.king-beds-count" => Some(("someUnits.totalLivingAreas.sleeping.kingBedsCount", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "some-units.total-living-areas.sleeping.king-beds-count-exception" => Some(("someUnits.totalLivingAreas.sleeping.kingBedsCountException", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "some-units.total-living-areas.sleeping.memory-foam-pillows" => Some(("someUnits.totalLivingAreas.sleeping.memoryFoamPillows", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "some-units.total-living-areas.sleeping.memory-foam-pillows-exception" => Some(("someUnits.totalLivingAreas.sleeping.memoryFoamPillowsException", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "some-units.total-living-areas.sleeping.other-beds-count" => Some(("someUnits.totalLivingAreas.sleeping.otherBedsCount", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "some-units.total-living-areas.sleeping.other-beds-count-exception" => Some(("someUnits.totalLivingAreas.sleeping.otherBedsCountException", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "some-units.total-living-areas.sleeping.queen-beds-count" => Some(("someUnits.totalLivingAreas.sleeping.queenBedsCount", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "some-units.total-living-areas.sleeping.queen-beds-count-exception" => Some(("someUnits.totalLivingAreas.sleeping.queenBedsCountException", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "some-units.total-living-areas.sleeping.roll-away-beds-count" => Some(("someUnits.totalLivingAreas.sleeping.rollAwayBedsCount", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "some-units.total-living-areas.sleeping.roll-away-beds-count-exception" => Some(("someUnits.totalLivingAreas.sleeping.rollAwayBedsCountException", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "some-units.total-living-areas.sleeping.single-or-twin-beds-count" => Some(("someUnits.totalLivingAreas.sleeping.singleOrTwinBedsCount", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "some-units.total-living-areas.sleeping.single-or-twin-beds-count-exception" => Some(("someUnits.totalLivingAreas.sleeping.singleOrTwinBedsCountException", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "some-units.total-living-areas.sleeping.sofa-beds-count" => Some(("someUnits.totalLivingAreas.sleeping.sofaBedsCount", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "some-units.total-living-areas.sleeping.sofa-beds-count-exception" => Some(("someUnits.totalLivingAreas.sleeping.sofaBedsCountException", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "some-units.total-living-areas.sleeping.synthetic-pillows" => Some(("someUnits.totalLivingAreas.sleeping.syntheticPillows", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "some-units.total-living-areas.sleeping.synthetic-pillows-exception" => Some(("someUnits.totalLivingAreas.sleeping.syntheticPillowsException", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "some-units.views.beach-view" => Some(("someUnits.views.beachView", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "some-units.views.beach-view-exception" => Some(("someUnits.views.beachViewException", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "some-units.views.city-view" => Some(("someUnits.views.cityView", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "some-units.views.city-view-exception" => Some(("someUnits.views.cityViewException", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "some-units.views.garden-view" => Some(("someUnits.views.gardenView", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "some-units.views.garden-view-exception" => Some(("someUnits.views.gardenViewException", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "some-units.views.lake-view" => Some(("someUnits.views.lakeView", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "some-units.views.lake-view-exception" => Some(("someUnits.views.lakeViewException", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "some-units.views.landmark-view" => Some(("someUnits.views.landmarkView", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "some-units.views.landmark-view-exception" => Some(("someUnits.views.landmarkViewException", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "some-units.views.ocean-view" => Some(("someUnits.views.oceanView", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "some-units.views.ocean-view-exception" => Some(("someUnits.views.oceanViewException", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "some-units.views.pool-view" => Some(("someUnits.views.poolView", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "some-units.views.pool-view-exception" => Some(("someUnits.views.poolViewException", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "some-units.views.valley-view" => Some(("someUnits.views.valleyView", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "some-units.views.valley-view-exception" => Some(("someUnits.views.valleyViewException", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "sustainability.energy-efficiency.carbon-free-energy-sources" => Some(("sustainability.energyEfficiency.carbonFreeEnergySources", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "sustainability.energy-efficiency.carbon-free-energy-sources-exception" => Some(("sustainability.energyEfficiency.carbonFreeEnergySourcesException", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "sustainability.energy-efficiency.energy-conservation-program" => Some(("sustainability.energyEfficiency.energyConservationProgram", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "sustainability.energy-efficiency.energy-conservation-program-exception" => Some(("sustainability.energyEfficiency.energyConservationProgramException", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "sustainability.energy-efficiency.energy-efficient-heating-and-cooling-systems" => Some(("sustainability.energyEfficiency.energyEfficientHeatingAndCoolingSystems", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "sustainability.energy-efficiency.energy-efficient-heating-and-cooling-systems-exception" => Some(("sustainability.energyEfficiency.energyEfficientHeatingAndCoolingSystemsException", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "sustainability.energy-efficiency.energy-efficient-lighting" => Some(("sustainability.energyEfficiency.energyEfficientLighting", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "sustainability.energy-efficiency.energy-efficient-lighting-exception" => Some(("sustainability.energyEfficiency.energyEfficientLightingException", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "sustainability.energy-efficiency.energy-saving-thermostats" => Some(("sustainability.energyEfficiency.energySavingThermostats", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "sustainability.energy-efficiency.energy-saving-thermostats-exception" => Some(("sustainability.energyEfficiency.energySavingThermostatsException", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "sustainability.energy-efficiency.green-building-design" => Some(("sustainability.energyEfficiency.greenBuildingDesign", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "sustainability.energy-efficiency.green-building-design-exception" => Some(("sustainability.energyEfficiency.greenBuildingDesignException", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "sustainability.energy-efficiency.independent-organization-audits-energy-use" => Some(("sustainability.energyEfficiency.independentOrganizationAuditsEnergyUse", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "sustainability.energy-efficiency.independent-organization-audits-energy-use-exception" => Some(("sustainability.energyEfficiency.independentOrganizationAuditsEnergyUseException", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "sustainability.sustainability-certifications.breeam-certification" => Some(("sustainability.sustainabilityCertifications.breeamCertification", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "sustainability.sustainability-certifications.breeam-certification-exception" => Some(("sustainability.sustainabilityCertifications.breeamCertificationException", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "sustainability.sustainability-certifications.leed-certification" => Some(("sustainability.sustainabilityCertifications.leedCertification", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "sustainability.sustainability-certifications.leed-certification-exception" => Some(("sustainability.sustainabilityCertifications.leedCertificationException", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "sustainability.sustainable-sourcing.eco-friendly-toiletries" => Some(("sustainability.sustainableSourcing.ecoFriendlyToiletries", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "sustainability.sustainable-sourcing.eco-friendly-toiletries-exception" => Some(("sustainability.sustainableSourcing.ecoFriendlyToiletriesException", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "sustainability.sustainable-sourcing.locally-sourced-food-and-beverages" => Some(("sustainability.sustainableSourcing.locallySourcedFoodAndBeverages", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "sustainability.sustainable-sourcing.locally-sourced-food-and-beverages-exception" => Some(("sustainability.sustainableSourcing.locallySourcedFoodAndBeveragesException", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "sustainability.sustainable-sourcing.organic-cage-free-eggs" => Some(("sustainability.sustainableSourcing.organicCageFreeEggs", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "sustainability.sustainable-sourcing.organic-cage-free-eggs-exception" => Some(("sustainability.sustainableSourcing.organicCageFreeEggsException", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "sustainability.sustainable-sourcing.organic-food-and-beverages" => Some(("sustainability.sustainableSourcing.organicFoodAndBeverages", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "sustainability.sustainable-sourcing.organic-food-and-beverages-exception" => Some(("sustainability.sustainableSourcing.organicFoodAndBeveragesException", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "sustainability.sustainable-sourcing.responsible-purchasing-policy" => Some(("sustainability.sustainableSourcing.responsiblePurchasingPolicy", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "sustainability.sustainable-sourcing.responsible-purchasing-policy-exception" => Some(("sustainability.sustainableSourcing.responsiblePurchasingPolicyException", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "sustainability.sustainable-sourcing.responsibly-sources-seafood" => Some(("sustainability.sustainableSourcing.responsiblySourcesSeafood", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "sustainability.sustainable-sourcing.responsibly-sources-seafood-exception" => Some(("sustainability.sustainableSourcing.responsiblySourcesSeafoodException", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "sustainability.sustainable-sourcing.vegan-meals" => Some(("sustainability.sustainableSourcing.veganMeals", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "sustainability.sustainable-sourcing.vegan-meals-exception" => Some(("sustainability.sustainableSourcing.veganMealsException", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "sustainability.sustainable-sourcing.vegetarian-meals" => Some(("sustainability.sustainableSourcing.vegetarianMeals", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "sustainability.sustainable-sourcing.vegetarian-meals-exception" => Some(("sustainability.sustainableSourcing.vegetarianMealsException", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "sustainability.waste-reduction.compostable-food-containers-and-cutlery" => Some(("sustainability.wasteReduction.compostableFoodContainersAndCutlery", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "sustainability.waste-reduction.compostable-food-containers-and-cutlery-exception" => Some(("sustainability.wasteReduction.compostableFoodContainersAndCutleryException", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "sustainability.waste-reduction.composts-excess-food" => Some(("sustainability.wasteReduction.compostsExcessFood", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "sustainability.waste-reduction.composts-excess-food-exception" => Some(("sustainability.wasteReduction.compostsExcessFoodException", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "sustainability.waste-reduction.donates-excess-food" => Some(("sustainability.wasteReduction.donatesExcessFood", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "sustainability.waste-reduction.donates-excess-food-exception" => Some(("sustainability.wasteReduction.donatesExcessFoodException", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "sustainability.waste-reduction.food-waste-reduction-program" => Some(("sustainability.wasteReduction.foodWasteReductionProgram", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "sustainability.waste-reduction.food-waste-reduction-program-exception" => Some(("sustainability.wasteReduction.foodWasteReductionProgramException", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "sustainability.waste-reduction.no-single-use-plastic-straws" => Some(("sustainability.wasteReduction.noSingleUsePlasticStraws", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "sustainability.waste-reduction.no-single-use-plastic-straws-exception" => Some(("sustainability.wasteReduction.noSingleUsePlasticStrawsException", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "sustainability.waste-reduction.no-single-use-plastic-water-bottles" => Some(("sustainability.wasteReduction.noSingleUsePlasticWaterBottles", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "sustainability.waste-reduction.no-single-use-plastic-water-bottles-exception" => Some(("sustainability.wasteReduction.noSingleUsePlasticWaterBottlesException", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "sustainability.waste-reduction.no-styrofoam-food-containers" => Some(("sustainability.wasteReduction.noStyrofoamFoodContainers", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "sustainability.waste-reduction.no-styrofoam-food-containers-exception" => Some(("sustainability.wasteReduction.noStyrofoamFoodContainersException", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "sustainability.waste-reduction.recycling-program" => Some(("sustainability.wasteReduction.recyclingProgram", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "sustainability.waste-reduction.recycling-program-exception" => Some(("sustainability.wasteReduction.recyclingProgramException", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "sustainability.waste-reduction.refillable-toiletry-containers" => Some(("sustainability.wasteReduction.refillableToiletryContainers", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "sustainability.waste-reduction.refillable-toiletry-containers-exception" => Some(("sustainability.wasteReduction.refillableToiletryContainersException", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "sustainability.waste-reduction.safely-disposes-batteries" => Some(("sustainability.wasteReduction.safelyDisposesBatteries", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "sustainability.waste-reduction.safely-disposes-batteries-exception" => Some(("sustainability.wasteReduction.safelyDisposesBatteriesException", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "sustainability.waste-reduction.safely-disposes-electronics" => Some(("sustainability.wasteReduction.safelyDisposesElectronics", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "sustainability.waste-reduction.safely-disposes-electronics-exception" => Some(("sustainability.wasteReduction.safelyDisposesElectronicsException", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "sustainability.waste-reduction.safely-disposes-lightbulbs" => Some(("sustainability.wasteReduction.safelyDisposesLightbulbs", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "sustainability.waste-reduction.safely-disposes-lightbulbs-exception" => Some(("sustainability.wasteReduction.safelyDisposesLightbulbsException", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "sustainability.waste-reduction.safely-handles-hazardous-substances" => Some(("sustainability.wasteReduction.safelyHandlesHazardousSubstances", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "sustainability.waste-reduction.safely-handles-hazardous-substances-exception" => Some(("sustainability.wasteReduction.safelyHandlesHazardousSubstancesException", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "sustainability.waste-reduction.soap-donation-program" => Some(("sustainability.wasteReduction.soapDonationProgram", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "sustainability.waste-reduction.soap-donation-program-exception" => Some(("sustainability.wasteReduction.soapDonationProgramException", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "sustainability.waste-reduction.toiletry-donation-program" => Some(("sustainability.wasteReduction.toiletryDonationProgram", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "sustainability.waste-reduction.toiletry-donation-program-exception" => Some(("sustainability.wasteReduction.toiletryDonationProgramException", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "sustainability.waste-reduction.water-bottle-filling-stations" => Some(("sustainability.wasteReduction.waterBottleFillingStations", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "sustainability.waste-reduction.water-bottle-filling-stations-exception" => Some(("sustainability.wasteReduction.waterBottleFillingStationsException", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "sustainability.water-conservation.independent-organization-audits-water-use" => Some(("sustainability.waterConservation.independentOrganizationAuditsWaterUse", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "sustainability.water-conservation.independent-organization-audits-water-use-exception" => Some(("sustainability.waterConservation.independentOrganizationAuditsWaterUseException", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "sustainability.water-conservation.linen-reuse-program" => Some(("sustainability.waterConservation.linenReuseProgram", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "sustainability.water-conservation.linen-reuse-program-exception" => Some(("sustainability.waterConservation.linenReuseProgramException", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "sustainability.water-conservation.towel-reuse-program" => Some(("sustainability.waterConservation.towelReuseProgram", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "sustainability.water-conservation.towel-reuse-program-exception" => Some(("sustainability.waterConservation.towelReuseProgramException", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "sustainability.water-conservation.water-saving-showers" => Some(("sustainability.waterConservation.waterSavingShowers", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "sustainability.water-conservation.water-saving-showers-exception" => Some(("sustainability.waterConservation.waterSavingShowersException", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "sustainability.water-conservation.water-saving-sinks" => Some(("sustainability.waterConservation.waterSavingSinks", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "sustainability.water-conservation.water-saving-sinks-exception" => Some(("sustainability.waterConservation.waterSavingSinksException", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "sustainability.water-conservation.water-saving-toilets" => Some(("sustainability.waterConservation.waterSavingToilets", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "sustainability.water-conservation.water-saving-toilets-exception" => Some(("sustainability.waterConservation.waterSavingToiletsException", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "transportation.airport-shuttle" => Some(("transportation.airportShuttle", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "transportation.airport-shuttle-exception" => Some(("transportation.airportShuttleException", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "transportation.car-rental-on-property" => Some(("transportation.carRentalOnProperty", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "transportation.car-rental-on-property-exception" => Some(("transportation.carRentalOnPropertyException", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "transportation.free-airport-shuttle" => Some(("transportation.freeAirportShuttle", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "transportation.free-airport-shuttle-exception" => Some(("transportation.freeAirportShuttleException", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "transportation.free-private-car-service" => Some(("transportation.freePrivateCarService", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "transportation.free-private-car-service-exception" => Some(("transportation.freePrivateCarServiceException", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "transportation.local-shuttle" => Some(("transportation.localShuttle", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "transportation.local-shuttle-exception" => Some(("transportation.localShuttleException", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "transportation.private-car-service" => Some(("transportation.privateCarService", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "transportation.private-car-service-exception" => Some(("transportation.privateCarServiceException", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "transportation.transfer" => Some(("transportation.transfer", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "transportation.transfer-exception" => Some(("transportation.transferException", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "wellness.doctor-on-call" => Some(("wellness.doctorOnCall", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "wellness.doctor-on-call-exception" => Some(("wellness.doctorOnCallException", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "wellness.elliptical-machine" => Some(("wellness.ellipticalMachine", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "wellness.elliptical-machine-exception" => Some(("wellness.ellipticalMachineException", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "wellness.fitness-center" => Some(("wellness.fitnessCenter", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "wellness.fitness-center-exception" => Some(("wellness.fitnessCenterException", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "wellness.free-fitness-center" => Some(("wellness.freeFitnessCenter", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "wellness.free-fitness-center-exception" => Some(("wellness.freeFitnessCenterException", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "wellness.free-weights" => Some(("wellness.freeWeights", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "wellness.free-weights-exception" => Some(("wellness.freeWeightsException", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "wellness.massage" => Some(("wellness.massage", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "wellness.massage-exception" => Some(("wellness.massageException", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "wellness.salon" => Some(("wellness.salon", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "wellness.salon-exception" => Some(("wellness.salonException", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "wellness.sauna" => Some(("wellness.sauna", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "wellness.sauna-exception" => Some(("wellness.saunaException", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "wellness.spa" => Some(("wellness.spa", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "wellness.spa-exception" => Some(("wellness.spaException", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "wellness.treadmill" => Some(("wellness.treadmill", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "wellness.treadmill-exception" => Some(("wellness.treadmillException", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "wellness.weight-machine" => Some(("wellness.weightMachine", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "wellness.weight-machine-exception" => Some(("wellness.weightMachineException", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    _ => {
                        let suggestion = FieldCursor::did_you_mean(key, &vec!["accessibility", "activities", "ada-compliant-unit", "ada-compliant-unit-exception", "adult-pool", "adult-pool-exception", "air-conditioning", "air-conditioning-exception", "airport-shuttle", "airport-shuttle-exception", "all-inclusive-available", "all-inclusive-available-exception", "all-inclusive-only", "all-inclusive-only-exception", "all-units", "babysitting", "babysitting-exception", "baggage-storage", "baggage-storage-exception", "balcony", "balcony-exception", "bar", "bar-exception", "bathtub", "bathtub-exception", "beach-access", "beach-access-exception", "beach-front", "beach-front-exception", "beach-view", "beach-view-exception", "beds-count", "beds-count-exception", "bicycle-rental", "bicycle-rental-exception", "bidet", "bidet-exception", "boutique-stores", "boutique-stores-exception", "breakfast-available", "breakfast-available-exception", "breakfast-buffet", "breakfast-buffet-exception", "breeam-certification", "breeam-certification-exception", "buffet", "buffet-exception", "built-year", "built-year-exception", "bungalow-or-villa", "bungalow-or-villa-exception", "bunk-beds-count", "bunk-beds-count-exception", "business", "business-center", "business-center-exception", "car-rental-on-property", "car-rental-on-property-exception", "carbon-free-energy-sources", "carbon-free-energy-sources-exception", "cash", "cash-exception", "casino", "casino-exception", "cats-allowed", "cats-allowed-exception", "checkin-time", "checkin-time-exception", "checkout-time", "checkout-time-exception", "cheque", "cheque-exception", "city-view", "city-view-exception", "coffee-maker", "coffee-maker-exception", "commercial-grade-disinfectant-cleaning", "commercial-grade-disinfectant-cleaning-exception", "common-areas-enhanced-cleaning", "common-areas-enhanced-cleaning-exception", "common-areas-offer-sanitizing-items", "common-areas-offer-sanitizing-items-exception", "common-areas-physical-distancing-arranged", "common-areas-physical-distancing-arranged-exception", "common-living-area", "compostable-food-containers-and-cutlery", "compostable-food-containers-and-cutlery-exception", "composts-excess-food", "composts-excess-food-exception", "concierge", "concierge-exception", "connecting-unit-available", "connecting-unit-available-exception", "connectivity", "contactless-checkin-checkout", "contactless-checkin-checkout-exception", "convenience-store", "convenience-store-exception", "cookware", "cookware-exception", "credit-card", "credit-card-exception", "cribs-count", "cribs-count-exception", "currency-exchange", "currency-exchange-exception", "daily-housekeeping", "daily-housekeeping-exception", "debit-card", "debit-card-exception", "digital-guest-room-keys", "digital-guest-room-keys-exception", "dining-areas-additional-sanitation", "dining-areas-additional-sanitation-exception", "dinner-buffet", "dinner-buffet-exception", "dishwasher", "dishwasher-exception", "disposable-flatware", "disposable-flatware-exception", "doctor-on-call", "doctor-on-call-exception", "dogs-allowed", "dogs-allowed-exception", "donates-excess-food", "donates-excess-food-exception", "double-beds-count", "double-beds-count-exception", "dryer", "dryer-exception", "eating", "eco-friendly-toiletries", "eco-friendly-toiletries-exception", "electric-car-charging-stations", "electric-car-charging-stations-exception", "electronic-room-key", "electronic-room-key-exception", "elevator", "elevator-exception", "elliptical-machine", "elliptical-machine-exception", "employees-trained-cleaning-procedures", "employees-trained-cleaning-procedures-exception", "employees-trained-thorough-hand-washing", "employees-trained-thorough-hand-washing-exception", "employees-wear-protective-equipment", "employees-wear-protective-equipment-exception", "energy-conservation-program", "energy-conservation-program-exception", "energy-efficiency", "energy-efficient-heating-and-cooling-systems", "energy-efficient-heating-and-cooling-systems-exception", "energy-efficient-lighting", "energy-efficient-lighting-exception", "energy-saving-thermostats", "energy-saving-thermostats-exception", "enhanced-cleaning", "executive-floor", "executive-floor-exception", "face-mask-required", "face-mask-required-exception", "families", "feather-pillows", "feather-pillows-exception", "features", "fireplace", "fireplace-exception", "fitness-center", "fitness-center-exception", "floors-count", "floors-count-exception", "food-and-drink", "food-preparation-and-serving-additional-safety", "food-preparation-and-serving-additional-safety-exception", "food-waste-reduction-program", "food-waste-reduction-program-exception", "free-airport-shuttle", "free-airport-shuttle-exception", "free-bicycle-rental", "free-bicycle-rental-exception", "free-breakfast", "free-breakfast-exception", "free-fitness-center", "free-fitness-center-exception", "free-parking", "free-parking-exception", "free-private-car-service", "free-private-car-service-exception", "free-self-parking", "free-self-parking-exception", "free-valet-parking", "free-valet-parking-exception", "free-watercraft-rental", "free-watercraft-rental-exception", "free-weights", "free-weights-exception", "free-wifi", "free-wifi-exception", "front-desk", "front-desk-exception", "full-service-laundry", "full-service-laundry-exception", "game-room", "game-room-exception", "garden-view", "garden-view-exception", "gift-shop", "gift-shop-exception", "golf", "golf-exception", "green-building-design", "green-building-design-exception", "guest-room-hygiene-kits-available", "guest-room-hygiene-kits-available-exception", "guest-rooms-enhanced-cleaning", "guest-rooms-enhanced-cleaning-exception", "hairdryer", "hairdryer-exception", "health-and-safety", "hearing-accessible-doorbell", "hearing-accessible-doorbell-exception", "hearing-accessible-fire-alarm", "hearing-accessible-fire-alarm-exception", "hearing-accessible-unit", "hearing-accessible-unit-exception", "heating", "heating-exception", "horseback-riding", "horseback-riding-exception", "hot-tub", "hot-tub-exception", "hours", "housekeeping", "housekeeping-available", "housekeeping-available-exception", "housekeeping-scheduled-request-only", "housekeeping-scheduled-request-only-exception", "hypoallergenic-bedding", "hypoallergenic-bedding-exception", "increased-food-safety", "independent-organization-audits-energy-use", "independent-organization-audits-energy-use-exception", "independent-organization-audits-water-use", "independent-organization-audits-water-use-exception", "individual-packaged-meals", "individual-packaged-meals-exception", "indoor-grill", "indoor-grill-exception", "indoor-pool", "indoor-pool-exception", "indoor-pools-count", "indoor-pools-count-exception", "inunit-safe", "inunit-safe-exception", "inunit-wifi-available", "inunit-wifi-available-exception", "ironing-equipment", "ironing-equipment-exception", "kettle", "kettle-exception", "kids-activities", "kids-activities-exception", "kids-club", "kids-club-exception", "kids-friendly", "kids-friendly-exception", "kids-stay-free", "kids-stay-free-exception", "king-beds-count", "king-beds-count-exception", "kitchen-available", "kitchen-available-exception", "lake-view", "lake-view-exception", "landmark-view", "landmark-view-exception", "last-renovated-year", "last-renovated-year-exception", "layout", "lazy-river", "lazy-river-exception", "leed-certification", "leed-certification-exception", "lifeguard", "lifeguard-exception", "linen-reuse-program", "linen-reuse-program-exception", "living-area-sq-meters", "living-area-sq-meters-exception", "local-shuttle", "local-shuttle-exception", "locally-sourced-food-and-beverages", "locally-sourced-food-and-beverages-exception", "loft", "loft-exception", "massage", "massage-exception", "max-adult-occupants-count", "max-adult-occupants-count-exception", "max-child-age", "max-child-age-exception", "max-child-occupants-count", "max-child-occupants-count-exception", "max-kids-stay-free-count", "max-kids-stay-free-count-exception", "max-occupants-count", "max-occupants-count-exception", "meeting-rooms", "meeting-rooms-count", "meeting-rooms-count-exception", "meeting-rooms-exception", "memory-foam-pillows", "memory-foam-pillows-exception", "metadata", "microwave", "microwave-exception", "minibar", "minibar-exception", "minimized-contact", "minutes", "mobile-nfc", "mobile-nfc-exception", "mobility-accessible", "mobility-accessible-bathtub", "mobility-accessible-bathtub-exception", "mobility-accessible-elevator", "mobility-accessible-elevator-exception", "mobility-accessible-exception", "mobility-accessible-parking", "mobility-accessible-parking-exception", "mobility-accessible-pool", "mobility-accessible-pool-exception", "mobility-accessible-shower", "mobility-accessible-shower-exception", "mobility-accessible-toilet", "mobility-accessible-toilet-exception", "mobility-accessible-unit", "mobility-accessible-unit-exception", "name", "nanos", "nightclub", "nightclub-exception", "no-high-touch-items-common-areas", "no-high-touch-items-common-areas-exception", "no-high-touch-items-guest-rooms", "no-high-touch-items-guest-rooms-exception", "no-single-use-plastic-straws", "no-single-use-plastic-straws-exception", "no-single-use-plastic-water-bottles", "no-single-use-plastic-water-bottles-exception", "no-styrofoam-food-containers", "no-styrofoam-food-containers-exception", "non-smoking", "non-smoking-exception", "ocean-view", "ocean-view-exception", "organic-cage-free-eggs", "organic-cage-free-eggs-exception", "organic-food-and-beverages", "organic-food-and-beverages-exception", "other-beds-count", "other-beds-count-exception", "outdoor-grill", "outdoor-grill-exception", "outdoor-pool", "outdoor-pool-exception", "outdoor-pools-count", "outdoor-pools-count-exception", "oven", "oven-exception", "parking", "parking-available", "parking-available-exception", "patio", "patio-exception", "pay-per-view-movies", "pay-per-view-movies-exception", "payment-options", "personal-protection", "pets", "pets-allowed", "pets-allowed-exception", "pets-allowed-free", "pets-allowed-free-exception", "physical-distancing", "physical-distancing-required", "physical-distancing-required-exception", "plastic-keycards-disinfected", "plastic-keycards-disinfected-exception", "policies", "pool", "pool-exception", "pool-view", "pool-view-exception", "pools", "pools-count", "pools-count-exception", "private-bathroom", "private-bathroom-exception", "private-beach", "private-beach-exception", "private-car-service", "private-car-service-exception", "private-home", "private-home-exception", "property", "protective-equipment-available", "protective-equipment-available-exception", "public-area-wifi-available", "public-area-wifi-available-exception", "public-internet-terminal", "public-internet-terminal-exception", "queen-beds-count", "queen-beds-count-exception", "recycling-program", "recycling-program-exception", "refillable-toiletry-containers", "refillable-toiletry-containers-exception", "refrigerator", "refrigerator-exception", "responsible-purchasing-policy", "responsible-purchasing-policy-exception", "responsibly-sources-seafood", "responsibly-sources-seafood-exception", "restaurant", "restaurant-exception", "restaurants-count", "restaurants-count-exception", "roll-away-beds-count", "roll-away-beds-count-exception", "room-bookings-buffer", "room-bookings-buffer-exception", "room-service", "room-service-exception", "rooms-count", "rooms-count-exception", "safely-disposes-batteries", "safely-disposes-batteries-exception", "safely-disposes-electronics", "safely-disposes-electronics-exception", "safely-disposes-lightbulbs", "safely-disposes-lightbulbs-exception", "safely-handles-hazardous-substances", "safely-handles-hazardous-substances-exception", "safety-dividers", "safety-dividers-exception", "salon", "salon-exception", "sauna", "sauna-exception", "scuba", "scuba-exception", "seconds", "self-parking-available", "self-parking-available-exception", "self-service-laundry", "self-service-laundry-exception", "services", "shared-areas-limited-occupancy", "shared-areas-limited-occupancy-exception", "shower", "shower-exception", "single-or-twin-beds-count", "single-or-twin-beds-count-exception", "single-use-food-menus", "single-use-food-menus-exception", "sink", "sink-exception", "sleeping", "smoke-free-property", "smoke-free-property-exception", "snackbar", "snackbar-exception", "snorkeling", "snorkeling-exception", "soap-donation-program", "soap-donation-program-exception", "social-hour", "social-hour-exception", "sofa-beds-count", "sofa-beds-count-exception", "some-units", "spa", "spa-exception", "stairs", "stairs-exception", "stove", "stove-exception", "suite", "suite-exception", "sustainability", "sustainability-certifications", "sustainable-sourcing", "synthetic-pillows", "synthetic-pillows-exception", "table-service", "table-service-exception", "tea-station", "tea-station-exception", "tennis", "tennis-exception", "tier", "tier-exception", "toaster", "toaster-exception", "toilet", "toilet-exception", "toiletry-donation-program", "toiletry-donation-program-exception", "total-living-areas", "towel-reuse-program", "towel-reuse-program-exception", "transfer", "transfer-exception", "transportation", "treadmill", "treadmill-exception", "turndown-service", "turndown-service-exception", "tv", "tv-casting", "tv-casting-exception", "tv-exception", "tv-streaming", "tv-streaming-exception", "twenty-four-hour-front-desk", "twenty-four-hour-front-desk-exception", "twenty-four-hour-room-service", "twenty-four-hour-room-service-exception", "universal-power-adapters", "universal-power-adapters-exception", "update-time", "valet-parking-available", "valet-parking-available-exception", "valley-view", "valley-view-exception", "vegan-meals", "vegan-meals-exception", "vegetarian-meals", "vegetarian-meals-exception", "vending-machine", "vending-machine-exception", "views", "wading-pool", "wading-pool-exception", "wake-up-calls", "wake-up-calls-exception", "washer", "washer-exception", "waste-reduction", "water-bottle-filling-stations", "water-bottle-filling-stations-exception", "water-conservation", "water-park", "water-park-exception", "water-saving-showers", "water-saving-showers-exception", "water-saving-sinks", "water-saving-sinks-exception", "water-saving-toilets", "water-saving-toilets-exception", "water-skiing", "water-skiing-exception", "watercraft-rental", "watercraft-rental-exception", "waterslide", "waterslide-exception", "wave-pool", "wave-pool-exception", "weight-machine", "weight-machine-exception", "wellness", "wellness-areas-have-private-spaces", "wellness-areas-have-private-spaces-exception", "wifi-available", "wifi-available-exception"]);
                        err.issues.push(CLIError::Field(FieldError::Unknown(temp_cursor.to_string(), suggestion, value.map(|v| v.to_string()))));
                        None
                    }
                };
            if let Some((field_cursor_str, type_info)) = type_info {
                FieldCursor::from(field_cursor_str).set_json_value(&mut object, value.unwrap(), type_info, err, &temp_cursor);
            }
        }
        let mut request: api::Lodging = json::value::from_value(object).unwrap();
        let mut call = self.hub.locations().update_lodging(request, opt.value_of("name").unwrap_or(""));
        for parg in opt.values_of("v").map(|i|i.collect()).unwrap_or(Vec::new()).iter() {
            let (key, value) = parse_kv_arg(&*parg, err, false);
            match key {
                "update-mask" => {
                    call = call.update_mask(        value.map(|v| arg_from_str(v, err, "update-mask", "google-fieldmask")).unwrap_or(FieldMask::default()));
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
                                                                           v.extend(["update-mask"].iter().map(|v|*v));
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
            ("locations", Some(opt)) => {
                match opt.subcommand() {
                    ("get-lodging", Some(opt)) => {
                        call_result = self._locations_get_lodging(opt, dry_run, &mut err).await;
                    },
                    ("lodging-get-google-updated", Some(opt)) => {
                        call_result = self._locations_lodging_get_google_updated(opt, dry_run, &mut err).await;
                    },
                    ("update-lodging", Some(opt)) => {
                        call_result = self._locations_update_lodging(opt, dry_run, &mut err).await;
                    },
                    _ => {
                        err.issues.push(CLIError::MissingMethodError("locations".to_string()));
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

            match client::application_secret_from_directory(&config_dir, "mybusinesslodging1-secret.json",
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
        ).persist_tokens_to_disk(format!("{}/mybusinesslodging1", config_dir)).build().await.unwrap();

        let engine = Engine {
            opt: opt,
            hub: api::MyBusinessLodging::new(client, auth),
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
        ("locations", "methods: 'get-lodging', 'lodging-get-google-updated' and 'update-lodging'", vec![
            ("get-lodging",
                    Some(r##"Returns the Lodging of a specific location."##),
                    "Details at http://byron.github.io/google-apis-rs/google_mybusinesslodging1_cli/locations_get-lodging",
                  vec![
                    (Some(r##"name"##),
                     None,
                     Some(r##"Required. Google identifier for this location in the form: `locations/{location_id}/lodging`"##),
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
            ("lodging-get-google-updated",
                    Some(r##"Returns the Google updated Lodging of a specific location."##),
                    "Details at http://byron.github.io/google-apis-rs/google_mybusinesslodging1_cli/locations_lodging-get-google-updated",
                  vec![
                    (Some(r##"name"##),
                     None,
                     Some(r##"Required. Google identifier for this location in the form: `locations/{location_id}/lodging`"##),
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
            ("update-lodging",
                    Some(r##"Updates the Lodging of a specific location."##),
                    "Details at http://byron.github.io/google-apis-rs/google_mybusinesslodging1_cli/locations_update-lodging",
                  vec![
                    (Some(r##"name"##),
                     None,
                     Some(r##"Required. Google identifier for this location in the form: `locations/{location_id}/lodging`"##),
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
    
    let mut app = App::new("mybusinesslodging1")
           .author("Sebastian Thiel <byronimo@gmail.com>")
           .version("5.0.5+20240625")
           .about("The My Business Lodging API enables managing lodging business information on Google. Note - If you have a quota of 0 after enabling the API, please request for GBP API access.")
           .after_help("All documentation details can be found at http://byron.github.io/google-apis-rs/google_mybusinesslodging1_cli")
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
