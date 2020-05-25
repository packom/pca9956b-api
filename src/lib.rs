#![allow(missing_docs, trivial_casts, unused_variables, unused_mut, unused_imports, unused_extern_crates, non_camel_case_types)]

use futures::Stream;
use std::io::Error;

#[deprecated(note = "Import swagger-rs directly")]
pub use swagger::{ApiError, ContextWrapper};
#[deprecated(note = "Import futures directly")]
pub use futures::Future;

pub const BASE_PATH: &'static str = "";
pub const API_VERSION: &'static str = "0.1.2";

#[derive(Debug, PartialEq)]
#[must_use]
pub enum ClearErrorResponse {
    /// OK
    OK
    ,
    /// Bad Request
    BadRequest
    (models::BadRequest)
    ,
    /// Operation Failed
    OperationFailed
    (models::OpError)
}

#[derive(Debug, PartialEq)]
#[must_use]
pub enum GetAddrEnabledResponse {
    /// OK
    OK
    (bool)
    ,
    /// Bad Request
    BadRequest
    (models::BadRequest)
    ,
    /// Operation Failed
    OperationFailed
    (models::OpError)
}

#[derive(Debug, PartialEq)]
#[must_use]
pub enum GetAddrInfoResponse {
    /// OK
    OK
    (models::AddrInfo)
    ,
    /// Bad Request
    BadRequest
    (models::BadRequest)
    ,
    /// Operation Failed
    OperationFailed
    (models::OpError)
}

#[derive(Debug, PartialEq)]
#[must_use]
pub enum GetAddrValueResponse {
    /// OK
    OK
    (i32)
    ,
    /// Bad Request
    BadRequest
    (models::BadRequest)
    ,
    /// Operation Failed
    OperationFailed
    (models::OpError)
}

#[derive(Debug, PartialEq)]
#[must_use]
pub enum GetApiResponse {
    /// OK
    OK
    (String)
    ,
    /// File not found
    FileNotFound
    (String)
}

#[derive(Debug, PartialEq)]
#[must_use]
pub enum GetConfigResponse {
    /// OK
    OK
    (models::Config)
    ,
    /// Bad Request
    BadRequest
    (models::BadRequest)
    ,
    /// Operation Failed
    OperationFailed
    (models::OpError)
}

#[derive(Debug, PartialEq)]
#[must_use]
pub enum GetCurrentResponse {
    /// OK
    OK
    (i32)
    ,
    /// Bad Request
    BadRequest
    (models::BadRequest)
    ,
    /// Operation Failed
    OperationFailed
    (models::OpError)
}

#[derive(Debug, PartialEq)]
#[must_use]
pub enum GetErrorResponse {
    /// OK
    OK
    (bool)
    ,
    /// Bad Request
    BadRequest
    (models::BadRequest)
    ,
    /// Operation Failed
    OperationFailed
    (models::OpError)
}

#[derive(Debug, PartialEq)]
#[must_use]
pub enum GetErrorsResponse {
    /// OK
    OK
    (models::LedErrors)
    ,
    /// Bad Request
    BadRequest
    (models::BadRequest)
    ,
    /// Operation Failed
    OperationFailed
    (models::OpError)
}

#[derive(Debug, PartialEq)]
#[must_use]
pub enum GetFreqResponse {
    /// OK
    OK
    (i32)
    ,
    /// Bad Request
    BadRequest
    (models::BadRequest)
    ,
    /// Operation Failed
    OperationFailed
    (models::OpError)
}

#[derive(Debug, PartialEq)]
#[must_use]
pub enum GetGroupResponse {
    /// OK
    OK
    (models::Group)
    ,
    /// Bad Request
    BadRequest
    (models::BadRequest)
    ,
    /// Operation Failed
    OperationFailed
    (models::OpError)
}

#[derive(Debug, PartialEq)]
#[must_use]
pub enum GetLedCurrentResponse {
    /// OK
    OK
    (i32)
    ,
    /// Bad Request
    BadRequest
    (models::BadRequest)
    ,
    /// Operation Failed
    OperationFailed
    (models::OpError)
}

#[derive(Debug, PartialEq)]
#[must_use]
pub enum GetLedErrorResponse {
    /// OK
    OK
    (models::LedError)
    ,
    /// Bad Request
    BadRequest
    (models::BadRequest)
    ,
    /// Operation Failed
    OperationFailed
    (models::OpError)
}

#[derive(Debug, PartialEq)]
#[must_use]
pub enum GetLedInfoResponse {
    /// OK
    OK
    (models::LedInfo)
    ,
    /// Bad Request
    BadRequest
    (models::BadRequest)
    ,
    /// Operation Failed
    OperationFailed
    (models::OpError)
}

#[derive(Debug, PartialEq)]
#[must_use]
pub enum GetLedInfoAllResponse {
    /// OK
    OK
    (models::LedInfoArray)
    ,
    /// Bad Request
    BadRequest
    (models::BadRequest)
    ,
    /// Operation Failed
    OperationFailed
    (models::OpError)
}

#[derive(Debug, PartialEq)]
#[must_use]
pub enum GetLedPwmResponse {
    /// OK
    OK
    (i32)
    ,
    /// Bad Request
    BadRequest
    (models::BadRequest)
    ,
    /// Operation Failed
    OperationFailed
    (models::OpError)
}

#[derive(Debug, PartialEq)]
#[must_use]
pub enum GetLedStateResponse {
    /// OK
    OK
    (models::LedState)
    ,
    /// Bad Request
    BadRequest
    (models::BadRequest)
    ,
    /// Operation Failed
    OperationFailed
    (models::OpError)
}

#[derive(Debug, PartialEq)]
#[must_use]
pub enum GetOffsetResponse {
    /// OK
    OK
    (i32)
    ,
    /// Bad Request
    BadRequest
    (models::BadRequest)
    ,
    /// Operation Failed
    OperationFailed
    (models::OpError)
}

#[derive(Debug, PartialEq)]
#[must_use]
pub enum GetOutputChangeResponse {
    /// OK
    OK
    (models::OutputChange)
    ,
    /// Bad Request
    BadRequest
    (models::BadRequest)
    ,
    /// Operation Failed
    OperationFailed
    (models::OpError)
}

#[derive(Debug, PartialEq)]
#[must_use]
pub enum GetOverTempResponse {
    /// OK
    OK
    (bool)
    ,
    /// Bad Request
    BadRequest
    (models::BadRequest)
    ,
    /// Operation Failed
    OperationFailed
    (models::OpError)
}

#[derive(Debug, PartialEq)]
#[must_use]
pub enum GetPwmResponse {
    /// OK
    OK
    (i32)
    ,
    /// Bad Request
    BadRequest
    (models::BadRequest)
    ,
    /// Operation Failed
    OperationFailed
    (models::OpError)
}

#[derive(Debug, PartialEq)]
#[must_use]
pub enum GetSleepResponse {
    /// OK
    OK
    (bool)
    ,
    /// Bad Request
    BadRequest
    (models::BadRequest)
    ,
    /// Operation Failed
    OperationFailed
    (models::OpError)
}

#[derive(Debug, PartialEq)]
#[must_use]
pub enum ResetResponse {
    /// OK
    OK
    ,
    /// Bad Request
    BadRequest
    (models::BadRequest)
    ,
    /// Operation Failed
    OperationFailed
    (models::OpError)
}

#[derive(Debug, PartialEq)]
#[must_use]
pub enum SetAddrEnabledResponse {
    /// OK
    OK
    ,
    /// Bad Request
    BadRequest
    (models::BadRequest)
    ,
    /// Operation Failed
    OperationFailed
    (models::OpError)
}

#[derive(Debug, PartialEq)]
#[must_use]
pub enum SetAddrValueResponse {
    /// OK
    OK
    ,
    /// Bad Request
    BadRequest
    (models::BadRequest)
    ,
    /// Operation Failed
    OperationFailed
    (models::OpError)
}

#[derive(Debug, PartialEq)]
#[must_use]
pub enum SetConfigResponse {
    /// OK
    OK
    ,
    /// Bad Request
    BadRequest
    (models::BadRequest)
    ,
    /// Operation Failed
    OperationFailed
    (models::OpError)
}

#[derive(Debug, PartialEq)]
#[must_use]
pub enum SetCurrentResponse {
    /// OK
    OK
    ,
    /// Bad Request
    BadRequest
    (models::BadRequest)
    ,
    /// Operation Failed
    OperationFailed
    (models::OpError)
}

#[derive(Debug, PartialEq)]
#[must_use]
pub enum SetFreqResponse {
    /// OK
    OK
    ,
    /// Bad Request
    BadRequest
    (models::BadRequest)
    ,
    /// Operation Failed
    OperationFailed
    (models::OpError)
}

#[derive(Debug, PartialEq)]
#[must_use]
pub enum SetGroupResponse {
    /// OK
    OK
    ,
    /// Bad Request
    BadRequest
    (models::BadRequest)
    ,
    /// Operation Failed
    OperationFailed
    (models::OpError)
}

#[derive(Debug, PartialEq)]
#[must_use]
pub enum SetLedCurrentResponse {
    /// OK
    OK
    ,
    /// Bad Request
    BadRequest
    (models::BadRequest)
    ,
    /// Operation Failed
    OperationFailed
    (models::OpError)
}

#[derive(Debug, PartialEq)]
#[must_use]
pub enum SetLedErrorResponse {
    /// OK
    OK
    ,
    /// Bad Request
    BadRequest
    (models::BadRequest)
    ,
    /// Operation Failed
    OperationFailed
    (models::OpError)
}

#[derive(Debug, PartialEq)]
#[must_use]
pub enum SetLedInfoResponse {
    /// OK
    OK
    ,
    /// Bad Request
    BadRequest
    (models::BadRequest)
    ,
    /// Operation Failed
    OperationFailed
    (models::OpError)
}

#[derive(Debug, PartialEq)]
#[must_use]
pub enum SetLedInfoAllResponse {
    /// OK
    OK
    ,
    /// Bad Request
    BadRequest
    (models::BadRequest)
    ,
    /// Operation Failed
    OperationFailed
    (models::OpError)
}

#[derive(Debug, PartialEq)]
#[must_use]
pub enum SetLedPwmResponse {
    /// OK
    OK
    ,
    /// Bad Request
    BadRequest
    (models::BadRequest)
    ,
    /// Operation Failed
    OperationFailed
    (models::OpError)
}

#[derive(Debug, PartialEq)]
#[must_use]
pub enum SetLedStateResponse {
    /// OK
    OK
    ,
    /// Bad Request
    BadRequest
    (models::BadRequest)
    ,
    /// Operation Failed
    OperationFailed
    (models::OpError)
}

#[derive(Debug, PartialEq)]
#[must_use]
pub enum SetOffsetResponse {
    /// OK
    OK
    ,
    /// Bad Request
    BadRequest
    (models::BadRequest)
    ,
    /// Operation Failed
    OperationFailed
    (models::OpError)
}

#[derive(Debug, PartialEq)]
#[must_use]
pub enum SetOutputChangeResponse {
    /// OK
    OK
    ,
    /// Bad Request
    BadRequest
    (models::BadRequest)
    ,
    /// Operation Failed
    OperationFailed
    (models::OpError)
}

#[derive(Debug, PartialEq)]
#[must_use]
pub enum SetPwmResponse {
    /// OK
    OK
    ,
    /// Bad Request
    BadRequest
    (models::BadRequest)
    ,
    /// Operation Failed
    OperationFailed
    (models::OpError)
}

#[derive(Debug, PartialEq)]
#[must_use]
pub enum SetSleepResponse {
    /// OK
    OK
    ,
    /// Bad Request
    BadRequest
    (models::BadRequest)
    ,
    /// Operation Failed
    OperationFailed
    (models::OpError)
}

/// API
pub trait Api<C> {
    fn clear_error(
        &self,
        bus_id: i32,
        addr: i32,
        context: &C) -> Box<dyn Future<Item=ClearErrorResponse, Error=ApiError> + Send>;

    fn get_addr_enabled(
        &self,
        bus_id: i32,
        addr: i32,
        num: i32,
        context: &C) -> Box<dyn Future<Item=GetAddrEnabledResponse, Error=ApiError> + Send>;

    fn get_addr_info(
        &self,
        bus_id: i32,
        addr: i32,
        num: i32,
        context: &C) -> Box<dyn Future<Item=GetAddrInfoResponse, Error=ApiError> + Send>;

    fn get_addr_value(
        &self,
        bus_id: i32,
        addr: i32,
        num: i32,
        context: &C) -> Box<dyn Future<Item=GetAddrValueResponse, Error=ApiError> + Send>;

    fn get_api(
        &self,
        context: &C) -> Box<dyn Future<Item=GetApiResponse, Error=ApiError> + Send>;

    fn get_config(
        &self,
        bus_id: i32,
        addr: i32,
        context: &C) -> Box<dyn Future<Item=GetConfigResponse, Error=ApiError> + Send>;

    fn get_current(
        &self,
        bus_id: i32,
        addr: i32,
        context: &C) -> Box<dyn Future<Item=GetCurrentResponse, Error=ApiError> + Send>;

    fn get_error(
        &self,
        bus_id: i32,
        addr: i32,
        context: &C) -> Box<dyn Future<Item=GetErrorResponse, Error=ApiError> + Send>;

    fn get_errors(
        &self,
        bus_id: i32,
        addr: i32,
        context: &C) -> Box<dyn Future<Item=GetErrorsResponse, Error=ApiError> + Send>;

    fn get_freq(
        &self,
        bus_id: i32,
        addr: i32,
        context: &C) -> Box<dyn Future<Item=GetFreqResponse, Error=ApiError> + Send>;

    fn get_group(
        &self,
        bus_id: i32,
        addr: i32,
        context: &C) -> Box<dyn Future<Item=GetGroupResponse, Error=ApiError> + Send>;

    fn get_led_current(
        &self,
        bus_id: i32,
        addr: i32,
        led: i32,
        context: &C) -> Box<dyn Future<Item=GetLedCurrentResponse, Error=ApiError> + Send>;

    fn get_led_error(
        &self,
        bus_id: i32,
        addr: i32,
        led: i32,
        context: &C) -> Box<dyn Future<Item=GetLedErrorResponse, Error=ApiError> + Send>;

    fn get_led_info(
        &self,
        bus_id: i32,
        addr: i32,
        led: i32,
        context: &C) -> Box<dyn Future<Item=GetLedInfoResponse, Error=ApiError> + Send>;

    fn get_led_info_all(
        &self,
        bus_id: i32,
        addr: i32,
        context: &C) -> Box<dyn Future<Item=GetLedInfoAllResponse, Error=ApiError> + Send>;

    fn get_led_pwm(
        &self,
        bus_id: i32,
        addr: i32,
        led: i32,
        context: &C) -> Box<dyn Future<Item=GetLedPwmResponse, Error=ApiError> + Send>;

    fn get_led_state(
        &self,
        bus_id: i32,
        addr: i32,
        led: i32,
        context: &C) -> Box<dyn Future<Item=GetLedStateResponse, Error=ApiError> + Send>;

    fn get_offset(
        &self,
        bus_id: i32,
        addr: i32,
        context: &C) -> Box<dyn Future<Item=GetOffsetResponse, Error=ApiError> + Send>;

    fn get_output_change(
        &self,
        bus_id: i32,
        addr: i32,
        context: &C) -> Box<dyn Future<Item=GetOutputChangeResponse, Error=ApiError> + Send>;

    fn get_over_temp(
        &self,
        bus_id: i32,
        addr: i32,
        context: &C) -> Box<dyn Future<Item=GetOverTempResponse, Error=ApiError> + Send>;

    fn get_pwm(
        &self,
        bus_id: i32,
        addr: i32,
        context: &C) -> Box<dyn Future<Item=GetPwmResponse, Error=ApiError> + Send>;

    fn get_sleep(
        &self,
        bus_id: i32,
        addr: i32,
        context: &C) -> Box<dyn Future<Item=GetSleepResponse, Error=ApiError> + Send>;

    fn reset(
        &self,
        bus_id: i32,
        context: &C) -> Box<dyn Future<Item=ResetResponse, Error=ApiError> + Send>;

    fn set_addr_enabled(
        &self,
        bus_id: i32,
        addr: i32,
        num: i32,
        enabled: bool,
        context: &C) -> Box<dyn Future<Item=SetAddrEnabledResponse, Error=ApiError> + Send>;

    fn set_addr_value(
        &self,
        bus_id: i32,
        addr: i32,
        num: i32,
        addr_val: i32,
        context: &C) -> Box<dyn Future<Item=SetAddrValueResponse, Error=ApiError> + Send>;

    fn set_config(
        &self,
        bus_id: i32,
        addr: i32,
        config: models::Config,
        context: &C) -> Box<dyn Future<Item=SetConfigResponse, Error=ApiError> + Send>;

    fn set_current(
        &self,
        bus_id: i32,
        addr: i32,
        current: i32,
        context: &C) -> Box<dyn Future<Item=SetCurrentResponse, Error=ApiError> + Send>;

    fn set_freq(
        &self,
        bus_id: i32,
        addr: i32,
        freq: i32,
        context: &C) -> Box<dyn Future<Item=SetFreqResponse, Error=ApiError> + Send>;

    fn set_group(
        &self,
        bus_id: i32,
        addr: i32,
        group: models::Group,
        context: &C) -> Box<dyn Future<Item=SetGroupResponse, Error=ApiError> + Send>;

    fn set_led_current(
        &self,
        bus_id: i32,
        addr: i32,
        led: i32,
        current: i32,
        context: &C) -> Box<dyn Future<Item=SetLedCurrentResponse, Error=ApiError> + Send>;

    fn set_led_error(
        &self,
        bus_id: i32,
        addr: i32,
        led: i32,
        error: models::LedError,
        context: &C) -> Box<dyn Future<Item=SetLedErrorResponse, Error=ApiError> + Send>;

    fn set_led_info(
        &self,
        bus_id: i32,
        addr: i32,
        led: i32,
        led_info: models::LedInfo,
        context: &C) -> Box<dyn Future<Item=SetLedInfoResponse, Error=ApiError> + Send>;

    fn set_led_info_all(
        &self,
        bus_id: i32,
        addr: i32,
        led_info_array: models::LedInfoArray,
        context: &C) -> Box<dyn Future<Item=SetLedInfoAllResponse, Error=ApiError> + Send>;

    fn set_led_pwm(
        &self,
        bus_id: i32,
        addr: i32,
        led: i32,
        pwm: i32,
        context: &C) -> Box<dyn Future<Item=SetLedPwmResponse, Error=ApiError> + Send>;

    fn set_led_state(
        &self,
        bus_id: i32,
        addr: i32,
        led: i32,
        state: models::LedState,
        context: &C) -> Box<dyn Future<Item=SetLedStateResponse, Error=ApiError> + Send>;

    fn set_offset(
        &self,
        bus_id: i32,
        addr: i32,
        offset: i32,
        context: &C) -> Box<dyn Future<Item=SetOffsetResponse, Error=ApiError> + Send>;

    fn set_output_change(
        &self,
        bus_id: i32,
        addr: i32,
        output_change: models::OutputChange,
        context: &C) -> Box<dyn Future<Item=SetOutputChangeResponse, Error=ApiError> + Send>;

    fn set_pwm(
        &self,
        bus_id: i32,
        addr: i32,
        pwm: i32,
        context: &C) -> Box<dyn Future<Item=SetPwmResponse, Error=ApiError> + Send>;

    fn set_sleep(
        &self,
        bus_id: i32,
        addr: i32,
        sleep: bool,
        context: &C) -> Box<dyn Future<Item=SetSleepResponse, Error=ApiError> + Send>;

}

/// API without a `Context`
pub trait ApiNoContext {
    fn clear_error(
        &self,
        bus_id: i32,
        addr: i32,
        ) -> Box<dyn Future<Item=ClearErrorResponse, Error=ApiError> + Send>;

    fn get_addr_enabled(
        &self,
        bus_id: i32,
        addr: i32,
        num: i32,
        ) -> Box<dyn Future<Item=GetAddrEnabledResponse, Error=ApiError> + Send>;

    fn get_addr_info(
        &self,
        bus_id: i32,
        addr: i32,
        num: i32,
        ) -> Box<dyn Future<Item=GetAddrInfoResponse, Error=ApiError> + Send>;

    fn get_addr_value(
        &self,
        bus_id: i32,
        addr: i32,
        num: i32,
        ) -> Box<dyn Future<Item=GetAddrValueResponse, Error=ApiError> + Send>;

    fn get_api(
        &self,
        ) -> Box<dyn Future<Item=GetApiResponse, Error=ApiError> + Send>;

    fn get_config(
        &self,
        bus_id: i32,
        addr: i32,
        ) -> Box<dyn Future<Item=GetConfigResponse, Error=ApiError> + Send>;

    fn get_current(
        &self,
        bus_id: i32,
        addr: i32,
        ) -> Box<dyn Future<Item=GetCurrentResponse, Error=ApiError> + Send>;

    fn get_error(
        &self,
        bus_id: i32,
        addr: i32,
        ) -> Box<dyn Future<Item=GetErrorResponse, Error=ApiError> + Send>;

    fn get_errors(
        &self,
        bus_id: i32,
        addr: i32,
        ) -> Box<dyn Future<Item=GetErrorsResponse, Error=ApiError> + Send>;

    fn get_freq(
        &self,
        bus_id: i32,
        addr: i32,
        ) -> Box<dyn Future<Item=GetFreqResponse, Error=ApiError> + Send>;

    fn get_group(
        &self,
        bus_id: i32,
        addr: i32,
        ) -> Box<dyn Future<Item=GetGroupResponse, Error=ApiError> + Send>;

    fn get_led_current(
        &self,
        bus_id: i32,
        addr: i32,
        led: i32,
        ) -> Box<dyn Future<Item=GetLedCurrentResponse, Error=ApiError> + Send>;

    fn get_led_error(
        &self,
        bus_id: i32,
        addr: i32,
        led: i32,
        ) -> Box<dyn Future<Item=GetLedErrorResponse, Error=ApiError> + Send>;

    fn get_led_info(
        &self,
        bus_id: i32,
        addr: i32,
        led: i32,
        ) -> Box<dyn Future<Item=GetLedInfoResponse, Error=ApiError> + Send>;

    fn get_led_info_all(
        &self,
        bus_id: i32,
        addr: i32,
        ) -> Box<dyn Future<Item=GetLedInfoAllResponse, Error=ApiError> + Send>;

    fn get_led_pwm(
        &self,
        bus_id: i32,
        addr: i32,
        led: i32,
        ) -> Box<dyn Future<Item=GetLedPwmResponse, Error=ApiError> + Send>;

    fn get_led_state(
        &self,
        bus_id: i32,
        addr: i32,
        led: i32,
        ) -> Box<dyn Future<Item=GetLedStateResponse, Error=ApiError> + Send>;

    fn get_offset(
        &self,
        bus_id: i32,
        addr: i32,
        ) -> Box<dyn Future<Item=GetOffsetResponse, Error=ApiError> + Send>;

    fn get_output_change(
        &self,
        bus_id: i32,
        addr: i32,
        ) -> Box<dyn Future<Item=GetOutputChangeResponse, Error=ApiError> + Send>;

    fn get_over_temp(
        &self,
        bus_id: i32,
        addr: i32,
        ) -> Box<dyn Future<Item=GetOverTempResponse, Error=ApiError> + Send>;

    fn get_pwm(
        &self,
        bus_id: i32,
        addr: i32,
        ) -> Box<dyn Future<Item=GetPwmResponse, Error=ApiError> + Send>;

    fn get_sleep(
        &self,
        bus_id: i32,
        addr: i32,
        ) -> Box<dyn Future<Item=GetSleepResponse, Error=ApiError> + Send>;

    fn reset(
        &self,
        bus_id: i32,
        ) -> Box<dyn Future<Item=ResetResponse, Error=ApiError> + Send>;

    fn set_addr_enabled(
        &self,
        bus_id: i32,
        addr: i32,
        num: i32,
        enabled: bool,
        ) -> Box<dyn Future<Item=SetAddrEnabledResponse, Error=ApiError> + Send>;

    fn set_addr_value(
        &self,
        bus_id: i32,
        addr: i32,
        num: i32,
        addr_val: i32,
        ) -> Box<dyn Future<Item=SetAddrValueResponse, Error=ApiError> + Send>;

    fn set_config(
        &self,
        bus_id: i32,
        addr: i32,
        config: models::Config,
        ) -> Box<dyn Future<Item=SetConfigResponse, Error=ApiError> + Send>;

    fn set_current(
        &self,
        bus_id: i32,
        addr: i32,
        current: i32,
        ) -> Box<dyn Future<Item=SetCurrentResponse, Error=ApiError> + Send>;

    fn set_freq(
        &self,
        bus_id: i32,
        addr: i32,
        freq: i32,
        ) -> Box<dyn Future<Item=SetFreqResponse, Error=ApiError> + Send>;

    fn set_group(
        &self,
        bus_id: i32,
        addr: i32,
        group: models::Group,
        ) -> Box<dyn Future<Item=SetGroupResponse, Error=ApiError> + Send>;

    fn set_led_current(
        &self,
        bus_id: i32,
        addr: i32,
        led: i32,
        current: i32,
        ) -> Box<dyn Future<Item=SetLedCurrentResponse, Error=ApiError> + Send>;

    fn set_led_error(
        &self,
        bus_id: i32,
        addr: i32,
        led: i32,
        error: models::LedError,
        ) -> Box<dyn Future<Item=SetLedErrorResponse, Error=ApiError> + Send>;

    fn set_led_info(
        &self,
        bus_id: i32,
        addr: i32,
        led: i32,
        led_info: models::LedInfo,
        ) -> Box<dyn Future<Item=SetLedInfoResponse, Error=ApiError> + Send>;

    fn set_led_info_all(
        &self,
        bus_id: i32,
        addr: i32,
        led_info_array: models::LedInfoArray,
        ) -> Box<dyn Future<Item=SetLedInfoAllResponse, Error=ApiError> + Send>;

    fn set_led_pwm(
        &self,
        bus_id: i32,
        addr: i32,
        led: i32,
        pwm: i32,
        ) -> Box<dyn Future<Item=SetLedPwmResponse, Error=ApiError> + Send>;

    fn set_led_state(
        &self,
        bus_id: i32,
        addr: i32,
        led: i32,
        state: models::LedState,
        ) -> Box<dyn Future<Item=SetLedStateResponse, Error=ApiError> + Send>;

    fn set_offset(
        &self,
        bus_id: i32,
        addr: i32,
        offset: i32,
        ) -> Box<dyn Future<Item=SetOffsetResponse, Error=ApiError> + Send>;

    fn set_output_change(
        &self,
        bus_id: i32,
        addr: i32,
        output_change: models::OutputChange,
        ) -> Box<dyn Future<Item=SetOutputChangeResponse, Error=ApiError> + Send>;

    fn set_pwm(
        &self,
        bus_id: i32,
        addr: i32,
        pwm: i32,
        ) -> Box<dyn Future<Item=SetPwmResponse, Error=ApiError> + Send>;

    fn set_sleep(
        &self,
        bus_id: i32,
        addr: i32,
        sleep: bool,
        ) -> Box<dyn Future<Item=SetSleepResponse, Error=ApiError> + Send>;

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
    fn clear_error(
        &self,
        bus_id: i32,
        addr: i32,
        ) -> Box<dyn Future<Item=ClearErrorResponse, Error=ApiError> + Send>
    {
        self.api().clear_error(bus_id, addr, &self.context())
    }

    fn get_addr_enabled(
        &self,
        bus_id: i32,
        addr: i32,
        num: i32,
        ) -> Box<dyn Future<Item=GetAddrEnabledResponse, Error=ApiError> + Send>
    {
        self.api().get_addr_enabled(bus_id, addr, num, &self.context())
    }

    fn get_addr_info(
        &self,
        bus_id: i32,
        addr: i32,
        num: i32,
        ) -> Box<dyn Future<Item=GetAddrInfoResponse, Error=ApiError> + Send>
    {
        self.api().get_addr_info(bus_id, addr, num, &self.context())
    }

    fn get_addr_value(
        &self,
        bus_id: i32,
        addr: i32,
        num: i32,
        ) -> Box<dyn Future<Item=GetAddrValueResponse, Error=ApiError> + Send>
    {
        self.api().get_addr_value(bus_id, addr, num, &self.context())
    }

    fn get_api(
        &self,
        ) -> Box<dyn Future<Item=GetApiResponse, Error=ApiError> + Send>
    {
        self.api().get_api(&self.context())
    }

    fn get_config(
        &self,
        bus_id: i32,
        addr: i32,
        ) -> Box<dyn Future<Item=GetConfigResponse, Error=ApiError> + Send>
    {
        self.api().get_config(bus_id, addr, &self.context())
    }

    fn get_current(
        &self,
        bus_id: i32,
        addr: i32,
        ) -> Box<dyn Future<Item=GetCurrentResponse, Error=ApiError> + Send>
    {
        self.api().get_current(bus_id, addr, &self.context())
    }

    fn get_error(
        &self,
        bus_id: i32,
        addr: i32,
        ) -> Box<dyn Future<Item=GetErrorResponse, Error=ApiError> + Send>
    {
        self.api().get_error(bus_id, addr, &self.context())
    }

    fn get_errors(
        &self,
        bus_id: i32,
        addr: i32,
        ) -> Box<dyn Future<Item=GetErrorsResponse, Error=ApiError> + Send>
    {
        self.api().get_errors(bus_id, addr, &self.context())
    }

    fn get_freq(
        &self,
        bus_id: i32,
        addr: i32,
        ) -> Box<dyn Future<Item=GetFreqResponse, Error=ApiError> + Send>
    {
        self.api().get_freq(bus_id, addr, &self.context())
    }

    fn get_group(
        &self,
        bus_id: i32,
        addr: i32,
        ) -> Box<dyn Future<Item=GetGroupResponse, Error=ApiError> + Send>
    {
        self.api().get_group(bus_id, addr, &self.context())
    }

    fn get_led_current(
        &self,
        bus_id: i32,
        addr: i32,
        led: i32,
        ) -> Box<dyn Future<Item=GetLedCurrentResponse, Error=ApiError> + Send>
    {
        self.api().get_led_current(bus_id, addr, led, &self.context())
    }

    fn get_led_error(
        &self,
        bus_id: i32,
        addr: i32,
        led: i32,
        ) -> Box<dyn Future<Item=GetLedErrorResponse, Error=ApiError> + Send>
    {
        self.api().get_led_error(bus_id, addr, led, &self.context())
    }

    fn get_led_info(
        &self,
        bus_id: i32,
        addr: i32,
        led: i32,
        ) -> Box<dyn Future<Item=GetLedInfoResponse, Error=ApiError> + Send>
    {
        self.api().get_led_info(bus_id, addr, led, &self.context())
    }

    fn get_led_info_all(
        &self,
        bus_id: i32,
        addr: i32,
        ) -> Box<dyn Future<Item=GetLedInfoAllResponse, Error=ApiError> + Send>
    {
        self.api().get_led_info_all(bus_id, addr, &self.context())
    }

    fn get_led_pwm(
        &self,
        bus_id: i32,
        addr: i32,
        led: i32,
        ) -> Box<dyn Future<Item=GetLedPwmResponse, Error=ApiError> + Send>
    {
        self.api().get_led_pwm(bus_id, addr, led, &self.context())
    }

    fn get_led_state(
        &self,
        bus_id: i32,
        addr: i32,
        led: i32,
        ) -> Box<dyn Future<Item=GetLedStateResponse, Error=ApiError> + Send>
    {
        self.api().get_led_state(bus_id, addr, led, &self.context())
    }

    fn get_offset(
        &self,
        bus_id: i32,
        addr: i32,
        ) -> Box<dyn Future<Item=GetOffsetResponse, Error=ApiError> + Send>
    {
        self.api().get_offset(bus_id, addr, &self.context())
    }

    fn get_output_change(
        &self,
        bus_id: i32,
        addr: i32,
        ) -> Box<dyn Future<Item=GetOutputChangeResponse, Error=ApiError> + Send>
    {
        self.api().get_output_change(bus_id, addr, &self.context())
    }

    fn get_over_temp(
        &self,
        bus_id: i32,
        addr: i32,
        ) -> Box<dyn Future<Item=GetOverTempResponse, Error=ApiError> + Send>
    {
        self.api().get_over_temp(bus_id, addr, &self.context())
    }

    fn get_pwm(
        &self,
        bus_id: i32,
        addr: i32,
        ) -> Box<dyn Future<Item=GetPwmResponse, Error=ApiError> + Send>
    {
        self.api().get_pwm(bus_id, addr, &self.context())
    }

    fn get_sleep(
        &self,
        bus_id: i32,
        addr: i32,
        ) -> Box<dyn Future<Item=GetSleepResponse, Error=ApiError> + Send>
    {
        self.api().get_sleep(bus_id, addr, &self.context())
    }

    fn reset(
        &self,
        bus_id: i32,
        ) -> Box<dyn Future<Item=ResetResponse, Error=ApiError> + Send>
    {
        self.api().reset(bus_id, &self.context())
    }

    fn set_addr_enabled(
        &self,
        bus_id: i32,
        addr: i32,
        num: i32,
        enabled: bool,
        ) -> Box<dyn Future<Item=SetAddrEnabledResponse, Error=ApiError> + Send>
    {
        self.api().set_addr_enabled(bus_id, addr, num, enabled, &self.context())
    }

    fn set_addr_value(
        &self,
        bus_id: i32,
        addr: i32,
        num: i32,
        addr_val: i32,
        ) -> Box<dyn Future<Item=SetAddrValueResponse, Error=ApiError> + Send>
    {
        self.api().set_addr_value(bus_id, addr, num, addr_val, &self.context())
    }

    fn set_config(
        &self,
        bus_id: i32,
        addr: i32,
        config: models::Config,
        ) -> Box<dyn Future<Item=SetConfigResponse, Error=ApiError> + Send>
    {
        self.api().set_config(bus_id, addr, config, &self.context())
    }

    fn set_current(
        &self,
        bus_id: i32,
        addr: i32,
        current: i32,
        ) -> Box<dyn Future<Item=SetCurrentResponse, Error=ApiError> + Send>
    {
        self.api().set_current(bus_id, addr, current, &self.context())
    }

    fn set_freq(
        &self,
        bus_id: i32,
        addr: i32,
        freq: i32,
        ) -> Box<dyn Future<Item=SetFreqResponse, Error=ApiError> + Send>
    {
        self.api().set_freq(bus_id, addr, freq, &self.context())
    }

    fn set_group(
        &self,
        bus_id: i32,
        addr: i32,
        group: models::Group,
        ) -> Box<dyn Future<Item=SetGroupResponse, Error=ApiError> + Send>
    {
        self.api().set_group(bus_id, addr, group, &self.context())
    }

    fn set_led_current(
        &self,
        bus_id: i32,
        addr: i32,
        led: i32,
        current: i32,
        ) -> Box<dyn Future<Item=SetLedCurrentResponse, Error=ApiError> + Send>
    {
        self.api().set_led_current(bus_id, addr, led, current, &self.context())
    }

    fn set_led_error(
        &self,
        bus_id: i32,
        addr: i32,
        led: i32,
        error: models::LedError,
        ) -> Box<dyn Future<Item=SetLedErrorResponse, Error=ApiError> + Send>
    {
        self.api().set_led_error(bus_id, addr, led, error, &self.context())
    }

    fn set_led_info(
        &self,
        bus_id: i32,
        addr: i32,
        led: i32,
        led_info: models::LedInfo,
        ) -> Box<dyn Future<Item=SetLedInfoResponse, Error=ApiError> + Send>
    {
        self.api().set_led_info(bus_id, addr, led, led_info, &self.context())
    }

    fn set_led_info_all(
        &self,
        bus_id: i32,
        addr: i32,
        led_info_array: models::LedInfoArray,
        ) -> Box<dyn Future<Item=SetLedInfoAllResponse, Error=ApiError> + Send>
    {
        self.api().set_led_info_all(bus_id, addr, led_info_array, &self.context())
    }

    fn set_led_pwm(
        &self,
        bus_id: i32,
        addr: i32,
        led: i32,
        pwm: i32,
        ) -> Box<dyn Future<Item=SetLedPwmResponse, Error=ApiError> + Send>
    {
        self.api().set_led_pwm(bus_id, addr, led, pwm, &self.context())
    }

    fn set_led_state(
        &self,
        bus_id: i32,
        addr: i32,
        led: i32,
        state: models::LedState,
        ) -> Box<dyn Future<Item=SetLedStateResponse, Error=ApiError> + Send>
    {
        self.api().set_led_state(bus_id, addr, led, state, &self.context())
    }

    fn set_offset(
        &self,
        bus_id: i32,
        addr: i32,
        offset: i32,
        ) -> Box<dyn Future<Item=SetOffsetResponse, Error=ApiError> + Send>
    {
        self.api().set_offset(bus_id, addr, offset, &self.context())
    }

    fn set_output_change(
        &self,
        bus_id: i32,
        addr: i32,
        output_change: models::OutputChange,
        ) -> Box<dyn Future<Item=SetOutputChangeResponse, Error=ApiError> + Send>
    {
        self.api().set_output_change(bus_id, addr, output_change, &self.context())
    }

    fn set_pwm(
        &self,
        bus_id: i32,
        addr: i32,
        pwm: i32,
        ) -> Box<dyn Future<Item=SetPwmResponse, Error=ApiError> + Send>
    {
        self.api().set_pwm(bus_id, addr, pwm, &self.context())
    }

    fn set_sleep(
        &self,
        bus_id: i32,
        addr: i32,
        sleep: bool,
        ) -> Box<dyn Future<Item=SetSleepResponse, Error=ApiError> + Send>
    {
        self.api().set_sleep(bus_id, addr, sleep, &self.context())
    }

}

#[cfg(feature = "client")]
pub mod client;

// Re-export Client as a top-level name
#[cfg(feature = "client")]
pub use client::Client;

#[cfg(feature = "server")]
pub mod server;

// Re-export router() as a top-level name
#[cfg(feature = "server")]
pub use self::server::Service;

#[cfg(feature = "server")]
pub mod context;

pub mod models;

#[cfg(any(feature = "client", feature = "server"))]
pub(crate) mod header;
