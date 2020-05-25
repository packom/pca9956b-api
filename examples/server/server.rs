//! Main library entry point for pca9956b_api implementation.

#![allow(unused_imports)]

mod errors {
    error_chain::error_chain!{}
}

pub use self::errors::*;

use chrono;
use futures::{future, Future, Stream};
use hyper::server::conn::Http;
use hyper::service::MakeService as _;
use log::info;
use openssl::ssl::SslAcceptorBuilder;
use std::marker::PhantomData;
use std::net::SocketAddr;
use std::sync::{Arc, Mutex};
use swagger;
use swagger::{Has, XSpanIdString};
use swagger::auth::MakeAllowAllAuthenticator;
use swagger::EmptyContext;
use tokio::net::TcpListener;


#[cfg(not(any(target_os = "macos", target_os = "windows", target_os = "ios")))]
use tokio_openssl::SslAcceptorExt;
#[cfg(not(any(target_os = "macos", target_os = "windows", target_os = "ios")))]
use openssl::ssl::{SslAcceptor, SslFiletype, SslMethod};

use pca9956b_api::models;

#[cfg(not(any(target_os = "macos", target_os = "windows", target_os = "ios")))]
/// Builds an SSL implementation for Simple HTTPS from some hard-coded file names
pub fn create(addr: &str, https: bool) -> Box<dyn Future<Item = (), Error = ()> + Send> {
    let addr = addr.parse().expect("Failed to parse bind address");

    let server = Server::new();

    let service_fn = MakeService::new(server);

    let service_fn = MakeAllowAllAuthenticator::new(service_fn, "cosmo");

    let service_fn =
        pca9956b_api::server::context::MakeAddContext::<_, EmptyContext>::new(
            service_fn
        );

    if https {
        #[cfg(any(target_os = "macos", target_os = "windows", target_os = "ios"))]
        {
            unimplemented!("SSL is not implemented for the examples on MacOS, Windows or iOS");
        }

        #[cfg(not(any(target_os = "macos", target_os = "windows", target_os = "ios")))]
        {
            let mut ssl = SslAcceptor::mozilla_intermediate_v5(SslMethod::tls()).expect("Failed to create SSL Acceptor");

            // Server authentication
            ssl.set_private_key_file("examples/server-key.pem", SslFiletype::PEM).expect("Failed to set private key");
            ssl.set_certificate_chain_file("examples/server-chain.pem").expect("Failed to set cerificate chain");
            ssl.check_private_key().expect("Failed to check private key");

            let tls_acceptor = ssl.build();
            let service_fn = Arc::new(Mutex::new(service_fn));
            let tls_listener = TcpListener::bind(&addr).unwrap().incoming().for_each(move |tcp| {
                let addr = tcp.peer_addr().expect("Unable to get remote address");

                let service_fn = service_fn.clone();

                hyper::rt::spawn(tls_acceptor.accept_async(tcp).map_err(|_| ()).and_then(move |tls| {
                    let ms = {
                        let mut service_fn = service_fn.lock().unwrap();
                        service_fn.make_service(&addr)
                    };

                    ms.and_then(move |service| {
                        Http::new().serve_connection(tls, service)
                    }).map_err(|_| ())
                }));

                Ok(())
            }).map_err(|_| ());

            Box::new(tls_listener)
        }
    } else {
        // Using HTTP
        Box::new(hyper::server::Server::bind(&addr).serve(service_fn).map_err(|e| panic!("{:?}", e)))
    }
}

#[derive(Copy, Clone)]
pub struct Server<C> {
    marker: PhantomData<C>,
}

impl<C> Server<C> {
    pub fn new() -> Self {
        Server{marker: PhantomData}
    }
}


use pca9956b_api::{
    Api,
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
    SetSleepResponse,
};
use pca9956b_api::server::MakeService;

impl<C> Api<C> for Server<C> where C: Has<XSpanIdString>{
    fn clear_error(
        &self,
        bus_id: i32,
        addr: i32,
        context: &C) -> Box<dyn Future<Item=ClearErrorResponse, Error=ApiError> + Send>
    {
        let context = context.clone();
        info!("clear_error({}, {}) - X-Span-ID: {:?}", bus_id, addr, context.get().0.clone());
        Box::new(future::err("Generic failure".into()))
    }

    fn get_addr_enabled(
        &self,
        bus_id: i32,
        addr: i32,
        num: i32,
        context: &C) -> Box<dyn Future<Item=GetAddrEnabledResponse, Error=ApiError> + Send>
    {
        let context = context.clone();
        info!("get_addr_enabled({}, {}, {}) - X-Span-ID: {:?}", bus_id, addr, num, context.get().0.clone());
        Box::new(future::err("Generic failure".into()))
    }

    fn get_addr_info(
        &self,
        bus_id: i32,
        addr: i32,
        num: i32,
        context: &C) -> Box<dyn Future<Item=GetAddrInfoResponse, Error=ApiError> + Send>
    {
        let context = context.clone();
        info!("get_addr_info({}, {}, {}) - X-Span-ID: {:?}", bus_id, addr, num, context.get().0.clone());
        Box::new(future::err("Generic failure".into()))
    }

    fn get_addr_value(
        &self,
        bus_id: i32,
        addr: i32,
        num: i32,
        context: &C) -> Box<dyn Future<Item=GetAddrValueResponse, Error=ApiError> + Send>
    {
        let context = context.clone();
        info!("get_addr_value({}, {}, {}) - X-Span-ID: {:?}", bus_id, addr, num, context.get().0.clone());
        Box::new(future::err("Generic failure".into()))
    }

    fn get_api(
        &self,
        context: &C) -> Box<dyn Future<Item=GetApiResponse, Error=ApiError> + Send>
    {
        let context = context.clone();
        info!("get_api() - X-Span-ID: {:?}", context.get().0.clone());
        Box::new(future::err("Generic failure".into()))
    }

    fn get_config(
        &self,
        bus_id: i32,
        addr: i32,
        context: &C) -> Box<dyn Future<Item=GetConfigResponse, Error=ApiError> + Send>
    {
        let context = context.clone();
        info!("get_config({}, {}) - X-Span-ID: {:?}", bus_id, addr, context.get().0.clone());
        Box::new(future::err("Generic failure".into()))
    }

    fn get_current(
        &self,
        bus_id: i32,
        addr: i32,
        context: &C) -> Box<dyn Future<Item=GetCurrentResponse, Error=ApiError> + Send>
    {
        let context = context.clone();
        info!("get_current({}, {}) - X-Span-ID: {:?}", bus_id, addr, context.get().0.clone());
        Box::new(future::err("Generic failure".into()))
    }

    fn get_error(
        &self,
        bus_id: i32,
        addr: i32,
        context: &C) -> Box<dyn Future<Item=GetErrorResponse, Error=ApiError> + Send>
    {
        let context = context.clone();
        info!("get_error({}, {}) - X-Span-ID: {:?}", bus_id, addr, context.get().0.clone());
        Box::new(future::err("Generic failure".into()))
    }

    fn get_errors(
        &self,
        bus_id: i32,
        addr: i32,
        context: &C) -> Box<dyn Future<Item=GetErrorsResponse, Error=ApiError> + Send>
    {
        let context = context.clone();
        info!("get_errors({}, {}) - X-Span-ID: {:?}", bus_id, addr, context.get().0.clone());
        Box::new(future::err("Generic failure".into()))
    }

    fn get_freq(
        &self,
        bus_id: i32,
        addr: i32,
        context: &C) -> Box<dyn Future<Item=GetFreqResponse, Error=ApiError> + Send>
    {
        let context = context.clone();
        info!("get_freq({}, {}) - X-Span-ID: {:?}", bus_id, addr, context.get().0.clone());
        Box::new(future::err("Generic failure".into()))
    }

    fn get_group(
        &self,
        bus_id: i32,
        addr: i32,
        context: &C) -> Box<dyn Future<Item=GetGroupResponse, Error=ApiError> + Send>
    {
        let context = context.clone();
        info!("get_group({}, {}) - X-Span-ID: {:?}", bus_id, addr, context.get().0.clone());
        Box::new(future::err("Generic failure".into()))
    }

    fn get_led_current(
        &self,
        bus_id: i32,
        addr: i32,
        led: i32,
        context: &C) -> Box<dyn Future<Item=GetLedCurrentResponse, Error=ApiError> + Send>
    {
        let context = context.clone();
        info!("get_led_current({}, {}, {}) - X-Span-ID: {:?}", bus_id, addr, led, context.get().0.clone());
        Box::new(future::err("Generic failure".into()))
    }

    fn get_led_error(
        &self,
        bus_id: i32,
        addr: i32,
        led: i32,
        context: &C) -> Box<dyn Future<Item=GetLedErrorResponse, Error=ApiError> + Send>
    {
        let context = context.clone();
        info!("get_led_error({}, {}, {}) - X-Span-ID: {:?}", bus_id, addr, led, context.get().0.clone());
        Box::new(future::err("Generic failure".into()))
    }

    fn get_led_info(
        &self,
        bus_id: i32,
        addr: i32,
        led: i32,
        context: &C) -> Box<dyn Future<Item=GetLedInfoResponse, Error=ApiError> + Send>
    {
        let context = context.clone();
        info!("get_led_info({}, {}, {}) - X-Span-ID: {:?}", bus_id, addr, led, context.get().0.clone());
        Box::new(future::err("Generic failure".into()))
    }

    fn get_led_info_all(
        &self,
        bus_id: i32,
        addr: i32,
        context: &C) -> Box<dyn Future<Item=GetLedInfoAllResponse, Error=ApiError> + Send>
    {
        let context = context.clone();
        info!("get_led_info_all({}, {}) - X-Span-ID: {:?}", bus_id, addr, context.get().0.clone());
        Box::new(future::err("Generic failure".into()))
    }

    fn get_led_pwm(
        &self,
        bus_id: i32,
        addr: i32,
        led: i32,
        context: &C) -> Box<dyn Future<Item=GetLedPwmResponse, Error=ApiError> + Send>
    {
        let context = context.clone();
        info!("get_led_pwm({}, {}, {}) - X-Span-ID: {:?}", bus_id, addr, led, context.get().0.clone());
        Box::new(future::err("Generic failure".into()))
    }

    fn get_led_state(
        &self,
        bus_id: i32,
        addr: i32,
        led: i32,
        context: &C) -> Box<dyn Future<Item=GetLedStateResponse, Error=ApiError> + Send>
    {
        let context = context.clone();
        info!("get_led_state({}, {}, {}) - X-Span-ID: {:?}", bus_id, addr, led, context.get().0.clone());
        Box::new(future::err("Generic failure".into()))
    }

    fn get_offset(
        &self,
        bus_id: i32,
        addr: i32,
        context: &C) -> Box<dyn Future<Item=GetOffsetResponse, Error=ApiError> + Send>
    {
        let context = context.clone();
        info!("get_offset({}, {}) - X-Span-ID: {:?}", bus_id, addr, context.get().0.clone());
        Box::new(future::err("Generic failure".into()))
    }

    fn get_output_change(
        &self,
        bus_id: i32,
        addr: i32,
        context: &C) -> Box<dyn Future<Item=GetOutputChangeResponse, Error=ApiError> + Send>
    {
        let context = context.clone();
        info!("get_output_change({}, {}) - X-Span-ID: {:?}", bus_id, addr, context.get().0.clone());
        Box::new(future::err("Generic failure".into()))
    }

    fn get_over_temp(
        &self,
        bus_id: i32,
        addr: i32,
        context: &C) -> Box<dyn Future<Item=GetOverTempResponse, Error=ApiError> + Send>
    {
        let context = context.clone();
        info!("get_over_temp({}, {}) - X-Span-ID: {:?}", bus_id, addr, context.get().0.clone());
        Box::new(future::err("Generic failure".into()))
    }

    fn get_pwm(
        &self,
        bus_id: i32,
        addr: i32,
        context: &C) -> Box<dyn Future<Item=GetPwmResponse, Error=ApiError> + Send>
    {
        let context = context.clone();
        info!("get_pwm({}, {}) - X-Span-ID: {:?}", bus_id, addr, context.get().0.clone());
        Box::new(future::err("Generic failure".into()))
    }

    fn get_sleep(
        &self,
        bus_id: i32,
        addr: i32,
        context: &C) -> Box<dyn Future<Item=GetSleepResponse, Error=ApiError> + Send>
    {
        let context = context.clone();
        info!("get_sleep({}, {}) - X-Span-ID: {:?}", bus_id, addr, context.get().0.clone());
        Box::new(future::err("Generic failure".into()))
    }

    fn reset(
        &self,
        bus_id: i32,
        context: &C) -> Box<dyn Future<Item=ResetResponse, Error=ApiError> + Send>
    {
        let context = context.clone();
        info!("reset({}) - X-Span-ID: {:?}", bus_id, context.get().0.clone());
        Box::new(future::err("Generic failure".into()))
    }

    fn set_addr_enabled(
        &self,
        bus_id: i32,
        addr: i32,
        num: i32,
        enabled: bool,
        context: &C) -> Box<dyn Future<Item=SetAddrEnabledResponse, Error=ApiError> + Send>
    {
        let context = context.clone();
        info!("set_addr_enabled({}, {}, {}, {}) - X-Span-ID: {:?}", bus_id, addr, num, enabled, context.get().0.clone());
        Box::new(future::err("Generic failure".into()))
    }

    fn set_addr_value(
        &self,
        bus_id: i32,
        addr: i32,
        num: i32,
        addr_val: i32,
        context: &C) -> Box<dyn Future<Item=SetAddrValueResponse, Error=ApiError> + Send>
    {
        let context = context.clone();
        info!("set_addr_value({}, {}, {}, {}) - X-Span-ID: {:?}", bus_id, addr, num, addr_val, context.get().0.clone());
        Box::new(future::err("Generic failure".into()))
    }

    fn set_config(
        &self,
        bus_id: i32,
        addr: i32,
        config: models::Config,
        context: &C) -> Box<dyn Future<Item=SetConfigResponse, Error=ApiError> + Send>
    {
        let context = context.clone();
        info!("set_config({}, {}, {:?}) - X-Span-ID: {:?}", bus_id, addr, config, context.get().0.clone());
        Box::new(future::err("Generic failure".into()))
    }

    fn set_current(
        &self,
        bus_id: i32,
        addr: i32,
        current: i32,
        context: &C) -> Box<dyn Future<Item=SetCurrentResponse, Error=ApiError> + Send>
    {
        let context = context.clone();
        info!("set_current({}, {}, {}) - X-Span-ID: {:?}", bus_id, addr, current, context.get().0.clone());
        Box::new(future::err("Generic failure".into()))
    }

    fn set_freq(
        &self,
        bus_id: i32,
        addr: i32,
        freq: i32,
        context: &C) -> Box<dyn Future<Item=SetFreqResponse, Error=ApiError> + Send>
    {
        let context = context.clone();
        info!("set_freq({}, {}, {}) - X-Span-ID: {:?}", bus_id, addr, freq, context.get().0.clone());
        Box::new(future::err("Generic failure".into()))
    }

    fn set_group(
        &self,
        bus_id: i32,
        addr: i32,
        group: models::Group,
        context: &C) -> Box<dyn Future<Item=SetGroupResponse, Error=ApiError> + Send>
    {
        let context = context.clone();
        info!("set_group({}, {}, {:?}) - X-Span-ID: {:?}", bus_id, addr, group, context.get().0.clone());
        Box::new(future::err("Generic failure".into()))
    }

    fn set_led_current(
        &self,
        bus_id: i32,
        addr: i32,
        led: i32,
        current: i32,
        context: &C) -> Box<dyn Future<Item=SetLedCurrentResponse, Error=ApiError> + Send>
    {
        let context = context.clone();
        info!("set_led_current({}, {}, {}, {}) - X-Span-ID: {:?}", bus_id, addr, led, current, context.get().0.clone());
        Box::new(future::err("Generic failure".into()))
    }

    fn set_led_error(
        &self,
        bus_id: i32,
        addr: i32,
        led: i32,
        error: models::LedError,
        context: &C) -> Box<dyn Future<Item=SetLedErrorResponse, Error=ApiError> + Send>
    {
        let context = context.clone();
        info!("set_led_error({}, {}, {}, {:?}) - X-Span-ID: {:?}", bus_id, addr, led, error, context.get().0.clone());
        Box::new(future::err("Generic failure".into()))
    }

    fn set_led_info(
        &self,
        bus_id: i32,
        addr: i32,
        led: i32,
        led_info: models::LedInfo,
        context: &C) -> Box<dyn Future<Item=SetLedInfoResponse, Error=ApiError> + Send>
    {
        let context = context.clone();
        info!("set_led_info({}, {}, {}, {:?}) - X-Span-ID: {:?}", bus_id, addr, led, led_info, context.get().0.clone());
        Box::new(future::err("Generic failure".into()))
    }

    fn set_led_info_all(
        &self,
        bus_id: i32,
        addr: i32,
        led_info_array: models::LedInfoArray,
        context: &C) -> Box<dyn Future<Item=SetLedInfoAllResponse, Error=ApiError> + Send>
    {
        let context = context.clone();
        info!("set_led_info_all({}, {}, {:?}) - X-Span-ID: {:?}", bus_id, addr, led_info_array, context.get().0.clone());
        Box::new(future::err("Generic failure".into()))
    }

    fn set_led_pwm(
        &self,
        bus_id: i32,
        addr: i32,
        led: i32,
        pwm: i32,
        context: &C) -> Box<dyn Future<Item=SetLedPwmResponse, Error=ApiError> + Send>
    {
        let context = context.clone();
        info!("set_led_pwm({}, {}, {}, {}) - X-Span-ID: {:?}", bus_id, addr, led, pwm, context.get().0.clone());
        Box::new(future::err("Generic failure".into()))
    }

    fn set_led_state(
        &self,
        bus_id: i32,
        addr: i32,
        led: i32,
        state: models::LedState,
        context: &C) -> Box<dyn Future<Item=SetLedStateResponse, Error=ApiError> + Send>
    {
        let context = context.clone();
        info!("set_led_state({}, {}, {}, {:?}) - X-Span-ID: {:?}", bus_id, addr, led, state, context.get().0.clone());
        Box::new(future::err("Generic failure".into()))
    }

    fn set_offset(
        &self,
        bus_id: i32,
        addr: i32,
        offset: i32,
        context: &C) -> Box<dyn Future<Item=SetOffsetResponse, Error=ApiError> + Send>
    {
        let context = context.clone();
        info!("set_offset({}, {}, {}) - X-Span-ID: {:?}", bus_id, addr, offset, context.get().0.clone());
        Box::new(future::err("Generic failure".into()))
    }

    fn set_output_change(
        &self,
        bus_id: i32,
        addr: i32,
        output_change: models::OutputChange,
        context: &C) -> Box<dyn Future<Item=SetOutputChangeResponse, Error=ApiError> + Send>
    {
        let context = context.clone();
        info!("set_output_change({}, {}, {:?}) - X-Span-ID: {:?}", bus_id, addr, output_change, context.get().0.clone());
        Box::new(future::err("Generic failure".into()))
    }

    fn set_pwm(
        &self,
        bus_id: i32,
        addr: i32,
        pwm: i32,
        context: &C) -> Box<dyn Future<Item=SetPwmResponse, Error=ApiError> + Send>
    {
        let context = context.clone();
        info!("set_pwm({}, {}, {}) - X-Span-ID: {:?}", bus_id, addr, pwm, context.get().0.clone());
        Box::new(future::err("Generic failure".into()))
    }

    fn set_sleep(
        &self,
        bus_id: i32,
        addr: i32,
        sleep: bool,
        context: &C) -> Box<dyn Future<Item=SetSleepResponse, Error=ApiError> + Send>
    {
        let context = context.clone();
        info!("set_sleep({}, {}, {}) - X-Span-ID: {:?}", bus_id, addr, sleep, context.get().0.clone());
        Box::new(future::err("Generic failure".into()))
    }

}
