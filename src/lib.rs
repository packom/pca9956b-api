#![allow(missing_docs, trivial_casts, unused_variables, unused_mut, unused_imports, unused_extern_crates, non_camel_case_types)]
extern crate serde;
#[macro_use]
extern crate serde_derive;
extern crate serde_json;


extern crate futures;
extern crate chrono;
#[macro_use]
extern crate lazy_static;
#[macro_use]
extern crate log;

// Logically this should be in the client and server modules, but rust doesn't allow `macro_use` from a module.
#[cfg(any(feature = "client", feature = "server"))]
#[macro_use]
extern crate hyper;

extern crate swagger;

#[macro_use]
extern crate url;

use futures::Stream;
use std::io::Error;

#[allow(unused_imports)]
use std::collections::HashMap;

pub use futures::Future;

#[cfg(any(feature = "client", feature = "server"))]
mod mimetypes;

pub use swagger::{ApiError, ContextWrapper};

pub const BASE_PATH: &'static str = "";
pub const API_VERSION: &'static str = "0.1.1";


#[derive(Debug, PartialEq)]
pub enum ClearErrorResponse {
    /// OK
    OK ,
    /// Bad Request
    BadRequest ( models::BadRequest ) ,
    /// Operation Failed
    OperationFailed ( models::OpError ) ,
}

#[derive(Debug, PartialEq)]
pub enum GetAddrEnabledResponse {
    /// OK
    OK ( bool ) ,
    /// Bad Request
    BadRequest ( models::BadRequest ) ,
    /// Operation Failed
    OperationFailed ( models::OpError ) ,
}

#[derive(Debug, PartialEq)]
pub enum GetAddrInfoResponse {
    /// OK
    OK ( models::AddrInfo ) ,
    /// Bad Request
    BadRequest ( models::BadRequest ) ,
    /// Operation Failed
    OperationFailed ( models::OpError ) ,
}

#[derive(Debug, PartialEq)]
pub enum GetAddrValueResponse {
    /// OK
    OK ( i32 ) ,
    /// Bad Request
    BadRequest ( models::BadRequest ) ,
    /// Operation Failed
    OperationFailed ( models::OpError ) ,
}

#[derive(Debug, PartialEq)]
pub enum GetApiResponse {
    /// OK
    OK ( String ) ,
    /// File not found
    FileNotFound ( String ) ,
}

#[derive(Debug, PartialEq)]
pub enum GetConfigResponse {
    /// OK
    OK ( models::Config ) ,
    /// Bad Request
    BadRequest ( models::BadRequest ) ,
    /// Operation Failed
    OperationFailed ( models::OpError ) ,
}

#[derive(Debug, PartialEq)]
pub enum GetCurrentResponse {
    /// OK
    OK ( i32 ) ,
    /// Bad Request
    BadRequest ( models::BadRequest ) ,
    /// Operation Failed
    OperationFailed ( models::OpError ) ,
}

#[derive(Debug, PartialEq)]
pub enum GetErrorResponse {
    /// OK
    OK ( bool ) ,
    /// Bad Request
    BadRequest ( models::BadRequest ) ,
    /// Operation Failed
    OperationFailed ( models::OpError ) ,
}

#[derive(Debug, PartialEq)]
pub enum GetErrorsResponse {
    /// OK
    OK ( Vec<models::AddrInfo> ) ,
    /// Bad Request
    BadRequest ( models::BadRequest ) ,
    /// Operation Failed
    OperationFailed ( models::OpError ) ,
}

#[derive(Debug, PartialEq)]
pub enum GetFreqResponse {
    /// OK
    OK ( i32 ) ,
    /// Bad Request
    BadRequest ( models::BadRequest ) ,
    /// Operation Failed
    OperationFailed ( models::OpError ) ,
}

#[derive(Debug, PartialEq)]
pub enum GetGroupResponse {
    /// OK
    OK ( models::Group ) ,
    /// Bad Request
    BadRequest ( models::BadRequest ) ,
    /// Operation Failed
    OperationFailed ( models::OpError ) ,
}

#[derive(Debug, PartialEq)]
pub enum GetLedCurrentResponse {
    /// OK
    OK ( i32 ) ,
    /// Bad Request
    BadRequest ( models::BadRequest ) ,
    /// Operation Failed
    OperationFailed ( models::OpError ) ,
}

#[derive(Debug, PartialEq)]
pub enum GetLedErrorResponse {
    /// OK
    OK ( models::LedError ) ,
    /// Bad Request
    BadRequest ( models::BadRequest ) ,
    /// Operation Failed
    OperationFailed ( models::OpError ) ,
}

#[derive(Debug, PartialEq)]
pub enum GetLedInfoResponse {
    /// OK
    OK ( models::LedInfo ) ,
    /// Bad Request
    BadRequest ( models::BadRequest ) ,
    /// Operation Failed
    OperationFailed ( models::OpError ) ,
}

#[derive(Debug, PartialEq)]
pub enum GetLedInfoAllResponse {
    /// OK
    OK ( Vec<models::LedInfo> ) ,
    /// Bad Request
    BadRequest ( models::BadRequest ) ,
    /// Operation Failed
    OperationFailed ( models::OpError ) ,
}

#[derive(Debug, PartialEq)]
pub enum GetLedPwmResponse {
    /// OK
    OK ( i32 ) ,
    /// Bad Request
    BadRequest ( models::BadRequest ) ,
    /// Operation Failed
    OperationFailed ( models::OpError ) ,
}

#[derive(Debug, PartialEq)]
pub enum GetLedStateResponse {
    /// OK
    OK ( models::LedState ) ,
    /// Bad Request
    BadRequest ( models::BadRequest ) ,
    /// Operation Failed
    OperationFailed ( models::OpError ) ,
}

#[derive(Debug, PartialEq)]
pub enum GetOffsetResponse {
    /// OK
    OK ( i32 ) ,
    /// Bad Request
    BadRequest ( models::BadRequest ) ,
    /// Operation Failed
    OperationFailed ( models::OpError ) ,
}

#[derive(Debug, PartialEq)]
pub enum GetOutputChangeResponse {
    /// OK
    OK ( models::OutputChange ) ,
    /// Bad Request
    BadRequest ( models::BadRequest ) ,
    /// Operation Failed
    OperationFailed ( models::OpError ) ,
}

#[derive(Debug, PartialEq)]
pub enum GetOverTempResponse {
    /// OK
    OK ( bool ) ,
    /// Bad Request
    BadRequest ( models::BadRequest ) ,
    /// Operation Failed
    OperationFailed ( models::OpError ) ,
}

#[derive(Debug, PartialEq)]
pub enum GetPwmResponse {
    /// OK
    OK ( i32 ) ,
    /// Bad Request
    BadRequest ( models::BadRequest ) ,
    /// Operation Failed
    OperationFailed ( models::OpError ) ,
}

#[derive(Debug, PartialEq)]
pub enum GetSleepResponse {
    /// OK
    OK ( bool ) ,
    /// Bad Request
    BadRequest ( models::BadRequest ) ,
    /// Operation Failed
    OperationFailed ( models::OpError ) ,
}

#[derive(Debug, PartialEq)]
pub enum ResetResponse {
    /// OK
    OK ,
    /// Bad Request
    BadRequest ( models::BadRequest ) ,
    /// Operation Failed
    OperationFailed ( models::OpError ) ,
}

#[derive(Debug, PartialEq)]
pub enum SetAddrEnabledResponse {
    /// OK
    OK ,
    /// Bad Request
    BadRequest ( models::BadRequest ) ,
    /// Operation Failed
    OperationFailed ( models::OpError ) ,
}

#[derive(Debug, PartialEq)]
pub enum SetAddrValueResponse {
    /// OK
    OK ,
    /// Bad Request
    BadRequest ( models::BadRequest ) ,
    /// Operation Failed
    OperationFailed ( models::OpError ) ,
}

#[derive(Debug, PartialEq)]
pub enum SetConfigResponse {
    /// OK
    OK ,
    /// Bad Request
    BadRequest ( models::BadRequest ) ,
    /// Operation Failed
    OperationFailed ( models::OpError ) ,
}

#[derive(Debug, PartialEq)]
pub enum SetCurrentResponse {
    /// OK
    OK ,
    /// Bad Request
    BadRequest ( models::BadRequest ) ,
    /// Operation Failed
    OperationFailed ( models::OpError ) ,
}

#[derive(Debug, PartialEq)]
pub enum SetFreqResponse {
    /// OK
    OK ,
    /// Bad Request
    BadRequest ( models::BadRequest ) ,
    /// Operation Failed
    OperationFailed ( models::OpError ) ,
}

#[derive(Debug, PartialEq)]
pub enum SetGroupResponse {
    /// OK
    OK ,
    /// Bad Request
    BadRequest ( models::BadRequest ) ,
    /// Operation Failed
    OperationFailed ( models::OpError ) ,
}

#[derive(Debug, PartialEq)]
pub enum SetLedCurrentResponse {
    /// OK
    OK ,
    /// Bad Request
    BadRequest ( models::BadRequest ) ,
    /// Operation Failed
    OperationFailed ( models::OpError ) ,
}

#[derive(Debug, PartialEq)]
pub enum SetLedErrorResponse {
    /// OK
    OK ,
    /// Bad Request
    BadRequest ( models::BadRequest ) ,
    /// Operation Failed
    OperationFailed ( models::OpError ) ,
}

#[derive(Debug, PartialEq)]
pub enum SetLedInfoResponse {
    /// OK
    OK ,
    /// Bad Request
    BadRequest ( models::BadRequest ) ,
    /// Operation Failed
    OperationFailed ( models::OpError ) ,
}

#[derive(Debug, PartialEq)]
pub enum SetLedInfoAllResponse {
    /// OK
    OK ,
    /// Bad Request
    BadRequest ( models::BadRequest ) ,
    /// Operation Failed
    OperationFailed ( models::OpError ) ,
}

#[derive(Debug, PartialEq)]
pub enum SetLedPwmResponse {
    /// OK
    OK ,
    /// Bad Request
    BadRequest ( models::BadRequest ) ,
    /// Operation Failed
    OperationFailed ( models::OpError ) ,
}

#[derive(Debug, PartialEq)]
pub enum SetLedStateResponse {
    /// OK
    OK ,
    /// Bad Request
    BadRequest ( models::BadRequest ) ,
    /// Operation Failed
    OperationFailed ( models::OpError ) ,
}

#[derive(Debug, PartialEq)]
pub enum SetOffsetResponse {
    /// OK
    OK ,
    /// Bad Request
    BadRequest ( models::BadRequest ) ,
    /// Operation Failed
    OperationFailed ( models::OpError ) ,
}

#[derive(Debug, PartialEq)]
pub enum SetOutputChangeResponse {
    /// OK
    OK ,
    /// Bad Request
    BadRequest ( models::BadRequest ) ,
    /// Operation Failed
    OperationFailed ( models::OpError ) ,
}

#[derive(Debug, PartialEq)]
pub enum SetPwmResponse {
    /// OK
    OK ,
    /// Bad Request
    BadRequest ( models::BadRequest ) ,
    /// Operation Failed
    OperationFailed ( models::OpError ) ,
}

#[derive(Debug, PartialEq)]
pub enum SetSleepResponse {
    /// OK
    OK ,
    /// Bad Request
    BadRequest ( models::BadRequest ) ,
    /// Operation Failed
    OperationFailed ( models::OpError ) ,
}


/// API
pub trait Api<C> {


    fn clear_error(&self, bus_id: i32, addr: i32, context: &C) -> Box<Future<Item=ClearErrorResponse, Error=ApiError>>;


    fn get_addr_enabled(&self, bus_id: i32, addr: i32, num: i32, context: &C) -> Box<Future<Item=GetAddrEnabledResponse, Error=ApiError>>;


    fn get_addr_info(&self, bus_id: i32, addr: i32, num: i32, context: &C) -> Box<Future<Item=GetAddrInfoResponse, Error=ApiError>>;


    fn get_addr_value(&self, bus_id: i32, addr: i32, num: i32, context: &C) -> Box<Future<Item=GetAddrValueResponse, Error=ApiError>>;


    fn get_api(&self, context: &C) -> Box<Future<Item=GetApiResponse, Error=ApiError>>;


    fn get_config(&self, bus_id: i32, addr: i32, context: &C) -> Box<Future<Item=GetConfigResponse, Error=ApiError>>;


    fn get_current(&self, bus_id: i32, addr: i32, context: &C) -> Box<Future<Item=GetCurrentResponse, Error=ApiError>>;


    fn get_error(&self, bus_id: i32, addr: i32, context: &C) -> Box<Future<Item=GetErrorResponse, Error=ApiError>>;


    fn get_errors(&self, bus_id: i32, addr: i32, context: &C) -> Box<Future<Item=GetErrorsResponse, Error=ApiError>>;


    fn get_freq(&self, bus_id: i32, addr: i32, context: &C) -> Box<Future<Item=GetFreqResponse, Error=ApiError>>;


    fn get_group(&self, bus_id: i32, addr: i32, context: &C) -> Box<Future<Item=GetGroupResponse, Error=ApiError>>;


    fn get_led_current(&self, bus_id: i32, addr: i32, led: i32, context: &C) -> Box<Future<Item=GetLedCurrentResponse, Error=ApiError>>;


    fn get_led_error(&self, bus_id: i32, addr: i32, led: i32, context: &C) -> Box<Future<Item=GetLedErrorResponse, Error=ApiError>>;


    fn get_led_info(&self, bus_id: i32, addr: i32, led: i32, context: &C) -> Box<Future<Item=GetLedInfoResponse, Error=ApiError>>;


    fn get_led_info_all(&self, bus_id: i32, addr: i32, context: &C) -> Box<Future<Item=GetLedInfoAllResponse, Error=ApiError>>;


    fn get_led_pwm(&self, bus_id: i32, addr: i32, led: i32, context: &C) -> Box<Future<Item=GetLedPwmResponse, Error=ApiError>>;


    fn get_led_state(&self, bus_id: i32, addr: i32, led: i32, context: &C) -> Box<Future<Item=GetLedStateResponse, Error=ApiError>>;


    fn get_offset(&self, bus_id: i32, addr: i32, context: &C) -> Box<Future<Item=GetOffsetResponse, Error=ApiError>>;


    fn get_output_change(&self, bus_id: i32, addr: i32, context: &C) -> Box<Future<Item=GetOutputChangeResponse, Error=ApiError>>;


    fn get_over_temp(&self, bus_id: i32, addr: i32, context: &C) -> Box<Future<Item=GetOverTempResponse, Error=ApiError>>;


    fn get_pwm(&self, bus_id: i32, addr: i32, context: &C) -> Box<Future<Item=GetPwmResponse, Error=ApiError>>;


    fn get_sleep(&self, bus_id: i32, addr: i32, context: &C) -> Box<Future<Item=GetSleepResponse, Error=ApiError>>;


    fn reset(&self, bus_id: i32, context: &C) -> Box<Future<Item=ResetResponse, Error=ApiError>>;


    fn set_addr_enabled(&self, bus_id: i32, addr: i32, num: i32, enabled: bool, context: &C) -> Box<Future<Item=SetAddrEnabledResponse, Error=ApiError>>;


    fn set_addr_value(&self, bus_id: i32, addr: i32, num: i32, addr_val: i32, context: &C) -> Box<Future<Item=SetAddrValueResponse, Error=ApiError>>;


    fn set_config(&self, bus_id: i32, addr: i32, config: models::Config, context: &C) -> Box<Future<Item=SetConfigResponse, Error=ApiError>>;


    fn set_current(&self, bus_id: i32, addr: i32, current: i32, context: &C) -> Box<Future<Item=SetCurrentResponse, Error=ApiError>>;


    fn set_freq(&self, bus_id: i32, addr: i32, freq: i32, context: &C) -> Box<Future<Item=SetFreqResponse, Error=ApiError>>;


    fn set_group(&self, bus_id: i32, addr: i32, group: models::Group, context: &C) -> Box<Future<Item=SetGroupResponse, Error=ApiError>>;


    fn set_led_current(&self, bus_id: i32, addr: i32, led: i32, current: i32, context: &C) -> Box<Future<Item=SetLedCurrentResponse, Error=ApiError>>;


    fn set_led_error(&self, bus_id: i32, addr: i32, led: i32, error: models::LedError, context: &C) -> Box<Future<Item=SetLedErrorResponse, Error=ApiError>>;


    fn set_led_info(&self, bus_id: i32, addr: i32, led: i32, led_info: models::LedInfo, context: &C) -> Box<Future<Item=SetLedInfoResponse, Error=ApiError>>;


    fn set_led_info_all(&self, bus_id: i32, addr: i32, led_info: &Vec<models::LedInfo>, context: &C) -> Box<Future<Item=SetLedInfoAllResponse, Error=ApiError>>;


    fn set_led_pwm(&self, bus_id: i32, addr: i32, led: i32, pwm: i32, context: &C) -> Box<Future<Item=SetLedPwmResponse, Error=ApiError>>;


    fn set_led_state(&self, bus_id: i32, addr: i32, led: i32, state: models::LedState, context: &C) -> Box<Future<Item=SetLedStateResponse, Error=ApiError>>;


    fn set_offset(&self, bus_id: i32, addr: i32, offset: i32, context: &C) -> Box<Future<Item=SetOffsetResponse, Error=ApiError>>;


    fn set_output_change(&self, bus_id: i32, addr: i32, output_change: models::OutputChange, context: &C) -> Box<Future<Item=SetOutputChangeResponse, Error=ApiError>>;


    fn set_pwm(&self, bus_id: i32, addr: i32, pwm: i32, context: &C) -> Box<Future<Item=SetPwmResponse, Error=ApiError>>;


    fn set_sleep(&self, bus_id: i32, addr: i32, sleep: bool, context: &C) -> Box<Future<Item=SetSleepResponse, Error=ApiError>>;

}

/// API without a `Context`
pub trait ApiNoContext {


    fn clear_error(&self, bus_id: i32, addr: i32) -> Box<Future<Item=ClearErrorResponse, Error=ApiError>>;


    fn get_addr_enabled(&self, bus_id: i32, addr: i32, num: i32) -> Box<Future<Item=GetAddrEnabledResponse, Error=ApiError>>;


    fn get_addr_info(&self, bus_id: i32, addr: i32, num: i32) -> Box<Future<Item=GetAddrInfoResponse, Error=ApiError>>;


    fn get_addr_value(&self, bus_id: i32, addr: i32, num: i32) -> Box<Future<Item=GetAddrValueResponse, Error=ApiError>>;


    fn get_api(&self) -> Box<Future<Item=GetApiResponse, Error=ApiError>>;


    fn get_config(&self, bus_id: i32, addr: i32) -> Box<Future<Item=GetConfigResponse, Error=ApiError>>;


    fn get_current(&self, bus_id: i32, addr: i32) -> Box<Future<Item=GetCurrentResponse, Error=ApiError>>;


    fn get_error(&self, bus_id: i32, addr: i32) -> Box<Future<Item=GetErrorResponse, Error=ApiError>>;


    fn get_errors(&self, bus_id: i32, addr: i32) -> Box<Future<Item=GetErrorsResponse, Error=ApiError>>;


    fn get_freq(&self, bus_id: i32, addr: i32) -> Box<Future<Item=GetFreqResponse, Error=ApiError>>;


    fn get_group(&self, bus_id: i32, addr: i32) -> Box<Future<Item=GetGroupResponse, Error=ApiError>>;


    fn get_led_current(&self, bus_id: i32, addr: i32, led: i32) -> Box<Future<Item=GetLedCurrentResponse, Error=ApiError>>;


    fn get_led_error(&self, bus_id: i32, addr: i32, led: i32) -> Box<Future<Item=GetLedErrorResponse, Error=ApiError>>;


    fn get_led_info(&self, bus_id: i32, addr: i32, led: i32) -> Box<Future<Item=GetLedInfoResponse, Error=ApiError>>;


    fn get_led_info_all(&self, bus_id: i32, addr: i32) -> Box<Future<Item=GetLedInfoAllResponse, Error=ApiError>>;


    fn get_led_pwm(&self, bus_id: i32, addr: i32, led: i32) -> Box<Future<Item=GetLedPwmResponse, Error=ApiError>>;


    fn get_led_state(&self, bus_id: i32, addr: i32, led: i32) -> Box<Future<Item=GetLedStateResponse, Error=ApiError>>;


    fn get_offset(&self, bus_id: i32, addr: i32) -> Box<Future<Item=GetOffsetResponse, Error=ApiError>>;


    fn get_output_change(&self, bus_id: i32, addr: i32) -> Box<Future<Item=GetOutputChangeResponse, Error=ApiError>>;


    fn get_over_temp(&self, bus_id: i32, addr: i32) -> Box<Future<Item=GetOverTempResponse, Error=ApiError>>;


    fn get_pwm(&self, bus_id: i32, addr: i32) -> Box<Future<Item=GetPwmResponse, Error=ApiError>>;


    fn get_sleep(&self, bus_id: i32, addr: i32) -> Box<Future<Item=GetSleepResponse, Error=ApiError>>;


    fn reset(&self, bus_id: i32) -> Box<Future<Item=ResetResponse, Error=ApiError>>;


    fn set_addr_enabled(&self, bus_id: i32, addr: i32, num: i32, enabled: bool) -> Box<Future<Item=SetAddrEnabledResponse, Error=ApiError>>;


    fn set_addr_value(&self, bus_id: i32, addr: i32, num: i32, addr_val: i32) -> Box<Future<Item=SetAddrValueResponse, Error=ApiError>>;


    fn set_config(&self, bus_id: i32, addr: i32, config: models::Config) -> Box<Future<Item=SetConfigResponse, Error=ApiError>>;


    fn set_current(&self, bus_id: i32, addr: i32, current: i32) -> Box<Future<Item=SetCurrentResponse, Error=ApiError>>;


    fn set_freq(&self, bus_id: i32, addr: i32, freq: i32) -> Box<Future<Item=SetFreqResponse, Error=ApiError>>;


    fn set_group(&self, bus_id: i32, addr: i32, group: models::Group) -> Box<Future<Item=SetGroupResponse, Error=ApiError>>;


    fn set_led_current(&self, bus_id: i32, addr: i32, led: i32, current: i32) -> Box<Future<Item=SetLedCurrentResponse, Error=ApiError>>;


    fn set_led_error(&self, bus_id: i32, addr: i32, led: i32, error: models::LedError) -> Box<Future<Item=SetLedErrorResponse, Error=ApiError>>;


    fn set_led_info(&self, bus_id: i32, addr: i32, led: i32, led_info: models::LedInfo) -> Box<Future<Item=SetLedInfoResponse, Error=ApiError>>;


    fn set_led_info_all(&self, bus_id: i32, addr: i32, led_info: &Vec<models::LedInfo>) -> Box<Future<Item=SetLedInfoAllResponse, Error=ApiError>>;


    fn set_led_pwm(&self, bus_id: i32, addr: i32, led: i32, pwm: i32) -> Box<Future<Item=SetLedPwmResponse, Error=ApiError>>;


    fn set_led_state(&self, bus_id: i32, addr: i32, led: i32, state: models::LedState) -> Box<Future<Item=SetLedStateResponse, Error=ApiError>>;


    fn set_offset(&self, bus_id: i32, addr: i32, offset: i32) -> Box<Future<Item=SetOffsetResponse, Error=ApiError>>;


    fn set_output_change(&self, bus_id: i32, addr: i32, output_change: models::OutputChange) -> Box<Future<Item=SetOutputChangeResponse, Error=ApiError>>;


    fn set_pwm(&self, bus_id: i32, addr: i32, pwm: i32) -> Box<Future<Item=SetPwmResponse, Error=ApiError>>;


    fn set_sleep(&self, bus_id: i32, addr: i32, sleep: bool) -> Box<Future<Item=SetSleepResponse, Error=ApiError>>;

}

/// Trait to extend an API to make it easy to bind it to a context.
pub trait ContextWrapperExt<'a, C> where Self: Sized {
    /// Binds this API to a context.
    fn with_context(self: &'a Self, context: C) -> ContextWrapper<'a, Self, C>;
}

impl<'a, T: Api<C> + Sized, C> ContextWrapperExt<'a, C> for T {
    fn with_context(self: &'a T, context: C) -> ContextWrapper<'a, T, C> {
         ContextWrapper::<T, C>::new(self, context)
    }
}

impl<'a, T: Api<C>, C> ApiNoContext for ContextWrapper<'a, T, C> {


    fn clear_error(&self, bus_id: i32, addr: i32) -> Box<Future<Item=ClearErrorResponse, Error=ApiError>> {
        self.api().clear_error(bus_id, addr, &self.context())
    }


    fn get_addr_enabled(&self, bus_id: i32, addr: i32, num: i32) -> Box<Future<Item=GetAddrEnabledResponse, Error=ApiError>> {
        self.api().get_addr_enabled(bus_id, addr, num, &self.context())
    }


    fn get_addr_info(&self, bus_id: i32, addr: i32, num: i32) -> Box<Future<Item=GetAddrInfoResponse, Error=ApiError>> {
        self.api().get_addr_info(bus_id, addr, num, &self.context())
    }


    fn get_addr_value(&self, bus_id: i32, addr: i32, num: i32) -> Box<Future<Item=GetAddrValueResponse, Error=ApiError>> {
        self.api().get_addr_value(bus_id, addr, num, &self.context())
    }


    fn get_api(&self) -> Box<Future<Item=GetApiResponse, Error=ApiError>> {
        self.api().get_api(&self.context())
    }


    fn get_config(&self, bus_id: i32, addr: i32) -> Box<Future<Item=GetConfigResponse, Error=ApiError>> {
        self.api().get_config(bus_id, addr, &self.context())
    }


    fn get_current(&self, bus_id: i32, addr: i32) -> Box<Future<Item=GetCurrentResponse, Error=ApiError>> {
        self.api().get_current(bus_id, addr, &self.context())
    }


    fn get_error(&self, bus_id: i32, addr: i32) -> Box<Future<Item=GetErrorResponse, Error=ApiError>> {
        self.api().get_error(bus_id, addr, &self.context())
    }


    fn get_errors(&self, bus_id: i32, addr: i32) -> Box<Future<Item=GetErrorsResponse, Error=ApiError>> {
        self.api().get_errors(bus_id, addr, &self.context())
    }


    fn get_freq(&self, bus_id: i32, addr: i32) -> Box<Future<Item=GetFreqResponse, Error=ApiError>> {
        self.api().get_freq(bus_id, addr, &self.context())
    }


    fn get_group(&self, bus_id: i32, addr: i32) -> Box<Future<Item=GetGroupResponse, Error=ApiError>> {
        self.api().get_group(bus_id, addr, &self.context())
    }


    fn get_led_current(&self, bus_id: i32, addr: i32, led: i32) -> Box<Future<Item=GetLedCurrentResponse, Error=ApiError>> {
        self.api().get_led_current(bus_id, addr, led, &self.context())
    }


    fn get_led_error(&self, bus_id: i32, addr: i32, led: i32) -> Box<Future<Item=GetLedErrorResponse, Error=ApiError>> {
        self.api().get_led_error(bus_id, addr, led, &self.context())
    }


    fn get_led_info(&self, bus_id: i32, addr: i32, led: i32) -> Box<Future<Item=GetLedInfoResponse, Error=ApiError>> {
        self.api().get_led_info(bus_id, addr, led, &self.context())
    }


    fn get_led_info_all(&self, bus_id: i32, addr: i32) -> Box<Future<Item=GetLedInfoAllResponse, Error=ApiError>> {
        self.api().get_led_info_all(bus_id, addr, &self.context())
    }


    fn get_led_pwm(&self, bus_id: i32, addr: i32, led: i32) -> Box<Future<Item=GetLedPwmResponse, Error=ApiError>> {
        self.api().get_led_pwm(bus_id, addr, led, &self.context())
    }


    fn get_led_state(&self, bus_id: i32, addr: i32, led: i32) -> Box<Future<Item=GetLedStateResponse, Error=ApiError>> {
        self.api().get_led_state(bus_id, addr, led, &self.context())
    }


    fn get_offset(&self, bus_id: i32, addr: i32) -> Box<Future<Item=GetOffsetResponse, Error=ApiError>> {
        self.api().get_offset(bus_id, addr, &self.context())
    }


    fn get_output_change(&self, bus_id: i32, addr: i32) -> Box<Future<Item=GetOutputChangeResponse, Error=ApiError>> {
        self.api().get_output_change(bus_id, addr, &self.context())
    }


    fn get_over_temp(&self, bus_id: i32, addr: i32) -> Box<Future<Item=GetOverTempResponse, Error=ApiError>> {
        self.api().get_over_temp(bus_id, addr, &self.context())
    }


    fn get_pwm(&self, bus_id: i32, addr: i32) -> Box<Future<Item=GetPwmResponse, Error=ApiError>> {
        self.api().get_pwm(bus_id, addr, &self.context())
    }


    fn get_sleep(&self, bus_id: i32, addr: i32) -> Box<Future<Item=GetSleepResponse, Error=ApiError>> {
        self.api().get_sleep(bus_id, addr, &self.context())
    }


    fn reset(&self, bus_id: i32) -> Box<Future<Item=ResetResponse, Error=ApiError>> {
        self.api().reset(bus_id, &self.context())
    }


    fn set_addr_enabled(&self, bus_id: i32, addr: i32, num: i32, enabled: bool) -> Box<Future<Item=SetAddrEnabledResponse, Error=ApiError>> {
        self.api().set_addr_enabled(bus_id, addr, num, enabled, &self.context())
    }


    fn set_addr_value(&self, bus_id: i32, addr: i32, num: i32, addr_val: i32) -> Box<Future<Item=SetAddrValueResponse, Error=ApiError>> {
        self.api().set_addr_value(bus_id, addr, num, addr_val, &self.context())
    }


    fn set_config(&self, bus_id: i32, addr: i32, config: models::Config) -> Box<Future<Item=SetConfigResponse, Error=ApiError>> {
        self.api().set_config(bus_id, addr, config, &self.context())
    }


    fn set_current(&self, bus_id: i32, addr: i32, current: i32) -> Box<Future<Item=SetCurrentResponse, Error=ApiError>> {
        self.api().set_current(bus_id, addr, current, &self.context())
    }


    fn set_freq(&self, bus_id: i32, addr: i32, freq: i32) -> Box<Future<Item=SetFreqResponse, Error=ApiError>> {
        self.api().set_freq(bus_id, addr, freq, &self.context())
    }


    fn set_group(&self, bus_id: i32, addr: i32, group: models::Group) -> Box<Future<Item=SetGroupResponse, Error=ApiError>> {
        self.api().set_group(bus_id, addr, group, &self.context())
    }


    fn set_led_current(&self, bus_id: i32, addr: i32, led: i32, current: i32) -> Box<Future<Item=SetLedCurrentResponse, Error=ApiError>> {
        self.api().set_led_current(bus_id, addr, led, current, &self.context())
    }


    fn set_led_error(&self, bus_id: i32, addr: i32, led: i32, error: models::LedError) -> Box<Future<Item=SetLedErrorResponse, Error=ApiError>> {
        self.api().set_led_error(bus_id, addr, led, error, &self.context())
    }


    fn set_led_info(&self, bus_id: i32, addr: i32, led: i32, led_info: models::LedInfo) -> Box<Future<Item=SetLedInfoResponse, Error=ApiError>> {
        self.api().set_led_info(bus_id, addr, led, led_info, &self.context())
    }


    fn set_led_info_all(&self, bus_id: i32, addr: i32, led_info: &Vec<models::LedInfo>) -> Box<Future<Item=SetLedInfoAllResponse, Error=ApiError>> {
        self.api().set_led_info_all(bus_id, addr, led_info, &self.context())
    }


    fn set_led_pwm(&self, bus_id: i32, addr: i32, led: i32, pwm: i32) -> Box<Future<Item=SetLedPwmResponse, Error=ApiError>> {
        self.api().set_led_pwm(bus_id, addr, led, pwm, &self.context())
    }


    fn set_led_state(&self, bus_id: i32, addr: i32, led: i32, state: models::LedState) -> Box<Future<Item=SetLedStateResponse, Error=ApiError>> {
        self.api().set_led_state(bus_id, addr, led, state, &self.context())
    }


    fn set_offset(&self, bus_id: i32, addr: i32, offset: i32) -> Box<Future<Item=SetOffsetResponse, Error=ApiError>> {
        self.api().set_offset(bus_id, addr, offset, &self.context())
    }


    fn set_output_change(&self, bus_id: i32, addr: i32, output_change: models::OutputChange) -> Box<Future<Item=SetOutputChangeResponse, Error=ApiError>> {
        self.api().set_output_change(bus_id, addr, output_change, &self.context())
    }


    fn set_pwm(&self, bus_id: i32, addr: i32, pwm: i32) -> Box<Future<Item=SetPwmResponse, Error=ApiError>> {
        self.api().set_pwm(bus_id, addr, pwm, &self.context())
    }


    fn set_sleep(&self, bus_id: i32, addr: i32, sleep: bool) -> Box<Future<Item=SetSleepResponse, Error=ApiError>> {
        self.api().set_sleep(bus_id, addr, sleep, &self.context())
    }

}

#[cfg(feature = "client")]
pub mod client;

// Re-export Client as a top-level name
#[cfg(feature = "client")]
pub use self::client::Client;

#[cfg(feature = "server")]
pub mod server;

// Re-export router() as a top-level name
#[cfg(feature = "server")]
pub use self::server::Service;

pub mod models;
