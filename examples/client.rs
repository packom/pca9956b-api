#![allow(missing_docs, unused_variables, trivial_casts)]

extern crate pca9956b_api;
#[allow(unused_extern_crates)]
extern crate futures;
#[allow(unused_extern_crates)]
#[macro_use]
extern crate swagger;
#[allow(unused_extern_crates)]
extern crate uuid;
extern crate clap;
extern crate tokio_core;

use swagger::{ContextBuilder, EmptyContext, XSpanIdString, Has, Push, AuthData};

#[allow(unused_imports)]
use futures::{Future, future, Stream, stream};
use tokio_core::reactor;
#[allow(unused_imports)]
use pca9956b_api::{ApiNoContext, ContextWrapperExt,
                      ApiError,
                      ClearErrorResponse,
                      GetAddrEnabledResponse,
                      GetAddrInfoResponse,
                      GetAddrValueResponse,
                      GetApiResponse,
                      GetConfigResponse,
                      GetCurrentResponse,
                      GetErrorResponse,
                      GetErrorsResponse,
                      GetFreqResponse,
                      GetGroupResponse,
                      GetLedCurrentResponse,
                      GetLedErrorResponse,
                      GetLedInfoResponse,
                      GetLedInfoAllResponse,
                      GetLedPwmResponse,
                      GetLedStateResponse,
                      GetOffsetResponse,
                      GetOutputChangeResponse,
                      GetOverTempResponse,
                      GetPwmResponse,
                      GetSleepResponse,
                      ResetResponse,
                      SetAddrEnabledResponse,
                      SetAddrValueResponse,
                      SetConfigResponse,
                      SetCurrentResponse,
                      SetFreqResponse,
                      SetGroupResponse,
                      SetLedCurrentResponse,
                      SetLedErrorResponse,
                      SetLedInfoResponse,
                      SetLedInfoAllResponse,
                      SetLedPwmResponse,
                      SetLedStateResponse,
                      SetOffsetResponse,
                      SetOutputChangeResponse,
                      SetPwmResponse,
                      SetSleepResponse
                     };
use clap::{App, Arg};

fn main() {
    let matches = App::new("client")
        .arg(Arg::with_name("operation")
            .help("Sets the operation to run")
            .possible_values(&[
    "ClearError",
    "GetAddrEnabled",
    "GetAddrInfo",
    "GetAddrValue",
    "GetApi",
    "GetConfig",
    "GetCurrent",
    "GetError",
    "GetErrors",
    "GetFreq",
    "GetGroup",
    "GetLedCurrent",
    "GetLedError",
    "GetLedInfo",
    "GetLedInfoAll",
    "GetLedPwm",
    "GetLedState",
    "GetOffset",
    "GetOutputChange",
    "GetOverTemp",
    "GetPwm",
    "GetSleep",
    "Reset",
    "SetAddrEnabled",
    "SetAddrValue",
    "SetCurrent",
    "SetFreq",
    "SetLedCurrent",
    "SetLedInfoAll",
    "SetLedPwm",
    "SetOffset",
    "SetPwm",
    "SetSleep",
])
            .required(true)
            .index(1))
        .arg(Arg::with_name("https")
            .long("https")
            .help("Whether to use HTTPS or not"))
        .arg(Arg::with_name("host")
            .long("host")
            .takes_value(true)
            .default_value("localhost")
            .help("Hostname to contact"))
        .arg(Arg::with_name("port")
            .long("port")
            .takes_value(true)
            .default_value("80")
            .help("Port to contact"))
        .get_matches();

    let mut core = reactor::Core::new().unwrap();
    let is_https = matches.is_present("https");
    let base_url = format!("{}://{}:{}",
                           if is_https { "https" } else { "http" },
                           matches.value_of("host").unwrap(),
                           matches.value_of("port").unwrap());
    let client = if matches.is_present("https") {
        // Using Simple HTTPS
        pca9956b_api::Client::try_new_https(core.handle(), &base_url, "examples/ca.pem")
            .expect("Failed to create HTTPS client")
    } else {
        // Using HTTP
        pca9956b_api::Client::try_new_http(core.handle(), &base_url)
            .expect("Failed to create HTTP client")
    };

    let context: make_context_ty!(ContextBuilder, EmptyContext, Option<AuthData>, XSpanIdString) =
        make_context!(ContextBuilder, EmptyContext, None as Option<AuthData>, XSpanIdString(self::uuid::Uuid::new_v4().to_string()));
    let client = client.with_context(context);

    match matches.value_of("operation") {

        Some("ClearError") => {
            let result = core.run(client.clear_error(56, 56));
            println!("{:?} (X-Span-ID: {:?})", result, (client.context() as &Has<XSpanIdString>).get().clone());
         },

        Some("GetAddrEnabled") => {
            let result = core.run(client.get_addr_enabled(56, 56, 56));
            println!("{:?} (X-Span-ID: {:?})", result, (client.context() as &Has<XSpanIdString>).get().clone());
         },

        Some("GetAddrInfo") => {
            let result = core.run(client.get_addr_info(56, 56, 56));
            println!("{:?} (X-Span-ID: {:?})", result, (client.context() as &Has<XSpanIdString>).get().clone());
         },

        Some("GetAddrValue") => {
            let result = core.run(client.get_addr_value(56, 56, 56));
            println!("{:?} (X-Span-ID: {:?})", result, (client.context() as &Has<XSpanIdString>).get().clone());
         },

        Some("GetApi") => {
            let result = core.run(client.get_api());
            println!("{:?} (X-Span-ID: {:?})", result, (client.context() as &Has<XSpanIdString>).get().clone());
         },

        Some("GetConfig") => {
            let result = core.run(client.get_config(56, 56));
            println!("{:?} (X-Span-ID: {:?})", result, (client.context() as &Has<XSpanIdString>).get().clone());
         },

        Some("GetCurrent") => {
            let result = core.run(client.get_current(56, 56));
            println!("{:?} (X-Span-ID: {:?})", result, (client.context() as &Has<XSpanIdString>).get().clone());
         },

        Some("GetError") => {
            let result = core.run(client.get_error(56, 56));
            println!("{:?} (X-Span-ID: {:?})", result, (client.context() as &Has<XSpanIdString>).get().clone());
         },

        Some("GetErrors") => {
            let result = core.run(client.get_errors(56, 56));
            println!("{:?} (X-Span-ID: {:?})", result, (client.context() as &Has<XSpanIdString>).get().clone());
         },

        Some("GetFreq") => {
            let result = core.run(client.get_freq(56, 56));
            println!("{:?} (X-Span-ID: {:?})", result, (client.context() as &Has<XSpanIdString>).get().clone());
         },

        Some("GetGroup") => {
            let result = core.run(client.get_group(56, 56));
            println!("{:?} (X-Span-ID: {:?})", result, (client.context() as &Has<XSpanIdString>).get().clone());
         },

        Some("GetLedCurrent") => {
            let result = core.run(client.get_led_current(56, 56, 56));
            println!("{:?} (X-Span-ID: {:?})", result, (client.context() as &Has<XSpanIdString>).get().clone());
         },

        Some("GetLedError") => {
            let result = core.run(client.get_led_error(56, 56, 56));
            println!("{:?} (X-Span-ID: {:?})", result, (client.context() as &Has<XSpanIdString>).get().clone());
         },

        Some("GetLedInfo") => {
            let result = core.run(client.get_led_info(56, 56, 56));
            println!("{:?} (X-Span-ID: {:?})", result, (client.context() as &Has<XSpanIdString>).get().clone());
         },

        Some("GetLedInfoAll") => {
            let result = core.run(client.get_led_info_all(56, 56));
            println!("{:?} (X-Span-ID: {:?})", result, (client.context() as &Has<XSpanIdString>).get().clone());
         },

        Some("GetLedPwm") => {
            let result = core.run(client.get_led_pwm(56, 56, 56));
            println!("{:?} (X-Span-ID: {:?})", result, (client.context() as &Has<XSpanIdString>).get().clone());
         },

        Some("GetLedState") => {
            let result = core.run(client.get_led_state(56, 56, 56));
            println!("{:?} (X-Span-ID: {:?})", result, (client.context() as &Has<XSpanIdString>).get().clone());
         },

        Some("GetOffset") => {
            let result = core.run(client.get_offset(56, 56));
            println!("{:?} (X-Span-ID: {:?})", result, (client.context() as &Has<XSpanIdString>).get().clone());
         },

        Some("GetOutputChange") => {
            let result = core.run(client.get_output_change(56, 56));
            println!("{:?} (X-Span-ID: {:?})", result, (client.context() as &Has<XSpanIdString>).get().clone());
         },

        Some("GetOverTemp") => {
            let result = core.run(client.get_over_temp(56, 56));
            println!("{:?} (X-Span-ID: {:?})", result, (client.context() as &Has<XSpanIdString>).get().clone());
         },

        Some("GetPwm") => {
            let result = core.run(client.get_pwm(56, 56));
            println!("{:?} (X-Span-ID: {:?})", result, (client.context() as &Has<XSpanIdString>).get().clone());
         },

        Some("GetSleep") => {
            let result = core.run(client.get_sleep(56, 56));
            println!("{:?} (X-Span-ID: {:?})", result, (client.context() as &Has<XSpanIdString>).get().clone());
         },

        Some("Reset") => {
            let result = core.run(client.reset(56));
            println!("{:?} (X-Span-ID: {:?})", result, (client.context() as &Has<XSpanIdString>).get().clone());
         },

        Some("SetAddrEnabled") => {
            let result = core.run(client.set_addr_enabled(56, 56, 56, true));
            println!("{:?} (X-Span-ID: {:?})", result, (client.context() as &Has<XSpanIdString>).get().clone());
         },

        Some("SetAddrValue") => {
            let result = core.run(client.set_addr_value(56, 56, 56, 56));
            println!("{:?} (X-Span-ID: {:?})", result, (client.context() as &Has<XSpanIdString>).get().clone());
         },

        // Disabled because there's no example.
        // Some("SetConfig") => {
        //     let result = core.run(client.set_config(56, 56, ???));
        //     println!("{:?} (X-Span-ID: {:?})", result, (client.context() as &Has<XSpanIdString>).get().clone());
        //  },

        Some("SetCurrent") => {
            let result = core.run(client.set_current(56, 56, 56));
            println!("{:?} (X-Span-ID: {:?})", result, (client.context() as &Has<XSpanIdString>).get().clone());
         },

        Some("SetFreq") => {
            let result = core.run(client.set_freq(56, 56, 56));
            println!("{:?} (X-Span-ID: {:?})", result, (client.context() as &Has<XSpanIdString>).get().clone());
         },

        // Disabled because there's no example.
        // Some("SetGroup") => {
        //     let result = core.run(client.set_group(56, 56, ???));
        //     println!("{:?} (X-Span-ID: {:?})", result, (client.context() as &Has<XSpanIdString>).get().clone());
        //  },

        Some("SetLedCurrent") => {
            let result = core.run(client.set_led_current(56, 56, 56, 56));
            println!("{:?} (X-Span-ID: {:?})", result, (client.context() as &Has<XSpanIdString>).get().clone());
         },

        // Disabled because there's no example.
        // Some("SetLedError") => {
        //     let result = core.run(client.set_led_error(56, 56, 56, ???));
        //     println!("{:?} (X-Span-ID: {:?})", result, (client.context() as &Has<XSpanIdString>).get().clone());
        //  },

        // Disabled because there's no example.
        // Some("SetLedInfo") => {
        //     let result = core.run(client.set_led_info(56, 56, 56, ???));
        //     println!("{:?} (X-Span-ID: {:?})", result, (client.context() as &Has<XSpanIdString>).get().clone());
        //  },

        Some("SetLedInfoAll") => {
            let result = core.run(client.set_led_info_all(56, 56, &Vec::new()));
            println!("{:?} (X-Span-ID: {:?})", result, (client.context() as &Has<XSpanIdString>).get().clone());
         },

        Some("SetLedPwm") => {
            let result = core.run(client.set_led_pwm(56, 56, 56, 56));
            println!("{:?} (X-Span-ID: {:?})", result, (client.context() as &Has<XSpanIdString>).get().clone());
         },

        // Disabled because there's no example.
        // Some("SetLedState") => {
        //     let result = core.run(client.set_led_state(56, 56, 56, ???));
        //     println!("{:?} (X-Span-ID: {:?})", result, (client.context() as &Has<XSpanIdString>).get().clone());
        //  },

        Some("SetOffset") => {
            let result = core.run(client.set_offset(56, 56, 56));
            println!("{:?} (X-Span-ID: {:?})", result, (client.context() as &Has<XSpanIdString>).get().clone());
         },

        // Disabled because there's no example.
        // Some("SetOutputChange") => {
        //     let result = core.run(client.set_output_change(56, 56, ???));
        //     println!("{:?} (X-Span-ID: {:?})", result, (client.context() as &Has<XSpanIdString>).get().clone());
        //  },

        Some("SetPwm") => {
            let result = core.run(client.set_pwm(56, 56, 56));
            println!("{:?} (X-Span-ID: {:?})", result, (client.context() as &Has<XSpanIdString>).get().clone());
         },

        Some("SetSleep") => {
            let result = core.run(client.set_sleep(56, 56, true));
            println!("{:?} (X-Span-ID: {:?})", result, (client.context() as &Has<XSpanIdString>).get().clone());
         },

        _ => {
            panic!("Invalid operation provided")
        }
    }
}

