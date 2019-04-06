//! Server implementation of pca9956b_api.

#![allow(unused_imports)]

use futures::{self, Future};
use chrono;
use std::collections::HashMap;
use std::marker::PhantomData;

use swagger;
use swagger::{Has, XSpanIdString};

use pca9956b_api::{Api, ApiError,
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
use pca9956b_api::models;

#[derive(Copy, Clone)]
pub struct Server<C> {
    marker: PhantomData<C>,
}

impl<C> Server<C> {
    pub fn new() -> Self {
        Server{marker: PhantomData}
    }
}

impl<C> Api<C> for Server<C> where C: Has<XSpanIdString>{


    fn clear_error(&self, bus_id: i32, addr: i32, context: &C) -> Box<Future<Item=ClearErrorResponse, Error=ApiError>> {
        let context = context.clone();
        println!("clear_error({}, {}) - X-Span-ID: {:?}", bus_id, addr, context.get().0.clone());
        Box::new(futures::failed("Generic failure".into()))
    }


    fn get_addr_enabled(&self, bus_id: i32, addr: i32, num: i32, context: &C) -> Box<Future<Item=GetAddrEnabledResponse, Error=ApiError>> {
        let context = context.clone();
        println!("get_addr_enabled({}, {}, {}) - X-Span-ID: {:?}", bus_id, addr, num, context.get().0.clone());
        Box::new(futures::failed("Generic failure".into()))
    }


    fn get_addr_info(&self, bus_id: i32, addr: i32, num: i32, context: &C) -> Box<Future<Item=GetAddrInfoResponse, Error=ApiError>> {
        let context = context.clone();
        println!("get_addr_info({}, {}, {}) - X-Span-ID: {:?}", bus_id, addr, num, context.get().0.clone());
        Box::new(futures::failed("Generic failure".into()))
    }


    fn get_addr_value(&self, bus_id: i32, addr: i32, num: i32, context: &C) -> Box<Future<Item=GetAddrValueResponse, Error=ApiError>> {
        let context = context.clone();
        println!("get_addr_value({}, {}, {}) - X-Span-ID: {:?}", bus_id, addr, num, context.get().0.clone());
        Box::new(futures::failed("Generic failure".into()))
    }


    fn get_api(&self, context: &C) -> Box<Future<Item=GetApiResponse, Error=ApiError>> {
        let context = context.clone();
        println!("get_api() - X-Span-ID: {:?}", context.get().0.clone());
        Box::new(futures::failed("Generic failure".into()))
    }


    fn get_config(&self, bus_id: i32, addr: i32, context: &C) -> Box<Future<Item=GetConfigResponse, Error=ApiError>> {
        let context = context.clone();
        println!("get_config({}, {}) - X-Span-ID: {:?}", bus_id, addr, context.get().0.clone());
        Box::new(futures::failed("Generic failure".into()))
    }


    fn get_current(&self, bus_id: i32, addr: i32, context: &C) -> Box<Future<Item=GetCurrentResponse, Error=ApiError>> {
        let context = context.clone();
        println!("get_current({}, {}) - X-Span-ID: {:?}", bus_id, addr, context.get().0.clone());
        Box::new(futures::failed("Generic failure".into()))
    }


    fn get_error(&self, bus_id: i32, addr: i32, context: &C) -> Box<Future<Item=GetErrorResponse, Error=ApiError>> {
        let context = context.clone();
        println!("get_error({}, {}) - X-Span-ID: {:?}", bus_id, addr, context.get().0.clone());
        Box::new(futures::failed("Generic failure".into()))
    }


    fn get_errors(&self, bus_id: i32, addr: i32, context: &C) -> Box<Future<Item=GetErrorsResponse, Error=ApiError>> {
        let context = context.clone();
        println!("get_errors({}, {}) - X-Span-ID: {:?}", bus_id, addr, context.get().0.clone());
        Box::new(futures::failed("Generic failure".into()))
    }


    fn get_freq(&self, bus_id: i32, addr: i32, context: &C) -> Box<Future<Item=GetFreqResponse, Error=ApiError>> {
        let context = context.clone();
        println!("get_freq({}, {}) - X-Span-ID: {:?}", bus_id, addr, context.get().0.clone());
        Box::new(futures::failed("Generic failure".into()))
    }


    fn get_group(&self, bus_id: i32, addr: i32, context: &C) -> Box<Future<Item=GetGroupResponse, Error=ApiError>> {
        let context = context.clone();
        println!("get_group({}, {}) - X-Span-ID: {:?}", bus_id, addr, context.get().0.clone());
        Box::new(futures::failed("Generic failure".into()))
    }


    fn get_led_current(&self, bus_id: i32, addr: i32, led: i32, context: &C) -> Box<Future<Item=GetLedCurrentResponse, Error=ApiError>> {
        let context = context.clone();
        println!("get_led_current({}, {}, {}) - X-Span-ID: {:?}", bus_id, addr, led, context.get().0.clone());
        Box::new(futures::failed("Generic failure".into()))
    }


    fn get_led_error(&self, bus_id: i32, addr: i32, led: i32, context: &C) -> Box<Future<Item=GetLedErrorResponse, Error=ApiError>> {
        let context = context.clone();
        println!("get_led_error({}, {}, {}) - X-Span-ID: {:?}", bus_id, addr, led, context.get().0.clone());
        Box::new(futures::failed("Generic failure".into()))
    }


    fn get_led_info(&self, bus_id: i32, addr: i32, led: i32, context: &C) -> Box<Future<Item=GetLedInfoResponse, Error=ApiError>> {
        let context = context.clone();
        println!("get_led_info({}, {}, {}) - X-Span-ID: {:?}", bus_id, addr, led, context.get().0.clone());
        Box::new(futures::failed("Generic failure".into()))
    }


    fn get_led_info_all(&self, bus_id: i32, addr: i32, context: &C) -> Box<Future<Item=GetLedInfoAllResponse, Error=ApiError>> {
        let context = context.clone();
        println!("get_led_info_all({}, {}) - X-Span-ID: {:?}", bus_id, addr, context.get().0.clone());
        Box::new(futures::failed("Generic failure".into()))
    }


    fn get_led_pwm(&self, bus_id: i32, addr: i32, led: i32, context: &C) -> Box<Future<Item=GetLedPwmResponse, Error=ApiError>> {
        let context = context.clone();
        println!("get_led_pwm({}, {}, {}) - X-Span-ID: {:?}", bus_id, addr, led, context.get().0.clone());
        Box::new(futures::failed("Generic failure".into()))
    }


    fn get_led_state(&self, bus_id: i32, addr: i32, led: i32, context: &C) -> Box<Future<Item=GetLedStateResponse, Error=ApiError>> {
        let context = context.clone();
        println!("get_led_state({}, {}, {}) - X-Span-ID: {:?}", bus_id, addr, led, context.get().0.clone());
        Box::new(futures::failed("Generic failure".into()))
    }


    fn get_offset(&self, bus_id: i32, addr: i32, context: &C) -> Box<Future<Item=GetOffsetResponse, Error=ApiError>> {
        let context = context.clone();
        println!("get_offset({}, {}) - X-Span-ID: {:?}", bus_id, addr, context.get().0.clone());
        Box::new(futures::failed("Generic failure".into()))
    }


    fn get_output_change(&self, bus_id: i32, addr: i32, context: &C) -> Box<Future<Item=GetOutputChangeResponse, Error=ApiError>> {
        let context = context.clone();
        println!("get_output_change({}, {}) - X-Span-ID: {:?}", bus_id, addr, context.get().0.clone());
        Box::new(futures::failed("Generic failure".into()))
    }


    fn get_over_temp(&self, bus_id: i32, addr: i32, context: &C) -> Box<Future<Item=GetOverTempResponse, Error=ApiError>> {
        let context = context.clone();
        println!("get_over_temp({}, {}) - X-Span-ID: {:?}", bus_id, addr, context.get().0.clone());
        Box::new(futures::failed("Generic failure".into()))
    }


    fn get_pwm(&self, bus_id: i32, addr: i32, context: &C) -> Box<Future<Item=GetPwmResponse, Error=ApiError>> {
        let context = context.clone();
        println!("get_pwm({}, {}) - X-Span-ID: {:?}", bus_id, addr, context.get().0.clone());
        Box::new(futures::failed("Generic failure".into()))
    }


    fn get_sleep(&self, bus_id: i32, addr: i32, context: &C) -> Box<Future<Item=GetSleepResponse, Error=ApiError>> {
        let context = context.clone();
        println!("get_sleep({}, {}) - X-Span-ID: {:?}", bus_id, addr, context.get().0.clone());
        Box::new(futures::failed("Generic failure".into()))
    }


    fn reset(&self, bus_id: i32, context: &C) -> Box<Future<Item=ResetResponse, Error=ApiError>> {
        let context = context.clone();
        println!("reset({}) - X-Span-ID: {:?}", bus_id, context.get().0.clone());
        Box::new(futures::failed("Generic failure".into()))
    }


    fn set_addr_enabled(&self, bus_id: i32, addr: i32, num: i32, enabled: bool, context: &C) -> Box<Future<Item=SetAddrEnabledResponse, Error=ApiError>> {
        let context = context.clone();
        println!("set_addr_enabled({}, {}, {}, {}) - X-Span-ID: {:?}", bus_id, addr, num, enabled, context.get().0.clone());
        Box::new(futures::failed("Generic failure".into()))
    }


    fn set_addr_value(&self, bus_id: i32, addr: i32, num: i32, addr_val: i32, context: &C) -> Box<Future<Item=SetAddrValueResponse, Error=ApiError>> {
        let context = context.clone();
        println!("set_addr_value({}, {}, {}, {}) - X-Span-ID: {:?}", bus_id, addr, num, addr_val, context.get().0.clone());
        Box::new(futures::failed("Generic failure".into()))
    }


    fn set_config(&self, bus_id: i32, addr: i32, config: models::Config, context: &C) -> Box<Future<Item=SetConfigResponse, Error=ApiError>> {
        let context = context.clone();
        println!("set_config({}, {}, {:?}) - X-Span-ID: {:?}", bus_id, addr, config, context.get().0.clone());
        Box::new(futures::failed("Generic failure".into()))
    }


    fn set_current(&self, bus_id: i32, addr: i32, current: i32, context: &C) -> Box<Future<Item=SetCurrentResponse, Error=ApiError>> {
        let context = context.clone();
        println!("set_current({}, {}, {}) - X-Span-ID: {:?}", bus_id, addr, current, context.get().0.clone());
        Box::new(futures::failed("Generic failure".into()))
    }


    fn set_freq(&self, bus_id: i32, addr: i32, freq: i32, context: &C) -> Box<Future<Item=SetFreqResponse, Error=ApiError>> {
        let context = context.clone();
        println!("set_freq({}, {}, {}) - X-Span-ID: {:?}", bus_id, addr, freq, context.get().0.clone());
        Box::new(futures::failed("Generic failure".into()))
    }


    fn set_group(&self, bus_id: i32, addr: i32, group: models::Group, context: &C) -> Box<Future<Item=SetGroupResponse, Error=ApiError>> {
        let context = context.clone();
        println!("set_group({}, {}, {:?}) - X-Span-ID: {:?}", bus_id, addr, group, context.get().0.clone());
        Box::new(futures::failed("Generic failure".into()))
    }


    fn set_led_current(&self, bus_id: i32, addr: i32, led: i32, current: i32, context: &C) -> Box<Future<Item=SetLedCurrentResponse, Error=ApiError>> {
        let context = context.clone();
        println!("set_led_current({}, {}, {}, {}) - X-Span-ID: {:?}", bus_id, addr, led, current, context.get().0.clone());
        Box::new(futures::failed("Generic failure".into()))
    }


    fn set_led_error(&self, bus_id: i32, addr: i32, led: i32, error: models::LedError, context: &C) -> Box<Future<Item=SetLedErrorResponse, Error=ApiError>> {
        let context = context.clone();
        println!("set_led_error({}, {}, {}, {:?}) - X-Span-ID: {:?}", bus_id, addr, led, error, context.get().0.clone());
        Box::new(futures::failed("Generic failure".into()))
    }


    fn set_led_info(&self, bus_id: i32, addr: i32, led: i32, led_info: models::LedInfo, context: &C) -> Box<Future<Item=SetLedInfoResponse, Error=ApiError>> {
        let context = context.clone();
        println!("set_led_info({}, {}, {}, {:?}) - X-Span-ID: {:?}", bus_id, addr, led, led_info, context.get().0.clone());
        Box::new(futures::failed("Generic failure".into()))
    }


    fn set_led_info_all(&self, bus_id: i32, addr: i32, led_info: &Vec<models::LedInfo>, context: &C) -> Box<Future<Item=SetLedInfoAllResponse, Error=ApiError>> {
        let context = context.clone();
        println!("set_led_info_all({}, {}, {:?}) - X-Span-ID: {:?}", bus_id, addr, led_info, context.get().0.clone());
        Box::new(futures::failed("Generic failure".into()))
    }


    fn set_led_pwm(&self, bus_id: i32, addr: i32, led: i32, pwm: i32, context: &C) -> Box<Future<Item=SetLedPwmResponse, Error=ApiError>> {
        let context = context.clone();
        println!("set_led_pwm({}, {}, {}, {}) - X-Span-ID: {:?}", bus_id, addr, led, pwm, context.get().0.clone());
        Box::new(futures::failed("Generic failure".into()))
    }


    fn set_led_state(&self, bus_id: i32, addr: i32, led: i32, state: models::LedState, context: &C) -> Box<Future<Item=SetLedStateResponse, Error=ApiError>> {
        let context = context.clone();
        println!("set_led_state({}, {}, {}, {:?}) - X-Span-ID: {:?}", bus_id, addr, led, state, context.get().0.clone());
        Box::new(futures::failed("Generic failure".into()))
    }


    fn set_offset(&self, bus_id: i32, addr: i32, offset: i32, context: &C) -> Box<Future<Item=SetOffsetResponse, Error=ApiError>> {
        let context = context.clone();
        println!("set_offset({}, {}, {}) - X-Span-ID: {:?}", bus_id, addr, offset, context.get().0.clone());
        Box::new(futures::failed("Generic failure".into()))
    }


    fn set_output_change(&self, bus_id: i32, addr: i32, output_change: models::OutputChange, context: &C) -> Box<Future<Item=SetOutputChangeResponse, Error=ApiError>> {
        let context = context.clone();
        println!("set_output_change({}, {}, {:?}) - X-Span-ID: {:?}", bus_id, addr, output_change, context.get().0.clone());
        Box::new(futures::failed("Generic failure".into()))
    }


    fn set_pwm(&self, bus_id: i32, addr: i32, pwm: i32, context: &C) -> Box<Future<Item=SetPwmResponse, Error=ApiError>> {
        let context = context.clone();
        println!("set_pwm({}, {}, {}) - X-Span-ID: {:?}", bus_id, addr, pwm, context.get().0.clone());
        Box::new(futures::failed("Generic failure".into()))
    }


    fn set_sleep(&self, bus_id: i32, addr: i32, sleep: bool, context: &C) -> Box<Future<Item=SetSleepResponse, Error=ApiError>> {
        let context = context.clone();
        println!("set_sleep({}, {}, {}) - X-Span-ID: {:?}", bus_id, addr, sleep, context.get().0.clone());
        Box::new(futures::failed("Generic failure".into()))
    }

}
