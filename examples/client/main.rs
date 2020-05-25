#![allow(missing_docs, unused_variables, trivial_casts)]


#[allow(unused_imports)]
use futures::{Future, future, Stream, stream};
#[allow(unused_imports)]
use pca9956b_api::{Api, ApiNoContext, Client, ContextWrapperExt, models,
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

#[allow(unused_imports)]
use log::info;

// swagger::Has may be unused if there are no examples
#[allow(unused_imports)]
use swagger::{ContextBuilder, EmptyContext, XSpanIdString, Has, Push, AuthData};

// rt may be unused if there are no examples
#[allow(unused_mut)]
fn main() {
    env_logger::init();

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
            .default_value("8080")
            .help("Port to contact"))
        .get_matches();

    let is_https = matches.is_present("https");
    let base_url = format!("{}://{}:{}",
                           if is_https { "https" } else { "http" },
                           matches.value_of("host").unwrap(),
                           matches.value_of("port").unwrap());

    let client = if matches.is_present("https") {
        // Using Simple HTTPS
        Client::try_new_https(&base_url)
            .expect("Failed to create HTTPS client")
    } else {
        // Using HTTP
        Client::try_new_http(
            &base_url)
            .expect("Failed to create HTTP client")
    };

    let context: swagger::make_context_ty!(ContextBuilder, EmptyContext, Option<AuthData>, XSpanIdString) =
        swagger::make_context!(ContextBuilder, EmptyContext, None as Option<AuthData>, XSpanIdString::default());

    let client = client.with_context(context);

    let mut rt = tokio::runtime::Runtime::new().unwrap();

    match matches.value_of("operation") {
        Some("ClearError") => {
            let result = rt.block_on(client.clear_error(
                  56,
                  56
            ));
            info!("{:?} (X-Span-ID: {:?})", result, (client.context() as &dyn Has<XSpanIdString>).get().clone());
        },
        Some("GetAddrEnabled") => {
            let result = rt.block_on(client.get_addr_enabled(
                  56,
                  56,
                  56
            ));
            info!("{:?} (X-Span-ID: {:?})", result, (client.context() as &dyn Has<XSpanIdString>).get().clone());
        },
        Some("GetAddrInfo") => {
            let result = rt.block_on(client.get_addr_info(
                  56,
                  56,
                  56
            ));
            info!("{:?} (X-Span-ID: {:?})", result, (client.context() as &dyn Has<XSpanIdString>).get().clone());
        },
        Some("GetAddrValue") => {
            let result = rt.block_on(client.get_addr_value(
                  56,
                  56,
                  56
            ));
            info!("{:?} (X-Span-ID: {:?})", result, (client.context() as &dyn Has<XSpanIdString>).get().clone());
        },
        Some("GetApi") => {
            let result = rt.block_on(client.get_api(
            ));
            info!("{:?} (X-Span-ID: {:?})", result, (client.context() as &dyn Has<XSpanIdString>).get().clone());
        },
        Some("GetConfig") => {
            let result = rt.block_on(client.get_config(
                  56,
                  56
            ));
            info!("{:?} (X-Span-ID: {:?})", result, (client.context() as &dyn Has<XSpanIdString>).get().clone());
        },
        Some("GetCurrent") => {
            let result = rt.block_on(client.get_current(
                  56,
                  56
            ));
            info!("{:?} (X-Span-ID: {:?})", result, (client.context() as &dyn Has<XSpanIdString>).get().clone());
        },
        Some("GetError") => {
            let result = rt.block_on(client.get_error(
                  56,
                  56
            ));
            info!("{:?} (X-Span-ID: {:?})", result, (client.context() as &dyn Has<XSpanIdString>).get().clone());
        },
        Some("GetErrors") => {
            let result = rt.block_on(client.get_errors(
                  56,
                  56
            ));
            info!("{:?} (X-Span-ID: {:?})", result, (client.context() as &dyn Has<XSpanIdString>).get().clone());
        },
        Some("GetFreq") => {
            let result = rt.block_on(client.get_freq(
                  56,
                  56
            ));
            info!("{:?} (X-Span-ID: {:?})", result, (client.context() as &dyn Has<XSpanIdString>).get().clone());
        },
        Some("GetGroup") => {
            let result = rt.block_on(client.get_group(
                  56,
                  56
            ));
            info!("{:?} (X-Span-ID: {:?})", result, (client.context() as &dyn Has<XSpanIdString>).get().clone());
        },
        Some("GetLedCurrent") => {
            let result = rt.block_on(client.get_led_current(
                  56,
                  56,
                  56
            ));
            info!("{:?} (X-Span-ID: {:?})", result, (client.context() as &dyn Has<XSpanIdString>).get().clone());
        },
        Some("GetLedError") => {
            let result = rt.block_on(client.get_led_error(
                  56,
                  56,
                  56
            ));
            info!("{:?} (X-Span-ID: {:?})", result, (client.context() as &dyn Has<XSpanIdString>).get().clone());
        },
        Some("GetLedInfo") => {
            let result = rt.block_on(client.get_led_info(
                  56,
                  56,
                  56
            ));
            info!("{:?} (X-Span-ID: {:?})", result, (client.context() as &dyn Has<XSpanIdString>).get().clone());
        },
        Some("GetLedInfoAll") => {
            let result = rt.block_on(client.get_led_info_all(
                  56,
                  56
            ));
            info!("{:?} (X-Span-ID: {:?})", result, (client.context() as &dyn Has<XSpanIdString>).get().clone());
        },
        Some("GetLedPwm") => {
            let result = rt.block_on(client.get_led_pwm(
                  56,
                  56,
                  56
            ));
            info!("{:?} (X-Span-ID: {:?})", result, (client.context() as &dyn Has<XSpanIdString>).get().clone());
        },
        Some("GetLedState") => {
            let result = rt.block_on(client.get_led_state(
                  56,
                  56,
                  56
            ));
            info!("{:?} (X-Span-ID: {:?})", result, (client.context() as &dyn Has<XSpanIdString>).get().clone());
        },
        Some("GetOffset") => {
            let result = rt.block_on(client.get_offset(
                  56,
                  56
            ));
            info!("{:?} (X-Span-ID: {:?})", result, (client.context() as &dyn Has<XSpanIdString>).get().clone());
        },
        Some("GetOutputChange") => {
            let result = rt.block_on(client.get_output_change(
                  56,
                  56
            ));
            info!("{:?} (X-Span-ID: {:?})", result, (client.context() as &dyn Has<XSpanIdString>).get().clone());
        },
        Some("GetOverTemp") => {
            let result = rt.block_on(client.get_over_temp(
                  56,
                  56
            ));
            info!("{:?} (X-Span-ID: {:?})", result, (client.context() as &dyn Has<XSpanIdString>).get().clone());
        },
        Some("GetPwm") => {
            let result = rt.block_on(client.get_pwm(
                  56,
                  56
            ));
            info!("{:?} (X-Span-ID: {:?})", result, (client.context() as &dyn Has<XSpanIdString>).get().clone());
        },
        Some("GetSleep") => {
            let result = rt.block_on(client.get_sleep(
                  56,
                  56
            ));
            info!("{:?} (X-Span-ID: {:?})", result, (client.context() as &dyn Has<XSpanIdString>).get().clone());
        },
        Some("Reset") => {
            let result = rt.block_on(client.reset(
                  56
            ));
            info!("{:?} (X-Span-ID: {:?})", result, (client.context() as &dyn Has<XSpanIdString>).get().clone());
        },
        Some("SetAddrEnabled") => {
            let result = rt.block_on(client.set_addr_enabled(
                  56,
                  56,
                  56,
                  true
            ));
            info!("{:?} (X-Span-ID: {:?})", result, (client.context() as &dyn Has<XSpanIdString>).get().clone());
        },
        Some("SetAddrValue") => {
            let result = rt.block_on(client.set_addr_value(
                  56,
                  56,
                  56,
                  56
            ));
            info!("{:?} (X-Span-ID: {:?})", result, (client.context() as &dyn Has<XSpanIdString>).get().clone());
        },
        /* Disabled because there's no example.
        Some("SetConfig") => {
            let result = rt.block_on(client.set_config(
                  56,
                  56,
                  ???
            ));
            info!("{:?} (X-Span-ID: {:?})", result, (client.context() as &dyn Has<XSpanIdString>).get().clone());
        },
        */
        Some("SetCurrent") => {
            let result = rt.block_on(client.set_current(
                  56,
                  56,
                  56
            ));
            info!("{:?} (X-Span-ID: {:?})", result, (client.context() as &dyn Has<XSpanIdString>).get().clone());
        },
        Some("SetFreq") => {
            let result = rt.block_on(client.set_freq(
                  56,
                  56,
                  56
            ));
            info!("{:?} (X-Span-ID: {:?})", result, (client.context() as &dyn Has<XSpanIdString>).get().clone());
        },
        /* Disabled because there's no example.
        Some("SetGroup") => {
            let result = rt.block_on(client.set_group(
                  56,
                  56,
                  ???
            ));
            info!("{:?} (X-Span-ID: {:?})", result, (client.context() as &dyn Has<XSpanIdString>).get().clone());
        },
        */
        Some("SetLedCurrent") => {
            let result = rt.block_on(client.set_led_current(
                  56,
                  56,
                  56,
                  56
            ));
            info!("{:?} (X-Span-ID: {:?})", result, (client.context() as &dyn Has<XSpanIdString>).get().clone());
        },
        /* Disabled because there's no example.
        Some("SetLedError") => {
            let result = rt.block_on(client.set_led_error(
                  56,
                  56,
                  56,
                  ???
            ));
            info!("{:?} (X-Span-ID: {:?})", result, (client.context() as &dyn Has<XSpanIdString>).get().clone());
        },
        */
        /* Disabled because there's no example.
        Some("SetLedInfo") => {
            let result = rt.block_on(client.set_led_info(
                  56,
                  56,
                  56,
                  ???
            ));
            info!("{:?} (X-Span-ID: {:?})", result, (client.context() as &dyn Has<XSpanIdString>).get().clone());
        },
        */
        /* Disabled because there's no example.
        Some("SetLedInfoAll") => {
            let result = rt.block_on(client.set_led_info_all(
                  56,
                  56,
                  ???
            ));
            info!("{:?} (X-Span-ID: {:?})", result, (client.context() as &dyn Has<XSpanIdString>).get().clone());
        },
        */
        Some("SetLedPwm") => {
            let result = rt.block_on(client.set_led_pwm(
                  56,
                  56,
                  56,
                  56
            ));
            info!("{:?} (X-Span-ID: {:?})", result, (client.context() as &dyn Has<XSpanIdString>).get().clone());
        },
        /* Disabled because there's no example.
        Some("SetLedState") => {
            let result = rt.block_on(client.set_led_state(
                  56,
                  56,
                  56,
                  ???
            ));
            info!("{:?} (X-Span-ID: {:?})", result, (client.context() as &dyn Has<XSpanIdString>).get().clone());
        },
        */
        Some("SetOffset") => {
            let result = rt.block_on(client.set_offset(
                  56,
                  56,
                  56
            ));
            info!("{:?} (X-Span-ID: {:?})", result, (client.context() as &dyn Has<XSpanIdString>).get().clone());
        },
        /* Disabled because there's no example.
        Some("SetOutputChange") => {
            let result = rt.block_on(client.set_output_change(
                  56,
                  56,
                  ???
            ));
            info!("{:?} (X-Span-ID: {:?})", result, (client.context() as &dyn Has<XSpanIdString>).get().clone());
        },
        */
        Some("SetPwm") => {
            let result = rt.block_on(client.set_pwm(
                  56,
                  56,
                  56
            ));
            info!("{:?} (X-Span-ID: {:?})", result, (client.context() as &dyn Has<XSpanIdString>).get().clone());
        },
        Some("SetSleep") => {
            let result = rt.block_on(client.set_sleep(
                  56,
                  56,
                  true
            ));
            info!("{:?} (X-Span-ID: {:?})", result, (client.context() as &dyn Has<XSpanIdString>).get().clone());
        },
        _ => {
            panic!("Invalid operation provided")
        }
    }
}
