#![allow(unused_extern_crates)]
extern crate serde_ignored;
extern crate tokio_core;
extern crate native_tls;
extern crate hyper_tls;
extern crate openssl;
extern crate mime;
extern crate uuid;
extern crate chrono;
extern crate percent_encoding;
extern crate url;


use std::sync::Arc;
use std::marker::PhantomData;
use futures::{Future, future, Stream, stream};
use hyper;
use hyper::{Request, Response, Error, StatusCode};
use hyper::header::{Headers, ContentType};
use self::url::form_urlencoded;
use mimetypes;

use serde_json;


#[allow(unused_imports)]
use std::collections::{HashMap, BTreeMap};
#[allow(unused_imports)]
use swagger;
use std::io;

#[allow(unused_imports)]
use std::collections::BTreeSet;

pub use swagger::auth::Authorization;
use swagger::{ApiError, XSpanId, XSpanIdString, Has, RequestParser};
use swagger::auth::Scopes;

use {Api,
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
#[allow(unused_imports)]
use models;

pub mod context;

header! { (Warning, "Warning") => [String] }

mod paths {
    extern crate regex;

    lazy_static! {
        pub static ref GLOBAL_REGEX_SET: regex::RegexSet = regex::RegexSet::new(&vec![
            r"^/pca9956b/api$",
            r"^/pca9956b/(?P<busId>[^/?#]*)/reset$",
            r"^/pca9956b/(?P<busId>[^/?#]*)/(?P<addr>[^/?#]*)/addr/(?P<num>[^/?#]*)$",
            r"^/pca9956b/(?P<busId>[^/?#]*)/(?P<addr>[^/?#]*)/addr/(?P<num>[^/?#]*)/addr$",
            r"^/pca9956b/(?P<busId>[^/?#]*)/(?P<addr>[^/?#]*)/addr/(?P<num>[^/?#]*)/addr/(?P<addrVal>[^/?#]*)$",
            r"^/pca9956b/(?P<busId>[^/?#]*)/(?P<addr>[^/?#]*)/addr/(?P<num>[^/?#]*)/enabled$",
            r"^/pca9956b/(?P<busId>[^/?#]*)/(?P<addr>[^/?#]*)/addr/(?P<num>[^/?#]*)/enabled/(?P<enabled>[^/?#]*)$",
            r"^/pca9956b/(?P<busId>[^/?#]*)/(?P<addr>[^/?#]*)/config$",
            r"^/pca9956b/(?P<busId>[^/?#]*)/(?P<addr>[^/?#]*)/current$",
            r"^/pca9956b/(?P<busId>[^/?#]*)/(?P<addr>[^/?#]*)/current/(?P<current>[^/?#]*)$",
            r"^/pca9956b/(?P<busId>[^/?#]*)/(?P<addr>[^/?#]*)/error$",
            r"^/pca9956b/(?P<busId>[^/?#]*)/(?P<addr>[^/?#]*)/error/clear$",
            r"^/pca9956b/(?P<busId>[^/?#]*)/(?P<addr>[^/?#]*)/errors$",
            r"^/pca9956b/(?P<busId>[^/?#]*)/(?P<addr>[^/?#]*)/freq$",
            r"^/pca9956b/(?P<busId>[^/?#]*)/(?P<addr>[^/?#]*)/freq/(?P<freq>[^/?#]*)$",
            r"^/pca9956b/(?P<busId>[^/?#]*)/(?P<addr>[^/?#]*)/group$",
            r"^/pca9956b/(?P<busId>[^/?#]*)/(?P<addr>[^/?#]*)/group/(?P<group>[^/?#]*)$",
            r"^/pca9956b/(?P<busId>[^/?#]*)/(?P<addr>[^/?#]*)/led$",
            r"^/pca9956b/(?P<busId>[^/?#]*)/(?P<addr>[^/?#]*)/led/(?P<led>[^/?#]*)$",
            r"^/pca9956b/(?P<busId>[^/?#]*)/(?P<addr>[^/?#]*)/led/(?P<led>[^/?#]*)/current$",
            r"^/pca9956b/(?P<busId>[^/?#]*)/(?P<addr>[^/?#]*)/led/(?P<led>[^/?#]*)/current/(?P<current>[^/?#]*)$",
            r"^/pca9956b/(?P<busId>[^/?#]*)/(?P<addr>[^/?#]*)/led/(?P<led>[^/?#]*)/error$",
            r"^/pca9956b/(?P<busId>[^/?#]*)/(?P<addr>[^/?#]*)/led/(?P<led>[^/?#]*)/error/(?P<error>[^/?#]*)$",
            r"^/pca9956b/(?P<busId>[^/?#]*)/(?P<addr>[^/?#]*)/led/(?P<led>[^/?#]*)/pwm$",
            r"^/pca9956b/(?P<busId>[^/?#]*)/(?P<addr>[^/?#]*)/led/(?P<led>[^/?#]*)/pwm/(?P<pwm>[^/?#]*)$",
            r"^/pca9956b/(?P<busId>[^/?#]*)/(?P<addr>[^/?#]*)/led/(?P<led>[^/?#]*)/state$",
            r"^/pca9956b/(?P<busId>[^/?#]*)/(?P<addr>[^/?#]*)/led/(?P<led>[^/?#]*)/state/(?P<state>[^/?#]*)$",
            r"^/pca9956b/(?P<busId>[^/?#]*)/(?P<addr>[^/?#]*)/offset$",
            r"^/pca9956b/(?P<busId>[^/?#]*)/(?P<addr>[^/?#]*)/offset/(?P<offset>[^/?#]*)$",
            r"^/pca9956b/(?P<busId>[^/?#]*)/(?P<addr>[^/?#]*)/outputChange$",
            r"^/pca9956b/(?P<busId>[^/?#]*)/(?P<addr>[^/?#]*)/outputChange/(?P<outputChange>[^/?#]*)$",
            r"^/pca9956b/(?P<busId>[^/?#]*)/(?P<addr>[^/?#]*)/overTemp$",
            r"^/pca9956b/(?P<busId>[^/?#]*)/(?P<addr>[^/?#]*)/pwm$",
            r"^/pca9956b/(?P<busId>[^/?#]*)/(?P<addr>[^/?#]*)/pwm/(?P<pwm>[^/?#]*)$",
            r"^/pca9956b/(?P<busId>[^/?#]*)/(?P<addr>[^/?#]*)/sleep$",
            r"^/pca9956b/(?P<busId>[^/?#]*)/(?P<addr>[^/?#]*)/sleep/(?P<sleep>[^/?#]*)$"
        ]).unwrap();
    }
    pub static ID_PCA9956B_API: usize = 0;
    pub static ID_PCA9956B_BUSID_RESET: usize = 1;
    lazy_static! {
        pub static ref REGEX_PCA9956B_BUSID_RESET: regex::Regex = regex::Regex::new(r"^/pca9956b/(?P<busId>[^/?#]*)/reset$").unwrap();
    }
    pub static ID_PCA9956B_BUSID_ADDR_ADDR_NUM: usize = 2;
    lazy_static! {
        pub static ref REGEX_PCA9956B_BUSID_ADDR_ADDR_NUM: regex::Regex = regex::Regex::new(r"^/pca9956b/(?P<busId>[^/?#]*)/(?P<addr>[^/?#]*)/addr/(?P<num>[^/?#]*)$").unwrap();
    }
    pub static ID_PCA9956B_BUSID_ADDR_ADDR_NUM_ADDR: usize = 3;
    lazy_static! {
        pub static ref REGEX_PCA9956B_BUSID_ADDR_ADDR_NUM_ADDR: regex::Regex = regex::Regex::new(r"^/pca9956b/(?P<busId>[^/?#]*)/(?P<addr>[^/?#]*)/addr/(?P<num>[^/?#]*)/addr$").unwrap();
    }
    pub static ID_PCA9956B_BUSID_ADDR_ADDR_NUM_ADDR_ADDRVAL: usize = 4;
    lazy_static! {
        pub static ref REGEX_PCA9956B_BUSID_ADDR_ADDR_NUM_ADDR_ADDRVAL: regex::Regex = regex::Regex::new(r"^/pca9956b/(?P<busId>[^/?#]*)/(?P<addr>[^/?#]*)/addr/(?P<num>[^/?#]*)/addr/(?P<addrVal>[^/?#]*)$").unwrap();
    }
    pub static ID_PCA9956B_BUSID_ADDR_ADDR_NUM_ENABLED: usize = 5;
    lazy_static! {
        pub static ref REGEX_PCA9956B_BUSID_ADDR_ADDR_NUM_ENABLED: regex::Regex = regex::Regex::new(r"^/pca9956b/(?P<busId>[^/?#]*)/(?P<addr>[^/?#]*)/addr/(?P<num>[^/?#]*)/enabled$").unwrap();
    }
    pub static ID_PCA9956B_BUSID_ADDR_ADDR_NUM_ENABLED_ENABLED: usize = 6;
    lazy_static! {
        pub static ref REGEX_PCA9956B_BUSID_ADDR_ADDR_NUM_ENABLED_ENABLED: regex::Regex = regex::Regex::new(r"^/pca9956b/(?P<busId>[^/?#]*)/(?P<addr>[^/?#]*)/addr/(?P<num>[^/?#]*)/enabled/(?P<enabled>[^/?#]*)$").unwrap();
    }
    pub static ID_PCA9956B_BUSID_ADDR_CONFIG: usize = 7;
    lazy_static! {
        pub static ref REGEX_PCA9956B_BUSID_ADDR_CONFIG: regex::Regex = regex::Regex::new(r"^/pca9956b/(?P<busId>[^/?#]*)/(?P<addr>[^/?#]*)/config$").unwrap();
    }
    pub static ID_PCA9956B_BUSID_ADDR_CURRENT: usize = 8;
    lazy_static! {
        pub static ref REGEX_PCA9956B_BUSID_ADDR_CURRENT: regex::Regex = regex::Regex::new(r"^/pca9956b/(?P<busId>[^/?#]*)/(?P<addr>[^/?#]*)/current$").unwrap();
    }
    pub static ID_PCA9956B_BUSID_ADDR_CURRENT_CURRENT: usize = 9;
    lazy_static! {
        pub static ref REGEX_PCA9956B_BUSID_ADDR_CURRENT_CURRENT: regex::Regex = regex::Regex::new(r"^/pca9956b/(?P<busId>[^/?#]*)/(?P<addr>[^/?#]*)/current/(?P<current>[^/?#]*)$").unwrap();
    }
    pub static ID_PCA9956B_BUSID_ADDR_ERROR: usize = 10;
    lazy_static! {
        pub static ref REGEX_PCA9956B_BUSID_ADDR_ERROR: regex::Regex = regex::Regex::new(r"^/pca9956b/(?P<busId>[^/?#]*)/(?P<addr>[^/?#]*)/error$").unwrap();
    }
    pub static ID_PCA9956B_BUSID_ADDR_ERROR_CLEAR: usize = 11;
    lazy_static! {
        pub static ref REGEX_PCA9956B_BUSID_ADDR_ERROR_CLEAR: regex::Regex = regex::Regex::new(r"^/pca9956b/(?P<busId>[^/?#]*)/(?P<addr>[^/?#]*)/error/clear$").unwrap();
    }
    pub static ID_PCA9956B_BUSID_ADDR_ERRORS: usize = 12;
    lazy_static! {
        pub static ref REGEX_PCA9956B_BUSID_ADDR_ERRORS: regex::Regex = regex::Regex::new(r"^/pca9956b/(?P<busId>[^/?#]*)/(?P<addr>[^/?#]*)/errors$").unwrap();
    }
    pub static ID_PCA9956B_BUSID_ADDR_FREQ: usize = 13;
    lazy_static! {
        pub static ref REGEX_PCA9956B_BUSID_ADDR_FREQ: regex::Regex = regex::Regex::new(r"^/pca9956b/(?P<busId>[^/?#]*)/(?P<addr>[^/?#]*)/freq$").unwrap();
    }
    pub static ID_PCA9956B_BUSID_ADDR_FREQ_FREQ: usize = 14;
    lazy_static! {
        pub static ref REGEX_PCA9956B_BUSID_ADDR_FREQ_FREQ: regex::Regex = regex::Regex::new(r"^/pca9956b/(?P<busId>[^/?#]*)/(?P<addr>[^/?#]*)/freq/(?P<freq>[^/?#]*)$").unwrap();
    }
    pub static ID_PCA9956B_BUSID_ADDR_GROUP: usize = 15;
    lazy_static! {
        pub static ref REGEX_PCA9956B_BUSID_ADDR_GROUP: regex::Regex = regex::Regex::new(r"^/pca9956b/(?P<busId>[^/?#]*)/(?P<addr>[^/?#]*)/group$").unwrap();
    }
    pub static ID_PCA9956B_BUSID_ADDR_GROUP_GROUP: usize = 16;
    lazy_static! {
        pub static ref REGEX_PCA9956B_BUSID_ADDR_GROUP_GROUP: regex::Regex = regex::Regex::new(r"^/pca9956b/(?P<busId>[^/?#]*)/(?P<addr>[^/?#]*)/group/(?P<group>[^/?#]*)$").unwrap();
    }
    pub static ID_PCA9956B_BUSID_ADDR_LED: usize = 17;
    lazy_static! {
        pub static ref REGEX_PCA9956B_BUSID_ADDR_LED: regex::Regex = regex::Regex::new(r"^/pca9956b/(?P<busId>[^/?#]*)/(?P<addr>[^/?#]*)/led$").unwrap();
    }
    pub static ID_PCA9956B_BUSID_ADDR_LED_LED: usize = 18;
    lazy_static! {
        pub static ref REGEX_PCA9956B_BUSID_ADDR_LED_LED: regex::Regex = regex::Regex::new(r"^/pca9956b/(?P<busId>[^/?#]*)/(?P<addr>[^/?#]*)/led/(?P<led>[^/?#]*)$").unwrap();
    }
    pub static ID_PCA9956B_BUSID_ADDR_LED_LED_CURRENT: usize = 19;
    lazy_static! {
        pub static ref REGEX_PCA9956B_BUSID_ADDR_LED_LED_CURRENT: regex::Regex = regex::Regex::new(r"^/pca9956b/(?P<busId>[^/?#]*)/(?P<addr>[^/?#]*)/led/(?P<led>[^/?#]*)/current$").unwrap();
    }
    pub static ID_PCA9956B_BUSID_ADDR_LED_LED_CURRENT_CURRENT: usize = 20;
    lazy_static! {
        pub static ref REGEX_PCA9956B_BUSID_ADDR_LED_LED_CURRENT_CURRENT: regex::Regex = regex::Regex::new(r"^/pca9956b/(?P<busId>[^/?#]*)/(?P<addr>[^/?#]*)/led/(?P<led>[^/?#]*)/current/(?P<current>[^/?#]*)$").unwrap();
    }
    pub static ID_PCA9956B_BUSID_ADDR_LED_LED_ERROR: usize = 21;
    lazy_static! {
        pub static ref REGEX_PCA9956B_BUSID_ADDR_LED_LED_ERROR: regex::Regex = regex::Regex::new(r"^/pca9956b/(?P<busId>[^/?#]*)/(?P<addr>[^/?#]*)/led/(?P<led>[^/?#]*)/error$").unwrap();
    }
    pub static ID_PCA9956B_BUSID_ADDR_LED_LED_ERROR_ERROR: usize = 22;
    lazy_static! {
        pub static ref REGEX_PCA9956B_BUSID_ADDR_LED_LED_ERROR_ERROR: regex::Regex = regex::Regex::new(r"^/pca9956b/(?P<busId>[^/?#]*)/(?P<addr>[^/?#]*)/led/(?P<led>[^/?#]*)/error/(?P<error>[^/?#]*)$").unwrap();
    }
    pub static ID_PCA9956B_BUSID_ADDR_LED_LED_PWM: usize = 23;
    lazy_static! {
        pub static ref REGEX_PCA9956B_BUSID_ADDR_LED_LED_PWM: regex::Regex = regex::Regex::new(r"^/pca9956b/(?P<busId>[^/?#]*)/(?P<addr>[^/?#]*)/led/(?P<led>[^/?#]*)/pwm$").unwrap();
    }
    pub static ID_PCA9956B_BUSID_ADDR_LED_LED_PWM_PWM: usize = 24;
    lazy_static! {
        pub static ref REGEX_PCA9956B_BUSID_ADDR_LED_LED_PWM_PWM: regex::Regex = regex::Regex::new(r"^/pca9956b/(?P<busId>[^/?#]*)/(?P<addr>[^/?#]*)/led/(?P<led>[^/?#]*)/pwm/(?P<pwm>[^/?#]*)$").unwrap();
    }
    pub static ID_PCA9956B_BUSID_ADDR_LED_LED_STATE: usize = 25;
    lazy_static! {
        pub static ref REGEX_PCA9956B_BUSID_ADDR_LED_LED_STATE: regex::Regex = regex::Regex::new(r"^/pca9956b/(?P<busId>[^/?#]*)/(?P<addr>[^/?#]*)/led/(?P<led>[^/?#]*)/state$").unwrap();
    }
    pub static ID_PCA9956B_BUSID_ADDR_LED_LED_STATE_STATE: usize = 26;
    lazy_static! {
        pub static ref REGEX_PCA9956B_BUSID_ADDR_LED_LED_STATE_STATE: regex::Regex = regex::Regex::new(r"^/pca9956b/(?P<busId>[^/?#]*)/(?P<addr>[^/?#]*)/led/(?P<led>[^/?#]*)/state/(?P<state>[^/?#]*)$").unwrap();
    }
    pub static ID_PCA9956B_BUSID_ADDR_OFFSET: usize = 27;
    lazy_static! {
        pub static ref REGEX_PCA9956B_BUSID_ADDR_OFFSET: regex::Regex = regex::Regex::new(r"^/pca9956b/(?P<busId>[^/?#]*)/(?P<addr>[^/?#]*)/offset$").unwrap();
    }
    pub static ID_PCA9956B_BUSID_ADDR_OFFSET_OFFSET: usize = 28;
    lazy_static! {
        pub static ref REGEX_PCA9956B_BUSID_ADDR_OFFSET_OFFSET: regex::Regex = regex::Regex::new(r"^/pca9956b/(?P<busId>[^/?#]*)/(?P<addr>[^/?#]*)/offset/(?P<offset>[^/?#]*)$").unwrap();
    }
    pub static ID_PCA9956B_BUSID_ADDR_OUTPUTCHANGE: usize = 29;
    lazy_static! {
        pub static ref REGEX_PCA9956B_BUSID_ADDR_OUTPUTCHANGE: regex::Regex = regex::Regex::new(r"^/pca9956b/(?P<busId>[^/?#]*)/(?P<addr>[^/?#]*)/outputChange$").unwrap();
    }
    pub static ID_PCA9956B_BUSID_ADDR_OUTPUTCHANGE_OUTPUTCHANGE: usize = 30;
    lazy_static! {
        pub static ref REGEX_PCA9956B_BUSID_ADDR_OUTPUTCHANGE_OUTPUTCHANGE: regex::Regex = regex::Regex::new(r"^/pca9956b/(?P<busId>[^/?#]*)/(?P<addr>[^/?#]*)/outputChange/(?P<outputChange>[^/?#]*)$").unwrap();
    }
    pub static ID_PCA9956B_BUSID_ADDR_OVERTEMP: usize = 31;
    lazy_static! {
        pub static ref REGEX_PCA9956B_BUSID_ADDR_OVERTEMP: regex::Regex = regex::Regex::new(r"^/pca9956b/(?P<busId>[^/?#]*)/(?P<addr>[^/?#]*)/overTemp$").unwrap();
    }
    pub static ID_PCA9956B_BUSID_ADDR_PWM: usize = 32;
    lazy_static! {
        pub static ref REGEX_PCA9956B_BUSID_ADDR_PWM: regex::Regex = regex::Regex::new(r"^/pca9956b/(?P<busId>[^/?#]*)/(?P<addr>[^/?#]*)/pwm$").unwrap();
    }
    pub static ID_PCA9956B_BUSID_ADDR_PWM_PWM: usize = 33;
    lazy_static! {
        pub static ref REGEX_PCA9956B_BUSID_ADDR_PWM_PWM: regex::Regex = regex::Regex::new(r"^/pca9956b/(?P<busId>[^/?#]*)/(?P<addr>[^/?#]*)/pwm/(?P<pwm>[^/?#]*)$").unwrap();
    }
    pub static ID_PCA9956B_BUSID_ADDR_SLEEP: usize = 34;
    lazy_static! {
        pub static ref REGEX_PCA9956B_BUSID_ADDR_SLEEP: regex::Regex = regex::Regex::new(r"^/pca9956b/(?P<busId>[^/?#]*)/(?P<addr>[^/?#]*)/sleep$").unwrap();
    }
    pub static ID_PCA9956B_BUSID_ADDR_SLEEP_SLEEP: usize = 35;
    lazy_static! {
        pub static ref REGEX_PCA9956B_BUSID_ADDR_SLEEP_SLEEP: regex::Regex = regex::Regex::new(r"^/pca9956b/(?P<busId>[^/?#]*)/(?P<addr>[^/?#]*)/sleep/(?P<sleep>[^/?#]*)$").unwrap();
    }
}

pub struct NewService<T, C> {
    api_impl: Arc<T>,
    marker: PhantomData<C>,
}

impl<T, C> NewService<T, C>
where
    T: Api<C> + Clone + 'static,
    C: Has<XSpanIdString>  + 'static
{
    pub fn new<U: Into<Arc<T>>>(api_impl: U) -> NewService<T, C> {
        NewService{api_impl: api_impl.into(), marker: PhantomData}
    }
}

impl<T, C> hyper::server::NewService for NewService<T, C>
where
    T: Api<C> + Clone + 'static,
    C: Has<XSpanIdString>  + 'static
{
    type Request = (Request, C);
    type Response = Response;
    type Error = Error;
    type Instance = Service<T, C>;

    fn new_service(&self) -> Result<Self::Instance, io::Error> {
        Ok(Service::new(self.api_impl.clone()))
    }
}

pub struct Service<T, C> {
    api_impl: Arc<T>,
    marker: PhantomData<C>,
}

impl<T, C> Service<T, C>
where
    T: Api<C> + Clone + 'static,
    C: Has<XSpanIdString>  + 'static {
    pub fn new<U: Into<Arc<T>>>(api_impl: U) -> Service<T, C> {
        Service{api_impl: api_impl.into(), marker: PhantomData}
    }
}

impl<T, C> hyper::server::Service for Service<T, C>
where
    T: Api<C> + Clone + 'static,
    C: Has<XSpanIdString>  + 'static
{
    type Request = (Request, C);
    type Response = Response;
    type Error = Error;
    type Future = Box<Future<Item=Response, Error=Error>>;

    fn call(&self, (req, mut context): Self::Request) -> Self::Future {
        let api_impl = self.api_impl.clone();
        let (method, uri, _, headers, body) = req.deconstruct();
        let path = paths::GLOBAL_REGEX_SET.matches(uri.path());

        // This match statement is duplicated below in `parse_operation_id()`.
        // Please update both places if changing how this code is autogenerated.
        match &method {

            // ClearError - POST /pca9956b/{busId}/{addr}/error/clear
            &hyper::Method::Post if path.matched(paths::ID_PCA9956B_BUSID_ADDR_ERROR_CLEAR) => {


                // Path parameters
                let path = uri.path().to_string();
                let path_params =
                    paths::REGEX_PCA9956B_BUSID_ADDR_ERROR_CLEAR
                    .captures(&path)
                    .unwrap_or_else(||
                        panic!("Path {} matched RE PCA9956B_BUSID_ADDR_ERROR_CLEAR in set but failed match against \"{}\"", path, paths::REGEX_PCA9956B_BUSID_ADDR_ERROR_CLEAR.as_str())
                    );

                let param_bus_id = match percent_encoding::percent_decode(path_params["busId"].as_bytes()).decode_utf8() {
                    Ok(param_bus_id) => match param_bus_id.parse::<i32>() {
                        Ok(param_bus_id) => param_bus_id,
                        Err(e) => return Box::new(future::ok(Response::new().with_status(StatusCode::BadRequest).with_body(format!("Couldn't parse path parameter busId: {}", e)))),
                    },
                    Err(_) => return Box::new(future::ok(Response::new().with_status(StatusCode::BadRequest).with_body(format!("Couldn't percent-decode path parameter as UTF-8: {}", &path_params["busId"]))))
                };
                let param_addr = match percent_encoding::percent_decode(path_params["addr"].as_bytes()).decode_utf8() {
                    Ok(param_addr) => match param_addr.parse::<i32>() {
                        Ok(param_addr) => param_addr,
                        Err(e) => return Box::new(future::ok(Response::new().with_status(StatusCode::BadRequest).with_body(format!("Couldn't parse path parameter addr: {}", e)))),
                    },
                    Err(_) => return Box::new(future::ok(Response::new().with_status(StatusCode::BadRequest).with_body(format!("Couldn't percent-decode path parameter as UTF-8: {}", &path_params["addr"]))))
                };





                Box::new({
                        {{

                                Box::new(api_impl.clear_error(param_bus_id, param_addr, &context)
                                    .then(move |result| {
                                        let mut response = Response::new();
                                        response.headers_mut().set(XSpanId((&context as &Has<XSpanIdString>).get().0.to_string()));

                                        match result {
                                            Ok(rsp) => match rsp {
                                                ClearErrorResponse::OK


                                                => {
                                                    response.set_status(StatusCode::try_from(200).unwrap());

                                                },
                                                ClearErrorResponse::BadRequest

                                                    (body)


                                                => {
                                                    response.set_status(StatusCode::try_from(400).unwrap());

                                                    response.headers_mut().set(ContentType(mimetypes::responses::CLEAR_ERROR_BAD_REQUEST.clone()));


                                                    let body = serde_json::to_string(&body).expect("impossible to fail to serialize");

                                                    response.set_body(body);
                                                },
                                                ClearErrorResponse::OperationFailed

                                                    (body)


                                                => {
                                                    response.set_status(StatusCode::try_from(502).unwrap());

                                                    response.headers_mut().set(ContentType(mimetypes::responses::CLEAR_ERROR_OPERATION_FAILED.clone()));


                                                    let body = serde_json::to_string(&body).expect("impossible to fail to serialize");

                                                    response.set_body(body);
                                                },
                                            },
                                            Err(_) => {
                                                // Application code returned an error. This should not happen, as the implementation should
                                                // return a valid response.
                                                response.set_status(StatusCode::InternalServerError);
                                                response.set_body("An internal error occurred");
                                            },
                                        }

                                        future::ok(response)
                                    }
                                ))

                        }}
                }) as Box<Future<Item=Response, Error=Error>>


            },


            // GetAddrEnabled - GET /pca9956b/{busId}/{addr}/addr/{num}/enabled
            &hyper::Method::Get if path.matched(paths::ID_PCA9956B_BUSID_ADDR_ADDR_NUM_ENABLED) => {


                // Path parameters
                let path = uri.path().to_string();
                let path_params =
                    paths::REGEX_PCA9956B_BUSID_ADDR_ADDR_NUM_ENABLED
                    .captures(&path)
                    .unwrap_or_else(||
                        panic!("Path {} matched RE PCA9956B_BUSID_ADDR_ADDR_NUM_ENABLED in set but failed match against \"{}\"", path, paths::REGEX_PCA9956B_BUSID_ADDR_ADDR_NUM_ENABLED.as_str())
                    );

                let param_bus_id = match percent_encoding::percent_decode(path_params["busId"].as_bytes()).decode_utf8() {
                    Ok(param_bus_id) => match param_bus_id.parse::<i32>() {
                        Ok(param_bus_id) => param_bus_id,
                        Err(e) => return Box::new(future::ok(Response::new().with_status(StatusCode::BadRequest).with_body(format!("Couldn't parse path parameter busId: {}", e)))),
                    },
                    Err(_) => return Box::new(future::ok(Response::new().with_status(StatusCode::BadRequest).with_body(format!("Couldn't percent-decode path parameter as UTF-8: {}", &path_params["busId"]))))
                };
                let param_addr = match percent_encoding::percent_decode(path_params["addr"].as_bytes()).decode_utf8() {
                    Ok(param_addr) => match param_addr.parse::<i32>() {
                        Ok(param_addr) => param_addr,
                        Err(e) => return Box::new(future::ok(Response::new().with_status(StatusCode::BadRequest).with_body(format!("Couldn't parse path parameter addr: {}", e)))),
                    },
                    Err(_) => return Box::new(future::ok(Response::new().with_status(StatusCode::BadRequest).with_body(format!("Couldn't percent-decode path parameter as UTF-8: {}", &path_params["addr"]))))
                };
                let param_num = match percent_encoding::percent_decode(path_params["num"].as_bytes()).decode_utf8() {
                    Ok(param_num) => match param_num.parse::<i32>() {
                        Ok(param_num) => param_num,
                        Err(e) => return Box::new(future::ok(Response::new().with_status(StatusCode::BadRequest).with_body(format!("Couldn't parse path parameter num: {}", e)))),
                    },
                    Err(_) => return Box::new(future::ok(Response::new().with_status(StatusCode::BadRequest).with_body(format!("Couldn't percent-decode path parameter as UTF-8: {}", &path_params["num"]))))
                };





                Box::new({
                        {{

                                Box::new(api_impl.get_addr_enabled(param_bus_id, param_addr, param_num, &context)
                                    .then(move |result| {
                                        let mut response = Response::new();
                                        response.headers_mut().set(XSpanId((&context as &Has<XSpanIdString>).get().0.to_string()));

                                        match result {
                                            Ok(rsp) => match rsp {
                                                GetAddrEnabledResponse::OK

                                                    (body)


                                                => {
                                                    response.set_status(StatusCode::try_from(200).unwrap());

                                                    response.headers_mut().set(ContentType(mimetypes::responses::GET_ADDR_ENABLED_OK.clone()));


                                                    let body = serde_json::to_string(&body).expect("impossible to fail to serialize");

                                                    response.set_body(body);
                                                },
                                                GetAddrEnabledResponse::BadRequest

                                                    (body)


                                                => {
                                                    response.set_status(StatusCode::try_from(400).unwrap());

                                                    response.headers_mut().set(ContentType(mimetypes::responses::GET_ADDR_ENABLED_BAD_REQUEST.clone()));


                                                    let body = serde_json::to_string(&body).expect("impossible to fail to serialize");

                                                    response.set_body(body);
                                                },
                                                GetAddrEnabledResponse::OperationFailed

                                                    (body)


                                                => {
                                                    response.set_status(StatusCode::try_from(502).unwrap());

                                                    response.headers_mut().set(ContentType(mimetypes::responses::GET_ADDR_ENABLED_OPERATION_FAILED.clone()));


                                                    let body = serde_json::to_string(&body).expect("impossible to fail to serialize");

                                                    response.set_body(body);
                                                },
                                            },
                                            Err(_) => {
                                                // Application code returned an error. This should not happen, as the implementation should
                                                // return a valid response.
                                                response.set_status(StatusCode::InternalServerError);
                                                response.set_body("An internal error occurred");
                                            },
                                        }

                                        future::ok(response)
                                    }
                                ))

                        }}
                }) as Box<Future<Item=Response, Error=Error>>


            },


            // GetAddrInfo - GET /pca9956b/{busId}/{addr}/addr/{num}
            &hyper::Method::Get if path.matched(paths::ID_PCA9956B_BUSID_ADDR_ADDR_NUM) => {


                // Path parameters
                let path = uri.path().to_string();
                let path_params =
                    paths::REGEX_PCA9956B_BUSID_ADDR_ADDR_NUM
                    .captures(&path)
                    .unwrap_or_else(||
                        panic!("Path {} matched RE PCA9956B_BUSID_ADDR_ADDR_NUM in set but failed match against \"{}\"", path, paths::REGEX_PCA9956B_BUSID_ADDR_ADDR_NUM.as_str())
                    );

                let param_bus_id = match percent_encoding::percent_decode(path_params["busId"].as_bytes()).decode_utf8() {
                    Ok(param_bus_id) => match param_bus_id.parse::<i32>() {
                        Ok(param_bus_id) => param_bus_id,
                        Err(e) => return Box::new(future::ok(Response::new().with_status(StatusCode::BadRequest).with_body(format!("Couldn't parse path parameter busId: {}", e)))),
                    },
                    Err(_) => return Box::new(future::ok(Response::new().with_status(StatusCode::BadRequest).with_body(format!("Couldn't percent-decode path parameter as UTF-8: {}", &path_params["busId"]))))
                };
                let param_addr = match percent_encoding::percent_decode(path_params["addr"].as_bytes()).decode_utf8() {
                    Ok(param_addr) => match param_addr.parse::<i32>() {
                        Ok(param_addr) => param_addr,
                        Err(e) => return Box::new(future::ok(Response::new().with_status(StatusCode::BadRequest).with_body(format!("Couldn't parse path parameter addr: {}", e)))),
                    },
                    Err(_) => return Box::new(future::ok(Response::new().with_status(StatusCode::BadRequest).with_body(format!("Couldn't percent-decode path parameter as UTF-8: {}", &path_params["addr"]))))
                };
                let param_num = match percent_encoding::percent_decode(path_params["num"].as_bytes()).decode_utf8() {
                    Ok(param_num) => match param_num.parse::<i32>() {
                        Ok(param_num) => param_num,
                        Err(e) => return Box::new(future::ok(Response::new().with_status(StatusCode::BadRequest).with_body(format!("Couldn't parse path parameter num: {}", e)))),
                    },
                    Err(_) => return Box::new(future::ok(Response::new().with_status(StatusCode::BadRequest).with_body(format!("Couldn't percent-decode path parameter as UTF-8: {}", &path_params["num"]))))
                };





                Box::new({
                        {{

                                Box::new(api_impl.get_addr_info(param_bus_id, param_addr, param_num, &context)
                                    .then(move |result| {
                                        let mut response = Response::new();
                                        response.headers_mut().set(XSpanId((&context as &Has<XSpanIdString>).get().0.to_string()));

                                        match result {
                                            Ok(rsp) => match rsp {
                                                GetAddrInfoResponse::OK

                                                    (body)


                                                => {
                                                    response.set_status(StatusCode::try_from(200).unwrap());

                                                    response.headers_mut().set(ContentType(mimetypes::responses::GET_ADDR_INFO_OK.clone()));


                                                    let body = serde_json::to_string(&body).expect("impossible to fail to serialize");

                                                    response.set_body(body);
                                                },
                                                GetAddrInfoResponse::BadRequest

                                                    (body)


                                                => {
                                                    response.set_status(StatusCode::try_from(400).unwrap());

                                                    response.headers_mut().set(ContentType(mimetypes::responses::GET_ADDR_INFO_BAD_REQUEST.clone()));


                                                    let body = serde_json::to_string(&body).expect("impossible to fail to serialize");

                                                    response.set_body(body);
                                                },
                                                GetAddrInfoResponse::OperationFailed

                                                    (body)


                                                => {
                                                    response.set_status(StatusCode::try_from(502).unwrap());

                                                    response.headers_mut().set(ContentType(mimetypes::responses::GET_ADDR_INFO_OPERATION_FAILED.clone()));


                                                    let body = serde_json::to_string(&body).expect("impossible to fail to serialize");

                                                    response.set_body(body);
                                                },
                                            },
                                            Err(_) => {
                                                // Application code returned an error. This should not happen, as the implementation should
                                                // return a valid response.
                                                response.set_status(StatusCode::InternalServerError);
                                                response.set_body("An internal error occurred");
                                            },
                                        }

                                        future::ok(response)
                                    }
                                ))

                        }}
                }) as Box<Future<Item=Response, Error=Error>>


            },


            // GetAddrValue - GET /pca9956b/{busId}/{addr}/addr/{num}/addr
            &hyper::Method::Get if path.matched(paths::ID_PCA9956B_BUSID_ADDR_ADDR_NUM_ADDR) => {


                // Path parameters
                let path = uri.path().to_string();
                let path_params =
                    paths::REGEX_PCA9956B_BUSID_ADDR_ADDR_NUM_ADDR
                    .captures(&path)
                    .unwrap_or_else(||
                        panic!("Path {} matched RE PCA9956B_BUSID_ADDR_ADDR_NUM_ADDR in set but failed match against \"{}\"", path, paths::REGEX_PCA9956B_BUSID_ADDR_ADDR_NUM_ADDR.as_str())
                    );

                let param_bus_id = match percent_encoding::percent_decode(path_params["busId"].as_bytes()).decode_utf8() {
                    Ok(param_bus_id) => match param_bus_id.parse::<i32>() {
                        Ok(param_bus_id) => param_bus_id,
                        Err(e) => return Box::new(future::ok(Response::new().with_status(StatusCode::BadRequest).with_body(format!("Couldn't parse path parameter busId: {}", e)))),
                    },
                    Err(_) => return Box::new(future::ok(Response::new().with_status(StatusCode::BadRequest).with_body(format!("Couldn't percent-decode path parameter as UTF-8: {}", &path_params["busId"]))))
                };
                let param_addr = match percent_encoding::percent_decode(path_params["addr"].as_bytes()).decode_utf8() {
                    Ok(param_addr) => match param_addr.parse::<i32>() {
                        Ok(param_addr) => param_addr,
                        Err(e) => return Box::new(future::ok(Response::new().with_status(StatusCode::BadRequest).with_body(format!("Couldn't parse path parameter addr: {}", e)))),
                    },
                    Err(_) => return Box::new(future::ok(Response::new().with_status(StatusCode::BadRequest).with_body(format!("Couldn't percent-decode path parameter as UTF-8: {}", &path_params["addr"]))))
                };
                let param_num = match percent_encoding::percent_decode(path_params["num"].as_bytes()).decode_utf8() {
                    Ok(param_num) => match param_num.parse::<i32>() {
                        Ok(param_num) => param_num,
                        Err(e) => return Box::new(future::ok(Response::new().with_status(StatusCode::BadRequest).with_body(format!("Couldn't parse path parameter num: {}", e)))),
                    },
                    Err(_) => return Box::new(future::ok(Response::new().with_status(StatusCode::BadRequest).with_body(format!("Couldn't percent-decode path parameter as UTF-8: {}", &path_params["num"]))))
                };





                Box::new({
                        {{

                                Box::new(api_impl.get_addr_value(param_bus_id, param_addr, param_num, &context)
                                    .then(move |result| {
                                        let mut response = Response::new();
                                        response.headers_mut().set(XSpanId((&context as &Has<XSpanIdString>).get().0.to_string()));

                                        match result {
                                            Ok(rsp) => match rsp {
                                                GetAddrValueResponse::OK

                                                    (body)


                                                => {
                                                    response.set_status(StatusCode::try_from(200).unwrap());

                                                    response.headers_mut().set(ContentType(mimetypes::responses::GET_ADDR_VALUE_OK.clone()));


                                                    let body = serde_json::to_string(&body).expect("impossible to fail to serialize");

                                                    response.set_body(body);
                                                },
                                                GetAddrValueResponse::BadRequest

                                                    (body)


                                                => {
                                                    response.set_status(StatusCode::try_from(400).unwrap());

                                                    response.headers_mut().set(ContentType(mimetypes::responses::GET_ADDR_VALUE_BAD_REQUEST.clone()));


                                                    let body = serde_json::to_string(&body).expect("impossible to fail to serialize");

                                                    response.set_body(body);
                                                },
                                                GetAddrValueResponse::OperationFailed

                                                    (body)


                                                => {
                                                    response.set_status(StatusCode::try_from(502).unwrap());

                                                    response.headers_mut().set(ContentType(mimetypes::responses::GET_ADDR_VALUE_OPERATION_FAILED.clone()));


                                                    let body = serde_json::to_string(&body).expect("impossible to fail to serialize");

                                                    response.set_body(body);
                                                },
                                            },
                                            Err(_) => {
                                                // Application code returned an error. This should not happen, as the implementation should
                                                // return a valid response.
                                                response.set_status(StatusCode::InternalServerError);
                                                response.set_body("An internal error occurred");
                                            },
                                        }

                                        future::ok(response)
                                    }
                                ))

                        }}
                }) as Box<Future<Item=Response, Error=Error>>


            },


            // GetApi - GET /pca9956b/api
            &hyper::Method::Get if path.matched(paths::ID_PCA9956B_API) => {







                Box::new({
                        {{

                                Box::new(api_impl.get_api(&context)
                                    .then(move |result| {
                                        let mut response = Response::new();
                                        response.headers_mut().set(XSpanId((&context as &Has<XSpanIdString>).get().0.to_string()));

                                        match result {
                                            Ok(rsp) => match rsp {
                                                GetApiResponse::OK

                                                    (body)


                                                => {
                                                    response.set_status(StatusCode::try_from(200).unwrap());

                                                    response.headers_mut().set(ContentType(mimetypes::responses::GET_API_OK.clone()));


                                                    response.set_body(body);
                                                },
                                                GetApiResponse::FileNotFound

                                                    (body)


                                                => {
                                                    response.set_status(StatusCode::try_from(404).unwrap());

                                                    response.headers_mut().set(ContentType(mimetypes::responses::GET_API_FILE_NOT_FOUND.clone()));


                                                    response.set_body(body);
                                                },
                                            },
                                            Err(_) => {
                                                // Application code returned an error. This should not happen, as the implementation should
                                                // return a valid response.
                                                response.set_status(StatusCode::InternalServerError);
                                                response.set_body("An internal error occurred");
                                            },
                                        }

                                        future::ok(response)
                                    }
                                ))

                        }}
                }) as Box<Future<Item=Response, Error=Error>>


            },


            // GetConfig - GET /pca9956b/{busId}/{addr}/config
            &hyper::Method::Get if path.matched(paths::ID_PCA9956B_BUSID_ADDR_CONFIG) => {


                // Path parameters
                let path = uri.path().to_string();
                let path_params =
                    paths::REGEX_PCA9956B_BUSID_ADDR_CONFIG
                    .captures(&path)
                    .unwrap_or_else(||
                        panic!("Path {} matched RE PCA9956B_BUSID_ADDR_CONFIG in set but failed match against \"{}\"", path, paths::REGEX_PCA9956B_BUSID_ADDR_CONFIG.as_str())
                    );

                let param_bus_id = match percent_encoding::percent_decode(path_params["busId"].as_bytes()).decode_utf8() {
                    Ok(param_bus_id) => match param_bus_id.parse::<i32>() {
                        Ok(param_bus_id) => param_bus_id,
                        Err(e) => return Box::new(future::ok(Response::new().with_status(StatusCode::BadRequest).with_body(format!("Couldn't parse path parameter busId: {}", e)))),
                    },
                    Err(_) => return Box::new(future::ok(Response::new().with_status(StatusCode::BadRequest).with_body(format!("Couldn't percent-decode path parameter as UTF-8: {}", &path_params["busId"]))))
                };
                let param_addr = match percent_encoding::percent_decode(path_params["addr"].as_bytes()).decode_utf8() {
                    Ok(param_addr) => match param_addr.parse::<i32>() {
                        Ok(param_addr) => param_addr,
                        Err(e) => return Box::new(future::ok(Response::new().with_status(StatusCode::BadRequest).with_body(format!("Couldn't parse path parameter addr: {}", e)))),
                    },
                    Err(_) => return Box::new(future::ok(Response::new().with_status(StatusCode::BadRequest).with_body(format!("Couldn't percent-decode path parameter as UTF-8: {}", &path_params["addr"]))))
                };





                Box::new({
                        {{

                                Box::new(api_impl.get_config(param_bus_id, param_addr, &context)
                                    .then(move |result| {
                                        let mut response = Response::new();
                                        response.headers_mut().set(XSpanId((&context as &Has<XSpanIdString>).get().0.to_string()));

                                        match result {
                                            Ok(rsp) => match rsp {
                                                GetConfigResponse::OK

                                                    (body)


                                                => {
                                                    response.set_status(StatusCode::try_from(200).unwrap());

                                                    response.headers_mut().set(ContentType(mimetypes::responses::GET_CONFIG_OK.clone()));


                                                    let body = serde_json::to_string(&body).expect("impossible to fail to serialize");

                                                    response.set_body(body);
                                                },
                                                GetConfigResponse::BadRequest

                                                    (body)


                                                => {
                                                    response.set_status(StatusCode::try_from(400).unwrap());

                                                    response.headers_mut().set(ContentType(mimetypes::responses::GET_CONFIG_BAD_REQUEST.clone()));


                                                    let body = serde_json::to_string(&body).expect("impossible to fail to serialize");

                                                    response.set_body(body);
                                                },
                                                GetConfigResponse::OperationFailed

                                                    (body)


                                                => {
                                                    response.set_status(StatusCode::try_from(502).unwrap());

                                                    response.headers_mut().set(ContentType(mimetypes::responses::GET_CONFIG_OPERATION_FAILED.clone()));


                                                    let body = serde_json::to_string(&body).expect("impossible to fail to serialize");

                                                    response.set_body(body);
                                                },
                                            },
                                            Err(_) => {
                                                // Application code returned an error. This should not happen, as the implementation should
                                                // return a valid response.
                                                response.set_status(StatusCode::InternalServerError);
                                                response.set_body("An internal error occurred");
                                            },
                                        }

                                        future::ok(response)
                                    }
                                ))

                        }}
                }) as Box<Future<Item=Response, Error=Error>>


            },


            // GetCurrent - GET /pca9956b/{busId}/{addr}/current
            &hyper::Method::Get if path.matched(paths::ID_PCA9956B_BUSID_ADDR_CURRENT) => {


                // Path parameters
                let path = uri.path().to_string();
                let path_params =
                    paths::REGEX_PCA9956B_BUSID_ADDR_CURRENT
                    .captures(&path)
                    .unwrap_or_else(||
                        panic!("Path {} matched RE PCA9956B_BUSID_ADDR_CURRENT in set but failed match against \"{}\"", path, paths::REGEX_PCA9956B_BUSID_ADDR_CURRENT.as_str())
                    );

                let param_bus_id = match percent_encoding::percent_decode(path_params["busId"].as_bytes()).decode_utf8() {
                    Ok(param_bus_id) => match param_bus_id.parse::<i32>() {
                        Ok(param_bus_id) => param_bus_id,
                        Err(e) => return Box::new(future::ok(Response::new().with_status(StatusCode::BadRequest).with_body(format!("Couldn't parse path parameter busId: {}", e)))),
                    },
                    Err(_) => return Box::new(future::ok(Response::new().with_status(StatusCode::BadRequest).with_body(format!("Couldn't percent-decode path parameter as UTF-8: {}", &path_params["busId"]))))
                };
                let param_addr = match percent_encoding::percent_decode(path_params["addr"].as_bytes()).decode_utf8() {
                    Ok(param_addr) => match param_addr.parse::<i32>() {
                        Ok(param_addr) => param_addr,
                        Err(e) => return Box::new(future::ok(Response::new().with_status(StatusCode::BadRequest).with_body(format!("Couldn't parse path parameter addr: {}", e)))),
                    },
                    Err(_) => return Box::new(future::ok(Response::new().with_status(StatusCode::BadRequest).with_body(format!("Couldn't percent-decode path parameter as UTF-8: {}", &path_params["addr"]))))
                };





                Box::new({
                        {{

                                Box::new(api_impl.get_current(param_bus_id, param_addr, &context)
                                    .then(move |result| {
                                        let mut response = Response::new();
                                        response.headers_mut().set(XSpanId((&context as &Has<XSpanIdString>).get().0.to_string()));

                                        match result {
                                            Ok(rsp) => match rsp {
                                                GetCurrentResponse::OK

                                                    (body)


                                                => {
                                                    response.set_status(StatusCode::try_from(200).unwrap());

                                                    response.headers_mut().set(ContentType(mimetypes::responses::GET_CURRENT_OK.clone()));


                                                    let body = serde_json::to_string(&body).expect("impossible to fail to serialize");

                                                    response.set_body(body);
                                                },
                                                GetCurrentResponse::BadRequest

                                                    (body)


                                                => {
                                                    response.set_status(StatusCode::try_from(400).unwrap());

                                                    response.headers_mut().set(ContentType(mimetypes::responses::GET_CURRENT_BAD_REQUEST.clone()));


                                                    let body = serde_json::to_string(&body).expect("impossible to fail to serialize");

                                                    response.set_body(body);
                                                },
                                                GetCurrentResponse::OperationFailed

                                                    (body)


                                                => {
                                                    response.set_status(StatusCode::try_from(502).unwrap());

                                                    response.headers_mut().set(ContentType(mimetypes::responses::GET_CURRENT_OPERATION_FAILED.clone()));


                                                    let body = serde_json::to_string(&body).expect("impossible to fail to serialize");

                                                    response.set_body(body);
                                                },
                                            },
                                            Err(_) => {
                                                // Application code returned an error. This should not happen, as the implementation should
                                                // return a valid response.
                                                response.set_status(StatusCode::InternalServerError);
                                                response.set_body("An internal error occurred");
                                            },
                                        }

                                        future::ok(response)
                                    }
                                ))

                        }}
                }) as Box<Future<Item=Response, Error=Error>>


            },


            // GetError - GET /pca9956b/{busId}/{addr}/error
            &hyper::Method::Get if path.matched(paths::ID_PCA9956B_BUSID_ADDR_ERROR) => {


                // Path parameters
                let path = uri.path().to_string();
                let path_params =
                    paths::REGEX_PCA9956B_BUSID_ADDR_ERROR
                    .captures(&path)
                    .unwrap_or_else(||
                        panic!("Path {} matched RE PCA9956B_BUSID_ADDR_ERROR in set but failed match against \"{}\"", path, paths::REGEX_PCA9956B_BUSID_ADDR_ERROR.as_str())
                    );

                let param_bus_id = match percent_encoding::percent_decode(path_params["busId"].as_bytes()).decode_utf8() {
                    Ok(param_bus_id) => match param_bus_id.parse::<i32>() {
                        Ok(param_bus_id) => param_bus_id,
                        Err(e) => return Box::new(future::ok(Response::new().with_status(StatusCode::BadRequest).with_body(format!("Couldn't parse path parameter busId: {}", e)))),
                    },
                    Err(_) => return Box::new(future::ok(Response::new().with_status(StatusCode::BadRequest).with_body(format!("Couldn't percent-decode path parameter as UTF-8: {}", &path_params["busId"]))))
                };
                let param_addr = match percent_encoding::percent_decode(path_params["addr"].as_bytes()).decode_utf8() {
                    Ok(param_addr) => match param_addr.parse::<i32>() {
                        Ok(param_addr) => param_addr,
                        Err(e) => return Box::new(future::ok(Response::new().with_status(StatusCode::BadRequest).with_body(format!("Couldn't parse path parameter addr: {}", e)))),
                    },
                    Err(_) => return Box::new(future::ok(Response::new().with_status(StatusCode::BadRequest).with_body(format!("Couldn't percent-decode path parameter as UTF-8: {}", &path_params["addr"]))))
                };





                Box::new({
                        {{

                                Box::new(api_impl.get_error(param_bus_id, param_addr, &context)
                                    .then(move |result| {
                                        let mut response = Response::new();
                                        response.headers_mut().set(XSpanId((&context as &Has<XSpanIdString>).get().0.to_string()));

                                        match result {
                                            Ok(rsp) => match rsp {
                                                GetErrorResponse::OK

                                                    (body)


                                                => {
                                                    response.set_status(StatusCode::try_from(200).unwrap());

                                                    response.headers_mut().set(ContentType(mimetypes::responses::GET_ERROR_OK.clone()));


                                                    let body = serde_json::to_string(&body).expect("impossible to fail to serialize");

                                                    response.set_body(body);
                                                },
                                                GetErrorResponse::BadRequest

                                                    (body)


                                                => {
                                                    response.set_status(StatusCode::try_from(400).unwrap());

                                                    response.headers_mut().set(ContentType(mimetypes::responses::GET_ERROR_BAD_REQUEST.clone()));


                                                    let body = serde_json::to_string(&body).expect("impossible to fail to serialize");

                                                    response.set_body(body);
                                                },
                                                GetErrorResponse::OperationFailed

                                                    (body)


                                                => {
                                                    response.set_status(StatusCode::try_from(502).unwrap());

                                                    response.headers_mut().set(ContentType(mimetypes::responses::GET_ERROR_OPERATION_FAILED.clone()));


                                                    let body = serde_json::to_string(&body).expect("impossible to fail to serialize");

                                                    response.set_body(body);
                                                },
                                            },
                                            Err(_) => {
                                                // Application code returned an error. This should not happen, as the implementation should
                                                // return a valid response.
                                                response.set_status(StatusCode::InternalServerError);
                                                response.set_body("An internal error occurred");
                                            },
                                        }

                                        future::ok(response)
                                    }
                                ))

                        }}
                }) as Box<Future<Item=Response, Error=Error>>


            },


            // GetErrors - POST /pca9956b/{busId}/{addr}/errors
            &hyper::Method::Post if path.matched(paths::ID_PCA9956B_BUSID_ADDR_ERRORS) => {


                // Path parameters
                let path = uri.path().to_string();
                let path_params =
                    paths::REGEX_PCA9956B_BUSID_ADDR_ERRORS
                    .captures(&path)
                    .unwrap_or_else(||
                        panic!("Path {} matched RE PCA9956B_BUSID_ADDR_ERRORS in set but failed match against \"{}\"", path, paths::REGEX_PCA9956B_BUSID_ADDR_ERRORS.as_str())
                    );

                let param_bus_id = match percent_encoding::percent_decode(path_params["busId"].as_bytes()).decode_utf8() {
                    Ok(param_bus_id) => match param_bus_id.parse::<i32>() {
                        Ok(param_bus_id) => param_bus_id,
                        Err(e) => return Box::new(future::ok(Response::new().with_status(StatusCode::BadRequest).with_body(format!("Couldn't parse path parameter busId: {}", e)))),
                    },
                    Err(_) => return Box::new(future::ok(Response::new().with_status(StatusCode::BadRequest).with_body(format!("Couldn't percent-decode path parameter as UTF-8: {}", &path_params["busId"]))))
                };
                let param_addr = match percent_encoding::percent_decode(path_params["addr"].as_bytes()).decode_utf8() {
                    Ok(param_addr) => match param_addr.parse::<i32>() {
                        Ok(param_addr) => param_addr,
                        Err(e) => return Box::new(future::ok(Response::new().with_status(StatusCode::BadRequest).with_body(format!("Couldn't parse path parameter addr: {}", e)))),
                    },
                    Err(_) => return Box::new(future::ok(Response::new().with_status(StatusCode::BadRequest).with_body(format!("Couldn't percent-decode path parameter as UTF-8: {}", &path_params["addr"]))))
                };





                Box::new({
                        {{

                                Box::new(api_impl.get_errors(param_bus_id, param_addr, &context)
                                    .then(move |result| {
                                        let mut response = Response::new();
                                        response.headers_mut().set(XSpanId((&context as &Has<XSpanIdString>).get().0.to_string()));

                                        match result {
                                            Ok(rsp) => match rsp {
                                                GetErrorsResponse::OK

                                                    (body)


                                                => {
                                                    response.set_status(StatusCode::try_from(200).unwrap());

                                                    response.headers_mut().set(ContentType(mimetypes::responses::GET_ERRORS_OK.clone()));


                                                    let body = serde_json::to_string(&body).expect("impossible to fail to serialize");

                                                    response.set_body(body);
                                                },
                                                GetErrorsResponse::BadRequest

                                                    (body)


                                                => {
                                                    response.set_status(StatusCode::try_from(400).unwrap());

                                                    response.headers_mut().set(ContentType(mimetypes::responses::GET_ERRORS_BAD_REQUEST.clone()));


                                                    let body = serde_json::to_string(&body).expect("impossible to fail to serialize");

                                                    response.set_body(body);
                                                },
                                                GetErrorsResponse::OperationFailed

                                                    (body)


                                                => {
                                                    response.set_status(StatusCode::try_from(502).unwrap());

                                                    response.headers_mut().set(ContentType(mimetypes::responses::GET_ERRORS_OPERATION_FAILED.clone()));


                                                    let body = serde_json::to_string(&body).expect("impossible to fail to serialize");

                                                    response.set_body(body);
                                                },
                                            },
                                            Err(_) => {
                                                // Application code returned an error. This should not happen, as the implementation should
                                                // return a valid response.
                                                response.set_status(StatusCode::InternalServerError);
                                                response.set_body("An internal error occurred");
                                            },
                                        }

                                        future::ok(response)
                                    }
                                ))

                        }}
                }) as Box<Future<Item=Response, Error=Error>>


            },


            // GetFreq - GET /pca9956b/{busId}/{addr}/freq
            &hyper::Method::Get if path.matched(paths::ID_PCA9956B_BUSID_ADDR_FREQ) => {


                // Path parameters
                let path = uri.path().to_string();
                let path_params =
                    paths::REGEX_PCA9956B_BUSID_ADDR_FREQ
                    .captures(&path)
                    .unwrap_or_else(||
                        panic!("Path {} matched RE PCA9956B_BUSID_ADDR_FREQ in set but failed match against \"{}\"", path, paths::REGEX_PCA9956B_BUSID_ADDR_FREQ.as_str())
                    );

                let param_bus_id = match percent_encoding::percent_decode(path_params["busId"].as_bytes()).decode_utf8() {
                    Ok(param_bus_id) => match param_bus_id.parse::<i32>() {
                        Ok(param_bus_id) => param_bus_id,
                        Err(e) => return Box::new(future::ok(Response::new().with_status(StatusCode::BadRequest).with_body(format!("Couldn't parse path parameter busId: {}", e)))),
                    },
                    Err(_) => return Box::new(future::ok(Response::new().with_status(StatusCode::BadRequest).with_body(format!("Couldn't percent-decode path parameter as UTF-8: {}", &path_params["busId"]))))
                };
                let param_addr = match percent_encoding::percent_decode(path_params["addr"].as_bytes()).decode_utf8() {
                    Ok(param_addr) => match param_addr.parse::<i32>() {
                        Ok(param_addr) => param_addr,
                        Err(e) => return Box::new(future::ok(Response::new().with_status(StatusCode::BadRequest).with_body(format!("Couldn't parse path parameter addr: {}", e)))),
                    },
                    Err(_) => return Box::new(future::ok(Response::new().with_status(StatusCode::BadRequest).with_body(format!("Couldn't percent-decode path parameter as UTF-8: {}", &path_params["addr"]))))
                };





                Box::new({
                        {{

                                Box::new(api_impl.get_freq(param_bus_id, param_addr, &context)
                                    .then(move |result| {
                                        let mut response = Response::new();
                                        response.headers_mut().set(XSpanId((&context as &Has<XSpanIdString>).get().0.to_string()));

                                        match result {
                                            Ok(rsp) => match rsp {
                                                GetFreqResponse::OK

                                                    (body)


                                                => {
                                                    response.set_status(StatusCode::try_from(200).unwrap());

                                                    response.headers_mut().set(ContentType(mimetypes::responses::GET_FREQ_OK.clone()));


                                                    let body = serde_json::to_string(&body).expect("impossible to fail to serialize");

                                                    response.set_body(body);
                                                },
                                                GetFreqResponse::BadRequest

                                                    (body)


                                                => {
                                                    response.set_status(StatusCode::try_from(400).unwrap());

                                                    response.headers_mut().set(ContentType(mimetypes::responses::GET_FREQ_BAD_REQUEST.clone()));


                                                    let body = serde_json::to_string(&body).expect("impossible to fail to serialize");

                                                    response.set_body(body);
                                                },
                                                GetFreqResponse::OperationFailed

                                                    (body)


                                                => {
                                                    response.set_status(StatusCode::try_from(502).unwrap());

                                                    response.headers_mut().set(ContentType(mimetypes::responses::GET_FREQ_OPERATION_FAILED.clone()));


                                                    let body = serde_json::to_string(&body).expect("impossible to fail to serialize");

                                                    response.set_body(body);
                                                },
                                            },
                                            Err(_) => {
                                                // Application code returned an error. This should not happen, as the implementation should
                                                // return a valid response.
                                                response.set_status(StatusCode::InternalServerError);
                                                response.set_body("An internal error occurred");
                                            },
                                        }

                                        future::ok(response)
                                    }
                                ))

                        }}
                }) as Box<Future<Item=Response, Error=Error>>


            },


            // GetGroup - GET /pca9956b/{busId}/{addr}/group
            &hyper::Method::Get if path.matched(paths::ID_PCA9956B_BUSID_ADDR_GROUP) => {


                // Path parameters
                let path = uri.path().to_string();
                let path_params =
                    paths::REGEX_PCA9956B_BUSID_ADDR_GROUP
                    .captures(&path)
                    .unwrap_or_else(||
                        panic!("Path {} matched RE PCA9956B_BUSID_ADDR_GROUP in set but failed match against \"{}\"", path, paths::REGEX_PCA9956B_BUSID_ADDR_GROUP.as_str())
                    );

                let param_bus_id = match percent_encoding::percent_decode(path_params["busId"].as_bytes()).decode_utf8() {
                    Ok(param_bus_id) => match param_bus_id.parse::<i32>() {
                        Ok(param_bus_id) => param_bus_id,
                        Err(e) => return Box::new(future::ok(Response::new().with_status(StatusCode::BadRequest).with_body(format!("Couldn't parse path parameter busId: {}", e)))),
                    },
                    Err(_) => return Box::new(future::ok(Response::new().with_status(StatusCode::BadRequest).with_body(format!("Couldn't percent-decode path parameter as UTF-8: {}", &path_params["busId"]))))
                };
                let param_addr = match percent_encoding::percent_decode(path_params["addr"].as_bytes()).decode_utf8() {
                    Ok(param_addr) => match param_addr.parse::<i32>() {
                        Ok(param_addr) => param_addr,
                        Err(e) => return Box::new(future::ok(Response::new().with_status(StatusCode::BadRequest).with_body(format!("Couldn't parse path parameter addr: {}", e)))),
                    },
                    Err(_) => return Box::new(future::ok(Response::new().with_status(StatusCode::BadRequest).with_body(format!("Couldn't percent-decode path parameter as UTF-8: {}", &path_params["addr"]))))
                };





                Box::new({
                        {{

                                Box::new(api_impl.get_group(param_bus_id, param_addr, &context)
                                    .then(move |result| {
                                        let mut response = Response::new();
                                        response.headers_mut().set(XSpanId((&context as &Has<XSpanIdString>).get().0.to_string()));

                                        match result {
                                            Ok(rsp) => match rsp {
                                                GetGroupResponse::OK

                                                    (body)


                                                => {
                                                    response.set_status(StatusCode::try_from(200).unwrap());

                                                    response.headers_mut().set(ContentType(mimetypes::responses::GET_GROUP_OK.clone()));


                                                    let body = serde_json::to_string(&body).expect("impossible to fail to serialize");

                                                    response.set_body(body);
                                                },
                                                GetGroupResponse::BadRequest

                                                    (body)


                                                => {
                                                    response.set_status(StatusCode::try_from(400).unwrap());

                                                    response.headers_mut().set(ContentType(mimetypes::responses::GET_GROUP_BAD_REQUEST.clone()));


                                                    let body = serde_json::to_string(&body).expect("impossible to fail to serialize");

                                                    response.set_body(body);
                                                },
                                                GetGroupResponse::OperationFailed

                                                    (body)


                                                => {
                                                    response.set_status(StatusCode::try_from(502).unwrap());

                                                    response.headers_mut().set(ContentType(mimetypes::responses::GET_GROUP_OPERATION_FAILED.clone()));


                                                    let body = serde_json::to_string(&body).expect("impossible to fail to serialize");

                                                    response.set_body(body);
                                                },
                                            },
                                            Err(_) => {
                                                // Application code returned an error. This should not happen, as the implementation should
                                                // return a valid response.
                                                response.set_status(StatusCode::InternalServerError);
                                                response.set_body("An internal error occurred");
                                            },
                                        }

                                        future::ok(response)
                                    }
                                ))

                        }}
                }) as Box<Future<Item=Response, Error=Error>>


            },


            // GetLedCurrent - GET /pca9956b/{busId}/{addr}/led/{led}/current
            &hyper::Method::Get if path.matched(paths::ID_PCA9956B_BUSID_ADDR_LED_LED_CURRENT) => {


                // Path parameters
                let path = uri.path().to_string();
                let path_params =
                    paths::REGEX_PCA9956B_BUSID_ADDR_LED_LED_CURRENT
                    .captures(&path)
                    .unwrap_or_else(||
                        panic!("Path {} matched RE PCA9956B_BUSID_ADDR_LED_LED_CURRENT in set but failed match against \"{}\"", path, paths::REGEX_PCA9956B_BUSID_ADDR_LED_LED_CURRENT.as_str())
                    );

                let param_bus_id = match percent_encoding::percent_decode(path_params["busId"].as_bytes()).decode_utf8() {
                    Ok(param_bus_id) => match param_bus_id.parse::<i32>() {
                        Ok(param_bus_id) => param_bus_id,
                        Err(e) => return Box::new(future::ok(Response::new().with_status(StatusCode::BadRequest).with_body(format!("Couldn't parse path parameter busId: {}", e)))),
                    },
                    Err(_) => return Box::new(future::ok(Response::new().with_status(StatusCode::BadRequest).with_body(format!("Couldn't percent-decode path parameter as UTF-8: {}", &path_params["busId"]))))
                };
                let param_addr = match percent_encoding::percent_decode(path_params["addr"].as_bytes()).decode_utf8() {
                    Ok(param_addr) => match param_addr.parse::<i32>() {
                        Ok(param_addr) => param_addr,
                        Err(e) => return Box::new(future::ok(Response::new().with_status(StatusCode::BadRequest).with_body(format!("Couldn't parse path parameter addr: {}", e)))),
                    },
                    Err(_) => return Box::new(future::ok(Response::new().with_status(StatusCode::BadRequest).with_body(format!("Couldn't percent-decode path parameter as UTF-8: {}", &path_params["addr"]))))
                };
                let param_led = match percent_encoding::percent_decode(path_params["led"].as_bytes()).decode_utf8() {
                    Ok(param_led) => match param_led.parse::<i32>() {
                        Ok(param_led) => param_led,
                        Err(e) => return Box::new(future::ok(Response::new().with_status(StatusCode::BadRequest).with_body(format!("Couldn't parse path parameter led: {}", e)))),
                    },
                    Err(_) => return Box::new(future::ok(Response::new().with_status(StatusCode::BadRequest).with_body(format!("Couldn't percent-decode path parameter as UTF-8: {}", &path_params["led"]))))
                };





                Box::new({
                        {{

                                Box::new(api_impl.get_led_current(param_bus_id, param_addr, param_led, &context)
                                    .then(move |result| {
                                        let mut response = Response::new();
                                        response.headers_mut().set(XSpanId((&context as &Has<XSpanIdString>).get().0.to_string()));

                                        match result {
                                            Ok(rsp) => match rsp {
                                                GetLedCurrentResponse::OK

                                                    (body)


                                                => {
                                                    response.set_status(StatusCode::try_from(200).unwrap());

                                                    response.headers_mut().set(ContentType(mimetypes::responses::GET_LED_CURRENT_OK.clone()));


                                                    let body = serde_json::to_string(&body).expect("impossible to fail to serialize");

                                                    response.set_body(body);
                                                },
                                                GetLedCurrentResponse::BadRequest

                                                    (body)


                                                => {
                                                    response.set_status(StatusCode::try_from(400).unwrap());

                                                    response.headers_mut().set(ContentType(mimetypes::responses::GET_LED_CURRENT_BAD_REQUEST.clone()));


                                                    let body = serde_json::to_string(&body).expect("impossible to fail to serialize");

                                                    response.set_body(body);
                                                },
                                                GetLedCurrentResponse::OperationFailed

                                                    (body)


                                                => {
                                                    response.set_status(StatusCode::try_from(502).unwrap());

                                                    response.headers_mut().set(ContentType(mimetypes::responses::GET_LED_CURRENT_OPERATION_FAILED.clone()));


                                                    let body = serde_json::to_string(&body).expect("impossible to fail to serialize");

                                                    response.set_body(body);
                                                },
                                            },
                                            Err(_) => {
                                                // Application code returned an error. This should not happen, as the implementation should
                                                // return a valid response.
                                                response.set_status(StatusCode::InternalServerError);
                                                response.set_body("An internal error occurred");
                                            },
                                        }

                                        future::ok(response)
                                    }
                                ))

                        }}
                }) as Box<Future<Item=Response, Error=Error>>


            },


            // GetLedError - GET /pca9956b/{busId}/{addr}/led/{led}/error
            &hyper::Method::Get if path.matched(paths::ID_PCA9956B_BUSID_ADDR_LED_LED_ERROR) => {


                // Path parameters
                let path = uri.path().to_string();
                let path_params =
                    paths::REGEX_PCA9956B_BUSID_ADDR_LED_LED_ERROR
                    .captures(&path)
                    .unwrap_or_else(||
                        panic!("Path {} matched RE PCA9956B_BUSID_ADDR_LED_LED_ERROR in set but failed match against \"{}\"", path, paths::REGEX_PCA9956B_BUSID_ADDR_LED_LED_ERROR.as_str())
                    );

                let param_bus_id = match percent_encoding::percent_decode(path_params["busId"].as_bytes()).decode_utf8() {
                    Ok(param_bus_id) => match param_bus_id.parse::<i32>() {
                        Ok(param_bus_id) => param_bus_id,
                        Err(e) => return Box::new(future::ok(Response::new().with_status(StatusCode::BadRequest).with_body(format!("Couldn't parse path parameter busId: {}", e)))),
                    },
                    Err(_) => return Box::new(future::ok(Response::new().with_status(StatusCode::BadRequest).with_body(format!("Couldn't percent-decode path parameter as UTF-8: {}", &path_params["busId"]))))
                };
                let param_addr = match percent_encoding::percent_decode(path_params["addr"].as_bytes()).decode_utf8() {
                    Ok(param_addr) => match param_addr.parse::<i32>() {
                        Ok(param_addr) => param_addr,
                        Err(e) => return Box::new(future::ok(Response::new().with_status(StatusCode::BadRequest).with_body(format!("Couldn't parse path parameter addr: {}", e)))),
                    },
                    Err(_) => return Box::new(future::ok(Response::new().with_status(StatusCode::BadRequest).with_body(format!("Couldn't percent-decode path parameter as UTF-8: {}", &path_params["addr"]))))
                };
                let param_led = match percent_encoding::percent_decode(path_params["led"].as_bytes()).decode_utf8() {
                    Ok(param_led) => match param_led.parse::<i32>() {
                        Ok(param_led) => param_led,
                        Err(e) => return Box::new(future::ok(Response::new().with_status(StatusCode::BadRequest).with_body(format!("Couldn't parse path parameter led: {}", e)))),
                    },
                    Err(_) => return Box::new(future::ok(Response::new().with_status(StatusCode::BadRequest).with_body(format!("Couldn't percent-decode path parameter as UTF-8: {}", &path_params["led"]))))
                };





                Box::new({
                        {{

                                Box::new(api_impl.get_led_error(param_bus_id, param_addr, param_led, &context)
                                    .then(move |result| {
                                        let mut response = Response::new();
                                        response.headers_mut().set(XSpanId((&context as &Has<XSpanIdString>).get().0.to_string()));

                                        match result {
                                            Ok(rsp) => match rsp {
                                                GetLedErrorResponse::OK

                                                    (body)


                                                => {
                                                    response.set_status(StatusCode::try_from(200).unwrap());

                                                    response.headers_mut().set(ContentType(mimetypes::responses::GET_LED_ERROR_OK.clone()));


                                                    let body = serde_json::to_string(&body).expect("impossible to fail to serialize");

                                                    response.set_body(body);
                                                },
                                                GetLedErrorResponse::BadRequest

                                                    (body)


                                                => {
                                                    response.set_status(StatusCode::try_from(400).unwrap());

                                                    response.headers_mut().set(ContentType(mimetypes::responses::GET_LED_ERROR_BAD_REQUEST.clone()));


                                                    let body = serde_json::to_string(&body).expect("impossible to fail to serialize");

                                                    response.set_body(body);
                                                },
                                                GetLedErrorResponse::OperationFailed

                                                    (body)


                                                => {
                                                    response.set_status(StatusCode::try_from(502).unwrap());

                                                    response.headers_mut().set(ContentType(mimetypes::responses::GET_LED_ERROR_OPERATION_FAILED.clone()));


                                                    let body = serde_json::to_string(&body).expect("impossible to fail to serialize");

                                                    response.set_body(body);
                                                },
                                            },
                                            Err(_) => {
                                                // Application code returned an error. This should not happen, as the implementation should
                                                // return a valid response.
                                                response.set_status(StatusCode::InternalServerError);
                                                response.set_body("An internal error occurred");
                                            },
                                        }

                                        future::ok(response)
                                    }
                                ))

                        }}
                }) as Box<Future<Item=Response, Error=Error>>


            },


            // GetLedInfo - GET /pca9956b/{busId}/{addr}/led/{led}
            &hyper::Method::Get if path.matched(paths::ID_PCA9956B_BUSID_ADDR_LED_LED) => {


                // Path parameters
                let path = uri.path().to_string();
                let path_params =
                    paths::REGEX_PCA9956B_BUSID_ADDR_LED_LED
                    .captures(&path)
                    .unwrap_or_else(||
                        panic!("Path {} matched RE PCA9956B_BUSID_ADDR_LED_LED in set but failed match against \"{}\"", path, paths::REGEX_PCA9956B_BUSID_ADDR_LED_LED.as_str())
                    );

                let param_bus_id = match percent_encoding::percent_decode(path_params["busId"].as_bytes()).decode_utf8() {
                    Ok(param_bus_id) => match param_bus_id.parse::<i32>() {
                        Ok(param_bus_id) => param_bus_id,
                        Err(e) => return Box::new(future::ok(Response::new().with_status(StatusCode::BadRequest).with_body(format!("Couldn't parse path parameter busId: {}", e)))),
                    },
                    Err(_) => return Box::new(future::ok(Response::new().with_status(StatusCode::BadRequest).with_body(format!("Couldn't percent-decode path parameter as UTF-8: {}", &path_params["busId"]))))
                };
                let param_addr = match percent_encoding::percent_decode(path_params["addr"].as_bytes()).decode_utf8() {
                    Ok(param_addr) => match param_addr.parse::<i32>() {
                        Ok(param_addr) => param_addr,
                        Err(e) => return Box::new(future::ok(Response::new().with_status(StatusCode::BadRequest).with_body(format!("Couldn't parse path parameter addr: {}", e)))),
                    },
                    Err(_) => return Box::new(future::ok(Response::new().with_status(StatusCode::BadRequest).with_body(format!("Couldn't percent-decode path parameter as UTF-8: {}", &path_params["addr"]))))
                };
                let param_led = match percent_encoding::percent_decode(path_params["led"].as_bytes()).decode_utf8() {
                    Ok(param_led) => match param_led.parse::<i32>() {
                        Ok(param_led) => param_led,
                        Err(e) => return Box::new(future::ok(Response::new().with_status(StatusCode::BadRequest).with_body(format!("Couldn't parse path parameter led: {}", e)))),
                    },
                    Err(_) => return Box::new(future::ok(Response::new().with_status(StatusCode::BadRequest).with_body(format!("Couldn't percent-decode path parameter as UTF-8: {}", &path_params["led"]))))
                };





                Box::new({
                        {{

                                Box::new(api_impl.get_led_info(param_bus_id, param_addr, param_led, &context)
                                    .then(move |result| {
                                        let mut response = Response::new();
                                        response.headers_mut().set(XSpanId((&context as &Has<XSpanIdString>).get().0.to_string()));

                                        match result {
                                            Ok(rsp) => match rsp {
                                                GetLedInfoResponse::OK

                                                    (body)


                                                => {
                                                    response.set_status(StatusCode::try_from(200).unwrap());

                                                    response.headers_mut().set(ContentType(mimetypes::responses::GET_LED_INFO_OK.clone()));


                                                    let body = serde_json::to_string(&body).expect("impossible to fail to serialize");

                                                    response.set_body(body);
                                                },
                                                GetLedInfoResponse::BadRequest

                                                    (body)


                                                => {
                                                    response.set_status(StatusCode::try_from(400).unwrap());

                                                    response.headers_mut().set(ContentType(mimetypes::responses::GET_LED_INFO_BAD_REQUEST.clone()));


                                                    let body = serde_json::to_string(&body).expect("impossible to fail to serialize");

                                                    response.set_body(body);
                                                },
                                                GetLedInfoResponse::OperationFailed

                                                    (body)


                                                => {
                                                    response.set_status(StatusCode::try_from(502).unwrap());

                                                    response.headers_mut().set(ContentType(mimetypes::responses::GET_LED_INFO_OPERATION_FAILED.clone()));


                                                    let body = serde_json::to_string(&body).expect("impossible to fail to serialize");

                                                    response.set_body(body);
                                                },
                                            },
                                            Err(_) => {
                                                // Application code returned an error. This should not happen, as the implementation should
                                                // return a valid response.
                                                response.set_status(StatusCode::InternalServerError);
                                                response.set_body("An internal error occurred");
                                            },
                                        }

                                        future::ok(response)
                                    }
                                ))

                        }}
                }) as Box<Future<Item=Response, Error=Error>>


            },


            // GetLedInfoAll - GET /pca9956b/{busId}/{addr}/led
            &hyper::Method::Get if path.matched(paths::ID_PCA9956B_BUSID_ADDR_LED) => {


                // Path parameters
                let path = uri.path().to_string();
                let path_params =
                    paths::REGEX_PCA9956B_BUSID_ADDR_LED
                    .captures(&path)
                    .unwrap_or_else(||
                        panic!("Path {} matched RE PCA9956B_BUSID_ADDR_LED in set but failed match against \"{}\"", path, paths::REGEX_PCA9956B_BUSID_ADDR_LED.as_str())
                    );

                let param_bus_id = match percent_encoding::percent_decode(path_params["busId"].as_bytes()).decode_utf8() {
                    Ok(param_bus_id) => match param_bus_id.parse::<i32>() {
                        Ok(param_bus_id) => param_bus_id,
                        Err(e) => return Box::new(future::ok(Response::new().with_status(StatusCode::BadRequest).with_body(format!("Couldn't parse path parameter busId: {}", e)))),
                    },
                    Err(_) => return Box::new(future::ok(Response::new().with_status(StatusCode::BadRequest).with_body(format!("Couldn't percent-decode path parameter as UTF-8: {}", &path_params["busId"]))))
                };
                let param_addr = match percent_encoding::percent_decode(path_params["addr"].as_bytes()).decode_utf8() {
                    Ok(param_addr) => match param_addr.parse::<i32>() {
                        Ok(param_addr) => param_addr,
                        Err(e) => return Box::new(future::ok(Response::new().with_status(StatusCode::BadRequest).with_body(format!("Couldn't parse path parameter addr: {}", e)))),
                    },
                    Err(_) => return Box::new(future::ok(Response::new().with_status(StatusCode::BadRequest).with_body(format!("Couldn't percent-decode path parameter as UTF-8: {}", &path_params["addr"]))))
                };





                Box::new({
                        {{

                                Box::new(api_impl.get_led_info_all(param_bus_id, param_addr, &context)
                                    .then(move |result| {
                                        let mut response = Response::new();
                                        response.headers_mut().set(XSpanId((&context as &Has<XSpanIdString>).get().0.to_string()));

                                        match result {
                                            Ok(rsp) => match rsp {
                                                GetLedInfoAllResponse::OK

                                                    (body)


                                                => {
                                                    response.set_status(StatusCode::try_from(200).unwrap());

                                                    response.headers_mut().set(ContentType(mimetypes::responses::GET_LED_INFO_ALL_OK.clone()));


                                                    let body = serde_json::to_string(&body).expect("impossible to fail to serialize");

                                                    response.set_body(body);
                                                },
                                                GetLedInfoAllResponse::BadRequest

                                                    (body)


                                                => {
                                                    response.set_status(StatusCode::try_from(400).unwrap());

                                                    response.headers_mut().set(ContentType(mimetypes::responses::GET_LED_INFO_ALL_BAD_REQUEST.clone()));


                                                    let body = serde_json::to_string(&body).expect("impossible to fail to serialize");

                                                    response.set_body(body);
                                                },
                                                GetLedInfoAllResponse::OperationFailed

                                                    (body)


                                                => {
                                                    response.set_status(StatusCode::try_from(502).unwrap());

                                                    response.headers_mut().set(ContentType(mimetypes::responses::GET_LED_INFO_ALL_OPERATION_FAILED.clone()));


                                                    let body = serde_json::to_string(&body).expect("impossible to fail to serialize");

                                                    response.set_body(body);
                                                },
                                            },
                                            Err(_) => {
                                                // Application code returned an error. This should not happen, as the implementation should
                                                // return a valid response.
                                                response.set_status(StatusCode::InternalServerError);
                                                response.set_body("An internal error occurred");
                                            },
                                        }

                                        future::ok(response)
                                    }
                                ))

                        }}
                }) as Box<Future<Item=Response, Error=Error>>


            },


            // GetLedPwm - GET /pca9956b/{busId}/{addr}/led/{led}/pwm
            &hyper::Method::Get if path.matched(paths::ID_PCA9956B_BUSID_ADDR_LED_LED_PWM) => {


                // Path parameters
                let path = uri.path().to_string();
                let path_params =
                    paths::REGEX_PCA9956B_BUSID_ADDR_LED_LED_PWM
                    .captures(&path)
                    .unwrap_or_else(||
                        panic!("Path {} matched RE PCA9956B_BUSID_ADDR_LED_LED_PWM in set but failed match against \"{}\"", path, paths::REGEX_PCA9956B_BUSID_ADDR_LED_LED_PWM.as_str())
                    );

                let param_bus_id = match percent_encoding::percent_decode(path_params["busId"].as_bytes()).decode_utf8() {
                    Ok(param_bus_id) => match param_bus_id.parse::<i32>() {
                        Ok(param_bus_id) => param_bus_id,
                        Err(e) => return Box::new(future::ok(Response::new().with_status(StatusCode::BadRequest).with_body(format!("Couldn't parse path parameter busId: {}", e)))),
                    },
                    Err(_) => return Box::new(future::ok(Response::new().with_status(StatusCode::BadRequest).with_body(format!("Couldn't percent-decode path parameter as UTF-8: {}", &path_params["busId"]))))
                };
                let param_addr = match percent_encoding::percent_decode(path_params["addr"].as_bytes()).decode_utf8() {
                    Ok(param_addr) => match param_addr.parse::<i32>() {
                        Ok(param_addr) => param_addr,
                        Err(e) => return Box::new(future::ok(Response::new().with_status(StatusCode::BadRequest).with_body(format!("Couldn't parse path parameter addr: {}", e)))),
                    },
                    Err(_) => return Box::new(future::ok(Response::new().with_status(StatusCode::BadRequest).with_body(format!("Couldn't percent-decode path parameter as UTF-8: {}", &path_params["addr"]))))
                };
                let param_led = match percent_encoding::percent_decode(path_params["led"].as_bytes()).decode_utf8() {
                    Ok(param_led) => match param_led.parse::<i32>() {
                        Ok(param_led) => param_led,
                        Err(e) => return Box::new(future::ok(Response::new().with_status(StatusCode::BadRequest).with_body(format!("Couldn't parse path parameter led: {}", e)))),
                    },
                    Err(_) => return Box::new(future::ok(Response::new().with_status(StatusCode::BadRequest).with_body(format!("Couldn't percent-decode path parameter as UTF-8: {}", &path_params["led"]))))
                };





                Box::new({
                        {{

                                Box::new(api_impl.get_led_pwm(param_bus_id, param_addr, param_led, &context)
                                    .then(move |result| {
                                        let mut response = Response::new();
                                        response.headers_mut().set(XSpanId((&context as &Has<XSpanIdString>).get().0.to_string()));

                                        match result {
                                            Ok(rsp) => match rsp {
                                                GetLedPwmResponse::OK

                                                    (body)


                                                => {
                                                    response.set_status(StatusCode::try_from(200).unwrap());

                                                    response.headers_mut().set(ContentType(mimetypes::responses::GET_LED_PWM_OK.clone()));


                                                    let body = serde_json::to_string(&body).expect("impossible to fail to serialize");

                                                    response.set_body(body);
                                                },
                                                GetLedPwmResponse::BadRequest

                                                    (body)


                                                => {
                                                    response.set_status(StatusCode::try_from(400).unwrap());

                                                    response.headers_mut().set(ContentType(mimetypes::responses::GET_LED_PWM_BAD_REQUEST.clone()));


                                                    let body = serde_json::to_string(&body).expect("impossible to fail to serialize");

                                                    response.set_body(body);
                                                },
                                                GetLedPwmResponse::OperationFailed

                                                    (body)


                                                => {
                                                    response.set_status(StatusCode::try_from(502).unwrap());

                                                    response.headers_mut().set(ContentType(mimetypes::responses::GET_LED_PWM_OPERATION_FAILED.clone()));


                                                    let body = serde_json::to_string(&body).expect("impossible to fail to serialize");

                                                    response.set_body(body);
                                                },
                                            },
                                            Err(_) => {
                                                // Application code returned an error. This should not happen, as the implementation should
                                                // return a valid response.
                                                response.set_status(StatusCode::InternalServerError);
                                                response.set_body("An internal error occurred");
                                            },
                                        }

                                        future::ok(response)
                                    }
                                ))

                        }}
                }) as Box<Future<Item=Response, Error=Error>>


            },


            // GetLedState - GET /pca9956b/{busId}/{addr}/led/{led}/state
            &hyper::Method::Get if path.matched(paths::ID_PCA9956B_BUSID_ADDR_LED_LED_STATE) => {


                // Path parameters
                let path = uri.path().to_string();
                let path_params =
                    paths::REGEX_PCA9956B_BUSID_ADDR_LED_LED_STATE
                    .captures(&path)
                    .unwrap_or_else(||
                        panic!("Path {} matched RE PCA9956B_BUSID_ADDR_LED_LED_STATE in set but failed match against \"{}\"", path, paths::REGEX_PCA9956B_BUSID_ADDR_LED_LED_STATE.as_str())
                    );

                let param_bus_id = match percent_encoding::percent_decode(path_params["busId"].as_bytes()).decode_utf8() {
                    Ok(param_bus_id) => match param_bus_id.parse::<i32>() {
                        Ok(param_bus_id) => param_bus_id,
                        Err(e) => return Box::new(future::ok(Response::new().with_status(StatusCode::BadRequest).with_body(format!("Couldn't parse path parameter busId: {}", e)))),
                    },
                    Err(_) => return Box::new(future::ok(Response::new().with_status(StatusCode::BadRequest).with_body(format!("Couldn't percent-decode path parameter as UTF-8: {}", &path_params["busId"]))))
                };
                let param_addr = match percent_encoding::percent_decode(path_params["addr"].as_bytes()).decode_utf8() {
                    Ok(param_addr) => match param_addr.parse::<i32>() {
                        Ok(param_addr) => param_addr,
                        Err(e) => return Box::new(future::ok(Response::new().with_status(StatusCode::BadRequest).with_body(format!("Couldn't parse path parameter addr: {}", e)))),
                    },
                    Err(_) => return Box::new(future::ok(Response::new().with_status(StatusCode::BadRequest).with_body(format!("Couldn't percent-decode path parameter as UTF-8: {}", &path_params["addr"]))))
                };
                let param_led = match percent_encoding::percent_decode(path_params["led"].as_bytes()).decode_utf8() {
                    Ok(param_led) => match param_led.parse::<i32>() {
                        Ok(param_led) => param_led,
                        Err(e) => return Box::new(future::ok(Response::new().with_status(StatusCode::BadRequest).with_body(format!("Couldn't parse path parameter led: {}", e)))),
                    },
                    Err(_) => return Box::new(future::ok(Response::new().with_status(StatusCode::BadRequest).with_body(format!("Couldn't percent-decode path parameter as UTF-8: {}", &path_params["led"]))))
                };





                Box::new({
                        {{

                                Box::new(api_impl.get_led_state(param_bus_id, param_addr, param_led, &context)
                                    .then(move |result| {
                                        let mut response = Response::new();
                                        response.headers_mut().set(XSpanId((&context as &Has<XSpanIdString>).get().0.to_string()));

                                        match result {
                                            Ok(rsp) => match rsp {
                                                GetLedStateResponse::OK

                                                    (body)


                                                => {
                                                    response.set_status(StatusCode::try_from(200).unwrap());

                                                    response.headers_mut().set(ContentType(mimetypes::responses::GET_LED_STATE_OK.clone()));


                                                    let body = serde_json::to_string(&body).expect("impossible to fail to serialize");

                                                    response.set_body(body);
                                                },
                                                GetLedStateResponse::BadRequest

                                                    (body)


                                                => {
                                                    response.set_status(StatusCode::try_from(400).unwrap());

                                                    response.headers_mut().set(ContentType(mimetypes::responses::GET_LED_STATE_BAD_REQUEST.clone()));


                                                    let body = serde_json::to_string(&body).expect("impossible to fail to serialize");

                                                    response.set_body(body);
                                                },
                                                GetLedStateResponse::OperationFailed

                                                    (body)


                                                => {
                                                    response.set_status(StatusCode::try_from(502).unwrap());

                                                    response.headers_mut().set(ContentType(mimetypes::responses::GET_LED_STATE_OPERATION_FAILED.clone()));


                                                    let body = serde_json::to_string(&body).expect("impossible to fail to serialize");

                                                    response.set_body(body);
                                                },
                                            },
                                            Err(_) => {
                                                // Application code returned an error. This should not happen, as the implementation should
                                                // return a valid response.
                                                response.set_status(StatusCode::InternalServerError);
                                                response.set_body("An internal error occurred");
                                            },
                                        }

                                        future::ok(response)
                                    }
                                ))

                        }}
                }) as Box<Future<Item=Response, Error=Error>>


            },


            // GetOffset - GET /pca9956b/{busId}/{addr}/offset
            &hyper::Method::Get if path.matched(paths::ID_PCA9956B_BUSID_ADDR_OFFSET) => {


                // Path parameters
                let path = uri.path().to_string();
                let path_params =
                    paths::REGEX_PCA9956B_BUSID_ADDR_OFFSET
                    .captures(&path)
                    .unwrap_or_else(||
                        panic!("Path {} matched RE PCA9956B_BUSID_ADDR_OFFSET in set but failed match against \"{}\"", path, paths::REGEX_PCA9956B_BUSID_ADDR_OFFSET.as_str())
                    );

                let param_bus_id = match percent_encoding::percent_decode(path_params["busId"].as_bytes()).decode_utf8() {
                    Ok(param_bus_id) => match param_bus_id.parse::<i32>() {
                        Ok(param_bus_id) => param_bus_id,
                        Err(e) => return Box::new(future::ok(Response::new().with_status(StatusCode::BadRequest).with_body(format!("Couldn't parse path parameter busId: {}", e)))),
                    },
                    Err(_) => return Box::new(future::ok(Response::new().with_status(StatusCode::BadRequest).with_body(format!("Couldn't percent-decode path parameter as UTF-8: {}", &path_params["busId"]))))
                };
                let param_addr = match percent_encoding::percent_decode(path_params["addr"].as_bytes()).decode_utf8() {
                    Ok(param_addr) => match param_addr.parse::<i32>() {
                        Ok(param_addr) => param_addr,
                        Err(e) => return Box::new(future::ok(Response::new().with_status(StatusCode::BadRequest).with_body(format!("Couldn't parse path parameter addr: {}", e)))),
                    },
                    Err(_) => return Box::new(future::ok(Response::new().with_status(StatusCode::BadRequest).with_body(format!("Couldn't percent-decode path parameter as UTF-8: {}", &path_params["addr"]))))
                };





                Box::new({
                        {{

                                Box::new(api_impl.get_offset(param_bus_id, param_addr, &context)
                                    .then(move |result| {
                                        let mut response = Response::new();
                                        response.headers_mut().set(XSpanId((&context as &Has<XSpanIdString>).get().0.to_string()));

                                        match result {
                                            Ok(rsp) => match rsp {
                                                GetOffsetResponse::OK

                                                    (body)


                                                => {
                                                    response.set_status(StatusCode::try_from(200).unwrap());

                                                    response.headers_mut().set(ContentType(mimetypes::responses::GET_OFFSET_OK.clone()));


                                                    let body = serde_json::to_string(&body).expect("impossible to fail to serialize");

                                                    response.set_body(body);
                                                },
                                                GetOffsetResponse::BadRequest

                                                    (body)


                                                => {
                                                    response.set_status(StatusCode::try_from(400).unwrap());

                                                    response.headers_mut().set(ContentType(mimetypes::responses::GET_OFFSET_BAD_REQUEST.clone()));


                                                    let body = serde_json::to_string(&body).expect("impossible to fail to serialize");

                                                    response.set_body(body);
                                                },
                                                GetOffsetResponse::OperationFailed

                                                    (body)


                                                => {
                                                    response.set_status(StatusCode::try_from(502).unwrap());

                                                    response.headers_mut().set(ContentType(mimetypes::responses::GET_OFFSET_OPERATION_FAILED.clone()));


                                                    let body = serde_json::to_string(&body).expect("impossible to fail to serialize");

                                                    response.set_body(body);
                                                },
                                            },
                                            Err(_) => {
                                                // Application code returned an error. This should not happen, as the implementation should
                                                // return a valid response.
                                                response.set_status(StatusCode::InternalServerError);
                                                response.set_body("An internal error occurred");
                                            },
                                        }

                                        future::ok(response)
                                    }
                                ))

                        }}
                }) as Box<Future<Item=Response, Error=Error>>


            },


            // GetOutputChange - GET /pca9956b/{busId}/{addr}/outputChange
            &hyper::Method::Get if path.matched(paths::ID_PCA9956B_BUSID_ADDR_OUTPUTCHANGE) => {


                // Path parameters
                let path = uri.path().to_string();
                let path_params =
                    paths::REGEX_PCA9956B_BUSID_ADDR_OUTPUTCHANGE
                    .captures(&path)
                    .unwrap_or_else(||
                        panic!("Path {} matched RE PCA9956B_BUSID_ADDR_OUTPUTCHANGE in set but failed match against \"{}\"", path, paths::REGEX_PCA9956B_BUSID_ADDR_OUTPUTCHANGE.as_str())
                    );

                let param_bus_id = match percent_encoding::percent_decode(path_params["busId"].as_bytes()).decode_utf8() {
                    Ok(param_bus_id) => match param_bus_id.parse::<i32>() {
                        Ok(param_bus_id) => param_bus_id,
                        Err(e) => return Box::new(future::ok(Response::new().with_status(StatusCode::BadRequest).with_body(format!("Couldn't parse path parameter busId: {}", e)))),
                    },
                    Err(_) => return Box::new(future::ok(Response::new().with_status(StatusCode::BadRequest).with_body(format!("Couldn't percent-decode path parameter as UTF-8: {}", &path_params["busId"]))))
                };
                let param_addr = match percent_encoding::percent_decode(path_params["addr"].as_bytes()).decode_utf8() {
                    Ok(param_addr) => match param_addr.parse::<i32>() {
                        Ok(param_addr) => param_addr,
                        Err(e) => return Box::new(future::ok(Response::new().with_status(StatusCode::BadRequest).with_body(format!("Couldn't parse path parameter addr: {}", e)))),
                    },
                    Err(_) => return Box::new(future::ok(Response::new().with_status(StatusCode::BadRequest).with_body(format!("Couldn't percent-decode path parameter as UTF-8: {}", &path_params["addr"]))))
                };





                Box::new({
                        {{

                                Box::new(api_impl.get_output_change(param_bus_id, param_addr, &context)
                                    .then(move |result| {
                                        let mut response = Response::new();
                                        response.headers_mut().set(XSpanId((&context as &Has<XSpanIdString>).get().0.to_string()));

                                        match result {
                                            Ok(rsp) => match rsp {
                                                GetOutputChangeResponse::OK

                                                    (body)


                                                => {
                                                    response.set_status(StatusCode::try_from(200).unwrap());

                                                    response.headers_mut().set(ContentType(mimetypes::responses::GET_OUTPUT_CHANGE_OK.clone()));


                                                    let body = serde_json::to_string(&body).expect("impossible to fail to serialize");

                                                    response.set_body(body);
                                                },
                                                GetOutputChangeResponse::BadRequest

                                                    (body)


                                                => {
                                                    response.set_status(StatusCode::try_from(400).unwrap());

                                                    response.headers_mut().set(ContentType(mimetypes::responses::GET_OUTPUT_CHANGE_BAD_REQUEST.clone()));


                                                    let body = serde_json::to_string(&body).expect("impossible to fail to serialize");

                                                    response.set_body(body);
                                                },
                                                GetOutputChangeResponse::OperationFailed

                                                    (body)


                                                => {
                                                    response.set_status(StatusCode::try_from(502).unwrap());

                                                    response.headers_mut().set(ContentType(mimetypes::responses::GET_OUTPUT_CHANGE_OPERATION_FAILED.clone()));


                                                    let body = serde_json::to_string(&body).expect("impossible to fail to serialize");

                                                    response.set_body(body);
                                                },
                                            },
                                            Err(_) => {
                                                // Application code returned an error. This should not happen, as the implementation should
                                                // return a valid response.
                                                response.set_status(StatusCode::InternalServerError);
                                                response.set_body("An internal error occurred");
                                            },
                                        }

                                        future::ok(response)
                                    }
                                ))

                        }}
                }) as Box<Future<Item=Response, Error=Error>>


            },


            // GetOverTemp - GET /pca9956b/{busId}/{addr}/overTemp
            &hyper::Method::Get if path.matched(paths::ID_PCA9956B_BUSID_ADDR_OVERTEMP) => {


                // Path parameters
                let path = uri.path().to_string();
                let path_params =
                    paths::REGEX_PCA9956B_BUSID_ADDR_OVERTEMP
                    .captures(&path)
                    .unwrap_or_else(||
                        panic!("Path {} matched RE PCA9956B_BUSID_ADDR_OVERTEMP in set but failed match against \"{}\"", path, paths::REGEX_PCA9956B_BUSID_ADDR_OVERTEMP.as_str())
                    );

                let param_bus_id = match percent_encoding::percent_decode(path_params["busId"].as_bytes()).decode_utf8() {
                    Ok(param_bus_id) => match param_bus_id.parse::<i32>() {
                        Ok(param_bus_id) => param_bus_id,
                        Err(e) => return Box::new(future::ok(Response::new().with_status(StatusCode::BadRequest).with_body(format!("Couldn't parse path parameter busId: {}", e)))),
                    },
                    Err(_) => return Box::new(future::ok(Response::new().with_status(StatusCode::BadRequest).with_body(format!("Couldn't percent-decode path parameter as UTF-8: {}", &path_params["busId"]))))
                };
                let param_addr = match percent_encoding::percent_decode(path_params["addr"].as_bytes()).decode_utf8() {
                    Ok(param_addr) => match param_addr.parse::<i32>() {
                        Ok(param_addr) => param_addr,
                        Err(e) => return Box::new(future::ok(Response::new().with_status(StatusCode::BadRequest).with_body(format!("Couldn't parse path parameter addr: {}", e)))),
                    },
                    Err(_) => return Box::new(future::ok(Response::new().with_status(StatusCode::BadRequest).with_body(format!("Couldn't percent-decode path parameter as UTF-8: {}", &path_params["addr"]))))
                };





                Box::new({
                        {{

                                Box::new(api_impl.get_over_temp(param_bus_id, param_addr, &context)
                                    .then(move |result| {
                                        let mut response = Response::new();
                                        response.headers_mut().set(XSpanId((&context as &Has<XSpanIdString>).get().0.to_string()));

                                        match result {
                                            Ok(rsp) => match rsp {
                                                GetOverTempResponse::OK

                                                    (body)


                                                => {
                                                    response.set_status(StatusCode::try_from(200).unwrap());

                                                    response.headers_mut().set(ContentType(mimetypes::responses::GET_OVER_TEMP_OK.clone()));


                                                    let body = serde_json::to_string(&body).expect("impossible to fail to serialize");

                                                    response.set_body(body);
                                                },
                                                GetOverTempResponse::BadRequest

                                                    (body)


                                                => {
                                                    response.set_status(StatusCode::try_from(400).unwrap());

                                                    response.headers_mut().set(ContentType(mimetypes::responses::GET_OVER_TEMP_BAD_REQUEST.clone()));


                                                    let body = serde_json::to_string(&body).expect("impossible to fail to serialize");

                                                    response.set_body(body);
                                                },
                                                GetOverTempResponse::OperationFailed

                                                    (body)


                                                => {
                                                    response.set_status(StatusCode::try_from(502).unwrap());

                                                    response.headers_mut().set(ContentType(mimetypes::responses::GET_OVER_TEMP_OPERATION_FAILED.clone()));


                                                    let body = serde_json::to_string(&body).expect("impossible to fail to serialize");

                                                    response.set_body(body);
                                                },
                                            },
                                            Err(_) => {
                                                // Application code returned an error. This should not happen, as the implementation should
                                                // return a valid response.
                                                response.set_status(StatusCode::InternalServerError);
                                                response.set_body("An internal error occurred");
                                            },
                                        }

                                        future::ok(response)
                                    }
                                ))

                        }}
                }) as Box<Future<Item=Response, Error=Error>>


            },


            // GetPwm - GET /pca9956b/{busId}/{addr}/pwm
            &hyper::Method::Get if path.matched(paths::ID_PCA9956B_BUSID_ADDR_PWM) => {


                // Path parameters
                let path = uri.path().to_string();
                let path_params =
                    paths::REGEX_PCA9956B_BUSID_ADDR_PWM
                    .captures(&path)
                    .unwrap_or_else(||
                        panic!("Path {} matched RE PCA9956B_BUSID_ADDR_PWM in set but failed match against \"{}\"", path, paths::REGEX_PCA9956B_BUSID_ADDR_PWM.as_str())
                    );

                let param_bus_id = match percent_encoding::percent_decode(path_params["busId"].as_bytes()).decode_utf8() {
                    Ok(param_bus_id) => match param_bus_id.parse::<i32>() {
                        Ok(param_bus_id) => param_bus_id,
                        Err(e) => return Box::new(future::ok(Response::new().with_status(StatusCode::BadRequest).with_body(format!("Couldn't parse path parameter busId: {}", e)))),
                    },
                    Err(_) => return Box::new(future::ok(Response::new().with_status(StatusCode::BadRequest).with_body(format!("Couldn't percent-decode path parameter as UTF-8: {}", &path_params["busId"]))))
                };
                let param_addr = match percent_encoding::percent_decode(path_params["addr"].as_bytes()).decode_utf8() {
                    Ok(param_addr) => match param_addr.parse::<i32>() {
                        Ok(param_addr) => param_addr,
                        Err(e) => return Box::new(future::ok(Response::new().with_status(StatusCode::BadRequest).with_body(format!("Couldn't parse path parameter addr: {}", e)))),
                    },
                    Err(_) => return Box::new(future::ok(Response::new().with_status(StatusCode::BadRequest).with_body(format!("Couldn't percent-decode path parameter as UTF-8: {}", &path_params["addr"]))))
                };





                Box::new({
                        {{

                                Box::new(api_impl.get_pwm(param_bus_id, param_addr, &context)
                                    .then(move |result| {
                                        let mut response = Response::new();
                                        response.headers_mut().set(XSpanId((&context as &Has<XSpanIdString>).get().0.to_string()));

                                        match result {
                                            Ok(rsp) => match rsp {
                                                GetPwmResponse::OK

                                                    (body)


                                                => {
                                                    response.set_status(StatusCode::try_from(200).unwrap());

                                                    response.headers_mut().set(ContentType(mimetypes::responses::GET_PWM_OK.clone()));


                                                    let body = serde_json::to_string(&body).expect("impossible to fail to serialize");

                                                    response.set_body(body);
                                                },
                                                GetPwmResponse::BadRequest

                                                    (body)


                                                => {
                                                    response.set_status(StatusCode::try_from(400).unwrap());

                                                    response.headers_mut().set(ContentType(mimetypes::responses::GET_PWM_BAD_REQUEST.clone()));


                                                    let body = serde_json::to_string(&body).expect("impossible to fail to serialize");

                                                    response.set_body(body);
                                                },
                                                GetPwmResponse::OperationFailed

                                                    (body)


                                                => {
                                                    response.set_status(StatusCode::try_from(502).unwrap());

                                                    response.headers_mut().set(ContentType(mimetypes::responses::GET_PWM_OPERATION_FAILED.clone()));


                                                    let body = serde_json::to_string(&body).expect("impossible to fail to serialize");

                                                    response.set_body(body);
                                                },
                                            },
                                            Err(_) => {
                                                // Application code returned an error. This should not happen, as the implementation should
                                                // return a valid response.
                                                response.set_status(StatusCode::InternalServerError);
                                                response.set_body("An internal error occurred");
                                            },
                                        }

                                        future::ok(response)
                                    }
                                ))

                        }}
                }) as Box<Future<Item=Response, Error=Error>>


            },


            // GetSleep - GET /pca9956b/{busId}/{addr}/sleep
            &hyper::Method::Get if path.matched(paths::ID_PCA9956B_BUSID_ADDR_SLEEP) => {


                // Path parameters
                let path = uri.path().to_string();
                let path_params =
                    paths::REGEX_PCA9956B_BUSID_ADDR_SLEEP
                    .captures(&path)
                    .unwrap_or_else(||
                        panic!("Path {} matched RE PCA9956B_BUSID_ADDR_SLEEP in set but failed match against \"{}\"", path, paths::REGEX_PCA9956B_BUSID_ADDR_SLEEP.as_str())
                    );

                let param_bus_id = match percent_encoding::percent_decode(path_params["busId"].as_bytes()).decode_utf8() {
                    Ok(param_bus_id) => match param_bus_id.parse::<i32>() {
                        Ok(param_bus_id) => param_bus_id,
                        Err(e) => return Box::new(future::ok(Response::new().with_status(StatusCode::BadRequest).with_body(format!("Couldn't parse path parameter busId: {}", e)))),
                    },
                    Err(_) => return Box::new(future::ok(Response::new().with_status(StatusCode::BadRequest).with_body(format!("Couldn't percent-decode path parameter as UTF-8: {}", &path_params["busId"]))))
                };
                let param_addr = match percent_encoding::percent_decode(path_params["addr"].as_bytes()).decode_utf8() {
                    Ok(param_addr) => match param_addr.parse::<i32>() {
                        Ok(param_addr) => param_addr,
                        Err(e) => return Box::new(future::ok(Response::new().with_status(StatusCode::BadRequest).with_body(format!("Couldn't parse path parameter addr: {}", e)))),
                    },
                    Err(_) => return Box::new(future::ok(Response::new().with_status(StatusCode::BadRequest).with_body(format!("Couldn't percent-decode path parameter as UTF-8: {}", &path_params["addr"]))))
                };





                Box::new({
                        {{

                                Box::new(api_impl.get_sleep(param_bus_id, param_addr, &context)
                                    .then(move |result| {
                                        let mut response = Response::new();
                                        response.headers_mut().set(XSpanId((&context as &Has<XSpanIdString>).get().0.to_string()));

                                        match result {
                                            Ok(rsp) => match rsp {
                                                GetSleepResponse::OK

                                                    (body)


                                                => {
                                                    response.set_status(StatusCode::try_from(200).unwrap());

                                                    response.headers_mut().set(ContentType(mimetypes::responses::GET_SLEEP_OK.clone()));


                                                    let body = serde_json::to_string(&body).expect("impossible to fail to serialize");

                                                    response.set_body(body);
                                                },
                                                GetSleepResponse::BadRequest

                                                    (body)


                                                => {
                                                    response.set_status(StatusCode::try_from(400).unwrap());

                                                    response.headers_mut().set(ContentType(mimetypes::responses::GET_SLEEP_BAD_REQUEST.clone()));


                                                    let body = serde_json::to_string(&body).expect("impossible to fail to serialize");

                                                    response.set_body(body);
                                                },
                                                GetSleepResponse::OperationFailed

                                                    (body)


                                                => {
                                                    response.set_status(StatusCode::try_from(502).unwrap());

                                                    response.headers_mut().set(ContentType(mimetypes::responses::GET_SLEEP_OPERATION_FAILED.clone()));


                                                    let body = serde_json::to_string(&body).expect("impossible to fail to serialize");

                                                    response.set_body(body);
                                                },
                                            },
                                            Err(_) => {
                                                // Application code returned an error. This should not happen, as the implementation should
                                                // return a valid response.
                                                response.set_status(StatusCode::InternalServerError);
                                                response.set_body("An internal error occurred");
                                            },
                                        }

                                        future::ok(response)
                                    }
                                ))

                        }}
                }) as Box<Future<Item=Response, Error=Error>>


            },


            // Reset - POST /pca9956b/{busId}/reset
            &hyper::Method::Post if path.matched(paths::ID_PCA9956B_BUSID_RESET) => {


                // Path parameters
                let path = uri.path().to_string();
                let path_params =
                    paths::REGEX_PCA9956B_BUSID_RESET
                    .captures(&path)
                    .unwrap_or_else(||
                        panic!("Path {} matched RE PCA9956B_BUSID_RESET in set but failed match against \"{}\"", path, paths::REGEX_PCA9956B_BUSID_RESET.as_str())
                    );

                let param_bus_id = match percent_encoding::percent_decode(path_params["busId"].as_bytes()).decode_utf8() {
                    Ok(param_bus_id) => match param_bus_id.parse::<i32>() {
                        Ok(param_bus_id) => param_bus_id,
                        Err(e) => return Box::new(future::ok(Response::new().with_status(StatusCode::BadRequest).with_body(format!("Couldn't parse path parameter busId: {}", e)))),
                    },
                    Err(_) => return Box::new(future::ok(Response::new().with_status(StatusCode::BadRequest).with_body(format!("Couldn't percent-decode path parameter as UTF-8: {}", &path_params["busId"]))))
                };





                Box::new({
                        {{

                                Box::new(api_impl.reset(param_bus_id, &context)
                                    .then(move |result| {
                                        let mut response = Response::new();
                                        response.headers_mut().set(XSpanId((&context as &Has<XSpanIdString>).get().0.to_string()));

                                        match result {
                                            Ok(rsp) => match rsp {
                                                ResetResponse::OK


                                                => {
                                                    response.set_status(StatusCode::try_from(200).unwrap());

                                                },
                                                ResetResponse::BadRequest

                                                    (body)


                                                => {
                                                    response.set_status(StatusCode::try_from(400).unwrap());

                                                    response.headers_mut().set(ContentType(mimetypes::responses::RESET_BAD_REQUEST.clone()));


                                                    let body = serde_json::to_string(&body).expect("impossible to fail to serialize");

                                                    response.set_body(body);
                                                },
                                                ResetResponse::OperationFailed

                                                    (body)


                                                => {
                                                    response.set_status(StatusCode::try_from(502).unwrap());

                                                    response.headers_mut().set(ContentType(mimetypes::responses::RESET_OPERATION_FAILED.clone()));


                                                    let body = serde_json::to_string(&body).expect("impossible to fail to serialize");

                                                    response.set_body(body);
                                                },
                                            },
                                            Err(_) => {
                                                // Application code returned an error. This should not happen, as the implementation should
                                                // return a valid response.
                                                response.set_status(StatusCode::InternalServerError);
                                                response.set_body("An internal error occurred");
                                            },
                                        }

                                        future::ok(response)
                                    }
                                ))

                        }}
                }) as Box<Future<Item=Response, Error=Error>>


            },


            // SetAddrEnabled - POST /pca9956b/{busId}/{addr}/addr/{num}/enabled/{enabled}
            &hyper::Method::Post if path.matched(paths::ID_PCA9956B_BUSID_ADDR_ADDR_NUM_ENABLED_ENABLED) => {


                // Path parameters
                let path = uri.path().to_string();
                let path_params =
                    paths::REGEX_PCA9956B_BUSID_ADDR_ADDR_NUM_ENABLED_ENABLED
                    .captures(&path)
                    .unwrap_or_else(||
                        panic!("Path {} matched RE PCA9956B_BUSID_ADDR_ADDR_NUM_ENABLED_ENABLED in set but failed match against \"{}\"", path, paths::REGEX_PCA9956B_BUSID_ADDR_ADDR_NUM_ENABLED_ENABLED.as_str())
                    );

                let param_bus_id = match percent_encoding::percent_decode(path_params["busId"].as_bytes()).decode_utf8() {
                    Ok(param_bus_id) => match param_bus_id.parse::<i32>() {
                        Ok(param_bus_id) => param_bus_id,
                        Err(e) => return Box::new(future::ok(Response::new().with_status(StatusCode::BadRequest).with_body(format!("Couldn't parse path parameter busId: {}", e)))),
                    },
                    Err(_) => return Box::new(future::ok(Response::new().with_status(StatusCode::BadRequest).with_body(format!("Couldn't percent-decode path parameter as UTF-8: {}", &path_params["busId"]))))
                };
                let param_addr = match percent_encoding::percent_decode(path_params["addr"].as_bytes()).decode_utf8() {
                    Ok(param_addr) => match param_addr.parse::<i32>() {
                        Ok(param_addr) => param_addr,
                        Err(e) => return Box::new(future::ok(Response::new().with_status(StatusCode::BadRequest).with_body(format!("Couldn't parse path parameter addr: {}", e)))),
                    },
                    Err(_) => return Box::new(future::ok(Response::new().with_status(StatusCode::BadRequest).with_body(format!("Couldn't percent-decode path parameter as UTF-8: {}", &path_params["addr"]))))
                };
                let param_num = match percent_encoding::percent_decode(path_params["num"].as_bytes()).decode_utf8() {
                    Ok(param_num) => match param_num.parse::<i32>() {
                        Ok(param_num) => param_num,
                        Err(e) => return Box::new(future::ok(Response::new().with_status(StatusCode::BadRequest).with_body(format!("Couldn't parse path parameter num: {}", e)))),
                    },
                    Err(_) => return Box::new(future::ok(Response::new().with_status(StatusCode::BadRequest).with_body(format!("Couldn't percent-decode path parameter as UTF-8: {}", &path_params["num"]))))
                };
                let param_enabled = match percent_encoding::percent_decode(path_params["enabled"].as_bytes()).decode_utf8() {
                    Ok(param_enabled) => match param_enabled.parse::<bool>() {
                        Ok(param_enabled) => param_enabled,
                        Err(e) => return Box::new(future::ok(Response::new().with_status(StatusCode::BadRequest).with_body(format!("Couldn't parse path parameter enabled: {}", e)))),
                    },
                    Err(_) => return Box::new(future::ok(Response::new().with_status(StatusCode::BadRequest).with_body(format!("Couldn't percent-decode path parameter as UTF-8: {}", &path_params["enabled"]))))
                };





                Box::new({
                        {{

                                Box::new(api_impl.set_addr_enabled(param_bus_id, param_addr, param_num, param_enabled, &context)
                                    .then(move |result| {
                                        let mut response = Response::new();
                                        response.headers_mut().set(XSpanId((&context as &Has<XSpanIdString>).get().0.to_string()));

                                        match result {
                                            Ok(rsp) => match rsp {
                                                SetAddrEnabledResponse::OK


                                                => {
                                                    response.set_status(StatusCode::try_from(200).unwrap());

                                                },
                                                SetAddrEnabledResponse::BadRequest

                                                    (body)


                                                => {
                                                    response.set_status(StatusCode::try_from(400).unwrap());

                                                    response.headers_mut().set(ContentType(mimetypes::responses::SET_ADDR_ENABLED_BAD_REQUEST.clone()));


                                                    let body = serde_json::to_string(&body).expect("impossible to fail to serialize");

                                                    response.set_body(body);
                                                },
                                                SetAddrEnabledResponse::OperationFailed

                                                    (body)


                                                => {
                                                    response.set_status(StatusCode::try_from(502).unwrap());

                                                    response.headers_mut().set(ContentType(mimetypes::responses::SET_ADDR_ENABLED_OPERATION_FAILED.clone()));


                                                    let body = serde_json::to_string(&body).expect("impossible to fail to serialize");

                                                    response.set_body(body);
                                                },
                                            },
                                            Err(_) => {
                                                // Application code returned an error. This should not happen, as the implementation should
                                                // return a valid response.
                                                response.set_status(StatusCode::InternalServerError);
                                                response.set_body("An internal error occurred");
                                            },
                                        }

                                        future::ok(response)
                                    }
                                ))

                        }}
                }) as Box<Future<Item=Response, Error=Error>>


            },


            // SetAddrValue - POST /pca9956b/{busId}/{addr}/addr/{num}/addr/{addrVal}
            &hyper::Method::Post if path.matched(paths::ID_PCA9956B_BUSID_ADDR_ADDR_NUM_ADDR_ADDRVAL) => {


                // Path parameters
                let path = uri.path().to_string();
                let path_params =
                    paths::REGEX_PCA9956B_BUSID_ADDR_ADDR_NUM_ADDR_ADDRVAL
                    .captures(&path)
                    .unwrap_or_else(||
                        panic!("Path {} matched RE PCA9956B_BUSID_ADDR_ADDR_NUM_ADDR_ADDRVAL in set but failed match against \"{}\"", path, paths::REGEX_PCA9956B_BUSID_ADDR_ADDR_NUM_ADDR_ADDRVAL.as_str())
                    );

                let param_bus_id = match percent_encoding::percent_decode(path_params["busId"].as_bytes()).decode_utf8() {
                    Ok(param_bus_id) => match param_bus_id.parse::<i32>() {
                        Ok(param_bus_id) => param_bus_id,
                        Err(e) => return Box::new(future::ok(Response::new().with_status(StatusCode::BadRequest).with_body(format!("Couldn't parse path parameter busId: {}", e)))),
                    },
                    Err(_) => return Box::new(future::ok(Response::new().with_status(StatusCode::BadRequest).with_body(format!("Couldn't percent-decode path parameter as UTF-8: {}", &path_params["busId"]))))
                };
                let param_addr = match percent_encoding::percent_decode(path_params["addr"].as_bytes()).decode_utf8() {
                    Ok(param_addr) => match param_addr.parse::<i32>() {
                        Ok(param_addr) => param_addr,
                        Err(e) => return Box::new(future::ok(Response::new().with_status(StatusCode::BadRequest).with_body(format!("Couldn't parse path parameter addr: {}", e)))),
                    },
                    Err(_) => return Box::new(future::ok(Response::new().with_status(StatusCode::BadRequest).with_body(format!("Couldn't percent-decode path parameter as UTF-8: {}", &path_params["addr"]))))
                };
                let param_num = match percent_encoding::percent_decode(path_params["num"].as_bytes()).decode_utf8() {
                    Ok(param_num) => match param_num.parse::<i32>() {
                        Ok(param_num) => param_num,
                        Err(e) => return Box::new(future::ok(Response::new().with_status(StatusCode::BadRequest).with_body(format!("Couldn't parse path parameter num: {}", e)))),
                    },
                    Err(_) => return Box::new(future::ok(Response::new().with_status(StatusCode::BadRequest).with_body(format!("Couldn't percent-decode path parameter as UTF-8: {}", &path_params["num"]))))
                };
                let param_addr_val = match percent_encoding::percent_decode(path_params["addrVal"].as_bytes()).decode_utf8() {
                    Ok(param_addr_val) => match param_addr_val.parse::<i32>() {
                        Ok(param_addr_val) => param_addr_val,
                        Err(e) => return Box::new(future::ok(Response::new().with_status(StatusCode::BadRequest).with_body(format!("Couldn't parse path parameter addrVal: {}", e)))),
                    },
                    Err(_) => return Box::new(future::ok(Response::new().with_status(StatusCode::BadRequest).with_body(format!("Couldn't percent-decode path parameter as UTF-8: {}", &path_params["addrVal"]))))
                };





                Box::new({
                        {{

                                Box::new(api_impl.set_addr_value(param_bus_id, param_addr, param_num, param_addr_val, &context)
                                    .then(move |result| {
                                        let mut response = Response::new();
                                        response.headers_mut().set(XSpanId((&context as &Has<XSpanIdString>).get().0.to_string()));

                                        match result {
                                            Ok(rsp) => match rsp {
                                                SetAddrValueResponse::OK


                                                => {
                                                    response.set_status(StatusCode::try_from(200).unwrap());

                                                },
                                                SetAddrValueResponse::BadRequest

                                                    (body)


                                                => {
                                                    response.set_status(StatusCode::try_from(400).unwrap());

                                                    response.headers_mut().set(ContentType(mimetypes::responses::SET_ADDR_VALUE_BAD_REQUEST.clone()));


                                                    let body = serde_json::to_string(&body).expect("impossible to fail to serialize");

                                                    response.set_body(body);
                                                },
                                                SetAddrValueResponse::OperationFailed

                                                    (body)


                                                => {
                                                    response.set_status(StatusCode::try_from(502).unwrap());

                                                    response.headers_mut().set(ContentType(mimetypes::responses::SET_ADDR_VALUE_OPERATION_FAILED.clone()));


                                                    let body = serde_json::to_string(&body).expect("impossible to fail to serialize");

                                                    response.set_body(body);
                                                },
                                            },
                                            Err(_) => {
                                                // Application code returned an error. This should not happen, as the implementation should
                                                // return a valid response.
                                                response.set_status(StatusCode::InternalServerError);
                                                response.set_body("An internal error occurred");
                                            },
                                        }

                                        future::ok(response)
                                    }
                                ))

                        }}
                }) as Box<Future<Item=Response, Error=Error>>


            },


            // SetConfig - POST /pca9956b/{busId}/{addr}/config
            &hyper::Method::Post if path.matched(paths::ID_PCA9956B_BUSID_ADDR_CONFIG) => {


                // Path parameters
                let path = uri.path().to_string();
                let path_params =
                    paths::REGEX_PCA9956B_BUSID_ADDR_CONFIG
                    .captures(&path)
                    .unwrap_or_else(||
                        panic!("Path {} matched RE PCA9956B_BUSID_ADDR_CONFIG in set but failed match against \"{}\"", path, paths::REGEX_PCA9956B_BUSID_ADDR_CONFIG.as_str())
                    );

                let param_bus_id = match percent_encoding::percent_decode(path_params["busId"].as_bytes()).decode_utf8() {
                    Ok(param_bus_id) => match param_bus_id.parse::<i32>() {
                        Ok(param_bus_id) => param_bus_id,
                        Err(e) => return Box::new(future::ok(Response::new().with_status(StatusCode::BadRequest).with_body(format!("Couldn't parse path parameter busId: {}", e)))),
                    },
                    Err(_) => return Box::new(future::ok(Response::new().with_status(StatusCode::BadRequest).with_body(format!("Couldn't percent-decode path parameter as UTF-8: {}", &path_params["busId"]))))
                };
                let param_addr = match percent_encoding::percent_decode(path_params["addr"].as_bytes()).decode_utf8() {
                    Ok(param_addr) => match param_addr.parse::<i32>() {
                        Ok(param_addr) => param_addr,
                        Err(e) => return Box::new(future::ok(Response::new().with_status(StatusCode::BadRequest).with_body(format!("Couldn't parse path parameter addr: {}", e)))),
                    },
                    Err(_) => return Box::new(future::ok(Response::new().with_status(StatusCode::BadRequest).with_body(format!("Couldn't percent-decode path parameter as UTF-8: {}", &path_params["addr"]))))
                };




                // Body parameters (note that non-required body parameters will ignore garbage
                // values, rather than causing a 400 response). Produce warning header and logs for
                // any unused fields.
                Box::new(body.concat2()
                    .then(move |result| -> Box<Future<Item=Response, Error=Error>> {
                        match result {
                            Ok(body) => {

                                let mut unused_elements = Vec::new();
                                let param_config: Option<models::Config> = if !body.is_empty() {

                                    let deserializer = &mut serde_json::Deserializer::from_slice(&*body);

                                    match serde_ignored::deserialize(deserializer, |path| {
                                            warn!("Ignoring unknown field in body: {}", path);
                                            unused_elements.push(path.to_string());
                                    }) {
                                        Ok(param_config) => param_config,
                                        Err(e) => return Box::new(future::ok(Response::new().with_status(StatusCode::BadRequest).with_body(format!("Couldn't parse body parameter Config - doesn't match schema: {}", e)))),
                                    }

                                } else {
                                    None
                                };
                                let param_config = match param_config {
                                    Some(param_config) => param_config,
                                    None => return Box::new(future::ok(Response::new().with_status(StatusCode::BadRequest).with_body("Missing required body parameter Config"))),
                                };


                                Box::new(api_impl.set_config(param_bus_id, param_addr, param_config, &context)
                                    .then(move |result| {
                                        let mut response = Response::new();
                                        response.headers_mut().set(XSpanId((&context as &Has<XSpanIdString>).get().0.to_string()));

                                        if !unused_elements.is_empty() {
                                            response.headers_mut().set(Warning(format!("Ignoring unknown fields in body: {:?}", unused_elements)));
                                        }

                                        match result {
                                            Ok(rsp) => match rsp {
                                                SetConfigResponse::OK


                                                => {
                                                    response.set_status(StatusCode::try_from(200).unwrap());

                                                },
                                                SetConfigResponse::BadRequest

                                                    (body)


                                                => {
                                                    response.set_status(StatusCode::try_from(400).unwrap());

                                                    response.headers_mut().set(ContentType(mimetypes::responses::SET_CONFIG_BAD_REQUEST.clone()));


                                                    let body = serde_json::to_string(&body).expect("impossible to fail to serialize");

                                                    response.set_body(body);
                                                },
                                                SetConfigResponse::OperationFailed

                                                    (body)


                                                => {
                                                    response.set_status(StatusCode::try_from(502).unwrap());

                                                    response.headers_mut().set(ContentType(mimetypes::responses::SET_CONFIG_OPERATION_FAILED.clone()));


                                                    let body = serde_json::to_string(&body).expect("impossible to fail to serialize");

                                                    response.set_body(body);
                                                },
                                            },
                                            Err(_) => {
                                                // Application code returned an error. This should not happen, as the implementation should
                                                // return a valid response.
                                                response.set_status(StatusCode::InternalServerError);
                                                response.set_body("An internal error occurred");
                                            },
                                        }

                                        future::ok(response)
                                    }
                                ))


                            },
                            Err(e) => Box::new(future::ok(Response::new().with_status(StatusCode::BadRequest).with_body(format!("Couldn't read body parameter Config: {}", e)))),
                        }
                    })
                ) as Box<Future<Item=Response, Error=Error>>

            },


            // SetCurrent - POST /pca9956b/{busId}/{addr}/current/{current}
            &hyper::Method::Post if path.matched(paths::ID_PCA9956B_BUSID_ADDR_CURRENT_CURRENT) => {


                // Path parameters
                let path = uri.path().to_string();
                let path_params =
                    paths::REGEX_PCA9956B_BUSID_ADDR_CURRENT_CURRENT
                    .captures(&path)
                    .unwrap_or_else(||
                        panic!("Path {} matched RE PCA9956B_BUSID_ADDR_CURRENT_CURRENT in set but failed match against \"{}\"", path, paths::REGEX_PCA9956B_BUSID_ADDR_CURRENT_CURRENT.as_str())
                    );

                let param_bus_id = match percent_encoding::percent_decode(path_params["busId"].as_bytes()).decode_utf8() {
                    Ok(param_bus_id) => match param_bus_id.parse::<i32>() {
                        Ok(param_bus_id) => param_bus_id,
                        Err(e) => return Box::new(future::ok(Response::new().with_status(StatusCode::BadRequest).with_body(format!("Couldn't parse path parameter busId: {}", e)))),
                    },
                    Err(_) => return Box::new(future::ok(Response::new().with_status(StatusCode::BadRequest).with_body(format!("Couldn't percent-decode path parameter as UTF-8: {}", &path_params["busId"]))))
                };
                let param_addr = match percent_encoding::percent_decode(path_params["addr"].as_bytes()).decode_utf8() {
                    Ok(param_addr) => match param_addr.parse::<i32>() {
                        Ok(param_addr) => param_addr,
                        Err(e) => return Box::new(future::ok(Response::new().with_status(StatusCode::BadRequest).with_body(format!("Couldn't parse path parameter addr: {}", e)))),
                    },
                    Err(_) => return Box::new(future::ok(Response::new().with_status(StatusCode::BadRequest).with_body(format!("Couldn't percent-decode path parameter as UTF-8: {}", &path_params["addr"]))))
                };
                let param_current = match percent_encoding::percent_decode(path_params["current"].as_bytes()).decode_utf8() {
                    Ok(param_current) => match param_current.parse::<i32>() {
                        Ok(param_current) => param_current,
                        Err(e) => return Box::new(future::ok(Response::new().with_status(StatusCode::BadRequest).with_body(format!("Couldn't parse path parameter current: {}", e)))),
                    },
                    Err(_) => return Box::new(future::ok(Response::new().with_status(StatusCode::BadRequest).with_body(format!("Couldn't percent-decode path parameter as UTF-8: {}", &path_params["current"]))))
                };





                Box::new({
                        {{

                                Box::new(api_impl.set_current(param_bus_id, param_addr, param_current, &context)
                                    .then(move |result| {
                                        let mut response = Response::new();
                                        response.headers_mut().set(XSpanId((&context as &Has<XSpanIdString>).get().0.to_string()));

                                        match result {
                                            Ok(rsp) => match rsp {
                                                SetCurrentResponse::OK


                                                => {
                                                    response.set_status(StatusCode::try_from(200).unwrap());

                                                },
                                                SetCurrentResponse::BadRequest

                                                    (body)


                                                => {
                                                    response.set_status(StatusCode::try_from(400).unwrap());

                                                    response.headers_mut().set(ContentType(mimetypes::responses::SET_CURRENT_BAD_REQUEST.clone()));


                                                    let body = serde_json::to_string(&body).expect("impossible to fail to serialize");

                                                    response.set_body(body);
                                                },
                                                SetCurrentResponse::OperationFailed

                                                    (body)


                                                => {
                                                    response.set_status(StatusCode::try_from(502).unwrap());

                                                    response.headers_mut().set(ContentType(mimetypes::responses::SET_CURRENT_OPERATION_FAILED.clone()));


                                                    let body = serde_json::to_string(&body).expect("impossible to fail to serialize");

                                                    response.set_body(body);
                                                },
                                            },
                                            Err(_) => {
                                                // Application code returned an error. This should not happen, as the implementation should
                                                // return a valid response.
                                                response.set_status(StatusCode::InternalServerError);
                                                response.set_body("An internal error occurred");
                                            },
                                        }

                                        future::ok(response)
                                    }
                                ))

                        }}
                }) as Box<Future<Item=Response, Error=Error>>


            },


            // SetFreq - POST /pca9956b/{busId}/{addr}/freq/{freq}
            &hyper::Method::Post if path.matched(paths::ID_PCA9956B_BUSID_ADDR_FREQ_FREQ) => {


                // Path parameters
                let path = uri.path().to_string();
                let path_params =
                    paths::REGEX_PCA9956B_BUSID_ADDR_FREQ_FREQ
                    .captures(&path)
                    .unwrap_or_else(||
                        panic!("Path {} matched RE PCA9956B_BUSID_ADDR_FREQ_FREQ in set but failed match against \"{}\"", path, paths::REGEX_PCA9956B_BUSID_ADDR_FREQ_FREQ.as_str())
                    );

                let param_bus_id = match percent_encoding::percent_decode(path_params["busId"].as_bytes()).decode_utf8() {
                    Ok(param_bus_id) => match param_bus_id.parse::<i32>() {
                        Ok(param_bus_id) => param_bus_id,
                        Err(e) => return Box::new(future::ok(Response::new().with_status(StatusCode::BadRequest).with_body(format!("Couldn't parse path parameter busId: {}", e)))),
                    },
                    Err(_) => return Box::new(future::ok(Response::new().with_status(StatusCode::BadRequest).with_body(format!("Couldn't percent-decode path parameter as UTF-8: {}", &path_params["busId"]))))
                };
                let param_addr = match percent_encoding::percent_decode(path_params["addr"].as_bytes()).decode_utf8() {
                    Ok(param_addr) => match param_addr.parse::<i32>() {
                        Ok(param_addr) => param_addr,
                        Err(e) => return Box::new(future::ok(Response::new().with_status(StatusCode::BadRequest).with_body(format!("Couldn't parse path parameter addr: {}", e)))),
                    },
                    Err(_) => return Box::new(future::ok(Response::new().with_status(StatusCode::BadRequest).with_body(format!("Couldn't percent-decode path parameter as UTF-8: {}", &path_params["addr"]))))
                };
                let param_freq = match percent_encoding::percent_decode(path_params["freq"].as_bytes()).decode_utf8() {
                    Ok(param_freq) => match param_freq.parse::<i32>() {
                        Ok(param_freq) => param_freq,
                        Err(e) => return Box::new(future::ok(Response::new().with_status(StatusCode::BadRequest).with_body(format!("Couldn't parse path parameter freq: {}", e)))),
                    },
                    Err(_) => return Box::new(future::ok(Response::new().with_status(StatusCode::BadRequest).with_body(format!("Couldn't percent-decode path parameter as UTF-8: {}", &path_params["freq"]))))
                };





                Box::new({
                        {{

                                Box::new(api_impl.set_freq(param_bus_id, param_addr, param_freq, &context)
                                    .then(move |result| {
                                        let mut response = Response::new();
                                        response.headers_mut().set(XSpanId((&context as &Has<XSpanIdString>).get().0.to_string()));

                                        match result {
                                            Ok(rsp) => match rsp {
                                                SetFreqResponse::OK


                                                => {
                                                    response.set_status(StatusCode::try_from(200).unwrap());

                                                },
                                                SetFreqResponse::BadRequest

                                                    (body)


                                                => {
                                                    response.set_status(StatusCode::try_from(400).unwrap());

                                                    response.headers_mut().set(ContentType(mimetypes::responses::SET_FREQ_BAD_REQUEST.clone()));


                                                    let body = serde_json::to_string(&body).expect("impossible to fail to serialize");

                                                    response.set_body(body);
                                                },
                                                SetFreqResponse::OperationFailed

                                                    (body)


                                                => {
                                                    response.set_status(StatusCode::try_from(502).unwrap());

                                                    response.headers_mut().set(ContentType(mimetypes::responses::SET_FREQ_OPERATION_FAILED.clone()));


                                                    let body = serde_json::to_string(&body).expect("impossible to fail to serialize");

                                                    response.set_body(body);
                                                },
                                            },
                                            Err(_) => {
                                                // Application code returned an error. This should not happen, as the implementation should
                                                // return a valid response.
                                                response.set_status(StatusCode::InternalServerError);
                                                response.set_body("An internal error occurred");
                                            },
                                        }

                                        future::ok(response)
                                    }
                                ))

                        }}
                }) as Box<Future<Item=Response, Error=Error>>


            },


            // SetGroup - POST /pca9956b/{busId}/{addr}/group/{group}
            &hyper::Method::Post if path.matched(paths::ID_PCA9956B_BUSID_ADDR_GROUP_GROUP) => {


                // Path parameters
                let path = uri.path().to_string();
                let path_params =
                    paths::REGEX_PCA9956B_BUSID_ADDR_GROUP_GROUP
                    .captures(&path)
                    .unwrap_or_else(||
                        panic!("Path {} matched RE PCA9956B_BUSID_ADDR_GROUP_GROUP in set but failed match against \"{}\"", path, paths::REGEX_PCA9956B_BUSID_ADDR_GROUP_GROUP.as_str())
                    );

                let param_bus_id = match percent_encoding::percent_decode(path_params["busId"].as_bytes()).decode_utf8() {
                    Ok(param_bus_id) => match param_bus_id.parse::<i32>() {
                        Ok(param_bus_id) => param_bus_id,
                        Err(e) => return Box::new(future::ok(Response::new().with_status(StatusCode::BadRequest).with_body(format!("Couldn't parse path parameter busId: {}", e)))),
                    },
                    Err(_) => return Box::new(future::ok(Response::new().with_status(StatusCode::BadRequest).with_body(format!("Couldn't percent-decode path parameter as UTF-8: {}", &path_params["busId"]))))
                };
                let param_addr = match percent_encoding::percent_decode(path_params["addr"].as_bytes()).decode_utf8() {
                    Ok(param_addr) => match param_addr.parse::<i32>() {
                        Ok(param_addr) => param_addr,
                        Err(e) => return Box::new(future::ok(Response::new().with_status(StatusCode::BadRequest).with_body(format!("Couldn't parse path parameter addr: {}", e)))),
                    },
                    Err(_) => return Box::new(future::ok(Response::new().with_status(StatusCode::BadRequest).with_body(format!("Couldn't percent-decode path parameter as UTF-8: {}", &path_params["addr"]))))
                };
                let param_group = match percent_encoding::percent_decode(path_params["group"].as_bytes()).decode_utf8() {
                    Ok(param_group) => match param_group.parse::<models::Group>() {
                        Ok(param_group) => param_group,
                        Err(e) => return Box::new(future::ok(Response::new().with_status(StatusCode::BadRequest).with_body(format!("Couldn't parse path parameter group: {}", e)))),
                    },
                    Err(_) => return Box::new(future::ok(Response::new().with_status(StatusCode::BadRequest).with_body(format!("Couldn't percent-decode path parameter as UTF-8: {}", &path_params["group"]))))
                };





                Box::new({
                        {{

                                Box::new(api_impl.set_group(param_bus_id, param_addr, param_group, &context)
                                    .then(move |result| {
                                        let mut response = Response::new();
                                        response.headers_mut().set(XSpanId((&context as &Has<XSpanIdString>).get().0.to_string()));

                                        match result {
                                            Ok(rsp) => match rsp {
                                                SetGroupResponse::OK


                                                => {
                                                    response.set_status(StatusCode::try_from(200).unwrap());

                                                },
                                                SetGroupResponse::BadRequest

                                                    (body)


                                                => {
                                                    response.set_status(StatusCode::try_from(400).unwrap());

                                                    response.headers_mut().set(ContentType(mimetypes::responses::SET_GROUP_BAD_REQUEST.clone()));


                                                    let body = serde_json::to_string(&body).expect("impossible to fail to serialize");

                                                    response.set_body(body);
                                                },
                                                SetGroupResponse::OperationFailed

                                                    (body)


                                                => {
                                                    response.set_status(StatusCode::try_from(502).unwrap());

                                                    response.headers_mut().set(ContentType(mimetypes::responses::SET_GROUP_OPERATION_FAILED.clone()));


                                                    let body = serde_json::to_string(&body).expect("impossible to fail to serialize");

                                                    response.set_body(body);
                                                },
                                            },
                                            Err(_) => {
                                                // Application code returned an error. This should not happen, as the implementation should
                                                // return a valid response.
                                                response.set_status(StatusCode::InternalServerError);
                                                response.set_body("An internal error occurred");
                                            },
                                        }

                                        future::ok(response)
                                    }
                                ))

                        }}
                }) as Box<Future<Item=Response, Error=Error>>


            },


            // SetLedCurrent - POST /pca9956b/{busId}/{addr}/led/{led}/current/{current}
            &hyper::Method::Post if path.matched(paths::ID_PCA9956B_BUSID_ADDR_LED_LED_CURRENT_CURRENT) => {


                // Path parameters
                let path = uri.path().to_string();
                let path_params =
                    paths::REGEX_PCA9956B_BUSID_ADDR_LED_LED_CURRENT_CURRENT
                    .captures(&path)
                    .unwrap_or_else(||
                        panic!("Path {} matched RE PCA9956B_BUSID_ADDR_LED_LED_CURRENT_CURRENT in set but failed match against \"{}\"", path, paths::REGEX_PCA9956B_BUSID_ADDR_LED_LED_CURRENT_CURRENT.as_str())
                    );

                let param_bus_id = match percent_encoding::percent_decode(path_params["busId"].as_bytes()).decode_utf8() {
                    Ok(param_bus_id) => match param_bus_id.parse::<i32>() {
                        Ok(param_bus_id) => param_bus_id,
                        Err(e) => return Box::new(future::ok(Response::new().with_status(StatusCode::BadRequest).with_body(format!("Couldn't parse path parameter busId: {}", e)))),
                    },
                    Err(_) => return Box::new(future::ok(Response::new().with_status(StatusCode::BadRequest).with_body(format!("Couldn't percent-decode path parameter as UTF-8: {}", &path_params["busId"]))))
                };
                let param_addr = match percent_encoding::percent_decode(path_params["addr"].as_bytes()).decode_utf8() {
                    Ok(param_addr) => match param_addr.parse::<i32>() {
                        Ok(param_addr) => param_addr,
                        Err(e) => return Box::new(future::ok(Response::new().with_status(StatusCode::BadRequest).with_body(format!("Couldn't parse path parameter addr: {}", e)))),
                    },
                    Err(_) => return Box::new(future::ok(Response::new().with_status(StatusCode::BadRequest).with_body(format!("Couldn't percent-decode path parameter as UTF-8: {}", &path_params["addr"]))))
                };
                let param_led = match percent_encoding::percent_decode(path_params["led"].as_bytes()).decode_utf8() {
                    Ok(param_led) => match param_led.parse::<i32>() {
                        Ok(param_led) => param_led,
                        Err(e) => return Box::new(future::ok(Response::new().with_status(StatusCode::BadRequest).with_body(format!("Couldn't parse path parameter led: {}", e)))),
                    },
                    Err(_) => return Box::new(future::ok(Response::new().with_status(StatusCode::BadRequest).with_body(format!("Couldn't percent-decode path parameter as UTF-8: {}", &path_params["led"]))))
                };
                let param_current = match percent_encoding::percent_decode(path_params["current"].as_bytes()).decode_utf8() {
                    Ok(param_current) => match param_current.parse::<i32>() {
                        Ok(param_current) => param_current,
                        Err(e) => return Box::new(future::ok(Response::new().with_status(StatusCode::BadRequest).with_body(format!("Couldn't parse path parameter current: {}", e)))),
                    },
                    Err(_) => return Box::new(future::ok(Response::new().with_status(StatusCode::BadRequest).with_body(format!("Couldn't percent-decode path parameter as UTF-8: {}", &path_params["current"]))))
                };





                Box::new({
                        {{

                                Box::new(api_impl.set_led_current(param_bus_id, param_addr, param_led, param_current, &context)
                                    .then(move |result| {
                                        let mut response = Response::new();
                                        response.headers_mut().set(XSpanId((&context as &Has<XSpanIdString>).get().0.to_string()));

                                        match result {
                                            Ok(rsp) => match rsp {
                                                SetLedCurrentResponse::OK


                                                => {
                                                    response.set_status(StatusCode::try_from(200).unwrap());

                                                },
                                                SetLedCurrentResponse::BadRequest

                                                    (body)


                                                => {
                                                    response.set_status(StatusCode::try_from(400).unwrap());

                                                    response.headers_mut().set(ContentType(mimetypes::responses::SET_LED_CURRENT_BAD_REQUEST.clone()));


                                                    let body = serde_json::to_string(&body).expect("impossible to fail to serialize");

                                                    response.set_body(body);
                                                },
                                                SetLedCurrentResponse::OperationFailed

                                                    (body)


                                                => {
                                                    response.set_status(StatusCode::try_from(502).unwrap());

                                                    response.headers_mut().set(ContentType(mimetypes::responses::SET_LED_CURRENT_OPERATION_FAILED.clone()));


                                                    let body = serde_json::to_string(&body).expect("impossible to fail to serialize");

                                                    response.set_body(body);
                                                },
                                            },
                                            Err(_) => {
                                                // Application code returned an error. This should not happen, as the implementation should
                                                // return a valid response.
                                                response.set_status(StatusCode::InternalServerError);
                                                response.set_body("An internal error occurred");
                                            },
                                        }

                                        future::ok(response)
                                    }
                                ))

                        }}
                }) as Box<Future<Item=Response, Error=Error>>


            },


            // SetLedError - POST /pca9956b/{busId}/{addr}/led/{led}/error/{error}
            &hyper::Method::Post if path.matched(paths::ID_PCA9956B_BUSID_ADDR_LED_LED_ERROR_ERROR) => {


                // Path parameters
                let path = uri.path().to_string();
                let path_params =
                    paths::REGEX_PCA9956B_BUSID_ADDR_LED_LED_ERROR_ERROR
                    .captures(&path)
                    .unwrap_or_else(||
                        panic!("Path {} matched RE PCA9956B_BUSID_ADDR_LED_LED_ERROR_ERROR in set but failed match against \"{}\"", path, paths::REGEX_PCA9956B_BUSID_ADDR_LED_LED_ERROR_ERROR.as_str())
                    );

                let param_bus_id = match percent_encoding::percent_decode(path_params["busId"].as_bytes()).decode_utf8() {
                    Ok(param_bus_id) => match param_bus_id.parse::<i32>() {
                        Ok(param_bus_id) => param_bus_id,
                        Err(e) => return Box::new(future::ok(Response::new().with_status(StatusCode::BadRequest).with_body(format!("Couldn't parse path parameter busId: {}", e)))),
                    },
                    Err(_) => return Box::new(future::ok(Response::new().with_status(StatusCode::BadRequest).with_body(format!("Couldn't percent-decode path parameter as UTF-8: {}", &path_params["busId"]))))
                };
                let param_addr = match percent_encoding::percent_decode(path_params["addr"].as_bytes()).decode_utf8() {
                    Ok(param_addr) => match param_addr.parse::<i32>() {
                        Ok(param_addr) => param_addr,
                        Err(e) => return Box::new(future::ok(Response::new().with_status(StatusCode::BadRequest).with_body(format!("Couldn't parse path parameter addr: {}", e)))),
                    },
                    Err(_) => return Box::new(future::ok(Response::new().with_status(StatusCode::BadRequest).with_body(format!("Couldn't percent-decode path parameter as UTF-8: {}", &path_params["addr"]))))
                };
                let param_led = match percent_encoding::percent_decode(path_params["led"].as_bytes()).decode_utf8() {
                    Ok(param_led) => match param_led.parse::<i32>() {
                        Ok(param_led) => param_led,
                        Err(e) => return Box::new(future::ok(Response::new().with_status(StatusCode::BadRequest).with_body(format!("Couldn't parse path parameter led: {}", e)))),
                    },
                    Err(_) => return Box::new(future::ok(Response::new().with_status(StatusCode::BadRequest).with_body(format!("Couldn't percent-decode path parameter as UTF-8: {}", &path_params["led"]))))
                };
                let param_error = match percent_encoding::percent_decode(path_params["error"].as_bytes()).decode_utf8() {
                    Ok(param_error) => match param_error.parse::<models::LedError>() {
                        Ok(param_error) => param_error,
                        Err(e) => return Box::new(future::ok(Response::new().with_status(StatusCode::BadRequest).with_body(format!("Couldn't parse path parameter error: {}", e)))),
                    },
                    Err(_) => return Box::new(future::ok(Response::new().with_status(StatusCode::BadRequest).with_body(format!("Couldn't percent-decode path parameter as UTF-8: {}", &path_params["error"]))))
                };





                Box::new({
                        {{

                                Box::new(api_impl.set_led_error(param_bus_id, param_addr, param_led, param_error, &context)
                                    .then(move |result| {
                                        let mut response = Response::new();
                                        response.headers_mut().set(XSpanId((&context as &Has<XSpanIdString>).get().0.to_string()));

                                        match result {
                                            Ok(rsp) => match rsp {
                                                SetLedErrorResponse::OK


                                                => {
                                                    response.set_status(StatusCode::try_from(200).unwrap());

                                                },
                                                SetLedErrorResponse::BadRequest

                                                    (body)


                                                => {
                                                    response.set_status(StatusCode::try_from(400).unwrap());

                                                    response.headers_mut().set(ContentType(mimetypes::responses::SET_LED_ERROR_BAD_REQUEST.clone()));


                                                    let body = serde_json::to_string(&body).expect("impossible to fail to serialize");

                                                    response.set_body(body);
                                                },
                                                SetLedErrorResponse::OperationFailed

                                                    (body)


                                                => {
                                                    response.set_status(StatusCode::try_from(502).unwrap());

                                                    response.headers_mut().set(ContentType(mimetypes::responses::SET_LED_ERROR_OPERATION_FAILED.clone()));


                                                    let body = serde_json::to_string(&body).expect("impossible to fail to serialize");

                                                    response.set_body(body);
                                                },
                                            },
                                            Err(_) => {
                                                // Application code returned an error. This should not happen, as the implementation should
                                                // return a valid response.
                                                response.set_status(StatusCode::InternalServerError);
                                                response.set_body("An internal error occurred");
                                            },
                                        }

                                        future::ok(response)
                                    }
                                ))

                        }}
                }) as Box<Future<Item=Response, Error=Error>>


            },


            // SetLedInfo - POST /pca9956b/{busId}/{addr}/led/{led}
            &hyper::Method::Post if path.matched(paths::ID_PCA9956B_BUSID_ADDR_LED_LED) => {


                // Path parameters
                let path = uri.path().to_string();
                let path_params =
                    paths::REGEX_PCA9956B_BUSID_ADDR_LED_LED
                    .captures(&path)
                    .unwrap_or_else(||
                        panic!("Path {} matched RE PCA9956B_BUSID_ADDR_LED_LED in set but failed match against \"{}\"", path, paths::REGEX_PCA9956B_BUSID_ADDR_LED_LED.as_str())
                    );

                let param_bus_id = match percent_encoding::percent_decode(path_params["busId"].as_bytes()).decode_utf8() {
                    Ok(param_bus_id) => match param_bus_id.parse::<i32>() {
                        Ok(param_bus_id) => param_bus_id,
                        Err(e) => return Box::new(future::ok(Response::new().with_status(StatusCode::BadRequest).with_body(format!("Couldn't parse path parameter busId: {}", e)))),
                    },
                    Err(_) => return Box::new(future::ok(Response::new().with_status(StatusCode::BadRequest).with_body(format!("Couldn't percent-decode path parameter as UTF-8: {}", &path_params["busId"]))))
                };
                let param_addr = match percent_encoding::percent_decode(path_params["addr"].as_bytes()).decode_utf8() {
                    Ok(param_addr) => match param_addr.parse::<i32>() {
                        Ok(param_addr) => param_addr,
                        Err(e) => return Box::new(future::ok(Response::new().with_status(StatusCode::BadRequest).with_body(format!("Couldn't parse path parameter addr: {}", e)))),
                    },
                    Err(_) => return Box::new(future::ok(Response::new().with_status(StatusCode::BadRequest).with_body(format!("Couldn't percent-decode path parameter as UTF-8: {}", &path_params["addr"]))))
                };
                let param_led = match percent_encoding::percent_decode(path_params["led"].as_bytes()).decode_utf8() {
                    Ok(param_led) => match param_led.parse::<i32>() {
                        Ok(param_led) => param_led,
                        Err(e) => return Box::new(future::ok(Response::new().with_status(StatusCode::BadRequest).with_body(format!("Couldn't parse path parameter led: {}", e)))),
                    },
                    Err(_) => return Box::new(future::ok(Response::new().with_status(StatusCode::BadRequest).with_body(format!("Couldn't percent-decode path parameter as UTF-8: {}", &path_params["led"]))))
                };




                // Body parameters (note that non-required body parameters will ignore garbage
                // values, rather than causing a 400 response). Produce warning header and logs for
                // any unused fields.
                Box::new(body.concat2()
                    .then(move |result| -> Box<Future<Item=Response, Error=Error>> {
                        match result {
                            Ok(body) => {

                                let mut unused_elements = Vec::new();
                                let param_led_info: Option<models::LedInfo> = if !body.is_empty() {

                                    let deserializer = &mut serde_json::Deserializer::from_slice(&*body);

                                    match serde_ignored::deserialize(deserializer, |path| {
                                            warn!("Ignoring unknown field in body: {}", path);
                                            unused_elements.push(path.to_string());
                                    }) {
                                        Ok(param_led_info) => param_led_info,
                                        Err(e) => return Box::new(future::ok(Response::new().with_status(StatusCode::BadRequest).with_body(format!("Couldn't parse body parameter LedInfo - doesn't match schema: {}", e)))),
                                    }

                                } else {
                                    None
                                };
                                let param_led_info = match param_led_info {
                                    Some(param_led_info) => param_led_info,
                                    None => return Box::new(future::ok(Response::new().with_status(StatusCode::BadRequest).with_body("Missing required body parameter LedInfo"))),
                                };


                                Box::new(api_impl.set_led_info(param_bus_id, param_addr, param_led, param_led_info, &context)
                                    .then(move |result| {
                                        let mut response = Response::new();
                                        response.headers_mut().set(XSpanId((&context as &Has<XSpanIdString>).get().0.to_string()));

                                        if !unused_elements.is_empty() {
                                            response.headers_mut().set(Warning(format!("Ignoring unknown fields in body: {:?}", unused_elements)));
                                        }

                                        match result {
                                            Ok(rsp) => match rsp {
                                                SetLedInfoResponse::OK


                                                => {
                                                    response.set_status(StatusCode::try_from(200).unwrap());

                                                },
                                                SetLedInfoResponse::BadRequest

                                                    (body)


                                                => {
                                                    response.set_status(StatusCode::try_from(400).unwrap());

                                                    response.headers_mut().set(ContentType(mimetypes::responses::SET_LED_INFO_BAD_REQUEST.clone()));


                                                    let body = serde_json::to_string(&body).expect("impossible to fail to serialize");

                                                    response.set_body(body);
                                                },
                                                SetLedInfoResponse::OperationFailed

                                                    (body)


                                                => {
                                                    response.set_status(StatusCode::try_from(502).unwrap());

                                                    response.headers_mut().set(ContentType(mimetypes::responses::SET_LED_INFO_OPERATION_FAILED.clone()));


                                                    let body = serde_json::to_string(&body).expect("impossible to fail to serialize");

                                                    response.set_body(body);
                                                },
                                            },
                                            Err(_) => {
                                                // Application code returned an error. This should not happen, as the implementation should
                                                // return a valid response.
                                                response.set_status(StatusCode::InternalServerError);
                                                response.set_body("An internal error occurred");
                                            },
                                        }

                                        future::ok(response)
                                    }
                                ))


                            },
                            Err(e) => Box::new(future::ok(Response::new().with_status(StatusCode::BadRequest).with_body(format!("Couldn't read body parameter LedInfo: {}", e)))),
                        }
                    })
                ) as Box<Future<Item=Response, Error=Error>>

            },


            // SetLedInfoAll - POST /pca9956b/{busId}/{addr}/led
            &hyper::Method::Post if path.matched(paths::ID_PCA9956B_BUSID_ADDR_LED) => {


                // Path parameters
                let path = uri.path().to_string();
                let path_params =
                    paths::REGEX_PCA9956B_BUSID_ADDR_LED
                    .captures(&path)
                    .unwrap_or_else(||
                        panic!("Path {} matched RE PCA9956B_BUSID_ADDR_LED in set but failed match against \"{}\"", path, paths::REGEX_PCA9956B_BUSID_ADDR_LED.as_str())
                    );

                let param_bus_id = match percent_encoding::percent_decode(path_params["busId"].as_bytes()).decode_utf8() {
                    Ok(param_bus_id) => match param_bus_id.parse::<i32>() {
                        Ok(param_bus_id) => param_bus_id,
                        Err(e) => return Box::new(future::ok(Response::new().with_status(StatusCode::BadRequest).with_body(format!("Couldn't parse path parameter busId: {}", e)))),
                    },
                    Err(_) => return Box::new(future::ok(Response::new().with_status(StatusCode::BadRequest).with_body(format!("Couldn't percent-decode path parameter as UTF-8: {}", &path_params["busId"]))))
                };
                let param_addr = match percent_encoding::percent_decode(path_params["addr"].as_bytes()).decode_utf8() {
                    Ok(param_addr) => match param_addr.parse::<i32>() {
                        Ok(param_addr) => param_addr,
                        Err(e) => return Box::new(future::ok(Response::new().with_status(StatusCode::BadRequest).with_body(format!("Couldn't parse path parameter addr: {}", e)))),
                    },
                    Err(_) => return Box::new(future::ok(Response::new().with_status(StatusCode::BadRequest).with_body(format!("Couldn't percent-decode path parameter as UTF-8: {}", &path_params["addr"]))))
                };




                // Body parameters (note that non-required body parameters will ignore garbage
                // values, rather than causing a 400 response). Produce warning header and logs for
                // any unused fields.
                Box::new(body.concat2()
                    .then(move |result| -> Box<Future<Item=Response, Error=Error>> {
                        match result {
                            Ok(body) => {

                                let mut unused_elements = Vec::new();
                                let param_led_info: Option<Vec<models::LedInfo>> = if !body.is_empty() {

                                    let deserializer = &mut serde_json::Deserializer::from_slice(&*body);

                                    match serde_ignored::deserialize(deserializer, |path| {
                                            warn!("Ignoring unknown field in body: {}", path);
                                            unused_elements.push(path.to_string());
                                    }) {
                                        Ok(param_led_info) => param_led_info,
                                        Err(e) => return Box::new(future::ok(Response::new().with_status(StatusCode::BadRequest).with_body(format!("Couldn't parse body parameter ledInfo - doesn't match schema: {}", e)))),
                                    }

                                } else {
                                    None
                                };
                                let param_led_info = match param_led_info {
                                    Some(param_led_info) => param_led_info,
                                    None => return Box::new(future::ok(Response::new().with_status(StatusCode::BadRequest).with_body("Missing required body parameter ledInfo"))),
                                };


                                Box::new(api_impl.set_led_info_all(param_bus_id, param_addr, param_led_info.as_ref(), &context)
                                    .then(move |result| {
                                        let mut response = Response::new();
                                        response.headers_mut().set(XSpanId((&context as &Has<XSpanIdString>).get().0.to_string()));

                                        if !unused_elements.is_empty() {
                                            response.headers_mut().set(Warning(format!("Ignoring unknown fields in body: {:?}", unused_elements)));
                                        }

                                        match result {
                                            Ok(rsp) => match rsp {
                                                SetLedInfoAllResponse::OK


                                                => {
                                                    response.set_status(StatusCode::try_from(200).unwrap());

                                                },
                                                SetLedInfoAllResponse::BadRequest

                                                    (body)


                                                => {
                                                    response.set_status(StatusCode::try_from(400).unwrap());

                                                    response.headers_mut().set(ContentType(mimetypes::responses::SET_LED_INFO_ALL_BAD_REQUEST.clone()));


                                                    let body = serde_json::to_string(&body).expect("impossible to fail to serialize");

                                                    response.set_body(body);
                                                },
                                                SetLedInfoAllResponse::OperationFailed

                                                    (body)


                                                => {
                                                    response.set_status(StatusCode::try_from(502).unwrap());

                                                    response.headers_mut().set(ContentType(mimetypes::responses::SET_LED_INFO_ALL_OPERATION_FAILED.clone()));


                                                    let body = serde_json::to_string(&body).expect("impossible to fail to serialize");

                                                    response.set_body(body);
                                                },
                                            },
                                            Err(_) => {
                                                // Application code returned an error. This should not happen, as the implementation should
                                                // return a valid response.
                                                response.set_status(StatusCode::InternalServerError);
                                                response.set_body("An internal error occurred");
                                            },
                                        }

                                        future::ok(response)
                                    }
                                ))


                            },
                            Err(e) => Box::new(future::ok(Response::new().with_status(StatusCode::BadRequest).with_body(format!("Couldn't read body parameter ledInfo: {}", e)))),
                        }
                    })
                ) as Box<Future<Item=Response, Error=Error>>

            },


            // SetLedPwm - POST /pca9956b/{busId}/{addr}/led/{led}/pwm/{pwm}
            &hyper::Method::Post if path.matched(paths::ID_PCA9956B_BUSID_ADDR_LED_LED_PWM_PWM) => {


                // Path parameters
                let path = uri.path().to_string();
                let path_params =
                    paths::REGEX_PCA9956B_BUSID_ADDR_LED_LED_PWM_PWM
                    .captures(&path)
                    .unwrap_or_else(||
                        panic!("Path {} matched RE PCA9956B_BUSID_ADDR_LED_LED_PWM_PWM in set but failed match against \"{}\"", path, paths::REGEX_PCA9956B_BUSID_ADDR_LED_LED_PWM_PWM.as_str())
                    );

                let param_bus_id = match percent_encoding::percent_decode(path_params["busId"].as_bytes()).decode_utf8() {
                    Ok(param_bus_id) => match param_bus_id.parse::<i32>() {
                        Ok(param_bus_id) => param_bus_id,
                        Err(e) => return Box::new(future::ok(Response::new().with_status(StatusCode::BadRequest).with_body(format!("Couldn't parse path parameter busId: {}", e)))),
                    },
                    Err(_) => return Box::new(future::ok(Response::new().with_status(StatusCode::BadRequest).with_body(format!("Couldn't percent-decode path parameter as UTF-8: {}", &path_params["busId"]))))
                };
                let param_addr = match percent_encoding::percent_decode(path_params["addr"].as_bytes()).decode_utf8() {
                    Ok(param_addr) => match param_addr.parse::<i32>() {
                        Ok(param_addr) => param_addr,
                        Err(e) => return Box::new(future::ok(Response::new().with_status(StatusCode::BadRequest).with_body(format!("Couldn't parse path parameter addr: {}", e)))),
                    },
                    Err(_) => return Box::new(future::ok(Response::new().with_status(StatusCode::BadRequest).with_body(format!("Couldn't percent-decode path parameter as UTF-8: {}", &path_params["addr"]))))
                };
                let param_led = match percent_encoding::percent_decode(path_params["led"].as_bytes()).decode_utf8() {
                    Ok(param_led) => match param_led.parse::<i32>() {
                        Ok(param_led) => param_led,
                        Err(e) => return Box::new(future::ok(Response::new().with_status(StatusCode::BadRequest).with_body(format!("Couldn't parse path parameter led: {}", e)))),
                    },
                    Err(_) => return Box::new(future::ok(Response::new().with_status(StatusCode::BadRequest).with_body(format!("Couldn't percent-decode path parameter as UTF-8: {}", &path_params["led"]))))
                };
                let param_pwm = match percent_encoding::percent_decode(path_params["pwm"].as_bytes()).decode_utf8() {
                    Ok(param_pwm) => match param_pwm.parse::<i32>() {
                        Ok(param_pwm) => param_pwm,
                        Err(e) => return Box::new(future::ok(Response::new().with_status(StatusCode::BadRequest).with_body(format!("Couldn't parse path parameter pwm: {}", e)))),
                    },
                    Err(_) => return Box::new(future::ok(Response::new().with_status(StatusCode::BadRequest).with_body(format!("Couldn't percent-decode path parameter as UTF-8: {}", &path_params["pwm"]))))
                };





                Box::new({
                        {{

                                Box::new(api_impl.set_led_pwm(param_bus_id, param_addr, param_led, param_pwm, &context)
                                    .then(move |result| {
                                        let mut response = Response::new();
                                        response.headers_mut().set(XSpanId((&context as &Has<XSpanIdString>).get().0.to_string()));

                                        match result {
                                            Ok(rsp) => match rsp {
                                                SetLedPwmResponse::OK


                                                => {
                                                    response.set_status(StatusCode::try_from(200).unwrap());

                                                },
                                                SetLedPwmResponse::BadRequest

                                                    (body)


                                                => {
                                                    response.set_status(StatusCode::try_from(400).unwrap());

                                                    response.headers_mut().set(ContentType(mimetypes::responses::SET_LED_PWM_BAD_REQUEST.clone()));


                                                    let body = serde_json::to_string(&body).expect("impossible to fail to serialize");

                                                    response.set_body(body);
                                                },
                                                SetLedPwmResponse::OperationFailed

                                                    (body)


                                                => {
                                                    response.set_status(StatusCode::try_from(502).unwrap());

                                                    response.headers_mut().set(ContentType(mimetypes::responses::SET_LED_PWM_OPERATION_FAILED.clone()));


                                                    let body = serde_json::to_string(&body).expect("impossible to fail to serialize");

                                                    response.set_body(body);
                                                },
                                            },
                                            Err(_) => {
                                                // Application code returned an error. This should not happen, as the implementation should
                                                // return a valid response.
                                                response.set_status(StatusCode::InternalServerError);
                                                response.set_body("An internal error occurred");
                                            },
                                        }

                                        future::ok(response)
                                    }
                                ))

                        }}
                }) as Box<Future<Item=Response, Error=Error>>


            },


            // SetLedState - POST /pca9956b/{busId}/{addr}/led/{led}/state/{state}
            &hyper::Method::Post if path.matched(paths::ID_PCA9956B_BUSID_ADDR_LED_LED_STATE_STATE) => {


                // Path parameters
                let path = uri.path().to_string();
                let path_params =
                    paths::REGEX_PCA9956B_BUSID_ADDR_LED_LED_STATE_STATE
                    .captures(&path)
                    .unwrap_or_else(||
                        panic!("Path {} matched RE PCA9956B_BUSID_ADDR_LED_LED_STATE_STATE in set but failed match against \"{}\"", path, paths::REGEX_PCA9956B_BUSID_ADDR_LED_LED_STATE_STATE.as_str())
                    );

                let param_bus_id = match percent_encoding::percent_decode(path_params["busId"].as_bytes()).decode_utf8() {
                    Ok(param_bus_id) => match param_bus_id.parse::<i32>() {
                        Ok(param_bus_id) => param_bus_id,
                        Err(e) => return Box::new(future::ok(Response::new().with_status(StatusCode::BadRequest).with_body(format!("Couldn't parse path parameter busId: {}", e)))),
                    },
                    Err(_) => return Box::new(future::ok(Response::new().with_status(StatusCode::BadRequest).with_body(format!("Couldn't percent-decode path parameter as UTF-8: {}", &path_params["busId"]))))
                };
                let param_addr = match percent_encoding::percent_decode(path_params["addr"].as_bytes()).decode_utf8() {
                    Ok(param_addr) => match param_addr.parse::<i32>() {
                        Ok(param_addr) => param_addr,
                        Err(e) => return Box::new(future::ok(Response::new().with_status(StatusCode::BadRequest).with_body(format!("Couldn't parse path parameter addr: {}", e)))),
                    },
                    Err(_) => return Box::new(future::ok(Response::new().with_status(StatusCode::BadRequest).with_body(format!("Couldn't percent-decode path parameter as UTF-8: {}", &path_params["addr"]))))
                };
                let param_led = match percent_encoding::percent_decode(path_params["led"].as_bytes()).decode_utf8() {
                    Ok(param_led) => match param_led.parse::<i32>() {
                        Ok(param_led) => param_led,
                        Err(e) => return Box::new(future::ok(Response::new().with_status(StatusCode::BadRequest).with_body(format!("Couldn't parse path parameter led: {}", e)))),
                    },
                    Err(_) => return Box::new(future::ok(Response::new().with_status(StatusCode::BadRequest).with_body(format!("Couldn't percent-decode path parameter as UTF-8: {}", &path_params["led"]))))
                };
                let param_state = match percent_encoding::percent_decode(path_params["state"].as_bytes()).decode_utf8() {
                    Ok(param_state) => match param_state.parse::<models::LedState>() {
                        Ok(param_state) => param_state,
                        Err(e) => return Box::new(future::ok(Response::new().with_status(StatusCode::BadRequest).with_body(format!("Couldn't parse path parameter state: {}", e)))),
                    },
                    Err(_) => return Box::new(future::ok(Response::new().with_status(StatusCode::BadRequest).with_body(format!("Couldn't percent-decode path parameter as UTF-8: {}", &path_params["state"]))))
                };





                Box::new({
                        {{

                                Box::new(api_impl.set_led_state(param_bus_id, param_addr, param_led, param_state, &context)
                                    .then(move |result| {
                                        let mut response = Response::new();
                                        response.headers_mut().set(XSpanId((&context as &Has<XSpanIdString>).get().0.to_string()));

                                        match result {
                                            Ok(rsp) => match rsp {
                                                SetLedStateResponse::OK


                                                => {
                                                    response.set_status(StatusCode::try_from(200).unwrap());

                                                },
                                                SetLedStateResponse::BadRequest

                                                    (body)


                                                => {
                                                    response.set_status(StatusCode::try_from(400).unwrap());

                                                    response.headers_mut().set(ContentType(mimetypes::responses::SET_LED_STATE_BAD_REQUEST.clone()));


                                                    let body = serde_json::to_string(&body).expect("impossible to fail to serialize");

                                                    response.set_body(body);
                                                },
                                                SetLedStateResponse::OperationFailed

                                                    (body)


                                                => {
                                                    response.set_status(StatusCode::try_from(502).unwrap());

                                                    response.headers_mut().set(ContentType(mimetypes::responses::SET_LED_STATE_OPERATION_FAILED.clone()));


                                                    let body = serde_json::to_string(&body).expect("impossible to fail to serialize");

                                                    response.set_body(body);
                                                },
                                            },
                                            Err(_) => {
                                                // Application code returned an error. This should not happen, as the implementation should
                                                // return a valid response.
                                                response.set_status(StatusCode::InternalServerError);
                                                response.set_body("An internal error occurred");
                                            },
                                        }

                                        future::ok(response)
                                    }
                                ))

                        }}
                }) as Box<Future<Item=Response, Error=Error>>


            },


            // SetOffset - POST /pca9956b/{busId}/{addr}/offset/{offset}
            &hyper::Method::Post if path.matched(paths::ID_PCA9956B_BUSID_ADDR_OFFSET_OFFSET) => {


                // Path parameters
                let path = uri.path().to_string();
                let path_params =
                    paths::REGEX_PCA9956B_BUSID_ADDR_OFFSET_OFFSET
                    .captures(&path)
                    .unwrap_or_else(||
                        panic!("Path {} matched RE PCA9956B_BUSID_ADDR_OFFSET_OFFSET in set but failed match against \"{}\"", path, paths::REGEX_PCA9956B_BUSID_ADDR_OFFSET_OFFSET.as_str())
                    );

                let param_bus_id = match percent_encoding::percent_decode(path_params["busId"].as_bytes()).decode_utf8() {
                    Ok(param_bus_id) => match param_bus_id.parse::<i32>() {
                        Ok(param_bus_id) => param_bus_id,
                        Err(e) => return Box::new(future::ok(Response::new().with_status(StatusCode::BadRequest).with_body(format!("Couldn't parse path parameter busId: {}", e)))),
                    },
                    Err(_) => return Box::new(future::ok(Response::new().with_status(StatusCode::BadRequest).with_body(format!("Couldn't percent-decode path parameter as UTF-8: {}", &path_params["busId"]))))
                };
                let param_addr = match percent_encoding::percent_decode(path_params["addr"].as_bytes()).decode_utf8() {
                    Ok(param_addr) => match param_addr.parse::<i32>() {
                        Ok(param_addr) => param_addr,
                        Err(e) => return Box::new(future::ok(Response::new().with_status(StatusCode::BadRequest).with_body(format!("Couldn't parse path parameter addr: {}", e)))),
                    },
                    Err(_) => return Box::new(future::ok(Response::new().with_status(StatusCode::BadRequest).with_body(format!("Couldn't percent-decode path parameter as UTF-8: {}", &path_params["addr"]))))
                };
                let param_offset = match percent_encoding::percent_decode(path_params["offset"].as_bytes()).decode_utf8() {
                    Ok(param_offset) => match param_offset.parse::<i32>() {
                        Ok(param_offset) => param_offset,
                        Err(e) => return Box::new(future::ok(Response::new().with_status(StatusCode::BadRequest).with_body(format!("Couldn't parse path parameter offset: {}", e)))),
                    },
                    Err(_) => return Box::new(future::ok(Response::new().with_status(StatusCode::BadRequest).with_body(format!("Couldn't percent-decode path parameter as UTF-8: {}", &path_params["offset"]))))
                };





                Box::new({
                        {{

                                Box::new(api_impl.set_offset(param_bus_id, param_addr, param_offset, &context)
                                    .then(move |result| {
                                        let mut response = Response::new();
                                        response.headers_mut().set(XSpanId((&context as &Has<XSpanIdString>).get().0.to_string()));

                                        match result {
                                            Ok(rsp) => match rsp {
                                                SetOffsetResponse::OK


                                                => {
                                                    response.set_status(StatusCode::try_from(200).unwrap());

                                                },
                                                SetOffsetResponse::BadRequest

                                                    (body)


                                                => {
                                                    response.set_status(StatusCode::try_from(400).unwrap());

                                                    response.headers_mut().set(ContentType(mimetypes::responses::SET_OFFSET_BAD_REQUEST.clone()));


                                                    let body = serde_json::to_string(&body).expect("impossible to fail to serialize");

                                                    response.set_body(body);
                                                },
                                                SetOffsetResponse::OperationFailed

                                                    (body)


                                                => {
                                                    response.set_status(StatusCode::try_from(502).unwrap());

                                                    response.headers_mut().set(ContentType(mimetypes::responses::SET_OFFSET_OPERATION_FAILED.clone()));


                                                    let body = serde_json::to_string(&body).expect("impossible to fail to serialize");

                                                    response.set_body(body);
                                                },
                                            },
                                            Err(_) => {
                                                // Application code returned an error. This should not happen, as the implementation should
                                                // return a valid response.
                                                response.set_status(StatusCode::InternalServerError);
                                                response.set_body("An internal error occurred");
                                            },
                                        }

                                        future::ok(response)
                                    }
                                ))

                        }}
                }) as Box<Future<Item=Response, Error=Error>>


            },


            // SetOutputChange - POST /pca9956b/{busId}/{addr}/outputChange/{outputChange}
            &hyper::Method::Post if path.matched(paths::ID_PCA9956B_BUSID_ADDR_OUTPUTCHANGE_OUTPUTCHANGE) => {


                // Path parameters
                let path = uri.path().to_string();
                let path_params =
                    paths::REGEX_PCA9956B_BUSID_ADDR_OUTPUTCHANGE_OUTPUTCHANGE
                    .captures(&path)
                    .unwrap_or_else(||
                        panic!("Path {} matched RE PCA9956B_BUSID_ADDR_OUTPUTCHANGE_OUTPUTCHANGE in set but failed match against \"{}\"", path, paths::REGEX_PCA9956B_BUSID_ADDR_OUTPUTCHANGE_OUTPUTCHANGE.as_str())
                    );

                let param_bus_id = match percent_encoding::percent_decode(path_params["busId"].as_bytes()).decode_utf8() {
                    Ok(param_bus_id) => match param_bus_id.parse::<i32>() {
                        Ok(param_bus_id) => param_bus_id,
                        Err(e) => return Box::new(future::ok(Response::new().with_status(StatusCode::BadRequest).with_body(format!("Couldn't parse path parameter busId: {}", e)))),
                    },
                    Err(_) => return Box::new(future::ok(Response::new().with_status(StatusCode::BadRequest).with_body(format!("Couldn't percent-decode path parameter as UTF-8: {}", &path_params["busId"]))))
                };
                let param_addr = match percent_encoding::percent_decode(path_params["addr"].as_bytes()).decode_utf8() {
                    Ok(param_addr) => match param_addr.parse::<i32>() {
                        Ok(param_addr) => param_addr,
                        Err(e) => return Box::new(future::ok(Response::new().with_status(StatusCode::BadRequest).with_body(format!("Couldn't parse path parameter addr: {}", e)))),
                    },
                    Err(_) => return Box::new(future::ok(Response::new().with_status(StatusCode::BadRequest).with_body(format!("Couldn't percent-decode path parameter as UTF-8: {}", &path_params["addr"]))))
                };
                let param_output_change = match percent_encoding::percent_decode(path_params["outputChange"].as_bytes()).decode_utf8() {
                    Ok(param_output_change) => match param_output_change.parse::<models::OutputChange>() {
                        Ok(param_output_change) => param_output_change,
                        Err(e) => return Box::new(future::ok(Response::new().with_status(StatusCode::BadRequest).with_body(format!("Couldn't parse path parameter outputChange: {}", e)))),
                    },
                    Err(_) => return Box::new(future::ok(Response::new().with_status(StatusCode::BadRequest).with_body(format!("Couldn't percent-decode path parameter as UTF-8: {}", &path_params["outputChange"]))))
                };





                Box::new({
                        {{

                                Box::new(api_impl.set_output_change(param_bus_id, param_addr, param_output_change, &context)
                                    .then(move |result| {
                                        let mut response = Response::new();
                                        response.headers_mut().set(XSpanId((&context as &Has<XSpanIdString>).get().0.to_string()));

                                        match result {
                                            Ok(rsp) => match rsp {
                                                SetOutputChangeResponse::OK


                                                => {
                                                    response.set_status(StatusCode::try_from(200).unwrap());

                                                },
                                                SetOutputChangeResponse::BadRequest

                                                    (body)


                                                => {
                                                    response.set_status(StatusCode::try_from(400).unwrap());

                                                    response.headers_mut().set(ContentType(mimetypes::responses::SET_OUTPUT_CHANGE_BAD_REQUEST.clone()));


                                                    let body = serde_json::to_string(&body).expect("impossible to fail to serialize");

                                                    response.set_body(body);
                                                },
                                                SetOutputChangeResponse::OperationFailed

                                                    (body)


                                                => {
                                                    response.set_status(StatusCode::try_from(502).unwrap());

                                                    response.headers_mut().set(ContentType(mimetypes::responses::SET_OUTPUT_CHANGE_OPERATION_FAILED.clone()));


                                                    let body = serde_json::to_string(&body).expect("impossible to fail to serialize");

                                                    response.set_body(body);
                                                },
                                            },
                                            Err(_) => {
                                                // Application code returned an error. This should not happen, as the implementation should
                                                // return a valid response.
                                                response.set_status(StatusCode::InternalServerError);
                                                response.set_body("An internal error occurred");
                                            },
                                        }

                                        future::ok(response)
                                    }
                                ))

                        }}
                }) as Box<Future<Item=Response, Error=Error>>


            },


            // SetPwm - POST /pca9956b/{busId}/{addr}/pwm/{pwm}
            &hyper::Method::Post if path.matched(paths::ID_PCA9956B_BUSID_ADDR_PWM_PWM) => {


                // Path parameters
                let path = uri.path().to_string();
                let path_params =
                    paths::REGEX_PCA9956B_BUSID_ADDR_PWM_PWM
                    .captures(&path)
                    .unwrap_or_else(||
                        panic!("Path {} matched RE PCA9956B_BUSID_ADDR_PWM_PWM in set but failed match against \"{}\"", path, paths::REGEX_PCA9956B_BUSID_ADDR_PWM_PWM.as_str())
                    );

                let param_bus_id = match percent_encoding::percent_decode(path_params["busId"].as_bytes()).decode_utf8() {
                    Ok(param_bus_id) => match param_bus_id.parse::<i32>() {
                        Ok(param_bus_id) => param_bus_id,
                        Err(e) => return Box::new(future::ok(Response::new().with_status(StatusCode::BadRequest).with_body(format!("Couldn't parse path parameter busId: {}", e)))),
                    },
                    Err(_) => return Box::new(future::ok(Response::new().with_status(StatusCode::BadRequest).with_body(format!("Couldn't percent-decode path parameter as UTF-8: {}", &path_params["busId"]))))
                };
                let param_addr = match percent_encoding::percent_decode(path_params["addr"].as_bytes()).decode_utf8() {
                    Ok(param_addr) => match param_addr.parse::<i32>() {
                        Ok(param_addr) => param_addr,
                        Err(e) => return Box::new(future::ok(Response::new().with_status(StatusCode::BadRequest).with_body(format!("Couldn't parse path parameter addr: {}", e)))),
                    },
                    Err(_) => return Box::new(future::ok(Response::new().with_status(StatusCode::BadRequest).with_body(format!("Couldn't percent-decode path parameter as UTF-8: {}", &path_params["addr"]))))
                };
                let param_pwm = match percent_encoding::percent_decode(path_params["pwm"].as_bytes()).decode_utf8() {
                    Ok(param_pwm) => match param_pwm.parse::<i32>() {
                        Ok(param_pwm) => param_pwm,
                        Err(e) => return Box::new(future::ok(Response::new().with_status(StatusCode::BadRequest).with_body(format!("Couldn't parse path parameter pwm: {}", e)))),
                    },
                    Err(_) => return Box::new(future::ok(Response::new().with_status(StatusCode::BadRequest).with_body(format!("Couldn't percent-decode path parameter as UTF-8: {}", &path_params["pwm"]))))
                };





                Box::new({
                        {{

                                Box::new(api_impl.set_pwm(param_bus_id, param_addr, param_pwm, &context)
                                    .then(move |result| {
                                        let mut response = Response::new();
                                        response.headers_mut().set(XSpanId((&context as &Has<XSpanIdString>).get().0.to_string()));

                                        match result {
                                            Ok(rsp) => match rsp {
                                                SetPwmResponse::OK


                                                => {
                                                    response.set_status(StatusCode::try_from(200).unwrap());

                                                },
                                                SetPwmResponse::BadRequest

                                                    (body)


                                                => {
                                                    response.set_status(StatusCode::try_from(400).unwrap());

                                                    response.headers_mut().set(ContentType(mimetypes::responses::SET_PWM_BAD_REQUEST.clone()));


                                                    let body = serde_json::to_string(&body).expect("impossible to fail to serialize");

                                                    response.set_body(body);
                                                },
                                                SetPwmResponse::OperationFailed

                                                    (body)


                                                => {
                                                    response.set_status(StatusCode::try_from(502).unwrap());

                                                    response.headers_mut().set(ContentType(mimetypes::responses::SET_PWM_OPERATION_FAILED.clone()));


                                                    let body = serde_json::to_string(&body).expect("impossible to fail to serialize");

                                                    response.set_body(body);
                                                },
                                            },
                                            Err(_) => {
                                                // Application code returned an error. This should not happen, as the implementation should
                                                // return a valid response.
                                                response.set_status(StatusCode::InternalServerError);
                                                response.set_body("An internal error occurred");
                                            },
                                        }

                                        future::ok(response)
                                    }
                                ))

                        }}
                }) as Box<Future<Item=Response, Error=Error>>


            },


            // SetSleep - POST /pca9956b/{busId}/{addr}/sleep/{sleep}
            &hyper::Method::Post if path.matched(paths::ID_PCA9956B_BUSID_ADDR_SLEEP_SLEEP) => {


                // Path parameters
                let path = uri.path().to_string();
                let path_params =
                    paths::REGEX_PCA9956B_BUSID_ADDR_SLEEP_SLEEP
                    .captures(&path)
                    .unwrap_or_else(||
                        panic!("Path {} matched RE PCA9956B_BUSID_ADDR_SLEEP_SLEEP in set but failed match against \"{}\"", path, paths::REGEX_PCA9956B_BUSID_ADDR_SLEEP_SLEEP.as_str())
                    );

                let param_bus_id = match percent_encoding::percent_decode(path_params["busId"].as_bytes()).decode_utf8() {
                    Ok(param_bus_id) => match param_bus_id.parse::<i32>() {
                        Ok(param_bus_id) => param_bus_id,
                        Err(e) => return Box::new(future::ok(Response::new().with_status(StatusCode::BadRequest).with_body(format!("Couldn't parse path parameter busId: {}", e)))),
                    },
                    Err(_) => return Box::new(future::ok(Response::new().with_status(StatusCode::BadRequest).with_body(format!("Couldn't percent-decode path parameter as UTF-8: {}", &path_params["busId"]))))
                };
                let param_addr = match percent_encoding::percent_decode(path_params["addr"].as_bytes()).decode_utf8() {
                    Ok(param_addr) => match param_addr.parse::<i32>() {
                        Ok(param_addr) => param_addr,
                        Err(e) => return Box::new(future::ok(Response::new().with_status(StatusCode::BadRequest).with_body(format!("Couldn't parse path parameter addr: {}", e)))),
                    },
                    Err(_) => return Box::new(future::ok(Response::new().with_status(StatusCode::BadRequest).with_body(format!("Couldn't percent-decode path parameter as UTF-8: {}", &path_params["addr"]))))
                };
                let param_sleep = match percent_encoding::percent_decode(path_params["sleep"].as_bytes()).decode_utf8() {
                    Ok(param_sleep) => match param_sleep.parse::<bool>() {
                        Ok(param_sleep) => param_sleep,
                        Err(e) => return Box::new(future::ok(Response::new().with_status(StatusCode::BadRequest).with_body(format!("Couldn't parse path parameter sleep: {}", e)))),
                    },
                    Err(_) => return Box::new(future::ok(Response::new().with_status(StatusCode::BadRequest).with_body(format!("Couldn't percent-decode path parameter as UTF-8: {}", &path_params["sleep"]))))
                };





                Box::new({
                        {{

                                Box::new(api_impl.set_sleep(param_bus_id, param_addr, param_sleep, &context)
                                    .then(move |result| {
                                        let mut response = Response::new();
                                        response.headers_mut().set(XSpanId((&context as &Has<XSpanIdString>).get().0.to_string()));

                                        match result {
                                            Ok(rsp) => match rsp {
                                                SetSleepResponse::OK


                                                => {
                                                    response.set_status(StatusCode::try_from(200).unwrap());

                                                },
                                                SetSleepResponse::BadRequest

                                                    (body)


                                                => {
                                                    response.set_status(StatusCode::try_from(400).unwrap());

                                                    response.headers_mut().set(ContentType(mimetypes::responses::SET_SLEEP_BAD_REQUEST.clone()));


                                                    let body = serde_json::to_string(&body).expect("impossible to fail to serialize");

                                                    response.set_body(body);
                                                },
                                                SetSleepResponse::OperationFailed

                                                    (body)


                                                => {
                                                    response.set_status(StatusCode::try_from(502).unwrap());

                                                    response.headers_mut().set(ContentType(mimetypes::responses::SET_SLEEP_OPERATION_FAILED.clone()));


                                                    let body = serde_json::to_string(&body).expect("impossible to fail to serialize");

                                                    response.set_body(body);
                                                },
                                            },
                                            Err(_) => {
                                                // Application code returned an error. This should not happen, as the implementation should
                                                // return a valid response.
                                                response.set_status(StatusCode::InternalServerError);
                                                response.set_body("An internal error occurred");
                                            },
                                        }

                                        future::ok(response)
                                    }
                                ))

                        }}
                }) as Box<Future<Item=Response, Error=Error>>


            },


            _ => Box::new(future::ok(Response::new().with_status(StatusCode::NotFound))) as Box<Future<Item=Response, Error=Error>>,
        }
    }
}

impl<T, C> Clone for Service<T, C>
{
    fn clone(&self) -> Self {
        Service {
            api_impl: self.api_impl.clone(),
            marker: self.marker.clone(),
        }
    }
}

/// Request parser for `Api`.
pub struct ApiRequestParser;
impl RequestParser for ApiRequestParser {
    fn parse_operation_id(request: &Request) -> Result<&'static str, ()> {
        let path = paths::GLOBAL_REGEX_SET.matches(request.uri().path());
        match request.method() {

            // ClearError - POST /pca9956b/{busId}/{addr}/error/clear
            &hyper::Method::Post if path.matched(paths::ID_PCA9956B_BUSID_ADDR_ERROR_CLEAR) => Ok("ClearError"),

            // GetAddrEnabled - GET /pca9956b/{busId}/{addr}/addr/{num}/enabled
            &hyper::Method::Get if path.matched(paths::ID_PCA9956B_BUSID_ADDR_ADDR_NUM_ENABLED) => Ok("GetAddrEnabled"),

            // GetAddrInfo - GET /pca9956b/{busId}/{addr}/addr/{num}
            &hyper::Method::Get if path.matched(paths::ID_PCA9956B_BUSID_ADDR_ADDR_NUM) => Ok("GetAddrInfo"),

            // GetAddrValue - GET /pca9956b/{busId}/{addr}/addr/{num}/addr
            &hyper::Method::Get if path.matched(paths::ID_PCA9956B_BUSID_ADDR_ADDR_NUM_ADDR) => Ok("GetAddrValue"),

            // GetApi - GET /pca9956b/api
            &hyper::Method::Get if path.matched(paths::ID_PCA9956B_API) => Ok("GetApi"),

            // GetConfig - GET /pca9956b/{busId}/{addr}/config
            &hyper::Method::Get if path.matched(paths::ID_PCA9956B_BUSID_ADDR_CONFIG) => Ok("GetConfig"),

            // GetCurrent - GET /pca9956b/{busId}/{addr}/current
            &hyper::Method::Get if path.matched(paths::ID_PCA9956B_BUSID_ADDR_CURRENT) => Ok("GetCurrent"),

            // GetError - GET /pca9956b/{busId}/{addr}/error
            &hyper::Method::Get if path.matched(paths::ID_PCA9956B_BUSID_ADDR_ERROR) => Ok("GetError"),

            // GetErrors - POST /pca9956b/{busId}/{addr}/errors
            &hyper::Method::Post if path.matched(paths::ID_PCA9956B_BUSID_ADDR_ERRORS) => Ok("GetErrors"),

            // GetFreq - GET /pca9956b/{busId}/{addr}/freq
            &hyper::Method::Get if path.matched(paths::ID_PCA9956B_BUSID_ADDR_FREQ) => Ok("GetFreq"),

            // GetGroup - GET /pca9956b/{busId}/{addr}/group
            &hyper::Method::Get if path.matched(paths::ID_PCA9956B_BUSID_ADDR_GROUP) => Ok("GetGroup"),

            // GetLedCurrent - GET /pca9956b/{busId}/{addr}/led/{led}/current
            &hyper::Method::Get if path.matched(paths::ID_PCA9956B_BUSID_ADDR_LED_LED_CURRENT) => Ok("GetLedCurrent"),

            // GetLedError - GET /pca9956b/{busId}/{addr}/led/{led}/error
            &hyper::Method::Get if path.matched(paths::ID_PCA9956B_BUSID_ADDR_LED_LED_ERROR) => Ok("GetLedError"),

            // GetLedInfo - GET /pca9956b/{busId}/{addr}/led/{led}
            &hyper::Method::Get if path.matched(paths::ID_PCA9956B_BUSID_ADDR_LED_LED) => Ok("GetLedInfo"),

            // GetLedInfoAll - GET /pca9956b/{busId}/{addr}/led
            &hyper::Method::Get if path.matched(paths::ID_PCA9956B_BUSID_ADDR_LED) => Ok("GetLedInfoAll"),

            // GetLedPwm - GET /pca9956b/{busId}/{addr}/led/{led}/pwm
            &hyper::Method::Get if path.matched(paths::ID_PCA9956B_BUSID_ADDR_LED_LED_PWM) => Ok("GetLedPwm"),

            // GetLedState - GET /pca9956b/{busId}/{addr}/led/{led}/state
            &hyper::Method::Get if path.matched(paths::ID_PCA9956B_BUSID_ADDR_LED_LED_STATE) => Ok("GetLedState"),

            // GetOffset - GET /pca9956b/{busId}/{addr}/offset
            &hyper::Method::Get if path.matched(paths::ID_PCA9956B_BUSID_ADDR_OFFSET) => Ok("GetOffset"),

            // GetOutputChange - GET /pca9956b/{busId}/{addr}/outputChange
            &hyper::Method::Get if path.matched(paths::ID_PCA9956B_BUSID_ADDR_OUTPUTCHANGE) => Ok("GetOutputChange"),

            // GetOverTemp - GET /pca9956b/{busId}/{addr}/overTemp
            &hyper::Method::Get if path.matched(paths::ID_PCA9956B_BUSID_ADDR_OVERTEMP) => Ok("GetOverTemp"),

            // GetPwm - GET /pca9956b/{busId}/{addr}/pwm
            &hyper::Method::Get if path.matched(paths::ID_PCA9956B_BUSID_ADDR_PWM) => Ok("GetPwm"),

            // GetSleep - GET /pca9956b/{busId}/{addr}/sleep
            &hyper::Method::Get if path.matched(paths::ID_PCA9956B_BUSID_ADDR_SLEEP) => Ok("GetSleep"),

            // Reset - POST /pca9956b/{busId}/reset
            &hyper::Method::Post if path.matched(paths::ID_PCA9956B_BUSID_RESET) => Ok("Reset"),

            // SetAddrEnabled - POST /pca9956b/{busId}/{addr}/addr/{num}/enabled/{enabled}
            &hyper::Method::Post if path.matched(paths::ID_PCA9956B_BUSID_ADDR_ADDR_NUM_ENABLED_ENABLED) => Ok("SetAddrEnabled"),

            // SetAddrValue - POST /pca9956b/{busId}/{addr}/addr/{num}/addr/{addrVal}
            &hyper::Method::Post if path.matched(paths::ID_PCA9956B_BUSID_ADDR_ADDR_NUM_ADDR_ADDRVAL) => Ok("SetAddrValue"),

            // SetConfig - POST /pca9956b/{busId}/{addr}/config
            &hyper::Method::Post if path.matched(paths::ID_PCA9956B_BUSID_ADDR_CONFIG) => Ok("SetConfig"),

            // SetCurrent - POST /pca9956b/{busId}/{addr}/current/{current}
            &hyper::Method::Post if path.matched(paths::ID_PCA9956B_BUSID_ADDR_CURRENT_CURRENT) => Ok("SetCurrent"),

            // SetFreq - POST /pca9956b/{busId}/{addr}/freq/{freq}
            &hyper::Method::Post if path.matched(paths::ID_PCA9956B_BUSID_ADDR_FREQ_FREQ) => Ok("SetFreq"),

            // SetGroup - POST /pca9956b/{busId}/{addr}/group/{group}
            &hyper::Method::Post if path.matched(paths::ID_PCA9956B_BUSID_ADDR_GROUP_GROUP) => Ok("SetGroup"),

            // SetLedCurrent - POST /pca9956b/{busId}/{addr}/led/{led}/current/{current}
            &hyper::Method::Post if path.matched(paths::ID_PCA9956B_BUSID_ADDR_LED_LED_CURRENT_CURRENT) => Ok("SetLedCurrent"),

            // SetLedError - POST /pca9956b/{busId}/{addr}/led/{led}/error/{error}
            &hyper::Method::Post if path.matched(paths::ID_PCA9956B_BUSID_ADDR_LED_LED_ERROR_ERROR) => Ok("SetLedError"),

            // SetLedInfo - POST /pca9956b/{busId}/{addr}/led/{led}
            &hyper::Method::Post if path.matched(paths::ID_PCA9956B_BUSID_ADDR_LED_LED) => Ok("SetLedInfo"),

            // SetLedInfoAll - POST /pca9956b/{busId}/{addr}/led
            &hyper::Method::Post if path.matched(paths::ID_PCA9956B_BUSID_ADDR_LED) => Ok("SetLedInfoAll"),

            // SetLedPwm - POST /pca9956b/{busId}/{addr}/led/{led}/pwm/{pwm}
            &hyper::Method::Post if path.matched(paths::ID_PCA9956B_BUSID_ADDR_LED_LED_PWM_PWM) => Ok("SetLedPwm"),

            // SetLedState - POST /pca9956b/{busId}/{addr}/led/{led}/state/{state}
            &hyper::Method::Post if path.matched(paths::ID_PCA9956B_BUSID_ADDR_LED_LED_STATE_STATE) => Ok("SetLedState"),

            // SetOffset - POST /pca9956b/{busId}/{addr}/offset/{offset}
            &hyper::Method::Post if path.matched(paths::ID_PCA9956B_BUSID_ADDR_OFFSET_OFFSET) => Ok("SetOffset"),

            // SetOutputChange - POST /pca9956b/{busId}/{addr}/outputChange/{outputChange}
            &hyper::Method::Post if path.matched(paths::ID_PCA9956B_BUSID_ADDR_OUTPUTCHANGE_OUTPUTCHANGE) => Ok("SetOutputChange"),

            // SetPwm - POST /pca9956b/{busId}/{addr}/pwm/{pwm}
            &hyper::Method::Post if path.matched(paths::ID_PCA9956B_BUSID_ADDR_PWM_PWM) => Ok("SetPwm"),

            // SetSleep - POST /pca9956b/{busId}/{addr}/sleep/{sleep}
            &hyper::Method::Post if path.matched(paths::ID_PCA9956B_BUSID_ADDR_SLEEP_SLEEP) => Ok("SetSleep"),
            _ => Err(()),
        }
    }
}
