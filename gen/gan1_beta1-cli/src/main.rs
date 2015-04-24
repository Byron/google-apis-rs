// DO NOT EDIT !
// This file was generated automatically from 'src/mako/cli/main.rs.mako'
// DO NOT EDIT !
#![feature(plugin, exit_status)]
#![plugin(docopt_macros)]
#![allow(unused_variables, unused_imports, dead_code, unused_mut)]

extern crate docopt;
extern crate yup_oauth2 as oauth2;
extern crate yup_hyper_mock as mock;
extern crate rustc_serialize;
extern crate serde;
extern crate hyper;
extern crate mime;
extern crate google_gan1_beta1 as api;

use std::env;
use std::io::{self, Write};

docopt!(Options derive Debug, "
Usage: 
  gan1-beta1 [options] advertisers get <role> <role-id> [-p <v>]... [-o <out>]
  gan1-beta1 [options] advertisers list <role> <role-id> [-p <v>]... [-o <out>]
  gan1-beta1 [options] cc-offers list <publisher> [-p <v>]... [-o <out>]
  gan1-beta1 [options] events list <role> <role-id> [-p <v>]... [-o <out>]
  gan1-beta1 [options] links get <role> <role-id> <link-id> [-p <v>]... [-o <out>]
  gan1-beta1 [options] links insert <role> <role-id> -r <kv>... [-p <v>]... [-o <out>]
  gan1-beta1 [options] links list <role> <role-id> [-p <v>]... [-o <out>]
  gan1-beta1 [options] publishers get <role> <role-id> [-p <v>]... [-o <out>]
  gan1-beta1 [options] publishers list <role> <role-id> [-p <v>]... [-o <out>]
  gan1-beta1 [options] reports get <role> <role-id> <report-type> [-p <v>]... [-o <out>]
  gan1-beta1 --help

All documentation details can be found TODO: <URL to github.io docs here, see #51>

Configuration:
  --config-dir <folder>
            A directory into which we will store our persistent data. Defaults to a user-writable
            directory that we will create during the first invocation.
            [default: ~/.google-service-cli]
  --debug
            Output all server communication to standard error. `tx` and `rx` are placed into 
            the same stream.
  --debug-auth
            Output all communication related to authentication to standard error. `tx` and `rx` are placed into 
            the same stream.
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
    hub: api::Gan<hyper::Client, Authenticator<DefaultAuthenticatorDelegate, JsonTokenStorage, hyper::Client>>,
}


impl Engine {
    fn _advertisers_get(&self, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Option<api::Error> {
        let mut call = self.hub.advertisers().get(&self.opt.arg_role, &self.opt.arg_role_id);
        for parg in self.opt.arg_v.iter() {
            let (key, value) = parse_kv_arg(&*parg, err, false);
            match key {
                "advertiser-id" => {
                    call = call.advertiser_id(value.unwrap_or(""));
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
                    serde::json::to_writer_pretty(&mut ostream, &output_schema).unwrap();
                    None
                }
            }
        }
    }

    fn _advertisers_list(&self, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Option<api::Error> {
        let mut call = self.hub.advertisers().list(&self.opt.arg_role, &self.opt.arg_role_id);
        for parg in self.opt.arg_v.iter() {
            let (key, value) = parse_kv_arg(&*parg, err, false);
            match key {
                "relationship-status" => {
                    call = call.relationship_status(value.unwrap_or(""));
                },
                "page-token" => {
                    call = call.page_token(value.unwrap_or(""));
                },
                "min-seven-day-epc" => {
                    call = call.min_seven_day_epc(arg_from_str(value.unwrap_or("0.0"), err, "min-seven-day-epc", "number"));
                },
                "min-payout-rank" => {
                    call = call.min_payout_rank(arg_from_str(value.unwrap_or("-0"), err, "min-payout-rank", "integer"));
                },
                "min-ninety-day-epc" => {
                    call = call.min_ninety_day_epc(arg_from_str(value.unwrap_or("0.0"), err, "min-ninety-day-epc", "number"));
                },
                "max-results" => {
                    call = call.max_results(arg_from_str(value.unwrap_or("-0"), err, "max-results", "integer"));
                },
                "advertiser-category" => {
                    call = call.advertiser_category(value.unwrap_or(""));
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
                    serde::json::to_writer_pretty(&mut ostream, &output_schema).unwrap();
                    None
                }
            }
        }
    }

    fn _cc_offers_list(&self, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Option<api::Error> {
        let mut call = self.hub.cc_offers().list(&self.opt.arg_publisher);
        for parg in self.opt.arg_v.iter() {
            let (key, value) = parse_kv_arg(&*parg, err, false);
            match key {
                "projection" => {
                    call = call.projection(value.unwrap_or(""));
                },
                "advertiser" => {
                    call = call.add_advertiser(value.unwrap_or(""));
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
                    serde::json::to_writer_pretty(&mut ostream, &output_schema).unwrap();
                    None
                }
            }
        }
    }

    fn _events_list(&self, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Option<api::Error> {
        let mut call = self.hub.events().list(&self.opt.arg_role, &self.opt.arg_role_id);
        for parg in self.opt.arg_v.iter() {
            let (key, value) = parse_kv_arg(&*parg, err, false);
            match key {
                "type" => {
                    call = call.type_(value.unwrap_or(""));
                },
                "status" => {
                    call = call.status(value.unwrap_or(""));
                },
                "sku" => {
                    call = call.sku(value.unwrap_or(""));
                },
                "publisher-id" => {
                    call = call.publisher_id(value.unwrap_or(""));
                },
                "product-category" => {
                    call = call.product_category(value.unwrap_or(""));
                },
                "page-token" => {
                    call = call.page_token(value.unwrap_or(""));
                },
                "order-id" => {
                    call = call.order_id(value.unwrap_or(""));
                },
                "modify-date-min" => {
                    call = call.modify_date_min(value.unwrap_or(""));
                },
                "modify-date-max" => {
                    call = call.modify_date_max(value.unwrap_or(""));
                },
                "member-id" => {
                    call = call.member_id(value.unwrap_or(""));
                },
                "max-results" => {
                    call = call.max_results(arg_from_str(value.unwrap_or("-0"), err, "max-results", "integer"));
                },
                "link-id" => {
                    call = call.link_id(value.unwrap_or(""));
                },
                "event-date-min" => {
                    call = call.event_date_min(value.unwrap_or(""));
                },
                "event-date-max" => {
                    call = call.event_date_max(value.unwrap_or(""));
                },
                "charge-type" => {
                    call = call.charge_type(value.unwrap_or(""));
                },
                "advertiser-id" => {
                    call = call.advertiser_id(value.unwrap_or(""));
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
                    serde::json::to_writer_pretty(&mut ostream, &output_schema).unwrap();
                    None
                }
            }
        }
    }

    fn _links_get(&self, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Option<api::Error> {
        let mut call = self.hub.links().get(&self.opt.arg_role, &self.opt.arg_role_id, &self.opt.arg_link_id);
        for parg in self.opt.arg_v.iter() {
            let (key, value) = parse_kv_arg(&*parg, err, false);
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
            let mut ostream = writer_from_opts(self.opt.flag_o, &self.opt.arg_out);
            match match protocol {
                "standard-request" => call.doit(),
                _ => unreachable!(),
            } {
                Err(api_err) => Some(api_err),
                Ok((mut response, output_schema)) => {
                    serde::json::to_writer_pretty(&mut ostream, &output_schema).unwrap();
                    None
                }
            }
        }
    }

    fn _links_insert(&self, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Option<api::Error> {
        let mut request = api::Link::default();
        let mut call = self.hub.links().insert(&request, &self.opt.arg_role, &self.opt.arg_role_id);
        for parg in self.opt.arg_v.iter() {
            let (key, value) = parse_kv_arg(&*parg, err, false);
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
        
        let mut field_name = FieldCursor::default();
        for kvarg in self.opt.arg_kv.iter() {
            let (key, value) = parse_kv_arg(&*kvarg, err, false);
            if let Err(field_err) = field_name.set(&*key) {
                err.issues.push(field_err);
            }
            fn request_epc_ninety_day_average_init(request: &mut api::Link) {
                if request.epc_ninety_day_average.is_none() {
                    request.epc_ninety_day_average = Some(Default::default());
                }
            }
            
            fn request_epc_seven_day_average_init(request: &mut api::Link) {
                if request.epc_seven_day_average.is_none() {
                    request.epc_seven_day_average = Some(Default::default());
                }
            }
            
            fn request_special_offers_free_shipping_min_init(request: &mut api::Link) {
                request_special_offers_init(request);
                if request.special_offers.as_mut().unwrap().free_shipping_min.is_none() {
                    request.special_offers.as_mut().unwrap().free_shipping_min = Some(Default::default());
                }
            }
            
            fn request_special_offers_init(request: &mut api::Link) {
                if request.special_offers.is_none() {
                    request.special_offers = Some(Default::default());
                }
            }
            
            fn request_special_offers_percent_off_min_init(request: &mut api::Link) {
                request_special_offers_init(request);
                if request.special_offers.as_mut().unwrap().percent_off_min.is_none() {
                    request.special_offers.as_mut().unwrap().percent_off_min = Some(Default::default());
                }
            }
            
            fn request_special_offers_price_cut_init(request: &mut api::Link) {
                request_special_offers_init(request);
                if request.special_offers.as_mut().unwrap().price_cut.is_none() {
                    request.special_offers.as_mut().unwrap().price_cut = Some(Default::default());
                }
            }
            
            fn request_special_offers_price_cut_min_init(request: &mut api::Link) {
                request_special_offers_init(request);
                if request.special_offers.as_mut().unwrap().price_cut_min.is_none() {
                    request.special_offers.as_mut().unwrap().price_cut_min = Some(Default::default());
                }
            }
            
            match &field_name.to_string()[..] {
                "link-type" => {
                        request.link_type = Some(value.unwrap_or("").to_string());
                    },
                "start-date" => {
                        request.start_date = Some(value.unwrap_or("").to_string());
                    },
                "kind" => {
                        request.kind = Some(value.unwrap_or("").to_string());
                    },
                "end-date" => {
                        request.end_date = Some(value.unwrap_or("").to_string());
                    },
                "description" => {
                        request.description = Some(value.unwrap_or("").to_string());
                    },
                "name" => {
                        request.name = Some(value.unwrap_or("").to_string());
                    },
                "special-offers.price-cut.amount" => {
                        request_special_offers_price_cut_init(&mut request);
                        request.special_offers.as_mut().unwrap().price_cut.as_mut().unwrap().amount = Some(arg_from_str(value.unwrap_or("0.0"), err, "special-offers.price-cut.amount", "number"));
                    },
                "special-offers.price-cut.currency-code" => {
                        request_special_offers_price_cut_init(&mut request);
                        request.special_offers.as_mut().unwrap().price_cut.as_mut().unwrap().currency_code = Some(value.unwrap_or("").to_string());
                    },
                "special-offers.price-cut-min.amount" => {
                        request_special_offers_price_cut_min_init(&mut request);
                        request.special_offers.as_mut().unwrap().price_cut_min.as_mut().unwrap().amount = Some(arg_from_str(value.unwrap_or("0.0"), err, "special-offers.price-cut-min.amount", "number"));
                    },
                "special-offers.price-cut-min.currency-code" => {
                        request_special_offers_price_cut_min_init(&mut request);
                        request.special_offers.as_mut().unwrap().price_cut_min.as_mut().unwrap().currency_code = Some(value.unwrap_or("").to_string());
                    },
                "special-offers.free-shipping" => {
                        request_special_offers_price_cut_min_init(&mut request);
                        request.special_offers.as_mut().unwrap().free_shipping = Some(arg_from_str(value.unwrap_or("false"), err, "special-offers.free-shipping", "boolean"));
                    },
                "special-offers.promotion-codes" => {
                        request_special_offers_price_cut_min_init(&mut request);
                        if request.special_offers.as_mut().unwrap().promotion_codes.is_none() {
                           request.special_offers.as_mut().unwrap().promotion_codes = Some(Default::default());
                        }
                                        request.special_offers.as_mut().unwrap().promotion_codes.as_mut().unwrap().push(value.unwrap_or("").to_string());
                    },
                "special-offers.percent-off" => {
                        request_special_offers_price_cut_min_init(&mut request);
                        request.special_offers.as_mut().unwrap().percent_off = Some(arg_from_str(value.unwrap_or("0.0"), err, "special-offers.percent-off", "number"));
                    },
                "special-offers.percent-off-min.amount" => {
                        request_special_offers_percent_off_min_init(&mut request);
                        request.special_offers.as_mut().unwrap().percent_off_min.as_mut().unwrap().amount = Some(arg_from_str(value.unwrap_or("0.0"), err, "special-offers.percent-off-min.amount", "number"));
                    },
                "special-offers.percent-off-min.currency-code" => {
                        request_special_offers_percent_off_min_init(&mut request);
                        request.special_offers.as_mut().unwrap().percent_off_min.as_mut().unwrap().currency_code = Some(value.unwrap_or("").to_string());
                    },
                "special-offers.free-gift" => {
                        request_special_offers_percent_off_min_init(&mut request);
                        request.special_offers.as_mut().unwrap().free_gift = Some(arg_from_str(value.unwrap_or("false"), err, "special-offers.free-gift", "boolean"));
                    },
                "special-offers.free-shipping-min.amount" => {
                        request_special_offers_free_shipping_min_init(&mut request);
                        request.special_offers.as_mut().unwrap().free_shipping_min.as_mut().unwrap().amount = Some(arg_from_str(value.unwrap_or("0.0"), err, "special-offers.free-shipping-min.amount", "number"));
                    },
                "special-offers.free-shipping-min.currency-code" => {
                        request_special_offers_free_shipping_min_init(&mut request);
                        request.special_offers.as_mut().unwrap().free_shipping_min.as_mut().unwrap().currency_code = Some(value.unwrap_or("").to_string());
                    },
                "epc-seven-day-average.amount" => {
                        request_epc_seven_day_average_init(&mut request);
                        request.epc_seven_day_average.as_mut().unwrap().amount = Some(arg_from_str(value.unwrap_or("0.0"), err, "epc-seven-day-average.amount", "number"));
                    },
                "epc-seven-day-average.currency-code" => {
                        request_epc_seven_day_average_init(&mut request);
                        request.epc_seven_day_average.as_mut().unwrap().currency_code = Some(value.unwrap_or("").to_string());
                    },
                "create-date" => {
                        request_epc_seven_day_average_init(&mut request);
                        request.create_date = Some(value.unwrap_or("").to_string());
                    },
                "image-alt-text" => {
                        request_epc_seven_day_average_init(&mut request);
                        request.image_alt_text = Some(value.unwrap_or("").to_string());
                    },
                "id" => {
                        request_epc_seven_day_average_init(&mut request);
                        request.id = Some(value.unwrap_or("").to_string());
                    },
                "advertiser-id" => {
                        request_epc_seven_day_average_init(&mut request);
                        request.advertiser_id = Some(value.unwrap_or("").to_string());
                    },
                "is-active" => {
                        request_epc_seven_day_average_init(&mut request);
                        request.is_active = Some(arg_from_str(value.unwrap_or("false"), err, "is-active", "boolean"));
                    },
                "promotion-type" => {
                        request_epc_seven_day_average_init(&mut request);
                        request.promotion_type = Some(value.unwrap_or("").to_string());
                    },
                "duration" => {
                        request_epc_seven_day_average_init(&mut request);
                        request.duration = Some(value.unwrap_or("").to_string());
                    },
                "authorship" => {
                        request_epc_seven_day_average_init(&mut request);
                        request.authorship = Some(value.unwrap_or("").to_string());
                    },
                "impression-tracking-url" => {
                        request_epc_seven_day_average_init(&mut request);
                        request.impression_tracking_url = Some(value.unwrap_or("").to_string());
                    },
                "epc-ninety-day-average.amount" => {
                        request_epc_ninety_day_average_init(&mut request);
                        request.epc_ninety_day_average.as_mut().unwrap().amount = Some(arg_from_str(value.unwrap_or("0.0"), err, "epc-ninety-day-average.amount", "number"));
                    },
                "epc-ninety-day-average.currency-code" => {
                        request_epc_ninety_day_average_init(&mut request);
                        request.epc_ninety_day_average.as_mut().unwrap().currency_code = Some(value.unwrap_or("").to_string());
                    },
                "availability" => {
                        request_epc_ninety_day_average_init(&mut request);
                        request.availability = Some(value.unwrap_or("").to_string());
                    },
                "click-tracking-url" => {
                        request_epc_ninety_day_average_init(&mut request);
                        request.click_tracking_url = Some(value.unwrap_or("").to_string());
                    },
                "destination-url" => {
                        request_epc_ninety_day_average_init(&mut request);
                        request.destination_url = Some(value.unwrap_or("").to_string());
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
                    serde::json::to_writer_pretty(&mut ostream, &output_schema).unwrap();
                    None
                }
            }
        }
    }

    fn _links_list(&self, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Option<api::Error> {
        let mut call = self.hub.links().list(&self.opt.arg_role, &self.opt.arg_role_id);
        for parg in self.opt.arg_v.iter() {
            let (key, value) = parse_kv_arg(&*parg, err, false);
            match key {
                "start-date-min" => {
                    call = call.start_date_min(value.unwrap_or(""));
                },
                "start-date-max" => {
                    call = call.start_date_max(value.unwrap_or(""));
                },
                "search-text" => {
                    call = call.search_text(value.unwrap_or(""));
                },
                "relationship-status" => {
                    call = call.relationship_status(value.unwrap_or(""));
                },
                "promotion-type" => {
                    call = call.add_promotion_type(value.unwrap_or(""));
                },
                "page-token" => {
                    call = call.page_token(value.unwrap_or(""));
                },
                "max-results" => {
                    call = call.max_results(arg_from_str(value.unwrap_or("-0"), err, "max-results", "integer"));
                },
                "link-type" => {
                    call = call.link_type(value.unwrap_or(""));
                },
                "create-date-min" => {
                    call = call.create_date_min(value.unwrap_or(""));
                },
                "create-date-max" => {
                    call = call.create_date_max(value.unwrap_or(""));
                },
                "authorship" => {
                    call = call.authorship(value.unwrap_or(""));
                },
                "asset-size" => {
                    call = call.add_asset_size(value.unwrap_or(""));
                },
                "advertiser-id" => {
                    call = call.add_advertiser_id(value.unwrap_or(""));
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
                    serde::json::to_writer_pretty(&mut ostream, &output_schema).unwrap();
                    None
                }
            }
        }
    }

    fn _publishers_get(&self, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Option<api::Error> {
        let mut call = self.hub.publishers().get(&self.opt.arg_role, &self.opt.arg_role_id);
        for parg in self.opt.arg_v.iter() {
            let (key, value) = parse_kv_arg(&*parg, err, false);
            match key {
                "publisher-id" => {
                    call = call.publisher_id(value.unwrap_or(""));
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
                    serde::json::to_writer_pretty(&mut ostream, &output_schema).unwrap();
                    None
                }
            }
        }
    }

    fn _publishers_list(&self, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Option<api::Error> {
        let mut call = self.hub.publishers().list(&self.opt.arg_role, &self.opt.arg_role_id);
        for parg in self.opt.arg_v.iter() {
            let (key, value) = parse_kv_arg(&*parg, err, false);
            match key {
                "relationship-status" => {
                    call = call.relationship_status(value.unwrap_or(""));
                },
                "publisher-category" => {
                    call = call.publisher_category(value.unwrap_or(""));
                },
                "page-token" => {
                    call = call.page_token(value.unwrap_or(""));
                },
                "min-seven-day-epc" => {
                    call = call.min_seven_day_epc(arg_from_str(value.unwrap_or("0.0"), err, "min-seven-day-epc", "number"));
                },
                "min-payout-rank" => {
                    call = call.min_payout_rank(arg_from_str(value.unwrap_or("-0"), err, "min-payout-rank", "integer"));
                },
                "min-ninety-day-epc" => {
                    call = call.min_ninety_day_epc(arg_from_str(value.unwrap_or("0.0"), err, "min-ninety-day-epc", "number"));
                },
                "max-results" => {
                    call = call.max_results(arg_from_str(value.unwrap_or("-0"), err, "max-results", "integer"));
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
                    serde::json::to_writer_pretty(&mut ostream, &output_schema).unwrap();
                    None
                }
            }
        }
    }

    fn _reports_get(&self, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Option<api::Error> {
        let mut call = self.hub.reports().get(&self.opt.arg_role, &self.opt.arg_role_id, &self.opt.arg_report_type);
        for parg in self.opt.arg_v.iter() {
            let (key, value) = parse_kv_arg(&*parg, err, false);
            match key {
                "status" => {
                    call = call.status(value.unwrap_or(""));
                },
                "start-index" => {
                    call = call.start_index(arg_from_str(value.unwrap_or("-0"), err, "start-index", "integer"));
                },
                "start-date" => {
                    call = call.start_date(value.unwrap_or(""));
                },
                "publisher-id" => {
                    call = call.add_publisher_id(value.unwrap_or(""));
                },
                "order-id" => {
                    call = call.add_order_id(value.unwrap_or(""));
                },
                "max-results" => {
                    call = call.max_results(arg_from_str(value.unwrap_or("-0"), err, "max-results", "integer"));
                },
                "link-id" => {
                    call = call.add_link_id(value.unwrap_or(""));
                },
                "event-type" => {
                    call = call.event_type(value.unwrap_or(""));
                },
                "end-date" => {
                    call = call.end_date(value.unwrap_or(""));
                },
                "calculate-totals" => {
                    call = call.calculate_totals(arg_from_str(value.unwrap_or("false"), err, "calculate-totals", "boolean"));
                },
                "advertiser-id" => {
                    call = call.add_advertiser_id(value.unwrap_or(""));
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
                    serde::json::to_writer_pretty(&mut ostream, &output_schema).unwrap();
                    None
                }
            }
        }
    }

    fn _doit(&self, dry_run: bool) -> (Option<api::Error>, Option<InvalidOptionsError>) {
        let mut err = InvalidOptionsError::new();
        let mut call_result: Option<api::Error>;
        let mut err_opt: Option<InvalidOptionsError> = None;

        if self.opt.cmd_advertisers {
            if self.opt.cmd_get {
                call_result = self._advertisers_get(dry_run, &mut err);
            } else if self.opt.cmd_list {
                call_result = self._advertisers_list(dry_run, &mut err);
            } else {
                unreachable!();
            }
        }
 else if self.opt.cmd_cc_offers {
            if self.opt.cmd_list {
                call_result = self._cc_offers_list(dry_run, &mut err);
            } else {
                unreachable!();
            }
        }
 else if self.opt.cmd_events {
            if self.opt.cmd_list {
                call_result = self._events_list(dry_run, &mut err);
            } else {
                unreachable!();
            }
        }
 else if self.opt.cmd_links {
            if self.opt.cmd_get {
                call_result = self._links_get(dry_run, &mut err);
            } else if self.opt.cmd_insert {
                call_result = self._links_insert(dry_run, &mut err);
            } else if self.opt.cmd_list {
                call_result = self._links_list(dry_run, &mut err);
            } else {
                unreachable!();
            }
        }
 else if self.opt.cmd_publishers {
            if self.opt.cmd_get {
                call_result = self._publishers_get(dry_run, &mut err);
            } else if self.opt.cmd_list {
                call_result = self._publishers_list(dry_run, &mut err);
            } else {
                unreachable!();
            }
        }
 else if self.opt.cmd_reports {
            if self.opt.cmd_get {
                call_result = self._reports_get(dry_run, &mut err);
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

            match cmn::application_secret_from_directory(&config_dir, "gan1-beta1-secret.json", 
                                                         "{\"installed\":{\"auth_uri\":\"https://accounts.google.com/o/oauth2/auth\",\"client_secret\":\"hCsslbCUyfehWMmbkG8vTYxG\",\"token_uri\":\"https://accounts.google.com/o/oauth2/token\",\"client_email\":\"\",\"redirect_uris\":[\"urn:ietf:wg:oauth:2.0:oob\",\"oob\"],\"client_x509_cert_url\":\"\",\"client_id\":\"620010449518-9ngf7o4dhs0dka470npqvor6dc5lqb9b.apps.googleusercontent.com\",\"auth_provider_x509_cert_url\":\"https://www.googleapis.com/oauth2/v1/certs\"}}") {
                Ok(secret) => (config_dir, secret),
                Err(e) => return Err(InvalidOptionsError::single(e, 4))
            }
        };

        let auth = Authenticator::new(  &secret, DefaultAuthenticatorDelegate,
                                        if opt.flag_debug_auth {
                                            hyper::Client::with_connector(mock::TeeConnector {
                                                    connector: hyper::net::HttpConnector(None) 
                                                })
                                        } else {
                                            hyper::Client::new()
                                        },
                                        JsonTokenStorage {
                                          program_name: "gan1-beta1",
                                          db_dir: config_dir.clone(),
                                        }, None);

        let client = 
            if opt.flag_debug {
                hyper::Client::with_connector(mock::TeeConnector {
                        connector: hyper::net::HttpConnector(None) 
                    })
            } else {
                hyper::Client::new()
            };
        let engine = Engine {
            opt: opt,
            hub: api::Gan::new(client, auth),
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
            writeln!(io::stderr(), "{}", err).ok();
            env::set_exit_status(err.exit_code);
        },
        Ok(engine) => {
            if let Some(err) = engine.doit() {
                writeln!(io::stderr(), "{:?}", err).ok();
                writeln!(io::stderr(), "{}", err).ok();
                env::set_exit_status(1);
            }
        }
    }
}